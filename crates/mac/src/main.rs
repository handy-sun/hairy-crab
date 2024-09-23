// mod proc;
// use crate::proc::cmd_execute;
use cmd_proc_macro::cmd_execute;
use std::str::{from_utf8, Utf8Error};

const SHORT_COMMIT: &[u8] = cmd_execute!("git rev-parse --short HEAD").trim_ascii_end(); // git rev-parse --short HEAD | tr -d '\n\r'

const RES: Result<&str, Utf8Error> = from_utf8(SHORT_COMMIT);

#[cfg(target_os = "windows")]
const BUILD_TIME: &[u8] = cmd_execute!("echo %date:~0,10% %time:~0,8%").trim_ascii_end();
#[cfg(not(target_os = "windows"))]
const BUILD_TIME: &[u8] = cmd_execute!("date '+%Y%m%d %H:%M:%S %:z'").trim_ascii_end();

// const COMMIT_TIME: &[u8] = cmd_execute!("git log -1 --format=%cd");
// const LATEST_TAG = cmd_execute!("git describe --tags --abbrev=0");
// const SUB_VERSION = cmd_execute!("git rev-list `git describe --tags --abbrev=0`..HEAD --count --first-parent");

fn main() {
    println!("{}", String::from_utf8_lossy(SHORT_COMMIT));
    println!("{}", RES.unwrap_or_default());

    println!("{}", String::from_utf8_lossy(BUILD_TIME));
}
