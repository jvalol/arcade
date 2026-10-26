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
use glam::{vec3, Quat, Vec3};

use crate::room::THICK;

/// Where the room sits: over the hall, which is the only place in this
/// building with nothing above it.
///
/// Measured rather than chosen. Every room here is between y -4.5 and y 4.5,
/// and nothing at all is above that.
pub const MIDDLE_X: f32 = -2.75;
pub const MIDDLE_Z: f32 = -3.5;
pub const BASE: f32 = 6.0;

/// How big the room is inside.
///
/// **A cube.** Spec 0038 puts three floors in here at right angles to each
/// other and they want to be the same floor, which a room taller or
/// shorter than it is wide cannot give them. It was 8.0 high for 0037's
/// loop, which wanted headroom over a flight and nothing else.
pub const ROOM: f32 = 12.5;
pub const TALL: f32 = ROOM;
pub const HALF: f32 = ROOM * 0.5;

/// The middle of the room, which everything in here is measured from.
///
/// The three frames are the same layout turned, so they need one point to
/// turn about, and this is it.
pub fn middle() -> Vec3 {
    vec3(MIDDLE_X, BASE + HALF, MIDDLE_Z)
}

/// How many ways there are to be the right way up in here. Spec 0038.
///
/// Three, and Jake's reason is that a computer drawing three dimensional
/// rooms is not fundamentally different from a drawing by hand on paper, so
/// what fills a picture is what fills this.
pub const FRAMES: u64 = 3;

/// Frame `n`'s own up, as the turn that carries this room's up on to it.
///
/// **The three cyclic turns of the axes**, which is a third of a turn about
/// the room's long diagonal. Nought leaves them alone, one sends x to z and
/// y to x, two sends x to y and y to z.
///
/// Two things fall out of that and both are the reason for it.
///
/// A box stays a box. Every collider in this building is axis aligned, and
/// a rotation that only permutes the axes leaves one alone, so three frames
/// cost no new kind of collider at all.
///
/// And the three flights climb to three different surfaces. Quarter turns
/// about single axes were tried first: they put one frame's door in the
/// floor, which is a hole you fall down, and gave two other frames the same
/// wall. Under these, up is y, x and z and the doors are in the east wall,
/// the north wall and the ceiling.
pub fn frame(n: u64) -> Quat {
    let diagonal = Vec3::ONE.normalize();
    let third = std::f32::consts::TAU / 3.0;

    match n % FRAMES {
        0 => Quat::IDENTITY,
        1 => Quat::from_axis_angle(diagonal, -third),
        _ => Quat::from_axis_angle(diagonal, third),
    }
}

/// A point in frame `n`'s own terms, put into the world.
///
/// Frame nought's terms are the world's, turned so that nought nought nought
/// is the middle of the room rather than its corner.
pub fn in_frame(n: u64, local: Vec3) -> Vec3 {
    middle() + frame(n) * local
}

/// A box in frame `n`'s own terms, put into the world.
///
/// Exact rather than a bounding box, because the turn is a right angle: the
/// two corners come back as two corners of the same box with the names
/// swapped, so taking the smaller and larger of each pair is the box itself.
pub fn box_in_frame(n: u64, low: Vec3, high: Vec3) -> Aabb {
    let (a, b) = (in_frame(n, low), in_frame(n, high));

    Aabb::new(a.min(b), a.max(b))
}

/// One flight: how wide, how deep a tread, how high a rise, and how many.
///
/// **A stair you can read as a stair**, which is the whole reason 0038
/// exists. 0037's loop rose 0.15 over a metre of going, which is eight and
/// a half degrees, and at that pitch the risers are invisible and what is
/// left is a row of shelves. This is 32 degrees, which is a staircase.
pub const STAIR_WIDE: f32 = 1.6;
pub const STAIR_GOING: f32 = 0.4;
pub const STAIR_RISE: f32 = 0.25;
pub const STAIR_STEPS: u64 = 12;
pub const STAIR_RUN: f32 = STAIR_GOING * STAIR_STEPS as f32;
pub const STAIR_CLIMB: f32 = STAIR_RISE * STAIR_STEPS as f32;

/// How far across its own floor a flight stands.
///
/// Off the middle, so that the three of them turn into three different
/// parts of the room rather than three copies of the same part. A test says
/// so rather than the eye.
pub const STAIR_ACROSS: f32 = -2.6;

/// The door each flight climbs to: how wide, how tall, and how deep the
/// dark behind it is.
///
/// **The flights have to go somewhere.** A staircase ending at a blank wall
/// is a ramp with steps on it, and the thing that makes Escher's room read
/// is that every flight is plainly on its way to a door somebody uses.
pub const DOOR_WIDE: f32 = STAIR_WIDE;
pub const DOOR_TALL: f32 = 2.0;
pub const DOOR_BACK: f32 = 0.5;

/// Every tread of frame `n`'s flight, in the world.
///
/// It climbs to the wall rather than stopping short of it, so the top tread
/// is the threshold of the door and there is no landing to explain.
pub fn treads(n: u64) -> Vec<Aabb> {
    (0..STAIR_STEPS)
        .map(|step| {
            let along = HALF - STAIR_RUN + STAIR_GOING * step as f32;
            let top = -HALF + STAIR_RISE * (step as f32 + 1.0);

            box_in_frame(
                n,
                vec3(along, -HALF, STAIR_ACROSS),
                vec3(along + STAIR_GOING, top, STAIR_ACROSS + STAIR_WIDE),
            )
        })
        .collect()
}

/// How many doors a frame has: one at the top of its flight and one at the
/// far end of its floor.
///
/// **Two, because one is a dead end.** With a door only at the top, the
/// only journey anybody can make is up and back down again, and Jake's
/// first word on the built room was that the characters just go back and
/// forth on the same staircase. They were, and no amount of staggering
/// fixes a path with one end. People in a building are going somewhere.
pub const DOORS: u64 = 2;

/// The hole frame `n`'s door `which` makes.
///
/// Nought is at the top of the flight, in the wall it climbs to. One is at
/// the far end of the same frame's floor, square to it, so the two of them
/// are the ends of a walk rather than the same wall twice.
pub fn doorway(n: u64, which: u64) -> Aabb {
    let across = STAIR_ACROSS + STAIR_WIDE * 0.5;

    match which % DOORS {
        0 => {
            let foot = -HALF + STAIR_CLIMB;

            box_in_frame(
                n,
                vec3(HALF - THICK, foot, across - DOOR_WIDE * 0.5),
                vec3(HALF + THICK, foot + DOOR_TALL, across + DOOR_WIDE * 0.5),
            )
        }
        // **Across the floor from the foot of the flight, not in line with
        // it.** Lined up, frame nought's stood on where the way up arrives
        // and walled it off, which the walk test found and the test meant
        // to guard that arrival missed, because it only read the treads.
        //
        // Putting the dark behind a door past its opening rather than
        // round it, which was a separate fault found the same hour, fixes
        // that one too, so this is no longer the only thing holding it. It
        // stays because of what it does to the walk: a floor leg that
        // crosses the room is somebody walking in the open, and in line it
        // was two paces and a door.
        _ => box_in_frame(
            n,
            vec3(-HALF + DOOR_WIDE, -HALF, HALF - THICK),
            vec3(-HALF + DOOR_WIDE * 2.0, -HALF + DOOR_TALL, HALF + THICK),
        ),
    }
}

/// Which way a doorway faces: the axis it is thin along, and which way
/// along that axis is out of the room.
fn door_faces(hole: &Aabb) -> (usize, f32) {
    let size = hole.size().to_array();
    let axis = (0..3)
        .min_by(|a, b| size[*a].total_cmp(&size[*b]))
        .expect("three axes");
    let sign = (hole.center() - middle()).to_array()[axis].signum();

    (axis, sign)
}

/// The recess behind frame `n`'s door `which`: a back and four sides.
///
/// **Closed, not a panel hung behind the hole.** One slab with daylight all
/// round it was the first build, and through the opening you saw a lit
/// surface square to the sun, which came out *brighter* than the wall it
/// was meant to be a hole in. Jake looked at the room and said the portals
/// the people walk through look like blank walls. They did: there is
/// nothing about a pale rectangle on a pale wall that says doorway.
///
/// Boxed in, the only light that gets in is the room's own, through the
/// opening, which is what makes a doorway dark.
pub fn door_recess(n: u64, which: u64) -> Vec<Aabb> {
    let hole = doorway(n, which);
    let (axis, sign) = door_faces(&hole);
    let (mut low, mut high) = (hole.min.to_array(), hole.max.to_array());

    // the recess runs from the wall's far face outward
    if sign > 0.0 {
        low[axis] = high[axis];
        high[axis] = low[axis] + DOOR_BACK;
    } else {
        high[axis] = low[axis];
        low[axis] = high[axis] - DOOR_BACK;
    }

    let mut out = Vec::new();

    // the back of it, grown to cover the ends of the four sides
    let (mut back_low, mut back_high) = (low, high);
    if sign > 0.0 {
        back_low[axis] = high[axis];
        back_high[axis] = high[axis] + THICK;
    } else {
        back_high[axis] = low[axis];
        back_low[axis] = low[axis] - THICK;
    }
    for k in 0..3 {
        if k != axis {
            back_low[k] -= THICK;
            back_high[k] += THICK;
        }
    }
    out.push(Aabb::new(Vec3::from(back_low), Vec3::from(back_high)));

    // and its four sides
    for k in 0..3 {
        if k == axis {
            continue;
        }

        for end in 0..2 {
            let (mut side_low, mut side_high) = (low, high);
            if end == 0 {
                side_high[k] = low[k];
                side_low[k] = low[k] - THICK;
            } else {
                side_low[k] = high[k];
                side_high[k] = high[k] + THICK;
            }
            out.push(Aabb::new(Vec3::from(side_low), Vec3::from(side_high)));
        }
    }

    out
}

/// How far the casing round a door stands proud of its wall, and how wide
/// it is.
///
/// **A doorway needs an edge.** A hole cut clean into a flat wall has
/// nothing to catch the light along, so from any distance it is a change of
/// shade and not an opening. An architrave is the oldest fix there is and
/// it is four boxes.
pub const CASE_OUT: f32 = 0.07;
pub const CASE_WIDE: f32 = 0.17;

/// The casing round frame `n`'s door `which`, on the side you see it from.
pub fn door_case(n: u64, which: u64) -> Vec<Aabb> {
    let hole = doorway(n, which);
    let (axis, sign) = door_faces(&hole);
    let (mut low, mut high) = (hole.min.to_array(), hole.max.to_array());

    // it stands on the room's side of the wall, which is the near face
    if sign > 0.0 {
        high[axis] = low[axis] + CASE_OUT;
        low[axis] -= CASE_OUT * 0.4;
    } else {
        low[axis] = high[axis] - CASE_OUT;
        high[axis] += CASE_OUT * 0.4;
    }

    let mut out = Vec::new();
    // **No sill.** A casing is two uprights and a head, which is what a
    // real one is and what this one has to be: the piece along the bottom
    // sat in the plane of the tread you step off, and the engine's own
    // coplanar check named it as a fight 0.10 by 1.60. Jake saw it as
    // banding on the floor.
    let down = (frame(n) * Vec3::NEG_Y).to_array();

    for k in 0..3 {
        if k == axis {
            continue;
        }

        for end in 0..2 {
            let toward = if end == 0 { -1.0 } else { 1.0 };
            if down[k] * toward > 0.5 {
                continue;
            }

            let (mut side_low, mut side_high) = (low, high);
            if end == 0 {
                side_high[k] = low[k];
                side_low[k] = low[k] - CASE_WIDE;
            } else {
                side_low[k] = high[k];
                side_high[k] = high[k] + CASE_WIDE;
            }

            // **Only the head oversails.** Growing the uprights as well
            // laps them into it at the corners, and two drawn faces that
            // lap in one plane speckle: the engine's check reported four
            // of those, one per corner. The head reaching over the
            // uprights is what a casing does anyway.
            if down[k].abs() > 0.5 {
                let other = (0..3).find(|m| *m != axis && *m != k).expect("a third");
                side_low[other] -= CASE_WIDE;
                side_high[other] += CASE_WIDE;
            }

            out.push(Aabb::new(Vec3::from(side_low), Vec3::from(side_high)));
        }
    }

    out
}

/// Where frame `n`'s own lamp hangs, which is from its own ceiling.
///
/// **One light per frame, and each one is overhead in its own terms.** A
/// room lit from a single direction tells you which way is really up, which
/// is the one thing this room must not say. Lit from three, no surface is
/// the lit one.
pub const LAMP_DOWN: f32 = 1.0;
pub const LAMP_RANGE: f32 = 11.0;
pub const LAMP_LIT: f32 = 0.55;

pub fn sconces() -> Vec<Vec3> {
    (0..FRAMES)
        .map(|n| {
            in_frame(
                n,
                vec3(
                    HALF - STAIR_RUN * 0.5,
                    HALF - LAMP_DOWN,
                    STAIR_ACROSS + STAIR_WIDE * 0.5,
                ),
            )
        })
        .collect()
}

/// And the lamps, which are those.
pub fn lamps() -> Vec<Vec3> {
    sconces()
}

/// Somebody on one of the flights: where they are, which way is up to them,
/// which way they face in their own terms, and where they are in their
/// stride.
pub struct Walker {
    pub at: Vec3,
    pub up: Quat,
    pub facing: f32,
    pub along: f32,
    pub shade: usize,
}

/// How many walk each flight, and how fast.
pub const WALKERS: u64 = 4;
pub const WALKS: f32 = 0.9;

/// How far one pace carries somebody, which is what turns metres walked into
/// legs swung.
pub const STRIDE: f32 = 0.7;

/// How long somebody is out of sight between journeys.
pub const WAITS: f32 = 3.0;

/// The walk through a frame, in that frame's own terms: out of the door at
/// the top of the flight, down it, along the floor, and out of the door at
/// the end of that.
///
/// Three corners rather than two, because the turn at the foot is the thing
/// that says they are going somewhere rather than pacing.
pub fn journey() -> [Vec3; 4] {
    let across = STAIR_ACROSS + STAIR_WIDE * 0.5;
    let over = -HALF + DOOR_WIDE * 1.5;

    [
        vec3(HALF, -HALF + STAIR_CLIMB, across),
        vec3(HALF - STAIR_RUN, -HALF, across),
        vec3(over, -HALF, across),
        vec3(over, -HALF, HALF - THICK),
    ]
}

/// The floor leg of that walk, laid on the floor as a path.
///
/// **A person on a blank wall reads as floating.** The first build with
/// journeys in it had people crossing the other two frames' floors, which
/// are this room's walls and ceiling, and from the middle they looked like
/// debris hanging in the air: nothing about a blank surface says it is a
/// floor, so nothing about somebody on it says they are standing. A worn
/// strip under them says both, which is spec 0020's trick in the hall.
pub fn path(n: u64) -> Vec<Aabb> {
    let walk = journey();

    // **Square legs and not one diagonal.** A box cannot be a strip laid
    // across a corner: the first build ran the floor leg from the foot of
    // the flight straight to the door, and the box round that diagonal was
    // a seven metre slab stuck to the wall. An L is two strips.
    walk.windows(2)
        .skip(1)
        .map(|leg| {
            let (low, high) = (leg[0].min(leg[1]), leg[0].max(leg[1]));

            box_in_frame(
                n,
                vec3(low.x - PATH_WIDE * 0.5, -HALF, low.z - PATH_WIDE * 0.5),
                vec3(
                    high.x + PATH_WIDE * 0.5,
                    -HALF + PATH_DEEP,
                    high.z + PATH_WIDE * 0.5,
                ),
            )
        })
        .collect()
}

/// How wide that path is and how proud of the floor it stands.
///
/// Proud rather than sunk, because a strip laid *in* a floor is two faces
/// in one plane, which is the fault this building has fought nine times.
pub const PATH_WIDE: f32 = 1.4;
pub const PATH_DEEP: f32 = 0.04;

/// How long that walk is, corner to corner.
pub fn journey_long() -> f32 {
    journey()
        .windows(2)
        .map(|leg| leg[0].distance(leg[1]))
        .sum()
}

/// Everybody in the room, at `since` seconds.
///
/// **Each of them is going somewhere and then is gone.** They used to walk
/// up the flight and back down it for ever, which Jake named the first time
/// he saw the room built: the characters just go back and forth on the same
/// staircase. They did, and no amount of staggering fixes a path with one
/// end. This one has a door at each end, so somebody comes out of one,
/// crosses the room and leaves by the other.
///
/// **Popping happens in a doorway and nowhere else.** A figure appearing in
/// the open reads as the building being broken, which is the one reading
/// this room cannot afford, and a door is the thing that makes an arrival
/// ordinary.
///
/// Half of them walk it the other way, so the traffic goes both ways and
/// people pass each other on the stair.
pub fn walking(since: f32) -> Vec<Walker> {
    let path = journey();
    let long = journey_long();
    let round = long / WALKS + WAITS;
    let mut out = Vec::new();

    for n in 0..FRAMES {
        for who in 0..WALKERS {
            // spread along the round, and the frames out of step with each
            // other, so the room is never three of the same thing at once
            let off = round * (who as f32 / WALKERS as f32 + n as f32 * 0.11);
            let walked = (since + off).rem_euclid(round) * WALKS;

            // out of sight, which is the wait past the end of the walk
            if walked > long {
                continue;
            }

            // and the odd ones take it from the other end
            let down = (n + who) % 2 == 0;
            let gone = if down { walked } else { long - walked };

            let mut left = gone;
            let mut leg = 0;
            while leg + 2 < path.len() && left > path[leg].distance(path[leg + 1]) {
                left -= path[leg].distance(path[leg + 1]);
                leg += 1;
            }
            let (from, to) = (path[leg], path[leg + 1]);
            let at = from.lerp(to, (left / from.distance(to)).clamp(0.0, 1.0));
            let way = (to - from).normalize_or(Vec3::X) * if down { 1.0 } else { -1.0 };

            // their feet stand on the tread under them rather than on the
            // slope between two, or they ride the flight like a ramp
            let at = if at.y > -HALF + 1e-4 {
                // clamped, because at the very top they are standing on
                // the threshold of the door rather than on a thirteenth
                // tread of a flight with twelve
                let step = ((at.x - (HALF - STAIR_RUN)) / STAIR_GOING)
                    .floor()
                    .clamp(0.0, STAIR_STEPS as f32 - 1.0);

                vec3(at.x, -HALF + (step + 1.0) * STAIR_RISE, at.z)
            } else {
                at
            };

            out.push(Walker {
                at: in_frame(n, at),
                up: frame(n),
                // yaw nought faces -z and grows toward +x
                facing: way.x.atan2(-way.z),
                // `people::swing` takes a fraction of a crossing and puts
                // `people::STEPS` paces in it, so this is distance walked
                // over the length of that many strides
                along: (since + off) * WALKS / (STRIDE * crate::people::STEPS),
                shade: (n * WALKERS + who) as usize,
            });
        }
    }

    out
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

/// The way up as it is drawn, which is the way up without its lap.
///
/// **Spec 0014's own answer, and spec 0011 got there first.** The landing
/// reaches a tread back under the last of them so that two walking surfaces
/// never meet in a plane, which `walk` needs and which leaves both of them
/// with a face at the same `z`. Two drawn faces in one plane speckle. So
/// the lap is in the colliders, where it is needed, and out of the drawing,
/// where it costs: the space behind the wall settled the same thing the
/// same way, with its floor lapping the hall's in `solid` and stopping
/// short of it in the picture.
pub fn up_drawn() -> Vec<Aabb> {
    let head = up_head();

    up_treads()
        .into_iter()
        .map(|box_| {
            if box_.size().z <= UP_GOING + 1e-4 || box_.min.z > head - 1e-4 {
                return box_;
            }

            Aabb::new(vec3(box_.min.x, box_.min.y, head), box_.max)
        })
        .collect()
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

/// The stair's own plan, as the two rectangles the well and the shaft are
/// both built on.
///
/// The first is the top of the first flight and the landing, from where
/// your head would be in the tiles. The second is the whole of the second
/// flight, all the way to where it arrives.
fn up_plan() -> Vec<Aabb> {
    let (west, head, foot) = (up_west(), up_head(), up_foot());
    // **From under the ceiling to this room's floor, which is the widest
    // either of the two things built on this wants.** Where the well stops
    // and where the shaft starts are different numbers and each is said
    // where that thing is built. They were one number here, half overridden
    // downstream, and three faults came out of it in a day: the shaft hung
    // below the ceiling bars, then speckled against them, then the well
    // inherited the shaft's bottom and left two centimetres of every bar
    // hanging in the stairwell. `room::minus` cuts in all three
    // directions, so a hole that stops inside a ceiling shaves it rather
    // than opening it.
    let (under, over) = (
        crate::room::TALL - crate::room::CEIL_DROP * 2.0,
        BASE - THICK,
    );

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

    // **Inside a tile's own thickness.** Level with the bars' underside the
    // shaft speckled against them, and level with the tiles' it speckles
    // against those, so it begins between the two faces a tile has, where
    // it is hidden and shares a plane with nothing.
    //
    // **From the tiles and not from the bars.** This was `soffit` plus a
    // share of the drop, which is eleven millimetres under the tiles, and
    // the shaft hung that far below the hall's ceiling the whole length of
    // the second flight: a pale slab in the one ceiling spec 0012 made
    // worth looking up at. The height a tile hangs at is `room`'s to say.
    let under = crate::room::tile_middle();

    for (n, well) in wells.iter().enumerate() {
        let (low, high) = (vec3(well.min.x, under, well.min.z), well.max);
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

            // **And out of the pieces already kept.** The two rings meet at
            // a corner and lap there, so two drawn faces ended up in one
            // plane three times over, which the engine's check reported and
            // which speckles on the way up. Disjoint, there is nothing for
            // two faces to share.
            for piece in pieces {
                let mut left = vec![piece];

                for kept in out.iter() {
                    left = left
                        .into_iter()
                        .flat_map(|one| crate::room::minus(one, *kept))
                        .collect();
                }

                out.extend(left);
            }
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

    // the three doors come out of the shell, whichever face each is in: one
    // is in the east wall, one in the north wall and one in the ceiling
    for n in 0..FRAMES {
        for which in 0..DOORS {
            let hole = doorway(n, which);
            out = out
                .into_iter()
                .flat_map(|box_| crate::room::minus(box_, hole))
                .collect();
        }
    }

    // then every frame's flight, and the dark behind every frame's door,
    // which are put in after the cutting rather than before it
    for n in 0..FRAMES {
        out.extend(treads(n));

        for which in 0..DOORS {
            out.extend(door_recess(n, which));
            out.extend(door_case(n, which));
        }
    }

    out.extend(way_up());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Nothing of the loop lies in the plane of the floor.
    ///
    /// Two faces in one plane speckle as you move, and this building has
    /// fought that nine times. The first landing had its top face at exactly
    /// the floor's height and the first picture of the room showed it round
    /// the foot of the cone.
    #[test]
    fn no_flight_is_in_its_own_floors_plane() {
        for n in 0..FRAMES {
            let floor = in_frame(n, vec3(0.0, -HALF, 0.0));
            let down = frame(n) * Vec3::Y;

            for tread in treads(n) {
                // the lowest tread's underside is the floor, which is one
                // box inside another and hidden. What must not happen is a
                // tread's *top* landing in the floor's plane.
                let top = (tread.center() - floor).dot(down) + tread.size().dot(down.abs()) * 0.5;

                assert!(
                    top > 1e-3,
                    "a tread of frame {} tops out in its own floor, {:.4} off it",
                    n,
                    top
                );
            }
        }
    }

    /// Spec 0014 and the engine's 0048: nothing drawn in these two rooms
    /// shares a plane with anything else drawn.
    ///
    /// **Asked of the engine rather than of the eye.** Jake sent three
    /// screenshots of this room and the hall under it with speckle in them,
    /// and the check named every one: a casing sill in the plane of the
    /// tread you step off, four casing corners where an upright lapped its
    /// head, the shaft's underside level with the ceiling bars it comes
    /// through, the landing's lap, and the two shaft rings meeting at a
    /// corner. Twelve in here, and none of them needed walking to.
    ///
    /// Over what is drawn and not over what is walked, which is the whole
    /// difference: `up_treads` laps its landing on purpose so no two
    /// walking surfaces meet in a plane, and that lap is meant to be in the
    /// colliders and out of the picture.
    #[test]
    fn nothing_drawn_in_here_is_in_one_plane_with_anything() {
        use blitzkit::renderer::coplanar::Fights;

        let room = crate::room::Room::of(crate::games());
        let looking = blitzkit::renderer::depth::Depth::of(&blitzkit::camera::Camera::default());

        let mut drawn: Vec<Aabb> = room
            .solid()
            .into_iter()
            .filter(|box_| !up_treads().contains(box_))
            .collect();
        drawn.extend(up_drawn());

        let mine = Aabb::new(
            middle() - Vec3::splat(HALF + 1.5),
            middle() + Vec3::splat(HALF + 1.5),
        );
        let way = Aabb::new(
            vec3(up_west() - 1.0, -1.0, up_foot() - 1.0),
            vec3(up_arrival() + 1.0, BASE + 1.0, up_head() + UP_WIDE + 1.0),
        );

        for fight in Fights::of(drawn, 0, 0, &looking).fights.iter() {
            let at = fight.patch.center();

            assert!(
                !mine.contains_point(at) && !way.contains_point(at),
                "two drawn faces share a plane at {:?}, over {:.2} by {:.2}",
                at,
                fight.patch.size().x.max(fight.patch.size().y),
                fight.patch.size().z.max(fight.patch.size().y)
            );
        }
    }

    /// Spec 0012: the shaft meets the nook's ceiling inside its thickness,
    /// not level with either face of it and not hanging below it.
    ///
    /// **The engine's check cannot see this one.** It reads boxes, and a
    /// suspended ceiling is drawn and never collided, so the bars are in no
    /// list it is given. Both ways of getting this wrong have shipped: the
    /// shaft started a tile's drop under the bars and hung across the hall
    /// as a white beam, then level with their underside and speckled along
    /// every edge. Jake found each by looking up.
    #[test]
    fn the_shaft_meets_the_ceiling_inside_its_thickness() {
        let room = crate::room::Room::of(crate::games());
        let (bars, tiles) = room.lattice();
        let hung = |pieces: &Vec<crate::room::Hung>| {
            pieces
                .iter()
                .fold((f32::MAX, f32::MIN), |(low, high), (at, size)| {
                    (low.min(at.y - size.y * 0.5), high.max(at.y + size.y * 0.5))
                })
        };

        let (bar_low, bar_high) = hung(&bars);
        let (tile_low, tile_high) = hung(&tiles);
        let low = bar_low.min(tile_low);
        let high = bar_high.max(tile_high);

        for (n, wall) in up_shaft().iter().enumerate() {
            assert!(
                wall.min.y > low + 1e-3 && wall.min.y < high - 1e-3,
                "shaft wall {} starts at {:.4}, and the ceiling it comes \
                 through is {:.4} to {:.4}",
                n,
                wall.min.y,
                low,
                high
            );
        }
    }

    /// Spec 0038: a flight here reads as a flight.
    ///
    /// **The whole reason 0038 exists.** 0037's loop rose 0.15 over a metre
    /// of going, which is eight and a half degrees, and at that pitch the
    /// risers are invisible and what is left is a row of shelves. Taken off
    /// the built boxes rather than off the two constants, because what gets
    /// drawn is the boxes.
    #[test]
    fn a_flight_reads_as_a_flight() {
        let treads = treads(0);
        let (low, high) = (treads[0], treads[treads.len() - 1]);
        let rise = high.max.y - low.max.y;
        let run = high.center().x - low.center().x;
        let degrees = (rise / run).atan().to_degrees();

        assert!(
            (25.0..45.0).contains(&degrees),
            "the flight climbs at {:.1} degrees, which is a ramp or a ladder",
            degrees
        );
    }

    /// Spec 0038: the three frames are at right angles to each other.
    #[test]
    fn the_three_frames_are_at_right_angles() {
        let ups: Vec<Vec3> = (0..FRAMES).map(|n| frame(n) * Vec3::Y).collect();

        for (a, one) in ups.iter().enumerate() {
            for (b, other) in ups.iter().enumerate() {
                if a == b {
                    continue;
                }

                assert!(
                    one.dot(*other).abs() < 1e-5,
                    "frame {} and frame {} are {:.3} apart rather than square",
                    a,
                    b,
                    one.dot(*other)
                );
            }
        }
    }

    /// Spec 0038: every frame is a quarter turn, so a box stays a box.
    ///
    /// Every collider in this building is axis aligned and a right angle is
    /// the only rotation that leaves one of those alone. This is what makes
    /// three frames buildable here at all, so it is worth a test rather
    /// than a comment.
    #[test]
    fn a_box_turned_into_any_frame_is_still_a_box() {
        let (low, high) = (vec3(-1.0, -2.0, -3.0), vec3(1.5, 2.5, 3.5));
        let mut want = (high - low).to_array();
        want.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for n in 0..FRAMES {
            let mut got = box_in_frame(n, low, high).size().to_array();
            got.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for (one, other) in want.iter().zip(got.iter()) {
                assert!(
                    (one - other).abs() < 1e-4,
                    "frame {} turned a {:?} box into a {:?} one",
                    n,
                    want,
                    got
                );
            }
        }
    }

    /// Spec 0038: no two frames stand in each other.
    ///
    /// The three are the same layout turned, so whether they collide is
    /// decided entirely by how far off its own middle a flight stands, and
    /// that is a number somebody picked. A test says it was picked right.
    #[test]
    fn no_two_frames_stand_in_each_other() {
        let all = |n: u64| {
            let mut out = treads(n);
            out.extend((0..DOORS).map(|which| doorway(n, which)));
            out
        };

        for a in 0..FRAMES {
            for b in 0..FRAMES {
                if a >= b {
                    continue;
                }

                for mine in all(a) {
                    for theirs in all(b) {
                        assert!(
                            !mine.intersects(&theirs),
                            "frames {} and {} overlap at {:?}",
                            a,
                            b,
                            mine.center()
                        );
                    }
                }
            }
        }
    }

    /// Spec 0038: every flight is inside the room it is in.
    #[test]
    fn every_flight_is_inside_the_room() {
        let room = Aabb::new(middle() - Vec3::splat(HALF), middle() + Vec3::splat(HALF));

        for n in 0..FRAMES {
            for (step, tread) in treads(n).iter().enumerate() {
                assert!(
                    room.contains_point(tread.min) && room.contains_point(tread.max),
                    "tread {} of frame {} is outside the room at {:?}",
                    step,
                    n,
                    tread.min
                );
            }
        }
    }

    /// Spec 0038: every flight climbs to a door, and the door is a hole.
    ///
    /// **A staircase that ends at a blank wall is a ramp with steps on it.**
    /// What makes Escher's room read is that every flight is plainly on its
    /// way to a door somebody uses, so the opening being real rather than
    /// painted on is the claim.
    #[test]
    fn every_flight_climbs_to_a_hole_in_the_wall() {
        let solid = solid();

        for n in 0..FRAMES {
            let top = treads(n)[STAIR_STEPS as usize - 1];

            assert!(
                top.intersects(&doorway(n, 0)),
                "frame {}'s top tread does not reach its own door",
                n
            );

            for which in 0..DOORS {
                let door = doorway(n, which);

                // a body standing in the opening is standing in nothing
                let through = door.center();
                for box_ in solid.iter() {
                    assert!(
                        !box_.contains_point(through),
                        "frame {}'s door {} is still wall: {:?} to {:?}",
                        n,
                        which,
                        box_.min,
                        box_.max
                    );
                }

                // with something behind it rather than the outside
                assert!(
                    solid
                        .iter()
                        .any(|box_| door_recess(n, which).iter().any(|part| part == box_)),
                    "frame {}'s door {} opens on to nothing at all",
                    n,
                    which
                );
            }
        }
    }

    /// Spec 0038: every frame's lamp hangs from that frame's own ceiling.
    ///
    /// **One light per frame and each one overhead in its own terms.** A
    /// room lit from one direction tells you which way is really up, which
    /// is the one thing this room must not say.
    #[test]
    fn every_frame_is_lit_from_its_own_ceiling() {
        let sconces = sconces();

        assert_eq!(sconces.len(), FRAMES as usize, "a frame with no light");
        assert_eq!(sconces, lamps(), "a lamp that is not in a fitting");

        for (n, at) in sconces.iter().enumerate() {
            let up = frame(n as u64) * Vec3::Y;
            let over = (*at - middle()).dot(up);

            assert!(
                over > HALF - LAMP_DOWN * 2.0,
                "frame {}'s lamp is {:.2} over its middle, not up at its ceiling",
                n,
                over
            );

            // and it is over its own flight rather than anywhere up there
            let flight: Vec<Aabb> = treads(n as u64);
            let under = Aabb::new(*at - up * ROOM - Vec3::splat(0.6), *at + Vec3::splat(0.6));
            assert!(
                flight.iter().any(|tread| under.intersects(tread)),
                "frame {}'s lamp lights no part of its own flight",
                n
            );
        }
    }

    /// Spec 0038: somebody is always about, and nobody walks on air.
    ///
    /// **Not everybody all the time.** They used to be a fixed cast pacing
    /// a flight, so counting them was the test. Now each of them walks a
    /// journey with a door at either end and is out of sight between
    /// journeys, so what can be claimed is that the room is never empty,
    /// that every frame gets used, and that nobody is ever between a tread
    /// and their own floor.
    #[test]
    fn somebody_is_always_about_and_nobody_walks_on_air() {
        let mut seen = vec![0; FRAMES as usize];

        for tick in 0..600 {
            let since = tick as f32 * 0.1;
            let who = walking(since);

            assert!(!who.is_empty(), "the room emptied at {:.2}s", since);

            for walker in who.iter() {
                let on = walker.shade as u64 / WALKERS;
                seen[on as usize] += 1;

                let under = Aabb::new(walker.at - Vec3::splat(0.06), walker.at + Vec3::splat(0.06));
                let floor = in_frame(on, vec3(0.0, -HALF, 0.0));
                let up = frame(on) * Vec3::Y;
                let off_the_floor = (walker.at - floor).dot(up);

                assert!(
                    off_the_floor.abs() < 0.06
                        || treads(on).iter().any(|tread| tread.intersects(&under)),
                    "at {:.2}s somebody on frame {} is {:.3} off their own floor and on no tread",
                    since,
                    on,
                    off_the_floor
                );
                assert!(
                    walker.up.is_normalized(),
                    "somebody on frame {} has no way up",
                    on
                );
            }
        }

        for (n, count) in seen.iter().enumerate() {
            assert!(*count > 0, "nobody ever used frame {}", n);
        }
    }

    /// Spec 0038: the one flight that is the right way up for you is one you
    /// can actually walk.
    ///
    /// You keep your own down and cannot change it, so two of the three
    /// flights are somebody else's. The third is yours and a stair you
    /// cannot climb in a room about climbing is worth a test.
    #[test]
    fn you_can_walk_up_your_own_frames_flight() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let flight = treads(0);
        let foot = flight[0];

        let start = vec3(foot.min.x - 0.8, BASE + 0.6, foot.center().z);
        let (mut at, mut falling) = (start, 0.0);
        for _ in 0..420 {
            let (next, fell) = crate::walk::walk(
                at,
                Vec3::X * crate::SPEED,
                falling,
                crate::RADIUS,
                1.0 / 60.0,
                &solid,
                true,
            );
            at = next;
            falling = fell;
        }

        let top = flight[flight.len() - 1].max.y;
        assert!(
            at.y > top - STAIR_RISE * 2.0,
            "walking at your own frame's flight ended at y {:.3} and its top is {:.3}",
            at.y,
            top
        );
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

    /// Spec 0038: nothing in the room stands where the way up arrives.
    ///
    /// It used to read the lane between the loop and the wall. The loop is
    /// gone and the three flights are somewhere else, so the claim is the
    /// one that outlived it: you get off the stair into room rather than
    /// into the side of something.
    #[test]
    fn nothing_stands_where_the_way_up_arrives() {
        let step_off = Aabb::new(
            vec3(up_arrival(), BASE, up_head() - crate::RADIUS),
            vec3(
                up_arrival() + crate::RADIUS * 3.0,
                BASE + crate::people::TALL,
                up_head() + UP_WIDE + crate::RADIUS,
            ),
        );

        for n in 0..FRAMES {
            let mut all = treads(n);
            for which in 0..DOORS {
                all.extend(door_recess(n, which));
                all.extend(door_case(n, which));
            }

            for (step, tread) in all.iter().enumerate() {
                assert!(
                    !step_off.intersects(tread),
                    "piece {} of frame {} stands where you step off the way up",
                    step,
                    n
                );
            }
        }
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

        // **Against the tiles and not against the soffit.** `soffit` is
        // the bars' underside, eleven millimetres below the tiles, so a
        // shaft that hung under the hall's ceiling the whole length of the
        // second flight passed this and the beam was on screen anyway. A
        // test that measures the wrong surface passes for the wrong
        // reason.
        for (n, wall) in up_shaft().iter().enumerate() {
            assert!(
                wall.min.y >= crate::room::tile_under() - 1e-4,
                "piece {} of the shaft starts at {:.4}, under the tiles at {:.4}",
                n,
                wall.min.y,
                crate::room::tile_under()
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
