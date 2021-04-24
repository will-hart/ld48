use bevy::prelude::*;

pub struct Colors {
    pub menu: Color,
    pub sand: Color,
    pub walls: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            menu: Color::rgb(10. / 255., 12. / 255., 15. / 255.),
            walls: Color::rgb(18. / 255., 12.25 / 255., 6.25 / 255.),
            sand: Color::rgb(218. / 255., 157. / 255., 82. / 255.),
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
