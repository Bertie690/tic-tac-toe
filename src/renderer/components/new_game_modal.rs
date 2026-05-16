use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    component::{AppComponent, Component},
    event::{Event, Key, KeyEvent},
    props::{AttrValue, Attribute, QueryResult},
    state::State,
};

use crate::{
    game::{GameConfig, Mark, OpponentKind},
    renderer::enums::{Message, UserEvent},
};

/// Enum representing the rows of the new game modal
#[derive(Clone, Copy, PartialEq, Eq)]
enum ModalRow {
    Mark,
    Opponent,
    FirstMove,
    Start,
}

impl ModalRow {
    fn next(self) -> Self {
        match self {
            Self::Mark => Self::Opponent,
            Self::Opponent => Self::FirstMove,
            Self::FirstMove => Self::Start,
            Self::Start => Self::Mark,
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::Mark => Self::Start,
            Self::Opponent => Self::Mark,
            Self::FirstMove => Self::Opponent,
            Self::Start => Self::FirstMove,
        }
    }
}

/// A [`NewGameModalComponent`] is responsible for managing the "new game" popup that appears when starting a new game.
struct NewGameModalComponent {
    /// The selected mark for the player (X or O).
    player_mark: Mark,
    /// The kind of opponent to play against.
    opponent: OpponentKind,
    /// Whether the player should make the first move.
    player_first: bool,
    /// The currently selected row in the modal.
    row: ModalRow,
    /// Whether the modal is focused.
    focused: bool,
}

impl NewGameModalComponent {
    fn new() -> Self {
        Self {
            player_mark: Mark::X,
            opponent: OpponentKind::Minimax,
            row: ModalRow::Mark,
            focused: false,
            player_first: true,
        }
    }

    fn toggle_mark(&mut self) {
        self.player_mark = self.player_mark.opposite();
    }

    fn toggle_opponent(&mut self) {
        self.opponent = match self.opponent {
            OpponentKind::Minimax => OpponentKind::Random,
            OpponentKind::Random => OpponentKind::Minimax,
        };
    }

    fn toggle_first_move(&mut self) {
        self.player_first = !self.player_first;
    }

    /// Build a [`GameConfig`] based on the current state of the modal.
    fn build_config(&self) -> GameConfig {
        GameConfig {
            player_mark: self.player_mark,
            opponent: self.opponent,
            player_first: self.player_first,
        }
    }
}

impl Component for NewGameModalComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Clear whatever was drawn behind the modal
        frame.render_widget(Clear, area);

        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let block = Block::default()
            .title(" New Game ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, frame.buffer_mut());

        // Styles
        let normal = Style::default();
        let row_cursor = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let selected = Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        let unselected = Style::default().fg(Color::DarkGray);

        let mark_row_style = if self.focused && self.row == ModalRow::Mark {
            row_cursor
        } else {
            normal
        };
        let opp_row_style = if self.focused && self.row == ModalRow::Opponent {
            row_cursor
        } else {
            normal
        };
        let first_move_row_style = if self.focused && self.row == ModalRow::FirstMove {
            row_cursor
        } else {
            normal
        };
        let start_style = if self.focused && self.row == ModalRow::Start {
            selected
        } else {
            unselected
        };

        let x_style = if self.player_mark == Mark::X {
            selected
        } else {
            normal
        };
        let o_style = if self.player_mark == Mark::O {
            selected
        } else {
            normal
        };
        let minimax_style = if self.opponent == OpponentKind::Minimax {
            selected
        } else {
            normal
        };
        let random_style = if self.opponent == OpponentKind::Random {
            selected
        } else {
            normal
        };

        let lines = vec![
            Line::from(vec![
                Span::styled("  Mark:         ", mark_row_style),
                Span::styled(" X ", x_style),
                Span::raw("  "),
                Span::styled(" O ", o_style),
            ]),
            Line::from(vec![
                Span::styled("  Enemy Type:   ", opp_row_style),
                Span::styled(" Minimax ", minimax_style),
                Span::raw("  "),
                Span::styled(" Random ", random_style),
            ]),
            Line::from(vec![
                Span::styled("  First Move:   ", first_move_row_style),
                Span::styled(
                    " Player ",
                    if self.player_first { selected } else { normal },
                ),
                Span::raw("  "),
                Span::styled(" CPU ", if !self.player_first { selected } else { normal }),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(" Start Game!  ", start_style),
            ]),
        ];

        frame.render_widget(Paragraph::new(lines), inner);
    }

    fn attr(&mut self, attr: Attribute, val: AttrValue) {
        if attr == Attribute::Focus
            && let AttrValue::Flag(focused) = val
        {
            self.focused = focused;
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Up) => {
                self.row = self.row.prev();
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                self.row = self.row.next();
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Left) | Cmd::Move(Direction::Right) => {
                match self.row {
                    ModalRow::Mark => self.toggle_mark(),
                    ModalRow::Opponent => self.toggle_opponent(),
                    ModalRow::FirstMove => self.toggle_first_move(),
                    ModalRow::Start => {}
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => CmdResult::Submit(self.state()),
            _ => CmdResult::NoChange,
        }
    }

    fn query(&self, _: Attribute) -> Option<QueryResult<'_>> {
        None
    }

    fn state(&self) -> State {
        State::None
    }
}

/// A thin wrapper around [`NewGameModalComponent`] that integrates it into the
/// tuirealm application event loop.
#[derive(Component)]
pub struct AppNewGameModal {
    component: NewGameModalComponent,
}

impl Default for AppNewGameModal {
    fn default() -> Self {
        Self::new()
    }
}

impl AppNewGameModal {
    pub fn new() -> Self {
        Self {
            component: NewGameModalComponent::new(),
        }
    }

    fn handle_key_event(&mut self, ke: &KeyEvent) -> Option<Message> {
        match ke {
            KeyEvent { code: Key::Esc, .. } => Some(Message::CloseModal),

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
                code: Key::Enter | Key::Char(' '),
                ..
            } => {
                // Submit always; if cursor is on a value row, also advance down
                let config = self.component.build_config();
                match self.component.row {
                    ModalRow::Start => Some(Message::StartGame(config)),
                    _ => {
                        let _ = self.perform(Cmd::Move(Direction::Down));
                        Some(Message::Redraw)
                    }
                }
            }
            _ => None,
        }
    }
}

impl AppComponent<Message, UserEvent> for AppNewGameModal {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Keyboard(ke) => self.handle_key_event(ke),
            _ => None,
        }
    }
}
