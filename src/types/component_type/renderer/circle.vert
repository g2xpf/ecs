#version 450

in vec2 coord;

uniform float radius;
uniform vec2 r;
uniform float angle;
uniform float scale;

vec2 rot(vec2 v){
    return mat2(cos(angle), sin(angle), -sin(angle), cos(angle)) * v;
}

void main(){
    gl_Position = vec4(scale * (r + rot(radius * coord)), 0.0, 1.0);
}
