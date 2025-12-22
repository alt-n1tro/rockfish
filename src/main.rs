mod chess_board;
mod pieces_logic;

use rand::seq::IndexedRandom;

fn main() {
    
    let mut board = chess_board::initialize_chess_board();
    
    let mut buffer_str = String::new();
 
    'outer_loop: loop {

        chess_board::print_chess_board(&board);

        let all_legal_moves_white = pieces_logic::get_all_legal_moves_for_this_turn(&board, true);
        
        if all_legal_moves_white.len() == 0 {
            if pieces_logic::is_checkmate(&board, true) {
                println!("\n************\n\nBlack Won!\n\n************")
            } else {
                println!("\n************\n\nStalemate!\n\n************")
            }
           break 'outer_loop; 
        }
        
        'inner_loop: loop {
            println!("UCI Moves (i.e. a2a4)\nMake Move: ");
                
            buffer_str.clear();
            let _ = std::io::stdin().read_line(&mut buffer_str);
        
            let user_input = pieces_logic::universal_chess_interface_to_move(buffer_str.clone());

            if user_input.is_ok() {
                if all_legal_moves_white.contains(user_input.as_ref().unwrap()) {
                    pieces_logic::make_move(&mut board, &user_input.as_ref().unwrap()); 
                    break 'inner_loop;
                }
                println!("Not a legal move...");
            } else {
                println!("{:?}", user_input.err());
            }
        }

        let all_legal_moves_black = pieces_logic::get_all_legal_moves_for_this_turn(&board, false);

        if all_legal_moves_black.len() == 0 {
            if pieces_logic::is_checkmate(&board, false) {
                println!("\n************\n\nWhite Won!\n\n************")
            } else {
                println!("\n************\n\nStalemate!\n\n************")
            }
           break 'outer_loop; 
        }
        
        let black_move = all_legal_moves_black.choose(&mut rand::rng()).unwrap();
        
        pieces_logic::make_move(&mut board, black_move);




    }



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
        pieces_logic::place_pawn_on_board(&mut board, &(5, 2), true);
        board[5][2].has_moved = true;

        let mut legal_moves: Vec<pieces_logic::Move> = vec![];

        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (5, 4)}); 
        legal_moves.push(pieces_logic::Move {current_square: (6, 4), destination_square: (4, 4)});

        let mut gen_legal_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_pawn(&board, &(6, 4));
        
        legal_moves.sort(); 
        gen_legal_moves.sort();

        assert_eq!(legal_moves, gen_legal_moves);
        
        legal_moves = vec![];
        legal_moves.push(pieces_logic::Move {current_square: (5, 2), destination_square: (4, 2)});
        
        gen_legal_moves = pieces_logic::get_legal_moves_for_pawn(&board, &(5, 2));

        assert_eq!(legal_moves, gen_legal_moves);


    }


    #[test]
    fn get_legal_moves_for_knight() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_knight_on_board(&mut board, &(5, 5), true);

        pieces_logic:: place_rook_on_board(&mut board, &(3, 6), false);
        pieces_logic::place_rook_on_board(&mut board, &(4, 7), true);
        chess_board::print_chess_board(&board);
        
        let mut exp_knight_moves: Vec<pieces_logic::Move> = vec![];

        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (3, 6) });
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (3, 4) });
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (4, 3) });
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (6, 3) });
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (7, 6) });
        exp_knight_moves.push(pieces_logic::Move { current_square: (5, 5), destination_square: (6, 7) });
        
        let mut gen_knight_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_knight(&board, &(5, 5));

        exp_knight_moves.sort();
        gen_knight_moves.sort();

        assert_eq!(exp_knight_moves, gen_knight_moves);
    
        // Pin the horse
        board = chess_board::create_empty_board();
         
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_knight_on_board(&mut board, &(6, 4), true);

        pieces_logic::place_rook_on_board(&mut board, &(0, 4), false);

        // No moves -- since pinned.
        exp_knight_moves = vec![];
        gen_knight_moves = pieces_logic::get_legal_moves_for_knight(&board, &(6, 4));

        assert_eq!(exp_knight_moves, gen_knight_moves);

    }

    #[test]
    fn get_legal_moves_for_bishop() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_bishop_on_board(&mut board, &(0, 0), true);

        let mut bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));
        let mut exp_bishop_moves: Vec<pieces_logic::Move> = vec![];

        for x in 1..8 {
            exp_bishop_moves.push(pieces_logic::Move {current_square: (0, 0), destination_square: (x as u8, x as u8)});
        }

        bishop_moves.sort();
        exp_bishop_moves.sort();

        assert_eq!(&bishop_moves, &exp_bishop_moves);

        pieces_logic::place_rook_on_board(&mut board, &(7, 7), false);
        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));

        assert_eq!(bishop_moves, [pieces_logic::Move {current_square: (0, 0), destination_square: (7, 7)}]);
        
        exp_bishop_moves.pop();

        pieces_logic::place_rook_on_board(&mut board, &(7, 7), true);
        
        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(0, 0));
        bishop_moves.sort();

        assert_eq!(exp_bishop_moves, bishop_moves);
        
        // --------CLEAR--------------
        
        exp_bishop_moves = vec![];

        board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_bishop_on_board(&mut board, &(4, 4), true);

        
        // top left
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 3)});
        pieces_logic::place_rook_on_board(&mut board, &(3, 3), false);

        // top right
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 5)});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (2, 6)});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (1, 7)});
        
        // bottom right 
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 5)});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (6, 6)});
        pieces_logic::place_rook_on_board(&mut board, &(7, 7), true);

        // bottom left 
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 3)});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (6, 2)});
        exp_bishop_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (7, 1)});
        pieces_logic::place_knight_on_board(&mut board, &(7, 1), false);

        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(4, 4));
        
        bishop_moves.sort();
        exp_bishop_moves.sort();
        
        assert_eq!(bishop_moves, exp_bishop_moves);


        board = chess_board::create_empty_board();
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_knight_on_board(&mut board, &(5, 3), false);
        pieces_logic::place_bishop_on_board(&mut board, &(1, 7), true);

        bishop_moves = pieces_logic::get_legal_moves_for_bishop(&board, &(1, 7));

        exp_bishop_moves = vec![];
        exp_bishop_moves.push(pieces_logic::Move {current_square: (1, 7), destination_square: (5, 3)});
        
        chess_board::print_chess_board(&board);

        assert_eq!(bishop_moves, exp_bishop_moves);


    }
    

    #[test]
    fn get_legal_moves_for_rook() {
        let mut board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_rook_on_board(&mut board, &(4, 5), true);
        
        pieces_logic::place_rook_on_board(&mut board, &(7, 0), false);

        let mut rook_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));
        let mut expected_rook_moves: Vec<pieces_logic::Move> = vec![];

        assert_eq!(rook_moves, expected_rook_moves);
        
        pieces_logic::make_square_empty(&mut board, &(7, 0));

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5)});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 5)});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7)});
        

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 2)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 1)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 0)});

        rook_moves = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));
        
        rook_moves.sort();
        expected_rook_moves.sort();

        assert_eq!(rook_moves, expected_rook_moves);
        
        expected_rook_moves = vec![];


        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5)});

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5)});
        pieces_logic::place_knight_on_board(&mut board, &(1, 5), false);


        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7)});
        

        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4)});
        expected_rook_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3)});
        pieces_logic::place_pawn_on_board(&mut board, &(4, 2), true);

        rook_moves = pieces_logic::get_legal_moves_for_rook(&board, &(4, 5));

        rook_moves.sort();
        expected_rook_moves.sort();

        assert_eq!(rook_moves, expected_rook_moves);

    }

    #[test]
    fn get_legal_moves_for_queen() {

        let mut board = chess_board::create_empty_board();

        let mut expected_queen_moves: Vec<pieces_logic::Move> = vec![];
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_queen_on_board(&mut board, &(4, 5), true);
        
        // Rook rays
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 5)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 5)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 5)});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 5)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 5)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 5)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 5)});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 6)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 7)});
        

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 4)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 3)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 2)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 1)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (4, 0)});

        // Bishop rays

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 6)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 7)});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 4)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 3)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (1, 2)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (0, 1)});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (3, 6)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (2, 7)});

        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (5, 4)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (6, 3)});
        expected_queen_moves.push(pieces_logic::Move {current_square: (4, 5), destination_square: (7, 2)});


        let mut queen_moves: Vec<pieces_logic::Move> = pieces_logic::get_legal_moves_for_queen(&board, &(4, 5));

        queen_moves.sort();
        expected_queen_moves.sort();
        
        assert_eq!(queen_moves, expected_queen_moves); 

    }

    #[test]
    fn get_legal_moves_for_king() {
        let mut board = chess_board::initialize_chess_board();
        
        let mut king_moves = pieces_logic::get_legal_moves_for_king(&board, &(7, 4));
        assert_eq!(king_moves, []);

        board = chess_board::create_empty_board();

        pieces_logic::place_king_on_board(&mut board, &(4, 4), true);
        
        let mut expected_king_moves: Vec<pieces_logic::Move> = vec![];

        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 5)});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 4)});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (5, 3)});

        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (4, 5)});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (4, 3)});
        
        pieces_logic::place_rook_on_board(&mut board, &(3, 0), false);
        
        king_moves = pieces_logic::get_legal_moves_for_king(&board, &(4, 4));
        
        king_moves.sort();
        expected_king_moves.sort();

        assert_eq!(king_moves, expected_king_moves);

        board[3][0] = pieces_logic::create_empty_piece(&(3, 0));


        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 5)});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 4)});
        expected_king_moves.push(pieces_logic::Move {current_square: (4, 4), destination_square: (3, 3)});

        king_moves = pieces_logic::get_legal_moves_for_king(&board, &(4, 4));

        king_moves.sort();
        expected_king_moves.sort();

        assert_eq!(king_moves, expected_king_moves);
         
    }

    #[test]
    fn get_castle_move() {
        assert!(false);
    }


    #[test]
    fn get_all_legal_moves_for_this_turn() {
        let mut board = chess_board::create_empty_board();
        let mut expected_board_moves: Vec<pieces_logic::Move> = vec![];

        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_pawn_on_board(&mut board, &(5, 4), true);
        board[5][4].has_moved = true;
        pieces_logic::place_pawn_on_board(&mut board, &(6, 3), true);   
        pieces_logic::place_pawn_on_board(&mut board, &(6, 5), true);   
        pieces_logic::place_pawn_on_board(&mut board, &(7, 3), true);   
        pieces_logic::place_pawn_on_board(&mut board, &(7, 5), true);

        
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 5), destination_square: (5, 5) });
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 5), destination_square: (4, 5) });
        expected_board_moves.push(pieces_logic::Move { current_square: (5, 4), destination_square: (4, 4) });
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 3), destination_square: (5, 3) });
        expected_board_moves.push(pieces_logic::Move { current_square: (6, 3), destination_square: (4, 3) });
        expected_board_moves.push(pieces_logic::Move { current_square: (7, 4), destination_square: (6, 4) });
        
        let mut board_moves: Vec<pieces_logic::Move> = pieces_logic::get_all_legal_moves_for_this_turn(&board, true);

        board_moves.sort();
        expected_board_moves.sort();

        assert_eq!(expected_board_moves, board_moves);
    }
    

    #[test]
    fn is_checkmate() {
        let mut board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_rook_on_board(&mut board, &(6, 0), false);
        pieces_logic::place_rook_on_board(&mut board, &(7, 7), false);

        assert_eq!(true, pieces_logic::is_checkmate(&board, true));

        pieces_logic::place_rook_on_board(&mut board, &(0, 5), true);

        assert_eq!(false, pieces_logic::is_checkmate(&board, true));


    }

    #[test]
    fn is_stalemate() { 
        let mut board = chess_board::create_empty_board();
        
        pieces_logic::place_king_on_board(&mut board, &(7, 4), true);
        pieces_logic::place_rook_on_board(&mut board, &(0, 3), false);
        pieces_logic::place_rook_on_board(&mut board, &(0, 5), false);
        pieces_logic::place_rook_on_board(&mut board, &(6, 0), false);

        assert_eq!(true, pieces_logic::is_stalemate(&board, true));
        
        pieces_logic::place_pawn_on_board(&mut board, &(3, 0), true);

        assert_eq!(false, pieces_logic::is_stalemate(&board, true));
    
        pieces_logic::place_knight_on_board(&mut board, &(2, 0), false);

        assert_eq!(true, pieces_logic::is_stalemate(&board, true));

    }

}


