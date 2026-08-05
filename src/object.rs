use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::base::{Vec2d, Vec3d};



#[derive(Copy, Clone, Debug)]
pub struct VertexProps {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
}

impl VertexProps {
    pub fn parse(s: &str, v_off: usize, t_off: usize, n_off: usize) -> Option<Self> {
        let mut parts = s.split('/');

        let v_str = parts.next()?;
        let v_idx: isize = v_str.parse().ok()?;

        if v_idx <= 0 {
            return None;
        }

        let global_idx = (v_idx - 1) as usize;
        let vertex = global_idx.checked_sub(v_off)?;


        let texture = match parts.next() {
            Some("") | None => None,
            Some(t_str) => {
                let t_idx: isize = t_str.parse().ok()?;

                if t_idx > 0 {
                    let global_idx = (t_idx - 1) as usize;
                    global_idx.checked_sub(t_off)

                } else {
                    None
                }
            }
        };

        let normal = match parts.next() {
            Some("") | None => None,
            Some(n_str) => {
                let n_idx: isize = n_str.parse().ok()?;

                if n_idx > 0 {
                    let global_idx = (n_idx - 1) as usize;
                    global_idx.checked_sub(n_off)

                } else {
                    None
                }
            }
        };

        Some(VertexProps {
            vertex, texture, normal
        })
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
    pub fn new(name: &str) -> Self {
        Object {
            name: name.to_string(),
            vertices: Vec::new(),
            normals: Vec::new(),
            texture_coords: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str, v_off: usize, vt_off: usize, vn_off: usize) {
        let mut tokens = line.split_whitespace();
        let keyword = match tokens.next() {
            Some(k) => k,
            None => return,
        };

        match keyword {
            "v" => {
                if let (Some(x), Some(y), Some(z)) = (tokens.next(), tokens.next(), tokens.next()) {
                    if let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) {
                        self.vertices.push(Vec3d(x, y, z));
                    }
                }
            }
            "vt" => {
                if let (Some(u), Some(v)) = (tokens.next(), tokens.next()) {
                    if let (Ok(u), Ok(v)) = (u.parse(), v.parse()) {
                        self.texture_coords.push(Vec2d(u, v));
                    }
                }
            }
            "vn" => {
                if let (Some(x), Some(y), Some(z)) = (tokens.next(), tokens.next(), tokens.next()) {
                    if let (Ok(x), Ok(y), Ok(z)) = (x.parse(), y.parse(), z.parse()) {
                        self.normals.push(Vec3d(x, y, z));
                    }
                }
            }
            "f" => {
                let face: Vec<VertexProps> = tokens
                    .filter_map(|s| VertexProps::parse(s, v_off, vt_off, vn_off))
                    .collect();
                if !face.is_empty() {
                    self.faces.push(face);
                }
            }
            _ => {}
        }
    }
}

pub fn parse_file(path: &Path) -> Result<Vec<Object>, std::io::Error> {
    let file = File::open(path)?;
    let mut objects: Vec<Object> = Vec::new();

    let mut v_offset = 0usize;
    let mut vt_offset = 0usize;
    let mut vn_offset = 0usize;

    for line in BufReader::new(file).lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let mut tokens = trimmed.split_whitespace();
        let keyword = match tokens.next() {
            Some(k) => k,
            None => continue,
        };

        match keyword {
            "o" | "g" => {

                if let Some(last_obj) = objects.last() {
                    v_offset += last_obj.vertices.len();
                    vt_offset += last_obj.texture_coords.len();
                    vn_offset += last_obj.normals.len();
                }

                let name = tokens.collect::<Vec<_>>().join(" ");
                objects.push(Object::new(if name.is_empty() { "object" } else { &name }));
            }
            "v" | "vt" | "vn" | "f" => {
                if objects.is_empty() {
                    objects.push(Object::new("untitled_object"));
                }
                objects.last_mut().unwrap().parse_line(trimmed, v_offset, vt_offset, vn_offset);
            }
            _ => continue,
        }
    }

    Ok(objects)
}