use super::Player;

use crate::game::{
    board::{Board, Position},
    mark::Mark,
    result::GameResult,
};

/// A `Minimax` represents a strategy determined by the minimax algorithm.
/// It attempts to recursively evaluate the game tree to find the optimal move for the current player
/// (assuming optimal play from the opponent).
pub struct Minimax {
    mark: Mark,
}

impl Strategy for Minimax {
    /// choose_move implements the minimax algorithm using alpha-beta pruning.
    fn choose_move(&self, board: &Board) -> Position {
        find_best_move(board, self.mark)
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

fn minimax_move(board: &Board, mark: Mark) {}
