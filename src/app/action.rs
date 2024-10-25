use ratatui::crossterm::event as xEvent;

#[derive(Debug,PartialEq)]
pub enum Action {
    Noop,
    Quit,
    TranslateRawEvent(xEvent::Event),
    ApplyEditedCode(bool), // is GameCode?
    Tick,
    ResetCounters,
    GameUpdate,
    HandleByEditor(xEvent::Event),
    PopupLuaEditor,
    QueueCommand(String),
    Warning(String),
}
