use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
  Alive,
  Dead
}

pub type GameBoard = Vec<Vec<Cell>>;

#[derive(Debug)]
pub struct Game {
  pub board: GameBoard
}

impl Game {
  pub fn new(x: usize, y: usize) -> Game {
    if x < 2 || y < 2 {
      panic!("Board must be at least 2x2");
    }

    let mut board = Vec::new();
    board.reserve(x);

    for _ in 0..x {
      let mut r = Vec::new();
      r.reserve(y);

      for _ in 0..y {
        if rand::random() {
          r.push(Cell::Dead)
        } else {
          r.push(Cell::Alive)
        }
      }
      board.push(r);
    }

    Game {
      board
    }
  }

  pub fn from_board(board: GameBoard) -> Game {
    Game {
      board
    }
  }

  pub fn next(&mut self) {
    // We must calculate the state of the next board 
    // Without modifying the current state.

    let board_len = self.board.len();

    let mut r = Vec::new();
    r.reserve(board_len);

    for i in 0..board_len {
      let board_i_len = self.board[i].len();

      let mut b = Vec::new();
      b.reserve(board_i_len);

      for j in 0..board_i_len {
        let mut c = 0;

        if i != 0 {
          if j != 0 {
            if self.board[i-1][j-1] == Cell::Alive {
              c += 1;
            }
          }
          if j != board_i_len - 1 {
            if self.board[i-1][j+1] == Cell::Alive {
              c += 1;
            }
          }

          if self.board[i-1][j] == Cell::Alive {
            c += 1;
          }
        }

        if i != board_len - 1 {
          if j != 0 {
            if self.board[i+1][j-1] == Cell::Alive {
              c += 1;
            }
          }

          if j != board_i_len -1 { 
            if self.board[i+1][j+1] == Cell::Alive {
              c += 1;
            }
          }

          if self.board[i+1][j] == Cell::Alive {
            c += 1;
          }
        }

        if j != 0 {
          if self.board[i][j-1] == Cell::Alive {
            c += 1;
          }
        }
        if j != board_i_len - 1 {
          if self.board[i][j+1] == Cell::Alive {
            c += 1;
          }
        }

        b.push(match self.board[i][j] {
          Cell::Alive if c < 2 => Cell::Dead,
          Cell::Alive if (c == 2 || c == 3) => Cell::Alive,
          Cell::Alive => Cell::Dead,
          Cell::Dead if c == 3 => Cell::Alive,
          _ => Cell::Dead
        });
      }
      r.push(b);
    }

    self.board = r;
  }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}", std::iter::repeat('=').take(self.board.len()).collect::<String>())?;
    for r in self.board.iter() {
      writeln!(f, "{}", r.iter().map(|v| match v { Cell::Alive => '#', _ => ' '}).collect::<String>())?;
    }
    writeln!(f, "{}", std::iter::repeat('=').take(self.board.len()).collect::<String>())
  }
}