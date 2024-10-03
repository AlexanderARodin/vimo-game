use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use game_model::GameModel;

//  //  //  //  //  //  //  //
pub struct AppModel {
    pub(crate) start_time: std::time::SystemTime,
    pub(crate) counter: i64,
    pub(super) game: Option<GameModel>,
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
            start_time: std::time::SystemTime::now(),
            counter: -1,
            game: None,
            ed_state: edtui::EditorState::new(edtui::Lines::from(START_CODE)),
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

static START_CODE: &str = r#"-- demo startup dummy code on Lua
-- print("hell no word!!1")
function update(time)
    -- print("time:", time)
    return {
        player = {3,2},
        target = {13,5},
        obstacles = {
            {9,9},
            {8,9},
            {9,9},
        },
        GameOver,
    }
end
"#;
