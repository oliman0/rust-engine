#version 330 core
    in vec4 vCol;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D tex;
    uniform bool usingTexture;

    void main() {
      if (usingTexture)
	    {
		    FragColor = vCol * texture(tex, texCoord);
	    }
	    else if (!usingTexture)
	    {
		    FragColor = vCol;
	    }
    }
