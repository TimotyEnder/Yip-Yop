use crate::{
    ecs::{
        component_system::core_components::{
            body::{self, Body},
            script_component::{self, ScriptComponent},
        },
        gameobject::{self, GameObject},
    },
    input::input_thread::{self, InputThread, run},
    logger::logger::LOG,
    screenspace::screen::screen::Screen,
}; // Add this line
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    mem::take,
    thread::sleep,
    time::{Duration, Instant},
};
use std::{
    io::{self, Write},
    sync::Mutex,
};

pub static INPUT: Lazy<Mutex<InputThread>> = Lazy::new(|| Mutex::new(InputThread::new()));

pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<usize, GameObject>,
    scripts: HashMap<usize, ScriptComponent>,
}
impl Scene {
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            gameobjects: HashMap::new(),
            scripts: HashMap::new(),
        }
    }
    pub fn add_object(&mut self, object: GameObject) {
        self.gameobjects.insert(object.get_id(), object);
    }
    pub fn add_script(&mut self, script: ScriptComponent, gameobject_id: usize) {
        self.scripts.insert(gameobject_id, script);
    }
    pub async fn run(mut self, fps: f32) {
        self.start_objects();
        let _input_stopped = InputThreadStopper;
        let mut last_frame_time = Instant::now();
        let frame_duration = Duration::from_secs_f32(1.0 / fps);
        let mut frame_timer = Instant::now();
        minigw::new::<u8, _>(
            "Yip-Yop",
            self.screen.get_width() as u32,
            self.screen.get_height() as u32,
            move |_window, _input, render_texture, _debug_ui| {
                let frame_time = frame_timer.elapsed();
                if frame_time < frame_duration {
                    std::thread::sleep(frame_duration - frame_time);
                }
                frame_timer = Instant::now();
                let now = Instant::now();
                let delta_time = now.duration_since(last_frame_time);
                last_frame_time = now;
                self.update_objects(delta_time.as_secs_f64());
                self.draw_objects();
                self.screen.draw_and_flush(render_texture);
            },
        );
    }
    fn draw_objects(&mut self) {
        for object in self.gameobjects.iter_mut() {
            if let Some(body) = object.1.get_component_mut::<Body>() {
                body.draw(&mut self.screen);
            }
        }
    }
    fn start_objects(&mut self) {
        for (id, script) in &mut self.scripts {
            script.start(self.gameobjects.get_mut(id).unwrap());
        }
    }
    fn update_objects(&mut self, delta_time: f64) {
        for (id, script) in &mut self.scripts {
            script.update(self.gameobjects.get_mut(id).unwrap(), delta_time);
        }
    }
}
struct InputThreadStopper();
impl Drop for InputThreadStopper {
    fn drop(&mut self) {
        print!("\x1B[?1049l\x1B[?25h");
        INPUT
            .lock()
            .unwrap_or_else(|error| {
                LOG.lock()
                    .expect("Could not aquire mutex lock to write lock")
                    .logerr(&error.to_string().as_str());
                panic!("{}", error);
            })
            .stop();
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
