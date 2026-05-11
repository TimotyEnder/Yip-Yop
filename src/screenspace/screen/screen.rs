use std::collections::HashMap;

use crate::screenspace::elements::{
    cell_color::CellColor, screenspace_object::ScreenspaceObject, screenspace_position::ScreenPos,
};
pub struct Screen {
    screen_vec: Vec<Vec<CellColor>>,
}

impl Screen {
    pub fn with_dimensions(width: &usize, height: &usize) -> Self {
        Self {
            screen_vec: vec![vec![CellColor::default(); *width]; *height],
        }
    }
    pub fn draw(&self) -> String {
        todo!("clear, call draw on all of the objects then display");
    }
}
