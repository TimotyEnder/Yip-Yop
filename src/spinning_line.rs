use crate::{
    impl_gameobject, model::objects::line::Line, screenspace::elements::drawable::Drawable,
};

pub struct SpinningLine {
    line: Line,
}
impl SpinningLine {
    pub fn new(line: Line) -> Self {
        Self { line }
    }
}
impl GameObjectImpl for SpinningLine {
    fn on_start(&mut self) {}

    fn on_update(&mut self, delta_time: &f64) {}
}
impl Drawable for SpinningLine {
    fn draw(&self, screen: &mut crate::screenspace::screen::screen::Screen) {
        self.line.draw(screen);
    }

    fn position(&self) -> crate::model::elements::pos3::Pos3 {
        self.line.position()
    }
}
impl_gameobject!(SpinningLine);
