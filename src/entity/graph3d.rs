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
    y_values_array_buffer: WebGlBuffer,
    normals_array_buffer: WebGlBuffer,
    n: usize,
}

impl Entity for Graph3d {
    fn render(&self, gl: &WebGlRenderingContext, shader_controller: &ShaderController, position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3) {
        let shader = shader_controller.get_shader(&self.shader_type);
        match shader {
            Some(shader) => {
                let current_state = get_current_app_state();

                shader_controller.use_shader(gl, self.shader_type);

                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertex_array_buffer));
                gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(0);

                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_values_array_buffer));
                gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(1);

                let y_vals = self.get_updated_3d_y_values(current_state.time);
                let y_values_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
                let y_values_location = y_vals.as_ptr() as u32 / 4;
                let y_values_array = js_sys::Float32Array::new(&y_values_array_memory_buffer).subarray(y_values_location, y_values_location + y_vals.len() as u32);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &y_values_array, GL::DYNAMIC_DRAW);

                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_array_buffer));
                gl.vertex_attrib_pointer_with_i32(2, 3, GL::FLOAT, false, 0, 0);
                gl.enable_vertex_attrib_array(2);

                let normals = self.get_grid_normals(&y_vals);
                let normals_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
                let normals_location = normals.as_ptr() as u32 / 4;
                let normals_array = js_sys::Float32Array::new(&normals_array_memory_buffer).subarray(normals_location, normals_location + normals.len() as u32);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals_array, GL::DYNAMIC_DRAW);

                let translate = glm::translate(&glm::Mat4::identity(), position);
                let rotate_x = glm::rotate_x(&glm::Mat4::identity(), rotation.y + current_state.rotation_y);
                let rotate_y = glm::rotate_y(&glm::Mat4::identity(), rotation.x + current_state.rotation_x);
                let rotate = rotate_x * rotate_y;
                let scale = glm::scale(&rotate_y, scale);
                let transformation_matrix = translate * rotate * scale;
                let normals_rotation = rotate.try_inverse().unwrap();
                let projection_matrix = current_state.get_projection_matrix();

                gl.uniform_matrix4fv_with_f32_array(shader.get_uniform_location(&gl, "uNormalsRotation").as_ref(), false, &normals_rotation.as_slice());
                gl.uniform_matrix4fv_with_f32_array(shader.get_uniform_location(&gl, "uModel").as_ref(), false, &transformation_matrix.as_slice());
                gl.uniform_matrix4fv_with_f32_array(shader.get_uniform_location(&gl, "uViewProjection").as_ref(), false, &projection_matrix.as_slice());

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
            y_values_array_buffer: gl.create_buffer().ok_or("Failed to create buffer").unwrap(),
            normals_array_buffer: gl.create_buffer().ok_or("Failed to create buffer").unwrap(),
            n: n,
        }
    }

    fn get_updated_3d_y_values(&self, curr_time: f32) -> Vec<f32> {
        let point_count_per_row = self.n + 1;
        let mut y_vals: Vec<f32> = vec![0.0; point_count_per_row * point_count_per_row];

        let half_grid = point_count_per_row as f32 / 2.0;
        let frequency_scale = 3.0 * std::f32::consts::PI;
        let y_scale = 0.15;
        let sin_offset = curr_time / 1000.0;

        for z in 0..point_count_per_row {
            for x in 0..point_count_per_row {
                let use_y_index = z * point_count_per_row + x;
                let scaled_x = frequency_scale * (x as f32 - half_grid) / half_grid;
                let scaled_z = frequency_scale * (z as f32 - half_grid) / half_grid;
                y_vals[use_y_index] = y_scale * ((scaled_x * scaled_x + scaled_z * scaled_z).sqrt() + sin_offset).sin();
            }
        }

        y_vals
    }

    fn get_grid_normals(&self, y_vals: &Vec<f32>) -> Vec<f32> {
        let point_count_per_row = self.n + 1;
        let graph_layout_width = 2.0;
        let square_size: f32 = graph_layout_width / self.n as f32;
        let mut normals: Vec<f32> = vec![0.0; 3 * point_count_per_row * point_count_per_row];

        for z in 0..point_count_per_row {
            for x in 0..point_count_per_row {
                let y_val_index_a = z * point_count_per_row + x;
                let normals_start_index = 3 * y_val_index_a;

                if z == self.n || x == self.n {
                    normals[normals_start_index + 1] = 1.0;
                } else {
                    let y_val_index_b = y_val_index_a + point_count_per_row;
                    let y_val_index_c = y_val_index_a + 1;

                    let x_val_1 = square_size * x as f32;
                    let x_val_2 = x_val_1 + square_size;

                    let z_val_1 = square_size * z as f32;
                    let z_val_2 = z_val_1 + square_size;

                    let a = glm::vec3(x_val_1, y_vals[y_val_index_a], z_val_1);
                    let b = glm::vec3(x_val_1, y_vals[y_val_index_b], z_val_2);
                    let c = glm::vec3(x_val_2, y_vals[y_val_index_c], z_val_2);

                    let normal = glm::triangle_normal(&a, &b, &c);

                    normals[normals_start_index] = normal.x;
                    normals[normals_start_index + 1] = normal.y;
                    normals[normals_start_index + 2] = normal.z;
                }
            }
        }
        normals
    }
}
