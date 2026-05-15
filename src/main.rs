use std::sync::mpsc::{self, RecvError};

use crate::{
    game::{Game, Mark, Position},
    player::{Minimax, TuiPlayer},
    renderer::{GameUpdate, TuiRenderer},
};

pub mod game;
pub mod player;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    let mark = Mark::X;

    let (update_tx, update_rx) = mpsc::channel::<GameUpdate>();
    let (move_tx, move_rx) = mpsc::channel::<Position>();

    // Pass the ends of the channel to the renderer and player:
    let mut renderer = TuiRenderer {};
    let mut player = TuiPlayer::new(mark, move_rx);
    let mut cpu = Minimax::new(mark.opposite());

    // Start the game thread in the background
    let thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let mut game = Game::new([&mut player, &mut cpu], update_tx)
            .expect("game initialization should succeed");

        while !game.is_finished {
            game.play_move()?
        }
        Ok(())
    });

    renderer.run(update_rx, move_tx)?;

    // explicitly disregard errors produced from a connection closure
    match thread.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => {
            if e.downcast_ref::<crate::player::PlayerDisconnected>()
                .is_some()
            {
                Ok(())
            } else {
                Err(e)
            }
        }
        Err(panic_payload) => {
            if let Some(e) = panic_payload.downcast_ref::<anyhow::Error>() {
                if e.downcast_ref::<crate::player::PlayerDisconnected>()
                    .is_some()
                {
                    return Ok(());
                }
            }
            Err(anyhow::anyhow!("Game thread panicked: {:?}", panic_payload))
        }
    }
}
