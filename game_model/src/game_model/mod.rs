use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use crate::lua_connector;
use crate::CellState;
use crate::GameCommand;
use crate::GameModelInterface;

//  //  //  //  //  //  //  //
mod impl_update;
mod impl_action;

#[derive(PartialEq)]
pub enum GameState {
    Undef,
    Running(GameObjects),
    GameOver(String),
}

#[derive(PartialEq)]
pub struct GameObjects {
    pub(super) player: Option<(u16, u16)>,
    pub(super) target: Option<(u16, u16)>,
    pub(super) obstacles: Vec<(u16, u16)>,
}

pub struct GameModel {
    lua: mlua::Lua,
    pub(crate) state: GameState,
}

impl GameModel {
    pub fn new(code: &str) -> Result<Self> {
        let new_one = Self {
            lua: lua_connector::init(code)?,
            state: GameState::Undef,
        };

        trace!(" + GameModel::new()");
        Ok(new_one)
    }
}
impl Drop for GameModel {
    fn drop(&mut self) {
        self.state = GameState::Undef;
        trace!(" - GameModel::drop()");
    }
}

impl GameModelInterface for GameModel {
    fn cell_state(&self, i: u16, j: u16) -> CellState {
        if let GameState::Running(objs) = &self.state {
            if objs.player == Some((i, j)) {
                return CellState::Player;
            }
            if objs.target == Some((i, j)) {
                return CellState::Target;
            }
            for obstacle in &objs.obstacles {
                if *obstacle == (i, j) {
                    return CellState::Obstacle;
                }
            }
        }
        CellState::Empty
    }

    fn update(&mut self, time: i64) -> Result<()> {
        self.update(time)
    }

    fn action(&mut self, act: GameCommand) -> Result<()> {
        self.action(act)
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn new_undef_state() -> Result<()> {
        let code = "";
        let model = GameModel::new(code)?;
        assert!(model.state == GameState::Undef);
        Ok(())
    }

    #[test]
    fn new_update_error() -> Result<()> {
        let code = "";
        let mut model = GameModel::new(code)?;
        let r = model.update(-1);
        assert!(r.is_err());
        Ok(())
    }
}
