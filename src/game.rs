use core::fmt;
use core::iter::repeat;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(rayon)]
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    New,
    Running,
    Stalemated,
    Extinct,
}

pub type GameBoard = Vec<Vec<Cell>>;

pub struct Game {
    pub board: GameBoard,
    pub state: GameState,
    pub iteration: usize,
}

impl Game {
    #[cfg(rayon)]
    pub fn new(x: usize, y: usize) -> Game {
        Game {
            board: (0..x)
                .into_par_iter()
                .map(|_| {
                    (0..y)
                        .into_par_iter()
                        .map(|_| {
                            if rand::random() {
                                Cell::Dead
                            } else {
                                Cell::Alive
                            }
                        })
                        .collect()
                })
                .collect(),
            state: GameState::New,
            iteration: 0,
        }
    }

    #[cfg(not(rayon))]
    pub fn new(x: usize, y: usize) -> Game {
        Game {
            board: (0..x)
                .map(|_| {
                    (0..y)
                        .map(|_| {
                            if rand::random() {
                                Cell::Dead
                            } else {
                                Cell::Alive
                            }
                        })
                        .collect()
                })
                .collect(),
            state: GameState::New,
            iteration: 0,
        }
    }

    pub fn from_board(board: GameBoard) -> Game {
        Game {
            board,
            state: GameState::New,
            iteration: 0,
        }
    }

    #[cfg(rayon)]
    pub fn next(&mut self) {
        match self.state {
            GameState::Stalemated | GameState::Extinct => return,
            _ => {}
        };

        let board_len = self.board.len();

        let all_blank = AtomicBool::new(true);
        let has_changed = AtomicBool::new(false);

        self.board = self
            .board
            .par_iter()
            .enumerate()
            .map(|(i, r)| {
                let board_i_len = r.len();

                r.par_iter()
                    .enumerate()
                    .map(|(j, c)| {
                        let mut count = 0;

                        if i != 0 {
                            if j != 0 {
                                if self.board[i - 1][j - 1] == Cell::Alive {
                                    count += 1;
                                }
                            }
                            if j != board_i_len - 1 {
                                if self.board[i - 1][j + 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if self.board[i - 1][j] == Cell::Alive {
                                count += 1;
                            }
                        }

                        if i != board_len - 1 {
                            if j != 0 {
                                if self.board[i + 1][j - 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if j != board_i_len - 1 {
                                if self.board[i + 1][j + 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if self.board[i + 1][j] == Cell::Alive {
                                count += 1;
                            }
                        }

                        if j != 0 {
                            if self.board[i][j - 1] == Cell::Alive {
                                count += 1;
                            }
                        }
                        if j != board_i_len - 1 {
                            if self.board[i][j + 1] == Cell::Alive {
                                count += 1;
                            }
                        }

                        let next_state = match c {
                            Cell::Alive if count < 2 => Cell::Dead,
                            Cell::Alive if (count == 2 || count == 3) => Cell::Alive,
                            Cell::Alive => Cell::Dead,
                            Cell::Dead if count == 3 => Cell::Alive,
                            _ => Cell::Dead,
                        };

                        if next_state != *c {
                            has_changed.compare_and_swap(false, true, Ordering::Relaxed);
                        }

                        if next_state == Cell::Alive {
                            all_blank.compare_and_swap(true, false, Ordering::Relaxed);
                        }

                        next_state
                    })
                    .collect()
            })
            .collect();

        let all_blank = all_blank.load(Ordering::Relaxed);
        let has_changed = has_changed.load(Ordering::Relaxed);

        self.state = match self.state {
            GameState::New | GameState::Running if all_blank => GameState::Extinct,
            GameState::New | GameState::Running if !has_changed => GameState::Stalemated,
            GameState::New => GameState::Running,
            _ => GameState::Running,
        };

        self.iteration += 1;
    }

    #[cfg(not(rayon))]
    pub fn next(&mut self) {
        match self.state {
            GameState::Stalemated | GameState::Extinct => return,
            _ => {}
        };

        let board_len = self.board.len();

        let all_blank = AtomicBool::new(true);
        let has_changed = AtomicBool::new(false);

        self.board = self
            .board
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let board_i_len = r.len();

                r.iter()
                    .enumerate()
                    .map(|(j, c)| {
                        let mut count = 0;

                        if i != 0 {
                            if j != 0 {
                                if self.board[i - 1][j - 1] == Cell::Alive {
                                    count += 1;
                                }
                            }
                            if j != board_i_len - 1 {
                                if self.board[i - 1][j + 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if self.board[i - 1][j] == Cell::Alive {
                                count += 1;
                            }
                        }

                        if i != board_len - 1 {
                            if j != 0 {
                                if self.board[i + 1][j - 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if j != board_i_len - 1 {
                                if self.board[i + 1][j + 1] == Cell::Alive {
                                    count += 1;
                                }
                            }

                            if self.board[i + 1][j] == Cell::Alive {
                                count += 1;
                            }
                        }

                        if j != 0 {
                            if self.board[i][j - 1] == Cell::Alive {
                                count += 1;
                            }
                        }
                        if j != board_i_len - 1 {
                            if self.board[i][j + 1] == Cell::Alive {
                                count += 1;
                            }
                        }

                        let next_state = match c {
                            Cell::Alive if count < 2 => Cell::Dead,
                            Cell::Alive if (count == 2 || count == 3) => Cell::Alive,
                            Cell::Alive => Cell::Dead,
                            Cell::Dead if count == 3 => Cell::Alive,
                            _ => Cell::Dead,
                        };

                        if next_state != *c {
                            has_changed.compare_and_swap(false, true, Ordering::Relaxed);
                        }

                        if next_state == Cell::Alive {
                            all_blank.compare_and_swap(true, false, Ordering::Relaxed);
                        }

                        next_state
                    })
                    .collect()
            })
            .collect();

        let all_blank = all_blank.load(Ordering::Relaxed);
        let has_changed = has_changed.load(Ordering::Relaxed);

        self.state = match self.state {
            GameState::New | GameState::Running if all_blank => GameState::Extinct,
            GameState::New | GameState::Running if !has_changed => GameState::Stalemated,
            GameState::New => GameState::Running,
            _ => GameState::Running,
        };

        self.iteration += 1;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Game has been running for {} iterations, and is currently: {}",
            self.iteration,
            match self.state {
                GameState::New => "Freshly created",
                GameState::Running => "Running",
                GameState::Extinct => "Extinct",
                GameState::Stalemated => "Stalemated",
            }
        )?;

        writeln!(
            f,
            "{}",
            repeat('=').take(self.board.len()).collect::<String>()
        )?;
        for r in self.board.iter() {
            writeln!(
                f,
                "{}",
                r.iter()
                    .map(|v| match v {
                        Cell::Alive => '#',
                        _ => ' ',
                    })
                    .collect::<String>()
            )?;
        }
        writeln!(
            f,
            "{}",
            repeat('=').take(self.board.len()).collect::<String>()
        )
    }
}
