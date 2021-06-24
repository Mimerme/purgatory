#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform TimeUniform_value {
    float time;
};

layout(set = 2, binding = 1) uniform MouseX_value {
    float mouse_x;
};

layout(set = 2, binding = 2) uniform MouseY_value {
    float mouse_y;
};

void main() {
    float speed = 1.0;
    float translation = sin(time * speed);
    float percentage = 1.0;
    float threshold = v_Uv.x + translation * percentage;

    vec3 red = vec3(mouse_x / 1279.0, 0., 0.);
    vec3 blue = vec3(0., 0., 1.);
    vec3 mixed = mix(red, blue, threshold);

    o_Target = vec4(mixed, 1.0);
}