//! The back room, through the unmarked door in the gallery's east wall.
//! Spec 0024.
//!
//! Where the building keeps its things, and where its one instrument stands:
//! a workbench with a block of cubes held in a jig over it, and gauges on the
//! wall saying what drawing them costs. Spec 0023 fills the window those
//! gauges read from, every frame, whether anybody is in here or not.
//!
//! Nothing in here starts a process. The building spawns a binary in exactly
//! one place, which is a cabinet in the hall, and this room is not one.
//!
//! Where everything is, is arithmetic, and arithmetic can be checked without
//! a window.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::{gallery_wall, GALLERY_WIDE, THICK};

/// How big the room is inside: west to east, and the depth along `z`.
///
/// Four and a half by four and a half, which is small for a room in this
/// building and is the point: a maintenance room is the space that was left
/// over. It still has to hold a bench, somebody standing at it, and a walk
/// from the door that nothing is in the way of, which is what stops it being
/// smaller.
pub const WIDE: f32 = 4.6;
pub const DEEP: f32 = 4.4;

/// And how high, which is lower than anywhere else in here.
///
/// The hall and the gallery are 3.2. A lower ceiling is most of what says
/// this is the back of the building rather than part of it, and it is safe
/// to go lower against a wall that is not: the fault with a borrowed wall is
/// a room that stands *taller* than it, which leaves a strip open above.
/// This building has got that wrong twice, both times upward.
///
/// Two and eight, which was two and six until the gauge board grew a heading
/// and a scale for each bar and no longer fitted between the card's top and
/// the ceiling. The compile-time assertion under `GAUGE_UP` is what said so,
/// before anything was drawn.
pub const TALL: f32 = 2.8;

/// Where the room's inside begins, which is the far face of the gallery's
/// east wall, and where it ends.
pub fn span() -> (f32, f32) {
    let west = GALLERY_WIDE * 0.5 + THICK;

    (west, west + WIDE)
}

/// And the same along `z`, taken off the gallery's far wall so the two rooms
/// end together.
pub fn deep(reaches: f32) -> (f32, f32) {
    let far = gallery_wall(reaches);

    (far, far + DEEP)
}

/// The doorway in the gallery's east wall, as the two `z` it runs between.
///
/// **Clear of the shapes by more than a body is wide.** The row of plinths
/// ends at `display::row()` across and stands within a metre of the gallery's
/// far wall, and a door beside it would leave seventy centimetres to squeeze
/// through. A body is 0.9 and wants 1.05 clear, so the door is a long stride
/// further down the wall and you reach it across open floor.
pub fn way(reaches: f32) -> (f32, f32) {
    let (far, _) = deep(reaches);
    let from = far + 2.35;

    (from, from + 1.3)
}

/// The gallery's east wall, in two pieces with the doorway between them.
///
/// Spec 0022 built it as one box. It is this now, and the gallery's own code
/// asks here for it rather than keeping a second account of where its wall
/// is: two lists of one thing drift, which is the fault under most of the
/// others in this building.
pub fn gallery_east(reaches: f32) -> Vec<Aabb> {
    let far = gallery_wall(reaches);
    let (west, _) = span();
    let (from, to) = way(reaches);

    vec![
        Aabb::new(
            vec3(GALLERY_WIDE * 0.5, 0.0, far - THICK),
            vec3(west, crate::room::TALL, from),
        ),
        Aabb::new(
            vec3(GALLERY_WIDE * 0.5, 0.0, to),
            vec3(west, crate::room::TALL, -reaches),
        ),
    ]
}

/// The room's floor, which laps under the gallery's rather than meeting it.
///
/// **Two floors at one height that meet edge to edge stop you dead**, and
/// `walk` reports it as nothing at all. This building found that three times
/// in one day. What works is burying both end faces inside the other slab, so
/// this one runs west past the gallery's east edge and under its wall, and
/// the gallery's east face is inside it.
pub fn floor(reaches: f32) -> Aabb {
    let (_, east) = span();
    let (far, near) = deep(reaches);

    Aabb::new(
        vec3(GALLERY_WIDE * 0.5 - THICK, -THICK, far - THICK),
        vec3(east + THICK, 0.0, near + THICK),
    )
}

/// Everything solid in here: three walls, the floor, the ceiling and the
/// bench.
///
/// The fourth wall is the gallery's, which stands already and is cut for the
/// door by [`gallery_east`]. The rig is not here: it is a bench in the
/// building's own list, which is what gives it a collider, a name under the
/// sight and the arrows, the same as the pool table down in the cellar. The
/// workbench is in here because it stopped being that when the rig came off
/// it: it is a thing in the way now, and nothing else.
/// The lid over this room, which is low, and is what it shows the sky. Spec
/// 0040 lays a roof on it, and two accounts of where it is would drift.
pub fn lid(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (far, near) = deep(reaches);

    Aabb::new(
        vec3(west, TALL, far - THICK),
        vec3(east + THICK, TALL + THICK, near + THICK),
    )
}

pub fn solid(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (far, near) = deep(reaches);

    vec![
        floor(reaches),
        bench_box(reaches),
        // the east wall, the whole span
        Aabb::new(
            vec3(east, 0.0, far - THICK),
            vec3(east + THICK, TALL, near + THICK),
        ),
        // the far wall, which the bench stands against. It runs west into
        // the gallery's own far wall rather than stopping against it: two
        // walls that meet exactly share a plane, and two drawn faces in one
        // plane is the fault this building has hit seven times.
        Aabb::new(
            vec3(GALLERY_WIDE * 0.5, 0.0, far - THICK),
            vec3(east + THICK, TALL, far),
        ),
        // and the near one
        Aabb::new(
            vec3(west, 0.0, near),
            vec3(east + THICK, TALL, near + THICK),
        ),
        lid(reaches),
    ]
    .into_iter()
    .chain(clutter_solid(reaches))
    .collect()
}

/// The dark end of the room, which is what the rig stands in. Spec 0025.
///
/// **Because the example this room is copying is a black window with one
/// bright thing in it.** The engine's own `load` is white cubes filling the
/// frame with nothing behind them, and Jake's word for it is stunning. The
/// same cubes in here were 62 centimetres on a bench, seen from two metres,
/// against a card the size of a tea tray, with walls as bright as they were
/// either side: the example is more stunning than the closet. Colour was
/// never the difference. Composition was.
///
/// So the far 1.6 metres of the room is lined black, floor to ceiling and
/// wall to wall, and the block stands inside it. 1.6 because the block is
/// 1.2 deep as well as wide and has to stand inside the lining with room
/// behind it and in front: anything shallower and the cubes hang out into
/// the grey.
pub const BAY: f32 = 1.6;

/// How thick the lining is drawn.
///
/// A skin over the room's own surfaces rather than a wall of its own, and
/// buried a little into each of them: a panel whose back face lands exactly
/// on the face it covers is two drawn faces in one plane, which is the fault
/// this building has hit seven times.
pub const LINING: f32 = 0.04;
pub const LINING_IN: f32 = 0.02;

/// The dark end as a volume: everything from the far wall to the lip.
pub fn bay(reaches: f32) -> Aabb {
    let (west, east) = span();
    let (far, _) = deep(reaches);

    Aabb::new(vec3(west, 0.0, far), vec3(east, TALL, far + BAY))
}

/// The lining itself: the back wall, the two returns and the ceiling of the
/// bay.
///
/// The floor is not here. A panel laid on the floor is a step at the lip,
/// and a face at the walking surface is the fault that stops you dead and
/// that `walk` reports as nothing at all. The bay's floor is the room's own
/// floor drawn in the lining's colour instead, which is paint rather than
/// geometry.
pub fn lining(reaches: f32) -> Vec<Aabb> {
    let (west, east) = span();
    let (far, _) = deep(reaches);
    let lip = far + BAY;

    vec![
        // the back wall
        Aabb::new(
            vec3(west, 0.0, far - LINING_IN),
            vec3(east, TALL, far + LINING),
        ),
        // the two returns
        Aabb::new(
            vec3(west - LINING_IN, 0.0, far - LINING_IN),
            vec3(west + LINING, TALL, lip),
        ),
        Aabb::new(
            vec3(east - LINING, 0.0, far - LINING_IN),
            vec3(east + LINING_IN, TALL, lip),
        ),
        // and the ceiling of it
        Aabb::new(
            vec3(west, TALL - LINING, far - LINING_IN),
            vec3(east, TALL + LINING_IN, lip),
        ),
    ]
}

/// What the bench is called under the sight, and how big it is.
///
/// A bench rather than a thing of its own, because the building already has
/// benches: you walk up to one, the corner names it, and the arrows work what
/// is on it. The pool table in the cellar is a bench by the same argument and
/// is not in the nook either.
pub const RIG: &str = "test rig";
pub const BENCH: Vec3 = vec3(0.72, 0.86, 2.2);

/// Where the bench stands: along the east wall, out of the bay.
///
/// It was across the far wall with the block clamped over it, which is where
/// the far wall is now lined black and the rig stands on the floor. Spec
/// 0025: the bench is furniture after that. You do not work the dial by
/// standing at it, you work it by looking at the cubes, which is what the
/// sight already does.
pub fn bench(reaches: f32) -> Vec3 {
    let (_, east) = span();
    let (far, _) = deep(reaches);

    vec3(
        east - BENCH.x * 0.5 - 0.05,
        BENCH.y * 0.5,
        far + BAY + 0.25 + BENCH.z * 0.5,
    )
}

pub fn bench_box(reaches: f32) -> Aabb {
    Aabb::from_center_size(bench(reaches), BENCH)
}

/// The rig, as one of the building's benches.
///
/// **The record is the block now, not the table.** A bench in this building
/// is a thing you walk up to, point at and press a key at, and what you
/// point at in here is the cubes: Jake, getting nothing for pointing at
/// them, if i'm pointing at the thing demonstrated it should be
/// interactable. That was a second box bolted onto the sight's chain while
/// the block sat over a bench. The block stands on the floor now, so the
/// bench the building knows about is the column the block is the top of, and
/// there is one box again rather than two.
pub fn benched(reaches: f32) -> crate::room::Benched {
    let at = block(reaches);

    crate::room::Benched {
        name: RIG,
        at: vec3(at.x, 0.0, at.z),
        size: vec3(BLOCK + 0.3, BLOCK_UP + BLOCK * 0.5, BLOCK + 0.3),
        // out into the room, which from the dark end is toward the door
        worked_from: Vec3::Z,
    }
}

/// Where the lamp hangs, and what its shade is.
///
/// One fitting, over the near half of the floor. It was over the middle of
/// the room, which is now partly the bay: a bulb that reaches the lining
/// makes it grey, and a grey lining is a black rectangle painted on a wall
/// rather than a space. Spec 0025. It still has to show the door and the
/// clutter, which is why it moved rather than went out.
pub const SHADE: Vec3 = vec3(0.30, 0.10, 0.30);
pub const LAMP_UP: f32 = TALL - 0.34;

pub fn lamp(reaches: f32) -> Vec3 {
    let (west, east) = span();
    let (_, near) = deep(reaches);

    vec3((west + east) * 0.5, LAMP_UP, near - 1.2)
}

/// And the bay's own lamp, which is the one that lights the cubes.
///
/// At the lip and above it, pointing at nothing because the engine's point
/// lamps point at nothing: what keeps it off the room is its range, which
/// dies a stride past the lip. The block is the brightest thing in here by
/// several times, which is the third of the three things `load` gets for
/// free.
pub const BAY_SHADE: Vec3 = vec3(0.36, 0.10, 0.24);

pub fn bay_lamp(reaches: f32) -> Vec3 {
    let (west, east) = span();
    let (far, _) = deep(reaches);

    vec3((west + east) * 0.5, TALL - 0.22, far + BAY - 0.18)
}

/// How many cubes the rig holds, as the steps of its dial.
///
/// **Every one of them a cube number, so the block is a block.** They were
/// round numbers first, 256 and 1024 and 16384, and a count that is not a
/// cube cannot fill a cube: 256 in a seven by seven by seven grid is five
/// full layers and a sixth with a third of it missing, and the missing third
/// is at the top because that is the order it fills in. Jake, looking at it:
/// why aren't the cubes organised as a cube, there are more layers at the
/// bottom than higher up.
///
/// So four, eight, twelve, sixteen and twenty four a side. Far enough apart
/// that the curve has a shape, and each one a solid block with nothing
/// missing off a corner.
pub const CUBES: [u32; 6] = [0, 64, 512, 1728, 4096, 13824];

/// How wide the block of them is, and how high it floats over the bench.
///
/// **Narrower than the bench is deep, which is what decides it.** It was 1.5
/// across, which looked right from the front and was two thirds of a metre
/// deeper than the bench: the back of the block stood inside the far wall
/// and the bottom of it inside the bench top. A thing under test that
/// reaches through the bench it is on is a wall of cubes through the room,
/// which is the one thing the spec asks the jig to prevent.
/// **As wide as your arms, and standing on the floor.** It was 0.62 over a
/// bench, which is a model of a thing rather than the thing: from where you
/// work it took a fifth of the view with a lit grey room round it. 1.2
/// across, with its middle at 1.2 off the floor, is about forty degrees from
/// the working spot, which is roughly what the block fills in the example's
/// window. Spec 0025.
pub const BLOCK: f32 = 1.2;
pub const BLOCK_UP: f32 = 1.2;

/// The top of it, which is what the gauges hang above.
pub const BLOCK_TOP: f32 = BLOCK_UP + BLOCK * 0.5;

/// Where the middle of the block is: in the bay, with the lining behind it
/// and a stride of it in front.
pub fn block(reaches: f32) -> Vec3 {
    let (west, east) = span();
    let (far, _) = deep(reaches);

    vec3(
        (west + east) * 0.5,
        BLOCK_UP,
        far + LINING + 0.21 + BLOCK * 0.5,
    )
}

/// The frame it stands in: how thick a post is, and how far off the block
/// the two of them stand.
pub const POST: f32 = 0.06;
pub const POST_OUT: f32 = BLOCK * 0.5 + 0.1;

/// How big one cube is drawn, for a given count.
///
/// The block is a fixed size and the cubes divide it, so turning the dial up
/// fills the same space with more, smaller things rather than growing a
/// block that eats the room. That is also the honest way to show what the
/// count costs: the pixels stay about the same and the draws do not.
pub fn cube_across(count: u32) -> f32 {
    BLOCK / blitzkit::grid::side(count) as f32 * 0.62
}

/// Where one of them sits, by its number, in the block's own frame.
///
/// The arithmetic is the engine's, because the engine's own `load` example
/// lays out the same block of cubes and the two of them had the same fault
/// in the same week. See `blitzkit::grid`.
pub fn cube_at(count: u32, n: u32) -> Vec3 {
    blitzkit::grid::at(count, n, BLOCK)
}

/// The board the gauges are on: where it hangs, how big it is, and how its
/// three bars are laid out on it.
///
/// **Bars rather than numbers.** Writing a number on a wall in this engine
/// means drawing it into a texture, per spec 0037, and these change every
/// frame: a texture a frame is a program that grows until it stops, which
/// this building has already done once. A bar is a box with a width, costs
/// nothing to move, and answers the question a gauge is for, which is
/// whether the needle is near the end rather than what it says. The numbers
/// themselves are under the sight and behind F.
/// Over the block rather than at a height of its own, so the two cannot
/// overlap however either of them is resized. The board went from one row of
/// three words to a heading and three rows and immediately wanted the
/// ceiling, which is the kind of thing a written down number lets happen
/// twice.
pub const GAUGE_UP: f32 = BLOCK_TOP + 0.12 + GAUGE.y * 0.5;

// and that it clears the block and the ceiling, checked here rather than in
// a test because it is arithmetic over constants: a build that got it wrong
// would not compile, which is a better place to find out than a picture.
const _: () = assert!(GAUGE_UP - GAUGE.y * 0.5 > BLOCK_TOP);
const _: () = assert!(GAUGE_UP + GAUGE.y * 0.5 < TALL);
/// How big the board is.
///
/// **Sized by what is written on it, not by taste.** Every word on here is
/// drawn at a fixed height in a font whose letters are square cells, so a
/// row is a name, a track and a reading, and the width of the two words is
/// arithmetic rather than a guess. At a width of one and a half the longest
/// reading hung off the right edge, and the test below is what said so.
pub const GAUGE: Vec3 = vec3(2.0, 0.78, 0.05);
pub const GAUGE_TRACK: f32 = 0.9;
pub const GAUGE_BAR: f32 = 0.1;
pub const GAUGE_STEP: f32 = 0.17;

/// Where the middle of the track sits across the board, which leaves the
/// name on its left and what the bar is full at on its right.
///
/// The middle, because the two words either side of it are about the same
/// length as each other.
pub const GAUGE_TRACK_AT: f32 = 0.0;

/// What the board says: a heading, a name for each bar, and what each bar is
/// full at.
///
/// **Because three coloured bars say nothing on their own, and three words
/// barely more.** Jake asked what the bars meant, so they were labelled
/// work, worst and draws, and his reading of that was that they still do not
/// convey meaning. He is right, and the reason is that a name without a
/// scale is half a label: a green bar half full of nothing in particular.
///
/// So the board says what it is across the top, each bar says what it
/// measures on its left, and the right of each track says the reading that
/// fills it. Time is in milliseconds because that is what a frame is
/// measured in and the heading says it is a frame.
///
/// All of it is written into textures at startup, which the readings
/// themselves could not be: a word that never changes is one texture for the
/// life of the program, and a number that changes every frame is a texture a
/// frame. Spec 0037 for the writing, and spec 0056 for why the readings
/// stayed bars.
pub const GAUGE_SAYS: &str = "what a frame costs";
pub const GAUGE_WORDS: [&str; 3] = ["time", "worst", "draws"];
pub const GAUGE_FULL_SAYS: [&str; 3] = ["8 ms", "16 ms", "4000"];
pub const GAUGE_TEXELS: f32 = 128.0;
pub const GAUGE_WORD_TALL: f32 = 0.075;

/// Where one of those words sits, which is the board's own left edge.
///
/// Off the edge rather than off the track: a word set where the track begins
/// is a word that ends where the track begins, and the first build of this
/// had the bar drawn over the last letter of each of them.
pub const GAUGE_WORD_IN: f32 = 0.05;

pub fn gauge_word(n: usize) -> Vec3 {
    let (at, _) = gauge_bar(n, 0.0);

    vec3(-GAUGE.x * 0.5 + GAUGE_WORD_IN, at.y, at.z)
}

/// Where what a bar is full at is written, which is past the end of its own
/// track.
pub fn gauge_full_at(n: usize) -> Vec3 {
    let (at, _) = gauge_bar(n, 0.0);

    vec3(
        GAUGE_TRACK_AT + GAUGE_TRACK * 0.5 + GAUGE_WORD_IN,
        at.y,
        at.z,
    )
}

/// And where the heading goes, which is across the top of the board.
pub fn gauge_says() -> Vec3 {
    vec3(
        -GAUGE.x * 0.5 + GAUGE_WORD_IN,
        GAUGE.y * 0.5 - GAUGE_WORD_TALL * 0.8,
        GAUGE.z * 0.6,
    )
}

/// What each bar is full at: milliseconds of work, milliseconds at the worst
/// of the window, and draw calls.
///
/// A bar pinned at the end means off the scale, which is a true thing for a
/// gauge to say. Eight milliseconds because that is about a frame on the
/// machine this was built on, and a room that costs a whole frame to draw is
/// the answer whatever the exact number is.
pub const GAUGE_FULL: [f32; 3] = [8.0, 16.0, 4000.0];

/// Where one of the three bars sits on the board, as an offset from its
/// middle, and how long it is for a reading.
///
/// The track runs from the right edge inward, so the left of the board is
/// free for the word that names it.
pub fn gauge_bar(n: usize, of: f32) -> (Vec3, f32) {
    let full = GAUGE_FULL[n.min(2)];
    let along = (of / full).clamp(0.0, 1.0);
    // the three rows sit in the space under the heading rather than hard up
    // against it: measured so the gap over the first bar and the gap under
    // the last are the same, because a board with its rows at the top and a
    // hand's width of nothing underneath reads as unfinished.
    let down = GAUGE.y * 0.5 - GAUGE_WORD_TALL - 0.20;

    (
        // in front of the board, which is toward the room. Behind it, which
        // is what the first build drew, is inside the wall.
        vec3(GAUGE_TRACK_AT, down - n as f32 * GAUGE_STEP, GAUGE.z * 0.6),
        (along * GAUGE_TRACK).max(0.012),
    )
}

pub fn gauge(reaches: f32) -> Vec3 {
    let (far, _) = deep(reaches);

    vec3(block(reaches).x, GAUGE_UP, far + LINING + GAUGE.z * 0.5)
}

/// Where somebody stands to work the rig.
///
/// Worked out from the rig's own `worked_from` rather than written down a
/// second time, which is the fault under most of the others in this
/// building. The staged picture of the room stands here too, so it is not
/// behind `cfg(test)` any more: a pose that guesses where the working spot
/// is, is a second account of it.
pub fn standing(reaches: f32) -> Vec3 {
    let rig = benched(reaches);

    rig.at + rig.worked_from * (rig.size.z * 0.5 + crate::RADIUS + 0.35)
}

/// The meshes the clutter is made of, each off the same wheel the cellar's
/// barrels come off.
///
/// A bucket is open at the top, so it is two sided or you see through its far
/// wall. A tin has its lid, which `lidded` closes for it. A jar is a tin that
/// has not been emptied. A reel is a drum with a cheek at each end, and the
/// cable on it is a belly between them rather than a coil modelled turn by
/// turn.
pub fn bucket_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(24, 10, |t| {
        // a touch narrower at the foot, which is what a bucket is
        (t, 0.78 + t * 0.22)
    })
    .two_sided()
}

pub fn tin_mesh() -> blitzkit::mesh::MeshData {
    // **The first number is where along the axis, not how far up from
    // nothing.** This was `(0.0, 1.0)`, which is a straight side at a height
    // that never changes, so every ring of it landed in the same plane and
    // what stood on the bench was a flat splat. Jake, looking at the three
    // jars: there's weird circles on the desk.
    crate::cellar::turned(24, 10, crate::cellar::lidded(|t| (t, 1.0)))
}

pub fn reel_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(
        24,
        18,
        crate::cellar::lidded(|t| {
            // a cheek at each end and the cable between them
            let cheek = 0.14;
            let waist = if t < cheek || t > 1.0 - cheek {
                1.0
            } else {
                0.76
            };

            (t, waist)
        }),
    )
}

/// What is standing about in here, and where.
///
/// **Not boxes.** Jake's standing note on this building is that a box is what
/// a thing looks like before it is built, and he has said so about a cabinet,
/// a plinth, a cube on a plinth and a block in a room. Clutter made of grey
/// boxes would be nine of them in a row. Everything round here comes off the
/// same wheel the cellar's barrels do, and the crates have boards and a lid
/// rather than a label saying crate.
///
/// **And none of it is where anybody walks.** A body is 0.9 across and wants
/// about 1.05 clear, so this is all against a wall or in a corner, and the
/// line from the door to the bench has nothing on it. That is a test rather
/// than an intention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thing {
    /// A bucket, open at the top.
    Bucket,
    /// A tin of paint with the lid half on.
    Tin,
    /// A jar of fixings.
    Jar,
    /// A crate, boarded.
    Crate,
    /// A drum of cable on its side.
    Reel,
}

impl Thing {
    /// How big one is: across, and tall.
    pub fn size(self) -> (f32, f32) {
        match self {
            Thing::Bucket => (0.30, 0.34),
            Thing::Tin => (0.24, 0.26),
            Thing::Jar => (0.12, 0.16),
            Thing::Crate => (0.52, 0.42),
            Thing::Reel => (0.46, 0.30),
        }
    }
}

/// One piece of clutter: what it is, where it stands and how it is turned.
pub struct Stood {
    pub thing: Thing,
    pub at: Vec3,
    pub turn: f32,
}

/// Everything standing about in the room.
///
/// Placed by hand rather than scattered by a rule. A rule that drops things
/// at random puts one of them in the doorway on the run nobody tests, and
/// what this is for is the look of a room somebody has been using, which is
/// not the same as the look of a room somebody has shaken.
pub fn clutter(reaches: f32) -> Vec<Stood> {
    let (west, east) = span();
    let (far, near) = deep(reaches);
    let lip = far + BAY;
    let floor = 0.0;

    vec![
        // the strip of west wall between the bay and the door, which is the
        // only piece of wall in here that is neither lined nor benched
        Stood {
            thing: Thing::Crate,
            at: vec3(west + 0.36, floor, lip + 0.32),
            turn: 0.12,
        },
        Stood {
            thing: Thing::Crate,
            at: vec3(west + 0.38, 0.42, lip + 0.36),
            turn: -0.26,
        },
        // the near wall, which is the long one nothing else wants
        Stood {
            thing: Thing::Reel,
            at: vec3(west + 1.56, floor, near - 0.3),
            turn: 0.2,
        },
        Stood {
            thing: Thing::Bucket,
            at: vec3(west + 2.0, floor, near - 0.26),
            turn: 0.0,
        },
        Stood {
            thing: Thing::Bucket,
            at: vec3(west + 2.36, floor, near - 0.34),
            turn: 0.6,
        },
        Stood {
            thing: Thing::Tin,
            at: vec3(west + 3.7, floor, near - 0.28),
            turn: 0.3,
        },
        Stood {
            thing: Thing::Tin,
            at: vec3(west + 3.98, floor, near - 0.32),
            turn: -0.5,
        },
        Stood {
            thing: Thing::Tin,
            at: vec3(west + 3.72, 0.26, near - 0.3),
            turn: 0.9,
        },
        // and on the bench, which is furniture now and carries what a bench
        // carries
        Stood {
            thing: Thing::Jar,
            at: vec3(
                bench(reaches).x - 0.1,
                BENCH.y,
                bench(reaches).z - BENCH.z * 0.5 + 0.2,
            ),
            turn: 0.0,
        },
        Stood {
            thing: Thing::Jar,
            at: vec3(
                bench(reaches).x + 0.08,
                BENCH.y,
                bench(reaches).z - BENCH.z * 0.5 + 0.34,
            ),
            turn: 0.4,
        },
        Stood {
            thing: Thing::Jar,
            at: vec3(
                bench(reaches).x - 0.16,
                BENCH.y,
                bench(reaches).z - BENCH.z * 0.5 + 0.44,
            ),
            turn: 0.8,
        },
        Stood {
            thing: Thing::Tin,
            at: vec3(
                bench(reaches).x,
                BENCH.y,
                bench(reaches).z + BENCH.z * 0.5 - 0.3,
            ),
            turn: 0.2,
        },
        Stood {
            thing: Thing::Bucket,
            at: vec3(east - 0.3, floor, near - 0.34),
            turn: 0.0,
        },
    ]
}

/// What the clutter stops you walking through.
///
/// **Solid, like everything else in this building that is drawn.** A bucket
/// you walk through is a ghost, and spec 0046 exists because a collider that
/// disagrees with what is drawn is a fault worth looking for. It also makes
/// the room read: you go round the crates rather than through them, which is
/// what a room somebody is using feels like.
///
/// The jars on the bench are not in here. They stand on a bench that is
/// already solid, they are a hand's width across, and a collider you can
/// only meet by climbing on the furniture is a collider that catches nobody.
pub fn clutter_solid(reaches: f32) -> Vec<Aabb> {
    let mut out: Vec<Aabb> = clutter(reaches)
        .into_iter()
        .filter(|stood| stood.thing != Thing::Jar)
        .map(|stood| {
            let (across, tall) = stood.thing.size();

            Aabb::from_center_size(stood.at + Vec3::Y * tall * 0.5, vec3(across, tall, across))
        })
        .collect();

    // the ladder leans, so its box is the footprint of the lean rather than
    // the rails
    let foot = ladder(reaches);

    out.push(Aabb::from_center_size(
        foot + vec3(-LADDER_TALL * LADDER_LEAN * 0.5, LADDER_TALL * 0.5, 0.0),
        vec3(
            0.18 + LADDER_TALL * LADDER_LEAN,
            LADDER_TALL,
            LADDER_WIDE + 0.1,
        ),
    ));

    // and the broom, which is thin enough to be a post
    out.push(Aabb::from_center_size(
        broom(reaches) + Vec3::Y * 0.7,
        vec3(0.22, 1.4, 0.34),
    ));

    out
}

/// The stepladder, which leans against the east wall rather than standing.
///
/// Its own thing because it is the one piece of clutter that is not a solid
/// of revolution or a box: two rails and four steps, leaning.
pub const LADDER_TALL: f32 = 1.9;
pub const LADDER_WIDE: f32 = 0.42;
pub const LADDER_LEAN: f32 = 0.22;

pub fn ladder(reaches: f32) -> Vec3 {
    let (west, _) = span();
    let (_, near) = deep(reaches);

    vec3(west + 1.02, 0.0, near - 0.32)
}

/// And the broom, which leans in the corner by the door.
pub fn broom(reaches: f32) -> Vec3 {
    let (west, _) = span();
    let (_, near) = deep(reaches);

    vec3(west + 0.26, 0.0, near - 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0024: the room stands east of the gallery and nothing of the
    /// building is already there.
    ///
    /// The one question that had to be answered before any of this was
    /// placed, and it was answered by asking rather than by looking at a
    /// plan: the garden is west and near the entrance, the cellar is below,
    /// spec 0011's space is behind the near end wall, and east of the
    /// gallery is unbuilt ground.
    ///
    /// The room's inside rather than its shell. Its walls are meant to run
    /// into the gallery's, which is how two walls meet without sharing a
    /// plane, so a test on the shell asks the wrong question and fails for
    /// the right construction.
    #[test]
    fn the_back_room_stands_where_nothing_else_does() {
        let reaches = reaches();
        let room = crate::room::Room::of(crate::games());
        let (west, east) = span();
        let (far, near) = deep(reaches);
        // a hair inside the walls, because two boxes that touch count as
        // meeting and every wall of this room is meant to touch one of the
        // gallery's
        let skin = 1e-3;
        let inside = Aabb::new(
            vec3(west + skin, skin, far + skin),
            vec3(east - skin, TALL - skin, near - skin),
        );

        for box_ in room.building() {
            // this room's own things are allowed in it, which is its shell
            // and the rig the building keeps in its own list
            if super::solid(reaches)
                .iter()
                .chain(std::iter::once(&crate::room::Room::bench_box(&benched(
                    reaches,
                ))))
                .any(|mine| (mine.min - box_.min).length() < 1e-4)
            {
                continue;
            }

            assert!(
                !box_.intersects(&inside),
                "{:?} to {:?} stands in the back room",
                box_.min,
                box_.max
            );
        }
    }

    /// Spec 0024: it is closed, and the doorway is the only way out.
    #[test]
    fn the_only_way_out_is_the_door() {
        let reaches = reaches();
        let (west, east) = span();
        let (far, near) = deep(reaches);
        let (from, to) = way(reaches);
        let solid = solid(reaches);
        let walls = gallery_east(reaches);

        // the four sides, sampled along their length at head height, are
        // solid everywhere except across the doorway
        let hits = |at: Vec3| {
            solid
                .iter()
                .chain(walls.iter())
                .any(|box_| box_.contains_point(at))
        };

        for n in 0..=40 {
            let along = n as f32 / 40.0;
            let z = far + (near - far) * along;
            let x = west + (east - west) * along;
            let head = 1.6;

            // the west side: solid unless this is the doorway
            let in_the_way = z > from && z < to;
            let shut = hits(vec3(west - THICK * 0.5, head, z));

            assert_eq!(
                shut,
                !in_the_way,
                "the west wall at z {:.2} is {}",
                z,
                if shut { "shut" } else { "open" }
            );

            assert!(
                hits(vec3(east + THICK * 0.5, head, z)),
                "the east wall at z {:.2}",
                z
            );
            assert!(
                hits(vec3(x, head, far - THICK * 0.5)),
                "the far wall at x {:.2}",
                x
            );
            assert!(
                hits(vec3(x, head, near + THICK * 0.5)),
                "the near wall at x {:.2}",
                x
            );
        }
    }

    /// Spec 0024: the threshold has floor under all of it and no step in it.
    ///
    /// It was written as a rule about the lap, because a face at the walking
    /// surface used to stop you dead. Spec 0007's step-over takes a face of
    /// no height in its stride and that fault no longer reproduces, which
    /// `behind::tests::the_floors_leave_no_hole_and_no_ledge` sets out. What
    /// the overlap is still worth is that it is an overlap rather than a
    /// hairline touch, so no arithmetic anywhere near this join can open a
    /// gap in the doorway.
    #[test]
    fn the_floors_leave_no_hole_and_no_ledge() {
        let reaches = reaches();
        let room = crate::room::Room::of(crate::games());
        let (at, size) = crate::room::gallery(reaches);
        let gallery =
            Aabb::from_center_size(vec3(at.x, -THICK * 0.5, at.z), vec3(size.x, THICK, size.z));
        let mine = floor(reaches);

        assert!(
            mine.intersects(&gallery),
            "the two floors do not overlap at all, which is a hole at the door"
        );
        assert!(
            mine.min.x < gallery.max.x - 0.1,
            "this floor's west edge only reaches the gallery's, so the two \
             meet on a line rather than overlapping"
        );
        assert!(
            gallery.max.x < mine.max.x - 0.1,
            "the gallery's east edge only reaches this floor's, so the two \
             meet on a line rather than overlapping"
        );
        assert!(
            (mine.max.y - gallery.max.y).abs() < 1e-5,
            "the two floors are at different heights, which is a ledge in a \
             doorway"
        );

        // and the doorway has floor under every part of it
        let (from, to) = way(reaches);

        for n in 0..=10 {
            let z = from + (to - from) * n as f32 / 10.0;
            let under = vec3(GALLERY_WIDE * 0.5 + THICK * 0.5, -THICK * 0.5, z);

            assert!(
                mine.contains_point(under) || gallery.contains_point(under),
                "the threshold at z {:.2} has nothing under it",
                z
            );
        }

        let _ = room;
    }

    /// Spec 0024: you can get from the gallery to the bench and back.
    ///
    /// Walked with the game's own `walk` rather than measured, because a
    /// gap that is wide enough on paper can still be a room you cannot get
    /// into: this building has had four legal rises that were one solid
    /// block. Both rooms' colliders are in the walk, or it walks through the
    /// wall it is supposed to come through.
    #[test]
    fn you_can_walk_in_and_out_again() {
        let reaches = reaches();
        let room = crate::room::Room::of(crate::games());
        let mut solid = room.building();

        solid.extend(super::solid(reaches));
        solid.extend(gallery_east(reaches));

        let (from, to) = way(reaches);
        let door = vec3(GALLERY_WIDE * 0.5 + THICK * 0.5, 0.0, (from + to) * 0.5);
        let inside = standing(reaches);
        let gallery_side = vec3(door.x - 1.4, 0.0, door.z);

        let walked = |start: Vec3, target: Vec3| {
            let mut at = start + Vec3::Y * 0.6;
            let mut falling = 0.0;

            for _ in 0..600 {
                let flat = vec3(target.x - at.x, 0.0, target.z - at.z);
                let wish = if flat.length() > 0.05 {
                    flat.normalize() * 3.0
                } else {
                    Vec3::ZERO
                };
                let (moved, fell) =
                    crate::walk::walk(at, wish, falling, crate::RADIUS, 1.0 / 60.0, &solid, true);

                at = moved;
                falling = fell;
            }

            at
        };

        // by way of the door, because `walk` has no idea where it is going:
        // it slides along whatever it meets, and a straight line from the
        // bench to the gallery crosses the wall a stride south of the
        // doorway. A walk test that fails for want of a waypoint is a test
        // about pathfinding, which this building does not have and does not
        // need.
        let there = walked(walked(gallery_side, door), inside);
        assert!(
            (there - inside).length() < 0.6,
            "walking in from the gallery ended at {:?} rather than {:?}",
            there,
            inside
        );

        let back = walked(walked(inside, door), gallery_side);
        assert!(
            (back - gallery_side).length() < 0.6,
            "walking out ended at {:?} rather than {:?}",
            back,
            gallery_side
        );
    }

    /// Spec 0025: the block stands inside the bay, clear on all six sides,
    /// at every step of the dial.
    ///
    /// The lining is what the cubes are seen against, so a cube outside it
    /// is a cube against the grey room. Written over the test that held the
    /// block over its bench, which is where it was until the bench stopped
    /// holding it.
    #[test]
    fn the_block_stands_clear_inside_the_bay() {
        let reaches = reaches();
        let bay = bay(reaches);
        let block = block(reaches);
        let clear = 0.12;

        for count in CUBES {
            let across = cube_across(count);

            for n in 0..count {
                let at = block + cube_at(count, n);
                let low = at - Vec3::splat(across * 0.5);
                let high = at + Vec3::splat(across * 0.5);

                assert!(
                    low.x > bay.min.x + clear && high.x < bay.max.x - clear,
                    "cube {} of {} is at the side of the bay",
                    n,
                    count
                );
                assert!(
                    low.z > bay.min.z + clear && high.z < bay.max.z - clear,
                    "cube {} of {} is through the back of the bay or out past the lip",
                    n,
                    count
                );
                assert!(
                    low.y > clear && high.y < bay.max.y - clear,
                    "cube {} of {} is in the floor or the ceiling",
                    n,
                    count
                );
            }
        }

        // and the gauges are above the whole of it rather than behind it
        assert!(
            gauge(reaches).y > block.y + BLOCK * 0.5,
            "the gauges are behind the block"
        );
    }

    /// Spec 0025: nothing grey is behind the cubes, from anywhere in the
    /// room you can stand and look at them.
    ///
    /// The one that matters, and the one that is arithmetic rather than
    /// taste: a ray from the eye through a corner of the block has to land
    /// on the lining. Where it misses, what is behind that corner is the lit
    /// room, and a white cube against a grey wall is the thing this whole
    /// spec exists to stop.
    ///
    /// Swept over the floor rather than taken from the working spot alone. A
    /// ray from straight in front cannot miss a bay that is as wide as the
    /// room, so a test from there is a test that passes whatever the lining
    /// is: that was the first version of this, and moving the block most of
    /// a metre out past the lip did not fail it, because the working spot is
    /// worked out from the block and moved with it.
    #[test]
    fn nothing_grey_is_behind_the_cubes() {
        let reaches = reaches();
        let bay = bay(reaches);
        let block = block(reaches);
        let half = BLOCK * 0.5;
        let (west, east) = span();
        let (_, near) = deep(reaches);
        let mut stood = 0;

        for across in 0..=8 {
            for along in 0..=8 {
                let at = vec3(
                    west + crate::RADIUS
                        + (east - west - crate::RADIUS * 2.0) * across as f32 / 8.0,
                    1.55,
                    bay.max.z
                        + crate::RADIUS
                        + (near - bay.max.z - crate::RADIUS * 2.0) * along as f32 / 8.0,
                );

                // only from where somebody could actually be, which is out
                // of the furniture and out of the clutter
                if clutter_solid(reaches)
                    .iter()
                    .chain(std::iter::once(&bench_box(reaches)))
                    .any(|box_| box_.contains_point(vec3(at.x, 0.9, at.z)))
                {
                    continue;
                }

                stood += 1;

                for up in [-1.0f32, 1.0] {
                    for side in [-1.0f32, 1.0] {
                        let corner = block + vec3(side * half, up * half, -half);
                        let way = (corner - at).normalize();
                        let lands = at + way * ((bay.min.z - at.z) / way.z);

                        assert!(
                            lands.x > bay.min.x && lands.x < bay.max.x,
                            "from {:.2} {:.2} the room shows past a corner, {:.2} across",
                            at.x,
                            at.z,
                            lands.x
                        );
                        assert!(
                            lands.y > 0.0 && lands.y < bay.max.y,
                            "from {:.2} {:.2} the room shows past a corner, {:.2} up",
                            at.x,
                            at.z,
                            lands.y
                        );
                    }
                }
            }
        }

        assert!(stood > 40, "only {} places to stand were swept", stood);
    }

    /// Spec 0025: from the working spot the block takes forty degrees of the
    /// view, which is about what it fills in the example's own window.
    ///
    /// The number this whole change is for. It was 0.62 across at two and a
    /// bit metres, which is seventeen degrees: a model of a thing on a
    /// bench. Jake, after the cubes had already been made white: the example
    /// is more stunning than the closet.
    #[test]
    fn the_block_fills_the_view_from_where_you_work() {
        let reaches = reaches();
        let eye = standing(reaches) + Vec3::Y * 1.55;
        let block = block(reaches);
        let away = (block - eye).length();
        let wide = 2.0 * (BLOCK * 0.5 / away).atan().to_degrees();

        assert!(
            wide > 40.0,
            "the block is {:.0} degrees of the view from {:.2} away",
            wide,
            away
        );

        // and not so close that you are standing in it, which is the other
        // way to fill a view
        assert!(
            away > BLOCK * 0.5 + crate::RADIUS,
            "the working spot is inside the block"
        );
    }

    /// Spec 0025: the bay is lit by its own lamp, and the room's bulb barely
    /// reaches it.
    ///
    /// A black lining under a bulb is grey, and grey is a rectangle painted
    /// on a wall rather than a space. Measured with the engine's own falloff
    /// rather than a rule of thumb about distance, because the falloff is
    /// what the shader does.
    #[test]
    fn the_rig_is_lit_by_its_own_lamp() {
        let reaches = reaches();
        let bay = bay(reaches);
        let block = block(reaches);
        let mine = blitzkit::lighting::PointLight::new(
            bay_lamp(reaches),
            crate::aim::BACK_BAY_LAMP,
            crate::aim::BACK_BAY_LIT,
            crate::aim::BACK_BAY_RANGE,
        );
        let theirs = blitzkit::lighting::PointLight::new(
            lamp(reaches),
            crate::aim::BACK_LAMP,
            crate::aim::BACK_LIT,
            crate::aim::BACK_RANGE,
        );

        for (what, at) in [
            ("the middle of the block", block),
            ("the top of it", block + Vec3::Y * BLOCK * 0.5),
            (
                "the lining behind it",
                vec3(block.x, block.y, bay.min.z + LINING),
            ),
        ] {
            let lit = mine.intensity * mine.falloff(mine.position.distance(at));
            let spill = theirs.intensity * theirs.falloff(theirs.position.distance(at));

            assert!(
                lit > spill * 4.0,
                "at {} the bay's lamp gives {:.3} and the room's gives {:.3}",
                what,
                lit,
                spill
            );
        }

        // and the room's own bulb still shows the door, or the room is a
        // dark box with one lit thing in it and no way out
        let (from, to) = way(reaches);
        let door = vec3(span().0, 1.0, (from + to) * 0.5);

        assert!(
            theirs.falloff(theirs.position.distance(door)) > 0.05,
            "the room's bulb does not reach the door"
        );
    }

    /// Spec 0025: the bay's lamp is in a fitting, and inside the bay.
    #[test]
    fn the_bay_lamp_is_in_its_fitting() {
        let reaches = reaches();
        let at = bay_lamp(reaches);
        let shade = Aabb::from_center_size(at + Vec3::Y * BAY_SHADE.y * 0.5, BAY_SHADE);

        assert!(shade.contains_point(at), "the bay's lamp is not in a shade");
        assert!(
            bay(reaches).contains_point(at),
            "the bay's lamp hangs outside the bay"
        );
        assert!(at.y < TALL, "the bay's lamp is above the ceiling");
    }

    /// Spec 0025: the bay holds the rig and nothing else.
    ///
    /// The clutter was all along the far wall and the bench was across it,
    /// which is where the dark is now. A bucket in the bay is a bucket lit
    /// by the one lamp that is meant to be lighting the cubes.
    #[test]
    fn the_bay_holds_nothing_but_the_rig() {
        let reaches = reaches();
        let bay = bay(reaches);

        for box_ in clutter_solid(reaches) {
            assert!(
                !box_.intersects(&bay),
                "a piece of clutter at {:?} is in the bay",
                box_.center()
            );
        }

        assert!(
            !bench_box(reaches).intersects(&bay),
            "the bench is in the bay"
        );

        // and the door does not open into it: walking in should not put you
        // in the dark before you have seen the room
        let (from, to) = way(reaches);

        assert!(
            from > bay.max.z,
            "the doorway opens into the bay at {:.2}, which ends at {:.2}",
            from,
            bay.max.z
        );
        assert!(to > from);
    }

    /// Spec 0024: nothing in the clutter stands where somebody has to walk,
    /// and none of it is inside anything else.
    ///
    /// The walk from the door to the bench is the one line in this room that
    /// has to stay clear, and a room you pick your way through is a room
    /// nobody goes back into. Checked as a corridor a body wide rather than
    /// as a line, because a body is 0.9 across and a line is nothing across.
    #[test]
    fn the_clutter_is_against_the_walls() {
        let reaches = reaches();
        let (from, to) = way(reaches);
        let door = vec3(GALLERY_WIDE * 0.5 + THICK, 0.0, (from + to) * 0.5);
        let bench = standing(reaches);
        let wide = 0.9;

        for box_ in clutter_solid(reaches) {
            for n in 0..=40 {
                let along = n as f32 / 40.0;
                let at = door + (bench - door) * along;
                let near = Aabb::from_center_size(at + Vec3::Y * 0.9, vec3(wide, 1.8, wide));

                assert!(
                    !box_.intersects(&near),
                    "something stands on the way to the bench, at {:?} to {:?}",
                    box_.min,
                    box_.max
                );
            }
        }

        // and none of it is inside another piece, or the room is a heap
        let all = clutter_solid(reaches);

        for (n, one) in all.iter().enumerate() {
            for other in &all[n + 1..] {
                let shared = (one.max.min(other.max) - one.min.max(other.min)).min_element();

                assert!(
                    shared <= 1e-3,
                    "two pieces of clutter overlap by {:.3}: {:?} and {:?}",
                    shared,
                    one.min,
                    other.min
                );
            }
        }

        // and all of it is inside the room
        let (west, east) = span();
        let (far, near) = deep(reaches);

        for box_ in all {
            assert!(
                box_.min.x > west && box_.max.x < east,
                "{:?} to {:?} is through a side wall",
                box_.min,
                box_.max
            );
            assert!(
                box_.min.z > far && box_.max.z < near,
                "{:?} to {:?} is through an end wall",
                box_.min,
                box_.max
            );
            assert!(box_.max.y < TALL, "something is through the ceiling");
        }
    }

    /// Spec 0024: every step of the dial is a cube number, so the block it
    /// draws is a block.
    ///
    /// The fault this is written against is visible rather than arithmetic,
    /// which is how it survived a picture and a test suite: 256 cubes in a
    /// seven a side grid is five full layers and a ragged one, and what you
    /// see is a block with a bite out of it.
    #[test]
    fn every_step_of_the_dial_is_a_cube() {
        for count in CUBES {
            assert!(
                blitzkit::grid::fills(count),
                "{} cubes is {} a side, which is {}",
                count,
                blitzkit::grid::side(count),
                blitzkit::grid::rounded(count)
            );
        }

        // and the block that draws has a cube in every slot, which is the
        // same claim from the other end: no two share a place, none is
        // outside the block, and the count fills the grid exactly
        for count in CUBES {
            let wide = blitzkit::grid::side(count);
            let mut seen = std::collections::HashSet::new();

            for n in 0..count {
                let at = cube_at(count, n);
                let key = (
                    (at.x * 1000.0).round() as i32,
                    (at.y * 1000.0).round() as i32,
                    (at.z * 1000.0).round() as i32,
                );

                assert!(
                    seen.insert(key),
                    "two cubes of {} stand in one place",
                    count
                );
                assert!(
                    at.abs().max_element() <= BLOCK * 0.5 + 1e-4,
                    "a cube of {} is outside the block",
                    count
                );
            }

            assert_eq!(seen.len() as u32, count);

            if count > 0 {
                assert_eq!(seen.len() as u32, wide * wide * wide);
            }
        }
    }

    /// Spec 0025: pointing at the cubes works the rig, and pointing at the
    /// workbench does not.
    ///
    /// The rig is one of the building's benches, and what it is is the block
    /// rather than a table: Jake, pointing at the cubes and getting nothing,
    /// if i'm pointing at the thing demonstrated it should be interactable.
    /// The workbench on the east wall is furniture after spec 0025 and is
    /// the control: it is the thing in here that looks most like a bench and
    /// it is not the one the sight answers with.
    #[test]
    fn pointing_at_the_cubes_finds_the_rig() {
        let reaches = reaches();
        let room = crate::room::Room::of(crate::games());
        let n = room
            .benches
            .iter()
            .position(|bench| bench.name == RIG)
            .expect("the rig is one of the benches");

        let eye = standing(reaches) + Vec3::Y * 1.55;

        for at in [
            block(reaches),
            block(reaches) + Vec3::Y * BLOCK * 0.4,
            block(reaches) - Vec3::Y * BLOCK * 0.4,
        ] {
            let way = (at - eye).normalize();

            assert_eq!(
                room.looking_at(eye, way),
                Some(crate::room::Seen::Bench(n)),
                "looking at {:?} from the working spot found something else",
                at
            );
        }

        // and the workbench is not it, from somebody standing at the
        // workbench
        let bench = bench(reaches);
        let beside = vec3(bench.x - 1.0, 1.55, bench.z);
        let at_it = (bench + Vec3::Y * BENCH.y - beside).normalize();

        assert_ne!(
            room.looking_at(beside, at_it),
            Some(crate::room::Seen::Bench(n)),
            "the workbench answers as the rig"
        );

        // and the sight is not answering from anywhere in the room: the far
        // corner is a wall away from it
        let (west, _) = span();
        let (_, near) = deep(reaches);
        let corner = vec3(west + 0.4, 1.55, near - 0.4);
        let away = (vec3(west, 1.2, near) - corner).normalize();

        assert_ne!(
            room.looking_at(corner, away),
            Some(crate::room::Seen::Bench(n)),
            "the rig answers from across the room, facing a wall"
        );
    }

    /// Spec 0024: every word on the gauge board is on the board, and clear
    /// of the tracks and of the other words.
    ///
    /// Written after the words were set where the track begins, which is
    /// where they end: the bar was drawn over the last letter of each of
    /// them. Then widened, after Jake read the board back as three names
    /// conveying nothing: worst and draws still don't really convey meaning.
    /// A heading and a full-scale reading per bar is three more places a
    /// word can run off the board, so all seven are measured here.
    ///
    /// Measured at the width the font will actually make them rather than at
    /// a guess, so a longer word later fails here instead of on the wall.
    #[test]
    fn every_word_on_the_board_is_on_the_board() {
        let wide_as = |says: &str| {
            let shape = blitzkit::text::room_for(says, GAUGE_TEXELS);

            GAUGE_WORD_TALL * (shape.x / shape.y.max(1e-4))
        };
        let on_the_board = |says: &str, at: Vec3| {
            assert!(
                at.x > -GAUGE.x * 0.5,
                "{} starts off the left of the board",
                says
            );
            assert!(
                at.x + wide_as(says) < GAUGE.x * 0.5,
                "{} runs off the right of the board",
                says
            );
            assert!(
                at.y - GAUGE_WORD_TALL * 0.5 > -GAUGE.y * 0.5
                    && at.y + GAUGE_WORD_TALL * 0.5 < GAUGE.y * 0.5,
                "{} is off the top or bottom of the board",
                says
            );
            assert!(at.z > 0.0, "{} is drawn behind the board", says);
        };

        // the heading, which is above every bar
        let says = gauge_says();

        on_the_board(GAUGE_SAYS, says);

        for n in 0..GAUGE_WORDS.len() {
            let (bar, _) = gauge_bar(n, 0.0);

            assert!(
                says.y - GAUGE_WORD_TALL * 0.5 > bar.y + GAUGE_BAR * 0.5,
                "the heading sits on bar {}",
                n
            );
        }

        // and the rows are not all crowded under it: what is over the first
        // bar and what is under the last come out the same
        let (first, _) = gauge_bar(0, 0.0);
        let (last, _) = gauge_bar(GAUGE_WORDS.len() - 1, 0.0);
        let over = says.y - GAUGE_WORD_TALL * 0.5 - (first.y + GAUGE_WORD_TALL * 0.5);
        let under = last.y - GAUGE_WORD_TALL * 0.5 + GAUGE.y * 0.5;

        assert!(
            (over - under).abs() < 0.03,
            "{:.3} over the first bar and {:.3} under the last",
            over,
            under
        );

        for (n, says) in GAUGE_WORDS.iter().enumerate() {
            let word = gauge_word(n);
            let full = gauge_full_at(n);
            let (bar, _) = gauge_bar(n, 0.0);
            let (begins, ends) = (bar.x - GAUGE_TRACK * 0.5, bar.x + GAUGE_TRACK * 0.5);

            on_the_board(says, word);
            on_the_board(GAUGE_FULL_SAYS[n], full);

            // the name stops before its track, and the reading starts after
            // it, which is the fault this test was written for
            assert!(
                word.x + wide_as(says) < begins,
                "{} ends at {:.3} and its track begins at {:.3}",
                says,
                word.x + wide_as(says),
                begins
            );
            assert!(
                full.x > ends,
                "{} starts at {:.3} and its track ends at {:.3}",
                GAUGE_FULL_SAYS[n],
                full.x,
                ends
            );

            // and both of them read as belonging to that bar, which they
            // only do level with it
            for (what, at) in [(says, word), (&GAUGE_FULL_SAYS[n], full)] {
                assert!(
                    (at.y - bar.y).abs() < 1e-5,
                    "{} is not level with its own bar",
                    what
                );
            }

            // the rows do not touch each other either
            if n + 1 < GAUGE_WORDS.len() {
                let (under, _) = gauge_bar(n + 1, 0.0);

                assert!(
                    bar.y - GAUGE_BAR * 0.5 > under.y + GAUGE_BAR * 0.5,
                    "bars {} and {} overlap",
                    n,
                    n + 1
                );
            }
        }
    }

    /// Spec 0024: the lamp is in its fitting, which is the fault every room
    /// in here has had at least once.
    #[test]
    fn the_lamp_is_in_the_shade() {
        let reaches = reaches();
        let at = lamp(reaches);
        let shade = Aabb::from_center_size(at + Vec3::Y * SHADE.y * 0.5, SHADE);

        assert!(
            shade.contains_point(at),
            "the lamp at {:?} is not inside its shade",
            at
        );
        assert!(at.y < TALL, "the lamp is above the ceiling");

        let (west, east) = span();
        let (far, near) = deep(reaches);

        assert!(at.x > west && at.x < east && at.z > far && at.z < near);
    }

    /// And somebody standing at the rig is in the room, clear of it, and
    /// not inside anything.
    #[test]
    fn there_is_room_to_stand_at_the_rig() {
        let reaches = reaches();
        let at = standing(reaches);
        let (west, east) = span();
        let (far, near) = deep(reaches);

        assert!(at.x > west + crate::RADIUS && at.x < east - crate::RADIUS);
        assert!(at.z > far + crate::RADIUS && at.z < near - crate::RADIUS);

        for box_ in solid(reaches) {
            assert!(
                !box_.contains_point(at + Vec3::Y * 1.0),
                "whoever stands at the bench is inside {:?} to {:?}",
                box_.min,
                box_.max
            );
        }

        // and a body's width clear of the rig itself, which is what makes
        // it somewhere to stand rather than somewhere to be wedged
        let rig = crate::room::Room::bench_box(&benched(reaches));

        assert!(
            at.z - rig.max.z > 0.5,
            "there is {:.2} between the rig and whoever is at it",
            at.z - rig.max.z
        );

        // out of the dark, too: you work it from the room, looking in
        assert!(
            at.z > bay(reaches).max.z,
            "the working spot is inside the bay"
        );
    }
}
