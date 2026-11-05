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
/// **Four parts and three colours, which is the whole of why the first one
/// did not work.** It was a single lathe from crown to toe in one colour: a
/// shape of a person in one material, which is what a shop window puts
/// clothes on. Jake walked up to it and called it a creepy mannequin, and
/// that is the word for it.
///
/// So a head, a torso, two legs and two arms, and the head is not the colour
/// of the coat and the coat is not the colour of the trousers. No face and
/// no hands still: those are detail the corridor cannot carry. What makes
/// somebody read as dressed is where the colour changes, not what is drawn
/// on them.
pub const TALL: f32 = 1.73;

/// How one person differs from the next in the body rather than the
/// coat.
///
/// **Because a crowd of one mesh is a crowd of one person.** Spec 0033
/// gave everybody a coat, trousers and a head of their own and left the
/// body identical: same height, same build, same outline. Colour does
/// the work at a glance and then the eye notices the outline repeating,
/// which is the second thing that reads as mannequins after the missing
/// face.
///
/// Nothing dramatic. Height within a hand either way of `TALL` and
/// build within an eighth, which is the spread a room full of people
/// actually has. A figure half again as tall as its neighbour is a
/// different problem.
pub struct Build {
    /// A share of `TALL`.
    pub tall: f32,
    /// A share of how wide the parts are drawn.
    pub wide: f32,
}

/// How tall and how heavy the person with this seed is.
///
/// **Hashed rather than indexed.** The wardrobe is four coats picked by
/// `seed % 4`, and taking the build off the same four would tie a
/// height to a coat: every person in the green coat the same height as
/// every other, which is worse than no variety at all because it is a
/// pattern. This is a different mix of the same number.
pub fn built(seed: usize) -> Build {
    let mut n = (seed as u32).wrapping_mul(0x9E37_79B9) ^ 0x5BF0_3635;
    let mut roll = move || {
        n ^= n << 13;
        n ^= n >> 17;
        n ^= n << 5;

        (n % 10_000) as f32 / 10_000.0
    };

    Build {
        tall: 1.0 + (roll() - 0.5) * 2.0 * TALLER,
        wide: 1.0 + (roll() - 0.5) * 2.0 * HEAVIER,
    }
}

/// How far either way of the standard a person may be.
pub const TALLER: f32 = 0.075;
pub const HEAVIER: f32 = 0.11;

/// The bar of shadow over the eyes, which is how you can tell which way
/// somebody is facing.
///
/// **Not a face.** A face at corridor distance is a promise the geometry
/// cannot keep, and spec 0033 settled that. But a head with no front at
/// all is a thing that might be looking at you, and that is most of what
/// is left of the uncanny once the proportions are right. One bar across
/// the brow says which way somebody is turned and says nothing else.
pub const BROW_UP: f32 = 0.46;
pub const BROW_WIDE: f32 = 0.82;
pub const BROW_TALL: f32 = 0.042;
pub const BROW_OUT: f32 = 0.055;

/// What a figure is made to look like.
///
/// **Two populations, on purpose.** The arcade's guests are people in a
/// corridor and want to be plain: spec 0033 settled what they are and
/// why they have no faces. The ones at the top of the bridge are not
/// guests who wandered outside. Jake wants a little style on them, and
/// the style that costs this renderer nothing is carving: a thing that
/// in life also has no face, so the absence stops being a gap.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Look {
    Guest,
    Carved,
}

/// How many sides a carved part has.
///
/// **Few, and that is the whole point.** Eighteen is a lathe turning so
/// fine that the facets are gone, which is right for somebody at the
/// far end of a corridor. A carving shows the cuts. Seven was tried on
/// the guests on 2026-10-21 and made them hooded, but the fault there
/// was a crown closing to a point, not the count: [`carved_head`] ends
/// flat, the way a billet does when the carver stops.
pub const CUT_SIDES: u32 = 9;

/// A carved head: the same egg, flattened off at the crown and squarer
/// at the jaw, so it reads as cut from a block rather than grown.
pub fn carved_head(v: f32) -> (f32, f32) {
    let v = v.clamp(0.0, 1.0);
    // flat on top: the last of it does not close, it stops
    let crown = ((1.0 - v) / 0.14).clamp(0.0, 1.0).powf(0.35);
    let round = (1.0 - (2.0 * v - 1.0).powi(2)).max(0.0).sqrt();
    let jaw = 0.86 + 0.14 * v;

    (
        v * HEAD_TALL,
        HEAD_WIDE * 1.1 * round.max(0.55) * crown * jaw,
    )
}

/// How far the cloak hangs down the figure and how wide it ends.
///
/// **The one change that buys the most at a distance.** A coloured
/// cylinder thirty metres off is nothing. A cloak is immediately
/// somebody, because the outline stops being a person's and becomes a
/// shape nobody mistakes for furniture.
/// **Wider at the neck than the shoulders are.** `torso_profile` tops
/// out at 0.435 and the first cloak started at 0.29, so it hung inside
/// the shoulders and read as a bib on a string rather than as cloth
/// over the top of somebody.
/// **A mantle over the shoulders, not a cloak to the calf.** A full
/// cloak is open down the front and this is a lathe, which cannot be:
/// closed all the way round and hanging to mid calf it swallowed the
/// arms and the legs and came out a barrel with a head on it. Cut to
/// the waist, the same closed cone reads as cloth over the shoulders
/// and leaves the figure underneath it a figure.
pub const CLOAK_FROM: f32 = 1.50;
pub const CLOAK_TALL: f32 = 0.46;
pub const CLOAK_WIDE: f32 = 0.68;
pub const CLOAK_NECK: f32 = 0.47;

/// The cloak, hanging from the shoulders: a cone with the fall of the
/// cloth broken into a few folds so it is not a traffic cone.
pub fn cloak_profile(v: f32) -> (f32, f32) {
    let v = v.clamp(0.0, 1.0);
    let flare = CLOAK_NECK + (CLOAK_WIDE - CLOAK_NECK) * v.powf(1.35);
    // folds, which are what says cloth: a few shallow ones down the
    // hang, deepest at the hem where the cloth has the most slack
    let folds = 1.0 + (v * 2.1).sin() * 0.055;

    ((v - 1.0) * CLOAK_TALL, flare * folds)
}

/// A helm: a cap over the crown with a straight bar down the nose.
pub const HELM_FROM: f32 = 0.52;
pub const HELM_WIDE: f32 = 1.12;
pub const NOSE_LONG: f32 = 0.30;
pub const NOSE_WIDE: f32 = 0.055;
pub const NOSE_THICK: f32 = 0.035;

pub fn helm_profile(v: f32) -> (f32, f32) {
    let v = v.clamp(0.0, 1.0);
    let tall = HEAD_TALL * (1.0 - HELM_FROM);
    // a dome that stops short of a point, with a brow band round its
    // foot standing a little proud of it
    let dome = (1.0 - v * v).max(0.0).powf(0.42);
    let band = if v < 0.16 { 1.06 } else { 1.0 };

    (v * tall, HEAD_WIDE * HELM_WIDE * dome * band)
}

/// How many sides a turned part has.
///
/// ~~Seven, so the silhouette has corners in it and nothing promises a face
/// it has not got.~~
///
/// Struck 2026-10-21, the same hour it was tried. Faceting made it worse in
/// both rooms: seven sides with a crown that closes to a point is a spike
/// rather than a head, so a mannequin became something hooded, and a
/// corridor of them was a corridor of hooded things. The answer was never
/// the geometry's resolution.
pub const SIDES: u32 = 18;

/// Where the parts sit, in metres off the floor.
///
/// **A seventh of the height, not a fifth.** The head ran from 0.66 of the
/// way up to the crown, which is 0.35 of a 1.72 figure: a doll's head, and
/// doll proportions are most of what reads as uncanny before anybody gets
/// close enough to look for a face.
pub const LEG_TALL: f32 = 0.95;
pub const TORSO_FROM: f32 = 0.80;
pub const TORSO_TALL: f32 = 0.70;
pub const HEAD_FROM: f32 = 1.47;
pub const HEAD_TALL: f32 = TALL - HEAD_FROM;
pub const HEAD_WIDE: f32 = 0.2;

/// How far from the middle a leg and an arm hang.
pub const LEGS_APART: f32 = 0.1;
/// **Wide enough to clear the shoulders**, which the first build was not.
/// The torso is 0.435 across at the shoulder and an arm is 0.051 of radius,
/// so hung at 0.205 it stood four centimetres proud of the body and read as
/// a seam rather than as an arm.
pub const ARMS_APART: f32 = 0.245;
pub const ARM_FROM: f32 = 1.44;
pub const ARM_TALL: f32 = 0.56;

/// Where a limb bends, as a fraction of its length down from the joint it
/// hangs off. Spec 0046.
///
/// **A knee is not the middle of a leg.** It sits a little above it, and
/// getting that wrong is most of what makes a walk read as a puppet's.
pub const KNEE_AT: f32 = 0.53;

/// The two bones that fall out of it.
pub const THIGH_TALL: f32 = LEG_TALL * KNEE_AT;
pub const SHIN_TALL: f32 = LEG_TALL - THIGH_TALL;

/// An arm is the same two bones, thinner and shorter.
pub const ARM_THIN: f32 = 0.62;
pub const ARM_SHORT: f32 = ARM_TALL / LEG_TALL;

/// A foot, which is most of what says which way somebody is facing when you
/// are behind them, and in the imaginarium you usually are.
pub const FOOT_LONG: f32 = 0.24;
pub const FOOT_WIDE: f32 = 0.09;
pub const FOOT_TALL: f32 = 0.05;

/// How much of it is in front of the ankle.
pub const FOOT_AHEAD: f32 = 0.07;

/// A hand, which is the head's own egg at a fifth of the size.
pub const HAND_WIDE: f32 = 0.085;

/// How far apart two people have to be before they are passing each other
/// rather than walking through each other.
///
/// **Not `ARMS_APART`, which the imaginarium's crowd used to borrow.** How
/// wide to hang an arm and how much room two people need to pass are two
/// questions that happened to have the same answer, and spec 0046 moved
/// the arms out four centimetres to clear the shoulders and tightened a
/// test about a different room by eight.
///
/// It is less than a body is wide, on purpose. Nobody in that room gives
/// way and the paces differ, so one walking the same way as another does
/// catch it up and pass through it, which over the shoulder at a glance is
/// a crowd. What this holds is that two coming the other way never share a
/// space.
///
/// Nothing draws with it, so it is only compiled for the tests that hold
/// it. A number that exists to be asserted against should say so rather
/// than sit in the build pretending somebody reads it.
#[cfg(test)]
pub const PASSES: f32 = 0.41;

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

/// Hips, waist, chest, shoulders and a neck to sit a head on.
///
/// Closed at the bottom, because an open lathe is one you see through from
/// the far side, and the point it closes to is at `TORSO_FROM`, between the
/// tops of the legs and out of sight. Open at the neck, which the head caps.
pub fn torso_profile(v: f32) -> (f32, f32) {
    const SHAPE: [(f32, f32); 8] = [
        (0.00, 0.00),
        (0.10, 0.32),
        (0.22, 0.345),
        (0.42, 0.33),
        (0.62, 0.40),
        (0.84, 0.435),
        (0.94, 0.30),
        (1.00, 0.135),
    ];

    (v * TORSO_TALL, through(&SHAPE, v))
}

/// A head, which is an egg rather than a cone.
///
/// **The crown closed to a point and that is what read as a hood.** It went
/// 0.19 wide at 0.95 of the way up and nothing at all at the top, which over
/// the last twentieth is a spike. This is an ellipsoid, so the top of it is
/// a dome the way a skull is, slightly narrower at the jaw than at the
/// temples.
pub fn head_profile(v: f32) -> (f32, f32) {
    let v = v.clamp(0.0, 1.0);
    // an ellipsoid, pulled in a little under the widest part for a jaw
    let round = (1.0 - (2.0 * v - 1.0).powi(2)).max(0.0).sqrt();
    let jaw = 1.0 - 0.1 * (1.0 - v).powi(2);

    (v * HEAD_TALL, HEAD_WIDE * round * jaw)
}

/// How wide a limb is, from the foot at nought to the hip at one.
///
/// **One table read by both bones.** The bottom of a thigh and the top of a
/// shin are the same circle, and two tables for one circle is two answers
/// that will come apart.
fn limb_wide(v: f32) -> f32 {
    const SHAPE: [(f32, f32); 8] = [
        // an ankle, where this used to close to a point. A leg that tapers
        // to nothing had nothing to meet, and now there is a foot under it:
        // a point above a flat sole reads as a tile lying beside a leg
        // rather than as the end of one.
        (0.00, 0.075),
        (0.03, 0.095),
        (0.05, 0.105),
        (0.26, 0.14),
        (0.52, 0.12),
        (0.80, 0.175),
        // and the joint itself, which the hip and the shoulder both are.
        // Flat across the top, an arm hung level with the shoulders put a
        // square corner out over each of them and the figure wore
        // epaulettes.
        (0.94, 0.168),
        (1.00, 0.105),
    ];

    through(&SHAPE, v)
}

/// A thigh, hanging from the hip. Spec 0046.
///
/// **Hung rather than stood on its own foot, which is the whole of the first
/// fault that spec names.** `with_rotation` turns a mesh about its origin,
/// so a limb whose origin is at the floor swings its hip out of the body
/// when you turn it, and a figure with a leg a third of a metre clear of its
/// hip is the stiff, disconnected thing Jake saw.
///
/// The answer is not to flip the lathe. That reverses the winding and a limb
/// wound away from you is drawn nowhere at all, which is the trap the first
/// attempt fell into and the reason the legs were slid instead. It is to run
/// the profile from minus its length up to nought: the origin is the joint,
/// `v` still climbs, and the winding never changes.
pub fn thigh_profile(v: f32) -> (f32, f32) {
    (
        (v - 1.0) * THIGH_TALL,
        limb_wide(1.0 - KNEE_AT + v * KNEE_AT),
    )
}

/// And a shin, hanging from the knee, picking up the width the thigh left
/// off at.
pub fn shin_profile(v: f32) -> (f32, f32) {
    ((v - 1.0) * SHIN_TALL, limb_wide(v * (1.0 - KNEE_AT)))
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

/// How fast somebody walks, which is the speed you do.
///
/// A crossing to the next number is about three metres and a crossing takes
/// a second and a half, so nobody going next door ever reaches this cap. It
/// is for the ones spec 0034 sends to room 1024.
pub const WALKS: f32 = crate::SPEED;

/// Somebody on the floor of the corridor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Walking {
    /// Which room they came out of.
    pub from: u64,
    /// Where they are, on the floor.
    pub at: Vec3,
    /// Which way they are facing, as a yaw.
    pub facing: f32,
    /// How far through the crossing, nought to one.
    pub along: f32,
    /// And how far they have actually walked, in metres, which is what
    /// swings the legs. Spec 0046.
    ///
    /// **Not the same number.** A crossing is a crossing whether the move
    /// sends somebody next door or to room 1024, and pacing the legs off it
    /// gave a three metre walk and a four hundred metre walk the same seven
    /// paces. Under a slid foot that was a skate. Under a hip angle derived
    /// from the stride it is simply wrong.
    pub walked: f32,
}

/// Everybody the wave has reached and who has not got there yet.
///
/// `front` is the hotel's own wave front, in rooms. Somebody comes out when
/// it passes their door and is in by the time it is `CROSSES_IN` past them.
pub fn crossing(
    reaches: f32,
    front: Option<f32>,
    first: u64,
    last: u64,
    sends: impl Fn(u64) -> Option<u64>,
) -> Vec<Walking> {
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
        // **Where the move sends them, not next door.** Every move was a
        // shuffle of one when this was written, so this said `n + 1`. Spec
        // 0034's prime powers send room 3 to room 8 and room 10 to room
        // 1024, and a corridor emptying westward is the point of it.
        let Some(to) = sends(n) else {
            continue;
        };
        let arrives = crate::aleph::at(reaches, to);
        let way = arrives - leaves;
        // **At a walk, however far they are going.** Somebody bound for room
        // 1024 is four hundred metres west, and lerping them there over a
        // crossing is a streak rather than a person. They walk, and when the
        // crossing is over they are wherever that got them, which for most
        // of them is out of the window.
        let whole = way.length();
        let gone = along * whole.min(WALKS * CROSSES_IN / crate::aleph::TRAVELS_AT);
        let at = leaves + way.normalize_or_zero() * gone;

        out.push(Walking {
            from: n,
            at: vec3(at.x, 0.0, at.z),
            facing: way.x.atan2(-way.z),
            along,
            walked: gone,
        });
    }

    out
}

/// How far one step carries somebody. Spec 0046.
///
/// **Here rather than in the room that happens to walk people.** It was
/// `aloft::STRIDE`, which made how far a leg carries you a fact about the
/// imaginarium. Everything below comes out of it and two rooms read it, so
/// it belongs beside the gait.
///
/// It was also being spent twice as fast as it says. The old phase put a
/// whole cycle of both legs into one of these, so a figure took two steps
/// for every one it was paced at and minced. A cycle is two steps.
pub const STEP: f32 = 0.7;

/// Where a leg is in its own cycle, in steps, nought to two.
///
/// Nought to one is the stance, when the foot is on the ground and holding
/// somebody up. One to two is the swing, when it is coming through.
fn cycle(walked: f32, leg: usize) -> f32 {
    (walked / STEP + leg as f32).rem_euclid(2.0)
}

/// Whether this is the leg holding somebody up.
pub fn planted(walked: f32, leg: usize) -> bool {
    cycle(walked, leg) < 1.0
}

/// How far a foot is in front of its own hip, in metres.
///
/// **A sine cannot do this and that is why the old one skated.** A planted
/// foot does not move, so while the body travels a step the foot has to
/// travel exactly one step backwards relative to it, at a constant rate. A
/// sine is only ever momentarily still. So the stance is a straight line
/// and the swing is the curve that joins its two ends.
///
/// The curve lands at the speed the body is going, which is what keeps the
/// plant from being a jerk: a foot that arrives with any speed of its own
/// has to stop dead at the moment it touches, and that is the one place in
/// a walk where the eye is looking. It leaves at rest instead, because a
/// cubic made to match at both ends swings the foot three centimetres
/// further back while the knee has already lifted it ten, which reads as
/// dragging. What is left is a toe lingering at the push off and a heel
/// reaching a little past the plant and settling back, which is what a leg
/// does.
pub fn foot_fore(walked: f32, leg: usize) -> f32 {
    let t = cycle(walked, leg);

    if t < 1.0 {
        STEP * (0.5 - t)
    } else {
        let q = t - 1.0;

        STEP * (-3.0 * q * q * q + 4.0 * q * q - 0.5)
    }
}

/// Which hangs the leg at this angle, positive forward.
pub fn hip(walked: f32, leg: usize) -> f32 {
    (foot_fore(walked, leg) / LEG_TALL).clamp(-1.0, 1.0).asin()
}

/// How far a knee bends at the middle of a swing.
pub const KNEE_BENDS: f32 = 1.0;

/// Bent through the swing, when the foot has to clear the ground, and dead
/// straight through the stance, when it is carrying somebody.
///
/// That is the half of the cycle the old `lift` raised the whole leg
/// through. The foot comes up for the same reason and the hip stays where
/// it belongs.
pub fn knee(walked: f32, leg: usize) -> f32 {
    let t = cycle(walked, leg);

    if t < 1.0 {
        0.0
    } else {
        ((t - 1.0) * std::f32::consts::PI).sin() * KNEE_BENDS
    }
}

/// How high the hips are, which is not a constant.
///
/// A straight leg carrying somebody is the long side of its own swing, so
/// the hips are highest when it is upright under them and lowest at either
/// end of its stance. Twice a cycle, about seven centimetres, and it is the
/// difference between walking and gliding. Taken off whichever leg is
/// planted, and the two agree at the handover because they swap at the same
/// angle.
pub fn hips(walked: f32) -> f32 {
    let holding = if planted(walked, 0) { 0 } else { 1 };

    LEG_TALL * hip(walked, holding).cos()
}

/// Where an ankle ends up in its owner's own terms: how far in front of
/// their middle, and how far off the floor.
///
/// **Nothing sets either number.** Both are where the two bones leave off
/// once the hips and the two angles are known, so a planted foot comes out
/// at exactly nothing off the floor and a swinging one comes up because the
/// knee is bent. The old build raised the whole leg by a number instead,
/// which is the same foot off the ground and a hip off the body with it.
///
/// **And the drawing puts the foot here rather than walking the bones a
/// second time.** A chain written out twice is two answers to where
/// somebody's foot is, and the day they disagree is the day a foot comes
/// off a leg.
pub fn ankle(walked: f32, leg: usize) -> (f32, f32) {
    let hip = hip(walked, leg);
    let knee = knee(walked, leg);
    let shin = hip - knee;

    (
        THIGH_TALL * hip.sin() + SHIN_TALL * shin.sin(),
        hips(walked) - (THIGH_TALL * hip.cos() + SHIN_TALL * shin.cos()),
    )
}

/// How far an arm swings against the leg on its own side.
pub const ARM_SWINGS: f32 = 0.55;

pub fn shoulder(walked: f32, leg: usize) -> f32 {
    -hip(walked, leg) * ARM_SWINGS
}

/// How far an elbow bends.
pub const ELBOW_BENDS: f32 = 0.5;

/// Forwards, where a knee bends back, and while the leg on the same side is
/// planted, which is when that arm is the one coming through.
///
/// **The bend is the only thing in this figure that tells an arm from a
/// leg.** The two are the same pair of meshes at a different size, neither
/// hand nor foot is big enough to say it, and a figure whose elbows bend
/// like knees is walking two ways at once.
pub fn elbow(walked: f32, leg: usize) -> f32 {
    let t = cycle(walked, leg);

    if t < 1.0 {
        (t * std::f32::consts::PI).sin() * ELBOW_BENDS
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleph::Arrival;

    /// Spec 0033: a figure has a surface.
    ///
    /// **Because three things in this building have had none.** The candles,
    /// the clock dial and the fire's coals were all invisible at once,
    /// because a turned mesh with too few steps draws two points and no skin.
    #[test]
    fn a_figure_is_not_nothing() {
        for (what, profile) in [
            ("the torso", torso_profile as fn(f32) -> (f32, f32)),
            ("the head", head_profile as fn(f32) -> (f32, f32)),
            ("a thigh", thigh_profile as fn(f32) -> (f32, f32)),
            ("a shin", shin_profile as fn(f32) -> (f32, f32)),
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

    /// Spec 0033: a head is about a seventh of somebody, and it is domed.
    ///
    /// **Both of these made it read as a mannequin.** The head ran from 0.66
    /// of the way up to the crown, which is a fifth of the figure and a
    /// doll's proportion, and it closed to a point over the last twentieth,
    /// which is a hood rather than a skull. Jake walked up to one and said
    /// so twice.
    #[test]
    fn a_head_is_a_seventh_of_somebody_and_it_is_domed() {
        // read off the profiles rather than the constants, which is what is
        // actually drawn and is also the only way to ask a question whose
        // answer is not already in the source
        let head = HEAD_FROM + head_profile(1.0).0;
        let skull = head_profile(1.0).0;
        let torso = TORSO_FROM + torso_profile(1.0).0;
        let leg = -(thigh_profile(0.0).0 + shin_profile(0.0).0);

        assert!(
            (head / skull) > 6.0 && (head / skull) < 9.0,
            "a head is one {:.1}th of the figure",
            head / skull
        );

        // domed: near the crown it is still carrying width, where a cone
        // would have run out of it
        let widest = (0..=40)
            .map(|step| head_profile(step as f32 / 40.0).1)
            .fold(0.0f32, f32::max);

        assert!(
            head_profile(0.9).1 > widest * 0.4,
            "the top of the head is a point: {:.3} against {:.3} at its widest",
            head_profile(0.9).1,
            widest
        );

        // and the parts stack into one body rather than leaving a gap
        assert!(
            TORSO_FROM < leg,
            "the torso starts above the top of the legs"
        );
        assert!(HEAD_FROM < torso, "the head floats over the neck");
        assert!(
            (head - TALL).abs() < 1e-5,
            "the figure comes out {:.3} tall rather than {:.3}",
            head,
            TALL
        );
    }

    /// Spec 0033: a coat, trousers and a head, and no two the same.
    ///
    /// One colour from crown to toe is a mannequin by definition: a single
    /// material in the shape of a person is what a shop window dresses.
    #[test]
    fn nobody_is_one_colour_from_crown_to_toe() {
        for which in 0..crate::aim::COATS.len() {
            let coat = crate::aim::COATS[which];
            let trousers = crate::aim::TROUSERS[which];
            let head = crate::aim::HEADS[which];

            assert_ne!(
                coat, trousers,
                "guest {} is one colour below the neck",
                which
            );
            assert_ne!(
                coat, head,
                "guest {}'s head is the colour of their coat",
                which
            );
        }

        assert_eq!(crate::aim::COATS.len(), crate::aim::TROUSERS.len());
        assert_eq!(crate::aim::COATS.len(), crate::aim::HEADS.len());

        // and a corridor of them is not one person repeated
        for which in 1..crate::aim::COATS.len() {
            assert_ne!(
                crate::aim::COATS[which],
                crate::aim::COATS[0],
                "two guests are wearing the same coat"
            );
        }
    }

    /// Spec 0046: the legs are paced off the ground somebody covers, not
    /// off the crossing they are in the middle of.
    ///
    /// **Because a crossing is a crossing however far it goes.** Spec
    /// 0034's moves send room 3 to room 8 and room 10 to room 1024, and
    /// `STEPS` paces fitted into either of them gave a four hundred metre
    /// walk the same seven steps as a three metre one.
    ///
    /// The second assertion is the control: the moves this asks about have
    /// to be far enough apart for the old pacing to have been visibly
    /// wrong, or a gait that still ignored the ground would pass.
    ///
    /// They are not as far apart as the hotel's arithmetic suggests.
    /// Nobody walks four hundred metres on camera: a crossing caps what it
    /// covers at `WALKS * CROSSES_IN / TRAVELS_AT`, which is 6.3 metres,
    /// and whoever is bound for room 1024 goes the rest of the way out of
    /// the window. So the spread here is about one to one and a half, and
    /// that is still one and a half times the paces.
    #[test]
    fn a_long_move_takes_the_same_step_as_a_short_one() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let out = crossing(reaches, Some(12.0), 1, 40, |n| Arrival::Coaches.sends(n));

        let mut legs: Vec<(u64, f32, f32)> = Vec::new();

        for who in out.iter().filter(|who| who.along > 0.5) {
            let went = who.at.distance(crate::aleph::at(reaches, who.from));

            assert!(
                (who.walked - went).abs() < 0.05,
                "room {}'s guest has walked {:.2} metres and is {:.2} from their door",
                who.from,
                who.walked,
                went
            );

            legs.push((who.from, went, who.walked / STEP));
        }

        assert!(legs.len() > 1, "nobody is halfway across");

        let (_, short, _) = *legs
            .iter()
            .min_by(|one, two| one.1.total_cmp(&two.1))
            .expect("somebody is crossing");
        let (_, long, _) = *legs
            .iter()
            .max_by(|one, two| one.1.total_cmp(&two.1))
            .expect("somebody is crossing");

        assert!(
            long > short * 1.4,
            "the longest move is {:.1} metres and the shortest {:.1}, which is not far enough apart to tell",
            long,
            short
        );
    }

    /// Spec 0033: nobody is out before the wave reaches them, and everybody
    /// it is passing is.
    #[test]
    fn only_the_ones_the_wave_has_reached_are_out() {
        let reaches = crate::room::Room::of(crate::games()).reaches;

        assert!(
            crossing(reaches, None, 1, 40, |n| Arrival::One.sends(n)).is_empty(),
            "somebody is crossing a corridor with no wave in it"
        );

        let out = crossing(reaches, Some(12.0), 1, 40, |n| Arrival::One.sends(n));

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
        let starting = crossing(reaches, Some(20.0), 1, 40, |n| Arrival::One.sends(n))
            .into_iter()
            .min_by(|one, two| one.along.total_cmp(&two.along))
            .expect("nobody is crossing");
        let ending = crossing(reaches, Some(20.0), 1, 40, |n| Arrival::One.sends(n))
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

    /// Spec 0034: a crossing walks to where the move sends somebody, not to
    /// the next number.
    ///
    /// **This said `n + 1`**, because every move was a shuffle of one when
    /// it was written. The prime powers send room 3 to room 8 and room 10 to
    /// room 1024, and a corridor emptying westward is the point of them.
    #[test]
    fn they_cross_to_where_the_move_sends_them() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let near = crossing(reaches, Some(20.0), 1, 40, |n| Arrival::One.sends(n));
        let far = crossing(reaches, Some(20.0), 1, 40, |n| Arrival::Coaches.sends(n));

        assert_eq!(near.len(), far.len(), "a different crowd is out");

        let mut further = 0;

        for (one, two) in near.iter().zip(far.iter()) {
            assert_eq!(one.from, two.from);

            // room 1 goes to room 2 either way, and room 2 to room 4 is two
            // doors rather than one: past that they part company
            if one.from < 3 || one.along < 0.2 {
                continue;
            }

            let went = (two.at - crate::aleph::at(reaches, two.from)).length();
            let next = (one.at - crate::aleph::at(reaches, one.from)).length();

            assert!(
                went > next,
                "room {} walks {:.2} to a prime power and {:.2} to next door",
                one.from,
                went,
                next
            );

            // west, which is where the bigger numbers are
            assert!(
                two.at.x < crate::aleph::at(reaches, two.from).x,
                "room {} is not heading west",
                two.from
            );

            further += 1;
        }

        assert!(further > 0, "nobody was sent further than next door");

        // and nobody streaks: a crossing is a walk however far away the room
        // they are going to is
        for who in far.iter() {
            let went = (who.at - crate::aleph::at(reaches, who.from)).length();

            assert!(
                went <= WALKS * CROSSES_IN / crate::aleph::TRAVELS_AT + 0.01,
                "room {} covered {:.1} in one crossing",
                who.from,
                went
            );
        }
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
            let out = crossing(reaches, Some(front), 1, last, |n| Arrival::One.sends(n));

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
                let walked = step as f32 / 400.0 * STEP * 2.0;
                let (_, up) = ankle(walked, leg);

                assert!(up > -1e-4, "a foot is {:.4} under the floor", up);

                // a centimetre, which is clear of the floor rather than
                // the heel settling back onto it at the plant
                if up > 0.01 {
                    let then = foot_fore(walked + 1e-4, leg);
                    let now = foot_fore(walked, leg);

                    assert!(
                        then >= now - 1e-6,
                        "a foot is up while it is going backwards"
                    );
                }
            }
        }
    }

    /// Spec 0046: a planted foot does not move.
    ///
    /// **The one the old gait could not pass.** A foot slid by a sine is
    /// only ever momentarily still, so every figure in this building skated.
    /// Through the stance the body travels a step and the foot travels a
    /// step backwards against it, at the same rate, so the ground it stands
    /// on never changes.
    #[test]
    fn a_planted_foot_does_not_slide() {
        let mut worst: f32 = 0.0;
        let was = 0.2 * STEP + foot_fore(0.2 * STEP, 0);

        for step in 0..=200 {
            // inside one stance, away from the handover at either end
            let walked = (0.2 + 0.6 * step as f32 / 200.0) * STEP;

            assert!(planted(walked, 0), "leg nought is not in its stance");

            worst = worst.max((walked + foot_fore(walked, 0) - was).abs());
        }

        assert!(worst < 1e-4, "a planted foot moved {:.4} metres", worst);
    }

    /// **Its control:** the sine this replaces does slide, and by a lot.
    #[test]
    fn a_slid_foot_does_not_stay_put() {
        // what the old one did: a foot swung 0.34 either side on a sine,
        // a whole cycle to the step rather than to the pair of them
        let slid = |walked: f32| walked + (walked / STEP * std::f32::consts::TAU).sin() * 0.34;

        let was = slid(0.2 * STEP);
        let mut worst: f32 = 0.0;

        for step in 0..=200 {
            let walked = (0.2 + 0.6 * step as f32 / 200.0) * STEP;

            worst = worst.max((slid(walked) - was).abs());
        }

        assert!(
            worst > 0.1,
            "the slide only moved a foot {:.3} metres, so this proves nothing",
            worst
        );
    }

    /// Spec 0046: a limb hangs from its joint, so turning it leaves the
    /// joint where it is.
    #[test]
    fn a_limb_never_leaves_its_joint() {
        for (what, profile) in [
            ("a thigh", thigh_profile as fn(f32) -> (f32, f32)),
            ("a shin", shin_profile as fn(f32) -> (f32, f32)),
        ] {
            assert!(
                profile(1.0).0.abs() < 1e-6,
                "{} tops out {:.3} off its own origin",
                what,
                profile(1.0).0
            );

            // and the whole of it hangs below, so nothing of it is above
            // the joint to swing out of the body
            for step in 0..=40 {
                let v = step as f32 / 40.0;

                assert!(
                    profile(v).0 <= 1e-6,
                    "{} reaches {:.3} above its joint",
                    what,
                    profile(v).0
                );
            }
        }
    }

    /// **Its control:** the lathe this replaces stood on its own foot, so
    /// turning it about its origin took the hip with it.
    #[test]
    fn a_limb_stood_on_its_foot_comes_away_from_the_body() {
        // the profile as it was: nought at the foot, the whole length up,
        // turned by the widest a straight leg gets, which is half a step
        // under a leg this long
        let was = |v: f32| v * LEG_TALL;
        let swung = (STEP * 0.5 / LEG_TALL).asin();
        let moved = was(1.0) * swung.sin();

        assert!(
            moved > 0.2,
            "turning the old leg by {:.3} moved its hip {:.3} metres, which is not enough to see",
            swung,
            moved
        );
    }

    /// Spec 0046: a knee bends through the swing and an elbow through the
    /// stance, and neither of them ever goes past straight.
    ///
    /// They are the same pair of meshes at two sizes, so which half of the
    /// cycle each bends in, and which way, is the whole of what tells an arm
    /// from a leg.
    #[test]
    fn a_knee_and_an_elbow_bend_in_opposite_halves() {
        for leg in 0..2 {
            for step in 0..=400 {
                let walked = step as f32 / 400.0 * STEP * 2.0;
                let (knee, elbow) = (knee(walked, leg), elbow(walked, leg));

                assert!(knee >= 0.0, "a knee bent {:.3} past straight", knee);
                assert!(elbow >= 0.0, "an elbow bent {:.3} past straight", elbow);

                if planted(walked, leg) {
                    assert!(knee < 1e-6, "a knee is bent while the foot is down");
                } else {
                    assert!(elbow < 1e-6, "an elbow is bent while the leg swings");
                }
            }

            // and both of them do bend, which the three above would pass
            // just as happily if neither ever did
            let swinging = STEP * (1.5 - leg as f32);
            let standing = STEP * (0.5 - leg as f32);

            assert!(
                knee(swinging.rem_euclid(STEP * 2.0), leg) > 0.5,
                "the knee never bends"
            );
            assert!(
                elbow(standing.rem_euclid(STEP * 2.0), leg) > 0.25,
                "the elbow never bends"
            );
        }
    }

    /// Spec 0046: the hips rise and fall twice a stride.
    ///
    /// Highest when the leg under them is upright, lowest at either end of
    /// its stance. The second assertion is the control: a body held at one
    /// height passes the first and fails this.
    #[test]
    fn the_hips_rise_twice_a_stride() {
        let high = hips(STEP * 0.5);
        let low = hips(0.0);

        assert!(
            (high - LEG_TALL).abs() < 1e-5,
            "the hips are {:.4} at mid stance rather than a leg's length",
            high
        );
        assert!(
            high - low > 0.03,
            "the hips only move {:.4} metres, which nobody can see",
            high - low
        );

        // twice over a stride of two steps, which is once a step
        for step in 0..=100 {
            let walked = step as f32 / 100.0 * STEP;

            assert!(
                (hips(walked) - hips(walked + STEP)).abs() < 1e-5,
                "the two steps of a stride are not the same height"
            );
            assert!(
                hips(walked) <= high + 1e-5 && hips(walked) >= low - 1e-5,
                "the hips went outside their own swing"
            );
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

        let out = crossing(reaches, hotel.travelling(), 1, 40, |n| {
            Arrival::One.sends(n)
        });

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
        let mut was = hip(0.0, 0) - hip(0.0, 1);

        for step in 1..=200 {
            // five strides of walking, so there are swaps to count
            let walked = step as f32 / 200.0 * STEP * 10.0;
            let now = hip(walked, 0) - hip(walked, 1);

            if now.signum() != was.signum() && now.abs() > 1e-6 {
                swapped += 1;
            }

            was = now;
        }

        assert!(
            swapped > 4,
            "the legs swapped {} times in five strides",
            swapped
        );
    }
}
