#version 110

attribute vec2 position;

uniform vec3 color;
uniform mat4 projection;
uniform mat4 model;

varying vec3 fcolor;

void main() {
    gl_Position = projection * model * vec4(position, 0.0, 1.0);
    fcolor = color;
}
