use log::info;
use mlua::Function;
use mlua::UserData;
use mlua::Value;
use mlua::Variadic;
use mlua::prelude::*;

use crate::engine::game_events::GameEvent;
use crate::engine::game_events::GameEventType;
use crate::engine::object::Object;

pub fn register_api(lua: Lua) -> LuaResult<Lua> {
    let globals = lua.globals();
    let game_event_constructor = lua.create_function(|lua, strings: Variadic<String>| {
        let mut arg_v = vec![];
        for x in strings {
            arg_v.push(x);
        }
        let mut iter_arg_v = arg_v.iter();
        let game_event_type = iter_arg_v.next().unwrap();

        let game_event_type = match game_event_type.as_str() {
            "NewInventory" => GameEventType::NewInventory { inventory_size: 16 },
            _ => GameEventType::Nil,
        };

        Ok(GameEvent {
            event_type: game_event_type,
        })
    })?;
    globals.set("GameEvent", game_event_constructor)?;

    Ok(lua)
}

// pub fn addon() -> LuaResult<()> {
//     // let mut obj = Object::spawn_from_script("mud_mixer".to_string()).unwrap();
//     // let a = obj.fire_tick();
//     // let ges = obj.get_game_events().unwrap();
//     // for game_event in ges {
//     //     info!("game event {:?}", game_event)
//     // }
//     // info!("{:?}", a);
//     Ok(())
// }
