#version 150 core

in vec2 position;
in vec2 texcoord;

uniform mat4 projection;
uniform mat4 model;

out vec2 texcoord_out;

void main(){
    gl_Position = projection * model * vec4(position, 0.0, 1.0);
    //gl_Position = v;
    texcoord_out = texcoord;
}
