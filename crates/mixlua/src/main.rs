use mlua::{Lua, ObjectLike, Table, Value};
use std::fs;
use std::path::PathBuf;

fn print_table<'a>(tab: &'a Table, indent: usize) -> mlua::Result<String> {
    let prefix = vec!["  "; indent].concat();
    let mut output: Vec<_> = Vec::with_capacity(16);

    for pair in tab.pairs::<Value, Value>() {
        let (key, value) = pair?;
        let key_str = match key {
            Value::Integer(i) => i.to_string(),
            Value::String(s) => s.to_string_lossy().to_string(),
            _ => String::from("nil"),
        };

        match value {
            Value::Table(child) => {
                output.push(format!("{}{}:\n", prefix, key_str));
                output.push(print_table(&child, indent + 1)?);
            }
            Value::Integer(integer) => {
                output.push(format!("{}{}: {}\n", prefix, key_str, integer))
            }
            Value::String(s) => output.push(format!(
                "{}{}: {}\n",
                prefix,
                key_str,
                s.to_string_lossy()
            )),
            _ => (),
        }
    }
    return Ok(output.concat());
}

fn main() -> mlua::Result<()> {
    let lua_path = if let Some(lua_file) = std::env::args().skip(1).next() {
        PathBuf::from(lua_file)
    } else {
        let self_path = fs::read_link("/proc/self/exe")?;
        self_path
            .parent()
            .expect("Should get dir")
            .join("../../crates/mixlua/lua/pos.lua")
            .canonicalize()?
    };

    if !fs::exists(&lua_path)? {
        eprintln!("Lua file not exist: {:?}", lua_path);
        std::process::exit(1);
    }

    // This loads the default Lua std library *without* the debug library.
    let lua_ctx = Lua::new();
    let globals = lua_ctx.globals();
    let file_content = fs::read_to_string(&lua_path)?;

    lua_ctx.load(&file_content).exec()?;
    // if pos_b want call method must create by globals
    let bytes = "\x04\x00\x01\x00\x0b\x03\x01\x0a";

    let res: Table = globals
        .get::<Table>("Structure")?
        .call_function("new", bytes)?;

    // println!("result: {:?}", res.to_string()?);

    let inner: Table = res.get("inner")?;
    println!("{}", print_table(&inner, 0)?);

    Ok(())
}
