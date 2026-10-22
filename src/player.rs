use raylib::prelude::*;

use crate::{render, tiles::TileType, world::World};

const CAMERA_SPEED: f32 = 50.0;

const PLAYER_WIDTH: f32 = 0.9;
const PLAYER_HEIGHT: f32 = 1.9;

const GRAVITY: f32 = 1.0;
const MAX_FALL_SPEED: f32 = GRAVITY * 5.0;

const ACCELERATION: f32 = 1.0;
const MAX_ACCELERATION: f32 = 7.0;
const FRICTION: f32 = ACCELERATION;

#[derive(Default)]
pub struct Player{
    pub position: Vector2,
    pub camera: Camera2D,
    pub velocity: Vector2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::zero(),
            camera: Camera2D::default(),
            velocity: Vector2::zero(),
        }
    }

    pub fn update(
        &mut self,
        handle: &RaylibDrawHandle,
        world: &mut World,
    ) {
        let delta = handle.get_frame_time();

        // Move player in world coordinates
        // if handle.is_key_down(KeyboardKey::KEY_D) {
        //     self.position.x += CAMERA_SPEED * delta;
        // }
        // if handle.is_key_down(KeyboardKey::KEY_A) {
        //     self.position.x -= CAMERA_SPEED * delta;
        // }

        self.update_physics(world, handle);

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

    fn update_physics(&mut self, world: &World, handle: &RaylibDrawHandle){
        let delta = handle.get_frame_time();
        // if(self.velocity.y > -MAX_FALL_SPEED){
        //     self.velocity.y -= GRAVITY;
        // }
        let left = handle.is_key_down(KeyboardKey::KEY_D);
        let right = handle.is_key_down(KeyboardKey::KEY_A);
        if left && self.velocity.x < MAX_ACCELERATION{
            self.velocity.x = (self.velocity.x + ACCELERATION).min(MAX_ACCELERATION);
        }
        if right && self.velocity.x > -MAX_ACCELERATION{
            self.velocity.x = (self.velocity.x - ACCELERATION).max(-MAX_ACCELERATION);
        }
        
        if !left && !right && self.velocity.x != 0.0{
            if self.velocity.x > 0.0{
                self.velocity.x = (self.velocity.x - FRICTION).max(0.0);
            }else{
                self.velocity.x = (self.velocity.x + FRICTION).min(0.0);
            }
        }


        self.position.x += self.velocity.x * delta;
        self.position.y += self.velocity.y * delta;
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