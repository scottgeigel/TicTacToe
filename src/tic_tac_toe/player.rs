use super::super::Game;//super::game::Game;
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
    pub fn pc_make_move(&self, game : &mut Game) -> usize {
        let input_result = super::super::ui::get_user_selection();
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
            PlayerMode::AI => super::ai::decide_move(&mut game.clone()),
        }
    }
    pub fn new (name : String, player_num : PlayerNumber) -> Player {
        Player {
            name : name,
            mode :  PlayerMode::Human,
            player_number : player_num,
            handicap : 1,
        }
    }

    pub fn make_ai(&mut self) {
        self.mode = PlayerMode::AI
    }
}
