use ratatui::{Frame, layout::Rect};
use tuirealm::{
    command::{Cmd, CmdResult},
    component::Component,
    props::AttrValue,
    state::{State, StateValue},
};

use crate::game::Mark;

fn clear_area(frame: &mut Frame, area: Rect) {
    let buf = frame.buffer_mut();
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            buf[(x, y)].set_char(' ');
        }
    }
}

fn draw_x(frame: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // small cell fallback: render one centered glyph.
    if area.width < 3 || area.height < 3 {
        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;
        frame.buffer_mut()[(center_x, center_y)].set_char('X');
        return;
    }

    // Work in local floating-point coordinates so the diagonals scale to any cell size.
    let cell_width = area.width as f64;
    let cell_height = area.height as f64;
    let max_x_index = cell_width - 1.0;
    let max_y_index = cell_height - 1.0;
    // Stroke width grows with cell size, with a 1-char minimum for small cells.
    let stroke_width = (cell_width.min(cell_height) / 6.0).max(1.0);
    let half_stroke_width = stroke_width / 2.0;

    let buf = frame.buffer_mut();
    for dy in 0..area.height {
        for dx in 0..area.width {
            let point_x = dx as f64;
            let point_y = dy as f64;

            // create a pair of diagonal strokes: top-left -> bottom-right and top-right -> bottom-left.
            let first_diagonal_x = point_y * max_x_index / max_y_index;
            let second_diagonal_x = max_x_index - first_diagonal_x;
            let on_first_diagonal = (point_x - first_diagonal_x).abs() <= half_stroke_width;
            let on_second_diagonal = (point_x - second_diagonal_x).abs() <= half_stroke_width;

            // draw the glyph if on either diagonal
            if on_first_diagonal || on_second_diagonal {
                let ch = if on_first_diagonal && on_second_diagonal {
                    'X'
                } else if on_first_diagonal {
                    '\\'
                } else {
                    '/'
                };
                buf[(area.x + dx, area.y + dy)].set_char(ch);
            }
        }
    }
}

fn draw_o(frame: &mut Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // small cell fallback: render one centered glyph
    if area.width < 4 || area.height < 4 {
        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;
        frame.buffer_mut()[(center_x, center_y)].set_char('O');
        return;
    }

    // Model O as an elliptical ring with an outer and inner border
    let cell_width = area.width as f64;
    let cell_height = area.height as f64;
    let center_x = (cell_width - 1.0) / 2.0;
    let center_y = (cell_height - 1.0) / 2.0;
    let radius_x = (cell_width - 1.0) / 2.0;
    let radius_y = (cell_height - 1.0) / 2.0;
    // Ring thickness scales with cell size
    let stroke_width = (cell_width.min(cell_height) / 7.0).max(1.0);
    let inner_radius_x = (radius_x - stroke_width).max(0.25);
    let inner_radius_y = (radius_y - stroke_width).max(0.25);

    let buf = frame.buffer_mut();
    for iy in 0..area.height {
        for ix in 0..area.width {
            let point_x = ix as f64;
            let point_y = iy as f64;
            let dx = point_x - center_x;
            let dy = point_y - center_y;

            // Inside the outer ellipse but outside the inner ellipse => ring pixel.
            let outer_ellipse_distance =
                (dx * dx) / (radius_x * radius_x) + (dy * dy) / (radius_y * radius_y);
            let inner_ellipse_distance = (dx * dx) / (inner_radius_x * inner_radius_x)
                + (dy * dy) / (inner_radius_y * inner_radius_y);

            if outer_ellipse_distance <= 1.0 && inner_ellipse_distance >= 1.0 {
                buf[(area.x + ix, area.y + iy)].set_char('O');
            }
        }
    }
}

/// A CellComponent displays the contents of a single cell on the board.
pub struct CellComponent {
    pub mark: Option<Mark>,
}

impl Component for CellComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        clear_area(frame, area);

        match self.mark {
            Some(Mark::X) => draw_x(frame, area),
            Some(Mark::O) => draw_o(frame, area),
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
        State::Single(StateValue::String(
            match self.mark {
                Some(Mark::X) => "X",
                Some(Mark::O) => "O",
                None => "",
            }
            .into(),
        ))
    }
}
