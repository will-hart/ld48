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
            walls: Color::rgb(9. / 255., 9. / 255., 11. / 255.),
            sand: Color::rgb(0.1, 1., 0.3),
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
