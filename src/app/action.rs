use ratatui::crossterm::event as xEvent;

#[derive(Debug,PartialEq)]
pub enum Action {
    Noop,
    Quit,
    TranslateRawEvent(xEvent::Event),
    HandleByEditor(xEvent::Event),
    Error(String),
}
