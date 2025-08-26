use std::rc::Rc;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use crate::geometry::vector::Vector;
use crate::scene::materials::Material;
use crate::scene::elements::{SceneElement, Triangle};

pub fn read_obj(filename: &str, material: Rc<dyn Material>) -> Vec<Rc<dyn SceneElement>> {
    let file = File::open(filename).expect("Couldn't open model file");

    let mut vertices: Vec<Vector> = Vec::new();
    let mut triangles: Vec<Rc<dyn SceneElement>> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.expect("Couldn't parse model file");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts[0] == "v" {
            let x: f64 = parts[1].parse().unwrap();
            let y: f64 = parts[2].parse().unwrap();
            let z: f64 = parts[3].parse().unwrap();
            vertices.push(Vector::new(x, y, z));

        } else if parts[0] == "f" {
            let mut indexes: Vec<usize> = Vec::new();
            for i in 1..parts.len() {
                let vertex = parts[i].split("/").next().unwrap();
                let index: usize = vertex.parse().unwrap();
                // Vertices are 1-indexed in .obj files
                indexes.push(index - 1);
            }

            for i in 2..indexes.len() {
                triangles.push(Rc::new(Triangle::new(
                    vertices[indexes[0]],
                    vertices[indexes[i - 1]],
                    vertices[indexes[i]],
                    material.clone()
                )));
            }
        }
    }

    triangles
}
