use crate::{model::elements::pos3::Pos3, screenspace::screen::screen::Screen};

pub trait Drawable {
    fn draw(&self, screen: &mut Screen);
    fn position(&self) -> Pos3;
}
