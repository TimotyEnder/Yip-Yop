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

    fn on_update(&mut self, delta_time: &f64) {
        let rotation_speed = 90.0_f64.to_radians(); // 90 degrees per second
        let rotation_amount: f64 = rotation_speed * *delta_time;

        self.cube.rotate(&0.0, &rotation_amount, &0.0);
    }
}
impl_gameobject! {SpinningCube}
