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
        let game_event_table: LuaTable = globals.get("game_event_table")?;
        let new_game_event_count: i32 = game_event_table.get("count")?;
        println!(
            "{} new game events queued by this object.",
            new_game_event_count
        );
        for game_event_idx in 0..new_game_event_count {
            let events_array: Result<GameEvent, LuaError> = game_event_table.get("events");
        }

        // let game_event_type: String = game_event_table.get("event_type")?;

        // for even in game_event_table {
        //     println!("game event {:?}", even);
        //     let game_event_type: String =
        //         even.get("event_type").unwrap_or("MarkerEvent".to_string());

        //     let count: Result<i32, LuaError> = game_event_table.get("count")?;

        //     let game_event = GameEvent {
        //         event_type: game_event_type.to_string(),
        //     };
        //     info!("Game event {:?}", game_event);
        //     ges.push(game_event);
        // }

        Ok(ges)
    }
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
