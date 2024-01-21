#version 330 core
    in vec4 vCol;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D tex;
    uniform bool usingTexture;

    void main() {
		  FragColor = vCol + texture(tex, texCoord);
    }
