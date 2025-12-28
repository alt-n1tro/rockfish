use crate::pieces_logic;


fn get_char_symbol_for_symbol_enum(symbol: pieces_logic::Symbol, side: bool) -> char {
    let mut output = match symbol {
        pieces_logic::Symbol::King => 'k',
        pieces_logic::Symbol::Queen => 'q',
        pieces_logic::Symbol::Rook => 'r',
        pieces_logic::Symbol::Bishop => 'b',
        pieces_logic::Symbol::Knight => 'n',
        pieces_logic::Symbol::Pawn => 'p',
        pieces_logic::Symbol::Empty => ' ',
    };
    
    if side {
        output = output.to_ascii_uppercase();
    }

    output
}


pub fn print_chess_board(board_layout: &[[pieces_logic::Piece;8];8]) {

    print!("\x1B[2J\x1B[1;1H"); // clear screen; set cursor to top row.

    let seperator = "  +---+---+---+---+---+---+---+---+";

    println!("{}", seperator);

    for x in 0..8 {
        print!("{} |", 8-x);
        for y in 0..8 {
            let piece = board_layout[x][y];
            print!(" {} |", get_char_symbol_for_symbol_enum(piece.symbol, piece.color));
        }
        println!("\n{}", seperator);
    }
    println!("    a   b   c   d   e   f   g   h");
}

pub fn initialize_chess_board() -> [[pieces_logic::Piece; 8]; 8] {
    
    let mut ouput: [[pieces_logic::Piece; 8]; 8] = [[pieces_logic::create_empty_piece(&(0u8, 0u8)); 8]; 8];
    
    for x in 2..6 {
        for y in 0..8 {
            ouput[x][y] = pieces_logic::create_empty_piece(&(x as u8, y as u8));
        }
    }
    
    // Black side
    pieces_logic::place_rook_on_board(&mut ouput, &(0, 0), false);
    pieces_logic::place_knight_on_board(&mut ouput, &(0, 1), false);
    pieces_logic::place_bishop_on_board(&mut ouput, &(0, 2), false);
    pieces_logic::place_queen_on_board(&mut ouput, &(0, 3), false);
    pieces_logic::place_king_on_board(&mut ouput, &(0, 4), false);
    pieces_logic::place_bishop_on_board(&mut ouput, &(0, 5), false);
    pieces_logic::place_knight_on_board(&mut ouput, &(0, 6), false);
    pieces_logic::place_rook_on_board(&mut ouput, &(0, 7), false);

    for y in 0..8 {
        pieces_logic::place_pawn_on_board(&mut ouput, &(1, y), false);
    }


    // White side
    for y in 0..8 {
        pieces_logic::place_pawn_on_board(&mut ouput, &(6, y), true);
    }
    
    pieces_logic::place_rook_on_board(&mut ouput, &(7, 0), true);
    pieces_logic::place_knight_on_board(&mut ouput, &(7, 1), true);
    pieces_logic::place_bishop_on_board(&mut ouput, &(7, 2), true);
    pieces_logic::place_queen_on_board(&mut ouput, &(7, 3), true);
    pieces_logic::place_king_on_board(&mut ouput, &(7, 4), true);
    pieces_logic::place_bishop_on_board(&mut ouput, &(7, 5), true);
    pieces_logic::place_knight_on_board(&mut ouput, &(7, 6), true);
    pieces_logic::place_rook_on_board(&mut ouput, &(7, 7), true);

    ouput
}

pub fn create_empty_board() -> [[pieces_logic::Piece; 8]; 8] {
    let mut board: [[pieces_logic::Piece;8]; 8] = [[pieces_logic::create_empty_piece(&(0,0));8];8];

    for x in 0..8 {
        for y in 0..8 {
            board[x][y] = pieces_logic::create_empty_piece(&(x as u8, y as u8));
        }
    }
    board
}

