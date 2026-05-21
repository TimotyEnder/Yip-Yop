use std::{cell::Cell, error::Error, io::SeekFrom, vec};

use crate::{
    io::parsing::obj_parser::parse_obj_into_mesh,
    model::elements::{edge::Edge, face::Face, pos3::Pos3},
    screenspace::elements::cell_color::CellColor,
};

pub struct Mesh {
    pub vertices: Vec<Pos3>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub center: Pos3,
    pub out_line_color: CellColor,
}
impl Mesh {
    pub fn empty() -> Self {
        Mesh {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            center: Pos3::default(),
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn from_obj(filename: &str, color: Option<CellColor>) -> Result<Mesh, Box<dyn Error>> {
        parse_obj_into_mesh(filename, color)
    }
    pub fn dot(pos: &Pos3) -> Self {
        Mesh {
            vertices: Vec::from(vec![*pos]),
            edges: Vec::new(),
            faces: Vec::new(),
            center: *pos,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn line(from: &Pos3, to: &Pos3) -> Self {
        Mesh {
            vertices: vec![*from, *to],
            edges: vec![Edge(0, 1)],
            faces: Vec::new(),
            center: mid_point_in_line(from, to),
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn cube(
        center: &Pos3,
        x_size: f64,
        y_size: f64,
        z_size: f64,
        fill_color: Option<CellColor>,
    ) -> Self {
        let half_x = x_size / 2.0;
        let half_y = y_size / 2.0;
        let half_z = z_size / 2.0;

        Mesh {
            vertices: vec![
                // 0: Front-top-right (x+, y+, z+)
                Pos3::new(center.x + half_x, center.y + half_y, center.z + half_z),
                // 1: Back-top-right (x+, y+, z-)
                Pos3::new(center.x + half_x, center.y + half_y, center.z - half_z),
                // 2: Front-bottom-right (x+, y-, z+)
                Pos3::new(center.x + half_x, center.y - half_y, center.z + half_z),
                // 3: Back-bottom-right (x+, y-, z-)
                Pos3::new(center.x + half_x, center.y - half_y, center.z - half_z),
                // 4: Front-top-left (x-, y+, z+)
                Pos3::new(center.x - half_x, center.y + half_y, center.z + half_z),
                // 5: Back-top-left (x-, y+, z-)
                Pos3::new(center.x - half_x, center.y + half_y, center.z - half_z),
                // 6: Front-bottom-left (x-, y-, z+)
                Pos3::new(center.x - half_x, center.y - half_y, center.z + half_z),
                // 7: Back-bottom-left (x-, y-, z-)
                Pos3::new(center.x - half_x, center.y - half_y, center.z - half_z),
            ],
            edges: vec![
                Edge(4, 6),
                Edge(6, 2),
                Edge(2, 0),
                Edge(0, 4), // Front face (z+)
                Edge(5, 7),
                Edge(7, 3),
                Edge(3, 1),
                Edge(1, 5), // Back face (z-)
                Edge(4, 0),
                Edge(0, 1),
                Edge(1, 5),
                Edge(5, 4), // Top face (y+)
                Edge(6, 2),
                Edge(2, 3),
                Edge(3, 7),
                Edge(7, 6), // Bottom face (y-)
                Edge(4, 6),
                Edge(6, 7),
                Edge(7, 5),
                Edge(5, 4), // Left face (x-)
                Edge(0, 2),
                Edge(2, 3),
                Edge(3, 1),
                Edge(1, 0), // Right face (x+)
            ],
            faces: vec![
                // Front face (z+) - split into two triangles
                Face {
                    indices: (4, 6, 2),
                    color: fill_color,
                },
                Face {
                    indices: (4, 2, 0),
                    color: fill_color,
                },
                // Back face (z-) - split into two triangles
                Face {
                    indices: (5, 7, 3),
                    color: fill_color,
                },
                Face {
                    indices: (5, 3, 1),
                    color: fill_color,
                },
                // Top face (y+) - split into two triangles
                Face {
                    indices: (4, 0, 1),
                    color: fill_color,
                },
                Face {
                    indices: (4, 1, 5),
                    color: fill_color,
                },
                // Bottom face (y-) - split into two triangles
                Face {
                    indices: (6, 2, 3),
                    color: fill_color,
                },
                Face {
                    indices: (6, 3, 7),
                    color: fill_color,
                },
                // Left face (x-) - split into two triangles
                Face {
                    indices: (4, 6, 7),
                    color: fill_color,
                },
                Face {
                    indices: (4, 7, 5),
                    color: fill_color,
                },
                // Right face (x+) - split into two triangles
                Face {
                    indices: (0, 2, 3),
                    color: fill_color,
                },
                Face {
                    indices: (0, 3, 1),
                    color: fill_color,
                },
            ],
            center: *center,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn custom_polygon(
        points: Vec<Pos3>,
        edges: Vec<Edge>,
        faces: Vec<Face>,
        center: &Pos3,
    ) -> Self {
        Mesh {
            vertices: points,
            edges,
            faces,
            center: *center,
            out_line_color: CellColor::WHITE,
        }
    }
    pub fn rotate(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) {
        for corner in self.vertices.iter_mut() {
            corner.rotate_around_pivot(angle_x, angle_y, angle_z, &self.center);
        }
    }
    pub fn translate(&mut self, point: &Pos3) {
        let vector = (
            point.x - self.center.x,
            point.y - self.center.y,
            point.z - self.center.z,
        );
        self.center = Pos3::from(*point);
        for vertex in self.vertices.iter_mut() {
            vertex.translate(vector);
        }
    }
}
fn mid_point_in_line(from: &Pos3, to: &Pos3) -> Pos3 {
    Pos3::new(
        (from.x + to.x) / 2.0,
        (from.y + to.y) / 2.0,
        (from.z + to.z) / 2.0,
    )
}
