//! The renderer package implements a simple TUI renderer

pub mod components;
mod id;
mod message;
mod model;
mod port;

pub use components::board::AppBoardComponent;
pub use components::sidebar::AppSidebarComponent;

use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

use tuirealm::application::PollStrategy;

use crate::{
    game::{Board, GameResult, Move, Position},
    renderer::model::Model,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A `GameUpdate` represents an update to the game state that the renderer should display.
pub enum GameUpdate {
    Move(
        /// The move having been made.
        Move,
    ),
    Finished {
        board: Board,
        result: GameResult,
    },
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

/// Drives the TUI event loop. Owns no channel ends itself — the caller splits
/// the channels and hands each end to the appropriate party:
/// - `board_tx` is given to `Game` as its `Renderer`
/// - `board_rx` is passed into [`TuiRenderer::run`]
/// - `move_tx` is passed into [`TuiRenderer::run`]
/// - `move_rx` is given to [`TuiPlayer`]
///
/// [`TuiPlayer`]: crate::player::TuiPlayer
pub struct TuiRenderer;

const POLL_TIMEOUT: u64 = 20; // ms

impl TuiRenderer {
    /// Start the main TUI event loop.
    /// Returns the first error encountered, if any occur before the game is quitted.
    ///
    /// It is the caller's responsibility to pass the other ends of the provided channels to their proper destinations.
    pub fn run(
        &mut self,
        update_rx: Receiver<GameUpdate>,
        move_tx: Sender<Position>,
    ) -> anyhow::Result<()> {
        let mut model = Model::new(update_rx, move_tx)?;
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
