use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::app_model::{AppModel, AppModelState};
use game_model::GameModelInterface;

mod command_string;
mod key_binder;

//  //  //  //  //  //  //  //
static TICK: std::time::Duration = std::time::Duration::from_millis(125);

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
                model
                    .game_editor_handler
                    .on_event(ev.clone(), &mut model.game_editor_state);
            } else {
                model
                    .ed_handler
                    .on_event(ev.clone(), &mut model.command_editor_state);
            }
            Ok(Action::Noop)
        }
        Action::UpdateTimer => {
            return update_timer(model);
        }
        Action::Tick => {
            if (model.tick_counter & 1) != 0 {
                return Ok(Action::GameAction);
            } else {
                model.game_counter += 1;
                return Ok(Action::GameUpdate);
            }
        }
        Action::GameAction => {
            return action_game(model);
        }
        Action::GameUpdate => {
            return update_game(model);
        }
        Action::QueueCommand(cmds) => {
            model.game_actions = cmds.chars().collect();
            Ok(Action::Noop)
        }
        Action::ApplyEditedCode => {
            if model.is_popup {
                return apply_game_code(model);
            } else {
                return apply_command_code(model);
            }
        }
        Action::PopupLuaEditor => {
            model.is_popup = !model.is_popup;
            Ok(Action::Noop)
        }
        Action::Warning(s) => {
            warn!("{s}");
            Ok(Action::Noop)
        }
        Action::Quit => {
            model.state = AppModelState::Exiting;
            Ok(Action::Noop)
        }
        _ => {
            debug!("unprocessed Message:\n{:?}", act);
            Ok(Action::Noop)
        }
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[inline(always)]
fn action_game(model: &mut AppModel) -> Result<Action> {
    if model.game_actions.is_empty() {
        return Ok(Action::Noop);
    }
    let c = model.game_actions.remove(0);
    let game_command = match c {
        'k' => game_model::GameCommand::Up,
        'j' => game_model::GameCommand::Down,
        'h' => game_model::GameCommand::Left,
        'l' => game_model::GameCommand::Right,
        _ => return Err(anyhow::anyhow!("Unexpected character <{}> in game_command", c)),
    };
    if let Some(game) = &mut model.game {
        if let Err(e) = game.action(game_command) {
            model.game = None;
            return Ok(Action::Warning(format!(
                "Lua code has errors (see below). Game has been reseted.\n{}",
                e
            )));
        }
    }
    Ok(Action::Noop)
}

#[inline(always)]
fn update_game(model: &mut AppModel) -> Result<Action> {
    if let Some(game) = &mut model.game {
        if let Err(e) = game.update(model.game_counter) {
            model.game = None;
            return Ok(Action::Warning(format!(
                "Lua code has errors (see below). Game has been reseted.\n{}",
                e
            )));
        }
    }
    Ok(Action::Noop)
}

#[inline(always)]
fn apply_command_code(model: &mut AppModel) -> Result<Action> {
    model.command_editor_state.mode = edtui::EditorMode::Normal;
    let src_command: String = model.command_editor_state.lines.clone().into();
    match command_string::convert(&src_command) {
        Ok(s) => Ok(Action::QueueCommand(s)),
        Err(e) => Ok(Action::Warning(format!(
                "game commands has unexpected instruction(s).\n{}",
                e
            ))),
    }

}

#[inline(always)]
fn apply_game_code(model: &mut AppModel) -> Result<Action> {
    model.game_editor_state.mode = edtui::EditorMode::Normal;
    let code: String = model.game_editor_state.lines.clone().into();
    match game_model::GameModel::new(&code) {
        Ok(new_game) => {
            model.game = Some(new_game);
            model.game_counter = -1;
            info!("Lua restarted with new code");
            Ok(Action::GameUpdate)
        }
        Err(e) => {
            model.game = None;
            return Ok(Action::Warning(format!(
                "Lua code has errors (see below). Game has been reseted\n{}",
                e
            )));
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
                model.tick_counter += 1;
                return Ok(Action::Tick);
            }
            return Ok(Action::Noop);
        }
        Err(e) => {
            model.tick_counter = 0;
            return Ok(Action::Warning(format!(
                "System timer error (see below):\n{}",
                e
            )));
        }
    }
}
