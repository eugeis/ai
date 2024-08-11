
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;

#[derive(Clone, Debug)]
struct ProviderInstance {
    name: String,
    provider_type: ProviderType,
}

#[derive(Clone, Debug)]
enum ProviderType {
    OpenAI,
    Ollama,
    AzureOpenAI,
    Gemini,
    Grog,
    Claude,
}

#[derive(Clone, Debug)]
struct ProviderSettings {
    api_key: Option<String>,
    api_entry_point: Option<String>,
    api_deployment: Option<String>,
}

struct AppState {
    provider_instances: HashMap<String, ProviderInstance>,
    provider_settings: HashMap<String, ProviderSettings>,
    selected_provider_type: Option<ProviderType>,
    adding_provider: bool,
    editing_provider: Option<String>,
    deleting_provider: Option<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            provider_instances: HashMap::new(),
            provider_settings: HashMap::new(),
            selected_provider_type: None,
            adding_provider: false,
            editing_provider: None,
            deleting_provider: None,
        }
    }

    fn add_provider(&mut self, provider: ProviderInstance, settings: ProviderSettings) {
        let provider_name = provider.name.clone();
        self.provider_instances.insert(provider_name.clone(), provider);
        self.provider_settings.insert(provider_name, settings);
    }

    fn get_provider_names(&self) -> Vec<String> {
        self.provider_instances.keys().cloned().collect()
    }

    fn remove_provider(&mut self, name: &str) {
        self.provider_instances.remove(name);
        self.provider_settings.remove(name);
    }

    fn edit_provider(&mut self, name: &str, new_provider: ProviderInstance, new_settings: ProviderSettings) {
        self.provider_instances.insert(name.to_string(), new_provider);
        self.provider_settings.insert(name.to_string(), new_settings);
    }
}

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application state
    let mut command_input = String::new();
    let mut info_message = String::new(); // Add info_message to store information messages
    let views = ["Contexts", "Sessions", "Patterns", "Providers", "Models"];
    let mut active_view = 0; // Start with the first view selected
    let context_files = vec!["file1.txt", "file2.txt", "file3.txt"]; // Replace with actual file names
    let mut selected_context = 0;
    let mut context_list_state = ListState::default(); // Initialize ListState for contexts
    context_list_state.select(Some(selected_context));
    let mut file_content = String::new();
    if let Ok(content) = fs::read_to_string(context_files[selected_context]) {
        file_content = content;
    }

    let mut app_state = AppState::new();

    // Add a few sample providers for testing
    app_state.add_provider(
        ProviderInstance {
            name: "SampleProvider1".to_string(),
            provider_type: ProviderType::OpenAI,
        },
        ProviderSettings {
            api_key: Some("sample_api_key".to_string()),
            api_entry_point: None,
            api_deployment: None,
        },
    );

    let mut provider_list_state = ListState::default(); // Initialize ListState for providers
    provider_list_state.select(Some(0));

    loop {
        // Clone necessary data before entering the closure
        let editing_provider = app_state.editing_provider.clone();
        let deleting_provider = app_state.deleting_provider.clone();
        let adding_provider = app_state.adding_provider;
        let provider_instances = app_state.provider_instances.clone();
        let provider_settings = app_state.provider_settings.clone();
        let provider_names_vec = app_state.get_provider_names();

        terminal.draw(|f| {
            let constraints: Vec<Constraint> = vec![
                Constraint::Length(1), // Header (always allocate space)
                Constraint::Length(1), // Actions (only for Providers)
                Constraint::Length(3), // Command line
                Constraint::Min(0),    // Working space
            ];
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_slice()) // Explicitly specify type
                .split(f.size());

            // Header
            let header_spans: Vec<Span> = views.iter().enumerate().map(|(i, view)| {
                Span::styled(
                    format!("({}) {} ", i + 1, view),
                    if i == active_view {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    },
                )
            }).collect();
            let header_line = Line::from(header_spans);
            let header = Paragraph::new(header_line);
            f.render_widget(header, chunks[0]);

            // Actions (only for Providers view)
            if views[active_view] == "Providers" {
                let actions_line = Line::from(vec![
                    Span::styled(" [a] Add ", Style::default().fg(Color::Green)),
                    Span::styled(" [e] Edit ", Style::default().fg(Color::Green)),
                    Span::styled(" [d] Delete ", Style::default().fg(Color::Green)),
                ]);
                let actions = Paragraph::new(actions_line);
                f.render_widget(actions, chunks[1]);
            }

            // Command Line / Info Area
            let command_line = Paragraph::new(Span::styled(info_message.as_str(), Style::default().fg(Color::Green)))
                .block(Block::default().borders(Borders::ALL).title("Info / Command"));
            f.render_widget(command_line, chunks[chunks.len() - 2]);

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
                        .split(chunks[chunks.len() - 1]);

                    // Left column: File list
                    let items: Vec<ListItem> = context_files.iter().map(|f| {
                        let content = vec![Line::from(Span::raw(*f))];
                        ListItem::new(content)
                    }).collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Contexts"))
                        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ");
                    f.render_stateful_widget(list, cols[0], &mut context_list_state);

                    // Right column: File content
                    let file_view = Paragraph::new(file_content.as_str())
                        .block(Block::default().borders(Borders::ALL).title("Content"))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(file_view, cols[1]);
                }
                "Providers" => {
                    if adding_provider {
                        let provider_types = vec![
                            "OpenAI",
                            "Ollama",
                            "AzureOpenAI",
                            "Gemini",
                            "Grog",
                            "Claude",
                        ];

                        let items: Vec<ListItem> = provider_types
                            .iter()
                            .map(|ptype| ListItem::new(Span::raw(*ptype)))
                            .collect();

                        let provider_list = List::new(items)
                            .block(Block::default().borders(Borders::ALL).title("Select Provider Type"))
                            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                            .highlight_symbol(">> ");

                        f.render_stateful_widget(provider_list, chunks[chunks.len() - 1], &mut provider_list_state);
                    } else if let Some(ref editing_name) = editing_provider {
                        let provider = provider_instances.get(editing_name).cloned();
                        let settings = provider_settings.get(editing_name).cloned();

                        if let Some(provider) = provider {
                            let provider_types = vec![
                                "OpenAI",
                                "Ollama",
                                "AzureOpenAI",
                                "Gemini",
                                "Grog",
                                "Claude",
                            ];

                            let items: Vec<ListItem> = provider_types
                                .iter()
                                .map(|ptype| ListItem::new(Span::raw(*ptype)))
                                .collect();

                            let provider_list = List::new(items)
                                .block(Block::default().borders(Borders::ALL).title("Select Provider Type"))
                                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                                .highlight_symbol(">> ");

                            f.render_stateful_widget(provider_list, chunks[chunks.len() - 1], &mut provider_list_state);
                        }
                    } else if let Some(ref deleting_name) = deleting_provider {
                        let items: Vec<ListItem> = provider_names_vec
                            .iter()
                            .map(|name| ListItem::new(Span::raw(name)))
                            .collect();

                        let provider_list = List::new(items)
                            .block(Block::default().borders(Borders::ALL).title("Select Provider to Delete"))
                            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                            .highlight_symbol(">> ");

                        f.render_stateful_widget(provider_list, chunks[chunks.len() - 1], &mut provider_list_state);
                    } else {
                        let provider_names: Vec<ListItem> = provider_names_vec
                            .iter()
                            .map(|name| ListItem::new(Span::raw(name)))
                            .collect();

                        let provider_list = List::new(provider_names)
                            .block(Block::default().borders(Borders::ALL).title("Providers"))
                            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                            .highlight_symbol(">> ");

                        f.render_stateful_widget(provider_list, chunks[chunks.len() - 1], &mut provider_list_state);
                    }
                }
                _ => {
                    let workspace = Paragraph::new(format!("This is the {} view", views[active_view]))
                        .block(Block::default().borders(Borders::ALL).title("Workspace"));
                    f.render_widget(workspace, chunks[chunks.len() - 1]);
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
                    KeyCode::Char('4') => active_view = 3, // Shortcut for Providers
                    KeyCode::Char('5') => active_view = 4, // Shortcut for Models

                    // Provider Actions
                    KeyCode::Char('a') if active_view == 3 => {
                        // Handle Add Provider action
                        app_state.adding_provider = true;
                        provider_list_state.select(Some(0));
                        info_message = "Add Provider action triggered".to_string();
                    }
                    KeyCode::Char('e') if active_view == 3 => {
                        // Handle Edit Provider action
                        if let Some(selected) = provider_list_state.selected() {
                            let provider_names = app_state.get_provider_names();
                            if selected < provider_names.len() {
                                app_state.editing_provider = Some(provider_names[selected].clone());
                                info_message = format!("Edit Provider action triggered for {}", provider_names[selected]);
                            }
                        }
                    }
                    KeyCode::Char('d') if active_view == 3 => {
                        // Handle Delete Provider action
                        if let Some(selected) = provider_list_state.selected() {
                            let provider_names = app_state.get_provider_names();
                            if selected < provider_names.len() {
                                app_state.deleting_provider = Some(provider_names[selected].clone());
                                info_message = format!("Delete Provider action triggered for {}", provider_names[selected]);
                            }
                        }
                    }

                    KeyCode::Down if active_view == 3 => {
                        if app_state.adding_provider {
                            // Navigate down in the provider type selection
                            let provider_types_count = 6; // Number of provider types
                            if let Some(selected) = provider_list_state.selected() {
                                if selected < provider_types_count - 1 {
                                    provider_list_state.select(Some(selected + 1));
                                }
                            } else {
                                provider_list_state.select(Some(0));
                            }
                        } else {
                            // Navigate down in the provider list
                            if let Some(selected) = provider_list_state.selected() {
                                if selected < provider_names_vec.len() - 1 {
                                    provider_list_state.select(Some(selected + 1));
                                }
                            } else {
                                provider_list_state.select(Some(0));
                            }
                        }
                    }
                    KeyCode::Up if active_view == 3 => {
                        if app_state.adding_provider {
                            // Navigate up in the provider type selection
                            if let Some(selected) = provider_list_state.selected() {
                                if selected > 0 {
                                    provider_list_state.select(Some(selected - 1));
                                }
                            }
                        } else {
                            // Navigate up in the provider list
                            if let Some(selected) = provider_list_state.selected() {
                                if selected > 0 {
                                    provider_list_state.select(Some(selected - 1));
                                }
                            }
                        }
                    }
                    KeyCode::Enter if active_view == 3 && app_state.adding_provider => {
                        // Finalize provider type selection and transition to provider settings
                        if let Some(selected) = provider_list_state.selected() {
                            let selected_type = match selected {
                                0 => ProviderType::OpenAI,
                                1 => ProviderType::Ollama,
                                2 => ProviderType::AzureOpenAI,
                                3 => ProviderType::Gemini,
                                4 => ProviderType::Grog,
                                5 => ProviderType::Claude,
                                _ => ProviderType::OpenAI,
                            };

                            // Create a new provider instance and settings
                            let new_provider = ProviderInstance {
                                name: format!("Provider{}", app_state.provider_instances.len() + 1),
                                provider_type: selected_type.clone(),
                            };

                            let new_settings = ProviderSettings {
                                api_key: Some("your_api_key".to_string()),
                                api_entry_point: Some("https://api.example.com".to_string()),
                                api_deployment: Some("your_deployment".to_string()),
                            };

                            // Add the new provider
                            app_state.add_provider(new_provider, new_settings);
                            app_state.adding_provider = false;
                            info_message = format!("Added new provider of type {:?}", selected_type);
                        }
                    }

                    KeyCode::Enter if active_view == 3 && app_state.editing_provider.is_some() => {
                        // Extract the editing_name before mutable borrow
                        if let Some(editing_name) = app_state.editing_provider.clone() {
                            let selected_provider = app_state.provider_instances.get(&editing_name).cloned();
                            let selected_settings = app_state.provider_settings.get(&editing_name).cloned();

                            if let Some(provider) = selected_provider {
                                let updated_provider = ProviderInstance {
                                    name: provider.name,
                                    provider_type: provider.provider_type,
                                };

                                let updated_settings = ProviderSettings {
                                    api_key: Some("updated_api_key".to_string()),
                                    api_entry_point: Some("https://api.updated.com".to_string()),
                                    api_deployment: Some("updated_deployment".to_string()),
                                };

                                app_state.edit_provider(&editing_name, updated_provider, updated_settings);
                                app_state.editing_provider = None;
                                info_message = format!("Edited provider {}", editing_name);
                            }
                        }
                    }

                    KeyCode::Enter if active_view == 3 && app_state.deleting_provider.is_some() => {
                        // Extract the deleting_name before mutable borrow
                        if let Some(deleting_name) = app_state.deleting_provider.clone() {
                            app_state.remove_provider(&deleting_name);
                            app_state.deleting_provider = None;
                            info_message = format!("Deleted provider {}", deleting_name);
                        }
                    }

                    KeyCode::Down if active_view == 0 => {
                        // Navigate down in the file list
                        if selected_context < context_files.len() - 1 {
                            selected_context += 1;
                            context_list_state.select(Some(selected_context));
                            if let Ok(content) = fs::read_to_string(context_files[selected_context]) {
                                file_content = content;
                            }
                        }
                    }
                    KeyCode::Up if active_view == 0 => {
                        // Navigate up in the file list
                        if selected_context > 0 {
                            selected_context -= 1;
                            context_list_state.select(Some(selected_context));
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