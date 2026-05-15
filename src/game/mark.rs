/// A Mark represents a player's move on the board, either an X or an O.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    #[default]
    X = 1,
    O = -1,
}

impl Mark {
    /// Returns the opposite mark (X becomes O, and O becomes X).
    pub fn opposite(&self) -> Mark {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
        }
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Mark::X => 'X',
            Mark::O => 'O',
        };
        write!(f, "{}", ch)
    }
}
