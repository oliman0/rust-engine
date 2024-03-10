#version 430 core
out vec4 FragColor;

in vec4 lcolour;

void main()
{
    FragColor = lcolour;
}