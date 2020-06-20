#version 140

in vec2 position;

uniform vec2 shift;

void main() {
    gl_Position = vec4(position + shift, 0.0, 1.0);
}