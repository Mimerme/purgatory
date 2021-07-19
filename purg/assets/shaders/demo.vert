#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 3) in vec2 Vertex_Screen;

layout(location = 0) out vec3 v_Pos;
layout(location = 1) out vec3 v_Norm;
layout(location = 2) out vec2 v_Uv;
layout(location = 3) out vec2 v_Screen;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    // gl_Position = vec4(Vertex_Uv.x, Vertex_Uv.y, 0.0, 1.0);
    v_Uv = Vertex_Uv;
    v_Pos = Vertex_Position;
    v_Norm = Vertex_Normal;
    v_Screen = Vertex_Screen;
}
