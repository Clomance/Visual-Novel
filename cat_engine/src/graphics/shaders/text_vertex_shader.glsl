#version 140

in vec3 p;

out float alpha;

void main() {
    alpha = p.z;
    gl_Position = vec4(p.x, p.y, 0.0, 1.0);
}