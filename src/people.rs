//! The first people in this building. Spec 0033.
//!
//! **Because forty lamps blinking is not forty guests changing rooms.** Spec
//! 0029 settled that a guest is the light under a shut door, which is right
//! for a corridor at rest and wrong for one in motion: spec 0030's whole
//! demonstration was a row of strips going out and coming back on, and Jake
//! watched it and said it would make more sense if there were bodies
//! actually moving rooms.
//!
//! What is left is the figure and its gait.

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
/// **Two of them, above the eyes.** It was one bar across 82% of the face,
/// which at the far end of a corridor is a brow and at arm's length in a
/// mirror is a visor: it covered spec 0061's eyes entirely, so the whole
/// face was a dark stripe. Jake, 2026-11-09: the figure needs work.
///
/// Two answers to "no face" had been stacked on one head, each right for its
/// own distance. A pair of brows is both answers at once, because that is
/// what a brow is, and it leaves the eyes somewhere to be.
///
/// Up the head from the chin at nought to the crown at one, so this sits
/// above `EYE_UP` the way a brow sits above an eye.
pub const BROW_UP: f32 = 0.57;
/// How far the pair reaches across the face, and the gap between them.
pub const BROW_WIDE: f32 = 0.62;
pub const BROW_APART: f32 = 0.16;
/// **A brow is a line and not a patch.** 0.042 was the height of the one
/// bar, where it read as the shadow under a brow ridge. Halved into a pair
/// it came out square, which is two eyes drawn in the wrong place and above
/// the real ones. A brow is about a centimetre.
pub const BROW_TALL: f32 = 0.013;
/// **How far a brow stands off the face, which is millimetres.** 0.055 was
/// most of a head's radius, and a flat bar laid across a round head sticks
/// out at its ends by however far the head has curved away: the brows poked
/// off the sides of the skull like the edge of a visor. Jake, 2026-11-09,
/// the eyebrows poke out noticeably. Set against the head's own depth at the
/// brow's outer end rather than at the middle of the face, so the outer end
/// is the part that is flush and the inner end buries itself.
pub const BROW_OUT: f32 = 0.010;

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
    /// The watchman at the top of the bridge, and nobody else. Spec 0055.
    Carved,
    /// The ones already in Valhalla. Spec 0056.
    ///
    /// **Carved like him, and without his horn.** They were `Carved`,
    /// which is the whole kit, so every one of them was holding
    /// Gjallarhorn. Jake saw it and said he thought that was just
    /// Heimdall. It is: the horn he has held since the world began and
    /// sounds once is not something a hall full of people each have one
    /// of, and it is the one thing on him that says he is the gate
    /// rather than another of them.
    Einherjar,
}

impl Look {
    /// Whether this one is cut from a block rather than grown: the head,
    /// the cloak and the helm. Both of the ones at the top of the climb
    /// are, which is what makes them read as somebody rather than as
    /// guests who came outside.
    pub fn carved(self) -> bool {
        matches!(self, Look::Carved | Look::Einherjar)
    }

    /// And whether Gjallarhorn hangs from the left fist, which is his
    /// alone.
    pub fn horn(self) -> bool {
        matches!(self, Look::Carved)
    }
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
/// **Measured against the canon rather than adjusted by eye**, because
/// three of these were wrong together and each hid the others. Jake,
/// 2026-11-08, standing at a mirror: the figure needs a face and better
/// proportions. Spec 0033 settled these for somebody at the far end of a
/// corridor, where none of it showed; a mirror is nothing but close range.
///
/// As fractions of `TALL`, against a 7.5 head figure:
///
/// - the head was 0.26, which is one head in 6.65 rather than one in 7.5.
/// - the arms reached to 0.51 of the height where a hand hangs at 0.39, so
///   they were a quarter short and ended at the hip.
/// - the shoulder sat at 1.50 and the head began at 1.47, so the head was
///   sunk into the shoulders and there was no neck at all.
///
/// A big head, short arms and no neck is the mannequin read, and no one of
/// them alone would have given it.
pub const SIDES: u32 = 18;

/// Where the parts sit, in metres off the floor.
///
/// **A seventh of the height, not a fifth.** The head ran from 0.66 of the
/// way up to the crown, which is 0.35 of a 1.72 figure: a doll's head, and
/// doll proportions are most of what reads as uncanny before anybody gets
/// close enough to look for a face.
pub const LEG_TALL: f32 = 0.95;
pub const TORSO_FROM: f32 = 0.80;
pub const TORSO_TALL: f32 = 0.62;
pub const HEAD_FROM: f32 = 1.50;
pub const HEAD_TALL: f32 = TALL - HEAD_FROM;
pub const HEAD_WIDE: f32 = 0.2;

/// Where the eyes sit on the head, how far apart, and how big.
///
/// **Spec 0033 said no face, and was right for what it was deciding.** Its
/// reasoning is that a face is detail a corridor cannot carry, and at the far
/// end of one it cannot. A mirror is the opposite case: it is nothing but
/// close range, and a featureless head at arm's length is the mannequin that
/// spec spent itself avoiding. Jake, 2026-11-08: the figure needs a face.
///
/// **Guests only.** The watchman and the ones in the hall are carvings, and
/// that spec's other half holds for them: a carved thing has no face in life
/// either, so the absence is not a gap. Giving them eyes would take away the
/// one thing that says they are not guests who wandered outside.
///
/// Measured off the head rather than written in metres, so a short person's
/// eyes are not a tall person's eyes in the wrong place.
pub const EYE_UP: f32 = 0.50;
/// **Small, because the first go came out as a visor.** At 0.17 of the
/// head's width each and 0.27 apart they ran together into one dark band
/// across the whole face. An eye is about a tenth of a head's width and
/// they sit about a third of it apart, which leaves an eye's worth of gap
/// between them, which is what a face has.
pub const EYE_APART: f32 = HEAD_WIDE * 0.16;
pub const EYE_WIDE: f32 = HEAD_WIDE * 0.085;
pub const EYE_TALL: f32 = HEAD_WIDE * 0.055;
pub const EYE_DEEP: f32 = HEAD_WIDE * 0.05;
/// How far forward of the head's axis they sit: on its face, not floating off
/// it and not buried in it.
/// Past half the head's width, so they sit proud of it. At 0.40 they were
/// inside a skull of radius 0.5 and invisible, which looked exactly like a
/// reflection showing somebody's back.
pub const EYE_OUT: f32 = HEAD_WIDE * 0.52;

/// Where the neck starts, which is the top of the torso, and how wide it is.
///
/// **It exists because the proportions were fixed.** The shoulder used to
/// sit at 1.50 and the head began at 1.47, so the head was sunk three
/// centimetres into the shoulders and nothing between them was ever needed.
/// Putting the shoulder where a shoulder goes opened an eight centimetre gap
/// and the head floated in it.
///
/// Narrow enough to be a neck and not a second torso: a little over half the
/// head's width, which is what a neck is.
pub const NECK_FROM: f32 = TORSO_FROM + TORSO_TALL;
pub const NECK_TALL: f32 = HEAD_FROM - NECK_FROM;
pub const NECK_WIDE: f32 = HEAD_WIDE * 0.58;

/// How far from the middle a leg and an arm hang.
pub const LEGS_APART: f32 = 0.1;
/// **Wide enough to clear the shoulders**, which the first build was not.
/// The torso is 0.435 across at the shoulder and an arm is 0.051 of radius,
/// so hung at 0.205 it stood four centimetres proud of the body and read as
/// a seam rather than as an arm.
pub const ARMS_APART: f32 = 0.245;
pub const ARM_FROM: f32 = 1.42;
pub const ARM_TALL: f32 = 0.74;

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
        (0.70, 0.42),
        // **The widest part is where the arms are, which it was not.** It
        // went widest at 0.84 and narrowed from there to the neck, so the
        // body bulged under the armpit and the shoulder itself was narrow:
        // a dome with the head set behind it, which is the silhouette of a
        // back however the figure is facing. Jake, 2026-11-09, twice.
        //
        // Wide at the arms and sloping in from there, which is a shoulder.
        // Flat to the neck instead is a drum with a hole in the lid, and
        // the opening has to come back in to 0.21 or the collar is a wide
        // dark ring with a head floating over it. Four shapes were drawn
        // and looked at; this is the one Jake picked.
        (0.92, 0.45),
        (0.98, 0.40),
        (1.00, 0.21),
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
pub fn limb_wide(v: f32) -> f32 {
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

use std::f32::consts::{PI, TAU};

use glam::Vec3;

// --- somebody with somewhere to be, per spec 0067 ---------------------------

/// How fast somebody walks, in metres a second.
///
/// **Slower than you.** The guest runs at `crate::RUNS` and walks at
/// `crate::WALKS`, and a villager crossing a yard is doing neither: this is
/// an errand rather than a journey. Measured against the gait rather than
/// chosen, which is the trap `STEP` records: at 1.1 a stride is 1.27 seconds
/// and the feet read as walking rather than as a wind-up toy.
pub const PACE: f32 = 1.1;

/// How long somebody takes to turn on the spot, in seconds.
///
/// **Because arriving is where a figure stops being a person.** Without it a
/// villager walks up to the fire facing the way it came and is facing the
/// fire on the next frame, which is the one thing in a walk cycle that no
/// amount of leg work survives. The turn is eased at both ends and taken the
/// short way round.
pub const TURNS_IN: f32 = 0.7;

/// One stop on a round: where to stand, which way to face while there, and
/// how long to stay.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stop {
    pub at: Vec3,
    pub facing: f32,
    pub stays: f32,
}

/// Where somebody on a round is at a given moment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Doing {
    pub at: Vec3,
    pub facing: f32,
    /// How far they have walked in all, which is what the gait runs on, or
    /// `None` while they are standing still.
    pub walked: Option<f32>,
}

/// Which way `from` has to turn to be facing `to`, the short way round.
///
/// **Its own function because the long way round is the obvious bug.** A
/// figure at 3.0 radians asked to face -3.0 turns through nearly a whole
/// circle rather than through the 0.28 that is actually between them.
pub fn shortest_turn(from: f32, to: f32) -> f32 {
    (to - from + PI).rem_euclid(TAU) - PI
}

/// Where somebody is, somewhere in a round that loops for ever. Spec 0067.
///
/// **Arithmetic and not a simulation**, like the strikes, the koi and the
/// moon. Nothing here is stepped forward and nothing is remembered: hand it
/// the round and the clock and it says where that body is. What it buys is
/// that a frame can be drawn at any `since` at all, which is what every
/// picture in this repo does.
///
/// **A round begins standing.** `walked` runs on across the whole round so
/// the gait never jumps mid-stride, and it starts again at nought when the
/// round comes round. That is only invisible because the first thing on a
/// round is a stop, where the legs are still anyway, and a round that opened
/// with a walk would snap there once a loop.
///
/// An empty round leaves them at the origin, which is a round nobody wrote
/// rather than a place.
pub fn doing(round: &[Stop], since: f32) -> Doing {
    let Some(first) = round.first() else {
        return Doing {
            at: Vec3::ZERO,
            facing: 0.0,
            walked: None,
        };
    };

    // each stop, then the walk from it to the next
    let legs: Vec<(f32, f32)> = round
        .iter()
        .enumerate()
        .map(|(n, stop)| {
            let next = round[(n + 1) % round.len()];

            (stop.stays, stop.at.distance(next.at) / PACE)
        })
        .collect();
    let whole: f32 = legs.iter().map(|(a, b)| a + b).sum();

    if whole <= 0.0 {
        return Doing {
            at: first.at,
            facing: first.facing,
            walked: None,
        };
    }

    let mut left = since.rem_euclid(whole);
    let mut walked = 0.0;

    for (n, (stays, takes)) in legs.iter().copied().enumerate() {
        let stop = round[n];
        let next = round[(n + 1) % round.len()];
        let way = next.at - stop.at;
        let goes = if way.length_squared() > 1e-8 {
            way.x.atan2(-way.z)
        } else {
            stop.facing
        };

        if left < stays {
            // **Standing, and turning at both ends of it.** The way in is
            // whichever way they came, which is the leg before this one, and
            // the way out is the one they are about to take.
            let before = round[(n + round.len() - 1) % round.len()];
            let came = next_way(before.at, stop.at, stop.facing);
            let facing = if left < TURNS_IN && stays > TURNS_IN * 2.0 {
                turned(came, stop.facing, left / TURNS_IN)
            } else if left > stays - TURNS_IN && stays > TURNS_IN * 2.0 {
                turned(stop.facing, goes, (left - (stays - TURNS_IN)) / TURNS_IN)
            } else {
                stop.facing
            };

            return Doing {
                at: stop.at,
                facing,
                walked: None,
            };
        }

        left -= stays;

        if left < takes {
            let share = if takes > 0.0 { left / takes } else { 0.0 };

            return Doing {
                at: stop.at + way * share,
                facing: goes,
                walked: Some(walked + way.length() * share),
            };
        }

        left -= takes;
        walked += way.length();
    }

    Doing {
        at: first.at,
        facing: first.facing,
        walked: None,
    }
}

/// Which way a walk from `from` to `to` points, or `spare` if it is no walk
/// at all.
fn next_way(from: Vec3, to: Vec3, spare: f32) -> f32 {
    let way = to - from;

    if way.length_squared() > 1e-8 {
        way.x.atan2(-way.z)
    } else {
        spare
    }
}

/// An eased turn from one facing to another, the short way round.
fn turned(from: f32, to: f32, share: f32) -> f32 {
    let t = share.clamp(0.0, 1.0);

    from + shortest_turn(from, to) * (t * t * (3.0 - 2.0 * t))
}

#[cfg(test)]
mod tests {

    // --- spec 0067: somebody with somewhere to be -------------------------

    fn a_round() -> Vec<Stop> {
        vec![
            Stop {
                at: Vec3::ZERO,
                facing: 0.0,
                stays: 4.0,
            },
            Stop {
                at: glam::vec3(6.0, 0.0, 0.0),
                facing: 1.0,
                stays: 3.0,
            },
            // a corner, walked straight through
            Stop {
                at: glam::vec3(6.0, 0.0, -5.0),
                facing: 2.0,
                stays: 0.0,
            },
        ]
    }

    /// How long one loop of a round takes.
    fn whole(round: &[Stop]) -> f32 {
        round
            .iter()
            .enumerate()
            .map(|(n, stop)| stop.stays + stop.at.distance(round[(n + 1) % round.len()].at) / PACE)
            .sum()
    }

    /// Spec 0067: a round comes round.
    #[test]
    fn a_round_comes_round() {
        let round = a_round();
        let loops = whole(&round);

        for n in 0..50 {
            let t = n as f32 / 50.0 * loops;
            let (now, again) = (doing(&round, t), doing(&round, t + loops));

            assert!(
                now.at.distance(again.at) < 1e-3,
                "at {:.2} they are at {:?} and a round later at {:?}",
                t,
                now.at,
                again.at
            );
        }
    }

    /// Spec 0067: the legs run while walking and not while standing.
    ///
    /// **Both ways round.** A gait that is always on is a figure marching on
    /// the spot, and one that is never on is what this spec was written to
    /// replace, and either passes half of this on its own.
    #[test]
    fn the_legs_run_only_while_walking() {
        let round = a_round();
        let loops = whole(&round);
        let (mut still, mut going) = (0, 0);

        for n in 0..400 {
            let t = n as f32 / 400.0 * loops;
            let doing = doing(&round, t);
            // where they are a blink later says which they are doing, off
            // the positions rather than off the thing under test
            let moved = doing.at.distance(super::doing(&round, t + 0.01).at);

            if moved > 1e-4 {
                going += 1;
                assert!(doing.walked.is_some(), "no gait while walking at {:.2}", t);
            } else {
                still += 1;
                assert!(doing.walked.is_none(), "a gait while standing at {:.2}", t);
            }
        }

        assert!(still > 20 && going > 20, "{} still, {} going", still, going);
    }

    /// Spec 0067: a turn goes the short way round.
    #[test]
    fn a_turn_goes_the_short_way() {
        for (from, to, want) in [
            (3.0f32, -3.0f32, 0.283_185_3f32),
            (-3.0, 3.0, -0.283_185_3),
            (0.0, 1.0, 1.0),
            (0.0, -1.0, -1.0),
        ] {
            let got = shortest_turn(from, to);

            assert!(
                (got - want).abs() < 1e-4,
                "{} to {} turned by {} and not {}",
                from,
                to,
                got,
                want
            );
        }
    }

    /// Spec 0067: nobody teleports.
    ///
    /// **The one a corner could break.** A stay of nought is walked straight
    /// through, and a corner handled by skipping the leg rather than by
    /// walking it would put somebody at the far end of it in one frame.
    #[test]
    fn nobody_moves_faster_than_they_walk() {
        let round = a_round();
        let loops = whole(&round);
        let step = loops / 2_000.0;

        for n in 1..2_000 {
            let (was, now) = (
                doing(&round, step * (n - 1) as f32),
                doing(&round, step * n as f32),
            );
            let moved = was.at.distance(now.at);

            assert!(
                moved <= PACE * step * 1.01,
                "they moved {:.4} in {:.4} seconds, which is {:.2} a second",
                moved,
                step,
                moved / step
            );
        }
    }
    /// Spec 0055: the horn is the watchman's and nobody else's.
    ///
    /// **Because every one of them had it.** The ones already in Valhalla
    /// were drawn `Carved`, which is the whole kit, so a hall full of
    /// people each held Gjallarhorn. Jake saw it and said he thought that
    /// was just Heimdall.
    #[test]
    fn only_the_watchman_carries_the_horn() {
        assert!(Look::Carved.horn());
        assert!(!Look::Einherjar.horn());
        assert!(!Look::Guest.horn());

        // and both of the ones at the top of the climb are cut from a
        // block, which is what keeps them from reading as guests who
        // came outside
        assert!(Look::Carved.carved());
        assert!(Look::Einherjar.carved());
        assert!(!Look::Guest.carved());
    }

    use super::*;

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
        // **The head used to have to overlap the torso, because there was
        // nothing between them.** It sat three centimetres into the
        // shoulders and this line said so. Putting the shoulder where a
        // shoulder goes opened a gap, so there is a neck now and what has to
        // hold is that the neck closes it at both ends.
        // consts, so this is a compile-time truth stated where a reader of
        // this test looks for it
        const _: () = assert!(NECK_TALL > 0.02);
        assert!(
            (NECK_FROM - torso).abs() < 1e-5,
            "the neck starts at {:.3} and the torso ends at {:.3}, so there \
             is a gap at the collar",
            NECK_FROM,
            torso
        );
        assert!(
            (HEAD_FROM - (NECK_FROM + NECK_TALL)).abs() < 1e-5,
            "the head starts at {:.3} and the neck ends at {:.3}, so the head \
             floats",
            HEAD_FROM,
            NECK_FROM + NECK_TALL
        );
        const _: () = assert!(NECK_WIDE < HEAD_WIDE);
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
