use crate::model::elements::face::Face;
use crate::model::elements::{mesh::Mesh, pos3::Pos3};
use crate::screenspace::elements::cell_color::CellColor;
use std::io::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_obj_into_mesh(
    filename: &str,
    color: Option<CellColor>,
) -> Result<Mesh, Box<dyn std::error::Error>> {
    let mut to_ret = Mesh::empty();
    for line in file_into_lines(filename)? {
        parse_line(line, &mut to_ret, color)?;
    }
    Ok(to_ret)
}
fn file_into_lines(filename: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<std::io::Result<Vec<String>>>()?;
    Ok(lines)
}
fn parse_line(
    line: String,
    mesh: &mut Mesh,
    color: Option<CellColor>,
) -> Result<(), Box<dyn std::error::Error>> {
    let line_split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    match line_split[0].as_str() {
        "v" => {
            mesh.vertices
                .push(parse_vertex((&line_split[1..]).to_vec())?);
        }
        "f" => {
            for face in parse_face((&line_split[1..]).to_vec(), color)? {
                mesh.faces.push(face);
            }
        }
        _ => {} // vn vt and other things we ignore
    }

    Ok(())
}
fn parse_vertex(args: Vec<String>) -> Result<Pos3, Box<dyn std::error::Error>> {
    let x: f64 = args[0].parse()?;
    let y: f64 = args[1].parse()?;
    let z: f64 = args[2].parse()?;
    Ok(Pos3 { x, y, z })
}
fn parse_face(
    args: Vec<String>,
    color: Option<CellColor>,
) -> Result<Vec<Face>, Box<dyn std::error::Error>> {
    let mut vertices = Vec::<usize>::new();
    let mut faces = Vec::<Face>::new();
    for vertex_value in args {
        vertices.push(parse_face_vertex(vertex_value)?);
    }
    for i in 1..vertices.len() - 1 {
        faces.push(Face {
            indices: (vertices[0], vertices[i - 1], vertices[i]),
            color: color,
        });
    }
    Ok(faces)
}
fn parse_face_vertex(arg: String) -> Result<usize, Box<dyn std::error::Error>> {
    let index: usize = arg
        .split("/")
        .nth(0)
        .ok_or(Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not split face vertex with /",
        ))?
        .parse()?;
    Ok(index)
}
