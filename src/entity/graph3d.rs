use super::entity::Entity;
use crate::app_state::*;
use crate::shader::shader_controller::{ShaderController, ShaderType};
use js_sys::WebAssembly;
use nalgebra_glm as glm;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Graph3d {
    shader_type: ShaderType,
    rect_vertex_array_buffer: WebGlBuffer,
    rect_index_array_length: usize,
}

impl Entity for Graph3d {
    fn render(&self, gl: &WebGlRenderingContext, shader_controller: &ShaderController, position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3) {
        let shader = shader_controller.get_shader(&self.shader_type);
        match shader {
            Some(shader) => {
                shader_controller.use_shader(gl, self.shader_type);
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertex_array_buffer));
                gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(0);

                //gl.uniform4f(shader.get_uniform_location(&gl, "u_Colour").as_ref(), 0.1, 0.9, 0.1, 1.0);

                let current_state = get_current_app_state();
                let translate = glm::translate(&glm::Mat4::identity(), position);
                let rotate_x = glm::rotate(&glm::Mat4::identity(), rotation.x, &glm::vec3(1.0, 0.0, 0.0));
                let rotate_y = glm::rotate(&glm::Mat4::identity(), rotation.y, &glm::vec3(0.0, 1.0, 0.0));
                let rotate_z = glm::rotate(&glm::Mat4::identity(), rotation.z, &glm::vec3(0.0, 0.0, 1.0));
                let scale = glm::scale(&glm::Mat4::identity(), scale);
                let transformation_matrix = translate * rotate_x * rotate_y * rotate_z * scale;

                gl.uniform_matrix4fv_with_f32_array(shader.get_uniform_location(&gl, "uModel").as_ref(), false, &transformation_matrix.as_slice());
                gl.uniform_matrix4fv_with_f32_array(
                    shader.get_uniform_location(&gl, "uViewProjection").as_ref(),
                    false,
                    &current_state.get_projection_matrix().as_slice(),
                );

                gl.draw_elements_with_i32(GL::TRIANGLES, self.rect_index_array_length as i32, GL::UNSIGNED_SHORT, 0);
            }
            None => {}
        }
    }

    fn update(&self, _time: f32) {}
}

impl Graph3d {
    pub fn new(gl: &WebGlRenderingContext, shader_type: ShaderType, n: usize) -> Self {
        let mut vertices: Vec<f32> = vec![0.0; 3 * (n + 1) * (n + 1)];
        let mut indices: Vec<u16> = vec![0; 6 * n * n];

        let square_size = 2.0 / n as f32;

        for z in 0..(n + 1) {
            for x in 0..(n + 1) {
                let start_pos_i = 3 * (z * (n + 1) + x);
                vertices[start_pos_i] = -1.0 + (x as f32) * square_size;
                vertices[start_pos_i + 2] = -1.0 + (z as f32) * square_size;

                if z < n && x < n {
                    let start_ind_i = 6 * (z * n + x);

                    let vertex_index_top_left = (z * (n + 1) + x) as u16;
                    let vertex_index_bottom_left = vertex_index_top_left + (n + 1) as u16;
                    let vertex_index_top_right = vertex_index_top_left + 1 as u16;
                    let vertex_index_bottom_right = vertex_index_bottom_left + 1 as u16;

                    indices[start_ind_i] = vertex_index_top_left;
                    indices[start_ind_i + 1] = vertex_index_bottom_left;
                    indices[start_ind_i + 2] = vertex_index_bottom_right;
                    indices[start_ind_i + 3] = vertex_index_top_left;
                    indices[start_ind_i + 4] = vertex_index_bottom_right;
                    indices[start_ind_i + 5] = vertex_index_top_right;
                }
            }
        }

        let vertex_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&vertex_array_memory_buffer).subarray(vertices_location, vertices_location + vertices.len() as u32);
        let vertex_array_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_array_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let index_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let indices_location = indices.as_ptr() as u32 / 2;
        let ind_array = js_sys::Uint16Array::new(&index_array_memory_buffer).subarray(indices_location, indices_location + indices.len() as u32);
        let index_array_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_array_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &ind_array, GL::STATIC_DRAW);

        Self {
            shader_type: shader_type,
            rect_vertex_array_buffer: vertex_array_buffer,
            rect_index_array_length: indices.len(),
        }
    }
}
