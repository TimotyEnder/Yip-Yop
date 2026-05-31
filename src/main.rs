use crate::{
    ecs::{
        component_system::core_components::script_component::ScriptComponent,
        gameobject_builder::GameObjectBuilder,
    },
    model::elements::{mesh::Mesh, pos3::Pos3},
    scene::Scene,
    screenspace::elements::cell_color::CellColor,
    spinner::Spinner,
};
pub mod ecs;
pub mod input;
pub mod io;
pub mod model;
pub mod scene;
pub mod screenspace;
pub mod spinner;
#[tokio::main]
async fn main() {
    let mut scene = Scene::with_dimensions(60, 200);
    scene_setup(&mut scene);
    scene.run(60).await;
}
fn scene_setup(scene: &mut Scene) {
    let cube = GameObjectBuilder::new_object_with_name("cube")
        .add_body(
            Mesh::cube(
                &Pos3::new(0.0, 0.0, 17.0),
                5.0,
                5.0,
                5.0,
                Some(CellColor::RED),
            ),
            (0.0, 0.0, 0.0),
        )
        .add_script(ScriptComponent::new("spinner", Spinner::new()), scene)
        .finish();
    scene.add_object(cube);
}
