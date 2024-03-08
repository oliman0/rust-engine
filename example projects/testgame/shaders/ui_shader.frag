#version 430 core
    in vec4 vCol;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D tex;
    uniform bool usingTexture;
    uniform bool addColour;

    void main() {
      if (usingTexture)
	    {
        if (addColour) { FragColor = (vec4(vCol.x, vCol.y, vCol.z, 0.0) + texture(tex, texCoord)) * vec4(1.0, 1.0, 1.0, vCol.w); }
        else { FragColor = vCol * texture(tex, texCoord); }
	    }
	    else if (!usingTexture)
	    {
		    FragColor = vCol;
	    }
    }
