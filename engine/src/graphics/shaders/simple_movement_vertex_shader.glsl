#version 140

in vec2 position;

uniform vec2 movement;

void main() {
    gl_Position = vec4(position + movement, 0.0, 1.0);
}