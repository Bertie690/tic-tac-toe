use std::array;

use ratatui::{
    Frame,
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget},
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    component::{AppComponent, Component},
    event::{Event, Key, KeyEvent},
    props::{AttrValue, Attribute, QueryResult},
    state::{State, StateValue},
};

use crate::{
    game::{Board, GameResult, Position},
    renderer::{
        GameUpdate,
        components::cell::CellComponent,
        message::{Message, UserEvent},
    },
};

pub struct BoardComponent {
    board: Board,
    /// The position of the selected cell.
    selected_cell: Position,
    game_result: GameResult,
    /// The [`CellComponent`]s that make up the board, stored to avoid re-creating them on page draw.
    cells: [[Box<CellComponent>; 3]; 3],
}

impl BoardComponent {
    pub fn new(board: Board) -> Self {
        let cells = array::from_fn(|_| {
            array::from_fn(|_|
                // CellComponents will be filled in with the correct marks in the view() method.
                Box::new(CellComponent { mark: None }))
        });

        Self {
            board,
            selected_cell: (0, 0),
            game_result: GameResult::Ongoing,
            cells,
        }
    }

    /// Ensure the cell being selected is valid, manually selecting the next available cell if not.
    fn normalize_selection(&mut self) {
        if self.board.grid()[self.selected_cell].is_none() {
            return;
        }

        if let Some(next_open) = self.board.available_moves().next() {
            self.selected_cell = next_open;
        }
    }
}

impl Component for BoardComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let side = area.width.min(area.height);
        if side == 0 {
            return;
        }

        let square_area = Rect::new(
            area.x + (area.width - side) / 2,
            area.y + (area.height - side) / 2,
            side,
            side,
        );

        let board_rows = square_area.layout_vec(&Layout::new(
            LayoutDirection::Vertical,
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ],
        ));

        for (row, row_area) in board_rows.into_iter().enumerate() {
            let row_cells = row_area.layout_vec(&Layout::new(
                LayoutDirection::Horizontal,
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ],
            ));

            for (col, cell_area) in row_cells.into_iter().enumerate() {
                self.cells[row][col].mark = self.board.grid()[(row, col)];

                let is_selected = self.selected_cell == (row, col);
                let border_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
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
        println!("Performing command: {:?}", cmd);
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
                if self.game_result != GameResult::Ongoing
                    || self.board.grid()[self.selected_cell].is_some()
                {
                    CmdResult::NoChange
                } else {
                    CmdResult::Submit(State::None)
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
        State::Single(StateValue::String(match self.game_result {
            GameResult::Winner(mark) => format!("{} wins!", mark),
            GameResult::Draw => "It's a draw!".into(),
            GameResult::Ongoing => "Game in progress".into(),
        }))
    }
}

impl AppComponent<Message, UserEvent> for BoardComponent {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
                // TODO: Don't quit the app and instead focus on the quit button?
            }) => Some(Message::AppQuit),
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Char('h') | Key::Char('a'),
                ..
            }) => {
                let _ = self.perform(Cmd::Move(Direction::Left));
                Some(Message::Redraw)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right | Key::Char('l') | Key::Char('d'),
                ..
            }) => {
                let _ = self.perform(Cmd::Move(Direction::Right));
                Some(Message::Redraw)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Up | Key::Char('k') | Key::Char('w'),
                ..
            }) => {
                let _ = self.perform(Cmd::Move(Direction::Up));
                Some(Message::Redraw)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down | Key::Char('j') | Key::Char('s'),
                ..
            }) => {
                let _ = self.perform(Cmd::Move(Direction::Down));
                Some(Message::Redraw)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter | Key::Char(' '),
                ..
            }) => match self.perform(Cmd::Submit) {
                CmdResult::Submit(_) => Some(Message::MoveMade(self.selected_cell)),
                _ => None,
            },
            Event::User(UserEvent::GameUpdated(GameUpdate::Move { board })) => {
                self.board = board.clone();
                self.game_result = GameResult::Ongoing;
                self.normalize_selection();
                Some(Message::Redraw)
            }
            Event::User(UserEvent::GameUpdated(GameUpdate::Finished { board, result })) => {
                self.board = board.clone();
                self.game_result = *result;
                self.normalize_selection();
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

    use super::BoardComponent;
    use crate::{
        game::Board,
        renderer::message::{Message, UserEvent},
    };

    /// Create a new empty board component for testing with an empty board starting at the top-left corner.
    fn empty_board_component() -> BoardComponent {
        BoardComponent::new(Board::new(Array2::from_elem((3, 3), None)))
    }

    /// Simulate a keypress.
    fn press(board: &mut BoardComponent, key: char) -> Option<Message> {
        board.on(&Event::<UserEvent>::Keyboard(KeyEvent::from(Key::Char(
            key,
        ))))
    }

    #[test]
    fn test_wasd_wraparound_from_origin() {
        let mut board = empty_board_component();
        assert_eq!(board.selected_cell, (0, 0));

        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (0, 2));

        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (2, 2));

        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (2, 0));

        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (0, 0));
    }

    #[test]
    fn test_wasd_wraparound_on_all_edges() {
        let mut board = empty_board_component();

        board.selected_cell = (1, 0);
        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (1, 2));

        board.selected_cell = (1, 2);
        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (1, 0));

        board.selected_cell = (0, 1);
        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (2, 1));

        board.selected_cell = (2, 1);
        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (0, 1));
    }

    #[test]
    fn test_wasd_non_wraparound_navigation() {
        let mut board = empty_board_component();
        board.selected_cell = (1, 1);

        assert_eq!(press(&mut board, 'a'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (1, 0));

        board.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 'd'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (1, 2));

        board.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 'w'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (0, 1));

        board.selected_cell = (1, 1);
        assert_eq!(press(&mut board, 's'), Some(Message::Redraw));
        assert_eq!(board.selected_cell, (2, 1));
    }
}
