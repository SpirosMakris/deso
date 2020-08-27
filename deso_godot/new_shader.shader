shader_type canvas_item;

uniform float mult = 1.0;
uniform sampler2D palette: hint_albedo;

void fragment() {
	vec4 new_color = texture(palette, UV);
	COLOR = mix(COLOR, new_color, mult);
}