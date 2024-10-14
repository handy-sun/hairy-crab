
use mlua::{Lua, ObjectLike, Table, Value};

fn print_table<'a>(tab: &'a Table, indent: usize) -> mlua::Result<String> {
    let prefix = vec!["  "; indent].concat();
    let mut output = String::with_capacity(64);
    for pair in tab.pairs::<Value, Value>() {
        let (key, value) = pair?;
        let key_str = match key {
            Value::Integer(i) => i.to_string(),
            Value::String(s) => s.to_string_lossy().to_string(),
            _ => String::from("nil"),
        };

        match value {
            Value::Table(child) => {
                output.push_str(format!("{}{:?}:\n", prefix, key_str).as_ref());
                output.push_str(&print_table(&child, indent + 1)?);
            }
            Value::Integer(integer) => output.push_str(format!("{}{:?}: {}\n", prefix, key_str, integer).as_ref()),
            Value::String(s) => output.push_str(format!("{}{:?}: {}\n", prefix, key_str, s.to_string_lossy()).as_ref()),
            _ => ()
        }
    }
    return Ok(output);
}

fn main() -> mlua::Result<()> {
    let Some(lua_file) = std::env::args().skip(1).next() else {
        eprintln!("Must input lua file");
        std::process::exit(1);
    };

    if !std::fs::exists(&lua_file)? {
        eprintln!("Lua file not exist");
        std::process::exit(1);
    }

    // This loads the default Lua std library *without* the debug library.
    let lua_ctx = Lua::new();
    let globals = lua_ctx.globals();
    let file_content = std::fs::read(&lua_file)?;
    lua_ctx.load(file_content.to_vec()).exec()?;

    // if pos_b want call method must create by globals
    let bytes = "\x04\x00\x01\x00\x0b\x03\x01\x0a";

    let res: Table = globals
        .get::<Table>("Structure")?
        .call_function("new", bytes)?;

    println!("result: {:?}", res.to_string()?);

    let inner: Table = res.get("inner")?;
    println!("{}", print_table(&inner, 0)?);

    Ok(())
}
