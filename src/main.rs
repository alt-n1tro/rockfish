mod chess_board;
mod pieces_logic;





fn main() {
    
    let board = chess_board::initialize_chess_board();


    chess_board::print_chess_board(&board);
    
    for x in [0, 13, 8, 34, 63, 10] {
        let square = pieces_logic::get_2d_location_of_piece(x);
        println!("{}, {}",square.0, square.1);
    }
}
