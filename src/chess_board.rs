use crate::pieces_logic::{Piece, create_empty_piece};

pub fn print_chess_board(board_layout: &[[Piece;8];8]) {

    print!("\x1B[2J\x1B[1;1H"); // clear screen; set cursor to top row.

    let seperator = "  +---+---+---+---+---+---+---+---+";

    println!("{}", seperator);

    for x in 0..8 {
        print!("{} |", 8-x);
        for y in 0..8 {
            let piece = board_layout[x][y].symbol;
            print!(" {} |", piece);
        }
        println!("\n{}", seperator);
    }
    println!("    a   b   c   d   e   f   g   h");
}

pub fn initialize_chess_board() -> [[Piece; 8]; 8] {
    
    let mut ouput: [[Piece; 8]; 8] = [[create_empty_piece((0u8, 0u8)); 8]; 8];
    
    for x in 2..6 {
        for y in 0..8 {
            ouput[x][y] = create_empty_piece((x as u8, y as u8));
        }
    }
    


    let piece_symbols = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];
    let piece_values =  [5,3,3,9,1000,3,3,5];
    
    // Black side
    for y in 0..8 {
        ouput[0][y] = Piece {
            color: false,
            symbol: piece_symbols[y],
            has_moved: false,
            value: piece_values[y],
            is_empty: false,
            current_square: (0 as u8, y as u8),
        };
    }
    for y in 0..8 {
        ouput[1][y] = Piece {
            color: false,
            symbol: 'p',
            has_moved: false, 
            value: 1,
            is_empty: false,
            current_square: (1 as u8, y as u8),
        };
    }

    // White side
    for y in 0..8 {
        ouput[6][y] = Piece {
            color: true,
            symbol: 'P',
            has_moved: false,
            value: 1,
            is_empty: false,
            current_square: (6 as u8, y as u8),
        };
    }
    for y in 0..8 {
        ouput[7][y] = Piece {
            color: true,
            symbol: piece_symbols[y].to_ascii_uppercase(),
            has_moved: false,
            value: piece_values[y],
            is_empty: false,
            current_square: (7 as u8, y as u8),
        };
    }
    ouput
}
