#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use raylib::prelude::*;

use crate::entity::Entity;

mod tiles;
mod render;
mod terrain;
mod chunk;
mod world;
mod player;
mod collision;
mod entity;

const IMAGE: &[u8] = include_bytes!("assets/spritesheet_tiles.png");

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Terrust")
        .resizable()
        .build();

    let img = Image::load_image_from_mem(".png", IMAGE).unwrap();
    let tilesheet = rl.load_texture_from_image(&thread, &img).unwrap();

    let mut world = world::World::generate_with_seed(1337);
    let mut player = player::Player::new();
    let spawn_x = world::MAX_WORLD_WIDTH as f32 / 2.0;
    player.set_position(Vector2::new(
        spawn_x,
        world.surface_y(spawn_x as i32) + 0.1,
    ));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);

        player.update(&d, &mut world);

        // After camera update, load all visible chunks.
        let top_left = render::screen_to_world(
            &d, Vector2::new(0.0, 0.0), player.camera,
        );
        let bottom_right = render::screen_to_world(
            &d,
            Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32),
            player.camera,
        );
        world.stream_visible_chunks(top_left, bottom_right, 1);

        let mut m2 = d.begin_mode2D(player.camera);
        render::draw_tile_map(&mut m2, player.camera, &tilesheet, &world);
        player.draw(&mut m2);
    }
}
