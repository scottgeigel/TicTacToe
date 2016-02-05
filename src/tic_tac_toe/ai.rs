use std::io;
use super::board;

fn score(game : &super::super::Game, ai_player : super::super::PlayerNumber, n : super::board::GameResult) -> i8 {
    match n {
        super::board::GameResult::XWin => {
            match ai_player {
                super::super::PlayerNumber::PlayerX => 1,
                super::super::PlayerNumber::PlayerO => -1,
            }
        },
        super::board::GameResult::OWin => {
            match ai_player {
                super::super::PlayerNumber::PlayerO => 1,
                super::super::PlayerNumber::PlayerX => -1,
            }
        },
        super::board::GameResult::Draw => {
            return 0;
        }
    }
}

fn minmax(game : &super::super::Game, ai_player : super::super::PlayerNumber, active_turn : bool) -> (i8, usize) {
    //if game over, return the score
    match game.board.eval() {
        Some(n) => {
            let winner = n;
            return (score(game, ai_player, winner), 10);
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
                let current_player = temp_game.current_player.clone();
                match current_player {
                    super::super::PlayerNumber::PlayerX => {
                        if !temp_game.board.place_x(m) {
                            panic!();
                        }
                    }
                    super::super::PlayerNumber::PlayerO => {
                        if !temp_game.board.place_o(m) {
                            panic!();
                        }
                    }
                }
                temp_game.next_turn();
                let(result, choice) = minmax(&temp_game, ai_player, !active_turn);
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

            if active_turn {
                //maximize
                //mix it up a bit
                if greatest_choices.len() > 1 {
                    return (greatest, greatest_choices[greatest_choice % greatest_choices.len()]);
                }
                return (greatest, greatest_choice);
            }
            else {
                //minimize
                return (smallest, smallest_choice);
            }
        },
    }
    panic!();
}

pub fn decide_move(game : &mut super::super::Game) -> usize  {
    let (result, choice) = minmax(game, game.current_player, true);
    return choice;
}
