use core::borrow;
use std::process::Output;
use std::usize;

use crate::chess_board::{self, print_chess_board};
use crate::pieces_logic;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub color: bool,
    pub symbol: char,
    pub has_moved: bool,
    pub value: u32,
    pub is_empty: bool,
    pub current_square: (u8, u8),
}


// This should only ever be created via a function, that checks if the move is actually legal.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub current_square: (u8, u8),
    pub destination_square: (u8, u8),
}


pub fn create_empty_piece(square: &(u8, u8)) -> Piece {
    Piece { color: false, 
        symbol: ' ',
        has_moved: false,
        value: 0,
        is_empty: true,
        current_square: *square }
}

pub fn place_pawn_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'p',
            true => 'P',
        },
        has_moved: false,
        value: 1,
        is_empty: false,
        current_square: *square
    };

}

pub fn place_bishop_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'b',
            true => 'B',
        },
        has_moved: false,
        value: 3,
        is_empty: false,
        current_square: *square
    };
    
}

pub fn place_knight_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'n',
            true => 'N',
        },
        has_moved: false,
        value: 3,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_rook_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'r',
            true => 'R',
        },
        has_moved: false,
        value: 5,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_queen_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'q',
            true => 'Q',
        },
        has_moved: false,
        value: 9,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_king_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: match color {
            false => 'k',
            true => 'K',
        },
        has_moved: false,
        value: 10000,
        is_empty: false,
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
pub fn get_square_of_king(board: &[[Piece; 8]; 8], color: bool) -> (u8, u8) {
    if color {
        if board[7][4].symbol == 'K' {
            return (7 as u8, 4 as u8);
        }
        
        for x in (0..8).rev() {
            for y in (0..8).rev() {
                if board[x][y].symbol == 'K' {
                    return (x as u8, y as u8);
                }
            }
        }
        unreachable!("No White King was found...\nAt least 2 Kings MUST exist at all times!");
    } else {
        if board[0][4].symbol == 'k' {
            return (0 as u8, 4 as u8);
        }
        
        for x in 0..8 {
            for y in 0..8 {
                if board[x][y].symbol == 'k' {
                    return (x as u8, y as u8);
                }
            }
        }
        unreachable!("No Black King was found...\nAt least 2 Kings MUST exist at all times!");
    }
}



// This function looks outward from the king -- Not from the enemy pieces!
pub fn is_king_in_check(board: &[[Piece;8];8], color: bool) -> bool {
    
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
            if  (square.color != king.color) && matches!(square.symbol, 'n' | 'N') {
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

            if !square.is_empty {
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, 'r' | 'R' | 'q' | 'Q') {
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

            if !square.is_empty {
                
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, 'b' | 'B' | 'q' | 'Q') {
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
    if king.color {
        for pawn in black_pawns {
            if pawn.0 >= 0 && pawn.1 < 8 && pawn.1 >= 0{
                if board[pawn.0 as usize][pawn.1 as usize].symbol == 'p' {
                    return true;
                }
            } 
        }
    } else {
        for pawn in white_pawns {
            if pawn.0 < 8 && pawn.1 < 8 && pawn.1 >= 0{
                if board[pawn.0 as usize][pawn.1 as usize].symbol == 'P' {
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

    let adder: i8 = if piece_to_move.color {
        -1
    } else {
        1
    };
    
    let new_row = square.0 as i8 + adder;

    if new_row >= 0 && new_row < 8 {
        
        if board[new_row as usize][square.1 as usize].is_empty {
            
            let up_move: Move = Move {current_square: *square, destination_square: (new_row as u8, square.1)};
            
            if !is_piece_pinned(&board, &up_move) {
            
                output.push(up_move);
                
                if (new_row + adder) >= 0 && (new_row + adder) < 8 {
                
                    if board[(new_row + adder) as usize][square.1 as usize].is_empty {
                    
                        let up_up_move: Move = Move { current_square: *square, destination_square: ((new_row + adder) as u8, square.1) };
                        
                        output.push(up_up_move); 
                    }
                }
            }
            
            
            
        }
        
        if square.1 < 7 {
            let board_square = board[new_row as usize][(square.1 + 1) as usize];
            if board_square.color != piece_to_move.color && !board_square.is_empty {
                let left_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 + 1)};
                if !is_piece_pinned(&board, &left_move) {
                    output.push(left_move);
                }
            }
        }
        if square.1 >= 1 {
            let board_square = board[new_row as usize][(square.1 - 1) as usize];
            if board_square.color != piece_to_move.color && !board_square.is_empty {
                let right_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 - 1)};
                if !is_piece_pinned(&board, &right_move) {
                    output.push(right_move);
                }
            }
        }
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
            if to_sqr.is_empty || (to_sqr.color != board[square.0 as usize][square.1 as usize].color) {
                let knight_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8)};
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
            
            let proposed_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8) };

            if !is_piece_pinned(&board, &proposed_move) {

                if destination_square.is_empty {  
                    output.push(proposed_move);
                 
                } else {
                    if destination_square.color != piece_color {
                        output.push(proposed_move);
                    }
                    break 'inner_while;
                }
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


pub fn get_legal_moves_for_king(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let king_moves: [(i8, i8); 8] = [(1, 1), (1, 0), (1, -1),
                                     (0, 1), (0, -1),
                                     (-1, 1), (-1, 0), (-1, -1)];
    
    let mut output: Vec<Move> = vec![];

    for king_move in king_moves {
        let row: i8 = square.0 as i8 + king_move.0;
        let col: i8 = square.1 as i8 + king_move.1;
        
        if row >= 0 && row < 8 && col >= 0 && col < 8 {
            let board_square = board[row as usize][col as usize];
            let piece_square = board[square.0 as usize][square.1 as usize];

            let k_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8) };
            
            if board_square.is_empty {
                if !is_piece_pinned(&board, &k_move) {
                    output.push(k_move);
                }
            } else {
                if board_square.color != piece_square.color && !is_piece_pinned(&board, &k_move) {
                    output.push(k_move);
                }
            }
        }
    }
    
    output

}



pub fn make_move(board: &mut [[Piece; 8]; 8], move_: &Move) {
    
    let cur_sq = move_.current_square;
    let des_sq = move_.destination_square;

    board[des_sq.0 as usize][des_sq.1 as usize] = board[cur_sq.0 as usize][cur_sq.1 as usize];
    board[cur_sq.0 as usize][cur_sq.1 as usize] = create_empty_piece(&cur_sq);
    
    board[des_sq.0 as usize][des_sq.1 as usize].current_square = des_sq;
    board[des_sq.0 as usize][des_sq.1 as usize].has_moved = true;
}



// For bot moves
//pub fn find_all_legal_moves_for_a_piece(piece: &Piece) -> Vec<Move> {}







// Communication
//pub fn move_to_universal_chess_interface(move_: &Move) -> String {}
//pub fn universal_chess_interface_to_move(uci: &String) -> Move {}
//pub fn get_universal_chess_interface_from_user_input() -> String {}


