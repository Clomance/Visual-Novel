#version 120

varying vec2 v_tex_coords;


uniform sampler2D tex;
uniform vec4 colour_filter;

void main() {
    gl_FragColor = colour_filter * texture2D(tex, v_tex_coords);
}