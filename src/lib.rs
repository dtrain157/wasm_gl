#[macro_use]
extern crate lazy_static;

mod app_state;
mod entity;
mod shader;

use app_state::*;
use entity::entity::Entity;
use entity::graph3d::Graph3d;
use entity::quad::Quad;
use nalgebra_glm as glm;
use shader::shader_controller::{ShaderController, ShaderType};
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
            entities: vec![
                Box::new(Quad::new(&gl, ShaderType::BasicShader)),
                Box::new(Graph3d::new(&gl, ShaderType::Graph3dShader, 100)),
            ],
            gl: gl,
            shader_controller: shader_controller,
        }
    }

    pub fn update(&self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        update_app_state(time, width, height);
        for e in self.entities.iter() {
            e.update(time);
        }
        Ok(())
    }

    pub fn render(&mut self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let position = glm::vec3(0.0, 0.0, 0.0);
        let rotation = glm::vec3(0.0, 0.0, 0.0);
        let scale = glm::vec3(1.0, 1.0, 1.0);

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

        Self::attach_mouse_down_handler(&canvas)?;
        Self::attach_mouse_up_handler(&canvas)?;
        Self::attach_mouse_move_handler(&canvas)?;

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.enable(GL::DEPTH_TEST);
        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear_depth(1.0);

        Ok(gl)
    }

    fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let handler = move |event: web_sys::MouseEvent| {
            update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
        };

        let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
        handler.forget();

        Ok(())
    }

    fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let handler = move |event: web_sys::MouseEvent| {
            update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
        };

        let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
        handler.forget();

        Ok(())
    }

    fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let handler = move |event: web_sys::MouseEvent| {
            update_mouse_position(event.client_x() as f32, event.client_y() as f32);
        };

        let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
        handler.forget();

        Ok(())
    }
}
