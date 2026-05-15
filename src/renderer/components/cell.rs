use ratatui::{Frame, layout::Rect};
use tuirealm::{
    command::{Cmd, CmdResult},
    component::Component,
    props::AttrValue,
    state::State,
};

use crate::game::Mark;

/// Clear the current drawing area.
fn clear_area(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            buf[(x, y)].set_char(' ');
        }
    }
}

fn draw_centered(ch: char, frame: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let center_x = area.x + area.width / 2;
    let center_y = area.y + area.height / 2;
    frame.buffer_mut()[(center_x, center_y)].set_char(ch);
}

/// A CellComponent displays the contents of a single cell on the board.
pub struct CellComponent {
    pub mark: Option<Mark>,
}

impl Component for CellComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        clear_area(frame, area);

        match self.mark {
            Some(Mark::X) => {
                draw_centered('X', frame, area);
            }
            Some(Mark::O) => {
                draw_centered('O', frame, area);
            }
            None => {}
        }
    }

    fn attr(&mut self, _: tuirealm::props::Attribute, _: AttrValue) {
        // not implemented
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::NoChange
    }

    fn query<'a>(
        &'a self,
        _: tuirealm::props::Attribute,
    ) -> Option<tuirealm::props::QueryResult<'a>> {
        None
    }

    fn state(&self) -> tuirealm::state::State {
        State::None
    }
}
