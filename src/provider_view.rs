use crate::traits::View;
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::collections::HashMap;

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

struct State {
    provider_instances: HashMap<String, ProviderInstance>,
    provider_settings: HashMap<String, ProviderSettings>,
    adding_provider: bool,
    editing_provider: Option<String>,
    deleting_provider: Option<String>,
    current_step: Option<AddProviderStep>,
    settings_input: [String; 3],
    active_input_index: usize,
}

enum AddProviderStep {
    SelectType,
    ConfigureSettings,
}

pub struct ProviderView {
    state: State,
    provider_list_state: ListState,
    provider_type_list_state: ListState,
    selected_provider_type: Option<ProviderType>,
}

impl ProviderView {
    pub fn new() -> Self {
        let mut provider_list_state = ListState::default();
        provider_list_state.select(Some(0));

        let mut provider_type_list_state = ListState::default();
        provider_type_list_state.select(Some(0));

        let state = State {
            provider_instances: HashMap::new(),
            provider_settings: HashMap::new(),
            adding_provider: false,
            editing_provider: None,
            deleting_provider: None,
            current_step: None,
            settings_input: Default::default(),
            active_input_index: 0,
        };

        Self {
            state,
            provider_list_state,
            provider_type_list_state,
            selected_provider_type: None,
        }
    }

    fn get_provider_type_names() -> Vec<&'static str> {
        vec![
            "OpenAI",
            "Ollama",
            "AzureOpenAI",
            "Gemini",
            "Grog",
            "Claude",
        ]
    }

    fn render_provider_settings(&self) -> List {
        let settings = vec![
            ListItem::new(Span::styled(
                format!("API Key: {}", self.state.settings_input[0]),
                if self.state.active_input_index == 0 {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            )),
            ListItem::new(Span::styled(
                format!("API Entry Point: {}", self.state.settings_input[1]),
                if self.state.active_input_index == 1 {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            )),
            ListItem::new(Span::styled(
                format!("API Deployment: {}", self.state.settings_input[2]),
                if self.state.active_input_index == 2 {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            )),
        ];

        let settings_list = List::new(settings)
            .block(Block::default().borders(Borders::ALL).title("Provider Settings"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        settings_list
    }
}

impl View for ProviderView {
    fn render(&self, f: &mut Frame, area: ratatui::layout::Rect, info_message: &str) {
        let constraints = vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_slice())
            .split(area);

        let actions_line = Line::from(vec![
            Span::styled(" [a] Add ", Style::default().fg(Color::Green)),
            Span::styled(" [e] Edit ", Style::default().fg(Color::Green)),
            Span::styled(" [d] Delete ", Style::default().fg(Color::Green)),
        ]);
        let actions = Paragraph::new(actions_line);
        f.render_widget(actions, chunks[0]);

        let info_paragraph = Paragraph::new(info_message)
            .block(Block::default().borders(Borders::ALL).title("Info / Command"));
        f.render_widget(info_paragraph, chunks[chunks.len() - 2]);

        match self.state.current_step {
            Some(AddProviderStep::SelectType) => {
                let provider_types = Self::get_provider_type_names();
                let items: Vec<ListItem> = provider_types.iter().map(|&name| ListItem::new(Span::raw(name))).collect();
                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Select Provider Type"))
                    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(list, chunks[chunks.len() - 1], &mut self.provider_type_list_state.clone());
            }
            Some(AddProviderStep::ConfigureSettings) => {
                let settings_list = self.render_provider_settings();
                f.render_stateful_widget(settings_list, chunks[chunks.len() - 1], &mut ListState::default().with_selected(Option::from(self.state.active_input_index)));
            }
            None => {
                let providers: Vec<String> = self.state.provider_instances.keys().cloned().collect();
                let items: Vec<ListItem> = providers.iter().map(|name| ListItem::new(Span::raw(name))).collect();
                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Providers"))
                    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(list, chunks[chunks.len() - 1], &mut self.provider_list_state.clone());
            }
        }
    }

    fn handle_input(&mut self, key: crossterm::event::KeyEvent, info_message: &mut String) {
        match self.state.current_step {
            Some(AddProviderStep::SelectType) => {
                match key.code {
                    KeyCode::Down => {
                        let provider_types = Self::get_provider_type_names();
                        if let Some(selected) = self.provider_type_list_state.selected() {
                            if selected < provider_types.len() - 1 {
                                self.provider_type_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(selected) = self.provider_type_list_state.selected() {
                            if selected > 0 {
                                self.provider_type_list_state.select(Some(selected - 1));
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = self.provider_type_list_state.selected() {
                            let provider_types = Self::get_provider_type_names();
                            self.selected_provider_type = match provider_types[selected] {
                                "OpenAI" => Some(ProviderType::OpenAI),
                                "Ollama" => Some(ProviderType::Ollama),
                                "AzureOpenAI" => Some(ProviderType::AzureOpenAI),
                                "Gemini" => Some(ProviderType::Gemini),
                                "Grog" => Some(ProviderType::Grog),
                                "Claude" => Some(ProviderType::Claude),
                                _ => None,
                            };
                            self.state.current_step = Some(AddProviderStep::ConfigureSettings);
                            info_message.clear();
                            info_message.push_str("Configure the selected provider settings");
                        }
                    }
                    KeyCode::Esc => {
                        self.state.current_step = None;
                        info_message.clear();
                    }
                    _ => {}
                }
            }
            Some(AddProviderStep::ConfigureSettings) => {
                match key.code {
                    KeyCode::Tab => {
                        self.state.active_input_index = (self.state.active_input_index + 1) % 3;
                    }
                    KeyCode::BackTab => {
                        if self.state.active_input_index == 0 {
                            self.state.active_input_index = 2;
                        } else {
                            self.state.active_input_index -= 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        self.state.settings_input[self.state.active_input_index].push(c);
                    }
                    KeyCode::Enter => {
                        let provider_name = format!("Provider{}", self.state.provider_instances.len() + 1);
                        let provider_instance = ProviderInstance {
                            name: provider_name.clone(),
                            provider_type: self.selected_provider_type.clone().unwrap(),
                        };
                        let provider_settings = ProviderSettings {
                            api_key: Some(self.state.settings_input[0].clone()),
                            api_entry_point: Some(self.state.settings_input[1].clone()),
                            api_deployment: Some(self.state.settings_input[2].clone()),
                        };

                        self.state.provider_instances.insert(provider_name.clone(), provider_instance);
                        self.state.provider_settings.insert(provider_name.clone(), provider_settings);

                        self.state.current_step = None;
                        info_message.clear();
                        info_message.push_str("Provider added successfully.");
                        self.state.settings_input = Default::default();
                    }
                    KeyCode::Backspace => {
                        if !self.state.settings_input[self.state.active_input_index].is_empty() {
                            self.state.settings_input[self.state.active_input_index].pop();
                        }
                    }
                    KeyCode::Esc => {
                        self.state.current_step = None;
                        info_message.clear();
                    }
                    _ => {}
                }
            }
            None => {
                match key.code {
                    KeyCode::Char('a') => {
                        self.state.adding_provider = true;
                        self.state.current_step = Some(AddProviderStep::SelectType);
                        info_message.clear();
                        info_message.push_str("Select the provider type");
                    }
                    KeyCode::Char('e') => {
                        if let Some(selected) = self.provider_list_state.selected() {
                            let provider_names = self.state.provider_instances.keys().cloned().collect::<Vec<_>>();
                            if selected < provider_names.len() {
                                self.state.editing_provider = Some(provider_names[selected].clone());
                                let settings = self.state.provider_settings.get(&provider_names[selected]).unwrap();
                                self.state.settings_input[0] = settings.api_key.clone().unwrap_or_default();
                                self.state.settings_input[1] = settings.api_entry_point.clone().unwrap_or_default();
                                self.state.settings_input[2] = settings.api_deployment.clone().unwrap_or_default();
                                self.state.current_step = Some(AddProviderStep::ConfigureSettings);
                                info_message.clear();
                                info_message.push_str(&format!("Edit Provider action triggered for {}", provider_names[selected]));
                            }
                        }
                    }
                    KeyCode::Char('d') => {
                        if let Some(selected) = self.provider_list_state.selected() {
                            let provider_names = self.state.provider_instances.keys().cloned().collect::<Vec<_>>();
                            if selected < provider_names.len() {
                                self.state.deleting_provider = Some(provider_names[selected].clone());
                                info_message.clear();
                                info_message.push_str(&format!("Delete Provider action triggered for {}", provider_names[selected]));
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(selected) = self.provider_list_state.selected() {
                            let provider_names = self.state.provider_instances.keys().cloned().collect::<Vec<_>>();
                            if selected < provider_names.len() - 1 {
                                self.provider_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(selected) = self.provider_list_state.selected() {
                            if selected > 0 {
                                self.provider_list_state.select(Some(selected - 1));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}