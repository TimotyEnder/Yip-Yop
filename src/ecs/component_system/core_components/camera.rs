use crate::{impl_component, screenspace::elements::cell_color::CellColor};

pub struct Camera {
    bg_color: CellColor,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            bg_color: CellColor::BLACK,
        }
    }
    pub fn set_bg_color(&mut self, col: CellColor) {
        self.bg_color = col;
    }
}
impl_component!(Camera);
