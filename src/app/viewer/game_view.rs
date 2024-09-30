#[allow(non_upper_case_globals)]
static ebg: Color = Color::Rgb(64, 64, 64);

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::prelude::*;
use ratatui::widgets::Block;

use game_model::{
    GameCellState::{self, *},
    GetGameCellState,
};
use super::tui_view::*;

//  //  //  //  //  //  //  //
pub struct GameView<'a>(pub &'a dyn GetGameCellState);
impl TuiView for GameView<'_> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered();
        let inner_area = block.inner(area);

        frame.render_widget(block, area);

        for i in 0x0..0x10 {
            for j in 0x0..0x10 {
                if let Some(rc) = ij2rect(&inner_area, i, j) {
                    let cell_state = self.0.get_game_cell_state(i, j);
                    frame.render_widget(GameCellWG(cell_state), rc);
                }
            }
        }
    }
}

//  //  //  //  //  //  //  //
struct GameCellWG(GameCellState);
impl Widget for GameCellWG {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let left = Position::new(area.x, area.y);
        let right = Position::new(area.x + 2, area.y);
        let center = Position::new(area.x + 1, area.y);

        match self.0 {
            Empty => {
                buf[center].set_char(' ').set_bg(ebg);
                buf[left].set_char(' ').set_bg(ebg);
                buf[right].set_char(' ').set_bg(ebg);
            }
            Test => {
                buf[center].set_char('+').set_bg(ebg);
                buf[left].set_char('>').set_bg(ebg);
                buf[right].set_char('<').set_bg(ebg);
            }
        }
    }
}

//  //  //  //  //  //  //  //
fn ij2rect(src_rect: &Rect, i: u16, j: u16) -> Option<Rect> {
    let x = i * 4 + 1 + src_rect.x;
    let y = j * 2 + src_rect.y;
    if (x + 3) > (src_rect.x + src_rect.width) {
        return None;
    }
    if (y + 1) > (src_rect.y + src_rect.height) {
        return None;
    }
    Some(Rect {
        x,
        width: 3,
        y,
        height: 1,
    })
}
