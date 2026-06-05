use winit::keyboard::KeyCode;

use crate::{
    ecs::component_system::core_components::{body::Body, script_component::ScriptBehavior},
    input::input_thread::INPUT,
};

pub struct FpsControllerDoom {}
impl FpsControllerDoom {
    pub fn new() -> Self {
        FpsControllerDoom {}
    }
}
impl ScriptBehavior for FpsControllerDoom {
    fn start(&mut self, gameobject: &mut crate::ecs::gameobject::GameObject) {}

    fn update(&mut self, gameobject: &mut crate::ecs::gameobject::GameObject, delta_time: f64) {
        let rotation_speed = 180.0_f64.to_radians();
        let rotation_speed: f64 = rotation_speed * delta_time;
        if let Some(body) = gameobject.get_component_mut::<Body>() {
            if INPUT.lock().unwrap().is_down(KeyCode::ArrowRight) {
                body.rotate((0.0, rotation_speed, 0.0));
            }
            if INPUT.lock().unwrap().is_down(KeyCode::ArrowLeft) {
                body.rotate((0.0, -rotation_speed, 0.0));
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyW) {
                body.translate_based_on_direction(1.0, 0.0, 0.0);
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyS) {
                body.translate_based_on_direction(-1.0, 0.0, 0.0);
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyA) {
                body.translate_based_on_direction(0.0, 1.0, 0.0);
            }
            if INPUT.lock().unwrap().is_down(KeyCode::KeyD) {
                body.translate_based_on_direction(1.0, -1.0, 0.0);
            }
        }
    }
}
