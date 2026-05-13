use crate::{
    model::{
        elements::pos3::Pos3,
        objects::{cube::Cube, dot::Dot, line::Line},
    },
    scene::Scene,
    spinning_cube::SpinningCube,
    spinning_line::SpinningLine,
};
pub mod gameobject;
pub mod model;
pub mod scene;
pub mod screenspace;
pub mod spinning_cube;
pub mod spinning_line;
fn main() {
    let mut scene = Scene::with_dimensions(&(40 as usize), &(100 as usize));
    // scene.add_object(Box::new(Line::from_to(
    //     &Pos3::new(&-10.0, &10.0, &30.0),
    //     &Pos3::new(&10.0, &-10.0, &30.0),
    // )));
    // scene.add_object(Box::new(Cube::from_center(
    //     &Pos3::new(&0.0, &0.0, &20.0),
    //     5 as usize,
    // )));
    scene.add_object(Box::new(SpinningCube::new(Cube::from_center(
        &Pos3::new(&0.0, &0.0, &20.0),
        4 as usize,
    ))));
    // scene.add_object(Box::new(SpinningLine::new(Line::from_to(
    //     &Pos3::new(&-10.0, &-5.0, &100.0),
    //     &Pos3::new(&10.0, &5.0, &100.0),
    // ))));
    scene.run(&15);
}
