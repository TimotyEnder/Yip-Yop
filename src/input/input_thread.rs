use std::collections::HashSet;
use std::sync::Mutex;
use winit::keyboard::KeyCode;

use once_cell::sync::Lazy;

use crate::logger::logger::LOG;

pub static INPUT: Lazy<Mutex<InputState>> = Lazy::new(|| Mutex::new(InputState::new()));

pub struct InputState {
    keys: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }

    pub fn press(&mut self, key: KeyCode) {
        self.keys.insert(key);
    }

    pub fn release(&mut self, key: KeyCode) {
        self.keys.remove(&key);
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.keys.contains(&key)
    }
}
