use raylib::prelude::*;

pub fn minkowski_sum(a: Rectangle, b: Rectangle) -> Rectangle{
    let width = a.width + b.width;
    let height = a.height + b.height;
    return Rectangle{
        x: (b.x + b.width / 2.0) - (width / 2.0),
        y: (b.y + b.height / 2.0) - (height / 2.0),
        width: width,
        height: height,
    };
}

pub fn aabb_overlaps(a: Rectangle, b: Rectangle) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y
}

#[derive(Default, Clone, Copy)]
pub struct RayResult{
    pub hit_pos: Vector2,
    pub normal: Vector2,
    pub hit_time: f32,
    pub is_hit: bool,
}

pub fn does_ray_intersect_aabb(x: f32, y: f32, magnitude: Vector2, aabb: Rectangle) -> RayResult {
    let mut entry = f32::NEG_INFINITY;
    let mut exit = f32::INFINITY;
    let mut hit_normal = Vector2::zero();
    let mut result = RayResult::default();

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
    } else if x <= aabb.x || x >= aabb.x + aabb.width {
        return result;
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
    } else if y <= aabb.y || y >= aabb.y + aabb.height {
        return result;
    }

    // No intersection within the movement segment
    if entry >= exit || exit <= 0.0 || entry >= 1.0 {
        return result;
    }

    // Starting inside the AABB has no entry face
    if entry < 0.0 {
        return result;
    }

    result.hit_time = entry;

    result.hit_pos = Vector2::new(
        x + magnitude.x * entry,
        y + magnitude.y * entry,
    );

    result.normal = hit_normal;
    result.is_hit = true;

    return result;
}
