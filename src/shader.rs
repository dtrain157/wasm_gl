use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Shader {
    vertex: WebGlShader,
    fragment: WebGlShader,
}

impl Shader {
    pub fn new(gl: &WebGlRenderingContext, vertex_path: &str, fragment_path: &str) {
        let vertex_code = std::fs::read_to_string(vertex_path)?;
        let fragment_code = std::fs::read_to_string(fragment_path)?;

        Self {
            vertex: Self::compile_shader(gl, GL::VERTEX_SHADER, &vertex_code)?,
            fragment: Self::compile_shader(gl, GL::FRAGMENT_SHADER, &fragment_code)?,
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
