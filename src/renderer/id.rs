#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// An `Id` represents a widget in the TUI, used for identification purposes.
pub enum Id {
    /// The 3×3 grid of tiles.
    ///
    /// Expects to receive a square area to place the cells.
    Board,

    /// A sidebar containing options, sttatus text, and other non-board information.
    Sidebar,
}
