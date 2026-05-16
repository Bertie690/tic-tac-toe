use std::array;

use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget},
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    component::{AppComponent, Component},
    event::{Event, Key, KeyEvent},
    props::{AttrValue, Attribute, QueryResult},
    state::State,
};

use crate::{
    game::{Board, Move, Position},
    renderer::{
        GameUpdate,
        components::cell::CellComponent,
        enums::{InvalidMoveReason, Message, UserEvent},
    },
};

struct BoardComponent {
    board: Board,
    /// The position of the selected cell.
    selected_cell: Position,
    /// The [`CellComponent`]s that make up the board, stored to avoid re-creating them on page draw.
    cells: [[Box<CellComponent>; 3]; 3],

    /// Whether to prevent input from being processed, such as when a game is not active.
    prevent_input: bool,
}

impl BoardComponent {
    pub fn new(board: Board) -> Self {
        let cells = array::from_fn(|_| array::from_fn(|_| Box::new(CellComponent { mark: None })));

        Self {
            board,
            selected_cell: (1, 1),
            cells,
            prevent_input: false,
        }
    }

    /// Split the provided `area` into an equally sized 3x3 grid of `Rect`s representing the cell areas,
    /// applying extra padding as necessary to ensure equal cell dimensions.
    ///
    /// Returns `None` if the provided area is too small to create a sufficiently padded 3x3 grid.
    fn split_area_into_cells(area: Rect) -> Option<[[Rect; 3]; 3]> {
        if area.width == 0 || area.height == 0 {
            return None;
        }

        let board_with_margins = area.inner(Margin::new(1, 1));

        // Truncate board dimensions to the next lowest multiple of 3 to ensure equal sizes,
        // using any leftover as padding
        let usable_width = board_with_margins.width - (board_with_margins.width % 3);
        let usable_height = board_with_margins.height - (board_with_margins.height % 3);

        if usable_width == 0 || usable_height == 0 {
            return None;
        }

        let offset_x = board_with_margins.x + ((board_with_margins.width - usable_width) / 2);
        let offset_y = board_with_margins.y + ((board_with_margins.height - usable_height) / 2);

        let cell_width = usable_width / 3;
        let cell_height = usable_height / 3;

        let cells: [[Rect; 3]; 3] = array::from_fn(|row| {
            array::from_fn(|col| {
                Rect::new(
                    offset_x + (col as u16) * cell_width,
                    offset_y + (row as u16) * cell_height,
                    cell_width,
                    cell_height,
                )
            })
        });

        Some(cells)
    }
}

impl Component for BoardComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let Some(cells) = BoardComponent::split_area_into_cells(area) else {
            return;
        };

        for (row, row_cells) in cells.into_iter().enumerate() {
            for (col, cell_area) in row_cells.into_iter().enumerate() {
                self.cells[row][col].mark = self.board.grid()[(row, col)];

                let is_selected = self.selected_cell == (row, col);
                let border_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style);
                let inner_area = block.inner(cell_area);
                block.render(cell_area, frame.buffer_mut());
                self.cells[row][col].view(frame, inner_area);
            }
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left) => {
                self.selected_cell.1 = (self.selected_cell.1 + 2) % 3;
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Right) => {
                self.selected_cell.1 = (self.selected_cell.1 + 1) % 3;
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Up) => {
                self.selected_cell.0 = (self.selected_cell.0 + 2) % 3;
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                self.selected_cell.0 = (self.selected_cell.0 + 1) % 3;
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // do nothing if game over or already occupied
                // TODO: Send a message to the status bar?
                if self.board.grid()[self.selected_cell].is_some() || self.prevent_input {
                    let reason = if self.prevent_input {
                        InvalidMoveReason::GameOver
                    } else {
                        InvalidMoveReason::CellOccupied
                    };

                    CmdResult::Invalid(Cmd::Custom(reason.into()))
                } else {
                    CmdResult::Submit(self.state())
                }
            }
            _ => CmdResult::NoChange,
        }
    }

    fn query(&self, _: Attribute) -> Option<QueryResult<'_>> {
        None
    }
    fn attr(&mut self, _: Attribute, _: AttrValue) {}
    fn state(&self) -> State {
        State::None
    }
}

#[derive(Component)]
pub struct AppBoardComponent {
    component: BoardComponent,
}

impl AppBoardComponent {
    pub fn new(board: Board) -> Self {
        Self {
            component: BoardComponent::new(board),
        }
    }

    fn handle_key_event(&mut self, ke: &KeyEvent) -> Option<Message> {
        match ke {
            KeyEvent { code: Key::Esc, .. } => Some(Message::FocusSidebar),
            KeyEvent {
                code: Key::Left | Key::Char('h') | Key::Char('a'),
                ..
            } => {
                let _ = self.perform(Cmd::Move(Direction::Left));
                Some(Message::Redraw)
            }
            KeyEvent {
                code: Key::Right | Key::Char('l') | Key::Char('d'),
                ..
            } => {
                let _ = self.perform(Cmd::Move(Direction::Right));
                Some(Message::Redraw)
            }
            KeyEvent {
                code: Key::Up | Key::Char('k') | Key::Char('w'),
                ..
            } => {
                let _ = self.perform(Cmd::Move(Direction::Up));
                Some(Message::Redraw)
            }
            KeyEvent {
                code: Key::Down | Key::Char('j') | Key::Char('s'),
                ..
            } => {
                let _ = self.perform(Cmd::Move(Direction::Down));
                Some(Message::Redraw)
            }
            KeyEvent {
                code: Key::Enter | Key::Char(' '),
                ..
            } => match self.perform(Cmd::Submit) {
                CmdResult::Submit(_) => Some(Message::MoveMade(self.component.selected_cell)),
                CmdResult::Invalid(Cmd::Custom(reason_str)) => {
                    let Ok(reason) = InvalidMoveReason::try_from(reason_str) else {
                        return None;
                    };

                    Some(Message::InvalidMove(reason))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl AppComponent<Message, UserEvent> for AppBoardComponent {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Keyboard(ke) => self.handle_key_event(ke),

            Event::User(UserEvent::GameUpdated(GameUpdate::Initial(board))) => {
                self.component.board = board.clone();
                self.component.selected_cell = (1, 1); // center
                self.component.prevent_input = false;

                Some(Message::Redraw)
            }
            Event::User(UserEvent::GameUpdated(GameUpdate::Move(Move { position, mark }))) => {
                self.component.board.set_mark(*position, *mark);

                Some(Message::Redraw)
            }
            Event::User(UserEvent::GameUpdated(GameUpdate::Finished { board, .. })) => {
                self.component.board = board.clone();
                self.component.prevent_input = true;

                Some(Message::Redraw)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array2;
    use tuirealm::{
        component::AppComponent,
        event::{Event, Key, KeyEvent},
    };

    use super::AppBoardComponent;
    use crate::{
        game::Board,
        renderer::enums::{Message, UserEvent},
    };

    /// Create a new empty board component for testing with an empty board starting at the top-left corner.
    fn empty_board_component() -> AppBoardComponent {
        AppBoardComponent::new(Board::new(Array2::from_elem((3, 3), None)))
    }

    /// Simulate a keypress.
    fn press(board: &mut AppBoardComponent, key: char) -> Option<Message> {
        board.on(&Event::<UserEvent>::Keyboard(KeyEvent::from(Key::Char(
            key,
        ))))
    }

    #[test]
    fn test_wasd_wraparound_from_origin() {
        let mut board = empty_board_component();
        board.component.selected_cell = (0, 0);

        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (0, 2));

        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (2, 2));

        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (2, 0));

        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (0, 0));
    }

    #[test]
    fn test_wasd_wraparound_on_all_edges() {
        let mut board = empty_board_component();
        board.component.selected_cell = (1, 0);

        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (1, 2));

        board.component.selected_cell = (1, 2);
        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (1, 0));

        board.component.selected_cell = (0, 1);
        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (2, 1));

        board.component.selected_cell = (2, 1);
        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (0, 1));
    }

    #[test]
    fn test_wasd_non_wraparound_navigation() {
        let mut board = empty_board_component();
        board.component.selected_cell = (1, 1);

        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (1, 0));

        board.component.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (1, 2));

        board.component.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (0, 1));

        board.component.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.component.selected_cell, (2, 1));
    }

    #[test]
    fn test_split_area_into_cells_dimensions() {
        use ratatui::layout::{Margin, Rect};

        let area = Rect::new(0, 0, 77, 16);
        let cells =
            super::BoardComponent::split_area_into_cells(area).expect("should split correctly");

        // ensure that all the cells have equal dimensions
        let first_cell = cells[0][0];
        for row in cells.iter().flatten().skip(1) {
            assert_eq!(row.width, first_cell.width);
            assert_eq!(row.height, first_cell.height);
        }

        // ensure that the entirety of the usable inner area (rounded down
        // to multiples of 3) is used by the cells
        let board_with_margins = area.inner(Margin::new(1, 1));
        let total_width: u16 = cells[0].iter().map(|r| r.width).sum();
        let total_height: u16 = cells.iter().map(|row| row[0].height).sum();

        let usable_width = board_with_margins.width - (board_with_margins.width % 3);
        let usable_height = board_with_margins.height - (board_with_margins.height % 3);

        assert_eq!(total_width, usable_width);
        assert_eq!(total_height, usable_height);
    }
}
