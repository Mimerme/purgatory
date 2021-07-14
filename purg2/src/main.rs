use macroquad::prelude::*;
use std::{thread, time};


enum Uniform {
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    Float(f32),
}

#[macroquad::main("Quadtoy")]
async fn main() {
    let mut fragment_shader = DEFAULT_FRAGMENT_SHADER.to_string();
    let mut vertex_shader = DEFAULT_VERTEX_SHADER.to_string();
    let mut uniforms : Vec<(String, UniformType)> = vec![
        ("iTime".to_string(), UniformType::Float1),
        ("iTimeDelta".to_string(), UniformType::Float1),
        ("iFrame".to_string(), UniformType::Int1),
        ("iDate".to_string(), UniformType::Float4),
        ("iMouse".to_string(), UniformType::Float4),
        ("iResolution".to_string(), UniformType::Float2),
    ];

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let mut material = load_material(
        &vertex_shader,
        &fragment_shader,
        MaterialParams {
            pipeline_params,
            uniforms: uniforms.clone(),
            ..Default::default()
        },
    )
    .unwrap();

    let mut camera = Camera3D {
        position: vec3(-15., 15., -5.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 5., -5.),
        ..Default::default()
    };

    let mut time : f32 = 1.0;
    let mut timeDelta : f32 = 0.0;
    let mut mouse : [f32; 4] = [0.0, 0.0,0.0,0.0];
    let mut date : [f32; 4] = [0.0, 0.0,0.0,0.0];
    let mut resolution : [f32;2] = [0.0, 0.0];

    loop {
        clear_background(WHITE);
        gl_use_material(material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), GREEN);

        set_camera(&camera);

        // Update the uniforms on every frame here
        material.set_uniform("iTime", time);
        material.set_uniform("iTimeDelta", timeDelta);
        material.set_uniform("iMouse",mouse);
        material.set_uniform("iDate", date);
        material.set_uniform("iResolution", resolution);

        set_default_camera();

        time += 1.0;
        resolution = [screen_width(), screen_height()];

        thread::sleep(time::Duration::from_millis(15));
        next_frame().await
    }

}

const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 450
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float iTime;
out vec4 fragColor;

void main() {
    //gl_FragColor = texture2D(Texture, uv);
    fragColor = vec4(iTime / 255.0, 0.0, 0.0, 1.0);
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 450
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform float iTime;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";

