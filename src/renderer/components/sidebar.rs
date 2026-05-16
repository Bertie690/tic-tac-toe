use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    component::{AppComponent, Component},
    event::{Event, Key, KeyEvent},
    props::{AttrValue, Attribute, QueryResult},
    state::State,
};

use crate::renderer::enums::{Message, UserEvent};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidebarOption {
    /// Open a new game. Pulls up a modal to configure players/difficulty.
    NewGame = 0,
    /// Forcibly quit the application.
    Quit = 1,
}

impl SidebarOption {
    const ALL: [SidebarOption; 2] = [SidebarOption::NewGame, SidebarOption::Quit];

    /// Obtain the next sidebar option in numerical order.
    fn next(&self) -> Self {
        match self {
            SidebarOption::NewGame => SidebarOption::Quit,
            SidebarOption::Quit => SidebarOption::NewGame,
        }
    }

    /// Obtain the previous sidebar option in numerical order.
    fn prev(&self) -> Self {
        match self {
            SidebarOption::NewGame => SidebarOption::Quit,
            SidebarOption::Quit => SidebarOption::NewGame,
        }
    }
}

impl std::fmt::Display for SidebarOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SidebarOption::NewGame => write!(f, "New Game"),
            SidebarOption::Quit => write!(f, "Quit"),
        }
    }
}

impl From<SidebarOption> for Message {
    fn from(value: SidebarOption) -> Self {
        match value {
            SidebarOption::NewGame => Message::OpenNewGameModal,
            SidebarOption::Quit => Message::AppQuit,
        }
    }
}

/// A [`SidebarComponent`] is responsible for managing the right sidebar and its various options.
struct SidebarComponent {
    selected_option: SidebarOption,
    focused: bool,
}

impl SidebarComponent {
    fn new() -> Self {
        Self {
            selected_option: SidebarOption::NewGame,
            focused: false,
        }
    }
}

impl Component for SidebarComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Menu ")
            .borders(Borders::ALL)
            .border_style(if self.focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        let inner = block.inner(area);
        block.render(area, frame.buffer_mut());

        let items: Vec<ListItem> = SidebarOption::ALL
            .iter()
            .map(|&opt| {
                let is_selected = self.focused && opt == self.selected_option;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                let prefix = if is_selected { "» " } else { "  " };
                ListItem::new(Line::from(Span::styled(
                    format!("{}{}", prefix, opt),
                    style,
                )))
            })
            .collect();

        frame.render_widget(List::new(items), inner);
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
                self.selected_option = self.selected_option.prev();
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                self.selected_option = self.selected_option.next();
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

/// An [`AppSidebarComponent`] is a thin wrapper around a [`SidebarComponent`].
#[derive(Component)]
pub struct AppSidebarComponent {
    component: SidebarComponent,
}

impl Default for AppSidebarComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl AppSidebarComponent {
    pub fn new() -> Self {
        Self {
            component: SidebarComponent::new(),
        }
    }

    fn handle_key_event(&mut self, ke: &KeyEvent) -> Option<Message> {
        match ke {
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
            } => {
                let CmdResult::Submit(_) = self.perform(Cmd::Submit) else {
                    return None;
                };
                Some(self.component.selected_option.into())
            }
            KeyEvent {
                code: Key::Tab | Key::Esc,
                ..
            } => Some(Message::FocusBoard),
            _ => None,
        }
    }
}

impl AppComponent<Message, UserEvent> for AppSidebarComponent {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::Keyboard(ke) => self.handle_key_event(ke),
            _ => None,
        }
    }
}
