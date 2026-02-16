use mlua::Function;
use mlua::prelude::*;

pub enum GameEvent {
    NewInventory,
}

pub fn addon() -> LuaResult<()> {
    let lua = Lua::new();
    let globals = lua.globals();
    let game_event_table = lua.create_table()?;
    globals.set("game_event_table", game_event_table)?;
    let f = lua.create_function(|_, ()| -> LuaResult<()> {
        // panic!("test panic");
        Ok(())
    })?;

    lua.globals().set("new_inventory", f)?;

    let lua_src = lua
        .load(include_str!(
            "../../../assets/addons/core/objects/mud_mixer.lua"
        ))
        .set_name("example code");
    lua_src.exec()?;

    let on_start: Function = globals.get("on_build")?;
    on_start.call::<()>(())?;
    let game_event_table: Result<LuaTable, LuaError> = globals.get("game_event_table");

    for event in game_event_table.iter() {
        println!(" event {:?}", event);
        let event_type: Result<u8, LuaError> = event.get("event_type");
    }

    // lua_src.call();

    Ok(())
}
