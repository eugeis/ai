use ratatui::Frame;
use crossterm::event::KeyEvent;

pub trait View {
    fn render(&self, f: &mut Frame, area: ratatui::layout::Rect, info_message: &str);

    fn handle_input(&mut self, key: KeyEvent, info_message: &mut String);
}
