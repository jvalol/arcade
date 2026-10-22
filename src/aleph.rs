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
        // the lobby's north and south walls
        Aabb::new(
            vec3(west - THICK, 0.0, from - THICK),
            vec3(east, LOBBY_TALL, from),
        ),
        Aabb::new(
            vec3(west - THICK, 0.0, to),
            vec3(east, LOBBY_TALL, to + THICK),
        ),
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
        // the lobby's ceiling
        Aabb::new(
            vec3(west - THICK, LOBBY_TALL, from - THICK),
            vec3(east, LOBBY_TALL + THICK, to + THICK),
        ),
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
        }
    }

    /// The move that houses them: room n becomes room a*n + b.
    ///
    /// Shuffling along frees as many rooms as you shuffle, and no shuffle
    /// frees infinitely many. Doubling frees every odd room, which is
    /// endlessly many, and is the only answer to the last of these.
    pub fn move_for(&self) -> (u64, u64) {
        match self {
            Arrival::One => (1, 1),
            Arrival::Coach(n) => (1, *n),
            Arrival::Endless => (2, 0),
        }
    }
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

#[derive(Clone, Debug)]
pub struct Hotel {
    /// Whether each drawn room has somebody in it, room 1 first.
    taken: Vec<bool>,
    /// What the rule is moving everybody to, while the change is travelling,
    /// and nothing when it has arrived.
    going: Option<Vec<bool>>,
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
    then_filled: Option<Vec<bool>>,
    /// Who is at the desk.
    at_the_door: Option<Arrival>,
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
            taken: vec![true; DRAWN as usize],
            going: None,
            front: 0.0,
            doing: Doing::Freeing,
            then_filled: None,
            // somebody is always at the door, because a hotel with nobody
            // wanting a room has nothing to demonstrate and this is the
            // resting state rather than a thing you set up
            at_the_door: Some(Arrival::One),
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

        self.at_the_door = Some(match arrival {
            Arrival::One => Arrival::Coach(42),
            Arrival::Coach(_) => Arrival::Endless,
            Arrival::Endless => Arrival::One,
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
        let (stride, shift) = arrival.move_for();

        self.apply(stride, shift);

        // and who is in once the arrivals are in: whatever the move leaves,
        // plus the rooms they take
        let mut filled = self.going.clone().unwrap_or_default();
        let mut came = 0u64;

        for n in 1..=DRAWN {
            if filled[n as usize - 1] {
                continue;
            }

            filled[n as usize - 1] = true;
            came += 1;
        }

        self.then_filled = Some(filled);
        // **Counted when they are in, not when they are asked for.** This
        // said `housed += came` here, so the desk read "1 housed" the instant
        // you pressed, before either wave had moved a door. Jake pressed and
        // read the outcome of something he had not watched happen.
        self.arriving = came;
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
            return self.at(&self.taken, n);
        };

        if (n as f32) <= self.front {
            self.at(going, n)
        } else {
            self.at(&self.taken, n)
        }
    }

    fn at(&self, which: &[bool], n: u64) -> bool {
        if n == 0 || n > DRAWN {
            return false;
        }

        which[n as usize - 1]
    }

    /// Move everybody: room n becomes room a·n + b.
    ///
    /// Only the rooms that are drawn can be said anything about. A guest sent
    /// past the fortieth has gone somewhere this corridor does not show,
    /// which is the point of the hotel rather than a limitation of it.
    pub fn apply(&mut self, stride: u64, shift: u64) {
        let mut going = vec![false; DRAWN as usize];

        for n in 1..=DRAWN {
            if !self.at(&self.taken, n) {
                continue;
            }

            let to = n * stride + shift;

            if (1..=DRAWN).contains(&to) {
                going[to as usize - 1] = true;
            }
        }

        self.going = Some(going);
        self.front = 0.0;
    }

    /// Let the change travel.
    pub fn tick(&mut self, dt: f32) {
        if self.going.is_none() {
            return;
        }

        self.front += dt * TRAVELS_AT;

        if self.front < DRAWN as f32 {
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
pub fn sconces(reaches: f32) -> Vec<Vec3> {
    let mut out = Vec::new();

    for n in 1..DRAWN {
        // the gap between room n and the next on the same side, which is two
        // along
        if n + 2 > DRAWN {
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
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    corridor_lamps(reaches)
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
pub fn corridor_lamps(reaches: f32) -> Vec<Vec3> {
    sconces(reaches)
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

#[cfg(test)]
mod tests {
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
        let fittings: Vec<_> = sconces(reaches)
            .into_iter()
            .chain(lobby_sconces(reaches))
            .collect();
        let lamps = lamps(reaches);

        assert_eq!(
            lamps.len(),
            fittings.len(),
            "there are {} lamps and {} fittings",
            lamps.len(),
            fittings.len()
        );
        assert_eq!(
            lamps.len(),
            corridor_lamps(reaches).len() + lobby_lamps(reaches).len(),
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
            crate::people::crossing(
                crate::room::Room::of(crate::games()).reaches,
                hotel.travelling(),
                1,
                DRAWN,
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

        hotel.apply(2, 0);
        // let the whole wave through
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);

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

        hotel.apply(2, 0);

        // one room's worth of travel, and nothing beyond it has moved
        hotel.tick(1.0 / TRAVELS_AT);

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

        hotel.tick(60.0);

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
    #[test]
    fn the_light_says_what_the_door_says() {
        let mut hotel = Hotel::new();

        hotel.apply(2, 1);

        for _ in 0..400 {
            hotel.tick(0.05);

            if hotel.travelling().is_some() {
                continue;
            }

            for n in 1..=DRAWN {
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
            hotel.tick(0.02);

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
    #[test]
    fn walked_from_the_gallery_into_the_corridor() {
        let reaches = reaches();
        let room = Room::of(crate::games());
        let solid = room.solid();
        let middle = middle_z(reaches);

        // dropped in from above rather than placed, because a point on the
        // floor can be inside the furniture
        let from = vec3(-crate::room::GALLERY_WIDE * 0.5 + 1.0, 0.6, middle);
        let to = vec3(at(reaches, DRAWN).x, 0.0, middle);

        let got = walked(&solid, from, to, 30.0);

        assert!(
            got.x < at(reaches, 6).x,
            "walked from the gallery only as far as {} and room 6 is at {}",
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
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);
            hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);

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

        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);

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
        assert_eq!(Arrival::One.move_for(), (1, 1));
        assert_eq!(Arrival::Coach(42).move_for(), (1, 42));
    }

    /// And a coach with a room for every number is the one that cannot be
    /// shuffled and has to be doubled.
    #[test]
    fn an_endless_coach_is_housed() {
        assert_eq!(Arrival::Endless.move_for(), (2, 0));

        let mut hotel = Hotel::new();

        hotel.at_the_door = Some(Arrival::Endless);
        hotel.house();
        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);

        // the freeing wave is through: every odd room stands open
        for n in 1..=DRAWN {
            if on_the_left(n) {
                assert!(!hotel.taken(n), "room {} is odd and still taken", n);
            }
        }

        hotel.tick(DRAWN as f32 / TRAVELS_AT + 1.0);

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

        hotel.tick(2.0 / TRAVELS_AT);
        assert!(!hotel.filling(), "it is filling while still freeing");
        assert!(
            !hotel.taken(1),
            "room 1 did not empty behind the first wave"
        );

        hotel.tick(DRAWN as f32 / TRAVELS_AT);
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
}
