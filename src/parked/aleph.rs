//! The hotel, through the doorway in the gallery's west wall. Spec 0029.
//!
//! A desk you arrive at, and a corridor of numbered doors running west with
//! no end to it. It is aleph's hotel, built as a game on 2026-10-04 and
//! parked the same night, rebuilt here because what it is is architecture
//! and this is a building.
//!
//! Rooms run 1, 2, 3 and on without end, odd on your left as you walk in and
//! even on your right. A shut door with light under it is a room with
//! somebody in it. A door standing open on a dark room is free. Nobody is
//! drawn: the paradox is about people who are never shown, and this building
//! has never drawn one.
//!
//! Where everything is, is arithmetic, and arithmetic can be checked without
//! a window.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::{gallery_wall, GALLERY_WIDE, THICK};

/// How far apart the rooms are along the corridor, and how wide and high the
/// corridor is inside.
///
/// aleph's own numbers rather than new ones. Two lists of one thing drift,
/// and the game is the other list.
pub const APART: f32 = 1.5;
pub const ACROSS: f32 = 3.2;
pub const TALL: f32 = 2.4;

/// A door, and how far it swings into the corridor.
pub const DOOR_WIDE: f32 = 1.05;
pub const DOOR_TALL: f32 = 1.9;
pub const DOOR_THICK: f32 = 0.07;

/// The gap under the leaf, which is the thing the light comes under.
///
/// **The first build had none.** The leaf stood on the floor and the light
/// was a strip starting at the middle of the door's thickness, so half of
/// every strip was buried inside the leaf and the rest came out from under a
/// door that was shut to the ground. Jake walked it and said the lights clip,
/// which is what that is.
pub const DOOR_GAP: f32 = 0.012;
pub const SWINGS_TO: f32 = 1.75;

/// How wide the one door with no number is.
///
/// **Wider than a bedroom door, because it is the only one you walk
/// through.** The other forty are scenery: they open and shut and nobody
/// passes them. This one is the only way into the register, and at
/// `DOOR_WIDE` its opening was 1.04 against a body 0.90 across, which is
/// 0.07 of room either side. Driving an approach into it a frame at a time,
/// 65, 66 and 67 got through, 68 and 69 did not, and 70 did. A player who
/// bounces off it twice concludes the room is not there.
///
/// It is a hall of records through there, and an archway to one has no
/// reason to be the size of a bedroom door.
pub const BLANK_WIDE: f32 = 1.4;

/// How wide door `n` is, which is the one above for the blank and
/// `DOOR_WIDE` for every room.
pub fn door_wide(n: u64) -> f32 {
    if n == BLANK {
        BLANK_WIDE
    } else {
        DOOR_WIDE
    }
}

/// The plaque beside each doorway, and where it sits.
pub const PLAQUE_WIDE: f32 = 0.3;
pub const PLAQUE_TALL: f32 = 0.22;
pub const PLAQUE_BESIDE: f32 = 0.72;
pub const PLAQUE_UP: f32 = 1.35;

/// How finely a room's number is drawn into its plaque.
///
/// One texture a door, made once when the building loads. Forty of them is
/// forty small textures, which is less than one cabinet's screenshot.
pub const PLAQUE_TEXELS: f32 = 64.0;

/// How many rooms are drawn at once.
///
/// **A window and not a count.** The first build drew rooms 1 to 40 at fixed
/// places and walled off what was past them, and Jake walked to room 36 and
/// found the doors stop while the hallway goes on. In a hotel whose premise
/// is that there is always more of it than you can see, reaching the last
/// door is the one thing that must not happen.
///
/// So this is how many are drawn around wherever you are standing. Which
/// forty they are is [`window`] below. The arithmetic for where room `n`
/// stands always took any `n`; all that was ever fixed was the range.
pub const DRAWN: u64 = 40;

/// Which rooms are drawn from where you are standing.
///
/// Centred on the nearest room, so the corridor is always the same depth of
/// doors ahead of you and behind you, and never runs out in either
/// direction. Room numbers start at 1, so near the lobby the window simply
/// starts there.
pub fn window(at: Vec3) -> std::ops::RangeInclusive<u64> {
    let near = nearest(at);
    let half = DRAWN / 2;
    let from = near.saturating_sub(half).max(1);

    from..=(from + DRAWN - 1)
}

/// Which room you are nearest, by where you are along the corridor.
///
/// The other half of [`at`], which puts room `n` at an `x`. This takes an `x`
/// back to an `n`, and the two have to agree or the corridor is numbered one
/// way and drawn another.
pub fn nearest(at: Vec3) -> u64 {
    let (west, _) = span();
    let along = (west - at.x) / APART;

    // two rooms share each step along, one either side, and `at` puts room n
    // at n / 2 steps, half a step in from the lobby. So a step along is two
    // rooms, and the half step is the one the blank door of spec 0031 took.
    //
    // **The inverse of `at` and nothing else.** When `at` was shifted a slot
    // to make room for that door, this went one out and the corridor was
    // numbered one way and drawn another, which its own test caught.
    let n = (along * 2.0).round() - 1.0;

    if n < 1.0 {
        1
    } else if n > MOST as f32 {
        MOST
    } else {
        n as u64
    }
}

/// The highest room this corridor can number.
///
/// Where a float stops resolving a metre is 16,777,216, which at 1.5 metres a
/// room is this. Spec 0029 says eleven million is not infinity and to say so
/// on the plaque rather than hide it.
pub const MOST: u64 = 11_184_810;

/// Where the light under a shut door lies, as a strip on the floor.
///
/// **On the floor and not in it.** A quad level with the floor is two faces
/// in one plane, which is the fault this building has fought seven times. It
/// stands a millimetre and a half proud, which is under a tenth of what the
/// eye resolves at this distance and is enough to settle the depth test.
pub const STRIP_UP: f32 = 0.0015;

/// How far the light reaches out from under the door, and how wide it is.
///
/// Narrower than the door, because a door sits in a frame and the gap is
/// under the leaf rather than under the opening.
///
/// **In steps rather than one slab.** The first build was a single quad
/// sixteen centimetres out at one brightness, and Jake walked it and called
/// it a block, which it was: light under a door is brightest at the threshold
/// and gone a hand's width away, and a rectangle of even white is a tile.
/// These are the reaches of three quads laid over each other, each dimmer and
/// further out than the last, which is a falloff drawn in three steps and
/// wants no texture and no guess about which way a quad's uvs run.
pub const STRIP_STEPS: [(f32, f32); 3] = [(0.045, 1.0), (0.10, 0.45), (0.17, 0.16)];
pub const STRIP_WIDE: f32 = DOOR_WIDE - 0.06;

// How far the furthest of them reaches was a `STRIP_OUT` of its own saying
// 0.17 beside a step that also said 0.17. Two lists of one thing, and clippy
// found it by noticing nothing outside a test ever read the constant.
// Whatever wants the furthest reach takes it off the steps.

// How bright the strip is, is `aim::HOTEL_LIGHT` and is not named twice
// here. This file had a `STRIP_GLOW` of 2.1 beside a colour in `aim` that
// already said the same thing, which is two lists of one thing and is the
// fault under most of the others in this building.

/// How deep the lobby is, west from the gallery's wall, and how wide.
///
/// **Not the size of the back room opposite.** That one is a maintenance
/// room and is the space that was left over. This is what a hotel is for:
/// you arrive in it, you stand at a desk, and a corridor leaves it. Every
/// room in this building has been too small on its first build, so the
/// walkways were laid out first and the furniture fitted to them.
pub const DEEP: f32 = 8.0;

/// And how high, which is the gallery's rather than the corridor's.
///
/// A lobby is a room you arrive in and the corridor is a thing you go down,
/// so the corridor being lower than the room it leaves is what says which is
/// which. The corridor is [`TALL`], a little over two and a half.
pub const LOBBY_TALL: f32 = crate::room::TALL;

/// Where the lobby's inside begins, which is the far face of the gallery's
/// west wall, and where it ends.
pub fn span() -> (f32, f32) {
    let east = -GALLERY_WIDE * 0.5 - THICK;

    (east - DEEP, east)
}

/// And the same along `z`, which is the gallery's own span exactly.
///
/// **Derived and not chosen.** The first build named 7.6 and it reached past
/// the gallery's near end into the nook's floor, which three of this
/// building's own tests caught before anything was drawn. Taking the
/// gallery's span means the two rooms line up by construction and a gallery
/// that changes takes this with it.
///
/// Note this is not quite centred on [`middle_z`]: the doorways sit on the
/// back room's line, which is 0.12 south of the gallery's middle. The
/// corridor follows the doorways rather than the room, because what has to
/// line up is the thing you look through.
pub fn across(reaches: f32) -> (f32, f32) {
    (gallery_wall(reaches), -reaches)
}

/// The line everything here is built on.
///
/// The back room's doorway, which is the gallery's other one, and this takes
/// its `z` from there rather than keeping a second account: spec 0029 says
/// the two are a pair and should be built as one thing so they cannot drift.
/// Stand in one and you look through the other.
pub fn middle_z(reaches: f32) -> f32 {
    let (from, to) = crate::back::way(reaches);

    (from + to) * 0.5
}

/// The doorway in the gallery's west wall, as the two `z` it runs between.
///
/// The back room's exactly, mirrored across the gallery. One account.
pub fn way(reaches: f32) -> (f32, f32) {
    crate::back::way(reaches)
}

/// The gallery's west wall, in two pieces with the doorway between them.
///
/// Spec 0022 built it as one box and `room.rs` kept it inline. It is this
/// now, and the gallery's own code asks here for it, the same way it asks
/// `back::gallery_east` for the other side.
pub fn gallery_west(reaches: f32) -> Vec<Aabb> {
    let far = gallery_wall(reaches);
    let (_, east) = span();
    let (from, to) = way(reaches);

    vec![
        Aabb::new(
            vec3(east, 0.0, far - THICK),
            vec3(-GALLERY_WIDE * 0.5, crate::room::TALL, from),
        ),
        Aabb::new(
            vec3(east, 0.0, to),
            vec3(-GALLERY_WIDE * 0.5, crate::room::TALL, -reaches),
        ),
    ]
}

/// The lobby's floor, which laps under the gallery's rather than meeting it.
///
/// **Two floors at one height that meet edge to edge stop you dead**, and
/// `walk` reports it as nothing at all. This building found that three times
/// in one day, and the back room opposite is built the same way for the same
/// reason. So this one runs east past the gallery's west edge and under its
/// wall, and the gallery's west face is inside it.
pub fn floor(reaches: f32) -> Aabb {
    let (west, _) = span();
    let (from, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, -THICK, from - THICK),
        vec3(-GALLERY_WIDE * 0.5 + THICK, 0.0, to + THICK),
    )
}

/// How far west the corridor's shell is built.
///
/// Spec 0029 says the corridor has no end and that where it actually stops is
/// where a float stops resolving a metre. Nothing is built that far: this is
/// the shell for what is drawn, and it runs four rooms past [`DRAWN`].
///
/// **Running it past the camera's far plane was tried and taken back out.**
/// A hundred and ten metres of corridor is a hundred and ten metres of
/// building, and this one is already 72 kilometres across because spec 0018
/// puts the planets out at true scale. Two things broke at once. The sun gets
/// one shadow map over what it is given, so the garden's shadows went coarse
/// in half a per cent of its pixels, which `the_garden_is_drawn` caught. And
/// the reach fill has a cell budget, which the wider world put it over, so it
/// refused and came back empty: two of this building's own tests said "the
/// fill reached nothing at all".
///
/// The sun's half of that is fixed and stays fixed, because an indoor room
/// does not belong in the sun's map whatever its length. The fill's half is
/// what sets this number. What the far end looks like is a lighting question
/// rather than a length one, and it is [`END`] below.
pub fn corridor_west() -> f32 {
    let (west, _) = span();

    west - RUNS
}

/// How long the corridor's shell actually is.
///
/// Long enough that nobody walks to the end of it: two thousand metres is
/// eight minutes at this building's 4.2 m/s, and the drawn rooms follow you
/// the whole way.
///
/// It is not eleven million rooms of shell and it does not pretend to be.
/// What stops it being longer is written in `room::building`: the reach fill
/// has a cell budget and the hotel is left out of the box it is given, which
/// is the price of this length and is named there.
pub const RUNS: f32 = 2000.0;

/// The two long faces the corridor shows the field, and the cope along the
/// top of each. Spec 0040.
///
/// **Two kilometres of shell at ground level, running straight through a
/// field.** It was never meant to be seen from outside, because until spec
/// 0040 there was no outside: from the grass it was a blank grey wall
/// going to the horizon. Dressed in the garden's own plaster with the
/// garden's own cope, what it is instead is the one view in this building
/// of how long spec 0029's corridor actually is. The endlessness is a
/// thing you look at rather than a thing you are told.
///
/// Spec 0035's corridor is the same shape and is not dressed; it is not
/// drawn from outside at all. Two of these on one skyline is the same
/// sentence said twice. See `twine::inside`.
/// And the top of it, which from the air was open: the shell's ceiling is a
/// collider and the drawing of it faces down into the corridor, so from
/// forty metres up you looked along two kilometres of floor, doors and door
/// numbers. Spec 0040.
///
/// It runs the whole length of that shell. The lobby's lid is eighty
/// centimetres higher and covers this end of it in plan, so the two never
/// meet in a plane and there is nothing to butt: the corridor is the lower
/// of the two roofs and the junction is a step rather than a join.
pub fn corridor_roof(reaches: f32) -> (Vec3, glam::Vec2) {
    let (west, _) = span();
    let middle = middle_z(reaches);
    let (from, to) = (middle - ACROSS * 0.5 - THICK, middle + ACROSS * 0.5 + THICK);
    let (near, far) = (corridor_west(), west + THICK);

    (
        vec3((near + far) * 0.5, TALL + THICK + 0.01, middle),
        glam::vec2(far - near, to - from),
    )
}

pub fn corridor_faces(reaches: f32) -> Vec<(Aabb, Vec3)> {
    let (west, _) = span();
    let middle = middle_z(reaches);
    let out = ACROSS * 0.5 + THICK;

    [-1.0f32, 1.0]
        .iter()
        .map(|side| {
            let on = middle + side * out;

            (
                Aabb::new(
                    vec3(corridor_west(), 0.0, on - 0.01),
                    vec3(west, TALL + THICK, on + 0.01),
                ),
                vec3(0.0, 0.0, *side),
            )
        })
        .collect()
}

/// The corridor's floor, which laps into the lobby's for the same reason the
/// lobby's laps into the gallery's.
pub fn corridor_floor(reaches: f32) -> Aabb {
    let (west, _) = span();
    let middle = middle_z(reaches);

    Aabb::new(
        vec3(corridor_west(), -THICK, middle - ACROSS * 0.5 - THICK),
        vec3(west + THICK, 0.0, middle + ACROSS * 0.5 + THICK),
    )
}

/// Where room `n` stands: which side of the corridor, and how far down it.
///
/// Odd on the left as you walk in and even on the right, which is not
/// decoration. The rule that doubles everyone sends every guest into an even
/// room, so the entire left wall opens in one move, and that is the clearest
/// picture this thing has of freeing infinitely many rooms.
///
/// Walking west, left is `+z`. Room 1 is the first on that side and room 2
/// the first on the other, so the two sides alternate as you look along it.
pub fn at(reaches: f32, n: u64) -> Vec3 {
    let (west, _) = span();
    let middle = middle_z(reaches);
    // **Counted from nought and not from one.** The place room 1 used to
    // stand in is the one door in this corridor with no number on it, per
    // spec 0031, which is where room 0 would be if nought were a counting
    // number. Everything moved one slot along to make space, and the blank
    // drops into the alternating pattern exactly: on the even side and 1.5
    // from room 2, which is the spacing every other same-side pair has.
    let along = n as f32 * 0.5;
    let side = if n % 2 == 1 { 1.0 } else { -1.0 };

    vec3(
        west - APART * 0.5 - along * APART,
        0.0,
        middle + side * ACROSS * 0.5,
    )
}

/// Whether room `n` is on the left as you walk in.
pub fn on_the_left(n: u64) -> bool {
    n % 2 == 1
}

/// Where the one door with no number stands. Spec 0031.
///
/// Room nought's place, on the even side, and the first door you pass. It is
/// not a room: its plaque is blank, which is the whole of the signposting and
/// is also the joke, since the room that proves not everything can be
/// numbered is behind the one door that is not.
pub const BLANK: u64 = 0;

pub fn blank_door(reaches: f32) -> Vec3 {
    at(reaches, BLANK)
}

/// The strip of light under room `n`'s door, when somebody is in it.
///
/// It lies on the corridor's floor at the threshold, reaching [`STRIP_OUT`]
/// into the corridor. Spec 0029: this is what carries once a door is smaller
/// than a pixel, because what reads at distance is contrast and not size.
pub fn strip(reaches: f32, n: u64, out: f32) -> Aabb {
    let stands = at(reaches, n);
    let inward = if on_the_left(n) { -1.0 } else { 1.0 };
    let (near, far) = {
        // the leaf's outer face and not its middle. A strip measured from the
        // middle has half its length inside the door.
        let face = stands.z + inward * DOOR_THICK * 0.5;
        let reaches = face + inward * out;

        (face.min(reaches), face.max(reaches))
    };

    Aabb::new(
        vec3(stands.x - STRIP_WIDE * 0.5, STRIP_UP, near),
        vec3(stands.x + STRIP_WIDE * 0.5, STRIP_UP, far),
    )
}

/// What closes the corridor's far end.
///
/// There has to be something there, because a shell that simply stops leaves
/// the floor, the walls and the ceiling ending on an edge against the clear
/// colour, and an edge is a thing you can see. So it is closed, and what
/// closes it is drawn in the near black this building keeps for the space
/// behind the wall.
///
/// **Whether that reads as darkness or as a wall is Jake's to say.** It is
/// four rooms past the last drawn one and the corridor is unlit at that
/// distance, so the arithmetic says it cannot be told from the dark. The
/// arithmetic has been wrong about how a thing reads before.
pub fn end(reaches: f32) -> Aabb {
    let far = corridor_west();
    let middle = middle_z(reaches);

    Aabb::new(
        vec3(far - THICK, 0.0, middle - ACROSS * 0.5 - THICK),
        vec3(far, TALL, middle + ACROSS * 0.5 + THICK),
    )
}

/// The leaf of room `n`'s door where it stands, as a box.
///
/// Axis aligned, because everything solid in this building is, so a door
/// swung half way is the box its two ends fit in. The baths' door is built
/// the same way and for the same reason.
///
/// This is what you point at and what stops you. A door that is drawn and
/// not solid is a door you walk through, which is what the first build of
/// this room shipped.
pub fn leaf_box(reaches: f32, n: u64, open: f32) -> Aabb {
    let stands = at(reaches, n);
    let inward = if on_the_left(n) { -1.0 } else { 1.0 };
    let wide = door_wide(n);
    let swing = open.clamp(0.0, 1.0) * SWINGS_TO;
    let hinge = vec3(stands.x - wide * 0.5, 0.0, stands.z);
    let tip = hinge + vec3(swing.cos(), 0.0, inward * swing.sin()) * wide;

    // **Across the leaf, not around the whole box.** This inflated the
    // hinge-to-tip box by `DOOR_THICK` on every side, which is right for the
    // two faces of a thin leaf and wrong for its ends: it put 0.07 of solid
    // past the hinge, inside the opening the door had just cleared. A leaf
    // is a rectangle and its corners are where its corners are.
    let across = vec3(-swing.sin(), 0.0, inward * swing.cos()) * DOOR_THICK * 0.5;
    let corners = [hinge + across, hinge - across, tip + across, tip - across];
    let low = corners.iter().fold(corners[0], |was, is| was.min(*is));
    let high = corners.iter().fold(corners[0], |was, is| was.max(*is));

    Aabb::new(vec3(low.x, 0.0, low.z), vec3(high.x, DOOR_TALL, high.z))
}

/// Everything solid in here: the lobby's shell, the corridor's, and the desk.
///
/// The gallery's wall is not in this list. It stands already and is cut for
/// the doorway by [`gallery_west`], which `room.rs` asks for.
/// The lid over the lobby, which is what this room shows the sky.
///
/// Named rather than built inside `solid`, because spec 0040 lays a roof on
/// it and a second account of where it is would drift from the first.
/// **East to the far face of the wall it shares, not to its near one.**
/// Every other side of this lid oversails by a thickness and this one used to
/// stop flush, which left the 0.3 the gallery's west wall stands on roofed by
/// neither room. Along the solid wall nothing noticed. At the doorway, where
/// that strip is cut away, `Room::outside` probed it, found nothing over it,
/// and took the opening for a wall the sky could see: it plastered a face on
/// each side of the gap, facing each other. Both are culled from either room,
/// so the doorway looked open, and standing between them the view was plaster
/// whichever way you turned along the wall. Jake walked into the hotel and
/// found it.
///
/// Taken off `GALLERY_WIDE` rather than `east + THICK`, because that is where
/// `gallery_west` puts the wall's far face. The two agree today and they are
/// not the same question.
pub fn lid(reaches: f32) -> Aabb {
    let (west, _) = span();
    let (from, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, LOBBY_TALL, from - THICK),
        vec3(-GALLERY_WIDE * 0.5, LOBBY_TALL + THICK, to + THICK),
    )
}

/// The lobby's north wall, which is also the nook's south wall.
///
/// Named because two rooms want it. This room builds it and `standing` puts
/// it in the hotel's run, so both its faces wear the hotel's plaster, and the
/// nook's side of it is dressed by `room::nook_lining` with the grain the
/// rest of that room wears.
pub fn lobby_north(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (_, to) = across(reaches);

    Aabb::new(
        vec3(west - THICK, 0.0, to),
        vec3(east, LOBBY_TALL, to + THICK),
    )
}

pub fn solid(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (from, to) = across(reaches);
    let middle = middle_z(reaches);
    let corridor_from = middle - ACROSS * 0.5;
    let corridor_to = middle + ACROSS * 0.5;

    vec![
        floor(reaches),
        corridor_floor(reaches),
        desk(reaches),
        // the lobby's far wall, in two pieces with the other corridor's
        // mouth between them. Spec 0035: a second hall leaves this room,
        // east of the desk, so the desk stands between the two of them.
        Aabb::new(
            vec3(west - THICK, 0.0, from - THICK),
            vec3(
                crate::parked::twine::middle_x() - crate::parked::twine::ACROSS * 0.5,
                LOBBY_TALL,
                from,
            ),
        ),
        Aabb::new(
            vec3(
                crate::parked::twine::middle_x() + crate::parked::twine::ACROSS * 0.5,
                0.0,
                from - THICK,
            ),
            vec3(east, LOBBY_TALL, from),
        ),
        // and the head over that mouth, since the hall through it is the
        // corridor's height and this room is taller
        Aabb::new(
            vec3(
                crate::parked::twine::middle_x() - crate::parked::twine::ACROSS * 0.5,
                TALL,
                from - THICK,
            ),
            vec3(
                crate::parked::twine::middle_x() + crate::parked::twine::ACROSS * 0.5,
                LOBBY_TALL,
                from,
            ),
        ),
        lobby_north(reaches),
        // and its west wall, in two pieces with the corridor's mouth between
        // them
        Aabb::new(
            vec3(west - THICK, 0.0, from - THICK),
            vec3(west, LOBBY_TALL, corridor_from),
        ),
        Aabb::new(
            vec3(west - THICK, 0.0, corridor_to),
            vec3(west, LOBBY_TALL, to + THICK),
        ),
        // the head over the corridor's mouth, since the corridor is lower
        // than the room it leaves
        Aabb::new(
            vec3(west - THICK, TALL, corridor_from),
            vec3(west, LOBBY_TALL, corridor_to),
        ),
        lid(reaches),
        end(reaches),
        // the corridor's two walls and its ceiling, run past the drawn rooms.
        //
        // **The even side is in two pieces with the blank doorway between
        // them**, per spec 0031, the same way the gallery's walls are cut for
        // the hotel and the back room. The register built its own near wall
        // first, in exactly this plane, which is two faces in one plane and
        // is the fault this building has fought seven times. Jake walked the
        // corridor and saw the speckle.
        Aabb::new(
            vec3(corridor_west(), 0.0, corridor_from - THICK),
            vec3(
                blank_door(reaches).x - BLANK_WIDE * 0.5,
                TALL,
                corridor_from,
            ),
        ),
        Aabb::new(
            vec3(
                blank_door(reaches).x + BLANK_WIDE * 0.5,
                0.0,
                corridor_from - THICK,
            ),
            vec3(west + THICK, TALL, corridor_from),
        ),
        Aabb::new(
            vec3(corridor_west(), 0.0, corridor_to),
            vec3(west + THICK, TALL, corridor_to + THICK),
        ),
        Aabb::new(
            vec3(corridor_west(), TALL, corridor_from - THICK),
            vec3(west + THICK, TALL + THICK, corridor_to + THICK),
        ),
    ]
}

/// How big the desk is and where it stands.
///
/// **Built rather than blocked out.** The first one was a single box in the
/// wall's own colour, which is what a thing looks like before it is built:
/// Jake stood at it and said it does not make sense to a user, and from close
/// to it filled half the frame and read as a wall. A counter is a body, a top
/// that oversails it and a bead under that, which is what the cellar's bar
/// and the pool table's rails are, and the shadow lines do the rest.
pub const DESK_WIDE: f32 = 2.6;
pub const DESK_DEEP: f32 = 0.7;
pub const DESK_TALL: f32 = 1.1;

/// How far the top oversails the body, and how thick it and the bead are.
pub const DESK_OVER: f32 = 0.06;
pub const DESK_TOP: f32 = 0.055;
pub const DESK_BEAD: f32 = 0.022;

/// Where the board of pigeonholes hangs: on the wall the desk stands
/// against, which is the low `z` side.
///
/// **Not on the desk's far face.** The first build put it at the counter's
/// `max.z`, which is the side facing into the room, so the board floated in
/// front of the desk between it and whoever was standing there.
pub fn board(reaches: f32) -> Vec3 {
    let counter = desk(reaches);

    vec3(
        counter.center().x,
        BOARD_UP,
        across(reaches).0 + BOARD_THICK * 0.5,
    )
}

/// The board of pigeonholes behind the desk, one hook to a room.
///
/// **Furniture and not a record.** Spec 0029 struck a board of keys that was
/// a second account of who is in, because two lists of one thing drift. This
/// is the same object with no job: it says what kind of place this is, and
/// what is in it tells you nothing.
pub const BOARD_WIDE: f32 = 2.2;
pub const BOARD_TALL: f32 = 1.15;
pub const BOARD_UP: f32 = 1.3;
pub const BOARD_THICK: f32 = 0.08;
pub const HOLES_ACROSS: u32 = 8;
pub const HOLES_DOWN: u32 = 4;

/// The bell on the counter, which is the one turned thing in here.
pub const BELL_WIDE: f32 = 0.11;
pub const BELL_TALL: f32 = 0.08;

/// Its profile, for `cellar::turned`: a dome on a little foot.
pub fn bell_profile(v: f32) -> (f32, f32) {
    // a quarter circle, so the dome meets the counter square and closes at
    // the top rather than coming to a point
    let along = v * BELL_TALL;
    let round = (v * std::f32::consts::FRAC_PI_2).cos();

    (along, BELL_WIDE * round.max(0.04))
}

/// Where the sign hangs, which is over the corridor's mouth.
///
/// It is the one place a person is looking when they walk in, and what it
/// says is the premise: without it the desk's numbers are reported to
/// somebody who has not been told what is unusual about them.
/// How tall a line of it is, how thick the board is, and the gap between the
/// two lines.
///
/// **No width.** The first build gave both lines one width of 2.4, which is
/// three quarters of the corridor, and stretched each line's texture to fill
/// it: sixteen characters and twenty four at the same width means two sizes
/// of letter, and Jake's picture has the top line half again as large as the
/// bottom. Each board takes its width from its own text now, through
/// `blitzkit::text::room_for`, the way the back room's gauge words do.
///
/// It also hung them 0.26 apart while each was 0.42 tall, so they overlapped.
///
/// **This is a ceiling on the line and not the line.** What sets the height
/// is `SIGN_FITS` over the longest line's aspect, which for twenty four
/// characters in a mouth 2.65 wide comes to 0.110. Raising this number does
/// nothing at all, which is worth knowing before anybody tries to make the
/// sign bigger by changing it. Held by `the_sign_hangs_in_the_band`.
pub const SIGN_TALL: f32 = 0.17;

/// How much of the corridor's mouth the longest line may take.
///
/// The board hangs in the opening, so a line wider than the opening has its
/// ends behind the wall: the second line came out as "E ROOMS DO NOT RUN O T"
/// with THE and OUT lost behind the jambs. The letters are sized down from
/// here rather than up from a height, so a longer line shrinks the sign
/// instead of running off it.
pub const SIGN_FITS: f32 = ACROSS - 0.55;
pub const SIGN_THICK: f32 = 0.05;
pub const SIGN_GAP: f32 = 0.05;

/// How far under the corridor's head the board hangs.
///
/// Tucked up against it rather than hung in the opening. At 2.02 it was below
/// the door heads and across the middle of the view from the lobby, which is
/// what made it read as filling the room.
///
/// Then at 0.06 it cleared the head by six centimetres, which is clear in the
/// arithmetic and reads as touching from any angle but square on: Jake said
/// the text runs into the ceiling. Twenty centimetres is a gap you can see,
/// and the board still stops at 1.95, above the door heads, because the
/// lines are sized from the width of the mouth rather than from a height.
pub const SIGN_UNDER: f32 = 0.2;

/// Where the top of the sign is, which is the head over the corridor's mouth.
pub fn sign_top() -> f32 {
    TALL - SIGN_UNDER
}

/// What the sign says, which is the premise.
///
/// **Nothing in this room stated it.** A person walked in, saw a corridor,
/// and was then told "The hotel is full. 1 housed", which only means anything
/// to somebody who already knows the hotel has a room for every counting
/// number and that it being full is supposed to leave no space. Jake stood at
/// the desk and said it does not make sense to a user, and this is most of
/// why.
///
/// Two lines, because the paradox needs both halves: every room is taken,
/// and there are always more rooms.
///
/// **The second line is Jake's, twice over.** It read THE ROOMS DO NOT RUN
/// OUT, which is stock language: a hotel that does not run out of rooms is a
/// hotel with a great many rooms, and that is not what this one does. He said
/// the point is that there is always a new one. These are his words for it.
pub const SIGN_SAYS: [&str; 2] = ["EVERY ROOM TAKEN", "THERE ARE ALWAYS MORE ROOMS"];

/// What the desk says while a move runs and after one.
///
/// Reporting the event rather than the state. "1 housed" is a counter, and
/// what happened was that a full hotel took another guest and nobody left,
/// which is a sentence.
pub const DESK_FULL: &str = "Full, and nobody has left.";
pub const DESK_FREEING: &str = "Everybody is moving up a room.";

/// And what it says while the prime powers run, which is not that.
///
/// **"Everybody is moving up a room" is false under this move.** Spec 0034:
/// room 3 goes to room 8 and room 10 to room 1024, and most of the corridor
/// is leaving rather than shuffling along. A line that describes a shuffle
/// over a scatter is the desk telling you the wrong thing while you watch
/// the right one.
pub const DESK_SCATTERING: &str = "Everybody to a prime power. Nobody shares.";
pub const DESK_SCATTERED: &str = "All of them in, and the hotel is emptier than it was.";
pub const DESK_FILLING: &str = "The new ones are taking the rooms that opened.";
pub const DESK_HOUSED: &str = "housed since you came in";

/// The counter you stand at.
///
/// **Not across the way out.** The corridor leaves the lobby's west wall on
/// the middle line, so a desk on that line is a desk you walk round to get
/// anywhere. It stands against the north side instead, square to the way in,
/// so you come through the door and it is on your right with the corridor
/// straight ahead.
pub fn desk(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (from, _) = across(reaches);
    let middle = (west + east) * 0.5;

    Aabb::new(
        vec3(middle - DESK_WIDE * 0.5, 0.0, from + 0.9),
        vec3(middle + DESK_WIDE * 0.5, DESK_TALL, from + 0.9 + DESK_DEEP),
    )
}

/// Where somebody waiting for a room stands.
///
/// In front of the counter on the way-in side, facing it, which is where
/// anybody who has just come through the door and wants a room would be.
/// Spec 0033.
pub fn waiting(reaches: f32) -> Vec3 {
    let desk = desk(reaches);

    vec3((desk.min.x + desk.max.x) * 0.5, 0.0, desk.max.z + 0.62)
}

/// Who is in, and how far the last rule has travelled.
///
/// The corridor itself is arithmetic. This is the one thing about it that is
/// a state, and it is kept apart from the drawing so a rule can be applied
/// without a window.
/// Who is at the desk wanting a room.
///
/// **The arrival sets the difficulty rather than the dials setting the
/// answer.** aleph asked the player to find a rule that works, and its own
/// parked note says that fails because every rule works: three right answers
/// and a player has them in two minutes. Here the move is the same key every
/// time and what changes is whether it is enough.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrival {
    /// One guest. A shuffle of one along is enough, and the room should not
    /// pretend otherwise by doubling the whole hotel to seat one person.
    One,
    /// A coach of this many. A shuffle of that many along.
    Coach(u64),
    /// A coach with a room for every counting number, which is the one that
    /// cannot be done by shuffling at all and is what doubling is for.
    Endless,
    /// Endlessly many coaches, each of them endless. Spec 0034.
    ///
    /// **The one no stride does.** The three above are all shuffles, which is
    /// why every move in this building draws as a wave running west. This one
    /// sends everybody to a prime power, so it is a scatter: room 1 to room 2,
    /// room 3 to room 8, room 10 to room 1024. Most of them go somewhere too
    /// far to walk to and the corridor in front of you empties.
    Coaches,
}

impl Arrival {
    /// Who wants a room, said to the person standing at the desk.
    ///
    /// **Not "at the door".** There is no door in that room: you come in
    /// through the gallery and nothing in the fiction is a front door, so
    /// "at the door" points at nothing. And the player is the clerk, so the
    /// line is the situation in front of them rather than a narration of it.
    /// Jake read the first draft and said the language does not land.
    pub fn says(&self) -> String {
        match self {
            // Jake's, so no marker. It is the guest speaking rather than
            // the room narrating them, which is what the other two had to
            // follow.
            Arrival::One => String::from("I would like a room."),
            Arrival::Coach(n) => format!("We would like {} rooms.", n),
            Arrival::Endless => String::from("We would like a room for every number."),
            // and the fourth, which is the guest speaking too and follows
            // the shape of the three above
            Arrival::Coaches => {
                String::from("There is a coach for every number, and we fill them.")
            }
        }
    }

    /// The move that houses them: room n becomes room a*n + b.
    ///
    /// Shuffling along frees as many rooms as you shuffle, and no shuffle
    /// frees infinitely many. Doubling frees every odd room, which is
    /// endlessly many, and is the only answer to the last of these.
    /// Where the guest in room `n` goes, or nowhere that can be drawn.
    ///
    /// **A function rather than a stride and a shift.** Every move here used
    /// to be `n * stride + shift`, which is the shape a shuffle has and the
    /// shape a scatter has not. Spec 0034: if the prime powers had been bent
    /// into a `(stride, shift)` pair, something would have been fudged.
    ///
    /// Nothing is an answer. Room 64 doubling is room 128 and room 64 to
    /// 2⁶⁴ is not a number anybody can be in, so that guest has left the
    /// part of the corridor you can see, which is what actually happens to
    /// them.
    pub fn sends(&self, n: u64) -> Option<u64> {
        match self {
            Arrival::One => n.checked_add(1),
            Arrival::Coach(many) => n.checked_add(*many),
            Arrival::Endless => n.checked_mul(2),
            // 2ⁿ, which runs out of u64 at n = 64 and out of corridor long
            // before that
            // 2ⁿ. Past 63 there is no such room in a u64, which is a
            // long way past the end of anything drawn.
            Arrival::Coaches if n < 64 => 2u64.checked_pow(n as u32),
            Arrival::Coaches => None,
        }
    }

    /// And the room somebody in room `m` was in before this move, which is
    /// the only part of a move an endless corridor can afford to ask.
    ///
    /// **A move is read backwards out here.** Pushing every taken room
    /// forward means knowing which rooms are taken, which is a list, and a
    /// list is forty rooms of a corridor two thousand metres long. Asked the
    /// other way it is one division: room `m` has somebody in it if the room
    /// this move sends here had somebody in it. Nothing is an answer, and
    /// means nobody was sent to `m` at all.
    ///
    /// The inverse of [`sends`](Arrival::sends), which
    /// `every_move_can_be_read_backwards` holds the two to.
    pub fn came_from(&self, m: u64) -> Option<u64> {
        let from = match self {
            Arrival::One => m.checked_sub(1),
            Arrival::Coach(many) => m.checked_sub(*many),
            Arrival::Endless => m.is_multiple_of(2).then_some(m / 2),
            // 2ⁿ backwards is which power of two it is, and a room that is
            // not one came from nobody
            Arrival::Coaches => m.is_power_of_two().then(|| m.trailing_zeros() as u64),
        };

        from.filter(|from| *from >= 1)
    }

    /// Whether this move's arrivals take every room it frees.
    ///
    /// **The second half of what a move does.** Three of these four seat
    /// their guests in everything they open up, so the hotel is full again
    /// however it started. The coaches do not: seat s of coach c goes to
    /// room pˢ, so the rooms that fill are the odd prime powers and room 6
    /// stays empty for ever.
    ///
    /// This is a second account of [`fills`](Arrival::fills) and two lists of
    /// one thing drift, so `what_fills_everything_fills_everything` holds
    /// them together rather than a comment asking the next person to.
    pub fn fills_everything(&self) -> bool {
        match self {
            Arrival::Coaches => false,
            Arrival::One | Arrival::Coach(_) | Arrival::Endless => true,
        }
    }

    /// Whether an arrival takes room `n`, once the move has freed it.
    ///
    /// **The coaches do not take everything that is free.** A shuffle frees
    /// as many rooms as it has arrivals and they all fill, which is why this
    /// used to be every empty room without asking. Under the prime powers,
    /// seat s of coach c goes to room pˢ for an odd prime p, so the rooms
    /// that fill are the odd prime powers and the rest stay empty for ever:
    /// room 6 is 2 times 3 and belongs to nobody.
    pub fn fills(&self, n: u64) -> bool {
        match self {
            Arrival::Coaches => odd_prime_power(n),
            _ => true,
        }
    }
}

/// The `which`th odd prime, counted from one: 3, 5, 7, 11 and so on.
///
/// `#[cfg(test)]`, because the corridor never needs to know which coach
/// somebody came off: `fills` asks only whether a room is an odd prime
/// power, and that is answered by dividing rather than by counting primes.
/// This is here so the no-collisions claim can be stated in the terms the
/// rule is written in.
///
/// Spec 0034. The same arithmetic as `rule::primes::odd` in aleph, which is
/// the repo this room came out of and is not a dependency of this one.
#[cfg(test)]
pub fn odd_prime(which: u64) -> u64 {
    let mut found = 0;
    let mut at = 1u64;

    while found < which {
        at += 2;

        if (3..)
            .step_by(2)
            .take_while(|by| by * by <= at)
            .all(|by| !at.is_multiple_of(by))
        {
            found += 1;
        }
    }

    at
}

/// Whether `n` is a power of a single odd prime, which is where seat s of
/// coach c goes.
///
/// Spec 0034. Room 1 is nobody's, because a seat is counted from one and p⁰
/// would be every coach's at once.
pub fn odd_prime_power(n: u64) -> bool {
    if n < 3 {
        return false;
    }

    let mut left = n;
    let mut by = 3;

    // the one prime it is allowed to be made of, found by dividing out the
    // smallest odd factor it has
    while by * by <= left && !left.is_multiple_of(by) {
        by += 2;
    }

    if !left.is_multiple_of(by) {
        // a prime itself, and odd because two was ruled out above
        return !left.is_multiple_of(2);
    }

    while left.is_multiple_of(by) {
        left /= by;
    }

    left == 1
}

/// Which half of a move is running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Doing {
    /// Everybody already housed moving up, which frees rooms.
    Freeing,
    /// And the arrivals walking into them, which is the half that makes this
    /// a paradox rather than a subtraction.
    Filling,
}

/// Who is in, as a question you can ask of any room.
///
/// **Not a list of forty.** It was `Vec<bool>`, one entry a drawn room, and
/// that was right while the drawn rooms were rooms 1 to 40 for ever. `window`
/// made the drawn rooms follow you and this did not follow them: past the
/// fortieth every room answered empty, so every door out there stood wide
/// open and dark, for two kilometres. A hotel whose whole premise is that it
/// is full showed an empty one everywhere except the first sixty metres, and
/// the rank of open leaves is what Jake saw and called blocky.
///
/// A list cannot be the answer, because there is no end to index to. So the
/// state is what it has always been in the argument itself: a full hotel with
/// some moves applied to it. Asking room `m` walks those moves backwards,
/// which is a division apiece.
#[derive(Clone, Debug, PartialEq)]
pub enum Who {
    /// Everybody, which is where this starts and is the whole premise.
    Everybody,
    /// Whoever a move sent here, which is the freeing wave.
    Moved { was: Box<Who>, by: Arrival },
    /// And the arrivals in the rooms it freed, which is the filling wave.
    Filled { was: Box<Who>, by: Arrival },
}

impl Who {
    /// Whether room `n` has somebody in it.
    ///
    /// Room 0 is not a room. Rooms are counted from one, here and in `at`.
    pub fn has(&self, n: u64) -> bool {
        if n == 0 {
            return false;
        }

        match self {
            Who::Everybody => true,
            Who::Moved { was, by } => by.came_from(n).is_some_and(|from| was.has(from)),
            Who::Filled { was, by } => was.has(n) || by.fills(n),
        }
    }

    /// What a move leaves: whoever it sent here.
    fn moved(self, by: Arrival) -> Who {
        Who::Moved {
            was: Box::new(self),
            by,
        }
    }

    /// And what the arrivals leave, once they are in.
    ///
    /// **A move that fills everything it frees is a full hotel**, whatever it
    /// was before, so that is what it is called. Said here rather than left
    /// to the reader, because otherwise every key press adds two links to a
    /// chain that is walked once per door per frame and never gets shorter.
    fn filled(self, by: Arrival) -> Who {
        if by.fills_everything() {
            return Who::Everybody;
        }

        Who::Filled {
            was: Box::new(self),
            by,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Hotel {
    /// Who is in, as it stands.
    taken: Who,
    /// What the rule is moving everybody to, while the change is travelling,
    /// and nothing when it has arrived.
    going: Option<Who>,
    /// How far down the corridor the change has got, in rooms.
    ///
    /// **The change does not arrive everywhere at once.** Spec 0029: it
    /// starts at room 1 and runs away from you at a speed you can watch, so
    /// what you see is a wave going somewhere you cannot see the end of.
    front: f32,
    /// Which half of the move is running.
    doing: Doing,
    /// And what the rooms look like once the arrivals are in, held until the
    /// freeing wave has finished so the two run one after the other.
    then_filled: Option<Who>,
    /// Who is at the desk.
    at_the_door: Option<Arrival>,
    /// And which move is running, so the people crossing know where they are
    /// walking to. Spec 0034: under the prime powers that is not next door.
    moving_by: Option<Arrival>,
    /// How many have been housed since the building opened, which is the
    /// number that says nobody left.
    housed: u64,
    /// And how many are on their way in, which join the count when the
    /// filling wave has run rather than when the key was pressed.
    arriving: u64,
}

/// How fast the change runs down the corridor, in rooms a second.
///
/// Slow enough to watch and fast enough that the near end is not a wait: a
/// body walks at 4.2 and the rooms are 1.5 apart, so this is a little under
/// walking pace and the wave pulls away from you only if you stand still.
pub const TRAVELS_AT: f32 = 6.0;

/// How many rooms either side of the front have their doors open.
///
/// **This is what makes the move visible at all.** Spec 0029's wave moved
/// which rooms were taken, and for the move that houses one guest that is
/// nearly nothing to look at: everybody shifts up a room, every room is full
/// before and full after, and a shut door that stays shut shows none of it.
/// Jake pressed Enter and said the doors all looked closed. They were.
///
/// aleph's own parked note is the same sentence: the guests do not travel, so
/// the transformation is invisible and only its result shows. A wave of
/// occupancy is a wave of re-evaluation. This is a wave of people: a door
/// stands open while the front is passing it, because somebody is going
/// through it, and shuts behind them.
pub const RIPPLE: f32 = 2.0;

impl Default for Hotel {
    fn default() -> Self {
        Self::new()
    }
}

impl Hotel {
    /// A full hotel, which is where this starts and is the whole premise.
    pub fn new() -> Self {
        Self {
            taken: Who::Everybody,
            going: None,
            front: 0.0,
            doing: Doing::Freeing,
            then_filled: None,
            // somebody is always at the door, because a hotel with nobody
            // wanting a room has nothing to demonstrate and this is the
            // resting state rather than a thing you set up
            at_the_door: Some(Arrival::One),
            moving_by: None,
            housed: 0,
            arriving: 0,
        }
    }

    /// Who is at the desk, and nothing while a move is running.
    pub fn at_the_door(&self) -> Option<Arrival> {
        if self.going.is_some() {
            return None;
        }

        self.at_the_door
    }

    /// How many have been housed since the building opened.
    pub fn housed(&self) -> u64 {
        self.housed
    }

    /// Whether every drawn room has somebody in it.
    pub fn full(&self) -> bool {
        (1..=DRAWN).all(|n| self.taken(n))
    }

    /// House whoever is at the door.
    ///
    /// Two waves, one after the other. The first moves everybody already
    /// housed up by the move the arrival needs, which frees rooms. The second
    /// is the arrivals walking into them.
    ///
    /// **The second is the point.** A hotel that frees rooms and leaves them
    /// free has done a subtraction. A hotel that frees them and fills them
    /// and is full again, with more people in it and nobody gone, has done
    /// the thing this room is about.
    pub fn house(&mut self) {
        let Some(arrival) = self.at_the_door() else {
            return;
        };

        self.take(arrival);

        // the building's argument in order: a shuffle of one, a shuffle of
        // many, a doubling, and the thing none of those can do
        self.at_the_door = Some(match arrival {
            Arrival::One => Arrival::Coach(42),
            Arrival::Coach(_) => Arrival::Endless,
            Arrival::Endless => Arrival::Coaches,
            Arrival::Coaches => Arrival::One,
        });
    }

    /// House the one you walked out of the register with.
    ///
    /// **The desk says yes, and that is spec 0032.** Spec 0031 had it refuse,
    /// on the grounds that what you were holding was on no line of the book.
    /// Jake walked up to the desk and said that if he is holding a number and
    /// there is no limit to how many rooms there are, he should be able to
    /// get a room. He is right, and Cantor's argument never said otherwise:
    /// one guest can always be housed, by this same move. What cannot be done
    /// is finish the list, and the way to show that is to house them and let
    /// whoever did it walk the book again.
    ///
    /// One guest, so one shuffle, which is `Arrival::One`'s move. The queue
    /// at the door is left alone: this one did not come off a coach and does
    /// not take a turn in the cycle.
    pub fn house_carried(&mut self) {
        if self.going.is_some() {
            return;
        }

        self.take(Arrival::One);
    }

    /// The two waves, for whoever it is.
    fn take(&mut self, arrival: Arrival) {
        self.moving_by = Some(arrival);
        self.apply(arrival);

        // and who is in once the arrivals are in: whatever the move leaves,
        // plus the rooms they take
        let freed = self.going.clone().unwrap_or(Who::Everybody);

        // **How many came in is counted over the forty by the desk.** The
        // corridor has no end, so how many rooms a move fills is not a
        // number for three of these four, and the desk is at this end of it.
        let came = (1..=DRAWN)
            .filter(|n| !freed.has(*n) && arrival.fills(*n))
            .count() as u64;

        self.then_filled = Some(freed.filled(arrival));
        // **Counted when they are in, not when they are asked for.** This
        // said `housed += came` here, so the desk read "1 housed" the instant
        // you pressed, before either wave had moved a door. Jake pressed and
        // read the outcome of something he had not watched happen.
        self.arriving = came;
    }

    /// Which move is running, so the people crossing know where they are
    /// going. Spec 0034: under the prime powers that is not next door.
    pub fn moving_by(&self) -> Option<Arrival> {
        self.moving_by
    }

    /// Whether either wave is running, which is the window a thing the desk
    /// says about what just happened is worth showing in.
    pub fn moving(&self) -> bool {
        self.travelling().is_some() || self.filling()
    }

    /// Whether room `n` has somebody in it, as it stands this instant.
    ///
    /// While a change is travelling, a room the front has passed is already
    /// at what it is going to be and a room beyond it is still as it was.
    /// That is the whole of the wave.
    pub fn taken(&self, n: u64) -> bool {
        let Some(going) = self.going.as_ref() else {
            return self.taken.has(n);
        };

        if (n as f32) <= self.front {
            going.has(n)
        } else {
            self.taken.has(n)
        }
    }

    /// Move everybody: room n becomes room `by.sends(n)`.
    ///
    /// **Every room, rather than the forty that used to be drawn.** This
    /// walked rooms 1 to 40 and wrote where each of them landed, which can
    /// only answer for a corridor with an end. `Who` reads the move
    /// backwards instead, so a room eleven thousand along answers in the
    /// same division as room 2.
    ///
    /// A move rather than a closure, because reading one backwards is the
    /// move's own arithmetic and `Arrival` is where that arithmetic lives.
    pub fn apply(&mut self, by: Arrival) {
        self.going = Some(self.taken.clone().moved(by));
        self.front = 0.0;
    }

    /// Let the change travel, as far as `seen_to`.
    ///
    /// **It ran forty rooms and stopped, which is a number and not a
    /// reason.** Forty is `DRAWN`, and while the drawn rooms were rooms 1 to
    /// 40 for ever that was the far edge of what anybody could see, so the
    /// wave ending there ended out of sight. `window` made the drawn rooms
    /// follow you and this did not follow them. You can outwalk it: the
    /// front travels at 6 rooms a second and a body at 4.2 m/s covers 2.8,
    /// so pressing the key at the desk and running west puts you at about
    /// room 38 when the second wave ends, looking at rooms up to 57, and the
    /// sixteen past the front change with nothing passing them.
    ///
    /// So it runs until it has passed the last room you can see, which is
    /// what the caller knows and this does not. Standing at the desk that is
    /// room 40 and nothing moves, because that is where the window ends
    /// there too.
    pub fn tick(&mut self, dt: f32, seen_to: u64) {
        if self.going.is_none() {
            return;
        }

        self.front += dt * TRAVELS_AT;

        if self.front < seen_to as f32 {
            return;
        }

        self.taken = self.going.take().unwrap();
        self.front = 0.0;

        // the freeing wave has reached the end, so the arrivals set off
        if self.doing == Doing::Freeing {
            if let Some(filled) = self.then_filled.take() {
                self.going = Some(filled);
                self.doing = Doing::Filling;

                return;
            }
        }

        // and they are in, which is when they count
        self.housed += std::mem::take(&mut self.arriving);
        self.doing = Doing::Freeing;
    }

    /// Which half of a move is running, for what the desk says while it does.
    pub fn filling(&self) -> bool {
        self.going.is_some() && self.doing == Doing::Filling
    }

    /// How far the change has got, in rooms, and nothing when none is
    /// travelling.
    pub fn travelling(&self) -> Option<f32> {
        self.going.as_ref().map(|_| self.front)
    }

    /// How far open room `n`'s door stands, nought shut and one wide.
    ///
    /// A taken room is shut. A free one stands open and dark.
    pub fn open_by(&self, n: u64) -> f32 {
        // a door the wave is passing stands open, because somebody is going
        // through it. Eased, so it swings rather than snapping.
        if let Some(front) = self.travelling() {
            let near = (n as f32 - front).abs();

            if near < RIPPLE {
                return (1.0 - near / RIPPLE).clamp(0.0, 1.0);
            }
        }

        if self.taken(n) {
            0.0
        } else {
            1.0
        }
    }

    /// Whether there is light under room `n`'s door.
    ///
    /// The same fact as the door, asked the other way, which is why it is
    /// derived here rather than stored: two accounts of who is in would
    /// drift, and that is the fault under most of the others in this
    /// building.
    pub fn lit_under(&self, n: u64) -> bool {
        self.taken(n)
    }
}

/// A sconce on the wall between two doors, and how big it is.
///
/// **Every lamp sits in a fitting.** The cellar was told that once and the
/// baths did it again worse, with lamps along a ceiling and sconces on a wall
/// so the ceiling glowed at nothing and every sconce was dark. [`lamps`]
/// below is this list lifted, so there is no lamp here that is not in a
/// fitting and nobody has to remember.
pub const SCONCE: Vec3 = vec3(0.12, 0.26, 0.09);

/// How high they sit, and how far they stand off the wall.
pub const SCONCE_UP: f32 = 1.72;
pub const SCONCE_OUT: f32 = 0.07;

/// How far one reaches, and how hard it burns.
///
/// Short. A hotel corridor at night is lit in pools with dark between them,
/// which is the opposite of a room lit evenly, and the pools are what say
/// how long the corridor is.
/// First build was 3.4 and 2.6, which lit the whole corridor evenly and put
/// a wash on the ceiling. A corridor you can read end to end is a corridor
/// in an office. This is a hotel at night, so a sconce reaches about as far
/// as the next one and no further, and what is between them is dark.
///
/// It matters for more than mood. Spec 0029's whole answer to what stands
/// for a guest is the light under a shut door, and a light under a door in a
/// brightly lit corridor is a grey line.
pub const SCONCE_RANGE: f32 = 2.5;
pub const SCONCE_LIT: f32 = 0.85;

/// Where the sconces are: one on each side, in the wall between each pair of
/// doors on that side.
///
/// Derived from where the doors are rather than laid out again. Two lists of
/// one thing drift, and this building has paid for that more than once.
///
/// **Over the rooms that are drawn, which is the half of that this missed.**
/// It took the doors' positions from `at` and their range from `DRAWN`, so
/// the geometry followed you and the range did not: the sconces sat over
/// rooms 1 to 40 for ever, which is the first sixty metres. `window` moves
/// the doors down the corridor with you and nothing moved the lamps after
/// them, so past about room 40 there were doors, numbers, skirting and a
/// runner and no light anywhere to show them. At room 250 the corridor was
/// black, the ceiling with it, and the walls read as flat slabs because the
/// only light on them was the ambient fill. Jake, from down the corridor:
/// the ceiling is missing.
pub fn sconces(reaches: f32, standing: Vec3) -> Vec<Vec3> {
    let drawn = window(standing);
    let mut out = Vec::new();

    for n in drawn.clone() {
        // the gap between room n and the next on the same side, which is two
        // along
        if n + 2 > *drawn.end() {
            continue;
        }

        let one = at(reaches, n);
        let next = at(reaches, n + 2);
        let inward = if on_the_left(n) { -1.0 } else { 1.0 };

        out.push(vec3(
            (one.x + next.x) * 0.5,
            SCONCE_UP,
            one.z + inward * SCONCE_OUT,
        ));
    }

    out
}

/// The lobby's ceiling: a grid of beams across it.
///
/// **The one surface in this room nobody had spent anything on.** The lobby
/// is `LOBBY_TALL` where the corridor is `TALL`, and that difference is the
/// whole of what says which is which, per the note on `LOBBY_TALL`. A room
/// that says it by being taller and then shows you a flat grey lid is making
/// the argument and dropping it. Jake asked for this after a sweep found the
/// lid bare.
///
/// **A hotel's and not the cellar's.** `cellar::ceiling` is a library's:
/// coffers of 2.1 on beams of 0.2, which is close work for a room lined with
/// books. This room is arrived in, so the bays are wider and the beams are
/// deeper, and they are the hotel's own wood rather than the cellar's oak.
///
/// Both ways, because beams one way are stripes. The two sets cross and the
/// shared volume is inside both, which is hidden rather than fighting; the
/// cellar settled that already.
pub const LOBBY_COFFER: f32 = 2.6;
pub const LOBBY_BEAM: f32 = 0.26;
pub const LOBBY_BEAM_DOWN: f32 = 0.22;

pub fn lobby_ceiling(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (from, to) = across(reaches);
    let (deep, along) = (east - west, to - from);
    let (bays_x, bays_z) = (
        (deep / LOBBY_COFFER).round().max(1.0),
        (along / LOBBY_COFFER).round().max(1.0),
    );
    let (step_x, step_z) = (deep / bays_x, along / bays_z);
    let hung = LOBBY_TALL - LOBBY_BEAM_DOWN;
    let mut out = Vec::new();

    // **Cut for the doorway, the way the walls are.** The gallery's opening
    // into this room is full height, y 0 to `LOBBY_TALL` with no head over
    // it, so the beam that runs down the east wall crosses the top of it. A
    // run carried straight across an opening is the fault spec 0029 had with
    // this hotel's own skirting and rail, and `a_doorway_can_be_seen_out_of`
    // caught this one before anybody walked it: from inside the opening the
    // view into the lobby fell to a third of the view back into the gallery.
    let (door_from, door_to) = way(reaches);

    for n in 0..=(bays_x as usize) {
        let x = west + n as f32 * step_x;
        let spans: Vec<(f32, f32)> = if x + LOBBY_BEAM * 0.5 >= east {
            vec![(from, door_from), (door_to, to)]
        } else {
            vec![(from, to)]
        };

        for (a, b) in spans {
            if b - a <= LOBBY_BEAM {
                continue;
            }

            out.push(Aabb::new(
                vec3(x - LOBBY_BEAM * 0.5, hung, a),
                vec3(x + LOBBY_BEAM * 0.5, LOBBY_TALL, b),
            ));
        }
    }

    for n in 0..=(bays_z as usize) {
        let z = from + n as f32 * step_z;

        out.push(Aabb::new(
            vec3(west, hung, z - LOBBY_BEAM * 0.5),
            vec3(east, LOBBY_TALL, z + LOBBY_BEAM * 0.5),
        ));
    }

    out
}

/// The lobby's own fittings: one over the desk and one over the sign.
///
/// **The lobby had none.** The corridor was given sconces and the room you
/// arrive in was left with nothing, so the sign stating the premise and the
/// desk you work both stood in the dark. That is the same fault the corridor
/// had one round earlier, and this building has now had it three times.
pub fn lobby_sconces(reaches: f32) -> Vec<Vec3> {
    let counter = desk(reaches);
    let (west, _) = span();
    let middle = middle_z(reaches);
    let _ = counter;

    vec![
        // over the desk, on the wall it stands against, which is the low
        // `z` side: the desk faces the room and the wall is behind it
        vec3(
            counter.center().x,
            SCONCE_UP + 0.24,
            across(reaches).0 + SCONCE_OUT,
        ),
        // and one lighting the sign.
        //
        // **Above it and not in front of it.** The first one hung 0.55 out
        // into the lobby at the sign's own height, which put the fitting
        // between the reader and the board: Jake's picture has a grey box
        // through the middle of "DO NOT". A light that blocks the thing it
        // lights is worse than no light.
        vec3(west + 0.3, sign_top() + SCONCE.y * 0.5 + 0.06, middle),
    ]
}

/// And the lamps, which are those lifted a little into the shade's mouth.
/// Both sets together, which is what the rule is stated over.
///
/// `#[cfg(test)]` because the drawing wants them apart now that the lobby
/// burns differently from the corridor. The rule is still about all of them,
/// so this is where it is said.
#[cfg(test)]
pub fn lamps(reaches: f32, standing: Vec3) -> Vec<Vec3> {
    corridor_lamps(reaches, standing)
        .into_iter()
        .chain(lobby_lamps(reaches))
        .collect()
}

/// How far the lobby's two reach, and how hard they burn.
///
/// **A lobby is not a corridor at night.** The corridor's `SCONCE_RANGE` of
/// 2.5 is short on purpose: pools with dark between them are what say how
/// long it is. Giving the lobby the same number left a room eight metres by
/// six with two lamps that reached a quarter of the way across it, and the
/// one place anybody stands in it, where somebody waits at the desk, was 2.91
/// from the nearest and so lit by nothing at all.
///
/// Jake came in and said the shadowy figure is creepy. Painting it lighter
/// did nothing, because the problem was never the paint: a colour here is a
/// multiplier and there was no light to multiply.
pub const LOBBY_RANGE: f32 = 4.6;
pub const LOBBY_LIT: f32 = 0.95;

/// The corridor's lamps and the lobby's, which burn differently.
pub fn corridor_lamps(reaches: f32, standing: Vec3) -> Vec<Vec3> {
    sconces(reaches, standing)
        .into_iter()
        .map(|at| at + Vec3::Y * SCONCE.y * 0.3)
        .collect()
}

pub fn lobby_lamps(reaches: f32) -> Vec<Vec3> {
    lobby_sconces(reaches)
        .into_iter()
        .map(|at| at + Vec3::Y * SCONCE.y * 0.3)
        .collect()
}

/// The runner down the middle of the corridor.
///
/// A floor you can tell from the walls, which every room in this building has
/// needed. It is also what a hotel corridor actually has, and it is narrower
/// than the corridor so the floor shows either side of it and the light
/// strips under the doors lie on the floor rather than on the carpet.
pub const RUNNER_WIDE: f32 = 1.7;

/// How far it stands proud, for the same reason the strips do: a border laid
/// *in* a floor is two faces in one plane, which is the fault this building
/// has fought seven times.
pub const RUNNER_UP: f32 = 0.004;

pub fn runner(reaches: f32) -> Aabb {
    let (west, _) = span();
    let middle = middle_z(reaches);

    Aabb::new(
        vec3(
            corridor_west() + THICK,
            RUNNER_UP,
            middle - RUNNER_WIDE * 0.5,
        ),
        vec3(west, RUNNER_UP, middle + RUNNER_WIDE * 0.5),
    )
}

/// The skirting along the foot of each wall, and the rail along the top.
///
/// A long wall with nothing breaking it is one slab. These are what break it,
/// and they are the cheapest thing in the building that does: two runs a side
/// and they carry the corridor's length rather than fighting it.
pub const SKIRTING: f32 = 0.14;
pub const SKIRTING_OUT: f32 = 0.025;
pub const RAIL_UP: f32 = 1.02;
pub const RAIL_TALL: f32 = 0.07;

/// The architrave round a doorway: two jambs and a head.
///
/// A door that is one box in a flat wall is a brown rectangle. The frame is
/// what makes it a doorway, and it is three boxes.
pub const JAMB: f32 = 0.08;
pub const JAMB_OUT: f32 = 0.04;

/// The panels sunk into a door's face, and the handle on it.
///
/// **A door is not a slab.** Jake walked past the register's and said it does
/// not look like a door except that it is open, which was true of all forty
/// one of them: what makes a door read as a door close to is the two panels
/// and the thing you turn, and at a distance the frame already does the work.
pub const PANEL_IN: f32 = 0.11;
pub const PANEL_GAP: f32 = 0.055;
pub const PANEL_DEEP: f32 = 0.012;
pub const HANDLE: Vec3 = vec3(0.035, 0.035, 0.1);
pub const HANDLE_UP: f32 = 0.98;
pub const HANDLE_IN: f32 = 0.12;

// ---------------------------------------------------------------------------
// The crowd on the corridor floor. Spec 0033, moved here by spec 0063.
//
// **It lived in `people` and is the hotel's.** That module is the figure and
// its gait, which every other room with somebody in it reads; this is the
// band of guests behind a wave of occupancy, which is this room and nothing
// else. Parking the hotel and leaving its crowd in a live module would have
// been a live module reaching in here for something only this room wants.
// ---------------------------------------------------------------------------
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

        let leaves = at(reaches, n);
        // **Where the move sends them, not next door.** Every move was a
        // shuffle of one when this was written, so this said `n + 1`. Spec
        // 0034's prime powers send room 3 to room 8 and room 10 to room
        // 1024, and a corridor emptying westward is the point of it.
        let Some(to) = sends(n) else {
            continue;
        };
        let arrives = at(reaches, to);
        let way = arrives - leaves;
        // **At a walk, however far they are going.** Somebody bound for room
        // 1024 is four hundred metres west, and lerping them there over a
        // crossing is a streak rather than a person. They walk, and when the
        // crossing is over they are wherever that got them, which for most
        // of them is out of the window.
        let whole = way.length();
        let gone = along * whole.min(WALKS * CROSSES_IN / TRAVELS_AT);
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

#[cfg(test)]
mod tests {
    /// Spec 0040: the corridor's roof covers its own shell, all of it, and
    /// sits a hair above it.
    ///
    /// **Not level with the lobby's lid**, which was the first guess and is
    /// eighty centimetres out: this corridor is `aleph::TALL` and the lobby
    /// is the hall's. Worked from the wrong one the roof lay inside the
    /// corridor with the shell over it, and the test that caught it was
    /// this one.
    #[test]
    fn the_corridors_roof_covers_its_shell() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let (middle, size) = corridor_roof(reaches);
        let shell = solid(reaches)
            .into_iter()
            .find(|box_| box_.min.y >= TALL - 1e-3 && box_.min.x <= corridor_west() + 1e-3)
            .expect("the corridor's own ceiling");

        assert!(
            middle.y > shell.max.y && middle.y < shell.max.y + 0.05,
            "the roof is laid at {} over a shell whose top is {}",
            middle.y,
            shell.max.y
        );

        for (one, two, low, high, way) in [
            (
                middle.x - size.x * 0.5,
                middle.x + size.x * 0.5,
                shell.min.x,
                shell.max.x,
                "along",
            ),
            (
                middle.z - size.y * 0.5,
                middle.z + size.y * 0.5,
                shell.min.z,
                shell.max.z,
                "across",
            ),
        ] {
            assert!(
                one <= low + 1e-3 && two >= high - 1e-3,
                "{} the roof spans {}..{} over a shell of {}..{}",
                way,
                one,
                two,
                low,
                high
            );
        }
    }

    /// The lobby's ceiling is dressed, and dressed all the way to its walls.
    ///
    /// **The baths' lesson, applied before it could happen again.** That
    /// room's ribs stopped short of one end and left 4.24 metres of bare lid,
    /// because the dressing took its extent from something that was not the
    /// room. This takes its extent from `span` and `across`, which are the
    /// room, and this says so.
    #[test]
    fn the_lobbys_ceiling_reaches_its_walls() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let beams = lobby_ceiling(reaches);
        let (west, east) = span();
        let (from, to) = across(reaches);

        assert!(beams.len() >= 4, "{} beams is not a ceiling", beams.len());

        let got = |lo: f32, hi: f32, of: fn(&Aabb) -> (f32, f32)| {
            let low = beams.iter().map(|b| of(b).0).fold(f32::MAX, f32::min);
            let high = beams.iter().map(|b| of(b).1).fold(f32::MIN, f32::max);

            (low - lo, hi - high)
        };
        let (west_short, east_short) = got(west, east, |b| (b.min.x, b.max.x));
        let (south_short, north_short) = got(from, to, |b| (b.min.z, b.max.z));

        for (way, short) in [
            ("west", west_short),
            ("east", east_short),
            ("south", south_short),
            ("north", north_short),
        ] {
            assert!(
                short <= LOBBY_BEAM * 0.5 + 1e-3,
                "the beams stop {:.2} short of the {} wall",
                short,
                way
            );
        }
    }

    /// And nothing it hangs crosses the way in.
    ///
    /// The gallery's opening into this room is full height, so a beam run
    /// along the east wall crosses the top of it: a bar across a doorway,
    /// which is what spec 0029's skirting and rail did to the corridor's
    /// openings before Jake walked into one. `a_doorway_can_be_seen_out_of`
    /// catches it from inside the opening; this says it in the arithmetic,
    /// where it reads as a rule rather than as a ratio of two pictures.
    #[test]
    fn no_beam_crosses_the_way_in() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let (door_from, door_to) = way(reaches);
        let (_, east) = span();

        for beam in lobby_ceiling(reaches) {
            // the ones lying in the wall's own plane, which are thin across
            // and long along it. A beam running the other way stops at the
            // wall rather than crossing the opening, and from inside the
            // opening it is the view rather than a bar across it.
            if beam.max.x < east - 1e-3 || beam.max.x - beam.min.x > LOBBY_BEAM + 1e-3 {
                continue;
            }

            assert!(
                beam.max.z <= door_from + 1e-3 || beam.min.z >= door_to - 1e-3,
                "a beam runs z {:.2}..{:.2} across a doorway at {:.2}..{:.2}",
                beam.min.z,
                beam.max.z,
                door_from,
                door_to
            );
        }
    }

    /// And nothing it hangs comes down to where a head is.
    #[test]
    fn the_lobbys_beams_are_overhead() {
        let reaches = crate::room::Room::of(crate::games()).reaches;

        for beam in lobby_ceiling(reaches) {
            assert!(
                beam.min.y > crate::EYE + 0.3,
                "a beam hangs to {:.2}, which is in the way of a head at {:.2}",
                beam.min.y,
                crate::EYE
            );
            assert!(
                beam.max.y <= LOBBY_TALL + 1e-3,
                "a beam at {:.2} is above the ceiling it hangs from",
                beam.max.y
            );
        }
    }

    /// Spec 0029: every lamp in here sits in a fitting, and every fitting
    /// has a lamp.
    ///
    /// **Derived rather than placed twice.** The cellar lit a wall with
    /// nothing on it and the baths ran the lamps along a ceiling while the
    /// sconces stood on a wall, so every sconce was dark and the ceiling
    /// glowed at nothing. `lamps` is the sconces lifted, which is why there
    /// cannot be one without the other, and this is what keeps the split
    /// into corridor and lobby from quietly dropping a set.
    #[test]
    fn every_lamp_is_in_a_fitting() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let standing = at(reaches, 1);
        let fittings: Vec<_> = sconces(reaches, standing)
            .into_iter()
            .chain(lobby_sconces(reaches))
            .collect();
        let lamps = lamps(reaches, standing);

        assert_eq!(
            lamps.len(),
            fittings.len(),
            "there are {} lamps and {} fittings",
            lamps.len(),
            fittings.len()
        );
        assert_eq!(
            lamps.len(),
            corridor_lamps(reaches, standing).len() + lobby_lamps(reaches).len(),
            "the two sets do not add up to the whole"
        );

        for lamp in lamps.iter() {
            assert!(
                fittings.iter().any(|at| (*at - *lamp).length() < SCONCE.y),
                "a lamp at {:?} is in no fitting",
                lamp
            );
        }

        // and the lobby reaches the one place anybody stands in it, which it
        // did not: 2.91 away with a reach of 2.5, so whoever waits at the
        // desk was lit by nothing at all
        let stands = waiting(reaches);

        assert!(
            lobby_lamps(reaches)
                .iter()
                .any(|at| (*at - stands).length() < LOBBY_RANGE),
            "nothing in the lobby reaches where somebody waits"
        );
    }

    /// Wherever you stand in this corridor, the doors around you are lit.
    ///
    /// **The doors follow you down it and the sconces did not.** `window`
    /// moves the drawn rooms with you and `sconces` took its range from
    /// `DRAWN` instead, so the lamps stayed over rooms 1 to 40 while the
    /// doors walked off west without them. Sixty metres in there was nothing
    /// lighting anything: the doors, the numbers, the runner and the ceiling
    /// were all still drawn and all of them were black, which from inside
    /// reads as a corridor with no ceiling on it. Jake found it walking.
    ///
    /// Stated over the drawn doors rather than over a count of lamps,
    /// because what went wrong was not how many there were.
    #[test]
    fn the_drawn_doors_are_lit_wherever_you_stand() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let middle = middle_z(reaches);

        // the lobby end, a little way in, and far enough down that nothing
        // of the lobby's own light can reach
        for room in [1u64, 40, 250, 10_000] {
            let standing = vec3(at(reaches, room).x, 0.0, middle);
            let lamps = corridor_lamps(reaches, standing);

            for n in window(standing) {
                let door = at(reaches, n);

                assert!(
                    lamps
                        .iter()
                        .any(|lamp| (*lamp - door).length() < SCONCE_RANGE * 2.0),
                    "standing at room {room}, door {n} at {:.1} has no lamp within {:.1}",
                    door.x,
                    SCONCE_RANGE * 2.0
                );
            }
        }
    }

    /// Whether `n` is a power of a single prime, which is where everybody
    /// ends up under spec 0034.
    #[cfg(test)]
    fn prime_power(n: u64) -> bool {
        if n < 2 {
            return false;
        }

        let mut left = n;
        let mut by = 2;

        while by * by <= left {
            if left.is_multiple_of(by) {
                while left.is_multiple_of(by) {
                    left /= by;
                }

                return left == 1;
            }

            by += 1;
        }

        true
    }

    /// Spec 0034: no two guests are sent to the same room, which is the
    /// whole rule.
    ///
    /// A number factors one way only, so 2ⁿ is only ever 2ⁿ and 3ˢ is only
    /// ever 3ˢ. That is why every coach gets endlessly many rooms of its own
    /// rather than queueing behind the one before it.
    #[test]
    fn a_prime_power_is_nobodys_but_its_own() {
        let mut taken = std::collections::HashSet::new();

        // the residents, to 2ⁿ
        for n in 1..=40u64 {
            let to = Arrival::Coaches.sends(n).expect("room {n} goes nowhere");

            assert!(taken.insert(to), "room {} was sent somewhere taken", n);
        }

        // and seat s of coach c, to pˢ for the c-th odd prime
        for coach in 1..=12u64 {
            let p = odd_prime(coach);

            for seat in 1..=10u32 {
                let Some(to) = p.checked_pow(seat) else {
                    continue;
                };

                assert!(
                    taken.insert(to),
                    "seat {} of coach {} was sent to room {}, which is taken",
                    seat,
                    coach,
                    to
                );
            }
        }
    }

    /// Spec 0034: the ones who go too far leave, rather than piling up at
    /// the end of what is drawn.
    #[test]
    fn the_ones_who_go_too_far_leave() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Coaches);
        hotel.house();
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        // after the freeing wave only the powers of two that still fit are
        // standing, and everybody else has gone somewhere this corridor does
        // not reach
        for n in 1..=DRAWN {
            let stayed = (1..=63)
                .filter_map(|k| 2u64.checked_pow(k))
                .any(|to| to == n);

            assert_eq!(
                hotel.taken(n),
                stayed,
                "room {} is {} after the move and should not be",
                n,
                if hotel.taken(n) { "taken" } else { "empty" }
            );
        }
    }

    /// Spec 0034: what is lit afterwards is exactly the prime powers, and
    /// what is dark is every number with two different primes in it.
    ///
    /// **This is the ending.** The desk takes endlessly many endless coaches
    /// and the corridor has more dark doors than lit ones, which is better
    /// than a full hotel and is true. Room 6 is 2 times 3 and belongs to
    /// nobody.
    #[test]
    fn what_is_left_is_every_number_with_two_primes_in_it() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Coaches);
        hotel.house();
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        let mut lit = 0;

        for n in 1..=DRAWN {
            assert_eq!(
                hotel.taken(n),
                prime_power(n),
                "room {} is {} and is {}a prime power",
                n,
                if hotel.taken(n) { "taken" } else { "empty" },
                if prime_power(n) { "" } else { "not " }
            );

            if hotel.taken(n) {
                lit += 1;
            }
        }

        assert!(
            lit * 2 < DRAWN,
            "{} of {} doors are lit, which is not more dark than lit",
            lit,
            DRAWN
        );
    }

    /// Spec 0034: the three older arrivals are the moves they always were.
    ///
    /// A move became a function so that the prime powers would fit. Nothing
    /// about a shuffle changed, and this is what says so.
    #[test]
    fn the_older_moves_are_the_moves_they_were() {
        for n in [1u64, 2, 7, 40] {
            assert_eq!(Arrival::One.sends(n), Some(n + 1));
            assert_eq!(Arrival::Coach(42).sends(n), Some(n + 42));
            assert_eq!(Arrival::Endless.sends(n), Some(n * 2));
        }

        // and they still take every room their move frees, where the coaches
        // take only the ones that are theirs
        for n in [1u64, 6, 10, 12] {
            assert!(Arrival::One.fills(n));
            assert!(Arrival::Coach(42).fills(n));
            assert!(Arrival::Endless.fills(n));
            assert!(!Arrival::Coaches.fills(n), "room {} is nobody's", n);
        }
    }

    /// Spec 0034: four arrivals, and it comes round.
    #[test]
    fn the_cycle_comes_round() {
        let mut hotel = Hotel::new();
        let mut seen = Vec::new();

        for _ in 0..5 {
            seen.push(hotel.at_the_door().expect("nobody at the desk"));
            hotel.house();
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
        }

        assert_eq!(seen[0], Arrival::One);
        assert_eq!(seen[1], Arrival::Coach(42));
        assert_eq!(seen[2], Arrival::Endless);
        assert_eq!(seen[3], Arrival::Coaches);
        assert_eq!(seen[4], Arrival::One, "the cycle did not come round");
    }

    /// Spec 0031: the one door you walk through is wide enough to walk
    /// through.
    ///
    /// **It was a hit and miss passage.** At `DOOR_WIDE` the opening was
    /// 1.04 against a body 0.90 across, and the open leaf's collider ate
    /// another 0.07 of it by inflating its whole hinge-to-tip box by
    /// `DOOR_THICK` on every side, which put solid past the hinge inside the
    /// opening the door had just cleared. Driving an approach into it a
    /// frame at a time, 65, 66 and 67 got through, 68 and 69 did not, and 70
    /// did. The register is behind this door and nothing else leads there,
    /// so a player who bounces off it twice concludes the room is not there.
    #[test]
    fn the_one_door_you_walk_through_is_wide_enough() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let blank = blank_door(reaches);
        let leaf = leaf_box(reaches, BLANK, 1.0);
        let body = crate::RADIUS * 2.0;

        // the opening, and what the leaf standing open takes out of it
        let west = blank.x - BLANK_WIDE * 0.5;
        let east = blank.x + BLANK_WIDE * 0.5;
        let clear = east - leaf.max.x.max(west);

        assert!(
            clear > body + 0.3,
            "the way into the register is {:.2} clear for a body {:.2} across",
            clear,
            body
        );

        // and the leaf is a leaf rather than the box it sweeps: it reaches
        // past the hinge into the opening by a thickness at most
        // half a thickness, which is what a leaf hinged on the jamb reaches
        // back by. A whole one is the old box inflated on every side, and a
        // bound of exactly `DOOR_THICK` lands on that to within a rounding
        // error, so it is not a bound at all.
        assert!(
            leaf.max.x - west < DOOR_THICK * 0.6,
            "the open leaf stands {:.3} inside its own doorway, which is {:.3} thick",
            leaf.max.x - west,
            DOOR_THICK
        );

        // every other door is scenery and stays the size it was
        assert_eq!(door_wide(1), DOOR_WIDE);
        assert_eq!(door_wide(BLANK), BLANK_WIDE);
    }

    /// Spec 0033: whoever is waiting at the desk is facing the desk.
    ///
    /// **It was facing the way in.** Yaw nought is `-z` and the counter is
    /// at `-z` of where they stand, and the first build turned them a half
    /// turn from that, so the one person in the lobby stood with its back to
    /// the desk watching the door. Jake came in and said the shadowy figure
    /// is creepy. It was not the dark that did that.
    #[test]
    fn whoever_is_waiting_faces_the_desk() {
        let reaches = crate::room::Room::of(crate::games()).reaches;
        let stands = waiting(reaches);
        let counter = desk(reaches);

        // the desk is behind them in `z`, which is the way yaw nought looks
        assert!(
            counter.center().z < stands.z,
            "the desk is not in front of where somebody waits"
        );

        // and they are clear of it rather than standing in it
        assert!(
            stands.z > counter.max.z,
            "whoever waits is standing inside the counter"
        );
        assert!(
            stands.z - counter.max.z < 1.0,
            "whoever waits is {:.2} back from the counter, which is not at it",
            stands.z - counter.max.z
        );
    }

    /// Spec 0033: a guest at rest is still a light under a shut door.
    ///
    /// **The settlement that stays.** Spec 0029 decided a guest is a lit
    /// strip and spec 0033 only overturns it for the ones the wave has
    /// reached: there is nothing to see in a room you cannot go into, and a
    /// corridor at rest has to look exactly as it did before anybody was
    /// drawn in it.
    #[test]
    fn a_room_at_rest_is_still_a_light_under_a_door() {
        let hotel = Hotel::new();

        assert!(hotel.travelling().is_none(), "it is not at rest");
        assert!(
            crossing(
                crate::room::Room::of(crate::games()).reaches,
                hotel.travelling(),
                1,
                DRAWN,
                |n| Arrival::One.sends(n),
            )
            .is_empty(),
            "somebody is out of their room in a corridor at rest"
        );

        for n in 1..=DRAWN {
            assert!(hotel.taken(n), "room {} is empty in a full hotel", n);
            assert_eq!(
                hotel.open_by(n),
                0.0,
                "room {}'s door is open with nobody going anywhere",
                n
            );
        }
    }

    /// Spec 0029: the sign hangs in the band between the door heads and the
    /// corridor's head, clear of both.
    ///
    /// **Both edges have been wrong.** The first build hung the top at 2.02,
    /// below the door heads and across the middle of the view in from the
    /// lobby. The fix put the top 0.06 under the head, which is a gap in the
    /// arithmetic and none at all on screen: Jake said the text runs into the
    /// ceiling. The band is what is left between the two, and the lines and
    /// the gap are sized to sit inside it rather than either one being
    /// chosen on its own.
    #[test]
    fn the_sign_hangs_in_the_band() {
        let wide = super::SIGN_SAYS
            .iter()
            .map(|says| blitzkit::text::room_for(says, 1.0).x)
            .fold(0.0f32, f32::max);

        // the same height `draw_the_hotel` works the lines out at, which for
        // these two lines is the width cap rather than `SIGN_TALL`
        let line = super::SIGN_TALL.min(super::SIGN_FITS / wide);
        let top = super::sign_top();
        let foot = top - line * 2.0 - super::SIGN_GAP;

        assert!(
            super::TALL - top >= 0.15,
            "the sign is {:.3} under the head, which reads as touching it",
            super::TALL - top
        );

        assert!(
            foot >= super::DOOR_TALL,
            "the sign reaches to {:.3}, below the door heads at {:.3}",
            foot,
            super::DOOR_TALL
        );
    }

    use super::*;
    use crate::room::Room;
    use crate::walk;

    fn reaches() -> f32 {
        Room::of(crate::games()).reaches
    }

    /// Spec 0029: odd on the left as you walk in, even on the right.
    ///
    /// Not decoration. Doubling sends every guest into an even room, so the
    /// whole left wall opens in one move, and that only reads because the
    /// sides are split odd and even.
    #[test]
    fn the_sides_alternate() {
        let reaches = reaches();
        let middle = middle_z(reaches);

        for n in 1..=DRAWN {
            let stands = at(reaches, n);

            if n % 2 == 1 {
                assert!(
                    stands.z > middle,
                    "room {} is odd and stands on the right at z {}",
                    n,
                    stands.z
                );
            } else {
                assert!(
                    stands.z < middle,
                    "room {} is even and stands on the left at z {}",
                    n,
                    stands.z
                );
            }
        }
    }

    /// And they run away from the desk rather than toward it.
    #[test]
    fn the_corridor_runs_away_from_the_lobby() {
        let reaches = reaches();
        let (west, _) = span();

        let mut last = west;

        for n in 1..=DRAWN {
            let x = at(reaches, n).x;

            assert!(x < west, "room {} is not west of the lobby", n);
            assert!(x <= last, "room {} doubles back", n);

            last = x;
        }
    }

    /// Spec 0029: doubling frees every room on the left and none on the
    /// right.
    #[test]
    fn doubling_empties_the_odd_side() {
        let mut hotel = Hotel::new();

        for n in 1..=DRAWN {
            assert!(hotel.taken(n), "room {} started empty", n);
        }

        hotel.apply(Arrival::Endless);
        // let the whole wave through
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        for n in 1..=DRAWN {
            if on_the_left(n) {
                assert!(!hotel.taken(n), "room {} is odd and still taken", n);
            } else {
                assert!(hotel.taken(n), "room {} is even and stands empty", n);
            }
        }
    }

    /// Spec 0029: the change travels rather than arriving everywhere at once.
    ///
    /// This is the half aleph could not have. Its own parked note calls the
    /// result the lesser half, because the guests never travel and only the
    /// outcome shows.
    #[test]
    fn the_change_travels() {
        let mut hotel = Hotel::new();

        hotel.apply(Arrival::Endless);

        // one room's worth of travel, and nothing beyond it has moved
        hotel.tick(1.0 / TRAVELS_AT, DRAWN);

        let front = hotel.travelling().expect("nothing is travelling");

        assert!((1.0..2.0).contains(&front), "the front is at {}", front);
        assert!(
            !hotel.taken(1),
            "room 1 is behind the front and did not move"
        );
        assert!(
            hotel.taken(3),
            "room 3 is beyond the front and should be as it was"
        );
        assert!(
            hotel.taken(DRAWN),
            "the far end moved before the wave reached it"
        );
    }

    /// And a control: with no rule applied, nothing is travelling and nothing
    /// moves however long it is ticked.
    #[test]
    fn nothing_travels_on_its_own() {
        let mut hotel = Hotel::new();

        hotel.tick(60.0, DRAWN);

        assert!(hotel.travelling().is_none());

        for n in 1..=DRAWN {
            assert!(hotel.taken(n), "room {} emptied with no rule applied", n);
        }
    }

    /// Spec 0029: the light and the door never disagree, once nothing is
    /// moving.
    ///
    /// **While a wave is passing they do, and that is the point.** A door
    /// the front is going through stands open because somebody is going
    /// through it, and the room behind it is still somebody's. The light
    /// says who is in and the door says who is moving, so they only have to
    /// agree when nobody is.
    /// A hotel that has had nothing done to it is full the whole way.
    ///
    /// **It was full for sixty metres and empty for two kilometres.** Who was
    /// in was a list of forty, indexed by the absolute room number, and
    /// `window` made the drawn rooms follow you without it. Past the
    /// fortieth every room answered empty, so `open_by` swung every door out
    /// there wide and `lit_under` put a light under none of them. Jake walked
    /// west and called it blocky, which is the rank of open leaves cutting
    /// into the corridor.
    #[test]
    fn a_full_hotel_is_full_wherever_you_look() {
        let hotel = Hotel::new();

        // the fortieth, the first one past it, one a player actually reaches
        // on foot, and one far enough out that only arithmetic gets there
        for n in [1, DRAWN, DRAWN + 1, 250, 10_000, MOST] {
            assert!(hotel.taken(n), "room {} is empty in a full hotel", n);
            assert_eq!(hotel.open_by(n), 0.0, "room {}'s door stands open", n);
            assert!(hotel.lit_under(n), "room {} has no light under its door", n);
        }
    }

    /// And the rule reaches as far as the corridor does.
    ///
    /// Under spec 0034 the coaches fill the odd prime powers and nothing
    /// else, so room 6 is nobody's. That has to be as true at 2187 as at 9,
    /// or the argument stops where the old list did.
    #[test]
    fn the_rule_reaches_past_the_fortieth() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Coaches);
        hotel.house();

        // both waves, so the arrivals are in
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        for n in [9u64, 2048, 2187, 14_641] {
            assert!(
                hotel.taken(n),
                "room {} is a prime power and stands empty",
                n
            );
        }

        // 6 is 2·3 and belongs to nobody, and so does every multiple of two
        // primes however far out
        for n in [6u64, 2190, 9999] {
            assert!(!hotel.taken(n), "room {} belongs to nobody and is taken", n);
        }
    }

    /// A wave passes every room you can see before it ends.
    ///
    /// **It ran forty rooms, because forty was once the far edge of the
    /// window.** `window` moved that edge and the wave kept its number, and
    /// the front travels faster than a body walks, so you can press the key
    /// at the desk, run west, and watch rooms ahead of you change with
    /// nothing passing them.
    ///
    /// Stated over the coaches, which is the one move that leaves rooms
    /// empty, so there is something to see at room 57.
    #[test]
    fn the_wave_passes_everything_you_can_see() {
        const SEEN_TO: u64 = 57;

        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Coaches);
        hotel.house();

        // 57 is 3 times 19, so the coaches leave it empty and it is taken
        // until the front gets there
        assert!(odd_prime_power(9), "9 is a prime power");
        assert!(!odd_prime_power(SEEN_TO), "57 is nobody's");

        while hotel
            .travelling()
            .is_some_and(|front| front < SEEN_TO as f32 - 1.0)
        {
            hotel.tick(0.05, SEEN_TO);
        }

        assert!(
            hotel.taken(SEEN_TO),
            "room {} changed before the wave reached it",
            SEEN_TO
        );

        // and once it has passed, it has
        for _ in 0..200 {
            hotel.tick(0.05, SEEN_TO);
        }

        assert!(
            !hotel.taken(SEEN_TO),
            "room {} never changed at all",
            SEEN_TO
        );
    }

    /// Reading a move backwards is reading the same move.
    ///
    /// `Who` asks `came_from` where `apply` used to push with `sends`, so the
    /// two are one account of a move seen from either end. They are written
    /// separately, which is what this holds together.
    #[test]
    fn every_move_can_be_read_backwards() {
        for by in [
            Arrival::One,
            Arrival::Coach(42),
            Arrival::Endless,
            Arrival::Coaches,
        ] {
            for n in 1..=200u64 {
                let Some(to) = by.sends(n) else { continue };

                assert_eq!(
                    by.came_from(to),
                    Some(n),
                    "{:?} sends {} to {} and reads it back as somebody else",
                    by,
                    n,
                    to
                );
            }

            // and a room nobody was sent to says so, rather than naming
            // whoever is nearest
            for m in 1..=200u64 {
                if let Some(from) = by.came_from(m) {
                    assert_eq!(
                        by.sends(from),
                        Some(m),
                        "{:?} reads {} back as {}, who is sent elsewhere",
                        by,
                        m,
                        from
                    );
                }
            }
        }
    }

    /// And what fills everything fills everything.
    ///
    /// `fills_everything` is a second account of `fills`, which is how this
    /// building keeps breaking itself, so the two are held together rather
    /// than left to whoever edits one of them.
    #[test]
    fn what_fills_everything_fills_everything() {
        for by in [
            Arrival::One,
            Arrival::Coach(42),
            Arrival::Endless,
            Arrival::Coaches,
        ] {
            assert_eq!(
                by.fills_everything(),
                (1..=200u64).all(|n| by.fills(n)),
                "{:?} disagrees with itself about what it fills",
                by
            );
        }
    }

    #[test]
    fn the_light_says_what_the_door_says() {
        let mut hotel = Hotel::new();

        // the one move that does not fill everything it frees, so the hotel
        // this is stated over has empty rooms in it to be wrong about
        hotel.apply(Arrival::Coaches);

        for _ in 0..400 {
            hotel.tick(0.05, DRAWN);

            if hotel.travelling().is_some() {
                continue;
            }

            // past the fortieth as well as inside it, which is where the
            // two used to agree only by both being wrong
            for n in (1..=DRAWN).chain([250, 10_000]) {
                assert_eq!(
                    hotel.lit_under(n),
                    hotel.open_by(n) == 0.0,
                    "room {} has a door and a light that disagree at rest",
                    n
                );
            }
        }
    }

    /// And while a wave is passing, a door it is going through stands open
    /// even though somebody is in that room.
    ///
    /// Without this the move that houses one guest shows nothing at all:
    /// everybody shifts up a room, every room is full before and after, and a
    /// shut door that stays shut says none of it. Jake pressed Enter and said
    /// the doors all looked closed. They were.
    #[test]
    fn a_door_the_wave_is_passing_stands_open() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::One);
        hotel.house();

        let mut ever_open = 0;

        for _ in 0..600 {
            hotel.tick(0.02, DRAWN);

            let Some(front) = hotel.travelling() else {
                continue;
            };

            let at = front.round().max(1.0) as u64;

            if at <= DRAWN && hotel.taken(at) && hotel.open_by(at) > 0.4 {
                ever_open += 1;
            }
        }

        assert!(
            ever_open > 0,
            "no taken room ever opened, so the guests never looked like they moved"
        );
    }

    /// Spec 0029: no strip is level with the floor it lies on.
    ///
    /// A glowing quad coplanar with the floor is two faces in one plane,
    /// which is the fault this building has fought seven times.
    #[test]
    fn no_strip_is_in_the_floor() {
        let reaches = reaches();
        let floor = corridor_floor(reaches).max.y;

        for n in 1..=DRAWN {
            let lies = strip(reaches, n, STRIP_STEPS[STRIP_STEPS.len() - 1].0);

            assert!(
                lies.min.y > floor,
                "room {}'s strip is at {} and the floor is at {}",
                n,
                lies.min.y,
                floor
            );
        }
    }

    /// And it lies in the corridor rather than through a wall.
    #[test]
    fn every_strip_is_on_the_corridors_floor() {
        let reaches = reaches();
        let middle = middle_z(reaches);
        let (from, to) = (middle - ACROSS * 0.5, middle + ACROSS * 0.5);

        for n in 1..=DRAWN {
            let lies = strip(reaches, n, STRIP_STEPS[STRIP_STEPS.len() - 1].0);

            assert!(
                lies.min.z >= from - 1e-4 && lies.max.z <= to + 1e-4,
                "room {}'s strip runs z {} to {} and the corridor is {} to {}",
                n,
                lies.min.z,
                lies.max.z,
                from,
                to
            );
        }
    }

    /// Spec 0029: room numbers are right at the far end as well as the near
    /// one, where a float is at its worst.
    #[test]
    fn the_last_room_is_numbered_correctly() {
        let reaches = reaches();
        let (west, _) = span();

        for n in [1u64, 2, DRAWN - 1, DRAWN] {
            let x = at(reaches, n).x;
            let along = n as f32 * 0.5;
            let want = west - APART * 0.5 - along * APART;

            assert!(
                (x - want).abs() < 1e-3,
                "room {} stands at {} and should at {}",
                n,
                x,
                want
            );
        }

        // and no two rooms stand in the same place
        for n in 1..DRAWN {
            let one = at(reaches, n);
            let next = at(reaches, n + 2);

            assert!(
                (one.x - next.x).abs() > APART * 0.5,
                "rooms {} and {} are on top of each other",
                n,
                n + 2
            );
        }
    }

    /// Spec 0029: nothing of the corridor is drawn inside another room.
    #[test]
    fn nothing_of_the_corridor_is_in_another_room() {
        let reaches = reaches();
        let mine = solid(reaches);

        let theirs: Vec<(&str, Vec<Aabb>)> = vec![
            ("the garden", crate::garden::solid(reaches)),
            ("the baths", crate::spa::solid(reaches)),
            ("the back room", crate::back::solid(reaches)),
            ("behind the wall", crate::behind::solid(reaches)),
            ("the tesseract", crate::tesseract::solid()),
        ];

        for (name, boxes) in theirs {
            for one in &mine {
                for other in &boxes {
                    let over = one.min.x < other.max.x - 1e-4
                        && other.min.x < one.max.x - 1e-4
                        && one.min.y < other.max.y - 1e-4
                        && other.min.y < one.max.y - 1e-4
                        && one.min.z < other.max.z - 1e-4
                        && other.min.z < one.max.z - 1e-4;

                    assert!(!over, "the hotel runs into {} at {:?}", name, other);
                }
            }
        }
    }

    /// Spec 0029: the corridor's floor clears the cellar's ceiling.
    ///
    /// **It does not have to miss it.** A face buried inside another solid is
    /// hidden rather than fighting, and the hall's own floor already overlaps
    /// a cellar box. What may not happen is the cellar reaching up to or past
    /// the surface you walk on, because a face at the walking surface stops
    /// you dead and `walk` reports it as nothing at all.
    #[test]
    fn the_corridor_clears_the_cellar() {
        let reaches = reaches();
        let floor = corridor_floor(reaches);
        let walk_on = floor.max.y;

        for b in crate::cellar::solid(reaches) {
            let under = b.min.x < floor.max.x
                && floor.min.x < b.max.x
                && b.min.z < floor.max.z
                && floor.min.z < b.max.z;

            if !under {
                continue;
            }

            assert!(
                b.max.y < walk_on - 1e-4,
                "the cellar reaches y {} under a corridor you walk at {}",
                b.max.y,
                walk_on
            );
        }
    }

    /// Spec 0029: you walk from the gallery into the corridor without your
    /// feet rising.
    ///
    /// Walked rather than measured. The pool's steps had four legal rises and
    /// were one solid block, and every rise passed.
    /// **It walked in from the gallery until spec 0063.** That spec
    /// parked this room and walled the gallery's west doorway up, so there
    /// is no way in from there any more and the walk starts in the lobby.
    /// What it holds is unchanged: that the corridor is walkable, which is
    /// the thing the pool's steps taught this building to check by walking
    /// rather than by measuring.
    #[test]
    fn walked_from_the_lobby_into_the_corridor() {
        let reaches = reaches();
        let room = Room::of(crate::games());
        // **The building no longer holds this room, so this hands it over.**
        // Spec 0063 parked it: `room.solid()` is the rest of the building,
        // which a walk still needs so it does not walk through the wall it
        // shares, and the room's own boxes come from the room.
        let solid: Vec<Aabb> = room
            .solid()
            .into_iter()
            .chain(solid(reaches))
            .chain(window(at(reaches, 1)).map(|n| leaf_box(reaches, n, 0.0)))
            .chain(std::iter::once(leaf_box(reaches, BLANK, 1.0)))
            .collect();
        let middle = middle_z(reaches);

        // dropped in from above rather than placed, because a point on the
        // floor can be inside the furniture
        let from = vec3(span().0 + 1.0, 0.6, middle);
        let to = vec3(at(reaches, DRAWN).x, 0.0, middle);

        let got = walked(&solid, from, to, 30.0);

        assert!(
            got.x < at(reaches, 6).x,
            "walked from the lobby only as far as {} and room 6 is at {}",
            got.x,
            at(reaches, 6).x
        );
        assert!(
            got.y < 0.2,
            "feet rose to {} on the way down the corridor",
            got.y
        );
    }

    /// Spec 0029: a body walks the corridor with doors open on both sides.
    ///
    /// The case that fails: two facing doors at [`SWINGS_TO`] leave 1.13 m of
    /// a 3.2 m corridor, against a body of 0.9 and this building's usual 1.05
    /// of clearance.
    #[test]
    fn walked_with_every_door_open() {
        let leaf = DOOR_WIDE * SWINGS_TO.sin();
        let clear = ACROSS - leaf * 2.0;

        assert!(
            clear > 1.05,
            "two facing doors leave {} and a body wants 1.05",
            clear
        );
        assert!(
            clear > 0.9,
            "two facing doors leave {} and a body is 0.9 across",
            clear
        );
    }

    /// Spec 0029: you can stand at the desk and see room 1.
    #[test]
    fn room_one_is_in_view_from_the_desk() {
        let reaches = reaches();
        let counter = desk(reaches);
        let stands = vec3(
            (counter.min.x + counter.max.x) * 0.5,
            0.0,
            counter.max.z + 1.0,
        );
        let one = at(reaches, 1);

        assert!(
            one.x < stands.x,
            "room 1 is not west of where you stand at the desk"
        );
        assert!(
            (one.x - stands.x).abs() < DEEP + APART * 3.0,
            "room 1 is {} from the desk",
            (one.x - stands.x).abs()
        );
    }

    /// Spec 0029: the doorway here and the back room's are one account.
    #[test]
    fn the_two_doorways_are_one_account() {
        let reaches = reaches();

        assert_eq!(way(reaches), crate::back::way(reaches));
    }

    /// Spec 0029: a door is solid, so you cannot walk through one.
    ///
    /// Drawn and not solid is what the first build shipped, and Jake found it
    /// by walking into a door.
    #[test]
    fn a_door_is_solid() {
        let reaches = reaches();

        for n in [1u64, 2, 9, DRAWN] {
            let shut = leaf_box(reaches, n, 0.0);
            let open = leaf_box(reaches, n, 1.0);

            assert!(shut.size().x > 0.0 && shut.size().y > 0.0);
            assert!(
                open.size().z > shut.size().z,
                "room {}'s door takes up no more corridor open than shut",
                n
            );
            assert!(
                open.max.y > DOOR_TALL * 0.9,
                "room {}'s leaf is not door height",
                n
            );
        }
    }

    /// And an open one reaches into the corridor rather than through the wall.
    #[test]
    fn an_open_door_swings_into_the_corridor() {
        let reaches = reaches();

        for n in 1..=DRAWN {
            let open = leaf_box(reaches, n, 1.0);
            let inward = if on_the_left(n) { -1.0 } else { 1.0 };
            let reached = if inward < 0.0 { open.min.z } else { open.max.z };
            let from_the_wall = (reached - at(reaches, n).z) * inward;

            assert!(
                from_the_wall > DOOR_WIDE * 0.5,
                "room {}'s open door reaches {} into the corridor",
                n,
                from_the_wall
            );
        }
    }

    /// Spec 0029: the light under a door is three quads and not one.
    #[test]
    fn the_light_falls_off_rather_than_stopping() {
        let mut last_out = 0.0;
        let mut last_strength = f32::MAX;

        for (out, strength) in STRIP_STEPS {
            assert!(out > last_out, "the steps do not reach further each time");
            assert!(strength < last_strength, "the steps do not dim");

            last_out = out;
            last_strength = strength;
        }

        assert!(
            last_out > STRIP_STEPS[0].0 * 2.0,
            "the furthest step reaches {}, which is barely past the first",
            last_out
        );
    }

    /// Spec 0030: after a move the hotel is full again.
    ///
    /// This is the whole point and the thing both earlier builds missed.
    /// Freeing rooms and leaving them free is a subtraction. Freeing them,
    /// filling them, and being full again with more people in it is the
    /// paradox.
    #[test]
    fn the_hotel_is_full_again() {
        for arrival in [Arrival::One, Arrival::Coach(9), Arrival::Endless] {
            let mut hotel = Hotel::new();

            hotel.at_the_door = Some(arrival);
            assert!(hotel.full(), "{:?}: it did not start full", arrival);

            hotel.house();
            // both waves all the way through
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

            assert!(hotel.full(), "{:?}: it is not full again", arrival);
        }
    }

    /// And more people are in it, with nobody having left.
    #[test]
    fn nobody_left_and_more_arrived() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Coach(9));
        hotel.house();

        // **Not yet.** They are counted when they are in, not when they are
        // asked for: the desk used to read the outcome the instant you
        // pressed, before either wave had moved a door.
        assert_eq!(hotel.housed(), 0, "they were counted before they walked in");

        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        assert_eq!(
            hotel.housed(),
            9,
            "nine arrived and {} were housed",
            hotel.housed()
        );
    }

    /// One guest does not need the doubling, because a shuffle of one is
    /// enough and the room should not say otherwise.
    #[test]
    fn one_guest_does_not_need_the_doubling() {
        // a shuffle of one, and of forty two, which is what those two are
        assert_eq!(Arrival::One.sends(7), Some(8));
        assert_eq!(Arrival::Coach(42).sends(7), Some(49));
    }

    /// And a coach with a room for every number is the one that cannot be
    /// shuffled and has to be doubled.
    #[test]
    fn an_endless_coach_is_housed() {
        assert_eq!(Arrival::Endless.sends(7), Some(14));

        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Endless);
        hotel.house();
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        // the freeing wave is through: every odd room stands open
        for n in 1..=DRAWN {
            if on_the_left(n) {
                assert!(!hotel.taken(n), "room {} is odd and still taken", n);
            }
        }

        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0, DRAWN);

        assert!(hotel.full(), "the endless coach did not fill the odd side");
        assert!(
            hotel.housed() >= DRAWN / 2,
            "only {} were housed",
            hotel.housed()
        );
    }

    /// Spec 0030: the filling follows the freeing rather than happening with
    /// it, so the two waves are two.
    #[test]
    fn the_filling_follows_the_freeing() {
        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Endless);
        hotel.house();

        assert!(
            !hotel.filling(),
            "it is filling before it has freed anything"
        );

        hotel.tick(2.0 / TRAVELS_AT, DRAWN);
        assert!(!hotel.filling(), "it is filling while still freeing");
        assert!(
            !hotel.taken(1),
            "room 1 did not empty behind the first wave"
        );

        hotel.tick(DRAWN as f32 / TRAVELS_AT, DRAWN);
        assert!(hotel.filling(), "the second wave never set off");
    }

    /// Spec 0031: the blank door is passed rather than hunted.
    ///
    /// It stands where room nought would be, so it is the first door you
    /// come to walking in, and it slots into the alternating pattern: on the
    /// even side, 1.5 from room 2, which is the spacing every other same-side
    /// pair has.
    #[test]
    fn the_blank_door_is_passed() {
        let reaches = reaches();
        let (west, _) = span();
        let blank = blank_door(reaches);

        assert!(
            !on_the_left(BLANK),
            "the blank door is not on the even side"
        );
        assert!(blank.x < west, "the blank door is not in the corridor");

        // nothing else stands in its place
        for n in 1..=DRAWN {
            let room = at(reaches, n);

            if (room.z - blank.z).abs() > 0.1 {
                continue;
            }

            assert!(
                (room.x - blank.x).abs() > DOOR_WIDE,
                "room {} overlaps the blank door",
                n
            );
        }

        // and room 2 is exactly a same-side step past it
        let two = at(reaches, 2);

        assert!(
            ((blank.x - two.x).abs() - APART).abs() < 1e-3,
            "the blank door is {} from room 2 and a same-side step is {}",
            (blank.x - two.x).abs(),
            APART
        );

        // you pass it before any room
        assert!(
            blank.x > at(reaches, 1).x,
            "room 1 comes before the blank door"
        );
    }

    /// Spec 0030: there is no last door. Wherever you stand, there are rooms
    /// drawn past you.
    ///
    /// Jake found the first build's last door by walking to room 36. In a
    /// hotel whose premise is that there is always more of it than you can
    /// see, reaching the end is the one thing that must not happen.
    #[test]
    fn there_is_always_another_door() {
        let reaches = reaches();

        for n in [1u64, 2, 40, 41, 500, 10_000, 1_000_000] {
            let stood = at(reaches, n);
            let drawn = window(stood);

            assert!(
                drawn.contains(&n),
                "standing at room {} and it is not among the drawn",
                n
            );

            let past = drawn.clone().filter(|m| *m > n).count();

            assert!(
                past >= (DRAWN / 2) as usize - 1,
                "standing at room {} there are only {} doors past it",
                n,
                past
            );
        }
    }

    /// And the numbers are right wherever you stand, not only near the lobby.
    #[test]
    fn the_numbers_are_right_far_from_the_desk() {
        let reaches = reaches();

        for n in [1u64, 2, 39, 40, 41, 999, 100_000] {
            let stood = at(reaches, n);

            assert_eq!(
                nearest(stood),
                n,
                "room {} stands where the corridor calls room {}",
                n,
                nearest(stood)
            );
        }
    }

    /// Drives a body your size from one place to another with the game's own
    /// walk, and says where it ended up.
    ///
    /// The whole building's boxes, not this room's. A walk that only knows
    /// its own room walks through the wall it shares.
    fn walked(solid: &[Aabb], from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
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

            let (moved, fell) = walk::walk(at, wish, falling, crate::RADIUS, step, solid, true);

            at = moved;
            falling = fell;
        }

        at
    }

    // ---- the crowd's own tests, moved here with it by spec 0063 ----
    //
    // **They were `people`'s and they are the hotel's.** That module is
    // the figure and its gait, which every room with somebody in it reads;
    // these drive a band of guests behind a wave of occupancy, which is
    // this room and nothing else. Parked code that nothing tests is the
    // thing parking is supposed to avoid.

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
            let went = who.at.distance(at(reaches, who.from));

            assert!(
                (who.walked - went).abs() < 0.05,
                "room {}'s guest has walked {:.2} metres and is {:.2} from their door",
                who.from,
                who.walked,
                went
            );

            legs.push((who.from, went, who.walked / crate::people::STEP));
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

        let out_of = at(reaches, starting.from);

        assert!(
            (starting.at.x - out_of.x).abs() < 0.2 && (starting.at.z - out_of.z).abs() < 0.2,
            "room {}'s guest did not start at their own door",
            starting.from
        );

        // and the one furthest through is nearly at the next number's door,
        // which is across the corridor
        let into = at(reaches, ending.from + 1);
        let left = at(reaches, ending.from);

        assert!(
            (left.z - into.z).abs() > ACROSS * 0.5,
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

            let went = (two.at - at(reaches, two.from)).length();
            let next = (one.at - at(reaches, one.from)).length();

            assert!(
                went > next,
                "room {} walks {:.2} to a prime power and {:.2} to next door",
                one.from,
                went,
                next
            );

            // west, which is where the bigger numbers are
            assert!(
                two.at.x < at(reaches, two.from).x,
                "room {} is not heading west",
                two.from
            );

            further += 1;
        }

        assert!(further > 0, "nobody was sent further than next door");

        // and nobody streaks: a crossing is a walk however far away the room
        // they are going to is
        for who in far.iter() {
            let went = (who.at - at(reaches, who.from)).length();

            assert!(
                went <= WALKS * CROSSES_IN / TRAVELS_AT + 0.01,
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

    /// Spec 0033: nobody is solid.
    ///
    /// **A deliberate exception to spec 0013**, which says a collider that
    /// disagrees with what is drawn is this building's commonest fault. A
    /// corridor where the demonstration can pin you against a wall is worse
    /// than one you can walk through, and the people are the demonstration.
    #[test]
    fn nobody_is_solid() {
        let room = crate::room::Room::of(crate::games());
        let reaches = room.reaches;
        let mut hotel = Hotel::new();

        let drawn = window(at(reaches, 8));
        let leaves = |hotel: &Hotel| -> usize {
            room.solid().len() + drawn.clone().filter(|n| hotel.open_by(*n) >= 0.0).count()
        };
        let at_rest = leaves(&hotel);

        hotel.house();

        // and again partway through a wave, with a corridor full of people
        for _ in 0..120 {
            hotel.tick(1.0 / 60.0, DRAWN);
        }

        let out = crossing(reaches, hotel.travelling(), 1, 40, |n| {
            Arrival::One.sends(n)
        });

        assert!(
            !out.is_empty(),
            "nobody was crossing, so this checked nothing"
        );
        assert_eq!(
            leaves(&hotel),
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
}
