use bevy::prelude::{Res, Texture};

use crate::{colors::to_u8s, dims::Dims, entity::WorldEntity};

pub struct Map {
    w: u32,
    h: u32,
    pub map: Vec<Option<WorldEntity>>,
    pub raw_texture: Texture,
}

impl Map {
    pub fn new(dims: Dims, raw_texture: Texture) -> Self {
        Map {
            map: vec![None; (dims.tex_w * dims.tex_h) as usize],
            raw_texture,
            w: dims.tex_w,
            h: dims.tex_h,
        }
    }

    pub fn get(&mut self, x: u32, y: u32) -> Option<WorldEntity> {
        self.map[self.to_grid(x, y)]
    }

    /// spawns a new entity at the given position
    pub fn spawn_entity(&mut self, dims: &Res<Dims>, entity: WorldEntity) {
        let x = entity.pos.x.floor() as u32;
        let y = entity.pos.y.floor() as u32;
        let idx = self.to_grid(x, y);
        let cols = to_u8s(entity.color);

        // add to the map
        self.map[idx] = Some(entity);

        // add to the texture
        for (pixel, idx) in dims.to_range_enumerate(x, y) {
            self.raw_texture.data[idx] = cols[pixel];
        }
    }

    /// moves an entity to a new position, swapping the colours
    pub fn move_entity(
        &mut self,
        dims: &Res<Dims>,
        prev: (u32, u32),
        next: (u32, u32),
        empty_colour: [u8; 4],
    ) {
        // update the entity mapping
        let old_idx = self.to_grid(prev.0, prev.1);
        let entity = self.map[old_idx];
        self.map[old_idx] = None;

        let idx = self.to_grid(next.0, next.1);
        self.map[idx] = entity;

        // update the texture, first set the old field to empty
        let old_pos = dims.to_range(prev.0, prev.1);
        let new_pos = dims.to_range(next.0, next.1);

        old_pos.zip(new_pos).for_each(|(o, n)| {
            self.raw_texture.data.swap(o, n);
        });

        for (pixel, idx) in dims.to_range_enumerate(prev.0, prev.1) {
            self.raw_texture.data[idx] = empty_colour[pixel];
        }
    }

    pub fn clear(&mut self, dims: Res<Dims>, clear_colour: &[u8; 4]) {
        // remove all entities
        let count = self.map.len();
        for n in 0..count {
            self.map[n] = None;
        }

        // set texture to the background colour
        for x in 0..dims.tex_w {
            for y in 0..dims.tex_h {
                for (pixel, idx) in dims.to_range_enumerate(x, y) {
                    self.raw_texture.data[idx] = clear_colour[pixel];
                }
            }
        }
    }

    fn to_grid(&self, x: u32, y: u32) -> usize {
        ((self.h - y - 1) * self.w + x) as usize
    }
}
