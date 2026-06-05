use crate::{
    ecs::{
        component_system::core_components::script_component::ScriptComponent,
        gameobject_builder::GameObjectBuilder,
    },
    fps_controller_doom::FpsControllerDoom,
    model::elements::{mesh::Mesh, pos3::Pos3},
    scene::Scene,
    screenspace::elements::cell_color::CellColor,
    spinner::Spinner,
};
pub mod ecs;
pub mod fps_controller_doom;
pub mod input;
pub mod io;
pub mod logger;
pub mod model;
pub mod scene;
pub mod screenspace;
pub mod spinner;

fn main() {
    let mut scene = Scene::with_dimensions(1000, 900);
    scene_setup(&mut scene);
    scene.run(200.0);
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
        .finish();
    scene.add_object(cube);
    let camera = GameObjectBuilder::new_object_with_name("camera")
        .add_body(Mesh::empty(), (0.0, 0.0, 0.0))
        .add_camera_component(Some(CellColor::BLACK))
        .add_script(
            ScriptComponent::new("fps_movement", FpsControllerDoom::new()),
            scene,
        )
        .finish();
    scene.add_object(camera);
    // let fish = GameObjectBuilder::new_object_with_name("fish")
    //     .add_body(
    //         Mesh::from_obj(
    //             "fish.obj",
    //             Some(CellColor::RED),
    //             &Pos3 {
    //                 x: 0.0,
    //                 y: 0.0,
    //                 z: 5.0,
    //             },
    //         )
    //         .unwrap(),
    //         (0.0, 0.0, 0.0),
    //     )
    //     .add_script(ScriptComponent::new("spinner", Spinner::new()), scene)
    //     .finish();
    // scene.add_object(fish);
}
