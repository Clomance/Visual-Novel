#version 120

varying float alpha;

uniform vec4 colour;

void main() {
    gl_FragColor = vec4(colour.xyz, colour.w * alpha);
}