use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tuirealm::{
    command::{Cmd, CmdResult},
    component::{AppComponent, Component},
    event::{Event, Key, KeyEvent, KeyModifiers},
    props::{AttrValue, Attribute, QueryResult},
    state::State,
};

use crate::{game::GameResult, renderer::{
    GameUpdate,
    enums::{Message, UserEvent},
}};

/// A small two-line component used to display messages pertaining to game state.
///
/// Receives game update messages, but does not emit any itself.
pub struct StatusComponent {
    message: String,

    /// The warning message shown when Ctrl+C is pressed, if applicable.
    ///
    /// Set to [`None`] when any other key is pressed, and will override the regular message .
    warning_message: Option<String>,
}

impl StatusComponent {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            warning_message: None,
        }
    }

    fn set(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
    }

    fn get_display_line(&'_ self) -> Line<'_> {
        if let Some(warning) = &self.warning_message {
            Line::from(Span::styled(
                warning,
                Style::default().fg(Color::Red).bold(),
            ))
        } else {
            Line::from(Span::raw(&self.message))
        }
    }
}

impl Component for StatusComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Status ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, frame.buffer_mut());

        let lines = vec![Line::from(Span::raw(" ")), self.get_display_line()];

        frame.render_widget(Paragraph::new(lines), inner);
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::NoChange
    }

    fn query(&self, _attr: Attribute) -> Option<QueryResult<'_>> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _val: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }
}

/// Thin wrapper to integrate `StatusComponent` into the application event loop.
#[derive(Component)]
pub struct AppStatusComponent {
    component: StatusComponent,
}

impl AppStatusComponent {
    pub fn new() -> Self {
        Self {
            component: StatusComponent::new(),
        }
    }

    fn handle_user_event(&mut self, user_event: &GameUpdate) {
        match user_event {
            GameUpdate::Initial(_) => {
                self.component.set("New game started");
            }
            GameUpdate::Move(mv) => {
                self.component.set(format!(
                    "{} moved to ({},{})",
                    mv.mark,
                    mv.position.0 + 1,
                    mv.position.1 + 1
                ));
            }
            GameUpdate::Finished { result, .. } => {
                let msg = match result {
                    GameResult::Draw => String::from("Game over: Draw"),
                    GameResult::Winner(m) => format!("Game over: {} wins!", m),
                    GameResult::Ongoing => unreachable!(),
                };
                self.component.set(msg);
            }
        }
    }
}

impl AppComponent<Message, UserEvent> for AppStatusComponent {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Message> {
        match ev {
            Event::User(UserEvent::GameUpdated(update)) => {
                self.handle_user_event(update);
                Some(Message::Redraw)
            }

            //
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) if self.component.warning_message.is_some() => Some(Message::AppQuit),
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) if self.component.warning_message.is_none() => {
                self.component.warning_message = Some(String::from("Press Ctrl+C again to quit"));
                Some(Message::Redraw)
            }
            Event::Keyboard(_) => {
                if self.component.warning_message.is_some() {
                    self.component.warning_message = None;
                    Some(Message::Redraw)
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}
