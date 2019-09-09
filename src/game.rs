use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
  Alive,
  Dead
}

#[derive(Debug, PartialEq)]
pub enum GameState {
  New,
  Running,
  Stalemated,
  Extinct
}

pub type GameBoard = Vec<Vec<Cell>>;

pub struct Game {
  pub board: GameBoard,
  pub state: GameState,
  pub iteration: usize
}

impl Game {
  pub fn new(x: usize, y: usize) -> Game {
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
      board,
      state: GameState::New,
      iteration: 0
    }
  }

  pub fn from_board(board: GameBoard) -> Game {
    Game {
      board,
      state: GameState::New,
      iteration: 0
    }
  }

  pub fn next(&mut self) {

    match self.state {
      GameState::Stalemated | GameState::Extinct => return,
      _ => {}
    };
    
    // We must calculate the state of the next board 
    // Without modifying the current state.

    let board_len = self.board.len();

    let mut r = Vec::new();
    let mut has_changed = false;
    let mut all_blank = true;
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

        let next_state = match self.board[i][j] {
          Cell::Alive if c < 2 => Cell::Dead,
          Cell::Alive if (c == 2 || c == 3) => Cell::Alive,
          Cell::Alive => Cell::Dead,
          Cell::Dead if c == 3 => Cell::Alive,
          _ => Cell::Dead
        };

        if next_state != self.board[i][j] {
          has_changed = true;
        }
        if next_state == Cell::Alive {
          all_blank = false;
        }

        b.push(next_state);
      }
      r.push(b);
    }

    self.state = match self.state {
      GameState::New | GameState::Running if all_blank => GameState::Extinct,
      GameState::New | GameState::Running if !has_changed => GameState::Stalemated,
      GameState::New => GameState::Running,
      _ => GameState::Running
    };

    self.iteration += 1;

    self.board = r;
  }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "Game has been running for {} iterations, and is currently: {}", self.iteration, match self.state {
      GameState::New => "Freshly created",
      GameState::Running => "Running",
      GameState::Extinct => "Extinct",
      GameState::Stalemated => "Stalemated"
    })?;

    writeln!(f, "{}", std::iter::repeat('=').take(self.board.len()).collect::<String>())?;
    for r in self.board.iter() {
      writeln!(f, "{}", r.iter().map(|v| match v { Cell::Alive => '#', _ => ' '}).collect::<String>())?;
    }
    writeln!(f, "{}", std::iter::repeat('=').take(self.board.len()).collect::<String>())
  }
}