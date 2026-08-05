use crate::base::{transpose_matrix3x3, Vec2i, multiply_vector3d_by_matrix3x3, Vec3d, RGBA};
use crate::canvas::Canvas;
use crate::object::Object;
use crate::test_canvas_primitives;
use crate::viewer::Viewer;

pub struct Renderer {
    pub objects: Vec<Object>,
    pub polygon_outline_thickness: isize,
    pub polygon_outline_color: RGBA
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas, camera: &Viewer) {
        let total_objects = self.objects.len();
        let total_faces: usize = self.objects.iter().map(|o| o.faces.len()).sum();
        eprintln!("\n[RENDER START] Objects: {}, Total Faces: {}", total_objects, total_faces);
        eprintln!("[RENDER] Camera pos: {}, rotation matrix: [[{:.2}, {:.2}, {:.2}], ...]", 
            camera.position, camera.rotation[0][0], camera.rotation[0][1], camera.rotation[0][2]);
        
        let transpose_rotation_matrix = transpose_matrix3x3(&camera.rotation);
        let mut v3d_buff: Vec3d;
        let mut total_rendered = 0;
        let mut total_culled = 0;
        let mut face_count = 0;

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            eprintln!("[RENDER] Object {}/{}: name='{}', vertices: {}, faces: {}", 
                obj_idx + 1, total_objects, obj.name, obj.vertices.len(), obj.faces.len());
            
            for (face_idx, face) in obj.faces.iter().enumerate() {
                face_count += 1;
                eprintln!("  [FACE {}/{}] Vertices in face: {}", face_count, total_faces, face.len());
                
                let mut v2i_vec: Vec<Vec2i> = Vec::with_capacity(face.len());

                for (vert_idx, vertex) in face.iter().enumerate() {
                    let vert_idx_in_obj = vertex.0 as usize;
                    if vert_idx_in_obj >= obj.vertices.len() {
                        eprintln!("    [ERROR] Vertex index {} out of bounds! (max: {})", 
                            vert_idx_in_obj, obj.vertices.len() - 1);
                        continue;
                    }
                    
                    v3d_buff = obj.vertices[vert_idx_in_obj];
                    eprintln!("    [VERT {}/{}] Original: {}", vert_idx + 1, face.len(), v3d_buff);
                    
                    v3d_buff = multiply_vector3d_by_matrix3x3(
                        Vec3d(
                            v3d_buff.0 - camera.position.0,
                            v3d_buff.1 - camera.position.1,
                            v3d_buff.2 - camera.position.2,
                        ),
                        &transpose_rotation_matrix
                    );
                    eprintln!("           After transform: {}", v3d_buff);
                    
                    let proj_x = (v3d_buff.0 / -(v3d_buff.2 + 0.001)) as isize;
                    let proj_y = (v3d_buff.1 / -(v3d_buff.2 + 0.001)) as isize;
                    eprintln!("           Projected: ({}, {})", proj_x, proj_y);
                    
                    let screen_pos = canvas.translate_centered_to_standard(Vec2i(proj_x, proj_y));
                    eprintln!("           Screen pos: {}", screen_pos);
                    
                    v2i_vec.push(screen_pos);
                }

                // Check if all vertices are off-screen
                let all_offscreen = v2i_vec.iter().all(|p| {
                    p.0 < 0 || p.0 >= canvas.width || p.1 < 0 || p.1 >= canvas.height
                });
                
                if all_offscreen {
                    eprintln!("  [FACE {}/{}] CULLED (all vertices off-screen)", face_count, total_faces);
                    total_culled += 1;
                    continue;
                }
                
                eprintln!("  [FACE {}/{}] RENDERING with {} vertices, thickness: {}", 
                    face_count, total_faces, v2i_vec.len(), self.polygon_outline_thickness);
                canvas.draw_polygon(&v2i_vec, self.polygon_outline_color, self.polygon_outline_thickness);
                total_rendered += 1;
                eprintln!("  [FACE {}/{}] DONE", face_count, total_faces);
            }
        }
        
        eprintln!("[RENDER END] Rendered: {}, Culled: {}, Total: {}\n", 
            total_rendered, total_culled, total_faces);
    }
}

