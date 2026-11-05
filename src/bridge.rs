//! The bridge at the end of the trail: a rainbow out of the field, going up.
//! Spec 0054.
//!
//! **Nothing here exists until the building has been done.** Every room that
//! asks you something has to have given up its answer, which is what
//! `trail::Asked` lists and what [`shows`] reads. Until then this draws
//! nothing and stands nothing up, and spec 0036's rule holds: somebody who
//! never finds the trail cannot tell it is there.
//!
//! **A crossing and not a room.** Spec 0036 warns that inventing a fourth
//! room as the prize is how you get a room nobody wanted, and the warning is
//! why there is nothing at the top of this. What the climb gives you is
//! height and the only sight of the building from outside it: eleven rooms
//! walked and nobody has ever seen the arcade whole.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3, Vec4};

use crate::trail::{Asked, Trail};

/// How far west of the garden's ground the foot of it stands.
///
/// Far enough to see the whole arc from the gap you come out of, near
/// enough to walk to without the building going behind you.
pub const OUT: f32 = 18.0;

/// How far west it runs from there, and how high it climbs over that run.
///
/// **A hundred and sixty metres is the field's own reach**, so a hundred of
/// run with eighteen of standing-off leaves the top well inside the ground
/// rather than hanging over the edge of it. Twenty-six up is six storeys of
/// the hall.
///
/// The two are a pair and not two numbers: the rise over the run is what
/// decides whether this can be walked at all, and at these it is steepest
/// in the middle at a shade under half, which is twenty-five degrees.
pub const RUN: f32 = 100.0;
pub const RISE: f32 = 26.0;

/// How wide the deck is.
///
/// **Six metres, which is deliberately more than it needs.** A body is 0.9
/// across and every walkway in this building has been built too narrow
/// first: the nook went 3.0 to 6.4, the baths 9 to 11 and then wide enough
/// for walkways. A bridge you edge along is not a prize.
pub const WIDE: f32 = 6.0;

/// How many colours lie along it, and how thick the deck is.
pub const BANDS: usize = 7;
pub const DEEP: f32 = 0.35;

/// How many pieces the drawn deck is cut into.
///
/// Only the drawing. What holds you up is [`treads`], which is a different
/// and much finer list for a different reason, and both come out of
/// [`high`] so there is one curve rather than two.
pub const PIECES: usize = 48;

/// How long one tread of the collider is.
///
/// **This is what makes it walkable or not.** Colliders here are boxes that
/// cannot tilt, so a slope is a staircase to the thing walking up it, and a
/// riser taller than `walk::STEP` is a wall. At the steepest part of the
/// arc, a tread of 0.6 rises 0.28, and the walk steps 0.42.
pub const TREAD: f32 = 0.6;

/// Where the last of the climb is done, as a share of the run.
///
/// **It ends level.** Arriving somewhere is standing on it, and a deck
/// still climbing at its last piece is a thing you walk off the end of.
pub const LEVELS_AT: f32 = 0.86;

/// The seven, in order from the foot.
///
/// Past one on purpose. Colour is an unclamped multiplier in this engine
/// and anything over one glows with no light on it, which is what a rainbow
/// is: light rather than paint. Neon in the cellar is 3.2 and this is
/// quieter than that, because the field has a sky behind it and a sun on it.
pub const COLOURS: [Vec3; BANDS] = [
    vec3(2.0, 0.42, 0.42),
    vec3(2.0, 1.02, 0.36),
    vec3(1.90, 1.74, 0.40),
    vec3(0.46, 1.86, 0.62),
    vec3(0.42, 1.10, 2.0),
    vec3(0.52, 0.56, 1.95),
    vec3(1.20, 0.52, 1.90),
];

/// Whether the building has given up everything it asks.
///
/// **Every variant rather than a count.** A number here would go stale the
/// first time a room grows a question, and spec 0036 is plain that this
/// must never become "3 of 8 found". `Asked::every` is the one list.
pub fn shows(trail: &Trail) -> bool {
    Asked::every().iter().all(|one| trail.given().contains(one))
}

/// Where the foot of it stands, in the field.
pub fn foot(reaches: f32) -> Vec3 {
    let out = crate::field::starts(reaches) - OUT;
    let (from, to) = crate::field::way(reaches);

    vec3(out, 0.0, (from + to) * 0.5)
}

/// How high the deck is a share `along` of the way out.
///
/// **It eases out of the ground and eases into the top**, so the steepest
/// part is the middle and both ends are nearly flat. The first shape tried
/// here was a quarter circle, which rises fastest where it leaves the
/// ground: at the numbers above it went up 1.29 in its first 1.46, which is
/// a wall to walk at. `it_leaves_the_ground_at_a_walkable_angle` is that
/// mistake, kept.
pub fn high(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);
    let climbing = (along / LEVELS_AT).min(1.0);

    RISE * (1.0 - (climbing * std::f32::consts::PI).cos()) * 0.5
}

/// How steeply it climbs there, as a rise over the run.
///
/// The slope of [`high`], which is what a drawn piece is tilted by.
pub fn pitch(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);

    if along >= LEVELS_AT {
        return 0.0;
    }

    let climbing = along / LEVELS_AT;

    RISE * std::f32::consts::PI * (climbing * std::f32::consts::PI).sin() / (2.0 * RUN * LEVELS_AT)
}

/// What holds you up: one box the full width, every [`TREAD`] along.
///
/// **A different list from the drawn deck, and the reason is the engine.**
/// A collider is an axis-aligned box and cannot tilt, so a slope has to be
/// a staircase to walk on, and a staircase fine enough to walk is far too
/// fine to draw as boxes. Both come out of `high`, which is the one place
/// the shape is written.
pub fn treads(reaches: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let steps = (RUN / TREAD).ceil() as usize;
    let mut out = Vec::with_capacity(steps);

    for step in 0..steps {
        let from = step as f32 * TREAD / RUN;
        let to = ((step + 1) as f32 * TREAD / RUN).min(1.0);
        let top = high(to).max(high(from));

        out.push(Aabb::new(
            vec3(foot.x - to * RUN, top - DEEP, foot.z - WIDE * 0.5),
            vec3(foot.x - from * RUN, top, foot.z + WIDE * 0.5),
        ));
    }

    out
}

/// One piece of the deck: where it is, how big, and which colour band.
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub at: Vec3,
    pub size: Vec3,
    pub band: usize,
}

impl Piece {
    pub fn colour(&self) -> Vec4 {
        let said = COLOURS[self.band];

        Vec4::new(said.x, said.y, said.z, 1.0)
    }
}

/// The deck, from the foot to the top.
///
/// Each piece spans one band across the width and one step along the run,
/// so the seven colours lie the length of it the way a rainbow's do.
pub fn deck(reaches: f32) -> Vec<Piece> {
    let foot = foot(reaches);
    let step = RUN / PIECES as f32;
    let band = WIDE / BANDS as f32;
    let mut out = Vec::with_capacity(PIECES * BANDS);

    for piece in 0..PIECES {
        let from = piece as f32 / PIECES as f32;
        let to = (piece + 1) as f32 / PIECES as f32;
        let middle = (high(from) + high(to)) * 0.5;

        for band_at in 0..BANDS {
            out.push(Piece {
                at: vec3(
                    foot.x - (from + to) * 0.5 * RUN,
                    middle + DEEP * 0.5,
                    foot.z - WIDE * 0.5 + band * (band_at as f32 + 0.5),
                ),
                // a hair longer than the step, so two pieces lap rather
                // than meet: two faces at the walking surface stop a walk
                // dead and `walk` reports nothing at all
                size: vec3(step * 1.08, DEEP, band),
                band: band_at,
            });
        }
    }

    out
}

/// What you can stand on, which is the treads and nothing else.
pub fn solid(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !shows(trail) {
        return Vec::new();
    }

    treads(reaches)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    #[test]
    fn nothing_until_the_building_is_done() {
        let empty = Trail::new();

        assert!(!shows(&empty));
        assert!(solid(REACHES, &empty).is_empty());
    }

    #[test]
    fn one_short_is_not_done() {
        let mut nearly = Trail::new();

        for one in Asked::every().iter().skip(1) {
            nearly.gave(*one);
        }

        assert!(!shows(&nearly), "one answer short and it is still there");
        assert!(solid(REACHES, &nearly).is_empty());
    }

    #[test]
    fn it_is_there_once_every_room_has_answered() {
        assert!(shows(&done()));
        assert_eq!(solid(REACHES, &done()).len(), (RUN / TREAD).ceil() as usize);
    }

    #[test]
    fn it_rises_out_of_the_field_and_levels_off() {
        assert_eq!(high(0.0), 0.0, "it starts on the ground");
        assert!((high(1.0) - RISE).abs() < 0.001, "it reaches its full rise");

        // level over the last stretch, which is what makes the top a place
        // to stand rather than an edge
        assert!((high(LEVELS_AT) - high(1.0)).abs() < 0.001);

        // and climbing all the way up to there
        let mut was = -1.0;
        for step in 0..=40 {
            let now = high(step as f32 / 40.0 * LEVELS_AT);
            assert!(now > was, "it climbs the whole way to the level part");
            was = now;
        }
    }

    /// The first shape here was a quarter circle, which rises fastest where
    /// it leaves the ground: 1.29 over its first 1.46, a wall to walk at.
    #[test]
    fn it_leaves_the_ground_at_a_walkable_angle() {
        let step = RUN / PIECES as f32;
        let up = high(1.0 / PIECES as f32);

        assert!(
            up < step * 0.34,
            "the foot rises {} over {}, which is a wall to walk at",
            up,
            step
        );
    }

    /// The one that decides whether any of this can be used. A collider is
    /// a box that cannot tilt, so the arc is a staircase to walk on, and a
    /// riser over `walk::STEP` is a wall with a view.
    #[test]
    fn no_tread_rises_more_than_a_step() {
        let treads = treads(REACHES);
        let mut worst: f32 = 0.0;

        for pair in treads.windows(2) {
            let up = pair[1].max.y - pair[0].max.y;
            worst = worst.max(up);
        }

        assert!(
            worst < crate::walk::STEP,
            "the steepest riser is {} and a walk steps {}",
            worst,
            crate::walk::STEP
        );
    }

    /// And the control for it, so the test above is known to be able to
    /// fail: four times the tread is a staircase nobody climbs.
    #[test]
    fn a_coarse_tread_would_not_be_walkable() {
        let foot = foot(REACHES);
        let coarse = TREAD * 4.0;
        let steps = (RUN / coarse).ceil() as usize;
        let mut worst: f32 = 0.0;
        let mut was = 0.0;

        for step in 0..steps {
            let to = ((step + 1) as f32 * coarse / RUN).min(1.0);
            let top = high(to);
            worst = worst.max(top - was);
            was = top;
        }

        let _ = foot;
        assert!(
            worst > crate::walk::STEP,
            "the control did not go past a step"
        );
    }

    #[test]
    fn the_treads_lie_under_the_drawn_deck() {
        let treads = treads(REACHES);
        let foot = foot(REACHES);

        // one curve, two lists off it: a tread's top and the deck's height
        // at the same place have to be the same thing
        for box_ in &treads {
            let along = (foot.x - box_.center().x) / RUN;

            assert!(
                (box_.max.y - high(along)).abs() < 0.2,
                "a tread at {} along tops out at {} and the deck is at {}",
                along,
                box_.max.y,
                high(along)
            );
        }
    }

    #[test]
    fn the_pieces_lap_rather_than_meet() {
        let deck = deck(REACHES);
        let step = RUN / PIECES as f32;

        // two faces at the walking surface stop a walk dead, and `walk`
        // reports that as nothing at all. Found three times in one day in
        // the baths. See the walk-the-building skill.
        assert!(
            deck[0].size.x > step,
            "each piece has to be longer than its step for the two to lap"
        );
    }

    #[test]
    fn every_band_is_the_same_width_and_they_fill_it() {
        let deck = deck(REACHES);
        let foot = foot(REACHES);
        let first: Vec<_> = deck.iter().take(BANDS).collect();

        let near = first
            .iter()
            .map(|one| one.at.z - one.size.z * 0.5)
            .fold(f32::MAX, f32::min);
        let far = first
            .iter()
            .map(|one| one.at.z + one.size.z * 0.5)
            .fold(f32::MIN, f32::max);

        assert!(
            (far - near - WIDE).abs() < 0.001,
            "the bands fill the width"
        );
        assert!(
            ((near + far) * 0.5 - foot.z).abs() < 0.001,
            "and are centred"
        );
    }

    /// Walked rather than measured, which is the lesson of the pool's
    /// steps: four legal risers in a row were one solid block you could
    /// not get out of, and every rise passed.
    fn walked(from: Vec3, to: Vec3, seconds: f32, solid: &[Aabb]) -> Vec3 {
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    #[test]
    fn walked_from_the_foot_to_the_top() {
        let solid = solid(REACHES, &done());
        let foot = foot(REACHES);
        let top = vec3(foot.x - RUN + 2.0, RISE, foot.z);

        // dropped in from above the foot rather than placed, because a
        // point on a deck can be inside it
        let start = vec3(foot.x - 1.0, 1.4, foot.z);
        let ended = walked(start, top, 60.0, &solid);

        assert!(
            ended.y > RISE * 0.9,
            "walked for a minute and got to {} up, and the top is {}",
            ended.y,
            RISE
        );
        assert!(
            foot.x - ended.x > RUN * 0.85,
            "got {} of the way along a run of {}",
            foot.x - ended.x,
            RUN
        );
    }

    /// The control. Without it the walk above passes for a bridge that is
    /// one flat slab at the height of the foot.
    #[test]
    fn a_walk_off_the_side_does_not_climb() {
        let solid = solid(REACHES, &done());
        let foot = foot(REACHES);
        let beside = vec3(foot.x - 1.0, 1.4, foot.z + WIDE);
        let ended = walked(beside, vec3(foot.x - RUN, 0.0, foot.z + WIDE), 60.0, &solid);

        assert!(
            ended.y < 1.0,
            "walking beside it climbed to {}, so the deck is wider than it says",
            ended.y
        );
    }

    #[test]
    fn it_crosses_nothing() {
        // west of where the garden's ground stops, every piece of it, so
        // it never lands on the garden or on the building behind it
        let stops = crate::field::starts(REACHES);

        for piece in deck(REACHES) {
            assert!(
                piece.at.x + piece.size.x * 0.5 < stops,
                "a piece at {} is east of the field's start at {}",
                piece.at.x,
                stops
            );
        }
    }

    #[test]
    fn it_stays_inside_the_ground_it_stands_on() {
        let stops = crate::field::starts(REACHES);
        let out = treads(REACHES)
            .iter()
            .map(|box_| stops - box_.min.x)
            .fold(f32::MIN, f32::max);

        assert!(
            out < crate::field::GROUND_REACH,
            "it runs {} out and the ground reaches {}",
            out,
            crate::field::GROUND_REACH
        );
    }
}
