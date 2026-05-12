use crate::{
    game::{Board, GameResult, Mark, Position},
    player::Player,
};

/// A `Minimax` represents a strategy determined by the minimax algorithm.
/// It attempts to recursively evaluate the game tree to find the optimal move for the current player
/// (assuming optimal play from the opponent).
pub struct Minimax {
    mark: Mark,
}

impl Minimax {
    /// Create a new Minimax player with the given mark.
    pub fn new(mark: Mark) -> Self {
        Self { mark }
    }
}

impl Player for Minimax {
    /// choose_move implements the minimax algorithm using alpha-beta pruning.
    fn choose_move(&mut self, board: &Board) -> Position {
        minimax_move(board, self.mark)
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

fn minimax_move(board: &Board, cpu_mark: Mark) -> Position {
    board
        .available_moves()
        .into_iter()
        .max_by_key(|&pos| {
            let b = &mut board.clone();
            b.play_mark(pos, cpu_mark);

            alpha_beta(b, cpu_mark, i32::MIN, i32::MAX, false)
        })
        .expect("choose_move should not be called on a filled board")
}

/// alpha_beta implements the minimax algorithm with alpha-beta pruning,
/// repeatedly evaluating the game tree until it reaches a terminal position (win/loss/draw) and pruning branches that cannot affect the final decision.
fn alpha_beta(
    board: &Board,
    cpu_mark: Mark,
    mut alpha: i32,
    mut beta: i32,
    is_maximizing: bool,
) -> i32 {
    let state = board.state();
    // Score terminal positions
    if let GameResult::Winner(winner) = state {
        return if winner == cpu_mark { 1 } else { -1 };
    } else if state == GameResult::Draw {
        return 0;
    }

    let current = if is_maximizing {
        cpu_mark
    } else {
        cpu_mark.opposite()
    };

    let mut best = if is_maximizing { i32::MIN } else { i32::MAX };

    // check each position, stopping early if we hit a guaranteed win/loss
    for pos in board.available_moves() {
        let b = &mut board.clone();
        b.play_mark(pos, current);

        let score = alpha_beta(&b, cpu_mark, alpha, beta, !is_maximizing);

        if is_maximizing {
            best = best.max(score);
            alpha = alpha.max(best);
        } else {
            best = best.min(score);
            beta = beta.min(best);
        };

        // prune
        if beta <= alpha {
            break;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_minimax_should_go_for_wins() {
        let board = Board::new(array![
            [Some(Mark::X), Some(Mark::X), None],
            [Some(Mark::O), Some(Mark::O), None],
            [None, None, None],
        ]);

        assert_eq!(minimax_move(&board, Mark::X), (0, 2));
        assert_eq!(minimax_move(&board, Mark::O), (1, 2));
    }
}
