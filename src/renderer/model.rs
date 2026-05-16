use ndarray::Array2;
use ratatui::layout::{Constraint, Layout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use tuirealm::props::{AttrValue, Attribute};
use tuirealm::{
    application::Application,
    listener::EventListenerCfg,
    subscription::{EventClause, Sub, SubClause},
    terminal::{CrosstermTerminalAdapter, TerminalAdapter},
};

use crate::renderer::components::redraw_on_resize::RedrawOnResizeComponent;
use crate::{
    game::{Board, Move, Position},
    renderer::{
        AppBoardComponent, AppNewGameModal, AppSidebarComponent, AppStatusComponent, GameRequest,
        GameUpdate,
        enums::Id,
        enums::{Message, UserEvent},
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
    /// Sends a new [`GameRequest`] to the game-loop thread whenever the user starts a game.
    start_game_tx: Sender<GameRequest>,
    /// The sending end of the current game's move channel.
    /// `None` when no game has been started yet.
    move_tx: Option<Sender<Position>>,
}

impl Model {
    pub fn new(
        update_rx: Receiver<GameUpdate>,
        start_game_tx: Sender<GameRequest>,
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

        let board = Board::new(Array2::from_elem((3, 3), None));
        // Subscribe the board to game update events so it receives them
        // even when the sidebar has focus
        app.mount(
            Id::Board,
            Box::new(AppBoardComponent::new(board)),
            vec![Sub::new(
                EventClause::Discriminant(UserEvent::GameUpdated(
                    GameUpdate::Move(Move::default()),
                )),
                SubClause::Always,
            )],
        )?;

        app.mount(
            Id::Status,
            Box::new(AppStatusComponent::new()),
            vec![Sub::new(
                // needed to allow passing any keyboard event when unfocused
                EventClause::Any,
                SubClause::Always,
            )],
        )?;

        // The sidebar only needs keyboard events, which it receives only when focused
        app.mount(Id::Sidebar, Box::new(AppSidebarComponent::new()), vec![])?;

        app.mount(
            Id::RedrawOnResize,
            Box::new(RedrawOnResizeComponent),
            vec![Sub::new(EventClause::WindowResize, SubClause::Always)],
        )?;

        app.active(&Id::Sidebar)?;

        Ok(Self {
            app,
            terminal: adapter,
            quit: false,
            redraw: false,
            start_game_tx,
            move_tx: None,
        })
    }

    /// Update the model and/or game state after a message is sent from a UI component.
    pub fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        match msg? {
            Message::MoveMade(pos) => {
                if let Some(tx) = &self.move_tx {
                    let _ = tx.send(pos);
                }
                None
            }
            Message::InvalidMove(reason) => {
                let _ = self.app.attr(
                    &Id::Status,
                    Attribute::Content,
                    AttrValue::String(reason.into()),
                );
                Some(Message::Redraw)
            }

            Message::Redraw => {
                self.redraw = true;
                None
            }
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
            Message::OpenNewGameModal => {
                if !self.app.mounted(&Id::NewGameModal) {
                    let _ =
                        self.app
                            .mount(Id::NewGameModal, Box::new(AppNewGameModal::new()), vec![]);
                }
                let _ = self.app.active(&Id::NewGameModal);
                Some(Message::Redraw)
            }
            Message::StartGame(config) => {
                let _ = self.app.umount(&Id::NewGameModal);
                let _ = self.app.active(&Id::Board);

                // Replace the move channel so the old TuiPlayer (if any) detects
                // disconnection and exits, then start the new game.
                let (move_tx, move_rx) = mpsc::channel::<Position>();
                self.move_tx = Some(move_tx);
                let _ = self.start_game_tx.send(GameRequest { config, move_rx });

                Some(Message::Redraw)
            }
            Message::CloseModal => {
                let _ = self.app.umount(&Id::NewGameModal);
                let _ = self.app.active(&Id::Sidebar);
                Some(Message::Redraw)
            }
        }
    }

    /// Render the current state of the model to the terminal.
    /// This should be called after any update that changes the model's state, and will trigger a redraw of the UI.
    pub fn view(&mut self) {
        let modal_open = self.app.mounted(&Id::NewGameModal);
        self.terminal
            .draw(|frame| {
                let [play_area, sidebar_area] =
                    Layout::horizontal([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)])
                        .areas(frame.area());

                // Split the left area into the board and a 2-line status area beneath
                let [board_area, status_area] =
                    Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).areas(play_area);

                self.app.view(&Id::Board, frame, board_area);
                self.app.view(&Id::Status, frame, status_area);
                self.app.view(&Id::Sidebar, frame, sidebar_area);

                if modal_open {
                    // Overlay the modal on the bottom half of the sidebar.
                    let [_, modal_area] =
                        Layout::vertical([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                            .areas(sidebar_area);
                    self.app.view(&Id::NewGameModal, frame, modal_area);
                }
            })
            .expect("terminal should be capable of being drawn to");
    }
}
