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
    pub fn parse(
        s: &str,
        vertex_count: usize,
        texture_count: usize,
        normal_count: usize,
    ) -> Option<Self> {
        let mut parts = s.split('/');

        let vertex = resolve_obj_index(parts.next()?, vertex_count)?;
        let texture = parse_optional_obj_index(parts.next(), texture_count)?;
        let normal = parse_optional_obj_index(parts.next(), normal_count)?;

        Some(VertexProps {
            vertex, texture, normal
        })
    }
}

fn resolve_obj_index(index: &str, item_count: usize) -> Option<usize> {
    let index = index.parse::<isize>().ok()?;
    let resolved = if index > 0 {
        usize::try_from(index - 1).ok()?
    } else if index < 0 {
        item_count.checked_sub(index.unsigned_abs())?
    } else {
        return None;
    };

    (resolved < item_count).then_some(resolved)
}

fn parse_optional_obj_index(index: Option<&str>, item_count: usize) -> Option<Option<usize>> {
    match index {
        Some("") | None => Some(None),
        Some(index) => Some(Some(resolve_obj_index(index, item_count)?)),
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

    pub fn parse_line(&mut self, line: &str) {
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
                let face: Option<Vec<VertexProps>> = tokens
                    .map(|s| {
                        VertexProps::parse(
                            s,
                            self.vertices.len(),
                            self.texture_coords.len(),
                            self.normals.len(),
                        )
                    })
                    .collect();
                if let Some(face) = face.filter(|face| face.len() >= 2) {
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
                let name = tokens.collect::<Vec<_>>().join(" ");
                if objects.is_empty() {
                    objects.push(Object::new(if name.is_empty() { "object" } else { &name }));
                }
            }
            "v" | "vt" | "vn" | "f" => {
                if objects.is_empty() {
                    objects.push(Object::new("untitled_object"));
                }
                objects.last_mut().unwrap().parse_line(trimmed);
            }
            _ => continue,
        }
    }

    Ok(objects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolves_negative_face_indices_relative_to_current_vertices() {
        let mut object = Object::new("test");
        object.parse_line("v 0 0 0");
        object.parse_line("v 1 0 0");
        object.parse_line("v 0 1 0");
        object.parse_line("f -3 -2 -1");

        assert_eq!(object.faces.len(), 1);
        assert_eq!(
            object.faces[0].iter().map(|vertex| vertex.vertex).collect::<Vec<_>>(),
            vec![0, 1, 2],
        );
    }

    #[test]
    fn resolves_global_vertex_indices_across_object_groups() {
        let path = std::env::temp_dir().join(format!(
            "funshite_obj_groups_{}.obj",
            std::process::id(),
        ));
        fs::write(
            &path,
            "v 0 0 0\nv 1 0 0\nv 0 1 0\no first\ng second\nf 1 2 3\n",
        )
        .unwrap();

        let objects = parse_file(&path).unwrap();
        fs::remove_file(path).unwrap();

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].faces.len(), 1);
        assert_eq!(
            objects[0].faces[0].iter().map(|vertex| vertex.vertex).collect::<Vec<_>>(),
            vec![0, 1, 2],
        );
    }

    #[test]
    fn rejects_a_face_with_an_invalid_vertex_reference() {
        let mut object = Object::new("test");
        object.parse_line("v 0 0 0");
        object.parse_line("v 1 0 0");
        object.parse_line("f 1 2 3");

        assert!(object.faces.is_empty());
    }
}