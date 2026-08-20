use std::sync::atomic::AtomicU32;
use crate::base::{transpose_matrix3x3, multiply_vector3d_by_matrix3x3, Vec2i, Vec3d, RGBA};
use crate::canvas::Canvas;
use crate::object::Object;
use crate::viewer::Viewer;
use rayon::prelude::*;

pub struct VertexIdx {
    obj_idx: usize,
    face_idx: usize,
    self_idx: usize,
}

pub struct Renderer {
    pub objects: Vec<Object>,
    pub polygon_outline_thickness: isize,
    pub polygon_outline_color: RGBA,
    pub fov_degrees: f32,
    pub close_visibility_distance: f32,
    pub far_visibility_distance: f32,
    pub focal_length: f32,

    // 0 - object idx, 1 - first vertex idx in the object buffer, 2 - second vertex idx in the object buffer
    pub index_pair_buffer: Vec<(VertexIdx, VertexIdx)>,

}

impl Renderer {
    pub fn initialize_index_pair_buffer(&mut self) {

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for (face_idx, face) in obj.faces.iter().enumerate() {
                match face.len() {
                    0 | 1 => continue,
                    2 => {
                        self.index_pair_buffer.push((
                            VertexIdx {
                                obj_idx,
                                face_idx,
                                self_idx: 0,
                            },
                            VertexIdx {
                                obj_idx,
                                face_idx,
                                self_idx: 1,
                            },
                        ));
                    },
                    _ => (),
                }

                for self_idx in 0..face.len() - 1 {
                    self.index_pair_buffer.push((
                        VertexIdx {
                            obj_idx,
                            face_idx,
                            self_idx,
                        },
                        VertexIdx {
                            obj_idx,
                            face_idx,
                            self_idx: self_idx + 1,
                        },
                        ));
                }

                self.index_pair_buffer.push((
                    VertexIdx {
                        obj_idx,
                        face_idx,
                        self_idx: 0,
                    },
                    VertexIdx {
                        obj_idx,
                        face_idx,
                        self_idx: face.len() - 1,
                    },
                ));
            }
        }

    }

    pub fn calculate_focal_length(&mut self, canvas: &Canvas<AtomicU32>) {
        self.focal_length = (canvas.width as f32) / (2.0 * (self.fov_degrees.to_radians() / 2.0).tan());
    }

    pub fn init(mut self, canvas: &Canvas<AtomicU32>) -> Self {
        self.initialize_index_pair_buffer();
        self.calculate_focal_length(canvas);
        self
    }


    pub fn render(&self, canvas: &Canvas<AtomicU32>, camera: &Viewer) {
        let transpose_rotation_matrix = transpose_matrix3x3(&camera.rotation);

        self.index_pair_buffer.par_iter().for_each(|(v1_idx, v2_idx)| {
            let mut v1 = self.objects[
                v1_idx.obj_idx].vertices[self.objects[v1_idx.obj_idx].faces[v1_idx.face_idx][v1_idx.self_idx].vertex
                ];
            let mut v2 = self.objects[
                v2_idx.obj_idx].vertices[self.objects[v2_idx.obj_idx].faces[v2_idx.face_idx][v2_idx.self_idx].vertex
                ];

            v1 = multiply_vector3d_by_matrix3x3(
                Vec3d(
                    v1.0 - camera.position.0,
                    v1.1 - camera.position.1,
                    v1.2 - camera.position.2,
                ),
                &transpose_rotation_matrix,
            );

            v2 = multiply_vector3d_by_matrix3x3(
                Vec3d(
                    v2.0 - camera.position.0,
                    v2.1 - camera.position.1,
                    v2.2 - camera.position.2,
                ),
                &transpose_rotation_matrix,
            );

            if v1.2 >= -self.close_visibility_distance as f64 || v2.2 >= -self.close_visibility_distance as f64 {
                return;
            }

            if v1.2 <= -self.far_visibility_distance as f64 || v2.2 <= -self.far_visibility_distance as f64 {
                return;
            }

            canvas.draw_line_thick(
                Vec2i(
                    (canvas.width as f64 / 2.0 + (v1.0 * self.focal_length as f64) / -v1.2) as isize,
                    (canvas.height as f64 / 2.0 - (v1.1 * self.focal_length as f64) / -v1.2) as isize,
                ),
                Vec2i(
                    (canvas.width as f64 / 2.0 + (v2.0 * self.focal_length as f64) / -v2.2) as isize,
                    (canvas.height as f64 / 2.0 - (v2.1 * self.focal_length as f64) / -v2.2) as isize,
                ),
                self.polygon_outline_color,
                self.polygon_outline_thickness,
            );
        });
    }
}