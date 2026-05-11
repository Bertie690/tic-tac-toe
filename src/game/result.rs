use crate::game::mark::Mark;

/// A `GameResult` represents the current state of a tic-tac-toe game.
pub enum GameResult {
    /// The game is still in the midst of being played.
    Ongoing,

    /// The game has ended in a draw.
    Draw,

    /// The game has ended with a winner.
    Winner(Mark),
}
