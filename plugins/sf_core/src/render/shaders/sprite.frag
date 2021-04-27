#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

// light source
layout(set = 3, binding = 0) uniform LightSource_pos {
    vec2 light_pos;
};
layout(set = 3, binding = 1) uniform LightSource_dims {
    vec2 grid_dims;
};
layout(set = 3, binding = 2) uniform LightSource_strength {
    float light_strength;
};

void main() {
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    vec4 tex_sample = texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);

// start by applying the texture
    color *= tex_sample;

    // now calculate the correct alpha based on the distance to the light source
    // need to do this in grid coordinates, not uv coordinates as these would result in
    // a non-circular light
    // 
    // first calculate the grid coordinates of the UVs. Pos should be in grid coords already
    float grid_x = grid_dims.x * v_Uv.x;
    float grid_y = grid_dims.y * v_Uv.y;

    float dx = grid_x - light_pos.x;
    float dy = grid_y - light_pos.y;

    color.a = 1.0 - clamp((dx * dx + dy * dy) / (light_strength * light_strength), 0.0, 1.0);
# endif
    o_Target = color;
}
