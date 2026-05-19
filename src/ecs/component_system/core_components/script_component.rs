use crate::{
    ecs::gameobject::{self, GameObject},
    impl_component,
    scene::Scene,
};

pub struct ScriptComponent {
    pub name: String,
    pub enabled: bool,
    behavior: Box<dyn ScriptBehavior>,
}
impl ScriptComponent {
    pub fn new<T: ScriptBehavior + 'static>(name: &str, behavior: T) -> Self {
        Self {
            name: name.to_string(),
            enabled: true,
            behavior: Box::new(behavior),
        }
    }

    pub fn start(&mut self, gameobject: &mut GameObject) {
        if self.enabled {
            self.behavior.start(gameobject);
        }
    }

    pub fn update(&mut self, gameobject: &mut GameObject, delta_time: &f64) {
        if self.enabled {
            self.behavior.update(gameobject, delta_time);
        }
    }
    pub fn set_enabled(&mut self, set: bool) {
        self.enabled = set;
    }
}
pub trait ScriptBehavior: Send + Sync {
    fn start(&mut self, gameobject: &mut GameObject);
    fn update(&mut self, gameobject: &mut GameObject, delta_time: &f64);
}
