use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, poll, read},
    execute, terminal,
};
use std::{
    collections::HashSet,
    io::{self, stdout},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering::Relaxed},
    },
    time::Duration,
};

use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};

use crate::logger::logger::LOG;

pub struct InputThread {
    input_buffer: Arc<Mutex<HashSet<crossterm::event::KeyCode>>>,
    run_flag: Arc<AtomicBool>,
}
impl InputThread {
    pub fn new() -> Self {
        Self {
            input_buffer: Arc::new(Mutex::new(HashSet::<crossterm::event::KeyCode>::new())),
            run_flag: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn get_run_flag(&self) -> Arc<AtomicBool> {
        self.run_flag.clone()
    }

    pub fn get_input_buffer(&self) -> Arc<Mutex<HashSet<crossterm::event::KeyCode>>> {
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
}
pub fn run(
    run_flag: Arc<AtomicBool>,
    input_buffer: Arc<Mutex<HashSet<KeyCode>>>,
) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    if let Err(e) = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
        )
    ) {
        LOG.lock()
            .expect("")
            .logerr(&format!("keyboard enhancement not supported: {e}"));
    }
    run_flag.store(true, Relaxed);
    while run_flag.load(Relaxed) {
        while poll(Duration::from_secs(0))? {
            match read()? {
                Event::Key(KeyEvent { code, kind, .. }) => {
                    if let Ok(mut buffer) = input_buffer.lock() {
                        match kind {
                            KeyEventKind::Press => {
                                buffer.insert(code);
                            }
                            KeyEventKind::Release => {
                                buffer.remove(&code);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        std::thread::yield_now();
    }
    Ok(())
}
