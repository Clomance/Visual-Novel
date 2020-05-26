#version 120
attribute  vec3 p;

varying float alpha;

void main() {
    alpha = p.z;
    gl_Position = vec4(p.x, p.y, 0.0, 1.0);
}