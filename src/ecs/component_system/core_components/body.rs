use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::{
    impl_component,
    model::elements::{edge::Edge, mesh::Mesh, pos3::Pos3},
    screenspace::{
        elements::{cell_color::CellColor, screenspace_position::ScreenPosition},
        screen::screen::Screen,
    },
};

pub struct Body {
    mesh: Mesh,
    position: Pos3,
    rotation: (f64, f64, f64),
    mesh_rotation: (f64, f64, f64),
}
impl Body {
    pub fn with_mesh(mesh: Mesh, rotation: (f64, f64, f64)) -> Self {
        let pos = mesh.center;
        Body {
            mesh: mesh,
            position: pos,
            rotation: rotation,
            mesh_rotation: (0.0, 0.0, 0.0),
        }
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }
    pub fn rotate(&mut self, rotation: (f64, f64, f64)) {
        self.rotation.0 = (self.rotation.0 + rotation.0) % 360.0;
        self.rotation.1 = (self.rotation.1 + rotation.1) % 360.0;
        self.rotation.2 = (self.rotation.2 + rotation.2) % 360.0;
    }
    fn correct_position_rotation(&mut self) {
        let (angle_x, angle_y, angle_z) = self.rotation;
        let (mesh_angle_x, mesh_angle_y, mesh_angle_z) = self.mesh_rotation;
        let to_rotate = (
            (angle_x - mesh_angle_x).abs(),
            (angle_y - mesh_angle_y).abs(),
            (angle_z - mesh_angle_z).abs(),
        );
        let new_pos = self.position;
        self.mesh.translate(&new_pos);
        self.mesh.rotate(to_rotate.0, to_rotate.1, to_rotate.2);
        self.mesh_rotation = self.rotation;
    }
    pub fn draw(&mut self, screen: &mut Screen) {
        self.correct_position_rotation();
        for vertex in self.mesh.vertices.iter() {
            let to_draw = screen.project_point(vertex);
            screen.color_cell(&to_draw, &self.mesh.out_line_color);
        }
        for edge in self.mesh.edges.iter() {
            let from_screen = screen.project_point(&self.mesh.vertices[edge.0]);
            let to_screen = screen.project_point(&self.mesh.vertices[edge.1]);
            bresenham_line_algorithm(&from_screen, &to_screen, screen, &self.mesh.out_line_color);
        }
        self.mesh.faces.sort_by(|x, y| {
            let z_x = highest_z_from_point_list(vec![
                self.mesh.vertices[x.indices.0],
                self.mesh.vertices[x.indices.1],
                self.mesh.vertices[x.indices.2],
            ]);
            let z_y = highest_z_from_point_list(vec![
                self.mesh.vertices[y.indices.0],
                self.mesh.vertices[y.indices.1],
                self.mesh.vertices[y.indices.2],
            ]);
            z_y.partial_cmp(&z_x).unwrap_or(std::cmp::Ordering::Equal)
        });
        for face in self.mesh.faces.iter() {
            if let Some(color) = &face.color {
                let (one, two, three) = face.indices;
                fill_triangle(
                    &screen.project_point(&self.mesh.vertices[one]),
                    &screen.project_point(&self.mesh.vertices[two]),
                    &screen.project_point(&self.mesh.vertices[three]),
                    color,
                    screen,
                );
                let arr = [one, two, three];
                for i in arr {
                    for j in arr {
                        if j != i
                            && (self.mesh.edges.contains(&Edge(i, j))
                                || self.mesh.edges.contains(&Edge(i, j)))
                        {
                            bresenham_line_algorithm(
                                &screen.project_point(&self.mesh.vertices[i]),
                                &screen.project_point(&self.mesh.vertices[j]),
                                screen,
                                &self.mesh.out_line_color,
                            );
                        }
                    }
                }
            }
        }
    }
}
fn highest_z_from_point_list(positions: Vec<Pos3>) -> f64 {
    let mut max_pos: f64 = f64::NEG_INFINITY;
    for pos in positions.iter() {
        max_pos = max(pos.z as isize, max_pos as isize) as f64;
    }
    max_pos
}
fn bresenham_line_algorithm(
    from: &ScreenPosition,
    to: &ScreenPosition,
    screen: &mut Screen,
    color: &CellColor,
) {
    let dx = (to.x as isize - from.x as isize).abs(); //total x distance
    let dy = (to.y as isize - from.y as isize).abs(); //total y distance
    let sx = if to.x >= from.x { 1 } else { -1 }; //step for x
    let sy = if to.y >= from.y { 1 } else { -1 }; //step for y
    let mut err = dx - dy; //deviation from mathematical line and actual pixel position, decides next movement
    let mut x = from.x as isize;
    let mut y = from.y as isize;
    loop {
        let to_color = ScreenPosition::with_pos(x as usize, y as usize);
        screen.color_cell(&to_color, color);

        if x == to.x as isize && y == to.y as isize {
            break;
        }
        //if 2*err > -dy, then take x step
        //if 2*err < dx, then take y step
        let e2 = 2 * err; //avoids fractions and enables the use of integer maths
        if e2 > -dy {
            //above the y of the current line so go forward
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            //in front of the x  so go up to be on the same height
            err += dx;
            y += sy;
        }
    }
}
fn fill_triangle(
    one: &ScreenPosition,
    two: &ScreenPosition,
    three: &ScreenPosition,
    fill_color: &CellColor,
    screen: &mut Screen,
) -> HashSet<ScreenPosition> {
    let mut colored_cells = HashSet::new();
    let min_x = min(min(one.x, two.x), three.x);
    let min_y = min(min(one.y, two.y), three.y);
    let max_x = max(max(one.x, two.x), three.x);
    let max_y = max(max(one.y, two.y), three.y);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let cur_pos = ScreenPosition::with_pos(x, y);
            if point_inside_triangle(one, two, three, &cur_pos) {
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
    let denominator: f64 = (p2.y as f64 - p3.y as f64) * (p1.x as f64 - p3.x as f64)
        + (p3.x as f64 - p2.x as f64) * (p1.y as f64 - p3.y as f64);
    if denominator == 0.0 {
        return false;
    }
    let a = ((p2.y as f64 - p3.y as f64) * (point.x as f64 - p3.x as f64)
        + (p3.x as f64 - p2.x as f64) * (point.y as f64 - p3.y as f64))
        / denominator;
    let b = ((p3.y as f64 - p1.y as f64) * (point.x as f64 - p3.x as f64)
        + (p1.x as f64 - p3.x as f64) * (point.y as f64 - p3.y as f64))
        / denominator;
    let c = 1.0 - a - b;
    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}
impl_component!(Body);
