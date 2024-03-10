#version 430 core

layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;
layout (location = 1) in vec3 aNormal;

out VS_OUT {
    vec3 FragPos;
    vec3 Normal;
    vec2 TexCoords;
    vec4 Colour;
} vs_out;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;
uniform vec4 colour;

void main() {
    gl_Position = projection * view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);

    vs_out.Colour = colour;
    vs_out.Normal = aNormal;
    vs_out.TexCoords = aTexCoords;
    vs_out.FragPos = (model * vec4(aPos, 1.0)).xyz;
}
