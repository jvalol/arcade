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

/// How much of the nook's north end is given to the stairwell.
///
/// **The flight runs across the nook and not along it**, which is the whole
/// of why this room is long. Spec 0037 measured the chain for a flight
/// running north and found 0.40 between the nook's end wall and the garden's
/// way, which is no room at all, and spec 0064 inherited that and said the
/// flight turns and climbs back south. Climbing back south costs the room
/// 4.2 metres of its own length for the going, and the length is the point
/// of the room.
///
/// The nook is ten metres wide and sixteen and a half long. Crossing it costs
/// the width of the flight instead, which is 1.8, and leaves fourteen metres
/// of room. Nothing has to turn.
pub const WELL_DEEP: f32 = FLIGHT + 0.4;

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

/// The well at the north end: open from the nook's floor to this room's
/// ceiling, with the flight crossing it.
///
/// It is a stairwell and not an opened ceiling. Spec 0064 rules the second
/// out as its own room, and the difference is that this one is as wide as the
/// stair and no wider.
pub fn well(reaches: f32) -> Aabb {
    let room = inside(reaches);

    Aabb::new(
        vec3(room.min.x, -THICK, room.max.z - WELL_DEEP),
        vec3(room.max.x, room.max.y, room.max.z),
    )
}

/// The floor you walk on up here, which is the nook's footprint less the
/// well.
pub fn floor(reaches: f32) -> Aabb {
    let room = inside(reaches);
    let well = well(reaches);

    Aabb::new(
        vec3(room.min.x - THICK, room::TALL, room.min.z - THICK),
        vec3(room.max.x + THICK, UP, well.min.z),
    )
}

/// How much daylight the flight's foot leaves in front of the nook's bench
/// row, which is the aisle you work the toys from.
///
/// A body is 0.9 and spec 0006's row is worked from the west, so this is
/// what somebody standing at the last bench needs behind them.
pub const CLEAR_OF_THE_ROW: f32 = 1.2;

/// Where the flight stands: its foot, and which way it climbs.
///
/// It climbs west, across the room rather than along it, and it starts where
/// the nook's bench row stops.
///
/// **Read off the row rather than measured against it.** The first go put
/// the foot against the east wall, which is where the row is: the bottom
/// four treads stood on the kociemba and the cascada, and the walk said so
/// by not reaching either of them. The row's own west face is the number
/// that decides this, so this asks for it.
///
/// **Flush with the floor's own north edge**, so that stepping off the top
/// tread is stepping onto the floor. The first go put the flight in the
/// middle of the well and left two tenths of nothing between the two, which
/// is a hole you cross on the way out of the room.
pub fn stair_foot(reaches: f32) -> Vec3 {
    let row = crate::room::benches(-reaches)
        .into_iter()
        .filter(|one| one.at.x < 0.0 && one.at.x > -20.0)
        .map(|one| one.at.x - one.size.x * 0.5)
        .fold(f32::INFINITY, f32::min);

    vec3(row - CLEAR_OF_THE_ROW, 0.0, floor(reaches).max.z + FLIGHT)
}

/// The treads, as the boxes you actually stand on.
///
/// Each one is a slab from the floor up to its own top, rather than a step
/// floating over nothing: spec 0007 found that what is in front of your foot
/// is a question about the floor, asked straight down, so a tread with
/// daylight under it is a tread you fall through.
pub fn treads(reaches: f32) -> Vec<Aabb> {
    let foot = stair_foot(reaches);
    let going = crate::cellar::TREAD;
    let riser = crate::cellar::RISER;

    (0..STEPS)
        .map(|n| {
            let top = riser * (n + 1) as f32;
            let west = foot.x - going * (n + 1) as f32;

            Aabb::new(
                vec3(west, -THICK, foot.z - FLIGHT),
                vec3(west + going, top, foot.z),
            )
        })
        .collect()
}

/// Where the flight arrives, which is the west end of it.
pub fn stair_head(reaches: f32) -> Vec3 {
    let foot = stair_foot(reaches);

    vec3(foot.x - GOING, UP, foot.z - FLIGHT)
}

/// How wide the gap in the rail is where the stair comes up.
///
/// Wider than the last tread, because what has to pass through it is a body
/// and not a step. A body is 0.9 and this project's own rule is 1.05 clear
/// anywhere somebody has to walk.
pub const WAY_UP: f32 = 1.3;

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
    let long = room.max.z - WELL_DEEP - room.min.z;
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
        (room.min.z + room.max.z - WELL_DEEP) * 0.5,
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
    let mut out = vec![floor(reaches), plinth(reaches)];

    out.extend(treads(reaches));
    // **The colonnade is solid and not only drawn.** A pilaster you walk
    // through is a painting of a pilaster, and there is a second reason
    // here: what the glass traces is `solid`, so a frame that is only drawn
    // is a frame the sphere cannot see. The first build of this room had
    // them drawn, and the ball came back as three flat bands of floor, wall
    // and ceiling, which is the thing the colonnade exists to stop.
    out.extend(
        frames(reaches)
            .into_iter()
            .map(|(at, size)| Aabb::new(at - size * 0.5, at + size * 0.5)),
    );
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

    // and the rail along the floor's own edge at the well, which is the only
    // place up here you could walk off. In two pieces, with the way up
    // between them: a rail across the head of a stair is a rail you climb
    // fourteen steps to meet.
    let edge = floor(reaches).max.z;
    let head = stair_head(reaches).x;
    let gap = (head, head + WAY_UP);

    for (from, to) in [(room.min.x, gap.0), (gap.1, room.max.x)] {
        if to - from > 1e-3 {
            out.push(Aabb::new(
                vec3(from, UP, edge - RAIL),
                vec3(to, UP + RAIL_UP, edge),
            ));
        }
    }

    out
}

/// The rail at the well's edge: how thick, and how high.
pub const RAIL: f32 = 0.12;
pub const RAIL_UP: f32 = 1.05;

/// The glazing in the slots, which is drawn and not solid: what stops you is
/// the sill, which is higher than anything here can ride.
///
/// **On the face and not in the opening, which is a compromise.** The slot
/// is a real hole: nothing solid stands in it, you can see out of it, and
/// the sphere traces through it. But the building's outside skin is
/// `Room::outside`, which samples columns and asks what is over each one,
/// so it knows walls and does not know openings and plasters straight over
/// this. A pane set inside the opening is a pane behind the plaster, which
/// is how this was built first and why the elevation came back blank.
///
/// So the glazing sits a little proud of the skin, which reads right from
/// the field and is a drawing decision rather than a lie about the wall.
/// Teaching `Room::outside` to cut round an opening is the real fix, it is
/// the thing spec 0038's ways out would want too, and it is its own piece
/// of work.
pub const PANE_PROUD: f32 = 0.06;

pub fn panes(reaches: f32) -> Vec<Aabb> {
    let room = inside(reaches);

    slots(reaches)
        .into_iter()
        .map(|slot| {
            let west = (slot.min.x + slot.max.x) * 0.5 < (room.min.x + room.max.x) * 0.5;
            let face = if west {
                room.min.x - THICK - PANE_PROUD
            } else {
                room.max.x + THICK + PANE_PROUD
            };

            Aabb::new(
                vec3(face - 0.03, slot.min.y, slot.min.z),
                vec3(face + 0.03, slot.max.y, slot.max.z),
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
    for box_ in room.solid().into_iter().chain(room.lids()) {
        if box_.center().distance(ball) > SEES {
            continue;
        }

        out.push(Traced::new(box_, colour(box_, ball.y, since)));
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
fn colour(box_: Aabb, up: f32, since: f32) -> glam::Vec4 {
    let size = box_.size();
    let flat = size.y < 0.5 && size.x.min(size.z) > 0.5;

    if (size - LAMP).abs().max_element() < 1e-3 {
        lit(since)
    } else if size.y > 1.0 && size.x.max(size.z) <= FRAME_DEEP.max(FRAME_WIDE) + 1e-3 {
        crate::aim::GLASS_PIER
    } else if flat && box_.center().y > up {
        crate::aim::GLASS_CEIL
    } else if flat {
        crate::aim::GLASS_FLOOR
    } else {
        crate::aim::GLASS_WALL
    }
}

/// Draws the room, and the ball in it.
pub fn draw(
    scene: &mut Scene,
    room: &crate::room::Room,
    since: f32,
    cube: MeshId,
    sphere: Option<MeshId>,
) {
    let reaches = room.reaches;

    let plinth = plinth(reaches);

    for piece in solid(reaches)
        .into_iter()
        .chain([lid(reaches), cornice(reaches)])
    {
        scene.push_material(
            cube,
            &Transform::at(piece.center()).with_scale(piece.size()),
            if piece == plinth {
                crate::aim::STAND
            } else {
                colour(piece, UP + TALL * 0.5, since)
            },
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

    let Some(sphere) = sphere else {
        return;
    };
    let (middle, across) = ball(reaches);

    scene.traces(&traced(room, since));
    scene.lens(
        sphere,
        &Transform::at(middle).with_scale(Vec3::splat(across * 2.0)),
        glam::Vec4::splat(GLOW),
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
/// Five is what brings the two to the same brightness. It is a fudge and it
/// is the engine's own idiom, the one `aim` uses everywhere: a colour past
/// one is a thing that gives light rather than takes it. The honest fix is
/// point lights in the tracer, which is engine work and its own spec.
pub const GLOW: f32 = 3.5;

/// What the glass bends light by.
///
/// Crown glass, which is what a paperweight is. **No dial**, per spec 0064:
/// an index on the arrow keys is the first thing anybody would reach for and
/// it is the mistake aleph is parked for.
pub const INDEX: f32 = 1.52;

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
            holds(floor(reaches).max.z),
            "the north end is not in the glass"
        );
    }

    /// You can walk its length and all the way round the plinth.
    #[test]
    fn you_can_walk_round_it() {
        let reaches = reaches();
        let room = inside(reaches);
        let floor = floor(reaches);
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

    /// The plinth stands in the middle of the room's length, and not against
    /// either end.
    ///
    /// **Which is the whole of what the spike found.** A sphere shows what is
    /// behind it, so a plinth at an end wall puts the room's length behind
    /// your eye. Within a tread of the middle, which is as near as anything
    /// in this building is placed.
    #[test]
    fn the_sphere_has_length_behind_it_either_way() {
        let reaches = reaches();
        let room = inside(reaches);
        let floor = floor(reaches);
        let (ball, _) = ball(reaches);
        let south = ball.z - room.min.z;
        let north = floor.max.z - ball.z;

        assert!(
            (south - north).abs() < crate::cellar::TREAD,
            "the ball stands {:.2} from one end and {:.2} from the other",
            south,
            north
        );
        assert!(
            south.min(north) > 5.0,
            "the shorter way to an end is {:.2} metres",
            south.min(north)
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
