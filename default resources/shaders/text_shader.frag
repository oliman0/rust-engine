#version 330 core
    in vec4 vCol;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D tex;
    uniform bool usingTexture;

    void main() {
		  FragColor = (vec4(vCol.x, vCol.y, vCol.z, 0.0) + texture(tex, texCoord)) * vec4(1.0, 1.0, 1.0, vCol.w);
    }
