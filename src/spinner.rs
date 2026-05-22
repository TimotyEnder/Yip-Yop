use crate::ecs::component_system::core_components::{body::Body, script_component::ScriptBehavior};

pub struct Spinner {}
impl Spinner {
    pub fn new() -> Self {
        Self {}
    }
}
impl ScriptBehavior for Spinner {
    fn start(&mut self, gameobject: &mut crate::ecs::gameobject::GameObject) {}

    fn update(&mut self, gameobject: &mut crate::ecs::gameobject::GameObject, delta_time: f64) {
        let rotation_speed = 90.0_f64.to_radians(); // 90 degrees per second
        let rotation_amount: f64 = rotation_speed * delta_time;

        if let Some(body) = gameobject.get_component_mut::<Body>() {
            body.rotate((0.0, rotation_amount, 0.0));
        }
    }
}
