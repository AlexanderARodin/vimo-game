use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use mlua::Lua;

//  //  //  //  //  //  //  //
pub fn init(source_code: &str) -> Result<Lua> {
    let lua = Lua::new();

    //set_printer_to_info(&lua)?;
    //set_trace_to_trace(&lua)?;

    lua.load(source_code).exec()?;

    Ok(lua)
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod lua_tests {
    use super::*;

    #[test]
    fn basic_creating() -> Result<()> {
        let code = "-- comment";
        let _ = init(code)?;
        Ok(())
    }
}
