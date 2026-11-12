//! The moon over the garden: where it stands, how big it is drawn, which way
//! it faces, and what is written on it. Spec 0065.
//!
//! **It is not in a room.** `Room::out_of_doors` is what draws it, so it hangs
//! over the garden, the field, the bridge and Valhalla's doorstep all at once
//! and over nothing with a lid on it. There is no wall to hang it on and
//! nothing in the plan says where it is.
//!
//! **The face is worked out rather than painted**, the way the carpet, the
//! wood grain and the night under Valhalla's ridge are. Highlands with a
//! saturated crater field on them, the near side's real seas flooded over
//! that, then what fell afterwards, then the ray systems. What it costs is
//! one bake at load, and what it buys is a body rather than a disc.
//!
//! Three things here are lies, and each one is here because the truth is
//! worse to look at. They are written up where they are told: `SUN_OFF`,
//! `EARTHSHINE` and `turned`.

use std::f32::consts::{PI, TAU};

use glam::{vec3, Quat, Vec3, Vec4};

/// How far off it hangs.
///
/// Further than anything in the field and well inside `field::SEES`, so it is
/// never clipped and nothing out there stands in front of it.
pub const OFF: f32 = 230.0;

/// How wide it is drawn, in metres at `OFF`.
///
/// **Measured rather than chosen.** A moon 16 wide at 230 is four degrees
/// across, which came to 43 pixels in a 900 by 720 frame, and at 43 pixels not
/// one crater on it reads: the seas are the whole of what is visible. Craters
/// start reading at about 180 pixels and the terminator at about 90. Eighty
/// metres is 20 degrees across and comes to 188, which is the bottom of where
/// craters read, and it is the number to change if the moon is ever the point
/// of a room rather than what is over one.
///
/// Still a lie about the size, as the 16 was. The real one is half a degree.
pub const WIDE: f32 = 80.0;

/// Which way the near side looks before it is turned.
///
/// `dir` puts `u` of nought on the positive x axis, so that is the middle of
/// the face and everything else here is measured from it.
pub const FACE: Vec3 = Vec3::X;

/// Where it stands: the sun's own place turned half a day, with a lean of its
/// own so the two do not run the same line across the sky.
///
/// Jake: throw a moon in there too.
pub fn at(since: f32) -> Vec3 {
    let turn = crate::garden::hour(since) * TAU + PI;

    vec3(turn.cos(), turn.sin(), -0.34).normalize()
}

/// How far below the horizon the moon's own light is gone, and over how much
/// of a drop it goes.
///
/// **A moon has to set behind something, not fade in mid-air.** This put it
/// out at a height of 0.14, which is eight degrees up, and the field's
/// horizon stands about five, so it died just short of the hills every night.
/// Jake, 2026-11-12: it sort of melted into the background hills instead of
/// setting behind them.
///
/// It keeps its light now until its middle is just under the horizon, and the
/// hills and the ground do the hiding, which is what they are there for. It
/// is out by the time the whole disc is under: the ground's own far edge sits
/// half a degree below the horizon from eye height, and a moon `WIDE` across
/// at `OFF` is ten degrees of radius, so this is well past where anything
/// could show under the land.
pub const SETS_AT: f32 = -0.05;
pub const SETS_OVER: f32 = 0.20;

/// Where in the sun's own climb the moon's light goes, as the sun's height.
///
/// **The other half of why it melted.** `at` hangs the moon opposite the sun,
/// so the moon is low exactly when the sun is near the horizon and a moonset
/// here is always a twilight. This was a straight ramp across the sun's whole
/// range and squared, which left the moon at a quarter of its light at the
/// moment it reached the hills, against a sky that is still bright. Fixing
/// the height alone would have moved the melt down rather than stopped it.
///
/// It holds its light until the sun is properly climbing and is gone by the
/// time the day is up, which is also what a full moon does at dawn.
pub const DAWNS_AT: f32 = -0.05;
pub const DAWNS_BY: f32 = 0.25;

/// How much of it there is to see: all of it through the night, out as the
/// sun comes up, and gone once it is under the land.
///
/// Read off the moon's own height rather than the sun's, so a change to
/// either one's lean carries.
pub fn showing(since: f32) -> f32 {
    let up = ((at(since).y - SETS_AT) / SETS_OVER + 1.0).clamp(0.0, 1.0);
    let dark = 1.0 - smoothstep(DAWNS_AT, DAWNS_BY, crate::garden::sun_up(since));

    up * dark
}

/// What to hand the engine so the face lands at `crate::aim::MOON` whatever the hour.
///
/// **Through `gives_off`, which the old number was not.** A flat 28 times was
/// what it took to read at midnight, and 28 times a night fill of 0.09 blue is
/// 2.5: measured in the garden, the blue channel sat at 255 across 1407 of the
/// moon's 1483 pixels, so a third of the face's range was gone before anything
/// was drawn on it. Dividing the fill out instead puts the whole range back
/// and says what lands rather than what multiplies.
///
/// **The fade goes on the colour and not on the alpha.** Taking the fourth
/// channel with it put a half transparent moon in the sky with the storm
/// showing through it.
pub fn lit(since: f32) -> Vec4 {
    crate::garden::gives_off(crate::aim::MOON * showing(since), since).extend(1.0)
}

/// Which way to turn the ball so the same face keeps to the eye.
///
/// **The moon is tidally locked and this one was not.** It is hung off the eye
/// in the direction `at` gives, with no rotation at all, so the hemisphere
/// pointed at you swung through the night as the moon crossed the sky. With
/// nothing on the face nobody could tell. With the near side's seas on it the
/// whole pattern would turn away, which is the one thing about the moon that
/// everybody would notice.
///
/// The roll is whatever the shortest turn gives, because there is no up out
/// there to hold it to.
pub fn turned(toward: Vec3) -> Quat {
    Quat::from_rotation_arc(FACE, toward.normalize())
}

/// Where a point of the ball is, for a given place on its picture.
///
/// **One account of it**, because the face is baked by asking this and the
/// mesh is built by asking this, and a texture that disagrees with its own
/// mesh about which way is north is a moon with its seas on the back.
pub fn dir(u: f32, v: f32) -> Vec3 {
    let round = -u * TAU;
    let up = v * PI;

    vec3(round.cos() * up.sin(), -up.cos(), round.sin() * up.sin())
}

/// Where a point of the ball falls on its picture, which is `dir` run
/// backwards.
///
/// **Here because `crater` worked it out for itself and got it wrong.** It
/// had `0.5 - lon / TAU` and `0.5 - lat / PI`, which between them name the
/// point opposite the one asked for, so every crater's footprint was laid out
/// over its own antipode and nothing inside it was near enough to draw. Not
/// one crater was ever written to the ground, through four rounds of looking
/// at a face that would not show any relief and twice deciding the picture was
/// too small for them.
pub fn uv(here: Vec3) -> (f32, f32) {
    (
        (-here.z.atan2(here.x) / TAU).rem_euclid(1.0),
        (here.y.clamp(-1.0, 1.0).asin() / PI + 0.5).clamp(0.0, 1.0),
    )
}

/// A point on the ball from the way round and the way up, in radians, with
/// nought and nought the middle of the near side.
pub fn place(lon: f32, lat: f32) -> Vec3 {
    vec3(lon.cos() * lat.cos(), lat.sin(), lon.sin() * lat.cos())
}

/// Which way is east and which way is north at a point, as a pair of unit
/// vectors.
///
/// **One place, because it was written twice and both were wrong.** These came
/// over from the spike, which measured the way round from the positive z axis
/// where `place` measures it from the positive x. Carried across unchanged,
/// east came out pointing straight up out of the ground at the middle of the
/// face and north had its x and z the wrong way about, so every slope on the
/// moon tilted somewhere other than where it should have.
pub fn basis(lon: f32, lat: f32) -> (Vec3, Vec3) {
    (
        vec3(-lon.sin(), 0.0, lon.cos()),
        vec3(-lat.sin() * lon.cos(), lat.cos(), -lat.sin() * lon.sin()),
    )
}

/// The ball.
///
/// Sixty-four round and thirty-two up, where it was forty and twenty-four. The
/// moon is drawn five times wider than it was and a straight edge on the only
/// round thing in the sky is the first thing anybody would see.
pub fn mesh() -> blitzkit::mesh::MeshData {
    let mut mesh = blitzkit::mesh::MeshData::surface(64, 32, |u, v| dir(u, v) * 0.5);
    mesh.compute_normals();

    mesh
}

// --- what is written on it --------------------------------------------------

/// How big the picture of the face is.
///
/// Sized by what a crater needs rather than by the disc it ends up as. At
/// 1024 by 512 the craters come out chunky and sparse, because a thirty
/// kilometre one is a step and a half across there and a ring that thin has
/// nothing left after the renderer has filtered it. Four times that is where
/// the ground stops looking like bubbles.
///
/// It costs 540 milliseconds at load, which is the most expensive thing in
/// this file by a long way and the number to come back to if the browser
/// build starts feeling slow. Halving it is one line.
pub const PICTURE_WIDE: usize = 2048;
pub const PICTURE_TALL: usize = 1024;

/// How far round from the middle of the face is worth drawing at all, in
/// degrees.
///
/// **Because the far side is never seen and it is half the bake.** The moon
/// keeps one face to the eye, and a ball of radius 40 at 230 metres shows the
/// cap within 80 degrees of the middle of that face. A hundred leaves twenty
/// degrees of margin past anything a camera can reach, and takes 44 percent
/// of the picture out of every pass over it.
///
/// What is left outside is written flat. A step on that boundary reads a
/// cliff when the slope is worked out, and the boundary is ten degrees round
/// the back.
pub const DRAWN_OUT: f32 = 100.0;

/// Which columns of the picture that is, as `dir` runs the way round
/// backwards and wraps.
fn drawn_columns() -> (usize, usize) {
    let share = DRAWN_OUT / 360.0;
    let half = (share * PICTURE_WIDE as f32).ceil() as usize;

    (half, PICTURE_WIDE - half)
}

/// Degrees across the picture per step of it, which is the finest thing that
/// can be written on it.
fn step() -> f32 {
    PI / PICTURE_TALL as f32
}

/// The radius of the body in kilometres, which is the only real number in this
/// file and is here so the seas and the craters can be given their real sizes.
pub const RADIUS_KM: f32 = 1737.0;

/// Kilometres across to radians of radius.
fn across(km: f32) -> f32 {
    km / 2.0 / RADIUS_KM
}

/// The near side's seas, in degrees round and up, with a diameter in
/// kilometres.
///
/// **Roughly where they really are**, because the near side's pattern is the
/// one thing about the moon that anybody can already recognise, and a set of
/// blobs in the wrong places is a planet rather than the moon. Procellarum and
/// Frigoris are each several entries, because neither is round and the shore
/// here is drawn round the union of these rather than round each one.
const SEAS: [(f32, f32, f32); 17] = [
    (-57.0, 19.0, 1400.0), // Procellarum, in three
    (-62.0, -2.0, 1120.0),
    (-45.0, 36.0, 840.0),
    (-16.0, 33.0, 1145.0), // Imbrium
    (17.0, 28.0, 674.0),   // Serenitatis
    (31.0, 9.0, 873.0),    // Tranquillitatis
    (54.0, -8.0, 909.0),   // Fecunditatis
    (59.0, 17.0, 418.0),   // Crisium
    (34.0, -15.0, 333.0),  // Nectaris
    (-39.0, -24.0, 389.0), // Humorum
    (-16.0, -21.0, 714.0), // Nubium
    (3.0, 13.0, 245.0),    // Vaporum
    (-23.0, -10.0, 380.0), // Cognitum and Insularum as one
    (-25.0, 56.0, 600.0),  // Frigoris, in three
    (0.0, 58.0, 600.0),
    (25.0, 56.0, 520.0),
    (87.0, -3.0, 460.0), // Smythii, on the limb
];

/// The craters worth putting where they belong: the ones with names you would
/// know, in degrees round and up, with a diameter in kilometres and whether
/// they are young enough to still be bright and throw rays.
const NAMED: [(f32, f32, f32, bool); 12] = [
    (-11.0, -43.0, 86.0, true),   // Tycho
    (-20.0, 10.0, 93.0, true),    // Copernicus
    (-38.0, 8.0, 31.0, true),     // Kepler
    (-47.0, 24.0, 40.0, true),    // Aristarchus
    (-14.0, -58.0, 231.0, false), // Clavius
    (-9.0, 51.0, 101.0, false),   // Plato
    (-68.0, -6.0, 173.0, false),  // Grimaldi
    (-2.0, -9.0, 118.0, true),    // Alphonsus
    (6.0, -16.0, 109.0, false),   // Albategnius
    (-17.0, -32.0, 118.0, false), // Deslandres
    (60.0, -38.0, 222.0, false),  // Janssen
    (-37.0, -43.0, 118.0, false), // Schiller, round here
];

/// How many craters are thrown at it before the seas and after them.
///
/// **A power law, which is what a crater count is.** Twice the size is a
/// quarter as many, so most of these are the smallest there is and the few
/// large ones are what you see.
///
/// **The first four bakes had no craters on them at all** and neither of the
/// two things that looked like the cause was. The face came back smooth, so
/// the picture was doubled and the count raised sevenfold, and it drew the
/// same face; then the sun was moved twice, and that did bring relief out at
/// the terminator, which made the lighting look like the answer. Both were
/// red herrings off one bug: `crater` worked out where on the picture to
/// write and named the antipode, so every footprint was laid over the far
/// side of the moon and not one crater was ever cut. See `uv`.
pub const CRATERS: usize = 45_000;
pub const SMALLEST_KM: f32 = 6.0;
pub const LARGEST_KM: f32 = 320.0;

/// What share of them fell before the seas came up, and so were drowned.
pub const DROWNED: f32 = 0.88;

/// A hash of a place in space.
///
/// Three dimensional rather than the two every other picture in this building
/// uses, because this one is wrapped round a ball: a noise worked out from the
/// picture's own corners has a seam down the back of the moon and a knot at
/// each pole, and one worked out from where the point actually is has neither.
fn hashed(x: i32, y: i32, z: i32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8DA6_B343)
        ^ (y as u32).wrapping_mul(0xD816_3841)
        ^ (z as u32).wrapping_mul(0xCB1A_B31F);
    h ^= h >> 13;
    h = h.wrapping_mul(0x2545_F491);
    h ^= h >> 16;

    (h & 0x00FF_FFFF) as f32 / 16_777_216.0
}

/// Value noise at a point, smoothed between the eight corners around it.
fn noise(p: Vec3) -> f32 {
    let base = p.floor();
    let f = p - base;
    let s = f * f * (Vec3::splat(3.0) - 2.0 * f);
    let (xi, yi, zi) = (base.x as i32, base.y as i32, base.z as i32);
    let mut out = 0.0;

    for dx in 0..2 {
        for dy in 0..2 {
            for dz in 0..2 {
                let w = (if dx == 1 { s.x } else { 1.0 - s.x })
                    * (if dy == 1 { s.y } else { 1.0 - s.y })
                    * (if dz == 1 { s.z } else { 1.0 - s.z });

                out += w * hashed(xi + dx, yi + dy, zi + dz);
            }
        }
    }

    out * 2.0 - 1.0
}

/// The same at several sizes at once, which is what makes ground rather than
/// lumps.
fn fractal(p: Vec3, octaves: usize, scale: f32) -> f32 {
    let (mut out, mut amp, mut all, mut at) = (0.0, 1.0, 0.0, scale);

    for _ in 0..octaves {
        out += amp * noise(p * at);
        all += amp;
        amp *= 0.5;
        at *= 2.07;
    }

    out / all
}

fn smoothstep(low: f32, high: f32, x: f32) -> f32 {
    let t = ((x - low) / (high - low)).clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

/// How deep a crater of this radius is, in radii of the body.
///
/// Depth over diameter runs about a fifth for the small ones and a twelfth for
/// the big, which is why this is not a straight multiple.
fn deep(radius: f32) -> f32 {
    0.25 * radius / (1.0 + 12.0 * radius)
}

/// The seas worked out once, as where each one is and how far it reaches.
///
/// **Because seventeen of them times half a million steps is seventeen
/// million sines.** Reading `SEAS` inside the loop and working the places out
/// there cost 29 milliseconds of a 226 millisecond bake, and all it was doing
/// was the same seventeen points over and over.
pub fn basins() -> Vec<(Vec3, f32)> {
    SEAS.iter()
        .map(|(lon, lat, km)| (place(lon.to_radians(), lat.to_radians()), across(*km)))
        .collect()
}

/// How much sea is at a point: nought on the highlands, one well inside a
/// basin, and the shore in between.
///
/// **Taken off the union of the basins rather than each one in turn.** A shore
/// drawn round every basin separately is a row of circles however much noise is
/// put on it, and the seas that read as the moon's are the ones that run into
/// each other.
pub fn sea_among(basins: &[(Vec3, f32)], here: Vec3) -> f32 {
    let mut most = f32::NEG_INFINITY;

    for (middle, reach) in basins.iter().copied() {
        let angle = here.dot(middle).clamp(-1.0, 1.0).acos();

        most = most.max(1.0 - angle / reach);
    }

    // and then the shore is that contour pushed about, because the thing that
    // gives a painted moon away is a circle
    let wobble = 0.62 * fractal(here, 3, 2.4)
        + 0.26 * fractal(here + vec3(9.0, 0.0, 0.0), 2, 6.5)
        + 0.10 * fractal(here + vec3(0.0, 0.0, 5.0), 2, 18.0);

    smoothstep(-0.03, 0.07, most + wobble)
}

/// The ground, as a picture of what colour it is and a picture of how high it
/// stands.
struct Ground {
    albedo: Vec<f32>,
    height: Vec<f32>,
}

impl Ground {
    fn blank() -> Self {
        Ground {
            albedo: vec![0.0; PICTURE_WIDE * PICTURE_TALL],
            height: vec![0.0; PICTURE_WIDE * PICTURE_TALL],
        }
    }

    fn at(&self, column: usize, row: usize) -> usize {
        row.min(PICTURE_TALL - 1) * PICTURE_WIDE + column % PICTURE_WIDE
    }

    /// Where a step of the picture is on the ball.
    fn point(&self, column: usize, row: usize) -> Vec3 {
        dir(
            (column as f32 + 0.5) / PICTURE_WIDE as f32,
            (row as f32 + 0.5) / PICTURE_TALL as f32,
        )
    }
}

/// Lays one crater into the ground.
///
/// Everything inside the rim is replaced and everything outside it is added,
/// which is what a later impact does to an earlier one.
///
/// `age` is nought for one that fell this morning and one for one that has
/// been hit a hundred times since. Without it every crater is the same age and
/// the ground reads as a golf ball.
fn crater(ground: &mut Ground, lon: f32, lat: f32, radius: f32, young: bool, rays: bool, age: f32) {
    let middle = place(lon, lat);

    // one that cannot reach the drawn part of the picture, which is most of
    // them, since most of the moon is round the back
    if middle.dot(FACE).acos().to_degrees() - radius.to_degrees() * 9.0 > DRAWN_OUT {
        return;
    }

    let (east, north) = basis(lon, lat);
    let worn = 1.0 - age;
    let depth = deep(radius) * (0.22 + 0.78 * worn);
    let rim = depth * 0.35 * worn;
    let wall = 0.78 - 0.26 * age;
    let reach = if rays { 9.0 } else { 2.6 };
    let out = radius * reach;

    // only the part of the picture this one can touch, or twelve thousand
    // craters is twelve thousand passes over the whole of it
    let (u, v) = uv(middle);
    let rows = (out / step()).ceil() as i32 + 2;
    let row0 = (v * PICTURE_TALL as f32) as i32;
    let span = if lat.cos() < 0.08 {
        PI
    } else {
        (out / lat.cos()).min(PI)
    };
    let columns = (span / TAU * PICTURE_WIDE as f32).ceil() as i32 + 2;
    let column0 = (u * PICTURE_WIDE as f32) as i32;

    for row in (row0 - rows).max(0)..(row0 + rows).min(PICTURE_TALL as i32) {
        for step_over in -columns..=columns {
            let column = (column0 + step_over).rem_euclid(PICTURE_WIDE as i32) as usize;
            let row = row as usize;
            let here = ground.point(column, row);
            let angle = here.dot(middle).clamp(-1.0, 1.0).acos();
            let at = ground.at(column, row);
            // a rim that is not a circle
            let wobble = 1.0 + 0.09 * fractal(here, 3, 60.0 / radius.max(0.004));
            let r = angle / radius / wobble;

            if r > reach {
                continue;
            }

            if r < 1.0 {
                let shape = if r < wall {
                    let mut floor = -depth;

                    if radius > 0.012 && age < 0.55 && r < 0.16 {
                        floor += depth * 0.85 * (1.0 - r / 0.16) * worn;
                    }

                    floor
                } else {
                    -depth + (depth + rim) * smoothstep(wall, 1.0, r)
                };

                ground.height[at] = ground.height[at] * (0.22 + 0.5 * age) + shape;
            } else if r < reach.min(2.6) {
                let t = (r - 1.0) / 1.6;

                ground.height[at] += rim * (1.0 - t).max(0.0).powf(2.2) * worn;
            }

            if young && rays && r > 1.1 {
                ground.albedo[at] += ray_at(here, middle, east, north, r, reach);
            }

            // fresh ground is brighter whatever hit it, which is most of what
            // there is to see on a face with the sun behind the eye
            if r < 1.25 {
                ground.albedo[at] += 0.09 * worn * (1.0 - smoothstep(0.85, 1.25, r));
            }

            if r < wall {
                ground.albedo[at] *= 1.0 - 0.05 * age;
            }
        }
    }
}

/// How much brighter a ray system makes a point outside a young crater.
///
/// **The bearing is taken along the ball and not across the picture.** Off the
/// difference of the two points it comes out as a four pointed star, which
/// reads as a lens flare rather than as rays.
fn ray_at(here: Vec3, middle: Vec3, east: Vec3, north: Vec3, r: f32, reach: f32) -> f32 {
    let tangent = (here - middle * here.dot(middle)).normalize_or_zero();
    let turn = tangent.dot(east).atan2(tangent.dot(north));
    let (c, s) = (turn.cos(), turn.sin());
    let streak = (noise(vec3(c * 13.0, s * 13.0, 2.0)) * 0.6
        + noise(vec3(c * 31.0, s * 31.0, 5.0)) * 0.3
        + noise(vec3(c * 67.0, s * 67.0, 9.0)) * 0.17)
        / 1.07;
    // and broken along their length, so they are rays and not wedges
    let broken = 0.55 + 0.45 * fractal(here, 3, 90.0);
    let along = (1.0 - smoothstep(1.1, reach, r)).powf(1.3);

    0.40 * smoothstep(0.26, 0.78, streak) * broken * along
}

/// The ground, built in the order it happened: highlands, what fell on them,
/// the seas flooding over that, and what has fallen since.
fn ground() -> Ground {
    let mut ground = Ground::blank();

    let (near, far) = drawn_columns();

    for row in 0..PICTURE_TALL {
        for column in (0..PICTURE_WIDE).filter(|c| *c < near || *c > far) {
            let here = ground.point(column, row);
            let at = row * PICTURE_WIDE + column;

            ground.height[at] = 0.004 * fractal(here, 4, 3.0);
            ground.albedo[at] = 0.90 + 0.05 * fractal(here, 3, 22.0);
        }
    }

    let mut which = 0u32;
    let mut pull = || {
        which = which.wrapping_add(1);

        hashed(which as i32, 7, 11)
    };
    let mut fell: Vec<(f32, f32, f32, bool, f32)> = Vec::with_capacity(CRATERS);

    for _ in 0..CRATERS {
        let lon = (pull() - 0.5) * TAU;
        let lat = (pull() * 2.0 - 1.0).asin();
        let size = across(SMALLEST_KM / pull().max(1e-4).sqrt()).min(across(LARGEST_KM));
        let young = pull() < 0.05;
        let age = pull().powf(1.6);

        fell.push((lon, lat, size, young, age));
    }

    let drowned = (CRATERS as f32 * DROWNED) as usize;

    // the old ground is saturated and worn: everything on it has been hit
    // again since, so little of it still has a rim
    for (lon, lat, size, young, age) in fell[..drowned].iter().copied() {
        crater(&mut ground, lon, lat, size, young, false, 0.15 + 0.85 * age);
    }

    flood(&mut ground);

    // and what has fallen since, which is thin on the seas because they are
    // the youngest ground there is
    let basins = basins();

    for (lon, lat, size, young, age) in fell[drowned..].iter().copied() {
        let on_a_sea = sea_among(&basins, place(lon, lat)) > 0.4;

        if on_a_sea && hashed((lon * 1000.0) as i32, (lat * 1000.0) as i32, 3) < 0.55 {
            continue;
        }

        crater(&mut ground, lon, lat, size, young, false, age * 0.55);
    }

    for (lon, lat, km, young) in NAMED.iter().copied() {
        crater(
            &mut ground,
            lon.to_radians(),
            lat.to_radians(),
            across(km),
            young,
            young && km > 25.0,
            if young { 0.05 } else { 0.45 },
        );
    }

    ground
}

/// The seas coming up over whatever was there.
fn flood(ground: &mut Ground) {
    let basins = basins();
    let (near, far) = drawn_columns();

    for row in 0..PICTURE_TALL {
        for column in (0..PICTURE_WIDE).filter(|c| *c < near || *c > far) {
            let here = ground.point(column, row);
            let at = row * PICTURE_WIDE + column;
            let flooded = sea_among(&basins, here);

            if flooded <= 0.001 {
                continue;
            }

            let level = -0.0028 + 0.0004 * fractal(here, 3, 14.0);
            // a sea is not one grey: the shallows are paler and there are
            // shades in the middle of every one of them
            let shade = 0.485
                + 0.055 * fractal(here, 3, 7.0)
                + 0.030 * fractal(here + vec3(0.0, 4.0, 0.0), 2, 26.0);

            ground.height[at] = ground.height[at] * (1.0 - flooded) + level * flooded;
            ground.albedo[at] = ground.albedo[at] * (1.0 - flooded) + shade * flooded;

            // wrinkle ridges, which are what a sea has instead of craters.
            // Nothing in flat light and the whole character of the thing when
            // the sun is low on it.
            let ridge = 1.0 - fractal(here, 3, 13.0).abs();

            ground.height[at] += flooded * 0.0009 * smoothstep(0.76, 0.99, ridge);
        }
    }
}

// --- the light on it --------------------------------------------------------

/// How far round from the eye the sun is put, in degrees.
///
/// **A lie, and the one this whole file turns on.** `at` hangs the moon
/// opposite the sun, so the moon over this garden is full every night, and a
/// full moon is a flat disc: at no phase angle does any crater on it cast a
/// shadow, which is why the real one looks pasted on. Every crater you can see
/// here is the sun being somewhere it is not.
///
/// Sixty degrees is a fat gibbous: the terminator is on the face rather than
/// at the limb, which is where the relief is, and three quarters of the disc
/// is still lit. Pushed out from 48 while chasing a flat face that had a
/// different cause, and kept on its own merits once the craters arrived.
pub const SUN_OFF: f32 = 60.0;

/// And how far above the eye, so the shadows fall down the face rather than
/// straight across it.
pub const SUN_UP: f32 = 0.18;

/// The least the face is ever drawn at, as a share of full sun.
///
/// The dark side of a real moon is lit by the earth, and this is that: without
/// it the unlit part is a hole cut in the sky at a sharp edge. Under the
/// night sky's own 0.055, so the dark limb reads as a body in front of the sky
/// rather than as a light.
pub const EARTHSHINE: f32 = 0.03;

/// Where the sun is, in the ball's own frame.
pub fn sun() -> Vec3 {
    let off = SUN_OFF.to_radians();

    vec3(off.cos(), SUN_UP, -off.sin()).normalize()
}

/// How bright a point of the ground is, given which way it faces, which way
/// the eye is and what colour it is.
///
/// **Lommel-Seeliger rather than Lambert.** The moon scatters back the way the
/// light came, which is why a full moon is a disc of one brightness instead of
/// a shaded ball, and why Lambert's cosine puts a dark ring round the limb
/// that nobody has ever seen up there. The eighth of Lambert left in is what
/// darkens the limb by the little that it really does.
fn how_lit(normal: Vec3, albedo: f32) -> f32 {
    let towards_sun = normal.dot(sun()).max(0.0);
    let towards_eye = normal.dot(FACE).max(0.02);
    let share = 0.88;
    let lit = share * (2.0 * towards_sun / (towards_sun + towards_eye).max(1e-3)).min(2.0)
        + (1.0 - share) * towards_sun;

    albedo * lit.max(EARTHSHINE) * 0.86
}

/// The face: the ground, with the light already on it.
///
/// **Baked rather than lit by the engine**, because a mesh here carries a
/// colour and a picture and nothing that says which way its surface tilts. The
/// relief is the whole of the detail, so the relief goes in the picture.
///
/// That is only honest because `turned` keeps one face to the eye: a shading
/// worked out for one direction and then turned away from it would be a moon
/// lit from inside.
pub fn face() -> blitzkit::texture::TextureData {
    let ground = ground();
    let mut pixels = Vec::with_capacity(PICTURE_WIDE * PICTURE_TALL * 4);

    let (near, far) = drawn_columns();

    for row in 0..PICTURE_TALL {
        for column in 0..PICTURE_WIDE {
            if column >= near && column <= far {
                pixels.extend_from_slice(&[0, 0, 0, 255]);

                continue;
            }

            let at = row * PICTURE_WIDE + column;
            let here = ground.point(column, row);
            let shade = how_lit(tilt(&ground, column, row, here), ground.albedo[at]);
            // the picture is read back out as it was written, so what is
            // stored is what is shown and the bend belongs here
            let grey = (shade.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;

            pixels.extend_from_slice(&[grey, grey, grey, 255]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(PICTURE_WIDE as u32, PICTURE_TALL as u32, pixels)
}

/// Which way the ground faces at a step of the picture, off the slope of the
/// height either side of it.
///
/// The height is in radii of the body and the distance across one step is an
/// angle, so the two divide straight into a slope with nothing to convert.
fn tilt(ground: &Ground, column: usize, row: usize, here: Vec3) -> Vec3 {
    let lat = here.y.clamp(-1.0, 1.0).asin();
    let lon = here.z.atan2(here.x);
    let read = |c: i32, r: i32| {
        ground.height[ground.at(
            (c.rem_euclid(PICTURE_WIDE as i32)) as usize,
            (r.clamp(0, PICTURE_TALL as i32 - 1)) as usize,
        )]
    };
    let (c, r) = (column as i32, row as i32);
    // `dir` runs the picture the other way round from `place`, so east is the
    // step before this one rather than after it
    let east = (read(c - 1, r) - read(c + 1, r)) / (2.0 * step() * lat.cos().max(0.05));
    // and row nought is the south pole, so up the picture is south
    let north = (read(c, r + 1) - read(c, r - 1)) / (2.0 * step());
    let (along, up) = basis(lon, lat);

    (here - along * east - up * north).normalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The face is built once here, because it is a fifth of a second and
    /// every test below wants the same one.
    fn baked() -> blitzkit::texture::TextureData {
        face()
    }

    /// How bright the face is at a place on the ball, as the picture holds it.
    fn read(picture: &blitzkit::texture::TextureData, lon: f32, lat: f32) -> f32 {
        let (u, v) = uv(place(lon.to_radians(), lat.to_radians()));
        let column = ((u * PICTURE_WIDE as f32) as usize).min(PICTURE_WIDE - 1);
        let row = ((v * PICTURE_TALL as f32) as usize).min(PICTURE_TALL - 1);

        picture.levels[0].pixels[(row * PICTURE_WIDE + column) * 4] as f32 / 255.0
    }

    /// A place on the picture and a place on the ball are the same place, both
    /// ways round.
    ///
    /// **This is the test that was missing.** `crater` had its own arithmetic
    /// for the way back and it named the antipode, so nothing it drew landed
    /// anywhere near where it was asked for, and nothing said so.
    #[test]
    fn the_picture_and_the_ball_are_the_same_ball() {
        let places: [(f32, f32); 5] = [
            (0.0, 0.0),
            (37.0, 18.0),
            (-80.0, -44.0),
            (150.0, 70.0),
            (-11.0, -43.0),
        ];

        for (lon, lat) in places {
            let want = place(lon.to_radians(), lat.to_radians());
            let (u, v) = uv(want);
            let got = dir(u, v);

            assert!(
                got.distance(want) < 1e-4,
                "{} {} went out to {} {} and came back {} instead of {}",
                lon,
                lat,
                u,
                v,
                got,
                want
            );
        }

        for (u, v) in [(0.0f32, 0.5f32), (0.25, 0.2), (0.6, 0.8), (0.99, 0.5)] {
            let (back_u, back_v) = uv(dir(u, v));

            assert!(
                (back_u - u).abs() < 1e-4 && (back_v - v).abs() < 1e-4,
                "{} {} came back as {} {}",
                u,
                v,
                back_u,
                back_v
            );
        }
    }

    /// And a crater is actually cut into the ground where it was asked for.
    ///
    /// Tycho, which is the one with the rays. Averaged round two rings rather
    /// than read at two points: the first go at this read the middle against a
    /// single point three radii out, and the middle of a crater that size is
    /// its central peak, while one point anywhere on the moon is as likely as
    /// not to be inside something else.
    #[test]
    fn a_crater_leaves_a_hole_where_it_was_put() {
        let ground = ground();
        let height = |here: Vec3| {
            let (u, v) = uv(here);
            let column = ((u * PICTURE_WIDE as f32) as usize).min(PICTURE_WIDE - 1);
            let row = ((v * PICTURE_TALL as f32) as usize).min(PICTURE_TALL - 1);

            ground.height[row * PICTURE_WIDE + column]
        };
        let (lon, lat, km, _) = NAMED[0];
        let (lon, lat) = (lon.to_radians(), lat.to_radians());
        let middle = place(lon, lat);
        let (east, north) = basis(lon, lat);
        let radius = across(km);
        let ring = |out: f32| {
            let mut all = 0.0;

            for step in 0..8 {
                let turn = step as f32 / 8.0 * TAU;
                let away = east * turn.cos() + north * turn.sin();

                all += height((middle + away * (radius * out).tan()).normalize());
            }

            all / 8.0
        };
        // inside the floor and outside the central peak, against well clear of
        // the rim and its ejecta
        let floor = ring(0.45);
        let out = ring(4.0);

        assert!(
            floor < out - deep(radius) * 0.25,
            "Tycho's floor averages {} and the ground four radii out averages {}",
            floor,
            out
        );
    }

    /// East points east and north points north, which is the thing that was
    /// wrong in both places it was written.
    ///
    /// Checked against the ball itself rather than against the formula: a step
    /// east of a point has to be further round and no higher up, and a step
    /// north has to be higher up and no further round.
    #[test]
    fn east_is_east_and_north_is_north() {
        let step = 1e-3;
        let places: [(f32, f32); 5] = [
            (0.0, 0.0),
            (37.0, 18.0),
            (-80.0, -44.0),
            (150.0, 70.0),
            (-120.0, -5.0),
        ];

        for (lon, lat) in places {
            let (lon, lat) = (lon.to_radians(), lat.to_radians());
            let here = place(lon, lat);
            let (east, north) = basis(lon, lat);

            for (name, way, went) in [
                ("east", east, place(lon + step, lat) - here),
                ("north", north, place(lon, lat + step) - here),
            ] {
                let went = went.normalize();

                assert!(
                    way.dot(went) > 0.999,
                    "at {} {} a step {} went {} and {} is called {}",
                    lon.to_degrees(),
                    lat.to_degrees(),
                    name,
                    went,
                    way,
                    name
                );
                assert!(
                    way.dot(here).abs() < 1e-5,
                    "at {} {} {} is not along the ground",
                    lon.to_degrees(),
                    lat.to_degrees(),
                    name
                );
            }

            assert!(
                east.dot(north).abs() < 1e-5,
                "east and north are not square to each other at {} {}",
                lon.to_degrees(),
                lat.to_degrees()
            );
        }
    }

    /// The seas read darker than the ground around them, which is the whole
    /// of what the face says at the size it is drawn.
    ///
    /// **Averaged over the middle of the face rather than read at a point.**
    /// The light is baked into this picture, so a sea in the middle and a
    /// highland near the limb are not comparable: the first reading of this
    /// put Imbrium at 0.66 against a highland at 0.54 and both were right.
    /// The window is the part of the face that is both well inside the limb
    /// and well inside the terminator, which `SUN_OFF` puts at 42 degrees
    /// round: sampling past it put a quarter of the highlands in the dark and
    /// measured the phase instead of the ground.
    #[test]
    fn the_seas_are_darker_than_the_highlands() {
        let picture = baked();
        let (mut sea, mut highland) = (Vec::new(), Vec::new());

        for lat in (-55..=55).step_by(5) {
            for lon in (-75..=20).step_by(5) {
                let (lon, lat) = (lon as f32, lat as f32);
                let how_much = sea_among(&basins(), place(lon.to_radians(), lat.to_radians()));
                let shade = read(&picture, lon, lat);

                if how_much > 0.9 {
                    sea.push(shade);
                } else if how_much < 0.05 {
                    highland.push(shade);
                }
            }
        }

        let mean = |of: &[f32]| of.iter().sum::<f32>() / of.len() as f32;

        assert!(
            sea.len() > 40 && highland.len() > 40,
            "{} sea and {} highland samples, which is not a near side",
            sea.len(),
            highland.len()
        );
        assert!(
            mean(&sea) < mean(&highland) * 0.85,
            "the seas read {} and the highlands {}",
            mean(&sea),
            mean(&highland)
        );
    }

    /// The face is not one shade.
    ///
    /// The one it replaced ran from 0.80 to 0.92 over the whole near side,
    /// which is why it read as a lamp rather than a moon, so this is set well
    /// above what that could pass.
    #[test]
    fn the_face_is_not_one_shade() {
        let picture = baked();
        let (mut low, mut high) = (f32::INFINITY, 0.0f32);

        for lat in (-80..=80).step_by(10) {
            for lon in (-85..=85).step_by(10) {
                let v = read(&picture, lon as f32, lat as f32);

                low = low.min(v);
                high = high.max(v);
            }
        }

        assert!(
            high - low > 0.35,
            "the near side runs {} to {}, which is one shade",
            low,
            high
        );
    }

    /// The side away from the sun is dark and the side towards it is lit,
    /// which is the terminator, which is where every crater that reads is.
    #[test]
    fn the_sun_leaves_a_dark_edge() {
        let picture = baked();
        // `SUN_OFF` is measured towards negative z, so the far limb from it is
        // the positive one
        let dark = read(&picture, 86.0, 0.0);
        let lit = read(&picture, -86.0, 0.0);

        assert!(
            dark < 0.35,
            "the limb away from the sun is {}, which is lit",
            dark
        );
        assert!(
            lit > 0.6,
            "the limb under the sun is {}, which is dark",
            lit
        );
    }

    /// And the dark part is still there, rather than being a hole cut in the
    /// sky.
    #[test]
    fn the_dark_side_is_not_a_hole() {
        let picture = baked();
        let dark = read(&picture, 88.0, 0.0);

        assert!(dark > 0.08, "the unlit limb is {}, which is nothing", dark);
        assert!(
            dark < 0.4,
            "the unlit limb is {}, which is not unlit at all",
            dark
        );
    }

    /// Whichever way it hangs, the same face is the one you see.
    #[test]
    fn one_face_keeps_to_the_eye() {
        for hour in 0..24 {
            let since = crate::garden::when(hour as f32 / 24.0);
            let hung = at(since);
            // the eye is at the origin and the moon is out along `hung`, so
            // the face has to look back down it
            let toward = -hung;
            let looking = turned(toward) * FACE;

            assert!(
                looking.distance(toward.normalize()) < 1e-4,
                "at hour {} the face looks {} and the eye is at {}",
                hour,
                looking,
                toward
            );
        }
    }

    /// What is asked for is what lands, which is the thing the flat twenty
    /// eight could not do.
    #[test]
    fn the_face_lands_at_what_it_is_asked_for() {
        let since = crate::garden::when(0.75);
        let showing = showing(since);

        assert!(
            showing > 0.99,
            "the moon is only {} up at midnight",
            showing
        );

        let lands = lit(since).truncate() * crate::garden::filled(since);

        assert!(
            lands.distance(crate::aim::MOON) < 1e-3,
            "the face lands at {} and was asked for {}",
            lands,
            crate::aim::MOON
        );
        assert!(
            lands.max_element() <= 1.0,
            "the face lands at {}, which clips",
            lands
        );
    }

    /// It is up at night and gone by day.
    #[test]
    fn it_is_a_night_thing() {
        assert!(showing(crate::garden::when(0.25)) < 0.01, "up at noon");
        assert!(showing(crate::garden::when(0.75)) > 0.9, "down at midnight");
    }

    /// And it still has its light while it is on the skyline, so that the
    /// hills are what hide it.
    ///
    /// **The control is the second half.** A moon that keeps its light all
    /// the way down shows under the land, which from eye height starts half a
    /// degree below the horizon. This asks for both: lit while anything of it
    /// is above the hills, and out before the whole disc is under the ground.
    #[test]
    fn it_sets_behind_the_hills_rather_than_fading_over_them() {
        // the field's horizon stands about five degrees up, so this is the
        // last place a hill can hide it
        let on_the_skyline = 6.0f32.to_radians().sin();
        // and half the disc, which is what has to be gone before it goes out
        let all_under = -(crate::moon::WIDE * 0.5 / crate::moon::OFF).atan().sin() - 0.1;
        let mut lit_low = false;
        let mut out_under = false;

        for n in 0..4000 {
            let since = crate::garden::when(n as f32 / 4000.0);
            let high = at(since).y;
            let lit = showing(since);

            if (high - on_the_skyline).abs() < 0.01 && lit > 0.5 {
                lit_low = true;
            }

            if high < all_under && lit > 0.01 {
                out_under = true;
            }
        }

        assert!(
            lit_low,
            "the moon is never lit while it is down on the skyline"
        );
        assert!(
            !out_under,
            "the moon still has light on it with the whole disc under the ground"
        );
    }
}
