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



pub fn is_piece_pinned(board: &[[Piece;8];8], king_square: &(u8, u8), piece_square: &(u8, u8)) -> bool {

    // We need to make sure that the squares between king and piece are is_empty: true
    // AND that the first piece we encounter from the piece that's pinned is an enemy piece
    // has the ability to pin -> queen, rook, bishop. 
    
    // This allows discovery checks.
    let king = &board[king_square.0 as usize][king_square.1 as usize];
    let piece = &board[piece_square.0 as usize][piece_square.1 as usize];
    if king.color != piece.color {
        return false;
    }


    let x: (i8, i8) = (
            (king_square.0 as i8 - piece_square.0 as i8),
            (king_square.1 as i8 - piece_square.1 as i8));
    
    let diagonal = (x.0.abs() - x.1.abs()) == 0;

    if diagonal {

        let mut is_diagonal_empty: bool = true;
        
        // IF           x.0 > x.1
        // THEN         piece is to the top right of king.
        // THEREFORE    iterate by ->  king.0-1 , king.1+1
        
        if x.0 > x.1 {

            let mut row: usize = (king_square.0-1) as usize;
            let mut col: usize = (king_square.1+1) as usize;

            // Checks if connection between king and piece is empty 
            while row > piece_square.0 as usize && col < piece_square.1 as usize {
                if !board[row][col].is_empty {
                    return false;
                }
                row -= 1;
                col += 1;
            }

            // jumping over piece 
            row -= 1;
            col += 1;
            // Checks if connection between piece and potential enemy piece is is_empty
            while row > 0 && col < 8 {
                
                let square: &Piece = &board[row][col];

                if !square.is_empty {
                    is_diagonal_empty = false;

                    // readability
                    let found_piece = &square;

                    if found_piece.color != king.color {
                        
                        if match found_piece.symbol {
                            'r' => true,
                            'R' => true,
                            'p' => true,
                            'P' => true,
                            'k' => true,
                            'K' => true,
                            _ => false,
                        } {
                            return false;
                        } else {
                            return true;
                        }
                    } else {
                        // If there is some other piece of the same color as our king between the
                        // piece and potentially some enemy piece (bishop/queen), we prematurely
                        // quit the lookup and return false, since a doubled-up color is never a
                        // pin
                        return false;
                    }
                }
                row -= 1;
                col += 1;
            }
            if is_diagonal_empty {
                return false;
            }
        }


        // IF           x.0 < x.1
        // THEN         piece is to the bottom left of the king.
        // THEREFORE    iterate by ->  king.0-1 , king.1+1 
        
        // IF           x.0 == x.1
    
        // AND           x.0 > 0
        // THEN         piece is to the top left of king
        // THEREFORE    iterate by ->  king.0+1 , king.1+1
        
        // AND           x.0 < 0
        // THEN         piece is to the bottom right of the king.
        // THEREFORE    iterate by ->   king.0-1 , king.1-1
        return true;
    }
    
    let vertical = x.1 == 0;

    if vertical {

        // IF           x.0 > 0
        // THEN         piece is above king 
        // THEREFORE    iterate by ->  king.0+1 , king.1+0

        // IF           x.0 < 0
        // THEN         piece is below king
        // THEREFORE    iterate by ->  king.0-1
    }

    let horizontal = x.0 == 0;

    if horizontal {

        // IF           x.1 > 0
        // THEN         piece is to the left of king
        // THEREFORE    iterate by ->  king.1+1
        
        // IF           x.1 < 0
        // THEN         piece is to the right of king
        // THEREFORE    iterate by ->  king.1-1
    }


    false
}





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
        board[cur_sq.0 as usize][cur_sq.1 as usize] = create_empty_piece(&cur_sq);

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


