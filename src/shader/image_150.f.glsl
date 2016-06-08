#version 150 core

uniform sampler2D image;

in vec2 texcoord_out;
out vec4 fragcolor;

void main() {
    fragcolor  = texture(image,texcoord_out);
}
