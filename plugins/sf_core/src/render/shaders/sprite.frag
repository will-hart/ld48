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
layout(set = 3, binding = 0) uniform LightSource_light_x {
    float light_x;
};
layout(set = 3, binding = 1) uniform LightSource_light_y {
    float light_y;
};
layout(set = 3, binding = 2) uniform LightSource_light_strength {
    float light_strength;
};

void main() {
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    vec4 tex_sample = texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);


    // color = vec4(step(0.5, tex_sample.a), 0.0, 0.0, 1.0);
    color *= tex_sample;

# endif
    o_Target = color;
}
