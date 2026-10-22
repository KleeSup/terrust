use std::collections::HashMap;

use crate::{entity::Entity, tiles::TileType};

pub const MAX_WORLD_WIDTH: usize = 100;
pub const MAX_WORLD_HEIGHT: usize = 50;

fn pack(x: f32, y: f32) -> u64{
    ((x as u32 as u64) << 32) | (y as u32 as u64)
}

pub struct World{
    tiles: HashMap<u64, TileType>,
}

impl World {

    pub fn generate() -> World{
        let mut world = Self{tiles: HashMap::new()};

        for x in 0..MAX_WORLD_WIDTH{
            for y in 0..5{
                world.set_tile(x as f32, y as f32, TileType::Dirt);
            }
        }

        return world;
    }

    #[inline]
    pub fn is_valid_tile(x: f32, y: f32) -> bool{
        x >= 0.0 && y >= 0.0 && x < MAX_WORLD_WIDTH as f32 && y < MAX_WORLD_HEIGHT as f32
    }
    
    pub fn get_tile(&self, x: f32, y: f32) -> Option<&TileType>{
        if !Self::is_valid_tile(x, y){
            return None;
        }
        return match self.tiles.get(&pack(x, y)) {
            Some(tile) => Some(tile),
            None => Some(&TileType::Air),
        }
    }

    pub fn set_tile(&mut self, x: f32, y: f32, tile: TileType) -> bool{
        if !Self::is_valid_tile(x, y){
            return false;
        }
        if tile.is_air(){
            self.tiles.remove(&pack(x, y));
        }else{
            self.tiles.insert(pack(x, y), tile);
        }
        return true;
    } 

 
    // Tile collision is swept one axis at a time. This handles wall sliding,
    // floor contact, and movement across more than one tile in one frame.
    // Entity position is bottom-left in a Y-up coordinate system.
    pub fn simulate<T: Entity>(&mut self, entity: &mut T, delta: f32) {
        let mut pos = entity.get_position();
        let size = entity.get_size();
        let mut velocity = entity.get_velocity();
        let mut grounded = false;

        // Horizontal sweep. Keep Y fixed so wall contact won't change height.
        let mut dx = velocity.x * delta;
        if dx != 0.0 {
            let min_x = pos.x.min(pos.x + dx).floor().max(0.0) as i32;
            let max_x = (pos.x + size.x).max(pos.x + size.x + dx)
                .ceil().min(MAX_WORLD_WIDTH as f32) as i32;
            let min_y = pos.y.floor().max(0.0) as i32;
            let max_y = (pos.y + size.y)
                .ceil().min(MAX_WORLD_HEIGHT as f32) as i32;

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if !self.get_tile(x as f32, y as f32)
                        .is_some_and(|tile| tile.is_solid())
                    {
                        continue;
                    }

                    let tile_x = x as f32;
                    let tile_y = y as f32;
                    // Touching along Y is not overlapping.
                    if pos.y >= tile_y + 1.0 || pos.y + size.y <= tile_y {
                        continue;
                    }

                    if dx > 0.0 {
                        let right = pos.x + size.x;
                        if right <= tile_x && right + dx > tile_x {
                            dx = dx.min(tile_x - right);
                            velocity.x = 0.0;
                        }
                    } else {
                        let left = pos.x;
                        let tile_right = tile_x + 1.0;
                        if left >= tile_right && left + dx < tile_right {
                            dx = dx.max(tile_right - left);
                            velocity.x = 0.0;
                        }
                    }
                }
            }
            pos.x += dx;
        }

        // Vertical sweep, using the X position after horizontal movement.
        let mut dy = velocity.y * delta;
        if dy != 0.0 {
            let min_x = pos.x.floor().max(0.0) as i32;
            let max_x = (pos.x + size.x)
                .ceil().min(MAX_WORLD_WIDTH as f32) as i32;
            let min_y = pos.y.min(pos.y + dy).floor().max(0.0) as i32;
            let max_y = (pos.y + size.y).max(pos.y + size.y + dy)
                .ceil().min(MAX_WORLD_HEIGHT as f32) as i32;

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if !self.get_tile(x as f32, y as f32)
                        .is_some_and(|tile| tile.is_solid())
                    {
                        continue;
                    }

                    let tile_x = x as f32;
                    let tile_y = y as f32;
                    // Touching along X is not overlapping.
                    if pos.x >= tile_x + 1.0 || pos.x + size.x <= tile_x {
                        continue;
                    }

                    if dy > 0.0 {
                        let top = pos.y + size.y;
                        if top <= tile_y && top + dy > tile_y {
                            dy = dy.min(tile_y - top);
                            velocity.y = 0.0;
                        }
                    } else {
                        let bottom = pos.y;
                        let tile_top = tile_y + 1.0;
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
