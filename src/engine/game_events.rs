use log::info;
use mlua::UserData;
use mlua::Value;
use mlua::prelude::*;
#[derive(Clone, Debug)]
pub enum GameEventType {
    Nil,
    NewInventory { inventory_size: u64 },
}

#[derive(Clone, Debug)]
pub struct GameEvent {
    pub event_type: GameEventType,
}

impl FromLua for GameEvent {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
            _ => unreachable!(),
        }
    }
}

impl UserData for GameEvent {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("fire", |lua, aself, ()| {
            info!("GameEvent Fired.");
            let globals = lua.globals();
            match &aself.event_type {
                _event_type => {
                    info!("Game Event Type {:?}", _event_type)
                }
            }
            let game_event_table: Result<LuaTable, LuaError> = globals.get("game_event_table");
            let sure_game_event_table = game_event_table.unwrap();

            // sure_game_event_table.set("events", value);
            let event_array_table = lua.create_table()?;
            event_array_table.set(
                1,
                GameEvent {
                    event_type: GameEventType::Nil,
                },
            )?;
            // event_array_table.set(
            //     2,
            //     GameEvent {
            //         event_type: todo!(),
            //     },
            // )?;
            // event_array_table.set(
            //     3,
            //     GameEvent {
            //         event_type: todo!(),
            //     },
            // )?;

            let count: Result<i32, LuaError> = sure_game_event_table.get("count");
            let mut re_co = count.clone().unwrap();
            re_co += 1;

            println!("current GameEvent count {:?}", re_co);
            Ok(())
        });
    }
}
