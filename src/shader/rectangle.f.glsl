#version 140

in vec3 fcolor;

out vec4 o_color;

void main() {
    o_color = vec4(fcolor, 1.0);
}
