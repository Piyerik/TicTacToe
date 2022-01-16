use std::io;
use std::collections::HashMap;
mod game;
mod squares;
use squares::Squares;

fn main() {

    // instantiate squares
    let squares = Squares::new();

    // create a new hashmap with the players' bitflag move data
    let mut players: HashMap<&str, u16> = HashMap::new();
    players.insert("1", 0);
    players.insert("2", 0);

    // clear the console, then log the board
    print!("\x1B[2J\x1B[1;1H\n");
    println!("{}", game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

    // Loop the game until the game has been resolved
    loop {

        // get the current turn by seeing how many turns have passed
        let turn = if game::get_turns(*players.get("1").unwrap() + *players.get("2").unwrap()) % 2 == 1 { "2" } else { "1" };
        println!("Player {}: ", if turn == "1" { "X" } else { "O"} );

        // take in their input as a number representing the square to place their piece on
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        let choice = choice.trim().parse::<u16>().unwrap();
        if choice > 9 || choice < 1 { 
            println!("Invalid square position");
            continue;
        }

        let choice = 1 << (choice - 1);

        let current_player = *players.get(turn).unwrap();

        // add the player 1 and player2 occupied square bitflag data to get the current board. Validate the chosen square is not already occupied
        let board = *players.get("1").unwrap() + *players.get("2").unwrap();

        if (board & choice) != 0 {
            println!("You cannot place a piece on that square; already occupied");
            continue;
        }

        // re-insert the player data with the new square
        players.remove(turn);
        players.insert(turn, current_player + choice);

        // clear the console and print the board
        print!("\x1B[2J\x1B[1;1H\n");
        println!("{}", game::create_board(*players.get("1").unwrap(), *players.get("2").unwrap()));

        // check for if the player won the game, if so, notify they won and quit the game
        if squares.winning_positions.iter().any(|position| ((current_player + choice) & position) == *position) {
            return println!("{} won the game!", if turn == "1" { "X" } else { "O" })
        };
        
        // check if all squares are occupied and the game resulted in a tie, and if so, notify the game resulted in a tie and quit the game
        if board + choice == squares.squares.iter().sum() {
            return println!("It's a tie!");
        }

    }
}