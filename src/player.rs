use raylib::prelude::*;

use crate::{render, tiles::TileType, world::World};

const CAMERA_SPEED: f32 = 50.0;

#[derive(Default)]
pub struct Player{
    pub camera: Camera2D
}

impl Player {

    pub fn new() -> Self{
        Self { camera: Camera2D::default() }
    }
    
    pub fn update(&mut self, handle: &RaylibDrawHandle, world: &mut World){
        render::apply_viewport(handle, &mut self.camera);

        let delta = handle.get_frame_time();

        if handle.is_key_down(KeyboardKey::KEY_D){
            self.camera.target.x += CAMERA_SPEED * delta;
        }
        if handle.is_key_down(KeyboardKey::KEY_A){
            self.camera.target.x -= CAMERA_SPEED * delta;
        }

        let mut click = handle.get_screen_to_world2D(handle.get_mouse_position(), self.camera);
        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            world.set_tile(click.x, click.y, TileType::Air);
        }
        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT){
            world.set_tile(click.x, click.y, TileType::Grass);
        }

    }

    pub fn draw<T : RaylibDraw>(&self, handle: &mut T){
        handle.draw_rectangle_rec(Rectangle::new(self.camera.target.x, self.camera.target.y - 1.9, 0.9, 1.9), Color::RED);
    }

}