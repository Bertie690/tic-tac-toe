use crate::{
    game::{
        Mark, {Board, Position},
    },
    player::Player,
};
use fastrand::Rng;

/// A `Random` represents a strategy that chooses moves randomly from the available options.
pub struct Random {
    /// The random number generator used to select moves; passed via dependency injection for testability.
    rng: Rng,
    mark: Mark,
}

impl Random {
    pub fn new(mark: Mark) -> Self {
        Self {
            mark,
            rng: Rng::new(),
        }
    }
}

impl Player for Random {
    fn choose_move(&mut self, board: &Board) -> anyhow::Result<Position> {
        self.rng
            .choice(board.available_moves().collect::<Vec<_>>())
            .ok_or_else(|| anyhow::anyhow!("No legal moves to perform!"))
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
