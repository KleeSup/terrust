#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use raylib::prelude::*;

use crate::entity::Entity;

mod tiles;
mod render;
mod world;
mod player;
mod collision;
mod entity;
mod terrain;

const IMAGE: &[u8] = include_bytes!("assets/spritesheet_tiles.png");


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Terrust")
        .resizable()
        .build();

    let mut world = world::World::generate();
    let mut player = player::Player::new();

    let x = world::MAX_WORLD_WIDTH as f32 / 2.0;
    let y = world.surface_y(x as i32);
    player.set_position(Vector2::new(x, y));

    let img = Image::load_image_from_mem(".png", IMAGE).unwrap();
    let tilesheet = rl.load_texture_from_image(&thread, &img).unwrap();

    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);

        //render::apply_viewport(&d, &mut camera);
        //update(&d, &mut camera);
        player.update(&d, &mut world);
        let mut m2 = d.begin_mode2D(player.camera);

        render::draw_tile_map(&mut m2, player.camera, &tilesheet, &world);
        player.draw(&mut m2);
    }
}