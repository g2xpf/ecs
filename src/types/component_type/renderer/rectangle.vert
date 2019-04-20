
#version 450 
in vec2 coord;

uniform vec2 r;
uniform float w;
uniform float h;
uniform float angle;

vec2 rot(vec2 v){
    return mat2(cos(angle), sin(angle), -sin(angle), cos(angle)) * v;
}

void main(){
    gl_Position = vec4(r + rot(vec2(w, h) * coord), 0.0, 1.0);
}
