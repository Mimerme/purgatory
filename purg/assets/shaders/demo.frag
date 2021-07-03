#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderToyUniform_time {
    float iTime;
};

layout(set = 2, binding = 1) uniform ShaderToyUniform_mouse {
    vec4 iMouse;
};

void main() {
    float speed = 1.0;
    float translation = sin(iTime * speed);
    float percentage = 1.0;
    float threshold = v_Uv.x + translation * percentage;

    vec3 red = vec3(iMouse.x / 2555., 0., 0.);
    vec3 blue = vec3(0., 0., iMouse.z);
    vec3 mixed = mix(red, blue, threshold);

    o_Target = vec4(mixed, 1.0);
}
