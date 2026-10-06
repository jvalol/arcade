//! The stair behind the bookcase, and the room at the bottom of it. Spec 0007.
//!
//! Behind the nook's back wall rather than down through its floor. A secret
//! stair is a hole in a wall, not a hole in a carpet, and it costs less: the
//! nook's floor is one quad and a quad has no hole in it, so a stairwell inside
//! the room would have meant laying the boards in pieces round an opening.
//! Beyond the wall there is nothing to cut.
//!
//! Nothing here draws or reads a key, so where every tread sits and whether you
//! can get down them is arithmetic, checked without a window.

use crate::room::{self, CABINET, NOOK_DEEP, THICK, WALL};
use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far down the cellar is from the nook.
pub const DOWN: f32 = 3.0;

/// The stair: how many treads, how tall each is, and how deep.
///
/// The riser is well inside what [`crate::walk::STEP`] can get you up, because
/// a stair you can only just climb is a stair you catch your feet on. Twelve of
/// them at a quarter each is the whole drop.
///
/// The tread is 0.3, which is what a stair looks like, and for a while it could
/// not be. It had to be 0.7, deeper than the body is from its middle to its
/// toe, because the step up was asked as a question about the body: raise a
/// sphere 0.9 across by one riser and it meets the next riser but one, with
/// nowhere to put its weight down. A stair you could walk down and not back up.
///
/// What is in front of your foot is a question about the floor. Asked straight
/// down instead of by moving a body at it, the tread goes back to being a
/// number about stairs.
pub const STEPS: usize = 12;
pub const RISER: f32 = DOWN / STEPS as f32;
pub const TREAD: f32 = 0.3;

/// How many bookcases the door is, and so how wide and how tall the opening is.
///
/// Two. One was 1.1, and you are 0.9 across: a tenth of a unit of daylight
/// either side, held for the nine units it takes to get down. Walked exactly
/// down the middle it fits, which is what the test did and why the test was
/// worth nothing. Nobody walks exactly down the middle of anything.
///
/// Two leaves rather than one wide one, each hinged at its own outer end, so
/// what comes out into the nook when it opens is a case on either side of the
/// opening rather than 2.2 of shelving across the floor.
pub const CASES: usize = 2;
pub const WIDE: f32 = crate::study::CASE.x * CASES as f32;
pub const HIGH: f32 = crate::study::CASE.z;

/// How much room there is over your head on the way down.
///
/// You are 1.55 at the eye, so this is comfortably over it, and the soffit goes
/// down with the stair rather than sitting flat: a flat ceiling at the cellar's
/// own height is a lid across the top of the flight.
pub const HEADROOM: f32 = 2.1;

/// How wide the passage behind it is.
///
/// Wider than you by half again. A corridor with a hand's breadth either side
/// is a corridor you scrape along, and this one is the length of a staircase.
pub const PASSAGE: f32 = 1.8;

/// How far the shaft reaches past the back wall, which is the run of the stair
/// and a landing at each end of it.
pub const LANDING: f32 = 1.0;
pub const SHAFT: f32 = LANDING * 2.0 + STEPS as f32 * TREAD;

/// The cellar itself: how far it reaches past the foot of the stair, how wide,
/// and how high its ceiling is.
///
/// Lower than the nook's 3.2. A cellar with the headroom of a hall is a
/// basement, and the difference between the two is entirely the ceiling.
pub const DEEP: f32 = 10.0;
pub const SPAN: f32 = 9.0;
pub const TALL: f32 = 2.7;

/// The first of the bookcases along the back wall that swing. [`CASES`] of them
/// from here.
///
/// Not the middle. A door in the middle of a wall is a door, and the one thing
/// a secret door must not be is the obvious candidate.
pub const CASE: usize = 2;

/// A wine rack: how wide, how tall, how deep, and how many bottles across and
/// up it holds.
pub const RACK: Vec3 = glam::vec3(0.34, 1.75, 1.5);
pub const BOTTLES: (usize, usize) = (7, 8);
pub const BOTTLE: f32 = 0.085;
pub const BOTTLE_LONG: f32 = 0.3;

/// A barrel: how fat, how tall, and how much it bulges at the waist.
pub const BARREL: Vec3 = glam::vec3(0.95, 1.36, 0.95);
pub const BULGE: f32 = 0.16;

/// How far off a wall a rack or a barrel stands.
pub const AGAINST: f32 = 0.08;

/// The one book that is not a book: which leaf it stands in, which shelf, and
/// which along that shelf.
///
/// The copy said one of them is not a book and the room let you pull any of
/// them: clicking anywhere on either case worked, which makes the sentence a
/// lie and the door a pair of very large buttons. A handle you can find by
/// waving at a wall is not a handle.
///
/// Low enough to reach for and not the end of the row. Four hundred books and
/// one of them opens the wall, and nothing marks it, which is the whole idea.
pub const BOOK_SHELF: usize = 2;
pub const BOOK: usize = 6;

/// Whether a bookcase is one of the leaves.
pub fn swings(n: usize) -> bool {
    (CASE..CASE + CASES).contains(&n)
}

/// The face of the nook's back wall, which is where the opening is.
pub fn back() -> f32 {
    -(WALL + CABINET.x) - NOOK_DEEP + THICK * 0.5
}

/// Where the opening is along the wall, taken from the bookcase that swings.
pub fn opening(reaches: f32) -> f32 {
    let cases = room::bookcases(-reaches);
    let span: Vec<f32> = (CASE..CASE + CASES)
        .filter_map(|n| cases.get(n).map(|shelved| shelved.at.z))
        .collect();

    if span.is_empty() {
        return -reaches + 1.0;
    }

    span.iter().sum::<f32>() / span.len() as f32
}

/// What stands in the cellar, which is the reason there is one.
///
/// poolhall is a game and there is something to win, so by the hall's own rule
/// it belongs in a cabinet out there. A cabinet with pool on the screen is the
/// wrong object: pool is a table, and the thing you want is to walk up to the
/// table. cascada showed the shape of it. A bench that opens a window is a toy
/// too big for a bench, and a table that opens a window is a game too big for a
/// cabinet.
pub const POOLHALL: &str = "poolhall";

/// How big it is, and how far off the floor the slate sits.
///
/// Nine foot by four and a half, which is what a full size table is, at the
/// scale the rest of this building is built to.
pub const TABLE: Vec3 = glam::vec3(2.7, 0.78, 1.35);

/// The lamp over the table: how high it hangs over the cloth, and what it
/// throws.
///
/// A table has a light over it, and a cellar lit by nothing was a room of flat
/// grey surfaces with no shape to any of them. One fitting answers both: the
/// thing a pool room is lit by is the thing hanging over the table.
pub const LAMP_UP: f32 = 0.95;
pub const SHADE: Vec3 = glam::vec3(0.5, 0.1, 0.3);
pub const LAMP_LIT: f32 = 0.8;
pub const LAMP_RANGE: f32 = 6.0;
pub const LAMP_COLOUR: Vec3 = glam::vec3(1.0, 0.93, 0.78);

/// Where it stands: the middle of the cellar floor, turned so you come down the
/// stair at its end rather than its side.
pub fn table(reaches: f32) -> crate::room::Benched {
    let foot = back() - LANDING - STEPS as f32 * TREAD;

    crate::room::Benched {
        name: POOLHALL,
        at: vec3(foot - DEEP * 0.5, -DOWN, opening(reaches)),
        size: vec3(TABLE.z, TABLE.y, TABLE.x),
        // you come down the stair facing the room, so the side you work it from
        // is the side the stair is
        worked_from: Vec3::X,
    }
}

/// A barrel, as a surface turned about its own axis.
///
/// The engine will build a surface from a function of two numbers, so a barrel
/// is the one it is: round the way round, and fatter at the waist than at the
/// ends. Stacked out of boxes it would be a bin.
pub fn barrel_mesh() -> blitzkit::mesh::MeshData {
    turned(
        STAVES,
        20,
        lidded(|t| {
            let along = t - 0.5;
            // the belly, with a short straight run at each end where the chime
            // is. A cask that curves all the way to its rim is one drawn by
            // somebody who has only read about them.
            let straight = 0.06;
            let waist = if t < straight || t > 1.0 - straight {
                1.0 - BULGE
            } else {
                1.0 - BULGE * (along * 2.0) * (along * 2.0)
            };

            (along, waist)
        }),
    )
}

/// A flame: fat at the bottom, pinched in, and drawn to a point.
///
/// Its own shape because a box is not one. A fire made of slabs is a pile of
/// hot bricks, and the one thing everybody knows the look of without being
/// told is a flame.
pub fn flame_mesh() -> blitzkit::mesh::MeshData {
    turned(10, 12, |v| {
        let up = v;
        // fat at the base, drawn in at the waist, and tapering off the top
        let fat = (1.0 - up).powf(0.55) * (1.0 - 0.45 * (up * 2.6).sin().abs() * up);

        (up - 0.5, fat.max(0.0))
    })
}

/// A plain closed cylinder: a barrel's lid, a clock's dial, a candle, a coal.
///
/// Six steps along it and not one. [`lidded`] spends the first and last tenth
/// of the run closing the ends, so at one step the only two samples it ever
/// takes are both on a lid, both at no radius at all, and the whole mesh is two
/// points. It drew nothing, and everything made of it drew nothing: the dial
/// off the front of the clock, the candles under their own flames, and the bed
/// of coals under the fire.
pub fn peg_mesh() -> blitzkit::mesh::MeshData {
    turned(STAVES, 6, lidded(|t| (t - 0.5, 1.0)))
}

/// A profile with a lid on each end of it.
///
/// A turn with no lids is a tube, and a tube is only ever half there: the near
/// side draws, the far side is wound away from you and culled, so what you see
/// is a shell whose open edge swings round to follow you as you walk past. From
/// straight on it passes for a barrel, which is why it took walking past one.
///
/// The first and last tenth of the run go flat across each end, from the axis
/// out to wherever the side starts, so the thing is closed.
fn lidded(side: impl Fn(f32) -> (f32, f32)) -> impl Fn(f32) -> (f32, f32) {
    const LID: f32 = 0.1;

    move |v| {
        if v < LID {
            let (along, waist) = side(0.0);

            (along, v / LID * waist)
        } else if v > 1.0 - LID {
            let (along, waist) = side(1.0);

            (along, (1.0 - v) / LID * waist)
        } else {
            side((v - LID) / (1.0 - LID * 2.0))
        }
    }
}

/// A bottle's end, which is a short closed cylinder seen down its own axis.
pub fn bottle_mesh() -> blitzkit::mesh::MeshData {
    turned(
        14,
        22,
        lidded(|t| {
            let waist = if t < SHOULDER {
                1.0
            } else if t < SHOULDER + SLOPE {
                // the shoulder, which eases in rather than steps
                let eased = (t - SHOULDER) / SLOPE;

                1.0 - eased * eased * (1.0 - NECK)
            } else if t > 1.0 - LIP {
                NECK * 1.22
            } else {
                NECK
            };

            (t - 0.5, waist)
        }),
    )
}

/// Which glass a given bottle is, out of the few a cellar holds.
///
/// Seeded off where it stands, so a rack is mixed and stays mixed: a shelf
/// restocked every frame is a shelf that boils, which the nook's books already
/// taught this building once.
pub fn glass(rack: usize, row: usize, n: usize) -> usize {
    let mut roll = (rack as u32)
        .wrapping_mul(2_654_435_761)
        .wrapping_add(row as u32 * 40_503)
        .wrapping_add(n as u32 * 2_246_822_519)
        | 1;
    roll ^= roll << 13;
    roll ^= roll >> 17;
    roll ^= roll << 5;

    roll as usize
}

/// How much of a bottle's length is cork, at the neck end.
pub const CORK: f32 = 0.055;

/// What a bottle is shaped like: where the shoulder starts, how long it takes
/// to come in, how thin the neck is, and how much of the end is the lip.
pub const SHOULDER: f32 = 0.52;
pub const SLOPE: f32 = 0.16;
pub const NECK: f32 = 0.3;
pub const LIP: f32 = 0.05;

/// How many staves round a barrel, which is how round everything turned down
/// here is.
pub const STAVES: u32 = 32;

/// A surface of revolution, skinned so that both its winding and its normals
/// face out of it.
///
/// Which way round you take the angle decides the winding, and the normals the
/// surface works out for itself are the cross of its two tangents, which that
/// same choice also decides. They are not the same thing and they went wrong
/// one at a time. Taken the obvious way round the faces draw and the normals
/// point into the barrel, so it lights as a pale lump. Taken the other way the
/// normals come out right and every triangle is wound away from you, so the
/// whole thing is culled and a barrel is a stray green fleck where one hoop
/// face survived.
///
/// So: the way round that winds them towards you, and the normals worked out
/// from that winding afterwards rather than from the parameters.
fn turned(
    round_steps: u32,
    steps: u32,
    profile: impl Fn(f32) -> (f32, f32),
) -> blitzkit::mesh::MeshData {
    let mut mesh = blitzkit::mesh::MeshData::surface(round_steps, steps, |u, v| {
        let round = -u * std::f32::consts::TAU;
        let (along, waist) = profile(v);

        vec3(round.cos() * waist * 0.5, along, round.sin() * waist * 0.5)
    });
    mesh.compute_normals();

    mesh
}

/// How fat a barrel is at a given height, as a fraction of its widest.
pub fn waist_at(up: f32) -> f32 {
    let along = (up - 0.5) * 2.0;

    1.0 - BULGE * along * along
}

/// A hoop: the same turn with no bulge in it, which is a plain band.
///
/// Its own mesh rather than a cube. A cube round a round barrel meets it at the
/// middle of each face and sticks out at all four corners, so what you see is
/// not a hoop, it is a square collar, and two of them turn a barrel into a box.
/// That is what the barrels down here looked like, and the barrel underneath
/// was right the whole time.
pub fn hoop_mesh() -> blitzkit::mesh::MeshData {
    turned(STAVES, 1, |v| (v - 0.5, 1.0))
}

/// Where the hoops sit up a barrel, and how proud of it they stand.
pub const HOOPS: [f32; 4] = [0.1, 0.33, 0.67, 0.9];
pub const HOOP_OUT: f32 = 1.04;

/// The lid sunk inside the rim at each end, and how far down it sits.
pub const LID: f32 = 0.9;
pub const LID_DOWN: f32 = 0.035;
pub const HOOP_THICK: f32 = 0.05;

/// Where the wine racks stand: along both long walls, facing in.
///
/// Not along the end the stair comes through, which is where you arrive and
/// where you want to be able to see the room.
pub fn racks(reaches: f32) -> Vec<(Vec3, f32)> {
    let foot = back() - LANDING - STEPS as f32 * TREAD;
    let middle = foot - DEEP * 0.5;
    let run = SPAN - 1.6;
    let fits = (run / (RACK.z + 0.25)) as usize;
    let along = fits as f32 * (RACK.z + 0.25) - 0.25;
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        for n in 0..fits {
            // skipping the stretch the table stands against, so the racks are a
            // wall of bottles and not a shelf behind a pool cue
            let at = middle + (n as f32 + 0.5) * (RACK.z + 0.25) - along * 0.5;
            if (at - middle).abs() < TABLE.x * 0.5 + 0.9 {
                continue;
            }

            out.push((
                vec3(
                    at,
                    -DOWN,
                    opening(reaches) + side * (SPAN * 0.5 - THICK * 0.5 - AGAINST - RACK.x * 0.5),
                ),
                side,
            ));
        }
    }

    out
}

/// Where the barrels stand: in the far corners, on end, in twos and threes.
///
/// On end rather than on their sides. A barrel on its side wants a cradle and a
/// cradle is three more boxes nobody will look at; a barrel on its end is what
/// a cellar with no room left looks like anyway.
pub fn barrels(reaches: f32) -> Vec<Vec3> {
    let foot = back() - LANDING - STEPS as f32 * TREAD;
    let far = foot - DEEP + THICK * 0.5 + AGAINST + BARREL.x * 0.5;
    let along = opening(reaches);
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        for n in 0..3 {
            out.push(vec3(
                far + (n % 2) as f32 * BARREL.x * 0.82,
                -DOWN,
                along + side * (SPAN * 0.5 - 0.9 - n as f32 * BARREL.z * 0.92),
            ));
        }
    }

    out
}

/// The fireplace in the far wall: how wide the opening is, how high, how deep
/// it is cut in, and how far the surround stands past it.
pub const FIRE_WIDE: f32 = 1.5;
pub const FIRE_HIGH: f32 = 1.15;
pub const FIRE_DEEP: f32 = 0.55;
pub const FIRE_ROUND: f32 = 0.28;
pub const MANTEL: f32 = 0.16;

/// The top of the mantel, which is what anything standing on it stands on.
///
/// Worked out here rather than in the drawing, so the shelf and the things on
/// it cannot disagree about where it is. Two lists of where a surface is has
/// already hung a sconce in a doorway and put a grey patch over four hundred
/// books in this building.
pub fn mantel_top(reaches: f32) -> Vec3 {
    let at = hearth(reaches);

    vec3(
        at.x + (THICK + MANTEL) * 0.5,
        at.y + FIRE_HIGH + FIRE_ROUND + MANTEL * 0.6,
        at.z,
    )
}

/// A candlestick: a foot, a stem with a knop in it, and a cup at the top.
pub fn candlestick_mesh() -> blitzkit::mesh::MeshData {
    turned(
        14,
        26,
        lidded(|t| {
            let waist = if t < 0.06 {
                1.0
            } else if t < 0.1 {
                0.9 - (t - 0.06) / 0.04 * 0.6
            } else if t < 0.46 {
                0.3
            } else if t < 0.56 {
                // the knop, which is the swelling halfway up a stem and the one
                // thing that says turned rather than cut
                0.3 + ((t - 0.46) / 0.1 * std::f32::consts::PI).sin() * 0.26
            } else if t < 0.88 {
                0.28
            } else {
                0.3 + (t - 0.88) / 0.12 * 0.5
            };

            (t - 0.5, waist)
        }),
    )
}

/// A decanter: a broad body, a shoulder, a narrow neck and a flared lip.
pub fn decanter_mesh() -> blitzkit::mesh::MeshData {
    turned(
        16,
        24,
        lidded(|t| {
            let waist = if t < 0.42 {
                (0.55 + t * 1.1).min(1.0)
            } else if t < 0.62 {
                let eased = (t - 0.42) / 0.2;

                1.0 - eased * eased * 0.72
            } else if t < 0.9 {
                0.28
            } else {
                0.28 + (t - 0.9) / 0.1 * 0.22
            };

            (t - 0.5, waist)
        }),
    )
}

/// A glass: a foot, a stem and a bowl.
pub fn glass_mesh() -> blitzkit::mesh::MeshData {
    turned(
        12,
        20,
        lidded(|t| {
            let waist = if t < 0.07 {
                1.0
            } else if t < 0.12 {
                0.95 - (t - 0.07) / 0.05 * 0.78
            } else if t < 0.42 {
                0.17
            } else {
                0.17 + ((t - 0.42) / 0.58).powf(0.6) * 0.78
            };

            (t - 0.5, waist)
        }),
    )
}

/// How big the things on the mantel are, and where along it they stand.
pub const CANDLESTICK: Vec3 = glam::vec3(0.12, 0.34, 0.12);
/// The candle in it, and the flame on that.
///
/// Short and fat enough to see. A taper the width of a pencil is a line a few
/// pixels wide at the far side of a room, so the flame looked like it was
/// floating a hand's width over an empty stick.
pub const CANDLE: f32 = 0.15;
pub const CANDLE_FAT: f32 = 0.052;
pub const CANDLE_FLAME: f32 = 0.075;
pub const CLOCK: Vec3 = glam::vec3(0.13, 0.34, 0.27);

/// The dial on the front of it: how much of the case it takes, how proud it
/// stands, and how thick the bezel round it is.
///
/// Big. A clock reads as a clock because of the dial and nothing else, and a
/// small pale disc on a brown box reads as a brown box.
pub const DIAL: f32 = 0.78;
pub const DIAL_OUT: f32 = 0.012;
pub const BEZEL: f32 = 1.16;
pub const DECANTER: Vec3 = glam::vec3(0.17, 0.27, 0.17);
pub const GLASS: Vec3 = glam::vec3(0.09, 0.14, 0.09);

/// What the fire throws: its colour, how bright, and how far.
///
/// The main light in the room now, with the table's lamp over the cloth and the
/// red ones low on the walls behind it. A cellar lit by a fire is the warmest
/// room in the building, which is the whole of what this one is for.
pub const FIRE_LIT: f32 = 1.5;
pub const FIRE_RANGE: f32 = 9.0;
pub const FIRE_COLOUR: Vec3 = glam::vec3(1.0, 0.56, 0.24);

/// How much a fire wanders, and how fast.
///
/// Two rates that do not divide into each other, so it never repeats on any
/// beat you could tap along to. One sine is a pulse and a pulse is a warning
/// light.
pub const FLICKER: f32 = 0.17;
pub fn flicker(since: f32) -> f32 {
    let wander = (since * 11.3).sin() * 0.62 + (since * 4.7).sin() * 0.38;

    1.0 - FLICKER + FLICKER * wander
}

/// Where the fire is: the middle of the far wall, at the floor.
pub fn hearth(reaches: f32) -> Vec3 {
    let foot = back() - LANDING - STEPS as f32 * TREAD;

    vec3(foot - DEEP + THICK * 0.5, -DOWN, opening(reaches))
}

/// The panelling: how wide a board is, the gap between two, and how high the
/// dado and its rail sit.
///
/// Boards rather than a picture of boards. A wall is one box, and a cube's
/// corners run nought to one however big it is, so a timber texture on one
/// would be a single plank the size of the wall.
pub const BOARD: f32 = 0.42;
pub const BOARD_GAP: f32 = 0.02;
pub const BOARD_OUT: f32 = 0.05;

/// Floor to ceiling, not to a dado rail.
///
/// Panelling that stops at waist height and leaves plain wall above it is what
/// a dining room has. A room panelled the whole way is a room somebody spent
/// money on, and that is the point of this one.
pub const SKIRTING: f32 = 0.2;
pub const CORNICE: f32 = 0.18;

/// The stile either side of each panel and the rail above and below it, which
/// is what turns a flat board into a raised panel.
pub const STILE: f32 = 0.075;
pub const PANEL_IN: f32 = 0.022;

/// Where the panelling goes: each wall as its face, which way it stands out,
/// and where it starts and ends along itself.
pub fn panelled(reaches: f32) -> Vec<(f32, f32, f32, f32, bool, f32)> {
    let foot = back() - LANDING - STEPS as f32 * TREAD;
    let along = opening(reaches);
    let (near, far) = (
        along - SPAN * 0.5 + THICK * 0.5,
        along + SPAN * 0.5 - THICK * 0.5,
    );

    // and the near end in two pieces either side of the way in, which was
    // missing: the wall you face coming down the stair, and so the one wall of
    // the room anybody arriving actually looks at.
    let gap = PASSAGE * 0.5 + THICK;

    vec![
        // the two long walls, which run along z
        (
            near,
            1.0,
            foot - DEEP + THICK * 0.5,
            foot - THICK * 0.5,
            false,
            0.0,
        ),
        (
            far,
            -1.0,
            foot - DEEP + THICK * 0.5,
            foot - THICK * 0.5,
            false,
            0.0,
        ),
        // the far end round the fireplace. In one piece the panelling ran
        // behind the fire, so through the opening you saw mahogany boards with
        // flames in front of them. In two pieces it left the chimney breast
        // bare, which is worse: the wall over a fireplace is the one piece of
        // wall in a room that everybody looks at.
        //
        // So: either side of the hearth the whole way up, and over it from the
        // mantel to the ceiling.
        (
            foot - DEEP + THICK * 0.5,
            1.0,
            near,
            along - FIRE_WIDE * 0.5 - FIRE_ROUND,
            true,
            0.0,
        ),
        (
            foot - DEEP + THICK * 0.5,
            1.0,
            along + FIRE_WIDE * 0.5 + FIRE_ROUND,
            far,
            true,
            0.0,
        ),
        (
            foot - DEEP + THICK * 0.5,
            1.0,
            along - FIRE_WIDE * 0.5 - FIRE_ROUND,
            along + FIRE_WIDE * 0.5 + FIRE_ROUND,
            true,
            FIRE_HIGH + FIRE_ROUND + MANTEL,
        ),
        // and the near one, round the stair
        (foot - THICK * 0.5, -1.0, near, along - gap, true, 0.0),
        (foot - THICK * 0.5, -1.0, along + gap, far, true, 0.0),
    ]
}

/// The floor of the room, as the middle and size of a quad lying on it.
///
/// Its own quad rather than the top of the stone box it sits on, because a
/// cube's corners run nought to one however big it is, so boards laid on one
/// are a single plank ten units long. A quad can be given a tile count.
pub const PLANK: f32 = 1.6;
pub fn floor(reaches: f32) -> (Vec3, Vec3) {
    let foot = back() - LANDING - STEPS as f32 * TREAD;

    (
        vec3(foot - DEEP * 0.5, -DOWN, opening(reaches)),
        vec3(DEEP - THICK, 0.0, SPAN - THICK),
    )
}

/// Where the rug lies: in front of the fire, which is where a rug goes.
pub fn rug(reaches: f32) -> (Vec3, Vec3) {
    let at = hearth(reaches);
    let long = 3.4;

    (
        vec3(at.x + 0.6 + long * 0.5, -DOWN, at.z),
        vec3(long, 0.0, 2.3),
    )
}

/// Where the cellar's own lamps hang, which are not the table's.
///
/// Low on the walls and few. A cellar lit like a hall is a basement with the
/// lights on; what makes a room downstairs worth sitting in is one bright thing
/// to stand round and everything else going dark at the edges.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    let foot = back() - LANDING - STEPS as f32 * TREAD;
    let along = opening(reaches);

    vec![
        vec3(
            foot - DEEP * 0.25,
            -DOWN + GLOW_UP,
            along - SPAN * 0.5 + THICK,
        ),
        vec3(
            foot - DEEP * 0.25,
            -DOWN + GLOW_UP,
            along + SPAN * 0.5 - THICK,
        ),
        vec3(
            foot - DEEP * 0.75,
            -DOWN + GLOW_UP,
            along - SPAN * 0.5 + THICK,
        ),
        vec3(
            foot - DEEP * 0.75,
            -DOWN + GLOW_UP,
            along + SPAN * 0.5 - THICK,
        ),
        vec3(foot - DEEP + THICK, -DOWN + GLOW_UP, along),
    ]
}

/// What those lamps are: how high, how far they reach, and their colour.
///
/// Red, and faint. Not a red room: the table's own lamp is warm and white and
/// is what you see by, and these are what the walls and the bottles are. A
/// cellar lit red throughout is a darkroom.
pub const GLOW_UP: f32 = 1.7;
pub const GLOW_LIT: f32 = 0.42;
pub const GLOW_RANGE: f32 = 5.5;
pub const GLOW: Vec3 = glam::vec3(1.0, 0.33, 0.26);

/// Every box the stair and the cellar are made of.
///
/// The shaft runs out in -x from the opening, so the stair goes away from the
/// nook rather than along it. Each tread is a slab reaching down to the cellar
/// floor rather than a step of its own thickness, because what is under a stair
/// is not anything anybody sees and a solid run is one box fewer to fall
/// between.
/// What a box down here is made of, which is the only reason the stair and the
/// cellar are told apart at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Made {
    Tread,
    Stone,
}

/// Every box the way down is built of, and what each one is.
///
/// The shell: the stair, the shaft and the room. Not the furniture, which has
/// its own shapes and is drawn as those. Taken off `solid` it drew the racks
/// and the barrels as the stone cubes that stop you walking into them, each one
/// standing exactly where the thing itself stands and completely hiding it. The
/// barrels were in there the whole time, inside a box.
pub fn built(reaches: f32) -> Vec<(Aabb, Made)> {
    shell(reaches)
        .into_iter()
        .enumerate()
        .map(|(n, box_)| {
            // the treads come first, then the landing, which is boards too
            let made = if n <= STEPS { Made::Tread } else { Made::Stone };

            (box_, made)
        })
        .collect()
}

pub fn solid(reaches: f32) -> Vec<Aabb> {
    let mut out = shell(reaches);

    // the racks and the barrels, which are things you walk into rather than
    // through. A cellar you can stand inside the furniture of is a cellar with
    // pictures of furniture in it.
    for (at, _) in racks(reaches) {
        out.push(Aabb::from_center_size(
            at + Vec3::Y * RACK.y * 0.5,
            vec3(RACK.z, RACK.y, RACK.x),
        ));
    }
    for at in barrels(reaches) {
        out.push(Aabb::from_center_size(
            at + Vec3::Y * BARREL.y * 0.5,
            BARREL,
        ));
    }

    out
}

/// The shell: the stair, the shaft and the room, and nothing standing in them.
fn shell(reaches: f32) -> Vec<Aabb> {
    let along = opening(reaches);
    let face = back();
    let mut out = Vec::new();

    // the treads, going down and out. Each is a slab from its own top all the
    // way to under the cellar floor, which is one box fewer to fall between
    // and is anyway what the underside of a stair looks like.
    let under = -DOWN - THICK;
    for n in 0..STEPS {
        let top = -(n as f32 + 1.0) * RISER;
        let from = face - LANDING - n as f32 * TREAD;

        out.push(Aabb::from_center_size(
            vec3(from - TREAD * 0.5, (top + under) * 0.5, along),
            vec3(TREAD, top - under, PASSAGE),
        ));
    }

    // a soffit over the stair, following it down. Flat at the cellar's own
    // ceiling it was a lid across the top of the flight: the stair starts at
    // nought and that ceiling is at -0.5, so the way down was sealed half a
    // unit below the landing. A stairwell's ceiling goes down with its stair.
    for n in 0..STEPS {
        let top = -(n as f32) * RISER + HEADROOM;
        let from = face - LANDING - n as f32 * TREAD;

        out.push(Aabb::from_center_size(
            vec3(from - TREAD * 0.5, top + THICK * 0.5, along),
            vec3(TREAD, THICK, PASSAGE + THICK * 2.0),
        ));
    }
    // the landing's own soffit stops at the far side of the back wall rather
    // than the near one. `back` is the wall's inner face, which is where the
    // nook is, so a box starting there reaches a third of a unit into the wall
    // and fights the lintel over the opening for the same pixels: a grey
    // rectangle with stippled edges, hanging over the books.
    out.push(Aabb::from_center_size(
        vec3(
            face - THICK - (LANDING - THICK) * 0.5,
            HEADROOM + THICK * 0.5,
            along,
        ),
        vec3(LANDING - THICK, THICK, PASSAGE + THICK * 2.0),
    ));

    // the landing at the top, level with the nook's own floor
    out.push(Aabb::from_center_size(
        vec3(face - LANDING * 0.5, -THICK * 0.5, along),
        vec3(LANDING, THICK, PASSAGE),
    ));

    // the cellar's floor
    let foot = face - LANDING - STEPS as f32 * TREAD;
    out.push(Aabb::from_center_size(
        vec3(foot - DEEP * 0.5, -DOWN - THICK * 0.5, along),
        vec3(DEEP, THICK, SPAN),
    ));

    // the shaft's two sides, so a stair with no bannister is still a stair you
    // cannot walk off sideways
    // from the far side of the back wall, like the soffit, and no higher than
    // the soffit either. They reached the wall's near face and a unit and a
    // half above the nook's floor, which puts them inside the wall and inside
    // the room over it.
    let shaft = SHAFT - LANDING - THICK;
    let up = HEADROOM + THICK;
    for side in [-1.0f32, 1.0] {
        out.push(Aabb::from_center_size(
            vec3(
                face - THICK - shaft * 0.5,
                (up - DOWN) * 0.5,
                along + side * (PASSAGE + THICK) * 0.5,
            ),
            vec3(shaft, up + DOWN, THICK),
        ));
    }

    // and the cellar's own walls, up to its own low ceiling
    let middle = -DOWN + TALL * 0.5;
    for (at, size) in [
        // the far end
        (vec3(foot - DEEP, middle, along), vec3(THICK, TALL, SPAN)),
        // and the near one in two pieces, because the stair comes through it.
        // In one piece it was a wall across the foot of the flight: you could
        // walk the whole way down and not get in.
        (
            vec3(foot, middle, along - (SPAN + PASSAGE) * 0.25),
            vec3(THICK, TALL, (SPAN - PASSAGE) * 0.5),
        ),
        (
            vec3(foot, middle, along + (SPAN + PASSAGE) * 0.25),
            vec3(THICK, TALL, (SPAN - PASSAGE) * 0.5),
        ),
        // the two long sides
        (
            vec3(foot - DEEP * 0.5, middle, along - SPAN * 0.5),
            vec3(DEEP, TALL, THICK),
        ),
        (
            vec3(foot - DEEP * 0.5, middle, along + SPAN * 0.5),
            vec3(DEEP, TALL, THICK),
        ),
        // and a lid on it
        (
            vec3(foot - DEEP * 0.5, -DOWN + TALL, along),
            vec3(DEEP, THICK, SPAN),
        ),
    ] {
        out.push(Aabb::from_center_size(at, size));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk::walk;
    use glam::Vec3;

    /// Spec 0007: everything turned down here has a surface on it.
    ///
    /// A profile with lids spends the ends of its run closing them, so a mesh
    /// given too few steps samples nothing but lids and comes out as a handful
    /// of points with no skin between. That is not a thin shape or a wrong
    /// shape, it is nothing at all, and it is invisible in the one way that
    /// matters: the thing simply is not there and the room looks like somebody
    /// forgot to add it.
    #[test]
    fn everything_turned_has_a_surface() {
        for (what, mesh) in [
            ("barrel", barrel_mesh()),
            ("hoop", hoop_mesh()),
            ("peg", peg_mesh()),
            ("bottle", bottle_mesh()),
            ("candlestick", candlestick_mesh()),
            ("decanter", decanter_mesh()),
            ("glass", glass_mesh()),
            ("flame", flame_mesh()),
        ] {
            let widest = mesh
                .vertices
                .iter()
                .map(|v| vec3(v.position[0], 0.0, v.position[2]).length())
                .fold(0.0f32, f32::max);
            let tall = mesh.bounds().size().y;

            assert!(
                mesh.triangle_count() > 16,
                "the {} is {} triangles",
                what,
                mesh.triangle_count()
            );
            assert!(widest > 0.1, "the {} is {} across", what, widest);
            assert!(tall > 0.5, "the {} is {} tall", what, tall);
        }
    }

    /// Spec 0007: a barrel is skinned the right way out.
    ///
    /// The normal is the cross of the surface's two tangents, so which way
    /// round you go decides which side of the skin is the outside. The obvious
    /// way round points it into the barrel: every face is wound away from you,
    /// the near half is culled, and what is left is the inside of the far wall,
    /// which draws as a pale lump with corners on it.
    #[test]
    fn a_barrel_is_skinned_outwards() {
        let mesh = barrel_mesh();

        // the winding, which is the half this test did not ask about and the
        // half that was wrong. A stored normal pointing out and a triangle
        // wound inwards are both possible at once, and a triangle wound away
        // from you is culled and drawn nowhere.
        for triangle in mesh.indices.chunks_exact(3) {
            let corner = |n: usize| Vec3::from(mesh.vertices[triangle[n] as usize].position);
            let (a, b, c) = (corner(0), corner(1), corner(2));
            let wound = (b - a).cross(c - b);
            let middle = (a + b + c) / 3.0;
            // out from the middle of the barrel rather than out from its axis,
            // because it has lids on it now and the way out of a lid is up
            let out = middle;

            if wound.length() < 1e-9 || out.length() < 1e-4 {
                continue;
            }

            assert!(
                wound.normalize().dot(out.normalize()) > 0.0,
                "a triangle at {:?} is wound {:?}, which is inwards",
                middle,
                wound.normalize()
            );
        }

        // and the normals, which have to agree with it
        for vertex in &mesh.vertices {
            let at = Vec3::from(vertex.position);
            let normal = Vec3::from(vertex.normal);

            if at.length() < 1e-3 || normal.length() < 1e-3 {
                continue;
            }

            assert!(
                normal.normalize().dot(at.normalize()) > -0.01,
                "a face at {:?} looks {:?}, which is inwards",
                at,
                normal
            );
        }

        // and it is closed, so there is no side of it that is not there
        let top = mesh
            .vertices
            .iter()
            .filter(|v| v.position[1] > 0.49)
            .count();
        let bottom = mesh
            .vertices
            .iter()
            .filter(|v| v.position[1] < -0.49)
            .count();

        assert!(
            top > 2 && bottom > 2,
            "a barrel with {}/{} ends",
            top,
            bottom
        );

        // and it is fatter at the waist than at its ends, which is the whole
        // difference between a barrel and a drum
        let fattest = mesh
            .vertices
            .iter()
            .map(|v| vec3(v.position[0], 0.0, v.position[2]).length())
            .fold(0.0f32, f32::max);
        let ends = mesh
            .vertices
            .iter()
            .filter(|v| v.position[1].abs() > 0.49)
            .map(|v| vec3(v.position[0], 0.0, v.position[2]).length())
            .fold(0.0f32, f32::max);

        assert!(
            fattest > ends * 1.1,
            "the waist is {} and the ends are {}",
            fattest,
            ends
        );
    }

    /// Spec 0007: the stair is one you can walk, which is a thing about the
    /// riser and not about the drawing.
    #[test]
    fn a_tread_is_lower_than_a_step() {
        let (riser, step) = (RISER, crate::walk::STEP);
        assert!(
            riser < step * 0.75,
            "a riser of {} against a step of {} is a stair you catch your feet on",
            riser,
            step
        );
        assert!((STEPS as f32 * RISER - DOWN).abs() < 1e-5);
    }

    /// Spec 0007: and the run fits in the shaft that holds it.
    #[test]
    fn the_stair_fits_the_shaft() {
        let run = STEPS as f32 * TREAD;

        assert!(run + LANDING * 2.0 <= SHAFT + 1e-5, "the run is {}", run);
    }

    /// Spec 0007: you can walk down it, and back up.
    ///
    /// Walked rather than reasoned about, by the same code that moves you in
    /// the room. A stair is the one thing in this building where what the
    /// arithmetic says and what happens when somebody walks at it have ever
    /// come apart.
    #[test]
    fn you_can_walk_down_it_and_back_up() {
        let reaches = 9.4;
        let solid = solid(reaches);
        let along = opening(reaches);
        let radius = crate::RADIUS;
        let speed = 4.2;
        let dt = 1.0 / 60.0;

        // from the landing, walking out away from the nook
        let mut at = vec3(back() - 0.5, 0.0, along);
        let mut falling = 0.0;
        for _ in 0..300 {
            let (next, fell) = walk(at, Vec3::NEG_X * speed, falling, radius, dt, &solid);
            at = next;
            falling = fell;
        }

        assert!(
            (at.y + DOWN).abs() < 0.05,
            "you got to {} and the cellar floor is at {}",
            at.y,
            -DOWN
        );

        // and back up the way you came, as far as the landing. Only as far: past
        // it is the nook, and the nook's floor is the room's business and not
        // in this test's geometry, so walking on walks off the end of the world.
        let mut back_up = false;
        for _ in 0..360 {
            let (next, fell) = walk(at, Vec3::X * speed, falling, radius, dt, &solid);
            at = next;
            falling = fell;

            if at.y.abs() < 0.01 {
                back_up = true;
                break;
            }
        }

        assert!(back_up, "you got as far back up as {}", at.y);
    }
}
