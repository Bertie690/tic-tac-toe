#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// An `Id` represents a widget in the TUI, used for identification purposes.
pub enum Id {
    /// the 3×3 grid
    Board,

    /// A line of text displaying the previous move made (P2 placed O at (0, 2)).
    /// Also displays game status and/or difficulty level.
    Status,

    /// A line of text holding menu option buttons like "Quit", "New Game", and so on.
    Options,
}