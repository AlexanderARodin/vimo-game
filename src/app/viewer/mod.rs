#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::app_model::AppModel;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use Constraint::*;
use Direction::*;

mod tui_view;
use tui_view::*;

mod game_view;
use game_view::GameView;

use game_model::GameModelInterface;

//  //  //  //  //  //  //  //
pub fn view(model: &mut AppModel, frame: &mut Frame) {
    let area = frame.area();
    let l = Layout::vertical([Length(5), Min(35), Min(4)]).split(area);
    {
        TitleView().view(frame, l[0]);
    }
    {
        if let Some(game) = &model.game {
            PlaygroundView(Some(game)).view(frame, l[1]);
        }else{
            PlaygroundView(None).view(frame, l[1]);
        }
    }
    {
        // fight with this MUT
        EditorView(&mut model.ed_state).view(frame, l[2]);
    }

    if model.is_popup {
        let popup_area = Rect {
            x: area.width / 6,
            y: area.height / 6,
            width: area.width * 4 / 6,
            height: area.height * 4 / 6,
        };
        frame.render_widget(ratatui::widgets::Clear::default(), popup_area);
        let popup_block = Block::bordered()
            .title("GameModel Lua Editor");
        frame.render_widget(popup_block, popup_area);
    }
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

struct PlaygroundView<'a>(Option<&'a dyn GameModelInterface>);
impl TuiView for PlaygroundView<'_> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        //let main_block = Block::into//bordered();
        let inner_area = area; //main_block.inner(area);
                               //frame.render_widget(main_block, area);

        sub_views_with_layouts(
            frame,
            inner_area,
            Horizontal,
            [
                (&mut PlaygoundLeftView(), Length(3)),
                (&mut PlaygoundRightView(self.0), Length(67)),
            ],
        );
    }
}

struct PlaygoundLeftView();
impl TuiView for PlaygoundLeftView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let mut s = String::from("\n\n");
        for i in 0x0..0x10 {
            s += &format!("[{:X}]\n\n", i);
        }
        let text = Paragraph::new(s);
        frame.render_widget(text, area);
    }
}

struct PlaygoundRightView<'a>(Option<&'a dyn GameModelInterface>);
impl TuiView for PlaygoundRightView<'_> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        sub_views_with_layouts(
            frame,
            area,
            Vertical,
            [
                (&mut PlaygoundRightTopView(), Length(1)),
                (&mut GameView(self.0), Length(33)),
            ],
        );
    }
}

struct PlaygoundRightTopView();
impl TuiView for PlaygoundRightTopView {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let mut s = String::from(" ");
        for i in 0x0..0x10 {
            s += &format!(" [{:X}]", i);
        }
        let text = Paragraph::new(s);
        frame.render_widget(text, area);
    }
}
