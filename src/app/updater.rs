use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::app_model::{AppModel, AppModelState};
use game_model::GameModelInterface;

use ratatui::crossterm::event as xEvent;


static TICK: std::time::Duration = std::time::Duration::from_millis(250);

//  //  //  //  //  //  //  //
pub fn update(model: &mut AppModel, act: &Action) -> Result<Action> {
    match (&model.state, act) {
        (_, Action::TranslateRawEvent(ev)) => translate_event(&model, ev),
        (_, Action::HandleByEditor(ev)) => {
            model.ed_handler.on_event(ev.clone(), &mut model.ed_state);
            Ok(Action::Noop)
        }
        (_, Action::Quit) => {
            model.state = AppModelState::Exiting;
            Ok(Action::Noop)
        }
        (_, Action::LoadCode) => {
            model.ed_state.mode = edtui::EditorMode::Normal;
            let code: String = model.ed_state.lines.clone().into();
            match game_model::GameModel::new(&code) {
                Ok(new_game) => {
                    model.game = Some(new_game);
                    model.counter = -1;
                    info!("Lua restarted with new code");
                    Ok(Action::GameUpdate(-1))
                }
                Err(e) => {
                    model.game = None;
                    warn!("Lua code has errors (see below). Game has been reseted.");
                    warn!("{}", e.to_string());
                    Ok(Action::Noop)
                }
            }
        }
        (_, Action::GameUpdate(t)) => {
            if let Some(game) = &mut model.game {
                game.update(*t);
            }
            Ok(Action::Noop)
        }
        (_, Action::UpdateTimer) => {
            let prev = model.start_time;
            match prev.elapsed() {
                Ok(delta) => {
                    if delta >= TICK {
                        model.start_time = std::time::SystemTime::now();
                        model.counter += 1;
                        return Ok(Action::GameUpdate(model.counter));
                    }
                }
                Err(_) => {
                    model.counter = -1;
                }
            }
            Ok(Action::Noop)
        }
        _ => {
            trace!("unprocessed Message:\n{:?}", act);
            Ok(Action::Noop)
        }
    }
}


//  //  //  //  //  //  //  //
fn translate_event(model: &AppModel, event: &xEvent::Event) -> Result<Action> {
    if let xEvent::Event::Key(key) = event {
        if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
            if key.code == xEvent::KeyCode::Char('y') {
            // TODO: <C-CR> doesn't work
                return Ok(Action::LoadCode);
            }
        }
    }
    if model.ed_state.mode == edtui::EditorMode::Normal {
        if let xEvent::Event::Key(key) = event {
            if key.code == xEvent::KeyCode::Char('q') {
                return Ok(Action::Quit);
            }
        }
    }
    Ok(Action::HandleByEditor(event.clone()))
}
