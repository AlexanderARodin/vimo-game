use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use game_model::prelude::*;

//  //  //  //  //  //  //  //
pub struct AppModel {
    pub(crate) start_time: std::time::SystemTime,
    pub(crate) tick_counter: u16,
    pub(crate) game_counter: i64,
    pub(crate) game_actions: Vec<char>,
    pub(super) game: Option<GameModel>,
    pub(super) game_editor_state: edtui::EditorState,
    pub(super) game_editor_handler: edtui::EditorEventHandler,
    pub(super) command_editor_state: edtui::EditorState,
    pub(super) ed_handler: edtui::EditorEventHandler,
    pub(super) state: AppModelState,
    pub(crate) is_popup: bool,
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
            tick_counter: 0,
            game_counter: 0,
            game_actions: Vec::new(),
            game: None,
            game_editor_state: edtui::EditorState::new(edtui::Lines::from(START_CODE)),
            game_editor_handler: edtui::EditorEventHandler::default(),
            command_editor_state: edtui::EditorState::new(edtui::Lines::from("0jjj3k\njkk")),
            ed_handler: edtui::EditorEventHandler::default(),
            state: AppModelState::OffFocused,
            is_popup: false,
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
local player;

function action(ac)
    -- print("act:", ac)
    if ac == 1 then
        player = { player[1], player[2] - 1 }
    end
    if ac == 2 then
        player = { player[1], player[2] + 1 }
    end
    if ac == 3 then
        player = { player[1] - 1, player[2] }
    end
    if ac == 4 then
        player = { player[1] + 1, player[2] }
    end
end
function update(time)
    if time == -1 then
        player = {2, 2}
    end
    return {
        player = player,
        target = {13,5},
        obstacles = {
            {9,9},
            {8,9},
            {9,13},
        },
        GameOver,
    }
end
"#;
