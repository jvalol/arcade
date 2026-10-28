//! The room, the cabinets in it, and where you are standing. Spec 0001.
//!
//! Nothing here draws or reads a key. The shape of the room and which cabinet
//! you are at are arithmetic, so all of it can be checked without a window.

use crate::cabinet::Cabinet;
use crate::cellar;
use crate::garden;
use blitzkit::collision::Aabb;
use blitzkit::mesh::{MeshData, Vertex};
use glam::{vec3, Vec3};

/// How big a cabinet is: deep into the wall, taller than you, and as wide along
/// the aisle as a cabinet's front needs to be.
///
/// The rows face each other across the aisle, so the frontage is the z extent
/// and the depth is the x one. Sized the other way round at first, which made
/// every screen a portrait panel with a landscape screenshot squashed into it.
pub const CABINET: Vec3 = vec3(0.7, 1.9, 0.95);

/// How much of a cabinet's front is screen, and the shape of the picture on it.
/// The screenshots are 1200 by 948, and a screen that does not match that is a
/// stretched game.
pub const SCREEN: f32 = 0.82;
pub const SCREEN_SHAPE: f32 = 948.0 / 1200.0;

/// How far apart they stand along a wall, middle to middle.
pub const APART: f32 = 2.2;

/// A bench: what a toy stands on, per spec 0006.
///
/// Waist high and deep enough to lean over. Not a cabinet, because there is no
/// game on it and nothing to win; not a plinth, because you work it rather than
/// walk round it. It is the third thing the room holds.
///
/// How tall and how deep are the same for all of them, because that is what
/// makes a row of them read as a row. How long is not: the size travels with
/// the toy, since a cradle's frame has to clear the swing of its end balls and
/// a metronome wants a square foot.
pub const BENCH_TALL: f32 = 0.95;
pub const BENCH_WIDE: f32 = 0.9;

/// How far a bench stands off the wall it is set against, and how much floor is
/// left between two of them.
///
/// Set against the open side, opposite the books, rather than stood out in the
/// floor. Out in the floor is what it was, and the aim of that was floor behind
/// the benches as well as in front. It got one and it was no use: a run of
/// benches the length of the wall with 0.3 between them seals the strip behind
/// it at both ends, so the only way in was between two benches and nobody is
/// 0.3 across. Three goes at widening that strip, 1.1 to 1.5 to 1.9 to 2.4, made
/// the unreachable part of the room bigger each time.
///
/// Against one long wall and the books against the other, the floor between them
/// is one piece and every part of it is somewhere you can be. The toys face the
/// books across it, which is also what the room is for.
pub const BENCH_OFF: f32 = 0.02;
pub const BENCH_GAP: f32 = 0.7;

/// The nook the toys live in: how far it cuts in behind the left wall and how
/// much of that wall it runs along.
///
/// A room off the room, not a table in it. Standing the benches in the aisle
/// put them in front of the plinths from every angle, which is the one place
/// nothing should stand.
///
/// Three times over. It was 3.0 by 8.0 with the benches end to end down one
/// wall, which is 24 square units of floor holding five benches and a wall of
/// books, and it read as a corridor because it was one. 5.4 by 8.0 furnished it
/// and still had you turning sideways. 6.4 by 14.0 was right until the way up
/// went in: a flight 1.2 wide with a parapet either side takes 1.8 out of the
/// middle of the floor, and what it left was a lane of 1.75 at the books and
/// 1.23 at the benches, which is over the 1.05 a body needs and reads as a
/// corridor either side of a staircase.
///
/// It is 10.0 by 14.0. The stair still stands in the middle of it, the lanes
/// are 3.63 and 3.03, and the open floor between the benches and the books is
/// 8.46 across.
///
/// **What stops it being wider than that is the room over it.** The stair is
/// in the middle of this floor and the imaginarium is the lid the stair comes
/// up through, so every metre of nook is half a metre of stair moving west,
/// and at 11.0 the parapet is outside the room above it.
pub const NOOK_DEEP: f32 = 10.0;
pub const NOOK_SPAN: f32 = 14.0;

/// How much room the far end of the room carries past the last cabinet.
///
/// It was one cabinet step. That was enough while that end held nothing but the
/// shapes on their plinths and you could walk straight through them. The plinths
/// are solid now, and the way into the nook runs between the last cabinet and
/// the nearest plinth: at one step that gap came out 0.74 wide for someone 0.9
/// across, so the nook had a doorway you could see through and not get to.
///
/// Flooded rather than worked out. The way opens between 2.35 and 2.4, and this
/// is the next round number clear of that, which leaves about half a step of
/// room either side of you going through.
pub const END: f32 = 2.8;

/// How wide the way in is.
///
/// The stretch of left wall past the last cabinet, which is where this spec
/// always said the nook opens. It was the whole span instead, so a cabinet
/// stood in the opening with nothing behind it: from the aisle you saw through
/// its back into the nook, and from the nook you saw the back of a cabinet.
/// Taking the wall away where the cabinets are was never the point. The nook is
/// behind that wall and you get to it round the end of it.
pub const NOOK_DOOR: f32 = END - CABINET.z * 0.5;

/// How far the room's walls are from its middle, how high, and how thick.
///
/// **It cannot grow without `REACH` growing with it**, which is worth writing
/// down because it is not obvious and it was measured rather than reasoned.
/// The aisle is 4.0 across and the sight carries 4.0, and those are the same
/// number on purpose: standing by one cabinet you can work the one opposite.
/// Widening the room by as little as 0.2 breaks that, and
/// `room::tests::the_one_you_are_looking_at_is_the_one` fails at every width
/// from 2.2 up.
///
/// Which matters because what bounds the row of shapes on the far wall is
/// where the end cabinets stand, and that is this. At 2.0 the gap between
/// their inner faces is 3.3, six shapes at 0.59 apart are 3.29 of row, and
/// the row clears the gap by half a centimetre at each end. There is no more
/// room on that wall, and there is no more room in the hall for the wall to
/// have, unless the sight is allowed to carry further.
pub const WALL: f32 = 2.0;
pub const TALL: f32 = 3.2;

/// A piece of the grid over the hall: where it is and how big. Spec 0012.
pub type Hung = (Vec3, Vec3);

/// What is left of a box with another taken out of it.
///
/// Six pieces at most, and none where the cut runs out to a face. Written
/// for spec 0037, which needed it three times in one room: the slot in the
/// imaginarium's floor, the shaft wall that must not cross the well beside
/// it, and the ceiling tiles the stairwell comes through.
///
/// **In all three directions and not only in plan.** It cut in `x` and `z`
/// alone to begin with, which is right for a hole in a floor or a ceiling
/// and wrong for a door: spec 0038 put a door in a wall and got a slot from
/// the floor to the lid, because a cut that ignores height takes the whole
/// height. The three earlier uses are unchanged by this, since in each of
/// them the cut already spans the box's full height.
pub fn minus(box_: Aabb, cut: Aabb) -> Vec<Aabb> {
    if !box_.intersects(&cut) {
        return vec![box_];
    }

    let mut out = Vec::new();
    let keep = |out: &mut Vec<Aabb>, low: Vec3, high: Vec3| {
        if (high - low).min_element() > 1e-4 {
            out.push(Aabb::new(low, high));
        }
    };

    // under the cut and over it
    keep(&mut out, box_.min, vec3(box_.max.x, cut.min.y, box_.max.z));
    keep(&mut out, vec3(box_.min.x, cut.max.y, box_.min.z), box_.max);

    // then to each side of what is left
    let (under, over) = (box_.min.y.max(cut.min.y), box_.max.y.min(cut.max.y));
    keep(
        &mut out,
        vec3(box_.min.x, under, box_.min.z),
        vec3(cut.min.x, over, box_.max.z),
    );
    keep(
        &mut out,
        vec3(cut.max.x, under, box_.min.z),
        vec3(box_.max.x, over, box_.max.z),
    );

    // and before and behind what is left of that
    let (near, far) = (box_.min.x.max(cut.min.x), box_.max.x.min(cut.max.x));
    keep(
        &mut out,
        vec3(near, under, box_.min.z),
        vec3(far, over, cut.min.z),
    );
    keep(
        &mut out,
        vec3(near, under, cut.max.z),
        vec3(far, over, box_.max.z),
    );

    out
}

/// The grid over the hall: how big a ceiling tile is, how far the bar between
/// two of them hangs below, and how wide that bar is. Spec 0012.
pub const CEIL_TILE: f32 = 1.15;
pub const CEIL_DROP: f32 = 0.055;
pub const CEIL_BAR: f32 = 0.05;

/// How high the ceiling you can see is, as against the lid over it.
///
/// Everything hung in the hall hangs from this. The neon's stem and the
/// sign's pendant both ran to `TALL`, which with a grid under it puts them
/// through the tiles: right for a cable and wrong for the two things in the
/// room that are meant to be hanging.
pub fn soffit() -> f32 {
    TALL - CEIL_DROP
}

/// How thick a ceiling tile is and where it hangs, which is above the bars
/// so that nothing up there is level with anything. Spec 0012.
///
/// **Named because the way up reads them.** The shaft spec 0037 climbs
/// through has to begin inside a tile's own thickness: level with their
/// underside it speckles against them, and anywhere lower it hangs across
/// the hall as a white beam. `aloft::up_shaft` worked that height out from
/// `soffit` instead, which is the bars' underside and eleven millimetres
/// lower, and the beam was in the hall's ceiling from the day the way up
/// went in. Two places saying where a tile is, and they disagreed.
pub const CEIL_TILE_THICK: f32 = CEIL_DROP * 0.3;

/// The lowest a tile reaches, which is what nothing above the ceiling may
/// come below.
pub fn tile_under() -> f32 {
    TALL - CEIL_DROP * 0.4
}

/// And the plane through the middle of one, which is where a thing that has
/// to be hidden by the ceiling without sharing a face with it begins.
pub fn tile_middle() -> f32 {
    tile_under() + CEIL_TILE_THICK * 0.5
}
pub const THICK: f32 = 0.3;

/// How far inside the near wall you wake up.
///
/// A named number because what is in front of you when you open your eyes is
/// measured from here, and the room's own sign is hung to be in it.
pub const INSIDE: f32 = 1.0;

/// The room across the end of the aisle, per spec 0022: how wide, how deep,
/// and how wide the way into it is.
///
/// **Wider than the aisle and shallower than it is wide.** The shapes stand
/// along its far wall and the row is a little over five metres, so the width
/// is the row plus a real margin at each end rather than the half centimetre
/// the strip of end wall was down to. The depth is what puts the whole row in
/// view from the back of the room: much less and you see two shapes, much
/// more and it is a corridor with the shapes at the end of it.
///
/// The way in is the clear aisle exactly, `WALL - CABINET.x * 0.5` either
/// side, so you walk out of the gap between the last two cabinets and into
/// something broader. A gap the full width of the room would be the end wall
/// simply stopping, which is what this is replacing.
/// How much of the hotel's corridor the reach fill is asked to cover.
///
/// Not two kilometres of it. See [`Room::fillable`].
#[cfg(test)]
pub const CORRIDOR_FILLED: f32 = 24.0;

/// Wide enough for the row with the picture's hole in the middle of it,
/// and `display::CLEAR` of bare wall past each end.
///
/// It was 6.8 for a row of six standing shoulder to shoulder. Spec 0039's
/// picture now stands in the middle of that row rather than behind it, per
/// `display::gap`, and the room holds what the row came to.
pub const GALLERY_WIDE: f32 = 10.4;

/// How much wall is in frame per unit you stand back from it.
///
/// Measured off a picture rather than worked out of the camera: standing
/// 3.65 back from the row, 4.2 of it was in the window, and 4.2 over 3.65 is
/// this. It is here so the depth below can be derived instead of guessed,
/// and the first guess at that depth was 4.2 and cut both ends off the row.
pub const IN_FRAME: f32 = 1.15;

/// How deep the gallery is: far enough back to see the whole row, plus room
/// to stand in behind that.
///
/// Derived, so a row that grows takes the room with it rather than quietly
/// not fitting in it, which is the whole fault this room exists to fix.
pub fn gallery_deep() -> f32 {
    let wants = crate::display::shapes() + crate::display::CLEAR * 2.0;

    wants / IN_FRAME + 0.6
}

/// Where its floor is and how big, with the hall's far wall as its near edge.
pub fn gallery(reaches: f32) -> (Vec3, Vec3) {
    (
        vec3(0.0, 0.0, -reaches - gallery_deep() * 0.5),
        vec3(GALLERY_WIDE, 0.0, gallery_deep()),
    )
}

/// The gap in what used to be the far wall, as the two x it runs between.
pub fn gallery_way() -> (f32, f32) {
    let clear = WALL - CABINET.x * 0.5;

    (-clear, clear)
}

/// Where the gallery's lamps hang and what they are.
///
/// **Over the shapes rather than over the room.** The one thing in here is a
/// row along the far wall, so the light belongs on that row: a lamp over the
/// middle of the room would light the floor and leave the shapes against a
/// bright wall. Three of them, which covers a row of six without making a
/// fitting per shape.
///
/// They hang from the ceiling on the shapes' own line, out from the wall by
/// the same amount the shapes are, so a shape is lit from in front and above
/// and throws its shadow back onto the wall rather than at you.
pub const GALLERY_LAMPS: usize = 3;
pub const GALLERY_LAMP_UP: f32 = TALL - 0.38;
pub const GALLERY_SHADE: Vec3 = vec3(0.34, 0.12, 0.26);

pub fn gallery_lamps(reaches: f32) -> Vec<Vec3> {
    let z = gallery_wall(reaches) + crate::display::OFF_THE_WALL;
    let span = crate::display::row();
    let step = span / GALLERY_LAMPS as f32;

    (0..GALLERY_LAMPS)
        .map(|n| {
            let along = (n as f32 + 0.5) * step - span * 0.5;

            vec3(along, GALLERY_LAMP_UP, z)
        })
        .collect()
}

/// Where the shapes stand: the gallery's far wall.
pub fn gallery_wall(reaches: f32) -> f32 {
    -reaches - gallery_deep()
}

/// How far the sight carries. Spec 0003.
///
/// Spec 0001 replaced "the nearest one in front of you, within reach and
/// nearly enough faced" with pointing and nothing else, because having to face
/// a thing meant walking the aisle to learn what anything was. Dropping the
/// facing was right and dropping the reach with it was not: from the doorway
/// the room named the Klein bottle thirteen units away and offered to turn it,
/// at a size where you cannot see what you would be turning.
///
/// Just under two steps down the aisle, so a thing comes alive as you reach it
/// rather than when you merely aim at it.
pub const REACH: f32 = 4.0;

/// And how far one of the nook's toys does, which is less.
///
/// Near enough that you had to walk to it, and far enough that you are not
/// standing on it: the benches are 0.9 deep and you work them from about
/// three quarters of a metre off the near face. The sight test that has
/// stood since the nook was built stands 1.4 away and still passes.
pub const BENCH_REACH: f32 = 1.8;

/// A floor quad whose u and v run past one, so a tiling texture repeats across
/// it rather than being stretched over the whole thing.
///
/// `MeshData::plane` runs its corners 0 to 1, which is one carpet tile thirty
/// units wide. The engine's sampler repeats, so the count belongs here.
pub fn tiled_floor(tiles: f32) -> MeshData {
    tiled_plane(glam::vec2(tiles, tiles))
}

/// The same, counted separately along each axis.
///
/// One count does for a carpet, whose weave has no direction to get wrong. It
/// will not do for a floor of boards in a room nearly twice as long as it is
/// wide: one count stretches the planks with the room, so the boards come out
/// as wide as a tabletop down one axis and as narrow as a pencil down the other.
pub fn tiled_plane(tiles: glam::Vec2) -> MeshData {
    let normal = [0.0, 1.0, 0.0];
    let (u, v) = (tiles.x, tiles.y);
    let corners = [
        ([-0.5, 0.0, 0.5], [0.0, v]),
        ([0.5, 0.0, 0.5], [u, v]),
        ([0.5, 0.0, -0.5], [u, 0.0]),
        ([-0.5, 0.0, -0.5], [0.0, 0.0]),
    ];

    MeshData::new(
        corners
            .iter()
            .map(|(at, uv)| Vertex::new(*at, normal, *uv))
            .collect(),
        vec![0, 1, 2, 0, 2, 3],
    )
}

/// Which way to turn a `tiled_plane` so its face looks the way `looks` does.
///
/// The quad is built flat, looking up, so this is the one rotation that takes
/// it to any wall. Written out in three places before it was named: the pool's
/// lining, the garden's plaster and the baths' tiling all want it, and three
/// copies of a rotation is three chances to get a sign wrong.
pub fn laid_on(looks: Vec3) -> glam::Quat {
    if looks.y.abs() > 0.5 {
        if looks.y > 0.0 {
            glam::Quat::IDENTITY
        } else {
            glam::Quat::from_rotation_x(std::f32::consts::PI)
        }
    } else if looks.x.abs() > 0.5 {
        // the quad's normal is its local +Y, and a turn of a quarter about z
        // takes +Y to -x, so the sign here is the other way round from the way
        // it reads. The baths' lining has the same two lines with both signs
        // flipped, because there `looks` is the way the water is rather than
        // the way the surface faces, and copying them straight over drew every
        // wall of the garden inside out.
        glam::Quat::from_rotation_z(-looks.x * std::f32::consts::FRAC_PI_2)
            * glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
    } else {
        glam::Quat::from_rotation_x(looks.z * std::f32::consts::FRAC_PI_2)
    }
}

/// How big to make a quad `laid_on(looks)` so it covers a box face of
/// `size`.
///
/// The quad lies in its own x and z with its normal up, so which of the
/// box's three extents goes in which slot depends on where that turn sent
/// them: a face looking along x is scaled by the box's z and its y, one
/// looking along z by its x and its y, and one looking up or down by its x
/// and its z. Worked out at the call rather than here, it is the third
/// slot that gets it wrong, and it gets it wrong silently.
pub fn quad_on(size: glam::Vec3, looks: Vec3) -> Vec3 {
    if looks.y.abs() > 0.5 {
        vec3(size.x, 1.0, size.z)
    } else if looks.x.abs() > 0.5 {
        vec3(size.z, 1.0, size.y)
    } else {
        vec3(size.x, 1.0, size.y)
    }
}

/// Which way to turn a quad so its one face looks the way a cabinet does.
pub fn turned_to(facing: Vec3) -> glam::Quat {
    glam::Quat::from_rotation_y(if facing.x >= 0.0 {
        0.0
    } else {
        std::f32::consts::PI
    })
}

/// The screen itself: one quad facing +x, so the screenshot sits on it the way
/// it was taken.
///
/// `MeshData::cube` would do for the shape, and it is what the cabinets are
/// made of, but its faces carry the texture whichever way round they please and
/// the pictures came out turned on their sides. A quad with the corners written
/// down is three lines and leaves nothing to find out.
pub fn screen_mesh() -> MeshData {
    let normal = [1.0, 0.0, 0.0];
    // u runs +z to -z, which is left to right for someone standing in front of
    // this face, since the screen's right runs towards -z from there.
    //
    // Twice now this has been written down the other way round from reasoning
    // about which way the room's axes point, and twice the pictures came out
    // mirrored. The winding and the u direction decide it together, and the
    // only reliable way to settle it is to draw a word and read it.
    let corners = [
        ([0.0, 0.5, -0.5], [1.0, 0.0]),
        ([0.0, 0.5, 0.5], [0.0, 0.0]),
        ([0.0, -0.5, 0.5], [0.0, 1.0]),
        ([0.0, -0.5, -0.5], [1.0, 1.0]),
    ];

    // Two faces carrying the same u, which is what a lit sign does: from
    // behind you read it backwards, through it.
    //
    // It began as two with the same u and nothing turning the quad, so the two
    // sides of the aisle were reading opposite faces of it and half the room
    // came out mirrored. Then one face, turned, which made the back of a sign
    // nothing at all.
    //
    // Both: `turned_to` points the front the way its cabinet faces, so each
    // side of the room reads its own cabinets the right way round, and the
    // back is the same sign seen through itself.
    let back = [-1.0, 0.0, 0.0];
    let mut vertices: Vec<Vertex> = corners
        .iter()
        .map(|(at, uv)| Vertex::new(*at, normal, *uv))
        .collect();
    vertices.extend(corners.iter().map(|(at, uv)| Vertex::new(*at, back, *uv)));

    // The front is wound counter clockwise seen from +x, which is the side its
    // normal points, and the back the other way about. A face wound the wrong
    // way is culled and drawn nowhere: the one sided build of this drew
    // nothing at all until the winding was turned round.
    MeshData::new(vertices, vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6])
}

/// The same quad, with the back reading forwards instead of backwards.
///
/// [`screen_mesh`] is a lit sign seen through itself: both faces carry the same
/// u, so from behind you read it the wrong way round, which is what a cabinet's
/// marquee does and is right for one. A sign hung across a corridor is not that.
/// People come at it from both ends and it has to read from both, so this one
/// is two signs back to back, which is the thing that doc warns against and is
/// exactly what is wanted here.
pub fn hung_mesh() -> MeshData {
    let mut mesh = screen_mesh();
    let (_, back) = mesh.vertices.split_at_mut(4);
    for vertex in back {
        vertex.uv[0] = 1.0 - vertex.uv[0];
    }

    mesh
}

/// How far along a ray a box is first met, if it is. The slab test, which the
/// engine has for an `Aabb` only through `Ray`, and this wants no `Ray`.
fn slab(from: Vec3, way: Vec3, box_: &Aabb) -> Option<f32> {
    let (lo, hi) = (
        box_.center() - box_.size() * 0.5,
        box_.center() + box_.size() * 0.5,
    );
    let mut entry = f32::NEG_INFINITY;
    let mut exit = f32::INFINITY;

    for n in 0..3 {
        if way[n].abs() < 1e-6 {
            if from[n] < lo[n] || from[n] > hi[n] {
                return None;
            }
            continue;
        }

        let (near, far) = ((lo[n] - from[n]) / way[n], (hi[n] - from[n]) / way[n]);
        entry = entry.max(near.min(far));
        exit = exit.min(near.max(far));
    }

    (exit >= entry.max(0.0)).then_some(entry.max(0.0))
}

/// A cabinet, where it stands and which way it faces.
#[derive(Debug, Clone)]
pub struct Stood {
    pub cabinet: Cabinet,
    pub at: Vec3,
    /// Which way its screen looks, flat on the floor.
    pub facing: Vec3,
}

/// A bench, where it stands. What is on it is the game's business.
#[derive(Debug, Clone, Copy)]
pub struct Benched {
    pub name: &'static str,
    pub at: Vec3,
    /// How big this one is, which the toy on it decides.
    pub size: Vec3,
    /// Which way off it you stand to work it, flat on the floor.
    ///
    /// Not something to work out from where it is. A bench against a wall has
    /// one side you can be on and it is the side the rest of the room is, and
    /// the room is the thing that knows which that is.
    pub worked_from: Vec3,
}

/// Where the benches stand: a row against the nook's open side, facing the
/// books, in the order you meet them coming through the way in.
///
/// The ball and chain first. It is the reason there is a nook: it is worked
/// rather than watched, so it is not one of the shapes, and there is nothing to
/// win, so it is not one of the games. The room had nowhere to put it and that
/// is what this is.
///
/// The row starts past the way in rather than centred in the whole span. The
/// open side is the wall the way in is a gap in, so a bench centred down it is a
/// bench in the doorway.
pub fn benches(far: f32) -> Vec<Benched> {
    /// What is on them, and how much of the nook's length each one takes.
    const ON_THEM: [(&str, f32); 6] = [
        ("ball and chain", 1.4),
        ("gyroscope", 0.9),
        ("newton's cradle", 2.1),
        ("metronome", 0.9),
        ("globe", 0.9),
        // last, at the shut end. It is the one that opens a window, so it comes
        // after the five you work where they stand rather than before them.
        (crate::cascada::NAME, crate::cascada::LONG),
    ];

    let wall = -(WALL + CABINET.x) - THICK * 0.5 - BENCH_OFF;
    let row: f32 =
        ON_THEM.iter().map(|(_, long)| long).sum::<f32>() + (ON_THEM.len() - 1) as f32 * BENCH_GAP;
    let (door, shut) = (far + NOOK_DOOR, far + NOOK_SPAN);
    let mut z = (door + shut) * 0.5 - row * 0.5;

    let mut out: Vec<Benched> = ON_THEM
        .iter()
        .map(|(name, long)| {
            let at = vec3(wall - BENCH_WIDE * 0.5, 0.0, z + long * 0.5);
            z += long + BENCH_GAP;

            Benched {
                name,
                at,
                size: vec3(BENCH_WIDE, BENCH_TALL, *long),
                // into the room, which from this wall is away from the aisle
                worked_from: -Vec3::X,
            }
        })
        .collect();

    // and the pool table down in the cellar, which is a bench in every way that
    // matters: a thing you walk up to, point at, and press a key at. Spec 0007.
    out.push(cellar::table(-far));
    // and the one in the back room, by the same argument. Spec 0024.
    out.push(crate::back::benched(-far));

    out
}

impl Benched {
    /// Whether this is one of the nook's row, per spec 0006.
    ///
    /// **Was "its top is above ground".** That was a true way of saying "not
    /// the pool table" while the cellar held the only bench outside the nook,
    /// and spec 0024 put one in the back room at ground level: two tests that
    /// are about the nook's row read it as a bench standing in the nook's
    /// doorway and worked from the wrong side. The nook is off the west of
    /// the hall and everything else is not, which is a question about where
    /// a thing is rather than a proxy for it.
    ///
    /// Asked only by the tests, which is why it is behind `cfg(test)`: four
    /// of them are about the nook's row and were each deciding for
    /// themselves what that meant.
    #[cfg(test)]
    pub fn in_the_nook(&self) -> bool {
        self.at.y >= 0.0 && self.at.x < 0.0
    }

    /// Which way is away from you, and which way is your right hand, stood at
    /// this bench to work it.
    ///
    /// Worked out from the side you stand on rather than written down. It was
    /// written down, in two files, as "you come in off the aisle looking along
    /// -x, so your right hand is -z". That was a true sentence about where the
    /// benches used to be, and a compiler has nothing to say about a true
    /// sentence concerning the wrong room: moving them to the other wall left
    /// both of them reading, and left left meaning right.
    pub fn away(&self) -> Vec3 {
        -self.worked_from
    }

    pub fn right(&self) -> Vec3 {
        self.away().cross(Vec3::Y)
    }
}

/// A bookcase, where it stands and which way its face looks.
#[derive(Debug, Clone, Copy)]
pub struct Shelved {
    pub at: Vec3,
    pub facing: Vec3,
}

/// Where the bookcases stand: a run of them the length of the nook's back wall.
///
/// Across the floor from the benches rather than behind them. Behind them is
/// where they were, and the aisle that left was sealed at both ends by the row
/// itself, so the books were a wall you could see and not reach.
///
/// One wall and no more. The open side is the one wall the nook has not got,
/// the far end is the way in and a bookcase in a doorway is a door, and the
/// closed end is where the last bench stands.
pub fn bookcases(far: f32) -> Vec<Shelved> {
    let back = -(WALL + CABINET.x) - NOOK_DEEP;
    let case = crate::study::CASE;
    let mut out = Vec::new();

    // the back wall, filled with as many as go into it
    let run = NOOK_SPAN - 0.3;
    let fits = (run / case.x) as usize;
    let along = fits as f32 * case.x;
    let from = far + NOOK_SPAN * 0.5 - along * 0.5;

    for n in 0..fits {
        out.push(Shelved {
            at: vec3(
                back + 0.15 + case.y * 0.5,
                0.0,
                from + (n as f32 + 0.5) * case.x,
            ),
            facing: Vec3::X,
        });
    }

    out
}

/// The nook's walls that carry sconces: the face of each, which way a bracket
/// reaches off it, and where that wall starts and ends.
///
/// Both long walls, because one was not enough: lit from the open side alone
/// the light fell on the wall it came out of and the four hundred books across
/// the room sat in the dark.
///
/// Each wall's own run, which is the part that was wrong. They shared one, the
/// whole length of the nook, and the open side is not that long: the way in is
/// a gap in it. So the far end of that run put sconces out in the doorway with
/// no wall behind them, hanging in the air over the aisle, lit, throwing light
/// on nothing. The panelling had been told this and the lighting had not, which
/// is what two lists of the same wall gets you.
pub fn sconce_runs(reaches: f32) -> [(f32, f32, f32, f32); 2] {
    let side = WALL + CABINET.x;
    let (far, inset) = (-reaches, 0.2);

    [
        // the open side, which starts past the way in
        (
            -side - THICK * 0.5,
            -1.0,
            far + NOOK_DOOR + inset,
            far + NOOK_SPAN - inset,
        ),
        // and the back wall, which runs the whole of it
        (
            -side - NOOK_DEEP + THICK * 0.5,
            1.0,
            far + inset,
            far + NOOK_SPAN - inset,
        ),
    ]
}

/// The nook's end wall: where it stands and how big it is.
///
/// Named because it is two rooms' wall. It closes the nook, and for the width
/// of the nook it is the garden's south side as well, so the garden carries a
/// parapet over it. A parapet worked out from the garden's own box rather
/// than from this lands a sixth of a metre off, overhanging into the garden
/// on two sides and stopping short on the other two, which from below is a
/// ledge. That is what it was doing until somebody outlined the colliders and
/// looked at the corner.
pub fn nook_end(reaches: f32) -> (Vec3, Vec3) {
    let side = WALL + CABINET.x;

    (
        vec3(-side - NOOK_DEEP * 0.5, TALL * 0.5, -reaches + NOOK_SPAN),
        vec3(NOOK_DEEP, TALL, THICK),
    )
}

/// The nook's own floor: the whole of it, wall to wall.
///
/// Boards under a rug rather than the arcade's carpet under a rug. The arcade's
/// is confetti on black and it reached under here, which the old rug hid by
/// being the size of the room. A rug the size of the room is a floor.
pub fn nook_floor(reaches: f32) -> (Vec3, Vec3) {
    let side = WALL + CABINET.x;

    (
        vec3(-side - NOOK_DEEP * 0.5, 0.0, -reaches + NOOK_SPAN * 0.5),
        vec3(NOOK_DEEP, 0.0, NOOK_SPAN),
    )
}

/// The nook's side of the wall it borrows from the hotel's lobby.
///
/// **Finish a shared wall on both sides.** The lobby's north wall is the
/// nook's south wall. It belongs to `aleph`, and `standing` puts every box of
/// the hotel into one run wearing the hotel's plaster, so the nook had a
/// blank pale slab across its whole south end where every other wall in the
/// room is grained. Jake read it off a screenshot on 2026-10-28. The baths
/// had a mahogany wall down one side of a tiled room for the same reason and
/// it was settled the same way.
///
/// One of the hall's own walls, drawn and collided like the rest of them.
/// `standing::tests::the_shells_are_the_colliders` wants every drawn piece to
/// be a thing that stops you, and a facing that is only a picture is exactly
/// what that rule is against. A hair proud of the borrowed face so the two
/// never share a plane, which is the fault this building has fought seven
/// times, and a hair is all it moves the wall you walk into.
pub fn nook_lining(reaches: f32) -> Vec<Aabb> {
    let wall = crate::aleph::lobby_north(reaches);
    let (at, size) = nook_floor(reaches);
    let proud = 0.01;
    // only as far as the nook is wide: the lobby's wall runs on west past it,
    // and the part past it is in the lobby alone
    let west = (at.x - size.x * 0.5).max(wall.min.x);
    let east = (at.x + size.x * 0.5).min(wall.max.x);

    if east - west <= 1e-3 {
        return Vec::new();
    }

    vec![Aabb::new(
        vec3(west, 0.0, wall.max.z),
        vec3(east, wall.max.y, wall.max.z + proud),
    )]
}

/// The open floor of the nook: where it is and how big, as the middle and the
/// size of a quad lying on it.
///
/// Between the benches on one long wall and the bookcases on the other, and
/// inset from both so there is bare floor showing round it. What a rug goes on.
/// Wall to wall is what it was, which is not a rug, it is a floor, and it ran
/// under four hundred books where nobody would ever see it.
///
/// Twice as long as it is wide and no longer. The nook is nearly three times as
/// long as the open floor is wide, and a rug that shape is a runner.
pub fn open_floor(reaches: f32) -> (Vec3, Vec3) {
    const INSET: f32 = 0.35;

    let side = WALL + CABINET.x;
    let benches = -side - THICK * 0.5 - BENCH_OFF - BENCH_WIDE - INSET;
    let books = -side - NOOK_DEEP + THICK * 0.5 + crate::study::CASE.y + INSET;
    let across = benches - books;

    (
        vec3((benches + books) * 0.5, 0.0, -reaches + NOOK_SPAN * 0.5),
        vec3(
            across,
            0.0,
            (NOOK_SPAN - THICK - INSET * 2.0).min(across * 2.0),
        ),
    )
}

/// What the middle of the screen is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    Cabinet(usize),
    Display(usize),
    Bench(usize),
    /// The one case that is a door. Spec 0007.
    Case,
    /// The sauna's glass door. Spec 0008.
    Sauna,
    /// The door between the cellar and the baths. Spec 0008.
    Baths,
    /// The hotel's desk, which is where somebody is housed. Spec 0030.
    Desk,
    /// The register on the far wall behind the blank door. Spec 0031.
    Book,
    /// The Mandelbrot panel at the end of the gallery's row. Spec 0039.
    Mandel,
    /// One of the hotel's doors, by its room number. Spec 0029.
    ///
    /// The one thing in that corridor you can work, and the only way there is
    /// to put somebody in a room or take them out of one.
    Hotel(u64),
}

pub struct Room {
    pub stood: Vec<Stood>,
    /// The engine's own shapes along the far wall, per spec 0002. Where they
    /// stand, not what they are made of.
    pub displays: Vec<crate::display::Display>,
    /// The benches in front of that wall, per spec 0006.
    pub benches: Vec<Benched>,
    /// The bookcases round them, which hold nothing up and do nothing. One of
    /// them does one thing. Spec 0007.
    pub bookcases: Vec<Shelved>,
    /// Whether the case over the way down is swung open.
    ///
    /// The first thing in this room that is ever one way or the other. Spec
    /// 0001 made `solid` a list worked out from the cabinets and never changed
    /// again, which is what let every test about getting about read it as a
    /// fact. It is a fact about a state now, and the tests that flood the floor
    /// have to say which state they mean.
    pub open: bool,
    /// And whether the sauna's glass door is, which eases the same way on its
    /// own clock. Spec 0008.
    pub sauna_open: bool,
    pub sauna_swing: f32,
    /// And the door to the baths, which is a third thing that swings.
    pub baths_open: bool,
    pub baths_swing: f32,
    /// Which of the hotel's doors are drawn and how far each stands open.
    /// Spec 0029.
    ///
    /// The room number as well as the swing, because which forty are drawn
    /// follows wherever you are standing: the corridor has no last door, so
    /// a list that started at room 1 would be a list of the wrong doors the
    /// moment you walked past the fortieth.
    ///
    /// Held here rather than worked out from the hotel's own state, because
    /// what the sight and the colliders need is where a leaf *is*, and the
    /// game owns who is in which room. The game writes this each frame, the
    /// way it does the baths' swing.
    pub hotel_doors: Vec<(u64, f32)>,
    /// How far it actually is, nought shut and one open, which follows `open`
    /// rather than being it. The box follows this too: a door drawn halfway
    /// open that stops you where it was shut is a worse thing than one that
    /// snaps.
    pub swing: f32,
    pub walls: Vec<Aabb>,
    /// How far the room reaches from its middle, worked out from how many
    /// cabinets there are.
    pub reaches: f32,
}

impl Room {
    /// How long a leaf takes to swing, in seconds.
    ///
    /// Slow. It is a bookcase on a hinge with four hundred books in it, and the
    /// whole of what makes a secret door worth having is the moment between
    /// pulling the book and seeing what is behind it. Snapped from shut to open
    /// in a frame, there is no moment.
    pub const SWINGS_IN: f32 = 1.3;

    /// Lays the cabinets down two facing rows, like a real one, and puts a wall
    /// behind each row and across each end.
    pub fn of(cabinets: Vec<Cabinet>) -> Self {
        let each_side = cabinets.len().div_ceil(2);
        let along = (each_side.max(1) - 1) as f32 * APART;
        let reaches = along * 0.5 + END;

        let stood = cabinets
            .into_iter()
            .enumerate()
            .map(|(n, cabinet)| {
                let side = if n % 2 == 0 { 1.0 } else { -1.0 };
                let down = (n / 2) as f32 * APART - along * 0.5;

                Stood {
                    cabinet,
                    at: vec3(side * WALL, 0.0, down),
                    // facing in across the room, which is where you walk
                    facing: vec3(-side, 0.0, 0.0),
                }
            })
            .collect::<Vec<_>>();

        let thick = THICK;
        let side = WALL + CABINET.x;
        // where the way down is, which is where one of the bookcases stands
        let hole = cellar::opening(reaches);
        // where the way in ends, and the nook's other end
        let door = -reaches + NOOK_DOOR;

        let walls = vec![
            // the right wall, whole
            Aabb::from_center_size(
                vec3(side, TALL * 0.5, 0.0),
                vec3(thick, TALL, reaches * 2.0),
            ),
            // the left wall, which backs the cabinets and closes the nook's
            // long side both. It starts past the last cabinet, and the gap it
            // leaves there is the way in.
            //
            // In three pieces, because the garden is through it: spec 0010 cuts
            // an opening in the bare stretch past the last cabinet, and
            // `garden::way` is the only place that says where.
            Aabb::from_center_size(
                vec3(-side, TALL * 0.5, (garden::way(reaches).0 + door) * 0.5),
                vec3(thick, TALL, garden::way(reaches).0 - door),
            ),
            Aabb::from_center_size(
                vec3(-side, TALL * 0.5, (reaches + garden::way(reaches).1) * 0.5),
                vec3(thick, TALL, reaches - garden::way(reaches).1),
            ),
            // the nook's back, in three pieces round the way down. Spec 0007:
            // one of the bookcases along it swings, and behind it the wall is
            // not there. A wall with a bookcase sized hole in it is still a
            // wall from the room, because a bookcase is standing in the hole.
            Aabb::from_center_size(
                vec3(
                    -side - NOOK_DEEP,
                    TALL * 0.5,
                    (-reaches + hole - cellar::WIDE * 0.5) * 0.5,
                ),
                vec3(thick, TALL, hole - cellar::WIDE * 0.5 + reaches),
            ),
            Aabb::from_center_size(
                vec3(
                    -side - NOOK_DEEP,
                    TALL * 0.5,
                    (hole + cellar::WIDE * 0.5 - reaches + NOOK_SPAN) * 0.5,
                ),
                vec3(
                    thick,
                    TALL,
                    -reaches + NOOK_SPAN - hole - cellar::WIDE * 0.5,
                ),
            ),
            // and the lintel over it
            Aabb::from_center_size(
                vec3(-side - NOOK_DEEP, (cellar::HIGH + TALL) * 0.5, hole),
                vec3(thick, TALL - cellar::HIGH, cellar::WIDE),
            ),
            {
                let (at, size) = nook_end(reaches);
                Aabb::from_center_size(at, size)
            },
            // the near end is not here. It is two boxes with a gap between
            // them, in `behind::near_wall`, and what is drawn is one wall
            // across the whole of it. That is the one place in this building
            // where the collider and the picture are meant to disagree, and it
            // is spec 0011.
            // and the far end, reaching across the nook too, in two pieces
            // round the way into the gallery. Spec 0022: the shapes used to
            // be on this wall and are through it now, so it has a gap in it
            // the width of the clear aisle.
            //
            // The right piece reaches past the hall's own right wall, because
            // the gallery is wider than the hall and this is its near side as
            // well as the hall's far one.
            {
                let (left, _) = gallery_way();
                let from = -side - NOOK_DEEP;

                Aabb::new(
                    vec3(from, 0.0, -reaches - thick * 0.5),
                    vec3(left, TALL, -reaches + thick * 0.5),
                )
            },
            {
                let (_, right) = gallery_way();
                let to = GALLERY_WIDE * 0.5 + thick;

                Aabb::new(
                    vec3(right, 0.0, -reaches - thick * 0.5),
                    vec3(to, TALL, -reaches + thick * 0.5),
                )
            },
            // the gallery's own three sides, per spec 0022. The west one is
            // cut for the hotel's door and is built there, per spec 0029, the
            // same way the east one is built in the back room: one account of
            // where a wall is rather than two that drift.
            crate::aleph::gallery_west(reaches)[0],
            crate::aleph::gallery_west(reaches)[1],
            // the east one is cut for the back room's door and is built
            // there, per spec 0024, so there is one account of where it is
            // rather than two that drift
            crate::back::gallery_east(reaches)[0],
            crate::back::gallery_east(reaches)[1],
            {
                let far = gallery_wall(reaches);

                Aabb::new(
                    vec3(-GALLERY_WIDE * 0.5 - thick, 0.0, far - thick),
                    vec3(GALLERY_WIDE * 0.5 + thick, TALL, far),
                )
            },
        ];

        let mut walls = walls;
        walls.extend(nook_lining(reaches));

        let benches = benches(-reaches);
        let bookcases = bookcases(-reaches);

        Self {
            stood,
            displays: crate::display::all_of_them(gallery_wall(reaches)),
            benches,
            bookcases,
            walls,
            reaches,
            open: false,
            sauna_open: false,
            sauna_swing: 0.0,
            baths_open: false,
            baths_swing: 0.0,
            hotel_doors: Vec::new(),
            swing: 0.0,
        }
    }

    /// The floor, which until spec 0007 was not a thing at all.
    ///
    /// It did not need to be. Nothing pulled you down, so the floor was the
    /// plane your feet were assumed to be on and the room was boxes standing on
    /// an idea. A stair is the first thing in the building at a height, and the
    /// moment there is a down there has to be something stopping you at the
    /// bottom of it.
    ///
    /// Its top is at nought, which is where everything else in the room already
    /// sits.
    pub fn floors(&self) -> [Aabb; 3] {
        let side = WALL + CABINET.x;
        // where the nook stops, which is where the floor under it stops too
        let shut = -self.reaches + NOOK_SPAN + THICK * 0.5;

        [
            // the hall, end to end
            Aabb::from_center_size(
                vec3(0.0, -THICK * 0.5, 0.0),
                vec3(side * 2.0, THICK, self.reaches * 2.0),
            ),
            // and the nook, which is shorter.
            //
            // One slab covering both used to be simpler and was wrong in a way
            // nothing could see until something was built out here: the nook
            // ends at `shut` and the hall runs on past it, so the slab left a
            // tongue of floor sticking out beyond the nook's end wall, outside
            // every room in the building. Spec 0010 put a garden on that
            // ground and found its own floor buried under the tongue.
            Aabb::from_center_size(
                vec3(
                    -side - NOOK_DEEP * 0.5,
                    -THICK * 0.5,
                    (shut - self.reaches) * 0.5,
                ),
                vec3(NOOK_DEEP, THICK, shut + self.reaches),
            ),
            // and the gallery across the end, per spec 0022. Its own slab
            // rather than the hall's reaching out, because it is wider than
            // the hall and a slab that covered both would stick out past the
            // hall's side walls, which is the mistake the nook's floor made.
            {
                let (at, size) = gallery(self.reaches);

                Aabb::from_center_size(vec3(at.x, -THICK * 0.5, at.z), vec3(size.x, THICK, size.z))
            },
        ]
    }

    /// The box a bench fills, which is what you cannot walk through and what
    /// the sight has to land on.
    pub fn bench_box(bench: &Benched) -> Aabb {
        Aabb::from_center_size(bench.at + Vec3::Y * bench.size.y * 0.5, bench.size)
    }

    /// The box a plinth fills, which is what holds you off the shape standing
    /// on it. A plinth is wider than the shape it carries, so stopping at the
    /// plinth stops you short of the shape as well.
    pub fn plinth_box(one: &crate::display::Display) -> Aabb {
        let (at, size) = crate::display::plinth_under(one);

        Aabb::from_center_size(at, size)
    }

    /// The two leaves as they stand right now, each as its middle, its turn and
    /// its half extents. The only things in the building that move.
    ///
    /// Turned rather than as the box they fill, because which way is out of a
    /// shelf is a question in the shelf's own frame. On the room's axes a leaf
    /// at forty five degrees fills a box half as big again as it is.
    pub fn leaves(&self) -> Vec<(Vec3, glam::Quat, Vec3)> {
        let case = crate::study::CASE;

        self.bookcases
            .iter()
            .enumerate()
            .filter(|(n, _)| cellar::swings(*n))
            .map(|(n, shelved)| {
                let (at, turn) = Self::swung(shelved, self.swings_out(n), self.swing);

                (
                    at + Vec3::Y * case.z * 0.5,
                    turn,
                    vec3(case.y * 0.5, case.z * 0.5, case.x * 0.5),
                )
            })
            .collect()
    }

    /// Where a bookcase stands and how it is turned, swung or not.
    pub fn shelf_frame(&self, n: usize) -> (Vec3, glam::Quat) {
        let Some(shelved) = self.bookcases.get(n) else {
            return (Vec3::ZERO, glam::Quat::IDENTITY);
        };
        let along = glam::Quat::from_rotation_y(if shelved.facing.x.abs() > 0.5 {
            std::f32::consts::FRAC_PI_2
        } else {
            0.0
        });

        if cellar::swings(n) {
            let (at, swing) = Self::swung(shelved, self.swings_out(n), self.swing);

            (at, swing * along)
        } else {
            (shelved.at, along)
        }
    }

    /// The one book that opens the wall, as the box it fills right now.
    ///
    /// It moves with the case it stands in, so it is still the handle when the
    /// door is open and still the handle halfway through the swing.
    pub fn handle(&self) -> Option<Aabb> {
        let case = crate::study::CASE;
        let books = crate::study::stock(cellar::CASE as u32, cellar::BOOK_SHELF);
        let book = books.get(cellar::BOOK)?;

        let along = crate::study::along(&books, cellar::BOOK) - case.x * 0.5;
        let up = crate::study::shelf_at(cellar::BOOK_SHELF);
        // the same frame the book is drawn in, from the same function, so
        // pointing at it and seeing it cannot come apart. Written out twice
        // they would be two lists of where a bookcase stands, which in this
        // room has already hung a sconce in a doorway.
        let (at, turn) = self.shelf_frame(cellar::CASE);
        let middle = at
            + turn
                * vec3(
                    along,
                    up + book.tall * 0.5,
                    -case.y * 0.5 + crate::study::BOOK_BACK + book.tall * 0.22,
                );
        let reach = (book.thick * 0.9).max(book.tall * 0.44) * 0.5;

        Some(Aabb::from_center_size(
            middle,
            vec3(reach * 2.0, book.tall, reach * 2.0),
        ))
    }

    /// Lets the door catch up with itself.
    pub fn ease(&mut self, dt: f32) {
        let to = if self.open { 1.0 } else { 0.0 };
        let by = dt / Self::SWINGS_IN;

        self.swing += (to - self.swing).clamp(-by, by);

        // the sauna's door is lighter than a bookcase with four hundred books
        // in it, so it moves at its own pace
        let to = if self.sauna_open { 1.0 } else { 0.0 };
        let by = dt / crate::spa::SAUNA_SWINGS;

        self.sauna_swing += (to - self.sauna_swing).clamp(-by, by);

        // and the door to the baths, heavier than glass and lighter than a
        // bookcase with four hundred books in it
        let to = if self.baths_open { 1.0 } else { 0.0 };
        let by = dt / crate::spa::SWINGS;

        self.baths_swing += (to - self.baths_swing).clamp(-by, by);
    }

    /// Where a leaf stands and how it is turned, part way through its swing.
    ///
    /// Hinged on its own outer end, so the two of them open away from each
    /// other and the way down is between them rather than behind one of them.
    pub fn swung(shelved: &Shelved, out: f32, swing: f32) -> (Vec3, glam::Quat) {
        let case = crate::study::CASE;
        let turn = glam::Quat::from_rotation_y(-out * swing * std::f32::consts::FRAC_PI_2);
        let hinge = vec3(
            shelved.at.x,
            shelved.at.y,
            shelved.at.z + out * case.x * 0.5,
        );

        (hinge + turn * (shelved.at - hinge), turn)
    }

    /// The box a leaf fills part way through its swing, which at nought is the
    /// box it fills standing still.
    pub fn swung_box(shelved: &Shelved, out: f32, swing: f32) -> Aabb {
        let case = crate::study::CASE;
        let (at, turn) = Self::swung(shelved, out, swing);
        let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));

        for across in [-1.0f32, 1.0] {
            for along in [-1.0f32, 1.0] {
                let corner = at + turn * vec3(across * case.y * 0.5, 0.0, along * case.x * 0.5);
                lo = lo.min(corner);
                hi = hi.max(corner);
            }
        }

        Aabb::from_center_size(
            vec3((lo.x + hi.x) * 0.5, case.z * 0.5, (lo.z + hi.z) * 0.5),
            vec3(hi.x - lo.x, case.z, hi.z - lo.z),
        )
    }

    /// Which way a leaf swings: away from the middle of the opening, so the two
    /// of them open outwards and leave the way down between them.
    pub fn swings_out(&self, n: usize) -> f32 {
        let middle = cellar::CASE as f32 + (cellar::CASES as f32 - 1.0) * 0.5;

        if (n as f32) < middle {
            -1.0
        } else {
            1.0
        }
    }

    /// The box a bookcase fills, turned to face the way it does.
    pub fn bookcase_box(shelved: &Shelved) -> Aabb {
        let case = crate::study::CASE;
        let size = if shelved.facing.x.abs() > 0.5 {
            vec3(case.y, case.z, case.x)
        } else {
            vec3(case.x, case.z, case.y)
        };

        Aabb::from_center_size(shelved.at + Vec3::Y * case.z * 0.5, size)
    }

    /// Everything you cannot walk through: the walls, the cabinets, the benches
    /// and the plinths.
    ///
    /// The plinths were the one thing in the room that let you through. You
    /// could walk into the far wall's row and stand inside a Klein bottle,
    /// which is a thing the room says you may pick up and turn.
    pub fn solid(&self) -> Vec<Aabb> {
        let mut out = self.building();
        out.push(crate::beyond::solid(self.reaches));

        out
    }

    /// The same, without the floor of spec 0018.
    ///
    /// The building, in other words, which is what `outline` is the outline
    /// of and what every measurement of the building's own size wants.
    pub fn building(&self) -> Vec<Aabb> {
        self.built(true)
    }

    /// The box the reach fill is given, which is the building's outline with
    /// the hotel's corridor pulled back out of it.
    ///
    /// **The corridor is two kilometres long so that its doors never stop**,
    /// per spec 0029, and the fill has a cell budget. Given the whole thing
    /// it goes over, refuses, and comes back empty, and an empty reach is
    /// indistinguishable from one that found nothing: two of this building's
    /// own tests said "the fill reached nothing at all" the first time that
    /// length was tried.
    ///
    /// Pulled back rather than left out of the colliders. The corridor's
    /// walls still stop you, which they must, and all that is cut is how far
    /// the fill is asked to look. What is lost is that it does not reach down
    /// the corridor, so spec 0048's coplanar narrowing does not narrow in
    /// there: forty doors of one shape repeated, whose pairs this building's
    /// own tests already hold.
    /// Only the tests ask for this, the same as `Arcade::reachable` which is
    /// the one thing that calls it. Nothing the game does while it runs fills
    /// the building.
    #[cfg(test)]
    pub fn fillable(&self) -> Aabb {
        let lobby = crate::aleph::span().0;
        // **Both corridors, not one.** The long boxes and nothing else.
        // Nothing real in this building reaches fifty metres west of the
        // lobby: the furthest is the cellar at -23.7, and the hotel's
        // corridor starts two kilometres out. An `x` clamp alone cut the
        // cellar with it, which is how that was found.
        //
        // Spec 0035 then put a second two kilometre hall off the same lobby
        // running in `z` instead, which an `x` clamp does not see at all:
        // the fill went over its cell budget, refused, and came back empty,
        // and the test that reads it says so rather than passing on nothing.
        let along = crate::twine::mouth(self.reaches);
        let reaching = lobby - 50.0;
        let reaching_z = along - 50.0;
        // **And up, which is the third of these.** Spec 0037 put a room
        // over the hall, and a fill that has to cover eight more metres of
        // height spends its cells on air and refuses.
        //
        // By which boxes they are rather than by how high they reach. A
        // clamp on `max.y` worked while the room was only a room, and then
        // the way up to it was built: a stair from the nook's floor to that
        // room's and a shaft round it, which is one thing spanning both
        // heights and cannot be cut by either. Three tests came back empty
        // at once and said so, which is the only reason this is not still a
        // number.
        let up = crate::aloft::solid();
        let rest: Vec<Aabb> = self
            .built(true)
            .into_iter()
            .filter(|box_| {
                box_.min.x > reaching
                    && box_.min.z > reaching_z
                    && !up.iter().any(|b| b.min == box_.min && b.max == box_.max)
            })
            .collect();
        let whole = Self::round(rest);

        // and a stub of each, so the fill still covers the part anybody
        // stands in on the way down
        Aabb::new(
            vec3(
                whole.min.x.min(lobby - CORRIDOR_FILLED),
                whole.min.y,
                whole.min.z.min(along - CORRIDOR_FILLED),
            ),
            whole.max,
        )
    }

    /// The same, with the rooms the sun cannot reach left out when asked.
    ///
    /// One list either way, because two lists of one thing drift.
    ///
    /// Two rooms are out of it now rather than one. The hotel of spec 0029
    /// joined the space behind the wall on 2026-10-20 for exactly the same
    /// reason, and it was caught the same way: its corridor outruns the far
    /// plane by design, which is a hundred and ten metres of building added
    /// west, and the sun's one map stretched over it until the garden's
    /// shadows went coarse. A picture test caught that before anybody looked.
    /// Every floor in this building, said once each, with the name of the room
    /// it belongs to.
    ///
    /// **Here rather than in a list of its own**, because `built` below is the
    /// one place that knows what rooms there are. A room added there and not
    /// here is a room whose ground nothing holds, and the two sitting in the
    /// same impl is the shortest distance between them that this building can
    /// offer. There is no way to make a missing room fail on its own: nothing
    /// can test for the absence of something nobody wrote down.
    ///
    /// Each entry is the floor as the walk meets it. A room whose floor is in
    /// pieces gives its pieces, because what the rules care about is where one
    /// room's ground ends and the next one's begins, and a piece's edge is as
    /// real as a whole slab's.
    ///
    /// **The field is not here.** `field::ground` re-centres on a tile grid
    /// around wherever you stand, so it is not a floor at a place: asked twice
    /// from two places it gives two different boxes, and a set that changes
    /// under you is not a set.
    /// Only the tests ask for this, the same as `fillable` above. Nothing the
    /// game does while it runs wants the floors as a set.
    #[cfg(test)]
    pub fn grounds(&self) -> Vec<(&'static str, Aabb)> {
        let reaches = self.reaches;
        let [hall, nook, gallery] = self.floors();
        let mut out = vec![
            ("the hall", hall),
            ("the nook", nook),
            ("the gallery", gallery),
            ("the cellar", crate::cellar::ground(reaches)),
            ("the garden", crate::garden::ground(reaches)),
            ("the space behind the wall", crate::behind::floor(reaches)),
            ("the back room", crate::back::floor(reaches)),
            ("the hotel's lobby", crate::aleph::floor(reaches)),
            (
                "the hotel's corridor",
                crate::aleph::corridor_floor(reaches),
            ),
            ("the other corridor", crate::twine::floor(reaches)),
            ("the register", crate::register::floor(reaches)),
            ("the space beyond the door", crate::beyond::solid(reaches)),
        ];

        out.extend(
            crate::spa::floors(reaches)
                .into_iter()
                .map(|slab| ("the baths", slab)),
        );
        out.extend(
            crate::aloft::floors()
                .into_iter()
                .map(|slab| ("the imaginarium", slab)),
        );

        out
    }

    fn built(&self, lit_by_the_sun: bool) -> Vec<Aabb> {
        let mut out = self.walls.clone();
        out.extend(self.floors());
        out.extend(crate::cellar::solid(self.reaches));
        out.extend(crate::spa::solid(self.reaches));
        out.extend(crate::garden::solid(self.reaches));
        // the hall's near end wall, which is two boxes with a gap between
        // them, and the space that gap leads into. Spec 0011.
        out.extend(crate::behind::near_wall(self.reaches));
        out.extend(crate::back::solid(self.reaches));

        if lit_by_the_sun {
            out.extend(crate::behind::solid(self.reaches));
            out.extend(crate::aleph::solid(self.reaches));
            // and the register behind the blank door, per spec 0031
            out.extend(crate::register::solid(self.reaches));
            out.extend(crate::twine::solid(self.reaches));
            out.extend(crate::aloft::solid());
        }
        out.push(crate::spa::sauna_leaf_box(self.reaches, self.sauna_swing));
        out.push(crate::spa::leaf_box(self.reaches, self.baths_swing));
        // and every leaf in the hotel's corridor, per spec 0029. A door that
        // is drawn and not solid is a door you walk through, which is what
        // the first build of that room shipped and what Jake found by
        // walking into one.
        out.extend(
            self.hotel_doors
                .iter()
                .map(|(n, open)| crate::aleph::leaf_box(self.reaches, *n, *open))
                .collect::<Vec<_>>(),
        );
        // **and the blank one**, which is not in that list because it is not
        // a room, so it had no collider at all and you walked straight
        // through it. Spec 0031, and Jake found it by doing exactly that. It
        // stands open and never moves.
        out.push(crate::aleph::leaf_box(
            self.reaches,
            crate::aleph::BLANK,
            1.0,
        ));
        out.extend(
            self.stood
                .iter()
                .map(|stood| Aabb::from_center_size(stood.at + Vec3::Y * CABINET.y * 0.5, CABINET)),
        );
        out.extend(self.benches.iter().map(Self::bench_box));
        out.extend(self.displays.iter().map(Self::plinth_box));
        out.extend(self.bookcases.iter().enumerate().map(|(n, case)| {
            if cellar::swings(n) {
                Self::swung_box(case, self.swings_out(n), self.swing)
            } else {
                Self::bookcase_box(case)
            }
        }));

        out
    }

    /// The ceiling over the hall and the nook, as the pieces it is drawn in.
    ///
    /// **With the stairwell out of it.** The grid that hangs under this is
    /// clipped to the well so the way up to the imaginarium has something
    /// to come through, and for a while this was not: the hole was only
    /// ever cut in the thing hanging under the hole, so looking up the
    /// stair was a flat lid over the whole opening. Spec 0038.
    ///
    /// Named rather than worked out where it is drawn, because the test
    /// that says the hole is open has to read the same list the picture
    /// does.
    /// Every lid this building shows the sky, which is what a roof goes on
    /// and what the walls under it hang from.
    ///
    /// The hall, the gallery and the nook share one, cut for the stairwell,
    /// and the three rooms that adjoin at that level each have their own at
    /// their own height. Spec 0015's is not here: it is a hundred and fifty
    /// nine metres square and it is a secret.
    pub fn lids(&self) -> Vec<Aabb> {
        let mut out = self.ceiling();

        out.push(crate::aleph::lid(self.reaches));
        out.push(crate::register::lid(self.reaches));
        out.push(crate::back::lid(self.reaches));
        out
    }

    /// Whether you are out of doors, which is what the ground, the sky and
    /// the far plane are all drawn on.
    ///
    /// **`field::in_view` is a walker's question and this is the flyer's as
    /// well.** That one asks whether the field's ground is what is holding
    /// you up, which is exactly right on foot and false of a body that is
    /// not standing on anything: forty metres over the hall it said you were
    /// indoors and the ground, the sky and the far plane went with it, which
    /// is spec 0041 flying into a black that nothing could get you out of.
    ///
    /// So it is either question. The field under your feet, or no roof over
    /// your head. Both, because neither alone is enough: the imaginarium
    /// oversails the hall by a metre, so there is grass at the building's
    /// east wall with a storey hanging five metres above it, and a body
    /// standing there is out of doors by every test but the roof.
    ///
    /// The lids as well as the colliders, because a suspended ceiling is
    /// drawn and never collided and the hall's roof is in no list `solid`
    /// keeps.
    pub fn out_of_doors(&self, at: Vec3) -> bool {
        let solid = self.solid();

        if crate::field::in_view(self.reaches, &solid, at) {
            return true;
        }

        let over = |box_: &Aabb| {
            box_.min.x <= at.x
                && box_.max.x >= at.x
                && box_.min.z <= at.z
                && box_.max.z >= at.z
                && box_.max.y >= at.y
        };

        !solid.iter().any(&over) && !self.lids().iter().any(&over)
    }

    /// And the faces under those lids that the sky can see, as (face, which
    /// way it looks), which is what plaster goes on.
    ///
    /// **Asked of what is over the spot rather than worked out from the
    /// plan.** The ground storey is five rooms of four different heights
    /// abutting along six edges, and a list of which faces are outward is a
    /// list that is wrong the day a room moves. It is also the fault under
    /// most of the others in here: two accounts of one thing drift.
    ///
    /// The question each column of each face is asked is whether anything of
    /// this building stands over the spot a hand's breadth outside it. It
    /// has to be that question and not whether anything is in the way: the
    /// hall's lid is eight pieces and the air a hand's breadth past the
    /// edge of one of them is the air under the next, which the first build
    /// called outdoors and plastered. Thirty of the sixty six faces it found
    /// were inside the gallery.
    ///
    /// The garden is left out, not because it is indoors but because it is
    /// dressed already: `garden::facing` lines the hall's wall from the
    /// garden's side, and a second skin sixteen centimetres in front of that
    /// one is two walls where there is one.
    ///
    /// Asking it this way is also how the strip over a borrowed wall gets
    /// plastered. The register is a metre taller than the lobby it abuts,
    /// and a face that is outward only above its neighbour is exactly what
    /// this building has twice left open and twice failed to notice.
    pub fn outside(&self) -> Vec<(Aabb, Vec3)> {
        let lids = self.lids();
        // **The lids as well as the colliders.** A suspended ceiling is
        // drawn and never collided, so the hall's lid is in no list `solid`
        // keeps, and asked of the colliders alone the whole south end of
        // the gallery reads as open sky.
        let mut solid = self.solid();
        solid.extend(lids.iter().copied());

        let (step, rise, out_by) = (0.4, 0.3, 0.12);
        let reaches = self.reaches;
        let over = |at: Vec3| {
            solid.iter().any(|box_| {
                box_.min.x <= at.x
                    && box_.max.x >= at.x
                    && box_.min.z <= at.z
                    && box_.max.z >= at.z
                    && box_.max.y >= at.y
            })
        };
        let mut out = Vec::new();

        for lid in lids.iter().copied() {
            let top = lid.max.y;

            for looks in [-Vec3::X, Vec3::X, -Vec3::Z, Vec3::Z] {
                let along = if looks.x.abs() > 0.5 { 2 } else { 0 };
                let (from, to) = (lid.min.to_array()[along], lid.max.to_array()[along]);
                let flat = if along == 2 { 0 } else { 2 };
                let edge = if looks.to_array()[flat] > 0.0 {
                    lid.max.to_array()[flat]
                } else {
                    lid.min.to_array()[flat]
                };

                // **Where the wall's outer face is, which is not the lid's
                // edge.** Some lids oversail their walls by a thickness and
                // some are the room's inside, so a plane taken off the lid
                // is sometimes the outside of the building and sometimes
                // buried in it. Probed along the normal from the middle of
                // the face: the wall is whatever the line runs through, and
                // its outer bound is the plane the plaster goes on.
                let mut on = edge;

                {
                    let (middle, half) = ((from + to) * 0.5, top * 0.5);
                    let mut d = -0.05;

                    while d < 0.8 {
                        let at = edge + looks.to_array()[flat] * d;
                        let probe = if along == 2 {
                            vec3(at, half, middle)
                        } else {
                            vec3(middle, half, at)
                        };

                        if let Some(box_) = solid.iter().find(|box_| box_.contains_point(probe)) {
                            on = if looks.to_array()[flat] > 0.0 {
                                box_.max.to_array()[flat].max(on)
                            } else {
                                box_.min.to_array()[flat].min(on)
                            };
                        }

                        d += 0.02;
                    }
                }

                // the lowest height each column of this face is open from,
                // or nothing if it is roofed all the way up
                let columns = ((to - from) / step).ceil().max(1.0) as usize;
                let mut opens: Vec<Option<f32>> = Vec::with_capacity(columns);

                for n in 0..columns {
                    let at = from + (n as f32 + 0.5) * (to - from) / columns as f32;
                    let mut lowest = None;
                    let mut up = top - rise * 0.5;

                    while up > 0.0 {
                        let probe = if along == 2 {
                            vec3(on + looks.x * out_by, up, at)
                        } else {
                            vec3(at, up, on + looks.z * out_by)
                        };

                        if over(probe)
                            || crate::garden::under_the_sky(reaches, probe)
                            || solid.iter().any(|box_| box_.contains_point(probe))
                        {
                            break;
                        }

                        lowest = Some(up - rise * 0.5);
                        up -= rise;
                    }

                    opens.push(lowest.map(|low| low.max(0.0)));
                }

                // and the runs of columns that open from the same height
                let mut n = 0;

                while n < columns {
                    let Some(low) = opens[n] else {
                        n += 1;
                        continue;
                    };
                    let mut end = n + 1;

                    while end < columns
                        && opens[end].map(|other| (other - low).abs() < 1e-3) == Some(true)
                    {
                        end += 1;
                    }

                    let span = (to - from) / columns as f32;
                    let (one, two) = (from + n as f32 * span, from + end as f32 * span);

                    out.push((
                        if along == 2 {
                            Aabb::new(vec3(on, low, one), vec3(on, top, two))
                        } else {
                            Aabb::new(vec3(one, low, on), vec3(two, top, on))
                        },
                        looks,
                    ));

                    n = end;
                }
            }
        }

        out
    }

    pub fn ceiling(&self) -> Vec<Aabb> {
        let well = crate::aloft::up_well();
        let mut out = Vec::new();

        for slab in self.floors().iter() {
            let mut pieces = vec![Aabb::new(
                vec3(slab.min.x, TALL - 0.01, slab.min.z),
                vec3(slab.max.x, TALL + 0.01, slab.max.z),
            )];

            for open in well.iter() {
                pieces = pieces
                    .into_iter()
                    .flat_map(|piece| minus(piece, *open))
                    .collect();
            }

            out.extend(pieces);
        }

        out
    }

    /// The grid over the lid: the bars, and the tile sunk behind each square
    /// of them. Spec 0012.
    ///
    /// Laid to the world rather than to each piece of the lid. The floor is
    /// an L and the ceiling is one quad per piece of it, so a grid taken from
    /// each piece's own corner has the two meeting out of step along the
    /// join, which is the join you walk through. Snapped to the world they
    /// are one grid with a wall standing in it.
    ///
    /// Each piece carries a trim flush inside its own edge as well, which is
    /// what a real grid has where it meets a wall. Flush inside rather than
    /// centred on the edge: centred, the two pieces' trims at the join would
    /// be the same box drawn twice, and two of those is the oldest fault in
    /// this building.
    pub fn lattice(&self) -> (Vec<Hung>, Vec<Hung>) {
        let (mut bars, mut tiles) = (Vec::new(), Vec::new());

        // the world's own lines that fall inside a run, with the run's two
        // ends for the trim. A line too near an end is dropped rather than
        // left to make a sliver of a tile nobody can see.
        let lines = |from: f32, to: f32| {
            let mut out = vec![from + CEIL_BAR * 0.5];
            let first = (from / CEIL_TILE).ceil() as i32;
            let last = (to / CEIL_TILE).floor() as i32;

            out.extend(
                (first..=last)
                    .map(|n| n as f32 * CEIL_TILE)
                    .filter(|on| *on > from + CEIL_BAR * 2.0 && *on < to - CEIL_BAR * 2.0),
            );
            out.push(to - CEIL_BAR * 0.5);

            out
        };

        for slab in self.floors().iter() {
            let (down, across) = (lines(slab.min.x, slab.max.x), lines(slab.min.z, slab.max.z));
            let (deep, span) = (slab.size().x, slab.size().z);
            let (middle_x, middle_z) = (slab.center().x, slab.center().z);

            for on in down.iter() {
                bars.push((
                    vec3(*on, TALL - CEIL_DROP * 0.5, middle_z),
                    vec3(CEIL_BAR, CEIL_DROP, span),
                ));
            }
            for on in across.iter() {
                bars.push((
                    vec3(middle_x, TALL - CEIL_DROP * 0.5, *on),
                    vec3(deep, CEIL_DROP, CEIL_BAR),
                ));
            }

            // and a tile behind every square, sunk above the bars so that
            // nothing up there is level with anything
            for (near_x, far_x) in down.iter().zip(down.iter().skip(1)) {
                for (near_z, far_z) in across.iter().zip(across.iter().skip(1)) {
                    tiles.push((
                        vec3(
                            (near_x + far_x) * 0.5,
                            tile_middle(),
                            (near_z + far_z) * 0.5,
                        ),
                        vec3(
                            far_x - near_x - CEIL_BAR,
                            CEIL_TILE_THICK,
                            far_z - near_z - CEIL_BAR,
                        ),
                    ));
                }
            }
        }

        // and the stairwell comes out of it. The nook's ceiling is the one
        // the way up to the imaginarium goes through, per spec 0037, so each
        // bar and each tile is clipped to the well rather than dropped: the
        // well is cut to the stair and a tile dropped whole leaves the hole
        // wider than the shaft wall that is meant to edge it.
        let well = crate::aloft::up_well();
        let clipped = |hung: &Vec<Hung>| {
            hung.iter()
                .flat_map(|(at, size)| {
                    let mut pieces = vec![Aabb::from_center_size(*at, *size)];

                    for open in well.iter() {
                        pieces = pieces.into_iter().flat_map(|p| minus(p, *open)).collect();
                    }

                    pieces
                        .into_iter()
                        .map(|p| (p.center(), p.size()))
                        .collect::<Vec<Hung>>()
                })
                .collect()
        };
        bars = clipped(&bars);
        tiles = clipped(&tiles);

        (bars, tiles)
    }

    /// A box round the whole building, every room of it.
    ///
    /// This is what the sun's shadow map is fitted to. Outside that map a
    /// fragment is lit and nothing casts, which is the engine's rule and the
    /// forgiving one for a world bigger than its map, so every room that has
    /// a roof over it has to be inside the map or the sun comes through the
    /// roof.
    ///
    /// Fitted to the hall alone it was sharp, and three rooms were outside
    /// it: the cellar and the baths are under the hall and below the box, and
    /// the space behind the wall runs forty-four by fifty-two past the end of
    /// it. All three were taking full sun through solid ground. It showed as
    /// a warm wedge on the back of the hall's end wall, the first thing you
    /// see on turning round in a room that is meant to be lit by nothing but
    /// its own six lamps.
    ///
    /// The whole building is nine times the hall across, so a texel of that
    /// map goes from eight millimetres to thirty-six. Nothing is lost by it:
    /// the hall and the nook are roofed, the sun reaches neither, and a
    /// measurement of both with the sun at full and at nought came back the
    /// same to three places. The garden is the one room the sun is for, and
    /// thirty-six millimetres is finer than anything it has to draw the edge
    /// of.
    /// **The space of spec 0018 is not in here**, which is the one thing this
    /// box leaves out and the reason it is worth saying so. That floor runs
    /// 33 kilometres: in the outline it takes the shadow map's texel from
    /// thirty-six millimetres to seventy metres, which is no shadow anywhere
    /// in the building, and it takes spec 0049's reach fill past the engine's
    /// cap of 32 million cells, which is no fill at all and a test that
    /// quietly proves nothing. The building is what this is the outline of.
    /// Test only since the sun stopped using it: what the game needs is
    /// [`Room::under_the_sun`], and what a walk needs is the whole building.
    #[cfg(test)]
    pub fn outline(&self) -> Aabb {
        Self::round(self.building())
    }

    /// What the sun has to cover, which is the building without the rooms the
    /// sun cannot reach.
    ///
    /// The sun gets one shadow map over whatever it is given, so what it is
    /// given decides how coarse a texel is. Given the whole building it is
    /// given the solar system too, per spec 0015: a hundred and eighty eight
    /// metres across for a building of forty five, because Neptune is seventy
    /// nine metres out and the walk to the sun is a hundred and eleven. Nine
    /// tenths of the map went on a room with no sky in it, a texel came to
    /// nine centimetres, which is coarser than these walls are thick, and
    /// daylight came through them into the baths in bands.
    ///
    /// Nothing is lost by leaving that room out. It is lit by its own lamps
    /// and by planets that glow, and the frame is identical pixel for pixel
    /// with it in the bounds and with it out.
    ///
    /// The hotel of spec 0029 is out for the same reason and was a worse
    /// case: a corridor built to outrun the far plane is a hundred and ten
    /// metres of indoor building, and with it in the bounds the garden's
    /// shadows moved in half a per cent of that room's pixels.
    pub fn under_the_sun(&self) -> Aabb {
        Self::round(self.built(false))
    }

    fn round(boxes: Vec<Aabb>) -> Aabb {
        let mut lo = Vec3::splat(f32::MAX);
        let mut hi = Vec3::splat(f32::MIN);

        for box_ in boxes {
            lo = lo.min(box_.min);
            hi = hi.max(box_.max);
        }

        Aabb::new(lo, hi)
    }

    /// What the sight cannot see through.
    ///
    /// The walls, and only the walls. Not the benches, the cabinets or the
    /// bookcases: those are the things being looked at, and a bookcase that
    /// blocks the sight blocks the one book in it that is a handle.
    pub fn opaque(&self) -> Vec<Aabb> {
        let mut out = self.walls.clone();

        // **and the hotel's and the register's.** Those two rooms were built
        // after this and their walls were never in the list, so the sight
        // went straight through them: standing in the register you could
        // point at the wall and the building named a corridor door behind
        // it, "Room 8 has somebody in it". Jake walked it and found that.
        out.extend(crate::aleph::solid(self.reaches));
        out.extend(crate::register::solid(self.reaches));
        // and spec 0035's, the other corridor off that lobby
        out.extend(crate::twine::solid(self.reaches));
        out.extend(crate::aloft::solid());

        out
    }

    /// What you are looking at: the nearest thing the line of sight meets, said
    /// as a cabinet or as one of the shapes on the far wall, so the room reads
    /// one way throughout.
    ///
    /// Pointing rather than standing. It was "the nearest one in front of you,
    /// within reach and nearly enough faced", which meant walking the aisle to
    /// find out what anything was. With the cursor held by the window there is
    /// no pointer but the middle of the screen, so looking at a thing is
    /// pointing at it, and a click is a click on it.
    ///
    /// There was a `looked_at` beside this that answered cabinets only. Spec
    /// 0004 gave the shapes something a click does, so every caller wants both
    /// and the room is read one way.
    pub fn looking_at(&self, from: Vec3, way: Vec3) -> Option<Seen> {
        let way = way.normalize_or_zero();
        if way == Vec3::ZERO {
            return None;
        }

        let cabinets = self.stood.iter().enumerate().filter_map(|(n, stood)| {
            let box_ = Aabb::from_center_size(stood.at + Vec3::Y * CABINET.y * 0.5, CABINET);
            slab(from, way, &box_).map(|far| (Seen::Cabinet(n), far))
        });

        let shapes = self.displays.iter().enumerate().filter_map(|(n, one)| {
            let box_ = Aabb::from_center_size(one.at, Vec3::splat(one.scale.max(0.4)));
            slab(from, way, &box_).map(|far| (Seen::Display(n), far))
        });

        // **A bench reaches less far than a cabinet does.** `REACH` is four
        // metres because the hall needs it: standing by one cabinet you work
        // the one opposite, and `WALL`'s own note says the aisle cannot
        // change width without that breaking. A bench is not that. It is a
        // thing you walk up to, six of them stand in a row down one side of
        // the nook, and at four metres almost any way you face in that room
        // has one in it.
        //
        // What that costs is your arrow keys. The arrows work the toy the
        // sight is on and turn you when it is on nothing, so crossing the
        // nook they stop turning you, for no reason you can see, and the
        // toy reacting is behind you. Jake walked into it by the way up and
        // said he loses control of the arrow keys near the stairs. The
        // stairs are not it: a probe says the nook's floor has done this
        // since the day it had toys on it, and the stair only gave him a
        // reason to cross the room holding an arrow down.
        let benches = self.benches.iter().enumerate().filter_map(|(n, one)| {
            slab(from, way, &Self::bench_box(one))
                .filter(|far| *far <= BENCH_REACH)
                .map(|far| (Seen::Bench(n), far))
        });

        // and the one book that is not a book, which is a case sized thing
        // because the book is the handle and the case is the door. Pointing at
        // a single spine on a shelf of four hundred is a thing nobody would
        // ever find; pointing at the case it is in is a thing you can.
        let case = self
            .handle()
            .and_then(|book| slab(from, way, &book).map(|far| (Seen::Case, far)));

        // and the one panel in the gallery, per spec 0039, which is the
        // only flat thing in this building you work
        let panel = {
            let (at, size) = crate::mandel::panel(self.reaches);

            slab(from, way, &Aabb::from_center_size(at, size)).map(|far| (Seen::Mandel, far))
        };

        let glass = {
            let leaf = crate::spa::sauna_leaf_box(self.reaches, self.sauna_swing);

            slab(from, way, &leaf).map(|far| (Seen::Sauna, far))
        };
        let baths = {
            let leaf = crate::spa::leaf_box(self.reaches, self.baths_swing);

            slab(from, way, &leaf).map(|far| (Seen::Baths, far))
        };

        // the desk, which is the one thing in the hotel you work. Spec 0030:
        // Jake walked the corridor and said there is no obvious way to check
        // a customer in, and there was not, because the only thing you could
        // point at was a door.
        let desk = slab(from, way, &crate::aleph::desk(self.reaches)).map(|far| (Seen::Desk, far));

        // the book on the register's far wall, per spec 0031
        let book = {
            // **Only where the book is written**, not the whole wall. It was
            // the wall, so pointing at a bare stretch of it still said "The
            // register", which Jake found by looking at nothing and being
            // told what it was.
            let (back, _) = crate::register::across(self.reaches);
            let left = crate::register::page_left(self.reaches);
            let cell = crate::register::CELL;
            let low = crate::register::cell_at(self.reaches, crate::register::ROWS - 1, 0);
            let high = crate::register::cell_at(self.reaches, 0, 0);
            let wall = Aabb::new(
                vec3(left - cell * 2.0, low.y - cell, back),
                vec3(
                    left + crate::register::columns(self.reaches) as f32 * cell,
                    high.y + cell * 2.5,
                    back + THICK,
                ),
            );

            slab(from, way, &wall).map(|far| (Seen::Book, far))
        };

        // and the hotel's doors, per spec 0029. Every drawn one, because a
        // corridor where only the near few can be worked is a corridor with
        // an invisible edge in it.
        let hotel: Vec<(Seen, f32)> = self
            .hotel_doors
            .iter()
            .filter_map(|(n, open)| {
                let leaf = crate::aleph::leaf_box(self.reaches, *n, *open);

                slab(from, way, &leaf).map(|far| (Seen::Hotel(*n), far))
            })
            .collect();

        // and, once it is moving, the leaves themselves. Hunting for the one
        // book again to shut the wall is the same needle in the same haystack,
        // and there is nothing secret left to keep: the hole is standing open.
        // Only while it is off its stop, so a shut wall is still the one book.
        let swung = self
            .bookcases
            .iter()
            .enumerate()
            .filter(|(n, _)| self.swing > 0.0 && cellar::swings(*n))
            .filter_map(|(n, shelved)| {
                let leaf = Self::swung_box(shelved, self.swings_out(n), self.swing);

                slab(from, way, &leaf).map(|far| (Seen::Case, far))
            });

        // and what is in the way, because until now nothing was. The sight took
        // the nearest thing the ray met among the things you can use and a wall
        // was never one of them, so from the nook you could point through the
        // back of a cabinet in the hall and the room offered to play it.
        let through = self
            .opaque()
            .into_iter()
            .filter_map(|box_| slab(from, way, &box_))
            .fold(f32::INFINITY, f32::min);

        cabinets
            .chain(shapes)
            .chain(benches)
            .chain(panel)
            .chain(case)
            .chain(glass)
            .chain(baths)
            .chain(desk)
            .chain(book)
            .chain(hotel)
            .chain(swung)
            .filter(|(_, far)| *far <= REACH && *far <= through + 1e-3)
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(what, _)| what)
    }

    /// Where you start: the middle of the room, looking down it.
    pub fn doorway(&self) -> Vec3 {
        vec3(0.0, 0.0, self.reaches - INSIDE)
    }
}

#[cfg(test)]
mod tests {
    /// Spec 0029: the gallery has a doorway either side, and they are a pair.
    ///
    /// The back room's is spec 0024's and the hotel's is spec 0029's, and
    /// the second takes its `z` from the first rather than keeping its own
    /// account. Two lists of one thing drift, and a pair that drifts is two
    /// doors in a room rather than a choice.
    #[test]
    fn the_gallery_has_a_doorway_either_side() {
        let reaches = Room::of(crate::games()).reaches;
        let east = crate::back::way(reaches);
        let west = crate::aleph::way(reaches);

        assert_eq!(east, west, "the two doorways are at different z");

        // and each wall is in two pieces with the gap between them
        for (name, pieces) in [
            ("east", crate::back::gallery_east(reaches)),
            ("west", crate::aleph::gallery_west(reaches)),
        ] {
            assert_eq!(pieces.len(), 2, "the {} wall is not cut for a door", name);

            let (from, to) = east;
            let gap = pieces[0].max.z.max(pieces[1].min.z) - pieces[0].max.z.min(pieces[1].min.z);

            assert!(
                (gap - (to - from)).abs() < 1e-3,
                "the {} wall's gap is {} and the doorway is {}",
                name,
                gap,
                to - from
            );
        }
    }

    /// Spec 0015: the sun's bounds leave out the space behind the wall.
    ///
    /// The sun gets one shadow map over whatever it is given, so a room with
    /// no sky in it spends that map's resolution for nothing. Given the whole
    /// building, nine tenths of it went on the solar system and a texel came
    /// to nine centimetres, coarser than these walls are thick, and daylight
    /// came through them into the baths in bands.
    ///
    /// Both halves matter. It has to lose the hidden room, and it has to keep
    /// every room that can see the sky.
    #[test]
    fn the_sun_is_not_asked_to_cover_the_solar_system() {
        let room = Room::of(some(12));
        let all = room.outline();
        let sunlit = room.under_the_sun();

        let wide = (all.max - all.min).max_element();
        let near = (sunlit.max - sunlit.min).max_element();

        assert!(
            near < wide * 0.5,
            "the sun covers {:.1} of the building's {:.1}",
            near,
            wide
        );

        for (boxes, name) in [
            (crate::garden::solid(room.reaches), "the garden"),
            (crate::spa::solid(room.reaches), "the baths"),
            (crate::cellar::solid(room.reaches), "the cellar"),
        ] {
            for one in boxes {
                assert!(
                    sunlit.min.cmple(one.max).all() && sunlit.max.cmpge(one.min).all(),
                    "{} is outside what the sun covers",
                    name
                );
            }
        }
    }

    use super::*;
    use std::path::Path;

    /// Whether two boxes share any room at all.
    fn overlapping(one: &Aabb, other: &Aabb) -> bool {
        (0..3).all(|n| one.min[n] < other.max[n] - 1e-4 && other.min[n] < one.max[n] - 1e-4)
    }

    /// Which cabinet the sight is on, for the tests that are about cabinets.
    fn cabinet_at(room: &Room, from: Vec3, way: Vec3) -> Option<usize> {
        match room.looking_at(from, way) {
            Some(Seen::Cabinet(n)) => Some(n),
            _ => None,
        }
    }

    fn some(count: usize) -> Vec<Cabinet> {
        (0..count)
            .map(|n| Cabinet::found(&format!("game{}", n), Path::new("/nowhere")))
            .collect()
    }

    /// Spec 0012: the grid over the lid is laid to the world.
    ///
    /// The floor is an L and the ceiling is drawn one quad per piece of it.
    /// Taken from each piece's own corner the two meet out of step along the
    /// join, which is the join you walk through. Taken off the world they are
    /// one grid with a wall standing in it, whatever shape the pieces are and
    /// wherever a later room puts its own.
    ///
    /// Asked of the lines rather than of the two pieces against each other,
    /// because these two happen to start at the same z and would agree there
    /// by luck however the grid was taken. A test that passes by luck was
    /// written first and is why this one says what it says.
    #[test]
    fn the_lid_is_one_grid_laid_to_the_world() {
        let room = Room::of(some(12));
        let floors = room.floors();
        let (bars, _) = room.lattice();

        // where a trim is: flush inside each piece's own four edges
        let mut trims = Vec::new();
        for slab in floors.iter() {
            for (near, far) in [(slab.min.x, slab.max.x), (slab.min.z, slab.max.z)] {
                trims.push(near + CEIL_BAR * 0.5);
                trims.push(far - CEIL_BAR * 0.5);
            }
        }

        let mut lines = 0;
        for (at, size) in bars.iter() {
            let on = if size.x > size.z { at.z } else { at.x };

            if trims.iter().any(|edge| (edge - on).abs() < 1e-3) {
                continue;
            }

            let off = on / CEIL_TILE;
            assert!(
                (off - off.round()).abs() < 1e-3,
                "a bar at {:.3} is neither a trim nor on the world's grid: it \
                 is {:.3} tiles out, so it was laid to its own piece of the lid",
                on,
                off
            );
            lines += 1;
        }

        assert!(lines > 8, "only {} lines to speak of", lines);
    }

    /// Spec 0012: no tile is level with the bar beside it.
    ///
    /// A suspended ceiling is a lattice with the tiles dropped onto it from
    /// above, so the bar's underside is the lowest thing up there. Laid
    /// level instead they are two faces in one plane, which is the fault
    /// this building has fought seven times.
    #[test]
    fn no_ceiling_tile_is_level_with_a_bar() {
        let room = Room::of(some(12));
        let (bars, tiles) = room.lattice();

        assert!(!bars.is_empty() && !tiles.is_empty(), "no grid to speak of");

        for (at, size) in tiles.iter() {
            let under = at.y - size.y * 0.5;

            for (bar, bar_size) in bars.iter() {
                let below = bar.y - bar_size.y * 0.5;

                assert!(
                    under > below + 1e-3,
                    "a tile's underside at {:.4} is not above the bar's at \
                     {:.4}",
                    under,
                    below
                );

                // and nothing of one is over anything of the other
                let apart = (at.x - bar.x).abs() > (size.x + bar_size.x) * 0.5 - 1e-4
                    || (at.z - bar.z).abs() > (size.z + bar_size.z) * 0.5 - 1e-4;
                assert!(
                    apart,
                    "a tile at {:?} lies over a bar at {:?}",
                    at.truncate(),
                    bar.truncate()
                );
            }
        }
    }

    /// The sun's map covers every room that has a roof on it.
    ///
    /// Outside the map a fragment is lit and nothing casts, so a roofed room
    /// outside it takes the sun straight through its roof. Fitted to the hall
    /// alone, the map left the cellar and the baths under the box and the
    /// space behind the wall running past the end of it, and all three were
    /// taking full sun through solid ground.
    ///
    /// The garden is not in this list and must not be: it is the one room the
    /// sun is for, and it is roofless.
    #[test]
    fn the_suns_map_covers_every_roofed_room() {
        let room = Room::of(some(12));
        let r = room.reaches;
        let map = room.outline();

        for (what, at) in [
            ("the hall", vec3(0.0, 1.0, 0.0)),
            ("the nook", nook_floor(r).0 + Vec3::Y),
            ("the cellar", crate::cellar::floor(r).0 + Vec3::Y),
            ("the baths", crate::spa::at(r) + Vec3::Y),
            ("the space behind the wall", crate::behind::at(r) + Vec3::Y),
        ] {
            assert!(
                map.contains_point(at),
                "{what} stands at {at:?}, outside the sun's map of {:?} .. {:?}, \
                 so the sun comes through its roof",
                map.min,
                map.max
            );
        }
    }

    /// Spec 0001: there is a cabinet for every game the project has.
    #[test]
    fn every_game_gets_a_cabinet() {
        for count in [1usize, 2, 7, 12, 13] {
            let room = Room::of(some(count));

            assert_eq!(
                room.stood.len(),
                count,
                "{} games got {} cabinets",
                count,
                room.stood.len()
            );
            for (n, stood) in room.stood.iter().enumerate() {
                assert_eq!(stood.cabinet.name, format!("game{}", n));
            }
        }
    }

    /// Spec 0001: and no two share a place, nor sit inside a wall.
    #[test]
    fn the_cabinets_all_fit() {
        let room = Room::of(some(12));

        for (n, one) in room.stood.iter().enumerate() {
            for other in &room.stood[n + 1..] {
                let apart = one.at.distance(other.at);
                assert!(
                    apart > CABINET.z,
                    "two cabinets are {} apart and are {} wide",
                    apart,
                    CABINET.z
                );
            }

            // and inside the room, not through an end
            assert!(
                one.at.z.abs() < room.reaches - CABINET.z * 0.5,
                "a cabinet at {} is through the end wall at {}",
                one.at.z,
                room.reaches
            );
        }
    }

    /// Spec 0001: the one you are looking at is the nearest the line of sight
    /// meets.
    #[test]
    fn the_one_you_are_looking_at_is_the_one() {
        let room = Room::of(some(12));
        let first = room.stood[0].at;
        let eye = Vec3::Y * 1.55;

        // in front of it, looking at it
        let you = first - vec3(1.4, 0.0, 0.0) + eye;
        assert_eq!(cabinet_at(&room, you, vec3(1.0, 0.0, 0.0)), Some(0));

        // the same spot, looking the other way: the opposite row, not nothing.
        // Which is the change. Standing somewhere no longer decides anything.
        let across = cabinet_at(&room, you, vec3(-1.0, 0.0, 0.0));
        assert!(
            across.is_some_and(|n| room.stood[n].at.x < 0.0),
            "looking across the aisle found {:?}",
            across
        );

        // and up the aisle at nothing
        assert_eq!(cabinet_at(&room, Vec3::Y * 1.55, vec3(0.0, 0.0, 1.0)), None);

        // and from the far end of the aisle, looking across at it, which the
        // old rule could not reach
        let along = vec3(0.0, 0.0, first.z + 4.0) + eye;
        assert_eq!(cabinet_at(&room, along, first + eye - along), Some(0));

        // the near one wins when two are in line. From the middle of the aisle
        // rather than from behind the first one: a wall stops the sight now, so
        // standing outside the room and looking in through it finds nothing,
        // which is the whole of the fix and was how this test used to stand.
        let behind = room
            .stood
            .iter()
            .position(|s| s.at.x < 0.0)
            .expect("a far side");
        let in_line = vec3(0.0, eye.y, first.z);
        assert_eq!(cabinet_at(&room, in_line, Vec3::X), Some(0));
        assert!(
            cabinet_at(&room, in_line, Vec3::NEG_X).is_some_and(|n| room.stood[n].at.x < 0.0),
            "the far side is not where {} is",
            behind
        );
    }

    /// Spec 0005: the floor repeats its carpet rather than stretching one tile
    /// over thirty units.
    #[test]
    fn the_floor_tiles_its_carpet() {
        let floor = tiled_floor(8.0);
        let widest = floor
            .vertices
            .iter()
            .map(|v| v.uv[0].max(v.uv[1]))
            .fold(0.0f32, f32::max);

        assert!(
            widest > 1.0,
            "the carpet is stretched, not tiled: u reaches {}",
            widest
        );
        assert_eq!(widest, 8.0, "it does not tile the number it was asked for");
    }

    /// Spec 0004: a screen reads the right way round from the front and
    /// backwards from behind, like a lit sign seen through itself.
    ///
    /// Two faces carrying the same u. Giving the back its own u is what makes
    /// a sign read forwards from behind, which is two signs back to back.
    #[test]
    fn a_screen_reads_backwards_from_behind() {
        let mesh = screen_mesh();
        let (front, back) = mesh.vertices.split_at(4);

        assert_eq!(mesh.vertices.len(), 8, "a two faced quad is eight corners");
        for (f, b) in front.iter().zip(back) {
            assert_eq!(f.position, b.position, "the faces are not the same quad");
            assert_eq!(f.uv, b.uv, "the back has its own u, so it reads forwards");
            assert_eq!(f.normal, [1.0, 0.0, 0.0]);
            assert_eq!(b.normal, [-1.0, 0.0, 0.0]);
        }

        // and each face is wound to be drawn from its own side. Seen from +x
        // the screen's right runs towards -z and a triangle has to come out
        // counter clockwise there, or the opaque pass culls it; from -x it is
        // the other way about.
        let corner = |n: usize| {
            let at = mesh.vertices[mesh.indices[n] as usize].position;
            glam::vec2(-at[2], at[1])
        };
        for (triangle, from_the_front) in [(0usize, true), (3, true), (6, false), (9, false)] {
            let (a, b, c) = (corner(triangle), corner(triangle + 1), corner(triangle + 2));
            let turning = (b - a).perp_dot(c - b);

            assert!(
                if from_the_front {
                    turning > 0.0
                } else {
                    turning < 0.0
                },
                "a triangle winds the wrong way and is culled: {}",
                turning
            );
        }

        // and turning it by a cabinet's facing points the front into the aisle
        for facing in [vec3(1.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0)] {
            let looks = turned_to(facing) * Vec3::X;

            assert!(
                looks.dot(facing) > 0.99,
                "turned to {:?} the face looks {:?}",
                facing,
                looks
            );
        }
    }

    /// Spec 0003: the sight does not carry the length of the room.
    ///
    /// From the doorway it named a shape on the far wall and offered to turn
    /// it, which is a thing to do to something you cannot see.
    #[test]
    fn the_sight_does_not_reach_across_the_room() {
        let room = Room::of(some(12));
        let eye = room.doorway() + Vec3::Y * 1.55;
        let shape = room.displays[2].at;

        assert!(
            eye.distance(shape) > REACH,
            "the room is too small for this test to mean anything"
        );
        assert_eq!(
            room.looking_at(eye, shape - eye),
            None,
            "the doorway can still see the far wall"
        );

        // and the same one is seen from a couple of units away
        let near = shape + Vec3::Z * 2.0;
        assert_eq!(
            room.looking_at(near, shape - near),
            Some(Seen::Display(2)),
            "it cannot be seen from two units away either"
        );
    }

    /// Spec 0002: looking at a display names it, and a cabinet when it is a
    /// cabinet.
    #[test]
    fn looking_at_a_display_names_it() {
        let room = Room::of(some(12));
        let eye = Vec3::Y * 1.55;
        let one = room.displays[2];

        // down the room at it, from the middle of the aisle
        let you = vec3(one.at.x, 0.0, one.at.z + 4.0) + eye;
        assert_eq!(
            room.looking_at(you, one.at - you),
            Some(Seen::Display(2)),
            "the shapes on the wall are not pickable"
        );
        assert_eq!(
            cabinet_at(&room, you, one.at - you),
            None,
            "a shape read as a cabinet"
        );

        // and a cabinet still reads as a cabinet
        let first = room.stood[0].at;
        let at_it = first - vec3(1.4, 0.0, 0.0) + eye;
        assert_eq!(
            room.looking_at(at_it, vec3(1.0, 0.0, 0.0)),
            Some(Seen::Cabinet(0))
        );
    }

    /// Spec 0001: walking into a cabinet or a wall stops you.
    #[test]
    fn you_cannot_walk_through_anything() {
        use blitzkit::collision::{move_and_slide, Sphere};

        let room = Room::of(some(12));
        let solid = room.solid();
        // the walls, the floor, the way down, the spa, the garden, the space
        // behind the near end wall, the cabinets, the benches, the plinths and
        // the bookcases
        assert_eq!(
            solid.len(),
            room.walls.len()
                // the floors, counted rather than written down: there were
                // two until spec 0022 put a room across the end of the aisle
                // and the number here was the only thing that noticed
                + room.floors().len()
                + cellar::solid(room.reaches).len()
                + crate::spa::solid(room.reaches).len()
                + crate::garden::solid(room.reaches).len()
                + crate::behind::near_wall(room.reaches).len()
                + crate::behind::solid(room.reaches).len()
                // and the back room, per spec 0024
                + crate::back::solid(room.reaches).len()
                // and the hotel through the other wall, per spec 0029,
                // with a leaf for every drawn room
                + crate::aleph::solid(room.reaches).len()
                + crate::register::solid(room.reaches).len()
                // and the other corridor off that lobby, per spec 0035
                + crate::twine::solid(room.reaches).len()
                // and the imaginarium over the hall, per spec 0037
                + crate::aloft::solid().len()
                + room.hotel_doors.len()
                // and the blank door's leaf, per spec 0031
                + 1
                // and the one slab beyond the door, per spec 0018
                + 1
                // the sauna's glass door and the baths' own, both solid
                // wherever they are
                + 2
                + 12
                + room.benches.len()
                + room.displays.len()
                + room.bookcases.len()
        );

        // straight at the first cabinet from the middle of the room
        let target = room.stood[0].at;
        let mut you = vec3(0.0, 0.45, target.z);
        for _ in 0..240 {
            you = move_and_slide(
                Sphere::new(you, 0.45),
                vec3(4.0, 0.0, 0.0),
                1.0 / 60.0,
                &solid,
            );
        }

        assert!(
            you.x < target.x - CABINET.x * 0.4,
            "you walked to {} and the cabinet is at {}",
            you.x,
            target.x
        );
    }

    /// How far a point is from a box, and nought when it is inside it.
    fn clear_of(at: Vec3, box_: &Aabb) -> f32 {
        (box_.min - at).max(at - box_.max).max(Vec3::ZERO).length()
    }

    /// Where you can get to on foot from the doorway, flooded cell by cell.
    ///
    /// Flooded rather than reasoned about. Making the plinths solid closed the
    /// only way into the nook and nothing said so: the route runs between the
    /// last cabinet and the nearest plinth, and that gap was 0.74 wide for
    /// someone 0.9 across. Arithmetic about one wall at a time cannot see a
    /// pinch between two things that were never thought about together.
    ///
    /// Free and reachable are not the same thing, and the difference is a whole
    /// layout. The aisle behind a wall length row of benches was free along all
    /// of it and sealed at both ends, so a test that asked whether you could
    /// stand there passed while you could not get there.
    fn walkable(room: &Room) -> impl Fn(Vec3) -> bool {
        const GRID: f32 = 0.12;

        let solid = room.solid();
        let radius = crate::RADIUS;
        let free = |at: Vec3| solid.iter().all(|box_| clear_of(at, box_) > radius);

        let low = vec3(-(WALL + CABINET.x) - NOOK_DEEP, 0.0, -room.reaches);
        let wide = ((WALL + CABINET.x) * 2.0 + NOOK_DEEP) / GRID;
        let long = room.reaches * 2.0 / GRID;
        let (wide, long) = (wide as usize + 1, long as usize + 1);
        // a hair over the floor rather than exactly on it. The floor is solid
        // now, and a sphere whose middle is exactly a radius above it is
        // touching it, which `clear_of` reads as standing inside something.
        let cell = |x: usize, z: usize| low + vec3(x as f32 * GRID, radius + 0.01, z as f32 * GRID);

        let start = room.doorway();
        let from = (
            ((start.x - low.x) / GRID).round() as usize,
            ((start.z - low.z) / GRID).round() as usize,
        );
        assert!(free(cell(from.0, from.1)), "you start inside something");

        let mut reached = vec![false; wide * long];
        let mut todo = vec![from];
        reached[from.0 * long + from.1] = true;
        while let Some((x, z)) = todo.pop() {
            for (dx, dz) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let (nx, nz) = (x as i32 + dx, z as i32 + dz);
                if nx < 0 || nz < 0 || nx as usize >= wide || nz as usize >= long {
                    continue;
                }
                let (nx, nz) = (nx as usize, nz as usize);
                if reached[nx * long + nz] || !free(cell(nx, nz)) {
                    continue;
                }
                reached[nx * long + nz] = true;
                todo.push((nx, nz));
            }
        }

        move |at: Vec3| {
            let (x, z) = (
                ((at.x - low.x) / GRID).round() as i32,
                ((at.z - low.z) / GRID).round() as i32,
            );

            x >= 0
                && z >= 0
                && (x as usize) < wide
                && (z as usize) < long
                && reached[x as usize * long + z as usize]
        }
    }

    /// Spec 0006: you can walk from where you come in to every bench.
    #[test]
    fn you_can_walk_to_every_bench() {
        let room = Room::of(some(13));
        let reaches = walkable(&room);

        // the nook's own row. The flood is a grid at one height and the
        // cellar is at another, so the one down there is not a thing this can
        // answer: getting to that one is `the_case_is_a_door` and
        // `you_can_get_back_up_the_stair`, which walk it rather than
        // flooding. The back room's is through a door off the gallery and is
        // walked by `back::tests::you_can_walk_in_and_out_again`.
        for bench in room.benches.iter().filter(|one| one.in_the_nook()) {
            // the floor in front of it, on the side you stand to work it
            let at = bench.at + bench.worked_from * (bench.size.x * 0.5 + crate::RADIUS + 0.1);

            assert!(
                reaches(at),
                "you cannot walk from the door to the {}",
                bench.name
            );
        }
    }

    /// Spec 0006: and you can get to the books.
    ///
    /// Not merely stand in front of them. The benches ran the length of the back
    /// wall with 0.3 between them, so the aisle they left behind was wide enough
    /// to stand in along all of it and shut at both ends by the first and last
    /// bench. This test asked whether that strip was free and it was, which is
    /// how a room nobody could walk round passed it.
    #[test]
    fn you_can_walk_to_the_books() {
        let room = Room::of(some(13));
        let reaches = walkable(&room);

        for case in &room.bookcases {
            let face = Room::bookcase_box(case).max.x + crate::RADIUS + 0.08;

            assert!(
                reaches(vec3(face, crate::RADIUS, case.at.z)),
                "you cannot walk from the door to the bookcase at {}",
                case.at.z
            );
        }
    }

    /// Spec 0002: a plinth stops you the way a cabinet does.
    ///
    /// They were the one thing in the room you could walk straight through, so
    /// the shapes on the far wall could be stood inside.
    #[test]
    fn you_cannot_walk_through_a_plinth() {
        use blitzkit::collision::{move_and_slide, Sphere};

        let room = Room::of(some(13));
        let solid = room.solid();

        for one in &room.displays {
            // straight down the room at it, from well short
            let mut you = vec3(one.at.x, 0.45, one.at.z + 3.0);
            for _ in 0..240 {
                you = move_and_slide(
                    Sphere::new(you, 0.45),
                    vec3(0.0, 0.0, -4.0),
                    1.0 / 60.0,
                    &solid,
                );
            }

            let plinth = Room::plinth_box(one);
            assert!(
                you.z > plinth.max.z,
                "you walked to {} and the {} plinth ends at {}",
                you.z,
                one.name,
                plinth.max.z
            );
        }
    }

    /// Spec 0006: and the row against the open side starts past the way in.
    ///
    /// The row used to be centred in the whole span, which was harmless while it
    /// stood out in the floor. Against the open side, the span it is centred in
    /// is the wall the way in is a gap in, and a bench centred down that is a
    /// bench in the doorway.
    #[test]
    fn no_bench_stands_in_the_way_in() {
        let room = Room::of(some(13));
        let door = -room.reaches + NOOK_DOOR;

        // the nook's own row, which is what this is about: the pool table is
        // under all of it and the back room's is through a door
        for bench in room.benches.iter().filter(|one| one.in_the_nook()) {
            assert!(
                Room::bench_box(bench).min.z > door,
                "the {} is in the way in: it starts at {} and the way in ends at {}",
                bench.name,
                Room::bench_box(bench).min.z,
                door
            );
        }
    }

    /// Spec 0006: and your right hand is worked out from the side you stand on.
    #[test]
    fn a_bench_knows_which_way_your_right_hand_is() {
        let room = Room::of(some(13));

        for bench in room.benches.iter().filter(|one| one.in_the_nook()) {
            // you stand on the side the rest of the room is, which is the side
            // the books are, so away from you is into the wall behind the bench
            assert!(
                bench.worked_from.dot(-Vec3::X) > 0.99,
                "the {} is worked from {:?}",
                bench.name,
                bench.worked_from
            );
            assert!(
                bench.at.x + bench.away().x * bench.size.x > bench.at.x,
                "away from the {} is back across the room",
                bench.name
            );
            // and your right hand is across that, not along it
            assert!(bench.right().dot(bench.away()).abs() < 1e-5);
            assert!((bench.right().length() - 1.0).abs() < 1e-5);
        }
    }

    /// Spec 0006: the rug lies on the open floor and not under the furniture.
    #[test]
    fn the_rug_lies_between_the_benches_and_the_books() {
        let room = Room::of(some(13));
        let (at, size) = open_floor(room.reaches);
        let rug = Aabb::from_center_size(at + Vec3::Y * 0.5, vec3(size.x, 1.0, size.z));

        for bench in &room.benches {
            assert!(
                !overlapping(&rug, &Room::bench_box(bench)),
                "the rug runs under the {}",
                bench.name
            );
        }
        for case in &room.bookcases {
            assert!(
                !overlapping(&rug, &Room::bookcase_box(case)),
                "the rug runs under the bookcase at {}",
                case.at.z
            );
        }
        for wall in &room.walls {
            assert!(!overlapping(&rug, wall), "the rug runs into a wall");
        }

        // and it is a rug rather than a runner
        assert!(
            size.z <= size.x * 2.0 + 1e-4,
            "a rug {} by {} is a runner",
            size.x,
            size.z
        );
    }

    /// Spec 0006: the sight lands on every bench from where you work it.
    ///
    /// One at a time this was only ever checked for the first one, so a bench
    /// you cannot point at is a toy that does not answer, and the only way to
    /// find out was to walk up to it.
    #[test]
    fn every_bench_can_be_pointed_at() {
        let room = Room::of(some(13));

        for (n, bench) in room.benches.iter().enumerate() {
            let eye = bench.at.y + 1.55;
            let from = bench.at + bench.worked_from * 1.2 + Vec3::Y * 1.55;
            let at = bench.at + Vec3::Y * bench.size.y;

            assert_eq!(
                room.looking_at(from, at - from),
                Some(Seen::Bench(n)),
                "the sight does not land on the {} from {:?} at eye {}",
                bench.name,
                from,
                eye
            );
        }
    }

    /// Spec 0003: you cannot point at a thing through a wall.
    ///
    /// The sight took the nearest thing the ray met among the things you can
    /// use, and a wall was never one of them. Standing at the ball and chain in
    /// the nook, with the hall on the other side of that wall, the room named a
    /// cabinet through the back of it and offered to play the game.
    #[test]
    fn the_sight_does_not_see_through_walls() {
        let room = Room::of(some(13));
        let eye = 1.55;

        // a cabinet on the row that backs onto the nook, and a spot in the nook
        // level with it. Aimed at a bench instead this passes either way, for
        // the dull reason that no cabinet happens to stand at that bench's end
        // of the hall.
        let hall = room
            .stood
            .iter()
            .find(|stood| stood.at.x < 0.0)
            .expect("a cabinet on the near row");
        let inside = vec3(-(WALL + CABINET.x) - 1.2, eye, hall.at.z);

        assert_eq!(
            room.looking_at(inside, Vec3::X),
            None,
            "the nook can see through its own wall into the hall"
        );

        // and from the aisle it is right there, so this is not passing because
        // there is nothing to find
        let aisle = vec3(0.0, eye, hall.at.z);
        assert!(
            matches!(room.looking_at(aisle, Vec3::NEG_X), Some(Seen::Cabinet(_))),
            "there is no cabinet on that line at all"
        );
    }

    /// Spec 0007: the handle is one book, not a wall of them.
    ///
    /// The room said one of them is not a book and then let you pull any of
    /// them: the sight landed on the whole case, so clicking anywhere on either
    /// leaf worked. That makes the sentence a lie and the door a pair of very
    /// large buttons.
    #[test]
    fn one_book_opens_the_wall() {
        let room = Room::of(some(13));
        let book = room.handle().expect("a book to pull");
        let eye = Vec3::Y * 1.55;

        // it is a book sized thing and not a bookcase sized one
        let case = crate::study::CASE;
        assert!(
            book.size().x < case.x * 0.2 && book.size().y < case.z * 0.3,
            "the handle is {:?} and a case is {:?}",
            book.size(),
            case
        );

        // pointing at it finds it
        let at = book.center();
        let from = vec3(at.x + 1.2, eye.y, at.z);
        assert_eq!(
            room.looking_at(from, at - from),
            Some(Seen::Case),
            "the book is not pickable"
        );

        // and pointing at the rest of the same shelf finds nothing
        for off in [-0.45f32, -0.3, 0.3, 0.45] {
            let elsewhere = vec3(at.x, at.y, at.z + off);
            let stood = vec3(elsewhere.x + 1.2, eye.y, elsewhere.z);

            assert_ne!(
                room.looking_at(stood, elsewhere - stood),
                Some(Seen::Case),
                "{} along the shelf is also a handle",
                off
            );
        }

        // and so does the other leaf, which has no book in it
        let other = &room.bookcases[cellar::CASE + 1];
        let face = Room::bookcase_box(other);
        let stood = vec3(face.max.x + 1.2, eye.y, other.at.z);
        assert_ne!(
            room.looking_at(stood, vec3(face.max.x, eye.y, other.at.z) - stood),
            Some(Seen::Case),
            "the whole of the other leaf is a handle"
        );
    }

    /// Spec 0007: and once it is open, the whole of either leaf shuts it.
    ///
    /// The one book is what makes the door a secret, and a secret is only worth
    /// keeping until it is out. With the wall standing open, finding that same
    /// spine again to shut it is the same hunt with none of the point, so from
    /// the moment a leaf leaves its stop the leaf itself is what you click.
    #[test]
    fn an_open_wall_shuts_from_anywhere_on_a_leaf() {
        let mut room = Room::of(some(13));
        let eye = Vec3::Y * 1.55;

        // the leaf with no book in it, which shut is not a handle anywhere on
        // it: `one_book_opens_the_wall` is the other half of this
        let other = cellar::CASE + 1;
        room.open = true;
        room.swing = 1.0;

        let leaf = Room::swung_box(&room.bookcases[other], room.swings_out(other), room.swing);
        let at = vec3(leaf.center().x, eye.y, leaf.center().z);
        let from = at + Vec3::X * (leaf.size().x * 0.5 + 1.2);

        assert_eq!(
            room.looking_at(from, at - from),
            Some(Seen::Case),
            "an open leaf is not pickable"
        );

        // and with the wall shut again it is back to being a wall of books
        room.open = false;
        room.swing = 0.0;
        let shut = Room::bookcase_box(&room.bookcases[other]);
        let at = vec3(shut.max.x, eye.y, room.bookcases[other].at.z);
        let from = vec3(at.x + 1.2, eye.y, at.z);

        assert_ne!(
            room.looking_at(from, at - from),
            Some(Seen::Case),
            "a shut leaf is a handle all over"
        );
    }

    /// Spec 0007: nothing down the way down is inside the room above it.
    ///
    /// Two surfaces in the same place fight for the same pixels and come out as
    /// a rectangle of the wrong colour with stippled edges. The stair's own
    /// soffit started at the back wall's inner face, which is a third of a unit
    /// inside the wall, and hung a grey patch over the books.
    #[test]
    fn the_way_down_keeps_out_of_the_room() {
        let room = Room::of(some(13));

        for (n, box_) in cellar::solid(room.reaches).iter().enumerate() {
            for wall in &room.walls {
                assert!(
                    !overlapping(box_, wall),
                    "box {} of the way down is inside a wall: {:?} in {:?}",
                    n,
                    box_,
                    wall
                );
            }

            for case in &room.bookcases {
                assert!(
                    !overlapping(box_, &Room::bookcase_box(case)),
                    "box {} of the way down is inside the bookcase at {}",
                    n,
                    case.at.z
                );
            }
        }
    }

    /// Spec 0007: shut, the way down is not there; open, it is.
    ///
    /// Walked rather than flooded. The flood is a grid at one height and this
    /// is the first thing in the building at two, so the only honest question
    /// is whether somebody walking at it gets down.
    #[test]
    fn the_case_is_a_door() {
        let mut room = Room::of(some(13));
        let from = vec3(
            cellar::back() + crate::RADIUS + 0.4,
            0.0,
            cellar::opening(room.reaches),
        );

        let walked = |room: &Room, frames: usize, off: f32| {
            let solid = room.solid();
            let (mut at, mut falling) = (from + Vec3::Z * off, 0.0);

            for _ in 0..frames {
                let (next, fell) = crate::walk::walk(
                    at,
                    Vec3::NEG_X * 4.2,
                    falling,
                    crate::RADIUS,
                    1.0 / 60.0,
                    &solid,
                    true,
                );
                at = next;
                falling = fell;
            }

            at
        };

        // shut, there is a bookcase in the way and the floor stays flat
        let stopped = walked(&room, 120, 0.0);
        assert!(
            stopped.y.abs() < 1e-3,
            "the way down is open with the case shut: you got to {}",
            stopped.y
        );
        assert!(
            stopped.x > cellar::back() - 0.1,
            "you walked through the bookcase to {}",
            stopped.x
        );

        // and open, it goes down. Off the middle as well as along it, which is
        // the part that was worth testing and was not tested: the way down was
        // one case wide, 1.1 against a body 0.9 across, and held at that for the
        // nine units it takes to get to the bottom. Walked exactly down the
        // centre line it fits, so this passed, and nobody walks exactly down the
        // centre line of anything.
        room.open = true;

        // and it takes its time about it, which is most of the point of a
        // secret door: the moment between pulling the book and seeing what is
        // behind it
        let mut frames = 0;
        while room.swing < 1.0 && frames < 600 {
            room.ease(1.0 / 60.0);
            frames += 1;
        }
        assert!(
            (frames as f32 / 60.0 - Room::SWINGS_IN).abs() < 0.05,
            "it took {} seconds to open and should take {}",
            frames as f32 / 60.0,
            Room::SWINGS_IN
        );

        // out to where the passage itself runs out, which is what it is for
        let edge = cellar::PASSAGE * 0.5 - crate::RADIUS - 0.05;
        assert!(edge > 0.3, "a passage with {} of room is not one", edge);

        for off in [0.0, 0.3, -0.3, edge, -edge] {
            let down = walked(&room, 420, off);

            assert!(
                (down.y + cellar::DOWN).abs() < 0.05,
                "{} off the middle you got to {}, not {}",
                off,
                down.y,
                -cellar::DOWN
            );
        }
    }

    /// Spec 0007: and you can get back up it, facing either way.
    ///
    /// Up and down are the same code and there is nothing in it that knows
    /// which way you are going, so this ought to be free. It is written down
    /// because "I can't walk backwards up them" is not a thing arithmetic says.
    #[test]
    fn you_can_get_back_up_the_stair() {
        let mut room = Room::of(some(13));
        room.open = true;
        room.swing = 1.0;

        let solid = room.solid();
        let walked = |from: Vec3, way: Vec3, frames: usize| {
            let (mut at, mut falling) = (from, 0.0);

            for _ in 0..frames {
                let (next, fell) =
                    crate::walk::walk(at, way, falling, crate::RADIUS, 1.0 / 60.0, &solid, true);
                at = next;
                falling = fell;
            }

            at
        };

        // down first, to somewhere on the stair
        let from = vec3(
            cellar::back() + crate::RADIUS + 0.4,
            0.0,
            cellar::opening(room.reaches),
        );
        let down = walked(from, Vec3::NEG_X * 4.2, 420);
        assert!(
            (down.y + cellar::DOWN).abs() < 0.05,
            "you did not get down, you got to {}",
            down.y
        );

        // and back up, which is the same walk with the sign turned round
        let up = walked(down, Vec3::X * 4.2, 600);
        assert!(
            up.y.abs() < 0.05,
            "you got back up as far as {} and the nook is at nought",
            up.y
        );
    }

    /// Spec 0007: the door does not open through you.
    ///
    /// You stand in front of the case to pull the book, and the case sweeps the
    /// floor you are standing on. Everything else about getting about is you
    /// moving and the room holding still, so nothing pushed back and the
    /// shelves swung through the viewer, which from the inside is books passing
    /// through your eye.
    #[test]
    fn the_door_does_not_open_through_you() {
        let mut room = Room::of(some(13));
        let case = &room.bookcases[cellar::CASE];
        let shut = Room::bookcase_box(case);

        // right in front of it, where somebody pulling the book would be
        let mut at = vec3(shut.max.x + crate::RADIUS - 0.05, 0.0, case.at.z);
        room.open = true;

        for _ in 0..120 {
            room.ease(1.0 / 60.0);
            at = crate::walk::shoved(at, crate::RADIUS, &room.leaves());

            let middle = at + Vec3::Y * crate::RADIUS;
            for (at_leaf, turn, half) in room.leaves() {
                let local = turn.inverse() * (middle - at_leaf);
                let near = vec3(
                    local.x.clamp(-half.x, half.x),
                    0.0,
                    local.z.clamp(-half.z, half.z),
                );
                let flat = vec3(local.x - near.x, 0.0, local.z - near.z);

                assert!(
                    flat.length() > crate::RADIUS - 0.02,
                    "a leaf is {} into you, at swing {}",
                    crate::RADIUS - flat.length(),
                    room.swing
                );
            }
        }

        assert!(room.swing >= 1.0, "it never finished opening");

        // and shutting it again, which is the half that was still going through
        // people. Opening pushes you out into an empty room; shutting sweeps
        // back towards the wall, and on the room's own axes the way out of a
        // leaf at forty five degrees is not the way out of the shelf.
        room.open = false;
        for _ in 0..120 {
            room.ease(1.0 / 60.0);
            at = crate::walk::shoved(at, crate::RADIUS, &room.leaves());

            let middle = at + Vec3::Y * crate::RADIUS;
            for (at_leaf, turn, half) in room.leaves() {
                let local = turn.inverse() * (middle - at_leaf);
                let near = vec3(
                    local.x.clamp(-half.x, half.x),
                    0.0,
                    local.z.clamp(-half.z, half.z),
                );
                let flat = vec3(local.x - near.x, 0.0, local.z - near.z);

                assert!(
                    flat.length() > crate::RADIUS - 0.02,
                    "a leaf is {} into you shutting, at swing {}",
                    crate::RADIUS - flat.length(),
                    room.swing
                );
            }
        }

        assert!(room.swing <= 0.0, "it never finished shutting");
    }

    /// Spec 0006: every sconce is on a wall.
    ///
    /// One was not. The two long walls shared a single run the length of the
    /// nook, and the open side is not that long, because the way in is a gap in
    /// it. So the far end of that run hung a lit sconce in the air over the
    /// aisle with nothing behind it, throwing light on nothing. The panelling
    /// knew where that wall starts and the lighting did not, which is what two
    /// lists of the same wall gets you.
    #[test]
    fn every_sconce_is_on_a_wall() {
        let room = Room::of(some(13));
        let mut counted = 0;

        for (face, out, from, to) in sconce_runs(room.reaches) {
            for along in crate::study::sconces(from, to) {
                // just inside the wall, behind the back plate
                let into = vec3(face - out * 0.02, crate::study::SCONCE_UP, along);
                counted += 1;

                assert!(
                    room.walls.iter().any(|wall| wall.contains_point(into)),
                    "a sconce at {} on the wall at {} has nothing behind it",
                    along,
                    face
                );
            }
        }

        assert!(counted >= 8, "only {} sconces in the whole room", counted);
    }

    #[test]
    fn the_nook_is_off_the_aisle_and_not_in_it() {
        let room = Room::of(some(12));

        for bench in room.benches.iter().filter(|one| one.in_the_nook()) {
            assert!(
                bench.at.x < -(WALL + CABINET.x),
                "a bench stood in the aisle at {}",
                bench.at.x
            );
        }
    }

    /// Spec 0022: the shapes have a room to themselves and nothing else is
    /// in it.
    ///
    /// **This used to be about benches.** While the shapes were on the hall's
    /// far wall anything standing in the aisle was in front of them from every
    /// angle, and a bench once was. Moving the wall into a room of its own
    /// makes that a stronger and simpler thing to ask: nothing in the building
    /// is in there at all, except the room itself and what stands the shapes
    /// up.
    #[test]
    fn nothing_stands_in_the_gallery() {
        let room = Room::of(some(12));
        let (at, size) = gallery(room.reaches);

        // the footprint, inside its own walls by their whole thickness so
        // they do not count as being in it. Half of one is not enough: a wall
        // is centred on the line it closes, so it reaches half its thickness
        // into the room already.
        let edge = THICK;
        let low = vec3(at.x - size.x * 0.5 + edge, 0.0, at.z - size.z * 0.5 + edge);
        let high = vec3(at.x + size.x * 0.5 - edge, TALL, at.z + size.z * 0.5 - edge);

        let plinths: Vec<Aabb> = room.displays.iter().map(Room::plinth_box).collect();

        for box_ in room.solid() {
            // the floors are under it rather than in it
            if box_.max.y <= 0.0 {
                continue;
            }
            // and the imaginarium is over it rather than in it, by the same
            // reasoning. This check is `x` and `z` only, which was right
            // while nothing in this building was ever above anything else:
            // spec 0037 put a room six metres up and it stands in the
            // gallery's plan while blocking nothing at all.
            if box_.min.y >= TALL {
                continue;
            }
            // and the plinths are what the room is for
            if plinths
                .iter()
                .any(|one| one.min == box_.min && one.max == box_.max)
            {
                continue;
            }

            let over = box_.min.x < high.x
                && box_.max.x > low.x
                && box_.min.z < high.z
                && box_.max.z > low.z;

            assert!(
                !over,
                "something stands in the gallery: {:?} to {:?}",
                box_.min, box_.max
            );
        }

        // and the shapes really are in there
        for shape in &room.displays {
            assert!(
                shape.at.x > low.x && shape.at.x < high.x,
                "{} is not in the gallery: x {:.2} against {:.2} to {:.2}",
                shape.name,
                shape.at.x,
                low.x,
                high.x
            );
        }
    }

    #[test]
    fn the_nook_has_a_way_in() {
        // the left wall starts past the last cabinet, and the gap it leaves
        // there is the doorway. A nook with four walls is a cupboard.
        let room = Room::of(some(12));
        let into = vec3(-(WALL + CABINET.x), 1.0, -room.reaches + NOOK_DOOR * 0.5);

        assert!(
            !room.solid().iter().any(|box_| box_.contains_point(into)),
            "the nook is walled in"
        );

        // and you can walk through it, which a gap narrower than you is not
        let wide = NOOK_DOOR;
        assert!(wide > 1.0, "a way in {} wide is not one", wide);
    }

    /// Spec 0006: and taking the wall away to make that way in is not it.
    ///
    /// The nook opened over the whole far end of the left wall, so the last
    /// cabinet on that side stood in the opening with nothing behind it: from
    /// the aisle you saw through its back into the nook.
    #[test]
    fn every_cabinet_has_a_wall_behind_it() {
        for count in [1usize, 2, 7, 12, 13] {
            let room = Room::of(some(count));

            for (n, stood) in room.stood.iter().enumerate() {
                let behind = room
                    .walls
                    .iter()
                    .filter_map(|wall| {
                        slab(stood.at + Vec3::Y * CABINET.y * 0.5, -stood.facing, wall)
                    })
                    .fold(f32::INFINITY, f32::min);

                assert!(
                    behind < CABINET.x,
                    "cabinet {} of {} has nothing behind it for {} units",
                    n,
                    count,
                    behind
                );
            }
        }
    }

    /// Spec 0006: and the benches stand in a row down the nook, inside it.
    #[test]
    fn the_benches_fit_down_the_nook() {
        let room = Room::of(some(13));
        let solid = room.solid();

        for bench in &room.benches {
            let box_ = Room::bench_box(bench);

            for wall in &room.walls {
                assert!(
                    !overlapping(&box_, wall),
                    "the bench holding the {} is inside a wall",
                    bench.name
                );
            }
        }

        // and not inside each other
        for (n, one) in room.benches.iter().enumerate() {
            for other in &room.benches[n + 1..] {
                assert!(
                    !overlapping(&Room::bench_box(one), &Room::bench_box(other)),
                    "the {} and the {} share a bench",
                    one.name,
                    other.name
                );
            }
        }

        assert_eq!(
            solid.len(),
            room.walls.len()
                + room.floors().len()
                + cellar::solid(room.reaches).len()
                + crate::spa::solid(room.reaches).len()
                + crate::garden::solid(room.reaches).len()
                + crate::behind::near_wall(room.reaches).len()
                + crate::behind::solid(room.reaches).len()
                // and the back room, per spec 0024
                + crate::back::solid(room.reaches).len()
                // and the hotel through the other wall, per spec 0029,
                // with a leaf for every drawn room
                + crate::aleph::solid(room.reaches).len()
                + crate::register::solid(room.reaches).len()
                // and the other corridor off that lobby, per spec 0035
                + crate::twine::solid(room.reaches).len()
                // and the imaginarium over the hall, per spec 0037
                + crate::aloft::solid().len()
                + room.hotel_doors.len()
                // and the blank door's leaf, per spec 0031
                + 1
                // and the one slab beyond the door, per spec 0018
                + 1
                // the sauna's glass door and the baths' own, both solid
                // wherever they are
                + 2
                + 13
                + room.benches.len()
                + room.displays.len()
                + room.bookcases.len()
        );
    }

    #[test]
    fn a_bench_is_something_you_bump_into() {
        let room = Room::of(some(12));
        let bench = room.benches.first().expect("a bench");
        let middle = bench.at + Vec3::Y * bench.size.y * 0.5;

        assert!(room.solid().iter().any(|box_| box_.contains_point(middle)));
    }

    #[test]
    fn the_sight_lands_on_a_bench() {
        let room = Room::of(some(12));
        let bench = room.benches.first().expect("a bench");

        // stood in the nook on the side you work it from, which is the side
        // the rest of the room is. From the other side the nook's own wall is
        // between you and it, and a wall stops the sight now.
        let from = bench.at + bench.worked_from * 1.4 + Vec3::Y * 1.5;
        let way = (bench.at + Vec3::Y * bench.size.y - from).normalize();

        assert_eq!(room.looking_at(from, way), Some(Seen::Bench(0)));
    }

    /// Spec 0006: crossing the nook does not quietly hand your arrows to a
    /// toy you are walking past.
    ///
    /// The arrows work the toy the sight is on and turn you when it is on
    /// nothing, so a toy that reaches across the room is a room you cannot
    /// look around in. Jake found it from the way up, reporting that the
    /// arrow keys stop working near the stairs; the stairs were not it.
    /// Spec 0038: nothing of the grid is left hanging in the stairwell.
    ///
    /// **A hole in a ceiling goes through the ceiling.** `minus` cuts in
    /// all three directions, and the well began inside the ceiling's own
    /// thickness, so every bar crossing the opening kept its lowest two
    /// centimetres. From up the stair they read as bars lying across the
    /// nook's floor, which is how Jake found them.
    #[test]
    fn nothing_of_the_grid_hangs_in_the_stairwell() {
        let room = Room::of(crate::games());
        let (bars, tiles) = room.lattice();

        // **Over the ceiling's own thickness, not over the well's.** Asked
        // of the well's height this cannot fail: whatever height the well
        // has, clipping to it removes exactly what is inside it, so the
        // question answers itself. What is being asked is whether anything
        // of the grid is left in the opening's footprint at any height the
        // grid occupies, which is how the shaved bars went unnoticed.
        for open in crate::aloft::up_well().iter() {
            let inside = Aabb::new(
                vec3(open.min.x + 1e-3, TALL - CEIL_DROP * 2.0, open.min.z + 1e-3),
                vec3(open.max.x - 1e-3, TALL + CEIL_DROP, open.max.z - 1e-3),
            );

            for (what, pieces) in [("bar", &bars), ("tile", &tiles)] {
                for (at, size) in pieces.iter() {
                    let box_ = Aabb::from_center_size(*at, *size);

                    assert!(
                        !inside.intersects(&box_),
                        "a {} is still in the stairwell at {:?}, {:.3} deep",
                        what,
                        at,
                        inside.max.y.min(box_.max.y) - inside.min.y.max(box_.min.y)
                    );
                }
            }
        }
    }

    /// Spec 0038: the ceiling is open where the way up comes through it.
    ///
    /// The grid hanging under it was clipped to the well from the day the
    /// stair was built and the lid behind the grid was not, so what you saw
    /// looking up the stair was the hole in the grid with a solid ceiling
    /// behind it.
    /// Spec 0041: the sky is over you in the air and over the field, and is
    /// not over you in a room.
    ///
    /// **The air is the half that was missing.** `field::in_view` asks what
    /// is holding you up, which is nothing at all when you are flying, so
    /// forty metres over this building it said indoors and the ground, the
    /// sky and the far plane all went out at once.
    #[test]
    fn the_sky_is_over_you_outside_and_not_in_a_room() {
        let room = Room::of(crate::games());
        let door = room.doorway();

        for (at, what) in [
            (vec3(door.x, 40.0, door.z), "forty metres over the hall"),
            (vec3(door.x, 190.0, door.z), "a hundred and ninety over it"),
            (
                vec3(crate::aloft::MIDDLE_X, 30.0, crate::aloft::MIDDLE_Z),
                "over the imaginarium",
            ),
            (vec3(-40.0, 0.5, 0.0), "out in the field"),
            // grass at the building's east wall with a storey hanging five
            // metres over it, which is out of doors by every test but one
            (vec3(3.0, 0.5, -8.0), "under the imaginarium's oversail"),
        ] {
            assert!(room.out_of_doors(at), "{} counts as indoors", what);
        }

        for (at, what) in [
            (vec3(door.x, 1.6, door.z), "in the hall"),
            (
                vec3(
                    crate::cellar::stair_foot() - 3.0,
                    -crate::cellar::DOWN + 0.5,
                    crate::cellar::opening(Room::of(crate::games()).reaches),
                ),
                "down in the cellar",
            ),
            (crate::aloft::middle(), "in the imaginarium"),
        ] {
            assert!(!room.out_of_doors(at), "{} counts as out of doors", what);
        }
    }

    /// Spec 0040: nothing the building shows the sky is plastered where the
    /// sky cannot see it.
    ///
    /// **Which is the only thing that can go wrong here.** A face that is
    /// missed leaves bare stone, and bare stone is what this whole job is
    /// about, so it is reported rather than hidden. A face that is found
    /// where it should not be is plaster inside somebody's room, which is
    /// the fault the baths had when they borrowed the cellar's wall and had
    /// mahogany down one side of a tiled room, and nobody looked for a year.
    #[test]
    fn nothing_outside_is_plastered_indoors() {
        let room = Room::of(crate::games());
        let faces = room.outside();
        let lids = room.lids();
        let mut solid = room.solid();
        solid.extend(lids.iter().copied());

        assert!(
            faces.len() > 12,
            "only {} faces round a building",
            faces.len()
        );

        for (face, looks) in faces.iter().copied() {
            let at = face.center() + looks * 0.12;

            assert!(
                !solid.iter().any(|box_| box_.contains_point(at)),
                "the face at {:?} has a wall a hand's breadth outside it",
                face.center()
            );
            assert!(
                !solid.iter().any(|box_| {
                    box_.min.x <= at.x
                        && box_.max.x >= at.x
                        && box_.min.z <= at.z
                        && box_.max.z >= at.z
                        && box_.max.y >= at.y
                }),
                "the face at {:?} has a roof over the spot outside it",
                face.center()
            );
        }
    }

    /// And the plaster sits on the wall's own outer face, not on the edge of
    /// the lid over it.
    ///
    /// Some lids here oversail their walls by a thickness and some are the
    /// room's own inside, so a plane taken off the lid is sometimes the
    /// outside of the building and sometimes buried in it. Buried, the
    /// column reads as covered and the face is not found at all, which is
    /// why the test above cannot see this one: it loses walls rather than
    /// misplacing them. Four of the hall's went that way.
    #[test]
    fn the_plaster_sits_on_the_wall_and_not_on_the_lid() {
        let room = Room::of(crate::games());
        let lids = room.lids();
        let proud = room
            .outside()
            .into_iter()
            .filter(|(face, looks)| {
                let at = face.center();
                let plane = if looks.x.abs() > 0.5 { at.x } else { at.z };

                // on no lid's edge, which is where a plane taken off a lid
                // would be and where the wall's own face rarely is
                lids.iter().all(|lid| {
                    let (low, high) = if looks.x.abs() > 0.5 {
                        (lid.min.x, lid.max.x)
                    } else {
                        (lid.min.z, lid.max.z)
                    };

                    (plane - low).abs() > 0.1 && (plane - high).abs() > 0.1
                })
            })
            .count();

        assert!(
            proud > 0,
            "every face sits on the edge of a lid, so none of them found a wall"
        );
    }

    /// And the strip over a borrowed wall is one of them. The register
    /// stands a metre taller than the lobby it abuts, so part of its east
    /// side is indoors to the lobby's height and open sky above that. Taken
    /// as whole walls rather than as columns it would be bare.
    #[test]
    fn the_strip_over_a_shorter_room_is_dressed() {
        let room = Room::of(crate::games());
        let high: Vec<_> = room
            .outside()
            .into_iter()
            .filter(|(face, _)| face.min.y > 0.5)
            .collect();

        assert!(
            !high.is_empty(),
            "every face found starts at the ground, so no borrowed wall was topped"
        );
    }

    #[test]
    fn the_ceiling_is_open_over_the_stairwell() {
        let room = Room::of(crate::games());
        let ceiling = room.ceiling();

        for (n, open) in crate::aloft::up_well().iter().enumerate() {
            let middle = vec3(open.center().x, TALL, open.center().z);

            assert!(
                !ceiling.iter().any(|piece| piece.contains_point(middle)),
                "piece {} of the stairwell still has a ceiling over it",
                n
            );
        }

        // and the rest of it is still there: the middle of the hall's aisle
        // is nowhere near the well and must still be covered
        let hall = room.floors()[0];
        let over = vec3(hall.center().x, TALL, hall.center().z);

        assert!(
            ceiling.iter().any(|piece| piece.contains_point(over)),
            "cutting the stairwell took the hall's ceiling with it"
        );
    }

    #[test]
    fn crossing_the_nook_leaves_your_arrows_alone() {
        let room = Room::of(some(12));
        let bench = room.benches.first().expect("a bench");

        // out in the nook's own lane, a stride and a half off the bench's
        // working side, looking the way somebody crossing the room looks
        let from = bench.at + bench.worked_from * 2.6 + Vec3::Y * 1.5;

        for turn in 0..16 {
            let yaw = turn as f32 * std::f32::consts::TAU / 16.0;
            let way = vec3(yaw.sin(), -0.25, -yaw.cos()).normalize();

            assert!(
                !matches!(room.looking_at(from, way), Some(Seen::Bench(_))),
                "crossing the nook at {:.2} radians took the arrows",
                yaw
            );
        }
    }

    #[test]
    fn a_bench_out_of_reach_is_not_seen() {
        let room = Room::of(some(12));
        let bench = room.benches.first().expect("a bench");

        // out in the aisle, where the cabinets are nearer than the bench. What
        // matters is that the bench is not what you are pointing at.
        let from = bench.at + Vec3::X * (REACH + 2.0) + Vec3::Y * 1.5;
        let way = (bench.at + Vec3::Y * bench.size.y - from).normalize();

        assert_ne!(room.looking_at(from, way), Some(Seen::Bench(0)));
    }

    /// How two floors sit against each other in plan: the daylight between
    /// them along each axis, negative where they overlap.
    fn apart_in_plan(a: &Aabb, b: &Aabb) -> (f32, f32) {
        (
            (b.min.x - a.max.x).max(a.min.x - b.max.x),
            (b.min.z - a.max.z).max(a.min.z - b.max.z),
        )
    }

    /// Spec 0011: no two rooms leave a hole between their floors.
    ///
    /// The fault is a strip of nothing where one room's ground stops and the
    /// next one's has not started. The garden had it in a doorway and it is
    /// written up there: you could walk up to the opening from either side and
    /// not through it, because the width of the wall had no ground under it.
    ///
    /// **Not pairwise.** Two floors leaving daylight between them is not by
    /// itself a fault, because a third can be lying under the strip: the
    /// gallery's floor stops two tenths short of the other corridor's and the
    /// lobby's runs under both. So the strip is sampled and the whole set is
    /// asked, rather than the pair being asked about itself.
    ///
    /// A gap wider than a body is not a seam between two rooms at all, so the
    /// rule stops caring there. What is on the far side of a gap you cannot
    /// fall into is a different room, and the walls hold that.
    #[test]
    fn no_two_rooms_leave_a_hole_between_their_floors() {
        let room = Room::of(crate::games());
        let grounds = room.grounds();
        let body = crate::RADIUS * 2.0;

        for (i, (a_name, a)) in grounds.iter().enumerate() {
            for (b_name, b) in grounds.iter().skip(i + 1) {
                if a_name == b_name {
                    continue;
                }

                // one storey at a time. Two floors three metres apart in
                // height have no seam between them to fall through, whatever
                // their footprints do: what is between them is a wall or the
                // open air of the room below, and neither is this rule's.
                if (b.max.y - a.max.y).abs() > crate::walk::STEP {
                    continue;
                }

                let (x, z) = apart_in_plan(a, b);
                // facing each other across a strip: daylight along one axis,
                // an overlap along the other
                let (from, to, along_from, along_to, across_x) = if x > 1e-4 && z < -1e-4 {
                    let (west, east) = if a.max.x <= b.min.x { (a, b) } else { (b, a) };

                    (
                        west.max.x,
                        east.min.x,
                        a.min.z.max(b.min.z),
                        a.max.z.min(b.max.z),
                        true,
                    )
                } else if z > 1e-4 && x < -1e-4 {
                    let (south, north) = if a.max.z <= b.min.z { (a, b) } else { (b, a) };

                    (
                        south.max.z,
                        north.min.z,
                        a.min.x.max(b.min.x),
                        a.max.x.min(b.max.x),
                        false,
                    )
                } else {
                    continue;
                };

                if to - from > body {
                    continue;
                }

                // Every part of the strip wants ground under it at a height
                // you could be walking at, which is one of these two floors'
                // own or a step off it.
                //
                // The whole length of the seam and not its middle. The hole
                // this rule found first was in the cellar's door to the baths
                // and the middle of that seam is a wall, so a rule that asked
                // about middles would have found it only by luck. A body is
                // `RADIUS * 2` across, so a step of `RADIUS` cannot miss a
                // hole wide enough to matter.
                let steps =
                    (((along_to - along_from) / crate::RADIUS).ceil() as usize).clamp(1, 4096);

                for (n, m) in (0..=steps).flat_map(|n| (0..=8).map(move |m| (n, m))) {
                    let f = m as f32 / 8.0;
                    let along = along_from + (along_to - along_from) * (n as f32 / steps as f32);
                    let (at_x, at_z) = if across_x {
                        (from + (to - from) * f, along)
                    } else {
                        (along, from + (to - from) * f)
                    };
                    let covered = grounds.iter().any(|(_, under)| {
                        under.min.x <= at_x + 1e-4
                            && under.max.x >= at_x - 1e-4
                            && under.min.z <= at_z + 1e-4
                            && under.max.z >= at_z - 1e-4
                            && ((under.max.y - a.max.y).abs() <= crate::walk::STEP
                                || (under.max.y - b.max.y).abs() <= crate::walk::STEP)
                    });

                    assert!(
                        covered,
                        "{} and {} stop {:.2} apart and nothing floors the strip \
                         between them at {:.2} {:.2}",
                        a_name,
                        b_name,
                        to - from,
                        at_x,
                        at_z
                    );
                }
            }
        }
    }

    /// Spec 0011: no two rooms meet at a ledge.
    ///
    /// Where two floors touch or lap in plan you may be walking from one onto
    /// the other, and the step between them has to be one you can take. Over
    /// `walk::STEP` the body walks into it instead, and a rise nobody built is
    /// a rise nobody can see.
    ///
    /// **Unless it is a storey and not a step.** Three of these rooms stand
    /// over others: the imaginarium is six above the hall, the cellar three
    /// under the lobby, the baths three under the garden. A floor more than a
    /// head above you is a ceiling and cannot trip you, so the rule takes it
    /// as another storey. The band it leaves is from a step you cannot take up
    /// to a head, which is where every ledge of this kind lives and where this
    /// building currently has nothing at all.
    ///
    /// What this does not cover is something that is not a floor poking up
    /// through one, which is the fault the baths' roof had against the
    /// garden's gravel: that is one room's lid against another room's ground
    /// and `garden::tests::the_garden_sits_on_the_roof_of_the_baths` is what
    /// holds it.
    #[test]
    fn no_two_rooms_meet_at_a_ledge() {
        let room = Room::of(crate::games());
        let grounds = room.grounds();

        for (i, (a_name, a)) in grounds.iter().enumerate() {
            for (b_name, b) in grounds.iter().skip(i + 1) {
                if a_name == b_name {
                    continue;
                }

                let (x, z) = apart_in_plan(a, b);
                if x > 1e-4 || z > 1e-4 {
                    continue;
                }

                let step = (b.max.y - a.max.y).abs();

                assert!(
                    step <= crate::walk::STEP + 1e-4 || step >= crate::EYE,
                    "{} is at {:.2} and {} at {:.2}, which is {:.2}: too high to \
                     step up and too low to be a storey",
                    a_name,
                    a.max.y,
                    b_name,
                    b.max.y,
                    step
                );
            }
        }
    }

    /// Every ground in the set is a box this building actually serves.
    ///
    /// The set is gathered by calling each room for its floor, and a room can
    /// have more than one thing it calls a floor. The cellar has two: the
    /// clear floor of the room, inset by a wall on every side and flat, which
    /// is what you stand things on, and the slab the walk meets. `grounds`
    /// asked for the first by mistake, and the two differ by half a wall all
    /// round, which was enough to report a hole along the length of the baths'
    /// wall that is not there.
    ///
    /// So each one is looked for in `solid`. A ground nothing serves is a
    /// ground the walk never meets, and a rule about it is a rule about
    /// nothing.
    #[test]
    fn every_ground_is_a_box_the_building_serves() {
        let room = Room::of(crate::games());
        let solid = room.solid();

        for (name, ground) in room.grounds() {
            let served = solid.iter().any(|box_| {
                (box_.min - ground.min).abs().max_element() < 1e-3
                    && (box_.max - ground.max).abs().max_element() < 1e-3
            });

            assert!(
                served,
                "{}'s floor is x {:.2}..{:.2} y {:.2}..{:.2} z {:.2}..{:.2} and no \
                 box the building serves is that, so nothing walks on it",
                name,
                ground.min.x,
                ground.max.x,
                ground.min.y,
                ground.max.y,
                ground.min.z,
                ground.max.z
            );
        }
    }
}
