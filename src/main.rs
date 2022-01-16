use std::io;
use std::collections::HashMap;

struct Game {}

impl Game {
    fn create_board(x: u16, o: u16) -> String {
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

    fn get_turns(board: u16) -> u8 {
        let mut turns: u8 = 0;

        for i in 0..9 {
            if ((1 << i) & board) != 0 { turns += 1 }
        };

        turns
    }
}

fn main() {

    let mut players: HashMap<&str, u16> = HashMap::new();
    players.insert("1", 0);
    players.insert("2", 0);

    let squares = vec![
        1 << 0,
        1 << 1,
        1 << 2,
        1 << 3,
        1 << 4,
        1 << 5,
        1 << 6,
        1 << 7,
        1 << 8
    ];

    let winning_positions = vec![
        squares[0] + squares[1] + squares[2],
        squares[3] + squares[4] + squares[5],
        squares[6] + squares[7] + squares[8],

        squares[0] + squares[3] + squares[6],
        squares[1] + squares[4] + squares[7],
        squares[2] + squares[5] + squares[8],

        squares[0] + squares[4] + squares[8],
        squares[6] + squares[4] + squares[2]
    ];

    print!("\x1B[2J\x1B[1;1H\n");
    println!("{}", Game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

    loop {

        let turn = if Game::get_turns(*players.get("1").unwrap() + *players.get("2").unwrap()) % 2 == 1 { "2" } else { "1" };
        println!("Player {}: ", if turn == "1" { "X" } else { "O"} );

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        let choice = choice.trim().parse::<u16>().unwrap();
        if choice > 9 || choice < 1 { 
            println!("Invalid square position");
            continue;
        }

        let choice = 1 << (choice - 1);

        let current_player = *players.get(turn).unwrap();

        let board = *players.get("1").unwrap() + *players.get("2").unwrap();

        if (board & choice) != 0 {
            println!("You cannot place a piece on that square; already occupied");
            continue;
        }

        players.remove(turn);
        players.insert(turn, current_player + choice);

        print!("\x1B[2J\x1B[1;1H\n");
        println!("{}", Game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

        if winning_positions.iter().any(|position| ((current_player + choice) & position) == *position) {
            return println!("{} won the game!", if turn == "1" { "X" } else { "O" })
        };
        
        if board + choice == squares.iter().sum() {
            return println!("It's a tie!");
        }

    }
}