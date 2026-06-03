use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::WindowAttributes;

use crate::ecs::component_system::core_components::body::Body;
use crate::ecs::component_system::core_components::script_component::ScriptComponent;
use crate::input::input_thread::INPUT;
use crate::screenspace::screen::screen::Screen;

impl ApplicationHandler for Scene {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Yip-Yop"))
                .unwrap(),
        );
        let width = self.screen.get_width() as u32;
        let height = self.screen.get_height() as u32;
        let surface_texture = SurfaceTexture::new(width, height, window.clone());
        let pixels = Pixels::new(width, height, surface_texture).unwrap();
        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let frame_duration = Duration::from_secs_f32(1.0 / self.fps);
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(ref mut pixels) = self.pixels {
                    let _ = pixels.resize_surface(size.width.max(1), size.height.max(1));
                }
            }
            WindowEvent::RedrawRequested => {
                let frame_time = self.frame_timer.elapsed();
                if frame_time < frame_duration {
                    std::thread::sleep(frame_duration - frame_time);
                }
                self.frame_timer = Instant::now();
                let now = Instant::now();
                let delta_time = now.duration_since(self.last_frame_time);
                self.last_frame_time = now;

                self.update_objects(delta_time.as_secs_f64());
                self.draw_objects();

                if let Some(ref mut pixels) = self.pixels {
                    let frame = pixels.frame_mut();
                    self.screen.draw_and_flush(frame);
                    let _ = pixels.render();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    let mut input = INPUT.lock().unwrap();
                    match event.state {
                        ElementState::Pressed => input.press(keycode),
                        ElementState::Released => input.release(keycode),
                    }
                }
            }
            _ => {}
        }
    }
}
pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<usize, crate::ecs::gameobject::GameObject>,
    scripts: HashMap<usize, ScriptComponent>,
    window: Option<Arc<winit::window::Window>>,
    pixels: Option<Pixels<'static>>,
    frame_timer: Instant,
    last_frame_time: Instant,
    fps: f32,
}
impl Scene {
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
    }
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            screen: Screen::with_dimensions(height, width),
            gameobjects: HashMap::new(),
            scripts: HashMap::new(),
            window: Option::None,
            pixels: Option::None,
            frame_timer: Instant::now(),
            last_frame_time: Instant::now(),
            fps: 60.0,
        }
    }
    pub fn add_object(&mut self, object: crate::ecs::gameobject::GameObject) {
        self.gameobjects.insert(object.get_id(), object);
    }
    pub fn add_script(&mut self, script: ScriptComponent, gameobject_id: usize) {
        self.scripts.insert(gameobject_id, script);
    }
    pub fn run(mut self, fps: f32) {
        self.start_objects();
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        self.set_fps(fps);
        event_loop.run_app(&mut self).unwrap();
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
