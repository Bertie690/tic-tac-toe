use std::sync::mpsc::{self, Receiver};

use tuirealm::{
    event::Event,
    listener::{Poll, PortError, PortResult},
};

use crate::renderer::{GameUpdate, message::UserEvent};

pub struct BoardUpdatePort {
    board_rx: Receiver<GameUpdate>,
}

impl BoardUpdatePort {
    pub fn new(board_rx: Receiver<GameUpdate>) -> Self {
        Self { board_rx }
    }
}

impl Poll<UserEvent> for BoardUpdatePort {
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        match self.board_rx.try_recv() {
            Ok(update) => Ok(Some(Event::User(UserEvent::GameUpdated(update)))),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(PortError::PermanentError(
                "Board update channel disconnected".into(),
            )),
        }
    }
}
