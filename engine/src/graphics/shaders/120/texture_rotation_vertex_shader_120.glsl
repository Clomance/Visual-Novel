#version 120

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

uniform float angle;
uniform vec2 window_center;

void main() {
    v_tex_coords = tex_coords;

    float sin = sin(angle);
    float cos = cos(angle);

    float x = position.x - window_center.x;
    float y = window_center.y - position.y;

    gl_Position = vec4(
        (x * cos - y * sin) / window_center.x,
        (x * sin + y * cos) / window_center.y,
        0.0,
        1.0
    );
}