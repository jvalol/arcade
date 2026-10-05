//! A globe on a bench. Spec 0006.
//!
//! The kind that stands in an office: a tilted sphere in a meridian ring, on a
//! stand, that you spin with a finger and watch run down.
//!
//! It is the one thing on a bench with no solver in it, and that is not
//! laziness. A sphere resists turning the same way about every axis, so there
//! is nothing for a solver to find out: the whole of its motion is one angle
//! and one rate, and the engine's own `turn` short circuits a sphere for the
//! same reason. What this toy is for is the other two things the engine does,
//! which is wrapping a picture on a surface and lighting it.
//!
//! The map is drawn rather than photographed, out of the only data in this
//! project that came from somewhere else. See `data/coastline.txt`.

use blitzkit::texture::TextureData;

/// What the bench it stands on is called.
pub const NAME: &str = "globe";

/// The world's land, carried as text and parsed once.
const LAND_RINGS: &str = include_str!("../data/land.txt");

/// How big the ball is, how far it leans, and how thick the meridian ring and
/// the stand are drawn.
///
/// The lean is the Earth's own, which is what makes a globe lean. Every office
/// globe in the world has it and almost nobody who owns one knows why it is not
/// upright.
pub const RADIUS: f32 = 0.16;
pub const TILT: f32 = 23.44;
pub const RING: f32 = 0.012;
pub const STAND: f32 = 0.022;

/// How high the ball's middle sits above the bench top.
pub const HIGH: f32 = 0.3;

/// How far the meridian ring stands off the ball, and how many pieces it is
/// drawn in.
pub const RING_OUT: f32 = 0.022;
pub const RING_PIECES: usize = 48;

/// How big the map is drawn, in texels.
///
/// Two to one, because an equirectangular map is: it runs 360 degrees across
/// and 180 down. At this a texel is a tenth of a degree, which is finer than
/// the land is recorded to.
pub const WIDE: u32 = 2048;
pub const TALL: u32 = 1024;

/// How fast a press of an arrow spins it, and how much of that it keeps a
/// second.
///
/// A globe runs down because its bearing rubs. Nothing here rubs, so this is
/// put in by hand, and it is the one number in the toy that is a choice rather
/// than a measurement.
pub const FLICK: f32 = 2.2;
pub const KEEPS: f32 = 0.55;

/// The sea, the land, the coast between them, the plain lines of latitude and
/// longitude, and the ones worth naming.
///
/// Paper and ink rather than a photograph from orbit, which is what the data
/// is: where the land is and nothing else. No rivers, no borders, no relief.
const SEA: [u8; 3] = [46, 86, 130];
const LAND: [u8; 3] = [226, 214, 182];
const COAST: [u8; 3] = [92, 76, 48];
const GRID: [u8; 3] = [150, 170, 192];
const NAMED: [u8; 3] = [196, 150, 70];

/// How far apart the plain lines of latitude and longitude are, in degrees.
pub const GRID_EVERY: f32 = 15.0;

/// Every ring of land, as longitude and latitude in degrees.
pub fn land() -> Vec<Vec<(f32, f32)>> {
    LAND_RINGS
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|pair| {
                    let (lon, lat) = pair.split_once(',')?;

                    Some((lon.parse().ok()?, lat.parse().ok()?))
                })
                .collect()
        })
        .collect()
}

/// Where a longitude and latitude land on the map.
///
/// Equirectangular, which is the layout the engine's own sphere is wrapped for:
/// its v runs from the north pole down and its u runs once round. Nothing is
/// projected, which is why the poles are smeared across the whole top and
/// bottom of the picture and look right on the ball.
pub fn at(lon: f32, lat: f32) -> (f32, f32) {
    (
        (lon + 180.0) / 360.0 * WIDE as f32,
        (90.0 - lat) / 180.0 * TALL as f32,
    )
}

/// The map: sea, land filled in, a grid over it, and the circles with names.
pub fn drawn() -> TextureData {
    let mut pixels = vec![0u8; (WIDE * TALL * 4) as usize];
    for texel in pixels.chunks_exact_mut(4) {
        texel[..3].copy_from_slice(&SEA);
        texel[3] = 255;
    }

    let rings: Vec<Vec<(f32, f32)>> = land()
        .into_iter()
        .map(|ring| ring.into_iter().map(|(lon, lat)| at(lon, lat)).collect())
        .collect();

    // the land, filled a row at a time: cross every edge with the row and paint
    // between the crossings in pairs. Rings wound either way fill the same,
    // which is what the odd and even counting is for.
    for y in 0..TALL {
        let row = y as f32 + 0.5;
        let mut cuts: Vec<f32> = Vec::new();

        for ring in &rings {
            for edge in ring.windows(2) {
                let (one, other) = (edge[0], edge[1]);
                if (one.1 > row) == (other.1 > row) {
                    continue;
                }
                let along = (row - one.1) / (other.1 - one.1);
                cuts.push(one.0 + along * (other.0 - one.0));
            }
        }

        cuts.sort_by(f32::total_cmp);
        for pair in cuts.chunks_exact(2) {
            let (from, to) = (pair[0].max(0.0) as u32, pair[1].min(WIDE as f32) as u32);
            for x in from..to {
                let at = ((y * WIDE + x) * 4) as usize;
                pixels[at..at + 3].copy_from_slice(&LAND);
            }
        }
    }

    let mut ink = |x: i64, y: i64, colour: [u8; 3]| {
        if y < 0 || y >= TALL as i64 {
            return;
        }
        // longitude wraps and latitude does not, which is the whole difference
        // between going round the world and falling off the top of it
        let x = x.rem_euclid(WIDE as i64);
        let at = ((y as u32 * WIDE + x as u32) * 4) as usize;
        pixels[at..at + 3].copy_from_slice(&colour);
    };

    // the coast, drawn round every ring so the fill has an edge
    for ring in &rings {
        for edge in ring.windows(2) {
            let (from, to) = (edge[0], edge[1]);
            let across = to.0 - from.0;
            let down = to.1 - from.1;
            let steps = across.abs().max(down.abs()).ceil().max(1.0);

            for step in 0..=steps as i64 {
                let on = step as f32 / steps;
                ink(
                    (from.0 + across * on) as i64,
                    (from.1 + down * on) as i64,
                    COAST,
                );
            }
        }
    }

    // and the grid over the top of it, then the circles that have names, so a
    // named one wins where they cross
    let mut rule = |lon: Option<f32>, lat: Option<f32>, colour: [u8; 3]| match (lon, lat) {
        (Some(lon), None) => {
            let (x, _) = at(lon, 0.0);
            for y in 0..TALL as i64 {
                if y % 4 != 3 {
                    ink(x as i64, y, colour);
                }
            }
        }
        (None, Some(lat)) => {
            let (_, y) = at(0.0, lat);
            for x in 0..WIDE as i64 {
                if x % 4 != 3 {
                    ink(x, y as i64, colour);
                }
            }
        }
        _ => (),
    };

    let mut line = -180.0;
    while line < 180.0 {
        rule(Some(line), None, GRID);
        line += GRID_EVERY;
    }
    let mut line = -90.0 + GRID_EVERY;
    while line < 90.0 {
        rule(None, Some(line), GRID);
        line += GRID_EVERY;
    }

    // the equator, the two tropics and the two polar circles. The tropics are
    // where the sun gets overhead and the polar circles are where it stops
    // setting, and both of them are the lean and nothing else: 23.44 out from
    // the middle, and 23.44 short of the pole.
    for lat in [0.0, TILT, -TILT, 90.0 - TILT, -(90.0 - TILT)] {
        rule(None, Some(lat), NAMED);
    }

    TextureData::from_pixels(WIDE, TALL, pixels)
}

/// The one on the bench: how far round it has been turned and how fast it is
/// still going.
pub struct Globe {
    pub turned: f32,
    pub spin: f32,
}

impl Default for Globe {
    fn default() -> Self {
        Self::new()
    }
}

impl Globe {
    pub fn new() -> Self {
        Self {
            turned: 0.0,
            spin: 0.0,
        }
    }

    /// A finger on the ball. Each press adds to whatever it is already doing,
    /// the way a real one does.
    pub fn flick(&mut self, way: f32) {
        self.spin += way * FLICK;
    }

    /// Turns it and lets it run down.
    pub fn advance(&mut self, dt: f32) {
        self.turned += self.spin * dt;
        self.spin *= KEEPS.powf(dt);

        if self.spin.abs() < 1e-3 {
            self.spin = 0.0;
        }
        self.turned = self.turned.rem_euclid(std::f32::consts::TAU);
    }

    /// Whether it is still going, which is what the bench says.
    pub fn going(&self) -> bool {
        self.spin != 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the map has at a place, as the texel it drew there.
    fn texel(map: &TextureData, lon: f32, lat: f32) -> [u8; 3] {
        let level = map.levels.first().expect("a picture");
        let (x, y) = at(lon, lat);
        let at = ((y as u32 * level.width + x as u32) * 4) as usize;

        [level.pixels[at], level.pixels[at + 1], level.pixels[at + 2]]
    }

    /// Spec 0006: the world is where it says it is.
    ///
    /// The one test worth having on a map. Twice now this project has laid a
    /// picture on a surface the wrong way round and found out by reading a word
    /// off it, and a globe has no word on it: a mirrored Earth, or one a quarter
    /// turn out, looks like an Earth. So the test names places and asks what is
    /// drawn there.
    #[test]
    fn the_world_is_where_it_says_it_is() {
        let map = drawn();

        // none of them on a round number, because the grid is drawn over the
        // map every fifteen degrees and a place on a rule reads as the rule
        for (lon, lat, what) in [
            (20.0, 5.0, "the Congo"),
            (-62.0, -8.0, "the Amazon"),
            (100.0, 47.0, "Mongolia"),
            (134.0, -24.0, "the middle of Australia"),
            (-103.0, 40.0, "Colorado"),
            (25.0, -78.0, "Antarctica"),
        ] {
            assert_eq!(texel(&map, lon, lat), LAND, "{} came out as sea", what);
        }

        for (lon, lat, what) in [
            (-140.0, 3.0, "the middle of the Pacific"),
            (-32.0, 40.0, "the middle of the Atlantic"),
            (77.0, -32.0, "the middle of the Indian ocean"),
            (2.0, 2.0, "the Gulf of Guinea"),
            (-148.0, 58.0, "the Gulf of Alaska"),
        ] {
            assert_eq!(texel(&map, lon, lat), SEA, "{} came out as land", what);
        }
    }

    /// Spec 0006: the map is the shape a wrapped sphere wants.
    #[test]
    fn the_map_is_two_to_one() {
        let map = drawn();
        let level = map.levels.first().expect("a picture");

        assert_eq!(level.width, WIDE);
        assert_eq!(level.height, TALL);
        assert_eq!(
            level.width,
            level.height * 2,
            "an equirectangular map runs 360 across and 180 down"
        );

        // and the corners are the poles smeared the whole way across
        assert_eq!(at(-180.0, 90.0), (0.0, 0.0));
        assert_eq!(at(180.0, -90.0), (WIDE as f32, TALL as f32));
    }

    /// Spec 0006: the circles with names are the lean and nothing else.
    #[test]
    fn the_named_circles_are_the_lean() {
        let map = drawn();

        for lat in [TILT, -TILT, 90.0 - TILT, -(90.0 - TILT)] {
            assert_eq!(
                texel(&map, -150.0, lat),
                NAMED,
                "the circle at {} is not drawn out at sea",
                lat
            );
        }

        // the tropics are the lean out from the middle and the polar circles
        // are the lean short of the pole, which is one number doing both
        assert!((TILT - 23.44).abs() < 1e-4, "the lean is not the Earth's");
        assert_eq!(90.0 - TILT, 66.56, "the polar circle is not the lean");
    }

    /// Spec 0006: the land reads as the land it was given.
    #[test]
    fn the_land_is_all_there() {
        let rings = land();

        assert_eq!(rings.len(), 127, "the land has the wrong number of rings");
        assert!(
            rings.iter().map(|ring| ring.len()).sum::<usize>() > 5000,
            "the land is thinner than it was given"
        );

        for ring in &rings {
            assert!(ring.len() > 3, "a ring of {} points is not one", ring.len());
            assert_eq!(
                ring.first(),
                ring.last(),
                "a ring does not come back to where it started"
            );
            for (lon, lat) in ring {
                assert!(
                    (-180.0..=180.0).contains(lon) && (-90.0..=90.0).contains(lat),
                    "a point at {} {} is off the world",
                    lon,
                    lat
                );
            }
        }
    }

    /// Spec 0006: flicked, it spins and runs down.
    #[test]
    fn it_runs_down() {
        let mut one = Globe::new();
        assert!(!one.going(), "it was turning before it was touched");

        one.flick(1.0);
        assert!(one.going());

        let was = one.spin;
        for _ in 0..60 {
            one.advance(1.0 / 60.0);
        }

        assert!(one.turned > 0.5, "it only turned {}", one.turned);
        assert!(
            one.spin < was * 0.7,
            "after a second it is still doing {} of {}",
            one.spin,
            was
        );

        for _ in 0..(20.0 * 60.0) as u32 {
            one.advance(1.0 / 60.0);
        }
        assert!(!one.going(), "it never stopped");

        // and a second flick adds to the first, the way a finger does
        one.flick(1.0);
        let once = one.spin;
        one.flick(1.0);
        assert!(one.spin > once, "a second flick did nothing");
    }

    /// Writes the map out so it can be looked at.
    #[test]
    #[ignore]
    fn draw_it() {
        let map = drawn();
        let level = map.levels.first().expect("a picture");
        std::fs::write("/tmp/globe.rgba", &level.pixels).expect("written");
        println!("{} by {} written", level.width, level.height);
    }
}
