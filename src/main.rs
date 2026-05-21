use crate::{
    ecs::{
        component_system::core_components::script_component::ScriptComponent,
        gameobject_builder::GameObjectBuilder,
    },
    model::elements::{mesh::Mesh, pos3::Pos3},
    scene::Scene,
    screenspace::elements::cell_color::CellColor,
    spinning_cube::SpinningCube,
};
pub mod ecs;
pub mod io;
pub mod model;
pub mod scene;
pub mod screenspace;
pub mod spinning_cube;
fn main() {
    let mut scene = Scene::with_dimensions(60, 200);
    // let cube = GameObjectBuilder::new_object_with_name("cube")
    //     .add_body(
    //         Mesh::cube(
    //             &Pos3::new(0.0, 0.0, 20.0),
    //             5.0,
    //             5.0,
    //             5.0,
    //             Some(CellColor::RED),
    //         ),
    //         (0.0, 0.0, 0.0),
    //     )
    //     .add_script(
    //         ScriptComponent::new("spinner", SpinningCube::new()),
    //         &mut scene,
    //     )
    //     .finish();
    let cube = GameObjectBuilder::new_object_with_name("cube")
        .add_body(
            Mesh::from_obj("horse.obj", Some(CellColor::RED)).expect("could not parse obj file"),
            (0.0, 0.0, 0.0),
        )
        .add_script(
            ScriptComponent::new("spinner", SpinningCube::new()),
            &mut scene,
        )
        .finish();
    scene.add_object(cube);
    scene.run(60);
}
