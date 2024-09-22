mod bg_fwrt;

use std::thread;
use std::time::Duration;

use chrono::Local;

use crate::bg_fwrt::BgFileWriter;

const TEST_FILE: &str = "/tmp/bg_fwrt.txt";

const HALF_SEC: Duration = Duration::from_millis(500);

fn main() -> Result<(), Box<str>> {
    let mut bgw = BgFileWriter::init(TEST_FILE)?;
    for _ in 0..5 {
        bgw.send_bytes(format!("{}\n", Local::now()).as_bytes())
            .unwrap_or_else(|op| eprintln!("Send Bytes: {}", op));
        thread::sleep(HALF_SEC);
    }
    bgw.send_bytes(b"finish\n")
        .map_err(|op| format!("Send Bytes: {}", op).into_boxed_str())
}
