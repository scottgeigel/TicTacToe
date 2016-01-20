use std::io;
use std::io::prelude::*;
mod tic_tac_toe;
mod ui;

const VERSION : &'static str = "1.0.0";

struct Game {
    pub /*temporary*/board : tic_tac_toe::Board,
    pub /*temporary*/current_player : u8,
    pub /*temporary*/error_msg : String,
    player_1_name : String,
    player_2_name : String,
}

impl Game {
    fn new(num_players : u8) -> Game {
        assert!(num_players <= 2);
        Game {
            board : tic_tac_toe::Board::new(),
            current_player : 1,
            error_msg : "".to_string(),
            player_1_name : "Adolfo".to_string(),
            player_2_name : "Hernandez".to_string(),
        }
    }

    pub fn set_player_1_name(&mut self, name : String) {
        self.player_1_name = name
    }

    pub fn set_player_2_name(&mut self, name : String) {
        self.player_2_name = name
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
        let mut ret = "Player ".to_string() + &self.current_player.to_string() + &"'s";
        match self.current_player {
            1 => {
                if self.player_1_name.len() > 0 {
                    ret = ret + " (" + &self.player_1_name + ") ";
                }
            },
            2 => {
                if self.player_2_name.len() > 0 {
                    ret = ret + " (" + &self.player_2_name + ") ";
                }
            },
            _ => panic!(),
        }
        ret = ret + &"turn\n".to_string() + &self.board.to_string() + &"\n".to_string() +
            &self.error_msg + &"\n".to_string();
        return ret;
    }

    pub fn set_error(&mut self, message : String) {
        self.error_msg = message
    }

    pub fn clear_error(&mut self) {
        self.error_msg = "".to_string()
    }
}

fn main() {
    println!("Tic Tac Toe version {}", VERSION);

    let mut game : Game = Game::new(1);
    let mut line : String = String::new();
    //TODO: add ui::user_prompt_input
    print!("Player 1's name: ");
    io::stdout().flush().unwrap();

    ui::get_user_input(&mut line).unwrap();
    game.set_player_1_name(line.trim().to_string());
    println!("Hello {}", line);
    //TODO: add ui::user_prompt_input
    print!("Player 2's name: ");
    io::stdout().flush().unwrap();

    ui::get_user_input(&mut line).unwrap();
    game.set_player_2_name(line.trim().to_string());
    println!("Hello {}", line);

    loop {
        ui::clear_screen();
        //display the board
        ui::display(game.to_string());
        game.clear_error();

        //TODO: add ui::user_prompt_selection
        println!("Player {}, make your selection", game.current_player);

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
        if input >= '0' && input <= '8' {
            let input = (input as usize - '0' as usize) as usize;
            if game.make_move(input) {
                match game.board.eval() {
                    Some(n) => {
                        match n {
                            tic_tac_toe::GameResult::XWin => println!("X wins"),
                            tic_tac_toe::GameResult::OWin => println!("O wins"),
                            tic_tac_toe::GameResult::Draw => println!("You're all losers"),
                        };
                        break;
                    },
                    None => game.next_turn(),
                }
            }
        } else {
            game.set_error("Invalid choice".to_string());
        }
    }
}
