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
    if line_split.len() < 1 {
        return Ok(());
    }
    match line_split[0].as_str() {
        "v" => {
            mesh.vertices
                .push(parse_vertex((&line_split[1..]).to_vec())?);
        }
        "f" => parse_face(mesh, (&line_split[1..]).to_vec(), color)?,
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
    mesh: &mut Mesh,
    args: Vec<String>,
    color: Option<CellColor>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut vertices = Vec::<usize>::new();
    for vertex_value in args {
        vertices.push(parse_face_vertex(vertex_value)?);
    }
    let mut centroid = Pos3::new(0.0, 0.0, 0.0);
    for vertex in &vertices {
        centroid.x += mesh.vertices[*vertex].x;
        centroid.y += mesh.vertices[*vertex].y;
        centroid.z += mesh.vertices[*vertex].z;
    }
    centroid = Pos3 {
        x: centroid.x / vertices.len() as f64,
        y: centroid.y / vertices.len() as f64,
        z: centroid.z / vertices.len() as f64,
    };
    mesh.vertices.push(centroid);
    let centoid_index = mesh.vertices.iter().len() - 1;
    for i in 0..vertices.len() - 1 {
        mesh.faces.push(Face {
            indices: (vertices[i], vertices[i + 1], centoid_index),
            color: color,
        });
    }
    Ok(())
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
    Ok(index - 1)
}
