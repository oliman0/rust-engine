#version 430 core

out vec4 FragColor;
  
in vec2 TexCoords;

uniform sampler2D screenTexture;

void main()
{
    //float p = 1.0 / 128.0;
    //vec2 coord =  p * floor(TexCoords / p);

    FragColor = texture(screenTexture, TexCoords);
}
