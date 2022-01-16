use std::io;
use std::collections::HashMap;
mod game;
mod squares;
use squares::Squares;

fn main() {

    let squares = Squares::new();

    let mut players: HashMap<&str, u16> = HashMap::new();
    players.insert("1", 0);
    players.insert("2", 0);

    print!("\x1B[2J\x1B[1;1H\n");
    println!("{}", game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

    loop {

        let turn = if game::get_turns(*players.get("1").unwrap() + *players.get("2").unwrap()) % 2 == 1 { "2" } else { "1" };
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
        println!("{}", game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

        if squares.winning_positions.iter().any(|position| ((current_player + choice) & position) == *position) {
            return println!("{} won the game!", if turn == "1" { "X" } else { "O" })
        };
        
        if board + choice == squares.squares.iter().sum() {
            return println!("It's a tie!");
        }

    }
}