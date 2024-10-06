#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::app_model::AppModel;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use Constraint::*;

mod game_widget;
use game_widget::GameWidget;

use game_model::GameModelInterface;

mod popup;

//  //  //  //  //  //  //  //
pub fn view(model: &mut AppModel, area: Rect, buf: &mut Buffer) {
    let l = Layout::vertical([Length(4), Min(35), Min(4)]).split(area);

    TitleWidget().render(l[0], buf);
    {
        if let Some(game) = &model.game {
            PlaygroundWidget(Some(game)).render(l[1], buf);
        } else {
            PlaygroundWidget(None).render(l[1], buf);
        }
    } // fight with this MUT
    EditorWidget(&mut model.ed_state).render(l[2], buf);

    if model.is_popup {
        popup::render_popup(area, buf);
    }
}

//  //  //  //  //  //  //  //
struct TitleWidget();
impl Widget for TitleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("main title here").block(Block::bordered().title("title of Main Title"))
            .render(area, buf);
    }
}

struct EditorWidget<'a>(&'a mut edtui::EditorState);
impl Widget for EditorWidget<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        edtui::EditorView::new(&mut self.0)
            .render(area, buf);
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
impl Widget for LeftGameBarWidget  {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from("\n\n");
        for i in 0x0..0x10 {
            s += &format!("[{:X}]\n\n", i);
        }
        Paragraph::new(s)
            .render(area, buf);
    }
}

struct TopGameBarWidget();
impl Widget for TopGameBarWidget  {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from(" ");
        for i in 0x0..0x10 {
            s += &format!(" [{:X}]", i);
        }
        Paragraph::new(s)
            .render(area, buf);
    }
}
