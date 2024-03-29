in vec2 fragCoord;
out vec4 fragColor;
uniform vec4 iMouse;
uniform float iTimeDelta;
uniform int iFrame;
uniform vec4 iDate;
uniform vec2 iResolution;
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
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
vec2 uv = (fragCoord)/((iResolution).xy);
uv *= 4.5;
float displacement = pattern(uv);
vec4 color = vec4((displacement)*(1.2), 0.2, (displacement)*(5.), 1.);
(color).a = min(((color).r)*(0.25), 1.);
fragColor = color;
}
void main() {
vec2 uv = (fragCoord)/((iResolution).xy);
uv *= 4.5;
float displacement = pattern(uv);
vec4 color = vec4((displacement)*(1.2), 0.2, (displacement)*(5.), 1.);
(color).a = min(((color).r)*(0.25), 1.);
fragColor = color;
}
