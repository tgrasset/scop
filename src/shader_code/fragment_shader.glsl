#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D texture1;

uniform bool displayTexture;

void main()
{
	if (displayTexture) {
		FragColor = texture(texture1, TexCoord);
	}
	else {
		FragColor = vec4(ourColor, 1.0);
	}
}