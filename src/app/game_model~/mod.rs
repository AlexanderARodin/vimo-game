use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod game_cell_state;
pub use game_cell_state::*;

mod lua_connector;

//  //  //  //  //  //  //  //
pub struct GameModel {
    lua: mlua::Lua,
}

impl GameModel {
    pub fn new() -> Result<Self> {
        let new_one = Self {
            lua: todo!("lua_connector.init()"),
        };

        trace!(" + GameModel::new()");
        Ok(new_one)
    }
}
impl Drop for GameModel {
    fn drop(&mut self) {
        trace!(" - GameModel::drop()");
    }
}

impl GetGameCellState for GameModel {
    fn get_game_cell_state(&self, _i: u16, _j: u16) -> GameCellState {
        GameCellState::Empty
    }
}
