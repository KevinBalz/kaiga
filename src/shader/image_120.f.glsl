#version 110

uniform sampler2D texture;

varying vec2 texcoord_out;

void main() {
    gl_FragColor  = texture2D(texture,texcoord_out);
}
