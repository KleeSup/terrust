#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use raylib::prelude::*;

mod tiles;
mod render;
mod world;
mod player;

const IMAGE: &[u8] = include_bytes!("assets/spritesheet_tiles.png");


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Terrust")
        .resizable()
        .build();

    let mut player = player::Player::new();
    player.set_position(Vector2::new(0.0, 5.0));

    let img = Image::load_image_from_mem(".png", IMAGE).unwrap();
    let tilesheet = rl.load_texture_from_image(&thread, &img).unwrap();

    let mut world = world::World::generate();

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
