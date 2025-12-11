use std::error::Error;

#[derive(Clone, Copy)]
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


pub fn create_empty_piece(square: (u8, u8)) -> Piece {
    Piece { color: false, 
        symbol: ' ',
        has_moved: false,
        value: 0,
        is_empty: true,
        current_square: square }
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
pub fn get_square_of_king(board: &[[Piece; 8]; 8], color: bool) -> Result<(u8, u8), &str> {
    if color {
        if board[7][4].symbol == 'K' {
            return Ok((7 as u8, 4 as u8));
        }
        
        for x in (0..8).rev() {
            for y in (0..8).rev() {
                if board[x][y].symbol == 'K' {
                    return Ok((x as u8, y as u8));
                }
            }
        }
        Err("No White King was found...")
    } else {
        if board[0][4].symbol == 'k' {
            return Ok((0 as u8, 4 as u8));
        }
        
        for x in 0..8 {
            for y in 0..8 {
                if board[x][y].symbol == 'k' {
                    return Ok((x as u8, y as u8));
                }
            }
        }
        Err("No Black King was found...")
    }
}



//pub fn does_xray_exist(board: &[[Piece;8];8], king_square: &(u8, u8), piece_square: &(u8, u8)) -> bool {}





// Piece specific move functions
//pub fn get_legal_moves_for_pawn(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_knight(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_bishop(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_rook(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_queen(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}
//pub fn get_legal_moves_for_king(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {}







// Moves 
pub fn check_legality_of_move(board: &[[Piece;8];8], move_: &Move) -> bool {true}


pub fn make_move(board: &mut [[Piece; 8]; 8], move_: &Move) {
    
    if check_legality_of_move(&board, &move_) {

        let cur_sq = move_.current_square;
        let des_sq = move_.destination_square;

        board[des_sq.0 as usize][des_sq.1 as usize] = board[cur_sq.0 as usize][cur_sq.1 as usize];
        board[cur_sq.0 as usize][cur_sq.1 as usize] = create_empty_piece(cur_sq);

    } else {
        println!("You are trying to make an Illegal move!\nAborting Program...");
    }

}



// For bot moves
//pub fn find_all_legal_moves_for_a_piece(piece: &Piece) -> Vec<Move> {}







// Communication
//pub fn move_to_universal_chess_interface(move_: &Move) -> String {}
//pub fn universal_chess_interface_to_move(uci: &String) -> Move {}
//pub fn get_universal_chess_interface_from_user_input() -> String {}


