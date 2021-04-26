use bevy::prelude::*;

pub struct Colors {
    pub background: Color,
    pub sand: Color,
    pub walls: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            background: Color::rgb(15. / 255., 10. / 255., 11. / 255.),
            walls: Color::hsla(6., 0.71, 0.19, 1.),
            sand: Color::hsla(140., 0.68, 0.55, 1.),
        }
    }
}

pub fn to_u8s(color: Color) -> [u8; 3] {
    [
        (color.r() * 255.) as u8,
        (color.g() * 255.) as u8,
        (color.b() * 255.) as u8,
    ]
}
