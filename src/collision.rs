use raylib::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct RayResult{
    hit_pos: Vector2,
    normal: Vector2,
    hit_time: f32,
    is_hit: bool,
}

pub fn does_ray_intersect_aabb(
    x: f32,
    y: f32,
    magnitude: Vector2,
    aabb: Rectangle,
    hit_pos: &mut Vector2,
    normal: &mut Vector2,
    hit_time: &mut f32,
) -> bool {
    let mut entry = f32::NEG_INFINITY;
    let mut exit = f32::INFINITY;
    let mut hit_normal = Vector2::zero();

    // X axis
    if magnitude.x != 0.0 {
        let inv = 1.0 / magnitude.x;

        let t1 = (aabb.x - x) * inv;
        let t2 = (aabb.x + aabb.width - x) * inv;

        let near = t1.min(t2);
        let far = t1.max(t2);

        if near > entry {
            entry = near;
            hit_normal = Vector2::new(
                -magnitude.x.signum(),
                0.0,
            );
        }

        exit = exit.min(far);
    } else if x < aabb.x || x > aabb.x + aabb.width {
        return false;
    }

    // Y axis
    if magnitude.y != 0.0 {
        let inv = 1.0 / magnitude.y;

        let t1 = (aabb.y - y) * inv;
        let t2 = (aabb.y + aabb.height - y) * inv;

        let near = t1.min(t2);
        let far = t1.max(t2);

        if near > entry {
            entry = near;
            hit_normal = Vector2::new(
                0.0,
                -magnitude.y.signum(),
            );
        }

        exit = exit.min(far);
    } else if y < aabb.y || y > aabb.y + aabb.height {
        return false;
    }

    // No intersection within the movement segment
    if entry >= exit || exit <= 0.0 || entry >= 1.0 {
        return false;
    }

    // Starting inside the AABB has no entry face
    if entry < 0.0 {
        return false;
    }

    *hit_time = entry;

    *hit_pos = Vector2::new(
        x + magnitude.x * entry,
        y + magnitude.y * entry,
    );

    *normal = hit_normal;

    true
}
