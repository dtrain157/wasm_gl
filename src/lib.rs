#[macro_use]
extern crate lazy_static;

use js_sys::WebAssembly;
use nalgebra_glm as glm;
use std::sync::Arc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

/**** Import functions from JS **(*/
#[wasm_bindgen]
extern "C" {
    type Buffer;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file(path: &str) -> Result<Buffer, JsValue>;
}

#[wasm_bindgen]
pub struct WebGlClient {
    gl: WebGlRenderingContext,
    program_2d: GlProgram,
}

#[wasm_bindgen]
impl WebGlClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = Self::init_webgl_context().unwrap();
        let shader = Shader::new(&gl);
        let program = GlProgram::new(&gl, &shader);

        Self { gl: gl, program_2d: program }
    }

    pub fn update(&self, _time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        update_app_state(width, height);
        Ok(())
    }

    pub fn render(&self) {
        let current_state = get_current_app_state();

        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program_2d
            .render(&self.gl, 100.0, 200.0, 100.0, 200.0, current_state.canvas_width, current_state.canvas_height);
    }
}

impl WebGlClient {
    pub fn init_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("wasmCanvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.1, 0.1, 0.9, 1.0);
        gl.clear_depth(1.0);

        Ok(gl)
    }
}

pub struct Shader {
    vertex: WebGlShader,
    fragment: WebGlShader,
}

impl Shader {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let vertex_code = include_bytes!("./assets/shaders/vertex.glsl");
        let fragment_code = include_bytes!("./assets/shaders/fragment.glsl");

        Self {
            vertex: Self::compile_shader(gl, GL::VERTEX_SHADER, &String::from_utf8_lossy(vertex_code)).unwrap(),
            fragment: Self::compile_shader(gl, GL::FRAGMENT_SHADER, &String::from_utf8_lossy(fragment_code)).unwrap(),
        }
    }

    fn compile_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Error creating shader"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        } else {
            Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unable to get shader info log")))
        }
    }
}

pub struct GlProgram {
    program: WebGlProgram,
    rect_vertex_array_buffer: WebGlBuffer,
    rect_vertex_array_length: usize,
    rect_index_array_buffer: WebGlBuffer,
    rect_index_array_length: usize,
    u_colour: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl GlProgram {
    pub fn new(gl: &WebGlRenderingContext, shader: &Shader) -> Self {
        let program = Self::link_program(gl, shader).unwrap();

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
            u_colour: gl.get_uniform_location(&program, "u_Colour").unwrap(),
            u_transform: gl.get_uniform_location(&program, "u_Transform").unwrap(),
            rect_vertex_array_buffer: vertex_array_buffer,
            rect_vertex_array_length: vertices_rect.len(),
            rect_index_array_buffer: index_array_buffer,
            rect_index_array_length: indices_rect.len(),
            program: program,
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, bottom: f32, top: f32, left: f32, right: f32, canvas_width: f32, canvas_height: f32) {
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertex_array_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_colour), 0.1, 0.9, 0.1, 1.0);
        let translation_vector = glm::vec3(2.0 * left / canvas_width - 1.0, 2.0 * bottom / canvas_height - 1.0, 0.0);
        let scale_vector = glm::vec3(2.0 * (right - left) / canvas_width, 2.0 * (top - bottom) / canvas_height, 0.0);
        let scale = glm::scale(&glm::Mat4::identity(), &scale_vector);
        let translation = glm::translate(&glm::Mat4::identity(), &translation_vector);
        let transformation_matrix = translation * scale;

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transformation_matrix.as_slice());

        gl.draw_elements_with_i32(GL::TRIANGLES, self.rect_index_array_length as i32, GL::UNSIGNED_SHORT, 0);
    }

    fn link_program(gl: &WebGlRenderingContext, shader: &Shader) -> Result<WebGlProgram, String> {
        let new_program = gl.create_program().ok_or_else(|| String::from("Error creating program")).unwrap();

        gl.attach_shader(&new_program, &shader.vertex);
        gl.attach_shader(&new_program, &shader.fragment);
        gl.link_program(&new_program);

        if gl.get_program_parameter(&new_program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(new_program)
        } else {
            Err(gl.get_program_info_log(&new_program).unwrap_or_else(|| String::from("Unable to create GL program")))
        }
    }
}

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_app_state(canvas_width: f32, canvas_height: f32) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_width: canvas_width,
        canvas_height: canvas_height,
        ..*data.clone()
    })
}

pub fn get_current_app_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.0,
            canvas_width: 0.0,
        }
    }
}
