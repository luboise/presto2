#version 430

layout(location = 0) in vec3 a_vertexPosition;
layout(location = 1) in vec3 a_colour;
layout(location = 2) in vec3 a_normal;
layout(location = 3) in vec2 a_texcoords;

layout(location = 0) out vec4 colour;
layout(location = 1) out vec3 normal;
layout(location = 2) out vec2 tex_coords;

layout(std140, binding = 0) uniform GlobalUniforms {
    mat4 view;
    mat4 projection;
};

layout(std140, binding = 1) uniform ObjectUniforms {
    mat4 model;
};

layout(std140, binding = 2) uniform PBROptions {
    vec4 u_baseColour;
    float u_metallic;
    float u_roughness;
};

void main() {
    gl_Position = projection * view * model * vec4(a_vertexPosition, 1.0);
    colour = vec4(a_colour, 1.0);
    tex_coords = a_texcoords;

    mat3 rotationMatrix = transpose(inverse(mat3(model)));
    normal = normalize(rotationMatrix * a_normal);
}
