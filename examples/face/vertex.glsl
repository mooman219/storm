precision highp float;
precision highp int;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in uvec4 a_kind;
layout(location = 2) in vec4 a_uv;

out vec2 v_uv;
out vec4 v_color;

layout(std140) uniform vertex {
    mat4 ortho;
};

// UV Layout: xmin xmax ymin ymax
// ymin and ymax are swapped below because OpenGL reads images from bottom row to top row, but
// they're stored top to bottom on upload, so this corrects that.
vec4 uv_lut[4] = vec4[4](
    vec4(1.0, 0.0, 1.0, 0.0),  // LB
    vec4(1.0, 0.0, 0.0, 1.0),  // LT
    vec4(0.0, 1.0, 1.0, 0.0),  // RB
    vec4(0.0, 1.0, 0.0, 1.0)   // RT
);

void main() {
    vec4 uv_temp = a_uv * uv_lut[gl_VertexID];
    v_uv = vec2(uv_temp.x + uv_temp.y, uv_temp.z + uv_temp.w);
    v_color = vec4(1.0, 1.0, 1.0, 1.0);

    uint pos_packed = a_kind[gl_VertexID];
    vec3 pos_offset = vec3(
        float((pos_packed & 0xF00u) >> 8),
        float((pos_packed & 0x0F0u) >> 4),
        float((pos_packed & 0x00Fu))
    ) * 0.125;
    gl_Position = ortho * vec4(a_pos + pos_offset, 1.0);
}