use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::{
    io::{self},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering::Relaxed},
    },
};

use crate::logger::logger::LOG;

pub struct InputThread {
    input_buffer: Arc<Mutex<Vec<crossterm::event::KeyCode>>>,
    run_flag: Arc<AtomicBool>,
}
impl InputThread {
    pub fn new() -> Self {
        Self {
            input_buffer: Arc::new(Mutex::new(Vec::<crossterm::event::KeyCode>::new())),
            run_flag: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn get_run_flag(&self) -> Arc<AtomicBool> {
        self.run_flag.clone()
    }

    pub fn get_input_buffer(&self) -> Arc<Mutex<Vec<crossterm::event::KeyCode>>> {
        self.input_buffer.clone()
    }
    pub fn stop(&mut self) {
        self.run_flag
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn input_recieved(
        &self,
        key_code: crossterm::event::KeyCode,
    ) -> Result<bool, &'static str> {
        if let Ok(buffer) = self.input_buffer.lock() {
            return Ok(buffer.contains(&key_code));
        }
        Err("could not gain control of input buffer mutex")
    }
    pub fn wipe_input_buffer(&mut self) -> Result<(), &'static str> {
        if let Ok(mut buffer) = self.input_buffer.lock() {
            LOG.lock()
                .expect("could not aquire logger lock")
                .logmsg(format!("{:?}", buffer).as_str());
            buffer.clear();
            return Ok(());
        }
        Err("could not gain control of input buffer mutex")
    }
}
pub fn run(run_flag: Arc<AtomicBool>, input_buffer: Arc<Mutex<Vec<KeyCode>>>) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    run_flag.store(true, Relaxed);
    while run_flag.load(Relaxed) {
        if let Event::Key(key_event) = event::read()? {
            if let Ok(mut buffer) = input_buffer.lock() {
                buffer.push(key_event.code);
            }
        }
    }
    Ok(())
}
