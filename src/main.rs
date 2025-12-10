use std::error::Error;


struct piece {
    color: bool,
    symbol: char,
    has_moved: bool,
}


fn print_chess_board(board_layout: &[[u8;8];8]) {

    print!("\x1B[2J\x1B[1;1H"); // clear screen; set cursor to top row.

    let seperator = "  +---+---+---+---+---+---+---+---+";

    println!("{}", seperator);

    for x in 0..8 {
        print!("{} |", 8-x);
        let row = board_layout[x];
        for y in 0..8 {
            let piece = match row[y] {
                1 => "r",
                2 => "n",
                3 => "b",
                4 => "q",
                5 => "k",
                6 => "p",
                7 => "R",
                8 => "N",
                9 => "B",
                10 => "Q",
                11 => "K",
                12 => "P",
                _ => " ",
            };
            print!(" {} |", piece);
        }
        println!("\n{}", seperator);
    }
    println!("    a   b   c   d   e   f   g   h");
}

fn get_2d_location_of_piece(square: u8) -> (u8, u8) {
    (square / 8, square % 8)
}





// This should call a couple helper functions based on the symbol of the piece (ie if it's a bishop
// vs knight)
fn legal_moves(current_board: &[[u8;8];8], square: u8) -> Vec<u8> {
    let found_legal_moves: Vec<u8> = vec![];
    let cur_square = get_2d_location_of_piece(square);
    
    


    found_legal_moves
}




fn main() {
    
    let board: [[u8;8];8] =[[1,2,3,4,5,3,2,1],
                            [6,6,6,6,6,6,6,6],
                            [0,0,0,0,0,0,0,0],
                            [0,0,0,0,0,0,0,0],
                            [0,0,0,0,0,0,0,0],
                            [0,0,0,0,0,0,0,0],
                            [12,12,12,12,12,12,12,12],
                            [7,8,9,11,10,9,8,7]]; 

    print_chess_board(&board);
    
    for x in [0, 13, 8, 34, 63, 10] {
        let square = get_2d_location_of_piece(x);
        println!("{}, {}",square.0, square.1);
    }
}
