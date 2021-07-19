use chrono::{Datelike, NaiveDateTime, Timelike};
use frame_counter::FrameCounter;
use macroquad::models::Vertex;
use macroquad::prelude::*;
use purgtwo::QuadToy;
use std::{thread, time};
use macroquad::math::{Vec3, Vec2};

enum Uniform {
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
    Float(f32),
}

const DEBUG : bool = false;

#[macroquad::main("Quadtoy")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let frag_path = &args[1];

    // let shader_base_path = args[1];
    // let vert_path = format!("{}.vert", shader_base_path);
    // let frag_path = format!("{}.frag", shader_base_path);
    //let vert_shader = std::fs::read_to_string(vert_path);
    let frag_shader = std::fs::read_to_string(frag_path).unwrap();

    let mut fragment_shader = frag_shader;
    let mut vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

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

    let mut quadtoy = QuadToy::new(material);

    let mut time: f32 = 0.0;
    let mut timeDelta: f32 = 0.0;
    let mut mouse: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut date: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut resolution: [f32; 2] = [screen_width(), screen_height()];
    let mut frame = 0;

    loop {
        let (x, y, w, h) = (0.0f32, 0.0f32, resolution[0], resolution[1]);
        quadtoy.framecounter.tick();

        clear_background(WHITE);
        gl_use_material(material);
        //draw_rectangle(0.0, 0.0, screen_width(), screen_height(), GREEN);

        quadtoy.draw();

        // Update the uniforms on every frame here
        quadtoy.update();
        set_default_camera();


        next_frame().await;
        quadtoy.framecounter.wait_until_framerate(60f64);
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

