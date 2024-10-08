#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Clear};
use Constraint::*;

//  //  //  //  //  //  //  //
static SPC: u16 = 20;

pub fn render_editor_popup(area: Rect, buf: &mut Buffer, state: &mut edtui::EditorState) {
    let vls = Layout::vertical([Percentage(SPC), Min(10), Max(1)]).split(area);
    let hls = Layout::horizontal([Percentage(SPC), Min(10), Max(2)]).split(vls[1]);

    let popup_area = hls[1];

    let popup_border = Block::bordered()
        .border_type(BorderType::Double)
        .title("GameModel Lua Editor");
    let inner = popup_border.inner(popup_area);

    Clear.render(popup_area, buf);
    popup_border.render(popup_area, buf);
    edtui::EditorView::new(state)
        .render(inner, buf);
}

