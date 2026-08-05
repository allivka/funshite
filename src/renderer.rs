use crate::base::{transpose_matrix3x3, multiply_vector3d_by_matrix3x3, Vec2i, Vec3d, RGBA, Vec2d};
use crate::canvas::Canvas;
use crate::object::Object;
use crate::viewer::Viewer;

pub struct Renderer {
    pub objects: Vec<Object>,
    pub polygon_outline_thickness: isize,
    pub polygon_outline_color: RGBA,
    pub fov_degrees: f32,
    pub close_visibility_distance: f32,
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas, camera: &Viewer) {
        let transpose_rotation_matrix = transpose_matrix3x3(&camera.rotation);

        let focal_length = (canvas.width as f32) / (2.0 * (self.fov_degrees.to_radians() / 2.0).tan());

        for obj in self.objects.iter() {
            for face in obj.faces.iter() {
                let mut v2i_vec: Vec<Vec2i> = Vec::with_capacity(face.len());
                let mut v3d_buff: Vec3d;

                let mut face_behind_camera = false;

                for vertex in face.iter() {
                    let world_v = obj.vertices[vertex.vertex];

                    v3d_buff = multiply_vector3d_by_matrix3x3(
                        Vec3d(
                            world_v.0 - camera.position.0,
                            world_v.1 - camera.position.1,
                            world_v.2 - camera.position.2,
                        ),
                        &transpose_rotation_matrix,
                    );

                    if v3d_buff.2 >= -self.close_visibility_distance as f64 {
                        face_behind_camera = true;
                        break;
                    }

                    v2i_vec.push(Vec2i(
                        (canvas.width as f64 / 2.0 + (v3d_buff.0 * focal_length as f64) / -v3d_buff.2) as isize,
                        (canvas.height as f64 / 2.0 - (v3d_buff.1 * focal_length as f64) / -v3d_buff.2) as isize,
                    ));
                }

                if face_behind_camera {
                    continue;
                }

                canvas.draw_polygon(&v2i_vec, self.polygon_outline_color, self.polygon_outline_thickness);
                // canvas.fill_polygon(&v2i_vec, self.polygon_outline_color);
            }
        }
    }
}