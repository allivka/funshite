use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::base::{Vec2d, Vec3d};

#[derive(Copy, Clone)]
pub struct VertexProps(pub isize, pub isize, pub isize);

impl VertexProps {
    pub fn new_from_string(s: &str) -> VertexProps {
        let v: Vec<&str> = s.split("/").collect();
        let mut res =  VertexProps(0, 0, 0);

        res.0 = v[0].parse::<isize>().unwrap();

        if v.len() == 1 {
            return res;
        }

        if v[1] != "" {
            res.1 = v[1].parse::<isize>().unwrap();
        }

        if v.len() == 2 {
            return res;
        }

        res.2 = v[2].parse::<isize>().unwrap();

        res

    }
}

pub struct Object {
    pub name: String,
    pub vertices: Vec<Vec3d>,
    pub normals: Vec<Vec3d>,
    pub texture_coords: Vec<Vec2d>,
    pub faces: Vec<Vec<VertexProps>>,
}

impl Object {

    pub fn new(name: &str) -> Object {
        Object {
            name: name.to_string(),
            vertices: Vec::new(),
            normals: Vec::new(),
            texture_coords: Vec::new(),
            faces: Vec::new(),

        }
    }
    pub fn parse_line(&mut self, s: &str) {

        let l: Vec<&str> = s.split(" ").collect();

        match l[0] {
            "v" => {
                self.vertices.push(Vec3d(l[1].parse().unwrap(), l[2].parse().unwrap(), l[3].parse().unwrap()));
            },
            "vt" => {
                self.texture_coords.push(Vec2d(l[1].parse().unwrap(), l[2].parse().unwrap()))

            },
            "vn" => {
                self.normals.push(Vec3d(l[1].parse().unwrap(), l[2].parse().unwrap(), l[3].parse().unwrap()));

            },
            "f" => {
                let mut t: Vec<VertexProps> = Vec::with_capacity(l.len() - 1);

                l[1..].iter().for_each(
                    |s| {
                        t.push(VertexProps::new_from_string(s))
                    }
                );

                self.faces.push(t)

            },
            _ => return
        }

    }
}

pub fn parse_file(path: &Path) -> Result<Vec<Object>, std::io::Error> {
    let file = File::open(path)?;

    let mut objects: Vec<Object> = Vec::new();

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(e) => { return Err(e) }
        };
        
        if line == "" {
            continue;
        }

        match &line[..2] {
            "o " => {
                objects.push(Object::new(line.split_once(' ').unwrap().1));
            },
            "v " | "f " | "vt" | "vn" => {
                if i == 0 {
                    objects.push(Object::new("untitled_object"));
                }

                objects.last_mut().unwrap().parse_line(&line);
            }
            _ => continue
        }
    }

    Ok(objects)
}

