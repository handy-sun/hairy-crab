use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

pub type BoxStrRE = Result<BgFileWriter, Box<str>>;
// type StaticStrRE = Result<BgFileWriter, &'static str>;

/// Start a background thread to write data into file
#[allow(dead_code)]
pub struct BgFileWriter {
    transmitte: Sender<String>,
    // rx: Receiver<T>,
    bg_thrd: JoinHandle<()>,
    // out_file: &'a mut File,
}

impl BgFileWriter {
    pub fn init(file_name: &str) -> BoxStrRE {
        let (tx, rx) = mpsc::channel::<String>();

        let handle = thread::Builder::new()
            .name("bg-file-writer-thread".to_string())
            .spawn({
                let mut file = File::create(file_name)
                    .map_err(|err| format!("Could not create file : {}", err).into_boxed_str())?;
                move || {
                    for msg in &rx {
                        file.write_all(msg.as_bytes())
                            .unwrap_or_else(|op| eprint!("op={}", op));
                    }
                }
            })
            .map_err(|err| format!("Thread error: {}", err).into_boxed_str())?;

        Ok(Self {
            transmitte: tx,
            bg_thrd: handle,
        })
    }

    #[allow(dead_code)]
    pub fn send_bytes(&mut self, str_data: &[u8]) -> Result<(), mpsc::SendError<String>> {
        // if let Err(err) = self.transmitte.send(String::from_utf8_lossy(str_data).to_string()) {
        //     return Err(anyhow!("SendError: {:?}", err));
        // }
        // Ok(())
        self.transmitte
            .send(String::from_utf8_lossy(str_data).to_string())
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        self.bg_thrd.is_finished()
    }
}
