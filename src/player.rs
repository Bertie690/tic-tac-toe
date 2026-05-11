//! This module contains the definition of the Player trait, alongside implementations of various CPU players.

use crate::game::{
    board::{Board, Position},
    mark::Mark,
};

mod tui;
mod minimax;
mod random;

/// A `Player` represents an agent in the game capable of choosing their next move.
pub trait Player {
    /// Choose a move based on the current state of the board.
    /// Should always be called with a board that has at least one available move.
    fn choose_move(&mut self, board: &Board) -> Position;

    /// Retrieve the mark that this strategy uses (X or O).
    fn get_mark(&self) -> Mark;
}
