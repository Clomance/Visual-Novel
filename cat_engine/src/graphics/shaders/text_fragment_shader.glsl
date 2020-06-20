#version 140

in float alpha;
out vec4 color;

uniform vec4 colour;

void main() {
    color = vec4(colour.xyz, colour.w * alpha);
}