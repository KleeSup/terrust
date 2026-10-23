use raylib::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
const ORIGINAL_SIZE: f32 = 128.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Air,
    Dirt,
    Grass,
    Stone,
}

impl TileType {
    
    pub fn draw<T : RaylibDraw>(self, handle: &mut T, tilesheet: &Texture2D, pos: Vector2){
        let (x, y ) = match self {
            Self::Air => return,
            Self::Dirt => (650.0, 130.0),
            Self::Grass => (650.0, 0.0),
            Self::Stone => (260.0, 650.0),
        };
        

        handle.draw_texture_pro(tilesheet, 
            Rectangle::new(x, y, ORIGINAL_SIZE, ORIGINAL_SIZE), 
            Rectangle::new(pos.x, -pos.y - 1.0, 1.0, 1.0), 
            Vector2::zero(), 0.0, Color::WHITE);
    }

    
    pub fn is_solid(self) -> bool{
        match self {
            Self::Air => false,
            _ => true,
        }
    }

    #[inline]
    pub fn is_air(self) -> bool{
        self == Self::Air
    }


}