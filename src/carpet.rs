//! What the room's surfaces are made of, woven here rather than loaded.
//! Spec 0005.
//!
//! The room was thirteen flat boxes in a flat corridor, and no amount of
//! colouring them fixed that: a box is a box at any hue. Every surface was one
//! colour across its whole face, which is the thing an eye has nothing to land
//! on.
//!
//! An arcade's floor is the loudest thing in it, and this is that: a dark
//! ground with bright shapes thrown over it, the pattern that exists so the
//! carpet does not show what gets spilled on it.
//!
//! Procedural and deterministic, the way cairn weaves its wood grain and
//! cascada draws its pips. Nothing is loaded and the same tile comes out every
//! time.

use blitzkit::texture::TextureData;

/// How many texels across one tile of it is.
pub const WOVEN: u32 = 160;

/// How many times it repeats across the floor.
///
/// The engine's sampler repeats, so this is the floor quad's own u and v rather
/// than anything baked into the tile. Eight across thirty units is a tile a
/// little under four units wide, which is about a stride and a half.
pub const TILES: f32 = 8.0;

/// The ground it is woven on, and the threads thrown over it.
///
/// Dark, because the room is dim on purpose and a bright floor would be the
/// brightest thing in it. The shapes are the arcade colours, the ones a cabinet
/// band already burns in.
const GROUND: [u8; 3] = [16, 15, 34];
const THREADS: [[u8; 3]; 5] = [
    [220, 44, 120],
    [38, 190, 210],
    [240, 190, 60],
    [120, 70, 200],
    [60, 200, 120],
];

/// How many shapes are thrown over one tile.
const THROWN: u32 = 52;

/// The ground it is woven on, as a colour.
///
/// Only the checks ask: the floor's darkness used to live in a constant they
/// could read, and it moved in here when the floor became a texture. Nothing
/// the room draws needs it, because the weave carries it.
#[cfg(test)]
pub fn ground() -> glam::Vec3 {
    glam::vec3(
        GROUND[0] as f32 / 255.0,
        GROUND[1] as f32 / 255.0,
        GROUND[2] as f32 / 255.0,
    )
}

/// A number from a number, so the same tile is woven every time.
fn from(seed: u32) -> u32 {
    let mut n = seed.wrapping_mul(0x9E37_79B9);
    n ^= n >> 15;
    n = n.wrapping_mul(0x85EB_CA6B);
    n ^= n >> 13;
    n
}

/// Where a texel lands when a shape runs off the edge.
///
/// A tile that does not wrap has a seam every few strides, and a floor made of
/// seams is worse than a floor of one colour.
fn wrapped(at: i32) -> u32 {
    at.rem_euclid(WOVEN as i32) as u32
}

/// The carpet.
pub fn woven() -> TextureData {
    let mut pixels = Vec::with_capacity((WOVEN * WOVEN * 4) as usize);
    for _ in 0..WOVEN * WOVEN {
        pixels.extend_from_slice(&[GROUND[0], GROUND[1], GROUND[2], 255]);
    }

    let mut ink = |x: i32, y: i32, colour: [u8; 3]| {
        let n = ((wrapped(y) * WOVEN + wrapped(x)) * 4) as usize;
        pixels[n] = colour[0];
        pixels[n + 1] = colour[1];
        pixels[n + 2] = colour[2];
    };

    for shape in 0..THROWN {
        let roll = from(shape);
        let x = (roll % WOVEN) as i32;
        let y = ((roll >> 8) % WOVEN) as i32;
        let colour = THREADS[(roll >> 16) as usize % THREADS.len()];
        let size = 4 + (roll >> 20) % 7;

        match (roll >> 24) % 3 {
            // a triangle
            0 => {
                for row in 0..size {
                    for across in 0..=row {
                        ink(x + across as i32 - row as i32 / 2, y + row as i32, colour);
                    }
                }
            }
            // a chevron
            1 => {
                for step in 0..size {
                    let step = step as i32;
                    for thick in 0..2 {
                        ink(x + step, y + step + thick, colour);
                        ink(x - step, y + step + thick, colour);
                    }
                }
            }
            // and a square
            _ => {
                let small = (size / 2).max(2) as i32;
                for row in 0..small {
                    for across in 0..small {
                        ink(x + across, y + row, colour);
                    }
                }
            }
        }
    }

    TextureData::from_pixels(WOVEN, WOVEN, pixels)
}

/// A surface with something on it rather than nothing: a base colour with a
/// fine speck through it.
///
/// Not a pattern, a texture. The walls and the cabinets are big flat faces and
/// what they need is for the eye to find something when it lands, not another
/// thing to look at. Deliberately quiet: the carpet is the loud one and the
/// marquees are the bright one.
///
/// Non directional on purpose. `MeshData::cube` carries its uvs whichever way
/// round each face pleases, which is why the screens are their own quad, so a
/// speck is safe on a cube where a stripe would come out sideways on half of
/// it.
pub fn mottled(seed: u32, base: [u8; 3], depth: i32) -> TextureData {
    let mut pixels = Vec::with_capacity((MOTTLE * MOTTLE * 4) as usize);

    for n in 0..MOTTLE * MOTTLE {
        let roll = from(n ^ seed.wrapping_mul(0x27D4_EB2D));
        // two rolls at different scales, so it is grain rather than static
        let fine = (roll % 256) as i32 - 128;
        let broad = (from(n / MOTTLE * 31 + n % MOTTLE / 7) % 256) as i32 - 128;
        let shift = (fine * depth / 400) + (broad * depth / 260);

        for channel in base {
            pixels.push((channel as i32 + shift).clamp(0, 255) as u8);
        }
        pixels.push(255);
    }

    TextureData::from_pixels(MOTTLE, MOTTLE, pixels)
}

/// How many texels across a mottled tile is, and how many times it repeats.
pub const MOTTLE: u32 = 128;

/// The seeds the room's two mottled surfaces are made with, so neither is the
/// other's pattern at a different colour.
pub const WALL_SEED: u32 = 0x5EED_0001;
pub const CABINET_SEED: u32 = 0x5EED_0002;

#[cfg(test)]
mod tests {
    use super::*;

    /// How much of a tile is not the ground it is woven on.
    fn thread(carpet: &TextureData) -> f32 {
        let level = &carpet.levels[0];
        let lit = level
            .pixels
            .chunks_exact(4)
            .filter(|texel| [texel[0], texel[1], texel[2]] != GROUND)
            .count();

        lit as f32 / (level.pixels.len() / 4) as f32
    }

    /// Spec 0005: there is a pattern on it, and it is mostly ground.
    ///
    /// A floor of one colour is what this exists to stop. A floor that is all
    /// pattern is the brightest thing in a room that is dim on purpose.
    #[test]
    fn it_has_a_pattern_and_is_still_dark() {
        let carpet = woven();
        let on_it = thread(&carpet);

        assert!(on_it > 0.02, "only {:.3} of it is pattern", on_it);
        assert!(
            on_it < 0.30,
            "{:.3} of it is pattern, which is not a ground",
            on_it
        );
    }

    /// Spec 0005: and it wraps, so a floor of them has no seams.
    #[test]
    fn it_wraps_rather_than_seaming() {
        for at in [
            -3i32,
            -1,
            0,
            1,
            WOVEN as i32 - 1,
            WOVEN as i32,
            WOVEN as i32 + 5,
        ] {
            let put = wrapped(at);

            assert!(put < WOVEN, "{} landed at {}, off the tile", at, put);
        }

        assert_eq!(wrapped(-1), WOVEN - 1, "one off the left is not the right");
        assert_eq!(
            wrapped(WOVEN as i32),
            0,
            "one off the right is not the left"
        );
    }

    /// Spec 0005: the same tile is woven every time.
    #[test]
    fn it_is_woven_the_same_every_time() {
        assert_eq!(woven().levels[0], woven().levels[0]);
    }

    /// Spec 0005: a mottled surface has grain in it and stays near its base.
    ///
    /// The point is for the eye to find something when it lands on a wall, not
    /// for the wall to become a thing to look at.
    #[test]
    fn a_mottle_is_quiet_but_not_nothing() {
        let base = [60u8, 40, 30];
        let wall = mottled(WALL_SEED, base, 40);
        let level = &wall.levels[0];

        let mut lowest = 255i32;
        let mut highest = 0i32;
        let mut flat = true;
        for texel in level.pixels.chunks_exact(4) {
            lowest = lowest.min(texel[0] as i32);
            highest = highest.max(texel[0] as i32);
            if texel[0] != base[0] {
                flat = false;
            }
        }

        assert!(!flat, "every texel is the base, which is a flat colour");
        assert!(
            highest - lowest > 8,
            "it only varies by {}, which nothing will see",
            highest - lowest
        );
        assert!(
            highest - lowest < 120,
            "it varies by {}, which is a pattern rather than a grain",
            highest - lowest
        );
    }

    /// Spec 0005: and the two mottled surfaces are not the same grain.
    #[test]
    fn the_wall_and_the_cabinet_differ() {
        let base = [60u8, 40, 30];

        assert_ne!(
            mottled(WALL_SEED, base, 40).levels[0],
            mottled(CABINET_SEED, base, 40).levels[0],
            "the cabinets wear the wall's own grain"
        );
    }

    /// Spec 0005: and it is the shape the texture wants.
    #[test]
    fn it_is_square_and_opaque() {
        let carpet = woven();

        assert_eq!(carpet.width(), WOVEN);
        assert_eq!(carpet.height(), WOVEN);
        for texel in carpet.levels[0].pixels.chunks_exact(4) {
            assert_eq!(texel[3], 255, "a texel is not opaque");
        }
    }
}
