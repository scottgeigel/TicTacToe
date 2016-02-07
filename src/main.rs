extern crate rand;
use std::io;
use std::io::prelude::*;
mod tic_tac_toe;
mod ui;
use tic_tac_toe::game::Game;
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
