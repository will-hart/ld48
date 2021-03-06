use bevy::prelude::{Entity, Res, Texture};

use crate::{colors::to_u8s, dims::Dims, entity::Particle};

pub struct Map {
    w: u32,
    h: u32,
    pub map: Vec<Option<Entity>>,
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

    /// Returns a grid around the given cell, which is true if the grid is unoccupied, false otherwise
    /// numbering is [top left, top mid, top right, mid left, mid mid, mid right, bottom left, bottom mid, bottom right]
    pub fn test_free_neighbours(&mut self, x: u32, y: u32) -> [bool; 9] {
        [
            if x > 0 && y < self.h - 1 {
                self.get(x - 1, y + 1).is_none()
            } else {
                false
            },
            //
            if y < self.h - 1 {
                self.get(x, y + 1).is_none()
            } else {
                false
            },
            //
            if x < self.w - 1 && y < self.h - 1 {
                self.get(x + 1, y + 1).is_none()
            } else {
                false
            },
            //
            if x > 0 {
                self.get(x - 1, y).is_none()
            } else {
                false
            },
            //
            false,
            //
            if x < self.w - 1 {
                self.get(x + 1, y).is_none()
            } else {
                false
            },
            //
            if x > 0 && y > 0 {
                self.get(x - 1, y - 1).is_none()
            } else {
                false
            },
            //
            if y > 0 {
                self.get(x, y - 1).is_none()
            } else {
                false
            },
            //
            if y > 0 && x < self.w - 1 {
                self.get(x + 1, y - 1).is_none()
            } else {
                false
            },
        ]
    }

    pub fn get(&mut self, x: u32, y: u32) -> Option<Entity> {
        if y >= self.h || x >= self.w {
            // safety checks failed
            return None;
        }
        let grid_idx = self.to_grid(x, y);
        self.map[grid_idx]
    }

    /// spawns a new entity at the given position
    pub fn spawn_entity(&mut self, dims: &Res<Dims>, particle: Particle, entity: Entity) {
        let x = particle.pos.x.floor() as u32;
        let y = particle.pos.y.floor() as u32;
        let idx = self.to_grid(x, y);
        let cols = to_u8s(particle.color);

        // add to the map
        self.map[idx] = Some(entity);

        // add to the texture
        self.set_pixel(dims, x, y, cols);
    }

    /// Sets RGB (NOT ALPHA!!) on the given
    pub fn set_pixel(&mut self, dims: &Res<Dims>, x: u32, y: u32, col: [u8; 3]) {
        for (pixel, idx) in dims.to_range_enumerate(x, y) {
            // set rgb
            self.raw_texture.data[idx] = col[pixel];
        }
    }

    /// gets the current alpha channel value at the given location
    pub fn get_alpha(&mut self, dims: &Res<Dims>, x: u32, y: u32) -> u8 {
        let alpha_channel = dims.to_alpha_index(x, y);
        self.raw_texture.data[alpha_channel]
    }

    /// sets the alpha channel
    pub fn set_alpha(&mut self, dims: &Res<Dims>, x: u32, y: u32, alpha: u8) {
        let alpha_channel = dims.to_alpha_index(x, y);
        self.raw_texture.data[alpha_channel] = alpha;
    }

    /// moves an entity to a new position, swapping the colours
    pub fn move_entity(
        &mut self,
        dims: &Res<Dims>,
        prev: (u32, u32),
        next: (u32, u32),
        empty_colour: [u8; 3],
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

    pub fn clear(&mut self, dims: &Res<Dims>, clear_colour: &[u8; 3]) {
        // remove all entities
        let count = self.map.len();
        for n in 0..count {
            self.map[n] = None;
        }

        // set texture to the background colour
        for x in 0..dims.tex_w {
            for y in 0..dims.tex_h {
                let mut last_idx = 0;
                for (pixel, idx) in dims.to_range_enumerate(x, y) {
                    self.raw_texture.data[idx] = clear_colour[pixel];
                    last_idx = idx;
                }

                // set the alpha value to 0
                self.raw_texture.data[last_idx + 1] = 0;
            }
        }
    }

    pub fn destroy_at(&mut self, x: u32, y: u32, dims: &Res<Dims>, clear_colour: &[u8; 3]) {
        let grid_idx = self.to_grid(x, y);
        self.map[grid_idx] = None;

        for (pixel, idx) in dims.to_range_enumerate(x, y) {
            self.raw_texture.data[idx] = clear_colour[pixel];
        }
    }

    fn to_grid(&self, x: u32, y: u32) -> usize {
        ((self.h - y - 1) * self.w + x) as usize
    }
}
