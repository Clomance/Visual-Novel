#version 140

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D tex;
uniform vec4 colour_filter;

void main() {
    color = colour_filter * texture(tex, v_tex_coords);
}