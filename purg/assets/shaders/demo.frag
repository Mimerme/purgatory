#version 450

layout(location = 0) in vec2 fragCoord;
layout(location = 0) out vec4 fragColor;

layout(set = 2, binding = 0) uniform ShaderToyUniform_time {
  float iTime;
};
     
layout(set = 2, binding = 1) uniform ShaderToyUniform_mouse {
  vec4 iMouse;
};
     
layout(set = 2, binding = 2) uniform ShaderToyUniform_time_delta {
  float iTimeDelta;
};
     
layout(set = 2, binding = 3) uniform ShaderToyUniform_frame {
  int iFrame;
};
     
layout(set = 2, binding = 4) uniform ShaderToyUniform_date {
  vec4 iDate;
};
     
layout(set = 2, binding = 5) uniform ShaderToyUniform_resolution {
  vec2 iResolution;
};


void main() {
//      float speed = 1.0;
//      float translation = sin(iTime * speed);
//      float percentage = 1.0;
//      float threshold = fragCoord.x + translation * percentage;

// //     vec3 red = vec3(iMouse.x / 2555., 0., 0.);
// //     vec3 blue = vec3(0., 0., iMouse.z);
// //     vec3 mixed = mix(red, blue, threshold);

// //     fragColor  = vec4(mixed, 1.0);
//      fragColor = vec4(translation, 1.0, 1.0, 1.0);

    fragColor = vec4(fragCoord.x, 1.0, 1.0, 1.0);
}
