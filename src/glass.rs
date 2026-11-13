//! The room over the nook, and the glass sphere standing in the middle of it.
//! Spec 0064.
//!
//! **What it demonstrates is refraction**, which is the one thing this engine
//! can show that spec 0061's corridor cannot. Those two walls are a render to
//! texture from a fixed plane, so they can only show what a second camera
//! could see. A sphere shows the parts of the room nothing is pointed at, and
//! upside down.
//!
//! **The shape of the room is the spike's and not the spec's.** A sphere
//! squeezes a wide cone of the world into a disc, so what it needs behind it
//! is depth and variety with angle rather than objects: a quiet room with one
//! thing in it came back as a flat disc on a flat wall, four placements
//! running. What worked was length with something repeating down it, which is
//! why this is a colonnade and why the plinth stands in the middle rather
//! than at an end. The record is in the spec.

use blitzkit::collision::Aabb;
use blitzkit::mesh::Transform;
use blitzkit::renderer::scene::{MeshId, Scene, Traced};
use glam::{vec3, Vec3};

use crate::room::{self, NOOK_SPAN, THICK};

/// The top of this room's floor, which is the nook's ceiling plus a slab.
///
/// Named off the room below rather than written down, because the one thing
/// a storey over another storey must not do is float or overlap it.
pub const UP: f32 = room::TALL + THICK;

/// And how much headroom it has, which is the nook's own.
///
/// The same as every room on the ground storey. This one is not a cellar and
/// not a hall; it is a room you walk the length of, and the hall's height is
/// what that reads as.
pub const TALL: f32 = room::TALL;

/// The stair, in the terms spec 0007's already uses, because a second account
/// of what a stair is would come apart from the first.
///
/// `cellar::RISER` and `cellar::TREAD` are 0.25 and 0.3, which walks: that
/// flight is in the building and has been climbed since spec 0007. Fourteen
/// risers is `UP` exactly, and the going is 4.2 metres.
pub const STEPS: usize = (UP / crate::cellar::RISER) as usize;
pub const GOING: f32 = STEPS as f32 * crate::cellar::TREAD;

/// How wide the flight is, which is the cellar's passage again.
pub const FLIGHT: f32 = crate::cellar::PASSAGE;

/// What the room is drawn and walked inside, as the nook's own footprint less
/// a wall all round.
///
/// Off the nook's floor slab, which is the thing that says where the nook
/// actually is. Spec 0022's gallery and this room have both been built by
/// reading a neighbour's numbers and both were wrong the first time.
pub fn inside(reaches: f32) -> Aabb {
    let (at, size) = room::nook_floor(reaches);
    let half = vec3(size.x * 0.5 - THICK, 0.0, size.z * 0.5 - THICK);
    // the nook's floor slab runs to `shut`, past the end wall, so the room
    // takes the wall's inner face as its north end rather than the slab's
    let shut = -reaches + NOOK_SPAN - THICK * 0.5;

    Aabb::new(
        vec3(at.x - half.x, UP, at.z - half.z),
        vec3(at.x + half.x, UP + TALL, shut),
    )
}

/// How wide the cut in the floor is: the flight, and a balustrade each side
/// of it.
pub const WELL_WIDE: f32 = FLIGHT + GUARD * 2.0;

/// The well: the slot the flight comes down, on the room's own axis.
///
/// It is a stairwell and not an opened ceiling. Spec 0064 rules the second
/// out as its own room, and the difference is that this one is as wide as the
/// stair and no wider.
///
/// **A slot down the middle and not the whole north end.** The first build
/// took the north strip of the floor across the full width, because the
/// flight crossed it. A flight running along the room needs 4.2 metres of
/// its length, and taking that at the full width would cost the colonnade
/// two bays and leave a nine metre cliff edge with a rail along all of it.
/// Cut as a slot instead it costs the room nothing but the slot: the floor
/// wraps round three sides of it and the colonnade, which stands well
/// outside it, runs the room's whole length.
pub fn well(reaches: f32) -> Aabb {
    let room = inside(reaches);
    let foot = stair_foot(reaches);

    Aabb::new(
        vec3(foot.x - WELL_WIDE * 0.5, -THICK, stair_head(reaches).z),
        vec3(foot.x + WELL_WIDE * 0.5, room.max.y, foot.z + LANDING),
    )
}

/// The floor you walk on up here, which is the nook's footprint with the
/// slot cut out of it: three slabs rather than one.
pub fn floors(reaches: f32) -> Vec<Aabb> {
    let room = inside(reaches);
    let well = well(reaches);
    let (under, top) = (room::TALL, UP);
    let (south, north) = (room.min.z - THICK, room.max.z + THICK);

    vec![
        // west of the slot, the room's whole length
        Aabb::new(
            vec3(room.min.x - THICK, under, south),
            vec3(well.min.x, top, north),
        ),
        // east of it, the same
        Aabb::new(
            vec3(well.max.x, under, south),
            vec3(room.max.x + THICK, top, north),
        ),
        // the width of the slot south of it, which is where you walk at the
        // stair head
        Aabb::new(
            vec3(well.min.x, under, south),
            vec3(well.max.x, top, well.min.z),
        ),
        // and the same north of it
        Aabb::new(
            vec3(well.min.x, under, well.max.z),
            vec3(well.max.x, top, north),
        ),
    ]
}

/// Where the flight stands: its foot, at the north end of the room's own
/// axis, with the flight climbing south from it.
///
/// **It runs along the room now and not across it.** Jake walked it on
/// 2026-11-12: *it should be a linear way out, not a left turn.* Crossing
/// the room meant you walked its whole length to the north end and then
/// turned ninety degrees to go down, which is what a fire escape does and
/// not what the way out of a room with one thing in it should do.
///
/// On the middle of the width, which is the plinth's own line, so coming up
/// the stair puts the sphere straight ahead of you and walking at the sphere
/// and past it walks you out.
pub fn stair_foot(reaches: f32) -> Vec3 {
    let room = inside(reaches);

    vec3(
        (room.min.x + room.max.x) * 0.5,
        0.0,
        room.max.z - CLEAR_AT_THE_FOOT,
    )
}

/// How much nook is left in front of the bottom tread.
///
/// **The flight is not at the end of the room, and that is the whole of
/// this.** Put at the north end it came down to a wall four tenths of a
/// metre away: Jake walked it and said the stairs end at a wall, which is
/// the same turn he had asked to have taken out, moved to the bottom. The
/// nook's open floor runs its whole length, so the fix is simply to stop
/// the flight short of the end and let it arrive in the room rather than
/// at its wall.
///
/// It is the number that trades against the sphere's own depth, one metre
/// for one metre, and sixteen metres of room has to do both jobs now.
pub const CLEAR_AT_THE_FOOT: f32 = 3.2;

/// How much of the well is left past the bottom tread, which is a step off
/// and not a landing: the landing is the nook, under this room's floor.
pub const LANDING: f32 = 0.3;

/// The treads, as the boxes you actually stand on.
///
/// Each one is a slab from the floor up to its own top, rather than a step
/// floating over nothing: spec 0007 found that what is in front of your foot
/// is a question about the floor, asked straight down, so a tread with
/// daylight under it is a tread you fall through.
///
/// **The top one is level with the floor it leaves.** Fourteen risers of
/// 0.25 is 3.5, which is this floor, so the last tread is the floor and
/// there is no step at the head of the flight at all. That is the fault the
/// first build had: the gap in the rail was 1.3 wide and only its first 0.3
/// was at floor level, so walking through it anywhere but the top tread
/// dropped you three risers. Jake found it by walking at the stairs.
pub fn treads(reaches: f32) -> Vec<Aabb> {
    let foot = stair_foot(reaches);
    let going = crate::cellar::TREAD;
    let riser = crate::cellar::RISER;
    let half = FLIGHT * 0.5;

    (0..STEPS)
        .map(|n| {
            let top = riser * (n + 1) as f32;
            let south = foot.z - going * (n + 1) as f32;

            Aabb::new(
                vec3(foot.x - half, -THICK, south),
                vec3(foot.x + half, top, south + going),
            )
        })
        .collect()
}

/// How thick a step is where it is drawn.
pub const STEP_THICK: f32 = 0.3;

/// The steps as they are drawn, which is not how they are walked.
///
/// **A tread is a slab from the floor up, and that is a wall.** Spec 0007
/// asks what is in front of your foot straight down, so a tread with
/// daylight under it is a tread you fall through, and every tread in
/// `treads` runs to the floor for that reason. Running along the room, the
/// top of the flight is 3.5 high and its south end faces the nook: the room
/// below got a blank pale slab standing in the middle of it, with the way
/// up hidden behind it.
///
/// So the steps are drawn a step thick and the underside of the flight is a
/// stepped rake, which is what a stair looks like from below. Consecutive
/// boards overlap by five centimetres, so there is no daylight through it
/// either.
pub fn tread_boards(reaches: f32) -> Vec<Aabb> {
    treads(reaches)
        .into_iter()
        .map(|tread| {
            Aabb::new(
                vec3(tread.min.x, tread.max.y - STEP_THICK, tread.min.z),
                tread.max,
            )
        })
        .collect()
}

/// The volume the steps stand in: what a head passes through on the way up.
pub fn flight(reaches: f32) -> Aabb {
    let foot = stair_foot(reaches);
    let head = stair_head(reaches);
    let half = FLIGHT * 0.5;

    Aabb::new(
        vec3(foot.x - half, 0.0, head.z),
        vec3(foot.x + half, UP, foot.z),
    )
}

/// Where the flight arrives, which is the south end of it.
pub fn stair_head(reaches: f32) -> Vec3 {
    let foot = stair_foot(reaches);

    vec3(foot.x, UP, foot.z - GOING)
}

/// The nosing a tread carries, which is the lip that throws the shadow line.
///
/// **A flight of plain slabs reads as a ramp with lines on it.** What tells
/// you a stair is a stair from across a room is the row of shadows under the
/// nosings, and a tread whose front is flush with its riser has none. Drawn
/// and not walked: thirty five millimetres is well under `reach::STEP`, so
/// nothing here is a ledge.
pub const NOSING_OUT: f32 = 0.035;
pub const NOSING_TALL: f32 = 0.045;

pub fn nosings(reaches: f32) -> Vec<Aabb> {
    treads(reaches)
        .into_iter()
        .map(|tread| {
            Aabb::new(
                vec3(tread.min.x, tread.max.y - NOSING_TALL, tread.max.z),
                vec3(tread.max.x, tread.max.y, tread.max.z + NOSING_OUT),
            )
        })
        .collect()
}

/// How thick the balustrade is, how far up the rail sits over the rake, and
/// what it is made of.
///
/// There is one to each side of the flight, because a flight down the middle
/// of a room is open on both. They stand outside the going, so the stair is
/// as wide to walk as `FLIGHT` says it is: taking a hand's breadth off the
/// nook's own passage for a rail is the thing this project keeps being told
/// not to do.
pub const GUARD: f32 = 0.14;
/// And how thick the joinery standing in that gap is.
///
/// **Thinner than the gap, so neither of its faces is in a plane with
/// anything.** Filling the gap exactly put the string's inner face on the
/// treads' own edge and its outer face on the floor slab's cut edge, which
/// is two faces in one plane twice over: `aloft`'s check caught a patch
/// three and a half metres square. Two centimetres of air each side costs
/// nothing and is not a gap a body can find, since a body is 0.9 across.
pub const BALUSTRADE: f32 = 0.1;
pub const HANDRAIL: f32 = 0.08;
pub const HANDRAIL_UP: f32 = 0.9;
pub const STRING_OVER: f32 = 0.08;
pub const STRING_DEEP: f32 = 0.34;
pub const BALUSTER: f32 = 0.055;
pub const NEWEL: f32 = 0.12;

/// Which side of the flight a balustrade is on: west is -1 and east is +1.
pub const SIDES: [f32; 2] = [-1.0, 1.0];

/// The line one side's joinery is set out from: the stair's own pitch, taken
/// through the nosings.
///
/// **Through the nosings and not along the corners of the flight.** Those are
/// two lines with similar answers, which this building has a rule about: the
/// flight's own diagonal runs from the floor to the floor above and the
/// nosing line runs a riser higher at the bottom and lands on the top tread,
/// so a rail set out from the first drifts a quarter of a step away from the
/// steps by the top.
pub fn rake(reaches: f32, side: f32) -> (Vec3, Vec3) {
    let foot = stair_foot(reaches);
    let going = crate::cellar::TREAD;
    let riser = crate::cellar::RISER;
    let across = foot.x + side * (FLIGHT * 0.5 + GUARD * 0.5);

    (
        vec3(across, riser, foot.z),
        vec3(
            across,
            riser * STEPS as f32,
            foot.z - going * (STEPS - 1) as f32,
        ),
    )
}

/// The point on that line at a given place along the room.
///
/// The rail and the string both run past the rake's own two ends, because
/// those are nosings and the posts they die into are not over nosings.
pub fn rake_at(reaches: f32, side: f32, z: f32) -> Vec3 {
    let (low, high) = rake(reaches, side);
    let share = (z - low.z) / (high.z - low.z);

    vec3(low.x, low.y + (high.y - low.y) * share, z)
}

/// Where each baluster stands: one to a tread, at the nosing.
pub fn balusters(reaches: f32, side: f32) -> Vec<Vec3> {
    let (low, _) = rake(reaches, side);
    let going = crate::cellar::TREAD;
    let riser = crate::cellar::RISER;

    (0..STEPS)
        .map(|n| {
            vec3(
                low.x,
                riser * (n + 1) as f32 + STRING_OVER,
                low.z - going * n as f32,
            )
        })
        .collect()
}

/// How tall one is: from the string it stands on to the underside of the
/// rail. The same for every one of them, because the rail is parallel to the
/// string.
pub fn baluster_tall() -> f32 {
    HANDRAIL_UP - STRING_OVER - HANDRAIL * 0.5
}

/// The two posts one side's balustrade is hung between.
///
/// The one at the head stands on the floor it arrives at and carries the
/// step from the flight's rail to the landing's, which stand at different
/// heights because a rail you hold going up is not a rail you lean on
/// standing still. The one at the foot stands on the nook's floor.
pub fn newels(reaches: f32, side: f32) -> Vec<Aabb> {
    let (low, high) = rake(reaches, side);
    let half = NEWEL * 0.5;

    vec![
        Aabb::new(
            vec3(low.x - half, -THICK, low.z + NOSING_OUT),
            vec3(
                low.x + half,
                low.y + HANDRAIL_UP + 0.12,
                low.z + NOSING_OUT + NEWEL,
            ),
        ),
        Aabb::new(
            vec3(high.x - half, UP, high.z - crate::cellar::TREAD),
            vec3(
                high.x + half,
                UP + RAIL_UP + 0.12,
                high.z - crate::cellar::TREAD + NEWEL,
            ),
        ),
    ]
}

/// What the balustrades stop, as boxes.
///
/// **A flight down the middle of a room is open on both sides.** The rail is
/// drawn along the rake, which is one turned box and no use to a walk: what
/// a body meets has to be an upright box, so it is one to a tread, stepping
/// up with them.
pub fn guard(reaches: f32) -> Vec<Aabb> {
    let mut out = Vec::new();

    for side in SIDES {
        let (low, _) = rake(reaches, side);

        out.extend(treads(reaches).into_iter().map(|tread| {
            Aabb::new(
                vec3(low.x - BALUSTRADE * 0.5, tread.max.y, tread.min.z),
                vec3(
                    low.x + BALUSTRADE * 0.5,
                    tread.max.y + HANDRAIL_UP,
                    tread.max.z,
                ),
            )
        }));
        out.extend(newels(reaches, side));
    }

    out
}

/// How high the balustrade round the well stands.
///
/// **There was a `WAY_UP` here and it was the fault Jake walked into.** It
/// was the gap in the old rail across the head of the stair, 1.3 wide
/// "because what has to pass through it is a body and not a step". A body
/// does need 1.3, and only the first 0.3 of that gap had the top tread under
/// it: the rest was open over treads a riser, two risers and three risers
/// down. Walking at the stair anywhere but dead on the top tread dropped
/// you three quarters of a metre.
///
/// The question was never how wide a body is. It is what is under the gap,
/// and the answer now is that the way in is the whole width of the flight
/// and every bit of it has the top tread under it, which is level with the
/// floor.
pub const RAIL_UP: f32 = 1.05;

/// The frames down the long walls, as where each one stands and how big it
/// is.
///
/// **At the corridor's own pitch**, because the one thing the spike found is
/// that a sphere wants something repeating with depth behind it, and this
/// building already owns a rhythm that works: `mirrors::PITCH`.
/// **Columns standing free in the room, and that took three goes.** They
/// were 0.12 deep on the wall first, which is a painting of a colonnade: it
/// read as one slab with lines on it and the glass did not see it at all.
/// Then they were piers a third of a metre proud, which reads in the room
/// and still not in the ball, and painting every traced box a colour of its
/// own showed why: the sphere was finding five surfaces, the floor, the
/// ceiling, the end wall and the two long ones, because that is all a room
/// with flat walls has.
///
/// Spec 0064 says it and the spike said it first. What works is the hall's
/// aisle, which is two rows of cabinets standing in the room with floor
/// behind, under and between them. A column a stride in from the wall is
/// that. A pilaster on the wall is the wall.
pub const FRAME_WIDE: f32 = 0.55;
pub const FRAME_TALL: f32 = TALL;
pub const FRAME_UP: f32 = 0.0;
pub const FRAME_DEEP: f32 = 0.55;

/// How far in from the wall each row stands, which is what puts floor and
/// wall behind them.
pub const FRAME_IN: f32 = 1.5;

pub fn frames(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let room = inside(reaches);
    // **The room's whole length, because the well no longer takes its north
    // end.** The colonnade stands well outside the slot the flight comes
    // down, so nothing about the stair shortens it: cut as a full width well
    // it would have lost two bays, and the rhythm is what the sphere is for.
    let long = room.max.z - room.min.z - FRAME_WIDE;
    let bays = (long / crate::mirrors::PITCH) as usize;
    // centred on the run, so the two ends match
    let first = room.min.z + (long - crate::mirrors::PITCH * (bays - 1) as f32) * 0.5;
    let mut out = Vec::with_capacity(bays * 2);

    for n in 0..bays {
        let z = first + crate::mirrors::PITCH * n as f32;

        for side in [room.min.x + FRAME_IN, room.max.x - FRAME_IN] {
            out.push((
                vec3(side, UP + FRAME_UP + FRAME_TALL * 0.5, z),
                vec3(FRAME_DEEP, FRAME_TALL, FRAME_WIDE),
            ));
        }
    }

    out
}

/// A lit panel over each frame, which is this room's whole lighting and the
/// only thing the glass can show that gives light.
///
/// **Because the tracer has no lamps.** Spec 0098's shader knows the ambient
/// and one directional light and nothing else, so a point light in this room
/// lights the room and not the room inside the ball. A box whose colour is
/// past one does both: the raster draws it bright and the trace reads it
/// bright, which is what `aim` means by a colour over one everywhere else in
/// this building.
/// What a bay comes to, whatever hour it is outside.
///
/// **Through `gives_off`, because this room is indoors and the sky is not.**
/// A colour here is a multiplier on the light, and the one light the engine
/// hands a scene is the world's: at noon the fill is 0.15 and at midnight it
/// is 0.03, so a bay written as a flat 3.4 was five times brighter by day
/// than by night. From the field at night the glazing was all but out, which
/// is the opposite of what a lit room looks like from outside.
///
/// The same answer `aim::MOON`, the runestone and Valhalla's hearth all
/// reached. See `garden::gives_off`.
/// What a sconce bowl on the flight comes to, which is what lands and not
/// what it is multiplied by: `garden::gives_off` again.
pub fn lit_sconce(since: f32) -> glam::Vec4 {
    crate::garden::gives_off(crate::aim::GLASS_SCONCE.truncate(), since).extend(1.0)
}

pub fn lit(since: f32) -> glam::Vec4 {
    crate::garden::gives_off(crate::aim::GLASS_LAMP.truncate(), since).extend(1.0)
}

pub const LAMP: Vec3 = vec3(0.1, 2.3, 0.75);
pub const LAMP_UP: f32 = 1.45;

/// What one of them throws into the room: its colour, how hard, and how far.
///
/// Warm against the cool stone, and short, so that the room is a run of
/// pools of light with dark between them rather than an evenly lit box. The
/// dark between is half of what the sphere has to show.
pub const LAMP_COLOUR: Vec3 = vec3(1.0, 0.94, 0.84);
pub const LAMP_LIT: f32 = 0.9;
pub const LAMP_RANGE: f32 = 4.5;

/// One in the middle of every bay, which is between two piers.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    let room = inside(reaches);
    let frames = frames(reaches);
    let mut out = Vec::new();

    for side in [room.min.x, room.max.x] {
        let mut along: Vec<f32> = frames
            .iter()
            .filter(|(at, _)| (at.x - side).abs() < FRAME_IN + FRAME_DEEP)
            .map(|(at, _)| at.z)
            .collect();

        along.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in along.windows(2) {
            let into = if side < -8.0 { LAMP.x } else { -LAMP.x };

            out.push(vec3(side + into, UP + LAMP_UP, (pair[0] + pair[1]) * 0.5));
        }
    }

    out
}

/// A slot of glazing on the outside of every bay, cut right through.
///
/// **Because the room was a blank box on the roof.** Seen from the field it
/// was the tallest thing on the building and had nothing on any of its four
/// faces, which reads as a plant room. Every other roofline here has
/// something; this one had the honest excuse that the room is about what is
/// inside it, and that is not an excuse a stranger in the field can hear.
///
/// **Cut through and not painted on.** A lit panel on the outside of a solid
/// wall is light coming out of nothing, and this building has a rule about
/// two accounts of one thing that covers it. A real opening costs the walls
/// being four pieces instead of one each, and buys three things: the room's
/// own light leaves the building at night, the bays read as a rhythm by day,
/// and the glass has somewhere with depth behind it to look.
///
/// Set high enough that it is a clerestory rather than a window, which is
/// what a room with one thing in it wants: 0.9 is over the step anything
/// here can ride, so nothing can walk out of one.
pub const SLOT_UP: f32 = 0.9;
pub const SLOT_TALL: f32 = 1.5;
pub const SLOT_WIDE: f32 = 0.5;

pub fn slots(reaches: f32) -> Vec<Aabb> {
    let room = inside(reaches);

    lamps(reaches)
        .into_iter()
        .map(|at| {
            let side = if at.x < (room.min.x + room.max.x) * 0.5 {
                room.min.x - THICK
            } else {
                room.max.x
            };

            Aabb::new(
                vec3(side - 0.01, UP + SLOT_UP, at.z - SLOT_WIDE * 0.5),
                vec3(
                    side + THICK + 0.01,
                    UP + SLOT_UP + SLOT_TALL,
                    at.z + SLOT_WIDE * 0.5,
                ),
            )
        })
        .collect()
}

/// The band round the top of it, which is what every other roofline in this
/// building has and this one did not.
pub const CORNICE: f32 = 0.22;
pub const CORNICE_OUT: f32 = 0.14;

pub fn cornice(reaches: f32) -> Aabb {
    let room = inside(reaches);

    Aabb::new(
        vec3(
            room.min.x - THICK - CORNICE_OUT,
            room.max.y - CORNICE,
            room.min.z - THICK - CORNICE_OUT,
        ),
        vec3(
            room.max.x + THICK + CORNICE_OUT,
            room.max.y + THICK,
            room.max.z + THICK + CORNICE_OUT,
        ),
    )
}

/// The plinth, and the ball on it.
///
/// **In the middle of the room's length.** A sphere shows what is behind it,
/// so a plinth at the end wall puts the room's length behind your eye instead
/// of behind the glass: the spike's first one came back as a flat disc in a
/// corner. In the middle there is length behind it whichever end you come
/// from, and the walk is the same either way.
pub const PLINTH: f32 = 0.9;
pub const PLINTH_UP: f32 = 1.0;
pub const BALL: f32 = 0.62;

pub fn plinth(reaches: f32) -> Aabb {
    let room = inside(reaches);
    let middle = middle(reaches);

    Aabb::new(
        vec3(middle.x - PLINTH * 0.5, room.min.y, middle.z - PLINTH * 0.5),
        vec3(
            middle.x + PLINTH * 0.5,
            room.min.y + PLINTH_UP,
            middle.z + PLINTH * 0.5,
        ),
    )
}

/// The middle of the room's own length, which is the floor's and not the
/// nook's: the well is not part of the walk.
pub fn middle(reaches: f32) -> Vec3 {
    let room = inside(reaches);

    vec3(
        (room.min.x + room.max.x) * 0.5,
        UP,
        (room.min.z + stair_head(reaches).z) * 0.5,
    )
}

/// The ball: where its middle is and how big it is.
///
/// At the eye's own height over the floor, so that walking at it is walking
/// into it rather than up at it.
pub fn ball(reaches: f32) -> (Vec3, f32) {
    let plinth = plinth(reaches);

    (
        vec3(
            (plinth.min.x + plinth.max.x) * 0.5,
            plinth.max.y + BALL,
            (plinth.min.z + plinth.max.z) * 0.5,
        ),
        BALL,
    )
}

/// Everything up here a body stops against.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let room = inside(reaches);
    let mut out = floors(reaches);

    out.push(plinth(reaches));

    out.extend(treads(reaches));
    out.extend(guard(reaches));
    // **The colonnade is solid and not only drawn.** A pilaster you walk
    // through is a painting of a pilaster, and there is a second reason
    // here: what the glass traces is `solid`, so a frame that is only drawn
    // is a frame the sphere cannot see. The first build of this room had
    // them drawn, and the ball came back as three flat bands of floor, wall
    // and ceiling, which is the thing the colonnade exists to stop.
    out.extend(column_boxes(reaches).into_iter().map(|(box_, _)| box_));
    out.extend(
        lamps(reaches)
            .into_iter()
            .map(|at| Aabb::new(at - LAMP * 0.5, at + LAMP * 0.5)),
    );

    // the four walls, each standing on the floor slab's own edge, with the
    // bays' slots taken out of the two long ones
    let slots = slots(reaches);

    for (low, high) in [
        (
            vec3(room.min.x - THICK, UP, room.min.z - THICK),
            vec3(room.min.x, UP + TALL, room.max.z),
        ),
        (
            vec3(room.max.x, UP, room.min.z - THICK),
            vec3(room.max.x + THICK, UP + TALL, room.max.z),
        ),
        (
            vec3(room.min.x - THICK, UP, room.min.z - THICK),
            vec3(room.max.x + THICK, UP + TALL, room.min.z),
        ),
        (
            vec3(room.min.x - THICK, UP, room.max.z),
            vec3(room.max.x + THICK, UP + TALL, room.max.z + THICK),
        ),
    ] {
        let mut pieces = vec![Aabb::new(low, high)];

        for slot in slots.iter() {
            pieces = pieces
                .into_iter()
                .flat_map(|piece| crate::room::minus(piece, *slot))
                .collect();
        }

        out.extend(pieces);
    }

    // **No rail across the head of the stair any more.** There was one, in
    // two pieces with a gap to walk through, because the well was the room's
    // whole north end and the floor stopped at a nine metre cliff. The well
    // is a slot now: the only edges up here are the slot's own two sides and
    // they are what the balustrades stand on, so a rail across the width
    // would be a rail across the way out.
    out
}

/// The glazing in the slots, which is drawn and not solid: what stops you is
/// the sill, which is higher than anything here can ride.
///
/// **In the opening, since 2026-11-13.** It sat a little proud of the
/// building's skin for a day, because `Room::outside` sampled columns and
/// asked what was over each one, so it knew walls and did not know
/// openings and plastered straight over this. A pane set inside the
/// opening was a pane behind the plaster, which is how this was built
/// first and why the elevation came back blank.
///
/// That face reads the wall's own spans now and leaves the hole alone, so
/// the glazing goes where glazing goes: in the middle of the wall's
/// thickness, with a reveal either side of it.
pub const PANE_THICK: f32 = 0.04;

pub fn panes(reaches: f32) -> Vec<Aabb> {
    slots(reaches)
        .into_iter()
        .map(|slot| {
            let middle = (slot.min.x + slot.max.x) * 0.5;

            Aabb::new(
                vec3(middle - PANE_THICK * 0.5, slot.min.y, slot.min.z),
                vec3(middle + PANE_THICK * 0.5, slot.max.y, slot.max.z),
            )
        })
        .collect()
}

/// The lid over it.
pub fn lid(reaches: f32) -> Aabb {
    let room = inside(reaches);

    Aabb::new(
        vec3(room.min.x - THICK, room.max.y, room.min.z - THICK),
        vec3(room.max.x + THICK, room.max.y + THICK, room.max.z + THICK),
    )
}

/// Where the room stands over, which is the nook and nothing else.
#[cfg(test)]
pub fn over(reaches: f32) -> Aabb {
    let room = inside(reaches);

    Aabb::new(
        vec3(room.min.x - THICK, 0.0, room.min.z - THICK),
        vec3(room.max.x + THICK, UP, room.max.z + THICK),
    )
}

/// The nook's own footprint, for the test that this room stands over nothing
/// else. Off `nook_floor` and the side the hall's wall is on, which is what
/// the nook is.
#[cfg(test)]
pub fn the_nook(reaches: f32) -> Aabb {
    let side = crate::room::WALL + crate::room::CABINET.x;
    let shut = -reaches + NOOK_SPAN + THICK * 0.5;

    Aabb::new(
        vec3(-side - crate::room::NOOK_DEEP, 0.0, -reaches),
        vec3(-side, UP, shut),
    )
}

/// How far from the ball a box has to be before the glass cannot show it.
///
/// **Because the tracer has a cap and the building has 635 boxes.** Spec
/// 0098 holds a thousand and twenty four, so the whole building fits, but
/// most of it is behind walls the first bounce never leaves. What the sphere
/// can show is what the room holds plus whatever is through the well, and
/// twenty-five metres reaches both with room to spare.
pub const SEES: f32 = 25.0;

/// What the glass is given to trace: the building's own boxes, near enough
/// to be worth it.
///
/// **`solid` and `lids` and not a list of its own.** Every room here is a
/// list of axis-aligned boxes because that is what `walk` needs, and that is
/// already the scene a tracer wants. A second account of the room would be
/// the fault this building has written down under five other names, and it
/// would be the one that shows: the glass would stop agreeing with the room
/// round it the first time a wall moved.
pub fn traced(room: &crate::room::Room, since: f32) -> Vec<Traced> {
    let (ball, _) = ball(room.reaches);
    let mut out = Vec::new();

    // **The glazing is not in here**, and that is the point of a slot: a ray
    // that leaves through one meets nothing and comes back as the room's
    // ambient, which is what a window looks like from inside a marble. The
    // list stays exactly `solid` and `lids`, which
    // `it_traces_the_building_itself` holds.
    let named = named(room.reaches);
    let round: Vec<Aabb> = column_boxes(room.reaches)
        .into_iter()
        .filter(|(_, round)| *round)
        .map(|(box_, _)| box_)
        .collect();

    for box_ in room.solid().into_iter().chain(room.lids()) {
        if box_.center().distance(ball) > SEES {
            continue;
        }

        let said = colour(box_, &named, ball.y, since);

        // **The colonnade is turned and the glass can say so now**, per the
        // engine's spec 0098. What a body walks into is still the box.
        out.push(if round.contains(&box_) {
            Traced::round(box_, said)
        } else {
            Traced::new(box_, said)
        });
    }

    out
}

/// What colour a box of the building is, asked once for the room and for the
/// glass both.
///
/// **By what the box is rather than by what room it is in.** Nothing in
/// `solid` says what it is made of, and a list that said so would be that
/// second account again. A flat box over your head is a ceiling, a flat one
/// under your feet is a floor, a tall thin one on a wall is a frame, and
/// everything else is the wall itself, which is what a room looks like from
/// inside a marble.
///
/// **One function for both, which the first build was not.** The drawing had
/// a frame's colour and the trace did not, so the colonnade was the wall's
/// own colour inside the glass and the ball came back as a flat grey disc.
/// The whole point of the room is that the frames read in it.
/// The pieces of this room whose colour no shape can tell.
///
/// **`colour` reads a box's shape, and that is right for a room of walls,
/// floors and columns.** It cannot tell timber from stone: the balustrade
/// is a run of upright boxes a column's width, so the glass showed Jake a
/// stone handrail over a timber one, and a tread is a tall slab, which that
/// function calls a wall.
///
/// **Not a second account.** The objection to a list of what things are
/// made of is that it drifts from the thing it describes. These boxes are
/// `guard` and `treads` themselves, asked by name, so there is nothing here
/// to drift.
pub fn named(reaches: f32) -> Vec<(Aabb, glam::Vec4)> {
    // **The blocks at a column's ends, which shape gets badly wrong.** An
    // abacus is a flat wide box over your head, which `colour` calls a
    // ceiling, so the glass put a dark slab on top of every column while
    // the room had a pale capital. The plinth under it came out as floor
    // for the same reason. Jake went up to the ball and said there is
    // something dark at the top.
    column_boxes(reaches)
        .into_iter()
        .filter(|(_, round)| !*round)
        .map(|(box_, _)| (box_, crate::aim::GLASS_TRIM))
        .chain(
            guard(reaches)
                .into_iter()
                .map(|box_| (box_, crate::aim::GLASS_STRING)),
        )
        .chain(
            treads(reaches)
                .into_iter()
                .map(|box_| (box_, crate::aim::GLASS_PIER)),
        )
        .collect()
}

fn colour(box_: Aabb, named: &[(Aabb, glam::Vec4)], up: f32, since: f32) -> glam::Vec4 {
    if let Some((_, said)) = named.iter().find(|(one, _)| *one == box_) {
        return *said;
    }

    let size = box_.size();
    let flat = size.y < 0.5 && size.x.min(size.z) > 0.5;

    if (size - LAMP).abs().max_element() < 1e-3 {
        lit(since)
    } else if size.y > 0.2 && size.x.max(size.z) <= FRAME_DEEP.max(FRAME_WIDE) + ABACUS_OUT * 2.0 {
        // **A column is a stack of short bands now, not one tall box.** The
        // height this asked for was a metre, which was right while a shaft
        // was one box and calls every band of a turned one a wall.
        crate::aim::GLASS_PIER
    } else if flat && box_.center().y > up {
        crate::aim::GLASS_CEIL
    } else if flat {
        crate::aim::GLASS_FLOOR
    } else {
        crate::aim::GLASS_WALL
    }
}

/// The lights on the flight, as the middle of each sconce bowl.
///
/// **The stair had none and was not meant to.** It climbed under the nook's
/// own ceiling, so it borrowed a pendant from the grid up there, and spec
/// 0064 then cut that ceiling away for the well: the pendant was left
/// hanging from nothing in the middle of the flight and the flight was left
/// dark when it went. A stair that borrows its light from the room it leaves
/// is dark halfway up whatever is done to either room.
///
/// Two of them, a third and two thirds of the way up. One at the bottom
/// leaves the top dark and one at the top is a light you climb toward, and
/// what a flight wants is to be lit the whole way.
pub const SCONCE_WIDE: f32 = 0.34;
pub const SCONCE_LIT: f32 = 0.8;
pub const SCONCE_RANGE: f32 = 4.0;

pub fn stair_sconces(reaches: f32) -> Vec<Vec3> {
    // **On the newels, because there is no wall beside this flight.** The
    // first pair were bowls on the nook's north wall, which is where the
    // stair ran when it crossed the room. Down the middle it has open air
    // on both sides and the only thing standing at the right height is the
    // post at each end of it.
    //
    // **One on every post, which is four.** The first go put one at the
    // head and one at the foot on opposite sides, reasoning that the light
    // would cross the steps rather than run down them. Standing at the head
    // of the stair that is a lamp on one post and a bare post facing it,
    // and Jake asked why there was a light on the right and not on the
    // left. A pair of newels is a pair.
    let posts: Vec<Aabb> = SIDES
        .iter()
        .flat_map(|side| newels(reaches, *side))
        .collect();

    posts
        .iter()
        .map(|post| {
            vec3(
                post.center().x,
                post.max.y + SCONCE_WIDE * 0.3,
                post.center().z,
            )
        })
        .collect()
}

/// And the light in each, which is the bowl itself.
///
/// Off the sconces rather than worked out again, so there is no lamp up here
/// that is not in a fitting and nobody has to remember. The baths' pair is
/// what this copies.
pub fn stair_lamps(reaches: f32) -> Vec<Vec3> {
    stair_sconces(reaches)
        .into_iter()
        .map(|at| at + Vec3::Y * 0.04)
        .collect()
}

/// One column of the colonnade, turned.
///
/// **A box with two bands on it is a pier, and this room wanted a
/// colonnade.** Jake, 2026-11-12: *the room should be more ornate and
/// decorated.* The lathe has been in this building since the cellar's
/// barrels and it is what the difference is made of.
///
/// Tuscan, roughly: a torus at the foot, a shaft with a little entasis so
/// it is not a tube, a necking, and an echinus flaring out under the
/// abacus. The square blocks at either end are boxes, because an abacus and
/// a plinth are square and a lathe cannot be.
pub fn column_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(18, 30, |v| (v - 0.5, column_waist(v)))
}

/// How fat the column is at a given height, as a share of its widest.
///
/// **One account of the profile**, because the lathe is not the only thing
/// that needs it: spec 0098's tracer knows boxes and nothing else, so what
/// the sphere shows of a turned column is whatever boxes stand in for it.
/// Jake saw twelve flat piers in the glass while the room had a colonnade.
/// `column_boxes` samples this same curve.
pub fn column_waist(v: f32) -> f32 {
    if v < 0.035 {
        1.0
    } else if v < 0.07 {
        // the torus, falling into the shaft
        1.0 - 0.14 * ((v - 0.035) / 0.035)
    } else if v < 0.88 {
        // entasis: fattest a third of the way up and tapering away, so
        // the shaft reads as carried rather than extruded
        let along = (v - 0.07) / 0.81;
        0.86 + 0.03 * (along * std::f32::consts::PI).sin() - 0.14 * along
    } else if v < 0.9 {
        0.70
    } else if v < 0.965 {
        // the echinus
        0.70 + 0.32 * ((v - 0.9) / 0.065)
    } else {
        1.02
    }
}

/// The column as a body meets it and as the glass sees it: a stack of
/// boxes following the turned profile.
///
/// **Not one box the shaft's full width.** That is what was here, and in
/// the sphere it read as a flat pier with no base and no capital while the
/// room had a column with both. The bands are wide enough to hold the lathe
/// inside them, so nothing of the turning pokes out of what you walk into.
pub fn column_boxes(reaches: f32) -> Vec<(Aabb, bool)> {
    // **The profile's own joints, not an even slicing of its height.** Ten
    // equal bands put the necking and the echinus in one band together, and
    // a band is as wide as the widest thing in it, so the top of the shaft
    // came out as wide as its foot and the taper disappeared. Jake, looking
    // at the ball: the columns in the room have wider and thinner bases,
    // the reflection doesn't match.
    //
    // Cut where the lathe changes instead: the torus, five down the shaft,
    // the necking on its own, and the echinus. Same count of boxes, a
    // column instead of a tube.
    const BANDS: [(f32, f32); 8] = [
        (0.0, 0.07),
        (0.07, 0.23),
        (0.23, 0.39),
        (0.39, 0.56),
        (0.56, 0.72),
        (0.72, 0.88),
        (0.88, 0.9),
        (0.9, 1.0),
    ];

    frames(reaches)
        .into_iter()
        .flat_map(|(at, size)| {
            let low = at.y - size.y * 0.5;
            let tall = size.y - ABACUS * 2.0;
            let round = size.x.max(size.z);
            let blocks = [
                (low, low + ABACUS),
                (at.y + size.y * 0.5 - ABACUS, at.y + size.y * 0.5),
            ]
            .map(|(from, to)| {
                let half = (round + ABACUS_OUT * 2.0) * 0.5;

                Aabb::new(
                    vec3(at.x - half, from, at.z - half),
                    vec3(at.x + half, to, at.z + half),
                )
            });

            let shaft = BANDS.iter().copied().map(move |(from, to)| {
                // the widest the lathe gets anywhere in this band, so the
                // box holds it
                let widest = (0..=8)
                    .map(|k| column_waist(from + (to - from) * k as f32 / 8.0))
                    .fold(0.0f32, f32::max);
                let half = widest * round * 0.5;

                Aabb::new(
                    vec3(at.x - half, low + ABACUS + tall * from, at.z - half),
                    vec3(at.x + half, low + ABACUS + tall * to, at.z + half),
                )
            });

            // the blocks are square and the shaft is turned, which is what
            // the second of each pair says. The glass reads it; the walk
            // does not, because a round collider buys nothing a box does
            // not already give.
            let mut out: Vec<(Aabb, bool)> = blocks.iter().map(|one| (*one, false)).collect();

            out.extend(shaft.map(|one| (one, true)));
            out
        })
        .collect()
}

/// How much of a column's height the blocks at either end take.
pub const ABACUS: f32 = 0.11;
pub const ABACUS_OUT: f32 = 0.06;

/// One baluster, turned.
///
/// **A box would have been four more boxes in a room that is already boxes.**
/// The one thing this room has to do is give a sphere something with shape in
/// it to show, and fourteen square sticks are fourteen more rectangles. The
/// lathe is already here for the cellar's barrels and the people's limbs.
pub fn baluster_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(12, 26, |v| {
        let waist = if v < 0.07 {
            1.0
        } else if v < 0.13 {
            0.58
        } else if v < 0.44 {
            // the vase, fattest just over the foot and falling away
            0.96 - 0.5 * ((v - 0.13) / 0.31)
        } else if v < 0.88 {
            0.40
        } else if v < 0.94 {
            0.74
        } else {
            0.92
        };

        (v - 0.5, waist)
    })
}

/// The joinery on the open side of the flight.
///
/// The string and the rail are one box each, turned to the stair's pitch,
/// because a raked board is a raked board and a staircase of little boxes
/// pretending to be one is the thing this building keeps being told reads as
/// a painting of a thing.
///
/// What a body meets is `guard`, which is upright boxes, and those are not
/// drawn. Two accounts of one balustrade, which this project has a rule
/// about, and the rule's own test is whether they would still agree if one
/// requirement changed: they would not. A collider wants to be axis aligned
/// and a rail wants to be at the stair's angle, and that is two questions
/// with similar answers. So the rake is named once, in `rake`, and both of
/// them are set out from it.
pub fn draw_the_stair(
    scene: &mut Scene,
    reaches: f32,
    since: f32,
    cube: MeshId,
    baluster: Option<MeshId>,
    sconce: MeshId,
) {
    // **Newel to newel, and not the rake plus a post's width.** A board
    // runs from the middle of one post to the middle of the other, which is
    // where joinery meets a post. The rake's own two ends are the bottom and
    // top nosings and neither is under a newel, so a board set out from the
    // rake overshoots at one end and falls short at the other. Jake's
    // picture of the stair head, 2026-11-12.
    for side in SIDES {
        let posts = newels(reaches, side);
        let low = rake_at(reaches, side, posts[0].center().z);
        let high = rake_at(reaches, side, posts[1].center().z);
        let run = high - low;
        let long = (run.y * run.y + run.z * run.z).sqrt();
        // the flight runs along z and climbs in y, so a board on it turns
        // about x: the box's own length is its z and the turn takes that to
        // the rake
        let turn = glam::Quat::from_rotation_x((-run.y).atan2(run.z));
        let middle = (low + high) * 0.5;
        // **A turned board's top edge is further from its middle than half
        // its depth, not nearer.** Two lines at the stair's pitch half a
        // depth apart square to the board are half a depth *divided by* the
        // cosine of that pitch apart measured straight up. Dropping the
        // string by a plain half depth put its top edge over the line the
        // balusters stand on and ate their feet; multiplying by the cosine,
        // which was the first guess at the fix, buried them twice as deep.
        let upright = run.z.abs() / long;

        for (up, deep, thick, colour) in [
            (
                STRING_OVER - STRING_DEEP * 0.5 / upright,
                STRING_DEEP,
                BALUSTRADE,
                crate::aim::GLASS_STRING,
            ),
            (
                HANDRAIL_UP,
                HANDRAIL,
                HANDRAIL * 1.4,
                crate::aim::GLASS_RAIL,
            ),
        ] {
            scene.push_material(
                cube,
                &Transform::at(middle + Vec3::Y * up)
                    .with_rotation(turn)
                    .with_scale(vec3(thick, deep, long)),
                colour,
                crate::aim::MATTE,
            );
        }

        for post in posts {
            scene.push_material(
                cube,
                &Transform::at(post.center()).with_scale(post.size()),
                crate::aim::GLASS_STRING,
                crate::aim::MATTE,
            );
            // the cap, which says a post ended rather than ran out
            let cap = 0.07;

            scene.push_material(
                cube,
                &Transform::at(vec3(
                    post.center().x,
                    post.max.y - cap * 0.5,
                    post.center().z,
                ))
                .with_scale(vec3(NEWEL + 0.05, cap, NEWEL + 0.05)),
                crate::aim::GLASS_RAIL,
                crate::aim::MATTE,
            );
        }

        if let Some(baluster) = baluster {
            let tall = baluster_tall();

            for at in balusters(reaches, side) {
                scene.push_material(
                    baluster,
                    &Transform::at(at + Vec3::Y * tall * 0.5)
                        .with_scale(vec3(BALUSTER, tall, BALUSTER)),
                    crate::aim::GLASS_BALUSTER,
                    crate::aim::MATTE,
                );
            }
        }
    }

    // the steps, in the stair's own stone.
    //
    // **Not through `colour`, which reads a box's shape.** A step drawn a
    // step thick is flat and wide, which that function calls a floor, so the
    // whole underside of the flight came out the glass room's navy and hung
    // over the nook's toy tables as a dark wedge. A stair is masonry and the
    // nosings on it are already this stone.
    for board in tread_boards(reaches) {
        scene.push_material(
            cube,
            &Transform::at(board.center()).with_scale(board.size()),
            crate::aim::GLASS_PIER,
            crate::aim::MATTE,
        );
    }

    // the bowls on the newel caps, which are this flight's only light
    for at in stair_sconces(reaches) {
        scene.push_material(
            sconce,
            &Transform::at(at).with_scale(Vec3::splat(SCONCE_WIDE)),
            lit_sconce(since),
            crate::aim::VOID,
        );
    }

    for nosing in nosings(reaches) {
        scene.push_material(
            cube,
            &Transform::at(nosing.center()).with_scale(nosing.size()),
            crate::aim::GLASS_PIER,
            crate::aim::MATTE,
        );
    }
}

/// How deep a base or a capital is, how far it stands out from the shaft,
/// and the bead that necks the two together.
pub const CAP: f32 = 0.16;
pub const BEAD_OUT: f32 = 0.025;

/// How proud of the floor the border and the panel are laid, and how wide
/// the margin between them is.
pub const LAID: f32 = 0.025;
pub const MARGIN: f32 = 0.34;

/// How far the skirting stands off the wall and how high it runs.
pub const SKIRT: f32 = 0.06;
pub const SKIRT_UP: f32 = 0.22;

fn middle_x(room: Aabb) -> f32 {
    (room.min.x + room.max.x) * 0.5
}

fn middle_z(room: Aabb) -> f32 {
    (room.min.z + room.max.z) * 0.5
}

/// How deep the beams across the ceiling are and how far they come down.
pub const BEAM: f32 = 0.2;
pub const BEAM_DOWN: f32 = 0.16;

/// What makes the room worked stone rather than a box with boxes in it.
///
/// **Everything here is drawn and nothing is walked.** A base is a hand's
/// breadth of overhang at ankle height and a capital is over your head, so
/// neither is a thing a body meets; what a body meets is the shaft, which is
/// already in `solid` and already what the glass traces. Adding them to the
/// walk would be a ring of ledges round every column for no gain.
pub fn draw_the_room(scene: &mut Scene, reaches: f32, cube: MeshId, column: Option<MeshId>) {
    let room = inside(reaches);
    // **The border stops at the stairwell rather than running over it.** It
    // is laid between the two colonnades and the slot is between them too,
    // so a border run the room's whole length would be a rug over a hole.
    let floor = Aabb::new(
        vec3(room.min.x, UP, room.min.z),
        vec3(room.max.x, UP, well(reaches).min.z - 0.3),
    );

    // the floor, as a border with a panel laid on it. Proud and not sunk:
    // a border cut into a floor is two faces in one plane, which is the
    // fault this building has made four times.
    let inner = (
        room.min.x + FRAME_IN + FRAME_DEEP * 0.5 + 0.1,
        room.max.x - FRAME_IN - FRAME_DEEP * 0.5 - 0.1,
    );

    for (shrink, up, colour) in [
        (0.0, LAID, crate::aim::GLASS_BORDER),
        (MARGIN, LAID * 2.0, crate::aim::GLASS_PANEL),
    ] {
        let low = vec3(inner.0 + shrink, UP, floor.min.z + 0.5 + shrink);
        let high = vec3(inner.1 - shrink, UP + up, floor.max.z - 0.5 - shrink);
        let box_ = Aabb::new(low, high);

        scene.push_material(
            cube,
            &Transform::at(box_.center()).with_scale(box_.size()),
            colour,
            crate::aim::MATTE,
        );
    }

    // a turned column on every one of the colonnade's places, with a square
    // block at each end of it.
    //
    // **The box in `frames` is what a body walks into and what the glass
    // traces; this is what you look at.** The same split the stair's
    // balustrade and the plinth are drawn under. A lathe cannot be traced
    // by spec 0098, which knows boxes, and a box is what the sphere needs
    // to find anyway: what it reads is the rhythm of something standing out
    // from the wall, and the turning is for the eye in the room.
    for (at, size) in frames(reaches) {
        let low = at.y - size.y * 0.5;
        let high = at.y + size.y * 0.5;

        for middle in [low + ABACUS * 0.5, high - ABACUS * 0.5] {
            let round = size.x.max(size.z);

            scene.push_material(
                cube,
                &Transform::at(vec3(at.x, middle, at.z)).with_scale(vec3(
                    round + ABACUS_OUT * 2.0,
                    ABACUS,
                    round + ABACUS_OUT * 2.0,
                )),
                crate::aim::GLASS_TRIM,
                crate::aim::MATTE,
            );
        }

        if let Some(column) = column {
            let tall = size.y - ABACUS * 2.0;

            scene.push_material(
                column,
                &Transform::at(vec3(at.x, at.y, at.z)).with_scale(vec3(
                    size.x.max(size.z),
                    tall,
                    size.x.max(size.z),
                )),
                crate::aim::GLASS_PIER,
                crate::aim::MATTE,
            );
        }
    }

    // a frame round every bay, which is what the wall between the columns
    // had instead of anything at all
    for slot in slots(reaches) {
        let west = slot.center().x < (room.min.x + room.max.x) * 0.5;
        let face = if west { room.min.x } else { room.max.x };
        let out = if west { BEAD_OUT } else { -BEAD_OUT };

        scene.push_material(
            cube,
            &Transform::at(vec3(face + out * 0.5, slot.center().y, slot.center().z)).with_scale(
                vec3(
                    BEAD_OUT.abs(),
                    slot.size().y + CAP * 2.0,
                    slot.size().z + CAP * 2.0,
                ),
            ),
            crate::aim::GLASS_TRIM,
            crate::aim::MATTE,
        );
    }

    // and a beam over every bay, on the colonnade's own pitch. The ceiling
    // is the largest flat thing in any view of this room after the floor
    // and it was one dark slab.
    let mut along: Vec<f32> = frames(reaches).into_iter().map(|(at, _)| at.z).collect();

    along.sort_by(|a, b| a.partial_cmp(b).unwrap());
    along.dedup_by(|a, b| (*a - *b).abs() < 1e-3);

    for z in along {
        scene.push_material(
            cube,
            &Transform::at(vec3(
                (room.min.x + room.max.x) * 0.5,
                room.max.y - BEAM_DOWN * 0.5,
                z,
            ))
            .with_scale(vec3(room.max.x - room.min.x, BEAM_DOWN, BEAM)),
            crate::aim::GLASS_TRIM,
            crate::aim::MATTE,
        );
    }

    // and ribs the other way, which is what turns a run of beams into
    // coffers. Beams one way alone are stripes.
    //
    // On the colonnade's lines and the room's middle, so the ceiling is
    // divided by what is standing under it rather than by a spacing of its
    // own: three ribs make four rows of coffers across a room nine metres
    // wide.
    for x in [
        room.min.x + FRAME_IN,
        (room.min.x + room.max.x) * 0.5,
        room.max.x - FRAME_IN,
    ] {
        scene.push_material(
            cube,
            &Transform::at(vec3(
                x,
                room.max.y - BEAM_DOWN * 0.5,
                (room.min.z + room.max.z) * 0.5,
            ))
            .with_scale(vec3(BEAM, BEAM_DOWN, room.max.z - room.min.z)),
            crate::aim::GLASS_TRIM,
            crate::aim::MATTE,
        );
    }

    // a skirting where the wall meets the floor, which every other room in
    // this building has and this one did not
    for (at, size) in [
        (
            vec3(
                room.min.x + SKIRT * 0.5,
                UP + SKIRT_UP * 0.5,
                middle_z(room),
            ),
            vec3(SKIRT, SKIRT_UP, room.max.z - room.min.z),
        ),
        (
            vec3(
                room.max.x - SKIRT * 0.5,
                UP + SKIRT_UP * 0.5,
                middle_z(room),
            ),
            vec3(SKIRT, SKIRT_UP, room.max.z - room.min.z),
        ),
        (
            vec3(
                middle_x(room),
                UP + SKIRT_UP * 0.5,
                room.min.z + SKIRT * 0.5,
            ),
            vec3(room.max.x - room.min.x, SKIRT_UP, SKIRT),
        ),
        (
            vec3(
                middle_x(room),
                UP + SKIRT_UP * 0.5,
                room.max.z - SKIRT * 0.5,
            ),
            vec3(room.max.x - room.min.x, SKIRT_UP, SKIRT),
        ),
    ] {
        scene.push_material(
            cube,
            &Transform::at(at).with_scale(size),
            crate::aim::GLASS_TRIM,
            crate::aim::MATTE,
        );
    }

    for (piece, colour) in plinth_pieces(reaches) {
        scene.push_material(
            cube,
            &Transform::at(piece.center()).with_scale(piece.size()),
            colour,
            crate::aim::MATTE,
        );
    }
}

/// The plinth as it is drawn: a base, a die and a cap, with what each is.
///
/// **The die stops short of the cap rather than running up to it.** The
/// first go laid the mouldings round the plinth's own box and left the two
/// topping out at the same height, which is two faces in one plane with both
/// on show. Jake's picture of the ball on its plinth came back with a patch
/// of blocky squares on top where they were fighting, and it read as a
/// shadow.
///
/// Named rather than built where it is pushed, so the engine's own coplanar
/// check can be pointed at it. The box in `plinth` is what a body walks into
/// and what the ball stands on, and it is not this.
pub fn plinth_pieces(reaches: f32) -> Vec<(Aabb, glam::Vec4)> {
    let die = plinth(reaches);
    let wide = die.size().x;
    let at = die.center();

    [
        (die.min.y, die.min.y + 0.1, 0.13, crate::aim::GLASS_TRIM),
        (
            die.min.y + 0.1,
            die.min.y + 0.16,
            0.08,
            crate::aim::GLASS_TRIM,
        ),
        (die.min.y + 0.16, die.max.y - 0.14, 0.0, crate::aim::STAND),
        (
            die.max.y - 0.14,
            die.max.y - 0.05,
            0.11,
            crate::aim::GLASS_TRIM,
        ),
        (die.max.y - 0.05, die.max.y, 0.06, crate::aim::GLASS_TRIM),
    ]
    .iter()
    .copied()
    .map(|(from, to, out, colour)| {
        let half = (wide + out * 2.0) * 0.5;

        (
            Aabb::new(
                vec3(at.x - half, from, at.z - half),
                vec3(at.x + half, to, at.z + half),
            ),
            colour,
        )
    })
    .collect()
}

/// Draws the room, and the ball in it.
/// What this room is drawn out of.
///
/// **One thing rather than six arguments.** The room gained a turned
/// baluster, a sconce bowl and a turned column in one afternoon and the
/// draw ran past what clippy will take, which is the right moment to stop
/// adding parameters. Each is an option because a mesh is only there once
/// the renderer has it.
#[derive(Clone, Copy)]
pub struct Meshes {
    pub cube: MeshId,
    pub sphere: Option<MeshId>,
    pub baluster: Option<MeshId>,
    pub sconce: Option<MeshId>,
    pub column: Option<MeshId>,
}

pub fn draw(scene: &mut Scene, room: &crate::room::Room, since: f32, meshes: Meshes) {
    let Meshes {
        cube,
        sphere,
        baluster,
        sconce,
        column,
    } = meshes;
    let reaches = room.reaches;

    let plinth = plinth(reaches);
    // the balustrade stands in `solid` as upright boxes and is drawn as
    // joinery; the plinth stands there as one box and is drawn as a base, a
    // die and a cap. What is walked into here is not what is looked at.
    let guard = guard(reaches);
    let treads = treads(reaches);
    // the colonnade stands in `solid` as boxes, for the walk and for the
    // glass, and is drawn as turned columns
    let shafts: Vec<Aabb> = column_boxes(reaches)
        .into_iter()
        .map(|(box_, _)| box_)
        .collect();
    let named = named(reaches);

    for piece in solid(reaches)
        .into_iter()
        .chain([lid(reaches), cornice(reaches)])
        .filter(|piece| {
            !guard.contains(piece)
                && !treads.contains(piece)
                && *piece != plinth
                && !(column.is_some() && shafts.contains(piece))
        })
    {
        scene.push_material(
            cube,
            &Transform::at(piece.center()).with_scale(piece.size()),
            colour(piece, &named, UP + TALL * 0.5, since),
            crate::aim::MATTE,
        );
    }

    // and the glazing in the bays, which is what the room shows the field
    for pane in panes(reaches) {
        scene.push_material(
            cube,
            &Transform::at(pane.center()).with_scale(pane.size()),
            lit(since),
            crate::aim::VOID,
        );
    }

    draw_the_room(scene, reaches, cube, column);

    if let Some(sconce) = sconce {
        draw_the_stair(scene, reaches, since, cube, baluster, sconce);
    }

    let Some(sphere) = sphere else {
        return;
    };
    let (middle, across) = ball(reaches);

    scene.traces(&traced(room, since));
    scene.lens(
        sphere,
        &Transform::at(middle).with_scale(Vec3::splat(across * 2.0)),
        silvered(since),
        INDEX,
    );
}

/// What the glass multiplies the world it traces by.
///
/// **Because the tracer has no lamps.** Spec 0098's shader shades what a ray
/// meets with the ambient and the one directional light, which is the sun,
/// and this room is indoors where the sun does not reach and the light comes
/// from lamps. So the room inside the glass came back at the room's colours
/// times the ambient alone, which is a fifth of what the same wall reads as
/// outside the glass: the first build of this was a dark disc with a pale
/// rim and nothing in it.
///
/// **A flat multiplier was a fudge and it carried the world's own colour
/// with it.** The ambient the tracer shades by is `garden::filled`, which
/// is the sky's: at night it is three hundredths red against nine
/// hundredths blue, so everything the ball showed came back blue over a
/// room of warm stone. Jake saw that before he saw anything else about it.
///
/// It was divided out through `garden::gives_off` for an afternoon, which
/// is the move the sky, the moon and this room's own bays all make and was
/// the wrong one here. Dividing the fill out was standing in for the lamps,
/// and the tracer has the lamps now, per the engine's spec 0098: the room
/// inside the ball is lit by the same bays and newel lamps that light the
/// room outside it, with the same falloff. Kept on, the division counted
/// that light twice and the ball washed out to white.
///
/// So this is a plain tint again, and what a mirror is worth is the only
/// thing in it.
///
/// **And it is what the ball gives back face on, not everywhere.** The
/// engine's spec 0098 carries it to all of the light at a grazing angle
/// now, which is what makes a ball read as a ball: the rim is a bright ring
/// and the face is not. At ninety two hundredths there was no ring to see,
/// because there was nowhere left for the edge to climb to. This is the
/// number that sets how much of one there is.
pub const SILVER: f32 = 0.7;

pub fn silvered(_since: f32) -> glam::Vec4 {
    Vec3::splat(SILVER).extend(1.0)
}

/// What the glass bends light by.
///
/// Crown glass, which is what a paperweight is. **No dial**, per spec 0064:
/// an index on the arrow keys is the first thing anybody would reach for and
/// it is the mistake aleph is parked for.
pub const INDEX: f32 = MIRROR;

/// What the engine takes for a mirror rather than a lens, per its spec
/// 0098: an index at or under nought, which no glass has.
///
/// **The ball was a lens and read as a smear.** Jake, 2026-11-12: the
/// reflections aren't accurate, and the colour could look a lot more like a
/// mirror instead of the blue, which isn't accurate for the room either.
/// Both are the lens: it bends the room twice, turns it over and squeezes
/// it into a disc, and what fills most of a sphere's view in here is the
/// floor, so the disc came out the floor's own navy over a room of warm
/// stone. A mirror shows the room the right way up and in the colours it
/// actually has.
pub const MIRROR: f32 = -1.0;

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Walks a body with the game's own `walk`, through the whole building,
    /// and says where it got to.
    ///
    /// The whole building and not this room's boxes: a walk that only knows
    /// one room walks through the wall it shares with the next.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid = crate::room::Room::of(crate::games()).solid();
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
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

            at = next;
            falling = fell;
        }

        at
    }

    /// What the glass is given is the building's own boxes.
    ///
    /// **The claim is that there is no second account.** Every box the
    /// tracer sees has to be one `solid` or `lids` already held, because the
    /// day the two come apart is the day the room inside the ball stops
    /// being the room round it, and nothing but an eye would notice.
    #[test]
    fn it_traces_the_building_itself() {
        let room = crate::room::Room::of(crate::games());
        let mine: Vec<Aabb> = room.solid().into_iter().chain(room.lids()).collect();
        let traced = traced(&room, 0.0);

        assert!(!traced.is_empty(), "the glass is given nothing to trace");

        for one in traced.iter() {
            let box_ = Aabb::new(
                vec3(one.least[0], one.least[1], one.least[2]),
                vec3(one.most[0], one.most[1], one.most[2]),
            );

            assert!(
                mine.iter().any(|held| {
                    held.min.distance(box_.min) < 1e-3 && held.max.distance(box_.max) < 1e-3
                }),
                "the glass traces {:?} to {:?}, which the building does not hold",
                box_.min,
                box_.max
            );
        }
    }

    /// And a ray that leaves the glass and meets nothing comes back as the
    /// room's own ambient rather than as a hole.
    ///
    /// **Which is a question about the light and not about the shader.**
    /// Spec 0098's `shade` returns `tracing.ambient` on a miss, so what
    /// decides whether a miss is a hole is whether this room is ever handed
    /// a black ambient. `garden::filled` is never nought, for spec 0019's
    /// own reason, and this holds that where the glass depends on it.
    #[test]
    fn a_ray_that_hits_nothing_is_not_a_hole() {
        for n in 0..24 {
            let since = crate::garden::when(n as f32 / 24.0);
            let fill = crate::garden::filled(since);

            assert!(
                fill.min_element() > 0.01,
                "at hour {:.2} the fill is {:?}, which is a hole in the glass",
                crate::garden::hour(since),
                fill
            );
        }
    }

    /// What is behind you is in the ball, which is the claim this room is
    /// for.
    ///
    /// Stood at either end of the walk, the wall at your back is in the list
    /// the glass is given. A render to texture from a fixed plane, which is
    /// what spec 0061's corridor does, cannot say that.
    #[test]
    fn the_room_behind_you_is_in_it() {
        let reaches = reaches();
        let room = inside(reaches);
        let traced = traced(&crate::room::Room::of(crate::games()), 0.0);
        let holds = |want: f32| {
            traced.iter().any(|one| {
                let near = one.least[2].min(one.most[2]);
                let far = one.least[2].max(one.most[2]);

                near <= want + 0.4 && far >= want - 0.4
            })
        };

        assert!(holds(room.min.z), "the south end is not in the glass");
        assert!(
            holds(well(reaches).min.z),
            "the north end is not in the glass"
        );
    }

    /// Spec 0064: the building's skin leaves the bays open.
    ///
    /// **It plastered straight over them for a day.** `Room::outside`
    /// samples a column of each face and asks what is over the spot, which
    /// tells it where a wall is and nothing about where a hole in one is,
    /// so the elevation came back blank and the glazing had to stand proud
    /// of the skin to be seen at all. That face reads the wall's own spans
    /// now.
    ///
    /// Asked of the middle of each slot, because an edge is where the
    /// plaster is meant to stop and a test on an edge is a test on a
    /// rounding.
    #[test]
    fn the_skin_leaves_the_bays_open() {
        let room = crate::room::Room::of(crate::games());
        let faces = room.outside();

        for slot in slots(room.reaches) {
            let middle = slot.center();
            // **A face is a plane, and its plane stands at the wall's outer
            // face rather than in the middle of the wall.** Asked at the
            // slot's own middle this found nothing whatever the skin did,
            // and passed its own control. What it has to ask is whether a
            // face anywhere in the wall's thickness crosses the hole.
            let over = faces.iter().find(|(face, _)| {
                face.min.x >= slot.min.x - 0.01
                    && face.max.x <= slot.max.x + 0.01
                    && face.min.y < middle.y
                    && face.max.y > middle.y
                    && face.min.z < middle.z
                    && face.max.z > middle.z
            });

            assert!(
                over.is_none(),
                "the skin covers the bay at {:?}: {:?}",
                middle,
                over
            );
        }
    }

    /// Spec 0064: and the glazing stands in the opening rather than on it.
    #[test]
    fn the_glazing_is_in_the_opening() {
        let reaches = reaches();

        for (pane, slot) in panes(reaches).into_iter().zip(slots(reaches)) {
            assert!(
                pane.min.x >= slot.min.x - 1e-3 && pane.max.x <= slot.max.x + 1e-3,
                "a pane at {:?} is not inside its slot at {:?}",
                pane,
                slot
            );
        }
    }

    /// Spec 0064: one piece of the plinth draws its top, and only one.
    ///
    /// **Jake's picture of the ball on its plinth, 2026-11-12.** A patch of
    /// blocky squares on the top of it, which read as a shadow and was two
    /// surfaces fighting for the same pixels: the cap moulding was laid
    /// round the plinth's own box and both topped out at the same height.
    ///
    /// **The engine's own coplanar check cannot answer this one.** It was
    /// asked first, over the whole building the way `behind.rs` says to ask
    /// it, and it passed the broken version: that check throws out a face
    /// buried in another solid, and the die's top is inside the cap it
    /// fights. The rule holds for what you can see past and not for what is
    /// drawn, and both of these are drawn. So this asks the specific thing
    /// that broke instead of a general one.
    #[test]
    fn one_piece_of_the_plinth_draws_its_top() {
        let reaches = reaches();
        let top = plinth(reaches).max.y;
        let drawing: Vec<Aabb> = plinth_pieces(reaches)
            .into_iter()
            .map(|(box_, _)| box_)
            .filter(|box_| (box_.max.y - top).abs() < 1e-4)
            .collect();

        assert_eq!(
            drawing.len(),
            1,
            "{} pieces of the plinth top out at {:.2} and every one of them \
             is drawn, so they fight for the same pixels: {:?}",
            drawing.len(),
            top,
            drawing
        );
    }

    /// Spec 0064: the open side of the flight stops a body.
    ///
    /// **It was a three and a half metre drop with nothing on it.** You
    /// climbed fourteen steps beside a void with no rail, no string and
    /// nothing in the walk to say there was an edge. Walked rather than
    /// measured, because a rail that is tall enough and a rail a body
    /// actually stops against are two different claims and this project has
    /// been caught by the difference before.
    #[test]
    fn you_cannot_walk_off_the_side_of_the_flight() {
        let reaches = reaches();
        let steps = treads(reaches);

        // from the middle of a tread half way up, pushed hard at each side
        // in turn. Both sides, because a flight down the middle of a room is
        // open on both.
        for side in SIDES {
            let open = rake(reaches, side).0.x;

            for n in [3usize, 7, 11] {
                let tread = steps[n];
                let from = vec3(
                    (tread.min.x + tread.max.x) * 0.5,
                    tread.max.y + 0.2,
                    (tread.min.z + tread.max.z) * 0.5,
                );
                let at = walked(from, vec3(open + side * 4.0, from.y, from.z), 4.0);

                assert!(
                    (at.x - from.x).abs() < FLIGHT * 0.5,
                    "pushed at the {} side from tread {} a body ended at x {:.2}, \
                     past the balustrade at {:.2}",
                    if side < 0.0 { "west" } else { "east" },
                    n,
                    at.x,
                    open
                );
            }
        }
    }

    /// Spec 0064: and it costs the flight none of its width.
    ///
    /// `FLIGHT` is the nook's own passage, and this project's rule is 1.05
    /// clear anywhere somebody has to walk. A rail standing on the tread
    /// ends would have taken a hand's breadth off that for nothing: it
    /// stands outside the going instead, so every tread is as wide to walk
    /// as it ever was.
    #[test]
    fn the_balustrade_costs_the_flight_none_of_its_going() {
        let reaches = reaches();

        for post in guard(reaches) {
            for tread in treads(reaches) {
                let over = post.min.x < tread.max.x
                    && post.max.x > tread.min.x
                    && post.min.z < tread.max.z - 1e-4
                    && post.max.z > tread.min.z + 1e-4;

                assert!(
                    !over,
                    "a piece of the balustrade at {:?} stands on the tread at {:?}",
                    post, tread
                );
            }
        }
    }

    /// Spec 0064: nothing of the room below hangs in the flight.
    ///
    /// **The nook's pendant grid had one in the middle of it.** That ceiling
    /// was solid when the grid was laid out and this spec cut a well through
    /// it, so the lamp was left hanging from nothing at the head height of
    /// anybody climbing. It is dropped where the list is made, which is what
    /// takes its light with it.
    #[test]
    fn nothing_of_the_nook_hangs_in_the_flight() {
        let reaches = reaches();
        let flight = flight(reaches);

        for shade in crate::room::pendant_shades(reaches) {
            let inside = shade.x > flight.min.x
                && shade.x < flight.max.x
                && shade.z > flight.min.z
                && shade.z < flight.max.z;

            assert!(
                !inside,
                "a pendant hangs at {:?}, which is in the flight",
                shade
            );
        }
    }

    /// You can walk its length and all the way round the plinth.
    #[test]
    fn you_can_walk_round_it() {
        let reaches = reaches();
        let room = inside(reaches);
        let floor = well(reaches);
        let middle = middle(reaches);
        // the length of it, from the head of the stair to the far end, down
        // the aisle between the plinth and one row rather than through the
        // plinth, which is what the first go did
        let aisle = middle.x - PLINTH * 0.5 - crate::RADIUS - 0.5;
        let south = walked(
            vec3(aisle, UP + 0.2, floor.max.z - 1.0),
            vec3(aisle, UP, room.min.z),
            12.0,
        );

        assert!(
            south.z < room.min.z + 1.6,
            "walking the length ended at z {:.2} and the end is {:.2}",
            south.z,
            room.min.z
        );
        assert!(
            (south.y - UP).abs() < 0.3,
            "walking the length ended at y {:.2} rather than on the floor",
            south.y
        );

        // and round the plinth, four corners of it in turn
        let out = PLINTH * 0.5 + crate::RADIUS + 0.4;
        let ring = [
            vec3(middle.x - out, UP + 0.2, middle.z - out),
            vec3(middle.x + out, UP + 0.2, middle.z - out),
            vec3(middle.x + out, UP + 0.2, middle.z + out),
            vec3(middle.x - out, UP + 0.2, middle.z + out),
        ];
        let mut at = ring[0];

        for corner in ring.iter().cycle().skip(1).take(4) {
            at = walked(at, *corner, 4.0);

            assert!(
                at.distance(*corner) < 0.8,
                "walking round it stopped {:.2} short of {:?}",
                at.distance(*corner),
                corner
            );
        }
    }

    /// The plinth has the room's length behind it from either end of the
    /// walk, and stands against neither.
    ///
    /// **Which is the whole of what the spike found.** A sphere shows what is
    /// behind it, so a plinth at an end wall puts the room's length behind
    /// your eye.
    ///
    /// **The bar came down from five metres to four, 2026-11-12, and that is
    /// worth defending.** It was five when the room was a plain box and the
    /// ball could stand in the middle of sixteen metres. The stair runs down
    /// the middle of the room now and takes four and a half of them, so the
    /// ball cannot be in the middle: it stands in the middle of the floor
    /// the stair arrives at, and that floor is eight and a half long. Five
    /// each way is not available at any position, and buying it back would
    /// mean leaving a metre and a half of nook in front of the bottom tread,
    /// which is the fault this move was made to fix.
    ///
    /// **Measured to the room's own ends and not to the well.** The well was
    /// the room's north end once and the floor stopped at it; it is a slot
    /// in the middle now and the walls, the colonnade and the ceiling run
    /// past it, so what is behind the ball from the south is the whole room
    /// and not the floor's edge.
    #[test]
    fn the_sphere_has_length_behind_it_either_way() {
        let reaches = reaches();
        let room = inside(reaches);
        let (ball, _) = ball(reaches);
        let south = ball.z - room.min.z;
        let north = room.max.z - ball.z;

        assert!(
            south.min(north) > 4.0,
            "the shorter way to an end is {:.2} metres",
            south.min(north)
        );
        assert!(
            south > PLINTH && north > PLINTH,
            "the ball stands against an end: {:.2} and {:.2}",
            south,
            north
        );
    }

    /// The frames run the room's length at the corridor's own pitch.
    #[test]
    fn the_room_is_a_colonnade() {
        let reaches = reaches();
        let frames = frames(reaches);
        let room = inside(reaches);

        assert!(
            frames.len() >= 14,
            "{} frames is not a colonnade",
            frames.len()
        );

        // one on each side of every bay, and the bays at the pitch
        let middle = (room.min.x + room.max.x) * 0.5;
        let mut west: Vec<f32> = frames
            .iter()
            .filter(|(at, _)| at.x < middle)
            .map(|(at, _)| at.z)
            .collect();

        west.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(west.len() * 2, frames.len(), "the two sides do not match");

        for pair in west.windows(2) {
            assert!(
                (pair[1] - pair[0] - crate::mirrors::PITCH).abs() < 1e-3,
                "two frames are {:.3} apart and the pitch is {:.3}",
                pair[1] - pair[0],
                crate::mirrors::PITCH
            );
        }
    }

    /// The stair's foot is inside the nook's end wall and its head is clear
    /// of the garden's way.
    ///
    /// **Spec 0037 measured this chain and found 0.40 missing.** It assumed a
    /// flight running north; this one runs across the nook, so nothing of it
    /// is north of the end wall at all.
    #[test]
    fn the_stair_fits_the_nook() {
        let reaches = reaches();
        let nook = the_nook(reaches);
        let treads = treads(reaches);

        for (n, tread) in treads.iter().enumerate() {
            assert!(
                tread.min.x >= nook.min.x && tread.max.x <= nook.max.x,
                "tread {} runs from {:.2} to {:.2} and the nook is {:.2} to {:.2}",
                n,
                tread.min.x,
                tread.max.x,
                nook.min.x,
                nook.max.x
            );
            assert!(
                tread.max.z <= nook.max.z,
                "tread {} reaches z {:.2} and the nook ends at {:.2}",
                n,
                tread.max.z,
                nook.max.z
            );
        }

        let top = treads.last().expect("a flight has treads");

        assert!(
            (top.max.y - UP).abs() < 1e-3,
            "the flight arrives at {:.3} and the floor is at {:.3}",
            top.max.y,
            UP
        );
    }

    /// And the room stands over the nook and over nothing else.
    #[test]
    fn nothing_of_the_building_is_under_it() {
        let reaches = reaches();
        let over = over(reaches);
        let nook = the_nook(reaches);

        assert!(
            over.min.x >= nook.min.x - 1e-3
                && over.max.x <= nook.max.x + 1e-3
                && over.min.z >= nook.min.z - 1e-3
                && over.max.z <= nook.max.z + 1e-3,
            "the room covers {:?} to {:?} and the nook is {:?} to {:?}",
            over.min,
            over.max,
            nook.min,
            nook.max
        );
    }
}
