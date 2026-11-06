//! The gate at the top of the bridge, and the riddle that opens it. Spec
//! 0058.
//!
//! **It stands in a gap that was already there.** `bridge::landing` splits
//! the far parapet round a way through three metres wide, so the deck, the
//! gap and the hall's door are one straight line in. Nothing here moves
//! that line; it builds a gate across it.

use crate::bridge;
use crate::trail::Trail;
use blitzkit::collision::Aabb;
use glam::{vec3, Quat, Vec3};

/// How high the gate stands to the underside of its head, how thick a leaf
/// is, and how far the posts and the head are across.
///
/// **Three and a half metres, against a parapet of 0.62.** The kerb round
/// this landing is knee high on purpose, so a gate hung off it would be a
/// gate you step over. It brings its own frame, and the frame is what has
/// to read from halfway down the deck: a hundred metres off and lit from
/// behind, the only thing of this anybody sees is its outline against
/// Valhalla.
pub const HIGH: f32 = 3.4;
pub const LEAF_THICK: f32 = 0.14;
pub const POST: f32 = 0.42;
pub const HEAD_DEEP: f32 = 0.36;

/// How far the head oversails the posts, each side.
pub const HEAD_OUT: f32 = 0.14;

/// How long the leaves take to swing, in seconds.
///
/// **Slower than any other door here.** The imaginarium's hatch is
/// `aloft::LEAF_DROPS` at 0.9 and the cellar's bookcase is quicker still,
/// and both of those are things you work and walk through. This is the end
/// of the trail: the swing is the last beat of it and the thing you stand
/// and watch, so it takes about as long as the walk from the watchman to
/// the opening.
pub const SWINGS: f32 = 3.0;

/// Where the gate stands and how wide the opening is: the line where the
/// landing's slab ends, on the way through the far parapet.
///
/// **Taken off the landing rather than written again.** The gap is
/// `bridge::WAY_WIDE` in a parapet `bridge::RAIL_THICK` thick at
/// `bridge::LANDING_OUT` from the top of the deck, and all three of those
/// belong to the landing. Written out here they would be a second account
/// of a hole, and this building knows what two accounts do.
pub fn at(reaches: f32) -> (Vec3, f32) {
    let top = bridge::top(reaches);

    (
        vec3(
            top.x - bridge::LANDING_OUT + bridge::RAIL_THICK * 0.5,
            top.y,
            top.z,
        ),
        bridge::WAY_WIDE,
    )
}

/// How wide one leaf is, which is half the opening.
pub fn leaf_wide(reaches: f32) -> f32 {
    at(reaches).1 * 0.5
}

/// How high a leaf is, which is clear of the head by a finger.
pub fn leaf_high() -> f32 {
    HIGH - 0.06
}

/// The two posts and the head over them.
///
/// Always there, open or shut. The leaves are what moves.
pub fn frame(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !shows(trail) {
        return Vec::new();
    }

    let (middle, wide) = at(reaches);
    let out = wide * 0.5 + POST * 0.5;
    let mut boxes = Vec::new();

    for side in [-1.0f32, 1.0] {
        boxes.push(Aabb::from_center_size(
            vec3(middle.x, middle.y + HIGH * 0.5, middle.z + side * out),
            vec3(POST, HIGH, POST),
        ));
    }

    // the head, oversailing both posts, which is what keeps the top of this
    // from reading as a third post laid down
    boxes.push(Aabb::from_center_size(
        vec3(middle.x, middle.y + HIGH + HEAD_DEEP * 0.5, middle.z),
        vec3(POST, HEAD_DEEP, (out + POST * 0.5 + HEAD_OUT) * 2.0),
    ));

    boxes
}

/// Which way a leaf has turned, `swung` of the way open.
///
/// `side` is -1 for the leaf hinged south and 1 for the one hinged north.
/// Both open west, into Valhalla's ground, so neither ever sweeps through
/// where you stand to watch it.
pub fn turned(side: f32, swung: f32) -> f32 {
    side * std::f32::consts::FRAC_PI_2 * swung.clamp(0.0, 1.0)
}

/// Where the middle of that leaf is, and how it is turned, for drawing.
///
/// Shut, the arm from the hinge to the middle of the leaf runs along the
/// opening; open, it runs west. One rotation carries it from the first to
/// the second, and the leaf's own box turns by the same amount, so there
/// is nothing here that says "open" twice.
pub fn leaf(reaches: f32, side: f32, swung: f32) -> (Vec3, Quat, f32) {
    let (middle, wide) = at(reaches);
    let half = leaf_wide(reaches) * 0.5;
    let hinge = vec3(middle.x, middle.y, middle.z + side * wide * 0.5);
    let turn = turned(side, swung);
    let arm = Quat::from_rotation_y(turn) * vec3(0.0, 0.0, -side * half);

    (
        hinge + arm + Vec3::Y * leaf_high() * 0.5,
        Quat::from_rotation_y(turn),
        leaf_high(),
    )
}

/// What a leaf stops you walking through, `swung` of the way open.
///
/// **The box round the leaf and not the leaf.** A collider here cannot
/// tilt, so a leaf halfway open has no box of its own shape. What it has
/// is the box that holds it, which is thin and across the opening when the
/// gate is shut, thin and along the jamb when it is open, and fatter than
/// the leaf in between. Fatter is the right way to be wrong: the leaf you
/// can see is inside the thing stopping you, rather than the other way
/// about, and one rule covers every angle instead of a switch halfway with
/// an invisible door on one side of it.
pub fn leaf_solid(reaches: f32, side: f32, swung: f32) -> Aabb {
    let (middle, _) = at(reaches);
    let (centre, turn, high) = leaf(reaches, side, swung);
    let half = vec3(LEAF_THICK * 0.5, 0.0, leaf_wide(reaches) * 0.5);
    let mut least = Vec3::splat(f32::MAX);
    let mut most = Vec3::splat(f32::MIN);

    for (x, z) in [(-1.0f32, -1.0f32), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
        let corner = centre + turn * vec3(half.x * x, 0.0, half.z * z);

        least = least.min(corner);
        most = most.max(corner);
    }

    Aabb::new(
        vec3(least.x, middle.y, least.z),
        vec3(most.x, middle.y + high, most.z),
    )
}

/// Everything up here you can walk into: the frame, and the leaves where
/// they have got to.
pub fn solid(reaches: f32, trail: &Trail, swung: f32) -> Vec<Aabb> {
    let mut out = frame(reaches, trail);

    if out.is_empty() {
        return out;
    }

    out.extend(wall(reaches, trail));

    for side in [-1.0f32, 1.0] {
        out.push(leaf_solid(reaches, side, swung));
    }

    out
}

/// Whether there is a gate here at all, which is whether there is a bridge.
pub fn shows(trail: &Trail) -> bool {
    bridge::shows(trail)
}

/// How far the gate has swung after `dt` more seconds, given where it was.
///
/// **One number is the whole state.** Nought is shut and unanswered, one is
/// open, and it only ever goes up, so there is no second flag saying the
/// riddle is answered and no way for the two to disagree.
pub fn swings(swung: f32, answered: bool, dt: f32) -> f32 {
    if !answered && swung <= 0.0 {
        return 0.0;
    }

    (swung + dt / SWINGS).clamp(0.0, 1.0)
}

/// The box the watchman fills, for looking at him.
///
/// A body and not a point. `people::TALL` is how tall one is drawn and
/// 0.9 across is what `walk` treats one as, which is the same body the
/// landing was measured against when spec 0055 put him a fifth of the way
/// off the line you walk in on.
pub fn watchman_box(reaches: f32) -> Aabb {
    let (at, _) = bridge::watchman(reaches);

    Aabb::from_center_size(
        at + Vec3::Y * crate::people::TALL * 0.5,
        vec3(0.9, crate::people::TALL, 0.9),
    )
}

/// Whether the sight is on the watchman, from where you are and the way
/// you are looking.
///
/// **The building's own ray and the building's own reach.** `room::slab`
/// and `room::REACH` are what every cabinet, case and door in here is
/// found with. A second test would be a second answer to "am I looking at
/// this", and the one thing this project keeps finding is what two
/// answers to one question do.
pub fn watched(reaches: f32, trail: &Trail, from: Vec3, way: Vec3) -> bool {
    let way = way.normalize_or_zero();

    if !shows(trail) || way == Vec3::ZERO {
        return false;
    }

    crate::room::slab(from, way, &watchman_box(reaches))
        .is_some_and(|far| far <= crate::room::REACH)
}

/// How a leaf is made up: how many boards across it, how wide the groove
/// between two of them is, how far ironwork stands off the face, how deep
/// a strap is and where up the leaf the straps run.
///
/// **An odd number of boards**, so the middle of a leaf is a board and not
/// a joint. Five across a metre and a half is a board of 0.3, which is a
/// plank somebody could have sawn.
pub const BOARDS: usize = 5;
pub const GROOVE: f32 = 0.025;
pub const PROUD: f32 = 0.018;
pub const STRAP: f32 = 0.13;
pub const STRAPS: [f32; 3] = [0.16, 0.5, 0.84];

/// The ring on each leaf: how far across, how thick the iron of it is, and
/// how far in from the meeting edge it hangs.
pub const RING: f32 = 0.26;
pub const RING_THICK: f32 = 0.05;
pub const RING_IN: f32 = 0.26;

/// The wall the gate is set in, which is the far parapet carried up.
///
/// **A gate you can see straight past is a prop.** Standing on the landing
/// with the gate shut, the whole steading was in plain sight either side
/// of it over a kerb 0.62 high, so the gate withheld nothing and read as a
/// flat somebody had stood up. Spec 0058: it is what is between you and
/// Valhalla, and that is only true if Valhalla is behind it.
///
/// **Set in from the parapet's faces rather than flush with them.** Flush,
/// the wall's two long faces and the kerb's share a plane and both are on
/// show, which is the fault this building has found eight times. Set in by
/// a finger, the kerb reads as the plinth the wall stands on, which is
/// what a wall on a parapet looks like anyway.
pub const WALL_IN: f32 = 0.04;

/// How far the wall stands over the head of the gate, and the course along
/// the top of it.
///
/// **The wall is taller than the head rather than level with it.** Level,
/// the two tops share a plane over the strip where the head oversails the
/// opening, and both are on show. Three centimetres of wall over it buries
/// the head's top where they overlap and leaves it exposed where the head
/// stands proud of the wall, which is two different heights and no shared
/// plane.
///
/// And a course along the top, because a wall ending in a straight line
/// against the sky is a flat. The garden's numbers, so this roofline and
/// that one are one roofline.
pub const WALL_OVER: f32 = 0.03;

pub fn wall(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !shows(trail) {
        return Vec::new();
    }

    let top = bridge::top(reaches);
    let (middle, wide) = at(reaches);
    let west = top.x - bridge::LANDING_OUT;
    let north = top.z + bridge::LANDING_WIDE * 0.5;
    let south = top.z - bridge::LANDING_WIDE * 0.5;

    [
        (south, middle.z - wide * 0.5),
        (middle.z + wide * 0.5, north),
    ]
    .iter()
    .map(|(from, to)| {
        Aabb::new(
            vec3(west + WALL_IN, top.y + bridge::RAIL_UP, *from),
            vec3(
                west + bridge::RAIL_THICK - WALL_IN,
                top.y + HIGH + HEAD_DEEP + WALL_OVER,
                *to,
            ),
        )
    })
    .collect()
}

/// The course along the top of that wall, as (middle, size).
pub fn cope(reaches: f32, trail: &Trail) -> Vec<(Vec3, Vec3)> {
    use crate::garden::{COPE_DEEP, COPE_OUT};

    wall(reaches, trail)
        .into_iter()
        .map(|panel| {
            let size = panel.size();

            (
                vec3(
                    panel.center().x,
                    panel.max.y + COPE_DEEP * 0.5,
                    panel.center().z,
                ),
                vec3(size.x + COPE_OUT * 2.0, COPE_DEEP, size.z),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::Asked;
    use glam::vec3;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// What you can walk into up here, with the gate `swung` of the way
    /// open: the bridge, the gate and the place through it.
    fn up_here(swung: f32) -> Vec<Aabb> {
        let trail = done();
        let mut out = bridge::solid(REACHES, &trail);

        out.extend(solid(REACHES, &trail, swung));
        out.extend(crate::valhalla::solid(REACHES, &trail));

        out
    }

    /// Spec 0058: shut, it is the way to Valhalla and it is closed.
    ///
    /// **Walked rather than measured.** A gate that fills the opening on
    /// paper is a gate with a body-width gap beside it if the posts are a
    /// finger out, and this building has had three colliders that looked
    /// right and let a walk through. Walked from the middle of the
    /// landing, straight at the opening, for long enough to cross it
    /// twice.
    #[test]
    fn the_way_through_is_shut() {
        let top = bridge::top(REACHES);
        let (middle, _) = at(REACHES);
        let start = vec3(top.x - bridge::LANDING_OUT * 0.5, top.y + 0.2, top.z);
        let ended = bridge::tests::walked(
            start,
            vec3(middle.x - 6.0, top.y, top.z),
            9.0,
            &up_here(0.0),
        );

        assert!(
            ended.x > middle.x + crate::RADIUS * 0.5,
            "the gate is at {:.2} and the walk fetched up at {:.2}, which \
             is through it",
            middle.x,
            ended.x
        );
    }

    /// And open, it is not.
    #[test]
    fn the_way_through_opens() {
        let top = bridge::top(REACHES);
        let (middle, _) = at(REACHES);
        let start = vec3(top.x - bridge::LANDING_OUT * 0.5, top.y + 0.2, top.z);
        let ended = bridge::tests::walked(
            start,
            vec3(middle.x - 6.0, top.y, top.z),
            9.0,
            &up_here(1.0),
        );

        assert!(
            ended.x < middle.x - 2.0,
            "the gate is open at {:.2} and the walk only reached {:.2}",
            middle.x,
            ended.x
        );
    }

    /// Spec 0058: it does not open until he has asked.
    ///
    /// **The one number carries both.** Nought is shut and unanswered, so
    /// a frame that has not been answered leaves it at nought however many
    /// of them go by.
    #[test]
    fn it_stays_shut_until_the_riddle_is_answered() {
        let mut swung = 0.0;

        for _ in 0..600 {
            swung = swings(swung, false, 1.0 / 60.0);
        }

        assert_eq!(swung, 0.0);

        // and once answered it goes on without being answered again, so
        // looking away halfway through does not shut it on you
        swung = swings(swung, true, 1.0 / 60.0);
        assert!(swung > 0.0);

        for _ in 0..(SWINGS * 60.0) as u32 {
            swung = swings(swung, false, 1.0 / 60.0);
        }

        assert_eq!(swung, 1.0);
    }

    /// Spec 0058: the leaves open away from where you stand to watch.
    ///
    /// **Which is the whole reason a swinging collider is survivable.**
    /// They turn west into Valhalla's ground, so no part of a leaf ever
    /// sweeps the landing you are standing on.
    #[test]
    fn the_leaves_open_away_from_the_landing() {
        let (middle, _) = at(REACHES);

        for side in [-1.0f32, 1.0] {
            for tenth in 0..=10 {
                let box_ = leaf_solid(REACHES, side, tenth as f32 / 10.0);

                assert!(
                    box_.max.x <= middle.x + LEAF_THICK,
                    "side {side} at {tenth} tenths reaches {:.3} east and the \
                     gate stands at {:.3}",
                    box_.max.x,
                    middle.x
                );
            }
        }
    }

    /// Spec 0058: looking at the watchman is how he asks.
    #[test]
    fn the_watchman_is_looked_at_from_where_you_arrive() {
        let top = bridge::top(REACHES);
        let (him, _) = bridge::watchman(REACHES);
        let eye = vec3(top.x, top.y + crate::EYE, top.z);
        let at_him = him + Vec3::Y * crate::people::TALL * 0.5 - eye;

        assert!(
            watched(REACHES, &done(), eye, at_him),
            "standing on the deck's last tread and looking straight at him \
             is not looking at him"
        );
        assert!(
            !watched(REACHES, &done(), eye, Vec3::NEG_X),
            "looking away down the way through is looking at him"
        );
        assert!(
            !watched(REACHES, &Trail::new(), eye, at_him),
            "there is a watchman before there is a bridge"
        );
    }
}
