extern crate rand;
use std::io;
use std::io::prelude::*;
mod tic_tac_toe;
mod ui;
const VERSION : &'static str = "1.0.0";
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerNumber{
    PlayerX,
    PlayerO,
}

#[derive(Clone)]
pub enum PlayerMode {
    Human,
    AI,
}
impl PlayerNumber {
    pub fn to_string(&self) -> String {
        match *self {
            PlayerNumber::PlayerX => return "X".to_string(),
            PlayerNumber::PlayerO => return "O".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub name : String,
    pub mode : PlayerMode,
    pub player_number : PlayerNumber,
    pub handicap : i16,
}

impl Player {
    fn pc_make_move(&self, game : &mut Game) -> usize {
        let input_result = ui::get_user_selection();
        let input;
        match input_result {
            Ok(ch) => input = ch,
            Err(err) => {
                game.set_error(err.to_string());
                return 10; //out of range number
            }
        }
        //validate input
        if input >= '0' && input <= '8' {
            return (input as usize - '0' as usize) as usize;
        } else {
            game.set_error("Invalid choice".to_string());
            return 10
        }
    }

    pub fn make_move(&self, game :&Game) -> usize {
        match self.mode {
            PlayerMode::Human => self.pc_make_move(&mut game.clone()),
            PlayerMode::AI => tic_tac_toe::ai::decide_move(&mut game.clone()),
        }
    }
    fn new (name : String, player_num : PlayerNumber) -> Player {
        Player {
            name : name,
            mode :  PlayerMode::Human,
            player_number : player_num,
            handicap : 100,
        }
    }

    fn make_ai(&mut self) {
        self.mode = PlayerMode::AI
    }
}

#[derive(Clone)]
pub struct Game {
    pub /*temporary*/board : tic_tac_toe::board::Board,
    pub /*temporary*/current_player : PlayerNumber,
    pub /*temporary*/error_msg : String,
    player_1 : Player,
    player_2 : Player,
}
/*
impl Clone for Game {
    fn clone(&self) -> Game {
        Game {
            board : self.board.clone(),
            current_player : self.current_player.clone(),
            error_msg : "".to_string(),
            player_1 : self.player_1.clone(),
            player_2 : self.player_2.clone(),
        }
    }
}*/

impl Game {
    fn new() -> Game {
        Game {
            board : tic_tac_toe::board::Board::new(),
            error_msg : "".to_string(),
            current_player : PlayerNumber::PlayerX,
            player_1 : Player::new("Franscesco".to_string(), PlayerNumber::PlayerX),
            player_2 : Player::new("Ramirez".to_string(), PlayerNumber::PlayerO),
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
            PlayerNumber::PlayerX => {
                let choice = self.player_1.make_move(&mut game);
                if choice < tic_tac_toe::board::SQUARES {
                    return self.board.place_x(choice);
                }
                else {
                    return false;
                }
             },
            PlayerNumber::PlayerO => {
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
            PlayerNumber::PlayerX => self.current_player = PlayerNumber::PlayerO,
            PlayerNumber::PlayerO => self.current_player = PlayerNumber::PlayerX,
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = "Player ".to_string() + &self.current_player.to_string() + &"'s";
        match self.current_player {
            PlayerNumber::PlayerX => {
                if self.player_1.name.len() > 0 {
                    ret = ret + " (" + &self.player_1.name + ") ";
                }
            },
            PlayerNumber::PlayerO => {
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
    println!("Tic Tac Toe version {}", VERSION);

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
