use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Shader {
    program: WebGlProgram,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>,
}

impl Shader {
    pub fn new(gl: &WebGlRenderingContext, vertex_code: &str, fragment_code: &str) -> Result<Self, String> {
        let program = gl.create_program().ok_or_else(|| String::from("Error creating program")).unwrap();
        let vertex_shader = Self::compile_shader(gl, GL::VERTEX_SHADER, vertex_code).unwrap();
        gl.attach_shader(&program, &vertex_shader);
        let fragment_shader = Self::compile_shader(gl, GL::FRAGMENT_SHADER, fragment_code).unwrap();
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);
        if gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(Self {
                program: program,
                uniforms: RefCell::new(HashMap::new()),
            })
        } else {
            Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Unable to create GL program")))
        }
    }

    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn get_uniform_location(&self, gl: &WebGlRenderingContext, uniform_name: &str) -> Option<WebGlUniformLocation> {
        let mut uniforms = self.uniforms.borrow_mut();
        if uniforms.get(uniform_name).is_none() {
            uniforms.insert(
                uniform_name.to_string(),
                gl.get_uniform_location(&self.program, uniform_name)
                    .expect(&format!("Uniform '{}' not found", uniform_name)),
            );
        }
        Some(uniforms.get(uniform_name).expect("loc").clone())
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
