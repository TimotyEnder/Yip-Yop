use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::{
    error::Error,
    io::{self},
    sync::{Arc, Mutex},
};

pub struct InputThread {
    input_buffer: Arc<Mutex<Vec<crossterm::event::KeyCode>>>,
    run_flag: bool,
}
impl InputThread {
    pub fn new() -> Self {
        Self {
            input_buffer: Arc::new(Mutex::new(Vec::<crossterm::event::KeyCode>::new())),
            run_flag: false,
        }
    }
    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.run_flag = true;
        while self.run_flag {
            if let Event::Key(key_event) = event::read()? {
                if let Ok(mut buffer) = self.input_buffer.lock() {
                    buffer.push(key_event.code);
                }
            }
        }
        Ok(())
    }
    pub fn stop(&mut self) {
        self.run_flag = false;
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
            buffer.clear();
            return Ok(());
        }
        Err("could not gain control of input buffer mutex")
    }
}
// fn main() -> io::Result<()> {
//     // Enable raw mode for immediate keypress detection
//     terminal::enable_raw_mode()?;
//     // Ensure raw mode is disabled when the program exits
//     let _raw_mode_guard = RawModeGuard;

//     println!("Press any key (press 'q' to quit)");
//     io::stdout().flush()?;  // Make sure the prompt appears immediately

//     loop {
//         // The read() function will now return immediately on any keypress
//         if let Event::Key(key_event) = event::read()? {
//             if key_event.code == KeyCode::Char('q') {
//                 println!("\nQuitting. Goodbye!");
//                 break;
//             } else {
//                 print!("\rYou pressed: {:<20}", key_event.code);
//                 io::stdout().flush()?;
//             }
//         }
//     }

//     Ok(())
// }
