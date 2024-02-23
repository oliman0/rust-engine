#version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 tex;
  
    out vec4 vCol;
    out vec2 texCoord;

    uniform mat4 projection;
    uniform mat4 model;
    uniform mat4 view;
    uniform vec4 colour;
    uniform bool useView;

    void main() {
      if (useView) { gl_Position = projection * view * model * vec4(aPos.x, aPos.y, aPos.z, 1.0); }
      else { gl_Position = projection * model * vec4(aPos.x, aPos.y, aPos.z, 1.0); }
      
      vCol = colour;

      texCoord = tex;
    }
