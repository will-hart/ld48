use bevy::prelude::{Color, Vec2};

#[derive(Debug, Copy, Clone)]
pub enum ParticleType {
    Obstacle,
    // Trap,
    Sand,
    Liquid,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub particle_type: ParticleType,
    pub next_update: f64,
    pub is_left_first: bool,
}

impl Particle {
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

pub struct Spawner {
    pub pos: (u32, u32),
    pub spawn_limit: u32,
    pub spawn_delay: f64,
    pub initial_vel: Vec2,
    pub color: Color,
    pub next_spawn: f64,
    pub particle_type: ParticleType,
}

pub struct Sink {
    pub pos: (u32, u32),
    pub sink_rate: f64,
    pub next_sink: f64,
    pub sink_limit: u32,
}
