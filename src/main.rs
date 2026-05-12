use std::sync::mpsc;

use crate::{
    game::{Board, Game, Mark, Position},
    player::{Minimax, TuiPlayer},
    renderer::{GameUpdate, TuiRenderer},
};

pub mod game;
pub mod player;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    // TODO: Create title screen to allow selecting player mark/diff level
    let mark = Mark::X; // TODO: get from title screen

    let (board_tx, board_rx) = mpsc::channel::<GameUpdate>();
    let (move_tx, move_rx) = mpsc::channel::<Position>();

    // Pass the ends of the channel to the renderer and player:
    let mut renderer = TuiRenderer {};
    let mut player = TuiPlayer::new(mark, move_rx);
    let mut cpu = Minimax::new(mark.opposite());

    // Start the game thread in the background
    let thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let mut game = Game::new([&mut player, &mut cpu], board_tx)
            .expect("game initialization should succeed");

        while !game.is_finished {
            game.play_move()?
        }
        Ok(())
    });

    renderer.run(board_rx, move_tx)?;

    thread
        .join()
        .map_err(|v| anyhow::anyhow!("Game thread panicked: {:?}", v))??;

    Ok(())
}
