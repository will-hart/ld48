use bevy::prelude::*;

pub struct Colors {
    pub background: Color,
    pub sand: Color,
    pub walls: Color,
    pub blue_sand: Color,
    pub red_sand: Color,
}

impl Default for Colors {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        return Colors {
            background: Color::rgb(56. / 255., 33. / 255., 37. / 255.),
            walls: Color::hsla(11., 0.85, 0.56, 1.),
            sand: Color::hsla(163., 0.66, 0.42, 1.),
            blue_sand: Color::hsla(194., 0.66, 0.42, 1.),
            red_sand: Color::hsla(348., 0.66, 0.42, 1.),
        };

        #[cfg(not(target_arch = "wasm32"))]
        return Colors {
            background: Color::rgb(15. / 255., 10. / 255., 11. / 255.),
            walls: Color::hsla(6., 0.71, 0.19, 1.),
            sand: Color::hsla(140., 0.92, 0.67, 1.),
            blue_sand: Color::hsla(211., 0.92, 0.67, 1.),
            red_sand: Color::hsla(11., 0.68, 0.55, 1.),
        };
    }
}

pub fn to_u8s(color: Color) -> [u8; 3] {
    [
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
    ]
}
