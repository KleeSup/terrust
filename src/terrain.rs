// Smooth, deterministic 1D value noise for tile-height terrain.

use crate::world;

fn hash01(mut x: u32) -> f32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x as f32 / u32::MAX as f32
}

fn value_noise(x: f32, seed: u32) -> f32 {
    let left = x.floor() as i32;
    let t = x - left as f32;
    let t = t * t * (3.0 - 2.0 * t); // Smooth interpolation

    let a = hash01((left as u32).wrapping_add(seed));
    let b = hash01(((left + 1) as u32).wrapping_add(seed));
    a + (b - a) * t
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

const AVG_MAP_HEIGHT: f32 = world::MAX_WORLD_HEIGHT as f32 / 2.0;
const HILL_WIDTH: f32 = 0.050; // Width of large hills (smaller = wider)
const HILL_STRENGTH: f32 = 15.0; // Strength of large hills (higher = taller)
const FREQUENCY_DETAIL: f32 = 0.125; // Frequency of medium surface details
const FLAT_VARIATION: f32 = 1.5; // Amount of variation in flat regions

/// First empty tile above the generated ground in Y-up world coordinates.
pub fn surface_height(x: i32, seed: u32, world_height: usize) -> i32 {
    let x = x as f32;
    let base = AVG_MAP_HEIGHT;

    // Flat areas with little gradual height variation.
    let plains = base + (value_noise(x * 0.025, seed ^ 0xA11C_E001) - 0.5) * FLAT_VARIATION;

    // Hills with several scales of smooth noise blended together.
    let hills = base
        + (value_noise(x * HILL_WIDTH, seed ^ 0xB22D_E002) - 0.5) * HILL_STRENGTH
        + (value_noise(x * FREQUENCY_DETAIL, seed ^ 0xC33E_E003) - 0.5) * 3.0
        + (value_noise(x * 0.250, seed ^ 0xD44F_E004) - 0.5) * 1.0;

    // Slow transition between flat stretches and hilly stretches.
    // The long sine wave ensures both terrain types appear in the tile world.
    let region = 0.5 + 0.5 * (x * 0.075 - 1.0).sin()
        + (value_noise(x * 0.018, seed ^ 0xE550_E005) - 0.5) * 0.15;
    let hilliness = smoothstep(0.3, 0.7, region);

    let height = plains + (hills - plains) * hilliness;
    (height.round() as i32).clamp(3, world_height.saturating_sub(3).max(3) as i32)
}
