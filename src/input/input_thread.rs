use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use crate::input::input::Input;

pub struct InputThread {
    input_buffer: Arc<Mutex<Vec<Input>>>,
    run_flag: bool,
}
impl InputThread {
    pub async fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.run_flag = true;
        let _raw_mode_guard = RawModeGuard;
        while self.run_flag {
            if let Event::Key(key_event) = event::read()? {
                if let Ok(mut buffer) = self.input_buffer.lock() {
                    buffer.push(Input {
                        input: key_event.code.to_string(),
                    });
                }
            }
        }
        Ok(())
    }
    pub async fn stop(&mut self) {
        self.run_flag = false;
    }
}

// A simple guard to ensure raw mode is disabled when the scope ends
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
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
