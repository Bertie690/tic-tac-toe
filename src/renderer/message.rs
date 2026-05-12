use crate::game::Position;
use crate::renderer::GameUpdate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// Confirmed a cell selection via Enter
    MoveMade(Position),
    AppQuit,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameUpdated(GameUpdate),
}
