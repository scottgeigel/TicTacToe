use std::io;
use std::io::prelude::*;
mod tic_tac_toe;

mod ui {
    use std::io;
    pub fn get_user_input(buffer: &mut String) -> Result<usize, io::Error> {
        io::stdin().read_line(buffer)
    }
}

fn main() {
    let mut input = io::stdin().bytes();
    let mut board : tic_tac_toe::Board = tic_tac_toe::Board::new();
    let mut player : u8 = 1;
    print!("Player 1's name: ");
    let mut line : String = String::new();
    io::stdout().flush();
    ui::get_user_input(&mut line);
    println!("Hello {}", line);
    loop {
        //display the board
        println!("{}", board.to_string());
        println!("Player {}, make your selection", player);

        let input: Option<char> = input.next().and_then(|result| result.ok()).map(|byte| byte as char);
        match input {
            Some(ch) => {
                if ch == '\n' {
                    continue;
                }
            }
            None => println!("the end"),
        }
        let input = input.unwrap() as usize - ('0' as usize);
        //validate input

        if player == 1 {
            if !board.place_x(input) {
                continue;
            }
            player = 2;
        }
        else if player == 2 {
            if !board.place_o(input) {
                continue;
            }
            player = 1;
        }
        else {
            println!("Internal error");
            panic!();
        }

        match board.eval() {
            Some(n) => {
                match n {
                    tic_tac_toe::GameResult::XWin => println!("X wins"),
                    tic_tac_toe::GameResult::OWin => println!("O wins"),
                    tic_tac_toe::GameResult::Draw => println!("You're all losers"),
                };
                break;
            },
            None => continue,
        }
    }
}
