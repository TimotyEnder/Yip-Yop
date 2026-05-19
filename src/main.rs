use crate::{
    ecs::gameobject_builder::GameObjectBuilder,
    model::elements::{mesh::Mesh, pos3::Pos3},
    scene::Scene,
};
pub mod ecs;
pub mod model;
pub mod scene;
pub mod screenspace;
fn main() {
    let mut scene = Scene::with_dimensions(&(60 as usize), &(200 as usize));
    let cube = GameObjectBuilder::new_object_with_name("cube")
        .add_body(
            Mesh::cube(&Pos3::new(&0.0, &0.0, &10.0), &10.0, &10.0, &10.0),
            (0.0, 0.0, 0.0),
        )
        .finish();
    scene.add_object(cube);
    scene.run(&60);
}
