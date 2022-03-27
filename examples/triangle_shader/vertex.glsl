precision highp float;
precision highp int;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_color;

out vec4 v_color;

layout(std140) uniform vertex {
    mat4 ortho;
};

void main() {
    gl_Position = ortho * vec4(a_pos, 1.0);
    v_color = vec4(a_color, 1.0);
}