#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};


pub struct AppComponent {
}

impl WidgetRef for AppComponent {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    }
}
