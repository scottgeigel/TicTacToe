const squares : usize = 9;
#[derive(Copy, Clone)]
pub enum Square {
    X,
    O,
    NotSet,
}

pub enum GameResult {
    XWin,
    YWin,
    Draw
}

pub struct Board {
    board : [Square; squares]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board : [Square::NotSet; squares]
        }
    }

    pub fn ToString(&self) -> String {
        let mut ret : String = String::new();
        for y in 0..3 {
            for x in 0..3 {
                let index = (3 * y) + x;
                let test = match self.board[index] {
                    Square::X => 'X',
                    Square::O => 'O',
                    Square::NotSet => (index as u8 + ('0' as u8)) as char,
                };
                print!("{} ", test);
            }
            println!("");
        }
        return ret;
    }

    fn PlaceToken(&mut self, index : usize, val : Square) -> bool{
        assert!(index < squares);
        match self.board[index] {
            Square::NotSet => {
                self.board[index] = val;
                return true;
            },
            _ => return false,
        }
    }

    pub fn PlaceX(&mut self, index : usize) -> bool {
        self.PlaceToken(index, Square::X)
    }

    pub fn PlaceO(&mut self, index : usize) -> bool {
        self.PlaceToken(index, Square::O)
    }

}

pub fn ToIndex(index : usize) -> Result<usize, String>
{
    if index <= squares {
        Ok(index)
    }
    else {
        Err("out of range".to_string())
    }
}
