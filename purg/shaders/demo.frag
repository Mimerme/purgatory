#version 450

in vec2 fragCoord;
out vec4 fragColor;

uniform sampler2D Texture;
uniform float iTime;
uniform vec2 iResolution;

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