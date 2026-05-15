use std::sync::mpsc::{self, Receiver};

use tuirealm::{
    event::Event,
    listener::{Poll, PortError, PortResult},
};

use crate::renderer::{GameUpdate, enums::UserEvent};

pub struct GameUpdatePort {
    update_rx: Receiver<GameUpdate>,
}

impl GameUpdatePort {
    pub fn new(update_rx: Receiver<GameUpdate>) -> Self {
        Self { update_rx }
    }
}

impl Poll<UserEvent> for GameUpdatePort {
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        match self.update_rx.try_recv() {
            Ok(update) => Ok(Some(Event::User(UserEvent::GameUpdated(update)))),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(PortError::PermanentError(
                "Board update channel disconnected".into(),
            )),
        }
    }
}
