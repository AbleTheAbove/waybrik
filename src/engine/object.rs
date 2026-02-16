//! An Object is a collection of bricks with possibly many inventories or other such things.
use std::collections::HashMap;

use crate::engine::addons::register_api;
use mlua::Function;
use mlua::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ObjectID {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
/// This structure represents an in memory chunk.
pub struct ObjectCache {
    pub cache: HashMap<ObjectID, Object>,
}
impl ObjectCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(skip_serializing, skip_deserializing)]
    pub lua: Lua,
    // The script name.
    pub script_name: String,
}
impl Object {
    pub fn fire_tick(&mut self) -> LuaResult<()> {
        let globals = self.lua.globals();

        let on_start: Function = globals.get("on_tick")?;
        on_start.call::<()>(())?;
        // TODO: Handle reading in GameEvents here.
        Ok(())
    }
    pub fn spawn_from_script(script_name: String) -> LuaResult<Self> {
        let object_name = format!("Object {}", script_name);

        let mut lua = Lua::new();
        let globals = lua.globals();
        let game_event_table = lua.create_table()?;
        game_event_table.set("count", 0)?;
        globals.set("game_event_table", game_event_table)?;

        lua = register_api(lua)?;

        let lua_src = lua
            .load(include_str!(
                "../../assets/addons/core/objects/mud_mixer.lua"
            ))
            .set_name("example code");
        lua_src.exec()?;

        let on_start: Function = globals.get("on_build")?;
        on_start.call::<()>(())?;
        let game_event_table: Result<LuaTable, LuaError> = globals.get("game_event_table");

        for event in game_event_table.iter() {
            println!("Event needs to be handled.")
        }

        Ok(Self { lua, script_name })
    }
}
