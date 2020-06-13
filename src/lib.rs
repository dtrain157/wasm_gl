#[macro_use]
extern crate lazy_static;

mod entity;
mod shader;

use entity::entity::Entity;
use entity::quad::Quad;
use nalgebra_glm as glm;
use shader::shader::ShaderType;
use shader::shader_controller::ShaderController;
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
    entities: Vec<Box<dyn Entity>>,
    shader_controller: ShaderController,
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl WebGlClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = Self::init_webgl_context().unwrap();
        let shader_controller = ShaderController::new(&gl);

        Self {
            entities: vec![Box::new(Quad::new(&gl, ShaderType::BasicShader))],
            gl: gl,
            shader_controller: shader_controller,
        }
    }

    pub fn update(&self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        //update_app_state(width, height);
        for e in self.entities.iter() {
            e.update(_time);
        }
        Ok(())
    }

    pub fn render(&mut self) {
        let current_state = get_current_app_state();

        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let position = glm::vec3(0.0, 0.0, 0.0);
        let rotation = glm::vec3(0.0, 0.0, 0.0);
        let scale = glm::vec3(0.2, 0.2, 1.0);

        for e in self.entities.iter() {
            e.render(&self.gl, &mut self.shader_controller, &position, &rotation, &scale);
        }
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
