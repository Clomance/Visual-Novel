#version 120

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

uniform float cos;
uniform float sin;
uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    gl_Position = vec4(
        (position.x * cos - position.y * sin) / window_center.x,
        (position.x * sin + position.y * cos) / window_center.y,
        0.0,
        1.0
    );
}