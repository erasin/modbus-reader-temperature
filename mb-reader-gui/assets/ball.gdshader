shader_type canvas_item;

uniform vec4 color : source_color;

void fragment() {
        vec4 pixel_color = texture(TEXTURE, UV);

        float brighthness = (pixel_color.r + pixel_color.g + pixel_color.b) / 3.0;
        vec4 grey_scale = vec4(brighthness, brighthness, brighthness, pixel_color.a);

        COLOR = grey_scale * color;
}
