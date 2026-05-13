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
        let s = scale as f64;
        Self {
            center: *center,
            corners: vec![
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() + s), &(center.y() - s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() + s), &(center.z() - s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() + s)),
                Pos3::new(&(center.x() - s), &(center.y() - s), &(center.z() - s)),
            ],
            outline_color: CellColor::WHITE,
            fill_color: CellColor::BLACK,
        }
    }
    pub fn rotate(&mut self, angle_x: &f64, angle_y: &f64, angle_z: &f64) {
        for corner in self.corners.iter_mut() {
            Self::transform_into_center_vector_and_rotate(
                angle_x,
                angle_y,
                angle_z,
                &self.center,
                corner,
            );
        }
    }
    fn transform_into_center_vector_and_rotate(
        angle_x: &f64,
        angle_y: &f64,
        angle_z: &f64,
        center: &Pos3,
        corner: &mut Pos3,
    ) {
        let x = corner.x() - center.x();
        let y = corner.y() - center.y();
        let z = corner.z() - center.z();
        *corner = Pos3::new(&x, &y, &z);
        corner.rotate(angle_x, angle_y, angle_z);
        let x = corner.x() + center.x();
        let y = corner.y() + center.y();
        let z = corner.z() + center.z();
        *corner = Pos3::new(&x, &y, &z);
    }
}
impl Drawable for Cube {
    fn draw(&self, screen: &mut crate::screenspace::screen::screen::Screen) {
        let edges = [
            // back face (z = cz+s): 0--2, 2--6, 6--4, 4--0
            (0, 2),
            (2, 6),
            (6, 4),
            (4, 0),
            // front face (z = cz-s): 1--3, 3--7, 7--5, 5--1
            (1, 3),
            (3, 7),
            (7, 5),
            (5, 1),
            // connectors between front and back
            (0, 1),
            (2, 3),
            (6, 7),
            (4, 5),
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
