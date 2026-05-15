//! The renderer package implements a simple TUI renderer

pub mod components;
mod id;
mod message;
mod model;
mod port;

pub use components::board::AppBoardComponent;
pub use components::new_game_modal::AppNewGameModal;
pub use components::sidebar::AppSidebarComponent;

use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

use tuirealm::application::PollStrategy;

use crate::{
    game::{Board, GameConfig, GameResult, Move, Position},
    renderer::model::Model,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A `GameUpdate` represents an update to the game state that the renderer should display.
pub enum GameUpdate {
    /// The initial state of the game board, sent immediately after game creation.
    Initial(Board),

    /// A move was made on the board.
    Move(
        Move,
    ),
    /// The game has finished with the given result.
    Finished {
        board: Board,
        result: GameResult,
    },
}

/// CarrStruct representing a request to start a new game.
///
/// Created by [`Model`] when the user confirms a new game in the modal; consumed
/// by the game-loop thread in `main`.
pub struct GameRequest {
    pub config: GameConfig,
    pub move_rx: Receiver<Position>,
}

pub trait Renderer {
    /// Render the current state of the game board to the user.
    ///
    /// It is expected that implementers will render the result in a way that
    /// enables the end user to trigger another board update.
    fn render(&mut self, update: GameUpdate) -> Result<(), anyhow::Error>;
}

/// `Sender<GameUpdate>` is itself a renderer: its entire purpose is to relay the game state
/// between the main thread (which owns the sending end) and the TUI event loop
/// (which owns the receiving end).
impl Renderer for mpsc::Sender<GameUpdate> {
    fn render(&mut self, update: GameUpdate) -> Result<(), anyhow::Error> {
        self.send(update).map_err(Into::into)
    }
}

/// Drives the TUI event loop.
///
/// Channel wiring:
/// - `update_rx` receives game-state snapshots from `Game` (via `update_tx`)
/// - `start_game_tx` is sent a [`GameRequest`] each time the user confirms a new
///   game in the modal; the game-loop thread in `main` owns the receiving end
pub struct TuiRenderer;

const POLL_TIMEOUT: u64 = 20; // ms

impl TuiRenderer {
    /// Start the main TUI event loop.
    /// Returns the first error encountered, if any occur before the game is quitted.
    pub fn run(
        &mut self,
        update_rx: Receiver<GameUpdate>,
        start_game_tx: Sender<GameRequest>,
    ) -> anyhow::Result<()> {
        let mut model = Model::new(update_rx, start_game_tx)?;
        model.view(); // initial draw

        while !model.quit {
            match model
                .app
                .tick(PollStrategy::TryFor(Duration::from_millis(POLL_TIMEOUT)))
            {
                Ok(msgs) => {
                    for msg in msgs {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = model.update(msg);
                        }
                        model.redraw = true;
                    }
                }
                Err(e) => return Err(e.into()),
            }
            if model.redraw {
                model.view();
                model.redraw = false;
            }
        }
        Ok(())
    }
}
