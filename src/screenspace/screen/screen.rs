use crate::{
    model::elements::pos3::Pos3,
    screenspace::elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
};
pub struct Screen {
    screen_vec: Vec<Vec<CellColor>>,
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
            screen_vec: vec![vec![CellColor::default(); *width]; *height],
            height: *height,
            width: *width,
        }
    }
    pub fn draw(&self) {
        for x in self.screen_vec.iter() {
            for y in x.iter() {
                Self::print_rgb_cell(&y);
            }
            print!("\n");
        }
    }
    pub fn color_cell(&mut self, pos: &ScreenPosition, color: &CellColor) {
        self.screen_vec[pos.y()][pos.x()] = *color;
    }
    fn print_rgb_cell(color: &CellColor) {
        let reset = "\x1b[0m";
        print!("{}{}{}", color.ansi_code(), "█", reset);
    }
    pub fn project_point(&self, value: &Pos3) -> ScreenPosition {
        let calc_x = (((value.x() / value.z()) + 1) / 2 * (self.width as isize));
        let calc_y = (((value.y() / value.z()) + 1) / 2 * (self.height as isize));
        ScreenPosition::with_pos(&(calc_x as usize), &(calc_y as usize))
    }
}
