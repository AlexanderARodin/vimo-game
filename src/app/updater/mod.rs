use anyhow::Result;
#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::action::Action;
use super::app_model::{AppModel, AppModelState};
use game_model::prelude::*;

mod command_string;
mod key_binder;

//  //  //  //  //  //  //  //
pub fn update(app: &mut AppModel, act: &Action) -> Result<Action> {
    match act {
        Action::Tick => {
            app.tick_counter += 1;
            let mask = app.tick_counter & 3;
            if mask == 1 {
                app.game_time += 1;
                return Ok(Action::GameUpdate);
            }
            return Ok(Action::Noop);
        }
        Action::GameUpdate => {
            return update_game(app);
        }
        Action::QueueCommand(cmds) => {
            app.game_actions = cmds.chars().collect();
            Ok(Action::ResetCounters)
        }
        Action::ResetCounters => {
            app.game_time = -2;
            app.tick_counter = 0;
            Ok(Action::Noop)
        }
        Action::ApplyEditedCode(is_game_code) => {
            if *is_game_code {
                return apply_game_code(app);
            } else {
                return apply_command_code(app);
            }
        }
        Action::PopupLuaEditor => {
            app.is_popup = !app.is_popup;
            Ok(Action::Noop)
        }
        Action::TranslateRawEvent(ev) => {
            return key_binder::translate_event(
                ev,
                app.is_popup,
                app.command_editor_state.mode == edtui::EditorMode::Normal,
            )
        }
        Action::HandleByEditor(ev) => {
            if app.is_popup {
                app.game_editor_handler
                    .on_event(ev.clone(), &mut app.game_editor_state);
            } else {
                app.ed_handler
                    .on_event(ev.clone(), &mut app.command_editor_state);
            }
            Ok(Action::Noop)
        }
        Action::Warning(s) => {
            warn!("{s}");
            Ok(Action::Noop)
        }
        Action::Quit => {
            app.state = AppModelState::Exiting;
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
fn extract_game_command(app: &mut AppModel) -> Result<Option<GameCommand>> {
    if app.game_actions.is_empty() {
        return Ok(None);
    }
    if app.game_time < 0 {
        return Ok(None);
    }
    let c = app.game_actions.remove(0);
    match c {
        'k' => return Ok(Some(GameCommand::Up)),
        'j' => return Ok(Some(GameCommand::Down)),
        'h' => return Ok(Some(GameCommand::Left)),
        'l' => return Ok(Some(GameCommand::Right)),
        _ => {
            return Err(anyhow::anyhow!(
                "Unexpected character <{}> in game_command",
                c
            ))
        }
    }
}

#[inline(always)]
fn update_game(app: &mut AppModel) -> Result<Action> {
    let game_command = extract_game_command(app)?;
    if let Some(game) = &mut app.game {
        if let Err(e) = game.update(app.game_time, game_command) {
            app.game = None;
            return Ok(Action::Warning(format!(
                "Lua code has errors (see below). Game has been reseted.\n{}",
                e
            )));
        }
    }
    Ok(Action::Noop)
}

#[inline(always)]
fn apply_command_code(app: &mut AppModel) -> Result<Action> {
    app.command_editor_state.mode = edtui::EditorMode::Normal;
    let src_command: String = app.command_editor_state.lines.clone().into();
    match command_string::convert(&src_command) {
        Ok(s) => Ok(Action::QueueCommand(s)),
        Err(e) => Ok(Action::Warning(format!(
            "game commands has unexpected instruction(s).\n{}",
            e
        ))),
    }
}

#[inline(always)]
fn apply_game_code(app: &mut AppModel) -> Result<Action> {
    app.game_editor_state.mode = edtui::EditorMode::Normal;
    let code: String = app.game_editor_state.lines.clone().into();
    match GameModel::new(&code) {
        Ok(new_game) => {
            app.game = Some(new_game);
            info!("Lua restarted with new code");
            Ok(Action::ResetCounters)
        }
        Err(e) => {
            app.game = None;
            return Ok(Action::Warning(format!(
                "Lua code has errors (see below). Game has been reseted\n{}",
                e
            )));
        }
    }
}
