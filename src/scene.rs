use crate::{
    gameobject::GameObject,
    model::elements::pos3::Pos3,
    screenspace::{elements::drawable::Drawable, screen::screen::Screen},
}; // Add this line
use std::io::{self, Write};
use std::{collections::HashMap, thread::sleep, time::Duration};
pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<Pos3, Box<dyn GameObject>>,
}
impl Scene {
    pub fn with_dimensions(height: &usize, width: &usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            gameobjects: HashMap::new(),
        }
    }
    pub fn add_object(&mut self, object: Box<dyn GameObject>) {
        self.gameobjects.insert(object.position(), object);
    }
    pub fn run(&mut self, fps: &u64) {
        let sleep_time: Duration = Duration::from_secs_f64(1.0 / *fps as f64);
        let delta_time: f64 = 1.0 / *fps as f64;
        print!("\x1B[?1049h\x1B[?25l");
        io::stdout().flush().unwrap();
        self.start_objects();
        loop {
            self.update_objects(&delta_time);
            self.draw_objects();
            self.screen.draw_and_flush();
            sleep(sleep_time);
        }
    }
    fn draw_objects(&mut self) {
        for object in self.gameobjects.iter() {
            object.1.draw(&mut self.screen);
        }
    }
    fn start_objects(&mut self) {
        for object in self.gameobjects.iter_mut() {
            object.1.start();
        }
    }
    fn update_objects(&mut self, delta_time: &f64) {
        for object in self.gameobjects.iter_mut() {
            object.1.update(&delta_time);
        }
    }
}
