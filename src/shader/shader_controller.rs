use super::shader::Shader;
use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::*;

static BASIC_SHADER_VERTEX: &'static str = include_str!("../assets/shaders/basic_vertex.glsl");
static BASIC_SHADER_FRAGMENT: &'static str = include_str!("../assets/shaders/basic_fragment.glsl");

static GRAPH3D_SHADER_VERTEX: &'static str = include_str!("../assets/shaders/graph3d_vertex.glsl");
static GRAPH3D_SHADER_FRAGMENT: &'static str = include_str!("../assets/shaders/graph3d_fragment.glsl");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderType {
    BasicShader,
    Graph3dShader,
}

pub struct ShaderController {
    shaders: HashMap<ShaderType, Shader>,
    active_shader: RefCell<ShaderType>,
}

impl ShaderController {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let mut shaders = HashMap::new();

        let basic_shader = Shader::new(&gl, BASIC_SHADER_VERTEX, BASIC_SHADER_FRAGMENT).unwrap();
        let graph3d_shader = Shader::new(&gl, GRAPH3D_SHADER_VERTEX, GRAPH3D_SHADER_FRAGMENT).unwrap();

        let active_shader = RefCell::new(ShaderType::BasicShader);
        gl.use_program(Some(&basic_shader.get_program()));

        shaders.insert(ShaderType::BasicShader, basic_shader);
        shaders.insert(ShaderType::Graph3dShader, graph3d_shader);

        Self { shaders, active_shader }
    }

    pub fn get_shader(&self, shader_type: &ShaderType) -> Option<&Shader> {
        self.shaders.get(shader_type)
    }

    pub fn use_shader(&self, gl: &WebGlRenderingContext, shader_type: ShaderType) {
        if *self.active_shader.borrow() == shader_type {
            return;
        }

        gl.use_program(Some(&self.shaders.get(&shader_type).unwrap().get_program()));
        *self.active_shader.borrow_mut() = shader_type;
    }
}
