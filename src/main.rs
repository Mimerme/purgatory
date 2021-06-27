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
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages, ShaderSource},
    },
};

pub mod debug_systems;
pub mod download;

/// This example shows how to animate a shader, by passing the global `time.seconds_since_startup()`
/// via a 'TimeComponent` to the shader.
pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_startup_system(setup.system())
        .add_system(animate_shader.system())
        .add_system(debug_systems::print_asset_events.system())
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

const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_Uv = Vertex_Uv;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderToyUniform_value {
    float time;
};

layout(set = 2, binding = 1) uniform ShaderToyUniform_another_value {
    float another_time;
};

void main() {
    float speed = 1.0;
    float translation = sin(time * speed);
    float percentage = 1.0;
    float threshold = v_Uv.x + translation * percentage;

    vec3 red = vec3(1., 0., 0.);
    vec3 blue = vec3(0., 0., 1.);
    vec3 mixed = mix(red, blue, threshold);

    o_Target = vec4(mixed, 1.0);
}
"#;

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
    let vertex_shader: Handle<Shader> = asset_server.load("./shaders/demo.vert");
    let fragment_shader: Handle<Shader> = asset_server.load("./shaders/demo.frag");

    bevy::log::info!("Shader {:?}", shaders.get(vertex_shader));

    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
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
/// In this system we query for the `TimeComponent` and global `Time` resource, and set
/// `time.seconds_since_startup()` as the `value` of the `TimeComponent`. This value will be
/// accessed by the fragment shader and used to animate the shader.
fn animate_shader(
    mut mouse_motion: EventReader<CursorMoved>,
    time: Res<Time>,
    mut query: Query<(&mut ShaderToyUniform)>,
) {
    let (mut time_uniform) = query.single_mut().unwrap();
    time_uniform.value = time.seconds_since_startup() as f32;
    time_uniform.another_value = time.seconds_since_startup() as f32;

    match mouse_motion.iter().last() {
        Some(x) => {
            // bevy::log::info!("{:?}", x);
            // .value = x.position.x;
            // mouse_y.value = x.position.y;
        }
        None => {}
    }
}

#[test]
fn test_download_shader() {
    println!("Download");
}
