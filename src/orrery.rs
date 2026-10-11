//! The solar system behind the wall. Spec 0015.
//!
//! The sun at the middle of a room ninety seven metres square, the eight
//! planets out from it, their moons round them, and stars on every surface.
//! You come in at the cold end and walk toward the light.
//!
//! **Two scales, and the ratio between them is the one lie.** One scale cannot
//! work: put Neptune fifty metres out at true proportion and the Earth is
//! fourteen hundredths of a millimetre. So bodies are measured against bodies
//! and distances against distances, each exactly true inside itself, and the
//! orbit scale is a hundred and sixty eight times the body scale. Said out
//! loud here rather than buried.
//!
//! **Why the sun is a metre.** In the orbiting arrangement Mercury has to
//! clear the sun's own ball, and Mercury is at 0.387 AU against Neptune's
//! thirty, so the disc you need comes out at about seventy eight times the
//! sun's diameter. A sun twice as wide costs four times the floor. At a metre
//! the disc is 96.4 across and Mercury's surface clears the sun's by 11.8 cm.

use glam::{vec3, Vec3};

/// How many kilometres a metre is, for a ball.
pub const SUN_ACROSS_KM: f32 = 1_392_700.0;

/// How many metres one astronomical unit is drawn as.
pub const AU: f32 = 1.6;

/// A kilometre, in metres, on the body scale.
///
/// One metre to 557,080 km. It was one metre to the sun's own diameter, which
/// made the sun exactly a metre and the Earth a nine millimetre pea, and a
/// pea in a room this size is a thing you walk past without seeing. This is
/// two and a half times that, so the Earth is 2.3 cm and Jupiter is 26.
///
/// **Two and a half and no more, and the Moon is what says so.** At three
/// times, the Moon's orbit round the Earth is exactly as wide as the gap
/// between the Earth's orbit and Mars's, and the model starts telling lies
/// about which things are near which. Callisto gives out at 5.2 times and
/// Saturn's rings at 71, so the Moon is the whole of the limit. See
/// `the_moon_is_what_stops_the_bodies_growing`.
pub const BODY: f32 = 2.5 / SUN_ACROSS_KM;

/// What fraction of its true size the sun is drawn at, and it is the second
/// lie in this model after the ratio between the two scales.
///
/// Two fifths. Its true size on the body scale is two and a half metres, and
/// two and a half metres does not fit: the orbit scale has to clear it with
/// Mercury at 0.387 AU, and that wants a disc two hundred and forty metres
/// across.
///
/// The fraction is the constant and the size is worked out from it, so that
/// growing the bodies keeps the sun the same fraction of them rather than
/// quietly making it a different lie.
///
/// It is the lie every planet walk in the world tells, for the same reason,
/// and it is the one that buys the planets their size. Said here and said in
/// the room: see `aim::SYSTEM_NOTE`.
pub const SUN_SHRUNK: f32 = 0.4;

/// How big the sun is drawn: two fifths of what the body scale says.
pub fn sun_drawn() -> f32 {
    SUN_ACROSS_KM * BODY * SUN_SHRUNK
}

/// How far the planets reach, and so how big the room has to be.
pub fn reach() -> f32 {
    PLANETS
        .iter()
        .fold(0.0f32, |most, planet| most.max(planet.out * AU))
}

/// One body: what it is called, how big it is, and how it turns.
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub name: &'static str,
    /// Across, in kilometres.
    pub across: f32,
    /// How far it leans, in degrees. Venus is upside down and Uranus is on
    /// its side, and both of those are real.
    pub tilt: f32,
    /// How long it takes to turn once, in hours. Negative is backwards.
    pub spins: f32,
}

impl Body {
    /// How big it is drawn, in metres.
    pub fn drawn(&self) -> f32 {
        self.across * BODY
    }
}

/// A planet: a body, how far out it goes, and how long it takes.
#[derive(Clone, Copy, Debug)]
pub struct Planet {
    pub body: Body,
    /// Its mean distance from the sun, in astronomical units.
    pub out: f32,
    /// How long it takes to go round, in years.
    pub year: f32,
    /// Its moons, in the order they stand out from it.
    pub moons: &'static [Moon],
}

impl Planet {
    /// How far out it is drawn, in metres.
    pub fn far(&self) -> f32 {
        self.out * AU
    }
}

/// A moon: a body, and how far it stands from its planet in kilometres.
///
/// On the **body** scale and not the orbit scale, which is not a slip. The
/// Moon at the orbit scale is four millimetres from the Earth, which is
/// inside it. On the body scale it is 2.5 mm across and 27.6 cm away, which is
/// the proportion everybody half knows and nobody quite believes.
#[derive(Clone, Copy, Debug)]
pub struct Moon {
    pub body: Body,
    pub out_km: f32,
    /// How long it takes to go round, in days.
    pub month: f32,
}

impl Moon {
    pub fn far(&self) -> f32 {
        self.out_km * BODY
    }
}

pub const SUN: Body = Body {
    name: "the sun",
    across: SUN_ACROSS_KM,
    tilt: 7.25,
    spins: 609.6,
};

const NO_MOONS: &[Moon] = &[];

const LUNA: &[Moon] = &[Moon {
    body: Body {
        name: "the moon",
        across: 3475.0,
        tilt: 6.68,
        spins: 655.7,
    },
    out_km: 384_400.0,
    month: 27.32,
}];

/// The four Galileans, which is the only place in the model where a planet's
/// moons read as a system of their own.
const GALILEAN: &[Moon] = &[
    Moon {
        body: Body {
            name: "Io",
            across: 3643.0,
            tilt: 0.0,
            spins: 42.46,
        },
        out_km: 421_700.0,
        month: 1.769,
    },
    Moon {
        body: Body {
            name: "Europa",
            across: 3122.0,
            tilt: 0.1,
            spins: 85.22,
        },
        out_km: 671_034.0,
        month: 3.551,
    },
    Moon {
        body: Body {
            name: "Ganymede",
            across: 5268.0,
            tilt: 0.2,
            spins: 171.7,
        },
        out_km: 1_070_412.0,
        month: 7.155,
    },
    Moon {
        body: Body {
            name: "Callisto",
            across: 4821.0,
            tilt: 0.0,
            spins: 400.5,
        },
        out_km: 1_882_709.0,
        month: 16.689,
    },
];

const TITAN: &[Moon] = &[Moon {
    body: Body {
        name: "Titan",
        across: 5150.0,
        tilt: 0.3,
        spins: 382.7,
    },
    out_km: 1_221_870.0,
    month: 15.945,
}];

const TRITON: &[Moon] = &[Moon {
    body: Body {
        name: "Triton",
        across: 2707.0,
        tilt: 0.0,
        spins: -141.0,
    },
    out_km: 354_759.0,
    month: -5.877,
}];

/// The eight, out from the sun.
///
/// Phobos and Deimos are not here. They are 22 km and 12 km across, which is
/// four hundredths of a millimetre on this scale, and a Mars with two visible
/// specks beside it would be a lie about the one thing this model is for.
pub const PLANETS: [Planet; 8] = [
    Planet {
        body: Body {
            name: "Mercury",
            across: 4879.0,
            tilt: 0.03,
            spins: 1407.6,
        },
        out: 0.3871,
        year: 0.2408,
        moons: NO_MOONS,
    },
    Planet {
        body: Body {
            name: "Venus",
            across: 12104.0,
            tilt: 177.4,
            spins: -5832.5,
        },
        out: 0.7233,
        year: 0.6152,
        moons: NO_MOONS,
    },
    Planet {
        body: Body {
            name: "Earth",
            across: 12756.0,
            tilt: 23.44,
            spins: 23.934,
        },
        out: 1.0,
        year: 1.0,
        moons: LUNA,
    },
    Planet {
        body: Body {
            name: "Mars",
            across: 6792.0,
            tilt: 25.19,
            spins: 24.623,
        },
        out: 1.5237,
        year: 1.8808,
        moons: NO_MOONS,
    },
    Planet {
        body: Body {
            name: "Jupiter",
            across: 142984.0,
            tilt: 3.13,
            spins: 9.925,
        },
        out: 5.2026,
        year: 11.862,
        moons: GALILEAN,
    },
    Planet {
        body: Body {
            name: "Saturn",
            across: 120536.0,
            tilt: 26.73,
            spins: 10.656,
        },
        out: 9.5549,
        year: 29.457,
        moons: TITAN,
    },
    Planet {
        body: Body {
            name: "Uranus",
            across: 51118.0,
            tilt: 97.77,
            spins: -17.24,
        },
        out: 19.2184,
        year: 84.011,
        moons: NO_MOONS,
    },
    Planet {
        body: Body {
            name: "Neptune",
            across: 49528.0,
            tilt: 28.32,
            spins: 16.11,
        },
        out: 30.1104,
        year: 164.79,
        moons: TRITON,
    },
];

/// Saturn's rings, as radii in kilometres: where the faint inner one starts,
/// the two edges of the Cassini division, and where the bright one ends.
///
/// Four numbers, and they are what makes a ringed ball read as Saturn rather
/// than as a ball with a hoop round it. The gap is the thing people know even
/// when they do not know they know it.
pub const RING_IN: f32 = 74_658.0;
pub const CASSINI_IN: f32 = 117_580.0;
pub const CASSINI_OUT: f32 = 122_170.0;
pub const RING_OUT: f32 = 136_775.0;

/// The two clocks, for the same reason there are two scales.
///
/// Tied to one, an Earth day would be a sixth of a second and every planet
/// would strobe. Apart, Jupiter visibly turns while Venus visibly does not,
/// and Mercury races round while Neptune hardly moves. Both of those are the
/// real thing and both are worth seeing.
///
/// A day is this many seconds for the Earth, and a year is this many.
pub const DAY: f32 = 20.0;
pub const YEAR: f32 = 60.0;

/// How long a body takes to turn once, on the spin clock, in seconds.
///
/// Signed: backwards is backwards. Venus comes out at over an hour, which
/// looks like a stuck planet until you know that Venus turns slower than it
/// goes round.
pub fn spin_seconds(body: &Body) -> f32 {
    body.spins / 23.934 * DAY
}

/// How long a planet takes to go round, on the orbit clock, in seconds.
pub fn year_seconds(planet: &Planet) -> f32 {
    planet.year * YEAR
}

/// How long a moon takes to go round its planet, on the orbit clock.
///
/// The same clock as the planets, so the Moon goes round the Earth twelve
/// times a year, which is the one fact about the Moon that everybody has.
pub fn month_seconds(moon: &Moon) -> f32 {
    moon.month / 365.256 * YEAR
}

/// Where a planet stands, as an angle round the sun, at `since` seconds.
///
/// Every planet starts lined up, so the line is what you see on the first
/// frame of the orbiting arrangement and the fan opens from there.
pub fn angle_at(planet: &Planet, since: f32) -> f32 {
    since / year_seconds(planet) * std::f32::consts::TAU
}

/// Where a moon stands round its planet, at `since` seconds.
pub fn moon_angle_at(moon: &Moon, since: f32) -> f32 {
    since / month_seconds(moon) * std::f32::consts::TAU
}

/// Where a planet is, between lined up and in orbit.
///
/// `swung` is nought lined up and one in orbit, and everything between is a
/// real place on the planet's own circle: it keeps its distance from the sun
/// the whole way, so the line opens into the orbits rather than the planets
/// flying off and coming back. That is the whole of why the key does not snap.
///
/// `toward` is the way the line points, which is the way the door is.
pub fn stands(planet: &Planet, toward: Vec3, swung: f32, since: f32) -> Vec3 {
    let along = toward.normalize_or_zero();
    let side = vec3(-along.z, 0.0, along.x);
    let turn = angle_at(planet, since) * swung.clamp(0.0, 1.0);

    (along * turn.cos() + side * turn.sin()) * planet.far()
}

/// How many texels round the equator a planet's picture gets. Half that from
/// pole to pole, which is what an equirectangular map is.
pub const WIDE: u32 = 1024;
pub const TALL: u32 = 512;

/// Value noise on a lattice, wrapped in longitude so the picture joins up.
///
/// The seam is the thing that gives a procedural planet away: a band of cloud
/// that stops dead down one meridian. Wrapping the lattice costs one modulo
/// and takes the seam out.
fn wrapped(seed: u32, cells: u32) -> impl Fn(f32, f32) -> f32 {
    let cells = cells.max(1) as i32;

    move |u: f32, v: f32| {
        let hash = |x: i32, y: i32| {
            let x = x.rem_euclid(cells);
            let mut n =
                (x as u32).wrapping_mul(0x27D4_EB2D) ^ (y as u32).wrapping_mul(0x1656_67B1) ^ seed;
            n ^= n >> 15;
            n = n.wrapping_mul(0x2C1B_3C6D);
            n ^= n >> 12;
            (n >> 8) as f32 / ((1u32 << 24) as f32)
        };
        let smooth = |t: f32| t * t * (3.0 - 2.0 * t);
        let (x, y) = (u * cells as f32, v * cells as f32);
        let (fx, fy) = (x.floor(), y.floor());
        let (ix, iy) = (fx as i32, fy as i32);
        let (tx, ty) = (smooth(x - fx), smooth(y - fy));
        let low = hash(ix, iy) + (hash(ix + 1, iy) - hash(ix, iy)) * tx;
        let high = hash(ix, iy + 1) + (hash(ix + 1, iy + 1) - hash(ix, iy + 1)) * tx;

        low + (high - low) * ty
    }
}

/// Several of those on top of each other, each finer and fainter.
/// One layer of turbulence: how much of it there is, and the noise itself.
type Layer = (f32, Box<dyn Fn(f32, f32) -> f32>);

fn turbulent(seed: u32, cells: u32, octaves: u32) -> impl Fn(f32, f32) -> f32 {
    let layers: Vec<Layer> = (0..octaves.max(1))
        .map(|n| {
            let step = 1u32 << n;
            let weight = 1.0 / step as f32;
            let layer: Box<dyn Fn(f32, f32) -> f32> =
                Box::new(wrapped(seed.wrapping_add(n * 7919), cells * step));

            (weight, layer)
        })
        .collect();
    let total: f32 = layers.iter().map(|(w, _)| w).sum();

    move |u, v| layers.iter().map(|(w, layer)| layer(u, v) * w).sum::<f32>() / total
}

/// Builds an equirectangular picture from a colour at each longitude and
/// latitude, both nought to one, latitude nought at the south pole.
fn painted(
    wide: u32,
    tall: u32,
    colour: impl Fn(f32, f32) -> [u8; 4],
) -> blitzkit::texture::TextureData {
    let mut pixels = Vec::with_capacity((wide * tall * 4) as usize);

    for j in 0..tall {
        let v = j as f32 / (tall - 1).max(1) as f32;

        for i in 0..wide {
            let u = i as f32 / wide as f32;
            pixels.extend_from_slice(&colour(u, v));
        }
    }

    blitzkit::texture::TextureData::from_pixels(wide, tall, pixels)
}

/// Mixes two colours.
fn between(one: [f32; 3], two: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);

    [
        one[0] + (two[0] - one[0]) * t,
        one[1] + (two[1] - one[1]) * t,
        one[2] + (two[2] - one[2]) * t,
    ]
}

fn eight(c: [f32; 3]) -> [u8; 4] {
    [
        (c[0] * 255.0).clamp(0.0, 255.0) as u8,
        (c[1] * 255.0).clamp(0.0, 255.0) as u8,
        (c[2] * 255.0).clamp(0.0, 255.0) as u8,
        255,
    ]
}

/// The sun: granulation, and a few spots.
///
/// Granulation is the top of the convection, which is why the cells are all
/// about one size and all about one shape. Past white, so the ball carries its
/// own light the way the neon does.
pub fn sunned(seed: u32) -> blitzkit::texture::TextureData {
    let grain = turbulent(seed, 40, 3);
    let spots = wrapped(seed ^ 0x5A5A, 7);

    painted(WIDE, TALL, move |u, v| {
        let cell = grain(u, v);
        let hot = 0.82 + cell * 0.36;
        let spot = spots(u, v);
        // a handful of them, dark and with a lighter edge, which is what a
        // sunspot is: a cool middle inside a warmer ring
        let dark = if spot > 0.86 {
            ((spot - 0.86) / 0.14).min(1.0)
        } else {
            0.0
        };
        let tone = (hot * (1.0 - dark * 0.72)).clamp(0.0, 1.0);

        // near white, with the granulation in it. The colour and the blaze
        // both come from the instance, because an eight bit texture cannot
        // hold a number past one: written warm and bright here it clamped to
        // flat white and the granulation went with it.
        eight([tone, tone * 0.97, tone * 0.93])
    })
}

/// A banded world: Jupiter, Saturn, and the two ice giants.
///
/// The bands are latitude and the turbulence is what stops them being
/// wallpaper. Jupiter gets its spot; the others pass `None` and get the
/// nothing they deserve, because Uranus really does look like that.
pub fn banded(
    seed: u32,
    bands: f32,
    dark: [f32; 3],
    light: [f32; 3],
    churn: f32,
    spot: Option<([f32; 3], f32, f32, f32)>,
) -> blitzkit::texture::TextureData {
    let swirl = turbulent(seed, 24, 4);
    let fine = turbulent(seed ^ 0x1234, 90, 2);

    painted(WIDE, TALL, move |u, v| {
        // the bands run round, and the turbulence pushes them up and down so
        // the edges wander the way a cloud belt's does
        let pushed = v + (swirl(u, v) - 0.5) * churn;
        let band = (pushed * bands * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        let mut tone = between(dark, light, band * 0.8 + fine(u, v) * 0.2);

        // and the spot, as an oval, because it is one
        if let Some((colour, at_u, at_v, wide)) = spot {
            let du = ((u - at_u + 0.5).rem_euclid(1.0) - 0.5) / wide;
            let dv = (v - at_v) / (wide * 0.42);
            let inside = 1.0 - (du * du + dv * dv).sqrt();

            if inside > 0.0 {
                tone = between(tone, colour, (inside * 2.4).min(1.0));
            }
        }

        eight(tone)
    })
}

/// A rocky world: Mercury, the moons, and anything else grey and hit.
///
/// `maria` floods the low ground dark, which is what makes the Moon the Moon
/// rather than a grey ball. Without it you get Callisto, which is right for
/// Callisto.
pub fn rocky(seed: u32, base: [f32; 3], maria: bool) -> blitzkit::texture::TextureData {
    let ground = turbulent(seed, 18, 4);
    let pits = turbulent(seed ^ 0x9E37, 60, 2);
    let seas = wrapped(seed ^ 0x3C6E, 5);

    painted(WIDE, TALL, move |u, v| {
        let rough = ground(u, v) * 0.65 + pits(u, v) * 0.35;
        let mut tone = between(
            [base[0] * 0.62, base[1] * 0.62, base[2] * 0.62],
            [base[0] * 1.25, base[1] * 1.25, base[2] * 1.25],
            rough,
        );

        if maria {
            let sea = seas(u, v);

            if sea > 0.56 {
                let deep = ((sea - 0.56) / 0.44).min(1.0);
                tone = between(tone, [base[0] * 0.42, base[1] * 0.42, base[2] * 0.44], deep);
            }
        }

        eight(tone)
    })
}

/// Mars: rust, the dark marks, and a cap at each pole.
///
/// The caps are not the same size, because they never are: one is in summer
/// while the other is in winter.
pub fn martian(seed: u32) -> blitzkit::texture::TextureData {
    let dust = turbulent(seed, 16, 4);
    let marks = turbulent(seed ^ 0x77AA, 7, 3);

    painted(WIDE, TALL, move |u, v| {
        let rust = between([0.46, 0.24, 0.13], [0.74, 0.46, 0.28], dust(u, v));
        // the albedo features, which are where the dust has blown off
        let mark = marks(u, v);
        let mut tone = if mark < 0.42 {
            between(rust, [0.33, 0.22, 0.17], (0.42 - mark) / 0.42)
        } else {
            rust
        };

        let cap = if v > 0.945 {
            (v - 0.945) / 0.055
        } else if v < 0.030 {
            (0.030 - v) / 0.030
        } else {
            0.0
        };

        if cap > 0.0 {
            tone = between(tone, [0.94, 0.95, 0.97], (cap * 1.6).min(1.0));
        }

        eight(tone)
    })
}

/// Venus: cream cloud, and nothing else, which is the whole of what Venus
/// looks like from outside.
pub fn venusian(seed: u32) -> blitzkit::texture::TextureData {
    let cloud = turbulent(seed, 10, 4);

    painted(WIDE, TALL, move |u, v| {
        // the one feature is a faint vee that the super rotation draws, and
        // it is faint
        let skew = u + v * 0.35;
        let streak = turbulent(seed ^ 0xBEEF, 6, 2)(skew.rem_euclid(1.0), v);
        let tone = between(
            [0.78, 0.70, 0.52],
            [0.96, 0.92, 0.78],
            cloud(u, v) * 0.75 + streak * 0.25,
        );

        eight(tone)
    })
}

/// Saturn's rings, as a strip from the inner edge to the outer with the
/// Cassini division cut clean out of it.
///
/// A cutout and not a fade: the gap is a gap, and the engine drops a texel
/// the texture says is not there, per spec 0047.
pub fn ringed(seed: u32) -> blitzkit::texture::TextureData {
    let across = 1024u32;
    let grain = wrapped(seed, 160);
    let mut pixels = Vec::with_capacity((across * 4 * 4) as usize);

    for _ in 0..4 {
        for i in 0..across {
            let t = i as f32 / (across - 1) as f32;
            let km = RING_IN + (RING_OUT - RING_IN) * t;

            if km > CASSINI_IN && km < CASSINI_OUT {
                pixels.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }

            // the C ring inside is faint and the B ring is the bright one
            let bright = if km < 92_000.0 { 0.42 } else { 1.0 };
            let banding = 0.72 + grain(t, 0.5) * 0.5;
            let tone = bright * banding;
            let fade = if km > CASSINI_OUT { 0.86 } else { 1.0 };

            pixels.extend_from_slice(&[
                (tone * fade * 214.0).clamp(0.0, 255.0) as u8,
                (tone * fade * 198.0).clamp(0.0, 255.0) as u8,
                (tone * fade * 168.0).clamp(0.0, 255.0) as u8,
                (tone * 230.0).clamp(0.0, 255.0) as u8,
            ]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(across, 4, pixels)
}

/// The sun as a lamp: its colour, how hard it burns, and how far it carries.
///
/// The range is the room's own half width, so the light gives out around
/// Neptune. That is not a budget: the far planets being nearly dark is the
/// single most surprising true thing about the solar system, and a model that
/// lights Neptune like Mercury has thrown it away.
pub const SUNLIGHT: Vec3 = vec3(1.0, 0.95, 0.86);
pub const SUN_LIT: f32 = 7.0;

/// How far the sun's light carries: out past Neptune and no further.
///
/// Taken from the model rather than typed, so a change to the orbit scale
/// takes the light with it.
pub fn sun_range() -> f32 {
    reach() + 4.0
}

/// How long the swing between lined up and in orbit takes, in seconds.
///
/// Three. Long enough to watch the line fan open and short enough that you
/// are not waiting. It must not be nought: a model that jumps between two
/// arrangements is two models, and a model that opens into the other one is
/// one model you understand.
pub const SWINGS: f32 = 3.0;

/// The six surfaces of the room, as planes to lay stars on: where each is,
/// how big, and which way it looks.
///
/// All six, floor and ceiling included. A room with stars on four sides and
/// grey underfoot is a room; a room with stars on all six is a place the room
/// has stopped being.
pub fn walls(reaches: f32) -> Vec<(Vec3, glam::Vec2, Vec3)> {
    let middle = crate::behind::at(reaches);
    let half = crate::behind::half();
    let high = crate::behind::HIGH;
    let proud = 0.02;

    vec![
        (
            vec3(middle.x, proud, middle.z),
            glam::vec2(half.x * 2.0, half.y * 2.0),
            Vec3::Y,
        ),
        (
            vec3(middle.x, high - proud, middle.z),
            glam::vec2(half.x * 2.0, half.y * 2.0),
            -Vec3::Y,
        ),
        (
            vec3(middle.x - half.x + proud, high * 0.5, middle.z),
            glam::vec2(half.y * 2.0, high),
            Vec3::X,
        ),
        (
            vec3(middle.x + half.x - proud, high * 0.5, middle.z),
            glam::vec2(half.y * 2.0, high),
            -Vec3::X,
        ),
        // the near one stands clear of the hall's own end wall, which is
        // drawn in one piece across the way in, per spec 0011, and is half a
        // wall's thickness into this room. Laid on the nominal edge, the
        // starfield was behind it and what you saw from in here was a warm
        // panelled rectangle hanging in space.
        (
            vec3(
                middle.x,
                high * 0.5,
                middle.z - half.y + crate::room::THICK * 0.5 + proud,
            ),
            glam::vec2(half.x * 2.0, high),
            Vec3::Z,
        ),
        (
            vec3(middle.x, high * 0.5, middle.z + half.y - proud),
            glam::vec2(half.x * 2.0, high),
            -Vec3::Z,
        ),
    ]
}

/// How fine the star pictures are, in texels to a metre, and how big one may
/// get. A star is a point and wants resolution more than it wants size.
pub const STAR_PER_UNIT: f32 = 16.0;
pub const STAR_MOST: u32 = 2048;
pub const STAR_SEED: u32 = 0x57A8_5EED;

/// Stars, as a picture to lay on a surface of the room.
///
/// A plausible spread of brightnesses and not a uniform sprinkle. A sky of
/// equal dots reads as noise or as a dirty window; what makes a sky a sky is
/// that almost all of them are nearly invisible and a few are not. So the
/// count falls off steeply with brightness, the way the real one does, and
/// the bright few get a little colour: hot ones blue, cool ones orange.
pub fn starred(seed: u32, size: glam::Vec2) -> blitzkit::texture::TextureData {
    let wide = ((size.x * STAR_PER_UNIT) as u32).clamp(64, STAR_MOST);
    let tall = ((size.y * STAR_PER_UNIT) as u32).clamp(64, STAR_MOST);
    let mut pixels = vec![0u8; (wide * tall * 4) as usize];

    for n in 0..(wide * tall * 4) {
        if n % 4 == 3 {
            pixels[n as usize] = 255;
        }
    }

    // one star per this many texels, which at sixteen to a metre is about two
    // hundred to a square metre of wall and reads as a country sky
    let many = (wide as u64 * tall as u64 / 420).max(16) as u32;
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };

    for _ in 0..many {
        let x = next() % wide;
        let y = next() % tall;
        // brightness falls off steeply: the fourth power of a flat draw gives
        // a great many faint ones and a handful that carry the sky
        let roll = (next() % 10_000) as f32 / 10_000.0;
        let bright = roll * roll * roll * roll;
        let warm = (next() % 1000) as f32 / 1000.0;
        let tone = 60.0 + bright * 420.0;
        let (r, g, b) = if warm > 0.72 {
            (tone * 1.12, tone * 0.95, tone * 0.78)
        } else if warm < 0.18 {
            (tone * 0.80, tone * 0.90, tone * 1.18)
        } else {
            (tone, tone, tone * 1.02)
        };

        let put = |pixels: &mut Vec<u8>, x: u32, y: u32, s: f32| {
            if x >= wide || y >= tall {
                return;
            }
            let at = ((y * wide + x) * 4) as usize;
            for (n, c) in [r, g, b].iter().copied().enumerate() {
                let was = pixels[at + n] as f32;
                pixels[at + n] = (was + c * s).min(255.0) as u8;
            }
        };

        put(&mut pixels, x, y, 1.0);

        // the bright few spill into their neighbours, which is what makes
        // them read as bright rather than as bigger
        if bright > 0.12 {
            let spill = (bright - 0.12) * 0.9;
            put(&mut pixels, x.wrapping_sub(1), y, spill);
            put(&mut pixels, x + 1, y, spill);
            put(&mut pixels, x, y.wrapping_sub(1), spill);
            put(&mut pixels, x, y + 1, spill);
        }
    }

    blitzkit::texture::TextureData::from_pixels(wide, tall, pixels)
}

/// The way out, named on a sign beside it.
///
/// The room is ninety seven metres square and black, the way out is a slot
/// the width of a door in one wall of it, and spec 0011 spent its whole
/// design making that slot hard to find. Hard to find from the hall is the
/// point. Hard to find from in here, with your back to it and a planet in
/// front of you, is a room you are stuck in.
///
/// So: a lit sign, which is what every hall this size has. It carries its own
/// light rather than standing under a lamp, because the building is already
/// pushing more lamps at the engine than it will take.
///
/// "WAY OUT" and not "EXIT", because that is what the rest of this building
/// calls it: the way in, the way out, in the code and in spec 0011.
pub const WAY_OUT: &str = "WAY OUT";
pub const SIGN_TEXELS: f32 = 64.0;

/// How big the lettering is, how far the housing stands round it, how thick
/// it is, how high it hangs, and how far it sits from the edge of the way
/// out.
///
/// A metre across was plenty to read standing under it and a green smudge
/// from forty metres, lost among stars that are brighter than it. Half again
/// as wide and the word is a word from the far end, which is the only
/// distance that matters: under it you already know where the door is.
pub const SIGN_WIDE: f32 = 1.5;
pub const SIGN_TALL: f32 = 0.42;
pub const SIGN_BORDER: f32 = 0.06;
pub const SIGN_THICK: f32 = 0.06;
pub const SIGN_UP: f32 = 2.4;
pub const SIGN_BESIDE: f32 = 0.45;

/// Where the sign hangs: its middle, and which way it faces.
///
/// Beside the way out rather than over it, because the way out is a slot the
/// full height of the wall and there is nothing above it to hang from. On the
/// wider side of the two, which is the side with wall enough to carry it.
pub fn exit_sign(reaches: f32) -> (Vec3, Vec3) {
    let (from, to) = crate::behind::way();
    let side = crate::room::WALL + crate::room::CABINET.x;
    let wider = (side - to) > (from + side);
    let x = if wider {
        to + SIGN_BESIDE + SIGN_WIDE * 0.5
    } else {
        from - SIGN_BESIDE - SIGN_WIDE * 0.5
    };

    (
        vec3(x, SIGN_UP, reaches + crate::room::THICK * 0.5),
        Vec3::Z,
    )
}

/// How high off the floor the whole model sits.
///
/// Eye height, so you look the planets in the face rather than down at them.
/// The sun swallows you on the way past, which has no collider and is the
/// right thing to happen.
pub const PLANE: f32 = 1.6;

/// Saturn's rings, as a flat annulus turned on the wheel.
///
/// Four steps across and ninety six round: the picture does all the work
/// across the strip and the roundness has to survive being looked along.
pub fn ring_mesh() -> blitzkit::mesh::MeshData {
    let mut mesh = blitzkit::mesh::MeshData::surface(4, 96, |u, v| {
        let round = v * std::f32::consts::TAU;
        let out = (RING_IN + (RING_OUT - RING_IN) * u) * BODY;

        vec3(round.cos() * out, 0.0, round.sin() * out)
    });

    // both sides, because you walk round Saturn and half of the time you are
    // under its rings. One sided, they vanish as you pass the plane of them,
    // which is a thing real rings do and this is not that: those go thin, not
    // absent.
    let half = mesh.indices.len();

    for n in (0..half).step_by(3) {
        let (a, b, c) = (mesh.indices[n], mesh.indices[n + 1], mesh.indices[n + 2]);
        mesh.indices.extend_from_slice(&[a, c, b]);
    }

    mesh
}

/// Every body in the model, in the order the drawing walks them: the sun,
/// then each planet followed by its own moons.
///
/// One list, because the pictures are built from it and the drawing reads it,
/// and two accounts of one thing come apart.
pub fn every_body() -> Vec<Body> {
    let mut out = vec![SUN];

    for planet in PLANETS.iter() {
        out.push(planet.body);

        for moon in planet.moons.iter() {
            out.push(moon.body);
        }
    }

    out
}

/// What a body wears.
///
/// By name, because the name is already the thing that tells them apart and a
/// second key would be a second list. Earth is the one with real data behind
/// it: `data/land.txt` has the world's coastlines and `globe::drawn` makes a
/// map from them, so the one body nobody could fake is the one that is true.
pub fn picture(body: &Body) -> blitzkit::texture::TextureData {
    let seed = body.name.bytes().fold(0x9E37_79B9u32, |n, b| {
        n.wrapping_mul(31).wrapping_add(b as u32)
    });

    match body.name {
        "the sun" => sunned(seed),
        "Earth" => crate::globe::drawn(),
        "Venus" => venusian(seed),
        "Mars" => martian(seed),
        "Jupiter" => banded(
            seed,
            9.0,
            [0.52, 0.38, 0.27],
            [0.90, 0.82, 0.70],
            0.055,
            // the Red Spot, two and a half Earths across, in the southern
            // belt, which is where it is
            Some(([0.72, 0.34, 0.21], 0.58, 0.33, 0.085)),
        ),
        "Saturn" => banded(
            seed,
            7.0,
            [0.70, 0.60, 0.42],
            [0.94, 0.87, 0.70],
            0.035,
            None,
        ),
        "Uranus" => banded(
            seed,
            3.0,
            [0.62, 0.82, 0.84],
            [0.74, 0.90, 0.90],
            0.015,
            None,
        ),
        "Neptune" => banded(
            seed,
            4.0,
            [0.18, 0.33, 0.70],
            [0.34, 0.52, 0.86],
            0.03,
            // the dark spot, which comes and goes and is there now
            Some(([0.10, 0.18, 0.46], 0.30, 0.38, 0.06)),
        ),
        "the moon" => rocky(seed, [0.56, 0.55, 0.52], true),
        "Io" => rocky(seed, [0.86, 0.78, 0.38], false),
        "Europa" => rocky(seed, [0.86, 0.84, 0.80], false),
        "Ganymede" => rocky(seed, [0.62, 0.58, 0.52], true),
        "Callisto" => rocky(seed, [0.44, 0.41, 0.38], false),
        "Titan" => rocky(seed, [0.78, 0.56, 0.26], false),
        "Triton" => rocky(seed, [0.80, 0.74, 0.72], true),
        // Mercury, and anything added later that nobody has painted yet
        _ => rocky(seed, [0.52, 0.50, 0.48], false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planet(name: &str) -> Planet {
        *PLANETS
            .iter()
            .find(|planet| planet.body.name == name)
            .expect("a planet by that name")
    }

    /// Spec 0015: every ball is true against every other ball.
    ///
    /// Ratios and not sizes, because a ratio is the claim. Jupiter is 11.21
    /// Earths across and that is a fact about the sky, not about this model.
    #[test]
    fn every_ball_is_true_against_every_other() {
        let earth = planet("Earth").body.drawn();
        let jupiter = planet("Jupiter").body.drawn();

        assert!(
            (jupiter / earth - 142984.0 / 12756.0).abs() < 1e-3,
            "Jupiter came out {:.3} Earths across, not {:.3}",
            jupiter / earth,
            142984.0 / 12756.0
        );
        // and the sun is the one body that is not true against them, which
        // is a decision and not a slip
        assert!(
            (SUN.drawn() - 2.5).abs() < 1e-3,
            "the sun's true size on the body scale is {:.3} m, not 2.50",
            SUN.drawn()
        );
        assert!(
            (sun_drawn() - 1.0).abs() < 1e-3,
            "two fifths of its true size is {:.3} m, not one",
            sun_drawn()
        );
        // and the room says the same fraction the constant does
        assert!(
            crate::aim::SYSTEM_NOTE.contains("two fifths") && (SUN_SHRUNK - 0.4).abs() < 1e-6,
            "the line in the room and the constant disagree about the sun"
        );
    }

    /// Spec 0015: the Moon is what stops the bodies growing.
    ///
    /// At three times this scale its orbit is as wide as the gap between the
    /// Earth's orbit and Mars's, and a model where the Moon reaches the next
    /// planet is a model lying about the one proportion everybody has. Every
    /// other body has room to spare: Callisto gives out at 5.2 times and
    /// Saturn's rings at 71.
    #[test]
    fn the_moon_is_what_stops_the_bodies_growing() {
        let earth = planet("Earth");
        let mars = planet("Mars");
        let moon = earth.moons[0];
        let gap = mars.far() - earth.far();

        assert!(
            moon.far() < gap * 0.9,
            "the Moon reaches {:.2} m from the Earth and Mars is {:.2} m away",
            moon.far(),
            gap
        );
        // and there is not much left: a fifth again and it is touching
        assert!(
            moon.far() * 1.25 > gap * 0.9,
            "the Moon reaches {:.2} m of a {:.2} m gap, so the scale has room \
             to grow and the note in this module saying it does not is wrong",
            moon.far(),
            gap
        );
    }

    /// Spec 0015: and every distance is true against every other distance.
    #[test]
    fn every_distance_is_true_against_every_other() {
        let earth = planet("Earth").far();
        let neptune = planet("Neptune").far();

        assert!(
            (neptune / earth - 30.1104).abs() < 1e-3,
            "Neptune came out {:.4} Earth orbits out, not 30.1104",
            neptune / earth
        );
    }

    /// Spec 0015: the first lie, pinned.
    ///
    /// It was 67.1 when the sun's own diameter set the body scale. Growing
    /// the bodies two and a half times to make them worth looking at grew
    /// the disparity with them, which is the cost of that decision and is
    /// why it is written down rather than left to be worked out.
    ///
    /// If this number ever drifts it will be because somebody changed one
    /// scale and not the other, and the model will still look fine.
    #[test]
    fn the_orbit_scale_is_a_hundred_and_sixty_eight_times_the_body_scale() {
        let orbit_km_per_metre = 149_597_871.0 / AU;
        let body_km_per_metre = 1.0 / BODY;
        let times = orbit_km_per_metre / body_km_per_metre;

        assert!(
            (times - 167.8).abs() < 0.2,
            "the orbit scale is {:.2} times the body scale, not 167.8",
            times
        );
    }

    /// Spec 0015: Mercury clears the sun, which is what set the sun's size.
    ///
    /// Its own control: the same arithmetic with a sun twice as wide puts
    /// Mercury inside it, which is why the sun is not twice as wide.
    #[test]
    fn mercury_clears_the_sun_and_would_not_with_a_bigger_one() {
        let mercury = planet("Mercury");
        let gap = mercury.far() - sun_drawn() * 0.5 - mercury.body.drawn() * 0.5;

        assert!(
            gap > 0.05,
            "Mercury's surface clears the sun's by {:.3} m",
            gap
        );

        // and the sun at its true size on the body scale would swallow it,
        // which is why it is not drawn at its true size
        let honest = mercury.far() - SUN.drawn() * 0.5 - mercury.body.drawn() * 0.5;

        assert!(
            honest < 0.0,
            "a sun at its true size would still leave Mercury {:.3} m clear, \
             so the reason given for shrinking it is wrong",
            honest
        );
    }

    /// Spec 0015: every moon stands outside its planet.
    ///
    /// This is what the body scale buys. On the orbit scale the Moon would be
    /// four millimetres out and inside the Earth.
    #[test]
    fn every_moon_stands_outside_its_planet() {
        for planet in PLANETS.iter() {
            for moon in planet.moons.iter() {
                let clear = moon.far() - planet.body.drawn() * 0.5 - moon.body.drawn() * 0.5;

                assert!(
                    clear > 0.0,
                    "{} is {:.3} m from {}, which is inside it",
                    moon.body.name,
                    moon.far(),
                    planet.body.name
                );
            }

            // and the orbit scale would not do it
            if let Some(moon) = planet.moons.first() {
                let wrong = moon.out_km / 149_597_871.0 * AU;

                assert!(
                    wrong < planet.body.drawn() * 0.5,
                    "the orbit scale puts {} outside {} after all, so the note \
                     in this module about why moons use the body scale is wrong",
                    moon.body.name,
                    planet.body.name
                );
            }
        }
    }

    /// Spec 0015: and no moon reaches the next planet along.
    #[test]
    fn no_moon_reaches_the_next_planet() {
        for pair in PLANETS.windows(2) {
            let (inner, outer) = (pair[0], pair[1]);
            let spread = inner
                .moons
                .iter()
                .fold(0.0f32, |most, moon| most.max(moon.far()));

            assert!(
                inner.far() + spread < outer.far(),
                "{}'s moons reach {:.2} m and {} is at {:.2} m",
                inner.body.name,
                inner.far() + spread,
                outer.body.name,
                outer.far()
            );
        }
    }

    /// Spec 0015: the rings start outside Saturn and the gap is between them.
    #[test]
    fn saturns_rings_start_outside_it_and_the_gap_falls_between() {
        let saturn = planet("Saturn");
        let half = saturn.body.across * 0.5;

        assert!(
            RING_IN > half,
            "the rings start at {:.0} km and Saturn's surface is at {:.0}",
            RING_IN,
            half
        );
        // and the picture really does cut the gap out of itself, which is the
        // claim. Comparing the four constants with each other was an
        // assertion the compiler could answer without running it.
        let picture = ringed(7);
        let level = &picture.levels[0];
        let wide = level.width as usize;
        let gap_at = ((CASSINI_IN + CASSINI_OUT) * 0.5 - RING_IN) / (RING_OUT - RING_IN);
        let gap = (gap_at * wide as f32) as usize;
        let solid = (0.1 * wide as f32) as usize;

        assert_eq!(
            level.pixels[gap * 4 + 3],
            0,
            "the Cassini division is not cut out of the ring"
        );
        assert!(
            level.pixels[solid * 4 + 3] > 0,
            "the ring is cut out where it should be solid, so the test above \
             proves nothing"
        );
    }

    /// Spec 0015: the planets are in order out from the sun.
    #[test]
    fn the_planets_are_in_order() {
        for pair in PLANETS.windows(2) {
            assert!(
                pair[0].out < pair[1].out,
                "{} is listed before {} and is further out",
                pair[0].body.name,
                pair[1].body.name
            );
        }
    }

    /// Spec 0015: Venus turns slower than it goes round, on both clocks.
    ///
    /// The two clocks are separate and each is proportional inside itself, so
    /// a fact that holds in the sky has to hold here. This is the oddest one
    /// there is, and it is the test that would catch somebody quietly tying
    /// the clocks together.
    #[test]
    fn venus_turns_slower_than_it_goes_round() {
        let venus = planet("Venus");

        assert!(
            spin_seconds(&venus.body).abs() > year_seconds(&venus),
            "Venus turns in {:.0} s and goes round in {:.0} s",
            spin_seconds(&venus.body).abs(),
            year_seconds(&venus)
        );

        // and the Earth does not, which is what makes the above a fact about
        // Venus rather than about the clocks
        let earth = planet("Earth");

        assert!(
            spin_seconds(&earth.body).abs() < year_seconds(&earth),
            "the Earth turns slower than it goes round too, so the clocks are wrong"
        );
    }

    /// Spec 0015: the swing keeps every planet at its own distance, the whole
    /// way through.
    ///
    /// This is what makes it read as one model rearranged rather than two
    /// models cross faded, and it is the only thing about the animation that
    /// can be checked without eyes.
    #[test]
    fn the_swing_keeps_every_planet_at_its_own_distance() {
        let toward = vec3(-1.0, 0.0, -0.4);

        for planet in PLANETS.iter() {
            for step in 0..=20 {
                let swung = step as f32 / 20.0;

                for since in [0.0f32, 7.0, 93.0, 1000.0].iter().copied() {
                    let at = stands(planet, toward, swung, since);

                    assert!(
                        (at.length() - planet.far()).abs() < 1e-3,
                        "{} is {:.3} m out at swung {:.2}, and belongs at {:.3}",
                        planet.body.name,
                        at.length(),
                        swung,
                        planet.far()
                    );
                    assert!(
                        at.y.abs() < 1e-6,
                        "{} left the plane of the model",
                        planet.body.name
                    );
                }
            }
        }
    }

    /// Spec 0015: lined up means lined up, whatever the clock says.
    #[test]
    fn lined_up_is_lined_up() {
        let toward = vec3(-1.0, 0.0, -0.4).normalize();

        for planet in PLANETS.iter() {
            let at = stands(planet, toward, 0.0, 4321.0);
            let want = toward * planet.far();

            assert!(
                at.distance(want) < 1e-3,
                "{} sits at {:?} when the line is along {:?}",
                planet.body.name,
                at,
                toward
            );
        }

        // and the control: swung all the way, at a time when Mercury is a
        // quarter of the way round, it is not on the line
        let mercury = planet("Mercury");
        let quarter = year_seconds(&mercury) * 0.25;
        let at = stands(&mercury, toward, 1.0, quarter);

        assert!(
            at.distance(toward * mercury.far()) > mercury.far(),
            "Mercury a quarter of a year round is still on the line"
        );
    }

    /// Spec 0015: the room has to hold the disc.
    #[test]
    fn the_room_holds_the_disc() {
        let across = reach() * 2.0;

        assert!(
            across <= crate::behind::SPAN && across <= crate::behind::DEEP,
            "the orbits are {:.1} m across and the room is {:.1} by {:.1}",
            across,
            crate::behind::SPAN,
            crate::behind::DEEP
        );
    }
}
