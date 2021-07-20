#version 450
in vec2 fragCoord;
out vec4 fragColor;
uniform vec4 iMouse;
uniform float iTime;
uniform float iTimeDelta;
uniform int iFrame;
uniform vec4 iDate;
uniform vec2 iResolution;
void main() {
vec2 uv = (fragCoord)/((iResolution).xy);
fragColor += ((texture(iChannel0, uv))*(float(iFrame)))*(0.0008);
}
