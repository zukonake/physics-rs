#version 140
precision lowp float;

in vec2 position;
in vec4 color;
out vec4 vertex_color;

uniform vec2 screen_size;
uniform mat3 matrix;

//transfers the ndc (from [-1, -1] to [1, 1]) to (from [0, 0] to [1, 1])
mat3 convert_ndc = mat3
(
     2.0,  0.0, 0.0,
     0.0, -2.0, 0.0,
    -1.0,  1.0, 1.0
);

vec2 transformed_position = (convert_ndc * matrix * vec3(position / screen_size, 1.0)).xy;

void main()
{
    vertex_color = color;
    gl_Position = vec4(transformed_position, 0.0, 1.0);
}
