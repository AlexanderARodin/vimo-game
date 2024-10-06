#[allow(non_upper_case_globals)]
static ebg: Color = Color::Rgb(64, 64, 64);

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::prelude::*;
use ratatui::widgets::Block;

use game_model::{
    CellState::{self, *},
    GameModelInterface,
};

//  //  //  //  //  //  //  //
pub struct GameWidget<'a>(pub Option<&'a dyn GameModelInterface>);
impl Widget for GameWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner_area = block.inner(area);

        block.render(area,buf);

        if let Some(game) = self.0 {
            for i in 0x0..0x10 {
                for j in 0x0..0x10 {
                    if let Some(rc) = ij2rect(&inner_area, i, j) {
                        let cell_state = game.cell_state(i, j);
                        GameCellWG(cell_state).render(rc, buf);
                    }
                }
            }
        }
    }
}

//  //  //  //  //  //  //  //
struct GameCellWG(CellState);
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
            Player => {
                buf[center].set_char('*').set_bg(ebg).set_fg(Color::Green);
                buf[left].set_char('[').set_bg(ebg);
                buf[right].set_char(']').set_bg(ebg);
            }
            Target => {
                buf[center].set_char('#').set_bg(Color::Black).set_fg(Color::Red);
                buf[left].set_char(' ').set_bg(Color::Black);
                buf[right].set_char(' ').set_bg(Color::Black);
            }
            Obstacle => {
                buf[center].set_char(' ').set_bg(Color::Black);
                buf[left].set_char(' ').set_bg(Color::Black);
                buf[right].set_char(' ').set_bg(Color::Black);
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
