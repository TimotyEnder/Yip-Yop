use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::{
    model::{
        elements::pos3::Pos3,
        objects::{dot::Dot, line::Line},
    },
    screenspace::{
        elements::{
            cell_color::CellColor, drawable::Drawable, screenspace_position::ScreenPosition,
        },
        screen::screen::Screen,
    },
    utils,
};

pub struct Cube {
    center: Pos3,
    corners: Vec<Pos3>,
    outline_color: CellColor,
    fill_color: Option<CellColor>,
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
            fill_color: None,
        }
    }

    pub fn from_center_with_outline_color(
        center: &Pos3,
        scale: usize,
        outline_color: &CellColor,
    ) -> Self {
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
            outline_color: *outline_color,
            fill_color: None,
        }
    }
    pub fn from_center_filled(
        center: &Pos3,
        scale: usize,
        outline_color: &CellColor,
        fill_color: &CellColor,
    ) -> Self {
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
            outline_color: *outline_color,
            fill_color: Some(*fill_color),
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
    fn fill_triangle(
        one: &ScreenPosition,
        two: &ScreenPosition,
        three: &ScreenPosition,
        fill_color: &CellColor,
        screen: &mut Screen,
    ) -> HashSet<ScreenPosition> {
        let mut colored_cells = HashSet::new();
        let min_x = min(min(one.x(), two.x()), three.x());
        let min_y = min(min(one.y(), two.y()), three.y());
        let max_x = max(max(one.x(), two.x()), three.x());
        let max_y = max(max(one.y(), two.y()), three.y());
        for x in min_x..max_x {
            for y in min_y..max_y {
                let cur_pos = ScreenPosition::with_pos(&x, &y);
                if Self::point_inside_triangle(one, two, three, &cur_pos) {
                    screen.color_cell(&cur_pos, fill_color);
                    colored_cells.insert(cur_pos);
                }
            }
        }
        colored_cells
    }
    fn point_inside_triangle(
        p1: &ScreenPosition,
        p2: &ScreenPosition,
        p3: &ScreenPosition,
        point: &ScreenPosition,
    ) -> bool {
        let denominator: f64 = (p2.y() as f64 - p3.y() as f64) * (p1.x() as f64 - p3.x() as f64)
            + (p3.x() as f64 - p2.x() as f64) * (p1.y() as f64 - p3.y() as f64);
        if denominator == 0.0 {
            return false;
        }
        let a = ((p2.y() as f64 - p3.y() as f64) * (point.x() as f64 - p3.x() as f64)
            + (p3.x() as f64 - p2.x() as f64) * (point.y() as f64 - p3.y() as f64))
            / denominator;
        let b = ((p3.y() as f64 - p1.y() as f64) * (point.x() as f64 - p3.x() as f64)
            + (p1.x() as f64 - p3.x() as f64) * (point.y() as f64 - p3.y() as f64))
            / denominator;
        let c = 1.0 - a - b;
        a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
    }
}
impl Drawable for Cube {
    fn draw(
        &self,
        screen: &mut crate::screenspace::screen::screen::Screen,
    ) -> HashSet<ScreenPosition> {
        let mut colored_cells = HashSet::new();
        let mut faces = [
            // Back face (z = cz+s)
            (4, 6, 2, 0), // top-left, top-right, bottom-right, bottom-left
            // Front face (z = cz-s)
            (5, 7, 3, 1), // top-left, top-right, bottom-right, bottom-left
            // Left face (connecting 4-0 and 5-1)
            (4, 0, 1, 5), // back-top-left, back-bottom-left, front-bottom-left, front-top-left
            // Right face (connecting 6-2 and 7-3)
            (6, 2, 3, 7), // back-top-right, back-bottom-right, front-bottom-right, front-top-right
            // Top face
            (4, 6, 7, 5), // back-top-left, back-top-right, front-top-right, front-top-left
            // Bottom face
            (0, 2, 3, 1), // back-bottom-left, back-bottom-right, front-bottom-right, front-bottom-left
        ];
        faces.sort_by(|x, y| {
            let z_x = utils::highest_z_from_point_list(vec![
                self.corners[x.0],
                self.corners[x.1],
                self.corners[x.2],
                self.corners[x.3],
            ]);
            let z_y = utils::highest_z_from_point_list(vec![
                self.corners[y.0],
                self.corners[y.1],
                self.corners[y.2],
                self.corners[y.3],
            ]);
            z_y.partial_cmp(&z_x).unwrap_or(std::cmp::Ordering::Equal)
        });
        for &(top_left, top_right, bottom_right, bottom_left) in &faces {
            if let Some(color) = self.fill_color {
                // Triangle 1: top half
                for cell in Self::fill_triangle(
                    &screen.project_point(&self.corners[top_left]),
                    &screen.project_point(&self.corners[top_right]),
                    &screen.project_point(&self.corners[bottom_right]),
                    &color,
                    screen,
                ) {
                    colored_cells.insert(cell);
                }

                // Triangle 2: bottom half (use the OTHER diagonal)
                for cell in Self::fill_triangle(
                    &screen.project_point(&self.corners[top_left]),
                    &screen.project_point(&self.corners[bottom_right]),
                    &screen.project_point(&self.corners[bottom_left]),
                    &color,
                    screen,
                ) {
                    colored_cells.insert(cell);
                }
            }
            //edges
            let line = Line::from_to(&self.corners[top_left], &self.corners[top_right]);
            for cell in line.draw(screen) {
                colored_cells.insert(cell);
            }
            let line = Line::from_to(&self.corners[top_right], &self.corners[bottom_right]);
            for cell in line.draw(screen) {
                colored_cells.insert(cell);
            }
            let line = Line::from_to(&self.corners[bottom_right], &self.corners[bottom_left]);
            for cell in line.draw(screen) {
                colored_cells.insert(cell);
            }
            let line = Line::from_to(&self.corners[bottom_left], &self.corners[top_left]);
            for cell in line.draw(screen) {
                colored_cells.insert(cell);
            }
        }

        colored_cells
    }

    fn position(&self) -> Pos3 {
        self.center
    }
}
