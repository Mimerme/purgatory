use chrono::Datelike;
use chrono::Timelike;
use frame_counter::FrameCounter;
use macroquad::models::Vertex;
use macroquad::prelude::*;

pub struct QuadToy {
    mat: Material,
    cam: Camera3D,
    time: f32,
    timeDelta: f32,
    mouse: [f32; 4],
    date: [f32; 4],
    pub resolution: [f32; 2],
    frame: u32,
    pub framecounter: FrameCounter,
}

const TOY_UNIFORMS: [(&'static str, UniformType); 6] = [
    ("iTime", UniformType::Float1),
    ("iTimeDelta", UniformType::Float1),
    ("iFrame", UniformType::Int1),
    ("iDate", UniformType::Float4),
    ("iMouse", UniformType::Float4),
    ("iResolution", UniformType::Float2),
];

pub fn default_material(vertex_shader : String, fragment_shader : String) -> Material {
    load_material(
        &vertex_shader,
        &fragment_shader,
        MaterialParams {
            pipeline_params: PipelineParams {
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            uniforms: vec![
                ("iTime".to_string(), UniformType::Float1),
                ("iTimeDelta".to_string(), UniformType::Float1),
                ("iFrame".to_string(), UniformType::Int1),
                ("iDate".to_string(), UniformType::Float4),
                ("iMouse".to_string(), UniformType::Float4),
                ("iResolution".to_string(), UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .unwrap()
}

impl QuadToy {
    pub fn new(material: Material) -> Self {
        QuadToy {
            mat: material,
            cam: Camera3D {
                position: vec3(-15., 15., -5.),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                ..Default::default()
            },
            time: 0.0,
            timeDelta: 0.0,
            mouse: [0.0, 0.0, 0.0, 0.0],
            date: [0.0, 0.0, 0.0, 0.0],
            resolution: [screen_width(), screen_height()],
            frame: 0,
            framecounter: FrameCounter::default(),
        }
    }

    pub fn update(&mut self) {
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

        self.mat.set_uniform("iTime", self.time);
        self.mat.set_uniform("iTimeDelta", self.timeDelta);
        self.mat.set_uniform("iMouse", self.mouse);
        self.mat.set_uniform("iDate", self.date);
        self.mat.set_uniform("iResolution", self.resolution);
        self.mat.set_uniform("iFrame", self.frame);

        self.time += self.framecounter.frame_time().as_secs_f32();
        self.timeDelta = self.framecounter.frame_time().as_secs_f32();
        self.frame += 1;
        self.resolution = [screen_width(), screen_height()];
        self.mouse = [m_pos.0, m_pos.1, l_down, r_down];
        let now = chrono::offset::Local::now().naive_local();
        let chrono_date = now.date();
        let time = now.time();
        let seconds: u128 = (time.hour() as u128)
            * (60 * 60)
            * (time.minute() as u128)
            * 60
            * (time.second() as u128);
        self.date = [
            chrono_date.year() as f32,
            chrono_date.month() as f32,
            chrono_date.day() as f32,
            seconds as f32,
        ];
    }

    pub fn draw(&self) {
        let (x, y, w, h) = (0.0f32, 0.0f32, self.resolution[0], self.resolution[1]);
        clear_background(WHITE);
        gl_use_material(self.mat);

        let shadertoy_mesh = Mesh {
            vertices: vec![
                Vertex {
                    position: Vec3::new(x, y, 0.),
                    uv: Vec2::new(0.0, 0.0),
                    color: GREEN,
                },
                Vertex {
                    position: Vec3::new(x + w, y, 0.),
                    uv: Vec2::new(1.0, 0.0),
                    color: GREEN,
                },
                Vertex {
                    position: Vec3::new(x + w, y + h, 0.),
                    uv: Vec2::new(1.0, 1.0),
                    color: GREEN,
                },
                Vertex {
                    position: Vec3::new(x, y + h, 0.),
                    uv: Vec2::new(0.0, 1.0),
                    color: GREEN,
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture: None,
        };

        draw_mesh(&shadertoy_mesh);
        set_camera(&self.cam);
    }
}
