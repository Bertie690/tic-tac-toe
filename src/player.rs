//! This module contains the definition of the Player trait, alongside implementations of various CPU players.

use crate::game::{
    Mark, {Board, Position},
};

mod minimax;
mod random;
mod tui;

pub use minimax::Minimax;
pub use random::Random;
pub use tui::PlayerDisconnected;
pub use tui::TuiPlayer;

/// A `Player` represents an agent in the game capable of choosing their next move.
pub trait Player {
    /// Choose a move based on the current state of the board.
    ///
    /// Callers should return None if no valid moves can be made.
    fn choose_move(&mut self, board: &Board) -> anyhow::Result<Position>;

    /// Retrieve the mark that this strategy uses (X or O).
    fn get_mark(&self) -> Mark;
}
