use ratatui::crossterm::event as xEvent;

#[derive(Debug,PartialEq)]
pub enum Action {
    Noop,
    Quit,
    TranslateRawEvent(xEvent::Event),
    UpdateTimer,
    ApplyEditedCode,
    Tick,
    GameUpdate,
    GameAction,
    HandleByEditor(xEvent::Event),
    PopupLuaEditor,
    QueueCommand(String),
    Warning(String),
}
