//! An Object is a collection of bricks with possibly many inventories or other such things.
use std::collections::HashMap;

use crate::engine::addons::register_api;
use crate::engine::game_events::GameEvent;
use log::info;
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
    pub fn get_game_events(&mut self) -> LuaResult<Vec<GameEvent>> {
        let mut ges = vec![];
        let lua = &self.lua;
        let globals = lua.globals();
        // let game_event_table: LuaTable = globals.get("game_event_table")?;
        // let new_game_event_count: i32 = game_event_table.get("count")?;
        // println!(
        //     "{} new game events queued by this object.",
        //     new_game_event_count
        // );
        // let events_array: LuaTable = game_event_table.get("events")?;
        // for game_event_idx in 0..new_game_event_count {
        //     let event = events_array.get(game_event_idx)?;
        //     ges.push(event);
        // }

        Ok(ges)
    }
    pub fn fire_tick(&mut self) -> LuaResult<Vec<GameEvent>> {
        let mut ges = vec![];
        let globals = self.lua.globals();

        let on_start: Function = globals.get("on_tick")?;
        on_start.call::<()>(())?;

        let game_event_table: LuaTable = globals.get("game_event_table")?;
        let new_game_event_count: i32 = game_event_table.get("count")?;
        if new_game_event_count > 0 {
            println!(
                "{} new game events queued by this object.",
                new_game_event_count
            );
        }
        let events_array: LuaTable = game_event_table.get("events")?;
        for game_event_idx in 0..new_game_event_count {
            let event = events_array.get(game_event_idx)?;
            ges.push(event);
        }

        // TODO: Handle reading in GameEvents here.
        Ok(ges)
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
            .set_name("mud mixer");
        lua_src.exec()?;

        let on_build: Function = globals.get("on_build")?;
        on_build.call::<()>(())?;

        Ok(Self { lua, script_name })
    }
}
