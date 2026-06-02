use std::collections::HashMap;

use minigw::{RcCell, RenderTexture, render_texture};

use crate::{
    model::elements::pos3::Pos3,
    screenspace::elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
};
use std::io::{self, Write};
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
    pub fn draw_and_flush(&mut self, render_texture: RcCell<RenderTexture<u8>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_pos = ScreenPosition::with_pos(y, x);
                if let Some(target_color) = self.changed_pixels.get(&current_pos) {
                    render_texture.as_mut().set_pixel(
                        x as u32,
                        y as u32,
                        target_color.r(),
                        target_color.g(),
                        target_color.b(),
                    );
                } else {
                    let target_color = CellColor::BLACK;
                    render_texture.as_mut().set_pixel(
                        x as u32,
                        y as u32,
                        target_color.r(),
                        target_color.g(),
                        target_color.b(),
                    );
                }
            }
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
