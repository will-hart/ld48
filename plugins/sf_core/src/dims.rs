use std::{iter::Enumerate, ops::Range};

use bevy::math::Vec2;

#[derive(Copy, Clone)]
pub struct Dims {
    pub tex_w: u32,
    pub tex_h: u32,
    pub tex_stride: u32,
    pub win_w: u32,
    pub win_h: u32,
}

impl From<(u32, u32, u32)> for Dims {
    fn from(input: (u32, u32, u32)) -> Self {
        Dims {
            tex_w: input.0 / input.2,
            tex_h: input.1 / input.2,
            tex_stride: input.2,
            win_w: input.0,
            win_h: input.1,
        }
    }
}

impl Dims {
    pub fn to_tex_index(&self, x: u32, y: u32) -> usize {
        ((x + (self.tex_h - y - 1) * self.tex_w) * 4) as usize
    }

    pub fn to_range(&self, x: u32, y: u32) -> Range<usize> {
        let offset = self.to_tex_index(x, y);
        offset..offset + 4
    }

    pub fn to_range_enumerate(&self, x: u32, y: u32) -> Enumerate<std::ops::Range<usize>> {
        let offset = self.to_tex_index(x, y);
        (offset..offset + 4).enumerate()
    }

    pub fn texture_values(&self) -> usize {
        (4 * self.tex_h * self.tex_w) as usize
    }

    pub fn screen_to_grid(&self, pos: Vec2) -> Vec2 {
        Vec2::new(
            self.tex_w as f32 * (pos.x / (self.win_w as f32)),
            self.tex_h as f32 * (pos.y / (self.win_h as f32)),
        )
    }
}
