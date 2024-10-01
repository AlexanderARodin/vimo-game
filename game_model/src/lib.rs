use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

mod game_interface;
pub use game_interface::*;

mod lua_connector;

//  //  //  //  //  //  //  //
pub struct GameModel {
    lua: mlua::Lua,
    game_state: GameState,
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
    fn cell_state(&self, i: u16, j: u16) -> CellState {
        if let GameState::Running(game_objects) = &self.game_state {
            if game_objects.player == Some((i, j)) {
                return CellState::Player;
            }
            if game_objects.target == Some((i, j)) {
                return CellState::Target;
            }
            for obstacle in &game_objects.obstacles {
                if *obstacle == (i, j) {
                    return CellState::Obstacle;
                }
            }
        }
        CellState::Empty
    }

    fn update(&mut self, time: i64) -> Result<()> {
        let update: mlua::Function = self.lua.globals().get("update")?;
        let update_result: mlua::Table =
            update.call::<_, mlua::Table>(mlua::Value::Integer(time))?;

        if let Ok(s) = update_result.get::<&str, String>("GameOver") {
            self.game_state = GameState::GameOver(s.clone());
            return Err(anyhow::anyhow!("GameOver <{}>", s));
        }
        {
            let objects = GameObjects {
                player: extract_xy(&update_result, "player"),
                target: extract_xy(&update_result, "target"),
                obstacles: Vec::new(),
            };

            self.game_state = GameState::Running(objects);
            Ok(())
        }
    }
    fn action(&mut self, _act: GameCommand) -> Result<()> {
        todo!("action()");
    }
}

fn extract_xy(update_result: &mlua::Table, name: &str) -> Option<(u16,u16)> {
    let Ok(pos) = update_result.get::<&str,mlua::Table>(name) else {
        return None;
    };
    let Ok(x)= pos.get::<&str,u16>("x") else {
        return None;
    };
    let Ok(y) = pos.get::<&str,u16>("y") else {
        return None;
    };
    Some((x,y))
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod game_model_tests {
    use super::*;

    #[test]
    fn basic_update() -> Result<()> {
        let code = "function update(time) print('in update', time); return {}; end";
        let mut gmodel = GameModel::new(code)?;
        let _ = gmodel.update(-1)?;
        Ok(())
    }

    #[test]
    fn basic_fail_update() -> Result<()> {
        let code = "print('in update()', time)";
        let mut gmodel = GameModel::new(code)?;
        let res = gmodel.update(-1);
        assert!(res.is_err(), "Must be an Error about update()");
        Ok(())
    }

    #[test]
    fn basic_action() -> Result<()> {
        let code = "";
        let mut gmodel = GameModel::new(code)?;
        let _ = gmodel.action(GameCommand::Up)?;
        Ok(())
    }

    #[test]
    fn basic_fail_action() -> Result<()> {
        let code = "print('in action()', time)";
        let mut gmodel = GameModel::new(code)?;
        let res = gmodel.action(GameCommand::Up);
        assert!(res.is_err(), "Must be an Error about action()");
        Ok(())
    }

}
