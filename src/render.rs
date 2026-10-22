use std::println;

use raylib::prelude::*;

use crate::{tiles, world::World};

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

pub fn apply_viewport(rl: &RaylibDrawHandle, camera: &mut Camera2D){

    let screen_w = rl.get_screen_width() as f32;
    let screen_h = rl.get_screen_height() as f32;

    let scale_x = screen_w / VIRTUAL_WIDTH;
    let scale_y = screen_h / VIRTUAL_HEIGHT;

    // Cover the screen without revealing more world
    camera.zoom = scale_x.max(scale_y) * tiles::TILE_SIZE;

    camera.offset = Vector2::new(
        screen_w / 2.0,
        screen_h / 2.0,
    );

}

pub fn draw_tile_map(
    rl: &mut RaylibMode2D<RaylibDrawHandle>,
    camera: Camera2D,
    tilesheet: &Texture2D,
    world: &World,
) {
    let min = rl.get_screen_to_world2D(
        Vector2::new(0.0, 0.0),
        camera,
    );

    let max = rl.get_screen_to_world2D(
        Vector2::new(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        ),
        camera,
    );

    let min_x = min.x.floor() as i32;
    let min_y = min.y.floor() as i32;

    let max_x = max.x.ceil() as i32;
    let max_y = max.y.ceil() as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            if let Some(tile) = world.get_tile(x as f32, y as f32){
                tile.draw(rl, tilesheet, Vector2::new(x as f32, y as f32));
            }
        }
    }
}