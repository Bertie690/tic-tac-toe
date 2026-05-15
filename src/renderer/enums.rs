#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// An `Id` represents a widget in the TUI, used for identification purposes.
pub enum Id {
    /// The 3×3 grid of tiles, taking up the left side of the screen.
    Board,

    /// The status widget rendered under the board.
    Status,

    /// A sidebar menu containing various options.
    Sidebar,

    /// The "New Game" configuration popup, overlaying the bottom half of the sidebar.
    NewGameModal,
}

use crate::game::{Board, GameConfig, GameResult, Move, Position};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// A cell was confirmed as selected by the user. \
    /// Will trigger the Model to send the position to the game thread for processing.
    MoveMade(Position),
    /// Force the screen to redraw itself.
    Redraw,
    /// Quit the application.
    AppQuit,
    /// Move keyboard focus to the sidebar.
    FocusSidebar,
    /// Move keyboard focus back to the board.
    FocusBoard,
    /// Open the new game configuration modal.
    OpenNewGameModal,
    /// Start a new game with the given configuration. \
    /// Will trigger the Model to send the configuration to the game thread to start a new game.
    StartGame(GameConfig),
    /// Close the current modal and return to the menu screen.
    CloseModal,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameStarted,
    GameUpdated(GameUpdate),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A `GameUpdate` represents an update to the game state that the renderer should display.
pub enum GameUpdate {
    /// The initial state of the game board, sent immediately after game creation.
    Initial(Board),
    /// A move was made on the board.
    Move(Move),
    /// The game has finished with the given result.
    Finished {
        /// The game board at the time of conclusion.
        board: Board,
        /// The result of the game at the moment of its conclusion.
        ///
        /// Is guaranteed to never be [`GameResult::Ongoing`], as this would signify an invalid game state. \
        /// Callers are encouraged to use [`unreachable!`] where necessary to reflect this invariant.
        result: GameResult,
    },
}
