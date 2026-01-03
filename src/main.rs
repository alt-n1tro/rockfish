mod chess_board;
mod pieces_logic;

use eframe::egui;
use egui::{Color32, FontId, Pos2, Rect, Sense, Vec2};
use num_format::{Locale, ToFormattedString};

use pieces_logic::{Color, Move, Piece, Promotion, Symbol};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc,
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

/* =========================
   CONFIG
   ========================= */

const SAMPLE_PERIOD: Duration = Duration::from_millis(350);

/* =========================
   ENGINE MESSAGES
   ========================= */

#[derive(Debug, Clone, Copy)]
enum EngineMsg {
    Progress { nodes: u64, nps: u64 },
    Done { best: Option<Move>, nodes: u64, nps: u64 },
}

/* =========================
   GAME TYPES
   ========================= */

#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    Standard,
    PawnGalore,
}

#[derive(Clone, Copy)]
enum GameOver {
    Checkmate { winner_white: bool },
    Stalemate,
    InsufficientMaterial,
}

/* =========================
   APP
   ========================= */

struct ChessApp {
    board: [[Piece; 8]; 8],
    white_to_move: bool,

    selected: Option<(u8, u8)>,
    selected_moves: Vec<Move>,

    game_over: Option<GameOver>,
    game_mode: GameMode,

    // engine
    engine_depth: u8,
    engine_thinking: bool,
    engine_rx: Option<mpsc::Receiver<EngineMsg>>,
    engine_stop: Option<Arc<AtomicBool>>,
    engine_handle: Option<thread::JoinHandle<()>>,
    progress_handle: Option<thread::JoinHandle<()>>,

    // displayed stats
    nodes_display: u64,
    nps_display: u64,
}

/* =========================
   INIT
   ========================= */

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            board: chess_board::initialize_chess_board(),
            white_to_move: true,

            selected: None,
            selected_moves: Vec::new(),

            game_over: None,
            game_mode: GameMode::Standard,

            engine_depth: 7,
            engine_thinking: false,
            engine_rx: None,
            engine_stop: None,
            engine_handle: None,
            progress_handle: None,

            nodes_display: 0,
            nps_display: 0,
        }
    }
}

/* =========================
   HELPERS
   ========================= */

fn opponent(side: Color) -> Color {
    match side {
        Color::White => Color::Black,
        Color::Black => Color::White,
        Color::None => Color::None,
    }
}

impl ChessApp {
    fn reset(&mut self) {
        self.stop_engine_threads();

        self.board = match self.game_mode {
            GameMode::Standard => chess_board::initialize_chess_board(),
            GameMode::PawnGalore => chess_board::initialize_pawn_galore_board(),
        };
        self.white_to_move = true;
        self.selected = None;
        self.selected_moves.clear();
        self.game_over = None;

        self.engine_thinking = false;
        self.nodes_display = 0;
        self.nps_display = 0;
    }

    fn engine_thread_running(&self) -> bool {
        match self.engine_handle.as_ref() {
            Some(h) => !h.is_finished(),
            None => false,
        }
    }

    /// If we previously dropped the receiver (e.g. New Game / Quit while thinking),
    /// the search thread will eventually finish; join it here to free resources.
    fn reap_finished_engine_thread(&mut self) {
        if let Some(h) = self.engine_handle.as_ref() {
            if h.is_finished() {
                if let Some(h) = self.engine_handle.take() {
                    let _ = h.join();
                }
            }
        }
    }

    fn stop_engine_threads(&mut self) {
        // Best-effort stop: we can reliably stop the *progress* thread, but the search thread
        // itself cannot be cancelled unless the search code checks a stop flag.
        if let Some(stop) = &self.engine_stop {
            stop.store(true, Ordering::Relaxed);
        }

        // Drop the receiver so an in-flight engine result can't be applied after a reset/quit.
        self.engine_rx = None;
        self.engine_stop = None;
        self.engine_thinking = false;

        // Progress thread exits quickly (<= SAMPLE_PERIOD).
        if let Some(h) = self.progress_handle.take() {
            let _ = h.join();
        }

        // Join the search thread only if it's already done; otherwise keep the handle so we don't
        // start a second search that would share the global node counter.
        if let Some(h) = self.engine_handle.as_ref() {
            if h.is_finished() {
                if let Some(h) = self.engine_handle.take() {
                    let _ = h.join();
                }
            }
        }
    }

    fn piece_glyph(piece: &Piece) -> Option<char> {
        if piece.symbol == Symbol::Empty || piece.color == Color::None {
            return None;
        }
        Some(match (piece.color, piece.symbol) {
            (Color::White, Symbol::King) => '♚',
            (Color::White, Symbol::Queen) => '♛',
            (Color::White, Symbol::Rook) => '♜',
            (Color::White, Symbol::Bishop) => '♝',
            (Color::White, Symbol::Knight) => '♞',
            (Color::White, Symbol::Pawn) => '♟',
            (Color::Black, Symbol::King) => '♚',
            (Color::Black, Symbol::Queen) => '♛',
            (Color::Black, Symbol::Rook) => '♜',
            (Color::Black, Symbol::Bishop) => '♝',
            (Color::Black, Symbol::Knight) => '♞',
            (Color::Black, Symbol::Pawn) => '♟',
            _ => return None,
        })
    }

    fn legal_moves_from_square(&self, from: (u8, u8), side: Color) -> Vec<Move> {
        pieces_logic::get_all_legal_moves_for_this_turn(&self.board, side)
            .into_iter()
            .filter(|m| m.current_square == from)
            .collect()
    }

    fn try_player_move(&mut self, from: (u8, u8), to: (u8, u8)) -> bool {
        let Some(mut mv) = self
            .selected_moves
            .iter()
            .find(|m| m.current_square == from && m.destination_square == to)
            .copied()
        else {
            return false;
        };

        // Autopromote to queen for UI simplicity.
        let piece = self.board[from.0 as usize][from.1 as usize];
        if piece.symbol == Symbol::Pawn {
            let last = if piece.color == Color::White { 0 } else { 7 };
            if to.0 == last {
                mv.promotion = Promotion::Queen;
            }
        }

        pieces_logic::make_move(&mut self.board, &mv);

        self.white_to_move = false;
        self.selected = None;
        self.selected_moves.clear();
        self.update_game_over();
        true
    }

    fn update_game_over(&mut self) {
        if pieces_logic::is_insufficient_material(&self.board) {
            self.game_over = Some(GameOver::InsufficientMaterial);
            return;
        }

        let side = if self.white_to_move {
            Color::White
        } else {
            Color::Black
        };
        let moves = pieces_logic::get_all_legal_moves_for_this_turn(&self.board, side);

        if moves.is_empty() {
            if pieces_logic::is_king_in_check(&self.board, side) {
                self.game_over = Some(GameOver::Checkmate {
                    winner_white: !self.white_to_move,
                });
            } else {
                self.game_over = Some(GameOver::Stalemate);
            }
        }
    }

    fn square_from_pos(rect: Rect, pos: Pos2) -> Option<(u8, u8)> {
        let cell = rect.width() / 8.0;
        let col = ((pos.x - rect.left()) / cell).floor() as i32;
        let row = ((pos.y - rect.top()) / cell).floor() as i32;
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some((row as u8, col as u8))
        } else {
            None
        }
    }

    fn square_rect(rect: Rect, sq: (u8, u8)) -> Rect {
        let cell = rect.width() / 8.0;
        Rect::from_min_size(
            Pos2::new(
                rect.left() + sq.1 as f32 * cell,
                rect.top() + sq.0 as f32 * cell,
            ),
            Vec2::splat(cell),
        )
    }

    fn start_engine_if_needed(&mut self) {
        // If we aborted a previous search (e.g. New Game while thinking),
        // join it once it finishes so we can start a fresh search later.
        self.reap_finished_engine_thread();

        if self.white_to_move
            || self.game_over.is_some()
            || self.engine_thinking
            || self.engine_thread_running()
        {
            return;
        }

        self.engine_thinking = true;
        self.nodes_display = 0;
        self.nps_display = 0;

        let board_copy = self.board;
        let engine_depth = self.engine_depth;

        let (tx, rx) = mpsc::channel::<EngineMsg>();
        self.engine_rx = Some(rx);

        let stop = Arc::new(AtomicBool::new(false));
        self.engine_stop = Some(stop.clone());

        // Progress thread: every 0.25s send nodes + nps (delta-based).
        let tx_prog = tx.clone();
        let stop_prog = stop.clone();
        self.progress_handle = Some(thread::spawn(move || {
            let mut last_tick = Instant::now();
            let mut last_nodes = 0u64;

            loop {
                if stop_prog.load(Ordering::Relaxed) {
                    break;
                }

                thread::sleep(SAMPLE_PERIOD);

                if stop_prog.load(Ordering::Relaxed) {
                    break;
                }

                let now = Instant::now();
                let nodes = pieces_logic::nodes_get();
                let dt = now.duration_since(last_tick).as_secs_f64();
                let dn = nodes.saturating_sub(last_nodes);
                let nps = if dt > 0.0 { (dn as f64 / dt) as u64 } else { 0 };

                if tx_prog.send(EngineMsg::Progress { nodes, nps }).is_err() {
                    break;
                }

                last_tick = now;
                last_nodes = nodes;
            }
        }));

        let stop_search = stop.clone();

        // Engine thread: TT+Zobrist negamax search (depth-limited).
        // This requires these to exist in pieces_logic.rs:
        //   - nodes_reset()
        //   - nodes_get()
        //   - get_best_move_negamax_tt(&board, depth, side) -> Move
        self.engine_handle = Some(thread::spawn(move || {
            if stop_search.load(Ordering::Relaxed) {
                let _ = tx.send(EngineMsg::Done {
                    best: None,
                    nodes: 0,
                    nps: 0,
                });
                return;
            }

            pieces_logic::nodes_reset();

            let legal = pieces_logic::get_all_legal_moves_for_this_turn(&board_copy, Color::Black);

            if stop_search.load(Ordering::Relaxed) {
                let nodes = pieces_logic::nodes_get();
                let _ = tx.send(EngineMsg::Done {
                    best: None,
                    nodes,
                    nps: 0,
                });
                return;
            }

            if legal.is_empty() {
                let nodes = pieces_logic::nodes_get();
                let _ = tx.send(EngineMsg::Done {
                    best: None,
                    nodes,
                    nps: 0,
                });
                return;
            }

            let start = Instant::now();

            // Best-move search (internally TT+Zobrist + move ordering inside negamax)
            let best_move = pieces_logic::get_best_move_iterative_tt(&board_copy, engine_depth, Color::Black);

            let nodes = pieces_logic::nodes_get();
            let total_dt = start.elapsed().as_secs_f64();
            let nps = if total_dt > 0.0 {
                (nodes as f64 / total_dt) as u64
            } else {
                0
            };

            let _ = tx.send(EngineMsg::Done {
                best: Some(best_move),
                nodes,
                nps,
            });
        }));
    }

    fn poll_engine_messages(&mut self) {
        let Some(rx) = self.engine_rx.as_ref() else { return; };

        let mut msgs = Vec::new();
        while let Ok(msg) = rx.try_recv() {
            msgs.push(msg);
        }

        if msgs.is_empty() {
            return;
        }

        for msg in msgs {
            match msg {
                EngineMsg::Progress { nodes, nps } => {
                    self.nodes_display = nodes;
                    self.nps_display = nps;
                }
                EngineMsg::Done { best, nodes, nps } => {
                    self.nodes_display = nodes;
                    self.nps_display = nps;

                    if let Some(stop) = &self.engine_stop {
                        stop.store(true, Ordering::Relaxed);
                    }

                    self.engine_rx = None;
                    self.engine_stop = None;

                    if let Some(h) = self.progress_handle.take() {
                        let _ = h.join();
                    }
                    if let Some(h) = self.engine_handle.take() {
                        let _ = h.join();
                    }

                    self.engine_thinking = false;

                    if let Some(mv) = best {
                        pieces_logic::make_move(&mut self.board, &mv);
                        self.white_to_move = true;
                        self.update_game_over();
                    }
                }
            }
        }
    }
}

/* =========================
   APP
   ========================= */

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Process engine messages first so UI reflects updates immediately.
        self.poll_engine_messages();

        // Keep repainting while engine is thinking so progress text updates.
        if self.engine_thinking {
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Game Mode:");
                ui.radio_value(&mut self.game_mode, GameMode::Standard, "Standard");
                ui.radio_value(&mut self.game_mode, GameMode::PawnGalore, "Pawn Galore");

                if ui.button("New Game").clicked() {
                    self.reset();
                }

                if ui.button("Quit").clicked() {
                    // Graceful close (also stops progress thread).
                    self.stop_engine_threads();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.separator();

            let available = ui.available_size();
            let panel_width = 170.0;
            let board_size = (available.x - panel_width)
                .min(available.y)
                .max(480.0);

            let (rect, response) = ui.allocate_exact_size(Vec2::splat(board_size), Sense::click());
            let painter = ui.painter_at(rect);

            // Board
            for r in 0..8u8 {
                for c in 0..8u8 {
                    let sq = (r, c);
                    let cell = Self::square_rect(rect, sq);

                    let light = (r + c) % 2 == 0;
                    painter.rect_filled(
                        cell,
                        0.0,
                        if light {
                            Color32::from_rgb(240, 217, 181)
                        } else {
                            Color32::from_rgb(181, 136, 99)
                        },
                    );

                    if self.selected == Some(sq) {
                        painter.rect_stroke(
                            cell.shrink(3.0),
                            0.0,
                            egui::Stroke::new(3.0, Color32::from_rgb(0, 200, 255)),
                            egui::StrokeKind::Inside,
                        );
                    }

                    if self.selected.is_some()
                        && self
                            .selected_moves
                            .iter()
                            .any(|m| m.destination_square == sq)
                    {
                        painter.circle_filled(
                            cell.center(),
                            cell.width() * 0.12,
                            Color32::from_rgba_unmultiplied(0, 0, 0, 80),
                        );
                    }

                    let p = self.board[r as usize][c as usize];
                    if let Some(g) = Self::piece_glyph(&p) {
                        painter.text(
                            cell.center(),
                            egui::Align2::CENTER_CENTER,
                            g,
                            FontId::proportional(cell.width() * 0.75),
                            if p.color == Color::White {
                                Color32::WHITE
                            } else {
                                Color32::BLACK
                            },
                        );
                    }
                }
            }

            // Click handling (player = White)
            if response.clicked()
                && self.white_to_move
                && self.game_over.is_none()
                && !self.engine_thinking
            {
                if let Some(pos) = response.interact_pointer_pos() {
                    if let Some(sq) = Self::square_from_pos(rect, pos) {
                        // deselect
                        if self.selected == Some(sq) {
                            self.selected = None;
                            self.selected_moves.clear();
                        } else if let Some(from) = self.selected {
                            // attempt move
                            if !self.try_player_move(from, sq) {
                                // if failed, maybe select a different white piece
                                let p = self.board[sq.0 as usize][sq.1 as usize];
                                if p.color == Color::White && p.symbol != Symbol::Empty {
                                    self.selected = Some(sq);
                                    self.selected_moves =
                                        self.legal_moves_from_square(sq, Color::White);
                                } else {
                                    self.selected = None;
                                    self.selected_moves.clear();
                                }
                            }
                        } else {
                            // select piece
                            let p = self.board[sq.0 as usize][sq.1 as usize];
                            if p.color == Color::White && p.symbol != Symbol::Empty {
                                self.selected = Some(sq);
                                self.selected_moves = self.legal_moves_from_square(sq, Color::White);
                            }
                        }
                    }
                }
            }

            // Side panel
            let panel_rect = Rect::from_min_size(
                Pos2::new(rect.right() + 8.0, rect.top()),
                Vec2::new(panel_width, board_size),
            );

            ui.allocate_ui_at_rect(panel_rect, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(12.0);

                    ui.heading(if self.white_to_move {
                        "White to move"
                    } else {
                        "Black to move"
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(format!("Nodes: {}", self.nodes_display.to_formatted_string(&Locale::en)));
                    ui.label(format!("N/s:   {}", self.nps_display.to_formatted_string(&Locale::en)));

                    ui.add_space(8.0);
                    if self.engine_thinking {
                        ui.label("Engine: thinking...");
                    } else if self.engine_thread_running() && self.engine_rx.is_none() {
                        ui.label("Engine: finishing previous search...");
                    } else {
                        ui.label("Engine: idle");
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    if ui.button("I give up").clicked() {
                        self.stop_engine_threads();
                        self.game_over = Some(GameOver::Checkmate {
                            winner_white: false,
                        });
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label("Player: White");
                    ui.label(format!("Engine depth: {}", self.engine_depth));
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        if ui.button("5").clicked() {
                            self.engine_depth = 5;
                        }
                        if ui.button("7").clicked() {
                            self.engine_depth = 7;
                        }
                        if ui.button("9").clicked() {
                            self.engine_depth = 9;
                        }
                    });
                });
            });
        });

        // Start engine after player move.
        self.start_engine_if_needed();

        // Game over overlay
        if let Some(go) = self.game_over {
            egui::Window::new("Game Over")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(match go {
                        GameOver::Checkmate { winner_white } => {
                            if winner_white {
                                "Checkmate — White wins"
                            } else {
                                "Checkmate — Black wins"
                            }
                        }
                        GameOver::Stalemate => "Stalemate",
                        GameOver::InsufficientMaterial => "Draw — Insufficient material",
                    });
                });
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.stop_engine_threads();
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Rockfish",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(ChessApp::default()))),
    )
}




#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn create_empty_piece() {
        let empty_square = pieces_logic::Piece {
            color: Color::None,
            symbol: pieces_logic::Symbol::Empty,
            has_moved: false,
            value: 0,
            current_square: (0 as u8, 5 as u8),
        };
        assert_eq!(empty_square, pieces_logic::create_empty_piece(&(0 as u8, 5 as u8)));
    }
    
    #[test]
    fn get_2d_location_of_board_square() {
        let test1: u8 = 0;
        let test2: u8 = 12;
        let test3: u8 = 40;
        let test4: u8 = 63;
        
        let result1: (u8, u8) = (0, 0);
        let result2: (u8, u8) = (1, 4);
        let result3: (u8, u8) = (5, 0);
        let result4: (u8, u8) = (7, 7);
        
        assert_eq!(result1, pieces_logic::get_2d_location_of_board_square(&test1));
        assert_eq!(result2, pieces_logic::get_2d_location_of_board_square(&test2));
        assert_eq!(result3, pieces_logic::get_2d_location_of_board_square(&test3));
        assert_eq!(result4, pieces_logic::get_2d_location_of_board_square(&test4));

    }
    
    #[test]
    fn make_move() {
        
        let mut board = chess_board::initialize_chess_board();

        let move_: pieces_logic::Move = pieces_logic::Move { current_square: (6, 0), destination_square: (4, 0), castle: false, promotion: pieces_logic::Promotion::NoPromotion};
        pieces_logic::make_move(&mut board, &move_);
        
        let mut board_2 = chess_board::initialize_chess_board();

        board_2[6][0] = pieces_logic::create_empty_piece(&(6, 0));
        pieces_logic::place_pawn_on_board(&mut board_2, &(4, 0), Color::White);
        board_2[4][0].has_moved = true;

        chess_board::print_chess_board(&board);
        chess_board::print_chess_board(&board_2);

        assert_eq!(board, board_2);

    }


    #[test]
    fn get_square_of_king() {
        let mut board = chess_board::initialize_chess_board();
        
        let initial_pos_white_king: (u8, u8) = (7, 4);
        let white_king_pos_: (u8, u8) = (3, 3);
        
        let initial_pos_black_king: (u8, u8) = (0, 4);
        let black_king_pos_: (u8, u8) = (3, 7);

        assert_eq!(initial_pos_white_king, pieces_logic::get_square_of_king(&board, Color::White));
        assert_eq!(initial_pos_black_king, pieces_logic::get_square_of_king(&board, Color::Black));
        
        pieces_logic::make_move(&mut board, &pieces_logic::Move {
            current_square: initial_pos_white_king, 
            destination_square: white_king_pos_, 
            castle: false, 
            promotion: pieces_logic::Promotion::NoPromotion});

        pieces_logic::make_move(&mut board, &pieces_logic::Move {
            current_square: initial_pos_black_king, 
            destination_square: black_king_pos_, 
            castle: false, 
            promotion: pieces_logic::Promotion::NoPromotion});
        
        assert_eq!(white_king_pos_, pieces_logic::get_square_of_king(&board, Color::White));
        assert_eq!(black_king_pos_, pieces_logic::get_square_of_king(&board, Color::Black));


    }
    
    #[test]
    fn is_king_in_check_knights() {
        let mut board = chess_board::create_empty_board();
        let knight_moves: [(isize, isize); 8] = [(-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right


        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); 
        pieces_logic::place_knight_on_board(&mut board, &(2, 3), Color::White);
        assert!(pieces_logic::is_king_in_check(&board, Color::Black));
        board[2][3] = pieces_logic::create_empty_piece(&(2, 3));


        for x in 1..knight_moves.len() {
            let row: u8 = (4 + knight_moves[x].0) as u8;
            let col: u8 = (4 + knight_moves[x].1) as u8;
            let square: (u8, u8) = (row, col);

            pieces_logic::place_knight_on_board(&mut board, &square, Color::White);
            assert!(pieces_logic::is_king_in_check(&board, Color::Black));
            board[row as usize][col as usize] = pieces_logic::create_empty_piece(&square);
        }

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 7), Color::Black);
        pieces_logic::place_knight_on_board(&mut board, &(5, 6), Color::White);

        assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));

    }

    #[test]
    fn is_king_in_check_straights_up_down() {
        let mut board = chess_board::create_empty_board();
        
        // Rook
        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(1, 4), Color::White); // Enemy Rook

        assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), Color::White);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), Color::Black);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(1, 4), Color::Black); // Friendly Rook
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));

        // Queen 
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(1, 4), Color::White); // Enemy Queen

        assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), Color::White);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), Color::Black);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(1, 4), Color::Black); // Friendly Queen
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));

        
        // Test Bishop (will not produce check)

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_bishop_on_board(&mut board, &(1, 4), Color::White); // Enemy Bishop

        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
    }

    #[test]
    fn is_king_in_check_left_right() {
        let mut board = chess_board::create_empty_board();
            
        // Rook
        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(4, 1), Color::White); // Enemy Rook

        assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));
        
        // PAWN BLOCK
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), Color::White);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
       
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), Color::Black);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(4, 1), Color::Black); // Friendly Rook
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));

        // Queen 
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(4, 1), Color::White); // Enemy Queen

        assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));
        
        // PAWN BLOCK
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), Color::White);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), Color::Black);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(4, 1), Color::Black); // Friendly Queen
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));

        
        // Test Bishop (will not produce check)

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black); // Friendly King
        pieces_logic::place_bishop_on_board(&mut board, &(4, 1), Color::White); // Enemy Bishop

        assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));

    }
    

    #[test]
    fn is_king_in_check_diagonals() {
        for tup in [(0, 0), (6, 6), (7, 1), (1, 7)] {
        
            for x in [pieces_logic::Symbol::Bishop, pieces_logic::Symbol::Queen] {
            
                let mut board = chess_board::create_empty_board();

                pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::Black);
                
                pieces_logic::place_pawn_on_board(&mut board, &tup, Color::White);
                board[tup.0 as usize][tup.1 as usize].symbol = x; // Little workaround for testing

                assert_eq!(true, pieces_logic::is_king_in_check(&board, Color::Black));

                for p in [pieces_logic::Symbol::Rook, pieces_logic::Symbol::Knight] {
                    
                    let mut row: u8 = (tup.0 + 4) >> 1;
                    let col: u8 = (tup.1 + 4) >> 1;
                    

                    if tup == (7, 1) || tup == (1, 7) {
                        row += 1;
                    }

                    pieces_logic::place_pawn_on_board(&mut board, &(row, col), Color::White);
                    board[row as usize][col as usize].symbol = p;
                    assert_eq!(false, pieces_logic::is_king_in_check(&board, Color::Black));
                    
                }
            }
        }
    }
    
    #[test]
    fn is_king_in_check_pawns() {
        
        let mut board = chess_board::create_empty_board();

        for color in [true, false] {
            pieces_logic::place_king_on_board(&mut board, &(4, 4), if color {Color::White} else {Color::Black});
            for pos in [(3,3, color), (3, 4, false), (3, 5, color),
                        (4,3, false), (4, 5, false),
                        (5,3, !color), (5, 4, false), (5, 5, !color)] {
                pieces_logic::place_pawn_on_board(&mut board, &(pos.0, pos.1), if color {Color::Black} else {Color::White});
                assert_eq!(pos.2, pieces_logic::is_king_in_check(&board, if color {Color::White} else {Color::Black}));
                board[pos.0 as usize][pos.1 as usize] = pieces_logic::create_empty_piece(&(pos.0, pos.1));
            }
        }
    }

    #[test]
    fn is_piece_pinned_pawn() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_pawn_on_board(&mut board, &(6, 5), Color::White);
        pieces_logic::place_bishop_on_board(&mut board, &(4, 7), Color::Black);
        
        let pawn_move: pieces_logic::Move = pieces_logic::Move { current_square: (6, 5), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion};

        assert_eq!(true, pieces_logic::is_piece_pinned(&board, &pawn_move));
        

    }


    #[test]
    fn get_legal_moves_for_pawn() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_pawn_on_board(&mut board, &(6, 4), Color::White);
        pieces_logic::place_pawn_on_board(&mut board, &(5, 2), Color::White);
        board[5][2].has_moved = true;

        let mut legal_moves: Vec<pieces_logic::Move> = vec![];

        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (5, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion}); 
        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (4, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        let mut gen_legal_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_pawn(&board, &(6, 4));
        
        legal_moves.sort(); 
        gen_legal_moves.sort();

        assert_eq!(legal_moves, gen_legal_moves);
        
        legal_moves = vec![];
        legal_moves.push(pieces_logic::Move {current_square: (5, 2), destination_square: (4, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        gen_legal_moves = pieces_logic::get_legal_moves_for_pawn(&board, &(5, 2));

        assert_eq!(legal_moves, gen_legal_moves);


    }


    #[test]
    fn get_legal_moves_for_knight() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_knight_on_board(&mut board, &(5, 5), Color::White);

        pieces_logic:: place_rook_on_board(&mut board, &(3, 6), Color::Black);
        pieces_logic::place_rook_on_board(&mut board, &(4, 7), Color::White);
        chess_board::print_chess_board(&board);
        
        let mut exp_knight_moves: Vec<pieces_logic::Move> = vec![];

        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (3, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (3, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (6, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (7, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (6, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        let mut gen_knight_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_knight(&board, &(5, 5));

        exp_knight_moves.sort();
        gen_knight_moves.sort();

        assert_eq!(exp_knight_moves, gen_knight_moves);
    
        // Pin the horse
        board = chess_board::create_empty_board();
         
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_knight_on_board(&mut board, &(6, 4), Color::White);

        pieces_logic::place_rook_on_board(&mut board, &(0, 4), Color::Black);

        // No moves -- since pinned.
        exp_knight_moves = vec![];
        gen_knight_moves = pieces_logic::get_legal_moves_for_knight(&board, &(6, 4));

        assert_eq!(exp_knight_moves, gen_knight_moves);

    }

    #[test]
    fn get_legal_moves_for_bishop() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_bishop_on_board(&mut board, &(0, 0), Color::White);

        let mut bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));
        let mut exp_bishop_moves: Vec<pieces_logic::Move> = vec![];

        for x in 1..8 {
            exp_bishop_moves.push(pieces_logic::Move {current_square: (0, 0), destination_square: (x as u8, x as u8), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        }

        bishop_moves.sort();
        exp_bishop_moves.sort();

        assert_eq!(&bishop_moves, &exp_bishop_moves);

        pieces_logic::place_rook_on_board(&mut board, &(7, 7), Color::Black);
        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));

        assert_eq!(bishop_moves, [pieces_logic::Move {current_square: (0, 0), destination_square: (7, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion}]);
        
        exp_bishop_moves.pop();

        pieces_logic::place_rook_on_board(&mut board, &(7, 7), Color::White);
        
        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));
        bishop_moves.sort();

        assert_eq!(exp_bishop_moves, bishop_moves);
        
        // --------CLEAR--------------
        
        exp_bishop_moves = vec![];

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_bishop_on_board(&mut board, &(4, 4), Color::White);

        
        // top left
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        pieces_logic::place_rook_on_board(&mut board, &(3, 3), Color::Black);

        // top right
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (2, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (1, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        // bottom right 
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (6, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        pieces_logic::place_rook_on_board(&mut board, &(7, 7), Color::White);

        // bottom left 
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (6, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (7, 1), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        pieces_logic::place_knight_on_board(&mut board, &(7, 1), Color::Black);

        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(4, 4));
        
        bishop_moves.sort();
        exp_bishop_moves.sort();
        
        assert_eq!(bishop_moves, exp_bishop_moves);


        board = chess_board::create_empty_board();
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_knight_on_board(&mut board, &(5, 3), Color::Black);
        pieces_logic::place_bishop_on_board(&mut board, &(1, 7), Color::White);

        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(1, 7));

        exp_bishop_moves = vec![];
        exp_bishop_moves.push(pieces_logic::Move {current_square: (1, 7), destination_square: (5, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        chess_board::print_chess_board(&board);

        assert_eq!(bishop_moves, exp_bishop_moves);


    }
    

    #[test]
    fn get_legal_moves_for_rook() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_rook_on_board(&mut board, &(4, 5), Color::White);
        
        pieces_logic::place_rook_on_board(&mut board, &(7, 0), Color::Black);

        let mut rook_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));
        let mut expected_rook_moves: Vec<pieces_logic::Move> = vec![];

        assert_eq!(rook_moves, expected_rook_moves);
        
        pieces_logic::make_square_empty(&mut board, &(7, 0));

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 1), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 0), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        rook_moves = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));
        
        rook_moves.sort();
        expected_rook_moves.sort();

        assert_eq!(rook_moves, expected_rook_moves);
        
        expected_rook_moves = vec![];


        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        pieces_logic::place_knight_on_board(&mut board, &(1, 5), Color::Black);


        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), Color::White);

        rook_moves = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));

        rook_moves.sort();
        expected_rook_moves.sort();

        assert_eq!(rook_moves, expected_rook_moves);

    }

    #[test]
    fn get_legal_moves_for_queen() {

        let mut board = chess_board::create_empty_board();

        let mut expected_queen_moves: Vec<pieces_logic::Move> = vec![];
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_queen_on_board(&mut board, &(4, 5), Color::White);
        
        // Rook rays
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 1), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 0), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        // Bishop rays

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 1), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 6), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 7), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 2), castle: false, promotion: pieces_logic::Promotion::NoPromotion});


        let mut queen_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_queen(&board, &(4, 5));

        queen_moves.sort();
        expected_queen_moves.sort();
        
        assert_eq!(queen_moves, expected_queen_moves); 

    }

    #[test]
    fn get_legal_moves_for_king() {
        let mut board = chess_board::initialize_chess_board();
        
        let mut king_moves = pieces_logic::get_legal_moves_for_king(&board, &(7, 4));
        assert_eq!(king_moves, []);

        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), Color::White);
        
        let mut expected_king_moves: Vec<pieces_logic::Move> = vec![];

        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (4, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        pieces_logic::place_rook_on_board(&mut board, &(3, 0), Color::Black);
        
        king_moves = pieces_logic::get_legal_moves_for_king(&board, &(4, 4));
        
        king_moves.sort();
        expected_king_moves.sort();

        assert_eq!(king_moves, expected_king_moves);

        board[3][0] = pieces_logic::create_empty_piece(&(3, 0));


        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});

        king_moves = pieces_logic::get_legal_moves_for_king(&board, &(4, 4));

        king_moves.sort();
        expected_king_moves.sort();

        assert_eq!(king_moves, expected_king_moves);
         
    }

    #[test]
    fn get_castle_move() {
        let mut board = chess_board::initialize_chess_board();

        board[7][6] = pieces_logic::create_empty_piece(&(7, 6));
        board[7][3] = pieces_logic::create_empty_piece(&(7, 3));
        board[7][2] = pieces_logic::create_empty_piece(&(7, 2));

        let mut castle_moves_white: Vec<pieces_logic::Move> = pieces_logic::get_castling_moves(&board, Color::White);
        castle_moves_white.sort();
        let castle_moves_black: Vec<pieces_logic::Move> = pieces_logic::get_castling_moves(&board, Color::Black);
        
        // Testing against full side 
        let mut exp_castle_moves: Vec<pieces_logic::Move> = vec![];
        assert_eq!(exp_castle_moves, castle_moves_black); 
        
        // Testing where right side is blocked by piece
        exp_castle_moves.push(pieces_logic::Move { current_square: (7, 4), destination_square: (7, 2), castle: true, promotion: pieces_logic::Promotion::NoPromotion });
        assert_eq!(exp_castle_moves, castle_moves_white);

        // Removing piece that's blocking right-side castling
        board[7][5] = pieces_logic::create_empty_piece(&(7, 5));
        castle_moves_white = pieces_logic::get_castling_moves(&board, Color::White);

        exp_castle_moves.push(pieces_logic::Move { current_square: (7, 4), destination_square: (7, 6), castle: true, promotion: pieces_logic::Promotion::NoPromotion });
        exp_castle_moves.sort();

        assert_eq!(castle_moves_white, exp_castle_moves);

    }


    #[test]
    fn get_all_legal_moves_for_this_turn() {
        let mut board = chess_board::create_empty_board();
        let mut expected_board_moves: Vec<pieces_logic::Move> = vec![];

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_pawn_on_board(&mut board, &(5, 4), Color::White);
        board[5][4].has_moved = true;
        pieces_logic::place_pawn_on_board(&mut board, &(6, 3), Color::White);   
        pieces_logic::place_pawn_on_board(&mut board, &(6, 5), Color::White);   
        pieces_logic::place_pawn_on_board(&mut board, &(7, 3), Color::White);   
        pieces_logic::place_pawn_on_board(&mut board, &(7, 5), Color::White);

        
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 5), destination_square: (5, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 5), destination_square: (4, 5), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_board_moves.push(pieces_logic::Move { current_square: (5, 4), destination_square: (4, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 3), destination_square: (5, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 3), destination_square: (4, 3), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        expected_board_moves.push(pieces_logic::Move { current_square: (7, 4), destination_square: (6, 4), castle: false, promotion: pieces_logic::Promotion::NoPromotion});
        
        let mut board_moves: Vec<pieces_logic::Move> = pieces_logic::get_all_legal_moves_for_this_turn(&board, Color::White);

        board_moves.sort();
        expected_board_moves.sort();

        assert_eq!(expected_board_moves, board_moves);
    }
    

    #[test]
    fn is_checkmate() {
        let mut board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_rook_on_board(&mut board, &(6, 0), Color::Black);
        pieces_logic::place_rook_on_board(&mut board, &(7, 7), Color::Black);

        assert_eq!(true, pieces_logic::is_checkmate(&board, Color::White));

        pieces_logic::place_rook_on_board(&mut board, &(0, 5), Color::White);

        assert_eq!(false, pieces_logic::is_checkmate(&board, Color::White));


    }

    #[test]
    fn is_stalemate() { 
        let mut board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_rook_on_board(&mut board, &(0, 3), Color::Black);
        pieces_logic::place_rook_on_board(&mut board, &(0, 5), Color::Black);
        pieces_logic::place_rook_on_board(&mut board, &(6, 0), Color::Black);

        assert_eq!(true, pieces_logic::is_stalemate(&board, Color::White));
        
        pieces_logic::place_pawn_on_board(&mut board, &(3, 0), Color::White);

        assert_eq!(false, pieces_logic::is_stalemate(&board, Color::White));
    
        pieces_logic::place_knight_on_board(&mut board, &(2, 0), Color::Black);

        assert_eq!(true, pieces_logic::is_stalemate(&board, Color::White));

    }


    #[test]
    fn pawn_promotion() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_pawn_on_board(&mut board, &(1, 4), Color::White);
        board[1][4].has_moved = true;

        let mut pawn_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_pawn(&board, &(1, 4));
        
        let mut exp_pawn_moves: Vec<pieces_logic::Move> = vec![];

        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Queen});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Rook});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Bishop});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Knight});
        
        pawn_moves.sort();
        exp_pawn_moves.sort();

        assert_eq!(pawn_moves, exp_pawn_moves);
        
        exp_pawn_moves = vec![];
        // No Moves
        pieces_logic::place_bishop_on_board(&mut board, &(0, 4), Color::Black);
        

        pieces_logic::place_bishop_on_board(&mut board, &(0, 3), Color::Black);
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 3), castle: false, promotion: pieces_logic::Promotion::Queen});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 3), castle: false, promotion: pieces_logic::Promotion::Rook});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 3), castle: false, promotion: pieces_logic::Promotion::Bishop});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 3), castle: false, promotion: pieces_logic::Promotion::Knight});
        
        pieces_logic::place_bishop_on_board(&mut board, &(0, 5), Color::Black);
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::Queen});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::Rook});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::Bishop});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 5), castle: false, promotion: pieces_logic::Promotion::Knight});

        pawn_moves = pieces_logic::get_legal_moves_for_pawn(&board, &(1, 4));

        pawn_moves.sort();
        exp_pawn_moves.sort();
        
        assert_eq!(pawn_moves, exp_pawn_moves);
        
        board[0][4] = pieces_logic::create_empty_piece(&(0, 4));

        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Queen});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Rook});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Bishop});
        exp_pawn_moves.push(pieces_logic::Move { current_square: (1, 4), destination_square: (0, 4), castle: false, promotion: pieces_logic::Promotion::Knight});

        
        pawn_moves = pieces_logic::get_legal_moves_for_pawn(&board, &(1, 4));

        pawn_moves.sort();
        exp_pawn_moves.sort();
        
        assert_eq!(pawn_moves, exp_pawn_moves);
    }

    #[test]
    fn en_passant() {
        assert!(false);
    }

    #[test]
    fn insufficient_material_stalemate() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), Color::White);
        pieces_logic::place_king_on_board(&mut board, &(0, 4), Color::Black);

        assert_eq!(true, pieces_logic::is_insufficient_material(&board));
        
        pieces_logic::place_pawn_on_board(&mut board, &(6, 7), Color::White);

        assert_eq!(false, pieces_logic::is_insufficient_material(&board));
        
        board[6][7] = pieces_logic::create_empty_piece(&(6, 7));


        pieces_logic::place_bishop_on_board(&mut board, &(7, 5), Color::White);

        assert_eq!(true, pieces_logic::is_insufficient_material(&board));

        pieces_logic::place_bishop_on_board(&mut board, &(7, 6), Color::White);

        assert_eq!(false, pieces_logic::is_insufficient_material(&board));
        
    }

}


