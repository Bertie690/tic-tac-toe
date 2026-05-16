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

    /// A hidden and entirely invisible widget used to trigger a redraw on window resize.
    RedrawOnResize,
}

use crate::game::{Board, GameConfig, GameResult, Move, Position};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// A cell was confirmed as selected by the user. \
    /// Will trigger the Model to send the position to the game thread for processing.
    MoveMade(Position),

    /// A user attempted to make an invalid move (e.g. selecting an occupied cell or trying to move after game over). \
    InvalidMove(InvalidMoveReason),

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidMoveReason {
    /// The user attempted to select a cell that was already occupied.
    CellOccupied,
    /// The user attempted to make a move after the game had already concluded.
    GameOver,
}

// Implement two-way conversion to and from strings to allow for storing inside custom event info

impl From<InvalidMoveReason> for &'static str {
    fn from(reason: InvalidMoveReason) -> Self {
        match reason {
            InvalidMoveReason::CellOccupied => "CellOccupied",
            InvalidMoveReason::GameOver => "GameOver",
        }
    }
}

impl From<InvalidMoveReason> for String {
    fn from(reason: InvalidMoveReason) -> Self {
        let s: &'static str = reason.into();
        s.to_string()
    }
}

impl TryFrom<&'static str> for InvalidMoveReason {
    type Error = String;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        match s {
            "CellOccupied" => Ok(InvalidMoveReason::CellOccupied),
            "GameOver" => Ok(InvalidMoveReason::GameOver),
            _ => Err(format!("Unknown InvalidMoveReason: {}", s)),
        }
    }
}

impl TryFrom<String> for InvalidMoveReason {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "CellOccupied" => Ok(InvalidMoveReason::CellOccupied),
            "GameOver" => Ok(InvalidMoveReason::GameOver),
            _ => Err(format!("Unknown InvalidMoveReason: {}", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UserEvent {
    GameStarted,
    GameUpdated(GameUpdate),
    InvalidMove(InvalidMoveReason),
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
