use std::sync::atomic::{AtomicU64, Ordering};
use crate::chess_board;
static NODES_EVALUATED: AtomicU64 = AtomicU64::new(0);

pub fn nodes_reset() {
    NODES_EVALUATED.store(0, Ordering::Relaxed);
}

pub fn nodes_inc() {
    NODES_EVALUATED.fetch_add(1, Ordering::Relaxed);
}

pub fn nodes_get() -> u64 {
    NODES_EVALUATED.load(Ordering::Relaxed)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub color: Color,
    pub symbol: Symbol,
    pub has_moved: bool,
    pub value: i64,
    pub current_square: (u8, u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
    NoPromotion,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Symbol {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    White,
    Black,
    None,
}

// This should only ever be created via a function, that checks if the move is actually legal.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub current_square: (u8, u8),
    pub destination_square: (u8, u8),
    pub castle: bool,
    pub promotion: Promotion,
}


pub fn create_empty_piece(square: &(u8, u8)) -> Piece {
    Piece { color: Color::None, 
        symbol: Symbol::Empty,
        has_moved: false,
        value: 0,
        current_square: *square }
}

pub fn place_pawn_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Pawn,
        has_moved: false,
        value: 100,
        current_square: *square
    };

}

pub fn place_bishop_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Bishop,
        has_moved: false,
        value: 330,
        current_square: *square
    };
    
}

pub fn place_knight_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Knight,
        has_moved: false,
        value: 320,
        current_square: *square
    };
}

pub fn place_rook_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Rook,
        has_moved: false,
        value: 500,
        current_square: *square
    };
}

pub fn place_queen_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Queen,
        has_moved: false,
        value: 900,
        current_square: *square
    };
}

pub fn place_king_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: Color) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::King,
        has_moved: false,
        value: 10000,
        current_square: *square
    };
}



pub fn make_square_empty(board: &mut [[Piece; 8]; 8], square: &(u8, u8)) {
    let piece = create_empty_piece(&square);
    board[square.0 as usize][square.1 as usize] = piece;
}


// Perhaps we won't need this function in the future.
pub fn get_2d_location_of_board_square(square: &u8) -> (u8, u8) {
    (square / 8, square % 8)
}

// Reverse X-ray checker
// If a piece is on the same diagonal/straight as the king, then we only need to check that particular diagonal/straight for a potential X-ray check.
// Essentially, the piece becomes the pointer which points through the diagonal / straight
// 
/// Takes a reference to the board (since we're just doing a lookup here).
/// color = true (white) ; color = false (black)
pub fn get_square_of_king(board: &[[Piece; 8]; 8], color: Color) -> (u8, u8) {

    if color == Color::White {
        if board[7][4].symbol == Symbol::King && board[7][4].color == Color::White {
            return (7 as u8, 4 as u8);
        }
    } else {
        if board[0][4].symbol == Symbol::King && board[0][4].color == Color::Black {
            return (0 as u8, 4 as u8);
        }
    }

    for x in (0..8).rev() {
        for y in (0..8).rev() {
            let piece = board[x][y];
            if piece.symbol == Symbol::King && piece.color == color {
                return (x as u8, y as u8);
            }
        }
    }
    chess_board::print_chess_board(&board);
    unreachable!("\nNo King was found...\nAt least 2 Kings MUST exist at all times!"); 
}



// This function looks outward from the king -- Not from the enemy pieces!
pub fn is_king_in_check(board: &[[Piece;8];8], color: Color) -> bool {
    
    let k_s: (u8, u8) = get_square_of_king(&board, color);
    let king: Piece = board[k_s.0 as usize][k_s.1 as usize];



    // Check Knights
    
    let knight_moves: [(isize, isize); 8] = [(-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right

    
    for x in 0..knight_moves.len() {
        let n_move = knight_moves[x];

        let row = k_s.0 as isize + n_move.0;
        let col = k_s.1 as isize + n_move.1;
        if (row >= 0 && row < 8) && (col >= 0 && col < 8) {
            let square = board[row as usize][col as usize];
            if  (square.color != king.color) && square.symbol == Symbol::Knight {
                return true;    
            }
        }  
    }

    // Check Rook + Queen(straights)
    // Check Up+Down
    // Check Left+Right
    

    let rook_moves = [(-1, 0), (1, 0),
                      (0, -1), (0, 1)];
    
    for rook_move in rook_moves {
        let mut row = k_s.0 as i8 + rook_move.0;
        let mut col = k_s.1 as i8 + rook_move.1;

        'inner_while: while row >= 0 && row < 8 && col >= 0 && col < 8 {
            
            let square = &board[row as usize][col as usize];

            if square.symbol != Symbol::Empty {
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, Symbol::Rook | Symbol::Queen) {
                        return true;
                    } else {
                        break 'inner_while;
                    }
                }
            }
            row += rook_move.0;
            col += rook_move.1;
        }
    }

    // Check Bishop + Queen(diagonals)
    // top right -> (-1, 1)
    // top left -> (-1, -1)
    // bottom right -> (1, 1) --> Check this.
    // bottom left -> (1, -1)
    
    let bishop_moves = [(1, 1), (1, -1),
                        (-1, 1), (-1, -1)];
    
    for b_move in bishop_moves {
        let mut bishop_row: i8 = k_s.0 as i8 + b_move.0;
        let mut bishop_col: i8 = k_s.1 as i8+ b_move.1;

        'inner_while: while bishop_row >= 0 && bishop_row < 8 && bishop_col >= 0 && bishop_col < 8 {
            
            let square = &board[bishop_row as usize][bishop_col as usize];

            if square.symbol != Symbol::Empty {
                
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, Symbol::Bishop | Symbol::Queen) {
                        return true;
                    } else {
                        break 'inner_while;
                    }
                }

            }

            bishop_row += b_move.0;
            bishop_col += b_move.1;
        }

    }
    

    let black_pawns: [(i8, i8);2] = [((k_s.0 as i8 - 1), (k_s.1 as i8 + 1)),
                                    ((k_s.0 as i8 - 1), (k_s.1 as i8 - 1))];
    let white_pawns: [(i8, i8);2] = [((k_s.0 as i8 + 1), (k_s.1 as i8 + 1)),
                                    ((k_s.0 as i8 + 1), (k_s.1 as i8 - 1))];
    // Check Pawns 
    if king.color == Color::White {
        for pawn in black_pawns {
            if pawn.0 >= 0 && pawn.1 < 8 && pawn.1 >= 0{
                let pawn = board[pawn.0 as usize][pawn.1 as usize];
                if pawn.symbol == Symbol::Pawn && pawn.color == Color::Black {
                    return true;
                }
            } 
        }
    } else {
        for pawn in white_pawns {
            if pawn.0 < 8 && pawn.1 < 8 && pawn.1 >= 0{
                let pawn = board[pawn.0 as usize][pawn.1 as usize];
                if pawn.symbol == Symbol::Pawn && pawn.color == Color::White {
                    return true;
                }
            } 
        }
    }

    return false;
}






pub fn is_piece_pinned(board: &[[Piece;8];8], piece_move: &Move) -> bool {
    
    let from = piece_move.current_square;

    let piece_to_move: Piece = board[from.0 as usize][from.1 as usize];
    
    let mut new_board = *board;

    make_move(&mut new_board, piece_move);

    is_king_in_check(&new_board, piece_to_move.color)

}



// Moves 

// Piece specific move functions
pub fn get_legal_moves_for_pawn(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let mut output: Vec<Move> = vec![];
        
    let piece_to_move = board[square.0 as usize][square.1 as usize];

    let adder: i8 = if piece_to_move.color == Color::White {
        -1
    } else {
        1
    };
    
    let new_row = square.0 as i8 + adder;

    if new_row >= 0 && new_row < 8 { 
        if board[new_row as usize][square.1 as usize].symbol == Symbol::Empty {
            let up_move: Move = Move {current_square: *square, destination_square: (new_row as u8, square.1), castle: false, promotion: Promotion::NoPromotion};
            if !is_piece_pinned(&board, &up_move) {
                output.push(up_move);
                if (new_row + adder) >= 0 && (new_row + adder) < 8 {
                    if board[(new_row + adder) as usize][square.1 as usize].symbol == Symbol::Empty && !piece_to_move.has_moved {
                        let up_up_move: Move = Move { current_square: *square, destination_square: ((new_row + adder) as u8, square.1), castle: false, promotion: Promotion::NoPromotion};
                        output.push(up_up_move); 
                    }
                }
            }
        }

        // right capture
        if square.1 < 7 {
            let board_square = board[new_row as usize][(square.1 + 1) as usize];
            if board_square.color != piece_to_move.color && board_square.symbol != Symbol::Empty && board_square.symbol != Symbol::King {
                let left_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 + 1), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &left_move) {
                    output.push(left_move);
                }
            }
        }
        // left capture
        if square.1 >= 1 {
            let board_square = board[new_row as usize][(square.1 - 1) as usize];
            if board_square.color != piece_to_move.color && board_square.symbol != Symbol::Empty && board_square.symbol != Symbol::King {
                let right_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 - 1), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &right_move) {
                    output.push(right_move);
                }
            }
        }
    }

    if new_row == 0 || new_row == 7 {
        let mut temp_output: Vec<Move> = vec![];
        for pawn_move in &output {
            let mut temp_pawn_move = *pawn_move;
            for x in [Promotion::Queen, Promotion::Rook, Promotion::Bishop, Promotion::Knight] {
                temp_pawn_move.promotion = x;
                temp_output.push(temp_pawn_move);
            }
        }
        return temp_output;
    }

    output
}

pub fn get_legal_moves_for_knight(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> { 
    
    let mut output: Vec<Move> = vec![];

    let knight_moves: [(isize, isize); 8] = [
        (-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right
    
    for knight in knight_moves {
        let row = knight.0 + square.0 as isize;
        let col = knight.1 + square.1 as isize;
        if row >= 0 && row < 8 && col >= 0 && col < 8 {
            let to_sqr = board[row as usize][col as usize];
            if to_sqr.symbol == Symbol::Empty || (to_sqr.color != board[square.0 as usize][square.1 as usize].color) && to_sqr.symbol != Symbol::King {
                let knight_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &knight_move) {
                    output.push(knight_move);
                }
            }
        }
    }
    output
}

pub fn get_legal_long_ray_moves(board: &[[Piece; 8]; 8], square: &(u8, u8), moves: [(i8, i8); 4]) -> Vec<Move> {

    let mut output: Vec<Move> = vec![];

    for x in moves {
        let mut row = square.0 as i8 + x.0;
        let mut col = square.1 as i8 + x.1;
        
        'inner_while: while row >= 0 && row < 8 && col >= 0 && col < 8 {
            let destination_square = board[row as usize][col as usize];
            let piece_color = board[square.0 as usize][square.1 as usize].color;
            
            let proposed_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8), castle: false, promotion: Promotion::NoPromotion};

            if destination_square.symbol != Symbol::Empty && destination_square.color == piece_color {  
                break 'inner_while; 
            }

            if destination_square.symbol == Symbol::King && destination_square.color != piece_color {
                break 'inner_while;
            }
                
            if !is_piece_pinned(&board, &proposed_move) {
                output.push(proposed_move);
            }

            if destination_square.symbol != Symbol::Empty {
                break 'inner_while;
            }                
            
            row += x.0;
            col += x.1;
        }
    }
    output

}


pub fn get_legal_moves_for_bishop(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    let bishop_moves: [(i8, i8); 4] = [(1, 1), (1, -1),
                                       (-1, 1), (-1, -1)];
    get_legal_long_ray_moves(&board, &square, bishop_moves)    
}

pub fn get_legal_moves_for_rook(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {

    let rook_moves: [(i8, i8); 4] = [(1, 0), (-1, 0),
                                     (0, 1), (0, -1)];
    get_legal_long_ray_moves(&board, &square, rook_moves)    
}



pub fn get_legal_moves_for_queen(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let mut output: Vec<Move> = get_legal_moves_for_rook(&board, &square);

    output.extend(get_legal_moves_for_bishop(&board, &square));
    
    output
}


pub fn get_legal_moves_for_king(board: &[[Piece; 8]; 8], square: &(u8, u8)) -> Vec<Move> {
    let king = board[square.0 as usize][square.1 as usize];
    let side = king.color;

    // Sanity
    if king.symbol != Symbol::King || side == Color::None {
        return vec![];
    }

    let opp = if side == Color::White { Color::Black } else { Color::White };

    // Find opponent king once (for adjacency rule)
    let mut opp_king_sq: Option<(u8, u8)> = None;
    'find: for r in 0..8 {
        for c in 0..8 {
            let p = board[r][c];
            if p.symbol == Symbol::King && p.color == opp {
                opp_king_sq = Some((r as u8, c as u8));
                break 'find;
            }
        }
    }

    #[inline]
    fn in_bounds(r: i8, c: i8) -> bool {
        (0..=7).contains(&r) && (0..=7).contains(&c)
    }

    let is_adjacent_to_opp_king = |to: (u8, u8)| -> bool {
        if let Some(ok) = opp_king_sq {
            let dr = (to.0 as i16 - ok.0 as i16).abs();
            let dc = (to.1 as i16 - ok.1 as i16).abs();
            dr <= 1 && dc <= 1
        } else {
            false
        }
    };

    let mut out = Vec::new();

    // Normal king steps
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = square.0 as i8 + dr;
            let nc = square.1 as i8 + dc;
            if !in_bounds(nr, nc) {
                continue;
            }

            let to = (nr as u8, nc as u8);
            let dst = board[to.0 as usize][to.1 as usize];

            // Can't land on own piece
            if dst.color == side {
                continue;
            }

            // King is not capturable (never allow a move that "captures" a king)
            if dst.symbol == Symbol::King {
                continue;
            }

            // Kings may never be adjacent
            if is_adjacent_to_opp_king(to) {
                continue;
            }

            let mv = Move {
                current_square: *square,
                destination_square: to,
                castle: false,
                promotion: Promotion::NoPromotion,
            };

            // Must not move into check
            let mut tmp = *board;
            make_move(&mut tmp, &mv);
            if is_king_in_check(&tmp, side) {
                continue;
            }

            out.push(mv);
        }
    }

    // Castling
    // Conditions:
    // - King/rook unmoved
    // - King not currently in check
    // - Squares between are empty
    // - Squares king crosses/lands on are not attacked
    // - Not adjacent to enemy king on destination/crossing squares
    let home_row = if side == Color::White { 7 } else { 0 };

    if square.0 == home_row && square.1 == 4 && !king.has_moved {
        if !is_king_in_check(board, side) {
            let king_safe_on = |to_col: u8| -> bool {
                let mv = Move {
                    current_square: *square,
                    destination_square: (home_row, to_col),
                    castle: false,
                    promotion: Promotion::NoPromotion,
                };
                let mut tmp = *board;
                make_move(&mut tmp, &mv);

                if is_king_in_check(&tmp, side) {
                    return false;
                }
                if is_adjacent_to_opp_king((home_row, to_col)) {
                    return false;
                }
                true
            };

            // Kingside: e -> g, rook h -> f
            let rook_h = board[home_row as usize][7];
            if rook_h.symbol == Symbol::Rook && rook_h.color == side && !rook_h.has_moved {
                if board[home_row as usize][5].symbol == Symbol::Empty
                    && board[home_row as usize][6].symbol == Symbol::Empty
                {
                    if king_safe_on(5) && king_safe_on(6) {
                        out.push(Move {
                            current_square: *square,
                            destination_square: (home_row, 6),
                            castle: true,
                            promotion: Promotion::NoPromotion,
                        });
                    }
                }
            }

            // Queenside: e -> c, rook a -> d
            let rook_a = board[home_row as usize][0];
            if rook_a.symbol == Symbol::Rook && rook_a.color == side && !rook_a.has_moved {
                if board[home_row as usize][1].symbol == Symbol::Empty
                    && board[home_row as usize][2].symbol == Symbol::Empty
                    && board[home_row as usize][3].symbol == Symbol::Empty
                {
                    if king_safe_on(3) && king_safe_on(2) {
                        out.push(Move {
                            current_square: *square,
                            destination_square: (home_row, 2),
                            castle: true,
                            promotion: Promotion::NoPromotion,
                        });
                    }
                }
            }
        }
    }

    out
}


pub fn get_castling_moves(board: &[[Piece;8];8], color: Color) -> Vec<Move> {
    
    let king = get_square_of_king(&board, color);
    let king_piece = board[king.0 as usize][king.1 as usize];

    let mut output: Vec<Move> = vec![];

    let row: usize = if color == Color::None {7} else {0};
    let left_corner: Piece = board[row][0];
    let right_corner: Piece = board[row][7];

    if king_piece.has_moved {
        return output;
    }

    if left_corner.symbol != Symbol::Empty && left_corner.symbol == Symbol::Rook && !left_corner.has_moved {
        
        if  board[king.0 as usize][(king.1 - 1) as usize].symbol == Symbol::Empty &&
            board[king.0 as usize][(king.1 - 2) as usize].symbol == Symbol::Empty {
            // Check for pins 
            if  !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 - 1), castle: false, promotion: Promotion::NoPromotion }) &&
                !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 - 2), castle: false, promotion: Promotion::NoPromotion }) {
                output.push(Move { current_square: king, destination_square: (king.0, king.1 - 2), castle: true, promotion: Promotion::NoPromotion });
            }
        }
    }
    
    if right_corner.symbol != Symbol::Empty && right_corner.symbol == Symbol::Rook && !right_corner.has_moved {
        
        if  board[king.0 as usize][(king.1 + 1) as usize].symbol == Symbol::Empty &&
            board[king.0 as usize][(king.1 + 2) as usize].symbol == Symbol::Empty {
            // Check for pins 
            if  !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 + 1), castle: false, promotion: Promotion::NoPromotion }) &&
                !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 + 2), castle: false, promotion: Promotion::NoPromotion }) {
                output.push(Move { current_square: king, destination_square: (king.0, king.1 + 2), castle: true, promotion: Promotion::NoPromotion });
            }
        }
    }
    output
}




// We ensure that every move is legal in other functions.
pub fn make_move(board: &mut [[Piece; 8]; 8], move_: &Move) {
    
    let cur_sq = move_.current_square;
    let des_sq = move_.destination_square;
    
    board[des_sq.0 as usize][des_sq.1 as usize] = board[cur_sq.0 as usize][cur_sq.1 as usize];    
    board[des_sq.0 as usize][des_sq.1 as usize].current_square = des_sq;
    board[des_sq.0 as usize][des_sq.1 as usize].has_moved = true;
    

    if move_.castle {
        let shift: i8 = if des_sq.1 > cur_sq.1 {1} else {-1};
        let rook_side: usize = if des_sq.1 > cur_sq.1 {7} else {0};
        
        board[cur_sq.0 as usize][(cur_sq.1 as i8 + shift) as usize] = board[cur_sq.0 as usize][rook_side];
        board[cur_sq.0 as usize][(cur_sq.1 as i8 + shift) as usize].current_square = (cur_sq.0, (cur_sq.1 as i8 + shift) as u8);
        board[cur_sq.0 as usize][(cur_sq.1 as i8 + shift) as usize].has_moved = true;

        board[cur_sq.0 as usize][rook_side] = create_empty_piece(&(cur_sq.0, rook_side as u8));    
    }

    match move_.promotion {
        Promotion::Queen => place_queen_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Rook => place_rook_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Bishop => place_bishop_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Knight => place_knight_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::NoPromotion => {},
    };


    board[cur_sq.0 as usize][cur_sq.1 as usize] = create_empty_piece(&cur_sq);

}



// For bot moves
pub fn find_all_legal_moves_for_a_piece(board: &[[Piece; 8]; 8], square: &(u8, u8)) -> Vec<Move> {
    
    let empty_vec: Vec<Move> = vec![];

    match board[square.0 as usize][square.1 as usize].symbol {
        Symbol::Pawn => get_legal_moves_for_pawn(&board, &square),
        Symbol::Bishop => get_legal_moves_for_bishop(&board, &square),
        Symbol::Knight => get_legal_moves_for_knight(&board, &square),
        Symbol::Rook => get_legal_moves_for_rook(&board, &square),
        Symbol::Queen => get_legal_moves_for_queen(&board, &square),
        Symbol::King => get_legal_moves_for_king(&board, &square),
        Symbol::Empty => empty_vec,
    }

}

pub fn get_all_legal_moves_for_this_turn(board: &[[Piece;8];8], side: Color) -> Vec<Move> {

    let mut output: Vec<Move> = vec![];
    
    for x in 0..8 {
        for y in 0..8 {
            let board_square: Piece = board[x][y];
            if board_square.color == side && board_square.symbol != Symbol::Empty { 
                output.extend(find_all_legal_moves_for_a_piece(&board, &(x as u8, y as u8)));
            }
        }
    } 

    output
}


// Game States
pub fn is_checkmate(board: &[[Piece; 8]; 8], side: Color) -> bool {

    let output: Vec<Move> = vec![];

    is_king_in_check(&board, side) && get_all_legal_moves_for_this_turn(&board, side) == output
}

pub fn is_stalemate(board: &[[Piece; 8]; 8], side: Color) -> bool {

    let output: Vec<Move> = vec![];

    !is_king_in_check(&board, side) && get_all_legal_moves_for_this_turn(&board, side) == output
}

pub fn is_insufficient_material(board: &[[Piece;8];8]) -> bool {
    
    let mut white_counts: u8 = 0;
    let mut black_counts: u8 = 0;

    for x in 0..8 {
        for y in 0..8 {
            let piece = board[x][y];
            if piece.symbol != Symbol::Empty && !matches!(piece.symbol, Symbol::King) {
                if matches!(piece.symbol, Symbol::Pawn | Symbol::Rook | Symbol::Queen) {
                    return false;
                } else {
                    if piece.color == Color::White {
                        white_counts += 1;
                    } else {
                        black_counts += 1;
                    }
                }
            }
        }
    }
    // King + bishop / knight == Insufficint material!
    // King + bishop + bishop / knight == sufficient material! 
    // king + knight + knight = sufficient material! 
    // Essentially, we need to be able to give check on TWO straight squares! Bishop/Knight can
    // only check one square. So, alone they are unable to deliver checkmate.
    if white_counts < 2 && black_counts < 2 {
        return true;
    }

    false
}

// *** AI GENERATED *** 
// ============================================================================
// FAST “BEST MOVE” SEARCH STACK (high N/s)
// TT + Zobrist + PVS Negamax + Iterative Deepening
// ============================================================================

#[inline]
fn opponent(side: Color) -> Color {
    match side {
        Color::White => Color::Black,
        Color::Black => Color::White,
        Color::None => Color::None,
    }
}

// =========================
// TRANSPOSITION TABLE
// =========================

#[derive(Clone, Copy)]
pub enum TtFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TtEntry {
    pub key: u64,
    pub depth: u8,
    pub value: i64,
    pub flag: TtFlag,
    pub best: Move,
    pub best_valid: bool,
}

pub struct TranspositionTable {
    mask: usize,
    table: Vec<TtEntry>,
}

#[inline]
fn empty_move() -> Move {
    Move {
        current_square: (0, 0),
        destination_square: (0, 0),
        castle: false,
        promotion: Promotion::NoPromotion,
    }
}

impl TranspositionTable {
    pub fn new_pow2(entries_pow2: usize) -> Self {
        let size = 1usize << entries_pow2;
        let empty = TtEntry {
            key: 0,
            depth: 0,
            value: 0,
            flag: TtFlag::Exact,
            best: empty_move(),
            best_valid: false,
        };
        Self {
            mask: size - 1,
            table: vec![empty; size],
        }
    }

    #[inline]
    pub fn probe(&self, key: u64) -> Option<TtEntry> {
        let e = self.table[(key as usize) & self.mask];
        if e.key == key && e.key != 0 {
            Some(e)
        } else {
            None
        }
    }

    #[inline]
    pub fn store(&mut self, entry: TtEntry) {
        let idx = (entry.key as usize) & self.mask;
        let cur = self.table[idx];

        if cur.key == 0 || entry.depth >= cur.depth {
            self.table[idx] = entry;
        }
    }
}

// =========================
// ZOBRIST HASHING
// =========================

pub struct Zobrist {
    piece: [[u64; 24]; 64],
    side_to_move: u64,
}

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

impl Zobrist {
    pub fn new() -> Self {
        let mut seed = 0xC0FFEE_u64;
        let mut piece = [[0u64; 24]; 64];

        for sq in 0..64 {
            for k in 0..24 {
                seed = splitmix64(seed);
                piece[sq][k] = seed;
            }
        }

        seed = splitmix64(seed);
        let side_to_move = seed;

        Self { piece, side_to_move }
    }
}

#[inline]
fn piece_index(p: Piece) -> Option<usize> {
    if p.symbol == Symbol::Empty || p.color == Color::None {
        return None;
    }

    let s = match p.symbol {
        Symbol::Pawn => 0,
        Symbol::Knight => 1,
        Symbol::Bishop => 2,
        Symbol::Rook => 3,
        Symbol::Queen => 4,
        Symbol::King => 5,
        _ => return None,
    };

    let c = match p.color {
        Color::White => 0,
        Color::Black => 1,
        _ => return None,
    };

    let m = if p.has_moved { 1 } else { 0 };

    Some(((s * 2 + c) * 2 + m) as usize)
}

#[inline]
pub fn zobrist_hash(board: &[[Piece; 8]; 8], side: Color, z: &Zobrist) -> u64 {
    let mut h = 0u64;
    for r in 0..8 {
        for c in 0..8 {
            if let Some(pi) = piece_index(board[r][c]) {
                h ^= z.piece[r * 8 + c][pi];
            }
        }
    }
    if side == Color::Black {
        h ^= z.side_to_move;
    }
    h
}

#[inline]
pub fn hash_after_move(
    mut h: u64,
    board: &[[Piece; 8]; 8],
    mv: &Move,
    side: Color,
    z: &Zobrist,
) -> u64 {
    let (fr, fc) = mv.current_square;
    let (tr, tc) = mv.destination_square;

    let from = fr as usize * 8 + fc as usize;
    let to = tr as usize * 8 + tc as usize;

    let mut moving = board[fr as usize][fc as usize];
    let target = board[tr as usize][tc as usize];

    if let Some(pi) = piece_index(moving) {
        h ^= z.piece[from][pi];
    }

    if let Some(pi) = piece_index(target) {
        h ^= z.piece[to][pi];
    }

    if moving.symbol == Symbol::Pawn {
        match mv.promotion {
            Promotion::Queen => moving.symbol = Symbol::Queen,
            Promotion::Rook => moving.symbol = Symbol::Rook,
            Promotion::Bishop => moving.symbol = Symbol::Bishop,
            Promotion::Knight => moving.symbol = Symbol::Knight,
            Promotion::NoPromotion => {}
        }
    }

    moving.has_moved = true;

    if let Some(pi) = piece_index(moving) {
        h ^= z.piece[to][pi];
    }

    if mv.castle {
        let row = fr as usize;
        let rook_from = if tc > fc { 7 } else { 0 };
        let rook_to = if tc > fc { 5 } else { 3 };

        let rook = board[row][rook_from];
        if let Some(pi) = piece_index(rook) {
            h ^= z.piece[row * 8 + rook_from][pi];
        }

        let mut rook2 = rook;
        rook2.has_moved = true;
        if let Some(pi) = piece_index(rook2) {
            h ^= z.piece[row * 8 + rook_to][pi];
        }
    }

    h ^ z.side_to_move
}

// =========================
// SOFT MOVE ORDERING
// =========================

#[inline]
fn val(sym: Symbol) -> i32 {
    match sym {
        Symbol::Pawn => 100,
        Symbol::Knight => 320,
        Symbol::Bishop => 330,
        Symbol::Rook => 500,
        Symbol::Queen => 900,
        Symbol::King => 100_000,
        _ => 0,
    }
}

#[inline]
fn is_tactical(board: &[[Piece; 8]; 8], mv: &Move) -> bool {
    mv.castle
        || mv.promotion != Promotion::NoPromotion
        || board[mv.destination_square.0 as usize][mv.destination_square.1 as usize].symbol
            != Symbol::Empty
}

#[inline]
fn soft_score(board: &[[Piece; 8]; 8], mv: &Move) -> i32 {
    let from = mv.current_square;
    let to = mv.destination_square;

    let mover = board[from.0 as usize][from.1 as usize];
    let target = board[to.0 as usize][to.1 as usize];

    let mut s = 0;

    if mv.promotion != Promotion::NoPromotion {
        s += 20_000;
    }

    if target.symbol != Symbol::Empty {
        s += 10_000 + val(target.symbol) - val(mover.symbol) / 10;
    }

    if mv.castle {
        s += 2_000;
    }

    s
}

#[inline]
pub fn order_moves_soft_in_negamax(board: &[[Piece; 8]; 8], moves: &mut Vec<Move>) {
    let mut i = 0;
    for j in 0..moves.len() {
        if is_tactical(board, &moves[j]) {
            moves.swap(i, j);
            i += 1;
        }
    }
    moves[..i].sort_unstable_by(|a, b| soft_score(board, b).cmp(&soft_score(board, a)));
}

// =========================
// NEGAMAX + TT + PVS
// =========================

pub fn negamax_tt_pvs(
    node: &[[Piece; 8]; 8],
    depth: u8,
    mut alpha: i64,
    beta: i64,
    side: Color,
    hash: u64,
    z: &Zobrist,
    tt: &mut TranspositionTable,
) -> i64 {
    nodes_inc();

    let alpha_orig = alpha;

    if let Some(e) = tt.probe(hash) {
        if e.depth >= depth {
            match e.flag {
                TtFlag::Exact => return e.value,
                TtFlag::LowerBound if e.value >= beta => return e.value,
                TtFlag::UpperBound if e.value <= alpha => return e.value,
                _ => {}
            }
        }
    }

    if depth == 0 {
        return if side == Color::White {
            evaluate(node)
        } else {
            -evaluate(node)
        };
    }

    let mut moves = get_all_legal_moves_for_this_turn(node, side);

    if moves.is_empty() {
        return if is_king_in_check(node, side) {
            -(1_000_000_000 - depth as i64)
        } else {
            0
        };
    }

    if let Some(e) = tt.probe(hash) {
        if e.best_valid {
            if let Some(p) = moves.iter().position(|m| *m == e.best) {
                moves.swap(0, p);
            }
        }
    }

    order_moves_soft_in_negamax(node, &mut moves);

    let mut best = -100_000_000;
    let mut best_move = moves[0];
    let mut first = true;

    for mv in moves.iter() {
        let child_hash = hash_after_move(hash, node, mv, side, z);

        let mut tmp = *node;
        make_move(&mut tmp, mv);

        let score = if first {
            first = false;
            -negamax_tt_pvs(&tmp, depth - 1, -beta, -alpha, opponent(side), child_hash, z, tt)
        } else {
            let mut s =
                -negamax_tt_pvs(&tmp, depth - 1, -(alpha + 1), -alpha, opponent(side), child_hash, z, tt);
            if s > alpha && s < beta {
                s = -negamax_tt_pvs(&tmp, depth - 1, -beta, -alpha, opponent(side), child_hash, z, tt);
            }
            s
        };

        if score > best {
            best = score;
            best_move = *mv;
        }
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            break;
        }
    }

    let flag = if best <= alpha_orig {
        TtFlag::UpperBound
    } else if best >= beta {
        TtFlag::LowerBound
    } else {
        TtFlag::Exact
    };

    tt.store(TtEntry {
        key: hash,
        depth,
        value: best,
        flag,
        best: best_move,
        best_valid: true,
    });

    best
}

// =========================
// ROOT SEARCH (ITERATIVE)
// =========================

pub fn get_best_move_iterative_tt(
    node: &[[Piece; 8]; 8],
    depth: u8,
    side: Color,
) -> Move {
    let z = Zobrist::new();
    let mut tt = TranspositionTable::new_pow2(20);

    let root_hash = zobrist_hash(node, side, &z);
    let mut moves = get_all_legal_moves_for_this_turn(node, side);

    if moves.is_empty() {
        return empty_move();
    }

    order_moves_soft_in_negamax(node, &mut moves);

    let mut best_move = moves[0];

    for d in 1..=depth {
        let mut alpha = -100_000_000;
        let beta = 100_000_000;

        if let Some(e) = tt.probe(root_hash) {
            if e.best_valid {
                if let Some(p) = moves.iter().position(|m| *m == e.best) {
                    moves.swap(0, p);
                }
            }
        }

        for mv in moves.iter() {
            let child_hash = hash_after_move(root_hash, node, mv, side, &z);
            let mut tmp = *node;
            make_move(&mut tmp, mv);

            let score = -negamax_tt_pvs(
                &tmp,
                d.saturating_sub(1),
                -beta,
                -alpha,
                opponent(side),
                child_hash,
                &z,
                &mut tt,
            );

            if score > alpha {
                alpha = score;
                best_move = *mv;
            }
        }

        tt.store(TtEntry {
            key: root_hash,
            depth: d,
            value: alpha,
            flag: TtFlag::Exact,
            best: best_move,
            best_valid: true,
        });
    }

    best_move
}

// =========================
// FAST STATIC EVALUATION
// White perspective (+ = good for White)
// =========================

#[inline(always)]
fn piece_value(sym: Symbol) -> i64 {
    match sym {
        Symbol::Pawn   => 100,
        Symbol::Knight => 320,
        Symbol::Bishop => 330,
        Symbol::Rook   => 500,
        Symbol::Queen  => 900,
        Symbol::King   => 0,   // king safety handled elsewhere
        _ => 0,
    }
}

// Simple piece-square tables (from White's perspective)
// Indexed as [row][col], row 0 = White back rank
const PAWN_PST: [[i64; 8]; 8] = [
    [  0,  0,  0,  0,  0,  0,  0,  0 ],
    [ 50, 50, 50, 50, 50, 50, 50, 50 ],
    [ 10, 10, 20, 30, 30, 20, 10, 10 ],
    [  5,  5, 10, 25, 25, 10,  5,  5 ],
    [  0,  0,  0, 20, 20,  0,  0,  0 ],
    [  5, -5,-10,  0,  0,-10, -5,  5 ],
    [  5, 10, 10,-20,-20, 10, 10,  5 ],
    [  0,  0,  0,  0,  0,  0,  0,  0 ],
];

const KNIGHT_PST: [[i64; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

const BISHOP_PST: [[i64; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

const ROOK_PST: [[i64; 8]; 8] = [
    [  0,  0,  0,  5,  5,  0,  0,  0 ],
    [ -5,  0,  0,  0,  0,  0,  0, -5 ],
    [ -5,  0,  0,  0,  0,  0,  0, -5 ],
    [ -5,  0,  0,  0,  0,  0,  0, -5 ],
    [ -5,  0,  0,  0,  0,  0,  0, -5 ],
    [ -5,  0,  0,  0,  0,  0,  0, -5 ],
    [  5, 10, 10, 10, 10, 10, 10,  5 ],
    [  0,  0,  0,  0,  0,  0,  0,  0 ],
];

const QUEEN_PST: [[i64; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [ -5,  0,  5,  5,  5,  5,  0, -5],
    [  0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

#[inline(always)]
fn pst_bonus(sym: Symbol, row: usize, col: usize) -> i64 {
    match sym {
        Symbol::Pawn   => PAWN_PST[row][col],
        Symbol::Knight => KNIGHT_PST[row][col],
        Symbol::Bishop => BISHOP_PST[row][col],
        Symbol::Rook   => ROOK_PST[row][col],
        Symbol::Queen  => QUEEN_PST[row][col],
        _ => 0,
    }
}

/// Main evaluation entry point
/// White perspective: positive = good for White
pub fn evaluate(board: &[[Piece; 8]; 8]) -> i64 {
    let mut score: i64 = 0;

    for r in 0..8 {
        for c in 0..8 {
            let p = board[r][c];
            if p.symbol == Symbol::Empty || p.color == Color::None {
                continue;
            }

            let base = piece_value(p.symbol);
            let pst = if p.color == Color::White {
                pst_bonus(p.symbol, r, c)
            } else {
                // mirror vertically for Black
                pst_bonus(p.symbol, 7 - r, c)
            };

            let piece_score = base + pst;

            if p.color == Color::White {
                score += piece_score;
            } else {
                score -= piece_score;
            }
        }
    }

    score
}


// *** AI GENERATED *** 



// Communication
//pub fn move_to_universal_chess_interface(move_: &Move) -> String {}

pub fn universal_chess_interface_to_move(board: &[[Piece;8];8], uci: String) -> Result<Move, &'static str> {
    
    let chars: Vec<char> = uci.chars().collect();
    let mut temp_move: Move = Move {current_square: (0, 0), destination_square: (0, 0), castle: false, promotion: Promotion::NoPromotion};
    
    if matches!(chars[0], 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') {
        temp_move.current_square.1 = chars[0] as u8 - 97;
    } else {
        return Err("Incorrect FROM Column! -> a-h");
    }

    if matches!(chars[1], '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') {
        temp_move.current_square.0 = 7 - (chars[1] as u8 - 49);
    } else {
        return Err("Incorrect FROM Row! -> 1-8");
    }

    if matches!(chars[2], 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') {
        temp_move.destination_square.1 = chars[2] as u8 - 97;
    } else {
        return Err("Incorrect TO Column! -> a-h");
    }

    if matches!(chars[3], '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') {
        temp_move.destination_square.0 = 7 - (chars[3] as u8 - 49);
    } else {
        return Err("Incorrect TO Row! -> 1-8");
    }

    if board[temp_move.current_square.0 as usize][temp_move.current_square.1 as usize].symbol == Symbol::King {
        if (temp_move.current_square.1 as i8 - temp_move.destination_square.1 as i8).abs() == 2 {
            temp_move.castle = true;
        }
    }

    if board[temp_move.current_square.0 as usize][temp_move.current_square.1 as usize].symbol == Symbol::Pawn {
        if temp_move.destination_square.0 == 7 || temp_move.destination_square.0 == 0 {
            if chars.len() > 4 {
                temp_move.promotion = match chars[4] {
                   'q' => Promotion::Queen,
                    'r' => Promotion::Rook,
                    'b' => Promotion::Bishop,
                    'n' => Promotion::Knight,
                    _ => return Err("Incorrect PROMOTION! -> q/r/b/n"),
                };
            } else {
                return Err("Ensure Proper Promotion Notation -> a1a2q");
            }
        }
    }
    



    Ok(temp_move)
}
