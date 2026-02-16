use mlua::Function;
use mlua::UserData;
use mlua::Value;
use mlua::prelude::*;

#[derive(Clone)]
pub struct GameEvent {
    event_type: String,
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
            // GameEvent Fired.
            let globals = lua.globals();
            match &aself.event_type {
                _event_type => {
                    println!("{}", _event_type)
                }
            }
            let game_event_table: Result<LuaTable, LuaError> = globals.get("game_event_table");
            let sure_game_event_table = game_event_table.unwrap();

            let count: Result<i32, LuaError> = sure_game_event_table.get("count");
            println!("current GameEvent count {:?}", count);
            Ok(())
        });

        // methods.add_meta_function(LuaMetaMethod::Add, |_, (vec1, vec2): (Vec2, Vec2)| {
        //     Ok(Vec2(vec1.0 + vec2.0, vec1.1 + vec2.1))
        // });
    }
}

pub fn addon() -> LuaResult<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    let game_event_table = lua.create_table()?;

    game_event_table.set("count", 0)?;

    globals.set("game_event_table", game_event_table)?;
    let f = lua.create_function(|_, ()| -> LuaResult<()> {
        // panic!("test panic");
        Ok(())
    })?;

    lua.globals().set("new_inventory", f)?;

    let game_event_constructor = lua.create_function(|_, (game_event_type): (String)| {
        Ok(GameEvent {
            event_type: game_event_type,
        })
    })?;
    globals.set("GameEvent", game_event_constructor)?;

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
        println!("Event needs to be handled.")
    }

    // lua_src.call();

    Ok(())
}
