use std::io;
use board;
pub fn decide_move(board : tic_tac_toe::Board) {
    let mut moves = board.get_iterator();
    for m in moves {
        println!("{}", m);
    }
}
