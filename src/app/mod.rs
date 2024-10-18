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

use ratatui::crossterm::event as xEvent;
use ratatui::prelude::*;

//  //  //  //  //  //  //  //
pub fn run(
    terminal: &mut ratatui::Terminal<impl Backend>,
    app_config: &crate::config::AppConfig,
) -> Result<()> {
    trace!(" -> app::run()");
    let mut app = AppModel::new(app_config)?;
    let mut auto_run = app_config.auto_run;
    let handler = event_handler::EventHandler::new(app.config.refresh_time);

    while !app.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&mut app, frame.area(), frame.buffer_mut()))?;

        // UPDATE
        //      auto-run
        if auto_run {
            auto_run = false;
            invoke_update_loop(Action::ApplyEditedCode(true), &mut app)?;
        }
        //      get inputs
        //let raw_inputs = collect_events(&app_config.refresh_tiem)?;
        let raw_inputs = handler.next()?;
        check_terminate_sequence(&raw_inputs)?;
        //      updating loop
        for event in raw_inputs {
            invoke_update_loop(Action::TranslateRawEvent(event), &mut app)?;
        }
        invoke_update_loop(Action::UpdateTimer, &mut app)?;
    }
    trace!("normal exit");
    Ok(())
}

fn invoke_update_loop(first_action: Action, model: &mut AppModel) -> Result<()> {
    let mut current_action = first_action;
    while current_action != Action::Noop {
        current_action = update(model, &current_action)?;
    }
    Ok(())
}

//  //  //  //  //  //  //  //
fn check_terminate_sequence(events: &Vec<xEvent::Event>) -> Result<()> {
    for event in events {
        match event {
            xEvent::Event::Key(key) => {
                if key.modifiers.contains(xEvent::KeyModifiers::CONTROL) {
                    // <C-c>
                    if key.code == xEvent::KeyCode::Char('c') {
                        let msg = "exiting by <C-c>";
                        warn!("{}", msg);
                        return Err(anyhow::anyhow!(msg));
                    }
                    // <C-e>
                    if key.code == xEvent::KeyCode::Char('x') {
                        let msg = "exiting with TEST error by <C-x>";
                        error!("{}", msg);
                        return Err(anyhow::anyhow!(msg));
                    }
                    // <C-p>
                    if key.code == xEvent::KeyCode::Char('p') {
                        panic!("TEST panic by <C-p>");
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
