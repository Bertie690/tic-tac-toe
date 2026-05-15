use crate::{
    game::{
        Mark, {Board, Position},
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
    fn choose_move(&mut self, board: &Board) -> anyhow::Result<Position> {
        self.rng
            .choice(board.available_moves().collect::<Vec<_>>())
            .ok_or_else(|| anyhow::anyhow!("No legal moves to perform!"))
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
