mod db;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application state
    let mut command_input = String::new();
    let views = ["Contexts", "Sessions", "Patterns", "Vendors", "Models"];
    let mut active_view = 0;
    let context_files = vec!["file1.txt", "file2.txt", "file3.txt"]; // Replace with actual file names
    let mut selected_context = 0;
    let mut list_state = ListState::default(); // Initialize ListState
    list_state.select(Some(selected_context));
    let mut file_content = String::new();
    if let Ok(content) = fs::read_to_string(context_files[selected_context]) {
        file_content = content;
    }

    loop {
        terminal.draw(|f| {
            // Layout setup
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Header
                        Constraint::Length(3), // Command line
                        Constraint::Min(0),    // Working space
                    ]
                        .as_ref(),
                )
                .split(f.size());

            // Header
            let header = Paragraph::new(Line::from(views.iter().enumerate().map(|(i, view)| {
                let style = if i == active_view {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                Span::styled(format!("({}) {}", i + 1, view), style)
            }).collect::<Vec<_>>()))
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            // Command Line
            let command_line = Paragraph::new(Text::raw(command_input.as_str()))
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title(": Command Line"));
            f.render_widget(command_line, chunks[1]);

            // Workspace
            match views[active_view] {
                "Contexts" => {
                    let cols = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Percentage(15), // File list column
                                Constraint::Percentage(85), // File content column
                            ]
                                .as_ref(),
                        )
                        .split(chunks[2]);

                    // Left column: File list
                    let items: Vec<ListItem> = context_files.iter().map(|f| {
                        let content = vec![Line::from(Span::raw(*f))];
                        ListItem::new(content)
                    }).collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Contexts"))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ");
                    f.render_stateful_widget(list, cols[0], &mut list_state);

                    // Right column: File content
                    let file_view = Paragraph::new(file_content.as_str())
                        .block(Block::default().borders(Borders::ALL).title("Content"))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(file_view, cols[1]);
                }
                _ => {
                    let workspace = Paragraph::new(format!("This is the {} view", views[active_view]))
                        .block(Block::default().borders(Borders::ALL).title("Workspace"));
                    f.render_widget(workspace, chunks[2]);
                }
            };
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(':') => {
                        // Start typing command
                        command_input.push(':');
                    }
                    KeyCode::Char(c) if command_input.starts_with(':') => {
                        // Continue typing command
                        command_input.push(c);
                    }
                    KeyCode::Enter if command_input.starts_with(':') => {
                        // Handle command input here
                        command_input.clear();
                    }
                    KeyCode::Tab => {
                        // Switch view with Tab key
                        active_view = (active_view + 1) % views.len();
                    }
                    KeyCode::Char('1') => active_view = 0, // Shortcut for Contexts
                    KeyCode::Char('2') => active_view = 1, // Shortcut for Sessions
                    KeyCode::Char('3') => active_view = 2, // Shortcut for Patterns
                    KeyCode::Char('4') => active_view = 3, // Shortcut for Vendors
                    KeyCode::Char('5') => active_view = 4, // Shortcut for Models
                    KeyCode::Down if active_view == 0 => {
                        // Navigate down in the file list
                        if selected_context < context_files.len() - 1 {
                            selected_context += 1;
                            list_state.select(Some(selected_context));
                            if let Ok(content) = fs::read_to_string(context_files[selected_context]) {
                                file_content = content;
                            }
                        }
                    }
                    KeyCode::Up if active_view == 0 => {
                        // Navigate up in the file list
                        if selected_context > 0 {
                            selected_context -= 1;
                            list_state.select(Some(selected_context));
                            if let Ok(content) = fs::read_to_string(context_files[selected_context]) {
                                file_content = content;
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Exit on escape key
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
