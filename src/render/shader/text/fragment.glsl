#version 300 es
precision mediump float;

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D tex[1];

void main() {
    a_color = vec4(v_color.rgb, texture(tex[0], v_uv).r);
    if (a_color.a <= 0.0) {
        discard;
    }
}