#version 450
in vec2 fragCoord;
out vec4 fragColor;
uniform vec4 iMouse;
uniform float iTime;
uniform float iTimeDelta;
uniform int iFrame;
uniform vec4 iDate;
uniform vec2 iResolution;
#define PI 3.141592
#define TAU 6.283185
#define rot(a) mat2(cos(a),sin(a),-sin(a),cos(a))
#define BPM (95./60.)
#define dt(sp, off) fract((iTime+off)*sp)
#define animCirc(speed, off) easeInOutCirc(time(speed,off))
#define bright_calc(color) (0.2126*color.x + 0.7152*color.g + 0.0722*color.b)
vec4 texNoise(vec2 uv) {
float f = 0.;
f += ((texture(iChannel0, (uv)*(0.125))).r)*(0.5);
f += ((texture(iChannel0, (uv)*(0.25))).r)*(0.25);
f += ((texture(iChannel0, (uv)*(0.5))).r)*(0.125);
f += ((texture(iChannel0, (uv)*(1.))).r)*(0.125);
f = pow(f, 1.2);
return vec4(((f)*(0.45))+(0.05));
}
float background(vec2 uv) {
float rep = 256.;
(uv).x += (dt((1.)/((rep)*(1.5)), 0.))*(rep);
return min((length((uv)-(0.5)))-((0.5)-((texNoise((uv)*(vec2(0.1, 0.5)))).x)), min((length((uv)+(vec2(0.9, 0.5))))-((0.8)-((texNoise((uv)*(vec2(0.1, 0.5)))).y)), (length((uv)+(vec2(-(0.1), 0.2))))-((0.4)-((texNoise((uv)*(vec2(0.1, 0.3)))).x))));
}
float foreground(vec2 uv) {
float rep = 256.;
(uv).x -= (dt((1.)/((rep)*(1.5)), 0.))*(rep);
return min((length((uv)-(0.5)))-((0.5)-((texNoise((uv)*(vec2(0.1, 0.5)))).x)), min((length((uv)+(vec2(0.9, 0.5))))-((0.8)-((texNoise((uv)*(vec2(0.1, 0.5)))).y)), (length((uv)+(vec2(-(0.1), 0.2))))-((0.4)-((texNoise((uv)*(vec2(0.1, 0.3)))).x))));
}
vec3 palette(float t, vec3 d) {
return (vec3(0.7))+((vec3(0.4))*(cos((TAU)*(((vec3(1.))*(t))+(d)))));
}
void main() {
vec2 uv = (((2.)*(fragCoord))-((iResolution).xy))/((iResolution).y);
vec2 backuv = vec2((((uv).x)*((texNoise((uv)*(0.2))).x))*(0.6), (uv).y);
vec2 foreuv = (backuv)*(rot((PI)/(4.)));
vec3 col1 = clamp(palette(background(backuv), vec3(0.9, 0.7, 0.5)), 0., 1.);
vec3 col2 = clamp(palette(foreground(foreuv), vec3(0.7, 0.8, 0.4)), 0., 1.);
float b1 = bright_calc(col1);
float b2 = bright_calc(col2);
vec3 col = (b1)<(b2) ? col2 : col1;
fragColor = vec4(sqrt(col), 1.);
}
