#version 120

attribute vec2 position;

uniform vec2 movement;

void main() {
    gl_Position = vec4(position + movement, 0.0, 1.0);
}