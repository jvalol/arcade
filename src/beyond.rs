//! The space beyond the door of spec 0017, where the system stands at true
//! scale. Spec 0018.
//!
//! Spec 0015 bends two numbers to make a solar system that fits in a room: the
//! bodies are drawn at one scale and their distances at another, 306 times
//! apart, and the sun is shrunk again on top of that. Both lies are written
//! down there and both are what make that model worth looking at.
//!
//! Here nothing is bent but the light. One scale, every body and every
//! distance true against every other, and 33 kilometres of floor to put it on.
//!
//! Nothing here draws. Where each body stands, how big it is, where you land
//! when you jump and how far the sun reaches are arithmetic, and arithmetic
//! can be checked without a window.

use glam::{vec3, Vec3};

use crate::orrery::{Moon, Planet, AU_KM, PLANE, PLANETS, SUN};
use crate::room::THICK;

/// How big the smallest body in the tables is drawn.
///
/// Two centimetres, which is a marble, and it is the only number in this file
/// that was chosen. Everything else is a consequence of it.
pub const MARBLE: f32 = 0.02;

/// How far the sun stands north of the door.
///
/// Thirty metres. The sun is 10.29 across, so from the doorway it subtends
/// nineteen and a half degrees, which is about 350 pixels of a 1080 screen:
/// unmistakably a ball and not a lamp, and eight seconds' walk.
pub const SUN_OUT: f32 = 30.0;

/// How wide the floor is.
///
/// Forty. Callisto stands 13.91 metres off the line and a moon you cannot
/// walk round is a moon drawn for nobody.
pub const WIDE: f32 = 40.0;

/// How far the floor laps back under the hidden room's own, and how far it
/// runs past the last body.
pub const LAPS: f32 = 3.0;
pub const SPARE: f32 = 40.0;

/// How far the camera sees out here.
///
/// The building's is a hundred metres, which is further than any of it goes
/// and is what keeps the far wall of the hidden room a thing you find rather
/// than a thing you see. Out here a hundred metres culls the sun from
/// everywhere past Mercury, which throws away the one reading this space
/// exists for.
///
/// So it reaches past the far end of the floor, and it does that only out
/// here: inside, the hundred stands.
pub fn sees() -> f32 {
    SUN_OUT + reach() + SPARE * 2.0
}

/// How far the sun's light throws, and how hard.
///
/// A hundred kilometres against a model thirty three long, which is the one
/// thing bent in this space. A point light gives `(1 - d/range)²`, so this
/// leaves Mercury at 0.99 and Neptune at 0.45. Real sunlight falls as the
/// inverse square: Neptune gets a nine hundredth of what the Earth gets, and
/// drawn honestly everything past Jupiter is black.
///
/// What this space is for survives the bend. The reading is the sun
/// shrinking, 22 pixels from Mercury to a third of one from Neptune, and that
/// needs the planets lit enough to find rather than lit correctly.
pub const SUN_RANGE: f32 = 100_000.0;
pub const SUN_LIT: f32 = 2.6;

/// How big a body is drawn, from its size in kilometres.
pub fn drawn(across_km: f32) -> f32 {
    across_km * per_km()
}

/// How many metres one kilometre is, which is the whole of the scale.
///
/// Worked out from the tables and not written down: the smallest body in them
/// is drawn as a marble, so a smaller moon joining them moves the whole model
/// rather than quietly breaking the one promise this spec makes.
///
/// It comes to one metre for every 135,350 kilometres, because the smallest
/// body in the tables is Triton at 2,707 km across.
pub fn per_km() -> f32 {
    MARBLE / smallest()
}

/// The smallest body in the tables, across, in kilometres.
pub fn smallest() -> f32 {
    crate::orrery::every_body()
        .into_iter()
        .fold(f32::INFINITY, |least, body| least.min(body.across))
}

/// How far out a planet stands, in metres.
pub fn far(planet: &Planet) -> f32 {
    planet.out * AU_KM * per_km()
}

/// How far out a moon stands from its planet, in metres.
///
/// Its true distance, where spec 0015 has to hold the moons in to a third to
/// keep the Moon's orbit out of Mars. Nothing is held in here.
pub fn moon_far(moon: &Moon) -> f32 {
    moon.out_km * per_km()
}

/// How far the whole thing reaches from the sun: Neptune, plus Triton beyond
/// it.
pub fn reach() -> f32 {
    PLANETS.iter().fold(0.0f32, |most, planet| {
        let moons = planet
            .moons
            .iter()
            .fold(0.0f32, |out, moon| out.max(moon_far(moon)));

        most.max(far(planet) + moons)
    })
}

/// One stop on the way out: what it is called, where it stands, and how big
/// it is drawn. A body, or the door home.
#[derive(Clone, Copy, Debug)]
pub struct Stop {
    pub name: &'static str,
    pub at: Vec3,
    pub across: f32,
    /// Which body this is in `orrery::every_body` order, which is what the
    /// textures and the name pictures are keyed by. The way home is not a
    /// body and has none.
    pub which: Option<usize>,
}

/// Where the sun stands.
pub fn sun(door: Vec3) -> Vec3 {
    vec3(door.x, drawn(SUN.across) * 0.5, door.z + SUN_OUT)
}

/// Every stop, in the order they come out from the sun, with each planet's
/// moons after it. The door is the first, so stepping in from the sun is how
/// you leave.
///
/// The one account of where anything in this space is. The drawing reads it,
/// the jump reads it, and the tests read it.
pub fn stops(door: Vec3) -> Vec<Stop> {
    let sun_at = sun(door);
    let mut out = vec![
        Stop {
            name: "the way back",
            at: door,
            across: 0.0,
            which: None,
        },
        Stop {
            name: SUN.name,
            at: sun_at,
            across: drawn(SUN.across),
            which: Some(0),
        },
    ];
    let mut which = 1;

    for planet in PLANETS.iter() {
        let at = vec3(sun_at.x, PLANE, sun_at.z + far(planet));

        out.push(Stop {
            name: planet.body.name,
            at,
            across: drawn(planet.body.across),
            which: Some(which),
        });
        which += 1;

        // the moons stand off at right angles to the line, so that a planet
        // and its moons are one picture rather than three bodies in a row you
        // cannot tell from the next planet out
        for moon in planet.moons.iter() {
            out.push(Stop {
                name: moon.body.name,
                at: at + Vec3::X * moon_far(moon),
                across: drawn(moon.body.across),
                which: Some(which),
            });
            which += 1;
        }
    }

    out
}

/// How far short of a body you stand when you arrive at it.
///
/// Its own size, two and a half times over, with a floor: a marble is 60 cm
/// away and the sun is 26 metres. Far enough that the body fills the view and
/// no further, which is the same question at every scale and so is answered
/// once.
pub const STANDOFF: f32 = 2.5;
pub const NEAREST: f32 = 0.6;

pub fn stands_off(across: f32) -> f32 {
    (across * STANDOFF).max(NEAREST)
}

/// Where you stand when you arrive at `stop`, having come from `from`, and
/// which way you then face.
///
/// Short of it on the line you travelled in along, looking at it. So pressing
/// on carries you the way you were already going and turning round takes you
/// back, and no key has to mean out or in.
pub fn arriving(from: Vec3, stop: &Stop) -> (Vec3, Vec3) {
    let along = (stop.at - from).normalize_or(Vec3::Z);
    let at = stop.at - along * stands_off(stop.across);

    (vec3(at.x, 0.0, at.z), along)
}

/// How fast the ride carries you down the line, per spec 0018.
///
/// Eight hundred metres a second, which is the whole system in 42 seconds of
/// travel: half a second to Mercury, five to Jupiter, and fifteen across the
/// gulf to Neptune. Running it is 73 minutes and walking it is two and a
/// quarter hours.
///
/// It was 185, which is three minutes to Neptune, and that was held there to
/// keep Mercury perceptible at a couple of seconds. The stops took the
/// constraint away: a body you are set down at is not something you have to
/// catch sight of in passing, so the inner legs are free to be as short as
/// they truly are.
///
/// Constant, and so the time of a leg stays in the same proportion as its
/// length whatever the speed. It would be wrong to ease it. An acceleration
/// would put the inner planets in slow motion and hurry the gulf, which is the
/// shape of the lie every diagram of the solar system tells.
pub const RIDES: f32 = 800.0;

/// How fast the ride slides you back onto the line.
///
/// You can only ride along the line, so a ride that starts from a moon has to
/// bring you back to it. Callisto is the furthest off at 13.9 metres, which
/// this covers in under two seconds of a leg that is twenty-six, so it is over
/// long before there is anything to look at.
pub const ASIDE: f32 = 8.0;

/// Whether a stop stands on the line, which is everything but the moons.
///
/// The ride runs along the line and can only put you down on it. A planet's
/// moons stand out at right angles, so they are the jump's to reach, and
/// `next_along` steps over them.
pub fn on_the_line(stop: &Stop, door: Vec3) -> bool {
    (stop.at.x - door.x).abs() < 1e-3
}

/// The next stop on the line from `here`, the way you are facing.
///
/// The first stop that way which is not a moon. From Jupiter facing on that is
/// Saturn, four moons later; from Callisto facing back it is Jupiter.
///
/// **Which way is the facing's own, and not the chain's**, which is where this
/// differs from the jump. The jump reads the facing against the way the chain
/// was going because the chain turns a right angle at every moon. The ride
/// never turns one: it runs along the line and nowhere else, so the line is
/// what the facing is read against. Taking the chain's rule here, riding out
/// of Callisto went to Saturn whichever way you faced, because the way in to
/// Callisto was at right angles to both answers.
pub fn next_along(stops: &[Stop], here: usize, facing: Vec3, door: Vec3) -> Option<usize> {
    let out = facing.z >= 0.0;
    let mut n = here;

    loop {
        n = if out { n + 1 } else { n.checked_sub(1)? };

        let stop = stops.get(n)?;

        if on_the_line(stop, door) {
            return Some(n);
        }
    }
}

/// Where you come to rest at `stop`, having ridden up to it from `from`.
///
/// Short of it by its own standoff, on the line, which is where the jump puts
/// you as well. One account of where you stand at a body, so arriving by one
/// means and leaving by the other does not shift you.
pub fn resting(stops: &[Stop], from: usize, at: usize, door: Vec3) -> Vec3 {
    let (stood, _) = arriving(stops[from].at, &stops[at]);

    vec3(door.x, stood.y, stood.z)
}

/// Where the ride takes you over `dt`, and the stop it has brought you to.
///
/// Along the line and nothing else. You keep the mouse, which is the whole
/// point of riding rather than jumping, because the thing to watch is the sun
/// going small behind you. You do not steer: at 185 metres a second a ten
/// degree turn crosses the 20 metres to the edge of the floor in under a
/// second, and over the edge there is no floor at all.
///
/// **It stops when it gets there.** Holding and watching and letting go is a
/// judgment nobody can make in the dark, with the thing you are aiming at a
/// speck that is still a speck a second before you reach it. So the ride is a
/// leg rather than a throttle: it carries you to the next body on the line and
/// sets you down where the jump would have, and going on is another press.
///
/// A frame at this speed is three metres, and the standoff at a marble is
/// sixty centimetres, so arriving has to be a crossing and not a nearness:
/// landing is clamped to the resting place rather than tested against it.
///
/// `None` back means you are still travelling. At either end facing past it
/// there is nowhere to go and you do not move at all.
pub fn ridden(
    stops: &[Stop],
    here: usize,
    at: Vec3,
    facing: Vec3,
    dt: f32,
    door: Vec3,
) -> (Vec3, Option<usize>) {
    let Some(next) = next_along(stops, here, facing, door) else {
        return (at, None);
    };
    let rest = resting(stops, here, next, door);
    let way = (rest.z - at.z).signum();
    let gone = at.z + way * RIDES * dt;
    // sideways, back toward the line, which only matters when the leg started
    // at a moon
    let aside = (door.x - at.x).clamp(-ASIDE * dt, ASIDE * dt);
    let arrived = (gone - rest.z) * way >= 0.0;

    if arrived {
        (rest, Some(next))
    } else {
        (vec3(at.x + aside, at.y, gone), None)
    }
}

/// What the map at the foot of the screen shows, per spec 0018.
///
/// Fractions and not pixels: where to put it and how big is the drawing's
/// business, and how far along you are is this module's. Nought is the door
/// and one is the last body on the line.
#[derive(Clone, Debug, Default)]
pub struct Map {
    /// Where you are along the whole line.
    pub along: f32,
    /// Where each stop on the line is, in the same measure.
    pub ticks: Vec<f32>,
    /// The leg you are on: what you came from, what you are going to, and how
    /// far through it you are. Nothing at the far end facing out, where there
    /// is nowhere to go.
    pub leg: Option<(&'static str, &'static str, f32)>,
}

/// Where you are, for the map.
///
/// Two measures and not one, because one cannot do it. Linear over the whole
/// line, Mercury is at a hundredth and the four inner planets are inside the
/// first twentieth: that picture is true, and it is the reason the solar
/// system is hard to draw, but you cannot read your distance to the next body
/// off it. So the leg is measured again on its own, nought where you left and
/// one where you are going, and that is the one that answers how far.
pub fn mapped(stops: &[Stop], here: usize, at: Vec3, facing: Vec3, door: Vec3) -> Map {
    let line: Vec<usize> = (0..stops.len())
        .filter(|n| on_the_line(&stops[*n], door))
        .collect();
    let Some(last) = line.last().map(|n| stops[*n].at.z) else {
        return Map::default();
    };
    let whole = (last - door.z).max(1e-3);
    let share = |z: f32| ((z - door.z) / whole).clamp(0.0, 1.0);

    // what you came from is the stop you are at when that is on the line, and
    // the one behind you when it is not: standing at a moon, the leg you are
    // on is its planet's
    let from = if on_the_line(&stops[here], door) {
        Some(here)
    } else {
        next_along(stops, here, -facing, door)
    };
    let to = next_along(stops, here, facing, door);

    Map {
        along: share(at.z),
        ticks: line.iter().map(|n| share(stops[*n].at.z)).collect(),
        leg: from.zip(to).map(|(from, to)| {
            let (one, other) = (stops[from].at.z, stops[to].at.z);
            let through = ((at.z - one) / (other - one)).clamp(0.0, 1.0);

            (stops[from].name, stops[to].name, through)
        }),
    }
}

/// How far apart the posts down the line stand, and how big they are.
///
/// A pair every 250 metres, which at the ride's speed is three a second: fast
/// enough to read as speed and slow enough to count. Without them the ride is
/// an unchanging black screen, which reads as broken rather than as vast.
///
/// It was every hundred, from when the ride was 185 metres a second. At 800
/// that is eight a second, which is not a rhythm and is a flicker rate to keep
/// away from besides.
///
/// **Standing up, and not laid on the floor**, which is what they were first.
/// A mark on the floor seen from eye height is edge-on past twenty metres: at
/// ninety the depression angle is one degree, so three tenths of a metre of
/// mark came to five thousandths of a metre of picture and nothing was drawn
/// at all. It is the lesson the hall's columns already teach, which spec 0011
/// puts as the only way to read a distance being to see the same thing at
/// several of them.
pub const POSTS: f32 = 250.0;
pub const POST_OFF: f32 = 2.0;
pub const POST_WIDE: f32 = 0.1;
pub const POST_HIGH: f32 = 2.0;

/// And the bigger ones, one to the astronomical unit.
///
/// Thirty of them over the whole line. Nothing says what they are and nothing
/// needs to: the Earth stands on the first.
///
/// They glow, which is the only thing that carries across this space. A post
/// nine tenths of a metre tall is a pixel at seven hundred metres and nothing
/// at a kilometre, and the gulfs here are measured in tens of kilometres; the
/// door of spec 0017 reads across seventy nine metres of unlit room on the
/// same trick, for the same reason.
pub const MARK_WIDE: f32 = 0.14;
pub const MARK_HIGH: f32 = 4.0;

/// The spine down the line, flat on the floor.
pub const SPINE_WIDE: f32 = 0.3;
pub const SPINE_HIGH: f32 = 0.02;

/// One thing standing by the line: where it is, how big, and whether it gives
/// off light of its own.
#[derive(Clone, Copy, Debug)]
pub struct Marker {
    pub at: Vec3,
    pub size: Vec3,
    pub glows: bool,
}

/// How far one astronomical unit is out here.
pub fn au() -> f32 {
    AU_KM * per_km()
}

/// How far the line runs: from the door to the far end of the floor.
pub fn runs(door: Vec3) -> (f32, f32) {
    let (middle, size) = floor(door);

    (door.z, middle.z + size.z * 0.5 - SPARE)
}

/// The spine down the line and everything standing beside it.
///
/// The spine is laid on the floor and not sunk into it, which is the way round
/// this building has learnt the hard way: a border sunk into the floor was two
/// surfaces in one plane and came back as speckle.
pub fn marked(door: Vec3) -> Vec<Marker> {
    let sun = sun(door);
    let (from, to) = runs(door);
    let mut out = vec![Marker {
        at: vec3(sun.x, SPINE_HIGH * 0.5, (from + to) * 0.5),
        size: vec3(SPINE_WIDE, SPINE_HIGH, to - from),
        glows: false,
    }];

    let mut pair = |at: f32, wide: f32, high: f32, glows: bool| {
        for side in [-1.0f32, 1.0] {
            out.push(Marker {
                at: vec3(sun.x + side * POST_OFF, high * 0.5, at),
                size: vec3(wide, high, wide),
                glows,
            });
        }
    };

    let mut at = sun.z;
    while at <= to {
        pair(at, POST_WIDE, POST_HIGH, false);
        at += POSTS;
    }

    // and the units. The one at the sun is not numbered: nought is where you
    // are measuring from and the sun is already named there.
    for unit in 1..=units() {
        pair(unit_at(door, unit).z, MARK_WIDE, MARK_HIGH, true);
    }

    out
}

/// How many whole astronomical units the line runs to.
pub fn units() -> u32 {
    (reach() / au()) as u32
}

/// Where the posts for a unit stand, at the floor.
///
/// The one account of it: `marked` puts the posts there and `numbered` hangs
/// the number over them, and two accounts of where a thing is come apart.
pub fn unit_at(door: Vec3, unit: u32) -> Vec3 {
    let sun = sun(door);

    vec3(sun.x, 0.0, sun.z + au() * unit as f32)
}

/// Which unit mark is the next one the way you are facing, and where its
/// number stands.
///
/// The next one and not the nearest. Nearest is the one you have just gone
/// past, so riding out you never see a number at all: it comes up behind your
/// shoulder a frame after you have passed the post it belongs to.
///
/// Over the posts rather than under them, because the bodies' own names hang
/// under out here and the Earth stands on the first of these: without the two
/// going opposite ways, "Earth" and "1 AU" are one smear.
///
/// Nothing past the thirtieth, which is as far as the line runs, and nothing
/// before the first: nought is where you are measuring from and the sun is
/// already named there.
pub fn numbered(door: Vec3, at: Vec3, facing: Vec3) -> Option<(u32, Vec3)> {
    let sun = sun(door);
    let out = (at.z - sun.z) / au();
    // the smallest post past you going out, the largest short of you coming
    // back, and never the one at the sun: from the door the first post ahead
    // is nought, which is the sun's own and carries no number.
    let unit = if facing.z >= 0.0 {
        (out.floor() + 1.0).max(1.0)
    } else {
        out.ceil() - 1.0
    };

    if !(1.0..=units() as f32).contains(&unit) {
        return None;
    }

    Some((unit as u32, vec3(sun.x, MARK_HIGH, sun.z + au() * unit)))
}

/// Which way the chain was going when it brought you to `here`.
///
/// From the stop before it to this one, and for the door, which has nothing
/// before it, the way on to the sun.
pub fn along(stops: &[Stop], here: usize) -> Vec3 {
    let (from, to) = match here.checked_sub(1) {
        Some(before) => (stops.get(before), stops.get(here)),
        None => (stops.first(), stops.get(1)),
    };

    match (from, to) {
        (Some(from), Some(to)) => (to.at - from.at).normalize_or(Vec3::Z),
        _ => Vec3::Z,
    }
}

/// Which stop a press of Return takes you to from `here`, per spec 0018.
///
/// Which way you go is which way you are facing, measured against the way the
/// chain was already going. On if you are facing along it, back if against,
/// and neither at the end you are facing past: at the far end facing out, and
/// at the door facing back into the room, there is nowhere to go.
///
/// **Against the chain's direction and not against the next stop's.** The
/// chain turns a right angle at every moon, because a planet's moons stand out
/// at right angles to the line: arriving at the Moon you face along the
/// Earth-to-Moon line, and Mars from there is 579 metres off at ninety
/// degrees to it. Compared with the way to Mars, facing on from the Moon
/// pointed at nothing and the chain stopped dead there.
///
/// **Where you are is remembered and not worked out**, which took two goes to
/// accept. In metres the nearest stop is wrong at the sun: you arrive 25.7
/// short of it and that is 4.6 from the door, so a second press takes you to
/// the sun you are already standing at. Measured in standoffs instead, so the
/// stop you travelled to always reads about one, it is wrong at every moon:
/// standing at Io you are 1.0 of Io's standoff from Io and 0.95 of Jupiter's
/// from Jupiter.
///
/// There is no metric that gets both, because a model whose largest body is
/// five hundred times its smallest has no one distance that means "here". The
/// chain knows where you are in it. That is one `usize`, and crossing the
/// threshold sets it to the door.
pub fn next(stops: &[Stop], here: usize, facing: Vec3) -> Option<usize> {
    if facing.dot(along(stops, here)) >= 0.0 {
        stops.get(here + 1).map(|_| here + 1)
    } else {
        here.checked_sub(1)
    }
}

/// The floor, as a middle and a size: one slab from under the hidden room's
/// own to past the last body.
///
/// It laps back under that one for the reason spec 0017 gives. Two floors at
/// one height that meet edge to edge put a vertical face at the surface you
/// are walking on, and a body resting on a surface counts as touching it.
pub fn solid(reaches: f32) -> blitzkit::collision::Aabb {
    let (middle, size) = floor(at(reaches));

    blitzkit::collision::Aabb::from_center_size(middle, size)
}

/// Where the door of spec 0017 stands, at the floor, which is where all of
/// this is measured from.
pub fn at(reaches: f32) -> Vec3 {
    let middle = crate::behind::at(reaches);

    vec3(
        middle.x,
        0.0,
        middle.z + crate::behind::half().y + THICK * 0.5,
    )
}

pub fn floor(door: Vec3) -> (Vec3, Vec3) {
    let from = door.z - LAPS;
    let to = sun(door).z + reach() + SPARE;

    (
        vec3(door.x, -THICK * 0.5, (from + to) * 0.5),
        vec3(WIDE, THICK, to - from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orrery::{every_body, PLANETS};

    fn reaches() -> f32 {
        crate::room::Room::of(Vec::new()).reaches
    }

    fn door() -> Vec3 {
        at(reaches())
    }

    fn neptune_out() -> f32 {
        neptune().out
    }

    fn neptune() -> &'static Planet {
        PLANETS
            .iter()
            .find(|planet| planet.body.name == "Neptune")
            .expect("Neptune is in the tables")
    }

    /// Spec 0018: the scale comes off the smallest body in the tables, and
    /// that body is a two centimetre marble.
    ///
    /// The middle assertion is the one doing the work, and the first draft did
    /// not have it. `drawn` is `MARBLE` divided by the smallest body, so
    /// asserting that the smallest body is drawn `MARBLE` across is asserting
    /// that a number equals itself: it passed with the marble set to five
    /// centimetres. One metre to 135,350 kilometres is the two of them
    /// multiplied out, and it moves if either does.
    #[test]
    fn the_smallest_body_is_a_marble() {
        let least = every_body()
            .into_iter()
            .min_by(|one, other| one.across.total_cmp(&other.across))
            .expect("the tables are not empty");

        assert_eq!(least.name, "Triton");
        assert_eq!(least.across, 2707.0);

        let km = 1.0 / per_km();

        assert!(
            (km - 135_350.0).abs() < 2.0,
            "one metre is {:.0} km, not 135,350",
            km
        );
        // and nothing in the tables is under the marble, which is the half
        // that would go wrong quietly if a smaller moon were added
        for body in every_body() {
            assert!(
                drawn(body.across) >= 0.02 - 1e-6,
                "{} is drawn {} across, under two centimetres",
                body.name,
                drawn(body.across)
            );
        }
    }

    /// Spec 0018: one scale, which is the whole of the difference between
    /// this space and the room of spec 0015.
    ///
    /// In there a body is drawn at one scale and its distance at another, 306
    /// times apart, and a moon is held to a third of even that. Here every
    /// size and every distance divides by the same number.
    #[test]
    fn one_scale_and_no_other() {
        let scale = per_km();

        for body in every_body() {
            assert!(
                (drawn(body.across) / body.across - scale).abs() < 1e-9,
                "{} is drawn at {} metres to the kilometre and the scale is {}",
                body.name,
                drawn(body.across) / body.across,
                scale
            );
        }

        for planet in PLANETS.iter() {
            assert!(
                (far(planet) / (planet.out * AU_KM) - scale).abs() < 1e-9,
                "{} stands at {} metres to the kilometre",
                planet.body.name,
                far(planet) / (planet.out * AU_KM)
            );

            for moon in planet.moons.iter() {
                assert!(
                    (moon_far(moon) / moon.out_km - scale).abs() < 1e-9,
                    "{} stands at {} metres to the kilometre",
                    moon.body.name,
                    moon_far(moon) / moon.out_km
                );
            }
        }

        // and against the room's, which is the model this one is the answer
        // to: there a body's scale and an orbit's are 306 apart
        let orbits = crate::orrery::au() / AU_KM;
        let apart = crate::orrery::BODY / orbits;

        assert!(
            (apart - 306.0).abs() < 1.0,
            "spec 0015's two scales are {} apart, not 306",
            apart
        );
    }

    /// Spec 0018: every stop stands on the floor, with nothing sunk into it
    /// and nothing off the edge.
    #[test]
    fn nothing_stands_off_the_floor() {
        let door = door();
        let slab = solid(reaches());

        for stop in stops(door) {
            assert!(
                stop.at.x - stop.across * 0.5 > slab.min.x
                    && stop.at.x + stop.across * 0.5 < slab.max.x,
                "{} runs off the side of the floor at {}",
                stop.name,
                stop.at
            );
            assert!(
                stop.at.z > slab.min.z && stop.at.z < slab.max.z,
                "{} stands past the end of the floor at {}",
                stop.name,
                stop.at
            );
            assert!(
                stop.at.y - stop.across * 0.5 > -1e-4,
                "{} is sunk {} into the floor",
                stop.name,
                stop.across * 0.5 - stop.at.y
            );
        }
    }

    /// Spec 0018: you arrive short of the body, looking at it, with it filling
    /// a sensible part of the view.
    ///
    /// The second half is the one worth measuring. A standoff proportional to
    /// the body is the only one that works at both ends of a model whose
    /// largest thing is five hundred times its smallest, and this says what
    /// the two ends come to.
    #[test]
    fn you_arrive_looking_at_it() {
        let stops = stops(door());

        for (n, stop) in stops.iter().enumerate().skip(1) {
            let (stood, facing) = arriving(stops[n - 1].at, stop);
            let eye = stood + Vec3::Y * crate::EYE;
            let away = eye.distance(stop.at);

            assert!(
                facing.dot((stop.at - eye).normalize_or_zero()) > 0.9,
                "arriving at {} you face {} and it is at {}",
                stop.name,
                facing,
                stop.at
            );
            assert!(
                (away / stands_off(stop.across) - 1.0).abs() < 0.1,
                "{} is {} away and the standoff is {}",
                stop.name,
                away,
                stands_off(stop.across)
            );

            // and it is worth looking at: a marble comes to two degrees and
            // the sun to twenty three
            let across = (stop.across / away).atan().to_degrees();

            assert!(
                (1.8..=24.0).contains(&across),
                "{} subtends {:.2} degrees where you land",
                stop.name,
                across
            );
        }
    }

    /// Spec 0018: which way you go is which way you face, and no key means
    /// out or in.
    #[test]
    fn the_way_you_face_is_the_way_you_go() {
        let stops = stops(door());

        for here in 1..stops.len() - 1 {
            let (_, facing) = arriving(stops[here - 1].at, &stops[here]);

            assert_eq!(
                next(&stops, here, facing),
                Some(here + 1),
                "facing on from {}, the next stop is not {}",
                stops[here].name,
                stops[here + 1].name
            );
            assert_eq!(
                next(&stops, here, -facing),
                Some(here - 1),
                "turned round at {}, the way back is not {}",
                stops[here].name,
                stops[here - 1].name
            );
        }
    }

    /// Spec 0018: the door is a stop, so there is a way home from anywhere,
    /// and there is nothing past either end.
    #[test]
    fn the_door_is_the_last_stop_in() {
        let door = door();
        let stops = stops(door);

        assert_eq!(stops[0].at, door);

        // stepping in from the sun is leaving
        let (_, facing) = arriving(door, &stops[1]);

        assert_eq!(next(&stops, 1, -facing), Some(0));

        // and nothing past either end
        let last = stops.len() - 1;
        let (_, facing) = arriving(stops[last - 1].at, &stops[last]);

        assert_eq!(
            next(&stops, last, facing),
            None,
            "there is a stop past Triton"
        );
        assert_eq!(
            next(&stops, 0, -Vec3::Z),
            None,
            "there is a stop behind the door"
        );
    }

    /// Spec 0018: the ride crosses in the same proportion as the distance,
    /// which is the whole reason it is not a jump.
    #[test]
    fn the_ride_crosses_in_proportion() {
        let took = |planet: &Planet| far(planet) / RIDES;
        let crossing = took(neptune());

        assert!(
            (crossing - 42.0).abs() < 4.0,
            "Neptune is {:.0} seconds off, not forty two",
            crossing
        );

        // and every leg in proportion to its own distance, which an eased
        // ride would not be: that is the shape of the lie every diagram of
        // the solar system tells
        for planet in PLANETS.iter() {
            let share = took(planet) / crossing;
            let truth = planet.out / neptune_out();

            assert!(
                (share - truth).abs() < 1e-5,
                "{} is {:.4} of the way by time and {:.4} by distance",
                planet.body.name,
                share,
                truth
            );
        }

        // the numbers the jump was written to get round
        assert!(
            far(neptune()) / (crate::SPEED * crate::RUNS) > 3600.0,
            "running to Neptune is under an hour, so the ride is not needed"
        );
    }

    /// Spec 0018: and it stays on the line whatever you are looking at.
    ///
    /// Looking about while you travel is the whole point of riding rather than
    /// jumping, because the thing to watch is the sun going small behind you.
    /// Steering at 185 metres a second is the thing that would put you over
    /// the edge of the floor in under a second.
    #[test]
    fn the_ride_stays_on_the_line() {
        let door = door();
        let stops = stops(door);
        // mid-way between Uranus and Neptune, which is the longest leg, and
        // from Uranus, which is on the line: a leg out of a moon is
        // `the_ride_steps_over_the_moons`
        let uranus = stops
            .iter()
            .position(|stop| stop.name == "Uranus")
            .expect("Uranus is a stop");
        let from = vec3(door.x, 0.0, sun(door).z + 27_000.0);

        for facing in [
            Vec3::X,
            -Vec3::X,
            vec3(0.9, 0.2, 0.3).normalize(),
            vec3(-0.9, -0.2, -0.3).normalize(),
        ] {
            let (went, got) = ridden(&stops, uranus, from, facing, 1.0, door);

            assert_eq!(got, None, "facing {} arrived somewhere in a second", facing);
            assert_eq!(went.x, from.x, "facing {} took you off the line", facing);
            assert_eq!(went.y, from.y);
            assert!(
                ((went.z - from.z).abs() - RIDES).abs() < 1e-3,
                "facing {} covered {} in a second",
                facing,
                went.z - from.z
            );
            // and out or back by the sign of the facing, nothing else
            assert_eq!(
                went.z > from.z,
                facing.z >= 0.0,
                "facing {} went the wrong way",
                facing
            );
        }
    }

    /// Spec 0018: a leg of the ride ends at the next body on the line, where
    /// the jump would have put you.
    ///
    /// Holding and watching and letting go is a judgment nobody can make in
    /// the dark, with the thing you are aiming at a speck that is still a
    /// speck a second before you reach it.
    #[test]
    fn a_leg_of_the_ride_ends_at_the_next_body() {
        let door = door();
        let stops = stops(door);
        let mut at = door;
        let mut here = 0;
        let step = 1.0 / 60.0;
        let mut legs = Vec::new();

        // ride out, one leg at a time, counting how long each took, until the
        // line runs out
        while next_along(&stops, here, Vec3::Z, door).is_some() {
            let was = here;
            let mut took = 0.0;

            loop {
                let (went, got) = ridden(&stops, here, at, Vec3::Z, step, door);
                at = went;
                took += step;

                if let Some(got) = got {
                    here = got;
                    break;
                }

                assert!(took < 600.0, "a leg from {} never ended", stops[was].name);
            }

            assert!(
                here > was,
                "the ride went backwards from {}",
                stops[was].name
            );
            legs.push((stops[here].name, took));

            // and set you down where the jump would have
            let jumped = arriving(stops[was].at, &stops[here]).0;

            assert!(
                at.distance(vec3(door.x, jumped.y, jumped.z)) < 0.05,
                "riding to {} put you at {} and jumping puts you at {}",
                stops[here].name,
                at,
                jumped
            );
        }

        // every planet and the sun, and not one moon: the moons stand out at
        // right angles to the line and are the jump's to reach
        assert_eq!(
            legs.iter().map(|(name, _)| *name).collect::<Vec<_>>(),
            vec![
                "the sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus",
                "Neptune"
            ]
        );

        // and the longest leg is the one the ride exists for
        let (worst, took) = legs
            .iter()
            .max_by(|one, other| one.1.total_cmp(&other.1))
            .expect("there are legs");

        assert_eq!(*worst, "Neptune");
        assert!(
            (13.0..=18.0).contains(took),
            "the gulf before Neptune is {:.0} seconds",
            took
        );
        // and the shortest crossing between two planets is the Earth's, which
        // is the shape of the thing: the inner four are over before the gulf
        // has begun.
        //
        // The sun's leg is left out because it is not a crossing. It is 30
        // metres from the door and you stand 25.7 short of it, so what is left
        // is four metres of threshold.
        let shortest = legs
            .iter()
            .filter(|(name, _)| *name != "the sun")
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .expect("there are legs");

        assert_eq!(shortest.0, "Earth");
        assert!(
            took / shortest.1 > 20.0,
            "the longest leg is only {:.0} times the shortest",
            took / shortest.1
        );
    }

    /// Spec 0018: and it lands on the body rather than near it.
    ///
    /// A frame at this speed is three metres and the standoff at a marble is
    /// sixty centimetres, so arriving has to be a crossing and not a nearness.
    /// One step long enough to pass the whole system still stops at the first
    /// body on the line.
    #[test]
    fn the_ride_does_not_overshoot() {
        let door = door();
        let stops = stops(door);
        let (at, got) = ridden(&stops, 0, door, Vec3::Z, 600.0, door);

        assert_eq!(got, Some(1), "a ten minute step did not stop at the sun");

        let rest = resting(&stops, 0, 1, door);

        assert!(
            at.distance(rest) < 1e-3,
            "it came to rest at {} and the sun's place is {}",
            at,
            rest
        );
    }

    /// Spec 0018: the ride steps over the moons, which stand off the line.
    #[test]
    fn the_ride_steps_over_the_moons() {
        let door = door();
        let stops = stops(door);
        let jupiter = stops
            .iter()
            .position(|stop| stop.name == "Jupiter")
            .expect("Jupiter is a stop");

        assert_eq!(
            next_along(&stops, jupiter, Vec3::Z, door).map(|n| stops[n].name),
            Some("Saturn"),
            "riding on from Jupiter stops at one of its moons"
        );

        // and from a moon it picks up the line either side
        let callisto = stops
            .iter()
            .position(|stop| stop.name == "Callisto")
            .expect("Callisto is a stop");

        assert_eq!(
            next_along(&stops, callisto, Vec3::Z, door).map(|n| stops[n].name),
            Some("Saturn")
        );
        assert_eq!(
            next_along(&stops, callisto, -Vec3::Z, door).map(|n| stops[n].name),
            Some("Jupiter")
        );
    }

    /// Spec 0018: the ride stops at the door rather than going through the
    /// wall, and there is nothing past the far end.
    ///
    /// It has to say so itself: the ride does not go through `walk`, because a
    /// 185 metre a second sweep is three metres a frame looking for something
    /// to catch on, over a floor that is one flat slab with nothing on it.
    #[test]
    fn the_ride_stops_at_both_ends() {
        let door = door();
        let stops = stops(door);
        let (back, got) = ridden(&stops, 1, sun(door), -Vec3::Z, 600.0, door);

        assert_eq!(
            got,
            Some(0),
            "riding in from the sun does not reach the door"
        );
        assert!(
            back.z >= door.z - 1e-3,
            "riding back put you at {}, past the door at {}",
            back.z,
            door.z
        );

        // and nothing past Neptune, which is the last stop on the line
        let neptune = stops
            .iter()
            .position(|stop| stop.name == "Neptune")
            .expect("Neptune is a stop");
        let at = resting(&stops, neptune - 1, neptune, door);
        let (went, got) = ridden(&stops, neptune, at, Vec3::Z, 600.0, door);

        assert_eq!(got, None, "there is a body past Neptune to ride to");
        assert_eq!(went, at, "it moved with nowhere to go");
    }

    /// Spec 0018: the unit posts say which unit they are, and the one that
    /// speaks is the one ahead of you.
    ///
    /// The nearest was the first try and it is the one you have just gone
    /// past: riding out you never see a number at all, because it comes up
    /// behind your shoulder a frame after you pass the post it belongs to.
    #[test]
    fn the_number_is_the_post_ahead() {
        let door = door();
        let sun = sun(door);
        let on =
            |z: f32, facing: Vec3| numbered(door, vec3(door.x, 0.0, z), facing).map(|(n, _)| n);

        // a metre past the fourth post, which is where the nearest rule failed
        assert_eq!(on(sun.z + au() * 4.0 + 1.0, Vec3::Z), Some(5));
        assert_eq!(on(sun.z + au() * 4.0 + 1.0, -Vec3::Z), Some(4));
        // and a metre short of it
        assert_eq!(on(sun.z + au() * 4.0 - 1.0, Vec3::Z), Some(4));
        assert_eq!(on(sun.z + au() * 4.0 - 1.0, -Vec3::Z), Some(3));
    }

    /// Spec 0018: the Earth stands on the first of them, which is what says
    /// what a unit is without a word being spent on it.
    #[test]
    fn the_earth_stands_on_the_first_unit() {
        let door = door();
        let (unit, stands) = numbered(door, door, Vec3::Z).expect("a first post");
        let earth = stops(door)
            .into_iter()
            .find(|stop| stop.name == "Earth")
            .expect("the Earth is a stop");

        assert_eq!(unit, 1);
        assert!(
            (stands.z - earth.at.z).abs() < 1e-2,
            "the first post is at {} and the Earth at {}",
            stands.z,
            earth.at.z
        );
    }

    /// Spec 0018: and there is no number before the first or past the last.
    ///
    /// Nought is where you are measuring from and the sun is already named
    /// there, and the line stops at Neptune. The drawing reads this straight
    /// into its list of pictures, so a number outside what the line holds is a
    /// post with nothing over it at best.
    #[test]
    fn there_is_no_number_off_the_ends() {
        let door = door();
        let sun = sun(door);
        let on =
            |z: f32, facing: Vec3| numbered(door, vec3(door.x, 0.0, z), facing).map(|(n, _)| n);

        assert_eq!(on(door.z, -Vec3::Z), None, "there is a post behind the sun");
        assert_eq!(
            on(sun.z + au() * units() as f32 + 10.0, Vec3::Z),
            None,
            "there is a post past the last unit"
        );

        // and everything it does give is a picture the drawing holds
        for step in 0..400 {
            let z = door.z + (reach() + SPARE) * step as f32 / 400.0;

            for facing in [Vec3::Z, -Vec3::Z] {
                if let Some((unit, _)) = numbered(door, vec3(door.x, 0.0, z), facing) {
                    assert!(
                        (1..=units()).contains(&unit),
                        "{} AU at {} is outside the thirty the line holds",
                        unit,
                        z
                    );
                }
            }
        }
    }

    /// Spec 0018: the map's whole-line bar is linear, and that is why it
    /// cannot be the whole map.
    ///
    /// Mercury sits at a hundredth of the way across and the four inner
    /// planets are all inside the first twentieth. That picture is true, and
    /// it is the reason the solar system is hard to draw, and you cannot read
    /// your distance to the next body off it.
    #[test]
    fn the_whole_line_crushes_the_inner_planets() {
        let door = door();
        let stops = stops(door);
        let map = mapped(&stops, 0, door, Vec3::Z, door);
        let share = |name: &str| {
            let n = stops
                .iter()
                .position(|stop| stop.name == name)
                .expect("a stop");
            let line: Vec<usize> = (0..stops.len())
                .filter(|n| on_the_line(&stops[*n], door))
                .collect();

            map.ticks[line.iter().position(|one| *one == n).expect("on the line")]
        };

        assert!(
            share("Mercury") < 0.02,
            "Mercury is {:.3} of the way across",
            share("Mercury")
        );
        assert!(
            share("Mars") < 0.06,
            "Mars is {:.3} of the way across",
            share("Mars")
        );
        assert!(
            (share("Neptune") - 1.0).abs() < 1e-3,
            "Neptune is {:.3} of the way across",
            share("Neptune")
        );
        // every body on the line and not one moon
        assert_eq!(map.ticks.len(), 10);
    }

    /// Spec 0018: and the leg fills the map whatever its length, which is what
    /// answers how far.
    #[test]
    fn the_leg_fills_the_map_whatever_its_length() {
        let door = door();
        let stops = stops(door);

        for (from, to) in [("Venus", "Earth"), ("Uranus", "Neptune")] {
            let one = stops.iter().position(|s| s.name == from).expect("a stop");
            let other = stops.iter().position(|s| s.name == to).expect("a stop");
            let at = |through: f32| {
                vec3(
                    door.x,
                    0.0,
                    stops[one].at.z + (stops[other].at.z - stops[one].at.z) * through,
                )
            };

            for want in [0.0, 0.25, 0.5, 1.0] {
                let map = mapped(&stops, one, at(want), Vec3::Z, door);
                let (said, next, through) = map.leg.expect("a leg");

                assert_eq!((said, next), (from, to));
                assert!(
                    (through - want).abs() < 1e-3,
                    "{} of the way from {} to {} read as {}",
                    want,
                    from,
                    to,
                    through
                );
            }
        }
    }

    /// Spec 0018: standing at a moon, the leg is its planet's, because the
    /// ride runs on the line and the moons stand off it.
    #[test]
    fn the_map_reads_a_moon_as_its_planet() {
        let door = door();
        let stops = stops(door);
        let io = stops.iter().position(|s| s.name == "Io").expect("a stop");
        let map = mapped(&stops, io, stops[io].at, Vec3::Z, door);

        assert_eq!(
            map.leg.map(|(from, to, _)| (from, to)),
            Some(("Jupiter", "Saturn"))
        );
    }

    /// Spec 0018: and there is no leg at the far end facing out, because there
    /// is nowhere to go.
    #[test]
    fn the_map_has_no_leg_past_the_end() {
        let door = door();
        let stops = stops(door);
        let neptune = stops
            .iter()
            .position(|s| s.name == "Neptune")
            .expect("a stop");
        let at = resting(&stops, neptune - 1, neptune, door);

        assert!(mapped(&stops, neptune, at, Vec3::Z, door).leg.is_none());
        assert_eq!(
            mapped(&stops, neptune, at, -Vec3::Z, door)
                .leg
                .map(|(from, to, _)| (from, to)),
            Some(("Neptune", "Uranus")),
            "turned round at Neptune the map does not show the way back"
        );
        // and the whole-line bar still says where you are
        assert!((mapped(&stops, neptune, at, Vec3::Z, door).along - 1.0).abs() < 1e-3);
    }

    /// Spec 0018: the posts stand up, and come at you often enough to read as
    /// speed.
    ///
    /// They were laid flat on the floor first and nothing was drawn at all: a
    /// mark on the floor seen from eye height is edge-on past twenty metres,
    /// and at ninety the three tenths of a metre of mark came to five
    /// thousandths of a metre of picture.
    #[test]
    fn the_posts_stand_up_and_come_often() {
        let marks = marked(door());
        let spine = marks[0];

        assert!(
            spine.size.z > 1000.0 && spine.size.y < 0.1,
            "the spine is {:?}, which is not a stripe down the line",
            spine.size
        );

        for mark in marks.iter().skip(1) {
            assert!(
                mark.size.y > crate::EYE * 0.5,
                "a post {:?} is under half eye height, so it is edge-on at any \
                 distance",
                mark.size
            );
            assert!(
                (mark.at.y - mark.size.y * 0.5).abs() < 1e-4,
                "a post at {:?} does not stand on the floor",
                mark.at
            );
        }

        let a_second = RIDES / POSTS;

        assert!(
            (1.0..=4.0).contains(&a_second),
            "{:.1} posts go by a second, which is not a rhythm",
            a_second
        );
        // and what you see when you are not riding: the next pair is close
        // enough to stand as a row rather than as two lights in the distance
        assert!(
            POSTS < au() * 0.25,
            "the posts are {} apart against a unit of {}",
            POSTS,
            au()
        );

        // and the big ones, one to the unit, with the Earth standing on the
        // first of them
        let earth = PLANETS
            .iter()
            .find(|planet| planet.body.name == "Earth")
            .expect("the Earth is in the tables");

        assert!(
            (far(earth) - au()).abs() < 1e-3,
            "the Earth is at {} and the unit is {}",
            far(earth),
            au()
        );
        assert!(
            marks.iter().filter(|mark| mark.glows).count() >= 60,
            "there are not thirty glowing pairs down the line"
        );
    }

    /// Spec 0018: the floor laps under the hidden room's rather than meeting
    /// it, for the reason spec 0017 gives.
    #[test]
    fn the_floor_laps_under_the_room() {
        let reaches = reaches();
        let slab = solid(reaches);
        let room = crate::behind::solid(reaches)
            .into_iter()
            .find(|box_| box_.max.y <= 0.0 && box_.min.y < 0.0)
            .expect("the hidden room has a floor");

        assert!(
            room.max.z - slab.min.z > 1.0,
            "the room's floor ends at {} and this one starts at {}, so there is \
             an end face at the seam you walk over",
            room.max.z,
            slab.min.z
        );
    }

    /// Spec 0018: the sun reaches Neptune with something left, which is the
    /// one thing bent out here.
    #[test]
    fn the_sun_still_reaches_neptune() {
        let lamp = blitzkit::lighting::PointLight::new(
            Vec3::ZERO,
            crate::orrery::SUNLIGHT,
            SUN_LIT,
            SUN_RANGE,
        );
        let out = far(neptune());

        assert!(
            lamp.falloff(out) > 0.4,
            "Neptune is {:.0} m out and keeps {:.3} of the light",
            out,
            lamp.falloff(out)
        );
        // and the inner planets are not blown past it: the spread is two,
        // where the truth is nine hundred
        let mercury = far(&PLANETS[0]);

        assert!(
            lamp.falloff(mercury) / lamp.falloff(out) < 3.0,
            "Mercury keeps {:.3} against Neptune's {:.3}",
            lamp.falloff(mercury),
            lamp.falloff(out)
        );
    }

    /// Spec 0018: and you can see it from there, which the building's own far
    /// plane does not allow.
    ///
    /// A hundred metres is further than any of the building goes and is what
    /// keeps the far wall of the hidden room a thing you find. Out here it
    /// culls the sun from everywhere past Mercury, and a sun you cannot see
    /// is the one thing this space is for.
    #[test]
    fn the_far_plane_reaches_the_sun_from_neptune() {
        let out = far(neptune());

        assert!(
            out > blitzkit::camera::Camera::new().far,
            "the building's far plane would have shown the sun from Neptune \
             anyway, at {:.0} m",
            out
        );
        assert!(
            sees() > out,
            "from Neptune the sun is {:.0} m off and the camera sees {:.0}",
            out,
            sees()
        );
    }
}
