
#[derive(Clone)]
pub struct Game {
    pub /*temporary*/board : super::board::Board,
    pub /*temporary*/current_player : super::player::PlayerNumber,
    pub /*temporary*/error_msg : String,
    pub player_1 : super::player::Player,
    pub player_2 : super::player::Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board : super::board::Board::new(),
            error_msg : "".to_string(),
            current_player :super::player:: PlayerNumber::PlayerX,
            player_1 : super::player::Player::new("Franscesco".to_string(), super::player::PlayerNumber::PlayerX),
            player_2 : super::player::Player::new("Ramirez".to_string(), super::player::PlayerNumber::PlayerO),
        }
    }

    pub fn make_move(&mut self) -> bool{
        let mut game : Game = self.clone();
        match self.current_player {
            super::player::PlayerNumber::PlayerX => {
                let choice = self.player_1.make_move(&mut game);
                if choice < super::board::SQUARES {
                    return self.board.place_x(choice);
                }
                else {
                    return false;
                }
             },
            super::player::PlayerNumber::PlayerO => {
                let choice = self.player_2.make_move(&mut game);
                if choice < super::board::SQUARES {
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
            super::player::PlayerNumber::PlayerX => self.current_player = super::player::PlayerNumber::PlayerO,
            super::player::PlayerNumber::PlayerO => self.current_player = super::player::PlayerNumber::PlayerX,
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = "Player ".to_string() + &self.current_player.to_string() + &"'s";
        match self.current_player {
            super::player::   PlayerNumber::PlayerX => {
                if self.player_1.name.len() > 0 {
                    ret = ret + " (" + &self.player_1.name + ") ";
                }
            },
            super::player::PlayerNumber::PlayerO => {
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

pub struct GameFactory {
    product : Game,
}
impl GameFactory {
    pub fn new() -> GameFactory {
        GameFactory {
            product : Game::new(),
        }
    }
    pub fn make_player_1_ai(&mut self) {
            self.product.player_1.make_ai()
    }

    pub fn set_player_1_name(&mut self, name : String) {
        self.product.player_1.name = name.clone()
    }

    pub fn make_player_2_ai(&mut self) {
        self.product.player_2.make_ai()
    }
    pub fn set_player_2_name(&mut self, name : String) {
        self.product.player_2.name = name.clone()
    }

    pub fn finalize(&mut self) -> Game{
        return self.product.clone();
    }
}
