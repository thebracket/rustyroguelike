#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;

void main()
{
    vec4 fg = texture(texture1, TexCoord);
    fg *= vec4(ourColor, 1.f);
	FragColor = fg;
    //FragColor = vec4(1.f, 1.f, 1.f, 1.f);
}
