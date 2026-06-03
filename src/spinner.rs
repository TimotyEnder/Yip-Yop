use winit::keyboard::KeyCode;

use crate::{
    ecs::component_system::core_components::{body::Body, script_component::ScriptBehavior},
    input::input_thread::INPUT,
};

pub struct Spinner {}
impl Spinner {
    pub fn new() -> Self {
        Self {}
    }
}
impl ScriptBehavior for Spinner {
    fn start(&mut self, _gameobject: &mut crate::ecs::gameobject::GameObject) {}

    fn update(&mut self, gameobject: &mut crate::ecs::gameobject::GameObject, delta_time: f64) {
        let rotation_speed = 180.0_f64.to_radians();
        let rotation_amount: f64 = rotation_speed * delta_time;

        if let Some(body) = gameobject.get_component_mut::<Body>() {
            if INPUT.lock().unwrap().is_down(KeyCode::KeyW) {
                body.rotate((0.0, rotation_amount, 0.0));
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyS) {
                body.rotate((0.0, -rotation_amount, 0.0));
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyA) {
                body.rotate((rotation_amount, 0.0, 0.0));
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyD) {
                body.rotate((-rotation_amount, 0.0, 0.0));
            }
        }
    }
}
