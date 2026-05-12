use std::{io::Cursor, vec};

use crate::{
    model::{
        elements::pos3::Pos3,
        objects::{dot::Dot, line::Line},
    },
    screenspace::elements::{cell_color::CellColor, drawable::Drawable},
};

pub struct Cube {
    center: Pos3,
    corners: Vec<Pos3>,
    outline_color: CellColor,
    fill_color: CellColor,
}
impl Cube {
    pub fn from_center(center: &Pos3, scale: usize) -> Self {
        Self {
            center: *center,
            corners: vec![
                Pos3::new(
                    &(center.x() + scale as isize),
                    &(center.y() + scale as isize),
                    &(center.z() + scale as isize),
                ),
                Pos3::new(
                    &(center.x() + scale as isize),
                    &(center.y() + scale as isize),
                    &(center.z() - scale as isize),
                ),
                Pos3::new(
                    &(center.x() + scale as isize),
                    &(center.y() - scale as isize),
                    &(center.z() + scale as isize),
                ),
                Pos3::new(
                    &(center.x() + scale as isize),
                    &(center.y() - scale as isize),
                    &(center.z() - scale as isize),
                ),
                Pos3::new(
                    &(center.x() - scale as isize),
                    &(center.y() + scale as isize),
                    &(center.z() + scale as isize),
                ),
                Pos3::new(
                    &(center.x() - scale as isize),
                    &(center.y() + scale as isize),
                    &(center.z() - scale as isize),
                ),
                Pos3::new(
                    &(center.x() - scale as isize),
                    &(center.y() - scale as isize),
                    &(center.z() + scale as isize),
                ),
                Pos3::new(
                    &(center.x() - scale as isize),
                    &(center.y() - scale as isize),
                    &(center.z() - scale as isize),
                ),
            ],
            outline_color: CellColor::WHITE,
            fill_color: CellColor::BLACK,
        }
    }
}
impl Drawable for Cube {
    fn draw(&self, screen: &mut crate::screenspace::screen::screen::Screen) {
        let edges = [
            (0, 1),
            (0, 2),
            (0, 4),
            (3, 1),
            (3, 2),
            (3, 7),
            (5, 1),
            (5, 4),
            (5, 7),
            (6, 2),
            (6, 4),
            (6, 7),
        ];

        for &(from, to) in &edges {
            let line = Line::from_to(&self.corners[from], &self.corners[to]);
            line.draw(screen);
        }
    }

    fn position(&self) -> Pos3 {
        self.center
    }
}
