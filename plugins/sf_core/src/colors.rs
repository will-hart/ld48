use bevy::prelude::*;

pub struct Colors {
    pub menu: Color,
    pub sand: Color,
    pub walls: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            menu: Color::rgb(41. / 255., 48. / 255., 56. / 255.),
            walls: Color::rgb(72. / 255., 49. / 255., 25. / 255.),
            sand: Color::rgb(218. / 255., 157. / 255., 82. / 255.),
        }
    }
}

pub fn to_u8s(color: Color) -> [u8; 4] {
    [
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
        255,
    ]
}
