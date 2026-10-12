//! The solar system behind the wall. Spec 0015.
//!
//! The sun at the middle of a room a hundred and fifty nine metres square,
//! the eight
//! planets out from it, their moons round them, and stars on every surface.
//! You come in at the cold end and walk toward the light.
//!
//! **Two scales, and the ratio between them is the one lie.** One scale cannot
//! work: put Neptune fifty metres out at true proportion and the Earth is
//! fourteen hundredths of a millimetre. So bodies are measured against bodies
//! and distances against distances, each exactly true inside itself, and the
//! orbit scale is a hundred and two times the body scale. Said out loud here
//! rather than buried.
//!
//! **What sets the orbit scale is the Moon.** A moon is on the body scale, so
//! its orbit is drawn a hundred times too wide against the planets' spacing,
//! and the Moon is the worst of them: it used to take 82 per cent of the way
//! from the Earth's orbit to Mars's and read as reaching for Mars. `MOON_SHARE`
//! holds it to half and `au` is worked out from that, which is what made this
//! room a hundred and fifty nine metres instead of ninety seven.
//!
//! **Why the sun is a metre.** In the orbiting arrangement Mercury has to
//! clear the sun's own ball, and Mercury is at 0.387 AU against Neptune's
//! thirty, so the disc you need comes out at about seventy eight times the
//! sun's diameter. A sun twice as wide costs four times the floor. At a metre
//! the disc is 158.7 across and Mercury's surface clears the sun's by 51.6 cm.
//! That clearance was 11.8 cm before the orbits grew, and it growing with them
//! is a thing the Moon bought that nobody asked for.

use glam::{vec3, Vec3};

/// How many kilometres a metre is, for a ball.
pub const SUN_ACROSS_KM: f32 = 1_392_700.0;

/// How much of the gap between the Earth's orbit and Mars's the Moon's orbit
/// is allowed to take up.
///
/// Half, and this is the number the whole orbit scale now hangs off.
///
/// A moon is on the body scale, per `Moon`, and the body scale is 168 times
/// the orbit scale, so a moon's orbit is drawn 168 times too wide against the
/// planets' spacing. The Moon is the worst of them by a long way: at an `AU`
/// of 1.6 it reached 0.690 m against a gap to Mars of 0.838, which is 82 per
/// cent of it and 15 cm of daylight between the Moon and a planet 1.2 cm
/// across. Jake looked at it and said the Moon's orbit gets really close to
/// Mars, which is exactly what it was doing.
///
/// Nothing about that is a slip and nothing about it was hidden: `BODY`'s own
/// note says the Moon is what capped the bodies at two and a half times,
/// because at three the Moon's orbit is exactly the gap. What it got wrong
/// was the threshold. It stopped at the point where the two touch, and the
/// point where a model starts telling a lie about which things are near which
/// is a long way before that.
///
/// So this is the stated threshold, and it is written as the share rather
/// than as a distance so that growing the bodies again moves the orbits with
/// them instead of quietly eating the margin. Same reasoning as `SUN_SHRUNK`.
///
/// Half is a judgment and the only one here. It is the point at which the
/// Moon's orbit reads as belonging to the Earth rather than as reaching for
/// Mars. Two fifths wants a room two hundred metres across.
pub const MOON_SHARE: f32 = 0.5;

/// Where the Earth and Mars go, in astronomical units.
///
/// Named out here because `au` works itself out from the gap between them and
/// `PLANETS` puts them at it, and two accounts of one distance is the fault
/// this building keeps paying for.
pub const EARTH_OUT: f32 = 1.0;
pub const MARS_OUT: f32 = 1.5237;

/// How many metres one astronomical unit is drawn as.
///
/// Worked out and not chosen: the Moon's orbit is fixed by the body scale, so
/// the orbit scale is whatever makes that orbit `MOON_SHARE` of the gap from
/// the Earth to Mars. It comes out at 2.63 m, against the 1.6 it was built
/// with, and that is a room a hundred and fifty nine metres square rather
/// than ninety seven.
///
/// What that buys is the model keeping its one promise. Every proportion in
/// here stays exactly what it was: the Moon is still thirty Earths from the
/// Earth, Callisto is still thirteen Jupiters from Jupiter, and Neptune is
/// still thirty times as far out as the Earth. The only thing that moved is
/// how much floor it takes, and the floor was the cheap thing.
pub fn au() -> f32 {
    LUNA[0].far() / (MOON_SHARE * (MARS_OUT - EARTH_OUT))
}

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
        .fold(0.0f32, |most, planet| most.max(planet.out * au()))
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
        self.out * au()
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
        out: EARTH_OUT,
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
        out: MARS_OUT,
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

/// How much slower the moons run than the planets' own clock would have
/// them, and it is the third lie in this model after the two scales and the
/// sun's size.
///
/// Twenty four. The moons were on the planets' clock, which put the Moon
/// round the Earth thirteen times an Earth year and was written down here as
/// the one fact about the Moon that everybody has. The arithmetic was right
/// and what it bought was a blur: a year is sixty seconds, a month is a
/// three hundred and sixty fifth of a year, and Io went round Jupiter three
/// and a half times a second. Jake watched it and said it looked janky.
///
/// It was also wrecking the names. The layout on the glass has to choose
/// between names that cross, and with the moons whipping round it was
/// choosing again thirty six times a second. Slowing them took that to two,
/// and two more things I had already added to damp it turned out to have
/// been damping this.
///
/// What it costs is the fact it was written for: the Moon now goes round the
/// Earth about half an Earth year. What it keeps is the better one. Io,
/// Europa and Ganymede come out at 7, 14 and 28 seconds, which is the 1:2:4
/// those three are actually in, and at those rates you can stand and watch
/// it happen. A resonance you can see beats a ratio you cannot.
pub const MOONS_DAWDLE: f32 = 24.0;

/// How long a moon takes to go round its planet, on the orbit clock slowed
/// by `MOONS_DAWDLE`.
pub fn month_seconds(moon: &Moon) -> f32 {
    moon.month / 365.256 * YEAR * MOONS_DAWDLE
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

/// The same place on a circle, said as the shortest turn from the line.
///
/// `angle_at` counts from the moment the building opened and never comes
/// back, so after five minutes Mercury is twenty one turns along. Wrapped
/// into a half turn either way it is the same place and a number the swing
/// can use. See `stands`.
pub fn shortest_turn(angle: f32) -> f32 {
    use std::f32::consts::{PI, TAU};

    let round = angle.rem_euclid(TAU);

    if round > PI {
        round - TAU
    } else {
        round
    }
}

/// Where a planet is, between lined up and in orbit.
///
/// `swung` is nought lined up and one in orbit, and everything between is a
/// real place on the planet's own circle: it keeps its distance from the sun
/// the whole way, so the line opens into the orbits rather than the planets
/// flying off and coming back. That is the whole of why the key does not snap.
///
/// `toward` is the way the line points, which is the way the door is.
///
/// **The shortest way round, and that is not a tidy-up.** This multiplied
/// `angle_at` by `swung`, and `angle_at` counts from the moment the building
/// opened and never comes back. So the sweep was however far the planet had
/// travelled since you walked in, crammed into the three seconds of the
/// swing: a planet whirling round the sun rather than fanning out to its
/// place. Half a minute in it was two turns of Mercury, five minutes in it
/// was twenty one, fifteen minutes in it was sixty two, and it went on
/// getting worse for as long as the building stayed open. Jake watched it and
/// said switching modes was a bit chaotic.
///
/// `shortest_turn` lands the planet in the same place and asks for at most
/// half a turn to get there, whatever the clock says.
///
/// **What is left is a jump once an orbit.** The wrap flips from half a turn
/// one way to half a turn the other, and while `swung` is between nought and
/// one that moves the planet. It is the full width of the orbit for a planet
/// caught exactly at the wrap, and Mercury passes it every fourteen seconds,
/// so a three second swing has a fair chance of meeting it. Taking it out
/// means carrying each planet's drawn angle from frame to frame and easing it
/// rather than working it out fresh, which is a different shape of code and
/// is not this. See `tests::a_swing_is_never_more_than_half_a_turn`.
pub fn stands(planet: &Planet, toward: Vec3, swung: f32, since: f32) -> Vec3 {
    let along = toward.normalize_or_zero();
    let side = vec3(-along.z, 0.0, along.x);
    let turn = shortest_turn(angle_at(planet, since)) * swung.clamp(0.0, 1.0);

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
/// The room is a hundred and fifty nine metres square and black, the way out
/// is a slot
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
/// How tall a body's name is drawn, as a fraction of how far off it is.
///
/// Fixed on the glass rather than in the room, which is the whole of why a
/// name out here can be read at all. Triton is five millimetres across and
/// sixty four metres from the door, and a name that shrank with it would be
/// one texel: the thing a label has to beat is not the dark, it is the
/// distance. Scaled with the distance instead, every name in the room reads
/// the same size from wherever you are standing.
///
/// The camera's `fov_y` is 60 degrees, so a screen is 1.155 times as tall as
/// it is far away and this is a line 2.4 per cent of the screen's height: 26
/// pixels on a 1080 window, which is three times over what this font needs.
///
/// What it costs is that the far names are enormous in the room. Neptune's is
/// a metre and a third of lettering beside a ball nine centimetres across.
/// That is the trade and it is the right way round: a model you cannot find
/// the planets in is not a model of anything.
pub const LABEL_SUBTENDS: f32 = 0.028;

/// The smallest a name is drawn, in metres.
///
/// Nose to the glass on the Moon, `LABEL_SUBTENDS` of 20 cm is 6 mm of
/// lettering, which is under this font's own grid. A floor rather than a
/// ceiling, because the fault at the far end is a name too big and the fault
/// at the near end is a name gone.
pub const LABEL_LEAST: f32 = 0.018;

/// How many texels tall a name's picture is drawn.
pub const LABEL_TEXELS: f32 = 64.0;

/// How far off a body its name stands, in names.
///
/// Clear of the ball by its own radius and then most of a line, so there is
/// sky between the two and the name is not sitting on the planet's shoulder.
pub const LABEL_UP: f32 = 0.8;

/// Which side of its body a name hangs on.
///
/// Planets over, moons under. Not decoration: in the lined up arrangement
/// every body is on one line, and read from along that line every name lands
/// on the same row and they pile into each other. A moon is always within a
/// stride of its planet and is the pair most likely to collide, so hanging
/// the moons underneath pulls the worst of it apart and says which body
/// belongs to which at the same time.
///
/// It does not fix the rest. Four inner planets within a metre of each other,
/// read from a metre away, still stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hangs {
    Over,
    Under,
}

/// How tall a name is drawn, seen from `how_far` away.
pub fn label_tall(how_far: f32) -> f32 {
    (how_far * LABEL_SUBTENDS).max(LABEL_LEAST)
}

/// Where a body's name stands: over the ball, clear of it and of its own
/// height.
///
/// `across` is what the body is **drawn** as and not what it is, so the sun
/// passes `sun_drawn` and not `SUN.drawn`.
pub fn label_at(body: Vec3, across: f32, eye: Vec3, hangs: Hangs) -> Vec3 {
    let tall = label_tall((body - eye).length());
    let side = match hangs {
        Hangs::Over => 1.0,
        Hangs::Under => -1.0,
    };

    body + Vec3::Y * side * (across * 0.5 + tall * (0.5 + LABEL_UP))
}

/// Which way to turn a `room::tiled_plane` so its face looks at `eye` and its
/// letters stand up.
///
/// `room::laid_on` is the same job for a wall and only answers the six
/// directions a wall faces, so this is the general one. The quad is built flat
/// with its normal on local +Y, its letters running along +X and their tops on
/// -Z: that last is what `laid_on` says, which is why this is checked against
/// it rather than against a drawing of it. See
/// `tests::a_name_turned_at_a_wall_is_the_wall`.
///
/// **`laid_on` is only upright for two of its four walls.** Checking this
/// against it turned that up: facing -x or -z it puts the quad's top at -y, so
/// anything with an up to it comes out upside down. Nothing has noticed
/// because the three surfaces it draws are tiling plaster, tile and lining,
/// none of which has a top, and the one sign that uses it faces +z. So the
/// comparison below holds for +x and +z and the normal alone for the rest.
///
/// The letters are made level by construction rather than by projecting world
/// up out of the normal. Projecting leaves a residue near the poles: a body a
/// thousandth off straight overhead came out rolled by a ten-thousandth, which
/// is nothing to look at and is still a quad whose letters are not level. The
/// run of the letters is `Vec3::Y` crossed into the normal, which has no y in
/// it at any angle, and the up is worked back out of that.
///
/// Straight up or straight down has no such run, and then any upright will do.
pub fn label_turn(from: Vec3, eye: Vec3) -> glam::Quat {
    let out = eye - from;
    if out.length_squared() < 1e-8 {
        return glam::Quat::IDENTITY;
    }

    let looks = out.normalize();
    let along = Vec3::Y.cross(looks);
    let along = if along.length_squared() < 1e-6 {
        Vec3::X
    } else {
        along.normalize()
    };
    let up = looks.cross(along);

    glam::Quat::from_mat3(&glam::Mat3::from_cols(along, looks, -up))
}

/// One body's name, waiting to be laid out: where its body is, how big that
/// body is drawn, and which side the name hangs on.
#[derive(Clone, Copy, Debug)]
pub struct Named {
    /// Its place in `every_body`, which is the order the pictures are in.
    pub which: usize,
    pub at: Vec3,
    pub across: f32,
    pub hangs: Hangs,
    /// How many times wider than tall the name's picture came out.
    pub wide: f32,
}

/// Which names get the room when two want it: the sun, then the planets,
/// then the moons, and distance only inside each.
///
/// A moon sweeping past is nearer than a planet for a moment, so on distance
/// alone it took the planet's name away and gave it back. The sun's own name
/// was among the ones going: it cannot move, so what was happening to it was
/// being walked over.
///
/// Ordering by what a name is worth puts the fast thing last, so the name
/// that goes is a moon's, which is the one worth least: its planet is right
/// beside it and named. Nothing a moon does can take a planet's name now.
///
/// Worth 1.0 changes a second against 1.5 with `STICKS` alone, measured
/// standing still.
fn worth(name: &Named) -> u8 {
    match (name.which, name.hangs) {
        (0, _) => 0,
        (_, Hangs::Over) => 1,
        (_, Hangs::Under) => 2,
    }
}

/// How much less room a name already up asks for than one trying to come up.
///
/// A quarter either way, so a pair has to close to three quarters of
/// touching before the one behind goes, and open to a quarter clear before
/// it comes back. Between those the answer is whatever it was last frame.
///
/// Worth 1.5 changes a second against 2.2 without it, measured standing
/// still. It was put in to fix a flicker of thirty six a second and did
/// almost nothing, because that flicker was `MOONS_DAWDLE` and not this; a
/// dwell time was added on top, got the same number down to 1.0, and was
/// taken out again once the clock was fixed, because it then bought nothing
/// and cost a name overlapping for four tenths of a second before giving
/// way. This and `worth` stayed because they each still earn their line.
pub const STICKS: f32 = 0.25;

/// How tall a name is on the glass, as a fraction of half the screen.
///
/// Constant, and that is the gift `LABEL_SUBTENDS` gives this: a name is
/// `LABEL_SUBTENDS` of its own distance tall, and the screen is
/// `2·d·tan(fov/2)` tall at that distance, so the distance cancels. Every
/// name in the room is the same size on the glass, so the layout below needs
/// only where each one is and not how far off.
pub fn label_on_the_glass(fov_y: f32) -> f32 {
    LABEL_SUBTENDS / (fov_y * 0.5).tan()
}

/// Which names to draw, nearest first, dropping any that would land on one
/// already kept.
///
/// **Why there is a layout at all.** Every name is the same size on the glass
/// and every body sits at eye height, so the names land on two rows and slide
/// through each other as you move. Lined up they are already touching;
/// swinging into the orbits they cross wholesale and the rows turn into one
/// band of letters: "ars EarVenMercury", "Ga.medeCalli". Jake watched the
/// swing and called it chaotic, and the planets whirling was only half of it.
///
/// Nearest wins, because the near body is the one you are looking at and the
/// one whose name you can check against a ball you can see. A name dropped is
/// a name you get by taking a step, which is how a planetarium behaves and is
/// better than a smear that names nothing.
///
/// Rectangles and not circles, because a name is a long thin thing and a
/// circle round one would reserve a square of sky it does not use. The width
/// comes off each name's own picture, so "the moon" keeps more room than
/// "Io".
///
/// Behind you and not wholly on the glass are both dropped before anything
/// is compared, so a name at your back cannot take the place of one in
/// front, and a name is never drawn with its end cut off by the frame.
///
/// `shown` is what was drawn last frame: a name already up keeps its place on
/// a smaller gap than a name trying to come up needs, by `STICKS` either
/// way, so a pair hovering at the edge of overlapping holds whatever it was
/// doing.
///
/// **It was not the flicker, though it was blamed for it.** Standing still
/// and touching nothing, this set changed thirty six times a second, and
/// hysteresis took that to twenty three and a ranking took it to twenty one.
/// None of it was the layout: it was `MOONS_DAWDLE`, with Io going round
/// Jupiter three and a half times a second. Fixing the clock took the same
/// number to two, from which these two together buy the last one.
pub fn laid_out(
    names: &[Named],
    eye: Vec3,
    seen_through: glam::Mat4,
    fov_y: f32,
    aspect: f32,
    shown: &[usize],
) -> Vec<usize> {
    let tall = label_on_the_glass(fov_y);
    let mut want: Vec<(u8, f32, usize, glam::Vec2, f32)> = Vec::new();

    for name in names {
        let stands = label_at(name.at, name.across, eye, name.hangs);
        let clip = seen_through * stands.extend(1.0);

        // behind the eye, where a divide by w turns the picture inside out
        if clip.w <= 1e-4 {
            continue;
        }

        let on_glass = glam::vec2(clip.x / clip.w, clip.y / clip.w);

        // over `aspect`, because `tall` is a fraction of the glass's height
        // and this is across its width. Without it a name reserves a third
        // more room than it uses on a window a third wider than it is tall,
        // and the test below throws out names that would have fitted.
        let half = glam::vec2(tall * name.wide * 0.5 / aspect.max(1e-3), tall * 0.5);

        // and anything not wholly on the glass. Wholly, and not merely
        // touching it: a name hanging off the edge is drawn sliced, and
        // "Ganymede" cut at the frame reads as "anymede", which is worse
        // than no name at all because it looks like a name. Gone, it comes
        // back the moment you turn a few degrees.
        if on_glass.x.abs() + half.x > 1.0 || on_glass.y.abs() + half.y > 1.0 {
            continue;
        }

        want.push((
            worth(name),
            (name.at - eye).length_squared(),
            name.which,
            on_glass,
            half.x,
        ));
    }

    want.sort_by(|one, other| one.0.cmp(&other.0).then(one.1.total_cmp(&other.1)));

    let mut kept: Vec<(glam::Vec2, f32)> = Vec::new();
    let mut out = Vec::new();

    for (_, _, which, on_glass, half_wide) in want {
        // a name that is already up is harder to push off than a name that
        // is not is to let up
        let asks = if shown.contains(&which) {
            1.0 - STICKS
        } else {
            1.0 + STICKS
        };

        let clear = kept.iter().all(|(other, other_wide)| {
            (on_glass.x - other.x).abs() >= (half_wide + other_wide) * asks
                || (on_glass.y - other.y).abs() >= tall * asks
        });

        if clear {
            kept.push((on_glass, half_wide));
            out.push(which);
        }
    }

    out
}

/// A body's name, cut out of its ground.
///
/// Stencilled, then drawn on a black plate, then stencilled again, and the
/// round trip is worth writing down because each leg had a reason that was
/// true when it was given.
///
/// Cut out first, so a name out here would hang in the sky with nothing
/// behind it. Then on a plate, because:
///
/// a cut-out name was gone at any distance, and a plate read against the sun
/// as well as against the dark. The sky here is black, so a black plate
/// against it looked free.
/// And cut out again, because the first of those two was never a property
/// of cut-out text. A cut-out surface wrote no depth anywhere, so whatever
/// drew after it painted over it however far behind it that was, and out at
/// Saturn the thing drawing after it was the star wall. The engine's spec
/// 0047 has the whole of it. A cut-out word stands up at any distance now,
/// which was checked at twenty two metres where there had been nothing.
///
/// So the plate is back to being the thing it always was: a black rectangle
/// over whatever it crosses. Invisible against the sky, which is what made
/// it look free, and a hole punched in Saturn's rings and in the planets.
/// That is the cost this takes off, and it takes it off by giving up the
/// second reason above: against the sun's own blown-out metre of white, a
/// name is now white on white and goes. Trade a name you cannot read in one
/// place for plates you cannot see anywhere, and the place in question is
/// the one body in the room nobody needs told the name of.
pub fn named(name: &str) -> blitzkit::texture::TextureData {
    blitzkit::text::stencilled(name, LABEL_TEXELS)
}

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

    /// Spec 0015: a moon goes round slowly enough to watch, and the
    /// Galileans keep their 1:2:4.
    ///
    /// The moons were on the planets' own clock, which is right arithmetic
    /// and was a blur: a year is sixty seconds and a month is a three
    /// hundred and sixty fifth of a year, so Io went round Jupiter three and
    /// a half times a second. Jake watched it and said it looked janky. It
    /// was also what made the names on the glass choose again thirty six
    /// times a second.
    ///
    /// Two halves, and the second is the one that matters. A ceiling on how
    /// fast the quickest moon may go is a number somebody picked. The 1:2:4
    /// is not: Io, Europa and Ganymede are really in that resonance, it is
    /// the famous thing about them, and `MOONS_DAWDLE` has to leave it
    /// alone. Turn `MOONS_DAWDLE` into something that is not a plain
    /// multiplier and the second half fails.
    #[test]
    fn a_moon_can_be_watched_and_the_galileans_keep_their_resonance() {
        // how long, not which way: Triton goes round Neptune backwards and
        // carries a negative month to say so, which this test found by
        // reporting the quickest moon as minus twenty three seconds
        let round: Vec<f32> = PLANETS
            .iter()
            .flat_map(|planet| planet.moons.iter())
            .map(|moon| month_seconds(moon).abs())
            .collect();
        let slowest = round.iter().copied().fold(0.0f32, f32::max);
        let quickest = round.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            (4.0..=20.0).contains(&quickest),
            "the quickest moon goes round in {:.2}s, which is not a thing to              stand and watch",
            quickest
        );
        assert!(
            slowest < 60.0 * 5.0,
            "the slowest moon takes {:.0}s, which is a moon that does not move",
            slowest
        );

        // and the resonance, read off the model rather than off the table
        let of = |name: &str| {
            month_seconds(
                PLANETS
                    .iter()
                    .flat_map(|planet| planet.moons.iter())
                    .find(|moon| moon.body.name == name)
                    .expect("a moon by that name"),
            )
            .abs()
        };

        let io = of("Io");

        for (name, times) in [("Europa", 2.0), ("Ganymede", 4.0)] {
            let got = of(name) / io;

            assert!(
                (got - times).abs() < 0.05,
                "{} goes round {:.3} times for Io's one, not {:.0}",
                name,
                got,
                times
            );
        }
    }

    /// Spec 0015: two names that would land on each other, do not.
    ///
    /// The fault this is for: every name is the same size on the glass and
    /// every body sits at eye height, so the names lie on two rows and slide
    /// through each other as you move. Swinging into the orbits they cross
    /// wholesale and the rows become one band of letters.
    ///
    /// Built here rather than taken from the room, so the overlap is a thing
    /// this test chose and not a thing the model happened to do: two bodies
    /// a hand apart, dead ahead, at the same height. Their names are five
    /// times as wide as they are tall and the glass is the camera's own 60
    /// degrees, so they cannot both fit.
    ///
    /// The control is the rule itself: drop the overlap test in `laid_out`
    /// and the first assertion goes.
    #[test]
    fn two_names_on_one_another_are_cut_to_one() {
        let eye = vec3(0.0, 0.0, 0.0);
        let seen_through = seen_from(eye);
        // not index 0, which `every_body` makes the sun and `worth` puts
        // first whatever its distance. A first draft used it and the swap
        // below came back with the far name, correctly.
        let names = vec![
            Named {
                which: 1,
                at: vec3(0.0, 0.0, -10.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 5.0,
            },
            Named {
                which: 2,
                at: vec3(0.1, 0.0, -11.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 5.0,
            },
        ];

        let fits = laid_out(&names, eye, seen_through, 60f32.to_radians(), 1.0, &[]);

        assert_eq!(fits, vec![1], "both names were kept, or the wrong one");

        // and the near one is the one kept. Swap the distances and the
        // answer swaps with them, which is the half of this that says it is
        // nearest-first and not first-in-the-list.
        let swapped = vec![
            Named {
                at: vec3(0.0, 0.0, -11.0),
                ..names[0]
            },
            Named {
                at: vec3(0.1, 0.0, -10.0),
                ..names[1]
            },
        ];

        assert_eq!(
            laid_out(&swapped, eye, seen_through, 60f32.to_radians(), 1.0, &[]),
            vec![2]
        );
    }

    /// Spec 0015: names far enough apart are all kept.
    ///
    /// The other half, and the one that stops the rule from being "draw one
    /// name". Four bodies spread across the glass, no two within a name's
    /// width of each other.
    ///
    /// The camera is 1:1 here, which is `Camera::new`'s own viewport and is
    /// what makes these numbers checkable by hand: at ten metres with a
    /// sixty degree glass, a body four metres off the middle lands at 0.69
    /// of the way out and a name is 0.05 tall. A first draft put them at six
    /// metres, which is 1.04 and off the glass, and the off-glass guard
    /// threw them out exactly as it should.
    #[test]
    fn names_that_clear_each_other_are_all_kept() {
        let eye = Vec3::ZERO;
        let seen_through = seen_from(eye);
        let names: Vec<Named> = [-4.0f32, -1.4, 1.4, 4.0]
            .iter()
            .enumerate()
            .map(|(n, across)| Named {
                which: n,
                at: vec3(*across, 0.0, -10.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 1.0,
            })
            .collect();

        let mut fits = laid_out(&names, eye, seen_through, 60f32.to_radians(), 1.0, &[]);
        fits.sort_unstable();

        assert_eq!(
            fits,
            vec![0, 1, 2, 3],
            "a name was dropped with room to spare"
        );
    }

    /// Spec 0015: a name hanging off the edge is not drawn at all.
    ///
    /// The guard used to throw out only a name that was wholly off the
    /// glass, so one straddling the frame was kept and drawn sliced.
    /// "Ganymede" cut at the right edge reads as "anymede", which is worse
    /// than no name because it looks like a name. Jake's screenshot of
    /// 2026-10-12 has that and "Titan" halved in the same frame.
    ///
    /// Walks a name out from the middle. There is a place where it stops
    /// being drawn, and it is before any part of it leaves, not after all of
    /// it has.
    #[test]
    fn a_name_half_off_the_glass_is_not_drawn() {
        let eye = Vec3::ZERO;
        let seen_through = seen_from(eye);
        let wide = 5.0;
        let at = |across: f32| {
            vec![Named {
                which: 1,
                at: vec3(across, 0.0, -10.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide,
            }]
        };
        let fits = |across: f32| {
            !laid_out(&at(across), eye, seen_through, 60f32.to_radians(), 1.0, &[]).is_empty()
        };

        assert!(fits(0.0), "a name in the middle was dropped");

        // the last place it is still drawn, to a centimetre
        let mut last = 0.0f32;
        let mut across = 0.0f32;

        while across < 12.0 {
            if fits(across) {
                last = across;
            }
            across += 0.01;
        }

        assert!(
            last > 1.0,
            "a name only reaches {:.2} m off the middle",
            last
        );

        // and at that place the whole of it is on the glass: its far edge
        // has not crossed the frame. Worked out here rather than taken from
        // `laid_out`, so the two have to agree.
        let tall = label_on_the_glass(60f32.to_radians());
        let edge = {
            let stands = label_at(vec3(last, 0.0, -10.0), 0.0, eye, Hangs::Over);
            let clip = seen_through * stands.extend(1.0);

            (clip.x / clip.w).abs() + tall * wide * 0.5
        };

        assert!(
            edge <= 1.0,
            "the last name drawn reaches {:.3} of the way out, which is past              the frame",
            edge
        );

        // and a centimetre further it is gone, so the edge is where it says
        assert!(
            !fits(last + 0.02),
            "it is still drawn past its own last place"
        );
    }

    /// Spec 0015: a wider window fits more names, not fewer.
    ///
    /// The half width of a name is a fraction of the glass's *height*
    /// scaled across its width, so it has to be divided by the aspect. It
    /// was not, which made every name reserve a third more room than it uses
    /// on an ordinary window and threw out names that would have fitted.
    ///
    /// Found while fixing the edge above, and invisible in every other test
    /// here because they all run at 1:1, which is `Camera::new`'s own
    /// viewport and the one aspect where the bug does nothing.
    #[test]
    fn a_wider_window_fits_more_names() {
        let eye = Vec3::ZERO;
        let seen_through = seen_from(eye);
        let names: Vec<Named> = (0..9)
            .map(|n| Named {
                which: n + 1,
                at: vec3(-4.0 + n as f32, 0.0, -10.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 5.0,
            })
            .collect();

        let square = laid_out(&names, eye, seen_through, 60f32.to_radians(), 1.0, &[]).len();
        let wide = laid_out(&names, eye, seen_through, 60f32.to_radians(), 1.8, &[]).len();

        assert!(
            wide > square,
            "a 1.8 window fits {} names and a square one {}",
            wide,
            square
        );
    }

    /// Spec 0015: a name behind you cannot take the place of one in front.
    ///
    /// The one that bites if the divide by w goes unguarded: a point behind
    /// the eye comes back through the projection mirrored into the middle of
    /// the glass, where it would sit on top of whatever is actually there.
    #[test]
    fn a_name_behind_you_is_not_laid_out() {
        let eye = Vec3::ZERO;
        let seen_through = seen_from(eye);
        let names = vec![
            // dead ahead, and further off than the one behind
            Named {
                which: 0,
                at: vec3(0.0, 0.0, -20.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 5.0,
            },
            // at your back, and nearer, so nearest-first would take it first
            Named {
                which: 1,
                at: vec3(0.0, 0.0, 3.0),
                across: 0.0,
                hangs: Hangs::Over,
                wide: 5.0,
            },
        ];

        assert_eq!(
            laid_out(&names, eye, seen_through, 60f32.to_radians(), 1.0, &[]),
            vec![0],
            "the name at your back was laid out"
        );
    }

    /// A camera at `eye` looking down -z, the way the tests above want it.
    fn seen_from(eye: Vec3) -> glam::Mat4 {
        let mut camera = blitzkit::camera::Camera::new();
        camera.position = eye;
        camera.target = eye - Vec3::Z;
        camera.fov_y = 60f32.to_radians();

        camera.view_projection()
    }

    /// Spec 0015: a name turned at a wall is the turn that wall already uses.
    ///
    /// `label_turn` is the general form of `crate::room::laid_on`, which has
    /// drawn every lined wall in this building and is the only thing that
    /// knows which way round the quad's letters go. Written from scratch it
    /// came out rolled a quarter turn twice, so it is checked against the one
    /// that works rather than against a drawing of it.
    ///
    /// Both basis vectors and not the quaternions, because a quaternion and
    /// its negative are the same turn.
    ///
    /// Only +x and +z, and that is the finding. `laid_on` faces the right way
    /// on all four but stands the quad's top at -y on -x and -z, so a sign on
    /// either of those walls would read upside down. The three surfaces it
    /// draws are tiling and have no top, and the one sign using it faces +z,
    /// which is why nothing has ever shown it. The other two are held to the
    /// normal alone below, which is the part it does get right.
    #[test]
    fn a_name_turned_at_a_wall_is_the_wall() {
        for looks in [Vec3::X, Vec3::Z] {
            let wall = crate::room::laid_on(looks);
            let name = label_turn(Vec3::ZERO, looks * 3.0);

            for local in [Vec3::X, Vec3::Y, Vec3::Z] {
                assert!(
                    (wall * local - name * local).length() < 1e-5,
                    "looking {:?}, a wall puts {:?} at {:?} and a name at {:?}",
                    looks,
                    local,
                    wall * local,
                    name * local
                );
            }
        }

        for looks in [-Vec3::X, -Vec3::Z] {
            let wall = crate::room::laid_on(looks);
            let name = label_turn(Vec3::ZERO, looks * 3.0);

            assert!(
                (wall * Vec3::Y - name * Vec3::Y).length() < 1e-5,
                "looking {:?}, the two face {:?} and {:?}",
                looks,
                wall * Vec3::Y,
                name * Vec3::Y
            );
            assert!(
                (wall * -Vec3::Z).dot(Vec3::Y) < 0.0,
                "`laid_on({:?})` is upright now, so this test is out of date",
                looks
            );
        }
    }

    /// Spec 0015: a name faces you, stands up, and does not roll.
    ///
    /// The three things a billboard has to do, from a dozen places including
    /// straight under a body, which is the one that has no up to find.
    #[test]
    fn a_name_faces_you_and_stands_up() {
        let eye = vec3(2.0, 1.55, -3.0);

        for at in [
            vec3(0.0, 1.2, 0.0),
            vec3(8.0, 1.2, 4.0),
            vec3(-20.0, 3.0, 40.0),
            // directly over the eye, where world up is edge on
            vec3(2.0, 9.0, -3.0),
            // and directly under it
            vec3(2.0, -4.0, -3.0),
            // and a hair off straight up, which is the case either side of
            // the fallback
            vec3(2.001, 9.0, -3.0),
        ] {
            let turn = label_turn(at, eye);
            let looks = turn * Vec3::Y;
            let want = (eye - at).normalize();

            assert!(
                (looks - want).length() < 1e-4,
                "a name at {:?} faces {:?} and not {:?}",
                at,
                looks,
                want
            );

            // no roll: the letters run level
            let along = turn * Vec3::X;
            assert!(
                along.dot(Vec3::Y).abs() < 1e-4,
                "the letters at {:?} run {:?}, which is not level",
                at,
                along
            );

            // and the right way up. The quad's letters have their tops on
            // local -Z, which is what `laid_on` says and what the test above
            // holds it to. Skipped where there is no up: over the eye and
            // under it, any upright is as good as another.
            if (want.dot(Vec3::Y)).abs() < 0.99 {
                assert!(
                    (turn * -Vec3::Z).dot(Vec3::Y) > 0.0,
                    "the letters at {:?} are upside down",
                    at
                );
            }

            // orthonormal, so nothing is sheared or mirrored
            assert!(
                (turn * Vec3::X).cross(turn * Vec3::Y).dot(turn * Vec3::Z) > 0.99,
                "the turn at {:?} is not a turn",
                at
            );
        }
    }

    /// Spec 0015: a name is the same size on the glass wherever it is read
    /// from, and never under this font's grid.
    ///
    /// The point of the whole thing. Triton five millimetres across and sixty
    /// metres off is the body this is for: its ball is a texel and its name is
    /// a name.
    #[test]
    fn a_name_is_the_same_size_however_far_off_it_is() {
        for how_far in [1.0f32, 4.0, 16.0, 64.0] {
            let subtends = label_tall(how_far) / how_far;

            assert!(
                (subtends - LABEL_SUBTENDS).abs() < 1e-6,
                "at {}m a name subtends {:.4} and not {:.4}",
                how_far,
                subtends,
                LABEL_SUBTENDS
            );
        }

        // and close up it stops shrinking rather than going to nothing
        assert_eq!(label_tall(0.0), LABEL_LEAST);
        assert!(label_tall(0.1) >= LABEL_LEAST);

        // 26 pixels of a 1080 window, at the camera's own 60 degree fov
        let screen = 2.0 * (60f32.to_radians() * 0.5).tan();
        let pixels = LABEL_SUBTENDS / screen * 1080.0;
        assert!(
            (20.0..=40.0).contains(&pixels),
            "a name is {:.0} pixels tall on a 1080 window",
            pixels
        );
    }

    /// Spec 0015: a name stands clear of its own body, at every size.
    ///
    /// Over it and not on it. The sun is the one that matters: it is a metre
    /// across and a name half a metre up is a name inside it.
    #[test]
    fn a_name_stands_clear_of_its_body() {
        let eye = vec3(0.0, 1.55, 0.0);

        for (what, at, across) in std::iter::once(("the sun", Vec3::ZERO, sun_drawn()))
            .chain(PLANETS.iter().map(|planet| {
                (
                    planet.body.name,
                    vec3(planet.far(), 1.2, 0.0),
                    planet.body.drawn(),
                )
            }))
            .chain(PLANETS.iter().flat_map(|planet| {
                planet.moons.iter().map(move |moon| {
                    (
                        moon.body.name,
                        vec3(planet.far() + moon.far(), 1.2, 0.0),
                        moon.body.drawn(),
                    )
                })
            }))
        {
            let tall = label_tall((at - eye).length());

            for hangs in [Hangs::Over, Hangs::Under] {
                let stands = label_at(at, across, eye, hangs);
                let gap = match hangs {
                    Hangs::Over => (stands.y - tall * 0.5) - (at.y + across * 0.5),
                    Hangs::Under => (at.y - across * 0.5) - (stands.y + tall * 0.5),
                };

                assert!(
                    gap > 0.0,
                    "{}'s name {:?} overlaps it by {:.3}m",
                    what,
                    hangs,
                    -gap
                );
                assert!(
                    (gap - tall * LABEL_UP).abs() < 1e-4,
                    "{}'s name {:?} stands {:.3}m off and not {:.3}m",
                    what,
                    hangs,
                    gap,
                    tall * LABEL_UP
                );
            }
        }
    }

    /// Spec 0015: every body has a name, and no two share one.
    ///
    /// The pictures and the names come off the same `every_body`, and the
    /// drawing walks it with one index for both. A name repeated is two
    /// bodies that cannot be told apart, and an empty one is a body with
    /// nothing over it.
    #[test]
    fn every_body_is_named_once() {
        let bodies = every_body();

        assert_eq!(
            bodies.len(),
            1 + PLANETS.len() + 7,
            "{} bodies",
            bodies.len()
        );

        assert_eq!(
            bodies[0].name, SUN.name,
            "the sun is not the first body, and `worth` ranks it by being so"
        );

        let mut seen = Vec::new();
        for body in &bodies {
            assert!(!body.name.trim().is_empty(), "a body with no name");
            assert!(
                !seen.contains(&body.name),
                "two bodies called {}",
                body.name
            );
            seen.push(body.name);
        }

        // and each name draws something. A string this font never heard of
        // comes back as one white texel, which is a name gone.
        for body in &bodies {
            let said = named(body.name);

            assert!(
                said.width() > 8 && said.height() > 4,
                "{} drew a picture {}x{}",
                body.name,
                said.width(),
                said.height()
            );
        }
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

    /// Spec 0015: the Moon takes half the gap to Mars, and the orbit scale is
    /// what gives it that.
    ///
    /// This test used to say the opposite, that the Moon was what stopped the
    /// bodies growing, and it passed on a margin of 15 cm with Mars 1.2 cm
    /// across. The old threshold was the point where the two touch, and a
    /// model starts lying about which things are near which a long way before
    /// then: Jake looked at it and said the Moon's orbit gets really close to
    /// Mars.
    ///
    /// `MOON_SHARE` is the threshold now and `au` is worked out from it, so
    /// this holds the share and not a distance. Growing the bodies again
    /// moves the orbits with them instead of eating the margin.
    #[test]
    fn the_moon_takes_half_the_gap_to_mars() {
        let earth = planet("Earth");
        let mars = planet("Mars");
        let moon = earth.moons[0];
        let gap = mars.far() - earth.far();
        let share = moon.far() / gap;

        assert!(
            (share - MOON_SHARE).abs() < 1e-4,
            "the Moon takes {:.3} of the {:.2} m gap to Mars and not {:.3}",
            share,
            gap,
            MOON_SHARE
        );

        // and it is still the Moon that binds. Nothing is gained by holding
        // the Moon to a half if Callisto is quietly worse.
        for pair in PLANETS.windows(2) {
            let (inner, outer) = (pair[0], pair[1]);
            let reach = inner
                .moons
                .iter()
                .fold(0.0f32, |most, moon| most.max(moon.far()));

            if reach == 0.0 {
                continue;
            }

            let took = reach / (outer.far() - inner.far());

            assert!(
                took <= MOON_SHARE + 1e-4,
                "{}'s moons take {:.3} of the gap to {}, past the Moon's {:.3}",
                inner.body.name,
                took,
                outer.body.name,
                MOON_SHARE
            );
        }

        // and the Moon is still thirty Earths out, which is the fact the whole
        // arrangement is for and the thing growing the room protects
        let earths = moon.far() / earth.body.drawn();

        assert!(
            (earths - 30.1).abs() < 0.2,
            "the Moon is {:.1} Earths out, not 30.1",
            earths
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
    /// It was 67.1 when the sun's own diameter set the body scale, and 167.8
    /// once the bodies grew two and a half times to be worth looking at.
    /// Growing the orbits to keep the Moon off Mars pulls it back to 101.9,
    /// which is the one good thing about the bigger room that nobody asked
    /// for: the two scales are closer together now than at any time since the
    /// bodies grew.
    ///
    /// If this number ever drifts it will be because somebody changed one
    /// scale and not the other, and the model will still look fine.
    #[test]
    fn the_orbit_scale_is_a_hundred_times_the_body_scale() {
        let orbit_km_per_metre = 149_597_871.0 / au();
        let body_km_per_metre = 1.0 / BODY;
        let times = orbit_km_per_metre / body_km_per_metre;

        assert!(
            (times - 101.9).abs() < 0.2,
            "the orbit scale is {:.2} times the body scale, not 101.9",
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
                let wrong = moon.out_km / 149_597_871.0 * au();

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

    /// Spec 0015: a swing asks for at most half a turn, however long the
    /// building has been open.
    ///
    /// It used to ask for however far the planet had travelled since you
    /// walked in. `stands` multiplied `angle_at` by `swung`, and `angle_at`
    /// counts from the moment the building opened and never comes back, so
    /// the sweep grew all evening: two turns of Mercury half a minute in,
    /// twenty one at five minutes, sixty two at fifteen. Three seconds of
    /// that is a planet whirling and not a line fanning open.
    ///
    /// `the_swing_keeps_every_planet_at_its_own_distance` passed throughout
    /// and could not have done otherwise: a planet twenty turns round its
    /// own circle is still exactly on it. That is the test that looked like
    /// it covered this and did not.
    ///
    /// The clocks here go well past any sitting anybody will have. The
    /// control is the line this replaced: put `angle_at` back in place of
    /// `shortest_turn` in `stands` and the first assertion fails at half a
    /// minute.
    #[test]
    fn a_swing_is_never_more_than_half_a_turn() {
        use std::f32::consts::PI;

        let toward = vec3(1.0, 0.0, 0.0);

        for since in [0.0f32, 1.0, 30.0, 120.0, 300.0, 900.0, 3_600.0, 36_000.0] {
            for planet in PLANETS.iter() {
                let turn = shortest_turn(angle_at(planet, since));

                assert!(
                    turn.abs() <= PI + 1e-4,
                    "{} asks for {:.1} turns after {:.0}s open",
                    planet.body.name,
                    turn.abs() / (PI * 2.0),
                    since
                );

                // and the swing still arrives where the orbit says, which is
                // the half of this that must not have changed. Worked out
                // from `angle_at` straight, not from `shortest_turn`, so the
                // two have to agree rather than agreeing by construction.
                let raw = angle_at(planet, since);
                let along = toward.normalize();
                let side = vec3(-along.z, 0.0, along.x);
                let orbit = (along * raw.cos() + side * raw.sin()) * planet.far();

                assert!(
                    (stands(planet, toward, 1.0, since) - orbit).length() < 1e-2,
                    "{} lands {:.3} m off its orbit after {:.0}s open",
                    planet.body.name,
                    (stands(planet, toward, 1.0, since) - orbit).length(),
                    since
                );

                // and lined up is still lined up
                let lined = stands(planet, toward, 0.0, since);

                assert!(
                    (lined - toward.normalize() * planet.far()).length() < 1e-3,
                    "{} is not on the line at rest",
                    planet.body.name
                );
            }
        }
    }

    /// Spec 0015: the swing keeps every planet at its own distance, the whole
    /// way through.
    ///
    /// This is what makes it read as one model rearranged rather than two
    /// models cross faded, and it is the only thing about the animation that
    /// can be checked without eyes.
    ///
    /// It is also the test that looked like it covered the sweep and did
    /// not: see `a_swing_is_never_more_than_half_a_turn`.
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
