#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aCol;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 col;

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    col = vec4(1.0f, 0.5f, 0.2f, 1.0f);
//    col = vec4(aCol,1.0);
}