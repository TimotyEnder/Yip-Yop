use crate::{
    ecs::{
        component_system::core_components::{body::Body, script_component::ScriptComponent},
        gameobject::GameObject,
    },
    model::elements::mesh::Mesh,
    scene::Scene,
};

pub struct GameObjectBuilder {
    gameobject: GameObject,
}
impl GameObjectBuilder {
    pub fn new_object_with_name(name: &str) -> Self {
        GameObjectBuilder {
            gameobject: GameObject::new(name),
        }
    }
    pub fn add_body(mut self, mesh: Mesh, rotation: (f64, f64, f64)) -> Self {
        self.gameobject
            .add_component(Body::with_mesh(mesh, rotation));
        self
    }
    pub fn add_script(self, script_component: ScriptComponent, scene: &mut Scene) -> Self {
        scene.add_script(script_component, &self.gameobject.get_id());
        self
    }
    pub fn finish(self) -> GameObject {
        self.gameobject
    }
}
