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
use glam::{vec2, vec3, Quat, Vec2, Vec3};

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

/// What this room shows the field: its four outward faces and the top of
/// them. Spec 0040.
///
/// **It is the only part of this building that stands above the compound
/// wall**, so it is the only part of it anybody outside can see, and until
/// there was a field nobody could. What it was from out there is a pale
/// slab in the stone of its own inside, floating two and a half metres
/// over the hall's roof with daylight under it, because the room's floor
/// is at 6 and the hall's lid is at 3.2 and what fills the gap is the
/// shaft the way up climbs.
///
/// From outside it stands on that roof: one mass from the hall's lid to
/// the top of the room, in the garden's plaster with the garden's cope, so
/// what the building is from its own field is a walled compound with one
/// storey standing out of it. The shaft is inside that mass and so is the
/// gap, which is right: a gap under a room is a thing a building does not
/// have.
pub fn outside() -> Vec<(Aabb, Vec3)> {
    let out = HALF + THICK;
    let (low, high) = (crate::room::TALL, BASE + ROOM + THICK);

    [
        (-Vec3::X, vec3(MIDDLE_X - out, 0.0, MIDDLE_Z)),
        (Vec3::X, vec3(MIDDLE_X + out, 0.0, MIDDLE_Z)),
        (-Vec3::Z, vec3(MIDDLE_X, 0.0, MIDDLE_Z - out)),
        (Vec3::Z, vec3(MIDDLE_X, 0.0, MIDDLE_Z + out)),
    ]
    .iter()
    .map(|(looks, on)| {
        let along = if looks.x.abs() > 0.5 {
            vec3(0.0, 0.0, out)
        } else {
            vec3(out, 0.0, 0.0)
        };

        (
            Aabb::new(
                vec3(on.x - along.x, low, on.z - along.z),
                vec3(on.x + along.x, high, on.z + along.z),
            ),
            *looks,
        )
    })
    .collect()
}

/// How far across one of those faces is, and how far up, which is what a
/// quad laid on one is counted for.
pub fn outside_size() -> Vec2 {
    let face = outside()[0].0.size();

    vec2(face.x.max(face.z), face.y)
}

/// And the top of that mass, dressed the way the compound wall's top is:
/// the garden's two courses, the tiles and the roll set in from them. Boxes
/// as (middle, size), like `garden::coping`, which is where every number
/// here comes from so that the two rooflines are one roofline.
///
/// **A parapet round the edge and not a lid over the whole top.** A slab
/// was the first build and it closed the two doorways that stand on this
/// room's ceiling: spec 0038 puts a way out on all six faces of the cube,
/// so two of them come up through the roof, and a cope across the footprint
/// seals both and shares a plane with the sides of each. What is up there
/// now is a flat roof with two lanterns on it, which is what those doorways
/// are from outside.
///
/// **And it oversails outward only.** The garden's cope throws rain clear
/// on both sides because a garden wall has two outsides. This one has a
/// roof on its inside, and those doorways begin exactly at the wall's inner
/// face, so eleven centimetres of inward oversail lay across the opening of
/// each: a lintel across the bottom corner of a hole you look up through
/// from inside the room.
pub fn coping() -> Vec<(Vec3, Vec3)> {
    use crate::garden::{COPE_DEEP, COPE_OUT, COPE_ROLL};

    let out = HALF + THICK + COPE_OUT;
    let top = tops();
    let mut boxes = Vec::new();

    for (course, deep, high) in [
        (THICK, COPE_DEEP, top + COPE_DEEP * 0.5),
        (
            THICK - COPE_OUT * 0.7,
            COPE_ROLL,
            top + COPE_DEEP + COPE_ROLL * 0.5,
        ),
    ] {
        // **Standing back from the room's inner face by the oversail.**
        // Flush with it, which is where the doorways on the ceiling begin,
        // the course and the reveal of each are one plane with both on
        // show: the engine's own check named it at 0.13 by 1.60, twice
        // over, in a place no eye of mine had looked. The roll is set in
        // from the course on both sides because the garden's is.
        let line = HALF + COPE_OUT + THICK * 0.5;

        // **The two runs butt rather than lap.** Four bars each the whole
        // length of a side overlap at the corners, and over that square
        // both tops are on show: 0.30 by 0.30 of speckle at each corner,
        // which is what the engine's check reported. The runs along x take
        // the corners and the runs along z stop at their faces, where the
        // two end faces look opposite ways and neither is hidden.
        let (long, short) = (out, line - course * 0.5);

        for (middle, size) in [
            (vec3(-line, 0.0, 0.0), vec3(course, 0.0, long * 2.0)),
            (vec3(line, 0.0, 0.0), vec3(course, 0.0, long * 2.0)),
            (vec3(0.0, 0.0, -line), vec3(short * 2.0, 0.0, course)),
            (vec3(0.0, 0.0, line), vec3(short * 2.0, 0.0, course)),
        ] {
            boxes.push((
                vec3(MIDDLE_X + middle.x, high, MIDDLE_Z + middle.z),
                vec3(size.x, deep, size.z),
            ));
        }
    }

    boxes
}

/// What stands outside this room's shell: every one of spec 0038's six
/// doorways, boxed in, with the way it points.
///
/// A way out on all six faces of a cube and a closed recess behind each
/// means six boxes standing off the outside of it, two on the roof and
/// four on the walls. Nothing had ever looked at the outside, so all six
/// were out there in the pale stone of the room's inside: the two on the
/// roof read as crates somebody had left and the four on the walls as
/// nothing at all.
pub fn jutting() -> Vec<(Aabb, Vec3)> {
    let mut out = Vec::new();

    for n in 0..FRAMES {
        for which in 0..DOORS {
            let hole = doorway(n, which);
            let (axis, sign) = door_faces(&hole);
            let mut looks = Vec3::ZERO;
            looks[axis] = sign;

            // the recess is a back and four sides, and the whole of it is
            // what stands out there: taking its bounds rather than writing
            // the box out again, because the recess is what is actually
            // outside and a second account of it would drift from the
            // first.
            let whole = door_recess(n, which)
                .into_iter()
                .fold(None::<Aabb>, |all, box_| {
                    Some(match all {
                        None => box_,
                        Some(all) => Aabb::new(all.min.min(box_.min), all.max.max(box_.max)),
                    })
                })
                .expect("a back and four sides");

            out.push((whole, looks));
        }
    }

    out
}

/// The faces of those that want plaster, four apiece, as (face, which way
/// it looks), the way `outside` gives the mass's.
///
/// Every face but two: the one against the wall it comes out of, which is
/// buried, and the one on top, which gets stone. For the two on the roof
/// that leaves the four sides; for the four on the walls it leaves the far
/// end, the two cheeks and the underside, which is as much on show as any
/// of them from a field fifteen metres below.
///
/// The recess's four sides leave the corners of the ring open, which is
/// what a ring of four boxes does and has never mattered inside a wall. A
/// quad across the whole face covers them.
pub fn jutting_faces() -> Vec<(Aabb, Vec3)> {
    let mut out = Vec::new();

    for (whole, points) in jutting() {
        for looks in [-Vec3::X, Vec3::X, -Vec3::Y, Vec3::Y, -Vec3::Z, Vec3::Z] {
            if looks == Vec3::Y || looks == -points {
                continue;
            }

            let (mut low, mut high) = (whole.min, whole.max);

            for k in 0..3 {
                if looks[k] > 0.5 {
                    low[k] = high[k];
                } else if looks[k] < -0.5 {
                    high[k] = low[k];
                }
            }

            out.push((Aabb::new(low, high), looks));
        }
    }

    out
}

/// And the stone over the top of one: a capstone on a lantern, a hood on a
/// bay, and the parapet's course on a smaller plan either way. One slab,
/// oversailing, with no roll on it, because a roll on a thing eighty
/// centimetres high is a third course.
///
/// The dashed line under the oversail is older than any of this and is on
/// the compound wall's cope in the same frame. Not the sun's shadow map,
/// which was the first guess: shrinking that map from a hundred and sixty
/// metres to thirty changes the line by nothing at all.
pub fn jutting_cap(whole: &Aabb) -> (Vec3, Vec3) {
    use crate::garden::{COPE_DEEP, COPE_OUT};

    let middle = whole.center();
    let size = whole.size();

    (
        vec3(middle.x, whole.max.y + COPE_DEEP * 0.5, middle.z),
        vec3(size.x + COPE_OUT * 2.0, COPE_DEEP, size.z + COPE_OUT * 2.0),
    )
}

/// The top of the mass as a surface: where the middle of it is and how far
/// across, for a quad laid flat on it.
///
/// **The largest surface this building has, and nobody had seen it.** It is
/// thirteen metres square and it was the pale stone of the room's own
/// ceiling, because until spec 0041 there was no way to be above it. The
/// parapet rings it and the two lanterns stand on it, so the quad runs the
/// whole footprint and the edges of it are buried under the cope.
pub fn roof() -> (Vec3, Vec2) {
    let out = HALF + THICK;

    (
        vec3(MIDDLE_X, tops() + 0.01, MIDDLE_Z),
        vec2(out * 2.0, out * 2.0),
    )
}

/// And its near corner in plan, which is where the screed on it is laid out
/// from. The same grid the storey below is laid out from, so the two roofs
/// are one lime rather than two.
pub fn roof_from() -> Vec2 {
    let (middle, size) = roof();

    vec2(middle.x - size.x * 0.5, middle.z - size.y * 0.5)
}

/// How high the top of that mass is, which is the room's own lid.
pub fn tops() -> f32 {
    BASE + ROOM + THICK
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

/// How far off the middle of the way somebody walks, and how far the four
/// of them are spread within that.
///
/// **They all walked the same line.** One journey, everybody on it, so two
/// going opposite ways met in the middle of the flight and passed through
/// each other, and four going the same way trod in one another's steps.
/// Which side you keep is which way you are going, and the spread inside
/// that is what stops the four of them being one queue.
///
/// A body is 0.41 across at the arms and the flight is `STAIR_WIDE`, so two
/// lanes a third of a metre either side of the middle leave a hand's
/// breadth between shoulders.
pub const LANE: f32 = 0.33;
pub const LANE_SPREAD: f32 = 0.18;

/// How far either side of somebody the way is read to work out which way
/// they face.
///
/// **A corner is turned over a stride rather than in a frame.** The facing
/// was the leg's own direction, so at the foot of the flight a walker
/// swapped from one heading to another between two frames, which is a thing
/// a cardboard cut-out does. Taken as the chord between where they were
/// half a stride back and where they will be half a stride on, the turn
/// takes the whole corner.
pub const TURNS_IN: f32 = 0.45;

/// The walk through a frame, in that frame's own terms: out of the door at
/// the top of the flight, down it, along the floor, and out of the door at
/// the end of that.
///
/// Three corners rather than two, because the turn at the foot is the thing
/// that says they are going somewhere rather than pacing.
///
/// `lane` moves the whole line sideways, so one walker's way is another's
/// shifted over: parallel on every leg, and the corners stay square.
pub fn journey_in(lane: f32) -> [Vec3; 4] {
    let across = STAIR_ACROSS + STAIR_WIDE * 0.5 + lane;
    let over = -HALF + DOOR_WIDE * 1.5 + lane;

    [
        vec3(HALF, -HALF + STAIR_CLIMB, across),
        vec3(HALF - STAIR_RUN, -HALF, across),
        vec3(over, -HALF, across),
        vec3(over, -HALF, HALF - THICK),
    ]
}

/// And the middle of it, which is what the worn path is laid along and what
/// the room is measured by.
pub fn journey() -> [Vec3; 4] {
    journey_in(0.0)
}

/// How far along a way a walk of `gone` has got.
fn along_the_way(path: &[Vec3; 4], gone: f32) -> Vec3 {
    let mut left = gone.max(0.0);
    let mut leg = 0;

    while leg + 2 < path.len() && left > path[leg].distance(path[leg + 1]) {
        left -= path[leg].distance(path[leg + 1]);
        leg += 1;
    }

    let (from, to) = (path[leg], path[leg + 1]);

    from.lerp(to, (left / from.distance(to)).clamp(0.0, 1.0))
}

/// How long one of those ways is, corner to corner.
fn way_long(path: &[Vec3; 4]) -> f32 {
    path.windows(2).map(|leg| leg[0].distance(leg[1])).sum()
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
        // **And not across the way up's slot.** The leg to the far door
        // passes the head of the stairwell and overlapped it by six
        // tenths, so a strip of worn floor was drawn hanging over a hole.
        // The floor is cut there and what is laid on it is cut with it.
        .flat_map(|leg| crate::room::minus(leg, over_the_slot()))
        .collect()
}

/// How wide that path is and how proud of the floor it stands.
///
/// Proud rather than sunk, because a strip laid *in* a floor is two faces
/// in one plane, which is the fault this building has fought nine times.
pub const PATH_WIDE: f32 = 1.4;
pub const PATH_DEEP: f32 = 0.04;

/// The border round each frame's floor: how far in from the wall it runs,
/// how wide the band is, and how far it stands proud.
///
/// **A floor you can tell from a wall.** Three of the six faces of this
/// room are floors and the other three are not, and nothing about a plain
/// pale face says which it is. A skirting and a cornice would say it, and
/// they would also say which way up the room is, which is the one thing it
/// must not say. A border laid on a floor says floor and says nothing
/// about up: each of the three gets one, in its own terms.
///
/// Lower than the worn path, so where the two cross the path is what you
/// see. The band's top is inside the path's box there, which is a face
/// hidden rather than two faces fighting.
pub const BORDER_IN: f32 = 0.9;
pub const BORDER_WIDE: f32 = 0.18;
pub const BORDER_UP: f32 = 0.02;

/// The slot, raised by the proudest thing anything lays on a floor.
///
/// **A strip laid on a floor and a hole cut through it only ever touch in
/// a plane**, so cutting one by the other takes nothing: the path sits at
/// the floor's top and the hole stops there. Raised, the cut bites.
fn over_the_slot() -> Aabb {
    let hole = up_hole();

    Aabb::new(
        hole.min,
        vec3(hole.max.x, hole.max.y + PATH_DEEP, hole.max.z),
    )
}

/// That border, as the four bands of it.
///
/// **Cut to the way up's slot, like the path.** What is laid on a floor is
/// not laid where the floor is not: the slot reaches the north wall, so a
/// band run along it would cross the stairwell's mouth. For the other two
/// frames the cut takes nothing, because the hole is in one floor only.
pub fn border(n: u64) -> Vec<Aabb> {
    let out = HALF - BORDER_IN;
    let inner = out - BORDER_WIDE;
    let (low, high) = (-HALF, -HALF + BORDER_UP);
    let mut ring = Vec::new();

    for (near, far) in [(-out, -inner), (inner, out)] {
        // the two along x take the corners, and the two along z stop at
        // their faces, so no two of the four lap at a corner
        ring.push(box_in_frame(n, vec3(-out, low, near), vec3(out, high, far)));
        ring.push(box_in_frame(
            n,
            vec3(near, low, -inner),
            vec3(far, high, inner),
        ));
    }

    ring.into_iter()
        .flat_map(|band| crate::room::minus(band, over_the_slot()))
        .collect()
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
    let mut out = Vec::new();

    for n in 0..FRAMES {
        for who in 0..WALKERS {
            // the odd ones take it from the other end, and which end you
            // start from is which side of the way you keep
            let down = (n + who) % 2 == 0;
            let lane = if down { LANE } else { -LANE }
                + (who as f32 / (WALKERS - 1) as f32 - 0.5) * LANE_SPREAD;
            let path = journey_in(lane);
            let long = way_long(&path);

            // **Nobody walks at the same pace as anybody else.** Twelve
            // people crossing a room at one speed is a parade, and the
            // speed is the first thing the eye reads. A sixth either side
            // of `WALKS`, which is a slow walk against a brisk one.
            let pace = WALKS * (0.84 + 0.32 * ((n * WALKERS + who) % 5) as f32 / 4.0);
            let round = long / pace + WAITS;

            // spread along the round, and the frames out of step with each
            // other, so the room is never three of the same thing at once
            let off = round * (who as f32 / WALKERS as f32 + n as f32 * 0.11);
            let walked = (since + off).rem_euclid(round) * pace;

            // out of sight, which is the wait past the end of the walk
            if walked > long {
                continue;
            }

            let gone = if down { walked } else { long - walked };
            let at = along_the_way(&path, gone);
            let ahead = along_the_way(&path, (gone + TURNS_IN).min(long));
            let behind = along_the_way(&path, (gone - TURNS_IN).max(0.0));
            let way = (ahead - behind).normalize_or(Vec3::X) * if down { 1.0 } else { -1.0 };

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
                along: (since + off) * pace / (STRIDE * crate::people::STEPS),
                shade: (n * WALKERS + who) as usize,
            });
        }
    }

    out
}

// ---- the way up ----

/// How wide a flight is, how deep one tread, and how many treads there are
/// in each of the two. Spec 0037.
///
/// **A dog-leg, because every collider in this building is a box.** A spiral
/// was drawn first: a wedge-shaped tread's bounding box is most of a
/// cylinder, the boxes overlap near the newel, and the walking surface there
/// is whichever of them is highest. Spec 0007 met the same thing from the
/// other end, where nested treads gave a flight four legal rises and no way
/// out of it. Two straight runs at right angles is what a box describes
/// exactly.
///
/// **Seventeen up and fifteen on, rather than sixteen each.** Thirty two
/// rises over six metres either way, so every step in here is the same
/// step; what the split decides is where the landing is. Sixteen put it at
/// 3.00 and the nook's ceiling is at 3.18, so the second flight set off
/// from under that ceiling and had to climb through it: its first two
/// treads hung 0.18 into the room below, in a hole cut in the tiles for
/// them, and from the lane underneath that is a wooden soffit coming out
/// of the ceiling. Dressing it was tried twice and it is not a dressing
/// fault.
///
/// Seventeen puts the landing at 3.1875, which is inside the thickness of
/// a ceiling tile. The second flight starts above the ceiling, nothing
/// hangs under it, and the shaft round it comes down onto the landing
/// instead of stopping a slot short of it. Those two heights are arrived
/// at separately and agree to a millimetre, so
/// `tests::the_landing_is_at_the_nooks_ceiling` holds them together.
pub const UP_WIDE: f32 = 1.2;
pub const UP_GOING: f32 = 0.28;
pub const UP_STEPS: u64 = 17;
pub const UP_STEPS_ON: u64 = 15;

/// And what falls out of those: one tread's rise, the run of each flight,
/// and the height of the landing between them.
pub const UP_RISE: f32 = BASE / (UP_STEPS + UP_STEPS_ON) as f32;
pub const UP_RUN: f32 = UP_GOING * UP_STEPS as f32;
pub const UP_RUN_ON: f32 = UP_GOING * UP_STEPS_ON as f32;
pub const UP_LANDING: f32 = UP_RISE * UP_STEPS as f32;

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
    up_west() + UP_WIDE + UP_RUN_ON
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
        vec3(west + UP_WIDE, UP_LANDING, head + UP_WIDE),
    ));

    // the second flight, over the nook's ceiling, climbing east.
    //
    // **No step hangs below the landing it starts from.** A rake `THICK`
    // deep is right once the flight is in the air and wrong for the first
    // steps of it, where it puts their underside four tenths below the
    // landing. So the rake is floored at the landing's own top, which
    // clamps the first two steps.
    //
    // While the landing was at 3.00 that floor was still under the nook's
    // ceiling, and those two steps hung 0.18 into the room below with a
    // hole cut in the tiles for them. Dressing that was tried twice, a trim
    // under the landing and every tread carried down to it, and the fault
    // was never a dressing fault: it was the landing's height, which
    // `UP_STEPS` now settles.
    for step in 0..UP_STEPS_ON {
        let top = UP_LANDING + UP_RISE * (step as f32 + 1.0);
        let at = west + UP_WIDE + UP_GOING * step as f32;

        out.push(Aabb::new(
            vec3(at, (top - UP_RISE - THICK).max(UP_LANDING), head),
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
    let steps = ((under - UP_HEADROOM - UP_LANDING) / UP_RISE)
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
        .filter(|t| t.max.y > UP_LANDING + 1e-4 && t.min.y < crate::room::TALL)
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

/// The hole the nook's ceiling is actually cut to, which is the well taken
/// out to the middle of the wall round it.
///
/// **Because a tile cut flush with a wall ends in that wall's own plane.**
/// The well is cut to the stair exactly, so the tiles and the bars stopped
/// at `up_west` and the parapet began there, and the end of every tile was
/// a drawn face in the same plane as the drawn face of the wall. Dark navy
/// against pale plaster, the whole rake of the flight, which is what Jake
/// kept calling a gap in the wall. The building has written this rule down
/// four times under other names: build behind a wall from its far face,
/// not its near one.
///
/// Half the wall's thickness, so the cut ends sit inside the solid and
/// show from neither side. Not the far face, which would only move the
/// same fault out into the nook.
///
/// Taken out on every side that has a wall beside it and not at the one
/// that has none: the second flight's hole stops where the flight leaves
/// the ceiling, and past that there is nothing over the tiles to hide an
/// edge behind.
pub fn up_well_cut() -> Vec<Aabb> {
    let out = THICK * 0.5;
    let well = up_well();

    vec![
        Aabb::new(
            well[0].min - vec3(out, 0.0, out),
            well[0].max + vec3(out, 0.0, out),
        ),
        Aabb::new(
            well[1].min - vec3(0.0, 0.0, out),
            well[1].max + vec3(0.0, 0.0, out),
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

    for step in (0..UP_STEPS_ON).step_by(every) {
        let step = step as f32;
        out.push(vec3(
            west + UP_WIDE + UP_GOING * (step + 0.5),
            UP_LANDING + UP_RISE * step + UP_LAMP_UP,
            head + UP_WIDE,
        ));
    }

    // **Only where there is a wall to put one in.** The first flight
    // stands free in the middle of the nook, so under the ceiling there is
    // nothing beside it to hang a fitting on, and the nook's own lamps
    // light that part anyway. Above the ceiling the shaft is the wall.
    //
    // The parapet is not a wall for this. It stands a metre over the tread
    // beside it and a fitting sits at 1.9, which is a lamp in the air over
    // a low wall.
    let shaft = up_shaft();
    out.retain(|at| {
        shaft
            .iter()
            .any(|wall| at.y > wall.min.y && at.y < wall.max.y - 0.15)
    });
    out
}

/// How high a newel lamp sits over the parapet it stands on, how wide it
/// is, and how much air it wants under the tiles to be worth standing
/// there.
///
/// One every fifth tread, each side, which is the cadence the sconces
/// above the tiles already keep.
pub const UP_NEWEL_UP: f32 = 0.34;
pub const UP_NEWEL_WIDE: f32 = 0.15;
pub const UP_NEWEL_ROOM: f32 = 0.55;
/// Dimmer and shorter than a sconce, because a sconce lights a wall you
/// walk along and a newel is at your elbow. At the shaft's own 0.5 and 5.0
/// the one two treads behind you washed the parapet white and the stair
/// came out lit like the hall it is a way out of.
pub const UP_NEWEL_LIT: f32 = 0.30;
pub const UP_NEWEL_RANGE: f32 = 3.5;
/// How far under the lamp the bowl sits, which is what makes it an
/// uplighter rather than a lantern.
pub const UP_NEWEL_BOWL: f32 = 0.07;
pub const UP_NEWEL_EVERY: usize = 5;

/// The lamps standing on the parapet beside the first flight.
///
/// **Because the ceiling over that flight had no light on it at all.** The
/// fittings on the way up hang on the shaft wall and the shaft starts at
/// the nook's ceiling, so everything under the tiles was lit by the nook's
/// own four from across the room. On the seventh tread the tiles are three
/// tenths of a metre over your eyes and edge on, and what that drew was a
/// flat black band with a dead straight edge where the shaft wall comes
/// down to meet it. Jake read it as a hole in the wall twice. Nothing was
/// open: of the rays in that view that meet no wall of the building, every
/// one lands on a tile.
///
/// **A lamp on a post and not a sconce**, because this stair has no wall
/// beside it. It stands free in the middle of the nook with a parapet
/// either side, which is the thing a newel lamp is for, and `up_sconces`
/// already says in as many words that a fitting at a sconce's height over a
/// parapet is a lamp hanging in the air.
///
/// The bowl sits under the lamp rather than over it, so what it lights is
/// the ceiling. Lighting the treads was never the problem, and a lamp open
/// downward over a parapet burns the cap it stands on white.
pub fn up_newels() -> Vec<Vec3> {
    let tiles = crate::room::soffit();
    let middle = up_west() + UP_WIDE * 0.5;
    let mut out = Vec::new();

    // **Only on the rail, which is the part of the parapet with air over
    // it.** The stretch the shaft stands over is a wall to the ceiling and
    // has the shaft's own sconces on it.
    for west in [true, false] {
        let mut rails: Vec<Aabb> = up_sides()
            .into_iter()
            .filter(|side| side.max.y + UP_NEWEL_UP + UP_NEWEL_ROOM <= tiles)
            .filter(|side| (side.center().x < middle) == west)
            .collect();

        rails.sort_by(|one, other| one.min.z.total_cmp(&other.min.z));

        out.extend(
            rails
                .iter()
                .step_by(UP_NEWEL_EVERY)
                .map(|side| vec3(side.center().x, side.max.y + UP_NEWEL_UP, side.center().z)),
        );
    }

    out
}

/// And the lamps, which are those, each with what it is worth.
///
/// **One list and not two.** Two lists of one kind of thing is the fault
/// this building keeps writing down, and a room with some of its lamps in
/// one place and the rest in another is how the baths ended up with a
/// ceiling glowing at nothing.
pub fn up_lamps() -> Vec<(Vec3, f32, f32)> {
    let mut out: Vec<(Vec3, f32, f32)> = up_sconces()
        .into_iter()
        .map(|at| (at, UP_LAMP_LIT, UP_LAMP_RANGE))
        .collect();

    out.extend(
        up_newels()
            .into_iter()
            .map(|at| (at, UP_NEWEL_LIT, UP_NEWEL_RANGE)),
    );

    out
}

/// How high the parapet stands over the tread beside it.
///
/// A body here is a sphere 0.9 across sitting on your feet, so anything
/// lower than that is something the walk rolls over the top of. This is a
/// handrail's height for the same reason a handrail is that height.
pub const UP_RAIL: f32 = 1.0;

/// The solid sides of the first flight, from the nook's floor to a rail's
/// height over each tread.
///
/// **Because the flight stands free in a room and the drop beside it is
/// the room's whole height.** The shaft hangs from the nook's ceiling at
/// 3.19 and everything under that was open air. A body here is a sphere
/// 0.9 across sitting on your feet, so on the twelfth tread its middle is
/// at 2.70, under the bottom edge of the lowest wall there was: pressing
/// sideways took it under that edge and off into a three metre drop. Jake
/// walked into it twice.
///
/// **A parapet and not a wall to the ceiling.** Both stop the fall. A wall
/// also hides the stair from the lane you walk to reach it, which is the
/// one view of it this building has, so the sides stop a metre over the
/// treads and you look over them at a staircase.
///
/// Stepped, because every collider in this building is a box and the top
/// of each piece is the tread it belongs to. Capped where the shaft starts,
/// so the two meet in a plane neither of them shows.
pub fn up_sides() -> Vec<Aabb> {
    let west = up_west();
    let cap = crate::room::tile_middle();
    let shaft = up_shaft();
    let treads: Vec<Aabb> = up_treads()
        .into_iter()
        .filter(|t| t.max.y <= UP_LANDING + 1e-4 && t.size().z <= UP_GOING + 1e-4)
        .collect();
    let mut out = Vec::new();

    for (from, to) in [
        (west - THICK, west),
        (west + UP_WIDE, west + UP_WIDE + THICK),
    ] {
        let under = |tread: &&Aabb| {
            shaft.iter().any(|wall| {
                wall.min.x < to - 1e-4
                    && wall.max.x > from + 1e-4
                    && wall.min.z < tread.max.z - 1e-4
                    && wall.max.z > tread.min.z + 1e-4
            })
        };

        // **A rail where there is nothing over it, and a wall where there
        // is.** The shaft hangs from the tiles and the rail stopped a metre
        // over the tread, so for the six treads under the hole there was a
        // slot between the two, a metre of it at the bottom tapering to
        // twelve centimetres at the top. Nothing measured it: the sweep
        // that called the stairwell sealed started at the tiles and this is
        // under them.
        for tread in treads.iter().filter(|t| !under(t)) {
            out.push(Aabb::new(
                vec3(from, -THICK, tread.min.z),
                vec3(to, (tread.max.y + UP_RAIL).min(cap), tread.max.z),
            ));
        }

        // **And the wall is one box for the whole stretch.** A box a tread,
        // each stopping where the next began, is a join every tread, and
        // every join is a hairline. One box has none.
        let walled: Vec<&Aabb> = treads.iter().filter(under).collect();

        if let (Some(first), Some(last)) = (walled.first(), walled.last()) {
            out.push(Aabb::new(
                vec3(from, -THICK, first.min.z),
                vec3(to, cap, last.max.z),
            ));
        }
    }

    out
}

/// The way up as one list: the stair, the sides of it and the shaft round
/// it.
///
/// Named once because spec 0026 keeps it, `fillable` leaves it out, and the
/// drawing walks it. Three places saying what the way up is made of is
/// three chances for two of them to be right.
pub fn way_up() -> Vec<Aabb> {
    let mut out = up_treads();
    out.extend(up_sides());
    out.extend(up_shaft());
    out
}

/// The floor of this room, in the pieces left round the stair's slot.
///
/// In pieces because that is what the building serves and what the walk
/// meets. The slot is a hole on purpose with a stair in it, and a hole inside
/// one room is that room's own business, which is why the rule over
/// `room::Room::grounds` asks nothing about two pieces of one floor.
pub fn floors() -> Vec<Aabb> {
    let half = ROOM * 0.5;

    crate::room::minus(
        Aabb::new(
            vec3(
                MIDDLE_X - half - THICK,
                BASE - THICK,
                MIDDLE_Z - half - THICK,
            ),
            vec3(MIDDLE_X + half + THICK, BASE, MIDDLE_Z + half + THICK),
        ),
        up_hole(),
    )
}

/// How far a pier stands off the face it is on, how wide it is across, and
/// how many run over one face.
///
/// **On the three faces nobody stands on, which is the only place they can
/// go.** Every face of this room is somebody's floor or somebody's ceiling
/// and there is no such thing as a plain wall in here. On a floor a pier is
/// a block in the way of whoever walks there. On a ceiling it is a beam, and
/// a beam over your head is a pilaster up the wall to the two frames that
/// read that face as a wall, which is the whole of what they are for.
///
/// Five, because the room is 12.5 across and four left a gap in the middle
/// wide enough to read as a gap rather than a rhythm.
pub const PIER_DEEP: f32 = 0.32;
pub const PIER_WIDE: f32 = 0.45;
pub const PIERS: u64 = 5;

/// The piers over frame `n`'s head.
///
/// Run the whole span so their ends are buried in the faces either side,
/// which is this building's rule for not putting two faces in one plane.
pub fn piers(n: u64) -> Vec<Aabb> {
    (0..PIERS)
        .map(|i| {
            let along = -HALF + ROOM * (i + 1) as f32 / (PIERS + 1) as f32;

            box_in_frame(
                n,
                vec3(-HALF, HALF - PIER_DEEP, along - PIER_WIDE * 0.5),
                vec3(HALF, HALF, along + PIER_WIDE * 0.5),
            )
        })
        .filter(|pier| {
            // **Dropped whole where something crosses one, not cut round
            // it.** A pier through a doorway is a bar across a door, and the
            // way up comes through the north face where frame two's piers
            // stand: the first build walled the arrival off and
            // `you_can_walk_up_it` stopped at 5.06 rather than on the floor.
            //
            // Cutting was tried first and makes slivers, and slivers put two
            // drawn faces in one plane, which is what this building has spent
            // nine sittings on. A missing pier reads as a gap in a rhythm,
            // which is what a stair coming through a wall should look like.
            //
            // A body's width of room round the way up, because the stair's
            // own boxes are where the stair is and not where somebody
            // climbing it needs to be.
            let clear = way_up()
                .into_iter()
                .map(|box_| {
                    Aabb::new(
                        box_.min - Vec3::splat(crate::RADIUS),
                        box_.max + Vec3::splat(crate::RADIUS),
                    )
                })
                .chain((0..FRAMES).flat_map(|m| (0..DOORS).map(move |which| doorway(m, which))));

            !clear
                .into_iter()
                .any(|cut| pier.min.cmplt(cut.max).all() && cut.min.cmplt(pier.max).all())
        })
        .collect()
}

/// Every pier in the room.
pub fn all_piers() -> Vec<Aabb> {
    (0..FRAMES).flat_map(piers).collect()
}

/// A point just inside each frame's own floor, for telling the three of them
/// from the rest of the shell.
///
/// **Three of this room's six faces are somebody's floor** and the other
/// three are over somebody's head. Nothing said which was which, so all six
/// were drawn as wall and the room came out one value from end to end: the
/// thing it is about is that three surfaces disagree about down, and not one
/// of them was telling you it was a surface you could stand on.
///
/// A point and not a test, because what the shell is made of is a box list
/// and the question "is this box that floor" is answered by whether the
/// floor's own middle is inside it.
pub fn under_each_floor() -> Vec<Vec3> {
    (0..FRAMES)
        .map(|n| in_frame(n, vec3(0.0, -HALF - THICK * 0.5, 0.0)))
        .collect()
}

/// Everything solid: the shell, the treads and the landings, and the way up.
pub fn solid() -> Vec<Aabb> {
    let half = ROOM * 0.5;
    let low = vec3(MIDDLE_X - half, BASE, MIDDLE_Z - half);
    let high = vec3(MIDDLE_X + half, BASE + TALL, MIDDLE_Z + half);
    // the floor you stand on in the middle, with the stair's slot out of it
    let mut out = floors();

    // and the piers over each frame's head, which are a wall's furniture to
    // the two frames that read that face as a wall and so have to be solid:
    // on the lid they are out of anybody's reach, and on the other two they
    // stand where a body can walk into them.
    out.extend(all_piers());

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
        // **And the outside of it**, which `room::solid` has no reason to
        // carry: the parapet and the two capstones are drawn and nothing
        // walks into them. The first parapet had its roll flush with the
        // course below it, which is a dashed line along the whole top of
        // the building, and this check names it where the four tests
        // either side of it do not.
        drawn.extend(
            coping()
                .into_iter()
                .chain(jutting().iter().map(|(whole, _)| jutting_cap(whole)))
                .map(|(middle, size)| Aabb::new(middle - size * 0.5, middle + size * 0.5)),
        );

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

    /// Spec 0038: nothing laid on a frame's floor is laid where that floor
    /// is not.
    ///
    /// **The way up's slot is the one hole in any of the three.** It
    /// reaches the north wall, so anything run along that wall crosses its
    /// mouth, and the worn path's leg to the far door overlapped it by six
    /// tenths: a strip of floor wear drawn hanging over a stairwell. The
    /// border would have done the same.
    ///
    /// Asked of the hole itself rather than of a list of what to cut,
    /// which is the same list `floors` cuts the floor by.
    #[test]
    fn nothing_is_laid_over_the_way_ups_slot() {
        // the slot itself rather than its edges: what is cut to it butts
        // it on every side, and a box that butts another intersects it
        let slot = over_the_slot();
        let hole = Aabb::new(slot.min + Vec3::splat(1e-3), slot.max - Vec3::splat(1e-3));

        for n in 0..FRAMES {
            for (what, laid) in [("the path", path(n)), ("the border", border(n))] {
                for piece in laid {
                    assert!(
                        !piece.intersects(&hole),
                        "{} of frame {} at {:?} is laid over the slot",
                        what,
                        n,
                        piece.min
                    );
                }
            }
        }
    }

    /// Spec 0038: every frame's floor carries the same border, and it is
    /// on the floor rather than in it.
    #[test]
    fn every_frames_floor_has_a_border_laid_on_it() {
        for n in 0..FRAMES {
            let bands = border(n);

            assert!(
                bands.len() >= 4,
                "frame {} has {} bands of border",
                n,
                bands.len()
            );

            for band in bands {
                let thick = band.size().min_element();

                assert!(
                    (thick - BORDER_UP).abs() < 1e-4,
                    "a band of frame {}'s border is {:.3} thick rather than {:.3}",
                    n,
                    thick,
                    BORDER_UP
                );
                assert!(
                    thick < PATH_DEEP,
                    "the border stands as proud as the path worn over it"
                );
            }
        }
    }

    /// Spec 0038: two people walking a frame the opposite way round pass
    /// each other rather than through each other.
    ///
    /// **They all walked one line.** `journey` was the way and everybody
    /// was on it, so a pair meeting head on at the foot of a flight walked
    /// into one another, overlapped for a stride and came out the other
    /// side. Jake saw it as the room not feeling like people.
    ///
    /// Each of them keeps the side of the way their own direction makes
    /// left, so the two lines are parallel on every leg and nested at the
    /// corners, and the spread inside a lane keeps the four of one
    /// direction out of a single queue.
    ///
    /// **It does not claim they never touch.** Nobody gives way to anybody
    /// here and the paces differ, so one walking the same way as another
    /// catches it up and goes through it. Over the shoulder, at a glance,
    /// that is a crowd. Head on it is a fault, which is what this holds.
    #[test]
    fn nobody_walks_through_anybody_coming_the_other_way() {
        let round = way_long(&journey()) / WALKS + WAITS;
        let apart = crate::people::ARMS_APART * 2.0;
        let mut met = 0;

        for tick in 0..(round * 20.0) as u32 {
            let since = tick as f32 * 0.05;
            let walking: Vec<(usize, Walker)> = walking(since)
                .into_iter()
                .map(|who| (who.shade / WALKERS as usize, who))
                .collect();

            for (n, one) in walking.iter() {
                for (m, two) in walking.iter() {
                    let facing = (one.facing - two.facing).abs();

                    if n != m || one.shade >= two.shade || facing < std::f32::consts::FRAC_PI_2 {
                        continue;
                    }

                    met += 1;

                    assert!(
                        one.at.distance(two.at) > apart,
                        "at {:.2} seconds two of frame {} are {:.2} apart, head on",
                        since,
                        n,
                        one.at.distance(two.at)
                    );
                }
            }
        }

        assert!(
            met > 100,
            "only {} pairs ever faced each other, so this proved nothing",
            met
        );
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
                .filter(|t| (rise(t) > UP_LANDING + 1e-4) == above)
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
            .filter(|t| t.size().z <= UP_GOING + 1e-4 && t.max.y <= UP_LANDING + 1e-4)
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
            .filter(|t| t.max.y <= UP_LANDING + 1e-4 && t.size().z <= UP_GOING + 1e-4)
            .collect();
        first.sort_by(|a, b| a.min.z.partial_cmp(&b.min.z).unwrap());

        let mut second: Vec<&Aabb> = treads
            .iter()
            .filter(|t| t.max.y > UP_LANDING + 1e-4)
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

        for tread in up_treads().iter().filter(|t| t.max.y > UP_LANDING + 1e-4) {
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
    /// **One rule about one number, after two tests said it twice.** The
    /// other asked the same of `up_shaft().min.y` against the whole
    /// ceiling assembly, which is five centimetres looser than the tiles
    /// and let in the one fault this test exists for.
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
        //
        // **Read off the tiles as they are drawn**, because the engine's
        // own check cannot see them: it reads boxes, and a suspended
        // ceiling is drawn and never collided, so no list it is given has
        // a tile in it.
        let (_, tiles) = crate::room::Room::of(crate::games()).lattice();
        let (low, high) = tiles
            .iter()
            .fold((f32::MAX, f32::MIN), |(low, high), (at, size)| {
                (low.min(at.y - size.y * 0.5), high.max(at.y + size.y * 0.5))
            });

        // **Nothing of the way up hangs under that ceiling.** A piece of
        // it either stands on the nook's floor, which is where the treads
        // and the parapet start, or it begins at the tiles or above them.
        // The band between the two is what hanging is.
        //
        // This is the rule the second flight broke for a day. Its first
        // two treads were floored at a landing 0.18 under the tiles, so
        // they hung that far into the room below through a hole cut in the
        // tiles for them. Put `UP_STEPS` back to sixteen and this names
        // them both at 3.0000.
        for (n, box_) in way_up().iter().enumerate() {
            assert!(
                (box_.min.y + THICK).abs() < 1e-4 || box_.min.y >= low - 1e-4,
                "piece {} of the way up starts at {:.4}, under the tiles at \
                 {:.4} and off the nook's floor",
                n,
                box_.min.y,
                low
            );
        }

        // and the shaft meets that ceiling inside a tile's own thickness.
        // Level with the bars it speckles against them and level with the
        // tiles against those, so it begins between the two faces a tile
        // has, where it is hidden and shares a plane with nothing.
        for (n, wall) in up_shaft().iter().enumerate() {
            assert!(
                wall.min.y >= low - 1e-4 && wall.min.y < high - 1e-4,
                "piece {} of the shaft starts at {:.4}, and the tiles it \
                 comes through are {:.4} to {:.4}",
                n,
                wall.min.y,
                low,
                high
            );
        }
    }

    /// Spec 0037: the landing is at the nook's ceiling.
    ///
    /// **Two numbers that agree to a millimetre and are arrived at apart.**
    /// The landing is seventeen rises over the nook's floor, which is the
    /// stair's own arithmetic; the tiles hang where spec 0012 hung them,
    /// which is the room's. Neither knows about the other, so the moment
    /// either moves this is the thing that says the way up has gone back
    /// to climbing through a ceiling.
    #[test]
    fn the_landing_is_at_the_nooks_ceiling() {
        let (_, tiles) = crate::room::Room::of(crate::games()).lattice();
        let (low, high) = tiles
            .iter()
            .fold((f32::MAX, f32::MIN), |(low, high), (at, size)| {
                (low.min(at.y - size.y * 0.5), high.max(at.y + size.y * 0.5))
            });

        assert!(
            UP_LANDING > low && UP_LANDING < high,
            "the landing is at {:.4} and the tiles are {:.4} to {:.4}",
            UP_LANDING,
            low,
            high
        );
    }

    /// Spec 0037: the shaft goes round the whole stair and not round the
    /// hole in the ceiling, which is a different shape.
    #[test]
    fn the_shaft_encloses_the_whole_second_flight() {
        let shaft = up_shaft();

        for tread in up_treads().iter().filter(|t| t.max.y > UP_LANDING + 1e-4) {
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

    /// Spec 0037: the hole the ceiling is cut to ends inside the wall round
    /// it, not in that wall's own face.
    ///
    /// The well was cut to the stair exactly, so every tile and every bar
    /// stopped at `up_west` and the parapet began there: a drawn face and a
    /// drawn face in one plane, dark navy against pale plaster, the whole
    /// rake of the flight. Jake reported it three times, as a gap in the
    /// wall, which is what it looks like.
    ///
    /// Asked of the hole and not of the ceiling. Spec 0012 lays its grid on
    /// each piece of the lid and the lid is already split at this stair, so
    /// asking the tiles themselves asks a question about `floors` instead.
    /// What this holds is the one thing that changed.
    ///
    /// The control is the second half: the well's own faces are those of
    /// the wall, which is the fault.
    #[test]
    fn the_hole_the_ceiling_is_cut_to_ends_inside_the_wall() {
        let walls: Vec<Aabb> = up_sides().into_iter().chain(up_shaft()).collect();

        // a plane is inside a wall when it falls between that wall's own two
        // faces rather than on either of them
        let buried = |at: f32, across: usize, open: &Aabb| {
            walls.iter().any(|wall| {
                let (low, high) = match across {
                    0 => (wall.min.x, wall.max.x),
                    _ => (wall.min.z, wall.max.z),
                };
                let along = match across {
                    0 => wall.min.z < open.max.z - 1e-3 && wall.max.z > open.min.z + 1e-3,
                    _ => wall.min.x < open.max.x - 1e-3 && wall.max.x > open.min.x + 1e-3,
                };

                along && at > low + 1e-4 && at < high - 1e-4
            })
        };

        // the faces the widening moved, which are the ones with a wall
        // beside them
        let moved = |cut: &[Aabb], well: &[Aabb]| {
            let mut out = Vec::new();

            for (open, was) in cut.iter().zip(well) {
                for (at, across) in [
                    (open.min.x, 0),
                    (open.max.x, 0),
                    (open.min.z, 2),
                    (open.max.z, 2),
                ] {
                    let same = match across {
                        0 => (at - was.min.x).abs() < 1e-4 || (at - was.max.x).abs() < 1e-4,
                        _ => (at - was.min.z).abs() < 1e-4 || (at - was.max.z).abs() < 1e-4,
                    };

                    if !same {
                        out.push((at, across, *open));
                    }
                }
            }

            out
        };

        let cut = up_well_cut();
        let well = up_well();
        let widened = moved(&cut, &well);

        assert!(
            widened.len() >= 5,
            "the hole was widened on {} sides, which is fewer than the stair has walls",
            widened.len()
        );

        for (at, across, open) in widened {
            assert!(
                buried(at, across, &open),
                "the hole is cut to {:.4} on axis {}, which is in no wall",
                at,
                across
            );
        }

        // and the control: cut to the stair itself, every one of those is a
        // face of the wall rather than inside it
        for open in well.iter() {
            for (at, across) in [
                (open.min.x, 0usize),
                (open.max.x, 0),
                (open.min.z, 2),
                (open.max.z, 2),
            ] {
                assert!(
                    !buried(at, across, open),
                    "the uncut well is already inside a wall at {:.4}, so this proves nothing",
                    at
                );
            }
        }
    }

    /// Spec 0037: every lamp on the way up stands in a fitting, and every
    /// fitting it could stand in is one of two.
    ///
    /// The rule this building keeps breaking. The baths ran their lamps
    /// along a ceiling and stood their sconces on a wall, so every sconce
    /// was dark and the ceiling glowed at nothing.
    ///
    /// Two kinds of fitting now, and the test says which each lamp is: a
    /// sconce is in a wall of the shaft, a newel stands on a parapet with
    /// air over it. A lamp that is neither is a lamp hanging in a
    /// stairwell.
    #[test]
    fn every_lamp_on_the_way_up_is_in_a_fitting() {
        let (sconces, newels, lamps) = (up_sconces(), up_newels(), up_lamps());
        let shaft = up_shaft();
        let sides = up_sides();
        let treads = up_treads();

        assert!(!lamps.is_empty(), "the way up is climbed in the dark");

        // **And the first flight has one of its own**, which is the whole
        // of why newels exist. The control is the line under it: every
        // sconce is above the tiles, because `up_sconces` keeps only the
        // ones with a shaft wall to hang on, so without a newel the part
        // of the stair you climb inside the nook has no lamp at all.
        let tiles = crate::room::soffit();

        assert!(
            newels.iter().any(|at| at.y < tiles),
            "the first flight has no lamp of its own"
        );
        assert!(
            sconces.iter().all(|at| at.y > tiles),
            "a sconce under the tiles, so this test proves nothing"
        );
        assert_eq!(
            lamps.iter().map(|(at, _, _)| *at).collect::<Vec<Vec3>>(),
            sconces
                .iter()
                .chain(newels.iter())
                .copied()
                .collect::<Vec<Vec3>>(),
            "a lamp that is not in a fitting"
        );

        for at in sconces {
            let bulb = Aabb::new(at - Vec3::splat(0.08), at + Vec3::splat(0.08));

            assert!(
                shaft.iter().any(|wall| wall.intersects(&bulb)),
                "the sconce at {:?} is in no wall of the shaft",
                at
            );
        }

        for at in newels {
            let post = Aabb::new(
                vec3(at.x - 0.08, at.y - UP_NEWEL_UP, at.z - 0.08),
                vec3(at.x + 0.08, at.y, at.z + 0.08),
            );

            assert!(
                sides.iter().any(|side| side.intersects(&post)),
                "the newel at {:?} stands on nothing",
                at
            );
            assert!(
                at.y + UP_NEWEL_ROOM <= tiles,
                "the newel at {:?} is in the ceiling",
                at
            );
        }

        for (at, _, _) in lamps {
            let bulb = Aabb::new(at - Vec3::splat(0.08), at + Vec3::splat(0.08));

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

    /// Spec 0037: a body pressed into either side of the first flight
    /// stays on the stair.
    ///
    /// **The fall it exists for.** The shaft hangs from the nook's ceiling
    /// at 3.19 and everything under that was open air. A body here is a
    /// sphere 0.9 across sitting on your feet, so on the twelfth tread its
    /// middle is at 2.70, under the bottom edge of the lowest wall there
    /// was: pressing west took it under that edge and off into a three
    /// metre drop. Jake walked into it twice, and the first fix, a trim
    /// under the landing, left it exactly as it was.
    ///
    /// **Walked rather than measured**, because the arithmetic was never
    /// wrong. Every wall was where the plan said and the plan was right.
    /// What was missing was anything between the lowest wall and the
    /// floor.
    ///
    /// Every tread of the flight, not only the ones under the opening in
    /// the ceiling. The parapet runs the whole flight, so the drop beside
    /// the first tread is held by the same rule as the drop beside the
    /// Spec 0038: no pier stands in anybody's way.
    ///
    /// **Which is the whole of why they are where they are.** There is no
    /// plain wall in this room: every face is one frame's floor or another's
    /// ceiling. A pier on a floor is a block in the way of whoever walks
    /// there, so they go on the three faces nobody stands on, where a beam
    /// over your head is a pilaster up the wall to the two frames that read
    /// that face as a wall.
    ///
    /// **Asked of the paths and not of the floors.** The first version of
    /// this asked whether a pier reached into a floor slab, which a pier
    /// standing on top of a floor does not, so it passed with the piers laid
    /// on the floors and proved nothing. What is actually being claimed is
    /// that the people walking their circuits do not walk into one, and the
    /// worn path is where they walk.
    ///
    /// The count is the other half of the control: without it this passes
    /// for a room with no piers in it.
    #[test]
    fn no_pier_stands_in_anybodys_way() {
        for n in 0..FRAMES {
            for leg in path(n) {
                // the air over the worn strip, which is the body and not the
                // wear: the strip itself is a film on the floor and nothing
                // would ever hit it
                let walked = Aabb::new(
                    leg.min - vec3(crate::RADIUS, 0.0, crate::RADIUS),
                    leg.max + vec3(crate::RADIUS, crate::people::TALL, crate::RADIUS),
                );

                for pier in all_piers() {
                    assert!(
                        !(pier.min.cmplt(walked.max).all() && walked.min.cmplt(pier.max).all()),
                        "a pier at {:?} stands in the way of frame {} at {:?}",
                        pier.center(),
                        n,
                        leg.center()
                    );
                }
            }
        }

        assert!(
            all_piers().len() >= FRAMES as usize * 2,
            "only {} piers in the whole room, so this proves nothing",
            all_piers().len()
        );
    }

    /// last.
    #[test]
    fn you_cannot_walk_off_the_side_of_the_way_up() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();

        let standing: Vec<Aabb> = up_treads()
            .into_iter()
            .filter(|t| t.max.y <= UP_LANDING + 1e-4 && t.size().z <= UP_GOING + 1e-4)
            .collect();

        assert_eq!(
            standing.len(),
            UP_STEPS as usize,
            "the first flight is not all here"
        );

        // the control, which is this building yesterday: take the parapet
        // out and leave the shaft hanging where it was.
        let sides = up_sides();
        let without: Vec<Aabb> = solid
            .iter()
            .filter(|box_| !sides.contains(box_))
            .copied()
            .collect();

        assert_eq!(
            without.len() + sides.len(),
            solid.len(),
            "the parapet is not in what the building serves"
        );

        let mut fell = 0;

        for tread in standing.iter() {
            for sideways in [-Vec3::X, Vec3::X] {
                let from = vec3(tread.center().x, tread.max.y + 0.01, tread.center().z);
                let at = pressed(&solid, from, sideways);

                assert!(
                    at.y > tread.max.y - UP_RISE,
                    "pressed {} off the tread at {:.2}, a body ended at y {:.2}",
                    if sideways.x < 0.0 { "west" } else { "east" },
                    tread.max.y,
                    at.y
                );

                if pressed(&without, from, sideways).y <= tread.max.y - UP_RISE {
                    fell += 1;
                }
            }
        }

        assert!(
            fell > 0,
            "without the parapet none of the {} presses walked off the side",
            standing.len() * 2
        );
    }

    /// Walks a body your size sideways off a tread for half a second and
    /// says where it got to.
    fn pressed(solid: &[Aabb], from: Vec3, wish: Vec3) -> Vec3 {
        let (mut at, mut falling) = (from, 0.0);

        for _ in 0..30 {
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

        at
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

#[cfg(test)]
mod outside_tests {
    use super::*;

    /// The footprint those faces enclose, read off the faces themselves.
    /// Written out again from `HALF` it agreed with them by coincidence and
    /// a control that halved the mass passed, because the test had halved
    /// with it.
    fn footprint() -> Aabb {
        outside()
            .iter()
            .fold(None::<Aabb>, |all, (face, _)| {
                Some(match all {
                    None => *face,
                    Some(all) => Aabb::new(all.min.min(face.min), all.max.max(face.max)),
                })
            })
            .expect("four faces")
    }

    #[test]
    fn the_imaginarium_stands_on_the_roof_under_it() {
        for (face, _) in outside() {
            assert!(
                (face.min.y - crate::room::TALL).abs() < 1e-4,
                "a face starts at {} and the roof is at {}",
                face.min.y,
                crate::room::TALL
            );
            assert!((face.max.y - tops()).abs() < 1e-4);
        }
    }

    /// Which is the point of starting it down there rather than at the
    /// room's own floor. Two and a half metres of daylight between a roof
    /// and the storey over it is not a thing a building has, and what stood
    /// in that daylight was the shaft the way up climbs.
    #[test]
    fn the_mass_reaches_the_roof_it_stands_on() {
        let room = crate::room::Room::of(crate::games());
        let low = outside()
            .iter()
            .map(|(face, _)| face.min.y)
            .fold(f32::INFINITY, f32::min);
        let middle = vec3(MIDDLE_X, low, MIDDLE_Z);

        assert!(
            room.ceiling()
                .iter()
                .any(|piece| piece.contains_point(middle)),
            "the mass starts at {} with nothing of the hall's lid there",
            low
        );
    }

    /// And the shaft is inside it, in plan, so none of it is left standing
    /// outside the mass that is meant to have swallowed it.
    #[test]
    fn the_way_up_is_inside_what_the_field_sees() {
        let foot = footprint();
        let mut under = 0;

        for box_ in up_shaft() {
            if box_.min.y >= BASE {
                continue;
            }

            assert!(
                box_.min.x >= foot.min.x - 1e-3
                    && box_.max.x <= foot.max.x + 1e-3
                    && box_.min.z >= foot.min.z - 1e-3
                    && box_.max.z <= foot.max.z + 1e-3,
                "the shaft reaches {:?}..{:?} and the mass covers {:?}..{:?}",
                box_.min,
                box_.max,
                foot.min,
                foot.max
            );
            under += 1;
        }

        assert!(under > 0, "no part of the shaft is under the room at all");
    }

    /// And it leaves the roof's own doorways alone. Spec 0038 puts one on
    /// every face of the cube, so two of them stand on the ceiling and both
    /// begin at the wall's inner face. A cope that reaches past that face
    /// lies across the opening of each, which is what a slab over the whole
    /// footprint did and what an inward oversail of eleven centimetres did
    /// after it.
    #[test]
    fn the_cope_is_clear_of_what_stands_on_the_roof() {
        for (middle, size) in coping() {
            let (x, z) = (
                (middle.x - MIDDLE_X).abs() - size.x * 0.5,
                (middle.z - MIDDLE_Z).abs() - size.z * 0.5,
            );

            assert!(
                x >= HALF - 1e-3 || z >= HALF - 1e-3,
                "a course at {:?} reaches {} and {} from the middle, inside a room of {}",
                middle,
                x,
                z,
                HALF
            );
        }
    }

    /// And nothing of this room stands outside its own shell undressed.
    /// Spec 0038's six doorways all do, which is the point of `jutting`;
    /// anything else out there is bare stone nobody has looked at, which
    /// is what all six were.
    #[test]
    fn everything_outside_the_shell_is_dressed() {
        let foot = footprint();
        let out: Vec<Aabb> = solid()
            .into_iter()
            .filter(|box_| {
                box_.min.x < foot.min.x - 1e-3
                    || box_.max.x > foot.max.x + 1e-3
                    || box_.min.z < foot.min.z - 1e-3
                    || box_.max.z > foot.max.z + 1e-3
                    || box_.max.y > tops() + 1e-3
            })
            .collect();

        assert_eq!(out.len(), 30, "five boxes to a recess and six recesses");

        let dressed = jutting();

        assert_eq!(dressed.len(), 6, "a way out on all six faces");
        assert_eq!(jutting_faces().len(), 24, "four plastered faces apiece");

        for box_ in out {
            assert!(
                dressed.iter().any(|(whole, _)| {
                    whole.min.cmple(box_.min + 1e-3).all() && whole.max.cmpge(box_.max - 1e-3).all()
                }),
                "{:?}..{:?} stands outside and is in nothing that is dressed",
                box_.min,
                box_.max
            );
        }
    }

    /// And the roof reaches under the cope, so there is no strip of bare
    /// lid between the screed and the parapet, and it is laid inside the
    /// course's own thickness, so it neither shares a plane with the lid it
    /// covers nor stands over the cope.
    #[test]
    fn the_roof_runs_under_the_cope() {
        use crate::garden::COPE_DEEP;

        let (middle, size) = roof();
        let inner = coping()
            .into_iter()
            .filter(|(at, _)| at.x < MIDDLE_X && at.z.to_bits() == MIDDLE_Z.to_bits())
            .map(|(at, size)| MIDDLE_X - at.x - size.x * 0.5)
            .fold(f32::INFINITY, f32::min);

        assert!(
            size.x * 0.5 >= inner - 1e-3,
            "the screed stops {} from the middle and the cope starts at {}",
            size.x * 0.5,
            inner
        );
        assert!(middle.y > tops() + 1e-4, "laid level with the lid");
        assert!(
            middle.y < tops() + COPE_DEEP,
            "laid above the course that is meant to bury its edge"
        );
    }

    /// And the cope sits on the top of it, one course then the roll, with
    /// nothing hanging below the lid.
    #[test]
    fn the_cope_sits_on_the_top() {
        let boxes = coping();

        assert_eq!(boxes.len(), 8);

        for (middle, size) in boxes {
            assert!(
                middle.y - size.y * 0.5 >= tops() - 1e-4,
                "a course reaches {} and the top is {}",
                middle.y - size.y * 0.5,
                tops()
            );
        }
    }
}
