use crate::game::{board::Board, mark::Mark};
use crate::game::result::Strategy;

/// A `Player` represents a participant in the tic-tac-toe game, executing a predetermined strategy.
pub struct Player {
    /// The strategy that this player follows to choose moves.
    strategy: Box<dyn Strategy>,
}

