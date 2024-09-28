use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::model::{AppModel as Model, AppModelState as ModelState};

use ratatui::crossterm::event as xEvent;

//  //  //  //  //  //  //  //
pub fn update(model: &mut Model, act: &Action) -> Result<Action> {
    match (&model.state, act) {
        (_, Action::TranslateRawEvent(ev)) => translate_event(&model, ev),
        (_, Action::HandleByEditor(ev)) => {
            model.ed_handler.on_event(ev.clone(), &mut model.ed_state);
            Ok(Action::Noop)
        }
        (_, Action::Quit) => {
            model.state = ModelState::Exiting;
            Ok(Action::Noop)
        }
        _ => {
            trace!("unprocessed Message:\n{:?}", act);
            Ok(Action::Noop)
        }
    }
}

//  //  //  //  //  //  //  //
fn translate_event(model: &Model, event: &xEvent::Event) -> Result<Action> {
    if model.ed_state.mode == edtui::EditorMode::Normal {
        if let xEvent::Event::Key(key) = event {
            if key.code == xEvent::KeyCode::Char('q') {
                return Ok(Action::Quit);
            }
        }
    }
    Ok(Action::HandleByEditor(event.clone()))
}
