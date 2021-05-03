#version 300 es

precision highp float;

in vec2 v_Uv;

out vec4 o_Target;

layout(std140) uniform ColorMaterial_color {  // set = 1, binding = 0
    vec4 Color;
};

# ifdef COLORMATERIAL_TEXTURE
uniform sampler2D ColorMaterial_texture;  // set = 1, binding = 1
# endif


// light source
uniform LightSource_pos { // set = 3, binding = 0
    vec2 light_pos;
};
uniform LightSource_dims { // set = 3, binding = 1
    vec2 grid_dims;
};
uniform LightSource_strength { // set = 3, binding = 2
    float light_strength;
};

void main() {
    vec4 color = Color;
#ifdef COLORMATERIAL_TEXTURE
    color *= texture(
        ColorMaterial_texture,
        v_Uv
    );

    // now calculate the correct alpha based on the distance to the light source
    // need to do this in grid coordinates, not uv coordinates as these would result in
    // a non-circular light
    // 
    // first calculate the grid coordinates of the UVs. Pos should be in grid coords already
    float grid_x = grid_dims.x * v_Uv.x;
    float grid_y = grid_dims.y * v_Uv.y;

    float dx = grid_x - light_pos.x;
    float dy = grid_y - light_pos.y;

    color.a = 1.0 - clamp(
        (dx * dx + dy * dy) / (light_strength * light_strength), 
        0.0, 
        1.0
    );
#endif

    o_Target = color;
}
