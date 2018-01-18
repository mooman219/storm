#version 330
#define M_PI   3.1415926535897932384626433832795
#define M_PI_2 1.5707963267948966192313216916398

layout(location = 0) in vec2 a_pos;
layout(location = 1) in float a_rot;
layout(location = 2) in vec4 a_color;
out vec4 v_color;

void main() {
    // Rotation
    float s = sin(radians(a_rot));
    float c = s - M_PI_2;

    vec2 pos = a_pos;
    pos.x = a_pos.x * c - a_pos.y * s;
    pos.y = a_pos.y * c + a_pos.x * s;

    // Finish
    gl_Position = vec4(pos, 0.0, 1.0);
    v_color = a_color;
}
