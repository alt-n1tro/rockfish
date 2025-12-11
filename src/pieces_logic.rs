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


// Moves 
pub fn check_legality_of_move(move_: &Move) -> bool {true}


pub fn make_move(board: &mut [[Piece; 8]; 8], move_: &Move) {
    
    if check_legality_of_move(&move_) {

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


