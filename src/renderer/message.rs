use crate::game::{GameConfig, Position};
use crate::renderer::GameUpdate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// A cell was confirmed as selected by the user.
    /// Will trigger the Model to pipe the position to the game thread for processing.
    MoveMade(Position),
    /// Force the screen to redraw itself.
    Redraw,
    AppQuit,
    /// Move keyboard focus to the sidebar.
    FocusSidebar,
    /// Move keyboard focus back to the board.
    FocusBoard,
    /// Open the new game configuration modal.
    OpenNewGameModal,
    /// Start a new game with the given configuration.
    StartGame(GameConfig),
    /// Close the current modal and return to the menu screen.
    CloseModal,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameStarted,
    GameUpdated(GameUpdate),
}
