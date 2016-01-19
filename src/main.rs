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
            Ok(_) => {
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
        io::stdout().write(&cmd).unwrap();
    }

    pub fn display(text : String) {
        clear_screen();
        io::stdout().write(text.as_bytes()).unwrap();
    }
}

struct Game {
    pub /*temporary*/board : tic_tac_toe::Board,
    pub /*temporary*/current_player : u8,
    pub /*temporary*/error_msg : String,
}

impl Game {
    fn new(num_players : u8) -> Game {
        assert!(num_players <= 2);
        Game {
            board : tic_tac_toe::Board::new(),
            current_player : 1,
            error_msg : "".to_string(),
        }
    }

    pub fn make_move(&mut self, choice : usize) -> bool{
        match self.current_player {
            1 => return self.board.place_x(choice),
            2 => return self.board.place_o(choice),
            _ => {
                println!("unexpected player {}", self.current_player);
                panic!();
            },
        }
    }

    pub fn next_turn(&mut self) {
        match self.current_player {
            1 => self.current_player = 2,
            2 => self.current_player = 1,
            _ => panic!(),
        }
    }

    pub fn to_string(&self) -> String {
        "Player ".to_string() +
        &self.current_player.to_string() +
        &"'s turn\n".to_string() +
        &self.board.to_string() +
        &"\n".to_string() +
        &self.error_msg + &"\n".to_string()
    }

    pub fn set_error(&mut self, message : String) {
        self.error_msg = message
    }

    pub fn clear_error(&mut self) {
        self.error_msg = "".to_string()
    }
}

fn main() {
    let mut game : Game = Game::new(1);
    print!("Player 1's name: ");
    let mut line : String = String::new();
    io::stdout().flush().unwrap();
    ui::get_user_input(&mut line).unwrap();
    println!("Hello {}", line);
    loop {
        ui::clear_screen();
        //display the board
        ui::display(game.to_string());
        println!("Player {}, make your selection", game.current_player);

        game.clear_error();
        let input_result = ui::get_user_selection();
        let input;
        match input_result {
            Ok(ch) => input = ch,
            Err(err) => {
                game.set_error(err.to_string());
                continue;
            }
        }
        //validate input
        if input >= '0' && input <= '8'{
            let input = (input as usize - '0' as usize) as usize;
            if game.make_move(input) {
                game.next_turn();
            }
        } else {
            game.set_error("Invalid choice".to_string());
        }

        match game.board.eval() {
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
