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


    let _ = pieces_logic::is_king_in_check(&board, true);


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
    fn is_king_in_check_knights() {
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

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 7), false);
        pieces_logic::place_knight_on_board(&mut board, &(5, 6), true);

        assert_eq!(true, pieces_logic::is_king_in_check(&board, false));

    }

    #[test]
    fn is_king_in_check_straights_up_down() {
        let mut board = chess_board::create_empty_board();
        
        // Rook
        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(1, 4), true); // Enemy Rook

        assert_eq!(true, pieces_logic::is_king_in_check(&board, false));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), true);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), false);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(1, 4), false); // Friendly Rook
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));

        // Queen 
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(1, 4), true); // Enemy Queen

        assert_eq!(true, pieces_logic::is_king_in_check(&board, false));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), true);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        pieces_logic::place_pawn_on_board(&mut board, &(2, 4), false);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(1, 4), false); // Friendly Queen
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));

        
        // Test Bishop (will not produce check)

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_bishop_on_board(&mut board, &(1, 4), true); // Enemy Bishop

        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
    }

    #[test]
    fn is_king_in_check_left_right() {
        let mut board = chess_board::create_empty_board();
            
        // Rook
        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(4, 1), true); // Enemy Rook

        assert_eq!(true, pieces_logic::is_king_in_check(&board, false));
        
        // PAWN BLOCK
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), true);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
       
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), false);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_rook_on_board(&mut board, &(4, 1), false); // Friendly Rook
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));

        // Queen 
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(4, 1), true); // Enemy Queen

        assert_eq!(true, pieces_logic::is_king_in_check(&board, false));
        
        // PAWN BLOCK
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), true);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), false);
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
        
        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_queen_on_board(&mut board, &(4, 1), false); // Friendly Queen
        
        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));

        
        // Test Bishop (will not produce check)

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(4, 4), false); // Friendly King
        pieces_logic::place_bishop_on_board(&mut board, &(4, 1), true); // Enemy Bishop

        assert_eq!(false, pieces_logic::is_king_in_check(&board, false));

    }
    

    #[test]
    fn is_king_in_check_diagonals() {
        for tup in [(0, 0), (6, 6), (7, 1), (1, 7)] {
        
            for x in ['B', 'Q'] {
            
                let mut board = chess_board::create_empty_board();

                pieces_logic::place_king_on_board(&mut board, &(4, 4), false);
                
                pieces_logic::place_pawn_on_board(&mut board, &tup, true);
                board[tup.0 as usize][tup.1 as usize].symbol = x; // Little workaround for testing

                assert_eq!(true, pieces_logic::is_king_in_check(&board, false));

                for p in ['R', 'N'] {
                    
                    let mut row: u8 = (tup.0 + 4) >> 1;
                    let col: u8 = (tup.1 + 4) >> 1;
                    

                    if tup == (7, 1) || tup == (1, 7) {
                        row += 1;
                    }

                    pieces_logic::place_pawn_on_board(&mut board, &(row, col), true);
                    board[row as usize][col as usize].symbol = p;
                    assert_eq!(false, pieces_logic::is_king_in_check(&board, false));
                    
                }
            }
        }
    }
    
    #[test]
    fn is_king_in_check_pawns() {
        
        let mut board = chess_board::create_empty_board();

        for color in [true, false] {
            pieces_logic::place_king_on_board(&mut board, &(4, 4), color);
            for pos in [(3,3, color), (3, 4, false), (3, 5, color),
                        (4,3, false), (4, 5, false),
                        (5,3, !color), (5, 4, false), (5, 5, !color)] {
                pieces_logic::place_pawn_on_board(&mut board, &(pos.0, pos.1), !color);
                assert_eq!(pos.2, pieces_logic::is_king_in_check(&board, color));
                board[pos.0 as usize][pos.1 as usize] = pieces_logic::create_empty_piece(&(pos.0, pos.1));
            }
        }
    }

    #[test]
    fn is_piece_pinned_pawn() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_pawn_on_board(&mut board, &(6, 5), true);
        pieces_logic::place_bishop_on_board(&mut board, &(4, 7), false);
        
        let pawn_move: pieces_logic::Move = pieces_logic::Move { current_square: (6, 5), destination_square: (5, 5) };

        assert_eq!(true, pieces_logic::is_piece_pinned(&board, &pawn_move));
        

    }


    #[test]
    fn get_legal_moves_for_pawn() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_pawn_on_board(&mut board, &(6, 4), true);

        let mut legal_moves: Vec<pieces_logic::Move> = vec![];

        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (5, 4)}); 
        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (4, 4)});

        let mut gen_legal_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_pawn(&board, &(6, 4));
        
        legal_moves.sort(); 
        gen_legal_moves.sort();

        assert_eq!(legal_moves, gen_legal_moves);


    }

}


















