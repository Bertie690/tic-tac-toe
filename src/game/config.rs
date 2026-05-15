use crate::game::Mark;

/// Enum representing the various CPU strategies the player can choose to play against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpponentKind {
    Minimax,
    Random,
}

impl std::fmt::Display for OpponentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpponentKind::Minimax => write!(f, "Minimax"),
            OpponentKind::Random => write!(f, "Random"),
        }
    }
}

/// Configuration for a new game session chosen by the player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameConfig {
    /// The mark the human player will use.
    pub player_mark: Mark,
    /// The CPU opponent's strategy.
    pub opponent: OpponentKind,
    /// Whether the human player should make the first move.
    pub player_first: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_mark: Mark::X,
            opponent: OpponentKind::Minimax,
            player_first: true,
        }
    }
}
