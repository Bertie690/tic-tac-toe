use ratatui::{Frame, layout::Rect};
use tuirealm::{
    command::{Cmd, CmdResult},
    component::{AppComponent, Component},
    event::Event,
    props::{AttrValue, Attribute, QueryResult},
    state::State,
};

use crate::renderer::{Message, UserEvent};

/// Dummy "component" that exists solely to force a redraw on window resize.
pub struct RedrawOnResizeComponent;

impl Component for RedrawOnResizeComponent {
    fn view(&mut self, _: &mut Frame, _: Rect) {
        // nothing to draw
    }

    fn attr(&mut self, _: Attribute, _: AttrValue) {
        // does nothing
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::NoChange
    }

    fn query(&self, _: Attribute) -> Option<QueryResult<'_>> {
        None
    }

    fn state(&self) -> State {
        State::None
    }
}

impl AppComponent<Message, UserEvent> for RedrawOnResizeComponent {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        if let Event::WindowResize(..) = ev {
            Some(Message::Redraw)
        } else {
            None
        }
    }
}
