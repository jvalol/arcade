//! The room, the cabinets in it, and where you are standing. Spec 0001.
//!
//! Nothing here draws or reads a key. The shape of the room and which cabinet
//! you are at are arithmetic, so all of it can be checked without a window.

use crate::cabinet::Cabinet;
use crate::cellar;
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
/// Twice over. It was 3.0 by 8.0 with the benches end to end down one wall,
/// which is 24 square units of floor holding five benches and a wall of books,
/// and it read as a corridor because it was one. 5.4 by 8.0 furnished it and
/// still had you turning sideways. It is 6.4 by 13.0, which is 83 against the
/// first 24, and the open floor between the benches and the books is 4.9 across.
pub const NOOK_DEEP: f32 = 6.4;
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
pub const WALL: f32 = 2.0;
pub const TALL: f32 = 3.2;
pub const THICK: f32 = 0.3;

/// How far inside the near wall you wake up.
///
/// A named number because what is in front of you when you open your eyes is
/// measured from here, and the room's own sign is hung to be in it.
pub const INSIDE: f32 = 1.0;

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

    out
}

impl Benched {
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
        let shut = -reaches + NOOK_SPAN;

        let walls = vec![
            // the right wall, whole
            Aabb::from_center_size(
                vec3(side, TALL * 0.5, 0.0),
                vec3(thick, TALL, reaches * 2.0),
            ),
            // the left wall, which backs the cabinets and closes the nook's
            // long side both. It starts past the last cabinet, and the gap it
            // leaves there is the way in.
            Aabb::from_center_size(
                vec3(-side, TALL * 0.5, (reaches + door) * 0.5),
                vec3(thick, TALL, reaches - door),
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
            Aabb::from_center_size(
                vec3(-side - NOOK_DEEP * 0.5, TALL * 0.5, shut),
                vec3(NOOK_DEEP, TALL, thick),
            ),
            // the near end
            Aabb::from_center_size(
                vec3(0.0, TALL * 0.5, reaches),
                vec3(side * 2.0, TALL, thick),
            ),
            // and the far end, reaching across the nook too
            Aabb::from_center_size(
                vec3(-NOOK_DEEP * 0.5, TALL * 0.5, -reaches),
                vec3(side * 2.0 + NOOK_DEEP, TALL, thick),
            ),
        ];

        let benches = benches(-reaches);
        let bookcases = bookcases(-reaches);

        Self {
            stood,
            displays: crate::display::all_of_them(-reaches),
            benches,
            bookcases,
            walls,
            reaches,
            open: false,
            sauna_open: false,
            sauna_swing: 0.0,
            baths_open: false,
            baths_swing: 0.0,
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
    pub fn floor(&self) -> Aabb {
        let side = WALL + CABINET.x;
        let across = side * 2.0 + NOOK_DEEP;

        Aabb::from_center_size(
            vec3(-NOOK_DEEP * 0.5, -THICK * 0.5, 0.0),
            vec3(across, THICK, self.reaches * 2.0),
        )
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
        let mut out = self.walls.clone();
        out.push(self.floor());
        out.extend(crate::cellar::solid(self.reaches));
        out.extend(crate::spa::solid(self.reaches));
        out.push(crate::spa::sauna_leaf_box(self.reaches, self.sauna_swing));
        out.push(crate::spa::leaf_box(self.reaches, self.baths_swing));
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

    /// What the sight cannot see through.
    ///
    /// The walls, and only the walls. Not the benches, the cabinets or the
    /// bookcases: those are the things being looked at, and a bookcase that
    /// blocks the sight blocks the one book in it that is a handle.
    pub fn opaque(&self) -> &[Aabb] {
        &self.walls
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

        let benches = self.benches.iter().enumerate().filter_map(|(n, one)| {
            slab(from, way, &Self::bench_box(one)).map(|far| (Seen::Bench(n), far))
        });

        // and the one book that is not a book, which is a case sized thing
        // because the book is the handle and the case is the door. Pointing at
        // a single spine on a shelf of four hundred is a thing nobody would
        // ever find; pointing at the case it is in is a thing you can.
        let case = self
            .handle()
            .and_then(|book| slab(from, way, &book).map(|far| (Seen::Case, far)));

        let glass = {
            let leaf = crate::spa::sauna_leaf_box(self.reaches, self.sauna_swing);

            slab(from, way, &leaf).map(|far| (Seen::Sauna, far))
        };
        let baths = {
            let leaf = crate::spa::leaf_box(self.reaches, self.baths_swing);

            slab(from, way, &leaf).map(|far| (Seen::Baths, far))
        };

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
            .iter()
            .filter_map(|box_| slab(from, way, box_))
            .fold(f32::INFINITY, f32::min);

        cabinets
            .chain(shapes)
            .chain(benches)
            .chain(case)
            .chain(glass)
            .chain(baths)
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
        // the walls, the floor, the way down, the spa, the cabinets, the
        // benches, the plinths and the bookcases
        assert_eq!(
            solid.len(),
            room.walls.len()
                + 1
                + cellar::solid(room.reaches).len()
                + crate::spa::solid(room.reaches).len()
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

        // the nook's own row. The flood is a grid at one height and the cellar
        // is at another, so the one down there is not a thing this can answer.
        // Getting to that one is `the_case_is_a_door` and
        // `you_can_get_back_up_the_stair`, which walk it rather than flooding.
        for bench in room.benches.iter().filter(|one| one.at.y >= 0.0) {
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

        // the nook's own row. The one in the cellar is under all of this.
        for bench in room.benches.iter().filter(|one| one.at.y >= 0.0) {
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

        for bench in room.benches.iter().filter(|one| one.at.y >= 0.0) {
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
                    crate::walk::walk(at, way, falling, crate::RADIUS, 1.0 / 60.0, &solid);
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

        for bench in room.benches.iter().filter(|one| one.at.y >= 0.0) {
            assert!(
                bench.at.x < -(WALL + CABINET.x),
                "a bench stood in the aisle at {}",
                bench.at.x
            );
        }
    }

    #[test]
    fn nothing_on_a_bench_stands_in_front_of_a_shape() {
        // the fault that put the benches here: a bench in the aisle is in the
        // way of the far wall from every angle, which is the one place nothing
        // should stand
        let room = Room::of(some(12));

        for bench in &room.benches {
            let box_ = Room::bench_box(bench);
            for shape in &room.displays {
                assert!(
                    box_.max.x < shape.at.x - shape.scale || box_.min.x > shape.at.x + shape.scale,
                    "a bench covers {}",
                    shape.name
                );
            }
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
                + 1
                + cellar::solid(room.reaches).len()
                + crate::spa::solid(room.reaches).len()
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
}
