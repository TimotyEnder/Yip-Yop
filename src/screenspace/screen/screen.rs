use std::collections::HashMap;

use crate::{
    model::elements::pos3::Pos3,
    screenspace::elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
};

pub struct Screen {
    changed_pixels: HashMap<ScreenPosition, CellColor>,
    height: usize,
    width: usize,
}

impl Screen {
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            changed_pixels: HashMap::new(),
            width,
            height,
        }
    }
    pub fn draw_and_flush(&mut self, buffer: &mut [u8]) {
        buffer.fill(0);
        for (pos, color) in self.changed_pixels.iter() {
            let idx = (pos.y * self.width + pos.x) * 4;
            buffer[idx] = color.r();
            buffer[idx + 1] = color.g();
            buffer[idx + 2] = color.b();
            buffer[idx + 3] = 255;
        }
        self.changed_pixels.clear();
    }
    pub fn color_cell(&mut self, pos: &ScreenPosition, color: &CellColor) {
        self.changed_pixels.insert(*pos, *color);
    }
    pub fn project_point(&self, value: &Pos3) -> ScreenPosition {
        let x = value.x / self.width as f64;
        let y = value.y / self.height as f64;
        let z = value.z / self.width as f64;

        if z == 0.0 {
            return ScreenPosition::with_pos(0, 0);
        }

        let calc_x = self.height as f64 - ((y / z) + 1.0) / 2.0 * (self.height as f64);
        let calc_y = ((x / z) + 1.0) / 2.0 * (self.width as f64);

        ScreenPosition::with_pos(calc_x as usize, calc_y as usize)
    }
}
