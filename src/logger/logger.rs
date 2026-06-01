pub struct Logger {
    writer: BufWriter<File>,
}
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{LockResult, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

//pub static INPUT: Lazy<Mutex<InputThread>> = Lazy::new(|| Mutex::new(InputThread::new()));
pub static LOG: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));
impl Logger {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let file = File::create(format!("Dascii3Log:{}.txt", timestamp))
            .expect("could not create log file");
        let writer = BufWriter::new(file);
        Self { writer }
    }
    pub fn logmsg(&mut self, msg: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let message = format!("[LOG:{}]:{}\n", timestamp, msg);
        self.writer
            .write(message.as_bytes())
            .expect("could not write into log file");
    }
    pub fn logerr(&mut self, err: &str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let message = format!("[ERR:{}]:{}\n", timestamp, err);
        self.writer
            .write(message.as_bytes())
            .expect("could not write into log file");
    }
}
