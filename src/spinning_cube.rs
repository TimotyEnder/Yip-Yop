use crate::{
    impl_gameobject, model::objects::cube::Cube, screenspace::elements::drawable::Drawable,
};

pub struct SpinningCube {
    cube: Cube,
}
impl SpinningCube {
    pub fn new(cube: Cube) -> Self {
        Self { cube: cube }
    }
}
impl Drawable for SpinningCube {
    fn draw(&self, screen: &mut crate::screenspace::screen::screen::Screen) {
        self.cube.draw(screen);
    }

    fn position(&self) -> crate::model::elements::pos3::Pos3 {
        self.cube.position()
    }
}
impl GameObjectImpl for SpinningCube {
    fn on_start(&mut self) {}

    fn on_update(&mut self, delta_time: f32) {
        self.cube.rotate(0.0, 1.0, 0.0);
    }
}
impl_gameobject! {SpinningCube}
