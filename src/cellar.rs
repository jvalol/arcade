//! The stair behind the bookcase, and the room at the bottom of it. Spec 0007.
//!
//! Behind the nook's back wall rather than down through its floor. A secret
//! stair is a hole in a wall, not a hole in a carpet, and it costs less: the
//! nook's floor is one quad and a quad has no hole in it, so a stairwell inside
//! the room would have meant laying the boards in pieces round an opening.
//! Beyond the wall there is nothing to cut.
//!
//! Nothing here draws or reads a key, so where every tread sits and whether you
//! can get down them is arithmetic, checked without a window.

use crate::room::{self, CABINET, NOOK_DEEP, THICK, WALL};
use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far down the cellar is from the nook.
pub const DOWN: f32 = 3.0;

/// The stair: how many treads, how tall each is, and how deep.
///
/// The riser is well inside what [`crate::walk::STEP`] can get you up, because
/// a stair you can only just climb is a stair you catch your feet on. Twelve of
/// them at a quarter each is the whole drop.
///
/// The tread is 0.3, which is what a stair looks like, and for a while it could
/// not be. It had to be 0.7, deeper than the body is from its middle to its
/// toe, because the step up was asked as a question about the body: raise a
/// sphere 0.9 across by one riser and it meets the next riser but one, with
/// nowhere to put its weight down. A stair you could walk down and not back up.
///
/// What is in front of your foot is a question about the floor. Asked straight
/// down instead of by moving a body at it, the tread goes back to being a
/// number about stairs.
pub const STEPS: usize = 12;
pub const RISER: f32 = DOWN / STEPS as f32;
pub const TREAD: f32 = 0.3;

/// How many bookcases the door is, and so how wide and how tall the opening is.
///
/// Two. One was 1.1, and you are 0.9 across: a tenth of a unit of daylight
/// either side, held for the nine units it takes to get down. Walked exactly
/// down the middle it fits, which is what the test did and why the test was
/// worth nothing. Nobody walks exactly down the middle of anything.
///
/// Two leaves rather than one wide one, each hinged at its own outer end, so
/// what comes out into the nook when it opens is a case on either side of the
/// opening rather than 2.2 of shelving across the floor.
pub const CASES: usize = 2;
pub const WIDE: f32 = crate::study::CASE.x * CASES as f32;
pub const HIGH: f32 = crate::study::CASE.z;

/// How much room there is over your head on the way down.
///
/// You are 1.55 at the eye, so this is comfortably over it, and the soffit goes
/// down with the stair rather than sitting flat: a flat ceiling at the cellar's
/// own height is a lid across the top of the flight.
pub const HEADROOM: f32 = 2.1;

/// How wide the passage behind it is.
///
/// Wider than you by half again. A corridor with a hand's breadth either side
/// is a corridor you scrape along, and this one is the length of a staircase.
pub const PASSAGE: f32 = 1.8;

/// How far the shaft reaches past the back wall, which is the run of the stair
/// and a landing at each end of it.
pub const LANDING: f32 = 1.0;
pub const SHAFT: f32 = LANDING * 2.0 + STEPS as f32 * TREAD;

/// The cellar itself: how far it reaches past the foot of the stair, how wide,
/// and how high its ceiling is.
///
/// Lower than the nook's 3.2. A cellar with the headroom of a hall is a
/// basement, and the difference between the two is entirely the ceiling.
pub const DEEP: f32 = 7.0;
pub const SPAN: f32 = 6.0;
pub const TALL: f32 = 2.5;

/// The first of the bookcases along the back wall that swing. [`CASES`] of them
/// from here.
///
/// Not the middle. A door in the middle of a wall is a door, and the one thing
/// a secret door must not be is the obvious candidate.
pub const CASE: usize = 2;

/// Whether a bookcase is one of the leaves.
pub fn swings(n: usize) -> bool {
    (CASE..CASE + CASES).contains(&n)
}

/// The face of the nook's back wall, which is where the opening is.
pub fn back() -> f32 {
    -(WALL + CABINET.x) - NOOK_DEEP + THICK * 0.5
}

/// Where the opening is along the wall, taken from the bookcase that swings.
pub fn opening(reaches: f32) -> f32 {
    let cases = room::bookcases(-reaches);
    let span: Vec<f32> = (CASE..CASE + CASES)
        .filter_map(|n| cases.get(n).map(|shelved| shelved.at.z))
        .collect();

    if span.is_empty() {
        return -reaches + 1.0;
    }

    span.iter().sum::<f32>() / span.len() as f32
}

/// What stands in the cellar, which is the reason there is one.
///
/// poolhall is a game and there is something to win, so by the hall's own rule
/// it belongs in a cabinet out there. A cabinet with pool on the screen is the
/// wrong object: pool is a table, and the thing you want is to walk up to the
/// table. cascada showed the shape of it. A bench that opens a window is a toy
/// too big for a bench, and a table that opens a window is a game too big for a
/// cabinet.
pub const POOLHALL: &str = "poolhall";

/// How big it is, and how far off the floor the slate sits.
///
/// Nine foot by four and a half, which is what a full size table is, at the
/// scale the rest of this building is built to.
pub const TABLE: Vec3 = glam::vec3(2.7, 0.78, 1.35);

/// The lamp over the table: how high it hangs over the cloth, and what it
/// throws.
///
/// A table has a light over it, and a cellar lit by nothing was a room of flat
/// grey surfaces with no shape to any of them. One fitting answers both: the
/// thing a pool room is lit by is the thing hanging over the table.
pub const LAMP_UP: f32 = 0.95;
pub const SHADE: Vec3 = glam::vec3(0.5, 0.1, 0.3);
pub const LAMP_LIT: f32 = 0.8;
pub const LAMP_RANGE: f32 = 6.0;
pub const LAMP_COLOUR: Vec3 = glam::vec3(1.0, 0.93, 0.78);

/// Where it stands: the middle of the cellar floor, turned so you come down the
/// stair at its end rather than its side.
pub fn table(reaches: f32) -> crate::room::Benched {
    let foot = back() - LANDING - STEPS as f32 * TREAD;

    crate::room::Benched {
        name: POOLHALL,
        at: vec3(foot - DEEP * 0.5, -DOWN, opening(reaches)),
        size: vec3(TABLE.z, TABLE.y, TABLE.x),
        // you come down the stair facing the room, so the side you work it from
        // is the side the stair is
        worked_from: Vec3::X,
    }
}

/// Every box the stair and the cellar are made of.
///
/// The shaft runs out in -x from the opening, so the stair goes away from the
/// nook rather than along it. Each tread is a slab reaching down to the cellar
/// floor rather than a step of its own thickness, because what is under a stair
/// is not anything anybody sees and a solid run is one box fewer to fall
/// between.
/// What a box down here is made of, which is the only reason the stair and the
/// cellar are told apart at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Made {
    Tread,
    Stone,
}

/// Every box the way down is built of, and what each one is.
pub fn built(reaches: f32) -> Vec<(Aabb, Made)> {
    solid(reaches)
        .into_iter()
        .enumerate()
        .map(|(n, box_)| {
            // the treads come first, then the landing, which is boards too
            let made = if n <= STEPS { Made::Tread } else { Made::Stone };

            (box_, made)
        })
        .collect()
}

pub fn solid(reaches: f32) -> Vec<Aabb> {
    let along = opening(reaches);
    let face = back();
    let mut out = Vec::new();

    // the treads, going down and out. Each is a slab from its own top all the
    // way to under the cellar floor, which is one box fewer to fall between
    // and is anyway what the underside of a stair looks like.
    let under = -DOWN - THICK;
    for n in 0..STEPS {
        let top = -(n as f32 + 1.0) * RISER;
        let from = face - LANDING - n as f32 * TREAD;

        out.push(Aabb::from_center_size(
            vec3(from - TREAD * 0.5, (top + under) * 0.5, along),
            vec3(TREAD, top - under, PASSAGE),
        ));
    }

    // a soffit over the stair, following it down. Flat at the cellar's own
    // ceiling it was a lid across the top of the flight: the stair starts at
    // nought and that ceiling is at -0.5, so the way down was sealed half a
    // unit below the landing. A stairwell's ceiling goes down with its stair.
    for n in 0..STEPS {
        let top = -(n as f32) * RISER + HEADROOM;
        let from = face - LANDING - n as f32 * TREAD;

        out.push(Aabb::from_center_size(
            vec3(from - TREAD * 0.5, top + THICK * 0.5, along),
            vec3(TREAD, THICK, PASSAGE + THICK * 2.0),
        ));
    }
    // the landing's own soffit stops at the far side of the back wall rather
    // than the near one. `back` is the wall's inner face, which is where the
    // nook is, so a box starting there reaches a third of a unit into the wall
    // and fights the lintel over the opening for the same pixels: a grey
    // rectangle with stippled edges, hanging over the books.
    out.push(Aabb::from_center_size(
        vec3(
            face - THICK - (LANDING - THICK) * 0.5,
            HEADROOM + THICK * 0.5,
            along,
        ),
        vec3(LANDING - THICK, THICK, PASSAGE + THICK * 2.0),
    ));

    // the landing at the top, level with the nook's own floor
    out.push(Aabb::from_center_size(
        vec3(face - LANDING * 0.5, -THICK * 0.5, along),
        vec3(LANDING, THICK, PASSAGE),
    ));

    // the cellar's floor
    let foot = face - LANDING - STEPS as f32 * TREAD;
    out.push(Aabb::from_center_size(
        vec3(foot - DEEP * 0.5, -DOWN - THICK * 0.5, along),
        vec3(DEEP, THICK, SPAN),
    ));

    // the shaft's two sides, so a stair with no bannister is still a stair you
    // cannot walk off sideways
    // from the far side of the back wall, like the soffit, and no higher than
    // the soffit either. They reached the wall's near face and a unit and a
    // half above the nook's floor, which puts them inside the wall and inside
    // the room over it.
    let shaft = SHAFT - LANDING - THICK;
    let up = HEADROOM + THICK;
    for side in [-1.0f32, 1.0] {
        out.push(Aabb::from_center_size(
            vec3(
                face - THICK - shaft * 0.5,
                (up - DOWN) * 0.5,
                along + side * (PASSAGE + THICK) * 0.5,
            ),
            vec3(shaft, up + DOWN, THICK),
        ));
    }

    // and the cellar's own walls, up to its own low ceiling
    let middle = -DOWN + TALL * 0.5;
    for (at, size) in [
        // the far end
        (vec3(foot - DEEP, middle, along), vec3(THICK, TALL, SPAN)),
        // and the near one in two pieces, because the stair comes through it.
        // In one piece it was a wall across the foot of the flight: you could
        // walk the whole way down and not get in.
        (
            vec3(foot, middle, along - (SPAN + PASSAGE) * 0.25),
            vec3(THICK, TALL, (SPAN - PASSAGE) * 0.5),
        ),
        (
            vec3(foot, middle, along + (SPAN + PASSAGE) * 0.25),
            vec3(THICK, TALL, (SPAN - PASSAGE) * 0.5),
        ),
        // the two long sides
        (
            vec3(foot - DEEP * 0.5, middle, along - SPAN * 0.5),
            vec3(DEEP, TALL, THICK),
        ),
        (
            vec3(foot - DEEP * 0.5, middle, along + SPAN * 0.5),
            vec3(DEEP, TALL, THICK),
        ),
        // and a lid on it
        (
            vec3(foot - DEEP * 0.5, -DOWN + TALL, along),
            vec3(DEEP, THICK, SPAN),
        ),
    ] {
        out.push(Aabb::from_center_size(at, size));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk::walk;
    use glam::Vec3;

    /// Spec 0007: the stair is one you can walk, which is a thing about the
    /// riser and not about the drawing.
    #[test]
    fn a_tread_is_lower_than_a_step() {
        let (riser, step) = (RISER, crate::walk::STEP);
        assert!(
            riser < step * 0.75,
            "a riser of {} against a step of {} is a stair you catch your feet on",
            riser,
            step
        );
        assert!((STEPS as f32 * RISER - DOWN).abs() < 1e-5);
    }

    /// Spec 0007: and the run fits in the shaft that holds it.
    #[test]
    fn the_stair_fits_the_shaft() {
        let run = STEPS as f32 * TREAD;

        assert!(run + LANDING * 2.0 <= SHAFT + 1e-5, "the run is {}", run);
    }

    /// Spec 0007: you can walk down it, and back up.
    ///
    /// Walked rather than reasoned about, by the same code that moves you in
    /// the room. A stair is the one thing in this building where what the
    /// arithmetic says and what happens when somebody walks at it have ever
    /// come apart.
    #[test]
    fn you_can_walk_down_it_and_back_up() {
        let reaches = 9.4;
        let solid = solid(reaches);
        let along = opening(reaches);
        let radius = crate::RADIUS;
        let speed = 4.2;
        let dt = 1.0 / 60.0;

        // from the landing, walking out away from the nook
        let mut at = vec3(back() - 0.5, 0.0, along);
        let mut falling = 0.0;
        for _ in 0..300 {
            let (next, fell) = walk(at, Vec3::NEG_X * speed, falling, radius, dt, &solid);
            at = next;
            falling = fell;
        }

        assert!(
            (at.y + DOWN).abs() < 0.05,
            "you got to {} and the cellar floor is at {}",
            at.y,
            -DOWN
        );

        // and back up the way you came, as far as the landing. Only as far: past
        // it is the nook, and the nook's floor is the room's business and not
        // in this test's geometry, so walking on walks off the end of the world.
        let mut back_up = false;
        for _ in 0..360 {
            let (next, fell) = walk(at, Vec3::X * speed, falling, radius, dt, &solid);
            at = next;
            falling = fell;

            if at.y.abs() < 0.01 {
                back_up = true;
                break;
            }
        }

        assert!(back_up, "you got as far back up as {}", at.y);
    }
}
