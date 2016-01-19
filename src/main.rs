use std::io;
use std::io::prelude::*;
mod tic_tac_toe;

mod ui {
    use std::io;
    use std::io::Write;
    use std::io::{Error, ErrorKind};
    pub fn get_user_input(buffer: &mut String) -> Result<usize, Error> {
        io::stdin().read_line(buffer)
    }

    pub fn get_user_selection() -> Result<char, Error> {
        let mut buffer : String = String::new();
        match get_user_input(&mut buffer) {
            Ok(length) => {
                if buffer.trim().len() > 1 {
                    Err(Error::new(ErrorKind::Other, "Expected single character"))
                } else {
                    Ok(buffer.as_bytes()[0] as char)
                }
            },
            Err(e) => Err(e),
        }
        //make sure the line only has 1 character
    }

    pub fn clear_screen() {
        let cmd : [u8; 12] = [0x1bu8, 0x5bu8, 0x33u8, 0x3bu8, 0x4au8, 0x1bu8, 0x5bu8, 0x48u8, 0x1bu8, 0x5bu8, 0x32u8, 0x4au8];
        io::stdout().write(&cmd);
    }

    pub fn display(text : String) {
        clear_screen();
        io::stdout().write(text.as_bytes());
    }
}

struct Game {
    board : tic_tac_toe::Board,
    current_player : u8,
    error_msg : String,
}

impl Game {
    fn new(num_players : u8) -> Game {
        assert!(num_players <= 2);
        Game {
            board : tic_tac_toe::Board::new(),
            current_player : 0,
            error_msg : "".to_string(),
        }
    }
}

fn main() {
    let mut input = io::stdin().bytes();
    let mut board : tic_tac_toe::Board = tic_tac_toe::Board::new();
    let mut player : u8 = 1;
    let mut error_msg : String = "".to_string();
    print!("Player 1's name: ");
    let mut line : String = String::new();
    io::stdout().flush();
    ui::get_user_input(&mut line);
    println!("Hello {}", line);
    loop {
        ui::clear_screen();
        //display the board
        ui::display(board.to_string());
        println!("{}", error_msg);
        println!("Player {}, make your selection", player);

        error_msg = "".to_string();
        let input_result = ui::get_user_selection();
        let mut input;
        match input_result {
            Ok(ch) => input = ch,
            Err(err) => {
                error_msg =  err.to_string();
                continue;
            }
        }
        //validate input
        if input >= '0' && input <= '8'{
            let input = (input as usize - '0' as usize) as usize;
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
                error_msg = "Internal error".to_string();
                panic!();
            }
        } else {
            error_msg = "Invalid choice".to_string();
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
