const SQUARES : usize = 9;
#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    X,
    O,
    NotSet,
}

pub enum GameResult {
    XWin,
    OWin,
    Draw
}

pub struct Board {
    board : [Square; SQUARES]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board : [Square::NotSet; SQUARES]
        }
    }

    //match against:
    //0 1 2
    //3 4 5
    //6 7 8
    pub fn eval(&self) -> Option<GameResult> {
        //verify whether all squares have been for draw detection
        let mut count = 0;
        for square in &self.board {
            if *square != Square::NotSet {
                count = count + 1;
            }
        }
        let full : bool = count == SQUARES;

        //check center square
        let mut winner : Square = Square::NotSet;
        if self.board[4] != Square::NotSet {
            //center square set, check possible winners
            let potential = self.board[4];
            //check vertical-middle
            if self.board[1] == potential && self.board[7] == potential {
                winner = potential;
            }//check horizontal-middle
            else if self.board[3] == potential && self.board[5] == potential {
                winner = potential;
            }//check diagonal going left
            else if self.board[0] == potential && self.board[8] == potential {
                winner = potential;
            } //check diagonal going right
            else if self.board[2] == potential && self.board[6] == potential {
                winner = potential;
            }
            //else no winner found
        }//check top-left square
        else if self.board[0] != Square::NotSet {
            let potential = self.board[0];
            //check horizontal
            if self.board[1] == potential && self.board[2] == potential {
                winner = potential;
            }//check vertical
            else if self.board[3] == potential && self.board[6] == potential {
                winner = potential;
            }
        }//check bottom-right square
        else if self.board[8] != Square::NotSet {
            let potential = self.board[8];
            //check horizontal
            if self.board[6] == potential && self.board[7] == potential {
                winner = potential;
            }//check vertical
            else if self.board[2] == potential && self.board[5] == potential {
                winner = potential;
            }
        }

        match winner {
            Square::X => Some(GameResult::XWin),
            Square::O => Some(GameResult::OWin),
            Square::NotSet => if full {
                Some(GameResult::Draw)
            } else {
                None
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = "".to_string();
        for y in 0..3 {
            for x in 0..3 {
                let index = (3 * y) + x;
                let test = match self.board[index] {
                    Square::X => ret = ret + "X",
                    Square::O => ret = ret + "O",
                    Square::NotSet => ret = ret + &index.to_string(),
                };
            }
            ret = ret + "\n";
        }
        return ret;
    }

    fn place_token(&mut self, index : usize, val : Square) -> bool{
        assert!(index < SQUARES);
        match self.board[index] {
            Square::NotSet => {
                self.board[index] = val;
                return true;
            },
            _ => return false,
        }
    }

    pub fn place_x(&mut self, index : usize) -> bool {
        self.place_token(index, Square::X)
    }

    pub fn place_o(&mut self, index : usize) -> bool {
        self.place_token(index, Square::O)
    }

}

pub fn to_index(index : usize) -> Result<usize, String>
{
    if index <= SQUARES {
        Ok(index)
    }
    else {
        Err("out of range".to_string())
    }
}
