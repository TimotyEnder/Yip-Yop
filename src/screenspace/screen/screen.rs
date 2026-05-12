use std::{arch::x86_64::_mm_shufflelo_epi16, collections::HashMap};

use crate::{
    model::elements::pos3::Pos3,
    screenspace::elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
};
use std::io::{self, Write};
pub struct Screen {
    changed_pixels: HashMap<ScreenPosition, CellColor>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn with_dimensions(width: &usize, height: &usize) -> Self {
        Self {
            changed_pixels: HashMap::new(),
            height: *height,
            width: *width,
        }
    }
    pub fn draw_and_flush(&mut self) {
        let mut to_print = String::new();
        to_print.push_str("\x1B[2J\x1B[3J\x1B[1;1H");
        for x in 0..self.height {
            for y in 0..self.width {
                let currentPos = ScreenPosition::with_pos(&x, &y);
                if (self.changed_pixels.contains_key(&currentPos)) {
                    to_print.push_str(&Self::print_rgb_cell(&self.changed_pixels[&currentPos]));
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
        let calc_x = (((value.x() / value.z()) + 1) / 2 * (self.width as isize));
        let calc_y = (((value.y() / value.z()) + 1) / 2 * (self.height as isize));
        ScreenPosition::with_pos(&(calc_x as usize), &(calc_y as usize))
    }
}
