#version 110

varying vec3 fcolor;

void main() {
    gl_FragColor = vec4(fcolor, 1.0);
}
