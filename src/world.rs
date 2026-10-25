//! Seeded, on-demand chunk world, with persistent in-memory tile edits.
//! Coordinates: +Y is up, player position is bottom-left.
use std::collections::HashMap;
use raylib::prelude::Vector2;

use crate::{chunk::{self, Chunk, ChunkPos, CHUNK_SIZE}, entity::Entity, tiles::TileType};

pub const MAX_WORLD_WIDTH: usize = CHUNK_SIZE as usize * 16;
pub const MAX_WORLD_HEIGHT: usize = CHUNK_SIZE as usize * 8;

pub struct World {
    seed: u32,
    loaded: HashMap<ChunkPos, Chunk>,
    // Overrides include Air. An empty/missing tile must persist after unloading.
    edits: HashMap<(i32, i32), TileType>,
}

impl World {
    pub fn generate() -> Self {
        Self::generate_with_seed(1337)
    }

    /// Only initializes metadata -> chunks are generated as needed.
    pub fn generate_with_seed(seed: u32) -> Self {
        Self {
            seed,
            loaded: HashMap::new(),
            edits: HashMap::new(),
        }
    }

    pub fn loaded_chunk_count(&self) -> usize {
        self.loaded.len()
    }

    /// Keep chunks within the visible world-space rectangle + a padding
    /// measured in whole chunks. Chunks outside it are dropped.
    pub fn stream_visible_chunks(
        &mut self,
        first: Vector2,
        second: Vector2,
        padding_chunks: i32,
    ) {
        if MAX_WORLD_WIDTH == 0 || MAX_WORLD_HEIGHT == 0 {
            self.loaded.clear();
            return;
        }
        if !first.x.is_finite() || !first.y.is_finite()
            || !second.x.is_finite() || !second.y.is_finite()
        {
            return;
        }

        let padding = padding_chunks.max(0).saturating_mul(CHUNK_SIZE);
        let min_x = (first.x.min(second.x).floor() as i32).saturating_sub(padding);
        let min_y = (first.y.min(second.y).floor() as i32).saturating_sub(padding);
        let max_x = (first.x.max(second.x).ceil() as i32).saturating_add(padding);
        let max_y = (first.y.max(second.y).ceil() as i32).saturating_add(padding);
        let last_world_x = MAX_WORLD_WIDTH as i32 - 1;
        let last_world_y = MAX_WORLD_HEIGHT as i32 - 1;

        // Viewport doesn't intersect the world.
        if max_x < 0 || max_y < 0 || min_x > last_world_x || min_y > last_world_y {
            self.loaded.clear();
            return;
        }

        let cx_min = min_x.max(0).div_euclid(CHUNK_SIZE);
        let cy_min = min_y.max(0).div_euclid(CHUNK_SIZE);
        let cx_max = max_x.min(last_world_x).div_euclid(CHUNK_SIZE);
        let cy_max = max_y.min(last_world_y).div_euclid(CHUNK_SIZE);

        self.loaded.retain(|pos, _| {
            (cx_min..=cx_max).contains(&pos.x) &&
            (cy_min..=cy_max).contains(&pos.y)
        });

        for cy in cy_min..=cy_max {
            for cx in cx_min..=cx_max {
                let key = ChunkPos { x: cx, y: cy };
                if self.loaded.contains_key(&key) {
                    continue;
                }
                let mut chunk = Chunk::generate(key, self.seed, MAX_WORLD_WIDTH, MAX_WORLD_HEIGHT);

                // Restore edits when this chunk returns into view.
                let start_x = cx * CHUNK_SIZE;
                let start_y = cy * CHUNK_SIZE;
                for local_y in 0..CHUNK_SIZE {
                    for local_x in 0..CHUNK_SIZE {
                        let x = start_x + local_x;
                        let y = start_y + local_y;
                        if let Some(&tile) = self.edits.get(&(x, y)) {
                            chunk.set(x, y, tile);
                        }
                    }
                }
                self.loaded.insert(key, chunk);
            }
        }
    }

    #[inline]
    pub fn is_valid_tile(x: f32, y: f32) -> bool {
        x.is_finite() && y.is_finite()
            && x >= 0.0 && y >= 0.0
            && x < MAX_WORLD_WIDTH as f32
            && y < MAX_WORLD_HEIGHT as f32
    }

    #[inline]
    fn in_bounds(x: i32, y: i32) -> bool {
        x >= 0 && y >= 0
            && (x as usize) < MAX_WORLD_WIDTH
            && (y as usize) < MAX_WORLD_HEIGHT
    }

    /// Returns the tile by value. A missing/unloaded chunk is sampled
    /// procedurally, so that the collision simulation doesn't think unloaded terrain is Air.
    pub fn get_tile(&self, x: f32, y: f32) -> Option<TileType> {
        if !Self::is_valid_tile(x, y) {
            return None;
        }
        self.get_tile_i32(x.floor() as i32, y.floor() as i32)
    }

    pub fn get_tile_i32(&self, x: i32, y: i32) -> Option<TileType> {
        if !Self::in_bounds(x, y) {
            return None;
        }
        let pos = ChunkPos::from_tile(x, y);
        if let Some(chunk) = self.loaded.get(&pos) {
            return Some(chunk.get(x, y));
        }
        if let Some(&edit) = self.edits.get(&(x, y)) {
            return Some(edit);
        }
        Some(chunk::generated_tile(x, y, self.seed, MAX_WORLD_HEIGHT))
    }

    /// Store only deviations from seed-generated terrain. Air is an edit too.
    pub fn set_tile(&mut self, x: f32, y: f32, tile: TileType) -> bool {
        if !Self::is_valid_tile(x, y) {
            return false;
        }
        let x = x.floor() as i32;
        let y = y.floor() as i32;
        let natural = chunk::generated_tile(x, y, self.seed, MAX_WORLD_HEIGHT);
        if tile == natural {
            self.edits.remove(&(x, y));
        } else {
            self.edits.insert((x, y), tile);
        }
        if let Some(chunk) = self.loaded.get_mut(&ChunkPos::from_tile(x, y)) {
            chunk.set(x, y, tile);
        }
        true
    }

    /// Current topmost solid tile plus one. Includes player tile edits.
    pub fn surface_y(&self, x: i32) -> f32 {
        if x < 0 || x as usize >= MAX_WORLD_WIDTH {
            return 0.0;
        }
        for y in (0..MAX_WORLD_HEIGHT as i32).rev() {
            if self.get_tile_i32(x, y).is_some_and(|t| t.is_solid()) {
                return (y + 1) as f32;
            }
        }
        0.0
    }

    /// Axis-separated swept tile collisions. Keeps the existing player physics.
    pub fn simulate<T: Entity>(&mut self, entity: &mut T, delta: f32) {
        let mut pos = entity.get_position();
        let size = entity.get_size();
        let mut velocity = entity.get_velocity();
        let mut grounded = false;

        // Sweep X, preserving vertical movement when hitting a wall.
        let mut dx = velocity.x * delta;
        if dx != 0.0 {
            let min_x = pos.x.min(pos.x + dx).floor().max(0.0) as i32;
            let max_x = (pos.x + size.x).max(pos.x + size.x + dx)
                .ceil().min(MAX_WORLD_WIDTH as f32) as i32;
            let min_y = pos.y.floor().max(0.0) as i32;
            let max_y = (pos.y + size.y).ceil().min(MAX_WORLD_HEIGHT as f32) as i32;

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if !self.get_tile_i32(x, y).is_some_and(|t| t.is_solid()) {
                        continue;
                    }
                    let tx = x as f32;
                    let ty = y as f32;
                    if pos.y >= ty + 1.0 || pos.y + size.y <= ty {
                        continue;
                    }
                    if dx > 0.0 {
                        let right = pos.x + size.x;
                        if right <= tx && right + dx > tx {
                            dx = dx.min(tx - right);
                            velocity.x = 0.0;
                        }
                    } else {
                        let left = pos.x;
                        let tile_right = tx + 1.0;
                        if left >= tile_right && left + dx < tile_right {
                            dx = dx.max(tile_right - left);
                            velocity.x = 0.0;
                        }
                    }
                }
            }
            pos.x += dx;
        }

        // Sweep Y using the X result, so the entity slides along the wall.
        let mut dy = velocity.y * delta;
        if dy != 0.0 {
            let min_x = pos.x.floor().max(0.0) as i32;
            let max_x = (pos.x + size.x).ceil().min(MAX_WORLD_WIDTH as f32) as i32;
            let min_y = pos.y.min(pos.y + dy).floor().max(0.0) as i32;
            let max_y = (pos.y + size.y).max(pos.y + size.y + dy)
                .ceil().min(MAX_WORLD_HEIGHT as f32) as i32;

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if !self.get_tile_i32(x, y).is_some_and(|t| t.is_solid()) {
                        continue;
                    }
                    let tx = x as f32;
                    let ty = y as f32;
                    if pos.x >= tx + 1.0 || pos.x + size.x <= tx {
                        continue;
                    }
                    if dy > 0.0 {
                        let top = pos.y + size.y;
                        if top <= ty && top + dy > ty {
                            dy = dy.min(ty - top);
                            velocity.y = 0.0;
                        }
                    } else {
                        let bottom = pos.y;
                        let tile_top = ty + 1.0;
                        if bottom >= tile_top && bottom + dy < tile_top {
                            dy = dy.max(tile_top - bottom);
                            velocity.y = 0.0;
                            grounded = true;
                        }
                    }
                }
            }
            pos.y += dy;
        }

        entity.set_position(pos);
        entity.set_velocity(velocity);
        entity.data_mut().grounded = grounded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_seam_matches_procedural_generation() {
        let mut world = World::generate_with_seed(123);
        let x = CHUNK_SIZE as f32;
        let y = (MAX_WORLD_HEIGHT as f32 / 2.0).floor();
        let before = world.get_tile(x, y);
        world.stream_visible_chunks(Vector2::new(x, y), Vector2::new(x, y), 0);
        assert_eq!(world.get_tile(x, y), before);
    }

    #[test]
    fn edited_tiles_survive_unload_and_reload() {
        let mut world = World::generate_with_seed(123);
        let x = 8.0;
        let y = 1.0;
        world.stream_visible_chunks(Vector2::new(x, y), Vector2::new(x, y), 0);
        assert!(world.set_tile(x, y, TileType::Air));
        assert_eq!(world.get_tile(x, y), Some(TileType::Air));
        world.stream_visible_chunks(Vector2::new(80.0, 40.0), Vector2::new(80.0, 40.0), 0);
        assert_eq!(world.get_tile(x, y), Some(TileType::Air));
        world.stream_visible_chunks(Vector2::new(x, y), Vector2::new(x, y), 0);
        assert_eq!(world.get_tile(x, y), Some(TileType::Air));
    }

    #[test]
    fn changing_tile_to_natural_removes_override() {
        let mut world = World::generate_with_seed(1337);
        let x = 12.0;
        let y = 4.0;
        let natural = world.get_tile(x, y).unwrap();
        world.set_tile(x, y, TileType::Air);
        world.set_tile(x, y, natural);
        assert!(world.edits.is_empty());
    }

    #[test]
    fn invalid_coords_do_not_wrap() {
        let mut world = World::generate();
        assert_eq!(world.get_tile(-0.1, 0.0), None);
        assert!(!world.set_tile(-0.1, 0.0, TileType::Grass));
        assert_eq!(world.get_tile(f32::NAN, 0.0), None);
    }
}
