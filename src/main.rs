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
    fn check_legality_of_move() {
        assert!(false);
    }
    
    #[test]
    fn make_move() {
        // Legal Moves
        let move_: pieces_logic::Move = pieces_logic::Move { current_square: (6, 0), destination_square: (5, 0)
        };
        assert!(false);
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
    fn is_piece_pinned_diagonals_no_blocks() {
        let mut board: [[pieces_logic::Piece;8];8] = chess_board::initialize_chess_board();

        
        let king_square: (u8, u8) = pieces_logic::get_square_of_king(&board, true);
        let piece_square: (u8, u8) = (6, 5);
        
        // No other pieces
        let mut is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);


        // Introduce pinner piece to board
        let pinner_piece = pieces_logic::Piece { 
            color: false, 
            symbol: 'q', 
            has_moved: true, 
            value: 9, 
            is_empty: false, 
            current_square: (4, 7) };
        
        // Standard pin
        board[4][7] = pinner_piece;
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(true, is_pinned);

        board[4][7].symbol = 'b';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(true, is_pinned);


        board[4][7].symbol = 'r';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);

        board[4][7].symbol = 'p';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);
        
        board[4][7].color = true; // White 
        board[4][7].symbol = 'B'; // Bishop
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);

        board[4][7].symbol = 'R'; // White Rook
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);



        // Some tests with gaps 

        board[6][5] = pieces_logic::create_empty_piece(&(6, 5));
        board[6][6] = pieces_logic::create_empty_piece(&(6, 6));
        board[5][6] = pieces_logic::Piece { color: true,
                                            symbol: 'P',
                                            has_moved: false,
                                            value: 1,
                                            is_empty: false,
                                            current_square: (5, 6) };
        



        board[4][7] = pinner_piece;
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(true, is_pinned);

        board[4][7].symbol = 'b';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(true, is_pinned);

        board[4][7].symbol = 'r';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);

        board[4][7].symbol = 'p';
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);
        
        board[4][7].color = true; // White 
        board[4][7].symbol = 'B'; // Bishop
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);

        board[4][7].symbol = 'R'; // White Rook
        is_pinned = pieces_logic::is_piece_pinned(&board, &king_square, &piece_square);
        assert_eq!(false, is_pinned);

    }
    
    #[test]
    fn is_piece_pinned_diagonals_close_blocker_piece() {

    
        
        let close_blocker_piece = pieces_logic::Piece {
            color: false, 
            symbol: 'q', 
            has_moved: true, 
            value: 9, 
            is_empty: false, 
            current_square: (4, 7) };

        assert!(false);        
    }

    #[test]
    fn is_piece_pinned_diagonals_far_blocker_piece() {


        let far_blocker_piece = pieces_logic::Piece { 
            color: false, 
            symbol: 'q', 
            has_moved: true, 
            value: 9, 
            is_empty: false, 
            current_square: (4, 7) };
    
        assert!(false);
    }


}


















