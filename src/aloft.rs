//! The imaginarium, above everything else. Spec 0037.
//!
//! **Four flights round a square, each climbing, and the top of the fourth
//! at the foot of the first.** You go up all the way round and arrive where
//! you set off, and nothing about any single flight is odd.
//!
//! It is shallow on purpose and that is the whole trick. Three metres of
//! climb over thirty two of run is 5.4 degrees, which the eye reads as level
//! from anywhere in the room, so a loop that genuinely rises all the way
//! round looks closed. Twelve metres is 20.6 degrees and is unmistakably a
//! spiral going away from you: all three were built and looked at before
//! this one was picked.
//!
//! **It is not forced perspective.** The geometry is honest and holds up
//! from every angle, which spec 0037 required, because there is nothing to
//! hold up. The illusion is in the gentleness.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::THICK;

/// Where the room sits: over the hall, which is the only place in this
/// building with nothing above it.
///
/// Measured rather than chosen. Every room here is between y -4.5 and y 4.5,
/// and nothing at all is above that.
pub const MIDDLE_X: f32 = -2.75;
pub const MIDDLE_Z: f32 = -3.5;
pub const BASE: f32 = 6.0;

/// How big the room is inside, and how high.
///
/// Tall enough for the loop, for headroom over its top flight, and for the
/// thing above it that no flight reaches.
pub const ROOM: f32 = 12.5;
pub const TALL: f32 = 8.0;

/// The loop: the square its flights run round, how wide a flight is, and how
/// much of each side is stair rather than landing.
pub const SIDE: f32 = 8.0;
pub const WIDE: f32 = 1.8;
pub const RUN: f32 = 5.0;
pub const LANDING: f32 = SIDE - RUN;

/// How far one flight climbs, and the steps it does it in.
///
/// **A monumental stair, because the arithmetic leaves no choice.** Three
/// quarters of a metre over five is a gradient that cannot be cut into
/// ordinary steps: twenty of them would rise under four centimetres each,
/// which is a trip hazard rather than a stair. Five treads of a metre and
/// rises of fifteen centimetres is what that gradient actually is, and it is
/// a real form. Processional, which suits the room.
pub const CLIMB: f32 = 0.75;
pub const STEPS: u64 = 5;
pub const TREAD: f32 = RUN / STEPS as f32;
pub const RISE: f32 = CLIMB / STEPS as f32;

/// How far the whole loop climbs, which is where it has to drop back to.
#[cfg(test)]
pub fn round_trip() -> f32 {
    CLIMB * 4.0
}

/// Which way flight `n` runs, counted from nought, and which way is its left.
fn way(n: u64) -> (Vec3, Vec3) {
    match n % 4 {
        0 => (vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        1 => (vec3(0.0, 0.0, 1.0), vec3(-1.0, 0.0, 0.0)),
        2 => (vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0)),
        _ => (vec3(0.0, 0.0, -1.0), vec3(1.0, 0.0, 0.0)),
    }
}

/// The corner flight `n` sets off from.
/// How far the whole loop stands off the floor.
///
/// **One step, so that nothing of it is in the floor's plane.** The first
/// landing sat with its top face at exactly `BASE`, which is where the floor
/// is drawn, and two faces in one plane speckle as you move: it is the fault
/// this building has fought nine times and the first picture of this room
/// had it round the foot of the cone.
/// A step and a half, not a step: the slabs are a rise thick and anchored
/// at their tops, so lifting by exactly one rise put the bottom of the first
/// one back in the floor and cancelled out.
pub const OFF_THE_FLOOR: f32 = RISE * 1.5;

pub fn corner(n: u64) -> Vec3 {
    let half = SIDE * 0.5;
    let (x, z) = match n % 4 {
        0 => (-half, -half),
        1 => (half, -half),
        2 => (half, half),
        _ => (-half, half),
    };

    vec3(MIDDLE_X + x, BASE + OFF_THE_FLOOR, MIDDLE_Z + z)
}

/// Every tread of every flight, as the boxes they are drawn and walked as.
pub fn treads() -> Vec<Aabb> {
    let mut out = Vec::new();

    for flight in 0..4u64 {
        let (along, across) = way(flight);
        let foot = CLIMB * flight as f32;
        let from = corner(flight) + along * LANDING;

        for step in 0..STEPS {
            let up = foot + RISE * (step as f32 + 1.0);
            let at = from + along * (TREAD * step as f32) + vec3(0.0, up, 0.0);
            let far = at + along * TREAD + across * WIDE;

            out.push(Aabb::new(
                vec3(at.x.min(far.x), at.y - RISE, at.z.min(far.z)),
                vec3(at.x.max(far.x), at.y, at.z.max(far.z)),
            ));
        }
    }

    out
}

/// The four landings, each at the top of one flight and the foot of the next.
pub fn landings() -> Vec<Aabb> {
    (0..4u64)
        .map(|n| {
            let (along, across) = way(n);
            let up = BASE + OFF_THE_FLOOR + CLIMB * n as f32;
            let at = corner(n) + vec3(0.0, CLIMB * n as f32, 0.0);
            let far = at + along * LANDING + across * WIDE;

            Aabb::new(
                vec3(at.x.min(far.x), up - RISE, at.z.min(far.z)),
                vec3(at.x.max(far.x), up, at.z.max(far.z)),
            )
        })
        .collect()
}

/// Where the thing on landing `n` stands, which is what tells the landings
/// apart.
///
/// **Spec 0021 learned this the hard way and wrote it down.** Eight rooms
/// alike in everything but a word on a wall, parked the day they were built
/// because nobody could tell where they were, and three rewrites of a map
/// failed before anybody saw why: a map can only point at places that
/// already mean something. Four flights that look alike is that room again.
pub fn on_the_landing(n: u64) -> Vec3 {
    let (along, across) = way(n);
    let at = corner(n) + vec3(0.0, CLIMB * n as f32, 0.0);

    at + along * (LANDING * 0.5) + across * (WIDE * 0.5) + vec3(0.0, 0.0, 0.0)
}

/// The four things on the four landings, as profiles for the wheel.
///
/// **One each, and no two alike**, which is spec 0021's whole lesson. A
/// colour would not do it: that spec tried telling eight rooms apart by a
/// word and a colour and parked the room over it.
pub fn standing_on(n: u64) -> fn(f32) -> (f32, f32) {
    match n % 4 {
        // a cone, point up
        0 => |v: f32| (v * 0.9, 0.55 * (1.0 - v)),
        // a sphere
        1 => |v: f32| {
            (
                v * 0.8,
                0.8 * (1.0 - (2.0 * v - 1.0).powi(2)).max(0.0).sqrt(),
            )
        },
        // a spindle, widest in the middle and closed at both ends
        2 => |v: f32| (v * 1.1, 0.5 * (1.0 - (2.0 * v - 1.0).abs())),
        // and a bowl, open at the top
        _ => |v: f32| (v * 0.6, 0.18 + 0.52 * v),
    }
}

/// How high the fittings hang, how far they reach and how hard they burn.
///
/// **The only room here with daylight in it**, or the nearest this building
/// gets: everything else is a cellar, a corridor at night or a hall lit in
/// pools, and this is the one room with nothing above it. The reach is long
/// so the whole loop is visible at once, because a loop you can only see a
/// quarter of is not a loop.
pub const LAMP_UP: f32 = TALL - 1.2;
pub const LAMP_RANGE: f32 = 11.0;
pub const LAMP_LIT: f32 = 0.9;

pub fn lamps() -> Vec<Vec3> {
    (0..4u64)
        .map(|n| {
            let at = on_the_landing(n);

            vec3(at.x, BASE + LAMP_UP, at.z)
        })
        .collect()
}

// ---- the way up ----

/// How wide a flight is, how deep one tread, how many treads to a flight and
/// how many flights there are. Spec 0037.
///
/// **A dog-leg, because every collider in this building is a box.** A spiral
/// was drawn first: a wedge-shaped tread's bounding box is most of a
/// cylinder, the boxes overlap near the newel, and the walking surface there
/// is whichever of them is highest. Spec 0007 met the same thing from the
/// other end, where nested treads gave a flight four legal rises and no way
/// out of it. Two straight runs at right angles is what a box describes
/// exactly.
pub const UP_WIDE: f32 = 1.2;
pub const UP_GOING: f32 = 0.28;
pub const UP_STEPS: u64 = 16;
pub const UP_FLIGHTS: u64 = 2;

/// And what falls out of those: one tread's rise, one flight's run, and the
/// height of the landing between them.
pub const UP_RISE: f32 = BASE / (UP_FLIGHTS * UP_STEPS) as f32;
pub const UP_RUN: f32 = UP_GOING * UP_STEPS as f32;
pub const UP_HALFWAY: f32 = BASE / UP_FLIGHTS as f32;

/// How much room a flight leaves over your head before the thing above it
/// has to be opened.
///
/// Taller than you are, with something to spare. A soffit at exactly your
/// height clears you by nothing and reads as a lid.
pub const UP_HEADROOM: f32 = 2.0;

/// The nook's two inner faces, west and east.
///
/// Taken from the cellar's own back wall and the hall's own width rather
/// than written out, so the stair moves when the nook does.
fn nook() -> (f32, f32) {
    (
        crate::cellar::back(),
        -(crate::room::WALL + crate::room::CABINET.x) - THICK * 0.5,
    )
}

/// The west side of the first flight, which stands free in the middle of the
/// nook.
///
/// **Not against the wall of books**, which was the first build and was
/// wrong twice over. Flush with the fronts of the cases it put two exposed
/// faces in one plane, which is the fault this building has fought nine
/// times, and standing it off by a reveal instead left four of the twelve
/// cases behind a staircase.
///
/// In the middle there is a lane either side: 2.13 to the books and 1.53 to
/// the benches, against the 1.05 a body needs. It is also the only placing
/// that leaves every shelf in the room reachable, including the one with
/// spec 0007's door in it.
pub fn up_west() -> f32 {
    let (west, east) = nook();

    (west + east) * 0.5 - UP_WIDE * 0.5
}

/// The south edge of the landing, which is the one number the rest of the
/// stair hangs off.
///
/// **Measured back from this room's north wall, not chosen.** The landing
/// and the second flight sit in the north lane, between the loop and the
/// wall, and that lane is `(ROOM - SIDE) / 2`, which is 2.25. A flight is
/// 1.2 of it. What is left is 1.05, which is exactly the width this project
/// has settled on for anywhere a body has to pass: 0.9 across with a tenth
/// either side was tried in spec 0007 and thrown out.
pub fn up_head() -> f32 {
    MIDDLE_Z + ROOM * 0.5 - UP_WIDE
}

/// The foot of the first flight, down in the nook.
pub fn up_foot() -> f32 {
    up_head() - UP_RUN
}

/// Where the second flight leaves off, which is where you step into the room.
pub fn up_arrival() -> f32 {
    up_west() + UP_WIDE + UP_RUN
}

/// Every tread of both flights, and the landing they turn on.
///
/// The first flight's treads run from their own top all the way under the
/// nook's floor, which is what spec 0007 settled on for the stair down: one
/// box fewer to fall between, and anyway what the underside of a stair on
/// the ground looks like. The second flight is in the air, so its treads are
/// a raking slab `THICK` deep instead, which is thicker than a rise and so
/// leaves no two of them sharing a face.
pub fn up_treads() -> Vec<Aabb> {
    let (west, head, foot) = (up_west(), up_head(), up_foot());
    let mut out = Vec::new();

    // the first flight, against the books, climbing north
    for step in 0..UP_STEPS {
        let top = UP_RISE * (step as f32 + 1.0);
        let at = foot + UP_GOING * step as f32;

        out.push(Aabb::new(
            vec3(west, -THICK, at),
            vec3(west + UP_WIDE, top, at + UP_GOING),
        ));
    }

    // the landing. It reaches a tread back under the last of them rather
    // than meeting it edge on: two walking surfaces at one height that meet
    // in a plane stop you dead, and `walk` reports nothing at all.
    out.push(Aabb::new(
        vec3(west, -THICK, head - UP_GOING),
        vec3(west + UP_WIDE, UP_HALFWAY, head + UP_WIDE),
    ));

    // the second flight, over the nook's ceiling, climbing east
    for step in 0..UP_STEPS {
        let top = UP_HALFWAY + UP_RISE * (step as f32 + 1.0);
        let at = west + UP_WIDE + UP_GOING * step as f32;

        out.push(Aabb::new(
            vec3(at, top - UP_RISE - THICK, head),
            vec3(at + UP_GOING, top, head + UP_WIDE),
        ));
    }

    out
}

/// The slot the second flight comes up through, in this room's own floor.
///
/// **Exactly the width of the flight**, so the stair fills it and there is
/// nothing beside it to fall down. A stairwell wide enough to walk into
/// wants a balustrade round it, and a hole you can fall four metres down is
/// worse than no way up at all.
///
/// It opens where the flight would otherwise have less than [`UP_HEADROOM`]
/// under the floor, and not before: the first treads pass under the slab
/// with room to spare and there is no reason to cut it there.
pub fn up_hole() -> Aabb {
    let head = up_head();
    let under = BASE - THICK;
    let steps = ((under - UP_HEADROOM - UP_HALFWAY) / UP_RISE)
        .floor()
        .max(0.0);

    Aabb::new(
        vec3(up_west() + UP_WIDE + UP_GOING * steps, BASE - THICK, head),
        vec3(up_arrival(), BASE, head + UP_WIDE),
    )
}

/// What comes out of the nook's suspended ceiling.
///
/// An L: the length of the first flight from where your head would be in
/// the tiles, plus the landing, plus the whole of the second flight, which
/// is above the tiles from its first tread.
///
/// **Cut to the stair exactly, and the tiles clipped to it.** Snapping the
/// well out to whole tiles was tried first, because spec 0012's ceiling is
/// bars on the world's own lines and a hole cut between them leaves part
/// tiles. It put the shaft wall up to seven tenths clear of the landing it
/// is meant to meet, which is a gap at the top of a flight with three
/// metres under it. The tiles are clipped instead, which `room::minus`
/// already did for this room's floor.
pub fn up_well() -> Vec<Aabb> {
    let (west, head) = (up_west(), up_head());
    let plan = up_plan();

    // **Only as far along the second flight as it is still in the ceiling.**
    // Opened along the whole flight, which was the first build, the hole ran
    // out over the hall's aisle and put a white box and a black hole in the
    // one ceiling spec 0012 made worth looking up at. By then the stair is
    // two metres over the tiles and wants no hole at all: it walks over an
    // intact ceiling the rest of the way.
    let crosses = up_treads()
        .iter()
        .filter(|t| t.max.y > UP_HALFWAY + 1e-4 && t.min.y < crate::room::TALL)
        .map(|t| t.max.x)
        .fold(west + UP_WIDE, f32::max);

    vec![
        plan[0],
        Aabb::new(
            vec3(west + UP_WIDE, plan[1].min.y, head),
            vec3(crosses, plan[1].max.y, head + UP_WIDE),
        ),
    ]
}

/// The stair's own plan, as the two rectangles the shaft is built round.
///
/// The first is the top of the first flight and the landing, from where
/// your head would be in the tiles. The second is the whole of the second
/// flight, all the way to where it arrives.
fn up_plan() -> Vec<Aabb> {
    let (west, head, foot) = (up_west(), up_head(), up_foot());
    let (under, over) = (crate::room::soffit(), BASE - THICK);

    // where your head first reaches the tiles on the way up
    let steps = ((crate::room::soffit() - crate::people::TALL) / UP_RISE)
        .floor()
        .max(0.0);

    vec![
        Aabb::new(
            vec3(west, under, foot + UP_GOING * steps),
            vec3(west + UP_WIDE, over, head + UP_WIDE),
        ),
        Aabb::new(
            vec3(west + UP_WIDE, under, head),
            vec3(up_arrival(), over, head + UP_WIDE),
        ),
    ]
}

/// The shaft the stair climbs between the nook's ceiling and this room's
/// floor, so that what you climb through is a wall and not the gap between
/// two storeys.
///
/// **Round the whole stair, not round the hole in the ceiling.** Those are
/// two different shapes and the first build used one for both: the shaft
/// stopped where the hole did, so the part of the flight that is over the
/// hall's ceiling climbed through an open void with the backs of things in
/// it, and the shaft's own underside hung below the tiles as a white beam
/// across the hall. It starts at the tiles' own underside now, so there is
/// nothing of it under them.
pub fn up_shaft() -> Vec<Aabb> {
    let wells = up_plan();
    let mut out = Vec::new();

    for (n, well) in wells.iter().enumerate() {
        let (low, high) = (well.min, well.max);
        let ring = [
            Aabb::new(
                vec3(low.x - THICK, low.y, low.z - THICK),
                vec3(low.x, high.y, high.z + THICK),
            ),
            Aabb::new(
                vec3(high.x, low.y, low.z - THICK),
                vec3(high.x + THICK, high.y, high.z + THICK),
            ),
            Aabb::new(
                vec3(low.x, low.y, low.z - THICK),
                vec3(high.x, high.y, low.z),
            ),
            Aabb::new(
                vec3(low.x, low.y, high.z),
                vec3(high.x, high.y, high.z + THICK),
            ),
        ];

        // **A ring round each piece of the well is not the wall round the
        // well.** The two pieces make an L, and the ring round the smaller
        // one runs straight across the larger: the first build put a wall
        // down the middle of the first flight, twelve treads up, and the
        // walk test found it where measuring would not have. So take the
        // other pieces back out of each ring.
        for wall in ring {
            let mut pieces = vec![wall];

            for (m, other) in wells.iter().enumerate() {
                if m == n {
                    continue;
                }
                pieces = pieces
                    .into_iter()
                    .flat_map(|p| crate::room::minus(p, *other))
                    .collect();
            }

            out.extend(pieces);
        }
    }

    out
}

/// How high a lamp on the way up sits over the tread beside it, how far it
/// reaches, and how bright it is.
///
/// Dimmer and shorter than the room's own four. A stairwell lit like a hall
/// is a hall, and what you want on the way up is to see the next tread.
pub const UP_LAMP_UP: f32 = 1.9;
pub const UP_LAMP_RANGE: f32 = 5.0;
pub const UP_LAMP_LIT: f32 = 0.5;

/// The fittings on the way up: along the shaft's west wall beside the first
/// flight, and along its north wall beside the landing and the second.
///
/// **Derived from the flights, which is the rule this building keeps
/// breaking.** The baths put their lamps along a ceiling and their sconces
/// on a wall, so every sconce was dark and the ceiling glowed at nothing.
/// Here the lamps *are* the fittings, lifted, so there cannot be one
/// without the other.
///
/// Thinned at the top of the second flight, where there is no shaft left
/// over it: the last treads come up through the slot and are lit by the
/// room they arrive in.
pub fn up_sconces() -> Vec<Vec3> {
    let (west, head, foot) = (up_west(), up_head(), up_foot());
    let mut out = Vec::new();
    let every = 5;

    for step in (0..UP_STEPS).step_by(every) {
        let step = step as f32;
        out.push(vec3(
            west,
            UP_RISE * step + UP_LAMP_UP,
            foot + UP_GOING * (step + 0.5),
        ));
    }

    for step in (0..UP_STEPS).step_by(every) {
        let step = step as f32;
        out.push(vec3(
            west + UP_WIDE + UP_GOING * (step + 0.5),
            UP_HALFWAY + UP_RISE * step + UP_LAMP_UP,
            head + UP_WIDE,
        ));
    }

    // **Only where there is a wall to put one in.** The first flight
    // stands free in the middle of the nook, so under the ceiling there is
    // nothing beside it to hang a fitting on, and the nook's own lamps
    // light that part anyway. Above the ceiling the shaft is the wall.
    let shaft = up_shaft();
    out.retain(|at| {
        shaft
            .iter()
            .any(|wall| at.y > wall.min.y && at.y < wall.max.y - 0.15)
    });
    out
}

/// And the lamps, which are those.
pub fn up_lamps() -> Vec<Vec3> {
    up_sconces()
}

/// The way up as one list: the stair and the shaft round it.
///
/// Named once because spec 0026 keeps it, `fillable` leaves it out, and the
/// drawing walks it. Three places saying what the way up is made of is
/// three chances for two of them to be right.
pub fn way_up() -> Vec<Aabb> {
    let mut out = up_treads();
    out.extend(up_shaft());
    out
}

/// Everything solid: the shell, the treads and the landings, and the way up.
pub fn solid() -> Vec<Aabb> {
    let half = ROOM * 0.5;
    let low = vec3(MIDDLE_X - half, BASE, MIDDLE_Z - half);
    let high = vec3(MIDDLE_X + half, BASE + TALL, MIDDLE_Z + half);
    // the floor you stand on in the middle, with the stair's slot out of it
    let mut out = crate::room::minus(
        Aabb::new(
            vec3(low.x - THICK, BASE - THICK, low.z - THICK),
            vec3(high.x + THICK, BASE, high.z + THICK),
        ),
        up_hole(),
    );

    // and the lid over the lot
    out.extend([
        Aabb::new(
            vec3(low.x - THICK, high.y, low.z - THICK),
            vec3(high.x + THICK, high.y + THICK, high.z + THICK),
        ),
        // and four walls
        Aabb::new(
            vec3(low.x - THICK, BASE, low.z - THICK),
            vec3(low.x, high.y, high.z + THICK),
        ),
        Aabb::new(
            vec3(high.x, BASE, low.z - THICK),
            vec3(high.x + THICK, high.y, high.z + THICK),
        ),
        Aabb::new(
            vec3(low.x, BASE, low.z - THICK),
            vec3(high.x, high.y, low.z),
        ),
        Aabb::new(
            vec3(low.x, BASE, high.z),
            vec3(high.x, high.y, high.z + THICK),
        ),
    ]);

    out.extend(treads());
    out.extend(landings());
    out.extend(way_up());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0037: the loop is shallow enough to read as closed.
    ///
    /// **This is the room.** Three of these were built and looked at: a
    /// twelve metre climb is 20.6 degrees and is plainly a spiral going away
    /// from you, four separate flights from the floor are an X of ramps, and
    /// this one reads as a square walkway. Lose the gradient and the room
    /// stops working.
    #[test]
    fn it_is_shallow_enough_to_read_as_closed() {
        // **Over the whole loop, not over the stairs.** The eye integrates
        // what it can see, which is the perimeter: rise over the stairs
        // alone ignores the landings and comes out at 8.5 degrees, and that
        // was not the number the picture was judged on.
        let around = SIDE * 4.0;
        let degrees = (round_trip() / around).atan().to_degrees();

        assert!(
            degrees < 7.0,
            "the loop climbs at {:.1} degrees over its own length, which reads as a spiral",
            degrees
        );
        assert!(
            degrees > 2.0,
            "the loop climbs at {:.1} degrees, which is a floor",
            degrees
        );

        // and no single flight is steep enough to give it away either
        let flight = (CLIMB / RUN).atan().to_degrees();

        assert!(
            flight < 12.0,
            "a flight climbs at {:.1} degrees, which is a staircase and not a gradient",
            flight
        );
    }

    /// And the steps are the gradient rather than a guess at one.
    ///
    /// A monumental stair, because that gradient cannot be cut into ordinary
    /// steps: twenty treads would rise under four centimetres each.
    #[test]
    fn the_steps_are_what_that_gradient_is() {
        // read off the treads that are built, because an assertion on the
        // constants they are built from has a constant value and proves
        // only that I can type
        let steps = treads();
        let first = steps[0];
        let second = steps[1];
        let rise = second.max.y - first.max.y;
        let tread = (second.center() - first.center()).length();

        assert_eq!(steps.len() as u64, STEPS * 4, "a flight is missing steps");
        assert!(
            rise > 0.1 && rise < 0.2,
            "a rise of {:.3} is not a step anybody takes",
            rise
        );
        assert!(
            tread > rise * 4.0,
            "a tread of {:.2} on a rise of {:.2} is an ordinary stair, not this gradient",
            tread,
            rise
        );

        // and a flight's steps add up to its climb
        let flight = steps[(STEPS - 1) as usize].max.y - first.max.y + rise;

        assert!(
            (flight - CLIMB).abs() < 1e-4,
            "a flight climbs {:.3} where its steps add up to {:.3}",
            CLIMB,
            flight
        );
    }

    /// Spec 0037: the loop closes, and the join is small enough to hide.
    #[test]
    fn the_fourth_flight_comes_back_to_the_first() {
        let first = landings()[0];
        let last = treads().last().copied().expect("a last tread");

        // round the square and back over the same corner
        assert!(
            (last.max.x - first.min.x).abs() < SIDE,
            "the loop does not come back over its own start"
        );

        // and the drop at the join is a step rather than a storey
        assert!(
            round_trip() < 3.5,
            "the join is a {:.1} metre drop, which is nowhere to put",
            round_trip()
        );
    }

    /// Nothing of the loop lies in the plane of the floor.
    ///
    /// Two faces in one plane speckle as you move, and this building has
    /// fought that nine times. The first landing had its top face at exactly
    /// the floor's height and the first picture of the room showed it round
    /// the foot of the cone.
    #[test]
    fn nothing_of_it_is_in_the_floors_plane() {
        for one in treads().into_iter().chain(landings()) {
            assert!(
                one.min.y > BASE,
                "a piece of the loop reaches the floor at {:.3}",
                one.min.y
            );
        }
    }

    /// Spec 0021's lesson: the landings have to be tellable apart.
    #[test]
    fn no_two_landings_are_in_the_same_place() {
        let mut seen: Vec<Vec3> = Vec::new();

        for n in 0..4u64 {
            let at = on_the_landing(n);

            for was in seen.iter() {
                assert!(
                    (at - *was).length() > 1.0,
                    "two landings are {:.2} apart",
                    (at - *was).length()
                );
            }

            seen.push(at);
        }

        // each is a step higher than the last, which is the only thing they
        // share and the thing that makes the climb real
        for n in 1..4u64 {
            assert!(
                on_the_landing(n).y > on_the_landing(n - 1).y,
                "landing {} is not above landing {}",
                n,
                n - 1
            );
        }
    }

    /// And it is above everything else in the building.
    #[test]
    fn nothing_in_the_building_is_up_here() {
        let room = crate::room::Room::of(crate::games());
        let mine = Aabb::new(
            vec3(MIDDLE_X - ROOM, BASE - THICK, MIDDLE_Z - ROOM),
            vec3(MIDDLE_X + ROOM, BASE + TALL + THICK, MIDDLE_Z + ROOM),
        );

        for one in room.solid() {
            if solid().contains(&one) {
                continue;
            }

            assert!(
                !one.intersects(&mine),
                "something is already up here: {:?} to {:?}",
                one.min,
                one.max
            );
        }
    }
    /// Spec 0037: the way up leaves the nook's floor, meets its own landing
    /// and arrives on the imaginarium's, with no step out of line at any of
    /// the three.
    ///
    /// **Topping out at `BASE` is not the claim, because it cannot fail.**
    /// The first of these asserted only that, and `UP_RISE` is `BASE` over
    /// the treads: halve the treads and the rise doubles and the stair
    /// still ends exactly where it did. The control proved nothing, which
    /// is how the test was found. What can go wrong is the joints, so the
    /// joints are what this reads.
    #[test]
    fn the_stair_runs_from_one_floor_to_the_other() {
        let treads = up_treads();
        let rise = |of: &Aabb| of.max.y;

        let flight = |above: bool| {
            let mut out: Vec<&Aabb> = treads
                .iter()
                .filter(|t| (rise(t) > UP_HALFWAY + 1e-4) == above)
                .filter(|t| t.size().z <= UP_GOING + 1e-4 || t.size().x <= UP_GOING + 1e-4)
                .collect();
            out.sort_by(|a, b| rise(a).partial_cmp(&rise(b)).unwrap());
            out
        };

        let (first, second) = (flight(false), flight(true));
        let landing = treads
            .iter()
            .find(|t| t.size().z > UP_GOING + 1e-4 && t.size().x > UP_GOING + 1e-4)
            .expect("the stair has no landing to turn on");

        assert!(
            (rise(first[0]) - UP_RISE).abs() < 1e-4,
            "the first tread tops out at {:.4}, which is not one rise off the nook's floor",
            rise(first[0])
        );
        assert!(
            (rise(first[first.len() - 1]) - rise(landing)).abs() < 1e-4,
            "the first flight ends at {:.4} and its landing is at {:.4}",
            rise(first[first.len() - 1]),
            rise(landing)
        );
        assert!(
            (rise(second[0]) - rise(landing) - UP_RISE).abs() < 1e-4,
            "the second flight starts {:.4} above its landing and a rise is {:.4}",
            rise(second[0]) - rise(landing),
            UP_RISE
        );
        assert!(
            (rise(second[second.len() - 1]) - BASE).abs() < 1e-4,
            "the stair tops out at {:.4} and the floor it arrives on is at {:.4}",
            rise(second[second.len() - 1]),
            BASE
        );
    }

    /// Spec 0014: the landing laps the flight rather than meeting it.
    ///
    /// Two walking surfaces at one height that meet edge on stop you dead,
    /// and `walk` reports it as nothing at all: this building has found it
    /// three times in one day and once more here, where moving the landing
    /// by a tread broke it and every other test still passed.
    #[test]
    fn the_landing_laps_the_flight() {
        let treads = up_treads();
        let landing = treads
            .iter()
            .find(|t| t.size().z > UP_GOING + 1e-4 && t.size().x > UP_GOING + 1e-4)
            .expect("the stair has no landing to turn on");

        let last = treads
            .iter()
            .filter(|t| t.size().z <= UP_GOING + 1e-4 && t.max.y <= UP_HALFWAY + 1e-4)
            .max_by(|a, b| a.max.y.partial_cmp(&b.max.y).unwrap())
            .expect("the first flight has no treads");

        let lap = last.max.z.min(landing.max.z) - last.min.z.max(landing.min.z);

        assert!(
            lap > 1e-3,
            "the landing and the last tread lap by {:.4}, so they meet in a plane",
            lap
        );
        assert!(
            (last.max.y - landing.max.y).abs() < 1e-4,
            "they lap at different heights: {:.4} against {:.4}",
            last.max.y,
            landing.max.y
        );
    }

    /// Spec 0037: every rise on it is one you can actually climb.
    ///
    /// Taken as the gap between one tread's top and the next along each
    /// flight, which is the thing `walk` is asked to get over, rather than
    /// as `UP_RISE`, which is only the number it was meant to be.
    #[test]
    fn every_rise_is_one_you_can_climb() {
        let treads = up_treads();

        // the landing tops out level with the last tread of the first
        // flight, on purpose, so it is not one of them. Told apart by its
        // depth rather than its height.
        let mut first: Vec<&Aabb> = treads
            .iter()
            .filter(|t| t.max.y <= UP_HALFWAY + 1e-4 && t.size().z <= UP_GOING + 1e-4)
            .collect();
        first.sort_by(|a, b| a.min.z.partial_cmp(&b.min.z).unwrap());

        let mut second: Vec<&Aabb> = treads
            .iter()
            .filter(|t| t.max.y > UP_HALFWAY + 1e-4)
            .collect();
        second.sort_by(|a, b| a.min.x.partial_cmp(&b.min.x).unwrap());

        for flight in [first, second] {
            for (below, above) in flight.iter().zip(flight.iter().skip(1)) {
                let rise = above.max.y - below.max.y;
                assert!(
                    rise > 0.0 && rise <= crate::walk::STEP,
                    "a rise of {:.4} is not one you can get up",
                    rise
                );
            }
        }
    }

    /// Spec 0037: the stair does not stand in the swing of spec 0007's door.
    ///
    /// The way down is a pair of bookcases hinged at their outer ends, and a
    /// stair standing in front of them is a door that cannot open. Taken
    /// from the leaves wide open, which is where they get to, rather than
    /// from a clearance worked out by hand: the hand-worked number was 0.57
    /// and said nothing about whether the leaves reach the stair, because a
    /// leaf swings towards its own hinge and not past it.
    #[test]
    fn the_stair_is_not_in_the_swing_of_the_door() {
        let room = crate::room::Room::of(crate::games());
        let treads = up_treads();

        for (n, case) in room.bookcases.iter().enumerate() {
            if !crate::cellar::swings(n) {
                continue;
            }

            let out = room.swings_out(n);

            for swing in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let leaf = crate::room::Room::swung_box(case, out, swing);

                for tread in treads.iter() {
                    assert!(
                        !leaf.intersects(tread),
                        "case {} {:.0} per cent open reaches the stair at {:?}",
                        n,
                        swing * 100.0,
                        tread.min
                    );
                }
            }
        }

        // and you can still stand at the shelf to work it
        let standing = Aabb::new(
            vec3(
                up_west() + 1e-3,
                0.0,
                crate::cellar::opening(room.reaches) - crate::RADIUS,
            ),
            vec3(
                up_west() + crate::RADIUS * 2.0,
                crate::people::TALL,
                crate::cellar::opening(room.reaches) + crate::RADIUS,
            ),
        );

        for tread in treads.iter() {
            assert!(
                !standing.intersects(tread),
                "the stair stands where you have to stand to reach the book: {:?}",
                tread.min
            );
        }
    }

    /// Spec 0037: you can get past the second flight to the rest of the room.
    ///
    /// The landing and the second flight stand in the north lane, which is
    /// what is left between the loop and the wall. If the flight takes too
    /// much of it, arriving in this room means arriving in a dead end.
    #[test]
    fn you_can_get_past_the_second_flight() {
        let lane = up_head() - (MIDDLE_Z + SIDE * 0.5);

        assert!(
            lane >= 1.05,
            "the lane past the second flight is {:.3}, and a body is 0.9",
            lane
        );
    }

    /// Spec 0037: the slot in the floor is exactly the flight wide, so there
    /// is nothing beside the stair to fall down.
    #[test]
    fn the_slot_is_filled_by_the_stair() {
        let hole = up_hole();
        let treads = up_treads();

        assert!(
            (hole.size().z - UP_WIDE).abs() < 1e-4,
            "the slot is {:.3} across and the flight is {:.3}",
            hole.size().z,
            UP_WIDE
        );

        // every slice of it along the way out has a tread under it
        let mut at = hole.min.x + 1e-3;
        while at < hole.max.x {
            assert!(
                treads
                    .iter()
                    .any(|t| t.min.x <= at && t.max.x >= at && t.max.y > BASE - 2.5),
                "the slot at x {:.3} has no tread in it",
                at
            );
            at += UP_GOING * 0.5;
        }
    }

    /// Spec 0037: where the floor is not open, the flight passes under it
    /// with room over your head.
    #[test]
    fn the_second_flight_clears_the_floor_it_comes_up_through() {
        let hole = up_hole();
        let under = BASE - THICK;

        for tread in up_treads().iter().filter(|t| t.max.y > UP_HALFWAY + 1e-4) {
            if tread.min.x >= hole.min.x - 1e-4 {
                continue;
            }

            let over = under - tread.max.y;
            assert!(
                over >= UP_HEADROOM - 1e-4,
                "a tread at x {:.2} has {:.3} over it and the floor is not open there",
                tread.min.x,
                over
            );
        }
    }

    /// Spec 0037: the stairwell does not cut the loop.
    ///
    /// The slot is in the north lane and the loop is a square inside it. One
    /// box of the two overlapping would be a hole in a flight.
    #[test]
    fn the_stairwell_does_not_cut_the_loop() {
        let hole = up_hole();

        for (n, box_) in treads().iter().chain(landings().iter()).enumerate() {
            assert!(
                !hole.intersects(box_),
                "the slot cuts box {} of the loop: {:?}",
                n,
                box_
            );
        }
    }

    /// Spec 0037: all of it is under this room, and none of it is outside.
    ///
    /// A stair arriving through a floor that is not there arrives nowhere,
    /// and `fillable` in spec 0026 is bounded by this room's own slab.
    #[test]
    fn the_whole_stair_is_under_the_room() {
        let half = ROOM * 0.5;
        let (low, high) = (
            vec3(MIDDLE_X - half, 0.0, MIDDLE_Z - half),
            vec3(MIDDLE_X + half, 0.0, MIDDLE_Z + half),
        );

        for tread in up_treads() {
            assert!(
                tread.min.x >= low.x - 1e-3
                    && tread.max.x <= high.x + 1e-3
                    && tread.min.z >= low.z - 1e-3
                    && tread.max.z <= high.z + 1e-3,
                "a tread at {:?} is outside the room above it",
                tread.min
            );
        }
    }

    /// Spec 0012: the way up takes nothing out of the hall's ceiling and
    /// hangs nothing under it.
    ///
    /// **The fault this caught was in a filmed clip and not in a test.**
    /// The hole in the ceiling was cut along the whole of the second
    /// flight, which runs out over the hall's aisle, and the shaft's wall
    /// was built on the hole and started a tile's drop below the tiles. The
    /// first frame of the arcade's own clip had a white beam across the one
    /// ceiling spec 0012 made worth looking up at, with a black hole behind
    /// it. By the time the flight is over the hall it is two metres above
    /// the tiles and wants no hole at all.
    #[test]
    fn the_way_up_leaves_the_halls_ceiling_alone() {
        let nook_east = -(crate::room::WALL + crate::room::CABINET.x) - THICK * 0.5;

        for (n, well) in up_well().iter().enumerate() {
            assert!(
                well.max.x <= nook_east + 1e-3,
                "piece {} of the hole in the ceiling reaches x {:.2}, past the nook at {:.2}",
                n,
                well.max.x,
                nook_east
            );
        }

        for (n, wall) in up_shaft().iter().enumerate() {
            assert!(
                wall.min.y >= crate::room::soffit() - 1e-4,
                "piece {} of the shaft starts at {:.4}, under the tiles at {:.4}",
                n,
                wall.min.y,
                crate::room::soffit()
            );
        }
    }

    /// Spec 0037: the shaft goes round the whole stair and not round the
    /// hole in the ceiling, which is a different shape.
    #[test]
    fn the_shaft_encloses_the_whole_second_flight() {
        let shaft = up_shaft();

        for tread in up_treads().iter().filter(|t| t.max.y > UP_HALFWAY + 1e-4) {
            let beside = Aabb::new(
                vec3(tread.min.x, crate::room::soffit() + 0.1, tread.min.z - 0.2),
                vec3(tread.max.x, BASE - THICK - 0.1, tread.max.z + 0.2),
            );

            assert!(
                shaft.iter().any(|wall| wall.intersects(&beside)),
                "the flight at x {:.2} climbs with no shaft round it",
                tread.min.x
            );
        }
    }

    /// Spec 0037: every lamp on the way up is in a fitting, in the shaft,
    /// and not inside the stair.
    ///
    /// The rule this building keeps breaking. The baths ran their lamps
    /// along a ceiling and stood their sconces on a wall, so every sconce
    /// was dark and the ceiling glowed at nothing.
    #[test]
    fn every_lamp_on_the_way_up_is_in_a_fitting() {
        let (sconces, lamps) = (up_sconces(), up_lamps());
        let shaft = up_shaft();
        let treads = up_treads();

        assert!(!lamps.is_empty(), "the way up is climbed in the dark");
        assert_eq!(sconces, lamps, "a lamp that is not in a fitting");

        for at in lamps {
            let bulb = Aabb::new(at - Vec3::splat(0.08), at + Vec3::splat(0.08));

            assert!(
                shaft.iter().any(|wall| wall.intersects(&bulb)),
                "the lamp at {:?} is in no wall of the shaft",
                at
            );
            assert!(
                !treads.iter().any(|tread| tread.intersects(&bulb)),
                "the lamp at {:?} is inside the stair",
                at
            );
        }
    }

    /// Spec 0037: somebody your size, walking, gets from the nook to the
    /// room.
    ///
    /// **Walked rather than measured**, which is what spec 0007 settled on
    /// after a flight of four legal rises turned out to be one solid block.
    /// Every box here is legal on its own and the question is whether the
    /// set of them is a stair.
    #[test]
    fn you_can_walk_up_it() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();

        let start = vec3(up_west() + UP_WIDE * 0.5, 1.0, up_foot() - 0.6);
        let landed = climbed(&solid, start);

        assert!(
            (landed.y - BASE).abs() < UP_RISE * 2.0,
            "walking up it ended at y {:.3} rather than on the floor at {:.3}",
            landed.y,
            BASE
        );
        assert!(
            landed.x > up_arrival() - 0.3,
            "walking up it stopped at x {:.3}, short of the top at {:.3}",
            landed.x,
            up_arrival()
        );

        // and the negative control, which is this building before today:
        // take the treads out and the same walk never leaves the nook.
        let treads = up_treads();
        let without: Vec<Aabb> = solid
            .iter()
            .filter(|b| !treads.iter().any(|t| t.min == b.min && t.max == b.max))
            .copied()
            .collect();
        let stuck = climbed(&without, start);

        assert!(
            stuck.y < UP_RISE * 2.0,
            "with the treads taken out the same walk still reached y {:.3}",
            stuck.y
        );
    }

    /// Walks a body your size north up the first flight, then east up the
    /// second, and says where it got to.
    fn climbed(solid: &[Aabb], from: Vec3) -> Vec3 {
        let (mut at, mut falling) = (from, 0.0);

        for wish in [Vec3::Z, Vec3::X] {
            for _ in 0..420 {
                let (next, fell) = crate::walk::walk(
                    at,
                    wish * crate::SPEED,
                    falling,
                    crate::RADIUS,
                    1.0 / 60.0,
                    solid,
                    true,
                );
                at = next;
                falling = fell;
            }
        }

        at
    }
}
