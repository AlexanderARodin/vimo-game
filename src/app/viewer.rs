#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::model::AppModel as Model;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use Constraint::*;
use Direction::*;

use super::tui_view::*;

//  //  //  //  //  //  //  //
pub fn view(model: &mut Model, frame: &mut Frame) {
    sub_views_with_layouts(
        frame,
        frame.area(),
        Vertical,
        [
            (&mut TitleView(), Length(5)),
            (&mut PlaygoundView(), Min(35)),
            (&mut EditorView(&mut model.ed_state), Min(4)),
        ],
    );
}
//  //  //  //  //  //  //  //
struct TitleView();
impl TuiView for TitleView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let title =
            Paragraph::new("main title here").block(Block::bordered().title("title of Main Title"));
        frame.render_widget(title, area);
    }
}

struct EditorView<'a>(&'a mut edtui::EditorState);
impl TuiView for EditorView<'_> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let editor = edtui::EditorView::new(&mut self.0);
        frame.render_widget(editor, area);
    }
}

struct PlaygoundView();
impl TuiView for PlaygoundView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let main_block = Block::bordered();
        let inner_area = main_block.inner(area);

        frame.render_widget(main_block, area);

        sub_views_with_layouts(
            frame,
            inner_area,
            Horizontal,
            [
                (&mut GameLeftView(), Length(3)),
                (&mut GameRightView(), Length(67)),
            ],
        );
    }
}

struct GameLeftView();
impl TuiView for GameLeftView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let mut s = String::from("\n\n");
        for i in 0..=15 {
            s += &format!("[{:X}]\n\n", i);
        }
        let text = Paragraph::new(s);
        frame.render_widget(text, area);
    }
}

struct GameRightView();
impl TuiView for GameRightView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        sub_views_with_layouts(
            frame,
            area,
            Vertical,
            [
                (&mut GameRightTopView(), Length(1)),
                (&mut GameMainView(), Length(33)),
            ],
        );
    }
}

struct GameRightTopView();
impl TuiView for GameRightTopView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let mut s = String::from(" ");
        for i in 0..=15 {
            s += &format!(" [{:X}]", i);
        }
        let text = Paragraph::new(s);
        frame.render_widget(text, area);
    }
}

struct GameMainView();
impl TuiView for GameMainView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered();
        frame.render_widget(block, area);
    }
}
