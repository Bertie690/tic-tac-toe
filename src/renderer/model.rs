use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use tuirealm::{
    application::Application,
    listener::EventListenerCfg,
    state::{State, StateValue},
    terminal::{CrosstermTerminalAdapter, TerminalAdapter},
};

use crate::{
    game::{Board, Position},
    renderer::{
        BoardComponent, GameUpdate,
        id::Id,
        message::{Message, UserEvent},
        port::GameUpdatePort,
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
        update_rx: Receiver<GameUpdate>,
        move_tx: Sender<Position>,
    ) -> Result<Self, anyhow::Error> {
        let mut terminal = CrosstermTerminalAdapter::new()?;

        terminal.clear_screen()?;
        let port = GameUpdatePort::new(update_rx);
        let mut app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(1), 10)
                .add_port(Box::new(port), Duration::from_millis(20), 1),
        );

        let board = Board::new(ndarray::Array2::from_elem((3, 3), None));
        app.mount(Id::Board, Box::new(BoardComponent::new(board)), vec![])?;
        app.active(&Id::Board)?;

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
            Message::Redraw => None,
            Message::AppQuit => {
                self.quit = true;
                self.terminal.clear_screen().ok()?;
                None
            }
        }
    }

    /// Render the current state of the model to the terminal.
    /// This should be called after any update that changes the model's state, and will trigger a redraw of the UI.
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

                let status_text = match self.app.state(&Id::Board) {
                    Ok(State::Single(StateValue::String(msg))) => msg,
                    _ => String::from("Game in progress"),
                };
                f.render_widget(Paragraph::new(Line::from(status_text)), status_area);

                f.render_widget(
                    Paragraph::new(vec![
                        Line::from("Arrows/WASD/HJKL: Move cursor"),
                        Line::from("Enter/Space: Place mark"),
                        Line::from("q/Esc: Quit"),
                    ]),
                    options_area,
                );
            })
            .expect("terminal should be capable of being drawn to");
    }
}
