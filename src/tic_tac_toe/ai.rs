use rand;
use super::board;
use super::super::Player;
use super::super::PlayerNumber;
use super::super::Game;

struct MinMax<'a> {
    ai_player : &'a super::super::Player,
    active_turn : bool,
    depth : i16,
    choice : usize,
}
impl<'a> MinMax<'a> {
    fn new(_ai_player : &'a super::super::Player) -> MinMax {
        MinMax {
            ai_player : _ai_player,
            active_turn : true,
            depth : 0,
            choice : 10, //invalid option
        }
    }

    fn score(&mut self, game : &Game, n : board::GameResult) -> i16 {
        match n {
            board::GameResult::XWin | board::GameResult::OWin => {
                if self.ai_player.player_number == game.current_player {
                    return  1;
                }
                else {
                    return -1;
                }
            },
            board::GameResult::Draw => {
                return 0;
            }
        }
    }

    fn minmax(&mut self, game : &Game) -> i16 {
        //if game over, return the score
        match game.board.eval() {
            Some(n) => {
                let winner = n;
                return self.score(game, winner);
            },
            None => {
                let mut moves = game.board.get_iterator();
                let mut greatest_choices = Vec::new();
                let mut smallest_choices = Vec::new();
                let mut greatest = -1;
                let mut smallest = 1;
                let mut greatest_choice = 1234;
                let mut smallest_choice = 5678;

                for m in moves {
                    let mut temp_game = game.clone();
                    match temp_game.current_player {
                        PlayerNumber::PlayerX => {
                            if !temp_game.board.place_x(m) {
                                panic!();
                            }
                        }
                        PlayerNumber::PlayerO => {
                            if !temp_game.board.place_o(m) {
                                panic!();
                            }
                        }
                    }
                    //set up next branch
                    temp_game.next_turn();
                    self.active_turn = !self.active_turn;
                    self.depth += 1;
                    let result = self.minmax(&temp_game);
                    //reset for this branch
                    self.depth -= 1;
                    self.active_turn = !self.active_turn;
                    if result < smallest {
                        smallest_choice = m;
                        smallest = result;
                        smallest_choices.clear();
                        smallest_choices.push(m)
                    }
                    else if result == smallest {
                        smallest_choices.push(m)
                    }

                    if result > greatest {
                        greatest_choice = m;
                        greatest = result;
                        greatest_choices.clear();
                        greatest_choices.push(m)
                    }
                    else if result == greatest {
                        greatest_choices.push(m)
                    }
                }

                if self.active_turn {
                    //maximize
                    //mix it up a bit
                    if greatest_choices.len() > 1 {
                        self.choice = greatest_choices[rand::random::<usize>() % greatest_choices.len()];
                        return greatest;
                    }
                    assert!(greatest != 1234);
                    self.choice = greatest_choice;
                    return greatest;
                }
                else {
                    //minimize
                    //mix it up a bit
                    if smallest_choices.len() > 1 {
                        self.choice = smallest_choices[rand::random::<usize>() % smallest_choices.len()];
                        return smallest;
                    }
                    assert!(smallest != 5678);
                    self.choice = smallest_choice;
                    return smallest;
                }
            },
        }
        panic!();
    }
}

pub fn decide_move(game : &mut Game) -> usize  {
    let mut ai = MinMax::new(&game.player_1);
    ai.minmax(game);
    if ai.choice < 0 || ai.choice > 8 {
        panic!();
    }
    return ai.choice;
}
