use super::Player;
use crate::{
    game::{
        board::{Board, Position},
        mark::Mark,
    },
    renderer::Renderer,
};
use std::sync::mpsc;

// A `TuiPlayer` represents a human player interacting through a terminal user interface (TUI).
pub struct TuiPlayer {
    /// The mark (X or O) that this player uses.
    mark: Mark,

    /// A channel for receiving the player's chosen move from the TUI.
    move_rx: mpsc::Receiver<Position>,
}

impl Player for TuiPlayer {
    fn choose_move(&mut self, board: &Board) -> Position {
        self.move_rx.recv().unwrap()
    }
    fn get_mark(&self) -> Mark { self.mark }
}