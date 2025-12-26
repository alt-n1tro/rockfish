#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    pub color: bool,
    pub symbol: Symbol,
    pub has_moved: bool,
    pub value: i64,
    pub is_empty: bool,
    pub current_square: (u8, u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
    NoPromotion,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Symbol {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

// This should only ever be created via a function, that checks if the move is actually legal.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub current_square: (u8, u8),
    pub destination_square: (u8, u8),
    pub castle: bool,
    pub promotion: Promotion,
}


pub fn create_empty_piece(square: &(u8, u8)) -> Piece {
    Piece { color: false, 
        symbol: Symbol::Empty,
        has_moved: false,
        value: 0,
        is_empty: true,
        current_square: *square }
}

pub fn place_pawn_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Pawn,
        has_moved: false,
        value: 100,
        is_empty: false,
        current_square: *square
    };

}

pub fn place_bishop_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Bishop,
        has_moved: false,
        value: 330,
        is_empty: false,
        current_square: *square
    };
    
}

pub fn place_knight_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Knight,
        has_moved: false,
        value: 320,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_rook_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Rook,
        has_moved: false,
        value: 500,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_queen_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::Queen,
        has_moved: false,
        value: 900,
        is_empty: false,
        current_square: *square
    };
}

pub fn place_king_on_board(board: &mut [[Piece; 8]; 8], square: &(u8, u8), color: bool) {
    board[square.0 as usize][square.1 as usize] = Piece {
        color: color,
        symbol: Symbol::King,
        has_moved: false,
        value: 10000,
        is_empty: false,
        current_square: *square
    };
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
        if board[7][4].symbol == Symbol::King {
            return (7 as u8, 4 as u8);
        }
        
        for x in (0..8).rev() {
            for y in (0..8).rev() {
                if board[x][y].symbol == Symbol::King {
                    return (x as u8, y as u8);
                }
            }
        }
        unreachable!("\nNo White King was found...\nAt least 2 Kings MUST exist at all times!");
    } else {
        if board[0][4].symbol == Symbol::King {
            return (0 as u8, 4 as u8);
        }
        
        for x in 0..8 {
            for y in 0..8 {
                if board[x][y].symbol == Symbol::King {
                    return (x as u8, y as u8);
                }
            }
        }
        unreachable!("\nNo Black King was found...\nAt least 2 Kings MUST exist at all times!");
    }
}



// This function looks outward from the king -- Not from the enemy pieces!
pub fn is_king_in_check(board: &[[Piece;8];8], color: bool) -> bool {
    
    let k_s: (u8, u8) = get_square_of_king(&board, color);
    let king: Piece = board[k_s.0 as usize][k_s.1 as usize];



    // Check Knights
    
    let knight_moves: [(isize, isize); 8] = [(-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right

    
    for x in 0..knight_moves.len() {
        let n_move = knight_moves[x];

        let row = k_s.0 as isize + n_move.0;
        let col = k_s.1 as isize + n_move.1;
        if (row >= 0 && row < 8) && (col >= 0 && col < 8) {
            let square = board[row as usize][col as usize];
            if  (square.color != king.color) && square.symbol == Symbol::Knight {
                return true;    
            }
        }  
    }

    // Check Rook + Queen(straights)
    // Check Up+Down
    // Check Left+Right
    

    let rook_moves = [(-1, 0), (1, 0),
                      (0, -1), (0, 1)];
    
    for rook_move in rook_moves {
        let mut row = k_s.0 as i8 + rook_move.0;
        let mut col = k_s.1 as i8 + rook_move.1;

        'inner_while: while row >= 0 && row < 8 && col >= 0 && col < 8 {
            
            let square = &board[row as usize][col as usize];

            if !square.is_empty {
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, Symbol::Rook | Symbol::Queen) {
                        return true;
                    } else {
                        break 'inner_while;
                    }
                }
            }
            row += rook_move.0;
            col += rook_move.1;
        }
    }

    // Check Bishop + Queen(diagonals)
    // top right -> (-1, 1)
    // top left -> (-1, -1)
    // bottom right -> (1, 1) --> Check this.
    // bottom left -> (1, -1)
    
    let bishop_moves = [(1, 1), (1, -1),
                        (-1, 1), (-1, -1)];
    
    for b_move in bishop_moves {
        let mut bishop_row: i8 = k_s.0 as i8 + b_move.0;
        let mut bishop_col: i8 = k_s.1 as i8+ b_move.1;

        'inner_while: while bishop_row >= 0 && bishop_row < 8 && bishop_col >= 0 && bishop_col < 8 {
            
            let square = &board[bishop_row as usize][bishop_col as usize];

            if !square.is_empty {
                
                if king.color == square.color {
                    break 'inner_while;
                } else {
                    if matches!(square.symbol, Symbol::Bishop | Symbol::Queen) {
                        return true;
                    } else {
                        break 'inner_while;
                    }
                }

            }

            bishop_row += b_move.0;
            bishop_col += b_move.1;
        }

    }
    

    let black_pawns: [(i8, i8);2] = [((k_s.0 as i8 - 1), (k_s.1 as i8 + 1)),
                                    ((k_s.0 as i8 - 1), (k_s.1 as i8 - 1))];
    let white_pawns: [(i8, i8);2] = [((k_s.0 as i8 + 1), (k_s.1 as i8 + 1)),
                                    ((k_s.0 as i8 + 1), (k_s.1 as i8 - 1))];
    // Check Pawns 
    if king.color {
        for pawn in black_pawns {
            if pawn.0 >= 0 && pawn.1 < 8 && pawn.1 >= 0{
                if board[pawn.0 as usize][pawn.1 as usize].symbol == Symbol::Pawn {
                    return true;
                }
            } 
        }
    } else {
        for pawn in white_pawns {
            if pawn.0 < 8 && pawn.1 < 8 && pawn.1 >= 0{
                if board[pawn.0 as usize][pawn.1 as usize].symbol == Symbol::Pawn {
                    return true;
                }
            } 
        }
    }


    return false;
}






pub fn is_piece_pinned(board: &[[Piece;8];8], piece_move: &Move) -> bool {
    
    let from = piece_move.current_square;

    let piece_to_move: Piece = board[from.0 as usize][from.1 as usize];
    
    let mut new_board = *board;

    make_move(&mut new_board, piece_move);

    is_king_in_check(&new_board, piece_to_move.color)

}



// Moves 

// Piece specific move functions
pub fn get_legal_moves_for_pawn(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let mut output: Vec<Move> = vec![];
        
    let piece_to_move = board[square.0 as usize][square.1 as usize];

    let adder: i8 = if piece_to_move.color {
        -1
    } else {
        1
    };
    
    let new_row = square.0 as i8 + adder;

    if new_row >= 0 && new_row < 8 { 
        if board[new_row as usize][square.1 as usize].is_empty {
            let up_move: Move = Move {current_square: *square, destination_square: (new_row as u8, square.1), castle: false, promotion: Promotion::NoPromotion};
            if !is_piece_pinned(&board, &up_move) {
                output.push(up_move);
                if (new_row + adder) >= 0 && (new_row + adder) < 8 {
                    if board[(new_row + adder) as usize][square.1 as usize].is_empty && !piece_to_move.has_moved {
                        let up_up_move: Move = Move { current_square: *square, destination_square: ((new_row + adder) as u8, square.1), castle: false, promotion: Promotion::NoPromotion};
                        output.push(up_up_move); 
                    }
                }
            }
        }

        // right capture
        if square.1 < 7 {
            let board_square = board[new_row as usize][(square.1 + 1) as usize];
            if board_square.color != piece_to_move.color && !board_square.is_empty {
                let left_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 + 1), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &left_move) {
                    output.push(left_move);
                }
            }
        }
        // left capture
        if square.1 >= 1 {
            let board_square = board[new_row as usize][(square.1 - 1) as usize];
            if board_square.color != piece_to_move.color && !board_square.is_empty {
                let right_move: Move = Move { current_square: *square, destination_square: (new_row as u8, square.1 - 1), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &right_move) {
                    output.push(right_move);
                }
            }
        }
    }

    if new_row == 0 || new_row == 7 {
        let mut temp_output: Vec<Move> = vec![];
        for pawn_move in &output {
            let mut temp_pawn_move = *pawn_move;
            for x in [Promotion::Queen, Promotion::Rook, Promotion::Bishop, Promotion::Knight] {
                temp_pawn_move.promotion = x;
                temp_output.push(temp_pawn_move);
            }
        }
        return temp_output;
    }

    output
}

pub fn get_legal_moves_for_knight(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> { 
    
    let mut output: Vec<Move> = vec![];

    let knight_moves: [(isize, isize); 8] = [
        (-2, -1), (-2, 1), // top
        (-1, -2), (1, -2), // left
        (2, -1), (2, 1), // bottom
        (1, 2),(-1, 2)]; // right
    
    for knight in knight_moves {
        let row = knight.0 + square.0 as isize;
        let col = knight.1 + square.1 as isize;
        if row >= 0 && row < 8 && col >= 0 && col < 8 {
            let to_sqr = board[row as usize][col as usize];
            if to_sqr.is_empty || (to_sqr.color != board[square.0 as usize][square.1 as usize].color) {
                let knight_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8), castle: false, promotion: Promotion::NoPromotion};
                if !is_piece_pinned(&board, &knight_move) {
                    output.push(knight_move);
                }
            }
        }
    }
    output
}

pub fn get_legal_long_ray_moves(board: &[[Piece; 8]; 8], square: &(u8, u8), moves: [(i8, i8); 4]) -> Vec<Move> {

    let mut output: Vec<Move> = vec![];

    for x in moves {
        let mut row = square.0 as i8 + x.0;
        let mut col = square.1 as i8 + x.1;
        
        'inner_while: while row >= 0 && row < 8 && col >= 0 && col < 8 {
            let destination_square = board[row as usize][col as usize];
            let piece_color = board[square.0 as usize][square.1 as usize].color;
            
            let proposed_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8), castle: false, promotion: Promotion::NoPromotion};

            if !destination_square.is_empty && destination_square.color == piece_color {  
                break 'inner_while; 
            }
                
            if !is_piece_pinned(&board, &proposed_move) {
                output.push(proposed_move);
            }

            if !destination_square.is_empty {
                break 'inner_while;
            }                
            
            row += x.0;
            col += x.1;
        }
    }
    output

}


pub fn get_legal_moves_for_bishop(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    let bishop_moves: [(i8, i8); 4] = [(1, 1), (1, -1),
                                       (-1, 1), (-1, -1)];
    get_legal_long_ray_moves(&board, &square, bishop_moves)    
}

pub fn get_legal_moves_for_rook(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {

    let rook_moves: [(i8, i8); 4] = [(1, 0), (-1, 0),
                                     (0, 1), (0, -1)];
    get_legal_long_ray_moves(&board, &square, rook_moves)    
}



pub fn get_legal_moves_for_queen(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let mut output: Vec<Move> = get_legal_moves_for_rook(&board, &square);

    output.extend(get_legal_moves_for_bishop(&board, &square));
    
    output
}


pub fn get_legal_moves_for_king(board: &[[Piece;8];8], square: &(u8, u8)) -> Vec<Move> {
    
    let king_moves: [(i8, i8); 8] = [(1, 1), (1, 0), (1, -1),
                                     (0, 1), (0, -1),
                                     (-1, 1), (-1, 0), (-1, -1)];
    
    let mut output: Vec<Move> = vec![];

    for king_move in king_moves {
        let row: i8 = square.0 as i8 + king_move.0;
        let col: i8 = square.1 as i8 + king_move.1;
        
        if row >= 0 && row < 8 && col >= 0 && col < 8 {
            let board_square = board[row as usize][col as usize];
            let piece_square = board[square.0 as usize][square.1 as usize];

            let k_move: Move = Move { current_square: *square, destination_square: (row as u8, col as u8), castle: false, promotion: Promotion::NoPromotion};
            
            if board_square.is_empty {
                if !is_piece_pinned(&board, &k_move) {
                    output.push(k_move);
                }
            } else {
                if board_square.color != piece_square.color && !is_piece_pinned(&board, &k_move) {
                    output.push(k_move);
                }
            }
        }
    }

    output.extend(get_castling_moves(&board, board[square.0 as usize][square.1 as usize].color));
    
    output
}


pub fn get_castling_moves(board: &[[Piece;8];8], color: bool) -> Vec<Move> {
    
    let king = get_square_of_king(&board, color);
    let king_piece = board[king.0 as usize][king.1 as usize];

    let mut output: Vec<Move> = vec![];

    let row: usize = if color {7} else {0};
    let left_corner: Piece = board[row][0];
    let right_corner: Piece = board[row][7];

    if king_piece.has_moved {
        return output;
    }

    if !left_corner.is_empty && left_corner.symbol == Symbol::Rook && !left_corner.has_moved {
        
        if  board[king.0 as usize][(king.1 - 1) as usize].is_empty &&
            board[king.0 as usize][(king.1 - 2) as usize].is_empty {
            // Check for pins 
            if  !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 - 1), castle: false, promotion: Promotion::NoPromotion }) &&
                !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 - 2), castle: false, promotion: Promotion::NoPromotion }) {
                output.push(Move { current_square: king, destination_square: (king.0, king.1 - 2), castle: true, promotion: Promotion::NoPromotion });
            }
        }
    }
    
    if !right_corner.is_empty && right_corner.symbol == Symbol::Rook && !right_corner.has_moved {
        
        if  board[king.0 as usize][(king.1 + 1) as usize].is_empty &&
            board[king.0 as usize][(king.1 + 2) as usize].is_empty {
            // Check for pins 
            if  !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 + 1), castle: false, promotion: Promotion::NoPromotion }) &&
                !is_piece_pinned(&board, &Move { current_square: king, destination_square: (king.0, king.1 + 2), castle: false, promotion: Promotion::NoPromotion }) {
                output.push(Move { current_square: king, destination_square: (king.0, king.1 + 2), castle: true, promotion: Promotion::NoPromotion });
            }
        }
    }
    output
}




// We ensure that every move is legal in other functions.
pub fn make_move(board: &mut [[Piece; 8]; 8], move_: &Move) {
    
    let cur_sq = move_.current_square;
    let des_sq = move_.destination_square;
    
    board[des_sq.0 as usize][des_sq.1 as usize] = board[cur_sq.0 as usize][cur_sq.1 as usize];    
    board[des_sq.0 as usize][des_sq.1 as usize].current_square = des_sq;
    board[des_sq.0 as usize][des_sq.1 as usize].has_moved = true;
    
    board[cur_sq.0 as usize][cur_sq.1 as usize] = create_empty_piece(&cur_sq);

    if move_.castle {
        let shift: i8 = if des_sq.1 > cur_sq.1 {1} else {-1};
        let rook_side: usize = if des_sq.1 > cur_sq.1 {7} else {0};
        
        board[cur_sq.0 as usize][cur_sq.1 as usize + shift as usize] = board[cur_sq.0 as usize][rook_side];
        board[cur_sq.0 as usize][cur_sq.1 as usize + shift as usize].current_square = (cur_sq.0, cur_sq.1 + shift as u8);
        board[cur_sq.0 as usize][cur_sq.1 as usize + shift as usize].has_moved = true;

        board[cur_sq.0 as usize][rook_side] = create_empty_piece(&(cur_sq.0, rook_side as u8));    
    }

    match move_.promotion {
        Promotion::Queen => place_queen_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Rook => place_rook_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Bishop => place_bishop_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::Knight => place_knight_on_board(board, &move_.destination_square, board[move_.current_square.0 as usize][move_.current_square.1 as usize].color),
        Promotion::NoPromotion => {},
    };

}



// For bot moves
pub fn find_all_legal_moves_for_a_piece(board: &[[Piece; 8]; 8], square: &(u8, u8)) -> Vec<Move> {
    
    let empty_vec: Vec<Move> = vec![];

    match board[square.0 as usize][square.1 as usize].symbol {
        Symbol::Pawn => get_legal_moves_for_pawn(&board, &square),
        Symbol::Bishop => get_legal_moves_for_bishop(&board, &square),
        Symbol::Knight => get_legal_moves_for_knight(&board, &square),
        Symbol::Rook => get_legal_moves_for_rook(&board, &square),
        Symbol::Queen => get_legal_moves_for_queen(&board, &square),
        Symbol::King => get_legal_moves_for_king(&board, &square),
        Symbol::Empty => empty_vec,
    }

}

pub fn get_all_legal_moves_for_this_turn(board: &[[Piece;8];8], side: bool) -> Vec<Move> {

    let mut output: Vec<Move> = vec![];
    
    for x in 0..8 {
        for y in 0..8 {
            let board_square: Piece = board[x][y];
            if board_square.color == side && !board_square.is_empty { 
                output.extend(find_all_legal_moves_for_a_piece(&board, &(x as u8, y as u8)));
            }
        }
    } 

    output
}


// Game States
pub fn is_checkmate(board: &[[Piece; 8]; 8], side: bool) -> bool {

    let output: Vec<Move> = vec![];

    is_king_in_check(&board, side) && get_all_legal_moves_for_this_turn(&board, side) == output
}

pub fn is_stalemate(board: &[[Piece; 8]; 8], side: bool) -> bool {

    let output: Vec<Move> = vec![];

    !is_king_in_check(&board, side) && get_all_legal_moves_for_this_turn(&board, side) == output
}

pub fn is_insufficient_material(board: &[[Piece;8];8]) -> bool {
    
    let mut white_counts: u8 = 0;
    let mut black_counts: u8 = 0;

    for x in 0..8 {
        for y in 0..8 {
            let piece = board[x][y];
            if !piece.is_empty && !matches!(piece.symbol, Symbol::King) {
                if matches!(piece.symbol, Symbol::Pawn | Symbol::Rook | Symbol::Queen) {
                    return false;
                } else {
                    if piece.color {
                        white_counts += 1;
                    } else {
                        black_counts += 1;
                    }
                }
            }
        }
    }
    // King + bishop / knight == Insufficint material!
    // King + bishop + bishop / knight == sufficient material! 
    // king + knight + knight = sufficient material! 
    // Essentially, we need to be able to give check on TWO straight squares! Bishop/Knight can
    // only check one square. So, alone they are unable to deliver checkmate.
    if white_counts < 2 && black_counts < 2 {
        return true;
    }

    false
}





// Minimax
pub fn evaluate(board: &[[Piece; 8]; 8]) -> i64 {
    
    let mut evaluation: i64 = 0;

    for x in 0..8 {
        for y in 0..8 {
            let piece = board[x][y];
            if piece.color {
                evaluation += piece.value;
            } else {
                evaluation -= piece.value;
            }
        }
    }
    evaluation
}


pub fn negamax(board: &[[Piece;8];8], node: Move, depth: u8, mut alpha: i64, beta: i64, color: i64) -> i64 {
    
    if depth == 0 {
        return color * evaluate(&board);
    }    
    let mut temp_board = *board;
    make_move(&mut temp_board, &node);

    let child_node: Vec<Move> = find_all_legal_moves_for_a_piece(&temp_board, &node.destination_square);
    let mut value: i64 = std::i64::MIN;

    for child in child_node {
        value = std::cmp::max(value, negamax(&board, child, depth-1, -beta, -alpha, -color));
        alpha = std::cmp::max(alpha, value);
        if alpha >= beta {
            break;
        }
    }
    value
}






// Communication
//pub fn move_to_universal_chess_interface(move_: &Move) -> String {}

pub fn universal_chess_interface_to_move(board: &[[Piece;8];8], uci: String) -> Result<Move, &'static str> {
    
    let chars: Vec<char> = uci.chars().collect();
    let mut temp_move: Move = Move {current_square: (0, 0), destination_square: (0, 0), castle: false, promotion: Promotion::NoPromotion};
    
    if matches!(chars[0], 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') {
        temp_move.current_square.1 = chars[0] as u8 - 97;
    } else {
        return Err("Incorrect FROM Column! -> a-h");
    }

    if matches!(chars[1], '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') {
        temp_move.current_square.0 = 7 - (chars[1] as u8 - 49);
    } else {
        return Err("Incorrect FROM Row! -> 1-8");
    }

    if matches!(chars[2], 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h') {
        temp_move.destination_square.1 = chars[2] as u8 - 97;
    } else {
        return Err("Incorrect TO Column! -> a-h");
    }

    if matches!(chars[3], '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') {
        temp_move.destination_square.0 = 7 - (chars[3] as u8 - 49);
    } else {
        return Err("Incorrect TO Row! -> 1-8");
    }

    if board[temp_move.current_square.0 as usize][temp_move.current_square.1 as usize].symbol == Symbol::King {
        if (temp_move.current_square.1 as i8 - temp_move.destination_square.1 as i8).abs() == 2 {
            temp_move.castle = true;
        }
    }

    if board[temp_move.current_square.0 as usize][temp_move.current_square.1 as usize].symbol == Symbol::Pawn {
        if temp_move.destination_square.0 == 7 || temp_move.destination_square.0 == 0 {
            if chars.len() > 4 {
                temp_move.promotion = match chars[4] {
                   'q' => Promotion::Queen,
                    'r' => Promotion::Rook,
                    'b' => Promotion::Bishop,
                    'n' => Promotion::Knight,
                    _ => return Err("Incorrect PROMOTION! -> q/r/b/n"),
                };
            } else {
                return Err("Ensure Proper Promotion Notation -> a1a2q");
            }
        }
    }
    



    Ok(temp_move)
}
