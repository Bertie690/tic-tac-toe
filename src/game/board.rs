use crate::game::mark::Mark;
use crate::game::result::GameResult;
use ndarray::{Array2, ArrayView2};

pub type Position = (usize, usize);

/// A Board represents a 3x3 tic-tac-toe grid.
// TODO: Generalize to an NxN grid
pub struct Board {
    /// The underlying square grid of cells.
    grid: Array2<Option<Mark>>,

    /// The turn player.
    turn: Mark,
}

impl Board {
    /// Create a new 3x3 board.
    pub fn new() -> Self {
        Board {
            grid: Array2::from_elem((3, 3), None),
            turn: Mark::X,
        }
    }

    /// Get a readonly view of the current grid.
    pub fn grid(&self) -> ArrayView2<Option<Mark>> {
        self.grid.view()
    }

    /// Set the mark at the given position.
    pub fn set(&mut self, (row, col): (usize, usize), mark: Mark) {
        self.grid[(row, col)] = Some(mark);
        self.turn = match self.turn {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
        };
    }

    /// Check for the current state of the game.
    pub fn state(&self) -> GameResult {
        // Check for a winner in rows, columns, and diagonals.
        let lines = self
            .grid
            .rows()
            .into_iter()
            .chain(self.grid.columns().into_iter())
            .chain(std::iter::once(self.grid.diag()))
            .chain(std::iter::once(self.grid.diag().reversed_axes()));

        for line in lines {
            if let Some(mark) = line.iter().cloned().flatten().next() {
                if line.iter().all(|&cell| cell == Some(mark)) {
                    return GameResult::Winner(mark);
                }
            }
        }

        // Check for a draw or ongoing game.
        if self.is_full() {
            GameResult::Draw
        } else {
            GameResult::Ongoing
        }
    }

    /// Check whether the board is currently full.
    pub fn is_full(&self) -> bool {
        self.grid.iter().all(|cell| cell.is_some())
    }
}
