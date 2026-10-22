use std::collections::HashMap;

use crate::tiles::TileType;

pub const MAX_WORLD_WIDTH: usize = 100;
pub const MAX_WORLD_HEIGHT: usize = 50;

fn pack(x: f32, y: f32) -> u64{
    ((x as u32 as u64) << 32) | (y as u32 as u64)
}

pub struct World{
    tiles: HashMap<u64, TileType>
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


}