use std::io;
use super::board;

pub fn decide_move(board : board::Board) {
    let mut moves = board.get_iterator();
    for m in moves {
        println!("{}", m);
    }
}
