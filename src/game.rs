/**
 * Log to the console the board using the X and O bitflags
 */
pub fn create_board(x: u16, o: u16) -> String {
    let mut string_board = String::from("  ");
    
    for i in 0..9 {
        if ((1 << i) & x) != 0 { string_board += "X | "; }
        else if ((1 << i) & o) != 0 { string_board += "O | "; }
        else { string_board += "  | "; };
        if i != 0 && (i + 1) % 3 == 0 { 
            string_board += "\n"; 
            string_board += " --- --- ---\n  "
        };
    };
    
    string_board
}

/**
 * Calculate how many turns have occurred in the game. This is useful to determine whos turn it is
 */
pub fn get_turns(board: u16) -> u8 {
    let mut turns: u8 = 0;

    for i in 0..9 {
        if ((1 << i) & board) != 0 { turns += 1 }
    };

    turns
}