/// A `Strategy` represents anything that can choose a move given the current state of the board.
pub trait Strategy {
    /// Choose a move based on the current state of the board.
    fn choose_move(&self, board: &Board) -> (usize, usize);

    /// Retrieve the mark that this strategy uses (X or O).
    fn get_mark(&self) -> Mark;
}

