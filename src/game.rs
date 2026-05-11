pub mod board;
pub mod mark;
pub mod result;

use ndarray::Array2;

use crate::{game::board::Board, player::Player};

const NUM_PLAYERS: usize = 2;

struct Game<'a> {
    /// The current turn number (starting from 0).
    turn: usize,

    /// The current state of the game board.
    board: Board,
    /// The players participating in the game.
    players: [&'a mut dyn Player; NUM_PLAYERS],
    /// The index of the player whose turn it is to play.
    turn_player: usize,
}

impl<'a> Game<'a> {
    /// Creates a new game with the given players.
    pub fn new(players: [&'a mut dyn Player; NUM_PLAYERS]) -> Result<Self, anyhow::Error> {
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

        Ok(Self {
            turn: 0,
            board: Board::new(Array2::from_elem((3, 3), None)),
            players,
            turn_player: 0,
        })
    }

    /// Execute a single turn of the game.
    pub fn play_move(&mut self) {
        let current_player = &mut self.players[self.turn_player];
        let pos = current_player.choose_move(&self.board);
        self.board.play_mark(pos, current_player.get_mark().into());
        self.turn += 1;
        self.turn_player = (self.turn_player + 1) % NUM_PLAYERS;
    }
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;
    use crate::game::{board::Position, mark::Mark};

    /// A dummy player that always makes the same moves.
    struct DumbPlayer {
        mark: Mark,
        preset_move: Position,
    }
    impl Player for DumbPlayer {
        fn choose_move(&mut self, _board: &Board) -> Position {
            self.preset_move
        }
        fn get_mark(&self) -> Mark {
            self.mark
        }
    }

    #[test]
    fn test_game_play_move() {
        let p1 = &mut DumbPlayer {
            preset_move: (0, 0),
            mark: Mark::X,
        };
        let p2 = &mut DumbPlayer {
            preset_move: (0, 1),
            mark: Mark::O,
        };
        let mut game = Game::new([p1, p2]).unwrap();

        assert_eq!(
            game.board.grid(),
            array![[None, None, None], [None, None, None], [None, None, None],]
        );

        game.play_move();
        assert_eq!(
            game.board.grid(),
            array![
                [Some(Mark::X), None, None],
                [None, None, None],
                [None, None, None],
            ]
        );
        assert_eq!(game.turn, 1);
        assert_eq!(game.turn_player, 1);

        game.play_move();
        assert_eq!(
            game.board.grid(),
            array![
                [Some(Mark::X), Some(Mark::O), None],
                [None, None, None],
                [None, None, None],
            ]
        );
    }
}
