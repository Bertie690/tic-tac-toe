use crate::{game::{Mark, Position}, player::Player};
use std::sync::mpsc;

/// A `TuiPlayer` represents a human player interacting through the TUI.
/// 
/// It receives the player's chosen move from the TUI thread via a channel.
pub struct TuiPlayer {
    /// The mark this player is using (X or O).
    mark: Mark,
    /// The receiving end of the move channel, used to receive the player's chosen move from the TUI thread.
    move_rx: mpsc::Receiver<Position>,
}

impl TuiPlayer {
    /// Create a new TUI player with the given mark and channel.
    ///
    /// It is the caller's responsibility to pass the sending end to the TUI thread's [`run`] method.
    ///
    /// [`run`]: crate::renderer::TuiRenderer::run
    pub fn new(mark: Mark, move_rx: mpsc::Receiver<Position>) -> Self {
        Self { mark, move_rx }
    }
}

impl Player for TuiPlayer {
    fn choose_move(&mut self, _board: &crate::game::Board) -> Position {
        // block until the TUI thread sends a move
        self.move_rx.recv().expect("TUI thread shouldn't disconnect mid-game")
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}