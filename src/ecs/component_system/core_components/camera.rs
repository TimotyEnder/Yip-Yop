use std::f64::consts::PI;

use crate::{impl_component, screenspace::elements::cell_color::CellColor};

pub struct Camera {
    bg_color: CellColor,
    fov_y: f64,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            bg_color: CellColor::BLACK,
            fov_y: PI / 2.0,
        }
    }
    pub fn set_bg_color(&mut self, col: CellColor) {
        self.bg_color = col;
    }
    pub fn set_fov_y(&mut self, fov_y: f64) {
        self.fov_y = fov_y;
    }
    pub fn get_fov_y(&self) -> f64 {
        self.fov_y
    }
}
impl_component!(Camera);
