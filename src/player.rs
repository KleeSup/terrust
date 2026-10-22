use raylib::prelude::*;

use crate::{entity::{Entity, EntityData}, render, tiles::TileType, world::World};

const PLAYER_WIDTH: f32 = 0.9;
const PLAYER_HEIGHT: f32 = 1.9;

const GRAVITY: f32 = 30.0;
const MAX_FALL_SPEED: f32 = 20.0;

const ACCELERATION: f32 = 45.0;
const MAX_ACCELERATION: f32 = 7.0;
const FRICTION: f32 = 55.0;

const JUMP_SPEED: f32 = 10.0;

pub struct Player{
    data: EntityData,
    pub camera: Camera2D,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::default(),
            data: EntityData::default(),
        }
    }

    pub fn update(
        &mut self,
        handle: &RaylibDrawHandle,
        world: &mut World,
    ) {
        self.update_physics(world, handle);

        // Camera follows player
        let center = self.get_position() + Vector2::new(PLAYER_WIDTH * 0.5, 0.0);
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

    fn update_physics(&mut self, world: &mut World, handle: &RaylibDrawHandle) {
        let delta = handle.get_frame_time();

        let right = handle.is_key_down(KeyboardKey::KEY_D);
        let left = handle.is_key_down(KeyboardKey::KEY_A);

        let mut velocity = self.data.velocity;

        // Horizontal acceleration
        let direction = (right as i32 - left as i32) as f32;

        if direction != 0.0 {
            velocity.x = (
                velocity.x + direction * ACCELERATION * delta
            ).clamp(-MAX_ACCELERATION, MAX_ACCELERATION);
        } else {
            let friction = FRICTION * delta;

            if velocity.x > 0.0 {
                velocity.x = (velocity.x - friction).max(0.0);
            } else {
                velocity.x = (velocity.x + friction).min(0.0);
            }
        }

        // Gravity (Y-up world)
        if handle.is_key_pressed(KeyboardKey::KEY_SPACE) && self.is_on_ground() {
            velocity.y = JUMP_SPEED;
            self.data.grounded = false;
        }

        velocity.y = (velocity.y - GRAVITY * delta).max(-MAX_FALL_SPEED);
        
        self.set_velocity(velocity);

        // Collision-aware movement
        world.simulate(self, delta);
    }

    pub fn draw<T: RaylibDraw>(&self, handle: &mut T) {
        let pos = render::world_to_render(self.get_position());

        handle.draw_rectangle_rec(
            Rectangle::new(pos.x, pos.y - PLAYER_HEIGHT, PLAYER_WIDTH, PLAYER_HEIGHT),
            Color::RED,
        );
    }


}

impl Entity for Player {
    fn data(&self) -> &EntityData {
        &self.data
    }
    fn data_mut(&mut self) -> &mut EntityData {
        &mut self.data
    }
    fn get_size(&self) -> Vector2{
        Vector2 { x: PLAYER_WIDTH, y: PLAYER_HEIGHT }
    }
}