use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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
    let views = ["Contexts", "Sessions", "Patterns", "Providers", "Models"];
    let mut active_view = 0;

    loop {
        terminal.draw(|f| {
            // Layout setup
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(6), // Combined Header and Actions
                        Constraint::Length(3), // Command line
                        Constraint::Min(0),    // Working space
                    ]
                        .as_ref(),
                )
                .split(f.size());

            // Combined Header and Actions
            let mut header_lines = vec![
                Line::from(views.iter().enumerate().map(|(i, view)| {
                    let style = if i == active_view {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    Span::styled(format!("({}) {}", i + 1, view), style)
                }).collect::<Vec<_>>())
            ];

            // Add specific actions if "Providers" view is selected
            if views[active_view] == "Providers" {
                let actions = vec![
                    Span::styled("(a) Add", Style::default().fg(Color::Green)),
                    Span::raw(" | "),
                    Span::styled("(e) Edit", Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::styled("(d) Delete", Style::default().fg(Color::Red)),
                ];
                header_lines.push(Line::from(actions));
            }

            let header = Paragraph::new(header_lines)
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            // Command Line
            let command_line = Paragraph::new("Command: ")
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title(": Command Line"));
            f.render_widget(command_line, chunks[1]);

            // Workspace
            let workspace_text = format!("This is the {} view", views[active_view]);
            let workspace = Paragraph::new(workspace_text)
                .block(Block::default().borders(Borders::ALL).title("Workspace"));
            f.render_widget(workspace, chunks[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => {
                        // Switch view with Tab key
                        active_view = (active_view + 1) % views.len();
                    }
                    KeyCode::Char('1') => active_view = 0,
                    KeyCode::Char('2') => active_view = 1,
                    KeyCode::Char('3') => active_view = 2,
                    KeyCode::Char('4') => active_view = 3,
                    KeyCode::Char('5') => active_view = 4,
                    KeyCode::Char('a') if views[active_view] == "Providers" => {
                        // Handle Add action for Providers
                        println!("Add action triggered"); // Debugging line
                    }
                    KeyCode::Char('e') if views[active_view] == "Providers" => {
                        // Handle Edit action for Providers
                        println!("Edit action triggered"); // Debugging line
                    }
                    KeyCode::Char('d') if views[active_view] == "Providers" => {
                        // Handle Delete action for Providers
                        println!("Delete action triggered"); // Debugging line
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
