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

use crate::renderer::message::{Message, UserEvent};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SidebarOption {
    /// Open a new game. Pulls up a modal to configure players/difficulty.
    NewGame,
    /// Forcibly quit the application.
    Quit,
}

impl SidebarOption {
    const ALL: [Self; 2] = [Self::NewGame, Self::Quit];

    fn label(self) -> &'static str {
        match self {
            Self::NewGame => "New Game",
            Self::Quit => "Quit",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::NewGame => 0,
            Self::Quit => 1,
        }
    }

    fn from_index(i: usize) -> Self {
        Self::ALL[i]
    }
}

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
                    format!("{}{}", prefix, opt.label()),
                    style,
                )))
            })
            .collect();

        frame.render_widget(List::new(items), inner);
    }

    fn attr(&mut self, attr: Attribute, val: AttrValue) {
        if attr == Attribute::Focus
        && let AttrValue::Flag(focused) = val {
            self.focused = focused;
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Up) => {
                let idx = self.selected_option.index();
                self.selected_option = SidebarOption::from_index(
                    (idx + SidebarOption::ALL.len() - 1) % SidebarOption::ALL.len(),
                );
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                let idx = self.selected_option.index();
                self.selected_option =
                    SidebarOption::from_index((idx + 1) % SidebarOption::ALL.len());
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

#[derive(Component)]
pub struct AppSidebarComponent {
    component: SidebarComponent,
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
            } => match self.perform(Cmd::Submit) {
                CmdResult::Submit(_) => match self.component.selected_option {
                    SidebarOption::NewGame => Some(Message::NewGame),
                    SidebarOption::Quit => Some(Message::AppQuit),
                },
                _ => None,
            },
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