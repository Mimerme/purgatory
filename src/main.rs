use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
};
use bevy::{
    input::InputPlugin,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderSource, ShaderStage, ShaderStages},
    },
};

pub mod debug_systems;
pub mod download;

// Some bevy examples for le newbs
// https://github.com/bevyengine/bevy/blob/main/examples/shader/hot_shader_reloading.rs

/// This example shows how to animate a shader, by passing the global `time.seconds_since_startup()`
/// via a 'TimeComponent` to the shader.
pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // .add_plugin(InputPlugin)
        // .add_asset::<Shader>()
        .add_startup_system(setup.system())
        .add_system(animate_shader.system())
        // .add_system(debug_systems::print_asset_events.system())
        .add_system(mouse_click_system.system())
        // .add_system(debug_systems::print_mouse_events.system())
        // .add_system(check_for_shader_updates())
        .run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
struct ShaderToyUniform {
    resolution: Vec3,
    time: f32,
    time_delta: f32,
    frame: f32,
    channel_time: Vec4,
    //channel_time: [f32; 4],
    mouse: Vec4,
    date: Vec4,
    sample_rate: f32,
    //channel_resolution: [Vec3; 4],
    //TODO_sampler_XX: Option<f32>,
    value: f32,
    another_value: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    bevy::log::info!("Creating render pipeline");
    asset_server.watch_for_changes().unwrap();

    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load("shaders/demo.vert"),
        fragment: Some(asset_server.load("shaders/demo.frag")),
    }));

    // Add a `RenderResourcesNode` to our `RenderGraph`. This will bind `TimeComponent` to our
    // shader.
    render_graph.add_system_node(
        "time_uniform",
        RenderResourcesNode::<ShaderToyUniform>::new(true),
    );

    // Add a `RenderGraph` edge connecting our new "time_component" node to the main pass node. This
    // ensures that "time_component" runs before the main pass.
    render_graph
        .add_node_edge("time_uniform", base::node::MAIN_PASS)
        .unwrap();
    // render_graph
    //     .add_node_edge("mouse_x", base::node::MAIN_PASS)
    //     .unwrap();
    // render_graph
    //     .add_node_edge("mouse_y", base::node::MAIN_PASS)
    //     .unwrap();

    bevy::log::info!("Creating entities");
    // Spawn a quad and insert the `TimeComponent`.
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(13.0, 7.0)))),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ShaderToyUniform::default());

    // Spawn a camera.
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    bevy::log::info!("Finished Initialization");
}
fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

/// In this system we query for the `TimeComponent` and global `Time` resource, and set
/// `time.seconds_since_startup()` as the `value` of the `TimeComponent`. This value will be
/// accessed by the fragment shader and used to animate the shader.
fn animate_shader(
    mut mouse_motion: EventReader<CursorMoved>,
    mouse_button: Res<Input<MouseButton>>,
    time: Res<Time>,
    windows : Res<Windows>,   
    mut query: Query<(&mut ShaderToyUniform)>,
) {
    let cursor_pos = if let Some(pos) = windows.iter().last().unwrap().cursor_position() {pos} else {Vec2::new(0.0,0.0)};
    let (mut time_uniform) = query.single_mut().unwrap();
    time_uniform.value = time.seconds_since_startup() as f32;
    time_uniform.another_value = time.seconds_since_startup() as f32;

    let mut left = if mouse_button.pressed(MouseButton::Left) {
        1.0
    } else {
        0.0
    };
    let mut right = if mouse_button.pressed(MouseButton::Right) {
        1.0
    } else {
        0.0
    };
    
    time_uniform.mouse = Vec4::new(cursor_pos.x, cursor_pos.y, left, right);
}

#[test]
fn test_download_shader() {
    println!("Download");
}
