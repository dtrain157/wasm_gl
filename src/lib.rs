mod gl_setup;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
        let gl = gl_setup::init_webgl_context().unwrap();
        Self { gl: gl }
    }

    pub fn update(&self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Update!");
        Ok(())
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}
