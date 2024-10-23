use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use crate::lua_connector;
use crate::prelude::*;

//  //  //  //  //  //  //  //
mod game_state;
mod impl_invoke_lua_update;

pub use game_state::*;

pub struct GameModel {
    lua: mlua::Lua,
    pub(crate) game_state: GameState,
}

impl GameModel {
    pub fn new(code: &str) -> Result<Self> {
        let new_one = Self {
            lua: lua_connector::init(code)?,
            game_state: GameState::Undef,
        };

        trace!(" + GameModel::new()");
        Ok(new_one)
    }
}
impl Drop for GameModel {
    fn drop(&mut self) {
        self.game_state = GameState::Undef;
        trace!(" - GameModel::drop()");
    }
}

impl GameModelInterface for GameModel {
    fn state(&self) -> &GameState {
        &self.game_state
    }

    fn update(&mut self, time: i64, opt_cmd: Option<GameCommand>) -> Result<()> {
        if let Some(GameCommand::Reset) = opt_cmd {
            self.game_state = GameState::Undef;
        }
        if let GameState::GameOver(_) = self.game_state {
            return Ok(());
        }
        let player: Option<(u16, u16)> = match &self.game_state {
            GameState::Undef => Some((2, 2)),
            GameState::Running(obj) => obj.player,
            _ => None,
        };
        let new_player = move_player(player, opt_cmd);
        self.game_state = self.invoke_lua_update(time, new_player)?;
        Ok(())
    }
}

fn move_player(opt_player: Option<(u16, u16)>, opt_cmd: Option<GameCommand>) -> Option<(u16, u16)> {
    match (opt_player, opt_cmd) {
        (Some(player), Some(command)) => match command {
            GameCommand::Up => Some((player.0, 0b1111 & player.1.overflowing_sub(1).0)),
            GameCommand::Down => Some((player.0, 0b1111 & (player.1 + 1))),
            GameCommand::Left => Some((0b1111 & player.0.overflowing_sub(1).0, player.1)),
            GameCommand::Right => Some((0b1111 & (player.0 + 1), player.1)),
            GameCommand::Reset => opt_player,
        },
        (_, None) | (None, _) => opt_player,
    }
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod move_player_tests {
    use super::*;

    #[test]
    fn no_moves() -> Result<()> {
        assert!(None == move_player(None, None));
        assert!(None == move_player(None, Some(GameCommand::Reset)));
        assert!(Some((2, 2)) == move_player(Some((2, 2)), None));
        assert!(Some((2, 2)) == move_player(Some((2, 2)), Some(GameCommand::Reset)));
        Ok(())
    }

    #[test]
    fn basic_moves() -> Result<()> {
        assert!(Some((2, 1)) == move_player(Some((2, 2)), Some(GameCommand::Up)));
        assert!(Some((2, 3)) == move_player(Some((2, 2)), Some(GameCommand::Down)));
        assert!(Some((1, 2)) == move_player(Some((2, 2)), Some(GameCommand::Left)));
        assert!(Some((3, 2)) == move_player(Some((2, 2)), Some(GameCommand::Right)));
        Ok(())
    }

    #[test]
    fn edge_moves() -> Result<()> {
        assert!(Some((0, 15)) == move_player(Some((0, 0)), Some(GameCommand::Up)));
        assert!(Some((0, 0)) == move_player(Some((0, 15)), Some(GameCommand::Down)));
        assert!(Some((15, 0)) == move_player(Some((0, 0)), Some(GameCommand::Left)));
        assert!(Some((0, 0)) == move_player(Some((15, 0)), Some(GameCommand::Right)));
        Ok(())
    }
}

#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn new_state_is_undef() -> Result<()> {
        let code = "";
        let model = GameModel::new(code)?;
        assert!(model.game_state == GameState::Undef);
        Ok(())
    }

    #[test]
    fn there_is_no_update_error() -> Result<()> {
        let code = "";
        let mut model = GameModel::new(code)?;
        let r = model.update(-1, None);
        assert!(r.is_err());
        Ok(())
    }
}
