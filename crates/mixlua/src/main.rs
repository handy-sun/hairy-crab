use std::{fs, env};
use mlua::{Lua, ObjectLike, Table, Value};
use walkdir::WalkDir;

#[allow(dead_code)]
fn print_table(tab: &Table, indent: usize) -> mlua::Result<String> {
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
    Ok(output.concat())
}

fn main() -> mlua::Result<()> {
    let self_dir = fs::read_link("/proc/self/exe")?;
    let self_dir = self_dir
        .parent()
        .expect("Should get dir")
        .join("../../crates/mixlua/lua")
        .canonicalize()?;

    env::set_current_dir(&self_dir)?;
    println!("Successfully changed working directory to {}", self_dir.display());

    // This loads the default Lua std library *without* the debug library.
    let lua = Lua::new();
    let globals = lua.globals();

    let walker = WalkDir::new(self_dir).max_depth(1).follow_links(true).into_iter();
    for res_entry in walker {
        if let Ok(entry) = res_entry {
            if entry.path().is_dir() {
                continue;
            }
            println!("{:?}", entry.path());
            let file_content = fs::read_to_string(entry.path())?;
            lua.load(&file_content).exec()?;
        }
    }

    let lua_str = lua.create_string(b"\x04\x00\x00\x00\x0b\x03\xa1\x0a")?;

    let _res: Table = globals
        .get::<Table>("Structure")?
        .call_function("new_inner", lua_str)?;

    let inner: Table = _res.get("inner")?;
    println!("{}", print_table(&inner, 0)?);

    let package: Table = globals.get("package")?;
    let path: mlua::String = package.get("path")?;
    println!("package.path: {}", path.to_str()?);

    let c_path: mlua::String = package.get("cpath")?;
    println!("package.cpath: {}", c_path.to_str()?);

    // let loaded = package.get::<Table>("loaded")?;
    // println!("{:?}", loaded);
    // let pair = globals.call_function("pairs", loaded)?;
    // println!("pairs of package.loaded: {}", print_table(&pair, 0)?);
    Ok(())
}
