use crate::{
    model::{elements::pos3::Pos3, objects::dot::Dot},
    scene::Scene,
    screenspace::screen::screen::Screen,
};

pub mod model;
pub mod scene;
pub mod screenspace;
fn main() {
    let mut scene = Scene::with_dimensions(&(200 as usize), &(200 as usize));
    scene.add_object(Box::new(Dot::at_pos(&Pos3::new(&0, &0, &1))));
}
