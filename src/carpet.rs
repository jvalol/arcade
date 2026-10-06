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

/// The rug in the nook, which is not the arcade's carpet.
///
/// The room out there is a dim neon hall and its floor is confetti on black,
/// which is right for it. The nook is a study off it, and a study's floor is a
/// rug: a dark ground, a border, and a figure repeated across it. The whole
/// point of the nook is that it is not the arcade, and the floor is the largest
/// single thing in anybody's view of it.
/// One rug and not a tiled floor, which is the whole difference between a rug
/// and lino. Laid three times over it came out as three stretched lozenges in a
/// row, because a tile count stretches with the quad and the nook is twice as
/// long as it is deep.
/// A boarded floor for the nook: how big the picture is, how much floor one
/// tile of it covers, and how many planks lie across that tile.
///
/// The nook had the arcade's carpet under it, which is confetti on black, and a
/// rug wall to wall was hiding that rather than fixing it. Shrink the rug to
/// something a room would actually have and the confetti comes back out round
/// the edges of it, which is worse than before: a study with a neon floor
/// showing at the skirting.
pub const BOARDS: u32 = 192;
pub const BOARD_TILE: f32 = 2.4;
pub const PLANKS: u32 = 6;
pub const BOARD_SEED: u32 = 0x5EED_0003;
const BOARD_PALE: i32 = 206;
const BOARD_JOINT: i32 = 96;

/// A floor of boards: planks down the long way, a dark line between them, and a
/// butt joint across each one at a place of its own.
///
/// The joints are staggered rather than in a line. Boards laid end to end in a
/// row is not a floor, it is a grid, and a grid is the one thing the eye reads
/// as a texture rather than as a thing.
pub fn boards(seed: u32) -> TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };

    // one tone and one joint per plank, decided before any pixel is laid, so a
    // plank is one colour the whole way down rather than noise in a strip
    let planks: Vec<(i32, u32)> = (0..PLANKS)
        .map(|_| (BOARD_PALE + (next() % 29) as i32 - 14, next() % BOARDS))
        .collect();

    let wide = BOARDS / PLANKS;
    let mut pixels = Vec::with_capacity((BOARDS * BOARDS * 4) as usize);
    for y in 0..BOARDS {
        for x in 0..BOARDS {
            let (tone, joint) = planks[(x / wide) as usize % planks.len()];
            // the grain, which runs down the plank and not across it
            let grain = ((next() % 11) as i32 - 5) / 2;
            let edge = x % wide == 0 || x % wide == wide - 1;
            let butt = y == joint || y == (joint + 1) % BOARDS;
            let shade = if edge || butt {
                BOARD_JOINT
            } else {
                (tone + grain).clamp(0, 255)
            } as u8;

            pixels.extend_from_slice(&[shade, shade, shade, 255]);
        }
    }

    TextureData::from_pixels(BOARDS, BOARDS, pixels)
}

pub const RUG: u32 = 512;
pub const RUG_TILES: f32 = 1.0;
const RUG_GROUND: [u8; 3] = [52, 20, 24];
const RUG_FIGURE: [u8; 3] = [122, 48, 44];
const RUG_THREAD: [u8; 3] = [168, 132, 72];

pub fn rug(long: f32) -> TextureData {
    let (wide, tall) = (RUG, (RUG as f32 * long).round() as u32);
    let mut pixels = Vec::with_capacity((wide * tall * 4) as usize);
    for _ in 0..wide * tall {
        pixels.extend_from_slice(&[RUG_GROUND[0], RUG_GROUND[1], RUG_GROUND[2], 255]);
    }

    // row major, which for a square rug it did not have to be: the old one
    // indexed x before y, which on a square is a transpose nobody can see in a
    // design symmetric about both axes. On a rug twice as long as it is wide it
    // is the picture sideways.
    let mut ink = |x: i32, y: i32, colour: [u8; 3]| {
        if x < 0 || y < 0 || x >= wide as i32 || y >= tall as i32 {
            return;
        }
        let n = ((y as u32 * wide + x as u32) * 4) as usize;
        pixels[n] = colour[0];
        pixels[n + 1] = colour[1];
        pixels[n + 2] = colour[2];
    };

    // a band round the outside, and a pair of threads inside it. Inset the same
    // number of pixels from every edge rather than the same fraction, which is
    // the whole reason this takes a shape: a square rug stretched over a room
    // twice as long as it is wide has a border twice as thick across the ends as
    // it is down the sides, and nothing else in the picture gives it away.
    const BANDS: [i32; 5] = [10, 11, 12, 20, 24];
    for &inset in &BANDS {
        for x in 0..wide as i32 {
            ink(x, inset, RUG_THREAD);
            ink(x, tall as i32 - 1 - inset, RUG_THREAD);
        }
        for y in 0..tall as i32 {
            ink(inset, y, RUG_THREAD);
            ink(wide as i32 - 1 - inset, y, RUG_THREAD);
        }
    }
    for inset in 13..20 {
        for x in 13..wide as i32 - 13 {
            ink(x, inset, RUG_FIGURE);
            ink(x, tall as i32 - 1 - inset, RUG_FIGURE);
        }
        for y in 13..tall as i32 - 13 {
            ink(inset, y, RUG_FIGURE);
            ink(wide as i32 - 1 - inset, y, RUG_FIGURE);
        }
    }

    // and one lozenge in the middle of it, which is what a rug has: a medallion
    // drawn three times over, each a little inside the last. It stretches with
    // the rug where the border does not, because a long rug has a long medallion
    // and a long border is just a mistake.
    let (mx, my) = (wide as i32 / 2, tall as i32 / 2);
    for (reach, colour) in [
        (wide as i32 * 7 / 20, RUG_THREAD),
        (wide as i32 * 6 / 20, RUG_FIGURE),
        (wide as i32 / 8, RUG_THREAD),
    ] {
        let (rx, ry) = (reach, (reach as f32 * long).round() as i32);
        let steps = rx.max(ry).max(1);
        for step in 0..=steps {
            let part = step as f32 / steps as f32;
            let (dx, dy) = ((rx as f32 * part) as i32, (ry as f32 * (1.0 - part)) as i32);

            for (x, y) in [
                (mx + dx, my + dy),
                (mx + dx, my - dy),
                (mx - dx, my + dy),
                (mx - dx, my - dy),
            ] {
                for thick in 0..3 {
                    ink(x, y + thick, colour);
                    ink(x + thick, y, colour);
                }
            }
        }
    }

    TextureData::from_pixels(wide, tall, pixels)
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

    /// Spec 0006: the boards run one way and are the same width all the way.
    #[test]
    fn the_boards_are_planks_and_not_a_grid() {
        let floor = boards(BOARD_SEED);
        let woven = &floor.levels[0].pixels;
        let at = |x: u32, y: u32| woven[((y * BOARDS + x) * 4) as usize];

        assert_eq!((floor.width(), floor.height()), (BOARDS, BOARDS));

        // a plank is a column, so the edge between two of them is a line of
        // constant x and shows up on every row
        let wide = BOARDS / PLANKS;
        for y in 0..BOARDS {
            for plank in 0..PLANKS {
                assert_eq!(
                    at(plank * wide, y) as i32,
                    BOARD_JOINT,
                    "the edge of plank {} breaks at row {}",
                    plank,
                    y
                );
            }
        }

        // and the butt joints are staggered, not laid in a row across the floor
        let joints: Vec<u32> = (0..PLANKS)
            .map(|plank| {
                let x = plank * wide + wide / 2;
                (0..BOARDS)
                    .find(|&y| at(x, y) as i32 == BOARD_JOINT)
                    .expect("a joint in every plank")
            })
            .collect();
        let same = joints.iter().filter(|&&y| y == joints[0]).count();

        assert!(
            same < joints.len(),
            "every plank joints at {}, which is a grid",
            joints[0]
        );
    }

    /// Spec 0006: a long rug has an even border, not a border that stretches.
    ///
    /// Drawn square and scaled onto a quad twice as long as it is wide, the band
    /// round the outside came out twice as thick across the ends as down the
    /// sides. Nothing else in the picture says which way it was stretched, so it
    /// reads as a badly made rug rather than as a bug.
    #[test]
    fn the_rug_keeps_its_border_even() {
        let long = 2.0;
        let rug = rug(long);
        let (wide, tall) = (RUG, (RUG as f32 * long) as u32);

        assert_eq!((rug.width(), rug.height()), (wide, tall));

        let woven = &rug.levels[0].pixels;
        let at = |x: u32, y: u32| {
            let n = ((y * wide + x) * 4) as usize;
            [woven[n], woven[n + 1], woven[n + 2]]
        };
        // in from each edge along the middle of that edge, to the first thread
        let from_left = (0..wide).find(|&x| at(x, tall / 2) == RUG_THREAD);
        let from_top = (0..tall).find(|&y| at(wide / 2, y) == RUG_THREAD);

        assert_eq!(from_left, Some(10), "the border down the side");
        assert_eq!(from_top, Some(10), "the border across the end");
        assert_eq!(from_left, from_top, "the border stretches with the rug");

        // and the far edges the same way in
        assert_eq!(
            (0..wide).find(|&x| at(wide - 1 - x, tall / 2) == RUG_THREAD),
            Some(10)
        );
        assert_eq!(
            (0..tall).find(|&y| at(wide / 2, tall - 1 - y) == RUG_THREAD),
            Some(10)
        );
    }

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
