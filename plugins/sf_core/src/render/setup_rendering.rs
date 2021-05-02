use bevy::{
    prelude::*,
    render::render_graph::{base, RenderGraph, RenderResourcesNode},
};

use super::render_pipeline::LightSource;

pub fn setup_rendering(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node(
        "light_source",
        RenderResourcesNode::<LightSource>::new(true),
    );

    render_graph
        .add_node_edge("light_source", base::node::MAIN_PASS)
        .unwrap();
}
