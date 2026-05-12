//! This module contains the definition of the Player trait, alongside implementations of various CPU players.

use crate::game::{
    Mark, {Board, Position},
};

mod minimax;
mod random;
mod tui;

pub use minimax::Minimax;
pub use random::Random;
pub use tui::TuiPlayer;

/// A `Player` represents an agent in the game capable of choosing their next move.
pub trait Player {
    /// Choose a move based on the current state of the board.
    /// Should always be called with a board that has at least one available move.
    fn choose_move(&mut self, board: &Board) -> Position;

    /// Retrieve the mark that this strategy uses (X or O).
    fn get_mark(&self) -> Mark;
}
