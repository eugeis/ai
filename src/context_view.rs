use crate::traits::View;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::fs;
use crossterm::event::KeyCode;

pub struct ContextView {
    context_files: Vec<&'static str>,
    selected_context: usize,
    context_list_state: ListState,
    file_content: String,
}

impl ContextView {
    pub fn new() -> Self {
        let context_files = vec!["file1.txt", "file2.txt", "file3.txt"];
        let mut context_list_state = ListState::default();
        context_list_state.select(Some(0));
        let file_content = fs::read_to_string(context_files[0]).unwrap_or_default();

        Self {
            context_files,
            selected_context: 0,
            context_list_state,
            file_content,
        }
    }
}

impl View for ContextView {
    fn render(&self, f: &mut Frame, area: ratatui::layout::Rect, _info_message: &str)
    {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(85),
                ]
                    .as_ref(),
            )
            .split(area);

        let items: Vec<ListItem> = self.context_files.iter().map(|f| {
            let content = vec![Line::from(Span::raw(*f))];
            ListItem::new(content)
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Contexts"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, cols[0], &mut self.context_list_state.clone());

        let file_view = Paragraph::new(self.file_content.as_str())
            .block(Block::default().borders(Borders::ALL).title("Content"))
            .style(Style::default().fg(Color::White));

        f.render_widget(file_view, cols[1]);
    }

    fn handle_input(&mut self, key: crossterm::event::KeyEvent, info_message: &mut String) {
        match key.code {
            KeyCode::Down => {
                if self.selected_context < self.context_files.len() - 1 {
                    self.selected_context += 1;
                    self.context_list_state.select(Some(self.selected_context));
                    self.file_content = fs::read_to_string(self.context_files[self.selected_context]).unwrap_or_default();
                }
            }
            KeyCode::Up => {
                if self.selected_context > 0 {
                    self.selected_context -= 1;
                    self.context_list_state.select(Some(self.selected_context));
                    self.file_content = fs::read_to_string(self.context_files[self.selected_context]).unwrap_or_default();
                }
            }
            _ => {}
        }
    }
}
