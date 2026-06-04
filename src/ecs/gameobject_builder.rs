use crate::{
    ecs::{
        component_system::core_components::{
            body::Body, camera::Camera, script_component::ScriptComponent,
        },
        gameobject::GameObject,
    },
    model::elements::mesh::Mesh,
    scene::Scene,
    screenspace::elements::cell_color::CellColor,
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
        scene.add_script(script_component, self.gameobject.get_id());
        self
    }
    pub fn add_camera_component(mut self, color_or_none_black: Option<CellColor>) -> Self {
        let mut camera = Camera::new();
        if let Some(color) = color_or_none_black {
            camera.set_bg_color(color);
        }
        self.gameobject.add_component(camera);
        self
    }
    pub fn finish(self) -> GameObject {
        self.gameobject
    }
}
