#version 120

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

uniform vec2 shift;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(position + shift, 0.0, 1.0);
}