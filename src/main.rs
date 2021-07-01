use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::CursorMoved,
};
use bevy::{
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};
use chrono::{Datelike, NaiveDateTime, Timelike};

pub mod debug_systems;
pub mod download;

// Some bevy examples for le newbs
// https://github.com/bevyengine/bevy/blob/main/examples/shader/hot_shader_reloading.rs

#[derive(Default)]
struct CurrentFrame(i32);

/// This example shows how to animate a shader, by passing the global `time.seconds_since_startup()`
/// via a 'TimeComponent` to the shader.
pub fn main() {
    App::build()
        .insert_resource(CurrentFrame(0))
        .add_plugins(DefaultPlugins)
        // .add_plugin(InputPlugin)
        .add_asset::<Shader>()
        .add_asset::<ShadertoyChannels>()
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(animate_shader.system())
        // .add_system(debug_systems::print_asset_events.system())
        // .add_system(mouse_click_system.system())
        // // .add_system(debug_systems::print_mouse_events.system())
        // .add_system(check_for_nshader_updates())
        .run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
struct ShaderToyUniform {
    resolution: Vec2,
    time: f32,
    time_delta: f32,
    frame: i32,
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

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "93fb26fc-6c05-489b-9029-601edf703b6b"]
struct ShadertoyChannels {
    pub channel0: Option<Handle<Texture>>,
    pub channel1: Option<Handle<Texture>>,
    pub channel2: Option<Handle<Texture>>,
    pub channel3: Option<Handle<Texture>>,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    _shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut channels: ResMut<Assets<ShadertoyChannels>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    bevy::log::info!("Creating render pipeline");
    asset_server.watch_for_changes().unwrap();

    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load("shaders/demo.vert"),
        fragment: Some(asset_server.load("shaders/demo.frag")),
    }));

    channels.add(ShadertoyChannels {
        channel0: Some(asset_server.load("noise.png")),
        channel1: Some(asset_server.load("noise.png")),
        channel2: Some(asset_server.load("noise.png")),
        channel3: Some(asset_server.load("noise.png")),
    });

    // Add a `RenderResourcesNode` to our `RenderGraph`. This will bind `TimeComponent` to our
    // shader.
    render_graph.add_system_node(
        "time_uniform",
        RenderResourcesNode::<ShaderToyUniform>::new(true),
    );

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

/// In this system we query for the `TimeComponent` and global `Time` resource, and set
/// `time.seconds_since_startup()` as the `value` of the `TimeComponent`. This value will be
/// accessed by the fragment shader and used to animate the shader.
fn animate_shader(
    _mouse_motion: EventReader<CursorMoved>,
    mut current_frame: ResMut<CurrentFrame>,
    mouse_button: Res<Input<MouseButton>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut channels: ResMut<Assets<Texture>>,

    mut query: Query<&mut ShaderToyUniform>,
) {
    let window = windows.iter().last().unwrap();
    let cursor_pos = if let Some(pos) = windows.iter().last().unwrap().cursor_position() {
        pos
    } else {
        Vec2::new(0.0, 0.0)
    };

    // bevy::log::info!("Current Frame: {:?}",current_frame.0);

    // Set the animated variables here
    let mut time_uniform = query.single_mut().unwrap();
    time_uniform.time = time.seconds_since_startup() as f32;
    time_uniform.frame = current_frame.0;
    time_uniform.time_delta = time.delta_seconds();

    let now = chrono::offset::Local::now().naive_local();
    now.year();
    let date = now.date();
    let time = now.time();
    let seconds = time.hour() * (60 * 60) * time.minute() * 60 * time.second();

    time_uniform.date = Vec4::new(
        date.year() as f32,
        date.month() as f32,
        date.month() as f32,
        seconds as f32,
    );
    time_uniform.resolution = Vec2::new(window.width(), window.height());

    let left = if mouse_button.pressed(MouseButton::Left) {
        1.0
    } else {
        0.0
    };
    let right = if mouse_button.pressed(MouseButton::Right) {
        1.0
    } else {
        0.0
    };

    time_uniform.mouse = Vec4::new(cursor_pos.x, cursor_pos.y, left, right);
    current_frame.0 += 1;
}
