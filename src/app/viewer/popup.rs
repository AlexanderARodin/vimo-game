#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};
use Constraint::*;

//  //  //  //  //  //  //  //
static SPC: u16 = 15;

pub fn render_popup(area: Rect, buf: &mut Buffer) {
    let vls = Layout::vertical([Percentage(SPC), Min(10), Percentage(SPC)]).split(area);
    let hls = Layout::horizontal([Percentage(SPC), Min(10), Percentage(SPC)]).split(vls[1]);

    let popup_area = hls[1];

    let popup_border = Block::bordered()
        .border_type(BorderType::Double)
        .title("GameModel Lua Editor");
    let inner = popup_border.inner(popup_area);

    Clear.render(popup_area, buf);
    popup_border.render(popup_area, buf);

    render_content(inner, buf);
}

//  //  //  //  //  //  //  //
fn render_content(area: Rect, buf: &mut Buffer) {
    Paragraph::new("t t t t t t t t").render(area, buf);
}
