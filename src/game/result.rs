use crate::game::mark::Mark;

/// A `GameResult` represents the current state of a tic-tac-toe game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    /// The game is still in the midst of being played.
    Ongoing,

    /// The game has ended in a draw (all squares are occupied).
    Draw,

    /// The game has ended with a winner.
    Winner(Mark),
}
