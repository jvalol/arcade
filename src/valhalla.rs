//! What is through the door at the top of the bridge. Spec 0056.
//!
//! **Nothing here exists until the bridge does**, which is until every room
//! has given up what it asks: spec 0054's rule, read from the same list, so
//! somebody who never finds the trail cannot tell any of this is here.
//!
//! **A hall and its steading and not a hall alone.** The myth gives Valhalla
//! as one great building and Jake asked for somewhere village-like, and a
//! Norse settlement is both at once: a longhouse with smaller buildings
//! round it. Spec 0055 carries the argument.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::trail::Trail;

/// How far the ground runs west from the landing's far edge, and how far
/// across.
///
/// **Thirty-two of the thirty-five metres there are.** The field's ground
/// stops at `field::GROUND_REACH` and the sky is sized to clear the ground,
/// so a place that runs past it is a place hanging out through the sky.
/// This does not need ground under it, being twenty-six metres up, but
/// staying inside that reach is what keeps it inside the weather.
pub const DEEP: f32 = 32.0;
pub const SPAN: f32 = 46.0;

/// How thick its ground is, which is only what you see of the edge.
pub const GROUND_DEEP: f32 = 1.4;

/// How far its ground lies under the landing's.
///
/// **Because the two lap, and two tops at one height that lap is the fault
/// this building has fought six times.** The ground runs a metre back
/// under spec 0055's landing so that a walk across does not meet two faces
/// at the walking surface, and with both tops at 26.0 exactly that metre
/// was a 1 by 46 band of z-fighting: Jake stood at the top of the bridge
/// and saw it stippling.
///
/// Lapping is the right move when one face ends up buried. Flush and
/// lapped, neither is: they are both on show in one plane.
///
/// Two centimetres, which is `field::DROP` and is there for exactly this
/// reason, the field's ground under the garden's. What it costs is a two
/// centimetre step on the way through, which `walk` takes without
/// noticing.
pub const UNDER: f32 = 0.02;

/// The hall: how long, how wide, how high to the eaves and how high to the
/// ridge.
///
/// **Twenty-four by nine**, which is a real longhouse and is also the only
/// size that makes the outbuildings read as outbuildings. Every room in
/// this building has been too small on its first build.
pub const HALL_LONG: f32 = 24.0;
pub const HALL_WIDE: f32 = 9.0;
pub const HALL_EAVES: f32 = 3.4;
pub const HALL_RIDGE: f32 = 6.6;

/// How wide the hall's door is and how high.
pub const DOOR_WIDE: f32 = 2.4;
pub const DOOR_HIGH: f32 = 2.8;

/// Where the middle of this place is: west of the landing by half its own
/// depth, on the line you walk in on.
pub fn at(reaches: f32) -> Vec3 {
    let top = crate::bridge::top(reaches);

    vec3(
        top.x - crate::bridge::LANDING_OUT - DEEP * 0.5,
        top.y - UNDER,
        top.z,
    )
}

/// Its ground, as the one slab.
///
/// Lapped back under the landing by a stride, because two faces at the
/// walking surface stop a walk dead and `walk` reports nothing at all, and
/// [`UNDER`] below it over that lap, because two tops in one plane
/// stipple.
pub fn ground(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let east = crate::bridge::top(reaches).x - crate::bridge::LANDING_OUT + 1.0;

    Aabb::new(
        vec3(
            middle.x - DEEP * 0.5,
            middle.y - GROUND_DEEP,
            middle.z - SPAN * 0.5,
        ),
        vec3(east, middle.y, middle.z + SPAN * 0.5),
    )
}

/// One building up here: where it stands, how big, and whether it is the
/// hall.
#[derive(Clone, Copy, Debug)]
pub struct Built {
    /// The middle of its footprint, on the ground.
    pub at: Vec3,
    /// How long it runs north and south, how wide east and west.
    pub long: f32,
    pub wide: f32,
    pub eaves: f32,
    pub ridge: f32,
    pub hall: bool,
}

impl Built {
    /// The box of its walls, which is what you walk round.
    pub fn walls(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.eaves * 0.5,
            vec3(self.wide, self.eaves, self.long),
        )
    }

    /// And the box its roof fills, taken as the whole of it, which is what
    /// one building must not share with another.
    ///
    /// Only the tests want it: what is drawn is the walls and the two
    /// pitches, and what holds you up is the walls.
    #[cfg(test)]
    pub fn footprint(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.ridge * 0.5,
            vec3(self.wide, self.ridge, self.long),
        )
    }
}

/// The hall and its outbuildings.
///
/// **The hall faces the bridge**, so what you see on arriving is its door
/// and not its flank. The others stand off the line you walk in on, which
/// leaves that line open and gives the place a lane rather than a yard.
pub fn built(reaches: f32) -> Vec<Built> {
    let middle = at(reaches);
    let one =
        |off_x: f32, off_z: f32, long: f32, wide: f32, eaves: f32, ridge: f32, hall: bool| Built {
            at: vec3(middle.x + off_x, middle.y, middle.z + off_z),
            long,
            wide,
            eaves,
            ridge,
            hall,
        };

    vec![
        // the hall, laid across the way you come in so its gable end and
        // its door are what face you
        one(
            -4.0, 0.0, HALL_WIDE, HALL_LONG, HALL_EAVES, HALL_RIDGE, true,
        ),
        // and the steading, off the lane either side
        one(7.5, -13.0, 6.0, 8.0, 2.4, 4.0, false),
        one(6.0, 13.5, 5.0, 7.0, 2.2, 3.6, false),
        one(-10.0, -14.5, 7.0, 6.0, 2.3, 3.8, false),
    ]
}

/// Where the hall's door is, and how big.
///
/// Its own account, because the light that comes out of it and the walk
/// that goes in through it both ask, and two places saying where a door is
/// is the fault this building keeps making.
pub fn doorway(reaches: f32) -> (Vec3, f32, f32) {
    let hall = built(reaches)[0];

    (
        vec3(hall.at.x + hall.wide * 0.5, hall.at.y, hall.at.z),
        DOOR_WIDE,
        DOOR_HIGH,
    )
}

/// How deep an opening is set into its wall, and how far the jamb round it
/// stands proud.
///
/// **So that it is a hole and not a panel.** Set flush and lit, the hall's
/// door was a bright rectangle painted on a wall. Set back behind a dark
/// reveal, the same rectangle is a room you are not in.
pub const SET_BACK: f32 = 0.34;
pub const JAMB: f32 = 0.16;

/// How far a lit opening stands off the wall it is in, so the two are not
/// one surface, and how wide and high a window is.
pub const PROUD: f32 = 0.03;
pub const WINDOW_WIDE: f32 = 0.9;
pub const WINDOW_HIGH: f32 = 1.1;
pub const WINDOW_UP: f32 = 1.5;

/// One lit opening: where it is, how big, and which way it faces.
#[derive(Clone, Copy, Debug)]
pub struct Lit {
    pub at: Vec3,
    pub wide: f32,
    pub high: f32,
    pub looks: Vec3,
}

/// Every lit opening up here: the hall's door, and windows down the side of
/// everything.
///
/// **The light is in the openings and not on the walls.** The whole of spec
/// 0055's brief is that the source is out of frame: a wall that glows is a
/// lamp, and a window that glows is a room you are not in.
/// **And the hall's door is not one of them any more.** Spec 0060 made
/// the hall hollow, and a lit panel across the way in is a painted
/// rectangle where there is now a room: you walk up to the thing it was
/// standing in for. What comes out of that doorway is the fire.
pub fn lit(reaches: f32) -> Vec<Lit> {
    let mut out = Vec::new();
    let (door, wide, _) = doorway(reaches);

    for one in built(reaches) {
        // down the face that looks at the lane, which for everything here
        // is its east side
        let face = one.at.x + one.wide * 0.5 + PROUD;
        let count = ((one.long / 4.0).floor() as i32).max(1);

        for n in 0..count {
            let along = one.at.z + (n as f32 + 0.5) / count as f32 * one.long - one.long * 0.5;

            // not where the door is
            if one.hall && (along - door.z).abs() < wide {
                continue;
            }

            out.push(Lit {
                at: vec3(face - SET_BACK, one.at.y + WINDOW_UP, along),
                wide: WINDOW_WIDE,
                high: WINDOW_HIGH,
                looks: Vec3::X,
            });
        }
    }

    out
}

/// Where the people stand, and which way each faces.
///
/// **Nobody is fighting and nobody is waiting for a war.** Spec 0055: keep
/// the figures, drop the rest of what the myth puts in this hall.
pub fn people(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);
    let (door, _, _) = doorway(reaches);

    vec![
        // one by the door, looking out along the lane at whoever is coming
        (
            vec3(door.x + 1.8, door.y, door.z + 2.2),
            std::f32::consts::FRAC_PI_2,
        ),
        // and two further in, together, facing each other
        (vec3(middle.x + 6.0, middle.y, middle.z - 5.5), 2.6),
        (vec3(middle.x + 7.4, middle.y, middle.z - 6.4), -0.5),
    ]
}

/// What you cannot walk through up here.
///
/// **The hall is hollow and the outbuildings are not.** Spec 0060: the
/// hall is the one you are meant to arrive in, so its walls are six
/// boxes with a way through them and the other three stay the solid
/// lumps they have always been.
pub fn solid(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !crate::bridge::shows(trail) {
        return Vec::new();
    }

    let mut out = vec![ground(reaches)];

    out.extend(
        built(reaches)
            .iter()
            .filter(|one| !one.hall)
            .map(Built::walls),
    );
    out.extend(hall_walls(reaches));
    out.push(floor(reaches));
    out.push(fire(reaches));
    out.extend(posts(reaches));
    out.extend(ties(reaches));
    out.extend(benches(reaches));
    out.push(high_seat(reaches));

    out
}

/// Where the light that spills out of each building sits.
///
/// **One a building and not one a window.** The lamp is inside and the
/// openings are holes in the same room, so a light per hole is the same
/// room lit four times. Just outside the face the openings are in, so what
/// it throws lands on the lane rather than inside a wall.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    built(reaches)
        .into_iter()
        .map(|one| {
            // well out from the face and low, and further out than looks
            // right written down. Close in it shone back into the opening
            // it was supposed to be coming out of and burnt a white core
            // into the panel, and it lit the wall harder than the ground,
            // which washed a daubed wall to cream. Out here the wall is
            // five metres from it and the lane is one, and a light falls
            // off with the square of that.
            vec3(one.at.x + one.wide * 0.5 + 5.0, one.at.y + 1.0, one.at.z)
        })
        .collect()
}

/// A box from two corners, which is what most of the inside is written
/// as. `Aabb::from_center_size` wants a middle and a size, and every
/// piece of a room is easier to say as the two faces it runs between.
fn corners(from: Vec3, to: Vec3) -> Aabb {
    Aabb::from_center_size((from + to) * 0.5, to - from)
}

/// The hall itself, named once.
///
/// **Because four things ask where it is.** The doorway, the walls, the
/// inside and the roof's hole all read it, and `built(reaches)[0]` in
/// four places is four chances to point at an outbuilding.
pub fn hall(reaches: f32) -> Built {
    built(reaches)[0]
}

/// How thick the hall's walls are.
///
/// **Daub on a frame, so not thin.** It is also the number that decides
/// how far the inside is from the outside, and a wall you can see the
/// thickness of through a doorway is most of what says you went in.
pub const WALL_THICK: f32 = 0.45;

/// How far the hall's floor stands above the yard outside it.
///
/// **Not flush, because flush is two faces in one plane.** The ground up
/// here is one slab under everything and a floor laid level with it
/// z-fights over the whole hall. A threshold is also just true. Well
/// under `walk::STEP`, so walking in is not a climb.
pub const FLOOR_UP: f32 = 0.06;

/// The clear air inside the hall.
pub fn inside(reaches: f32) -> Aabb {
    let one = hall(reaches);

    corners(
        vec3(
            one.at.x - one.wide * 0.5 + WALL_THICK,
            one.at.y + FLOOR_UP,
            one.at.z - one.long * 0.5 + WALL_THICK,
        ),
        vec3(
            one.at.x + one.wide * 0.5 - WALL_THICK,
            one.at.y + one.eaves,
            one.at.z + one.long * 0.5 - WALL_THICK,
        ),
    )
}

/// The hall's walls, which used to be one box and are now six.
///
/// **The flanks run the whole length and the gables fit between them.**
/// Mitred, the two would meet face to face in one plane down every
/// corner. Run through, the gable's ends are buried inside the flanks and
/// there is nothing in a plane with anything.
///
/// **And the east flank is in three pieces**, because the door is in it:
/// the run each side and the lintel over it. The lintel is above head
/// height and could be left out of the colliders without anybody
/// noticing, which is the reason to put it in.
pub fn hall_walls(reaches: f32) -> Vec<Aabb> {
    let one = hall(reaches);
    let (door, wide, high) = doorway(reaches);
    let (foot, head) = (one.at.y, one.at.y + one.eaves);
    let (west, east) = (one.at.x - one.wide * 0.5, one.at.x + one.wide * 0.5);
    let (south, north) = (one.at.z - one.long * 0.5, one.at.z + one.long * 0.5);

    vec![
        corners(
            vec3(west, foot, south),
            vec3(west + WALL_THICK, head, north),
        ),
        corners(
            vec3(east - WALL_THICK, foot, south),
            vec3(east, head, door.z - wide * 0.5),
        ),
        corners(
            vec3(east - WALL_THICK, foot, door.z + wide * 0.5),
            vec3(east, head, north),
        ),
        corners(
            vec3(east - WALL_THICK, foot + high, door.z - wide * 0.5),
            vec3(east, head, door.z + wide * 0.5),
        ),
        corners(
            vec3(west + WALL_THICK, foot, south),
            vec3(east - WALL_THICK, head, south + WALL_THICK),
        ),
        corners(
            vec3(west + WALL_THICK, foot, north - WALL_THICK),
            vec3(east - WALL_THICK, head, north),
        ),
    ]
}

/// The floor you stand on in there, which is boards and not the yard.
pub fn floor(reaches: f32) -> Aabb {
    let one = hall(reaches);
    let air = inside(reaches);

    corners(
        vec3(air.min.x, one.at.y - 0.5, air.min.z),
        vec3(air.max.x, one.at.y + FLOOR_UP, air.max.z),
    )
}

/// The fire down the middle: how far it runs, how far across, and how
/// high its kerb stands.
///
/// **A trench down the length and not a hearth at one end.** You come
/// in at the east gable and the hall runs twenty-four metres away from
/// you, so a fire at the far end is a bright wall with a dark walk to
/// it. A trench is what you walk along, and it is what a longhouse has.
///
/// **Along `wide` and not along `long`.** The hall's `wide` is its
/// twenty-four metres and its `long` is its nine, which is the other
/// way round from what those names say and is the first thing this room
/// was built wrong: a sixteen metre fire laid along `long` runs out
/// through both walls.
pub const FIRE_LONG: f32 = 16.0;
pub const FIRE_WIDE: f32 = 1.3;

/// How high the kerb round it stands, which is **higher than a body walks
/// over**.
///
/// It was 0.22, and you walked up onto the hearth and along the length of
/// the fire through the flames. Jake, with a recording, 2026-11-11: in the
/// hall i can walk on the candles.
///
/// **Not `reach::STEP`, which is the tempting number and is the wrong
/// one.** A step is what `walk` climbs on purpose. What carries you over a
/// low kerb is not that at all: `in_the_way` keeps a box out of the walls
/// until it stands `rides` over your feet, and `rides` grows with distance,
/// so a box this low is still ground when the body's own sphere is already
/// touching it, and the sphere is then pushed up and over. Walked at,
/// rather than worked out: 0.38 is walked over and 0.42 stops you, which is
/// `RIDES` plus `RAMPS` across a body's radius, and a little.
///
/// So it is written as that sum. A number here would have been right until
/// somebody moved `RADIUS`, and nothing would have said so.
pub const FIRE_KERB: f32 = crate::walk::RIDES + crate::walk::RAMPS * crate::RADIUS + 0.05;

/// The trench, which is what you cannot walk into.
///
/// **The whole box and not the stones round it.** What is drawn is a rim
/// with an open middle; what stops you is the lot, because the open
/// middle is a fire.
pub fn fire(reaches: f32) -> Aabb {
    let one = hall(reaches);
    let foot = one.at.y + FLOOR_UP;

    corners(
        vec3(one.at.x - FIRE_LONG * 0.5, foot, one.at.z - FIRE_WIDE * 0.5),
        vec3(
            one.at.x + FIRE_LONG * 0.5,
            foot + FIRE_KERB,
            one.at.z + FIRE_WIDE * 0.5,
        ),
    )
}

/// How wide the stones round the rim are.
pub const RAIL_WIDE: f32 = 0.3;

/// The four stones round the rim, which is what is drawn of the kerb.
///
/// **A rim round a hole, not a lid on a box.** The first build drew the
/// whole trench as one stone with the coals laid over its top, which is
/// a raised block with a glowing lid: no trench, no depth, and the two
/// tops in one plane stippling down the length of the hall.
pub fn fire_rails(reaches: f32) -> Vec<Aabb> {
    let whole = fire(reaches);

    vec![
        corners(
            whole.min,
            vec3(whole.max.x, whole.max.y, whole.min.z + RAIL_WIDE),
        ),
        corners(
            vec3(whole.min.x, whole.min.y, whole.max.z - RAIL_WIDE),
            whole.max,
        ),
        // **The ends run between the sides and not across them.** Run
        // through, the two lap at each corner and their tops are two
        // exposed faces at one height: a thirty centimetre square of
        // stipple at each end of the trench.
        corners(
            vec3(whole.min.x, whole.min.y, whole.min.z + RAIL_WIDE),
            vec3(
                whole.min.x + RAIL_WIDE,
                whole.max.y,
                whole.max.z - RAIL_WIDE,
            ),
        ),
        corners(
            vec3(
                whole.max.x - RAIL_WIDE,
                whole.min.y,
                whole.min.z + RAIL_WIDE,
            ),
            vec3(whole.max.x, whole.max.y, whole.max.z - RAIL_WIDE),
        ),
    ]
}

/// The bed of coals inside the rim.
///
/// **Under the rim's top and not level with it.** Level, the two faces
/// fight; under, the rim throws a shadow line along the coals, which is
/// the thing that says there is a trench.
pub fn coals(reaches: f32) -> Aabb {
    let whole = fire(reaches);

    corners(
        vec3(
            whole.min.x + RAIL_WIDE,
            whole.min.y,
            whole.min.z + RAIL_WIDE,
        ),
        vec3(
            whole.max.x - RAIL_WIDE,
            whole.max.y - 0.07,
            whole.max.z - RAIL_WIDE,
        ),
    )
}

/// A flame, turned rather than boxed.
///
/// **Because a box the size of a flame is a crate.** Wide at the foot,
/// pinched, and drawn to a point: the first build put three cubes in
/// the trench and they read as three cubes.
pub fn flame_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(10, 14, |v| {
        let waist = (v * std::f32::consts::PI).sin().powf(0.7) * (1.0 - v * 0.55);

        (v, waist.max(0.0))
    })
}

/// How many lights the fire is and how high above the kerb they sit.
///
/// **Three, because sixteen metres lit from a point in the middle is a
/// hearth with a long dark trench either side.** Few enough that the
/// three read as one fire and the falloff between them is the length of
/// the hall rather than three lamps in a row.
pub const FIRES: usize = 3;
/// How wide a flame stands at its foot before its own size is applied, and
/// the smallest it comes out and how much bigger the largest is.
///
/// Here rather than at the one call that draws them, because what the
/// spacing below has to know is how wide they are.
pub const FLAME_WIDE: f32 = 0.46;
pub const FLAME_SMALL: f32 = 0.7;
pub const FLAME_GROWS: f32 = 0.75;

/// How far apart they stand: **closer than the narrowest of them is wide**,
/// so that every flame runs into the ones beside it whatever size the two
/// came out.
///
/// **This is the whole of what makes it a fire rather than candles.** At
/// twenty-two over the length they stood 0.68 apart against a foot of 0.32
/// to 0.67, so no two ever touched: a line of evenly spaced teardrops, each
/// one whole and separate. The note over `flame_sits` had already written
/// down what that would read as, and it read as exactly that. Jake, with a
/// recording, 2026-11-11: make the flames closer together and overlapping.
pub const FLAME_APART: f32 = FLAME_WIDE * FLAME_SMALL * 0.7;

/// And so how many there are, which is the length divided by that.
///
/// **A count derived and not chosen.** Written down it was right until
/// somebody moved the trench or the flame, and neither file would have
/// said so. The lights are a different question and stay at three: those
/// are what the room is lit by and these are what you look at.
pub const FLAMES: usize = ((FIRE_LONG - 1.0) / FLAME_APART) as usize;

/// Where a flame sits across the trench and how big it is, which is not
/// the same for any two of them.
///
/// **Because evenly spaced and all one size is a row of eggs.** Eleven
/// identical teardrops a metre and a third apart read as candles in a
/// line, which is what Jake would have said about them. A fire is one
/// ragged thing: close enough together to run into each other, and no
/// two the same.
///
/// Off the count rather than off a clock, so the fire is the same shape
/// from one frame to the next and only its burning moves.
pub fn flame_sits(which: usize) -> (f32, f32) {
    let mut hash = (which as u32).wrapping_mul(0x9e37_79b9);

    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x85eb_ca6b);
    hash ^= hash >> 13;

    let aside = (hash & 0xffff) as f32 / 65535.0 - 0.5;
    let size = ((hash >> 16) & 0xffff) as f32 / 65535.0;

    (
        aside * (FIRE_WIDE - RAIL_WIDE * 2.0 - 0.3),
        FLAME_SMALL + size * FLAME_GROWS,
    )
}
/// **Up at the flames and not down on the coals.** At a hand above the
/// bed, the three lamps lit the bed itself harder than anything else in
/// the room and it came out white: a sixteen metre lightbox. A fire
/// gives its light off from its flames.
pub const FIRE_UP: f32 = 0.62;

pub fn fires(reaches: f32) -> Vec<Vec3> {
    let one = hall(reaches);
    let foot = one.at.y + FLOOR_UP + FIRE_KERB + FIRE_UP;

    (0..FIRES)
        .map(|n| {
            let along = (n as f32 + 0.5) / FIRES as f32 - 0.5;

            vec3(one.at.x + along * FIRE_LONG, foot, one.at.z)
        })
        .collect()
}

/// The posts that carry the roof: how far either side of the middle, how
/// thick, how many bays they make, and how high the beam across them sits.
///
/// **Seven bays and so eight posts a side, which is an even number.** A
/// longhouse is a nave and two aisles and the posts are what makes the
/// length read as length, and they are what the firelight has to fall
/// across: a room lit off the floor with nothing standing up in it is a
/// lit floor. An odd count puts one of them in the middle, which is dead
/// in front of the door.
pub const POST_ASIDE: f32 = 2.6;
pub const POST_THICK: f32 = 0.32;
pub const BAYS: usize = 7;
pub const TIE_UP: f32 = 2.8;
pub const TIE_DEEP: f32 = 0.26;

/// Where each post stands along the hall, which is along `wide`.
fn post_line(reaches: f32) -> Vec<f32> {
    let one = hall(reaches);
    let run = one.wide - WALL_THICK * 2.0 - POST_THICK * 2.0 - 1.2;

    (0..=BAYS)
        .map(|n| one.at.x + (n as f32 / BAYS as f32 - 0.5) * run)
        .collect()
}

pub fn posts(reaches: f32) -> Vec<Aabb> {
    let one = hall(reaches);
    let foot = one.at.y + FLOOR_UP;
    let head = one.at.y + TIE_UP;
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        for along in post_line(reaches) {
            let middle = vec3(along, (foot + head) * 0.5, one.at.z + side * POST_ASIDE);

            out.push(Aabb::from_center_size(
                middle,
                vec3(POST_THICK, head - foot, POST_THICK),
            ));
        }
    }

    out
}

/// The beams across the hall, one on each pair of posts.
pub fn ties(reaches: f32) -> Vec<Aabb> {
    let one = hall(reaches);

    post_line(reaches)
        .into_iter()
        .map(|along| {
            Aabb::from_center_size(
                vec3(along, one.at.y + TIE_UP + TIE_DEEP * 0.5, one.at.z),
                vec3(POST_THICK, TIE_DEEP, POST_ASIDE * 2.0 + POST_THICK),
            )
        })
        .collect()
}

/// The benches down each wall, and the one seat that is not a bench.
pub const BENCH_HIGH: f32 = 0.52;
pub const BENCH_DEEP: f32 = 1.1;

/// **Down both flanks, and nothing in the way of the door.** The door is
/// in the east gable, so a bench against either flank is beside it and
/// not across it, and both run the whole length. They fill the aisle
/// from the wall to just short of the posts, which is a strip narrower
/// than a body and was never anywhere to walk.
pub fn benches(reaches: f32) -> Vec<Aabb> {
    let one = hall(reaches);
    let air = inside(reaches);
    let foot = one.at.y + FLOOR_UP;
    let head = foot + BENCH_HIGH;
    let seat = high_seat(reaches);

    [-1.0f32, 1.0]
        .iter()
        .copied()
        .map(|side| {
            let wall = one.at.z + side * (one.long * 0.5 - WALL_THICK);
            let in_to = wall - side * BENCH_DEEP;

            corners(
                vec3(seat.max.x + 0.2, foot, wall.min(in_to)),
                vec3(air.max.x - 0.5, head, wall.max(in_to)),
            )
        })
        .collect()
}

/// The high seat at the west gable, which is the one thing in here that
/// is somewhere rather than somewhere to sit.
///
/// **At the far end, which is what you walk towards.** You come in at
/// the east gable and the fire runs away from you down the length; the
/// thing at the end of it is what the walk is for.
pub fn high_seat(reaches: f32) -> Aabb {
    let one = hall(reaches);
    let air = inside(reaches);
    let foot = one.at.y + FLOOR_UP;

    corners(
        vec3(air.min.x, foot, one.at.z - 1.7),
        vec3(air.min.x + 1.2, foot + BENCH_HIGH + 0.3, one.at.z + 1.7),
    )
}

/// How far along the slope a pitch runs, which is the one number the
/// roof's pieces are cut out of.
pub fn slope_run(one: &Built) -> f32 {
    (one.wide * 0.5).hypot(one.ridge - one.eaves)
}

/// The pieces one pitch of a roof is drawn in: how far along the slope
/// each runs, from the eaves at nought to the ridge at one, and the two
/// ends of it along the building.
///
/// **One piece, and the roof is whole again.** It was three, with a
/// louvre cut out of the ridge and a painted night hung under the hole.
/// Spec 0063's note on that room applies to a hole as well as to a room:
/// a small rectangle of sky in a dark ceiling reads as a panel with
/// something playing on it, whatever is painted on it. Jake, 2026-11-11,
/// twice: it looks like a rectangle of swirling colour, and then a small
/// rectangle doesn't land, just make it the whole ceiling.
///
/// So the night is the ceiling now, per [`night_pitches`], and the roof
/// over it has nothing cut out of it.
pub fn pitch_pieces(one: &Built) -> Vec<(f32, f32, f32, f32)> {
    // the gable overhang, which is what the length was being multiplied
    // by before any of this
    let over = one.long * 0.52;

    vec![(0.0, 1.0, -over, over)]
}

/// How far under the roof the night hangs.
///
/// **Under it and not in it.** Two faces in one plane is the fault this
/// building has hit eight times, and the roof's own boards are what the
/// sky sees from outside. Clear of the pitch's own half thickness, which
/// is 0.11.
pub const NIGHT_UNDER: f32 = 0.12;

/// How deep a rafter is, and how far under the night it hangs.
///
/// **Under the night and not through it**, which is what makes the
/// rafters read as beams across a sky rather than as lines drawn on one.
/// The night is a plane and a rafter is a box, and a plane that passes
/// through a box leaves a seam along it: at 0.16 under the pitch the
/// night at 0.12 cut every rafter in the hall lengthways.
///
/// Written as the sum so the two cannot drift back together.
pub const RAFTER_DEEP: f32 = 0.13;
pub const RAFTER_UNDER: f32 = NIGHT_UNDER + RAFTER_DEEP * 0.5 + 0.02;

/// Said at build time rather than in a test, the way `back::GAUGE_UP`
/// says its own: a rafter cut lengthways by the sky is a thing you see
/// rather than a thing you measure, and this refuses to compile instead.
const _: () = assert!(RAFTER_UNDER - RAFTER_DEEP * 0.5 > NIGHT_UNDER);

/// The painted night, as the two slopes of the roof seen from inside:
/// where each one's middle is, how big its quad is, and how far it is
/// rolled.
///
/// **The whole ceiling and not a hole in it.** What a longhouse gives you
/// when you lie on a bench is the whole underside of its roof, and if
/// that is night then the room is open to it. A rectangle in the middle
/// is a skylight, and a skylight is a thing in a ceiling rather than the
/// ceiling being gone.
///
/// Drawn rather than opened, because from outside this has to stay a
/// roof: the building reads from the top of the bridge by its roofline
/// and one lantern.
pub fn night_pitches(reaches: f32) -> Vec<(Vec3, Vec3, f32)> {
    let one = hall(reaches);
    let rise = one.ridge - one.eaves;
    let slope = (rise / (one.wide * 0.5)).atan();
    let along = slope_run(&one);

    [-1.0f32, 1.0]
        .iter()
        .copied()
        .map(|side| {
            let eaves = vec3(
                one.at.x + side * one.wide * 0.5,
                one.at.y + one.eaves,
                one.at.z,
            );
            let ridge = vec3(one.at.x, one.at.y + one.ridge, one.at.z);
            let middle = (eaves + ridge) * 0.5 - Vec3::Y * NIGHT_UNDER;

            (middle, vec3(along, 1.0, one.long), -side * slope)
        })
        .collect()
}

/// The triangle over each gable wall, between the eaves and the roof.
///
/// **Open until somebody could get inside.** The walls stop at the eaves
/// and the roof rises past them to the ridge, which from outside is a
/// gap tucked under an overhang and from inside is a triangle of night
/// sky at each end of the hall. Nobody could be inside until spec 0060,
/// so nobody had seen it.
///
/// A tent in a unit square: `x` from minus one to one, `y` from nought
/// at the eaves to one at the apex, flat in `z`. Scaled to the half
/// width and the rise, it is the shape the two pitches leave.
///
/// **Flat and drawn twice.** It is one surface and has one side, so each
/// gable gets two: one facing in and one facing out, the second turned
/// about `y` rather than scaled by a negative, which is the thing that
/// turns a mesh inside out quietly.
pub fn gable_mesh() -> blitzkit::mesh::MeshData {
    let mut mesh = blitzkit::mesh::MeshData::surface(28, 10, |u, v| {
        let across = u * 2.0 - 1.0;

        vec3(across, v * (1.0 - across.abs()), 0.0)
    });

    mesh.compute_normals();
    mesh
}

/// How far either side of the gable's middle plane its two faces sit.
pub const GABLE_APART: f32 = 0.16;

/// How fine the painted night is drawn.
pub const NIGHT_TEXELS: u32 = 512;

/// The eddies the sky turns about: where each one is, how many times the
/// band goes round it, and how far its pull reaches.
const EDDIES: [(f32, f32, f32, f32); 3] = [
    (0.32, 0.38, 3.0, 2.6),
    (0.70, 0.60, -2.0, 3.4),
    (0.52, 0.80, 2.0, 4.2),
];

/// How many stars there are, how wide the widest one's halo is, how faint
/// the faintest burns, and how steeply the bright ones thin out.
///
/// **Seven was the whole of the trouble.** Seven hand placed stars over a
/// swirl leaves the swirl carrying the picture, and a swirl carrying a
/// picture is a pattern rather than a sky: Jake, 2026-11-11, is it supposed
/// to be the northern lights, it looks like a rectangle of swirling colour.
/// What a night has is a great many faint ones and a few that catch you.
///
/// `STAR_FEW` is the exponent the pick is raised to, so most come out near
/// the faint end and the bright ones are rare without being listed.
pub const STARS: usize = 54;
const STAR_HALO: f32 = 0.055;
const STAR_FAINT: f32 = 0.18;
const STAR_FEW: f32 = 3.0;

/// Where one of them is, how far its halo reaches and how hard it burns.
///
/// Off a hash rather than a table, the way `flame_sits` is: fifty-four
/// stars written out is fifty-four rows nobody can check, and the thing
/// worth saying about them is the distribution and not the places.
fn star_at(which: usize) -> (f32, f32, f32, f32) {
    let mut hash = (which as u32).wrapping_mul(0x9e37_79b9);

    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x85eb_ca6b);
    hash ^= hash >> 13;

    let u = (hash & 0x3ff) as f32 / 1023.0;
    let v = ((hash >> 10) & 0x3ff) as f32 / 1023.0;
    let pick = ((hash >> 20) & 0x3ff) as f32 / 1023.0;
    let hard = STAR_FAINT + pick.powf(STAR_FEW) * (1.0 - STAR_FAINT);

    (u, v, STAR_HALO * (0.4 + hard * 0.8), hard)
}

/// How much of the swirl survives into the colour.
///
/// **It was the subject and it wants to be the ground.** At its full
/// strength the bands are what you read, which is the whole of why the
/// panel looked like a screen with something playing on it rather than a
/// hole with a night behind it.
const SWIRL: f32 = 0.42;

/// A night painted rather than photographed, for the hole in the ridge.
///
/// **It is a panel in the hall and not the sky.** Spec 0056 parked the
/// aurora because the weather moves by sliding the sky's own picture and
/// there is one sky for the whole building, so anything painted into it
/// turns up over the zen garden. A picture under a hole owes nothing to
/// any of that.
///
/// **Swirl, then stars, in that order.** The swirl is the ground and the
/// stars are laid over it, which is the way round the painting is made
/// and the way round that keeps a star from being half a band.
///
/// The stars are a bright core inside a wide halo rather than bright
/// dots. This canvas clips at one, so a star that is only brighter is a
/// white dot with a hard edge, which is the fault the runestone's carving
/// spent a day on.
pub fn night_sky() -> blitzkit::texture::TextureData {
    let n = NIGHT_TEXELS;
    let mut pixels = vec![0u8; (n * n * 4) as usize];
    // the two ends of the sky's own colour, which the bands run between
    // **Deep, because the panel is multiplied past one.** At the colour
    // a night sky looks on paper, every band of it clipped and the
    // whole thing came out light blue with white blotches in it.
    // **Far darker than it was.** It ran to a cobalt that clipped in blue
    // once `NIGHT_BURNS` had multiplied it, so the hole in the ridge was a
    // lit rectangle in a dark ceiling. A night sky is nearly black and what
    // you see in it is the stars.
    let deep = vec3(0.008, 0.016, 0.055);
    let pale = vec3(0.030, 0.085, 0.210);
    let star = vec3(1.0, 0.74, 0.26);

    for down in 0..n {
        for across in 0..n {
            let u = (across as f32 + 0.5) / n as f32;
            let v = (down as f32 + 0.5) / n as f32;
            let mut band = ((u * 11.0).sin() + (v * 9.3).cos()) * 0.17;

            for (at_u, at_v, turn, reach) in EDDIES {
                let (du, dv) = (u - at_u, v - at_v);
                let away = (du * du + dv * dv).sqrt().max(1e-3);

                // round the eddy and out from it at once, which is what
                // makes a band curl rather than ring
                band += (dv.atan2(du) * turn + away * 24.0).sin() * (-(away * reach)).exp();
            }

            let mut colour = deep + (pale - deep) * (band * SWIRL * 0.5 + 0.5).clamp(0.0, 1.0);

            for (at_u, at_v, reach, hard) in (0..STARS).map(star_at) {
                let away = (u - at_u).hypot(v - at_v);
                let near = 1.0 - (away / reach).min(1.0);
                // the halo, and a core inside it that goes white
                let halo = near * near * near;
                let core = (1.0 - (away / (reach * 0.26)).min(1.0)).powi(2);

                colour += star * hard * halo + Vec3::ONE * hard * core;
            }

            let at = ((down * n + across) * 4) as usize;

            for (c, one) in colour.to_array().iter().copied().enumerate() {
                pixels[at + c] = (one.clamp(0.0, 1.0) * 255.0).round() as u8;
            }

            pixels[at + 3] = 255;
        }
    }

    blitzkit::texture::TextureData::from_pixels(n, n, pixels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::Asked;

    const REACHES: f32 = 20.0;

    /// Everything you can walk into up here, which is this place and the
    /// bridge it stands on the end of.
    fn everything() -> Vec<Aabb> {
        let mut out = crate::bridge::solid(REACHES, &done());
        out.extend(solid(REACHES, &done()));
        out
    }

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// Spec 0056: a hall in the sky over a field with no way up to it is
    /// worse than nothing, so it is there exactly when the bridge is.
    #[test]
    fn nothing_until_the_bridge_is_there() {
        assert!(solid(REACHES, &Trail::new()).is_empty());
        assert!(!solid(REACHES, &done()).is_empty());
    }

    /// Spec 0056: stepping through is stepping across and not up, and the
    /// two do not share a plane while they are at it.
    ///
    /// **This test asked for the fault.** It said the tops were equal and
    /// that the slabs lapped, which together is a metre by forty-six of
    /// two exposed faces in one plane: Jake saw it stippling from the top
    /// of the bridge. Lapping only works where one face ends up buried.
    #[test]
    fn the_ground_is_flush_with_the_landing() {
        let ours = ground(REACHES);
        let landing = crate::bridge::landing(REACHES, &done())[0];
        let under = landing.max.y - ours.max.y;

        assert!(
            under > 1e-4,
            "its ground and the landing are both at {:.3}, so the metre \
             they lap over is two faces in one plane",
            ours.max.y
        );
        assert!(
            under < crate::walk::STEP,
            "its ground is {:.3} under the landing, which is a step and \
             not a lap",
            under
        );
        assert!(
            ours.max.x > landing.min.x + 1e-3,
            "it stops at {:.3} and the landing starts at {:.3}, which is \
             two faces in one plane at the walking surface",
            ours.max.x,
            landing.min.x
        );
    }

    /// Spec 0056: and you can actually get across.
    /// Spec 0060: the hearth stops you rather than carrying you.
    ///
    /// **Walked at and not measured.** The kerb was 0.22 against a step of
    /// 0.25 and every number in that trench was right: it was the right
    /// width, the right length, drawn as a rim with an open middle, and
    /// solid all the way across so that nobody could get into the fire.
    /// What it was not was taller than a stride, so you walked up onto it
    /// and the whole length of the fire was a path. A test of the boxes
    /// would have passed.
    /// Spec 0060: the flames run into each other, whatever size they
    /// come out.
    ///
    /// **Held at the worst pair and not at the average one.** Two
    /// neighbours both drawn at their narrowest are the only pair that can
    /// leave a gap along the trench, so that is the pair this asks about;
    /// anything wider closes faster. It was 0.68 apart against a narrowest
    /// foot of 0.32, which is the row of candles Jake saw.
    #[test]
    fn the_flames_run_into_each_other() {
        let narrowest = FLAME_WIDE * FLAME_SMALL;

        assert!(
            FLAME_APART < narrowest,
            "they stand {} apart with a foot of {}",
            FLAME_APART,
            narrowest
        );
        assert_eq!(
            FLAMES,
            ((FIRE_LONG - 1.0) / FLAME_APART) as usize,
            "the count and the spacing disagree about the same trench"
        );
    }

    #[test]
    fn you_cannot_walk_up_onto_the_fire() {
        let solid = everything();
        let trench = fire(REACHES);
        assert!(
            FIRE_KERB > crate::walk::rides(crate::RADIUS),
            "the kerb is {} against a body that rides {}",
            FIRE_KERB,
            crate::walk::rides(crate::RADIUS)
        );

        // from beside it, straight at it, from three places down its own
        // length. Off the trench and not off the hall's middle, which is
        // not the same point: the fire sits where `hall` puts it.
        for along in [0.2, 0.5, 0.8] {
            let x = trench.min.x + along * (trench.max.x - trench.min.x);
            // dropped in beside it at floor height, not over the kerb
            let start = vec3(x, trench.min.y + 0.4, trench.min.z - 1.6);
            let want = vec3(x, trench.min.y, trench.max.z + 1.6);
            let ended = crate::bridge::tests::walked(start, want, 6.0, &solid);

            assert!(
                ended.z < trench.min.z,
                "walked to {:?}, which is in or past the fire",
                ended
            );
            assert!(
                ended.y < trench.max.y,
                "ended {:.2} up, which is standing on the hearth",
                ended.y
            );
        }
    }

    #[test]
    fn walked_from_the_landing_into_valhalla() {
        let mut solid = crate::bridge::solid(REACHES, &done());
        solid.extend(super::solid(REACHES, &done()));

        let landing = crate::bridge::landing(REACHES, &done())[0];
        let middle = at(REACHES);
        let start = vec3(landing.max.x - 1.5, landing.max.y + 0.4, middle.z);
        // in along the lane, which is east of the hall
        let want = vec3(middle.x + 9.0, middle.y, middle.z);
        let ended = crate::bridge::tests::walked(start, want, 14.0, &solid);

        assert!(
            ended.x < landing.min.x - 1.0,
            "it got to {:.2} and the landing ends at {:.2}, so it never \
             crossed",
            ended.x,
            landing.min.x
        );
        assert!(
            (ended.y - middle.y).abs() < 0.6,
            "it ended {:.2} up and the ground is at {:.2}",
            ended.y,
            middle.y
        );
    }

    /// Spec 0060: everything built inside the hall is inside the hall.
    ///
    /// **Written because the names lie.** `Built::wide` is the hall's
    /// twenty-four and `Built::long` is its nine, which is the wrong way
    /// round from what they say, and the first build of this room was
    /// written to the names: a sixteen metre fire through both walls and
    /// benches across the door. Nothing failed. It drew, it was solid,
    /// and it was a hall with its furniture outside it.
    #[test]
    fn everything_inside_is_inside() {
        let air = inside(REACHES);
        let mut every = vec![fire(REACHES), floor(REACHES), high_seat(REACHES)];

        every.extend(posts(REACHES));
        every.extend(ties(REACHES));
        every.extend(benches(REACHES));

        for one in every {
            assert!(
                one.min.x >= air.min.x - 1e-3 && one.max.x <= air.max.x + 1e-3,
                "something runs from {:.2} to {:.2} across a hall that is \
                 {:.2} to {:.2}",
                one.min.x,
                one.max.x,
                air.min.x,
                air.max.x
            );
            assert!(
                one.min.z >= air.min.z - 1e-3 && one.max.z <= air.max.z + 1e-3,
                "something runs from {:.2} to {:.2} along a hall that is \
                 {:.2} to {:.2}",
                one.min.z,
                one.max.z,
                air.min.z,
                air.max.z
            );
        }

        for at in fires(REACHES) {
            assert!(
                air.contains_point(at),
                "a light of the fire is at {:?}, which is not in the hall",
                at
            );
        }
    }

    /// Spec 0060: nothing drawn inside the hall shares a plane with
    /// anything else drawn inside it.
    ///
    /// **Eight times in this building, and the first room anybody could
    /// stand in the middle of.** The floor is laid on the yard, the
    /// coals sit in a stone rim, the benches stand against the walls and
    /// the rafters hang under the pitches: four chances to put two
    /// exposed faces at one height, and the stipple only shows up from
    /// inside.
    #[test]
    fn nothing_inside_is_in_one_plane_with_anything() {
        use blitzkit::renderer::coplanar::Fights;

        let looking = blitzkit::renderer::depth::Depth::of(&blitzkit::camera::Camera::default());
        let air = inside(REACHES);
        let mut drawn = vec![floor(REACHES), coals(REACHES), ground(REACHES)];

        drawn.extend(hall_walls(REACHES));
        drawn.extend(fire_rails(REACHES));
        drawn.extend(posts(REACHES));
        drawn.extend(ties(REACHES));
        drawn.extend(benches(REACHES));
        drawn.push(high_seat(REACHES));

        // a little outside the clear air, because a fight against the
        // wall's own inner face has its patch in the wall
        let room = Aabb::new(air.min - Vec3::splat(0.6), air.max + Vec3::splat(0.6));

        for fight in Fights::of(drawn, 0, 0, &looking).fights.iter() {
            let at = fight.patch.center();

            assert!(
                !room.contains_point(at),
                "two drawn faces share a plane at {:?}, over {:.2} by {:.2}",
                at,
                fight.patch.size().x.max(fight.patch.size().y),
                fight.patch.size().z.max(fight.patch.size().y)
            );
        }
    }

    /// Spec 0060: you can get in through the door, and out again.
    #[test]
    fn the_hall_is_a_way_in_and_a_way_out() {
        let solid = everything();
        let one = hall(REACHES);
        let (door, _, _) = doorway(REACHES);
        let outside = vec3(door.x + 3.0, door.y + 0.4, door.z);
        let in_to = vec3(one.at.x + 6.0, one.at.y, door.z);
        let got = crate::bridge::tests::walked(outside, in_to, 10.0, &solid);

        assert!(
            got.x < door.x - 0.6,
            "it got to {:.2} and the door is at {:.2}, so it never went in",
            got.x,
            door.x
        );

        let back = crate::bridge::tests::walked(
            vec3(got.x, got.y + 0.3, got.z),
            vec3(door.x + 4.0, door.y, door.z),
            10.0,
            &solid,
        );

        assert!(
            back.x > door.x + 0.6,
            "it got back to {:.2} and the door is at {:.2}, so it is shut in",
            back.x,
            door.x
        );
    }

    /// Spec 0060: and the hall is hollow rather than a box with a hole in
    /// the front of it.
    #[test]
    fn the_hall_is_hollow() {
        let solid = everything();
        let air = inside(REACHES);
        let (door, _, _) = doorway(REACHES);
        let start = vec3(door.x - 1.2, door.y + 0.4, door.z);

        for side in [-1.0f32, 1.0] {
            let want = vec3(air.min.x + 2.0, door.y, door.z + side * 2.6);
            let got = crate::bridge::tests::walked(start, want, 14.0, &solid);

            assert!(
                got.x < air.max.x - 8.0,
                "from the door it got to {:.2} going to {:?}, and the hall \
                 runs back to {:.2}",
                got.x,
                want,
                air.min.x
            );
        }
    }

    /// Spec 0060: and nothing stands in the doorway.
    #[test]
    fn nothing_stands_in_the_doorway() {
        let (door, wide, high) = doorway(REACHES);
        let way = Aabb::from_center_size(
            vec3(door.x - 1.5, door.y + high * 0.5, door.z),
            vec3(3.0, high, wide),
        );

        for post in posts(REACHES) {
            assert!(
                !post.intersects(&way),
                "a post at {:?} stands in the way in",
                post.center()
            );
        }

        for bench in benches(REACHES).into_iter().chain([high_seat(REACHES)]) {
            assert!(
                !bench.intersects(&way),
                "something to sit on at {:?} stands in the way in",
                bench.center()
            );
        }
    }

    /// Spec 0060: the fire is the only light in there.
    #[test]
    fn the_fire_is_the_only_light_in_here() {
        let air = inside(REACHES);
        let kerb = fire(REACHES);

        for at in lamps(REACHES) {
            assert!(
                !air.contains_point(at),
                "a lamp of the steading is at {:?}, which is inside the hall",
                at
            );
        }

        for at in fires(REACHES) {
            assert!(
                at.x > kerb.min.x && at.x < kerb.max.x,
                "a light of the fire is at {:.2} and the trench runs {:.2} \
                 to {:.2}",
                at.x,
                kerb.min.x,
                kerb.max.x
            );
        }

        assert_eq!(fires(REACHES).len(), FIRES);
    }

    /// Spec 0060: the night is the whole ceiling.
    ///
    /// **Not a rectangle in it.** It was a louvre in the ridge with a
    /// painted night hung under the hole, and a small lit rectangle in a
    /// dark ceiling reads as a panel with something on it however it is
    /// painted. Jake, 2026-11-11: a small rectangle doesn't land, just
    /// make it the whole ceiling.
    ///
    /// Asked of the two quads against the room they are the ceiling of,
    /// rather than of either on its own.
    #[test]
    fn the_night_is_the_whole_ceiling() {
        let one = hall(REACHES);
        let pitches = night_pitches(REACHES);
        let air = inside(REACHES);

        assert_eq!(pitches.len(), 2, "a roof has two slopes");

        for (at, size, roll) in pitches.iter().copied() {
            assert!(
                (size.x - slope_run(&one)).abs() < 1e-3,
                "a slope is {:.2} long and the quad on it is {:.2}",
                slope_run(&one),
                size.x
            );
            assert!(
                size.z >= air.max.z - air.min.z,
                "the quad is {:.2} along the hall and the room is {:.2}",
                size.z,
                air.max.z - air.min.z
            );
            assert!(
                roll.abs() > 0.05,
                "the quad lies flat, so it is a lid and not a pitch"
            );
            assert!(
                at.y < one.at.y + one.ridge && at.y > one.at.y + one.eaves,
                "the night hangs at {:.2}, and the roof runs {:.2} to {:.2}",
                at.y,
                one.at.y + one.eaves,
                one.at.y + one.ridge
            );
        }

        // and the two of them lie either side of the ridge
        assert!(
            pitches[0].0.x < one.at.x && pitches[1].0.x > one.at.x,
            "both slopes are on the same side of the ridge"
        );
    }

    /// Spec 0060: and the roof over it is whole.
    ///
    /// **Because the night is drawn and not opened.** From outside this
    /// has to stay a roof: the building reads from the top of the bridge
    /// by its roofline and one lantern, and a hole in the ridge with a
    /// night under it was a hole in that too.
    #[test]
    fn the_roof_is_one_piece_down_each_slope() {
        let one = hall(REACHES);
        let pieces = pitch_pieces(&one);

        assert_eq!(pieces.len(), 1, "a pitch is one piece");
        assert!(
            pieces[0].0 == 0.0 && pieces[0].1 == 1.0,
            "the pitch runs {:.2} to {:.2} rather than eaves to ridge",
            pieces[0].0,
            pieces[0].1
        );
    }

    /// Spec 0060: the painted night is dark, and the stars are the only
    /// thing in it that burns.
    ///
    /// **Which is the whole of what it got wrong.** The sky ran to a cobalt
    /// that clipped in blue once `NIGHT_BURNS` had multiplied it, so the
    /// hole in the ridge was a lit rectangle in a dark ceiling with a
    /// pattern moving on it. Jake, 2026-11-11: is it supposed to be the
    /// northern lights, it looks like a rectangle of swirling colour.
    ///
    /// Asked of the picture and not of the two colours it is mixed from,
    /// because what reads is what comes out: the swirl and the stars are
    /// laid over that mix and either of them could put it back.
    #[test]
    fn the_night_is_dark_and_only_its_stars_burn() {
        let sky = night_sky();
        let level = sky.levels.first().expect("the full size picture");
        let mut lit: Vec<f32> = Vec::new();

        for texel in level.pixels.chunks_exact(4) {
            let (r, g, b) = (texel[0] as f32, texel[1] as f32, texel[2] as f32);

            lit.push((r * 0.2126 + g * 0.7152 + b * 0.0722) / 255.0);
        }

        lit.sort_by(f32::total_cmp);

        let middle = lit[lit.len() / 2];
        let burning = lit.iter().filter(|one| **one > 0.5).count() as f32 / lit.len() as f32;

        assert!(
            middle < 0.1,
            "the middle of the sky is {:.3}, which is a lit panel",
            middle
        );
        assert!(
            burning > 0.0005,
            "{:.4} of it burns, so there is nothing to look at",
            burning
        );
        assert!(
            burning < 0.05,
            "{:.4} of it burns, which is not a night",
            burning
        );
    }

    /// Spec 0056: inside the ground the field draws, and so inside the sky.
    #[test]
    fn it_stays_inside_the_field() {
        let ours = ground(REACHES);
        let west = crate::field::starts(REACHES) - crate::field::GROUND_REACH;

        assert!(
            ours.min.x > west,
            "it runs out to {:.2} and the field's ground stops at {:.2}",
            ours.min.x,
            west
        );
    }

    /// Spec 0056: what you see on arriving is the door and not a flank.
    #[test]
    fn the_hall_faces_the_way_you_come_in() {
        let hall = built(REACHES)[0];
        let (door, _, _) = doorway(REACHES);

        assert!(hall.hall, "the first building is not the hall");
        assert!(
            door.x > hall.at.x,
            "the door at {:.2} is on the far side of a hall at {:.2}",
            door.x,
            hall.at.x
        );
        // and it is on the line you walk in on
        assert!(
            (door.z - at(REACHES).z).abs() < 1.0,
            "the door is {:.2} off the way in",
            (door.z - at(REACHES).z).abs()
        );
    }

    /// Spec 0056: nothing stands in the lane you arrive along.
    #[test]
    fn nothing_stands_in_the_way_in() {
        let middle = at(REACHES);
        let (door, _, _) = doorway(REACHES);
        let body = crate::RADIUS * 2.0;

        for one in built(REACHES) {
            if one.hall {
                continue;
            }

            let box_ = one.footprint();
            // the lane runs from the landing west to the hall's door, on
            // the middle line
            let across = box_.min.z - body < middle.z && box_.max.z + body > middle.z;
            let along = box_.max.x > door.x && box_.min.x < middle.x + DEEP * 0.5;

            assert!(
                !(across && along),
                "a building stands in the lane: {:?} to {:?}",
                box_.min,
                box_.max
            );
        }

        for (where_, _) in people(REACHES) {
            let off = (where_.z - middle.z).abs();

            assert!(
                off > crate::RADIUS || where_.x < door.x,
                "somebody stands in the doorway at {:?}",
                where_
            );
        }
    }

    /// Spec 0056: and no two of them are in the same place.
    #[test]
    fn no_two_buildings_share_a_space() {
        let all = built(REACHES);

        for (n, one) in all.iter().enumerate() {
            for other in all[n + 1..].iter() {
                let (a, b) = (one.footprint(), other.footprint());
                let apart = a.min.x > b.max.x
                    || b.min.x > a.max.x
                    || a.min.z > b.max.z
                    || b.min.z > a.max.z;

                assert!(apart, "two buildings overlap: {:?} and {:?}", a.min, b.min);
            }
        }
    }

    /// Spec 0056: the light is in the openings, and an opening is a hole
    /// in a wall rather than a panel on one.
    #[test]
    fn every_light_is_in_an_opening() {
        let all = built(REACHES);

        for one in lit(REACHES) {
            let found = all.iter().any(|built| {
                let walls = built.walls();

                // behind the face it is in, by the set-back and no more.
                // On the face it is a panel painted on a wall, and this
                // test said on the face until the day it stopped being
                // true.
                one.at.x < walls.max.x + 1e-3
                    && one.at.x > walls.max.x - SET_BACK - 0.1
                    && one.at.z > walls.min.z
                    && one.at.z < walls.max.z
                    && one.at.y + one.high * 0.5 < built.at.y + built.eaves
            });

            assert!(found, "a light hangs in the air at {:?}", one.at);
        }
    }

    /// Spec 0056: and everybody is standing on the ground, outdoors.
    #[test]
    fn everybody_stands_on_the_ground() {
        let floor = ground(REACHES);
        let all = built(REACHES);

        for (where_, _) in people(REACHES) {
            assert!(
                (where_.y - floor.max.y).abs() < 1e-3,
                "somebody stands at {:.3} and the ground is at {:.3}",
                where_.y,
                floor.max.y
            );
            assert!(
                where_.x > floor.min.x && where_.x < floor.max.x,
                "somebody stands off the edge at {:?}",
                where_
            );

            for one in all.iter() {
                let walls = one.walls();
                let inside = where_.x > walls.min.x - crate::RADIUS
                    && where_.x < walls.max.x + crate::RADIUS
                    && where_.z > walls.min.z - crate::RADIUS
                    && where_.z < walls.max.z + crate::RADIUS;

                assert!(!inside, "somebody stands in a wall at {:?}", where_);
            }
        }
    }
}
