use bevy::render::shader::ShaderStage;
use bevy::sprite::build_sprite_pipeline;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{pipeline::PipelineDescriptor, renderer::RenderResources},
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "ddedb840-80d2-4c53-93b4-6cc6f642684b"]
pub struct LightSource {
    pub pos: Vec2,
    pub dims: Vec2,
    pub strength: f32,
}

pub fn get_custom_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    let mut base_pipeline = build_sprite_pipeline(shaders);

    base_pipeline.shader_stages.fragment = Some(shaders.add(Shader::from_glsl(
        ShaderStage::Fragment,
        include_str!("shaders/sprite_web.frag"),
    )));

    base_pipeline.shader_stages.vertex = shaders.add(Shader::from_glsl(
        ShaderStage::Vertex,
        include_str!("shaders/sprite_web.vert"),
    ));

    base_pipeline
}
