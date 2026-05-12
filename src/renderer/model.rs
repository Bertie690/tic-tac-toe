use ratatui::layout::{Constraint, Layout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use tuirealm::{
    application::Application,
    listener::EventListenerCfg,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter},
};

use crate::{
    game::{Board, Position},
    renderer::{
        GameUpdate, id::Id, message::{Message, UserEvent}, port::BoardUpdatePort
    },
};

/// The `Model` struct encapsulates the state of the TUI application, including the UI component tree, terminal adapter, and communication channels with the game thread.
pub struct Model {
    /// The underlying `Application` instance holding the UI component tree.
    pub app: Application<Id, Message, UserEvent>,
    /// The underlying terminal adapter, used to draw UI components.
    pub terminal: CrosstermTerminalAdapter,
    /// Whether the user has requested to quit the application.
    pub quit: bool,
    /// Whether the application should be redrawn.
    pub redraw: bool,
    /// The underlying connection to the main game thread, to which move information will be sent.
    move_tx: mpsc::Sender<Position>,
}

impl Model {
    pub fn new(
        move_tx: Sender<Position>,
        board_rx: Receiver<GameUpdate>,
    ) -> Result<Self, anyhow::Error> {
        let terminal = CrosstermTerminalAdapter::new()?;
        let port = BoardUpdatePort::new(board_rx);
        let app = Application::init(EventListenerCfg::default().add_port(
            Box::new(port),
            Duration::from_millis(20),
            1,
        ));

        Ok(Self {
            app,
            terminal,
            quit: false,
            redraw: false,
            move_tx,
        })
    }

    /// Update the model and/or game state after a message is sent from a UI component.
    pub fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        match msg? {
            Message::MoveMade(pos) => {
                let _ = self.move_tx.send(pos);
                None
            }
            Message::AppQuit => {
                self.quit = true;
                None
            }
        }
    }

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                let [board_area, status_area, options_area] = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Min(4),
                ])
                .areas(f.area());
                self.app.view(&Id::Board, f, board_area);
                self.app.view(&Id::Status, f, status_area);
                self.app.view(&Id::Options, f, options_area);
            })
            .expect("terminal should be capable of being drawn to");
    }
}
