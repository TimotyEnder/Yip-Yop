use std::{arch::x86_64::_mm_shufflelo_epi16, collections::HashMap};

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
        self.height
    }
    pub fn get_height(&self) -> usize {
        self.width
    }
    pub fn with_dimensions(height: &usize, width: &usize) -> Self {
        Self {
            changed_pixels: HashMap::new(),
            width: *width,
            height: *height,
        }
    }
    pub fn draw_and_flush(&mut self) {
        let mut to_print = String::new();
        to_print.push_str("\x1B[2J\x1B[3J\x1B[1;1H");
        for x in 0..self.height {
            for y in 0..self.width {
                let current_pos = ScreenPosition::with_pos(&x, &y);
                to_print.push_str("\x1B#6");
                if (self.changed_pixels.contains_key(&current_pos)) {
                    to_print.push_str(&Self::print_rgb_cell(&self.changed_pixels[&current_pos]));
                } else {
                    to_print.push_str(&Self::print_rgb_cell(&CellColor::BLACK));
                }
            }
            to_print.push_str("\n");
        }
        print!("{}", to_print);
        io::stdout().flush().unwrap();
        self.changed_pixels.clear();
    }
    pub fn color_cell(&mut self, pos: &ScreenPosition, color: &CellColor) {
        self.changed_pixels.insert(*pos, *color);
    }
    fn print_rgb_cell(color: &CellColor) -> String {
        let reset = "\x1b[0m";
        format!("{}{}{}", color.ansi_code(), "█", reset)
    }
    pub fn project_point(&self, value: &Pos3) -> ScreenPosition {
        let x = value.x() / self.width as f64;
        let y = value.y() / self.height as f64;
        let z = value.z();

        if z == 0.0 {
            return ScreenPosition::with_pos(&0, &0);
        }

        let calc_x = self.height as f64 - ((y / z) + 1.0) / 2.0 * (self.height as f64);
        let calc_y = ((x / z) + 1.0) / 2.0 * (self.width as f64);

        ScreenPosition::with_pos(&(calc_x as usize), &(calc_y as usize))
    }
}
