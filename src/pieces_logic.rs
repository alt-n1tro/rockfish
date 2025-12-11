#[derive(Clone, Copy)]
pub struct Piece {
    pub color: bool,
    pub symbol: char,
    pub has_moved: bool,
    pub value: u32,
    pub is_empty: bool,
}


// This should call a couple helper functions based on the symbol of the piece (ie if it's a bishop
// vs knight)
fn legal_moves(current_board: &[[u8;8];8], square: u8) -> Vec<u8> {
    let found_legal_moves: Vec<u8> = vec![];
    let cur_square = get_2d_location_of_piece(square);




    found_legal_moves
}

pub fn get_2d_location_of_piece(square: u8) -> (u8, u8) {
    (square / 8, square % 8)
}
