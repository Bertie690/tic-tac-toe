use crate::game::{Mark, Position};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// A Move represents information about a single move in the game.
pub struct Move {
    /// The mark that was played (X or O).
    pub mark: Mark,
    /// The position on the board where the mark was played.
    pub position: Position,
}
