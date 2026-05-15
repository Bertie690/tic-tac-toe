use crate::game::mark::Mark;
use crate::game::result::GameResult;
use ndarray::{Array2, ArrayView2};

/// An (x, y) position in the playing field.
pub type Position = (usize, usize);

/// A Board represents a 3x3 tic-tac-toe grid.
// TODO: Generalize to an NxN grid
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The underlying square grid of cells.
    grid: Array2<Option<Mark>>,
}

impl Board {
    /// Create a new 3x3 board with the given contents.
    ///
    /// **Panics** if the contents are not 3x3.
    pub fn new(contents: Array2<Option<Mark>>) -> Self {
        if contents.shape() != [3, 3] {
            panic!(
                "Board must be initialized with a 3x3 array, got shape {:?}",
                contents.shape()
            );
        }
        Board { grid: contents }
    }

    /// Get a readonly view of the current grid.
    pub fn grid(&self) -> ArrayView2<'_, Option<Mark>> {
        self.grid.view()
    }

    /// Set the mark at the given position.
    /// Does not perform any error checking for whether the move is valid.
    pub fn set_mark(&mut self, (row, col): Position, mark: Mark) {
        // array is indexed by (row, col)
        self.grid[(row, col)] = Some(mark);
    }

    /// Check for the current state of the game: win, loss or draw.
    ///
    /// If multiple win conditions are satisfied, the return value is unspecified
    /// (as this signifies an invalid board state).
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

        if self.is_full() {
            GameResult::Draw
        } else {
            GameResult::Ongoing
        }
    }

    /// Validate whether a move is legal given the current board state.
    pub fn validate_board_state(&self, pos: Position) -> Option<anyhow::Error> {
        let (row, col) = pos;
        // index must be in range & unoccupied
        if row >= self.grid.nrows() || col >= self.grid.ncols() {
            return Some(anyhow::anyhow!("Position {:?} is out of bounds", pos));
        }
        if self.grid[(row, col)].is_some() {
            return Some(anyhow::anyhow!("Position {:?} is already occupied", pos));
        }

        None
    }

    /// Check whether the board is currently full.
    fn is_full(&self) -> bool {
        self.grid.iter().all(|cell| cell.is_some())
    }

    pub fn available_moves(&self) -> impl Iterator<Item = Position> + '_ {
        self.grid.indexed_iter().filter_map(|((row, col), cell)| {
            if cell.is_none() {
                Some((row, col))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{mark::Mark, result::GameResult};
    use ndarray::array;

    #[test]
    fn test_board_state_x_wins() {
        let board = Board::new(array![
            [Some(Mark::X), Some(Mark::X), Some(Mark::X)],
            [Some(Mark::O), Some(Mark::O), None],
            [None, None, None],
        ]);
        assert_eq!(board.state(), GameResult::Winner(Mark::X));
    }
    #[test]
    fn test_board_state_x_wins_column() {
        let board = Board::new(array![
            [Some(Mark::X), Some(Mark::O), Some(Mark::O)],
            [Some(Mark::X), None, None],
            [Some(Mark::X), None, None],
        ]);
        assert_eq!(board.state(), GameResult::Winner(Mark::X));
    }
    #[test]
    fn test_board_state_o_wins() {
        let board = Board::new(array![
            [Some(Mark::O), Some(Mark::X), Some(Mark::O)],
            [Some(Mark::X), Some(Mark::O), None],
            [Some(Mark::X), Some(Mark::X), Some(Mark::O)],
        ]);
        assert_eq!(board.state(), GameResult::Winner(Mark::O));
    }
    #[test]
    fn test_board_state_draw() {
        let board = Board::new(array![
            [Some(Mark::X), Some(Mark::O), Some(Mark::X)],
            [Some(Mark::O), Some(Mark::X), Some(Mark::O)],
            [Some(Mark::O), Some(Mark::X), Some(Mark::O)],
        ]);
        assert_eq!(board.state(), GameResult::Draw);
    }
    #[test]
    fn test_board_state_ongoing() {
        let board = Board::new(array![
            [Some(Mark::X), Some(Mark::O), Some(Mark::X)],
            [Some(Mark::O), Some(Mark::X), Some(Mark::O)],
            [Some(Mark::O), Some(Mark::X), Some(Mark::O)],
        ]);
        assert_eq!(board.state(), GameResult::Draw);
    }

    #[test]
    fn test_board_available_moves() {
        let board = Board::new(array![
            [Some(Mark::X), None, Some(Mark::O)],
            [None, None, None],
            [Some(Mark::X), None, None],
        ]);
        let moves: Vec<Position> = board.available_moves().collect();
        assert_eq!(moves, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1), (2, 2)]);
    }
}
