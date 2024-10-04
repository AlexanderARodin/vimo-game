use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::crossterm::event as xEvent;
use super::Action;

#[inline(always)]
pub(super) fn translate_event(event: &xEvent::Event, is_normal_mode: bool) -> Result<Action> {
    if let xEvent::Event::Key(key) = event {
        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
            if key.code == xEvent::KeyCode::Char('y') {
                // TODO: <C-CR> doesn't work
                return Ok(Action::LoadCode);
            }
            if key.code == xEvent::KeyCode::Char('d') {
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

