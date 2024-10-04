use ratatui::crossterm::event as xEvent;

#[derive(Debug,PartialEq)]
pub enum Action {
    Noop,
    Quit,
    TranslateRawEvent(xEvent::Event),
    UpdateTimer,
    LoadCode,
    GameUpdate(i64),
    HandleByEditor(xEvent::Event),
    PopupLuaEditor,
    //Error(String),
}
