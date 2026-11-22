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

            let start = (
                canvas.width as f64 / 2.0
                    + (v1.0 * self.focal_length as f64) / -v1.2,
                canvas.height as f64 / 2.0
                    - (v1.1 * self.focal_length as f64) / -v1.2,
            );
            let end = (
                canvas.width as f64 / 2.0
                    + (v2.0 * self.focal_length as f64) / -v2.2,
                canvas.height as f64 / 2.0
                    - (v2.1 * self.focal_length as f64) / -v2.2,
            );

            if let Some((start, end)) =
                clip_line_to_screen(start, end, canvas.width, canvas.height)
            {
                canvas.draw_line(start, end, self.polygon_outline_color);
            }
        });
    }
}

fn clip_line_to_screen(
    start: (f64, f64),
    end: (f64, f64),
    width: isize,
    height: isize,
) -> Option<(Vec2i, Vec2i)> {
    if width <= 0 || height <= 0 {
        return None;
    }

    let (x0, y0) = start;
    let (x1, y1) = end;
    if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
        return None;
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    if !dx.is_finite() || !dy.is_finite() {
        return None;
    }

    let mut t_enter: f64 = 0.0;
    let mut t_exit: f64 = 1.0;
    let right = (width - 1) as f64;
    let bottom = (height - 1) as f64;

    for (p, q) in [
        (-dx, x0),
        (dx, right - x0),
        (-dy, y0),
        (dy, bottom - y0),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
            continue;
        }

        let t = q / p;
        if p < 0.0 {
            t_enter = t_enter.max(t);
        } else {
            t_exit = t_exit.min(t);
        }
        if t_enter > t_exit {
            return None;
        }
    }

    let clipped_start = (
        (x0 + t_enter * dx).clamp(0.0, right) as isize,
        (y0 + t_enter * dy).clamp(0.0, bottom) as isize,
    );
    let clipped_end = (
        (x0 + t_exit * dx).clamp(0.0, right) as isize,
        (y0 + t_exit * dy).clamp(0.0, bottom) as isize,
    );

    Some((
        Vec2i(clipped_start.0, clipped_start.1),
        Vec2i(clipped_end.0, clipped_end.1),
    ))
}