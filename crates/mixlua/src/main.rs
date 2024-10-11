use mlua::{Lua, ObjectLike, Table};

fn main() -> mlua::Result<()> {
    // This loads the default Lua std library *without* the debug library.
    let lua_ctx = Lua::new();
    let globals = lua_ctx.globals();

    globals.set("string_var", "hello")?;
    globals.set("int_var", 42)?;

    assert_eq!(globals.get::<String>("string_var")?, "hello");
    assert_eq!(globals.get::<i64>("int_var")?, 42);

    lua_ctx
        .load(
            r#"
            Position = {}
            Position.__index = Position
            function Position.new(x, y)
                return setmetatable({x=x, y=y}, Position)
            end
 
            function Position:add(other)
                return Position.new(self.x + other.x, self.y + other.y)
            end
            "#,
        )
        .set_name("example code")
        .exec()?;

    let pos_a = lua_ctx.create_table()?;
    pos_a.set("x", 1i32)?;
    pos_a.set("y", 2i32)?;

    let pos_b = lua_ctx.create_table()?;
    pos_b.set("x", 3i32)?;
    pos_b.set("y", 4i32)?;

    // let pos_sum = globals.get::<Table>("Position")?
    //     .call::<()>("new", 1i32, 2i32)?
    //     .call::<()>("add", pos_b)?;
    let pos_sum: Table = pos_a.call_function("Position:add", pos_b)?;

    println!(
        "Lua structure result: ({}, {})",
        pos_sum.get::<i32>("x")?,
        pos_sum.get::<i32>("y")?
    );

    Ok(())
}
