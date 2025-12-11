use crate::pieces_logic::Piece;


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
    
    let default_piece = Piece {
                        color: false,
                        symbol: ' ',
                        has_moved: false,
                        value: 0,
                        is_empty: true,
    };



    let mut ouput: [[Piece; 8]; 8] = [[default_piece; 8]; 8];
     
    let piece_symbols = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];
    let piece_values =  [5,3,3,9,1000,3,3,5];
    
    // Black side
    for x in 0..8 {
        ouput[0][x] = Piece {
            color: false,
            symbol: piece_symbols[x],
            has_moved: false,
            value: piece_values[x],
            is_empty: false,
        };
    }
    for x in 0..8 {
        ouput[1][x] = Piece {
            color: false,
            symbol: 'p',
            has_moved: false, 
            value: 1,
            is_empty: false,
        };
    }

    // White side
    for x in 0..8 {
        ouput[6][x] = Piece {
            color: true,
            symbol: 'P',
            has_moved: false,
            value: 1,
            is_empty: false,
        };
    }
    for x in 0..8 {
        ouput[7][x] = Piece {
            color: true,
            symbol: piece_symbols[x].to_ascii_uppercase(),
            has_moved: false,
            value: piece_values[x],
            is_empty: false,
        };
    }
    ouput
}
