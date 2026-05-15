use crate::game::Position;
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
    /// Request to start a new game (modal config not yet implemented).
    NewGame,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameStarted,
    GameUpdated(GameUpdate),
}
