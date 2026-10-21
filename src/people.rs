//! The first people in this building. Spec 0033.
//!
//! **Because forty lamps blinking is not forty guests changing rooms.** Spec
//! 0029 settled that a guest is the light under a shut door, which is right
//! for a corridor at rest and wrong for one in motion: spec 0030's whole
//! demonstration was a row of strips going out and coming back on, and Jake
//! watched it and said it would make more sense if there were bodies
//! actually moving rooms.
//!
//! The light stays. A guest at rest is still a strip under a door, because
//! there is nothing to see in a room you cannot go into. What this adds is
//! the ones the wave has reached.

use glam::{vec3, Vec3};

/// How tall somebody is, and where they come apart.
///
/// **A figure of turned parts, which is how this building makes anything
/// round.** No face and no hands: both are detail the corridor's distance
/// cannot carry, and a smeared face is the thing that turns a figure into a
/// mistake.
pub const TALL: f32 = 1.72;
pub const LEG_TALL: f32 = 0.84;
pub const BODY_FROM: f32 = 0.66;
pub const BODY_TALL: f32 = TALL - BODY_FROM;

/// How far from the middle a leg and an arm hang.
pub const LEGS_APART: f32 = 0.098;
pub const ARMS_APART: f32 = 0.2;
pub const ARM_FROM: f32 = 1.38;
pub const ARM_TALL: f32 = 0.52;

/// Read a profile off a table of `(how far up, how wide)`, between the two
/// entries either side.
///
/// **Rather than a formula.** A body is not an equation and the places it
/// changes are the shoulders, the neck and the jaw, which is three numbers to
/// move when Jake says it does not read.
fn through(table: &[(f32, f32)], v: f32) -> f32 {
    let v = v.clamp(0.0, 1.0);

    for pair in table.windows(2) {
        let (from, was) = pair[0];
        let (to, is) = pair[1];

        if v <= to {
            let how_far = ((v - from) / (to - from).max(1e-6)).clamp(0.0, 1.0);

            return was + (is - was) * how_far;
        }
    }

    table.last().map(|(_, wide)| *wide).unwrap_or(0.0)
}

/// Hips, chest, shoulders, neck, head, closed at both ends.
///
/// Closed, because an open lathe is one you see through from the far side.
/// The bottom closes to a point at `BODY_FROM`, which is between the tops of
/// the legs and out of sight.
pub fn body_profile(v: f32) -> (f32, f32) {
    const SHAPE: [(f32, f32); 10] = [
        (0.00, 0.00),
        (0.09, 0.33),
        (0.20, 0.40),
        (0.42, 0.44),
        (0.60, 0.43),
        (0.655, 0.17),
        (0.715, 0.23),
        (0.84, 0.28),
        (0.95, 0.19),
        (1.00, 0.00),
    ];

    (v * BODY_TALL, through(&SHAPE, v))
}

/// A leg, and the same mesh scaled down is an arm.
pub fn limb_profile(v: f32) -> (f32, f32) {
    const SHAPE: [(f32, f32); 6] = [
        (0.00, 0.00),
        (0.05, 0.12),
        (0.22, 0.15),
        (0.52, 0.13),
        (0.80, 0.18),
        (1.00, 0.17),
    ];

    (v * LEG_TALL, through(&SHAPE, v))
}

/// How many rooms of the wave's travel somebody takes to cross.
///
/// **The crowd is this number and not the corridor's length.** The front
/// moves at `aleph::TRAVELS_AT`, so however long the hotel is, the only
/// people out are the ones it has passed within this many rooms. The hotel
/// stays endless and nobody is counting it.
///
/// At six rooms a second this is a second and a half on the floor, which over
/// a corridor 3.2 wide is a brisk walk rather than a sprint.
pub const CROSSES_IN: f32 = 9.0;

/// Somebody on the floor of the corridor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Walking {
    /// Which room they came out of.
    pub from: u64,
    /// Where they are, on the floor.
    pub at: Vec3,
    /// Which way they are facing, as a yaw.
    pub facing: f32,
    /// How far through the crossing, nought to one, which is what swings the
    /// legs.
    pub along: f32,
}

/// Everybody the wave has reached and who has not got there yet.
///
/// `front` is the hotel's own wave front, in rooms. Somebody comes out when
/// it passes their door and is in by the time it is `CROSSES_IN` past them.
pub fn crossing(reaches: f32, front: Option<f32>, first: u64, last: u64) -> Vec<Walking> {
    let Some(front) = front else {
        return Vec::new();
    };

    let mut out = Vec::new();

    // **The band behind the front, and only that, in one place.** It was
    // bounded twice, once here and once by throwing away anybody whose
    // crossing had finished, and two guards that each do the whole job mean
    // neither of them is tested: break one and the other covers for it.
    let from = (front - CROSSES_IN).ceil().max(first as f32).max(1.0);
    let to = front.floor().min(last as f32);

    if to < from {
        return out;
    }

    for n in (from as u64)..=(to as u64) {
        let along = (front - n as f32) / CROSSES_IN;

        let leaves = crate::aleph::at(reaches, n);
        let arrives = crate::aleph::at(reaches, n + 1);
        // out of the doorway and across, which is a diagonal because rooms
        // alternate sides: room n and room n+1 face each other
        let at = leaves + (arrives - leaves) * along;
        let way = arrives - leaves;

        out.push(Walking {
            from: n,
            at: vec3(at.x, 0.0, at.z),
            facing: way.x.atan2(-way.z),
            along,
        });
    }

    out
}

/// How many paces somebody takes to get across.
pub const STEPS: f32 = 7.0;

/// Where a leg is in its swing, as an angle.
fn pace(along: f32, leg: usize) -> f32 {
    along * STEPS * std::f32::consts::TAU + leg as f32 * std::f32::consts::PI
}

/// How far a leg has swung, fore and aft, in metres.
///
/// **The ceiling on animation here.** Legs that swap is what spec 0033 says
/// is enough, and anything past it is a character, which that spec says not
/// to build.
///
/// Swung rather than turned, because `with_rotation` pivots a mesh about its
/// own origin and a leg's origin is its foot: rotating one swings the hip off
/// the body. Flipping the lathe to put the origin at the hip reverses the
/// winding, and a limb wound away from you is drawn nowhere at all. A foot
/// that slides fore and aft and lifts as it comes forward is what is left,
/// and at corridor distance in the dark it is a walk.
pub fn swing(along: f32, leg: usize) -> f32 {
    pace(along, leg).sin() * 0.34
}

/// And how far off the floor it is while it comes forward.
pub fn lift(along: f32, leg: usize) -> f32 {
    pace(along, leg).cos().max(0.0) * 0.07
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0033: a figure has a surface.
    ///
    /// **Because three things in this building have had none.** The candles,
    /// the clock dial and the fire's coals were all invisible at once,
    /// because a turned mesh with too few steps draws two points and no skin.
    #[test]
    fn a_figure_is_not_nothing() {
        for (what, profile) in [
            ("the body", body_profile as fn(f32) -> (f32, f32)),
            ("a limb", limb_profile as fn(f32) -> (f32, f32)),
        ] {
            let mesh = crate::cellar::turned(16, 24, profile);

            assert!(
                mesh.vertices.len() > 64,
                "{} came out with {} points",
                what,
                mesh.vertices.len()
            );
            assert!(!mesh.indices.is_empty(), "{} has no triangles", what);

            // and it has width somewhere along it, rather than being a line
            let widest = (0..=40)
                .map(|step| profile(step as f32 / 40.0).1)
                .fold(0.0f32, f32::max);

            assert!(
                widest > 0.1,
                "{} is a wire, {:.3} at its widest",
                what,
                widest
            );
        }
    }

    /// Spec 0033: nobody is out before the wave reaches them, and everybody
    /// it is passing is.
    #[test]
    fn only_the_ones_the_wave_has_reached_are_out() {
        let reaches = crate::room::Room::of(crate::games()).reaches;

        assert!(
            crossing(reaches, None, 1, 40).is_empty(),
            "somebody is crossing a corridor with no wave in it"
        );

        let out = crossing(reaches, Some(12.0), 1, 40);

        assert!(
            !out.is_empty(),
            "the wave passed twelve doors and nobody came out"
        );

        for who in out.iter() {
            assert!(
                who.from <= 12,
                "room {} is out and the wave has not reached it",
                who.from
            );
            assert!(
                who.from as f32 >= 12.0 - CROSSES_IN,
                "room {} is still out long after the wave passed",
                who.from
            );
        }

        // the one the front is on this instant has just stepped out
        assert!(
            out.iter().any(|who| who.from == 12 && who.along < 0.05),
            "nobody stepped out of the room the front is passing"
        );
    }

    /// Spec 0033: they cross to the next number, which is the other side.
    ///
    /// **The first version of this tested `aleph::at` and not the walk.** It
    /// worked out both doors itself and then checked they were on opposite
    /// sides, which is true of the corridor however wrong the crossing is:
    /// sending everybody to room n+2 instead left it passing.
    #[test]
    fn they_cross_to_the_next_number() {
        let reaches = crate::room::Room::of(crate::games()).reaches;

        // somebody at the very start of a crossing is in their own doorway,
        // and somebody at the end of one is in the next room's
        let starting = crossing(reaches, Some(20.0), 1, 40)
            .into_iter()
            .min_by(|one, two| one.along.total_cmp(&two.along))
            .expect("nobody is crossing");
        let ending = crossing(reaches, Some(20.0), 1, 40)
            .into_iter()
            .max_by(|one, two| one.along.total_cmp(&two.along))
            .expect("nobody is crossing");

        let out_of = crate::aleph::at(reaches, starting.from);

        assert!(
            (starting.at.x - out_of.x).abs() < 0.2 && (starting.at.z - out_of.z).abs() < 0.2,
            "room {}'s guest did not start at their own door",
            starting.from
        );

        // and the one furthest through is nearly at the next number's door,
        // which is across the corridor
        let into = crate::aleph::at(reaches, ending.from + 1);
        let left = crate::aleph::at(reaches, ending.from);

        assert!(
            (left.z - into.z).abs() > crate::aleph::ACROSS * 0.5,
            "the next number is not across the corridor"
        );
        assert!(
            (ending.at.z - into.z).abs() < (ending.at.z - left.z).abs(),
            "room {}'s guest is {:.2} through and still nearer the door they left",
            ending.from,
            ending.along
        );
        assert!(
            ending.along > 0.8,
            "nobody in the band is near the end of a crossing"
        );
    }

    /// Spec 0033: a longer corridor is not a bigger crowd.
    ///
    /// The hotel is endless. What is on screen is the band the wave is
    /// passing, and it is the same band whether the corridor is forty doors
    /// or forty thousand.
    #[test]
    fn a_longer_corridor_is_not_a_bigger_crowd() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let most = (CROSSES_IN as usize) + 1;

        for (front, last) in [(30.0, 40u64), (3_000.0, 40_000), (300_000.0, 4_000_000)] {
            let out = crossing(reaches, Some(front), 1, last);

            assert!(
                out.len() <= most,
                "{} people are out of a corridor {} long",
                out.len(),
                last
            );
        }
    }

    /// And a foot only leaves the floor while it is swinging forward.
    ///
    /// A foot up at the back of its swing is somebody being dragged.
    #[test]
    fn a_foot_lifts_only_on_the_way_forward() {
        for leg in 0..2 {
            for step in 0..=400 {
                let along = step as f32 / 400.0;
                let up = lift(along, leg);

                assert!(up >= 0.0, "a foot is under the floor");

                if up > 1e-4 {
                    let then = swing(along + 1e-4, leg);
                    let now = swing(along, leg);

                    assert!(
                        then >= now - 1e-6,
                        "a foot is up while it is going backwards"
                    );
                }
            }
        }
    }

    /// Spec 0033: nobody is solid.
    ///
    /// **A deliberate exception to spec 0013**, which says a collider that
    /// disagrees with what is drawn is this building's commonest fault. A
    /// corridor where the demonstration can pin you against a wall is worse
    /// than one you can walk through, and the people are the demonstration.
    #[test]
    fn nobody_is_solid() {
        let mut room = crate::room::Room::of(crate::games());
        let reaches = room.reaches;
        let mut hotel = crate::aleph::Hotel::new();

        room.hotel_doors = crate::aleph::window(crate::aleph::at(reaches, 8))
            .map(|n| (n, hotel.open_by(n)))
            .collect();

        let at_rest = room.solid().len();

        hotel.house();

        // and again partway through a wave, with a corridor full of people
        for _ in 0..120 {
            hotel.tick(1.0 / 60.0);
        }

        room.hotel_doors = crate::aleph::window(crate::aleph::at(reaches, 8))
            .map(|n| (n, hotel.open_by(n)))
            .collect();

        let out = crossing(reaches, hotel.travelling(), 1, 40);

        assert!(
            !out.is_empty(),
            "nobody was crossing, so this checked nothing"
        );
        assert_eq!(
            room.solid().len(),
            at_rest,
            "a corridor with {} people in it has colliders a corridor at rest does not",
            out.len()
        );

        // and the floor of the corridor is clear where they are walking,
        // away from the doorways, where the leaves swing
        let solid = room.solid();

        for who in out.iter().filter(|who| (0.2..0.8).contains(&who.along)) {
            let chest = who.at + vec3(0.0, 1.2, 0.0);

            for box_ in solid.iter() {
                assert!(
                    !box_.contains_point(chest),
                    "room {}'s guest is halfway across and inside something solid",
                    who.from
                );
            }
        }
    }

    /// Spec 0033: and the legs swap rather than hold still.
    #[test]
    fn the_legs_swap() {
        let mut swapped = 0;
        let mut was = swing(0.0, 0) - swing(0.0, 1);

        for step in 1..=200 {
            let along = step as f32 / 200.0;
            let now = swing(along, 0) - swing(along, 1);

            // the two legs are never in the same place as each other
            assert!(
                (swing(along, 0) - swing(along, 1)).abs() > 1e-6 || now.abs() < 1e-6,
                "both legs are in one place"
            );

            if now.signum() != was.signum() {
                swapped += 1;
            }

            was = now;
        }

        assert!(
            swapped > 4,
            "the legs swapped {} times in a crossing",
            swapped
        );
    }
}
