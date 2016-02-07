extern crate rand;
use std::io;
use std::io::prelude::*;
mod tic_tac_toe;
mod ui;

#[derive(Clone)]
pub struct Game {
    pub /*temporary*/board : tic_tac_toe::board::Board,
    pub /*temporary*/current_player : tic_tac_toe::player::PlayerNumber,
    pub /*temporary*/error_msg : String,
    player_1 : tic_tac_toe::player::Player,
    player_2 : tic_tac_toe::player::Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board : tic_tac_toe::board::Board::new(),
            error_msg : "".to_string(),
            current_player :tic_tac_toe::player:: PlayerNumber::PlayerX,
            player_1 : tic_tac_toe::player::Player::new("Franscesco".to_string(), tic_tac_toe::player::PlayerNumber::PlayerX),
            player_2 : tic_tac_toe::player::Player::new("Ramirez".to_string(), tic_tac_toe::player::PlayerNumber::PlayerO),
        }
    }

    pub fn make_player_1_ai(&mut self) {
            self.player_1.make_ai()
    }

    pub fn set_player_1_name(&mut self, name : String) {
        self.player_1.name = name.clone()
    }

    pub fn make_player_2_ai(&mut self) {
        self.player_2.make_ai()
    }
    pub fn set_player_2_name(&mut self, name : String) {
        self.player_2.name = name.clone()
    }

    pub fn make_move(&mut self) -> bool{
        let mut game : Game = self.clone();
        match self.current_player {
            tic_tac_toe::player::PlayerNumber::PlayerX => {
                let choice = self.player_1.make_move(&mut game);
                if choice < tic_tac_toe::board::SQUARES {
                    return self.board.place_x(choice);
                }
                else {
                    return false;
                }
             },
            tic_tac_toe::player::PlayerNumber::PlayerO => {
                let choice = self.player_2.make_move(&mut game);
                if choice < tic_tac_toe::board::SQUARES {
                    return self.board.place_o(choice);
                }
                else {
                    return false;
                }
             },
        }
    }

    pub fn next_turn(&mut self) {
        match self.current_player {
            tic_tac_toe::player::PlayerNumber::PlayerX => self.current_player = tic_tac_toe::player::PlayerNumber::PlayerO,
            tic_tac_toe::player::PlayerNumber::PlayerO => self.current_player = tic_tac_toe::player::PlayerNumber::PlayerX,
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = "Player ".to_string() + &self.current_player.to_string() + &"'s";
        match self.current_player {
            tic_tac_toe::player::   PlayerNumber::PlayerX => {
                if self.player_1.name.len() > 0 {
                    ret = ret + " (" + &self.player_1.name + ") ";
                }
            },
            tic_tac_toe::player::PlayerNumber::PlayerO => {
                if self.player_2.name.len() > 0 {
                    ret = ret + " (" + &self.player_2.name + ") ";
                }
            },
        }
        ret = ret + &" turn\n".to_string() + &self.board.to_string() + &"\n".to_string() +
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
    let mut game : Game = Game::new();
    let mut line : String = String::new();

    //player 1 config loop
    //TODO: refactor this out
    {
        let mut invalid = false;
        loop
        {
            ui::clear_screen();
            if invalid {
                println!("Invalid choice");
            }
            println!("Tic Tac Toe version {}", env!("CARGO_PKG_VERSION"));
            print!("Is Player 1 an AI?\n(y/n): ");
            io::stdout().flush().unwrap();
            match ui::get_user_selection() {
                Ok(choice) => {
                    if choice == 'y' {
                        game.make_player_1_ai();
                        break;
                    }
                    else if choice == 'n' {
                        //players default as human
                        break;
                    }
                    else {
                        invalid = true;
                    }
                },
                Err(_) => {
                    invalid = false;
                },
            }
        }
    }

    print!("Player 1's name: ");
    io::stdout().flush().unwrap();

    ui::get_user_input(&mut line).unwrap();
    game.set_player_1_name(line.trim().to_string());

        //player 2 config loop
        //TODO: refactor this out
        {
            let mut invalid = false;
            loop
            {
                ui::clear_screen();
                if invalid {
                    println!("Invalid choice");
                }
                print!("Is Player 2 an AI?\n(y/n): ");
                io::stdout().flush().unwrap();
                match ui::get_user_selection() {
                    Ok(choice) => {
                        if choice == 'y' {
                            game.make_player_2_ai();
                            break;
                        }
                        else if choice == 'n' {
                            //players default as human
                            break;
                        }
                        else {
                            invalid = true;
                        }
                    },
                    Err(_) => {
                        invalid = false;
                    },
                }
            }
        }

    print!("Player 2's name: ");
    io::stdout().flush().unwrap();

    ui::get_user_input(&mut line).unwrap();
    game.set_player_2_name(line.trim().to_string());

    loop {
        ui::clear_screen();
        //display the board
        ui::display(game.to_string());
        game.clear_error();

        //TODO: add ui::user_prompt_selection
        println!("Player {:?}, make your selection", game.current_player);

        if game.make_move() {
            match game.board.eval() {
                Some(n) => {
                    //TODO: temp patch to make board draw at the end. I don't want it like this long-term
                    ui::clear_screen();
                    ui::display(game.to_string());
                    game.clear_error();
                    //display the board
                    match n {
                        tic_tac_toe::board::GameResult::XWin => println!("X wins"),
                        tic_tac_toe::board::GameResult::OWin => println!("O wins"),
                        tic_tac_toe::board::GameResult::Draw => println!("You're all losers"),
                    };
                    break;
                },
                None => game.next_turn(),
            }
        }
        else {
            game.set_error("Inalid selection".to_string());
        }
    }
}
