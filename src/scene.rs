use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::WindowAttributes;

use crate::ecs::component_system::core_components::body::Body;
use crate::ecs::component_system::core_components::script_component::ScriptComponent;
use crate::input::input_thread::INPUT;
use crate::screenspace::screen::screen::Screen;

pub struct Scene {
    screen: Screen,
    gameobjects: HashMap<usize, crate::ecs::gameobject::GameObject>,
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
    pub fn add_object(&mut self, object: crate::ecs::gameobject::GameObject) {
        self.gameobjects.insert(object.get_id(), object);
    }
    pub fn add_script(&mut self, script: ScriptComponent, gameobject_id: usize) {
        self.scripts.insert(gameobject_id, script);
    }
    pub fn run(mut self, fps: f32) {
        self.start_objects();

        let event_loop = EventLoop::new().unwrap();
        let frame_duration = Duration::from_secs_f32(1.0 / fps);
        let pixel_width = self.screen.get_width() as u32;
        let pixel_height = self.screen.get_height() as u32;

        let mut window: Option<Arc<winit::window::Window>> = None;
        let mut pixels: Option<Pixels<'static>> = None;
        let mut frame_timer = Instant::now();
        let mut last_frame_time = Instant::now();

        event_loop
            .run(move |event, active_event_loop| {
                active_event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::Resumed => {
                        let w = Arc::new(
                            active_event_loop
                                .create_window(WindowAttributes::default().with_title("Yip-Yop"))
                                .unwrap(),
                        );
                        let window_size = w.inner_size();
                        let surface_texture = SurfaceTexture::new(
                            window_size.width.max(1),
                            window_size.height.max(1),
                            w.clone(),
                        );
                        pixels =
                            Some(Pixels::new(pixel_width, pixel_height, surface_texture).unwrap());
                        window = Some(w);
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            active_event_loop.exit();
                        }
                        WindowEvent::Resized(size) => {
                            if let Some(ref mut pixels) = pixels {
                                let _ =
                                    pixels.resize_surface(size.width.max(1), size.height.max(1));
                            }
                        }
                        WindowEvent::RedrawRequested => {
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

                            if let Some(ref mut pixels) = pixels {
                                let frame = pixels.frame_mut();
                                self.screen.draw_and_flush(frame);
                                let _ = pixels.render();
                            }
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
                    },
                    Event::AboutToWait => {
                        if let Some(ref window) = window {
                            window.request_redraw();
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
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
