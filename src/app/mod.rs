use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod action;
use action::Action;

mod app_model;
use app_model::AppModel;

mod updater;
use updater::update;
mod viewer;

use ratatui::prelude::*;

//  //  //  //  //  //  //  //
pub fn run(
    terminal: &mut ratatui::Terminal<impl Backend>,
    app_config: &crate::config::AppConfig,
) -> Result<()> {
    trace!(" -> app::run()");
    let mut app = AppModel::new(app_config)?;
    let handler = event_handler::EventHandler::new(app.config.refresh_delay);

    //      auto-run
    if app_config.auto_run {
        invoke_update_loop(Action::ApplyEditedCode(true), &mut app)?;
    }

    while !app.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&mut app, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      get inputs
        let raw_input = handler.wait_next()?;
        //      process inputs
        match raw_input {
            event_handler::Events::Input(raw_event) => {
                invoke_update_loop(Action::TranslateRawEvent(raw_event), &mut app)?;
            }
            event_handler::Events::Tick => {
                invoke_update_loop(Action::Tick, &mut app)?;
            }
            event_handler::Events::Exit => {
                invoke_update_loop(Action::Quit, &mut app)?;
            }
        }
    }
    trace!("normal exit");
    Ok(())
}

//  //  //  //  //  //  //  //
fn invoke_update_loop(first_action: Action, model: &mut AppModel) -> Result<()> {
    let mut current_action = first_action;
    while current_action != Action::Noop {
        current_action = update(model, &current_action)?;
    }
    Ok(())
}
