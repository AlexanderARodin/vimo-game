#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::game_cell_state::{GameCellState,GetGameCellState};

//  //  //  //  //  //  //  //
pub struct AppModel {
    pub(in super) ed_state: edtui::EditorState,
    pub(in super) ed_handler: edtui::EditorEventHandler,
    pub(in super) state: AppModelState,
}

#[derive(PartialEq)]
pub enum AppModelState {
    //EditorFocused,
    OffFocused,
    Exiting,
}

impl AppModel {
    pub fn new() -> Self {
        let new_model = Self {
            ed_state: edtui::EditorState::new(edtui::Lines::from("started text.\n\nline 3\nFIN")),
            ed_handler: edtui::EditorEventHandler::default(),
            state: AppModelState::OffFocused,
        };

        trace!(" + AppModel::new()");
        new_model
    }

    pub fn is_exiting(&self) -> bool {
        self.state == AppModelState::Exiting
    }
}

impl GetGameCellState for AppModel {
    fn get_game_cell_state(&self, _i: u16, _j: u16) -> GameCellState {
        GameCellState::Empty
    }
}
