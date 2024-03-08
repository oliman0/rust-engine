#version 430 core

out vec4 FragColour;

in VS_OUT {
    vec3 FragPos;
    vec3 Normal;
    vec2 TexCoords;
    vec4 Colour;
} fs_in;

uniform sampler2D tex;
uniform bool usingTexture;

uniform vec3 lightPos;
uniform vec3 viewPos;

void main() {
    vec4 value = vec4(0.0, 0.0, 0.0, 1.0);

    if (usingTexture) {
        value = fs_in.Colour * texture(tex, fs_in.TexCoords);
    }
    else if (!usingTexture) {
        value = fs_in.Colour;
    }

    // ambient
    float ambientStrength = 0.1;
    vec3 ambient = vec3(ambientStrength);
  	
    // diffuse 
    vec3 norm = normalize(fs_in.Normal);
    vec3 lightDir = normalize(lightPos - fs_in.FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = vec3(diff);

    FragColour = (vec4(ambient, 1.0) + vec4(diffuse, 1.0)) * value;
}
