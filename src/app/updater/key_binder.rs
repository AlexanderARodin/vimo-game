use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::Action;
use ratatui::crossterm::event as xEvent;

#[inline(always)]
pub(super) fn translate_event(event: &xEvent::Event, is_popuped: bool, is_normal_mode: bool) -> Result<Action> {
    if let xEvent::Event::Key(key) = event {
        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
            if key.code == xEvent::KeyCode::Char('y') {
                // TODO: <C-CR> doesn't work
                return Ok(Action::ApplyEditedCode(is_popuped));
            }
            if key.code == xEvent::KeyCode::Char('p') {
                return Ok(Action::PopupLuaEditor);
            }
        }
    }
    if is_normal_mode {
        if let xEvent::Event::Key(key) = event {
            if key.code == xEvent::KeyCode::Char('q') {
                return Ok(Action::Quit);
            }
        }
    }
    Ok(Action::HandleByEditor(event.clone()))
}
