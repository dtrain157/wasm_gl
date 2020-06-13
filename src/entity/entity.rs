use crate::shader::shader_controller::ShaderController;
use nalgebra_glm as glm;
use web_sys::*;

pub trait Entity {
    fn render(&self, gl: &WebGlRenderingContext, shader_controller: &ShaderController, position: &glm::Vec3, rotation: &glm::Vec3, scale: &glm::Vec3);
    fn update(&self, _time: f32);
}
