#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderToyUniform_value {
    float time;
};

layout(set = 2, binding = 1) uniform ShaderToyUniform_mouse {
    vec4 mouse;
};

void main() {
    float speed = 1.0;
    float translation = sin(time * speed);
    float percentage = 1.0;
    float threshold = v_Uv.x + translation * percentage;

    vec3 red = vec3(mouse.x / 2555., 0., 0.);
    vec3 blue = vec3(0., 0., mouse.z);
    vec3 mixed = mix(red, blue, threshold);

    o_Target = vec4(mixed, 1.0);
}
