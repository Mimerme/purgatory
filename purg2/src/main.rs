use chrono::{Datelike, NaiveDateTime, Timelike};
use frame_counter::FrameCounter;
use macroquad::models::Vertex;
use macroquad::prelude::*;
use std::{thread, time};
use macroquad::math::{Vec3, Vec2};

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
    let mut uniforms: Vec<(String, UniformType)> = vec![
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
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let mut time: f32 = 1.0;
    let mut timeDelta: f32 = 0.0;
    let mut mouse: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut date: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut resolution: [f32; 2] = [screen_width(), screen_height()];
    let mut frame = 0;

    let mut frame_counter = FrameCounter::default();


    loop {
        let (x, y, w, h) = (0.0f32, 0.0f32, resolution[0], resolution[1]);
        frame_counter.tick();

        clear_background(WHITE);
        gl_use_material(material);
        //draw_rectangle(0.0, 0.0, screen_width(), screen_height(), GREEN);

        let shadertoy_mesh = Mesh{
            vertices: vec![
            Vertex{position: Vec3::new(x, y, 0.), uv: Vec2::new(0.0, 0.0), color: GREEN},
            Vertex{position: Vec3::new(x + w, y, 0.), uv: Vec2::new(1.0, 0.0), color: GREEN},
            Vertex{position: Vec3::new(x + w, y + h, 0.), uv: Vec2::new(1.0, 1.0), color: GREEN},
            Vertex{position: Vec3::new(x, y + h, 0.), uv: Vec2::new(0.0, 1.0), color: GREEN}],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture: None,

        };
        draw_mesh(&shadertoy_mesh);

        set_camera(&camera);

        // Update the uniforms on every frame here
        let m_pos = mouse_position();
        let l_down: f32 = if is_mouse_button_down(MouseButton::Left) {
            1.0
        } else {
            0.0
        };
        let r_down: f32 = if is_mouse_button_down(MouseButton::Right) {
            1.0
        } else {
            0.0
        };

        material.set_uniform("iTime", time);
        material.set_uniform("iTimeDelta", timeDelta);
        material.set_uniform("iMouse", mouse);
        material.set_uniform("iDate", date);
        material.set_uniform("iResolution", resolution);
        material.set_uniform("iFrame", frame);

        set_default_camera();

        time += frame_counter.frame_time().as_secs_f32();
        timeDelta = frame_counter.frame_time().as_secs_f32();
        frame += 1;
        resolution = [screen_width(), screen_height()];
        mouse = [m_pos.0, m_pos.1, l_down, r_down];
        let now = chrono::offset::Local::now().naive_local();
        let chrono_date = now.date();
        let time = now.time();
        let seconds: u128 = (time.hour() as u128)
            * (60 * 60)
            * (time.minute() as u128)
            * 60
            * (time.second() as u128);
        date = [
            chrono_date.year() as f32,
            chrono_date.month() as f32,
            chrono_date.day() as f32,
            seconds as f32,
        ];

        next_frame().await;
        frame_counter.wait_until_framerate(60f64);

        println!("fps stats - {}", frame_counter);
    }
}

const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 450
precision lowp float;

in vec2 fragCoord;

uniform sampler2D Texture;
uniform float iTime;
uniform vec2 iResolution;
out vec4 fragColor;

void main() {
    //gl_FragColor = texture2D(Texture, uv);
    //fragColor = vec4(iTime / 255.0, 0.0, 0.0, 1.0);

    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;


    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(uv,0.0, 1.0);
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 450
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;
uniform vec2 iResolution;
out vec2 fragCoord;

uniform mat4 Model;
uniform mat4 Projection;
uniform float iTime;

void main() {
    //gl_Position = Model * Projection * vec4(position, 1);
    fragCoord = position.xy;
    gl_Position = vec4(((position.xy / iResolution) * 2) - 1, 0.0, 1);
    uv = texcoord;
}
";

