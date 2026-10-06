//! Getting about on foot: along the floor, up a step, and down under gravity.
//! Spec 0007.
//!
//! The arcade had none of this. You were a sphere pushed horizontally against
//! boxes that run from the floor to the ceiling, on a plane at y nought, and
//! your height never changed because nothing in the building was ever at a
//! different height. A stair is the first thing that is.
//!
//! Nothing here draws or reads a key, so all of it is checked without a window.

use blitzkit::collision::{move_and_slide, Aabb, Sphere};
use glam::{vec3, Vec3, Vec3Swizzles};

/// How fast you gather speed falling, and the fastest you fall.
///
/// Not the solver's 9.81. The arcade is not pretending to be the world and a
/// real g over the height of one room is a drop you cannot see the start of.
/// This is enough to put you back on the floor at once and slow enough that
/// walking off the top step reads as walking down rather than as dropping.
pub const FALL: f32 = 22.0;
pub const FASTEST: f32 = 9.0;

/// How high a thing you walk over rather than into.
///
/// The whole of why this file exists. `move_and_slide` turns the part of a
/// movement that is into a surface into movement along it, and the engine does
/// that correctly: walked at a step it reports the top edge rather than the
/// face, with a normal that leans up. But at a walking pace the part that is
/// available to be turned is tiny. You cover 0.07 of ground in a frame, and
/// 0.07 pushed against a rounded edge comes out as hundredths of a unit of
/// height. Measured over three seconds walking straight at a step, with gravity
/// off so nothing could undo it:
///
/// ```text
/// riser   reached
/// 0.10     0.00
/// 0.20     0.00
/// 0.30     0.01
/// 0.45     0.00
/// ```
///
/// It does not climb. It grinds. So a step is climbed by being stepped over:
/// the move is tried again from this much higher up and then let back down,
/// which is what every character controller does and what this one never had.
pub const STEP: f32 = 0.42;

/// How much of the movement you have to lose before it is worth trying again
/// from higher up.
///
/// Almost none of it, which is not where this started. The guess was that
/// losing most of your speed is what a step feels like and losing a little is
/// what sliding along a wall feels like, so the threshold was 0.7.
///
/// A shallow step is the case that breaks it. The engine reports a step's top
/// edge with a normal that leans more upright the lower the step is, and the
/// more upright that normal, the more of your speed the slide hands back. At a
/// riser of 0.05 it hands back enough to look unobstructed, so nothing was
/// tried, and the hair of height the slide did win was taken off again by the
/// same frame's gravity. The one riser too small to be in the way was the one
/// you could not get over.
///
/// So anything at all in the way is worth a look, and what decides it is
/// [`over_a_step`], which takes the climb only if it actually got you further
/// along the floor and found something to stand on. That is a real test. A
/// number for how blocked is blocked enough is a guess.
pub const BLOCKED: f32 = 0.995;

/// Where a sphere of this radius sits when its feet are at `at`.
fn body(at: Vec3, radius: f32) -> Sphere {
    Sphere::new(at + Vec3::Y * radius, radius)
}

/// Moves one frame: along the floor, over anything no taller than a step, and
/// down under gravity. Gives back where you are and how fast you are falling.
///
/// The three are in that order for a reason. Stepping up before falling means a
/// stair climbed on the way up is a stair walked down on the way back, by the
/// same code and with nothing that knows which way you are going.
pub fn walk(
    at: Vec3,
    wish: Vec3,
    falling: f32,
    radius: f32,
    dt: f32,
    solid: &[Aabb],
) -> (Vec3, f32) {
    let mut at = at;
    let mut climbed = false;

    if wish.length_squared() > 1e-6 {
        let flat = move_and_slide(body(at, radius), wish, dt, solid) - Vec3::Y * radius;
        let got = (flat.xz() - at.xz()).length();
        let wanted = wish.length() * dt;

        at = if got < wanted * BLOCKED {
            over_a_step(at, flat, wish, radius, dt, solid)
        } else {
            flat
        };
        climbed = at.y > flat.y + 1e-4;
    }

    // Stuck to the ground on the way down, if you were on it to start with.
    //
    // The other half of the step, and it was missing. Walking down a stair you
    // leave each tread at the top and fall to the next, and falling starts at
    // nothing: a riser of a quarter takes nine frames to fall and a tread of
    // 0.3 takes four to cross. So you never land on the next tread, you sail
    // out over the whole flight and come down somewhere near the bottom of it.
    // Which is what running off a staircase really does, and is not what
    // walking down one should feel like.
    //
    // Only a step's worth, and only if there is something there. Further than
    // that is a drop and a drop is meant to be fallen.
    //
    // Not on a frame you have just stepped up on, which undid every step up a
    // stair: you climb onto the next tread before you are over it, so what is
    // under your feet is still the tread you left, and this put you straight
    // back on it. One of the two always wins and it has to be the step.
    if falling == 0.0 && !climbed {
        if let Some(ground) = standing_at(at, at.y - STEP, at.y + 1e-3, solid) {
            at.y = ground;
        }
    }

    // and down. Carried as a speed rather than a distance so a drop builds up,
    // and reset the moment the ground stops you, or stepping off a kerb would
    // have you still falling at the bottom of the stair.
    let falling = (falling + FALL * dt).min(FASTEST);
    let under =
        move_and_slide(body(at, radius), Vec3::NEG_Y * falling, dt, solid) - Vec3::Y * radius;
    let landed = under.y >= at.y - falling * dt * 0.5;

    (under, if landed { 0.0 } else { falling })
}

/// Out of anything you are standing inside, sideways.
///
/// Everything else in this file is about you moving and the room holding still,
/// which is what `move_and_slide` is for and was the whole of getting about
/// until something in the building moved. A bookcase on a hinge sweeps the
/// floor in front of it, and the floor in front of it is where you stand to
/// pull the book: it swung through the viewer, which from the inside is the
/// shelves passing through your eye.
///
/// Sideways only. Being pushed out of a thing is one problem and being lifted
/// onto it is another, and a door that put you on top of itself would be worse
/// than one that went through you.
pub fn shoved(at: Vec3, radius: f32, leaves: &[(Vec3, glam::Quat, Vec3)]) -> Vec3 {
    let mut at = at;

    // twice over, because being pushed out of one leaf can put you into the
    // other, and the two of them meet at the opening
    for _ in 0..2 {
        for (middle_of, turn, half) in leaves {
            let middle = at + Vec3::Y * radius;
            if (middle.y - middle_of.y).abs() > half.y + radius {
                continue;
            }

            // into the leaf's own frame, where it is square again. The box it
            // fills on the room's axes is no use for this: at forty five
            // degrees that box is half as big again as the shelf is, so which
            // way is out of it is not which way is out of the shelf, and a
            // closing leaf shoved you along itself and then swept through you.
            let local = turn.inverse() * (middle - *middle_of);
            let near = vec3(
                local.x.clamp(-half.x, half.x),
                0.0,
                local.z.clamp(-half.z, half.z),
            );
            let away = vec3(local.x - near.x, 0.0, local.z - near.z);
            let gap = away.length();

            if gap > 1e-6 {
                if gap < radius {
                    at += *turn * (away / gap * (radius - gap + 1e-3));
                }
                continue;
            }

            // dead inside it, where there is no direction out: the nearest face
            // of the shelf, which is its front or its back and not its end
            let sides = [
                (half.x - local.x.abs(), Vec3::X * local.x.signum()),
                (half.z - local.z.abs(), Vec3::Z * local.z.signum()),
            ];
            let out = sides
                .iter()
                .copied()
                .min_by(|one, other| one.0.total_cmp(&other.0));

            if let Some((deep, way)) = out {
                at += *turn * (way * (deep + radius + 1e-3));
            }
        }
    }

    at
}

/// The highest thing you could stand on at this spot, between two heights.
///
/// A ray straight down, and the only part of getting about that is not about
/// how wide you are. Which is the whole lesson of this file. The body has a
/// width because the eye sits in it and a camera with its near plane at a tenth
/// cannot be let up against a wall, and because every gap in this building was
/// measured for somebody 0.9 across. None of that has anything to say about
/// what is under your feet.
///
/// Asked with the body instead, every answer came back wrong in a different
/// way. A sphere leaving a tread hangs on its lip and a sphere on a lip cannot
/// go down. A sphere walked at a step rides its edge and creeps up a wall of
/// any height, given frames. And a sphere raised by one riser meets the next
/// riser but one, so a tread had to be deeper than the body was wide: 0.7, for
/// a stair that would look right at 0.3.
fn standing_at(at: Vec3, from: f32, to: f32, solid: &[Aabb]) -> Option<f32> {
    solid
        .iter()
        .filter(|box_| {
            at.x >= box_.min.x && at.x <= box_.max.x && at.z >= box_.min.z && at.z <= box_.max.z
        })
        .map(|box_| box_.max.y)
        .filter(|top| *top > from && *top <= to)
        .fold(None, |best: Option<f32>, top| {
            Some(best.map_or(top, |best| best.max(top)))
        })
}

/// Up onto whatever is in front of you, if it is no more than a step up and you
/// can get along once you are on it.
///
/// What is in front of you is asked a body's length ahead, because that is how
/// far your weight has to travel to be over it, and asked straight down,
/// because what is there is a question about the floor and not about you.
fn over_a_step(at: Vec3, flat: Vec3, wish: Vec3, radius: f32, dt: f32, solid: &[Aabb]) -> Vec3 {
    let way = wish.normalize_or_zero();
    let ahead = at + way * (radius + wish.length() * dt);
    let Some(top) = standing_at(ahead, at.y + 1e-3, at.y + STEP, solid) else {
        return flat;
    };

    // and there has to be room to stand there, which is not the same question.
    // A step you can put a foot on with a wall just above it is a wall.
    let lifted = vec3(at.x, top, at.z);
    let raised =
        move_and_slide(body(at, radius), Vec3::Y * (top - at.y), 1.0, solid) - Vec3::Y * radius;
    if raised.y < top - 1e-3 {
        return flat;
    }

    let over = move_and_slide(body(lifted, radius), wish, dt, solid) - Vec3::Y * radius;
    if (over.xz() - at.xz()).length() <= (flat.xz() - at.xz()).length() {
        return flat;
    }

    over
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    const RADIUS: f32 = crate::RADIUS;
    const SPEED: f32 = 4.2;
    const DT: f32 = 1.0 / 60.0;

    /// A floor with its top at nought, and one step of `riser` from x two on.
    fn a_step(riser: f32) -> Vec<Aabb> {
        vec![
            Aabb::from_center_size(vec3(0.0, -1.0, 0.0), vec3(40.0, 2.0, 40.0)),
            Aabb::from_center_size(
                vec3(8.0, riser * 0.5 - 1.0, 0.0),
                vec3(12.0, 2.0 + riser, 40.0),
            ),
        ]
    }

    /// Walks at +x for a while and says where it ends up.
    fn walked(solid: &[Aabb], from: Vec3, seconds: f32) -> Vec3 {
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / DT) as usize {
            let (next, fell) = walk(at, Vec3::X * SPEED, falling, RADIUS, DT, solid);
            at = next;
            falling = fell;
        }

        at
    }

    /// Spec 0007: a step of the stair's own riser is something you walk up.
    #[test]
    fn it_climbs_a_step() {
        for riser in [0.05f32, 0.1, 0.2, 0.3, 0.4] {
            let at = walked(&a_step(riser), Vec3::ZERO, 3.0);

            assert!(
                (at.y - riser).abs() < 0.02,
                "a riser of {} left you at {}",
                riser,
                at.y
            );
            assert!(at.x > 4.0, "you climbed it and stopped: x {}", at.x);
        }
    }

    /// Spec 0007: and one taller than a step is a wall.
    ///
    /// Otherwise the pass that gets you up a stair gets you on top of
    /// everything else in the building.
    #[test]
    fn it_does_not_climb_a_wall() {
        for riser in [STEP + 0.1, 1.0, 2.0] {
            let at = walked(&a_step(riser), Vec3::ZERO, 3.0);

            assert!(at.y < 0.05, "you got up a {} wall, to {}", riser, at.y);
            assert!(at.x < 2.0, "you walked through it to {}", at.x);
        }
    }

    /// Spec 0007: you come to rest on the floor rather than sinking or hovering.
    #[test]
    fn it_puts_you_on_the_floor() {
        let floor = vec![Aabb::from_center_size(
            vec3(0.0, -1.0, 0.0),
            vec3(40.0, 2.0, 40.0),
        )];

        // dropped from a height
        let mut at = vec3(0.0, 3.0, 0.0);
        let mut falling = 0.0;
        for _ in 0..240 {
            let (next, fell) = walk(at, Vec3::ZERO, falling, RADIUS, DT, &floor);
            at = next;
            falling = fell;
        }

        assert!(at.y.abs() < 1e-3, "you came to rest at {}", at.y);
        assert_eq!(falling, 0.0, "you are still falling at {}", falling);
    }

    /// Spec 0007: walking off the top of a step puts you at the bottom of it.
    #[test]
    fn it_walks_you_back_down() {
        let riser = 0.3;
        let solid = a_step(riser);

        // start up on the step and walk back the way you came
        let mut at = vec3(6.0, riser, 0.0);
        let mut falling = 0.0;
        for _ in 0..180 {
            let (next, fell) = walk(at, Vec3::NEG_X * SPEED, falling, RADIUS, DT, &solid);
            at = next;
            falling = fell;
        }

        assert!(at.x < 1.0, "you did not get off the step: x {}", at.x);
        assert!(at.y.abs() < 1e-3, "you came off it at {}", at.y);
    }

    /// Spec 0007: and a wall still stops you, which is what the room is built of.
    #[test]
    fn a_wall_is_still_a_wall() {
        let solid = vec![
            Aabb::from_center_size(vec3(0.0, -1.0, 0.0), vec3(40.0, 2.0, 40.0)),
            Aabb::from_center_size(vec3(4.0, 1.5, 0.0), vec3(0.4, 3.0, 40.0)),
        ];
        let at = walked(&solid, Vec3::ZERO, 3.0);

        assert!(
            at.x < 3.8 - RADIUS + 0.05,
            "you walked into the wall: {}",
            at.x
        );
        assert!(at.y.abs() < 1e-3, "you climbed the wall to {}", at.y);
    }
}
