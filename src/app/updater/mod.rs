use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::app_model::{AppModel, AppModelState};
use game_model::GameModelInterface;

mod key_binder;

//  //  //  //  //  //  //  //
static TICK: std::time::Duration = std::time::Duration::from_millis(250);

pub fn update(model: &mut AppModel, act: &Action) -> Result<Action> {
    match act {
        Action::TranslateRawEvent(ev) => {
            return key_binder::translate_event(
                ev,
                model.ed_state.mode == edtui::EditorMode::Normal,
            )
        }
        Action::HandleByEditor(ev) => {
            model.ed_handler.on_event(ev.clone(), &mut model.ed_state);
            Ok(Action::Noop)
        }
        Action::Quit => {
            model.state = AppModelState::Exiting;
            Ok(Action::Noop)
        }
        Action::LoadCode => {
            return load_code(model);
        }
        Action::GameUpdate(t) => {
            if let Some(game) = &mut model.game {
                if let Err(e) = game.update(*t) {
                    model.game = None;
                    warn!("Lua code has errors (see below). Game has been reseted.");
                    warn!("{}", e.to_string());
                }
            }
            Ok(Action::Noop)
        }
        Action::UpdateTimer => {
            return update_timer(model);
        }
        Action::PopupLuaEditor => {
            model.is_popup = !model.is_popup;
            Ok(Action::Noop)
        }
        _ => {
            debug!("unprocessed Message:\n{:?}", act);
            Ok(Action::Noop)
        }
    }
}

//  //  //  //  //  //  //  //
#[inline(always)]
fn load_code(model: &mut AppModel) -> Result<Action> {
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

//  //  //  //  //  //  //  //
#[inline(always)]
fn update_timer(model: &mut AppModel) -> Result<Action> {
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
