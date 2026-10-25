//! Deterministic, dependency-free terrain and underground noise.
//! Coordinates are Y-up. Each unit represents one tile.

// Increase tunnel width for wider corridors.
// Decrease cavern / coal thresholds for more caverns / coal.
pub const TUNNEL_HALF_WIDTH: f32 = 0.045;
pub const CAVERN_THRESHOLD: f32 = 0.69;
pub const COAL_THRESHOLD: f32 = 0.73;

fn hash(mut v: u32) -> u32 {
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846c_a68b);
    v ^ (v >> 16)
}

fn hash01(v: u32) -> f32 {
    hash(v) as f32 / u32::MAX as f32
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn noise1(x: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let t = smooth(x - x0 as f32);
    let a = hash01((x0 as u32).wrapping_add(seed));
    let b = hash01((x0.wrapping_add(1) as u32).wrapping_add(seed));
    mix(a, b, t)
}

/// Smooth, deterministic two-dimensional value noise in [0, 1].
fn noise2(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = smooth(x - x0 as f32);
    let ty = smooth(y - y0 as f32);

    let sample = |ix: i32, iy: i32| -> f32 {
        let key = (ix as u32).wrapping_mul(0x9e37_79b9)
            ^ (iy as u32).wrapping_mul(0x85eb_ca6b)
            ^ seed;
        hash01(key)
    };

    let bottom = mix(sample(x0, y0), sample(x0.wrapping_add(1), y0), tx);
    let top = mix(
        sample(x0, y0.wrapping_add(1)),
        sample(x0.wrapping_add(1), y0.wrapping_add(1)),
        tx,
    );
    mix(bottom, top, ty)
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    smooth(((x - a) / (b - a)).clamp(0.0, 1.0))
}

/// Y coordinate of the first air tile above the surface.
/// Uses the world height, so changing MAX_WORLD_HEIGHT moves the terrain's baseline.
pub fn surface_height(x: i32, seed: u32, world_height: usize) -> i32 {
    let height = world_height as f32;
    let base = height * 0.5;
    let x = x as f32;

    let plains = base + (noise1(x * 0.025, seed ^ 0xA11C_E001) - 0.5) * (height * 0.03);
    let hills = base
        + (noise1(x * 0.050, seed ^ 0xB22D_E002) - 0.5) * (height * 0.24)
        + (noise1(x * 0.125, seed ^ 0xC33E_E003) - 0.5) * (height * 0.06)
        + (noise1(x * 0.250, seed ^ 0xD44F_E004) - 0.5) * (height * 0.02);

    // Hill and plain stretches recur as the world gets wider.
    let region = 0.5 + 0.5 * (x * 0.075 - 1.0).sin()
        + (noise1(x * 0.018, seed ^ 0xE550_E005) - 0.5) * 0.15;
    let hilliness = smoothstep(0.3, 0.7, region);

    let margin = ((world_height as i32) / 12).max(2);
    let maximum = (world_height as i32 - margin).max(1);
    (mix(plains, hills, hilliness).round() as i32).clamp(1, maximum)
}

/// Cave carve mask -> winding narrow tunnels plus larger connected caverns.
/// Sample in world tile coordinates so a wider/taller world reveals more caves,
/// rather than stretching existing caves to fill its dimensions.
pub fn is_cave(x: i32, y: i32, seed: u32) -> bool {
    let x = x as f32 + 0.5;
    let y = y as f32 + 0.5;

    // Contour lines of a broad noise field form winding tunnels.
    let tunnel = noise2(x / 16.0, y / 12.0, seed ^ 0xCA7E_1011);
    let meander = noise2(x / 38.0, y / 30.0, seed ^ 0xCA7E_2022);
    let tunnel_distance = (tunnel + (meander - 0.5) * 0.25 - 0.5).abs();
    let narrow_tunnel = tunnel_distance < TUNNEL_HALF_WIDTH;

    // Large open pockets with softened, irregular outlines.
    let cavern = noise2(x / 22.0, y / 16.0, seed ^ 0xCA7E_3033) * 0.75
        + noise2(x / 8.0, y / 7.0, seed ^ 0xCA7E_4044) * 0.25;
    let large_cavern = cavern > CAVERN_THRESHOLD;

    narrow_tunnel || large_cavern
}

/// Coal occurs in small clustered veins. Its map is independent of the cave map.
pub fn is_coal(x: i32, y: i32, seed: u32) -> bool {
    let x = x as f32 + 0.5;
    let y = y as f32 + 0.5;
    let vein = noise2(x / 4.5, y / 4.0, seed ^ 0xC0A1_0001);
    let detail = noise2(x / 2.5, y / 2.5, seed ^ 0xC0A1_0002);
    vein * 0.85 + detail * 0.15 > COAL_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_stays_inside_world_for_multiple_sizes() {
        for height in [10, 50, 120, 500] {
            for x in 0..1000 {
                let y = surface_height(x, 1337, height);
                assert!(y >= 1 && y < height as i32);
            }
        }
    }

    #[test]
    fn noise_is_seeded_and_repeatable() {
        assert_eq!(surface_height(43, 1337, 50), surface_height(43, 1337, 50));
        assert_eq!(is_cave(43, 10, 1337), is_cave(43, 10, 1337));
        assert_eq!(is_coal(43, 10, 1337), is_coal(43, 10, 1337));
    }
}
