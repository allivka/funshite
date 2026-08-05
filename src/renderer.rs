use crate::base::{transpose_matrix3x3, Vec2i, multiply_vector3d_by_matrix3x3, Vec3d, RGBA};
use crate::canvas::Canvas;
use crate::object::Object;
use crate::viewer::Viewer;

pub struct Renderer {
    pub objects: Vec<Object>,
    pub polygon_outline_thickness: isize,
    pub polygon_outline_color: RGBA
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas, camera: &Viewer) {
        let transpose_rotation_matrix = transpose_matrix3x3(&camera.rotation);
        let mut v3d_buff: Vec3d;


        for obj in self.objects.iter() {
            for face in obj.faces.iter() {
                let mut v2i_vec: Vec<Vec2i> = Vec::with_capacity(face.len());

                for vertex in face.iter() {
                    v3d_buff = obj.vertices[vertex.0 as usize];
                    v3d_buff = multiply_vector3d_by_matrix3x3(
                        Vec3d(
                            v3d_buff.0 - camera.position.0,
                            v3d_buff.1 - camera.position.1,
                            v3d_buff.2 - camera.position.2,
                        ),
                        &transpose_rotation_matrix
                    );

                    v2i_vec.push(canvas.translate_centered_to_standard(Vec2i(
                        (v3d_buff.0 / -(v3d_buff.2 + 0.001)) as isize,
                        (v3d_buff.1 / -(v3d_buff.2 + 0.001)) as isize
                    )));
                }

                canvas.draw_polygon(&v2i_vec, self.polygon_outline_color, self.polygon_outline_thickness);
            }
        }
    }
}

