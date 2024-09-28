use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod action;
use action::Action;


mod model;
use model::AppModel as Model;
mod updater;
use updater::update;
mod viewer;

use ratatui::crossterm::event as xEvent;
use ratatui::prelude::*;

//  //  //  //  //  //  //  //

pub fn run(terminal: &mut ratatui::Terminal<impl Backend>) -> Result<()> {
    trace!(" -> app::run()");
    let mut model = Model::new();

    while !model.is_exiting() {
        // DRAW
        terminal.draw(|frame| viewer::view(&mut model, frame))?;

        // UPDATE
        //      get inputs
        let raw_inputs = collect_events()?;
        check_terminate_sequence(&raw_inputs)?;
        //      updating loop
        for raw_input in raw_inputs {
            let mut current_action = Action::TranslateRawEvent(raw_input);
            while current_action != Action::Noop {
                current_action = update(&mut model, &current_action)?;
            }
        }
    }
    trace!("normal exit");
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

//  //  //  //  //  //  //  //
static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(8); //from_secs(0);
fn collect_events() -> Result<Vec<xEvent::Event>> {
    let mut result = Vec::new();
    while xEvent::poll(POLL_WAIT_TIME)? {
        result.push(xEvent::read()?);
    }
    Ok(result)
}



/*

    fn other_input_handlers(&mut self, events: &Vec<xEvent::Event>) -> Result<()> {
        for event in events {
            self.ed_handler.on_event(event.clone(), &mut self.ed_state);
        }
        Ok(())
    }
}
*/
//  //  //  //
/*
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // layout
        let [title_area, main_area, status_area] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Min(19),
            Constraint::Min(2),
        ])
        .areas(area);

        Paragraph::new("main title here")
            .block(Block::bordered().title("title of Main Title"))
            .render(title_area, buf);
        let main_block = Block::bordered();
        {
            let main_inner = main_block.inner(main_area);
            let [main_left, main_right] =
                Layout::horizontal([Constraint::Length(3), Constraint::Min(16)]).areas(main_inner);

            main_block.render(main_area, buf);
            Paragraph::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\nC\nD\nE\nF\n<-->")
                .render(main_left, buf);
            edtui::EditorView::new(&mut self.ed_state).render(main_right, buf);
        }
        // status info
        Paragraph::new(self.status.clone())
            .wrap(Wrap { trim: true })
            .block(Block::bordered().title("debug information:"))
            .render(status_area, buf);
    }
}
*/
