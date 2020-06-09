use shader::Shader;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

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
    gl.attach_shader(&program, s.vertex);
    gl.attach_shader(&program, s.fragment);
    gl.link_program(&program);

    Ok(gl)
}
