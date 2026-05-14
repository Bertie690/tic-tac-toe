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
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameUpdated(GameUpdate),
}
