mod chess_board;
mod pieces_logic;





fn main() {
    
    let mut board = chess_board::initialize_chess_board();


    chess_board::print_chess_board(&board);
    

    let move_: pieces_logic::Move = pieces_logic::Move {
        current_square: (7, 4),
        destination_square: (4, 4),
    };
    
    pieces_logic::make_move(&mut board, &move_);
    
    chess_board::print_chess_board(&board);
    
    let x = pieces_logic::get_square_of_king(&board, true);
    
    if x.is_ok() {
        println!("{:?}", x.unwrap());
    }

}
