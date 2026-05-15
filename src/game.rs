mod board;
mod config;
mod mark;
mod r#move;
mod result;
use std::sync::mpsc::Sender;

// re-export these for easier access by other modules
pub use board::{Board, Position};
pub use config::{GameConfig, OpponentKind};
pub use mark::Mark;
pub use r#move::Move;
pub use result::GameResult;

use ndarray::Array2;

use crate::{player::Player, renderer::GameUpdate};

const NUM_PLAYERS: usize = 2;

pub struct Game {
    /// The current turn number (starting from 0).
    turn: usize,
    /// The current state of the game board.
    board: Board,
    /// The players participating in the game.
    players: [Box<dyn Player>; NUM_PLAYERS],
    /// The index of the player whose turn it is to play.
    turn_player: usize,
    /// The sending end of the channel used to send the current board state to the renderer.
    update_tx: Sender<GameUpdate>,
    /// Whether the game has finished. Used to prevent moves from being played after the game is over.
    pub is_finished: bool,
}

impl Game {
    /// Creates a new game with the given players, who will take turns in the given order.
    ///
    /// Returns an error if 2 players share the same mark.
    pub fn new(
        players: [Box<dyn Player>; NUM_PLAYERS],
        update_tx: Sender<GameUpdate>,
    ) -> Result<Self, anyhow::Error> {
        // yes this is O(n^2) but n is small and this is only called once so it's fiiiiine
        for i in 0..players.len() {
            for j in (i + 1)..players.len() {
                if players[i].get_mark() == players[j].get_mark() {
                    return Err(anyhow::anyhow!(
                        "Player {} shares a mark with player {}: {:?}",
                        i,
                        j,
                        players[i].get_mark()
                    ));
                }
            }
        }

        // send the initial board state to the renderer
        let board = Board::new(Array2::from_elem((3, 3), None));
        update_tx.send(GameUpdate::Initial(board.clone()))?;
        Ok(Self {
            turn: 0,
            board,
            players,
            turn_player: 0,
            update_tx,
            is_finished: false,
        })
    }

    /// Execute a single turn of the game.
    pub fn play_move(&mut self) -> anyhow::Result<()> {
        self.verify_unfinished()?;

        let current_player = &mut self.players[self.turn_player];
        let mark = current_player.get_mark();

        let pos = current_player.choose_move(&self.board)?;

        self.board.set_mark(pos, current_player.get_mark().into());
        self.turn += 1;

        self.turn_player = (self.turn_player + 1) % NUM_PLAYERS;
        self.update_tx.send(GameUpdate::Move(Move {
            mark,
            position: pos,
        }))?;

        let result = self.board.state();
        if result == GameResult::Ongoing {
            Ok(())
        } else {
            self.finish_game(result)
        }
    }

    /// Verify that the game is not already finished, returning an error if it is.
    fn verify_unfinished(&self) -> Result<(), anyhow::Error> {
        if self.is_finished {
            Err(anyhow::anyhow!("Game is already finished"))
        } else {
            Ok(())
        }
    }

    /// End the game, sending the final board state and result to the renderer.
    ///
    /// Takes the result as an argument to avoid having to recompute it.
    fn finish_game(&mut self, result: GameResult) -> Result<(), anyhow::Error> {
        self.verify_unfinished()?;

        self.is_finished = true;
        self.update_tx.send(GameUpdate::Finished {
            board: self.board.clone(),
            result,
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;
    use crate::game::{board::Position, mark::Mark};
    use std::sync::mpsc::channel;

    /// A dummy player that always makes the same moves.
    struct DumbPlayer {
        mark: Mark,
        preset_move: Position,
    }
    impl Player for DumbPlayer {
        fn choose_move(&mut self, _board: &Board) -> anyhow::Result<Position> {
            Ok(self.preset_move)
        }
        fn get_mark(&self) -> Mark {
            self.mark
        }
    }

    #[test]
    fn test_game_play_move() {
        let p1 = DumbPlayer {
            preset_move: (0, 0),
            mark: Mark::X,
        };
        let p2 = DumbPlayer {
            preset_move: (0, 1),
            mark: Mark::O,
        };

        let (board_tx, board_rx) = channel::<GameUpdate>();
        let mut game = Game::new([Box::new(p1), Box::new(p2)], board_tx).unwrap();

        let new_board = array![[None, None, None], [None, None, None], [None, None, None],];
        assert_eq!(game.board.grid(), new_board);
        assert!(
            board_rx.try_recv().is_ok_and(
                |b| matches!(b, GameUpdate::Initial(board) if board.grid() == new_board)
            ),
            "Game should send initial board state on creation"
        );

        let new_board = array![
            [Some(Mark::X), None, None],
            [None, None, None],
            [None, None, None],
        ];

        game.play_move()
            .expect("connection to renderer shouldn't fail mid-game");
        assert_eq!(game.board.grid(), new_board,);
        assert!(
            board_rx
                .try_recv()
                .is_ok_and(|b| matches!(b, GameUpdate::Move (Move {position, mark} )
                    if position == (0, 0) && mark == Mark::X)),
            "Game should send the move info to the renderer after each move"
        );
        assert_eq!(game.turn, 1);
        assert_eq!(game.turn_player, 1);

        let new_board = array![
            [Some(Mark::X), Some(Mark::O), None],
            [None, None, None],
            [None, None, None],
        ];
        game.play_move()
            .expect("connection to renderer shouldn't fail mid-game");
        assert_eq!(game.board.grid(), new_board);
        assert!(
            board_rx
                .try_recv()
                .is_ok_and(|b| matches!(b, GameUpdate::Move (Move {position, mark} )
                    if position == (0, 1) && mark == Mark::O)),
            "Game should send the move info to the renderer after each move"
        );
        assert_eq!(game.turn, 2);
        assert_eq!(game.turn_player, 0);
    }

    #[test]
    fn test_game_finish_game() {
        let p1 = DumbPlayer {
            preset_move: (0, 0),
            mark: Mark::X,
        };
        let p2 = DumbPlayer {
            preset_move: (1, 0),
            mark: Mark::O,
        };

        let (board_tx, board_rx) = channel::<GameUpdate>();
        let mut game = Game::new([Box::new(p1), Box::new(p2)], board_tx).unwrap();

        // discard initial board state
        board_rx
            .try_recv()
            .expect("game should send initial board state during init");

        game.finish_game(GameResult::Winner(Mark::X))
            .expect("connection to renderer shouldn't fail mid-game");

        let upd = board_rx.try_recv().unwrap();
        assert!(
            matches!(upd, GameUpdate::Finished { ref board, result }
                if *board == game.board && result == GameResult::Winner(Mark::X)),
            "Game should send the final board state and result to the renderer when the game finishes; got {:?}",
            upd.clone()
        );
    }
}
