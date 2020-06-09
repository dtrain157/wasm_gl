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
}

#[wasm_bindgen]
impl WebGlClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = Self::init_webgl_context().unwrap();
        Self { gl: gl }
    }

    pub fn update(&self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Update!");
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
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
        let s = Shader::new(&gl, "./assets/shaders/vertex.glsl", "./assets/shaders/fragment.glsl");
        let program = gl.create_program().ok_or_else(|| String::from("Error creating program"))?;
        gl.attach_shader(&program, &s.vertex);
        gl.attach_shader(&program, &s.fragment);
        gl.link_program(&program);
        Ok(gl)
    }
}

pub struct Shader {
    pub vertex: WebGlShader,
    pub fragment: WebGlShader,
}

impl Shader {
    pub fn new(gl: &WebGlRenderingContext, vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_code = read_file(vertex_path).unwrap();
        let fragment_code = read_file(fragment_path).unwrap();

        Self {
            vertex: Self::compile_shader(gl, GL::VERTEX_SHADER, &vertex_code).unwrap(),
            fragment: Self::compile_shader(gl, GL::FRAGMENT_SHADER, &fragment_code).unwrap(),
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
