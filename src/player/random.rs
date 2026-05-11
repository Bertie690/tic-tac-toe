use crate::{
    game::{
        board::{Board, Position},
        mark::Mark,
    },
    player::Player,
};
use fastrand::Rng;

/// A `Random` represents a strategy that chooses moves randomly from the available options.
pub struct Random<'a> {
    /// The random number generator used to select moves; passed via dependency injection for testability.
    rng: &'a mut Rng,
    mark: Mark,
}

impl<'a> Random<'a> {
    /// Creates a new `Random` strategy with the given mark and random number generator.
    pub fn new(mark: Mark, rng: &'a mut Rng) -> Self {
        Self { mark, rng }
    }
}

impl<'a> Player for Random<'a> {
    /// choose_move selects a random move from the available moves on the board.
    fn choose_move(&mut self, board: &Board) -> Position {
        self.rng
            .choice(board.available_moves().collect::<Vec<_>>())
            .expect("Board should have available moves when choose_move is called")
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
