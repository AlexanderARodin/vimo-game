use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::app_model::{AppModel, AppModelState};
use game_model::GameModelInterface;

mod key_binder;
mod command_string;

//  //  //  //  //  //  //  //
static TICK: std::time::Duration = std::time::Duration::from_millis(250);

pub fn update(model: &mut AppModel, act: &Action) -> Result<Action> {
    match act {
        Action::TranslateRawEvent(ev) => {
            return key_binder::translate_event(
                ev,
                model.command_editor_state.mode == edtui::EditorMode::Normal,
            )
        }
        Action::HandleByEditor(ev) => {
            if model.is_popup {
                model.game_editor_handler.on_event(ev.clone(), &mut model.game_editor_state);
            }else{
                model.ed_handler.on_event(ev.clone(), &mut model.command_editor_state);
            }
            Ok(Action::Noop)
        }
        Action::Quit => {
            model.state = AppModelState::Exiting;
            Ok(Action::Noop)
        }
        Action::ApplyEditedCode => {
            if model.is_popup {
                return apply_game_code(model);
            }else{
                return apply_command_code(model);
            }
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
fn apply_command_code(model: &mut AppModel) -> Result<Action> {
    model.command_editor_state.mode = edtui::EditorMode::Normal;
    let src_command: String = model.command_editor_state.lines.clone().into();
    let clean_command: String = command_string::convert(&src_command)?;

    trace!("COMMAND <{}>", clean_command);

    Ok(Action::Noop)
    //Err(anyhow::anyhow!("No implementaion for Command Encoding"))
}

#[inline(always)]
fn apply_game_code(model: &mut AppModel) -> Result<Action> {
    model.game_editor_state.mode = edtui::EditorMode::Normal;
    let code: String = model.game_editor_state.lines.clone().into();
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
