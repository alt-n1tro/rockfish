mod chess_board;
mod pieces_logic;





fn main() {
    
    let mut board = chess_board::initialize_chess_board();


    chess_board::print_chess_board(&board);
    

    let move_: pieces_logic::Move = pieces_logic::Move {
        current_square: (7, 4),
        destination_square: (4, 4),
    };
    
    //pieces_logic::make_move(&mut board, &move_);


    board[6][5] = pieces_logic::create_empty_piece(&(6, 5));
    board[6][6] = pieces_logic::create_empty_piece(&(6, 6));
    board[5][6] = pieces_logic::Piece { color: true,
                                            symbol: 'P',
                                            has_moved: false,
                                            value: 1,
                                            is_empty: false,
                                            current_square: (5, 6) };
    board[4][7] = pieces_logic::Piece { 
            color: false, 
            symbol: 'q', 
            has_moved: true, 
            value: 9, 
            is_empty: false, 
            current_square: (4, 7) };    
    
    
    chess_board::print_chess_board(&board);
    
    let x = pieces_logic::get_square_of_king(&board, true);
    
    println!("{}, {}", x.0, x.1);

}







#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn create_empty_piece() {
        let empty_square = pieces_logic::Piece {
            color: false,
            symbol: ' ',
            has_moved: false,
            value: 0, 
            is_empty: true,
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

        let move_: pieces_logic::Move = pieces_logic::Move { current_square: (6, 0), destination_square: (4, 0)
        };
        pieces_logic::make_move(&mut board, &move_);
        
        let mut board_2 = chess_board::initialize_chess_board();

        board_2[6][0] = pieces_logic::create_empty_piece(&(6, 0));
        pieces_logic::place_pawn_on_board(&mut board_2, &(4, 0), true);
        board_2[4][0].has_moved = true;

        chess_board::print_chess_board(&board);
        chess_board::print_chess_board(&board_2);

        assert_eq!(board, board_2);

    }


    #[test]
    fn get_square_of_king() {
        let mut board = chess_board::initialize_chess_board();
        
        let init_white_king: (u8, u8) = (7, 4);
        let init_black_king: (u8, u8) = (0, 4);
        let white_king_pos_: (u8, u8) = (3, 3);
        let black_king_pos_: (u8, u8) = (5, 7);

        assert_eq!(init_white_king, pieces_logic::get_square_of_king(&board, true));
        assert_eq!(init_black_king, pieces_logic::get_square_of_king(&board, false));
        
        pieces_logic::make_move(&mut board, &pieces_logic::Move {current_square: init_white_king, destination_square: white_king_pos_});
        pieces_logic::make_move(&mut board, &pieces_logic::Move {current_square: init_black_king, destination_square: black_king_pos_});
        
        assert_eq!(white_king_pos_, pieces_logic::get_square_of_king(&board, true));
        assert_eq!(black_king_pos_, pieces_logic::get_square_of_king(&board, false));


    }
    
    #[test]
    fn is_king_in_check_knight() {
        let mut board = chess_board::create_empty_board();
        let knight_moves: [(isize, isize); 8] = [(-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right


        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); 
        pieces_logic::place_knight_on_board(&mut board, &(2, 3), true);
        assert!(pieces_logic::is_king_in_check(&board, false));
        board[2][3] = pieces_logic::create_empty_piece(&(2, 3));


        for x in 1..knight_moves.len() {
            let row: u8 = (4 + knight_moves[x].0) as u8;
            let col: u8 = (4 + knight_moves[x].1) as u8;
            let square: (u8, u8) = (row, col);

            pieces_logic::place_knight_on_board(&mut board, &square, true);
            assert!(pieces_logic::is_king_in_check(&board, false));
            board[row as usize][col as usize] = pieces_logic::create_empty_piece(&square);
        }
    }



}


















