use bevy::prelude::{Color, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct WorldEntity {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub is_static: bool,
    pub next_update: f64,
}

impl WorldEntity {
    pub fn move_entity(&mut self) {
        self.pos += self.vel;
    }

    pub fn x(&self) -> u32 {
        self.pos.x.floor() as u32
    }

    pub fn y(&self) -> u32 {
        self.pos.y.floor() as u32
    }

    pub fn get_next_pos(&mut self) -> (u32, u32) {
        (
            (self.pos.x.floor() + self.vel.x) as u32,
            (self.pos.y.floor() + self.vel.y) as u32,
        )
    }
}
