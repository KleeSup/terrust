use raylib::prelude::*;

use crate::{render, tiles::TileType, world::World};

const CAMERA_SPEED: f32 = 50.0;

const PLAYER_WIDTH: f32 = 0.9;
const PLAYER_HEIGHT: f32 = 1.9;

#[derive(Default)]
pub struct Player{
    pub position: Vector2,
    pub camera: Camera2D
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::zero(),
            camera: Camera2D::default(),
        }
    }

    pub fn update(
        &mut self,
        handle: &RaylibDrawHandle,
        world: &mut World,
    ) {
        let delta = handle.get_frame_time();

        // Move player in world coordinates
        if handle.is_key_down(KeyboardKey::KEY_D) {
            self.position.x += CAMERA_SPEED * delta;
        }
        if handle.is_key_down(KeyboardKey::KEY_A) {
            self.position.x -= CAMERA_SPEED * delta;
        }

        // Camera follows player
        let center = self.position + Vector2::new(PLAYER_WIDTH * 0.5, 0.0);
        self.camera.target = render::world_to_render(center);

        // Configure zoom and center offset
        render::apply_viewport(handle, &mut self.camera);

        let click = render::screen_to_world(
            handle,
            handle.get_mouse_position(),
            self.camera,
        );

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            world.set_tile(click.x, click.y, TileType::Air);
        }

        if handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            world.set_tile(click.x, click.y, TileType::Grass);
        }
    }

    pub fn draw<T: RaylibDraw>(&self, handle: &mut T) {
        let pos = render::world_to_render(self.position);

        handle.draw_rectangle_rec(
            Rectangle::new(pos.x, pos.y - PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_HEIGHT),
            Color::RED,
        );
    }

    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
    }
}