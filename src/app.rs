use std::io;
use crate::{context_view::ContextView, provider_view::ProviderView, traits::View};
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use std::time::Duration;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::{Line, Span};

#[derive(Clone)]
pub enum AppView {
    ContextView,
    ProviderView,
}

pub struct App {
    views: Vec<Box<dyn View>>,
    active_view: usize,
    command_input: String,
    info_message: String,
    current_view: AppView,
}

impl App {
    pub fn new() -> Self {
        Self {
            views: vec![
                Box::new(ContextView::new()),
                Box::new(ProviderView::new()),
            ],
            active_view: 0,
            command_input: String::new(),
            info_message: String::new(),
            current_view: AppView::ContextView,
        }
    }

    pub fn run<B: ratatui::backend::Backend>(&mut self, mut terminal: Terminal<B>) -> Result<(), io::Error> {
        loop {
            terminal.draw(|f| {
                let size = f.size();
                let active_style = Style::default().fg(Color::Green).add_modifier(Modifier::BOLD);
                let inactive_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(0),
                        ]
                            .as_ref(),
                    )
                    .split(size);

                let header_line = Line::from(vec![
                    Span::styled("Context View [1]", if matches!(self.current_view, AppView::ContextView) { active_style } else { inactive_style }),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("Provider View [2]", if matches!(self.current_view, AppView::ProviderView) { active_style } else { inactive_style }),
                ]);
                let header = Paragraph::new(header_line)
                    .block(Block::default().borders(Borders::ALL).title("Header"));
                f.render_widget(header, layout[0]);

                self.views[self.active_view].render(f, layout[1], &self.info_message);
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(':') => {
                            self.command_input.push(':');
                        }
                        KeyCode::Char(c) if self.command_input.starts_with(':') => {
                            self.command_input.push(c);
                        }
                        KeyCode::Enter if self.command_input.starts_with(':') => {
                            self.command_input.clear();
                        }
                        KeyCode::Tab => {
                            self.active_view = (self.active_view + 1) % self.views.len();
                            self.current_view = match self.active_view {
                                0 => AppView::ContextView,
                                1 => AppView::ProviderView,
                                _ => self.current_view.clone(),
                            };
                        }
                        KeyCode::Char('1') => {
                            self.active_view = 0;
                            self.current_view = AppView::ContextView;
                        }
                        KeyCode::Char('2') => {
                            self.active_view = 1;
                            self.current_view = AppView::ProviderView;
                        }
                        KeyCode::Esc => break,
                        _ => self.views[self.active_view].handle_input(key, &mut self.info_message),
                    }
                }
            }
        }
        Ok(())
    }
}

