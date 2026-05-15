use ratatui::layout::{Constraint, Layout, Margin, Size};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{CompletedFrame, Frame};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use tuirealm::terminal::TerminalResult;
use tuirealm::{
    application::Application,
    listener::EventListenerCfg,
    state::{State, StateValue},
    subscription::{EventClause, Sub, SubClause},
    terminal::{CrosstermTerminalAdapter, TerminalAdapter},
};

use crate::{
    game::{Board, Mark, Move, Position},
    renderer::{
        AppBoardComponent, AppSidebarComponent, GameUpdate,
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
        let mut adapter = CrosstermTerminalAdapter::new()?;
        // NB: This is REQUIRED for proper input handling (i wish the docs mentioned this...)
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;

        let port = GameUpdatePort::new(update_rx);
        let mut app = Application::init(
            EventListenerCfg::default()
                .tick_interval(Duration::from_millis(20))
                .crossterm_input_listener(Duration::from_millis(20), 50)
                .add_port(Box::new(port), Duration::from_millis(20), 1),
        );

        let board = Board::new(ndarray::Array2::from_elem((3, 3), None));
        // Subscribe the board to game-update user events so it receives them
        // even when the sidebar has focus.
        // Keyboard events reach it only when focused.
        app.mount(
            Id::Board,
            Box::new(AppBoardComponent::new(board)),
            vec![Sub::new(
                EventClause::Discriminant(UserEvent::GameUpdated(GameUpdate::Move(
                    Move::default()
                ))),
                SubClause::Always,
            )],
        )?;
        // The sidebar only needs keyboard events, which it receives only when focused.
        app.mount(
            Id::Sidebar,
            Box::new(AppSidebarComponent::new()),
            vec![],
        )?;
        app.active(&Id::Sidebar)?;

        Ok(Self {
            app,
            terminal: adapter,
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
                self.terminal.restore().ok()?;
                None
            }
            Message::FocusSidebar => {
                let _ = self.app.active(&Id::Sidebar);
                Some(Message::Redraw)
            }
            Message::FocusBoard => {
                let _ = self.app.active(&Id::Board);
                Some(Message::Redraw)
            }
            Message::NewGame => {
                // TODO: Open player/difficulty configuration modal
                None
            }
        }
    }

    /// Render the current state of the model to the terminal.
    /// This should be called after any update that changes the model's state, and will trigger a redraw of the UI.
    pub fn view(&mut self) {
        self.terminal
            .draw(|frame| {
                let [board_area, sidebar_area] = Layout::horizontal([
                    Constraint::Ratio(3, 5),
                    Constraint::Ratio(2, 5),
                ])
                .areas(frame.area());

                // Add a 1 character margin around the board and clamp it to a square, ensuring it is always a multiple of 3 for easy division
                let board_with_margins = board_area.inner(Margin::new(1, 1));
                let side_length = board_with_margins.width.min(board_with_margins.height);
                let side_rounded = side_length - (side_length % 3);
                let squared = board_with_margins.resize(Size::new(side_rounded, side_rounded));

                self.app.view(&Id::Board, frame, squared);
                self.app.view(&Id::Sidebar, frame, sidebar_area);
            })
            .expect("terminal should be capable of being drawn to");
    }
}
