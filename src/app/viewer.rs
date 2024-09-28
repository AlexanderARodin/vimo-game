use super::model::AppModel as Model;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};

//  //  //  //  //  //  //  //
// edtui requires mut state for new()
pub fn view(model: &mut Model, frame: &mut Frame) {
    let area = frame.area();
    // layout
    let [title_area, main_area, editor_area] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(19),
        Constraint::Min(2),
    ])
    .areas(area);

    view_title(frame, &title_area);
    view_main(frame, &main_area);
    view_editor(frame, &editor_area, &mut model.ed_state);
}

//  //  //  //  //  //  //  //
fn view_title(frame: &mut Frame, area: &Rect) {
    let title =
        Paragraph::new("main title here").block(Block::bordered().title("title of Main Title"));
    frame.render_widget(title, *area);
}

fn view_editor(frame: &mut Frame, area: &Rect, ed_state: &mut edtui::EditorState) {
    let editor = edtui::EditorView::new(ed_state);
    frame.render_widget(editor, *area);
}

fn view_main(frame: &mut Frame, area: &Rect) {
    let main_block = Block::bordered();
    let main_inner = main_block.inner(*area);
    frame.render_widget(main_block, *area);

    let [main_left, main_right] =
        Layout::horizontal([Constraint::Length(3), Constraint::Min(16)]).areas(main_inner);
    view_main_left(frame, &main_left);
    //frame.render_widget(main_right_ed, main_right);
}

fn view_main_left(frame: &mut Frame, area: &Rect) {
    let main_left_text = Paragraph::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\nC\nD\nE\nF\n<-->");
    frame.render_widget(main_left_text, *area);
}
