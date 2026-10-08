//! The spa behind the cellar: a pool you can walk into, a hot tub and a sauna.
//! Spec 0008.
//!
//! The building has had nothing you could be in rather than look at. Water is
//! the first thing in it that answers back, which is why this room is where the
//! engine's spec 0043 arrived.
//!
//! One room holding three things, not three rooms. Everything here is laid out
//! from `cellar::stair_foot` and `cellar::opening`, so the spa moves when the
//! cellar does and the door between them cannot come apart.
//!
//! Nothing here draws or reads a key. Where the pool is, how deep it is and
//! whether you can get out of it are arithmetic.

use crate::cellar;
use crate::room::THICK;
use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

/// How far the room runs along the cellar, how far across, and how high.
///
/// Higher than the cellar. Steam wants somewhere to go, and a low ceiling over
/// water reads as a cellar with a puddle in it.
pub const DEEP: f32 = 18.0;
pub const SPAN: f32 = 12.0;
///
/// Not three, which is what it was and is exactly how far down the cellar
/// goes. The room's ceiling landed at nought, the slab over it stood from
/// nought up to a third of a unit, and spec 0010 then put a garden on that
/// ground: the baths' roof came up through the garden's gravel as a ledge you
/// could not see, could walk up, and could then step off into the koi pond.
///
/// Two thirds and not a third, so the slab's top lands below the garden's
/// ground rather than level with it. Level, the two are coplanar and the
/// roof's own end face stands in the middle of the garden's floor: a body
/// resting exactly on a surface counts as touching it, the sweep against that
/// face is a degenerate one, and what it does is let you most of the way
/// through and then jam. Walking west across the garden stuck on nothing, at
/// the line where the baths below happen to end.
pub const TALL: f32 = 2.4;

/// The way through from the cellar: how wide, how high, and how far along the
/// cellar's wall it is cut.
///
/// In a long wall and not the far one. The far wall is the fireplace, and a
/// door through a chimney breast is a door into a fire.
pub const DOOR: f32 = 1.8;
pub const LINTEL: f32 = 2.2;
pub const DOOR_AT: f32 = 5.0;

/// The pool: how long, how wide, how deep, and where its corner sits from the
/// foot of the stair and the shared wall.
///
/// Shallow enough to stand up in everywhere, which spec 0008 chose over
/// swimming on purpose: out of your depth is a camera and a stroke and a
/// surface you can be under, and this room is about water.
pub const POOL: Vec2 = vec2(8.0, 5.0);
pub const POOL_DEEP: f32 = 1.15;

/// How far the water sits below the coping.
///
/// Every pool has it, and this one needs it for a second reason: at exactly
/// floor level the surface is coplanar with the tiles it meets, and two
/// surfaces in one plane fight for the same pixels. The near edge of this pool
/// came out as a sheet of stripes lying across the floor.
pub const FREEBOARD: f32 = 0.07;
pub const POOL_AT: Vec2 = vec2(8.5, 6.0);

/// The steps down into it: how many, and how far each one reaches out into the
/// water.
pub const STEPS: usize = 3;
pub const TREAD: f32 = 0.42;

/// How fine a grid the water is stepped on. Cells of about eight centimetres
/// across a six unit pool, which is fine enough that a ring reads as a ring.
pub const CELLS: u32 = 80;

/// How quickly the pool goes back to still.
///
/// Slower than the engine's default. A pool is a big body of water in a hard
/// box: a ring crosses it and comes back, and the engine's number is sized for
/// something that settles while you watch.
pub const SETTLES: f32 = 0.3;

/// The hot tub: how far across, how high it stands above the floor, how deep
/// the water in it is, and where it sits.
///
/// Square, and cedar. A heightfield is a rectangle, so a round shell round one
/// is a lie at four corners: either the water runs through the staves or the
/// tub has a rectangle of water floating inside it. A square cedar tub is a
/// thing that exists.
pub const TUB: f32 = 2.2;
pub const TUB_UP: f32 = 0.55;
pub const TUB_DEEP: f32 = 0.42;
pub const TUB_AT: Vec2 = vec2(2.0, 6.0);

/// How far the shell stands above the water, how far the cap over it oversails
/// each way, and the two things that make it something you can get into.
///
/// A tub whose rim is higher than a step is a barrel of hot water. The rim has
/// to be high or it does not read as a tub, so getting in takes a step outside
/// and a seat inside, which is what every one of these has anyway.
pub const TUB_RIM: f32 = 0.12;
pub const TUB_CAP: f32 = 0.1;
/// Two treads and not one. One left you standing on it with the rim still a
/// whole step above you, which is a stile rather than a way in.
pub const TUB_STEP: [f32; 2] = [0.3, 0.58];
pub const TUB_SEAT: f32 = 0.3;
/// How fine a grid the tub's own water is stepped on.
pub const TUB_CELLS: u32 = 24;

/// How much of your pace the water takes, and how hard you push the surface as
/// you go through it.
///
/// The ripples following you about are the whole of why this room exists, so
/// the number that makes them is not a detail.
pub const WADE: f32 = 0.42;

/// How hard you push the water as you go through it.
///
/// It was 2.6 over a radius of three bodies, every frame, which is eighty times
/// what the idle stirring puts in and more than the damping can take out: two
/// seconds of wading and the surface was half a unit off still, which is a wall
/// of streaks from inside it. A wake is body sized and it is a push, not a
/// shove.
///
/// It was then 0.3, which is a push you cannot see: two seconds of wading left
/// the surface a hundredth off still in a pool a unit deep, where one ball
/// dropped into `ripple` leaves a seventh of its depth. The room was built for
/// the ripples following you about and they were below the threshold of being
/// there at all. At 2.0 five minutes of walking up and down holds a seventh of
/// the depth, which is the same share of the pool that a ball makes of its own,
/// and a third of what reads as a wall of streaks.
///
/// How wide is not a number here. A wake is a ring round your own body, inner
/// radius yours and outer radius the engine's `SPLASH` times it, because the
/// water you are standing in is where you are: a disc centred on you holds down
/// the one patch of surface you can never see move, and spends the wake doing
/// it. The pool read as a sheet with a dent following it about.
pub const WAKE: f32 = 2.0;

/// The most of it there can be, per second rather than per frame.
///
/// Per frame it was, and per frame is a cap that is not a cap: twice the frame
/// rate is twice the pushes and so twice the energy, and the pool on a machine
/// drawing at a hundred and twenty was taking in double what it was tested
/// with. It blew up on Jake's screen and not on mine, which is the whole
/// signature of a number that should have been a rate.
///
/// There is a cap at all because stepping up carries you most of a stride in
/// one frame, per `walk`, and `went` is a distance over a time: on that frame
/// it reads as thirty units a second. Without this the first step out of the
/// pool is also the biggest wave in it.
///
/// A multiple of the wading rate and not a number of its own, which is about
/// two and a half times `SPEED · WADE · WAKE`. Set as an absolute it sat just
/// above ordinary wading, so raising `WAKE` moved nothing but the spike: the
/// cap was doing the steering and the number meant to do it was not.
pub const WAKE_MOST: f32 = 8.8;

/// How often the tub blows, and how hard. Not simulated: a hot tub bubbles, and
/// a push in the middle of it is what that looks like from outside.
///
/// Upwards, which is to say a negative push. A jet of bubbles is air on its way
/// out and it carries the water with it, so the middle of a hot tub stands
/// above its own rim and spills. Pushed down instead, the tub dented on the
/// beat like something heavy landing in it, every three and a third seconds,
/// for ever.
///
/// A disc and not a ring, unlike the wake: there is nothing sitting in the
/// middle of a plume.
pub const BLOWS: f32 = 0.35;
pub const BLOWN: f32 = 0.5;

/// The sauna: how big the box is, how tall, and where its middle sits.
pub const SAUNA: Vec3 = vec3(3.4, 2.3, 3.0);
/// In the corner, where a timber box belongs. Stood out in the floor it was an
/// object in a room; in the corner it is part of the building.
pub const SAUNA_AT: Vec2 = vec2(15.9, 1.82);
/// Wide enough to walk through. It was 0.85 for somebody 0.9 across, which is
/// a door you can see and not use, and is the fault the nook's own way in had
/// before anybody measured it.
pub const SAUNA_DOOR: f32 = 1.1;

/// Its two tiers of bench, and the stove.
pub const BENCH: f32 = 0.6;
pub const BENCH_LOW: f32 = 0.45;
pub const BENCH_HIGH: f32 = 0.95;
pub const STOVE: Vec3 = vec3(0.55, 0.8, 0.55);
pub const STONE: f32 = 0.09;

/// The tiling: how high the dado runs, the border course over it, the cornice
/// under the ceiling, and the piers down the long walls.
///
/// A bath house is not a plastered box. Glazed tile to shoulder height with a
/// border over it, plaster above, a cornice, and piers breaking the run: that
/// is what every one of these has ever looked like, and it is four bands and a
/// row of boxes.
pub const DADO: f32 = 1.5;
pub const BAND: f32 = 0.13;
pub const CORNICE: f32 = 0.17;
/// How thick this room's own face on the cellar's wall is.
///
/// Named because the tiling has to start outside it. Taken from the wall behind
/// instead, the dado and the skin fill the same slab and fight for every pixel:
/// a dissolving chequerboard down one side of the room.
pub const SKIN: f32 = 0.05;

pub const PIER: f32 = 0.36;
pub const PIER_OUT: f32 = 0.1;
pub const PIERS: usize = 5;

/// The border laid into the floor, and how far in from the walls it runs.
pub const INLAY: f32 = 0.22;
pub const INLAY_IN: f32 = 0.55;

/// The fittings: how big a sconce is, how far up it sits, and the urns.
pub const SCONCE: Vec3 = vec3(0.26, 0.4, 0.16);
pub const SCONCE_UP: f32 = 2.0;
pub const URN: f32 = 0.46;
pub const URN_TALL: f32 = 0.72;
pub const PLINTH: f32 = 0.5;

/// The fountain on the far wall: the basin, how far it stands out, and the
/// spout over it.
pub const BASIN: Vec3 = vec3(1.5, 0.3, 0.5);
pub const BASIN_UP: f32 = 0.95;
pub const SPOUT: f32 = 0.09;

/// The seat along the wall, which is where you sit between one thing and the
/// next and is the one piece of furniture a bath house actually needs.
pub const SEAT: Vec3 = vec3(2.6, 0.12, 0.6);
pub const SEAT_UP: f32 = 0.46;

/// What a box in this room is, so the drawing can tell a tiled floor from a
/// basin from a timber wall without keeping a second list of them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Made {
    /// The tiled floor of the room, and the coping round the pool.
    Tile,
    /// Plastered wall and ceiling.
    Wall,
    /// The inside of the pool: its floor and the four sides of the hole.
    Basin,
    /// A step down into it.
    Step,
    /// The tub's own shell.
    Tub,
    /// The sauna's timber.
    Timber,
    /// A bench in the sauna.
    Bench,
    /// The stove.
    Stove,
    /// Glazed tile up the walls to shoulder height.
    Dado,
    /// The border course over it, and the cornice under the ceiling.
    Band,
    /// A pier breaking the run of a long wall.
    Pier,
    /// The border laid into the floor.
    Inlay,
    /// A fitting in brass: a sconce's bracket, the fountain's spout.
    Brass,
    /// Cut stone: the plinths, the seat, the fountain's basin.
    Cut,
}

/// The spa's own side of the wall it shares with the cellar, which is the wall
/// the way through is cut in.
///
/// Everything in this room is measured from here and from the foot of the
/// stair, so the spa moves when the cellar does.
pub fn near(reaches: f32) -> f32 {
    cellar::opening(reaches) + cellar::SPAN * 0.5 + THICK * 0.5
}

/// Where the way through stands, in the middle of the doorway.
pub fn doorway(reaches: f32) -> Vec3 {
    vec3(
        cellar::stair_foot() - DOOR_AT,
        -cellar::DOWN,
        near(reaches) - THICK * 0.5,
    )
}

/// The middle of the room's floor.
pub fn at(reaches: f32) -> Vec3 {
    vec3(
        cellar::stair_foot() - DEEP * 0.5,
        -cellar::DOWN,
        near(reaches) + SPAN * 0.5,
    )
}

/// The still surface of the pool: its middle, how far it reaches, and how deep
/// it stands. This is what the engine's `Water` is built from.
pub fn pool(reaches: f32) -> (Vec3, Vec2, f32) {
    (
        vec3(
            cellar::stair_foot() - POOL_AT.x,
            -cellar::DOWN - FREEBOARD,
            near(reaches) + POOL_AT.y,
        ),
        POOL,
        POOL_DEEP - FREEBOARD,
    )
}

/// The still surface of the tub: its middle, how far across, and how deep.
pub fn tub(reaches: f32) -> (Vec3, f32, f32) {
    (
        vec3(
            cellar::stair_foot() - TUB_AT.x,
            -cellar::DOWN + TUB_UP,
            near(reaches) + TUB_AT.y,
        ),
        TUB,
        TUB_DEEP,
    )
}

/// The sauna's box, which is where its timber stands.
pub fn sauna(reaches: f32) -> Aabb {
    Aabb::from_center_size(
        vec3(
            cellar::stair_foot() - SAUNA_AT.x,
            -cellar::DOWN + SAUNA.y * 0.5,
            near(reaches) + SAUNA_AT.y,
        ),
        SAUNA,
    )
}

/// The rectangle the pool's hole takes out of the floor.
fn hole(reaches: f32) -> (f32, f32, f32, f32) {
    let (middle, size, _) = pool(reaches);

    (
        middle.x - size.x * 0.5,
        middle.x + size.x * 0.5,
        middle.z - size.y * 0.5,
        middle.z + size.y * 0.5,
    )
}

/// Every box the room is built of, and what each one is.
///
/// The wall it shares with the cellar is the cellar's own, cut round the way
/// through by `cellar::shell`. Built here as well it would be two walls in one
/// place, which is the stippled grey rectangle this building has already drawn
/// twice.
pub fn built(reaches: f32) -> Vec<(Aabb, Made)> {
    let foot = cellar::stair_foot();
    let side = near(reaches);
    let floor = -cellar::DOWN;
    let back = foot - DEEP;
    let far_side = side + SPAN;
    let (from_x, to_x, from_z, to_z) = hole(reaches);
    let mut out = Vec::new();

    // the floor, in four slabs round the hole the pool takes out of it. One
    // slab with a hole in it is not a thing a box can be, and four is what the
    // nook's back wall already does round the way down.
    for (one, two, three, four) in [
        (back, from_x, side, far_side),
        (to_x, foot, side, far_side),
        (from_x, to_x, side, from_z),
        (from_x, to_x, to_z, far_side),
    ] {
        if two - one <= 1e-4 || four - three <= 1e-4 {
            continue;
        }

        out.push((
            Aabb::from_center_size(
                vec3((one + two) * 0.5, floor - THICK * 0.5, (three + four) * 0.5),
                vec3(two - one, THICK, four - three),
            ),
            Made::Tile,
        ));
    }

    // the basin: its floor, and the four sides of the hole, which are what you
    // see of the pool from anywhere in the room
    out.push((
        Aabb::from_center_size(
            vec3(
                (from_x + to_x) * 0.5,
                floor - POOL_DEEP - THICK * 0.5,
                (from_z + to_z) * 0.5,
            ),
            vec3(to_x - from_x, THICK, to_z - from_z),
        ),
        Made::Basin,
    ));
    // the sides stop at the underside of the floor rather than level with its
    // top. Level with it they are coplanar with every tile they meet, and two
    // opaque surfaces in one plane are the stripes that ran round this pool.
    let wall = POOL_DEEP - THICK;
    let middle_y = floor - THICK - wall * 0.5;
    for (middle, size) in [
        (
            vec3(from_x - THICK * 0.5, middle_y, (from_z + to_z) * 0.5),
            vec3(THICK, wall, to_z - from_z),
        ),
        (
            vec3(to_x + THICK * 0.5, middle_y, (from_z + to_z) * 0.5),
            vec3(THICK, wall, to_z - from_z),
        ),
        (
            vec3((from_x + to_x) * 0.5, middle_y, from_z - THICK * 0.5),
            vec3(to_x - from_x, wall, THICK),
        ),
        (
            vec3((from_x + to_x) * 0.5, middle_y, to_z + THICK * 0.5),
            vec3(to_x - from_x, wall, THICK),
        ),
    ] {
        out.push((Aabb::from_center_size(middle, size), Made::Basin));
    }

    // the steps down into it, on the side you arrive at. Each one reaches
    // further out into the water than the one above it, which is the way round
    // a flight is: written the other way the treads nest inside each other and
    // the whole thing is one block with a single rise of two steps, which is
    // what you cannot climb out of.
    //
    // One more riser than there are treads, because the basin's own floor is
    // the bottom of the flight. Counted as one tread per riser the lowest tread
    // sat exactly on the floor and was a box of no height at all.
    let riser = POOL_DEEP / (STEPS + 1) as f32;
    for n in 0..STEPS {
        let top = floor - (n as f32 + 1.0) * riser;
        let reach = (n as f32 + 1.0) * TREAD;

        out.push((
            Aabb::from_center_size(
                vec3(
                    (from_x + to_x) * 0.5,
                    (top + floor - POOL_DEEP) * 0.5,
                    from_z + reach * 0.5,
                ),
                vec3(to_x - from_x, top - (floor - POOL_DEEP), reach),
            ),
            Made::Step,
        ));
    }

    // the near side, where the cellar is. The cellar's own wall closes the part
    // of it the cellar reaches, and that is ten of eighteen: past the cellar's
    // far end this room has no near wall at all, and you walked out of the
    // building through the gap. The cellar is also lower than this room, so
    // there is a strip over its wall as well.
    let cellar_ends = foot - cellar::DEEP;
    let over = floor + cellar::TALL;
    if cellar_ends - back > 1e-4 {
        out.push((
            Aabb::from_center_size(
                vec3(
                    (back + cellar_ends) * 0.5,
                    floor + TALL * 0.5,
                    side - THICK * 0.5,
                ),
                vec3(cellar_ends - back, TALL, THICK),
            ),
            Made::Wall,
        ));
    }
    if floor + TALL - over > 1e-4 {
        out.push((
            Aabb::from_center_size(
                vec3(
                    (cellar_ends + foot) * 0.5,
                    (over + floor + TALL) * 0.5,
                    side - THICK * 0.5,
                ),
                vec3(foot - cellar_ends, floor + TALL - over, THICK),
            ),
            Made::Wall,
        ));
    }

    // and this room's own face on the cellar's wall, in three pieces round the
    // way through. A wall between two rooms is finished on both sides: left as
    // the cellar's, the baths had a mahogany wall down one side, which is the
    // cellar's vibe in the wrong room.
    let door = doorway(reaches);
    for across in [-1.0f32, 1.0] {
        let middle = if across < 0.0 {
            (cellar_ends + door.x - DOOR * 0.5) * 0.5
        } else {
            (door.x + DOOR * 0.5 + foot) * 0.5
        };
        let long = if across < 0.0 {
            door.x - DOOR * 0.5 - cellar_ends
        } else {
            foot - door.x - DOOR * 0.5
        };

        if long > 1e-4 {
            out.push((
                Aabb::from_center_size(
                    vec3(middle, floor + TALL * 0.5, side + SKIN * 0.5),
                    vec3(long, TALL, SKIN),
                ),
                Made::Wall,
            ));
        }
    }
    out.push((
        Aabb::from_center_size(
            vec3(
                door.x,
                floor + LINTEL + (TALL - LINTEL) * 0.5,
                side + SKIN * 0.5,
            ),
            vec3(DOOR, TALL - LINTEL, SKIN),
        ),
        Made::Wall,
    ));

    // three walls and a ceiling. The fourth is the cellar's.
    for (middle, size) in [
        (
            vec3(
                back - THICK * 0.5,
                floor + TALL * 0.5,
                (side + far_side) * 0.5,
            ),
            vec3(THICK, TALL, SPAN + THICK * 2.0),
        ),
        (
            vec3(
                foot + THICK * 0.5,
                floor + TALL * 0.5,
                (side + far_side) * 0.5,
            ),
            vec3(THICK, TALL, SPAN + THICK * 2.0),
        ),
        (
            vec3(
                (back + foot) * 0.5,
                floor + TALL * 0.5,
                far_side + THICK * 0.5,
            ),
            vec3(DEEP, TALL, THICK),
        ),
        (
            vec3(
                (back + foot) * 0.5,
                floor + TALL + THICK * 0.5,
                (side + far_side) * 0.5,
            ),
            vec3(DEEP, THICK, SPAN),
        ),
    ] {
        out.push((Aabb::from_center_size(middle, size), Made::Wall));
    }

    out.extend(cedar(reaches));
    out.extend(timber(reaches));
    out
}

/// The hot tub: four staved sides on a base, standing on the floor rather than
/// sunk into it.
fn cedar(reaches: f32) -> Vec<(Aabb, Made)> {
    let (surface, wide, deep) = tub(reaches);
    let floor = -cellar::DOWN;
    let side = THICK * 0.45;
    let rim = surface.y + TUB_RIM;
    let mut out = Vec::new();

    // the base it stands on, under the water
    out.push((
        Aabb::from_center_size(
            vec3(surface.x, (floor + surface.y - deep) * 0.5, surface.z),
            vec3(
                wide + side * 2.0,
                (surface.y - deep - floor).max(0.05),
                wide + side * 2.0,
            ),
        ),
        Made::Tub,
    ));

    // the four staved sides
    for (along, across) in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
        let long = if along == 0.0 { wide } else { side };
        let other = if along == 0.0 {
            side
        } else {
            wide + side * 2.0
        };

        out.push((
            Aabb::from_center_size(
                vec3(
                    surface.x + along * (wide + side) * 0.5,
                    (floor + rim) * 0.5,
                    surface.z + across * (wide + side) * 0.5,
                ),
                vec3(long, rim - floor, other),
            ),
            Made::Tub,
        ));
    }

    // a cap over them, oversailing both ways. It is what a cedar tub has, and
    // it is also the only way the rim is something you can stand on: the staves
    // alone are a hand's width, and nobody is a hand wide.
    for (along, across) in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
        let thick = side + TUB_CAP * 2.0;
        let long = if along == 0.0 {
            wide + TUB_CAP * 2.0
        } else {
            thick
        };
        let other = if along == 0.0 {
            thick
        } else {
            wide + side * 2.0 + TUB_CAP * 2.0
        };

        out.push((
            Aabb::from_center_size(
                vec3(
                    surface.x + along * (wide + side) * 0.5,
                    rim + 0.03,
                    surface.z + across * (wide + side) * 0.5,
                ),
                vec3(long, 0.06, other),
            ),
            Made::Tub,
        ));
    }

    // the steps up to it, on the side the pool is, the upper one nearer
    let face = surface.z - (wide + side) * 0.5;
    for (n, up) in TUB_STEP.iter().enumerate() {
        let deep = 0.34;
        let back = face - deep * (TUB_STEP.len() - n) as f32;

        out.push((
            Aabb::from_center_size(
                vec3(surface.x, floor + up * 0.5, back + deep * 0.5),
                vec3(wide * 0.7, *up, deep),
            ),
            Made::Tub,
        ));
    }

    // and the seat inside, round two sides, which is where you sit and is the
    // tread on the way down
    for across in [-1.0f32, 1.0] {
        out.push((
            Aabb::from_center_size(
                vec3(
                    surface.x,
                    floor + TUB_SEAT * 0.5,
                    surface.z + across * (wide * 0.5 - 0.22),
                ),
                vec3(wide, TUB_SEAT, 0.44),
            ),
            Made::Tub,
        ));
    }

    out
}

/// The sauna: four walls round a doorway, a roof, two benches and a stove.
fn timber(reaches: f32) -> Vec<(Aabb, Made)> {
    let box_ = sauna(reaches);
    let middle = box_.center();
    let (lo, hi) = (middle - box_.size() * 0.5, middle + box_.size() * 0.5);
    let wall = THICK * 0.5;
    let mut out = Vec::new();

    // three walls whole and a roof
    for (at, size) in [
        (
            vec3(lo.x - wall * 0.5, middle.y, middle.z),
            vec3(wall, SAUNA.y, SAUNA.z),
        ),
        (
            vec3(middle.x, middle.y, lo.z - wall * 0.5),
            vec3(SAUNA.x, SAUNA.y, wall),
        ),
        (
            vec3(middle.x, middle.y, hi.z + wall * 0.5),
            vec3(SAUNA.x, SAUNA.y, wall),
        ),
        (
            vec3(middle.x, hi.y + wall * 0.5, middle.z),
            vec3(SAUNA.x + wall * 2.0, wall, SAUNA.z + wall * 2.0),
        ),
    ] {
        out.push((Aabb::from_center_size(at, size), Made::Timber));
    }

    // and the fourth, facing back the way you came in, in two pieces with the
    // way in between them
    let rest = (SAUNA.z - SAUNA_DOOR) * 0.5;
    for across in [-1.0f32, 1.0] {
        out.push((
            Aabb::from_center_size(
                vec3(
                    hi.x + wall * 0.5,
                    middle.y,
                    middle.z + across * (SAUNA_DOOR + rest) * 0.5,
                ),
                vec3(wall, SAUNA.y, rest),
            ),
            Made::Timber,
        ));
    }

    // two tiers of bench along the back
    for up in [BENCH_LOW, BENCH_HIGH] {
        out.push((
            Aabb::from_center_size(
                vec3(middle.x, lo.y + up, lo.z + BENCH * 0.5 + wall),
                vec3(SAUNA.x - wall * 2.0, 0.09, BENCH),
            ),
            Made::Bench,
        ));
    }

    // and the stove in the corner by the door
    out.push((
        Aabb::from_center_size(
            vec3(
                hi.x - STOVE.x * 0.5 - wall,
                lo.y + STOVE.y * 0.5,
                hi.z - STOVE.z * 0.5 - wall,
            ),
            STOVE,
        ),
        Made::Stove,
    ));

    out
}

/// How many sconces run down each long wall, what they throw, and how far.
///
/// Every lamp in this room sits in one. A light with no fitting is a bright
/// patch on a wall and no reason for it, which the cellar was told off for, and
/// this room had it worse: the lamps hung along the ceiling and the sconces
/// stood on a wall, so the ceiling glowed at nothing and the sconces were dark.
pub const SCONCES: usize = 4;
pub const LAMP_LIT: f32 = 0.62;
pub const LAMP_RANGE: f32 = 7.0;

/// And the sauna's own, which is the opposite: close, low and orange, so that
/// stepping into it is stepping into a different warmth.
pub const SAUNA_LIT: f32 = 0.55;
pub const SAUNA_RANGE: f32 = 3.2;

/// Where the sconces hang: down both long walls, clear of the way through.
pub fn sconces(reaches: f32) -> Vec<Vec3> {
    let foot = cellar::stair_foot();
    let side = near(reaches);
    let far_side = side + SPAN;
    let floor = -cellar::DOWN;
    let door = doorway(reaches);
    let mut out = Vec::new();

    for (face, into) in [(side + SKIN, 1.0f32), (far_side, -1.0)] {
        for n in 0..SCONCES {
            let along = foot - DEEP * (n as f32 + 0.5) / SCONCES as f32;

            // never in the doorway, which is the one piece of wall that has to
            // stay empty
            if into > 0.0 && (along - door.x).abs() < DOOR * 0.5 + SCONCE.x {
                continue;
            }

            out.push(vec3(
                along,
                floor + SCONCE_UP,
                face + into * (SCONCE.z * 0.5 + PIER_OUT),
            ));
        }
    }

    out
}

/// Where every lamp in the room hangs, which is in a sconce and nowhere else.
///
/// Derived rather than listed. Two lists of where the light comes from is two
/// lists that drift, and these two did: the lamps were along the ceiling and
/// the fittings were on the walls.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    sconces(reaches)
        .into_iter()
        .map(|at| at + Vec3::Y * SCONCE.y * 0.55)
        .collect()
}

/// Where the fountain's stream leaves the spout, and how far it falls.
///
/// A fountain that does not run is a stone shelf. The water is drawn and not
/// simulated: a spout's stream is a thread of water a centimetre across, and
/// the engine's heightfield carries nothing of the kind.
pub const STREAM: f32 = 0.045;

pub fn stream(reaches: f32) -> (Vec3, f32) {
    let at = fountain(reaches);
    let from = vec3(at.x, at.y + 0.5, at.z - SPOUT * 1.6);

    (from, 0.5 - BASIN.y * 0.3)
}

/// Where the sauna's own lamp sits: over the stove, which is where the light in
/// one of these comes from.
pub fn stove_lamp(reaches: f32) -> Vec3 {
    let box_ = sauna(reaches);
    let (lo, hi) = (
        box_.center() - box_.size() * 0.5,
        box_.center() + box_.size() * 0.5,
    );

    vec3(
        hi.x - STOVE.x * 0.5 - THICK * 0.25,
        lo.y + STOVE.y + STONE,
        hi.z - STOVE.z * 0.5 - THICK * 0.25,
    )
}

/// The sauna's door: how thick the glass is, how wide a frame it carries, how
/// far it stands clear of the floor and the lintel, and how long it takes.
///
/// Glass, because a sauna door is glass. It is also the one thing that makes
/// the room read as a room you are inside rather than a box you are in: shut
/// behind you, the rest of the spa is still there and is on the other side of
/// something.
pub const PANE: f32 = 0.045;
pub const FRAME: f32 = 0.08;
pub const SAUNA_CLEARS: f32 = 0.06;
pub const SAUNA_SWINGS: f32 = 0.8;

/// Where the door's leaf stands and how it is turned, part way through its
/// swing, as a middle, a turn and its half extents.
///
/// Hinged on one edge of the opening and opening out into the room, which is
/// the same arithmetic the bookcases use and for the same reason: a leaf that
/// slides is a lift door.
pub fn sauna_leaf(reaches: f32, swing: f32) -> (Vec3, glam::Quat, Vec3) {
    let box_ = sauna(reaches);
    let middle = box_.center();
    let hi = middle + box_.size() * 0.5;
    let lo = middle - box_.size() * 0.5;

    let tall = SAUNA.y - SAUNA_CLEARS * 2.0;
    let half = vec3(PANE * 0.5, tall * 0.5, SAUNA_DOOR * 0.5);
    let shut = vec3(
        hi.x + PANE * 0.5,
        lo.y + SAUNA_CLEARS + tall * 0.5,
        middle.z,
    );
    let hinge = vec3(shut.x, shut.y, middle.z - SAUNA_DOOR * 0.5);
    let turn = glam::Quat::from_rotation_y(swing * std::f32::consts::FRAC_PI_2);

    (hinge + turn * (shut - hinge), turn, half)
}

/// The box that leaf fills, part way through its swing.
pub fn sauna_leaf_box(reaches: f32, swing: f32) -> Aabb {
    let (middle, turn, half) = sauna_leaf(reaches, swing);
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));

    for across in [-1.0f32, 1.0] {
        for along in [-1.0f32, 1.0] {
            let corner = middle + turn * vec3(across * half.x, 0.0, along * half.z);
            lo = lo.min(corner);
            hi = hi.max(corner);
        }
    }

    Aabb::from_center_size(
        vec3((lo.x + hi.x) * 0.5, middle.y, (lo.z + hi.z) * 0.5),
        vec3(hi.x - lo.x, half.y * 2.0, hi.z - lo.z),
    )
}

/// The steam: how many puffs, how wide one gets, how long it lives, how far it
/// rises, and the most of any one of them there ever is.
///
/// Eight fat ones at a sixth opacity read as eight balls, which is what they
/// were. Steam is not made of objects: it is a lot of very faint things that
/// overlap, and what you see is where they pile up. So: three times as many, a
/// third the size, and a third of the opacity each.
pub const PUFFS: usize = 26;
pub const PUFF: f32 = 0.24;
pub const PUFF_LIVES: f32 = 7.0;
pub const PUFF_RISES: f32 = 1.7;
pub const PUFF_MOST: f32 = 0.055;

/// Where the steam over a surface is now: each puff as its middle, how big it
/// is, and how much of it there is.
///
/// A pure function of the time, so there is nothing to keep, nothing to seed
/// and nothing to reset. Each puff is on its own clock, offset by the golden
/// ratio so they neither pulse together nor ever line up.
///
/// Not particles. The engine has no particle system and this does not need
/// one: steam over a tub is a lot of soft things going up slowly.
pub fn steam(over: Vec3, wide: f32, since: f32) -> Vec<(Vec3, Vec3, f32)> {
    const GOLDEN: f32 = 0.618_034;

    (0..PUFFS)
        .map(|n| {
            let seed = (n as f32 + 1.0) * GOLDEN;
            let life = ((since / PUFF_LIVES) + seed).fract();

            // where it started, which does not move with the clock, and the
            // drift it picks up as it goes. Steam spreads as it rises, so the
            // drift is on the life and the start is not.
            let turn = seed * std::f32::consts::TAU;
            let away = wide * 0.34 * (seed * 7.0).fract();
            let drift = life * life * wide * 0.3;

            let at = over
                + vec3(
                    turn.cos() * (away + drift),
                    life * PUFF_RISES,
                    turn.sin() * (away + drift),
                );

            // no two the same size, and all of them wider than they are tall.
            // A ball reads as a ball however faint it is.
            let fat = 0.6 + (seed * 13.0).fract() * 0.9;
            let size = PUFF * fat * (0.5 + life * 1.6);

            // in over the first fifth of its life and out over the rest, so
            // nothing appears or vanishes
            let thick = if life < 0.2 {
                life / 0.2
            } else {
                1.0 - (life - 0.2) / 0.8
            };

            (
                at,
                vec3(size, size * (0.45 + (seed * 5.0).fract() * 0.3), size),
                PUFF_MOST * thick,
            )
        })
        .collect()
}

/// How long the sauna takes to fill with steam, and how long to clear.
///
/// Shut, it fills; open, it empties, and much faster. A door is a hole and
/// steam is lighter than air, so a sauna left open is a sauna that is cold.
pub const FILLS: f32 = 30.0;
pub const CLEARS: f32 = 6.0;

/// The fug: how many puffs it is made of and how thick any one of them gets.
pub const FUG: usize = 44;
pub const FUG_MOST: f32 = 0.075;

/// The steam standing in the sauna: each puff as its middle, its size and how
/// much of it there is.
///
/// It fills from the ceiling down. Steam rises, so a room fills like a glass
/// held upside down under a tap: the top goes first and the floor last, and
/// that is why you sit on the high bench.
///
/// A pure function of the clock and of how full the room is, so there is
/// nothing kept here either.
pub fn fug(room: Aabb, since: f32, full: f32) -> Vec<(Vec3, Vec3, f32)> {
    const GOLDEN: f32 = 0.618_034;
    let full = full.clamp(0.0, 1.0);
    if full <= 0.0 {
        return Vec::new();
    }

    let middle = room.center();
    let half = room.size() * 0.5;

    (0..FUG)
        .filter_map(|n| {
            let seed = (n as f32 + 1.0) * GOLDEN;
            let up = (seed * 3.0).fract();

            // the top fills first, so a puff is there only once the fug has
            // reached down as far as it sits
            if up < 1.0 - full {
                return None;
            }

            let drift = (since * 0.11 + seed * 7.0).sin() * 0.3;
            let at = middle
                + vec3(
                    ((seed * 11.0).fract() - 0.5) * 2.0 * half.x * 0.78 + drift * half.x * 0.2,
                    (up - 0.5) * 2.0 * half.y * 0.82,
                    ((seed * 17.0).fract() - 0.5) * 2.0 * half.z * 0.78 - drift * half.z * 0.2,
                );
            let size = 0.3 + (seed * 13.0).fract() * 0.45;

            // thickest where it has stood longest, which is the top, and
            // thinning at the edge of where it has reached
            let reached = ((up - (1.0 - full)) / full.max(1e-3)).min(1.0);

            Some((at, vec3(size, size * 0.7, size), FUG_MOST * reached * full))
        })
        .collect()
}

/// How often the water is stirred by nothing in particular, and how hard.
///
/// Still water in a heightfield is perfectly flat, and perfectly flat water is
/// a sheet of tinted glass. Real water in a room is never still: the pump, the
/// air, somebody two rooms away. This is that, and it is the difference between
/// a pool and a blue rectangle.
/// Often, small and weak. It was one push a second over a radius of a unit and
/// a half, which is a wavelength the size of the pool: the surface came out as
/// a slow waved blanket rather than water. A ripple is short, and a short
/// ripple needs a small source.
pub const STIRS: f32 = 0.22;
pub const STIRRED: f32 = 0.055;
pub const STIR_WIDE: f32 = 0.34;

/// How fast a wave crosses this pool, which is quicker than the engine's own
/// number. Slow waves on a big surface read as cloth.
pub const RUNS: f32 = 3.4;

/// Where the next idle stir lands, which wanders rather than repeating.
pub fn stirred(middle: Vec3, size: Vec2, since: f32) -> Vec3 {
    let turn = since * 0.7;
    let away = (since * 0.31).sin() * 0.5 + 0.5;

    middle
        + vec3(
            turn.cos() * size.x * 0.42 * away,
            0.0,
            (turn * 1.37).sin() * size.y * 0.42,
        )
}

/// What the sign over the door says. Spec 0008.
///
/// Not "spa", which is what the thing behind the door is and not what anybody
/// would have written on it. A basement with a pool, a hot tub and a steam room
/// in it is the baths, and that is a word you can cut into a tile.
pub const SAYS: &str = "THE BATHS";

/// The sign: how big the panel is, how far it stands off the wall, how tall the
/// lettering is drawn and how much of the panel it crosses.
///
/// A tiled panel and not neon and not a painted plank. The hall is neon because
/// the hall is a hall of neon cabinets; the nook is a plank on chains because
/// the nook is a study. This door leads to a tiled room full of steam, so the
/// sign is what a bath house has always had: glazed tile, a border round it,
/// and the name cut into it dark.
/// Sized to the wall it is on. Between the lintel at 2.2 and the cellar's own
/// ceiling at 2.7 there is half a unit, and the first panel was 0.52 tall
/// hung 0.3 clear: it sat inside the ceiling, where nobody has ever read a
/// sign.
pub const SIGN: Vec2 = vec2(2.0, 0.34);
pub const SIGN_OUT: f32 = 0.04;
pub const SIGN_EDGE: f32 = 0.05;
pub const SIGN_UP: f32 = 0.06;
pub const TEXELS: f32 = 64.0;
pub const LETTERS: f32 = 0.17;

/// The door's own joinery: the stiles up its sides, the rails across it, and
/// how far down the middle rail sits.
pub const STILE: f32 = 0.13;
pub const DOOR_RAIL: f32 = 0.15;
pub const MID_RAIL: f32 = 0.12;
pub const MID_AT: f32 = 0.36;

/// The door itself: how thick, how far it clears the floor and the lintel, and
/// how long it takes to swing.
///
/// Heavier than the sauna's glass and lighter than a bookcase with four hundred
/// books in it, which is what a door between two rooms is.
pub const LEAF: f32 = 0.07;
pub const LEAF_CLEARS: f32 = 0.04;
pub const SWINGS: f32 = 1.0;

/// Where the sign hangs: on the cellar's side of the wall, over the doorway.
///
/// On the cellar's side because that is where it is read. A sign inside the
/// room it names is read only by somebody already standing in it, which is the
/// thing the nook's own sign was placed to avoid.
pub fn sign(reaches: f32) -> Vec3 {
    let door = doorway(reaches);

    vec3(
        door.x,
        -cellar::DOWN + LINTEL + SIGN_UP + SIGN.y * 0.5,
        // clear of the panelling and not merely clear of the wall. The boards
        // stand 0.05 off the stone and the first sign stood 0.02 off it, so the
        // panelling was in front of the sign and ate the bottom half of it.
        door.z - THICK * 0.5 - cellar::BOARD_OUT - SIGN_OUT * 0.5,
    )
}

/// The architrave round the door on the cellar's side, as boards: a middle and
/// a size each.
///
/// The panelling is cut away for the opening, which left the cellar's own
/// stone showing round it: a grey frame in a mahogany room. Every panelled room
/// ever built lines its openings, for exactly this reason.
pub const ARCH: f32 = 0.17;
pub const ARCH_OUT: f32 = 0.07;

pub fn architrave(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let door = doorway(reaches);
    let floor = -cellar::DOWN;
    let face = door.z - THICK * 0.5 - cellar::BOARD_OUT - ARCH_OUT * 0.5;
    let head = floor + LINTEL;
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        out.push((
            vec3(
                door.x + side * (DOOR + ARCH) * 0.5,
                floor + (head + ARCH) * 0.5,
                face,
            ),
            vec3(ARCH, head + ARCH, ARCH_OUT),
        ));
    }
    out.push((
        vec3(door.x, head + ARCH * 0.5, face),
        vec3(DOOR + ARCH * 2.0, ARCH, ARCH_OUT),
    ));

    // and the reveal, which is the stone you see edge on walking through
    for side in [-1.0f32, 1.0] {
        out.push((
            vec3(door.x + side * DOOR * 0.5, floor + head * 0.5, door.z),
            vec3(0.03, head, THICK),
        ));
    }
    out.push((vec3(door.x, head - 0.015, door.z), vec3(DOOR, 0.03, THICK)));

    out
}

/// Where the door's leaf stands and how it is turned, part way through its
/// swing, as a middle, a turn and its half extents.
///
/// Hinged on one edge of the opening and opening into the spa, so it never
/// swings into the cellar where the barrels are.
pub fn leaf(reaches: f32, swing: f32) -> (Vec3, glam::Quat, Vec3) {
    let door = doorway(reaches);
    let tall = LINTEL - LEAF_CLEARS * 2.0;
    let half = vec3(DOOR * 0.5, tall * 0.5, LEAF * 0.5);
    let shut = vec3(
        door.x,
        door.y + LEAF_CLEARS + tall * 0.5,
        door.z + THICK * 0.5 + LEAF * 0.5,
    );
    let hinge = vec3(shut.x - DOOR * 0.5, shut.y, shut.z);
    let turn = glam::Quat::from_rotation_y(-swing * std::f32::consts::FRAC_PI_2);

    (hinge + turn * (shut - hinge), turn, half)
}

/// The box that leaf fills, part way through its swing.
pub fn leaf_box(reaches: f32, swing: f32) -> Aabb {
    let (middle, turn, half) = leaf(reaches, swing);
    let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));

    for across in [-1.0f32, 1.0] {
        for along in [-1.0f32, 1.0] {
            let corner = middle + turn * vec3(across * half.x, 0.0, along * half.z);
            lo = lo.min(corner);
            hi = hi.max(corner);
        }
    }

    Aabb::from_center_size(
        vec3((lo.x + hi.x) * 0.5, middle.y, (lo.z + hi.z) * 0.5),
        vec3(hi.x - lo.x, half.y * 2.0, hi.z - lo.z),
    )
}

/// Whether somebody standing here is in the pool.
///
/// Over it and under its surface. Over it alone is standing on the coping
/// looking down, which is not wading. Taken from the still surface rather than
/// from the water itself: the difference is a centimetre or two of wave, and a
/// pace that flickers with the swell is worse than one that does not.
pub fn wading(reaches: f32, at: Vec3) -> bool {
    let (middle, size, _) = pool(reaches);

    (at.x - middle.x).abs() <= size.x * 0.5
        && (at.z - middle.z).abs() <= size.y * 0.5
        && at.y < middle.y - 0.05
}

/// How fast you get about from here.
pub fn pace(reaches: f32, at: Vec3, walking: f32) -> f32 {
    if wading(reaches, at) {
        walking * WADE
    } else {
        walking
    }
}

/// How big a tile in the basin is, which is the whole of where the depth comes
/// from.
///
/// Not the water. What tells your eye how far down the bottom is, is seeing
/// something of a known size through it and watching that something get
/// smaller. A flat colour on the bottom of a pool is a flat colour at any
/// depth, which is why this one read as a blanket however well the surface
/// moved.
pub const TILE: f32 = 0.25;

/// The basin's inner faces, each as a middle, how far it reaches across and
/// down, and which way it looks.
///
/// Laid as quads and not as the boxes behind them: a cube's texture runs nought
/// to one on every face however big the face is, so a tiled texture on the
/// basin would be one tile four units across. A quad can be given its own
/// count.
pub fn lining(reaches: f32) -> Vec<(Vec3, Vec2, Vec3)> {
    let (middle, size, deep) = pool(reaches);
    let floor = middle.y + FREEBOARD;
    let (half_x, half_z) = (size.x * 0.5, size.y * 0.5);
    let down = deep + FREEBOARD;
    let proud = 0.006;

    vec![
        // the bottom, looking up
        (
            vec3(middle.x, floor - down + proud, middle.z),
            size,
            Vec3::Y,
        ),
        // and the four sides, each looking in across the water
        (
            vec3(middle.x - half_x + proud, floor - down * 0.5, middle.z),
            vec2(size.y, down),
            Vec3::X,
        ),
        (
            vec3(middle.x + half_x - proud, floor - down * 0.5, middle.z),
            vec2(size.y, down),
            Vec3::NEG_X,
        ),
        (
            vec3(middle.x, floor - down * 0.5, middle.z - half_z + proud),
            vec2(size.x, down),
            Vec3::Z,
        ),
        (
            vec3(middle.x, floor - down * 0.5, middle.z + half_z - proud),
            vec2(size.x, down),
            Vec3::NEG_Z,
        ),
    ]
}

/// Where the urns stand, where the sconces hang and where the fountain is.
///
/// Apart from the boxes, because these are thrown on a wheel rather than sawn.
/// Boxes are what the cellar's barrels and bottles were before anybody turned
/// them, and Jake read them off as placeholders both times.
pub fn urns(reaches: f32) -> Vec<Vec3> {
    let middle = at(reaches);
    let far_side = near(reaches) + SPAN;
    let floor = -cellar::DOWN;

    [-1.0f32, 1.0]
        .iter()
        .map(|across| {
            vec3(
                middle.x + across * (DEEP * 0.5 - PLINTH * 1.3),
                floor + PLINTH + URN_TALL * 0.5,
                far_side - PLINTH * 1.1,
            )
        })
        .collect()
}

pub fn fountain(reaches: f32) -> Vec3 {
    vec3(
        at(reaches).x,
        -cellar::DOWN + BASIN_UP,
        near(reaches) + SPAN - PIER_OUT - BASIN.z * 0.5,
    )
}

/// An urn: a foot, a belly, a neck and a lip, turned about its own axis.
///
/// Two sided, because an urn is open at the top and the inside of the far wall
/// is what you see looking into one. Culled it is a hole with nothing in it.
pub fn urn_mesh() -> blitzkit::mesh::MeshData {
    cellar::turned(26, 28, |v| {
        let wide = if v < 0.07 {
            // the foot, tapering in to the stem
            0.56 - v / 0.07 * 0.16
        } else if v < 0.74 {
            let t = (v - 0.07) / 0.67;

            // the belly, fullest a third of the way up and closing to the neck
            0.4 + (t * std::f32::consts::PI).sin() * 0.6 - t * 0.36
        } else {
            let t = (v - 0.74) / 0.26;

            // and the lip, which flares
            0.44 + t * t * 0.3
        };

        (v - 0.5, wide)
    })
    .two_sided()
}

/// A sconce: a bowl that opens upward, which is what throws light at a ceiling.
pub fn sconce_mesh() -> blitzkit::mesh::MeshData {
    cellar::turned(20, 12, |v| {
        let wide = 0.3 + (v * std::f32::consts::FRAC_PI_2).sin() * 0.7;

        (v - 0.5, wide)
    })
    .two_sided()
}

/// The stream from the spout: a tube open at both ends, necking in as it goes.
///
/// Its own mesh because the nearest one to hand was the candles', and that is
/// `lidded`: it spends the first and last tenth of itself closing to a point,
/// so the fountain poured a pencil. Water falling necks in as it speeds up,
/// which is what makes a thread of it read as falling rather than hanging.
/// How fast the ripples run down it, how far apart they are, and how deep.
///
/// A thread of water wavering from side to side is a wobbly rod: nothing about
/// it moves the way it is pointing. What says flowing is motion along the
/// thread, so the ripples travel down it, and the mesh is rebuilt every frame
/// to carry them. Spec 0042 of the engine is what makes that cost nothing.
pub const RUNS_DOWN: f32 = 2.6;
pub const BEADS: f32 = 13.0;
pub const BEADED: f32 = 0.16;

pub fn stream_mesh(since: f32) -> blitzkit::mesh::MeshData {
    cellar::turned(12, 36, |v| {
        // necking in as it speeds up, with the beads a falling thread breaks
        // into running down it
        let thin = 1.0 - v * 0.42;
        let bead = ((v * BEADS - since * RUNS_DOWN) * std::f32::consts::TAU).sin();

        (0.5 - v, thin * (1.0 + bead * BEADED * v))
    })
    .two_sided()
}

/// The splash where it lands: how wide the ring is and how it beats.
pub const SPLASH: f32 = 0.19;
pub const SPLASHES: f32 = 3.1;

/// How wide the splash ring is now, which pulses rather than holding still.
pub fn splash(since: f32) -> f32 {
    let beat = (since * SPLASHES).fract();

    SPLASH * (0.45 + beat * 0.9)
}

/// And how much of it there is, which fades as it spreads.
pub fn splashed(since: f32) -> f32 {
    let beat = (since * SPLASHES).fract();

    (1.0 - beat) * 0.5
}

/// The water standing in the basin, which is round because the basin is.
pub fn dish_mesh() -> blitzkit::mesh::MeshData {
    cellar::turned(24, 3, |v| (0.0, v))
}

/// The fountain's basin, a half bowl against the wall.
pub fn basin_mesh() -> blitzkit::mesh::MeshData {
    cellar::turned(24, 14, |v| {
        let wide = (v * std::f32::consts::FRAC_PI_2).sin() * 0.94 + 0.06;

        (v - 0.5, wide)
    })
    .two_sided()
}

/// The tiling and the fittings: everything that makes this a bath house rather
/// than a plastered box with water in it.
///
/// None of it is solid. A dado a hand's depth proud of a wall you cannot walk
/// into anyway is a collider for nothing, and every one of these boxes would be
/// another thing for somebody to get wedged behind.
pub fn fittings(reaches: f32) -> Vec<(Aabb, Made)> {
    let foot = cellar::stair_foot();
    let side = near(reaches);
    let floor = -cellar::DOWN;
    let back = foot - DEEP;
    let far_side = side + SPAN;
    let door = doorway(reaches);
    let mut out = Vec::new();

    // the four walls, each as its face, which way it looks, and the run along
    // it. The near one is in two pieces because the way through is in it.
    let walls: Vec<(Vec3, Vec3, f32, f32)> = vec![
        // the runs along z start at the near wall's own face and not at the
        // wall behind it, so a dado stops in the corner rather than running
        // into the skin it meets there
        (vec3(back, 0.0, 0.0), Vec3::X, side + SKIN, far_side),
        (vec3(foot, 0.0, 0.0), Vec3::NEG_X, side + SKIN, far_side),
        (vec3(0.0, 0.0, far_side), Vec3::NEG_Z, back, foot),
        // the near wall's face is this room's own skin over the cellar's
        // stone, so the tiling starts outside that and not on the stone
        (
            vec3(0.0, 0.0, side + SKIN),
            Vec3::Z,
            back,
            door.x - DOOR * 0.5,
        ),
        (
            vec3(0.0, 0.0, side + SKIN),
            Vec3::Z,
            door.x + DOOR * 0.5,
            foot,
        ),
    ];

    for (face, looks, from, to) in walls {
        if to - from <= 1e-3 {
            continue;
        }
        let along = looks.z.abs() > 0.5;
        let middle = (from + to) * 0.5;
        let run = to - from;

        // the dado, the border over it, and the cornice under the ceiling
        for (up, deep, out_by, made) in [
            (DADO * 0.5, DADO, PIER_OUT * 0.5, Made::Dado),
            (DADO + BAND * 0.5, BAND, PIER_OUT * 0.75, Made::Band),
            (TALL - CORNICE * 0.5, CORNICE, PIER_OUT, Made::Band),
        ] {
            let at = if along {
                vec3(middle, floor + up, face.z + looks.z * out_by * 0.5)
            } else {
                vec3(face.x + looks.x * out_by * 0.5, floor + up, middle)
            };
            let size = if along {
                vec3(run, deep, out_by)
            } else {
                vec3(out_by, deep, run)
            };

            out.push((Aabb::from_center_size(at, size), made));
        }

        // and the piers, which are what stops a long wall reading as one slab
        for n in 0..PIERS {
            let step = (n as f32 + 0.5) / PIERS as f32;
            let on = from + run * step;
            if run < PIER * 3.0 {
                break;
            }

            let at = if along {
                vec3(
                    on,
                    floor + (TALL - CORNICE) * 0.5,
                    face.z + looks.z * PIER_OUT * 0.5,
                )
            } else {
                vec3(
                    face.x + looks.x * PIER_OUT * 0.5,
                    floor + (TALL - CORNICE) * 0.5,
                    on,
                )
            };
            let size = if along {
                vec3(PIER, TALL - CORNICE, PIER_OUT)
            } else {
                vec3(PIER_OUT, TALL - CORNICE, PIER)
            };

            out.push((Aabb::from_center_size(at, size), Made::Pier));
        }
    }

    // the border laid into the floor, as four runs round the room
    // laid on the floor and not half into it: centred on the floor's own top
    // the border sinks half its thickness into the tiles, and the two fight
    let laid = floor + 0.01;
    // the border frames the open floor, so it starts past the sauna rather than
    // running under it: laid to the room alone it crossed the timber and the
    // two fought along three units of it
    let box_ = sauna(reaches);
    let (one, two) = (
        (back + INLAY_IN).max(box_.max.x + INLAY_IN),
        foot - INLAY_IN,
    );
    let (three, four) = (side + INLAY_IN, far_side - INLAY_IN);
    for (at, size) in [
        (
            vec3((one + two) * 0.5, laid, three),
            vec3(two - one, 0.02, INLAY),
        ),
        (
            vec3((one + two) * 0.5, laid, four),
            vec3(two - one, 0.02, INLAY),
        ),
        (
            vec3(one, laid, (three + four) * 0.5),
            vec3(INLAY, 0.02, four - three),
        ),
        (
            vec3(two, laid, (three + four) * 0.5),
            vec3(INLAY, 0.02, four - three),
        ),
    ] {
        out.push((Aabb::from_center_size(at, size), Made::Inlay));
    }

    // the brackets the sconces sit on, and the spout over the fountain. The
    // bowls themselves are turned and drawn from `sconces` and `fountain`: a
    // bowl is not a box, and a box is what a placeholder looks like.
    for at in sconces(reaches) {
        out.push((
            Aabb::from_center_size(
                vec3(at.x, at.y - SCONCE.y * 0.6, at.z),
                vec3(0.06, 0.26, 0.06),
            ),
            Made::Brass,
        ));
    }

    let fountain = fountain(reaches);
    out.push((
        Aabb::from_center_size(
            vec3(fountain.x, fountain.y + 0.5, fountain.z - SPOUT * 0.5),
            vec3(SPOUT, SPOUT, SPOUT * 4.0),
        ),
        Made::Brass,
    ));
    out.push((
        Aabb::from_center_size(
            vec3(fountain.x, fountain.y + 0.78, fountain.z + 0.04),
            vec3(0.5, 0.56, 0.12),
        ),
        Made::Cut,
    ));

    // the plinths the urns stand on
    for urn in urns(reaches) {
        out.push((
            Aabb::from_center_size(
                vec3(urn.x, -cellar::DOWN + PLINTH * 0.5, urn.z),
                vec3(PLINTH, PLINTH, PLINTH),
            ),
            Made::Cut,
        ));
    }

    // and a seat along the near wall, between the door and the corner
    let seat = vec3(
        door.x + DOOR * 0.5 + SEAT.x * 0.6,
        floor + SEAT_UP,
        side + SEAT.z * 0.5 + PIER_OUT,
    );
    // a slab with a moulding under its edge, which is what stone furniture is
    out.push((Aabb::from_center_size(seat, SEAT), Made::Cut));
    out.push((
        Aabb::from_center_size(
            seat - Vec3::Y * SEAT.y,
            vec3(SEAT.x - 0.14, SEAT.y * 0.7, SEAT.z - 0.1),
        ),
        Made::Band,
    ));
    for across in [-1.0f32, 1.0] {
        out.push((
            Aabb::from_center_size(
                vec3(
                    door.x + DOOR * 0.5 + SEAT.x * 0.6 + across * (SEAT.x * 0.5 - 0.12),
                    floor + SEAT_UP * 0.5,
                    side + SEAT.z * 0.5 + PIER_OUT,
                ),
                vec3(0.16, SEAT_UP, SEAT.z * 0.8),
            ),
            Made::Cut,
        ));
    }

    out
}

/// Everything in the room you can walk into.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    built(reaches).into_iter().map(|(box_, _)| box_).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How far off still a pool's own surface is.
    ///
    /// Its own surface and not its mesh. The mesh carries a skirt hanging from
    /// the rim down to the basin floor, per spec 0043, so asking the whole of
    /// it how far it is off still answers with the depth of the pool and every
    /// one of these fails the moment it is still.
    fn off_still(water: &blitzkit::water::Water) -> f32 {
        let (across, along) = water.nodes();
        let level = water.at.y;

        water.surface().vertices[..across * along]
            .iter()
            .fold(0.0f32, |most, v| most.max((v.position[1] - level).abs()))
    }

    /// A reaches the room is laid out for, which is what the arcade runs with.
    const REACHES: f32 = 8.3;

    /// Whether two boxes share any room at all.
    fn overlap(one: &Aabb, other: &Aabb) -> bool {
        one.min.x < other.max.x - 1e-4
            && other.min.x < one.max.x - 1e-4
            && one.min.y < other.max.y - 1e-4
            && other.min.y < one.max.y - 1e-4
            && one.min.z < other.max.z - 1e-4
            && other.min.z < one.max.z - 1e-4
    }

    /// The whole room, as one box, floor to ceiling.
    fn room(reaches: f32) -> Aabb {
        let middle = at(reaches);

        Aabb::from_center_size(middle + Vec3::Y * TALL * 0.5, vec3(DEEP, TALL, SPAN))
    }

    /// The pool, as the box the water fills.
    fn pool_box(reaches: f32) -> Aabb {
        let (middle, size, deep) = pool(reaches);

        Aabb::from_center_size(middle - Vec3::Y * deep * 0.5, vec3(size.x, deep, size.y))
    }

    /// Spec 0008: the way through is in a long wall of the cellar, and the two
    /// rooms meet at it.
    ///
    /// Not the far wall, which is the fireplace. A door through a chimney
    /// breast is a door into a fire, and the first layout of this room had one.
    #[test]
    fn the_way_through_is_in_a_long_wall() {
        let door = doorway(REACHES);
        let along = cellar::opening(REACHES);

        // it is in the cellar's +z wall
        assert!(
            (door.z - (along + cellar::SPAN * 0.5)).abs() < THICK,
            "the door is at {} and the wall at {}",
            door.z,
            along + cellar::SPAN * 0.5
        );
        // and along the cellar rather than off the end of it
        assert!(
            door.x < cellar::stair_foot() && door.x > cellar::stair_foot() - cellar::DEEP,
            "the door is at {}, outside a cellar from {} to {}",
            door.x,
            cellar::stair_foot() - cellar::DEEP,
            cellar::stair_foot()
        );
        // and clear of the fireplace, which is on the far wall
        assert!(door.x > cellar::stair_foot() - cellar::DEEP + 1.0);
    }

    /// Spec 0008: and you can walk through it.
    ///
    /// Measured as the room measures everything else: a body your size, put in
    /// the opening, meeting nothing. The nook's first doorway was 0.74 wide for
    /// somebody 0.9 across, which looked like a door and was a wall.
    #[test]
    fn you_can_walk_in_from_the_cellar() {
        let door = doorway(REACHES);
        let solid: Vec<Aabb> = solid(REACHES)
            .into_iter()
            .chain(cellar::solid(REACHES))
            .collect();

        // standing in the opening, and a step either side of it
        for along in [-0.6f32, 0.0, 0.6] {
            let you = Aabb::from_center_size(
                vec3(door.x, door.y + crate::EYE * 0.5, door.z + along),
                vec3(crate::RADIUS * 2.0, crate::EYE, crate::RADIUS * 2.0),
            );

            for box_ in &solid {
                assert!(
                    !overlap(&you, box_),
                    "{} through the door you are inside {:?}",
                    along,
                    box_
                );
            }
        }
    }

    /// Spec 0008: the pool is sunk into the floor rather than standing on it.
    #[test]
    fn the_pool_is_sunk_into_the_floor() {
        let (middle, _, deep) = pool(REACHES);
        let floor = at(REACHES).y;

        // under the coping rather than level with it, by the freeboard every
        // pool has and this one needs twice over
        assert!(
            (floor - middle.y - FREEBOARD).abs() < 1e-5,
            "the water is {} under a coping it should be {} under",
            floor - middle.y,
            FREEBOARD
        );
        assert!(middle.y < floor, "the water stands above the floor");
        assert!(deep > 0.5, "a pool {} deep is a puddle", deep);

        // and nothing the floor is made of shares a plane with the basin's
        // sides, which is what put a sheet of stripes round this pool
        let sides: Vec<Aabb> = built(REACHES)
            .into_iter()
            .filter(|(_, made)| *made == Made::Basin)
            .map(|(box_, _)| box_)
            .collect();
        for (tile, made) in built(REACHES) {
            if made != Made::Tile {
                continue;
            }
            for wall in &sides {
                assert!(
                    (tile.max.y - wall.max.y).abs() > 1e-4 || wall.max.y < tile.min.y + 1e-4,
                    "a tile topping at {} meets a basin wall topping at {}",
                    tile.max.y,
                    wall.max.y
                );
            }
        }

        // and the floor has a hole in it exactly where the water is
        let standing = pool_box(REACHES);
        for (box_, made) in built(REACHES) {
            if made != Made::Tile {
                continue;
            }
            assert!(
                !overlap(&box_, &standing),
                "the floor is laid across the pool at {:?}",
                box_
            );
        }
    }

    /// Spec 0008: its steps are a flight you can climb, by the same rule the
    /// stair down to the cellar is.
    #[test]
    fn the_steps_are_steps_you_can_take() {
        let steps: Vec<Aabb> = built(REACHES)
            .into_iter()
            .filter(|(_, made)| *made == Made::Step)
            .map(|(box_, _)| box_)
            .collect();

        assert_eq!(steps.len(), STEPS);

        // from the basin floor up, each top is a step above the one under it
        let mut tops: Vec<f32> = steps.iter().map(|box_| box_.max.y).collect();
        tops.sort_by(f32::total_cmp);
        tops.push(at(REACHES).y);

        let mut under = at(REACHES).y - POOL_DEEP;
        for top in tops {
            assert!(
                top - under <= crate::walk::STEP + 1e-4,
                "a rise of {} where you can take {}",
                top - under,
                crate::walk::STEP
            );
            under = top;
        }
    }

    /// Spec 0008: you can stand up anywhere in it.
    ///
    /// The spec chose this over swimming on purpose. Out of your depth is a
    /// camera and a stroke and a surface you can be under, which is a different
    /// spec; this one is about water.
    #[test]
    fn you_can_stand_up_anywhere_in_it() {
        let (_, _, deep) = pool(REACHES);
        let (_, _, hot) = tub(REACHES);

        assert!(
            deep < crate::EYE,
            "the pool is {} deep and your eye is at {}",
            deep,
            crate::EYE
        );
        assert!(
            hot < deep,
            "the tub is {} deep against the pool's {}, which is backwards",
            hot,
            deep
        );
    }

    /// Walks somebody from `from` toward `to`, the way the game does, and says
    /// where they end up.
    ///
    /// The only honest way to ask whether you can get out of a hole. Measuring
    /// the rises says every step is climbable; it does not say the steps are
    /// anywhere you will meet them, or that you are not held off them by your
    /// own width.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        // both rooms. The wall along this room's near side is the cellar's, so
        // a walk that only knows the spa's own boxes walks through it.
        let solid: Vec<Aabb> = solid(REACHES)
            .into_iter()
            .chain(cellar::solid(REACHES))
            .collect();
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * pace(REACHES, at, crate::SPEED)
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

    /// Spec 0008: you can get out of the pool.
    ///
    /// Walked rather than measured. Every rise in this flight was under a step
    /// from the first build, and the steps were still a shelf you could swim
    /// under and not climb.
    #[test]
    fn you_can_climb_out_of_the_pool() {
        let (middle, size, _) = pool(REACHES);
        let floor = at(REACHES).y;

        // from everywhere in it, not from one spot. One path out is not a way
        // out: you are in the water where you are, not where a test put you.
        for across in [-0.42f32, 0.0, 0.42] {
            for along in [-0.42f32, 0.0, 0.42] {
                // dropped in from the waterline rather than placed on the
                // basin floor: a point on the floor can be inside the flight,
                // which is not a place anybody can stand and not a fair thing
                // to ask them to walk out of
                let from = vec3(
                    middle.x + size.x * across,
                    floor - 0.05,
                    middle.z + size.y * along,
                );
                // straight out, which is what anybody in a pool does: the
                // flight runs the whole width, so the near edge is the steps
                // from wherever you are standing
                let steps = vec3(from.x, floor, middle.z - size.y * 0.5 - 1.0);
                let out = walked(from, steps, 14.0);

                assert!(
                    out.y > floor - 0.05,
                    "from {:?} you are still at {} after fourteen seconds, with the floor at {}",
                    from,
                    out.y,
                    floor
                );
            }
        }
    }

    /// Spec 0008: there is somewhere to stand all round the pool.
    ///
    /// The thing that made the pool a trap, and no test saw it: the sauna's
    /// wall stood 0.33 from the pool's near edge, and a body is 0.9 across. You
    /// climbed out, had nowhere to be, and were pushed back in. Every rise was
    /// legal and every step was climbable and none of that was the question.
    #[test]
    fn there_is_somewhere_to_stand_round_the_pool() {
        let (middle, size, _) = pool(REACHES);
        let floor = at(REACHES).y;
        let solid = solid(REACHES);
        let out = crate::RADIUS + 0.15;

        for (way, along) in [
            (vec3(0.0, 0.0, -1.0), vec3(1.0, 0.0, 0.0)),
            (vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0)),
            (vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
            (vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0)),
        ] {
            let edge = vec3(size.x, 0.0, size.y) * 0.5;
            let reach = (along * vec3(size.x, 0.0, size.y)).length() * 0.5;

            for step in [-0.6f32, 0.0, 0.6] {
                let stand =
                    middle + way * (edge * way.abs()).length() + way * out + along * reach * step;
                let you = Aabb::from_center_size(
                    vec3(stand.x, floor + crate::EYE * 0.5, stand.z),
                    vec3(crate::RADIUS * 2.0, crate::EYE, crate::RADIUS * 2.0),
                );

                for box_ in &solid {
                    assert!(
                        !overlap(&you, box_),
                        "standing at {:?} off the pool you are inside something {:?} to {:?}",
                        stand,
                        box_.min,
                        box_.max
                    );
                }
            }
        }
    }

    /// Spec 0008: and you can get into the tub.
    ///
    /// A tub you cannot climb into is a barrel of hot water. Its rim stands
    /// higher than a step, which is what makes it read as a tub, so getting in
    /// takes something to stand on.
    #[test]
    fn you_can_climb_into_the_tub() {
        let (surface, wide, deep) = tub(REACHES);
        let floor = at(REACHES).y;

        // from the floor beside it, on the side the pool is
        let from = vec3(surface.x, floor, surface.z - wide * 0.5 - 1.2);
        let inside = vec3(surface.x, surface.y - deep, surface.z);
        let got = walked(from, inside, 10.0);

        assert!(
            got.y > floor + 0.05,
            "after ten seconds you are at {} and the tub's floor is at {}",
            got.y,
            surface.y - deep
        );
        assert!(
            (got.x - surface.x).abs() < wide * 0.5 && (got.z - surface.z).abs() < wide * 0.5,
            "you are at {:?} and the tub is at {:?}",
            got,
            surface
        );
    }

    /// Spec 0008: nothing in the room stands in the water or in the sauna.
    #[test]
    fn nothing_stands_where_the_water_is() {
        let water = pool_box(REACHES);
        let (tub_at, tub_wide, tub_deep) = tub(REACHES);
        let hot = Aabb::from_center_size(
            tub_at - Vec3::Y * tub_deep * 0.5,
            vec3(tub_wide, tub_deep, tub_wide),
        );
        let box_ = sauna(REACHES);

        assert!(!overlap(&water, &hot), "the tub is standing in the pool");
        assert!(!overlap(&water, &box_), "the sauna is standing in the pool");
        assert!(!overlap(&hot, &box_), "the sauna is standing in the tub");

        // nothing at all stands in the pool
        for (built, made) in built(REACHES) {
            if !matches!(made, Made::Timber | Made::Bench | Made::Stove | Made::Tub) {
                continue;
            }
            assert!(!overlap(&built, &water), "something stands in the pool");
        }

        // and nothing but the tub's own fittings stands in the tub. Its seat is
        // under its water on purpose: that is what a seat in a tub is.
        for (built, made) in built(REACHES) {
            if !matches!(made, Made::Timber | Made::Bench | Made::Stove) {
                continue;
            }
            assert!(!overlap(&built, &hot), "something stands in the tub");
        }
    }

    /// Spec 0008: the tub holds its own water, clear of the pool's.
    #[test]
    fn the_tub_is_its_own_pool() {
        let (tub_at, _, _) = tub(REACHES);
        let (pool_at, _, _) = pool(REACHES);

        assert!(
            tub_at.y > pool_at.y,
            "the tub's water is at {} and the pool's at {}, so it is not raised",
            tub_at.y,
            pool_at.y
        );
        assert!(
            tub_at.y - pool_at.y > 0.3,
            "a step up of {}",
            tub_at.y - pool_at.y
        );
    }

    /// Spec 0008: the sauna is a room with a way in you fit through.
    #[test]
    fn the_sauna_has_a_door_you_fit_through() {
        // read off the box the timber actually stands in rather than off the
        // constants, so a sauna that is built differently from how it is
        // described fails here too
        let box_ = sauna(REACHES);
        let gap = box_.size().z - (box_.size().z - SAUNA_DOOR);

        assert!(
            gap > crate::RADIUS * 2.0,
            "a door {} wide for somebody {} across",
            gap,
            crate::RADIUS * 2.0
        );
        assert!(
            box_.size().y > crate::EYE + 0.2,
            "a sauna {} tall for an eye at {}",
            box_.size().y,
            crate::EYE
        );
    }

    /// Spec 0008: its benches are under its ceiling and clear of its stove.
    #[test]
    fn the_benches_fit_in_the_sauna() {
        let box_ = sauna(REACHES);
        let benches: Vec<Aabb> = built(REACHES)
            .into_iter()
            .filter(|(_, made)| *made == Made::Bench)
            .map(|(one, _)| one)
            .collect();
        let stove: Vec<Aabb> = built(REACHES)
            .into_iter()
            .filter(|(_, made)| *made == Made::Stove)
            .map(|(one, _)| one)
            .collect();

        assert_eq!(benches.len(), 2, "a sauna has two tiers");
        for bench in &benches {
            assert!(
                bench.max.y < box_.max.y,
                "a bench at {} under a ceiling at {}",
                bench.max.y,
                box_.max.y
            );
            // you can sit on the high one without your head in the roof
            assert!(box_.max.y - bench.max.y > 1.0);

            for one in &stove {
                assert!(!overlap(bench, one), "a bench is in the stove");
            }
        }
    }

    /// Spec 0008: every lamp is in a sconce.
    ///
    /// Not merely "there are sconces and there are lamps". Those were two lists
    /// of where the light in this room comes from, and they drifted the way two
    /// lists always do: the lamps ran along the ceiling and the fittings stood
    /// on a wall, so the ceiling glowed at nothing and every sconce was dark.
    #[test]
    fn every_lamp_is_in_a_sconce() {
        let bowls = sconces(REACHES);
        let lit = lamps(REACHES);

        assert!(!bowls.is_empty(), "a room with no fittings in it");
        assert_eq!(lit.len(), bowls.len(), "a lamp that is not in a sconce");

        for lamp in &lit {
            let bowl = bowls
                .iter()
                .min_by(|one, other| {
                    one.distance_squared(*lamp)
                        .total_cmp(&other.distance_squared(*lamp))
                })
                .expect("a sconce to sit in");

            assert!(
                lamp.distance(*bowl) < SCONCE.y,
                "a lamp at {:?} is {} from the nearest sconce",
                lamp,
                lamp.distance(*bowl)
            );
            // and above it, which is where the light leaves a bowl
            assert!(lamp.y > bowl.y, "a lamp under its own sconce");
        }

        // down both long walls, so the room is lit from both sides
        let side = near(REACHES);
        let far_side = side + SPAN;
        assert!(
            bowls.iter().any(|at| at.z < side + SPAN * 0.5)
                && bowls.iter().any(|at| at.z > far_side - SPAN * 0.5),
            "every sconce is on one wall"
        );
    }

    /// Spec 0008: every lamp is inside the room it lights.
    ///
    /// The nook hung a sconce in its own doorway once, from a list of walls
    /// that had stopped matching the walls.
    #[test]
    fn every_lamp_is_in_the_room() {
        let walls = room(REACHES);

        for lamp in lamps(REACHES) {
            assert!(
                lamp.x > walls.min.x
                    && lamp.x < walls.max.x
                    && lamp.z > walls.min.z
                    && lamp.z < walls.max.z
                    && lamp.y > walls.min.y
                    && lamp.y < walls.max.y,
                "a lamp at {:?} is outside a room from {:?} to {:?}",
                lamp,
                walls.min,
                walls.max
            );
        }

        // and the sauna's is inside the sauna
        let stove = stove_lamp(REACHES);
        let box_ = sauna(REACHES);
        assert!(
            stove.x > box_.min.x
                && stove.x < box_.max.x
                && stove.z > box_.min.z
                && stove.z < box_.max.z
                && stove.y < box_.max.y,
            "the stove's lamp at {:?} is outside the sauna",
            stove
        );
    }

    /// Spec 0008: the sauna's door fills its doorway when it is shut.
    ///
    /// A pane that does not fill the hole is a hole with glass beside it, which
    /// is what the first one was: hinged at the middle of the opening rather
    /// than its edge, it covered half.
    #[test]
    fn the_glass_fills_the_doorway_when_it_is_shut() {
        let shut = sauna_leaf_box(REACHES, 0.0);

        assert!(
            shut.size().z >= SAUNA_DOOR - 1e-4,
            "a pane {} across a door {} wide",
            shut.size().z,
            SAUNA_DOOR
        );
        assert!(
            shut.size().y > crate::EYE,
            "a pane {} tall for an eye at {}",
            shut.size().y,
            crate::EYE
        );
        // and it is in the opening rather than beside it
        let box_ = sauna(REACHES);
        assert!(
            (shut.center().z - box_.center().z).abs() < 1e-4,
            "the glass is at {} and the doorway at {}",
            shut.center().z,
            box_.center().z
        );
    }

    /// Spec 0008: and swings clear of it when it is open.
    #[test]
    fn the_glass_swings_out_of_the_way() {
        let box_ = sauna(REACHES);
        let open = sauna_leaf_box(REACHES, 1.0);
        let hole = Aabb::from_center_size(
            vec3(box_.max.x, box_.center().y, box_.center().z),
            vec3(
                THICK,
                SAUNA.y - SAUNA_CLEARS * 2.0,
                SAUNA_DOOR - crate::RADIUS * 2.0,
            ),
        );

        assert!(
            !overlap(&open, &hole),
            "the open door is still across the way in: {:?} against {:?}",
            open,
            hole
        );
        // and it is hinged rather than slid: it is still touching its own edge
        let shut = sauna_leaf_box(REACHES, 0.0);
        assert!(
            (open.min.z - shut.min.z).abs() < 0.2,
            "the door moved sideways from {} to {}",
            shut.min.z,
            open.min.z
        );
    }

    /// Spec 0008: steam rises, fades at both ends, and stays over the water it
    /// comes off.
    ///
    /// A pure function of the clock, so there is nothing kept and nothing to
    /// reset. The same time gives the same steam, which is what makes it
    /// drawable without state.
    #[test]
    fn the_steam_rises_and_fades() {
        let over = vec3(0.0, 0.0, 0.0);
        let wide = 2.2;

        // the same moment gives the same steam
        assert_eq!(steam(over, wide, 3.25).len(), steam(over, wide, 3.25).len());
        for (one, other) in steam(over, wide, 3.25).iter().zip(steam(over, wide, 3.25)) {
            assert_eq!(one.0, other.0);
        }

        for since in [0.0f32, 0.7, 2.4, 5.9, 11.3] {
            for (at, size, thick) in steam(over, wide, since) {
                assert!(at.y >= over.y - 1e-5, "steam fell to {}", at.y);
                assert!(at.y <= over.y + PUFF_RISES, "steam reached {}", at.y);
                assert!(
                    (at.x - over.x).abs() < wide,
                    "steam drifted {} off a tub {} across",
                    at.x - over.x,
                    wide
                );
                assert!(size.min_element() > 0.0, "a puff of no size");
                // wider than it is tall: a ball reads as a ball however faint
                assert!(
                    size.y < size.x,
                    "a puff {} tall and {} across",
                    size.y,
                    size.x
                );
                assert!(
                    (0.0..=PUFF_MOST + 1e-5).contains(&thick),
                    "steam at {} thick",
                    thick
                );
            }
        }

        // and over a whole life nothing pops into being at full strength
        let most = (0..240)
            .map(|n| steam(over, wide, n as f32 * 0.05))
            .flat_map(|puffs| puffs.into_iter().map(|(_, _, thick)| thick))
            .fold(0.0f32, f32::max);
        assert!(most <= PUFF_MOST + 1e-5, "steam reached {}", most);
    }

    /// Spec 0008: the water fills the hole in the floor and no more.
    ///
    /// Two descriptions of one rectangle: the hole `built` cuts, and the grid
    /// the engine lays out from the same numbers. The engine rounds its cell
    /// count, so they can come apart, and water that reaches past the coping is
    /// a sheet lying on the floor.
    #[test]
    fn the_water_fills_the_hole_and_no_more() {
        let (middle, size, deep) = pool(REACHES);
        let water = blitzkit::water::Water::new(middle, size, deep, CELLS);
        let mesh = water.surface();

        let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
        for vertex in &mesh.vertices {
            lo = lo.min(Vec3::from_array(vertex.position));
            hi = hi.max(Vec3::from_array(vertex.position));
        }

        let (from_x, to_x, from_z, to_z) = hole(REACHES);
        assert!(
            (lo.x - from_x).abs() < 1e-3 && (hi.x - to_x).abs() < 1e-3,
            "the water runs {} to {} across a hole {} to {}",
            lo.x,
            hi.x,
            from_x,
            to_x
        );
        assert!(
            (lo.z - from_z).abs() < 1e-3 && (hi.z - to_z).abs() < 1e-3,
            "the water runs {} to {} along a hole {} to {}",
            lo.z,
            hi.z,
            from_z,
            to_z
        );

        // and the same for the tub, which is a different shape of grid
        let (tub_at, wide, hot) = tub(REACHES);
        let its = blitzkit::water::Water::new(tub_at, glam::vec2(wide, wide), hot, TUB_CELLS);
        let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
        for vertex in &its.surface().vertices {
            lo = lo.min(vertex.position[0]);
            hi = hi.max(vertex.position[0]);
        }
        assert!(
            (hi - lo - wide).abs() < 1e-3,
            "the tub's water is {} across a tub {} wide",
            hi - lo,
            wide
        );
    }

    /// Spec 0008: the sign is on the wall, between the lintel and the ceiling.
    ///
    /// It was 0.52 tall hung 0.3 clear of a lintel with half a unit of wall
    /// over it, so it sat inside the cellar's ceiling. A sign nobody can see is
    /// not a sign, and nothing about where it hangs needs a window to check.
    #[test]
    fn the_sign_is_on_the_wall_over_the_door() {
        let at = sign(REACHES);
        let door = doorway(REACHES);
        let floor = -cellar::DOWN;

        assert!(
            at.y - SIGN.y * 0.5 > floor + LINTEL - 1e-4,
            "the sign reaches down to {} and the lintel is at {}",
            at.y - SIGN.y * 0.5,
            floor + LINTEL
        );
        assert!(
            at.y + SIGN.y * 0.5 < floor + cellar::TALL,
            "the sign reaches up to {} and the cellar's ceiling is at {}",
            at.y + SIGN.y * 0.5,
            floor + cellar::TALL
        );

        // and in front of the panelling, not behind it. The boards stand off
        // the stone, and a sign that only clears the stone is a sign with
        // mahogany across the bottom half of it.
        assert!(
            (door.z - at.z) > THICK * 0.5 + cellar::BOARD_OUT,
            "the sign stands {} off the wall and the panelling stands {}",
            door.z - at.z - THICK * 0.5,
            cellar::BOARD_OUT
        );

        assert!(
            at.z < door.z,
            "the sign is at {} and the doorway at {}",
            at.z,
            door.z
        );
        assert!(!SAYS.is_empty(), "a sign with nothing on it");
    }

    /// Spec 0008: the room is closed. You cannot walk out of the building.
    ///
    /// The spa borrowed the cellar's wall for its near side, and the spa is
    /// eighteen long against the cellar's ten: past the cellar's far end there
    /// was no wall at all, and the floor stopped with it. You walked into a
    /// black wedge and fell out of the world.
    ///
    /// Walked at every wall rather than reasoned about, because a hole is
    /// exactly what a list of walls does not show: it is the piece that is not
    /// in the list.
    #[test]
    fn you_cannot_walk_out_of_the_room() {
        let middle = at(REACHES);
        let floor = middle.y;
        let walls = vec3(DEEP, 0.0, SPAN) * 0.5;

        for way in [
            vec3(1.0, 0.0, 0.0),
            vec3(-1.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, -1.0),
        ] {
            // from three places along the wall it is walking at, since a hole
            // is somewhere and not everywhere
            let across = vec3(way.z, 0.0, way.x);

            for along in [-0.38f32, 0.0, 0.38] {
                let from = vec3(
                    middle.x + across.x * walls.x * along * 2.0,
                    floor,
                    middle.z + across.z * walls.z * along * 2.0,
                );
                let out = walked(from, from + way * 40.0, 12.0);

                // the pool counts as somewhere in the room: walking into it
                // drops you its own depth, which is the point of it
                assert!(
                    out.y > floor - POOL_DEEP - 0.3,
                    "walking {:?} from {:?} you fell to {}, past a basin at {}",
                    way,
                    from,
                    out.y,
                    floor - POOL_DEEP
                );
                assert!(
                    (out.x - middle.x).abs() < walls.x + 0.5
                        && (out.z - middle.z).abs() < walls.z + 0.5,
                    "walking {:?} you got to {:?}, outside a room {:?} by {:?}",
                    way,
                    out,
                    middle,
                    walls
                );
            }
        }
    }

    /// Spec 0008: the sauna fills with steam from the ceiling down.
    ///
    /// Which way up it fills is the whole of it. Steam rises, so a room fills
    /// like a glass held upside down under a tap: the top goes first and the
    /// floor last, and that is why you sit on the high bench.
    #[test]
    fn the_sauna_fills_from_the_ceiling_down() {
        let room = sauna(REACHES);

        assert!(
            fug(room, 4.0, 0.0).is_empty(),
            "an open sauna has steam in it"
        );

        // part full, every puff is in the top of the room
        let early = fug(room, 4.0, 0.25);
        assert!(
            !early.is_empty(),
            "a quarter full and there is nothing there"
        );
        for (at, _, _) in &early {
            assert!(
                at.y > room.center().y,
                "steam at {} in the bottom of a room whose middle is {}",
                at.y,
                room.center().y
            );
        }

        // and full, it reaches further down and there is more of it
        let late = fug(room, 4.0, 1.0);
        assert!(
            late.len() > early.len(),
            "a full room holds {} puffs and a quarter full holds {}",
            late.len(),
            early.len()
        );
        assert!(
            late.iter()
                .map(|(at, _, _)| at.y)
                .fold(f32::INFINITY, f32::min)
                < early
                    .iter()
                    .map(|(at, _, _)| at.y)
                    .fold(f32::INFINITY, f32::min),
            "filling up did not reach any lower"
        );

        // all of it inside the room, and none of it opaque
        for (at, _, thick) in &late {
            assert!(
                at.x > room.min.x && at.x < room.max.x && at.z > room.min.z && at.z < room.max.z,
                "steam at {:?} is outside the sauna",
                at
            );
            assert!(at.y > room.min.y && at.y < room.max.y, "steam at {}", at.y);
            assert!(
                (0.0..=FUG_MOST + 1e-5).contains(thick),
                "steam at {} thick",
                thick
            );
        }
    }

    /// Spec 0008: the fittings are in the room and in nobody's way.
    ///
    /// None of them is a collider, so the only way one can hurt is by being
    /// somewhere it should not be: in the pool, in the doorway, or outside the
    /// walls it hangs on.
    #[test]
    fn the_fittings_are_in_the_room() {
        let water = pool_box(REACHES);
        let room = room(REACHES);
        let door = doorway(REACHES);
        let got = fittings(REACHES);

        assert!(got.len() > 20, "a bath house of {} pieces", got.len());

        for (box_, made) in got {
            assert!(
                !overlap(&box_, &water),
                "a {:?} is standing in the pool",
                made
            );

            // inside the room, give or take the hand's depth everything stands
            // proud of its own wall by
            assert!(
                box_.min.x > room.min.x - PIER_OUT - 1e-3
                    && box_.max.x < room.max.x + PIER_OUT + 1e-3
                    && box_.min.z > room.min.z - PIER_OUT - 1e-3
                    && box_.max.z < room.max.z + PIER_OUT + 1e-3,
                "a {:?} at {:?} to {:?} is outside the room",
                made,
                box_.min,
                box_.max
            );
            assert!(
                box_.max.y < room.max.y + 1e-3,
                "a {:?} reaches {} through a ceiling at {}",
                made,
                box_.max.y,
                room.max.y
            );

            // and clear of the way through, which is the one piece of wall that
            // has to stay empty
            let through = Aabb::from_center_size(
                vec3(door.x, door.y + LINTEL * 0.5, door.z),
                vec3(DOOR, LINTEL, THICK * 3.0),
            );
            assert!(
                !overlap(&box_, &through),
                "a {:?} is across the doorway",
                made
            );
        }
    }

    /// Spec 0008: and the sauna is in a corner rather than out in the floor.
    #[test]
    fn the_sauna_is_in_a_corner() {
        let box_ = sauna(REACHES);
        let room = room(REACHES);

        // against two walls, within a wall's thickness of each
        let near_x = (box_.min.x - room.min.x).min(room.max.x - box_.max.x);
        let near_z = (box_.min.z - room.min.z).min(room.max.z - box_.max.z);

        assert!(
            near_x < THICK * 2.0,
            "the sauna is {} off the nearest end wall",
            near_x
        );
        assert!(
            near_z < THICK * 2.0,
            "the sauna is {} off the nearest side wall",
            near_z
        );
    }

    /// Spec 0008: everything thrown on a wheel has a surface and a shape.
    ///
    /// The cellar's barrels taught this twice: a turned mesh built with too few
    /// steps, or with a profile that collapses, comes back as two points and no
    /// skin, and draws nothing at all. An urn that is a cylinder is a tin can,
    /// so the belly has to be wider than the neck.
    #[test]
    fn the_turned_pieces_are_turned() {
        for (what, mesh) in [
            ("urn", urn_mesh()),
            ("sconce", sconce_mesh()),
            ("basin", basin_mesh()),
            ("stream", stream_mesh(0.0)),
        ] {
            assert!(
                mesh.indices.len() > 48,
                "the {} is {} triangles",
                what,
                mesh.indices.len() / 3
            );

            let (mut lo, mut hi) = (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY));
            for vertex in &mesh.vertices {
                lo = lo.min(Vec3::from_array(vertex.position));
                hi = hi.max(Vec3::from_array(vertex.position));
            }

            assert!(
                hi.x - lo.x > 0.2 && hi.y - lo.y > 0.5,
                "the {} is {:?} across and {} tall",
                what,
                hi.x - lo.x,
                hi.y - lo.y
            );

            // and open where it is meant to be open. The candles' mesh is
            // lidded, which spends the first and last tenth of itself closing
            // to a point, and that is what made the fountain pour a pencil.
            if what == "stream" {
                let at_ends: Vec<f32> = mesh
                    .vertices
                    .iter()
                    .filter(|v| v.position[1] > hi.y - 1e-3)
                    .map(|v| (v.position[0] * v.position[0] + v.position[2] * v.position[2]).sqrt())
                    .collect();

                assert!(
                    at_ends.iter().all(|wide| *wide > 0.2),
                    "the stream closes to a point at its top"
                );
            }
        }

        // and the urn is an urn: a belly wider than its neck
        let mesh = urn_mesh();
        let widest = |from: f32, to: f32| {
            mesh.vertices
                .iter()
                .filter(|v| v.position[1] >= from && v.position[1] <= to)
                .map(|v| (v.position[0] * v.position[0] + v.position[2] * v.position[2]).sqrt())
                .fold(0.0f32, f32::max)
        };
        let belly = widest(-0.3, 0.0);
        let neck = widest(0.2, 0.35);

        assert!(
            belly > neck * 1.4,
            "a belly of {} on a neck of {} is a tin can",
            belly,
            neck
        );
    }

    /// Spec 0008: the basin is lined on every face, and the tiles are the same
    /// size on all of them.
    ///
    /// The depth comes from the tiles and not from the water, so a face left
    /// bare is a face with no depth, and a face whose tiles are a different
    /// size is a face at a different distance. One mesh for all five would make
    /// the tiles on the sides as tall as the sides are.
    #[test]
    fn the_basin_is_lined_all_over() {
        let (middle, size, deep) = pool(REACHES);
        let faces = lining(REACHES);

        assert_eq!(faces.len(), 5, "a basin is a bottom and four sides");

        // one looking up and four looking in across the water
        let up = faces.iter().filter(|(_, _, looks)| looks.y > 0.5).count();
        assert_eq!(up, 1, "{} of them are the bottom", up);

        for (at, face, looks) in &faces {
            assert!(
                (looks.length() - 1.0).abs() < 1e-5,
                "a face looks {:?}",
                looks
            );
            assert!(
                face.x > 0.0 && face.y > 0.0,
                "a face {} by {}",
                face.x,
                face.y
            );

            // inside the hole it lines, give or take the hair it stands proud
            assert!(
                (at.x - middle.x).abs() <= size.x * 0.5 + 1e-3
                    && (at.z - middle.z).abs() <= size.y * 0.5 + 1e-3,
                "a face at {:?} is outside the pool",
                at
            );
            assert!(
                at.y <= middle.y + FREEBOARD + 1e-3 && at.y >= middle.y - deep - 1e-3,
                "a face at {} is outside a basin from {} to {}",
                at.y,
                middle.y - deep,
                middle.y + FREEBOARD
            );

            // and every one of them carries whole tiles of the same size
            let counts = *face / TILE;
            assert!(
                counts.x > 1.0 && counts.y > 0.9,
                "a face of {} by {} tiles",
                counts.x,
                counts.y
            );
        }

        // the bottom is the pool's own size, so the tiles on it are the scale
        // everything else is read against
        let (_, bottom, _) = faces[0];
        assert!(
            (bottom - size).length() < 1e-4,
            "the bottom is {:?} and the pool is {:?}",
            bottom,
            size
        );
    }

    /// Spec 0008: the tiling on the near wall starts outside this room's own
    /// face on it, not on the cellar's stone behind that.
    ///
    /// The wall between the two rooms is the cellar's, and this room lays a
    /// skin of its own over it. Tiling taken from the wall instead of from the
    /// skin fills the same slab, and the two then fight for every pixel: a
    /// chequerboard dissolving down one side of the room, which is the fourth
    /// time this building has drawn two surfaces in one place.
    ///
    /// Not a general rule about overlapping, which would be wrong: a pier is
    /// deeper than the dado it breaks and buries part of it, and a face inside
    /// another solid is hidden rather than fighting.
    #[test]
    fn the_tiling_starts_outside_the_skin() {
        let side = near(REACHES);
        let face = side + SKIN;
        let mut found = 0;

        for (box_, made) in fittings(REACHES) {
            if !matches!(made, Made::Dado | Made::Band | Made::Pier) {
                continue;
            }
            // only the pieces on the near wall, which are the ones that meet
            // the skin at all
            if box_.min.z > face + 1e-3 {
                continue;
            }

            found += 1;
            assert!(
                box_.min.z >= face - 1e-4,
                "a {:?} begins at {} and this room's face is at {}",
                made,
                box_.min.z,
                face
            );
        }

        assert!(found > 0, "no tiling on the near wall at all");
    }

    /// Spec 0008: the stream moves along itself, and the splash beats.
    ///
    /// A thread wavering from side to side is a wobbly rod: nothing about it
    /// moves the way it is pointing, which is what Jake read off it straight
    /// away. Flowing is the beads running down, so what has to be true is that
    /// the shape at one moment is not the shape at the next, and that the
    /// difference is along the thread rather than across it.
    #[test]
    fn the_stream_runs_rather_than_wiggles() {
        let one = stream_mesh(0.0);
        let other = stream_mesh(0.14);

        assert_eq!(one.vertices.len(), other.vertices.len());

        let mut moved = 0;
        for (a, b) in one.vertices.iter().zip(other.vertices.iter()) {
            // the thread keeps its length: a bead travels down it, the whole
            // thing does not slide
            assert!(
                (a.position[1] - b.position[1]).abs() < 1e-5,
                "the stream moved down bodily rather than beading"
            );
            if (a.position[0] - b.position[0]).abs() > 1e-4 {
                moved += 1;
            }
        }

        assert!(
            moved > one.vertices.len() / 4,
            "only {} of {} points moved, so it is the same thread",
            moved,
            one.vertices.len()
        );

        // and the splash spreads and fades together, so it reads as one ring
        // rather than a flicker
        let (early, late) = (splash(0.0), splash(0.2));
        assert!(late > early, "the ring does not spread");
        assert!(
            splashed(0.2) < splashed(0.0),
            "the ring spreads without fading"
        );
        assert!(splashed(0.0) > 0.0, "a splash nobody can see");
    }

    /// Spec 0008: the pool does not run away with itself.
    ///
    /// Left open for a couple of hours the surface blew up into streaks the
    /// height of the room. The stirring puts energy in every fifth of a second
    /// for as long as the game is running, and damping is the only thing taking
    /// it out: if the two do not balance, the pool is a slow bomb.
    #[test]
    fn the_pool_settles_rather_than_building_up() {
        let (middle, size, deep) = pool(REACHES);
        let mut water = blitzkit::water::Water::new(middle, size, deep, CELLS);
        water.damping = SETTLES;
        water.speed = RUNS;

        let step = 1.0 / 60.0;
        let mut since = 0.0f32;
        let mut owed = 0.0f32;
        let mut most = 0.0f32;

        // ten minutes of standing there watching it
        for _ in 0..36_000 {
            since += step;
            owed += step;
            if owed >= STIRS {
                owed -= STIRS;
                water.push(stirred(middle, size, since), STIR_WIDE, STIRRED);
            }
            water.step(step);

            let high = off_still(&water);
            most = most.max(high);

            assert!(
                high < 0.5,
                "after {:.0} seconds the surface reached {} off still",
                since,
                high
            );
        }

        assert!(most > 0.001, "the pool never moved at all");
    }

    /// Spec 0008: nor does it when somebody is wading about in it.
    ///
    /// The stirring is a tenth of what walking puts in. The first two of these
    /// tests ran the pool for ten minutes with nobody in it, which is why they
    /// both passed while the thing Jake was looking at was a wall of streaks:
    /// he was standing in the water.
    #[test]
    fn the_pool_settles_with_somebody_in_it() {
        let (middle, size, deep) = pool(REACHES);
        let mut water = blitzkit::water::Water::new(middle, size, deep, CELLS);
        water.damping = SETTLES;
        water.speed = RUNS;

        let pace = crate::SPEED * WADE;

        // and at every frame rate anybody's screen runs at, because the wake
        // goes in once a frame: a cap per frame is not a cap at all, and the
        // pool blew up on a hundred and twenty where sixty was fine.
        for rate in [30.0f32, 60.0, 120.0, 240.0] {
            settles_at(rate, pace);
        }
    }

    /// Walks somebody up and down the pool for five minutes at a given frame
    /// rate, and says it stayed a pool.
    fn settles_at(rate: f32, pace: f32) {
        let (middle, size, deep) = pool(REACHES);
        let mut water = blitzkit::water::Water::new(middle, size, deep, CELLS);
        water.damping = SETTLES;
        water.speed = RUNS;

        let step = 1.0 / rate;
        let mut since = 0.0f32;
        let mut owed = 0.0f32;

        for _ in 0..(300.0 * rate) as u32 {
            since += step;
            owed += step;
            if owed >= STIRS {
                owed -= STIRS;
                water.push(stirred(middle, size, since), STIR_WIDE, STIRRED);
            }

            let at = vec3(
                middle.x + (since * 0.7).sin() * size.x * 0.4,
                middle.y,
                middle.z + (since * 0.5).cos() * size.y * 0.4,
            );
            water.ring(
                at,
                crate::RADIUS,
                crate::RADIUS * blitzkit::water::SPLASH,
                (pace * WAKE).min(WAKE_MOST) * step,
            );
            water.step(step);

            let high = off_still(&water);

            assert!(
                high < 0.5,
                "at {} frames a second, after {:.0} seconds of wading the surface reached {} off still",
                rate,
                since,
                high
            );
        }
    }

    /// Spec 0008: and neither does the tub.
    ///
    /// Smaller water and a bigger push: the blower goes every third of a second
    /// into two units across, where the pool's stir is a tenth the size into
    /// eight. Whichever of them ran away with itself, the way to find out is to
    /// run them both for longer than anybody would stand there.
    #[test]
    fn the_tub_settles_rather_than_building_up() {
        let (surface, wide, deep) = tub(REACHES);
        let mut water =
            blitzkit::water::Water::new(surface, glam::vec2(wide, wide), deep, TUB_CELLS);

        let step = 1.0 / 60.0;
        let mut owed = 0.0f32;
        let mut since = 0.0f32;

        for _ in 0..36_000 {
            since += step;
            owed += step;
            if owed >= BLOWS {
                owed -= BLOWS;
                water.push(water.at, TUB * 0.35, -BLOWN);
            }
            water.step(step);

            let high = off_still(&water);

            assert!(
                high < 0.4,
                "after {:.0} seconds the tub reached {} off still",
                since,
                high
            );
        }
    }

    /// Spec 0008: water slows you.
    #[test]
    fn wading_is_slower_than_walking() {
        let (middle, _, deep) = pool(REACHES);
        let standing = middle - Vec3::Y * deep * 0.5;
        let coping = middle + Vec3::Y * 0.01;

        assert!(
            wading(REACHES, standing),
            "standing in the pool is not wading"
        );
        assert!(!wading(REACHES, coping), "standing on the lip is wading");
        assert!(
            !wading(REACHES, standing + Vec3::X * 20.0),
            "the whole room is the pool"
        );

        assert!(
            pace(REACHES, standing, crate::SPEED) < pace(REACHES, coping, crate::SPEED),
            "the water does not slow you: {} against {}",
            pace(REACHES, standing, crate::SPEED),
            pace(REACHES, coping, crate::SPEED)
        );
    }

    /// Spec 0008: and your eye goes down with the floor under you, which is to
    /// say the basin is a floor and not a hole.
    #[test]
    fn your_eye_drops_as_you_wade() {
        let floor = at(REACHES).y;
        let basin = built(REACHES)
            .into_iter()
            .find(|(_, made)| *made == Made::Basin)
            .map(|(box_, _)| box_)
            .expect("a basin to stand on");

        assert!(
            (basin.max.y - (floor - POOL_DEEP)).abs() < 1e-4,
            "the basin's floor is at {} and the water is {} deep",
            basin.max.y,
            POOL_DEEP
        );
        // standing on it your eye is still above the water
        assert!(
            basin.max.y + crate::EYE > floor,
            "standing in the pool your eye is under the surface"
        );
    }
}
