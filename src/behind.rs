//! The space behind the wall you wake with your back to. Spec 0011.
//!
//! There is no door, no sign, and nothing in the hall that says it is here.
//! You find it by walking into the wall off centre, or because somebody told
//! you.
//!
//! Every other room in this building is furnished and this is the opposite: a
//! space, very large and very dark, with nothing in it to look at and no
//! reason given for it being there.
//!
//! **The way in is a hole in the collider that the drawn wall covers.** That
//! is a fault everywhere else in this building and the point here. If a test
//! fails over it, the test is right to ask and the answer is spec 0011.
//!
//! Nothing here draws. Where the gap is, how big the space is and where the
//! columns stand are arithmetic, and arithmetic can be checked without a
//! window.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

use crate::room::{CABINET, THICK, WALL};

/// How far the space runs east of the hall's left wall, and how far back from
/// the hall's near end.
///
/// East, because west is taken. The garden is off the hall's left and reaches
/// well past the near end; east of the hall is unbuilt ground.
///
/// Further than the light goes, which is the only size that matters. The
/// camera's far plane is a hundred and a point light falls off long before
/// that, so what you see is dark in every direction and the far wall is a
/// thing you find rather than a thing you see.
///
/// A hundred and fifty nine, which is not a round number and is not a choice
/// either: the orbits of spec 0015 are 158.7 across and the room has to hold
/// them. It was ninety seven against a disc of 96.4, and the orbits grew when
/// `orrery::MOON_SHARE` took over setting the orbit scale, to stop the Moon's
/// orbit reaching for Mars.
///
/// Past the far plane now, which is the thing to watch and is not a fault:
/// you cannot see the far wall from the door and were never meant to. The
/// stars on it are the same stars at any distance.
pub const SPAN: f32 = 159.0;
pub const DEEP: f32 = 159.0;

/// How high the ceiling is.
///
/// It was 2.7, lower than the hall's, on the reasoning that low and vast is
/// uneasy in a way that tall and vast is not: tall and vast is a cathedral,
/// and a cathedral is a thing somebody built on purpose and is proud of.
///
/// Three and a fifth now, which is the hall's. Spec 0015 put the solar system
/// in here and eight metres was the right height for it, and eight metres
/// does not fit: this room's west wall stands fifteen centimetres outside the
/// garden's, the garden's sky springs at 3.70, and a wall taller than that
/// pokes past the edge of the sky. Four and a half metres of it stood up
/// through the clouds.
///
/// The vastness of this room is a hundred and fifty nine metres of floor, not
/// its ceiling. The sun is one metre across.
pub const HIGH: f32 = 3.2;

/// How far this floor runs back under the hall's.
///
/// Far enough that neither slab's end face is anywhere near the join. See the
/// floor in `built`.
pub const LAPS: f32 = 3.0;

/// How wide the way through the wall is, and how far off the middle of it.
///
/// Off centre, so walking straight back from where you wake meets solid wall.
/// A gap in the middle is found by everybody on their first turn round and a
/// gap at the very end is found by nobody.
///
/// A body is 0.9 across. At 1.15 it was a gap you had to line up with: the walk
/// through it wedged half way, because a sweep with an eighth of a unit either
/// side of a sphere is a sweep that catches on a shoulder. A third of a unit
/// either side is a gap you walk through.
pub const WAY_WIDE: f32 = 1.5;
pub const WAY_OFF: f32 = -1.4;

/// Where the way through is, as the two x it runs between.
///
/// The one account of it. The wall is built round it, the test that says the
/// rest of the wall is a wall reads it, and the columns are kept out of it.
pub fn way() -> (f32, f32) {
    (WAY_OFF - WAY_WIDE * 0.5, WAY_OFF + WAY_WIDE * 0.5)
}

/// How far back from the wall the worn track in the carpet runs, how wide the
/// worn part of it is, and how much of it is the soft edge either side.
///
/// Spec 0020. The track is the only thing in the hall that says the way in is
/// here, and what lets it say so without saying anything is that it is wear
/// and not a mark. Everything else in this hall is a thing somebody put there.
/// This is a thing that happened.
///
/// 3.2 back, and that is the dial on this. Nobody looks at the wall they woke
/// with their back to, so a track that only exists at the foot of it is seen
/// by somebody already standing there. What everybody does do is walk back up
/// the aisle, and the length of this is how much of that walk it is in front
/// of them for.
///
/// It was 2.2, the depth of the open bay at the near end, and 2.2 came out as
/// a stain at the skirting rather than as a path: a lane shorter than it is
/// away from you reads as a patch, and at that length most of it is behind
/// the turn. Longer is the whole of the fix. Nothing is in the way of it
/// either, because the track keeps to the middle of the aisle until it is
/// well inside the bay and the cabinets are out at the walls.
///
/// 0.78 wide, which is a body's 0.9 less a little. A worn lane is narrower
/// than the people who wore it, because feet land under somebody rather than
/// out at their shoulders.
pub const TRACK_BACK: f32 = 3.2;
pub const TRACK_WIDE: f32 = 0.78;
pub const TRACK_FADE: f32 = 0.14;

/// How much plain quad there is round it.
///
/// The band fades out well inside the quad, so nothing drawn has a straight
/// edge and the rectangle it is painted on is never a thing you can see. Wear
/// with a corner on it is a rug.
pub const TRACK_ROUND: f32 = 0.3;

/// How far past the wall's inner face it carries, and how far above the carpet
/// it lies.
///
/// The one edge of this that is straight is the one at the wall, and it is
/// straight because the wall is standing on it. Carried a hand past the face,
/// the texture's own seam is inside the wall rather than at the foot of it: a
/// repeating sampler wraps at the edge of its picture, and the wrap would
/// otherwise show as a hair of half strength wear along the skirting.
///
/// Two millimetres up, the hair the stars out past the door lie above their
/// own floor. In the carpet's own plane it would be two exposed faces sharing
/// one, which is this building's commonest fault and has cost it seven
/// sittings.
pub const TRACK_UNDER: f32 = 0.1;
pub const TRACK_HIGH: f32 = 0.002;

/// The worn track: the quad it is painted on, and where its two ends fall in
/// that quad's own u and v.
///
/// The one account of it. The quad, the weave's curve and the tests all read
/// this, because a hint whose drawn end has drifted off the hole it points at
/// is a hint that lies.
pub struct Track {
    pub at: Vec3,
    pub size: Vec3,
    /// Where it leaves the aisle, and where it meets the wall.
    pub aisle: Vec2,
    pub wall: Vec2,
    /// Its half width and its soft edge, as shares of the quad's length down
    /// the room, and how much wider than long the quad is.
    ///
    /// The quad was square for a while so that one unit of it was one length
    /// either way and the weaving needed neither of these. A square quad also
    /// caps how long the track can be, because the hall is only 5.4 across,
    /// and the length is the one number on this worth turning. `shape` buys
    /// that back: it is what the weaving measures across with, so a lane of
    /// one width comes out that width wherever the curve has got to.
    pub wide: f32,
    pub fade: f32,
    pub shape: f32,
}

pub fn track(reaches: f32) -> Track {
    let face = reaches - THICK * 0.5;
    // the middle of the aisle at the mouth of the bay, and the middle of the
    // hole at the foot of the wall
    let aisle = vec2(0.0, face - TRACK_BACK);
    let wall = vec2(WAY_OFF, face);

    // the two ends, and the band's own room round them
    let round = Vec2::splat(TRACK_WIDE * 0.5 + TRACK_FADE + TRACK_ROUND);
    let span = (wall - aisle).abs() + round * 2.0;
    // centred across, and hung off the wall end down the room, so the quad
    // stops a hand inside the wall rather than coming out the far side of it
    let far = vec2((aisle.x + wall.x) * 0.5 + span.x * 0.5, face + TRACK_UNDER);
    let near = far - span;
    let on_it = |end: Vec2| (end - near) / span;

    Track {
        at: vec3((near.x + far.x) * 0.5, TRACK_HIGH, (near.y + far.y) * 0.5),
        size: vec3(span.x, 1.0, span.y),
        aisle: on_it(aisle),
        wall: on_it(wall),
        wide: TRACK_WIDE * 0.5 / span.y,
        fade: TRACK_FADE / span.y,
        shape: span.x / span.y,
    }
}

/// How wide and how high the door at the far end is, per spec 0017.
///
/// 1.5 because `WAY_WIDE` is, and that number was paid for once already: at
/// 1.15 the walk through the way in wedged half way, because a sweep with an
/// eighth of a unit either side of a body catches on a shoulder. A door that
/// walks worse than the hole in the wall would be a joke at its own expense.
pub const DOOR_WIDE: f32 = WAY_WIDE;
pub const DOOR_HIGH: f32 = 2.2;

/// How far the frame stands proud of the wall into the room, how thick its
/// members are, and how far it is sunk into the wall behind.
///
/// Sunk, so that its back face is inside the wall rather than in the wall's
/// own plane. Two faces in one plane fight and this building has lost that
/// fight five times; a face buried in another solid is hidden and is fine.
pub const FRAME_PROUD: f32 = 0.12;
pub const FRAME_THICK: f32 = 0.16;
pub const FRAME_SUNK: f32 = 0.04;

/// How high the sill across the bottom of it stands.
///
/// Three centimetres, laid on the floor and not sunk into it, which is the
/// way round this building has learnt the hard way: a border sunk into the
/// floor was two surfaces in one plane and came back as speckle.
///
/// Three and not the frame's own 0.16. A member that closes the rectangle is
/// a member you walk over on your way through, and the one thing a doorway
/// may not be is something to trip on.
pub const SILL_HIGH: f32 = 0.03;

/// Where the door at the far end is, as the two x it runs between.
///
/// On the room's own axis, which puts it dead opposite the sun and 79.5
/// metres past it. The one account of it: the wall is built round it, the
/// frame is hung in it, the columns are kept off the way to it, and the tests
/// read it back.
pub fn door(reaches: f32) -> (f32, f32) {
    let middle = at(reaches);

    (middle.x - DOOR_WIDE * 0.5, middle.x + DOOR_WIDE * 0.5)
}

/// The hall's near end wall, in two pieces with the way in between them.
///
/// Only the collider is in two pieces. What is drawn is one wall across the
/// whole of it, per spec 0011, which is the whole trick and is the one place
/// in this building where the two are meant to disagree.
pub fn near_wall(reaches: f32) -> Vec<Aabb> {
    let side = WALL + CABINET.x;
    let (from, to) = way();

    vec![
        Aabb::from_center_size(
            vec3((-side + from) * 0.5, crate::room::TALL * 0.5, reaches),
            vec3(from + side, crate::room::TALL, THICK),
        ),
        Aabb::from_center_size(
            vec3((to + side) * 0.5, crate::room::TALL * 0.5, reaches),
            vec3(side - to, crate::room::TALL, THICK),
        ),
    ]
}

/// Whether a point is in the space behind the wall.
///
/// Past the hall's own reach, which is where the wall with the hole in it
/// stands, and above the floors below. Nothing else in this building is out
/// there, so that is the whole question.
///
/// It is not the room's drawn footprint, which was the first answer and was
/// wrong by four orders of magnitude. [`SPAN`] and [`DEEP`] are what is built
/// and lit; what is in here goes a great deal further, because spec 0018 put
/// the solar system in at true scale and Saturn stands ten and a half
/// kilometres out. Bounded by the floor, the staged shot of Saturn came back
/// as the hall.
///
/// The floor laps three metres back under the hall's, per spec 0011, and that
/// lap is not in here: it is walked and not drawn, it is under the hall's
/// floor rather than in this room, and anybody standing over it is standing
/// in the hall.
pub fn inside(reaches: f32, at: Vec3) -> bool {
    at.y > -0.1 && at.z >= reaches
}

/// The middle of the floor, and how far it reaches from there.
///
/// Its near edge is the hall's far edge, meeting it rather than lapping over
/// it. Two floors at one height that overlap put one's end face in the middle
/// of the other's floor, and a body resting on a surface counts as touching
/// it: that is what jammed the walk across the garden where the baths below
/// happened to end.
pub fn at(reaches: f32) -> Vec3 {
    let side = WALL + CABINET.x;

    vec3(-side + SPAN * 0.5, 0.0, reaches + DEEP * 0.5)
}

pub fn half() -> Vec2 {
    vec2(SPAN * 0.5, DEEP * 0.5)
}

/// What a box of this space is, so the drawing does not have to count them.
///
/// Told apart here rather than by where they fall in the list. A list read by
/// index is two accounts of one thing, and this building knows what those do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Made {
    /// The floor and the lid, which are the same thing twice.
    Ground,
    Wall,
    Column,
    /// The frame round the door of spec 0017, which glows.
    Frame,
}

/// The floor, running `back` past the near wall and under the hall's own.
///
/// It is drawn with no lap and walked with one, and the two numbers are not
/// the same because they are answering different questions.
///
/// Walked: two floors at one height that meet edge to edge put a vertical
/// face at the surface you are walking on, and a body resting on a surface
/// counts as touching it, so the walk through the gap stopped dead four
/// centimetres short of the seam. Lapped a good way under, both faces are
/// deep inside the other slab and there is no edge at the join at all.
///
/// Drawn: lapped, its top and the hall's are one plane over five metres by
/// three, and the two fight. That patch is the floor you wake standing on,
/// and it came back as the carpet torn into bands with the dark of this room
/// showing between them. Nothing under the hall's floor needs drawing at all,
/// so the drawn slab stops where the hall's starts.
fn ground(middle: Vec3, back: f32) -> Aabb {
    Aabb::from_center_size(
        vec3(middle.x, -THICK * 0.5, middle.z - back * 0.5),
        vec3(SPAN, THICK, DEEP + back),
    )
}

/// Everything solid, and what each of it is: the floor, the lid, the four
/// walls, and the columns.
///
/// Four walls and not three. The hall's near end wall is in the way of the
/// south side only where the hall is, and the hall is five units of
/// forty-four.
pub fn built(reaches: f32) -> Vec<(Aabb, Made)> {
    let middle = at(reaches);
    let half = half();
    let side = WALL + CABINET.x;
    let (west, east) = (middle.x - half.x, middle.x + half.x);
    let (south, north) = (middle.z - half.y, middle.z + half.y);
    let gap = door(reaches);

    let mut out = vec![
        // the floor, drawn only as far as the hall's own floor reaches and
        // walked a good deal further. See `ground`.
        (ground(middle, 0.0), Made::Ground),
        // and a lid, because a space with no top is a space you are standing
        // outside of, which is the fault the hall was fixed for in spec 0001
        (
            Aabb::from_center_size(
                vec3(middle.x, HIGH + THICK * 0.5, middle.z),
                vec3(SPAN, THICK, DEEP),
            ),
            Made::Ground,
        ),
        // the walls go to the top of the lid and not to its underside. Stopped
        // at the underside they leave a slot the lid's own thickness, the
        // whole length of the room, and through it you see the lit side of the
        // building: it showed as a bright sliver along the ceiling.
        (
            Aabb::from_center_size(
                vec3(west - THICK * 0.5, (HIGH + THICK) * 0.5, middle.z),
                vec3(THICK, HIGH + THICK, DEEP + THICK * 2.0),
            ),
            Made::Wall,
        ),
        (
            Aabb::from_center_size(
                vec3(east + THICK * 0.5, (HIGH + THICK) * 0.5, middle.z),
                vec3(THICK, HIGH + THICK, DEEP + THICK * 2.0),
            ),
            Made::Wall,
        ),
        // the north wall, in three pieces round the door of spec 0017: the
        // stretch either side of it, and the head over it
        (
            Aabb::from_center_size(
                vec3(
                    (west + gap.0) * 0.5,
                    (HIGH + THICK) * 0.5,
                    north + THICK * 0.5,
                ),
                vec3(gap.0 - west, HIGH + THICK, THICK),
            ),
            Made::Wall,
        ),
        (
            Aabb::from_center_size(
                vec3(
                    (gap.1 + east) * 0.5,
                    (HIGH + THICK) * 0.5,
                    north + THICK * 0.5,
                ),
                vec3(east - gap.1, HIGH + THICK, THICK),
            ),
            Made::Wall,
        ),
        (
            Aabb::from_center_size(
                vec3(
                    middle.x,
                    (DOOR_HIGH + HIGH + THICK) * 0.5,
                    north + THICK * 0.5,
                ),
                vec3(DOOR_WIDE, HIGH + THICK - DOOR_HIGH, THICK),
            ),
            Made::Wall,
        ),
        // the frame round it, standing proud on the room's side and sunk into
        // the wall behind, so no face of it shares a plane with the wall's
        (
            Aabb::from_center_size(
                vec3(
                    gap.0 - FRAME_THICK * 0.5,
                    (DOOR_HIGH + FRAME_THICK) * 0.5,
                    north + (FRAME_SUNK - FRAME_PROUD) * 0.5,
                ),
                vec3(
                    FRAME_THICK,
                    DOOR_HIGH + FRAME_THICK,
                    FRAME_PROUD + FRAME_SUNK,
                ),
            ),
            Made::Frame,
        ),
        (
            Aabb::from_center_size(
                vec3(
                    gap.1 + FRAME_THICK * 0.5,
                    (DOOR_HIGH + FRAME_THICK) * 0.5,
                    north + (FRAME_SUNK - FRAME_PROUD) * 0.5,
                ),
                vec3(
                    FRAME_THICK,
                    DOOR_HIGH + FRAME_THICK,
                    FRAME_PROUD + FRAME_SUNK,
                ),
            ),
            Made::Frame,
        ),
        (
            Aabb::from_center_size(
                vec3(
                    middle.x,
                    DOOR_HIGH + FRAME_THICK * 0.5,
                    north + (FRAME_SUNK - FRAME_PROUD) * 0.5,
                ),
                vec3(
                    DOOR_WIDE + FRAME_THICK * 2.0,
                    FRAME_THICK,
                    FRAME_PROUD + FRAME_SUNK,
                ),
            ),
            Made::Frame,
        ),
        // and a sill across the bottom, so the opening is a closed rectangle
        // rather than three sides of one
        (
            Aabb::from_center_size(
                vec3(
                    middle.x,
                    SILL_HIGH * 0.5,
                    north + (FRAME_SUNK - FRAME_PROUD) * 0.5,
                ),
                vec3(
                    DOOR_WIDE + FRAME_THICK * 2.0,
                    SILL_HIGH,
                    FRAME_PROUD + FRAME_SUNK,
                ),
            ),
            Made::Frame,
        ),
        // the south side, which is the hall's near end wall for the width of
        // the hall and this room's own wall for the rest of it
        (
            Aabb::from_center_size(
                vec3((east + side) * 0.5, (HIGH + THICK) * 0.5, south),
                vec3(east - side, HIGH + THICK, THICK),
            ),
            Made::Wall,
        ),
    ];

    out.extend(columns(reaches).into_iter().map(|at_| {
        (
            Aabb::from_center_size(at_ + Vec3::Y * HIGH * 0.5, vec3(COLUMN, HIGH, COLUMN)),
            Made::Column,
        )
    }));

    out
}

/// The same, as what you cannot walk through, with the floor lapped back
/// under the hall's. See `ground`.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let middle = at(reaches);

    built(reaches)
        .into_iter()
        .map(|(box_, made)| match made {
            // the floor, which is the Ground that is not the lid
            Made::Ground if box_.max.y <= 0.0 => ground(middle, LAPS),
            _ => box_,
        })
        .collect()
}

/// How far apart the columns stand, how thick they are, and how far from the
/// way in the nearest of them may be.
///
/// A regular grid. Everywhere else in this building the rule is that nothing
/// lines up, because nothing in a furnished room ever does. This is not a
/// furnished room, and the regularity is what makes it feel like nobody is
/// meant to be here.
pub const STRIDE: f32 = 7.5;
pub const COLUMN: f32 = 0.72;
pub const CLEAR: f32 = 2.2;

/// How far along the line from the sun to the way out a point lies, and how
/// far off it, both in metres.
///
/// Its own function because `columns` keeps the line clear and
/// `tests::nothing_stands_on_the_line` reads the same line back to check it,
/// and two accounts of one line is the fault this building keeps paying for.
/// `along` runs from nought at the sun to the length of it at the door, and
/// is negative behind the sun.
pub fn down_the_line(reaches: f32, point: Vec2) -> (f32, f32) {
    let (from, to) = way();

    from_the_sun(reaches, vec2((from + to) * 0.5, reaches), point)
}

/// The same, for the line from the sun to the door at the far end, per spec
/// 0017.
///
/// A second line to keep clear. The grid puts a column 0.75 off the room's
/// own axis, which is three quarters of a metre from the middle of a doorway
/// 1.5 wide: without this you walk out of the light, find the door, and the
/// last thing between you and it is a pillar.
pub fn down_the_door_line(reaches: f32, point: Vec2) -> (f32, f32) {
    let middle = at(reaches);
    let half = half();

    from_the_sun(reaches, vec2(middle.x, middle.z + half.y), point)
}

/// Both of the above. How far along the line from the sun to `target` a point
/// lies, and how far off it.
fn from_the_sun(reaches: f32, target: Vec2, point: Vec2) -> (f32, f32) {
    let middle = at(reaches);
    let sun = vec2(middle.x, middle.z);
    let run = target - sun;
    let length = run.length();

    if length < 1e-6 {
        return (0.0, (point - sun).length());
    }

    let heading = run / length;
    let v = point - sun;
    let along = v.dot(heading);

    (along, (v - heading * along).length())
}

/// Where the columns stand.
///
/// They are what gives the size away. An empty floor going off into the dark
/// is a corridor with the lights off, and the only way to read a distance is
/// to see the same thing at several of them.
///
/// Three things are kept clear of: the way in, every orbit, and the line the
/// bodies stand on lined up. All three with `CLEAR`, and the grid is what is
/// left.
pub fn columns(reaches: f32) -> Vec<Vec3> {
    let middle = at(reaches);
    let half = half();
    let (from, to) = way();
    let door = vec2((from + to) * 0.5, reaches);

    let across = (SPAN / STRIDE) as i32;
    let along = (DEEP / STRIDE) as i32;
    let mut out = Vec::new();

    for i in 0..across {
        for j in 0..along {
            let at_ = vec2(
                middle.x - half.x + STRIDE * (i as f32 + 0.5),
                middle.z - half.y + STRIDE * (j as f32 + 0.5),
            );

            // nothing in the way in. One column there and the whole thing is a
            // wall you walked into twice.
            if at_.distance(door) < CLEAR {
                continue;
            }

            // and nothing standing on an orbit, per spec 0015. The grid stays,
            // because a regular grid in a vast dark room is what says nobody
            // is meant to be here, and a column between two orbits is better
            // than no column at all. A column *on* an orbit is a planet
            // flying through a pillar twice a year.
            let from_the_sun = at_.distance(vec2(middle.x, middle.z));

            if crate::orrery::PLANETS
                .iter()
                .any(|planet| (from_the_sun - planet.far()).abs() < CLEAR)
            {
                continue;
            }

            // and nothing on the line the bodies stand on, per spec 0015.
            // Lined up, every body is strung along the diagonal from the sun
            // to the way out, and a forty five degree line through a square
            // grid passes near a grid point every stride and a half. Nine
            // stand on it, from a fifth of a metre off to two thirds, and one
            // of them is four metres inside the door: from the door the whole
            // model was behind a pillar, which is most of why the planets
            // were hard to find.
            //
            // The width is not a judgment. The nine on the line are all
            // within 0.7 of it and the nearest one that is not is at 4.46, so
            // anything from one metre to four drops exactly the same nine of
            // the three hundred and sixty eight, and `CLEAR` is reused rather
            // than a fourth number invented. See
            // `tests::the_line_is_cleared_by_a_wide_margin`.
            let (down, off) = down_the_line(reaches, at_);

            if (0.0..=run_of_the_line(reaches)).contains(&down) && off < CLEAR {
                continue;
            }

            // and nothing on the way from the sun to the door, per spec 0017,
            // which is the other line you walk in this room
            let (down, off) = down_the_door_line(reaches, at_);

            if (0.0..=run_to_the_door()).contains(&down) && off < CLEAR {
                continue;
            }

            out.push(vec3(at_.x, 0.0, at_.y));
        }
    }

    out
}

/// How long the line from the sun to the way out is.
///
/// A hundred and eleven and a half metres, of which the planets take the
/// first seventy nine. The whole of it is cleared and not just the planets'
/// part, because the columns past Neptune are the ones nearest the door and
/// so the ones that stand in front of everything.
pub fn run_of_the_line(reaches: f32) -> f32 {
    let middle = at(reaches);
    let (from, to) = way();

    (vec2((from + to) * 0.5, reaches) - vec2(middle.x, middle.z)).length()
}

/// How far the door at the far end is from the sun.
///
/// Seventy nine and a half metres, which is half the room. The camera's far
/// plane is a hundred, so the door is there to be seen from the light and is
/// not there to be seen from the way in, 177 metres off. See spec 0017.
///
/// No `reaches`, unlike `run_of_the_line`. The sun sits at the middle of this
/// room and the door is in the middle of its north wall, so the run between
/// them is half the room's depth whatever the hall does.
pub fn run_to_the_door() -> f32 {
    half().y
}

/// How far apart the lamps are, how far each throws and how hard.
///
/// Far apart and faint, and cool where every other light in this building is
/// warm. Most of this is not lit at all, which is the point: there is no fog
/// in this engine and none is wanted, because a point light falls off and the
/// far end goes black on its own.
pub const LAMPS: f32 = 15.0;
pub const LAMP_RANGE: f32 = 9.5;
pub const LAMP_LIT: f32 = 0.42;
pub const LAMP_COLOUR: Vec3 = vec3(0.72, 0.78, 0.92);

/// Where the lamps are, on the ceiling.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    let middle = at(reaches);
    let half = half();
    let across = (SPAN / LAMPS).max(1.0) as i32;
    let along = (DEEP / LAMPS).max(1.0) as i32;
    let mut out = Vec::new();

    for i in 0..across {
        for j in 0..along {
            out.push(vec3(
                middle.x - half.x + LAMPS * (i as f32 + 0.5),
                HIGH - 0.08,
                middle.z - half.y + LAMPS * (j as f32 + 0.5),
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use blitzkit::camera::Camera;
    use blitzkit::renderer::coplanar::Fights;

    /// Spec 0015: no column stands on the line the bodies are strung along.
    ///
    /// The whole line, sun to door, and not only the seventy nine metres the
    /// planets take: the ones past Neptune are nearest the door and so the
    /// ones that stand in front of everything. Nine of the three hundred and
    /// sixty eight were on it before this, the nearest four metres inside the
    /// door, and from the door the model was behind a pillar.
    ///
    /// Cleared to the centre and then again to the box, because `CLEAR` is a
    /// distance between points and a column is 0.72 across.
    #[test]
    fn nothing_stands_on_the_line() {
        let reaches = room().reaches;
        let run = run_of_the_line(reaches);

        for column in columns(reaches) {
            let (along, off) = down_the_line(reaches, vec2(column.x, column.z));

            if !(0.0..=run).contains(&along) {
                continue;
            }

            assert!(
                off >= CLEAR,
                "a column at {:.2} {:.2} stands {:.2}m off the line, {:.2}m down it",
                column.x,
                column.z,
                off,
                along
            );
            assert!(
                off > COLUMN * 0.5,
                "a column at {:.2} {:.2} is on the line itself",
                column.x,
                column.z
            );
        }
    }

    /// Spec 0015: and the width that clears it is not a judgment call.
    ///
    /// The nine that were on the line were all within seven tenths of a metre
    /// of it and the nearest one still standing is 4.46 metres off, so every
    /// width from one metre to four drops exactly the same nine. `CLEAR` is
    /// 2.2 and sits in the middle of that band, reused from the other two
    /// rules rather than invented as a fourth number.
    ///
    /// This is the test to look at if somebody wants to move `CLEAR`: it says
    /// how much room there is to move it in, and it fails when there is none.
    #[test]
    fn the_line_is_cleared_by_a_wide_margin() {
        let reaches = room().reaches;
        let run = run_of_the_line(reaches);

        let nearest = columns(reaches)
            .into_iter()
            .filter_map(|column| {
                let (along, off) = down_the_line(reaches, vec2(column.x, column.z));

                (0.0..=run).contains(&along).then_some(off)
            })
            .fold(f32::MAX, f32::min);

        assert!(
            nearest > CLEAR * 1.8,
            "the nearest column off the line is {:.2}m, against a clearance of {:.2}m,              so the clearance is doing fine work and this room has got crowded",
            nearest,
            CLEAR
        );

        // and clearing it cost the room almost nothing. The grid is what says
        // how big this place is, so a rule that took a tenth of it would be
        // buying the view with the thing the view is of.
        let kept = columns(reaches).len();
        let grid = ((SPAN / STRIDE) as usize) * ((DEEP / STRIDE) as usize);

        assert!(
            kept * 10 >= grid * 7,
            "{} columns left standing of a grid of {}",
            kept,
            grid
        );
    }

    /// Spec 0015: and the way to each body is clear from the doorway.
    ///
    /// The property the rule is for, said as what you see rather than as the
    /// arithmetic that gets it. Lined up, every body sits on the line, so a
    /// clear line is a clear view of all sixteen. Says nothing about the
    /// orbiting arrangement, where the bodies are off the line by their own
    /// distance from the sun and a column between two orbits is the whole
    /// point of the grid.
    #[test]
    fn every_body_is_in_view_from_the_doorway() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let (from, to) = way();
        let sun = vec3(middle.x, crate::orrery::PLANE, middle.z);
        let door = vec3((from + to) * 0.5, crate::orrery::PLANE, reaches);
        let toward = (door - sun).normalize();
        let boxes: Vec<_> = columns(reaches)
            .into_iter()
            .map(|at_| {
                Aabb::from_center_size(vec3(at_.x, HIGH * 0.5, at_.z), vec3(COLUMN, HIGH, COLUMN))
            })
            .collect();

        for planet in crate::orrery::PLANETS.iter() {
            let body = sun + crate::orrery::stands(planet, toward, 0.0, 0.0, 0.0);

            // a hundred steps down the way from the door to it, and none of
            // them inside a column
            for n in 0..=100 {
                let at_ = door + (body - door) * (n as f32 / 100.0);

                for (column, box_) in columns(reaches).iter().zip(&boxes) {
                    assert!(
                        !box_.contains_point(at_),
                        "the way to {} runs through the column at {:.2} {:.2}",
                        planet.body.name,
                        column.x,
                        column.z
                    );
                }
            }
        }
    }

    /// The room these tests walk, taken from the room rather than written down.
    fn room() -> crate::room::Room {
        crate::room::Room::of(
            (0..12)
                .map(|n| {
                    crate::cabinet::Cabinet::found(
                        &format!("game{}", n),
                        std::path::Path::new("/nowhere"),
                    )
                })
                .collect(),
        )
    }

    /// Walks a body your size from one place to another and says where it got
    /// to. The whole building's boxes, because the wall it goes through is the
    /// hall's.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid = room().solid();
        let step = 1.0 / 60.0;
        let (mut at_, mut falling, mut owed) = (from, 0.0f32, 0.0f32);

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at_.x, 0.0, to.z - at_.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at_, wish, falling, crate::RADIUS, step, &solid, owed <= 0.0);
            owed = (owed - crate::walk::CLIMBS * step).max(0.0) + (next.y - at_.y).max(0.0);
            at_ = next;
            falling = fell;
        }

        at_
    }

    /// Spec 0011: you can walk through the wall where the gap is.
    ///
    /// The whole room is this one walk. Everything else in here is scenery for
    /// it.
    #[test]
    fn you_can_walk_through_where_the_gap_is() {
        let reaches = room().reaches;
        let (from, to) = way();
        let on = (from + to) * 0.5;
        let got = walked(
            vec3(on, 0.0, reaches - 2.0),
            vec3(on, 0.0, reaches + 4.0),
            3.0,
        );

        assert!(got.z > reaches + THICK, "it did not get through: {:?}", got);
    }

    /// Spec 0015: the ceiling grid alone never fills the engine's lamp
    /// list, from anywhere in the room, looking any way.
    ///
    /// A hundred lamps over a hundred and fifty nine metres against an
    /// engine that carries sixty four. That overflow is what made the sun
    /// get dropped, and culling by `Camera::frustum` is what ends it: a lamp
    /// gives nothing past its range, so one whose reach does not meet the
    /// picture lights nothing you can see.
    ///
    /// The grid only, which is the honest limit of this test. The building
    /// pushes thirty three other fittings and they are not counted here, so
    /// the headroom below is not the whole story; what covers the rest is
    /// the engine saying so once a frame when a cap is hit, and the ranking
    /// that decides what spills.
    ///
    /// Sampled over the floor and round the compass, because the worst case
    /// is a direction and not a place.
    #[test]
    fn the_ceiling_grid_stays_under_the_lamps_the_engine_carries() {
        use blitzkit::lighting::MAX_POINT_LIGHTS;

        let reaches = room().reaches;
        let middle = at(reaches);
        let half_room = half();
        let lamps = lamps(reaches);
        let mut worst = (0usize, Vec3::ZERO, 0.0f32);

        for across in 0..7 {
            for along in 0..7 {
                let eye = vec3(
                    middle.x - half_room.x + SPAN * (across as f32 + 0.5) / 7.0,
                    crate::EYE,
                    middle.z - half_room.y + DEEP * (along as f32 + 0.5) / 7.0,
                );

                for turn in 0..8 {
                    let yaw = turn as f32 / 8.0 * std::f32::consts::TAU;
                    let mut camera = blitzkit::camera::Camera::new();
                    camera.position = eye;
                    camera.target = eye + vec3(yaw.sin(), 0.0, -yaw.cos());
                    let seen = camera.frustum();
                    let kept = lamps
                        .iter()
                        .filter(|at_| seen.holds_sphere(**at_, LAMP_RANGE))
                        .count();

                    if kept > worst.0 {
                        worst = (kept, eye, yaw);
                    }
                }
            }
        }

        assert!(
            worst.0 < MAX_POINT_LIGHTS,
            "{} of the grid are in view at {:?} looking {:.2}, cap {}",
            worst.0,
            worst.1,
            worst.2,
            MAX_POINT_LIGHTS
        );

        // and it is not passing by culling everything: standing in the
        // middle of the floor you should see a good few
        assert!(worst.0 > 8, "only {} lamps were ever in view", worst.0);
    }

    /// Spec 0015: you can walk from the way in to the sun, and round it.
    ///
    /// A hundred and eleven metres of it, which is what the room became when
    /// the Moon took over setting the orbit scale. Walked rather than
    /// measured, with the game's own `walk` against the whole building's
    /// boxes, because the thing being checked is that the corridor
    /// `columns` clears is actually walkable and not merely clear on paper.
    ///
    /// Then round the sun, which has no collider: it is a metre of light
    /// that swallows you on the way past, and that is the right thing to
    /// happen. A body that stopped at it would be a body that had hit the
    /// sun.
    ///
    /// Thirty seconds of walking for a twenty seven second walk, so a body
    /// that arrives has a little to spare and one that is being held up
    /// anywhere plainly does not.
    #[test]
    fn you_can_walk_from_the_way_in_to_the_sun_and_round_it() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let sun = vec3(middle.x, 0.0, middle.z);
        let (from, to) = way();
        let door = vec3((from + to) * 0.5, 0.0, reaches + 2.0);

        let got = walked(door, sun, 30.0);
        let left = vec2(got.x - sun.x, got.z - sun.z).length();

        assert!(
            left < 2.0,
            "it got {:.1} m from the sun, with {:.1} m to walk",
            left,
            (sun - door).length()
        );

        // and out the far side, which is through where the sun is drawn
        let past = sun + (sun - door).normalize() * 6.0;
        let went = walked(got, past, 8.0);

        assert!(
            vec2(went.x - past.x, went.z - past.z).length() < 2.0,
            "it stopped at {:?} rather than going on past the sun",
            went
        );
    }

    /// Spec 0011: and not where it is not, which is most of the wall.
    ///
    /// Swept across the whole of it, because a wall with one hole in it is a
    /// wall everywhere else and that is what makes the hole worth finding.
    #[test]
    fn the_rest_of_the_wall_is_a_wall() {
        let reaches = room().reaches;
        let side = WALL + CABINET.x;
        let (from, to) = way();

        for step in 0..=20 {
            let on = -side + (side * 2.0) * step as f32 / 20.0;

            // the gap itself, and a body's width either side of it, where you
            // get through by sliding rather than by walking straight
            if on > from - crate::RADIUS - 0.1 && on < to + crate::RADIUS + 0.1 {
                continue;
            }

            let got = walked(
                vec3(on, 0.0, reaches - 2.0),
                vec3(on, 0.0, reaches + 4.0),
                3.0,
            );

            assert!(
                got.z < reaches - THICK,
                "the wall let a body through at x {:.2}: {:?}",
                on,
                got
            );
        }
    }

    /// Spec 0011: the way in is not in front of you when you wake.
    ///
    /// You wake on the middle of the hall facing away from this wall. A gap on
    /// the middle is found by everybody on their first turn round, and the
    /// whole of the room is that it is not.
    #[test]
    fn the_way_in_is_not_in_front_of_you() {
        let (from, to) = way();

        assert!(
            from > crate::RADIUS || to < -crate::RADIUS,
            "the way in is straight behind where you wake: {:.2} to {:.2}",
            from,
            to
        );
        // and not so far over that it is in a corner nobody walks into
        let side = WALL + CABINET.x;
        assert!(
            from > -side + 0.5 && to < side - 0.5,
            "the way in is in a corner: {:.2} to {:.2}",
            from,
            to
        );
    }

    /// Spec 0011: the space is closed but for that gap, and spec 0017's door.
    ///
    /// Walked at every wall from inside, because a room this big with one way
    /// out has a lot of wall to get wrong.
    ///
    /// The north wall is walked at twenty metres off the room's axis rather
    /// than along it. Along it is the door, and walking out of the door is
    /// `the_door_can_be_walked_through`, which wants the opposite answer.
    #[test]
    fn nothing_else_gets_out() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let inside = vec3(middle.x, 0.0, middle.z);

        for (what, to) in [
            (
                "the west wall",
                vec3(middle.x - half.x - 6.0, 0.0, middle.z),
            ),
            (
                "the east wall",
                vec3(middle.x + half.x + 6.0, 0.0, middle.z),
            ),
            (
                "the north wall",
                vec3(middle.x + 20.0, 0.0, middle.z + half.y + 6.0),
            ),
            (
                "the south wall",
                vec3(middle.x, 0.0, middle.z - half.y - 6.0),
            ),
        ] {
            let got = walked(inside, to, 20.0);
            let off = vec2(got.x - middle.x, got.z - middle.z).abs();

            assert!(
                off.x < half.x && off.y < half.y,
                "it got out through {} to {:?}",
                what,
                got
            );
        }
    }

    /// Spec 0011: it keeps out of every other room.
    #[test]
    fn it_keeps_out_of_every_other_room() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let garden = crate::garden::at(reaches);
        let gap = crate::garden::half();

        let apart = |a: Vec2, ah: Vec2, b: Vec2, bh: Vec2| {
            (a.x - b.x).abs() >= ah.x + bh.x - 1e-4 || (a.y - b.y).abs() >= ah.y + bh.y - 1e-4
        };

        assert!(
            apart(
                vec2(middle.x, middle.z),
                half,
                vec2(garden.x, garden.z),
                gap
            ),
            "it is over the garden"
        );

        // the hall and the nook are the other side of the wall it is behind
        assert!(
            middle.z - half.y >= reaches - 1e-4,
            "it comes through into the hall: its near edge is at {:.2} and the \
             hall's near end wall is at {:.2}",
            middle.z - half.y,
            reaches
        );
    }

    /// Spec 0011: its floor laps under the hall's rather than meeting it.
    ///
    /// The other way round from how this was first written, and the walk is
    /// what settled it. Two floors at one height that meet edge to edge put a
    /// vertical face at the surface you are walking on, and a body resting on a
    /// surface counts as touching it: the walk through the gap stopped dead
    /// four centimetres short of the seam. Lapped a good way under, both end
    /// faces are deep inside the other slab and the join is nothing at all.
    ///
    /// The lap has to be a good way, not a hair. A face just under the join is
    /// still a face you cross at the walking surface, only harder to find.
    #[test]
    fn the_floors_lap_rather_than_meet() {
        let room = room();
        let hall = room.floors()[0];
        // the floor as walked, found under the foot of somebody who has just
        // stepped through the gap, rather than by its place in the list
        let (from, to) = way();
        let foot = vec3((from + to) * 0.5, -0.05, hall.max.z - crate::RADIUS);
        let ours = solid(room.reaches)
            .into_iter()
            .find(|box_| box_.contains_point(foot))
            .expect("a floor under the way in");

        assert!(
            (ours.max.y - hall.max.y).abs() < 1e-4,
            "the floors are at different heights"
        );

        let over = hall.max.z - ours.min.z;
        assert!(
            over > crate::RADIUS * 2.0,
            "they lap by {:.2} and a body is {:.2} across",
            over,
            crate::RADIUS * 2.0
        );
        // and the lap is under the hall rather than the hall under it: this
        // floor's near face has to be inside the hall's slab in x as well, or
        // it is a face in the open after all
        assert!(
            ours.min.x <= hall.min.x + 1e-4,
            "this floor starts east of the hall's, so its near face is in the open"
        );
    }

    /// The lap is walked and not drawn.
    ///
    /// Drawn, it put this room's floor and the hall's in one plane over five
    /// metres by three, and the patch they fight over is the floor you wake
    /// standing on: the carpet came back torn into bands with the dark of
    /// this room showing between them.
    #[test]
    fn nothing_drawn_lies_in_the_halls_floor() {
        let room = room();

        for hall in room.floors() {
            for (box_, made) in built(room.reaches) {
                let level = (box_.max.y - hall.max.y).abs() < 1e-4;
                let over = box_.min.x < hall.max.x - 1e-4
                    && box_.max.x > hall.min.x + 1e-4
                    && box_.min.z < hall.max.z - 1e-4
                    && box_.max.z > hall.min.z + 1e-4;

                assert!(
                    !(level && over),
                    "a {made:?} drawn at {box_:?} tops out in the hall's floor plane and \
                     laps over it by {:.2} by {:.2}",
                    box_.max.x.min(hall.max.x) - box_.min.x.max(hall.min.x),
                    box_.max.z.min(hall.max.z) - box_.min.z.max(hall.min.z),
                );
            }
        }
    }

    /// Spec 0048 of the engine, on the fault that spec was written for.
    ///
    /// This room's floor and the hall's were in one plane over five metres by
    /// three for as long as the lap was drawn, and the carpet came back torn
    /// into bands. The engine's coplanar check reads boxes and says so, which
    /// is the point of it: nobody had to walk here.
    ///
    /// It is run over the whole building's boxes and not over this room's,
    /// because the check asks whether a face looks into a room or out at the
    /// weather, and only the rest of the building can answer that. Over this
    /// room alone the lap came back as nothing at all, which is the third time
    /// in one afternoon that handing it a fragment gave a wrong answer.
    ///
    /// Its own control, both ways round. The lap is in the colliders, where it
    /// belongs and where the check finds it at the size it really is; drawn
    /// without it, which is what ships, there is nothing in the hall's floor
    /// plane at all.
    #[test]
    fn the_engines_check_catches_the_lap_and_passes_what_ships() {
        let room = room();
        let middle = at(room.reaches);
        let hall = room.floors()[0];

        // the whole building, with this room's floor as given. The walked list
        // laps and the drawn one does not, which is the one difference.
        let whole = |back: f32| {
            let lapped = ground(middle, LAPS);
            let mut boxes: Vec<Aabb> = room
                .solid()
                .into_iter()
                .filter(|box_| *box_ != lapped)
                .collect();

            boxes.push(ground(middle, back));

            // this game's own camera, which is what decides how fine a gap
            // the depth buffer still blurs at a given distance. The arcade
            // leaves the near and far planes as the engine sets them.
            let looking = blitzkit::renderer::depth::Depth::of(&Camera::default());

            Fights::of(boxes, 0, 0, &looking)
        };

        // a fight in the hall's own floor plane, over the hall's own floor,
        // which is the patch you wake up standing on
        let in_the_halls_floor = |report: &Fights| {
            report
                .fights
                .iter()
                .find(|fight| {
                    fight.axis == 1
                        && fight.facing > 0.0
                        && (fight.patch.min.y - hall.max.y).abs() < 1e-4
                        && fight.patch.min.x < hall.max.x - 1e-4
                        && fight.patch.max.x > hall.min.x + 1e-4
                })
                .copied()
        };

        let caught = whole(LAPS);

        let found = in_the_halls_floor(&caught).unwrap_or_else(|| {
            panic!(
                "the lap lies in the hall's floor plane and the check missed it; \
                 the biggest it found was {:?}",
                caught.fights.first()
            )
        });

        assert!(
            (found.area() - 5.4 * LAPS).abs() < 0.1,
            "the lap is 5.40 by {:.2} and came back as {:.2} square metres, {}",
            LAPS,
            found.area(),
            found
        );

        let ships = whole(0.0);

        assert!(
            in_the_halls_floor(&ships).is_none(),
            "what ships draws no lap and the check found one anyway: {:?}",
            in_the_halls_floor(&ships)
        );

        // and it survives the filter, which is the half that can go wrong
        // without anybody noticing. Spec 0049 of the engine throws out every
        // pair nobody can get in front of, and this patch is the floor you
        // wake up standing on: if the filter loses that, it loses anything.
        let reach = blitzkit::reach::Reach::found(
            vec3(0.0, crate::EYE, 0.0),
            blitzkit::reach::Body {
                radius: crate::RADIUS,
                rise: 0.25,
                tall: 4.0,
            },
            &caught.boxes,
            room.outline().expanded(Vec3::ONE),
            blitzkit::reach::STEP,
        );

        assert!(
            !reach.is_empty(),
            "the fill reached nothing, so the next assertion proves nothing"
        );
        assert!(
            in_the_halls_floor(&caught.only_seen_from(&reach)).is_some(),
            "the filter threw away the floor you wake up standing on"
        );
    }

    /// Spec 0011: no column stands in the way in.
    ///
    /// One there and the whole thing is a wall you walked into twice.
    #[test]
    fn nothing_stands_in_the_way_in() {
        let reaches = room().reaches;
        let (from, to) = way();
        let door = vec2((from + to) * 0.5, reaches);

        for at_ in columns(reaches) {
            let off = vec2(at_.x, at_.z).distance(door);

            assert!(
                off > COLUMN * 0.5 + crate::RADIUS,
                "a column stands in the way in, {:.2} from it",
                off
            );
        }
    }

    /// Spec 0011: you can get back out.
    ///
    /// The way in is the way out and there is no other. A space you can get
    /// into and not out of is not a secret, it is a hole.
    #[test]
    fn you_can_walk_back_out() {
        let reaches = room().reaches;
        let (from, to) = way();
        let on = (from + to) * 0.5;
        let got = walked(
            vec3(on, 0.0, reaches + 3.0),
            vec3(on, 0.0, reaches - 4.0),
            4.0,
        );

        assert!(got.z < reaches - THICK, "it could not get out: {:?}", got);
    }

    /// Where the middle of the door is, at the floor, for the walks below.
    fn doorstep(reaches: f32) -> Vec3 {
        let middle = at(reaches);

        vec3(middle.x, 0.0, middle.z + half().y + THICK * 0.5)
    }

    /// Spec 0017: the far wall is a wall everywhere except the opening.
    ///
    /// Walked at head height and at knee height both, because a head that
    /// passes where a knee does not is a lintel left in, and a knee that
    /// passes where a head does not is the head of the wall left out.
    #[test]
    fn the_far_wall_is_solid_either_side_of_the_door() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let north = middle.z + half().y;
        let (from, to) = door(reaches);

        for off in [-40.0, -8.0, -2.0, -1.4, 1.4, 2.0, 8.0, 40.0] {
            let start = vec3(middle.x + off, 0.0, north - 4.0);
            let got = walked(start, start + Vec3::Z * 12.0, 6.0);

            assert!(
                got.z < north,
                "the wall let a body through at {} off the door, which runs {} to {}",
                off,
                from,
                to
            );
        }
    }

    /// Spec 0017: and you can walk through the opening onto the landing.
    #[test]
    fn the_door_can_be_walked_through() {
        let reaches = room().reaches;
        let step = doorstep(reaches);
        let out = step + Vec3::Z * 8.0;
        let got = walked(step - Vec3::Z * 6.0, out, 8.0);

        assert!(
            got.distance(out) < 0.6,
            "a body aimed at the landing got to {} rather than {}",
            got,
            out
        );
    }

    /// Spec 0017: and back in again, which is a different walk.
    ///
    /// Different because the seam between the two floors is crossed the other
    /// way, and because the frame's jambs stand proud on the room's side and
    /// so are met on the way in and not on the way out.
    #[test]
    fn the_landing_leads_back_into_the_room() {
        let reaches = room().reaches;
        let step = doorstep(reaches);
        let back = step - Vec3::Z * 8.0;
        let got = walked(step + Vec3::Z * 6.0, back, 8.0);

        assert!(
            got.distance(back) < 0.6,
            "a body aimed back into the room got to {} rather than {}",
            got,
            back
        );
    }

    /// Spec 0017: the sun is what shows you the door.
    ///
    /// Not a judgment, two distances against the camera's far plane. From the
    /// way in the door is 177 metres off and is not drawn; from the sun it is
    /// 79.5 and is.
    #[test]
    fn the_sun_is_what_shows_you_the_door() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let far = Camera::new().far;
        let (from, to) = way();
        let entrance = vec2((from + to) * 0.5, reaches);
        let sun = vec2(middle.x, middle.z);
        let at_ = vec2(middle.x, middle.z + half().y);

        assert!(
            entrance.distance(at_) > far,
            "the door is {} from the way in, inside a far plane of {}",
            entrance.distance(at_),
            far
        );
        assert!(
            sun.distance(at_) < far,
            "the door is {} from the sun, outside a far plane of {}",
            sun.distance(at_),
            far
        );
    }

    /// Spec 0017: and nothing stands between the two.
    ///
    /// Cleared to the centre and then again to the box, the way
    /// `nothing_stands_on_the_line` does, because `CLEAR` is a distance
    /// between points and a column is 0.72 across.
    #[test]
    fn the_way_to_the_door_is_clear_of_columns() {
        let reaches = room().reaches;
        let run = run_to_the_door();

        for column in columns(reaches) {
            let (along, off) = down_the_door_line(reaches, vec2(column.x, column.z));

            if !(0.0..=run).contains(&along) {
                continue;
            }

            assert!(
                off - COLUMN * 0.5 > 1.0,
                "a column at {} stands {} off the way to the door, {} along it",
                column,
                off,
                along
            );
        }
    }

    /// Spec 0017: the frame gives off light and casts none.
    ///
    /// Which in this engine is one fact: a colour past one is a surface
    /// giving off more than it is given, and `Instance::glows` is what keeps
    /// such a thing out of the shadow pass.
    #[test]
    fn the_frame_gives_off_light_and_casts_none() {
        let colour = crate::aim::BEHIND_FRAME;

        assert!(
            colour.x > 1.0 && colour.y > 1.0 && colour.z > 1.0,
            "the frame's colour is {}, which does not glow",
            colour
        );
        // and it is the lamps' colour and not another one, per spec 0017
        let lit = LAMP_COLOUR * (colour.x / LAMP_COLOUR.x);

        assert!(
            (lit - vec3(colour.x, colour.y, colour.z)).length() < 1e-3,
            "the frame is {} where the lamps are {}",
            colour,
            LAMP_COLOUR
        );
        assert!(
            built(room().reaches)
                .iter()
                .any(|(_, made)| *made == Made::Frame),
            "the frame is not built"
        );
    }

    /// Spec 0017: the sill is nothing to trip on.
    ///
    /// Measured rather than asserted about its height. The same walk, for the
    /// same time, over open floor and through the door: a sill that cost
    /// anything would show as the door walk falling behind.
    #[test]
    fn the_sill_costs_nothing_to_cross() {
        let reaches = room().reaches;
        let step = doorstep(reaches);
        let open = step - Vec3::X * 30.0;
        let through = walked(step - Vec3::Z * 4.0, step + Vec3::Z * 4.0, 2.0);
        let across = walked(open - Vec3::Z * 4.0, open + Vec3::Z * 4.0, 2.0);
        let (one, other) = (through.z - (step.z - 4.0), across.z - (open.z - 4.0));

        assert!(
            one > other - 0.02,
            "two seconds over open floor covers {} and through the door {}",
            other,
            one
        );
    }

    /// Where a point of the track's quad falls in the room, which is the trip
    /// the weave makes in reverse.
    fn back(track: &Track, on: Vec2) -> Vec2 {
        vec2(
            track.at.x + (on.x - 0.5) * track.size.x,
            track.at.z + (on.y - 0.5) * track.size.z,
        )
    }

    /// Spec 0020: the wall end of the track is in the hole, and the aisle end
    /// is out where somebody walking the hall actually walks.
    ///
    /// Round the trip rather than off the constants, because the constants
    /// cannot drift from themselves. What can drift is the mapping into the
    /// quad's own units: a quad moved or resized without the two ends moving
    /// with it is a track pointing at a patch of wall.
    #[test]
    fn the_worn_track_ends_at_the_hole() {
        let reaches = room().reaches;
        let track = track(reaches);
        let (from, to) = way();
        let (at_wall, at_aisle) = (back(&track, track.wall), back(&track, track.aisle));

        assert!(
            at_wall.x > from && at_wall.x < to,
            "the track meets the wall at x {:.3}, and the hole is {:.3} to {:.3}",
            at_wall.x,
            from,
            to
        );
        assert!(
            (at_wall.y - (reaches - THICK * 0.5)).abs() < 1e-3,
            "the track meets the wall at z {:.3} and the wall's face is at {:.3}",
            at_wall.y,
            reaches - THICK * 0.5
        );

        // and it comes from outside the hole's own width, so it crosses the
        // floor rather than sitting in the doorway
        assert!(
            at_aisle.x > to,
            "the track starts at x {:.3}, which is inside the hole's {:.3} to {:.3}",
            at_aisle.x,
            from,
            to
        );
        assert!(
            at_wall.y - at_aisle.y > 1.0,
            "the track is only {:.3} long down the room",
            at_wall.y - at_aisle.y
        );
    }

    /// Spec 0020: the wear fades out inside its own quad on three sides, so
    /// nothing drawn has a straight edge and the rectangle it is painted on is
    /// not a thing you can see.
    ///
    /// Three sides. The fourth is at the wall and is straight because the wall
    /// is standing on it.
    ///
    /// The two ends are enough to say it of the whole curve. A quadratic lies
    /// inside the triangle of its three points, and `carpet::worn` puts its
    /// middle point at one end's u and between the two ends' v, so the curve
    /// never leaves the box the ends make. That is
    /// `carpet::tests::the_wear_keeps_between_its_two_ends`.
    #[test]
    fn the_worn_track_fades_out_before_the_edge_of_its_quad() {
        let track = track(room().reaches);
        // how far the wear reaches, down the room and across it. The two are
        // not the same number: `wide` and `fade` are shares of the quad's
        // length, and across it a share is a different distance.
        let reach = track.wide + track.fade;
        let across = reach / track.shape;

        for (which, on) in [("the aisle", track.aisle), ("the wall", track.wall)]
            .iter()
            .copied()
        {
            for (side, room, reach) in [
                ("the west edge", on.x, across),
                ("the east edge", 1.0 - on.x, across),
                ("down the room", on.y, reach),
            ]
            .iter()
            .copied()
            {
                assert!(
                    room > reach,
                    "{} end is {:.4} off {} and the wear reaches {:.4}",
                    which,
                    room,
                    side,
                    reach
                );
            }
        }
    }

    /// Spec 0020: all of it lies on the hall's own carpet.
    ///
    /// Both ways. Off the sides it would lie on nothing at all, and out past
    /// the near wall it would come up on the floor of the dark room, where
    /// there is no carpet to be worn and no reason for a patch of one.
    #[test]
    fn the_worn_track_lies_on_the_halls_carpet() {
        let room = room();
        let track = track(room.reaches);
        let carpet = room.floors()[0];
        let (at, half) = (track.at, vec2(track.size.x, track.size.z) * 0.5);

        assert!(
            at.x - half.x > carpet.min.x - 1e-3 && at.x + half.x < carpet.max.x + 1e-3,
            "the track runs x {:.3} to {:.3} and the carpet {:.3} to {:.3}",
            at.x - half.x,
            at.x + half.x,
            carpet.min.x,
            carpet.max.x
        );
        assert!(
            at.z + half.y < carpet.max.z + 1e-3,
            "the track reaches z {:.3} and the carpet stops at {:.3}",
            at.z + half.y,
            carpet.max.z
        );
        assert!(
            at.z - half.y > carpet.min.z - 1e-3,
            "the track starts at z {:.3} and the carpet starts at {:.3}",
            at.z - half.y,
            carpet.min.z
        );
    }
}
