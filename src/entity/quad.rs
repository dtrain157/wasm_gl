use super::entity::Entity;
use crate::shader::shader::Shader;
use js_sys::WebAssembly;
use nalgebra_glm as glm;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Quad<'a, T: Shader> {
    shader: &'a T,
    rect_vertex_array_buffer: WebGlBuffer,
    rect_index_array_length: usize,
}

impl<'a, T: Shader> Entity for Quad<'a, T> {
    fn render(&self, gl: &WebGlRenderingContext, position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3) {
        gl.use_program(Some(&self.shader.get_program()));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertex_array_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(self.shader.get_uniform_location(&gl, "u_Colour").as_ref(), 0.1, 0.9, 0.1, 1.0);

        let translate = glm::translate(&glm::Mat4::identity(), position);
        let rotate_x = glm::rotate(&glm::Mat4::identity(), rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let rotate_y = glm::rotate(&glm::Mat4::identity(), rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let rotate_z = glm::rotate(&glm::Mat4::identity(), rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let scale = glm::scale(&glm::Mat4::identity(), scale);
        let transformation_matrix = translate * rotate_x * rotate_y * rotate_z * scale;

        gl.uniform_matrix4fv_with_f32_array(self.shader.get_uniform_location(&gl, "u_Transform").as_ref(), false, &transformation_matrix.as_slice());

        gl.draw_elements_with_i32(GL::TRIANGLES, self.rect_index_array_length as i32, GL::UNSIGNED_SHORT, 0);
    }

    fn update(&self, _time: f32) {}
}

impl<'a, T: Shader> Quad<'a, T> {
    pub fn new(gl: &WebGlRenderingContext, shader: &'a T) -> Self {
        let vertices_rect: [f32; 8] = [
            0.0, 1.0, //x, y
            0.0, 0.0, //x, y
            1.0, 1.0, //x, y
            1.0, 0.0, //x, y
        ];

        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let vertex_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&vertex_array_memory_buffer).subarray(vertices_location, vertices_location + vertices_rect.len() as u32);
        let vertex_array_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_array_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let index_array_memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
        let indices_location = indices_rect.as_ptr() as u32 / 2;
        let ind_array = js_sys::Uint16Array::new(&index_array_memory_buffer).subarray(indices_location, indices_location + indices_rect.len() as u32);
        let index_array_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_array_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &ind_array, GL::STATIC_DRAW);

        Self {
            rect_vertex_array_buffer: vertex_array_buffer,
            rect_index_array_length: indices_rect.len(),
            shader: shader,
        }
    }
}
