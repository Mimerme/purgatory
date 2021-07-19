#version 450

// layout (location = 0) in vec2 fragCoord;
layout(location = 0) in vec2 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec3 Vertex_Uv;
layout(location = 3) in vec2 Vertex_Screen;


layout (location = 0) out vec4 fragColor;
layout (set = 2, binding = 0) uniform ShaderToyUniform_time {float iTime;

};
layout (set = 2, binding = 1) uniform ShaderToyUniform_mouse {vec4 iMouse;

};
layout (set = 2, binding = 2) uniform ShaderToyUniform_time_delta {float iTimeDelta;

};
layout (set = 2, binding = 3) uniform ShaderToyUniform_frame {int iFrame;

};
layout (set = 2, binding = 4) uniform ShaderToyUniform_date {vec4 iDate;

};
layout (set = 2, binding = 5) uniform ShaderToyUniform_resolution {vec2 iResolution;

};
float rand(vec2 n) {
return fract((sin(dot(n, vec2(12.9898, 4.1414))))*(43758.547));
}
float noise(vec2 p) {
vec2 ip = floor(p);
vec2 u = fract(p);
u = ((u)*(u))*((3.)-((2.)*(u)));
float res = mix(mix(rand(ip), rand((ip)+(vec2(1., 0.))), (u).x), mix(rand((ip)+(vec2(0., 1.))), rand((ip)+(vec2(1., 1.))), (u).x), (u).y);
return (res)*(res);
}
const mat2 m2 = mat2(0.8, -(0.6), 0.6, 0.8);
float fbm(in vec2 p) {
float f = 0.;
f += (0.5)*(noise(p));
p = ((m2)*(p))*(2.02);
f += (0.25)*(noise(p));
p = ((m2)*(p))*(2.03);
f += (0.125)*(noise(p));
p = ((m2)*(p))*(2.01);
f += (0.0625)*(noise(p));
return (f)/(0.769);
}
float pattern(in vec2 p) {
vec2 q = vec2(fbm((p)+(vec2(0., 0.))));
vec2 r = vec2(fbm(((p)+((4.)*(q)))+(vec2(1.7, 9.2))));
r += (iTime)*(0.15);
return fbm((p)+((1.76)*(r)));
}

void main() {
// vec2 uv = (Vertex_Screen)/((iResolution).xy);
// uv *= 4.5;
// float displacement = pattern(uv);
// vec4 color = vec4((displacement)*(1.2), 0.2, (displacement)*(5.), 1.);
// (color).a = min(((color).r)*(0.25), 1.);
// fragColor = vec4(color.xyz, 1.0);

  // Normalized pixel coordinates (between 0 and 1)
  vec2 uv = Vertex_Screen/iResolution.xy;

  // Set R and G values based on position
  vec3 col = vec3(uv.x,uv.y,0);

  // Output to screen
  fragColor = vec4(0.0, 0.5, 0.0, 0.0);

}

void main_calibrate(){
  // Normalized pixel coordinates (between -0.5 and 0.5)
  // vec2 uv = (Vertex_Screen - iResolution.xy * 0.5)/iResolution.xy;
  // vec2 uv = (iResolution.xy * 0.5)/iResolution.xy;
  vec2 uv = vec2(0.5, 0.5);
    
  // Set R and G values based on position
  //vec3 col = vec3(uv.x,uv.y,0);
  vec3 col = vec3(uv ,0.0);

  // Output to screen
  fragColor = vec4(0.0, 0.0, 0.0 ,0.43);
}


// void main()
// {
//   // Normalized pixel coordinates (from 0 to 1)
//   vec2 uv = Vertex_Screen/iResolution.xy;

//   // Time varying pixel color
//   vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

//   // Output to screen
//   fragColor = vec4(col,1.0);
// }
