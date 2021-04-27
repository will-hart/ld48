use bevy::render::shader::ShaderStage;
use bevy::sprite::build_sprite_pipeline;
use bevy::{prelude::*, render::pipeline::PipelineDescriptor};

pub fn get_custom_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    let mut base_pipeline = build_sprite_pipeline(shaders);

    base_pipeline.shader_stages.fragment = Some(shaders.add(Shader::from_glsl(
        ShaderStage::Fragment,
        include_str!("shaders/sprite.frag"),
    )));

    base_pipeline
}
