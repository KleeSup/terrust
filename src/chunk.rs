//! 32x32 tile chunks. Tile positions and coordinates are Y-up.
use crate::{terrain, tiles::TileType};

pub const CHUNK_SIZE: i32 = 32;
const CHUNK_AREA: usize = (CHUNK_SIZE as usize) * (CHUNK_SIZE as usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn from_tile(x: i32, y: i32) -> Self {
        Self {
            x: x.div_euclid(CHUNK_SIZE),
            y: y.div_euclid(CHUNK_SIZE),
        }
    }
}

pub struct Chunk {
    tiles: Box<[TileType; CHUNK_AREA]>,
}

/// Generate one tile independently of any loaded chunks.
/// Used as a fallback if physics queries outside the camera's loaded area.
pub fn generated_tile(x: i32, y: i32, seed: u32, world_height: usize) -> TileType {
    let surface = terrain::surface_height(x, seed, world_height);
    generated_tile_with_surface(x, y, surface, seed, world_height)
}

fn generated_tile_with_surface(
    x: i32,
    y: i32,
    surface: i32,
    seed: u32,
    world_height: usize,
) -> TileType {
    if y >= surface {
        return TileType::Air;
    }

    let soil_depth = ((world_height as f32 * 0.08).round() as i32).clamp(3, 12);
    let cave_roof = ((world_height as f32 * 0.08).round() as i32).clamp(4, 16);
    let depth = surface - 1 - y;

    if depth == 0 {
        TileType::Grass
    } else if depth < soil_depth {
        TileType::Dirt
    } else if y >= 2 && depth >= cave_roof && terrain::is_cave(x, y, seed) {
        TileType::Air
    } else if terrain::is_coal(x, y, seed) {
        TileType::CoalOre
    } else {
        TileType::Stone
    }
}

impl Chunk {
    pub fn generate(
        pos: ChunkPos,
        seed: u32,
        world_width: usize,
        world_height: usize,
    ) -> Self {
        let mut tiles = [TileType::Air; CHUNK_AREA];

        for local_x in 0..CHUNK_SIZE {
            let x = pos.x * CHUNK_SIZE + local_x;
            if x < 0 || x as usize >= world_width {
                continue;
            }

            // Cache the surface for this column while generating the chunk.
            let surface = terrain::surface_height(x, seed, world_height);

            for local_y in 0..CHUNK_SIZE {
                let y = pos.y * CHUNK_SIZE + local_y;
                if y < 0 || y as usize >= world_height {
                    continue;
                }
                let index = Self::index(local_x, local_y);
                tiles[index] = generated_tile_with_surface(x, y, surface, seed, world_height);
            }
        }

        Self { tiles: Box::new(tiles) }
    }

    #[inline]
    fn index(local_x: i32, local_y: i32) -> usize {
        (local_y as usize) * CHUNK_SIZE as usize + local_x as usize
    }

    pub fn get(&self, x: i32, y: i32) -> TileType {
        let local_x = x.rem_euclid(CHUNK_SIZE);
        let local_y = y.rem_euclid(CHUNK_SIZE);
        self.tiles[Self::index(local_x, local_y)]
    }

    pub fn set(&mut self, x: i32, y: i32, tile: TileType) {
        let local_x = x.rem_euclid(CHUNK_SIZE);
        let local_y = y.rem_euclid(CHUNK_SIZE);
        self.tiles[Self::index(local_x, local_y)] = tile;
    }
}
