#![allow(special_module_name)]
mod lib;
use std::env;
// use std::time::Instant;

fn main() -> Result<(), &'static str> {
    match env::args().skip(1).next() {
        Some(p) => {
            // let t = Instant::now();
            // let mut iter = lib::collect_matched_pids(p);
            // for proc in iter.by_ref() {
            //     println!("{:?}", proc);
            //     println!("{:?}", t.elapsed());
            // }
            println!("{:}", lib::join_result_pids(&p));
            // eprintln!("elapsed: {:?}", t.elapsed());
            Ok(())
        }
        None => return Err("Must input one arg"),
    }
}
