use std::sync::mpsc;

use crate::{
    game::{Game, OpponentKind},
    player::{Minimax, Random, TuiPlayer},
    renderer::{GameRequest, GameUpdate, TuiRenderer},
};

pub mod game;
pub mod player;
pub mod renderer;

fn main() -> anyhow::Result<()> {
    let (update_tx, update_rx) = mpsc::channel::<GameUpdate>();
    let (start_game_tx, start_game_rx) = mpsc::channel::<GameRequest>();

    // Game loop: waits for a GameRequest, runs the game, then loops.
    // Terminated when `start_game_tx` is dropped (i.e. the renderer quits).
    let thread = std::thread::spawn(move || -> anyhow::Result<()> {
        loop {
            let GameRequest { config, move_rx } = match start_game_rx.recv() {
                Ok(req) => req,
                // renderer has exited and dropped its sender
                Err(_) => return Ok(()),
            };

            let mark = config.player_mark;
            let player = TuiPlayer::new(mark, move_rx);
            let cpu: Box<dyn crate::player::Player> = match config.opponent {
                OpponentKind::Minimax => Box::new(Minimax::new(mark.opposite())),
                OpponentKind::Random => Box::new(Random::new(mark.opposite())),
            };
            let players = if config.player_first {
                [Box::new(player), cpu]
            } else {
                [cpu, Box::new(player)]
            };

            let mut game =
                Game::new(players, update_tx.clone()).expect("game initialization should succeed");

            loop {
                match game.play_move() {
                    Ok(()) if game.is_finished => break,
                    Ok(()) => {}
                    // The move channel was dropped — the renderer started a new
                    // game or quit. Either way, cleanly exit this game.
                    Err(e)
                        if e.downcast_ref::<crate::player::PlayerDisconnected>()
                            .is_some() =>
                    {
                        break;
                    }
                    Err(e) => return Err(e),
                }
            }
            // Loop back to waiting for the next GameRequest.
        }
    });

    let mut renderer = TuiRenderer {};
    renderer.run(update_rx, start_game_tx)?;

    // The renderer has exited. Wait for the game thread.
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
            if let Some(e) = panic_payload.downcast_ref::<anyhow::Error>()
                && e.downcast_ref::<crate::player::PlayerDisconnected>()
                    .is_some()
            {
                return Ok(());
            }
            Err(anyhow::anyhow!("Game thread panicked: {:?}", panic_payload))
        }
    }
}
