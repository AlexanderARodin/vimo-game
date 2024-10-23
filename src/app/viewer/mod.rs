#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::app_model::AppModel;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use ratatui::widgets::Widget;
use Constraint::*;

mod game_widget;
use game_widget::GameWidget;

use game_model::prelude::*;

mod popup;

//  //  //  //  //  //  //  //
pub fn view(app: &mut AppModel, area: Rect, buf: &mut Buffer) {
    let [top_area, game_area, command_area] =
        Layout::vertical([Length(4), Min(35), Min(4)]).areas(area);

    TitleWidget(&app.game_actions).render(top_area, buf);

    if let Some(game) = &app.game {
        PlaygroundWidget(Some(game)).render(game_area, buf);
    } else {
        PlaygroundWidget(None).render(game_area, buf);
    }

    edtui::EditorView::new(&mut app.command_editor_state)
        .render(command_area, buf);


    if app.is_popup {
        popup::render_editor_popup(area, buf, &mut app.game_editor_state);
    }
}

//  //  //  //  //  //  //  //
struct TitleWidget<'a>(&'a Vec<char>);
impl Widget for TitleWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.0.is_empty() {
            Paragraph::new("")
                .block(Block::bordered().title("there is no action"))
                .render(area, buf);
        } else {
            let text: String = self.0.into_iter().collect();
            Paragraph::new(text)
                .block(Block::bordered().title("ACTIONS in queue:"))
                .render(area, buf);
        }
    }
}

struct PlaygroundWidget<'a>(Option<&'a dyn GameModelInterface>);
impl Widget for PlaygroundWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left_bar, other] = Layout::horizontal([Length(3), Length(67)]).areas(area);
        let [top_bar, play_zone] = Layout::vertical([Length(1), Length(33)]).areas(other);

        LeftGameBarWidget().render(left_bar, buf);
        TopGameBarWidget().render(top_bar, buf);
        GameWidget(self.0).render(play_zone, buf);
    }
}

struct LeftGameBarWidget();
impl Widget for LeftGameBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from("\n\n");
        for i in 0x0..0x10 {
            s += &format!("[{:X}]\n\n", i);
        }
        Paragraph::new(s).render(area, buf);
    }
}

struct TopGameBarWidget();
impl Widget for TopGameBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from(" ");
        for i in 0x0..0x10 {
            s += &format!(" [{:X}]", i);
        }
        Paragraph::new(s).render(area, buf);
    }
}
