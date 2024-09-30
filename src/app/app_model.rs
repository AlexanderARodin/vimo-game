use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use super::game_model::GameModel;

//  //  //  //  //  //  //  //
pub struct AppModel {
    pub(super) game: GameModel,
    pub(super) ed_state: edtui::EditorState,
    pub(super) ed_handler: edtui::EditorEventHandler,
    pub(super) state: AppModelState,
}

#[derive(PartialEq)]
pub enum AppModelState {
    //EditorFocused,
    OffFocused,
    Exiting,
}

impl AppModel {
    pub fn new() -> Result<Self> {
        let new_model = Self {
            game: GameModel::new()?,
            ed_state: edtui::EditorState::new(edtui::Lines::from("started text.\n\nline 3\nFIN")),
            ed_handler: edtui::EditorEventHandler::default(),
            state: AppModelState::OffFocused,
        };

        trace!(" + AppModel::new()");
        Ok(new_model)
    }

    pub fn is_exiting(&self) -> bool {
        self.state == AppModelState::Exiting
    }
}
