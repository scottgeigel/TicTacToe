use std::io;
use std::io::prelude::*;
mod TicTacToe;

fn main() {
    let mut input = io::stdin().bytes();
    let mut board : TicTacToe::Board = TicTacToe::Board::new();
    let mut player : u8 = 1;
    loop {
        //display the board
        println!("{}", board.ToString());
        println!("Player {}, make your selection", player);

        let input: Option<char> = input.next().and_then(|result| result.ok()).map(|byte| byte as char);
        match input {
            Some(ch) => {
                if ch == '\n' {
                    continue;
                }
            }
            None => println!("the end"),
        }
        let input = input.unwrap() as usize - ('0' as usize);
        //validate input

        if player == 1 {
            if !board.PlaceX(input) {
                continue;
            }
            player = 2;
        }
        else if player == 2 {
            if !board.PlaceO(input) {
                continue;
            }
            player = 1;
        }
        else {
            println!("Internal error");
            panic!();
        }
    }
}
