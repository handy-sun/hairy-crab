// use std::env;

use std::fs::File;
use std::io::{self, Write};
use std::thread::{self, ThreadId};

use std::sync::mpsc;

use chrono::{Local, Timelike};

use std::time::Duration;

const TEST_FILE: &str = "/tmp/output.txt";

const THOU_MILL: Duration = Duration::from_millis(500);

fn write_to_file(
    thrd_id: &ThreadId,
    filename: &str,
    messages: mpsc::Receiver<String>,
) -> io::Result<()> {
    let mut file = File::create(filename).expect("Could not create file");
    for msg in messages {

        let now = Local::now().time().nanosecond();
        let str_data = format!(
            "{:?}: {}{:?}: <recv time: {:10}>\n",
            thrd_id,
            msg,
            thread::current().id(),
            // now.timestamp_micros()
            now
        );
        file.write_all(str_data.as_bytes())?;
    }
    file.write_all(b"step\n")
}

fn main() -> io::Result<()> {
    let (tx0, rx0) = mpsc::channel();
    // let (tx1, rx1) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            let now = Local::now().time().nanosecond();
            tx0.send(format!("<send time: {:10}>\n", now)).unwrap_or_default();
            thread::sleep(THOU_MILL);
        }
    });

    let h1 = handle1.thread().id();
    let handle2 = thread::spawn(move || match write_to_file(&h1, TEST_FILE, rx0) {
        Err(_err) => {
            eprintln!("Error: {}", _err);
            std::process::exit(1);
        }
        _ => {}
    });
    // dbg!(&handle1.thread().id());
    // dbg!(&handle2.thread().id());
    handle1.join().unwrap_or_default();
    handle2.join().unwrap_or_default();
    // drop(tx0);
    // drop(tx1);
    println!("main: {:?}", thread::current().id());
    Ok(())
}
