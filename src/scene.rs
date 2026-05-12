use crate::{
    model::elements::pos3::Pos3,
    screenspace::{elements::drawable::Drawable, screen::screen::Screen},
}; // Add this line
use std::io::{self, Write};
use std::{collections::HashMap, thread::sleep, time::Duration};
pub struct Scene {
    screen: Screen,
    objects: HashMap<Pos3, Box<dyn Drawable>>,
}
impl Scene {
    pub fn with_dimensions(height: &usize, width: &usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            objects: HashMap::new(),
        }
    }
    pub fn add_object(&mut self, object: Box<dyn Drawable>) {
        self.objects.insert(object.position(), object);
    }
    pub fn run(&mut self, fps: &u64) {
        let sleep_time: Duration = Duration::from_secs(1 / fps);
        print!("\x1B[?1049h\x1B[?25l");
        io::stdout().flush().unwrap();
        loop {
            self.draw_objects();
            self.screen.draw_and_flush();
            sleep(sleep_time);
        }
    }
    fn draw_objects(&mut self) {
        for object in self.objects.iter() {
            object.1.draw(&mut self.screen);
        }
    }
}
