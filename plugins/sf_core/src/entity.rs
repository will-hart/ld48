use bevy::prelude::{Color, Vec2};

#[derive(Clone, Copy)]
pub struct WorldEntity {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub is_static: bool,
}

impl WorldEntity {
    pub fn move_entity(&mut self) {
        self.pos += self.vel;
    }
}
