#![allow(special_module_name)]
mod lib;
use std::env;
use std::time::Instant;

fn main() -> Result<(),  &'static str> {
    match env::args().skip(1).collect::<Vec<_>>().first() {
        Some(p) => {
            let t = Instant::now();
            // println!("{:#?}", lib::collect_matched_pids(p));
            let v = lib::collect_matched_pids(p);
            println!("{:?}", t.elapsed());
            for e in v {
                println!("pid: {}, cmdline: {}", e.pid, e.cmdline);
            }
            Ok(())
        }
        None => return Err("Must input one arg"),
    }
}
