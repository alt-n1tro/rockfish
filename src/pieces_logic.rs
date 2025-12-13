#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: bool,
    pub symbol: char,
    pub has_moved: bool,
    pub value: u32,
    pub is_empty: bool,
    pub current_square: (u8, u8),
}

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


    // Check knights
    
    let knight_moves: [(isize, isize); 8] = [(-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right

    
    for x in 0..knight_moves.len() {
        let n_move = knight_moves[x];

        let row = k_s.0 as isize + n_move.0;
        let col = k_s.1 as isize + n_move.1;
        if (row > 0 && row < 8) && (col > 0 && col < 8) {
            let square = board[row as usize][col as usize];
            if  (square.color != king.color) && 
                (square.symbol == 'n' || square.symbol == 'N') {
                return true;    
            }
        }  
    }
    


    return false;
}






//pub fn is_piece_pinned(board: &[[Piece;8];8], king_square: &(u8, u8), piece_square: &(u8, u8)) -> bool {false}



// Moves 

// Piece specific move functions
//pub fn get_legal_moves_for_pawn(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_knight(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_bishop(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_rook(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_queen(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_king(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}





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


