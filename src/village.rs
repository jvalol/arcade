//! A village out in the field, with people living in it. Spec 0068.
//!
//! **South of the way out, where the bridge is west.** Spec 0054's bridge runs
//! west along the gap's own middle line and the whole of the trail ends up
//! there. A village on that line is on the way to something; this one is off
//! it, so walking to it is a choice rather than a step.
//!
//! Nothing here is behind a door. The cabins are solid lumps with lit windows,
//! as spec 0056's outbuildings are: the hall at the top of the bridge is the
//! one building in this game you go into. What this place is for is being
//! walked through, and everything it has to show is on the ground.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far west of the garden's ground the village's middle stands, and how
/// far south of the way out.
///
/// **Measured off the things it must not touch.** The bridge's foot is
/// `bridge::OUT` west of the garden on the way's own line, and it climbs west
/// from there, so south is the free direction. Forty-four is far enough that
/// the garden's wall does not frame it and near enough that you can see it
/// from the gap and decide to go.
pub const WEST: f32 = 56.0;
pub const SOUTH: f32 = 46.0;

/// The middle of the street, on the ground.
pub fn at(reaches: f32) -> Vec3 {
    vec3(
        crate::field::starts(reaches) - WEST,
        0.0,
        crate::garden::at(reaches).z - SOUTH,
    )
}

/// How long the street runs and how wide it is between the fronts.
///
/// **Ten across, which is two wagons and a walk.** A wagon here is 2.4 wide
/// and the rule this building furnishes by wants 1.05 clear of everything for
/// a body: a street of six would be a corridor with carts in it.
pub const STREET_LONG: f32 = 58.0;
pub const STREET_WIDE: f32 = 10.0;

/// And how far the whole place reaches, which is what keeps trees out of it
/// and what the tests measure against.
///
/// **Worked out from what is in it rather than written down.** A second
/// number for how big the village is drifts from the first the moment a
/// cabin moves, and the first build had exactly that: a box that said 38
/// metres while the barn stood at 43.
pub fn around(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let mut box_ =
        Aabb::from_center_size(middle + Vec3::Y * 5.0, vec3(STREET_LONG, 10.0, STREET_WIDE));

    for one in built(reaches) {
        let walls = one.footprint();
        box_ = Aabb::new(box_.min.min(walls.min), box_.max.max(walls.max));
    }

    // and a margin, so the edge of it is not the wall of a cabin
    Aabb::new(
        box_.min - vec3(5.0, 0.0, 5.0),
        box_.max + vec3(5.0, 0.0, 5.0),
    )
}

/// Whether this is inside the village, for the field to keep its trees and
/// its flowers out.
///
/// **A share wider than `around`**, because a tree whose trunk is just
/// outside still has a crown over the roof.
pub fn shades(reaches: f32, at: Vec3) -> bool {
    let box_ = around(reaches);
    let out = 3.0;

    at.x > box_.min.x - out
        && at.x < box_.max.x + out
        && at.z > box_.min.z - out
        && at.z < box_.max.z + out
}

/// One building: the middle of its footprint, how far it runs each way, and
/// how high its eaves and its ridge stand.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Built {
    pub at: Vec3,
    /// East and west, and north and south.
    pub wide: f32,
    pub long: f32,
    pub eaves: f32,
    pub ridge: f32,
    /// Which way its gable looks, as a facing. The street is east and west,
    /// so the cabins face across it and the barn closes the end.
    pub faces: f32,
}

impl Built {
    /// The box of its walls, which is what you walk round.
    pub fn walls(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.eaves * 0.5,
            vec3(self.wide, self.eaves, self.long),
        )
    }

    /// And the box its roof fills, which is what one building must not share
    /// with another.
    pub fn footprint(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.ridge * 0.5,
            vec3(self.wide, self.ridge, self.long),
        )
    }
}

/// How far back from the street the fronts stand.
pub const SET_OFF: f32 = STREET_WIDE * 0.5;

/// The cabins and the barn.
///
/// **None of them the same size**, which is the whole of what tells a village
/// from a row of sheds. Spec 0056's steading has three outbuildings off a lane
/// and reads as a steading because they differ; four identical ones read as a
/// barracks, which is what this was on its first build.
pub fn built(reaches: f32) -> Vec<Built> {
    let middle = at(reaches);
    let one = |along: f32, side: f32, wide: f32, long: f32, eaves: f32, ridge: f32| Built {
        at: vec3(
            middle.x + along,
            middle.y,
            middle.z + side * (SET_OFF + long * 0.5),
        ),
        wide,
        long,
        eaves,
        ridge,
        // **Looking at the street, which is the other way round from the
        // first build.** A facing of nought in this building looks down
        // -z, so the row standing north of the street has to be PI to
        // look back at it. Written the obvious way, every door and every
        // window was on the back of its own cabin and the street was a
        // run of blank gables.
        faces: if side > 0.0 { 0.0 } else { PI },
    };

    vec![
        // the north side, nearest the way you come in
        one(-16.0, -1.0, 7.0, 6.0, 2.5, 4.1),
        one(-4.0, -1.0, 9.0, 7.0, 2.8, 4.6),
        one(9.0, -1.0, 6.0, 5.5, 2.4, 3.8),
        // and the south, set along differently so the street is not a
        // corridor of pairs, and with a wide gap left between the last two
        // for the common the fire stands on
        one(-22.0, 1.0, 6.5, 6.0, 2.5, 4.0),
        one(-11.0, 1.0, 8.0, 6.5, 2.6, 4.3),
        one(11.0, 1.0, 7.5, 6.0, 2.5, 4.2),
        // **And the barn, closing the west end from beyond it.** It was
        // put inside the street's own run on the first build, 15 deep
        // against a street 10 wide, so it lay across the thing it was
        // meant to close. `tests::the_street_is_clear` said so before
        // anything had been drawn.
        Built {
            at: vec3(middle.x - STREET_LONG * 0.5 - 8.0, middle.y, middle.z),
            wide: 12.0,
            long: 15.0,
            eaves: 4.2,
            ridge: 7.4,
            faces: FRAC_PI_2,
        },
    ]
}

/// How far clear of a thing somebody stands when their errand is at it.
pub const CLEAR_OF: f32 = 1.6;

/// The firepit in the middle of the street, and the ring round it.
pub const RING_WIDE: f32 = 2.4;
pub const RING_HIGH: f32 = 0.3;
pub const RING_STONES: usize = 13;
pub const RING_FLAMES: usize = 3;

/// **On the common and not in the street.** It was in the middle of the way
/// through on the first build, which is where a village puts a fire in a
/// picture and nowhere a village would put one: a street is walked down
/// before it is looked at, and `tests::the_street_is_clear` said so.
///
/// The common is the wide gap left in the south side between the last two
/// cabins, which `fences` leaves open for the same reason.
pub fn ring(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x + 0.5, middle.y, middle.z + SET_OFF + 3.6)
}

/// The stones round it, each a lump of its own.
pub fn ring_stones(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = ring(reaches);

    (0..RING_STONES)
        .map(|n| {
            let round = n as f32 / RING_STONES as f32 * TAU;
            // not evenly, because a ring of even stones is a wheel
            let out = RING_WIDE * 0.5 * (0.93 + 0.14 * (round * 2.0).sin());

            (
                vec3(
                    middle.x + round.cos() * out,
                    middle.y + RING_HIGH * 0.5,
                    middle.z + round.sin() * out,
                ),
                round,
            )
        })
        .collect()
}

/// How high the pot hangs over it and how big it is.
pub const POT_UP: f32 = 1.15;
pub const POT_WIDE: f32 = 0.78;

/// The tripod the pot hangs from: three legs meeting over the fire.
pub fn tripod(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let middle = ring(reaches);
    let foot = RING_WIDE * 0.62;

    (0..3)
        .map(|n| {
            let round = n as f32 / 3.0 * TAU + 0.4;

            (
                vec3(
                    middle.x + round.cos() * foot,
                    middle.y,
                    middle.z + round.sin() * foot,
                ),
                vec3(middle.x, middle.y + POT_UP + 0.5, middle.z),
            )
        })
        .collect()
}

/// The cook pot that hangs over the fire.
///
/// **Turned, because a pot is.** `cellar::turned` is what everything round
/// in this building comes off and a box with a lid on it is a crate. Open
/// at the top and two sided, or you see through the far wall of it.
pub fn pot_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(18, 12, |v| {
        // a base that flares, a belly, a shoulder and a lid that closes.
        // **Closed at the top**, because `turned` makes one surface and an
        // open pot is one you see the far wall through from above, which is
        // where a pot over a fire is looked at from.
        let waist = if v < 0.14 {
            0.6 + v / 0.14 * 0.34
        } else if v < 0.72 {
            0.94 + ((v - 0.14) / 0.58 * PI).sin() * 0.1
        } else {
            let t = (v - 0.72) / 0.28;

            0.94 * (1.0 - t * t)
        };

        (v, waist)
    })
}

/// Where the well stands, and the trough beside it.
pub const WELL_WIDE: f32 = 1.5;
pub const WELL_HIGH: f32 = 0.85;
pub const TROUGH_LONG: f32 = 2.6;
pub const TROUGH_WIDE: f32 = 0.8;
pub const TROUGH_HIGH: f32 = 0.55;

/// Against the fronts on the south side, in the gap between two cabins, so
/// it is off the lane a wagon takes and still in the street.
pub fn well(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(
        middle.x - 16.5,
        middle.y,
        middle.z + SET_OFF - WELL_WIDE * 0.7,
    )
}

pub fn trough(reaches: f32) -> Aabb {
    let one = well(reaches);

    Aabb::from_center_size(
        vec3(one.x + 2.6, one.y + TROUGH_HIGH * 0.5, one.z),
        vec3(TROUGH_LONG, TROUGH_HIGH, TROUGH_WIDE),
    )
}

/// How big a wagon is: the bed, and the wheels at its corners.
pub const WAGON_LONG: f32 = 3.6;
pub const WAGON_WIDE: f32 = 2.0;
pub const WAGON_BED: f32 = 0.95;
pub const WAGON_SIDE: f32 = 0.55;
pub const WHEEL_WIDE: f32 = 1.1;
pub const WHEEL_THICK: f32 = 0.16;
pub const SPOKES: usize = 10;

/// Where the wagons stand and which way they are pointed.
///
/// **Off the street and not on it.** A wagon in the middle of the way
/// through is the thing `tests::the_street_is_clear` exists to stop: this
/// village is walked down before it is looked at.
pub fn wagons(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);

    vec![
        (
            vec3(middle.x + 14.0, middle.y, middle.z - SET_OFF + 1.1),
            0.12,
        ),
        (
            vec3(middle.x - 24.0, middle.y, middle.z + SET_OFF - 1.3),
            PI - 0.2,
        ),
    ]
}

/// The hitching rail: two posts and a bar between them.
pub const RAIL_LONG: f32 = 5.0;
pub const RAIL_HIGH: f32 = 1.15;
pub const RAIL_THICK: f32 = 0.14;

pub fn rail(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x + 19.5, middle.y, middle.z + SET_OFF - 1.2)
}

/// How big a horse is, as four heights off the ground and two across.
///
/// **Measured against a body**, which is `people::TALL` at 1.78. A horse's
/// back is about 1.5 and it is a bit over two metres nose to tail, so the
/// one standing next to somebody is lower at the shoulder and a good deal
/// longer. Drawn smaller it reads as a dog.
///
/// **Each one is a height and not an offset**, which is the lesson of the
/// first build: the legs were given the back's height and hung from the
/// barrel's middle, so they ran twenty centimetres into the ground and the
/// whole animal read as a table somebody had left out.
pub const HORSE_LONG: f32 = 2.1;
pub const HORSE_WIDE: f32 = 0.6;
/// The top of the back, the bottom of the barrel, and so how deep it is.
pub const HORSE_BACK: f32 = 1.5;
pub const HORSE_BELLY: f32 = 0.82;
pub const HORSE_DEEP: f32 = HORSE_BACK - HORSE_BELLY;
/// And how high the head is carried.
pub const HORSE_HEAD: f32 = 1.82;

/// Where the horses stand, and which way each faces.
///
/// At the rail, nose to it, which is where a horse that has been left is.
pub fn horses(reaches: f32) -> Vec<(Vec3, f32)> {
    let one = rail(reaches);

    (0..2)
        .map(|n| {
            let along = (n as f32 - 0.5) * RAIL_LONG * 0.46;

            (vec3(one.x + along, one.y, one.z + 1.3), 0.0)
        })
        .collect()
}

/// The fences between the cabins, each as the two ends of a run.
///
/// **So the street is a street.** Without them the gaps between the cabins
/// are ways out of it, and a village you can leave sideways from anywhere is
/// a scattering of sheds.
pub const FENCE_HIGH: f32 = 1.05;
pub const FENCE_POSTS: f32 = 1.5;

pub fn fences(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let houses = built(reaches);
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        let mut along: Vec<&Built> = houses
            .iter()
            .filter(|one| one.faces != FRAC_PI_2 && (one.at.z - at(reaches).z).signum() == side)
            .collect();
        along.sort_by(|a, b| a.at.x.total_cmp(&b.at.x));

        for pair in along.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            let z = at(reaches).z + side * (SET_OFF + 0.4);
            let from = left.at.x + left.wide * 0.5;
            let to = right.at.x - right.wide * 0.5;

            // **A gap wide enough to take a wagon is a way in and not a
            // hole to patch.** Fencing every gap closes the common the
            // fire stands on, and a village with one way into it is a
            // stockade.
            if to - from > WAGON_LONG * 2.0 {
                continue;
            }

            out.push((vec3(from, 0.0, z), vec3(to, 0.0, z)));
        }
    }

    out
}

/// The beaten earth of the street, which is what tells it from a field.
///
/// **A village has a track through it and a field does not.** Without this
/// the street was grass between two rows of cabins, which reads as a mown
/// gap rather than as the way everybody walks. Laid a hair over the field's
/// own ground, as every floor in this building is laid over what is under
/// it, because two surfaces in one plane is the fault this place keeps
/// making.
pub const TRACK_UP: f32 = 0.015;

pub fn track(reaches: f32) -> Aabb {
    let middle = at(reaches);

    Aabb::from_center_size(
        middle + Vec3::Y * (TRACK_UP - 0.05),
        vec3(STREET_LONG + 14.0, 0.1, STREET_WIDE),
    )
}

/// The wheel ruts worn down it, as a run of short pieces each.
///
/// **A track and not two painted lines.** The street was one flat slab of
/// earth, which is a surface and not a place anybody has driven over. What
/// says a cart goes up and down here is a pair of ruts at a wagon's own
/// wheel spacing, and what stops them reading as road markings is that they
/// wander: each piece is laid off the one before it by a little, the way a
/// wheel follows the last wheel without following it exactly.
pub const RUT_WIDE: f32 = 0.34;
pub const RUT_PIECES: usize = 58;

pub fn ruts(reaches: f32) -> Vec<Aabb> {
    let middle = at(reaches);
    let long = STREET_LONG + 14.0;
    let piece = long / RUT_PIECES as f32;
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        // a wagon's own track, which is the wheels either side of its bed
        let apart = (WAGON_WIDE * 0.5 + WHEEL_THICK) * side;

        for n in 0..RUT_PIECES {
            let along = middle.x - long * 0.5 + (n as f32 + 0.5) * piece;
            let seed = along * 0.37 + side * 11.0;
            let wander = (seed * 1.3).sin() * 0.22 + (seed * 0.47).sin() * 0.16;

            out.push(Aabb::from_center_size(
                vec3(
                    along,
                    middle.y + TRACK_UP + 0.002,
                    middle.z + apart + wander,
                ),
                // **Barely off the track, and in short pieces.** It was a
                // centimetre proud in pieces nearly three metres long,
                // which lit along its edge and stepped sideways at every
                // join: a kerb laid down the street rather than a wheel
                // worn into it. What a rut is here is a darker line, and
                // the lift is only enough to keep it off the track's face.
                vec3(piece * 1.06, 0.005, RUT_WIDE),
            ));
        }
    }

    out
}

/// And the common off it, where the fire stands.
pub fn common(reaches: f32) -> Aabb {
    let fire = ring(reaches);

    Aabb::from_center_size(
        vec3(fire.x, at(reaches).y + TRACK_UP - 0.05, fire.z),
        vec3(13.0, 0.1, 11.0),
    )
}

/// A cabin's door, which is a board a shade darker than its wall.
pub const DOOR_WIDE: f32 = 0.95;
pub const DOOR_HIGH: f32 = 1.95;

/// Where each one is, and which way it looks.
pub fn doors(reaches: f32) -> Vec<(Vec3, Vec3)> {
    built(reaches)
        .into_iter()
        .map(|one| {
            let looks = vec3(one.faces.sin(), 0.0, -one.faces.cos());
            // a stride off the middle, so the door and the window are not
            // stacked on one line
            let aside = vec3(-looks.z, 0.0, looks.x) * one.wide * 0.22;

            (
                one.at + looks * (one.long * 0.5) + aside + Vec3::Y * DOOR_HIGH * 0.5,
                looks,
            )
        })
        .collect()
}

/// How far a roof oversails its gable, as a share of the building's length.
///
/// Spec 0056's roofs use 0.52 of the length either way. A pioneer cabin
/// wants more of an overhang than a longhouse, because the eaves are what
/// read as folksy at any distance.
pub const OVERSAILS: f32 = 0.58;

/// The run of one pitch, from eaves to ridge.
pub fn slope_run(one: &Built) -> f32 {
    (one.wide * 0.5).hypot(one.ridge - one.eaves)
}

/// How wide a board is, the gap between two, and how far they stand proud.
///
/// **Boards rather than a picture of boards.** The cellar's panelling says
/// why, and it is the same reason here: a cube's corners run nought to one
/// however big it is, so a timber texture on a cabin's wall would be a
/// single plank seven metres long. What makes sawn boards read is the
/// shadow line between one and the next, and a shadow line wants an edge.
pub const BOARD: f32 = 0.3;
pub const BOARD_GAP: f32 = 0.025;
pub const BOARD_OUT: f32 = 0.035;

/// Every board on one building: where it is, how big, and how its own tone
/// stands against its neighbours.
///
/// **No two the same shade.** A wall of identical boards is a wall with
/// lines ruled on it. Weathered sawn timber goes patchy, and the patchiness
/// is most of what says this was built by hand out of what was to hand.
///
/// Off the board's own place rather than off a roll, so one building is one
/// building from one frame to the next and nothing has to be remembered.
pub fn boarding(one: &Built) -> Vec<(Vec3, Vec3, f32)> {
    let courses = ((one.eaves / (BOARD + BOARD_GAP)).floor() as usize).max(1);
    let mut out = Vec::new();

    for course in 0..courses {
        let up = (course as f32 + 0.5) * (BOARD + BOARD_GAP);

        for (face, size) in [
            (
                vec3(one.wide * 0.5 + BOARD_OUT * 0.5, up, 0.0),
                vec3(BOARD_OUT, BOARD, one.long),
            ),
            (
                vec3(-one.wide * 0.5 - BOARD_OUT * 0.5, up, 0.0),
                vec3(BOARD_OUT, BOARD, one.long),
            ),
            (
                vec3(0.0, up, one.long * 0.5 + BOARD_OUT * 0.5),
                vec3(one.wide, BOARD, BOARD_OUT),
            ),
            (
                vec3(0.0, up, -one.long * 0.5 - BOARD_OUT * 0.5),
                vec3(one.wide, BOARD, BOARD_OUT),
            ),
        ] {
            let seed = face.x * 3.1 + face.z * 7.7 + up * 13.3;
            // two sines that do not divide, so the pattern up a wall never
            // repeats and no two walls of one building agree
            let tone = 1.0 + (seed * 1.7).sin() * 0.07 + (seed * 0.41).sin() * 0.05;

            out.push((one.at + face, size, tone));
        }
    }

    out
}

/// How thick a roof's pitch is, which is the slab the courses lie on.
///
/// **Here rather than only where it is drawn.** `shingling` has to stand
/// its courses clear of this and the two were written in two places: the
/// courses came off the pitch's middle line by five centimetres and the
/// pitch is twenty thick, so every shingle on every roof in the village
/// was inside the slab it was laid on. Nothing showed, and nothing could.
/// `tests::every_course_stands_proud_of_its_pitch` holds it now.
pub const PITCH_DEEP: f32 = 0.2;

/// How deep one course of shingles is up the slope, the gap between one
/// course and the next, and how far each stands proud of the pitch.
///
/// Laid the way the boards are laid on the walls: proud, with a gap, so
/// the shadow line under a course is what the eye reads. Lapping them was
/// the first try and it puts two courses' outer faces in one plane over
/// the lap, which is this building's oldest fault.
pub const SHINGLE: f32 = 0.42;
pub const SHINGLE_GAP: f32 = 0.05;
pub const SHINGLE_OUT: f32 = 0.05;

/// The courses on one roof: where each sits, how big, which way it leans,
/// and its own tone.
///
/// **Laid from the eaves up, each standing off the pitch.** A roof drawn as
/// two flat slabs is a roof nobody has been on. What reads as shingle at
/// thirty metres is the line of shadow under every course, which is why
/// each one stands `SHINGLE_OUT` off the face of the pitch it lies on.
pub fn shingling(one: &Built) -> Vec<(Vec3, Vec3, f32, f32)> {
    let run = slope_run(one);
    let rise = one.ridge - one.eaves;
    let slope = (rise / (one.wide * 0.5)).atan();
    let over = one.long * OVERSAILS;
    let courses = ((run / SHINGLE).floor() as usize).max(2);
    let mut out = Vec::new();

    for side in [-1.0f32, 1.0] {
        let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
        let ridge = vec3(one.at.x, one.at.y + one.ridge, 0.0);

        for course in 0..courses {
            let along = (course as f32 + 0.5) / courses as f32;
            let on = eaves + (ridge - eaves) * along;
            // **Out of the pitch along its own normal, far enough to clear
            // it.** Half the pitch and half the course, so the course sits
            // on the pitch's face the way a board sits on a wall.
            let deep = (PITCH_DEEP + SHINGLE_OUT) * 0.5;
            let up = vec3(side * slope.sin() * deep, slope.cos() * deep, 0.0);
            let seed = along * 23.0 + side * 5.0 + one.at.x * 0.7;
            let tone = 1.0 + (seed * 2.3).sin() * 0.09 + (seed * 0.57).sin() * 0.06;

            out.push((
                vec3(on.x + up.x, on.y + up.y, one.at.z),
                // and a little short of the gable at either end, so no two
                // faces end in one plane
                vec3(
                    run / courses as f32 - SHINGLE_GAP,
                    SHINGLE_OUT,
                    over * 2.0 - 0.04,
                ),
                -side * slope,
                tone,
            ));
        }
    }

    out
}

/// Where a cabin's lit window is, and which way it looks.
///
/// One a cabin, in the gable that faces the street. Spec 0056 lights its
/// outbuildings from inside and the opening is a hole in the same room;
/// this does the same thing with one opening instead of a row.
pub const WINDOW_WIDE: f32 = 0.85;
pub const WINDOW_HIGH: f32 = 0.95;
pub const WINDOW_UP: f32 = 1.25;
pub const SET_BACK: f32 = 0.3;

pub fn windows(reaches: f32) -> Vec<(Vec3, Vec3)> {
    built(reaches)
        .into_iter()
        .map(|one| {
            let looks = vec3(one.faces.sin(), 0.0, -one.faces.cos());
            let out = if one.faces == FRAC_PI_2 {
                vec3(one.wide * 0.5 - SET_BACK, 0.0, 0.0)
            } else {
                vec3(
                    0.0,
                    0.0,
                    -(one.long * 0.5 - SET_BACK) * one.faces.cos().signum(),
                )
            };

            (one.at + out + Vec3::Y * WINDOW_UP, looks)
        })
        .collect()
}

/// Where the lamp inside each one sits, which is what the window shows.
///
/// Inside and back from the opening, so what you see through it is a room
/// and not a bulb. Spec 0056 learnt this at five metres; a cabin is four.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    built(reaches)
        .into_iter()
        .map(|one| one.at + Vec3::Y * (one.eaves * 0.45))
        .collect()
}

/// Where somebody stands to use a thing: clear of it, facing it.
fn at_the(thing: Vec3, from: Vec3, clear: f32) -> (Vec3, f32) {
    let way = (from - thing).normalize_or_zero();
    let at = vec3(thing.x + way.x * clear, 0.0, thing.z + way.z * clear);

    // facing the thing, which is back down the way we came out
    (at, crate::people::facing(at, thing))
}

/// What the people here are doing, as a round apiece. Spec 0068, over spec
/// 0067's machinery.
///
/// **One errand each, and each one Jake's.** From a cabin to the cook fire,
/// from the wagons to the horses, from the well to a cabin, and one that
/// walks the length of the street, which is somebody going somewhere you are
/// not. Four rounds in a village of seven buildings is not a crowd, and a
/// crowd is not what this is: what makes a place read as lived in is that
/// something is always happening somewhere in it.
pub fn rounds(reaches: f32) -> Vec<Vec<crate::people::Stop>> {
    use crate::people::Stop;

    let middle = at(reaches);
    let fire = ring(reaches);
    let houses = built(reaches);
    let doors = doors(reaches);
    let carts = wagons(reaches);
    let rail_at = rail(reaches);
    let water = well(reaches);
    let stop = |at: Vec3, facing: f32, stays: f32| Stop { at, facing, stays };
    let before = |thing: Vec3, looks: Vec3, stays: f32| {
        let (at, facing) = at_the(thing, thing + looks * 4.0, CLEAR_OF);

        stop(at, facing, stays)
    };
    // the lane a round walks along, which is the middle of the street and
    // where nothing stands
    let lane = |x: f32| stop(vec3(x, 0.0, middle.z), 0.0, 0.0);

    vec![
        // **Cooking**: out of the south cabin nearest the common, to the
        // fire, and back. This is the errand Jake named first.
        {
            let (by_fire, at_fire) = at_the(
                fire,
                vec3(fire.x + 6.0, 0.0, fire.z),
                RING_WIDE * 0.5 + CLEAR_OF,
            );
            // **Out past the cabin's own corner before turning for the
            // fire.** Walked straight, the leg from the doorstep cut the
            // south-west corner of the cabin it had just come out of, which
            // is what `tests::nobody_walks_through_anything` said twice:
            // once for cutting it and once for not cutting it far enough.
            // The corner is due south of where they end up, so the last
            // run is straight up a line nothing stands on.
            let round = stop(
                vec3(by_fire.x, 0.0, doors[5].0.z + doors[5].1.z * 4.0),
                0.0,
                0.0,
            );

            vec![
                before(doors[5].0, doors[5].1, 5.0),
                round,
                stop(by_fire, at_fire, 7.0),
                round,
            ]
        },
        // **Loading**: between the wagon at the east end and the horses at
        // the rail, which is the other errand he named.
        vec![
            {
                let (at, facing) = at_the(
                    carts[0].0,
                    carts[0].0 + vec3(0.0, 0.0, 4.0),
                    WAGON_WIDE * 0.5 + CLEAR_OF,
                );
                stop(at, facing, 5.5)
            },
            lane(rail_at.x - 2.0),
            {
                let (at, facing) =
                    at_the(rail_at, rail_at + vec3(0.0, 0.0, -4.0), RAIL_LONG * 0.36);
                stop(at, facing, 6.0)
            },
            lane(rail_at.x - 2.0),
        ],
        // **Water**: the well to the cabin across the street and back.
        vec![
            {
                let (at, facing) = at_the(
                    water,
                    water + vec3(0.0, 0.0, -4.0),
                    WELL_WIDE * 0.5 + CLEAR_OF,
                );
                stop(at, facing, 6.0)
            },
            lane(water.x + 1.0),
            before(doors[1].0, doors[1].1, 4.5),
            lane(water.x + 1.0),
        ],
        // **And somebody walking the length of it**, barn end to the east
        // end and back, which is what makes the street a street rather than
        // three errands that happen to be near each other.
        vec![
            stop(
                vec3(
                    houses[6].at.x + houses[6].wide * 0.5 + CLEAR_OF * 2.0,
                    0.0,
                    middle.z,
                ),
                -std::f32::consts::FRAC_PI_2,
                4.0,
            ),
            lane(middle.x),
            stop(
                vec3(middle.x + STREET_LONG * 0.5 - 3.0, 0.0, middle.z),
                std::f32::consts::FRAC_PI_2,
                4.0,
            ),
            lane(middle.x),
        ],
    ]
}

/// What you cannot walk through out here.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let mut out: Vec<Aabb> = built(reaches).iter().map(Built::walls).collect();

    out.push(Aabb::from_center_size(
        ring(reaches) + Vec3::Y * crate::walk::RIDES * 1.5,
        vec3(RING_WIDE, crate::walk::RIDES * 3.0, RING_WIDE),
    ));
    out.push(Aabb::from_center_size(
        well(reaches) + Vec3::Y * WELL_HIGH * 0.5,
        vec3(WELL_WIDE, WELL_HIGH, WELL_WIDE),
    ));
    out.push(trough(reaches));

    for (at, _) in wagons(reaches) {
        out.push(Aabb::from_center_size(
            at + Vec3::Y * (WAGON_BED + WAGON_SIDE) * 0.5,
            vec3(WAGON_LONG, WAGON_BED + WAGON_SIDE, WAGON_WIDE),
        ));
    }

    let one = rail(reaches);
    out.push(Aabb::from_center_size(
        one + Vec3::Y * RAIL_HIGH * 0.5,
        vec3(RAIL_LONG, RAIL_HIGH, RAIL_THICK * 2.0),
    ));

    for (from, to) in fences(reaches) {
        let middle = (from + to) * 0.5;
        out.push(Aabb::from_center_size(
            middle + Vec3::Y * FENCE_HIGH * 0.5,
            vec3((to.x - from.x).abs().max(0.1), FENCE_HIGH, 0.12),
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const REACHES: f32 = 20.0;

    /// Spec 0068: a course lies on the face of its pitch, not inside it.
    ///
    /// **Because the course and the pitch were written in two places.**
    /// The pitch's thickness lived where it was drawn and the courses came
    /// off the pitch's middle line, so every shingle on every roof was
    /// buried in the slab it was laid on and the roofs read as two smooth
    /// planes. Nothing in either file was wrong on its own.
    #[test]
    fn every_course_stands_proud_of_its_pitch() {
        for one in built(REACHES) {
            let slope = ((one.ridge - one.eaves) / (one.wide * 0.5)).atan();

            for (at, size, lean, _) in shingling(&one) {
                // which pitch it is on, which its own lean says
                let side = if lean < 0.0 { 1.0f32 } else { -1.0 };
                let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
                let normal = vec3(side * slope.sin(), slope.cos(), 0.0);
                let off = (vec3(at.x, at.y, 0.0) - eaves).dot(normal);

                assert!(
                    off - size.y * 0.5 >= PITCH_DEEP * 0.5 - 1e-3,
                    "a course's underside is {:.3} off the pitch's middle and the pitch is {:.3} thick",
                    off - size.y * 0.5,
                    PITCH_DEEP
                );
                assert!(
                    off + size.y * 0.5 >= PITCH_DEEP * 0.5 + SHINGLE_OUT - 1e-3,
                    "a course stands {:.3} proud of the pitch's face",
                    off + size.y * 0.5 - PITCH_DEEP * 0.5
                );
            }
        }
    }

    /// Spec 0068: there is a clear way down the street.
    ///
    /// **A lane through it and not an empty rectangle.** The street is the
    /// one thing here everybody uses and a village you have to pick your
    /// way through is a yard, but a well and a trough standing against the
    /// fronts are what a street has in it. What has to hold is that a
    /// wagon can go from one end to the other, which is `WAGON_WIDE` and
    /// a body either side.
    #[test]
    fn the_street_is_clear() {
        let middle = at(REACHES);
        let body = crate::RADIUS * 2.0;
        let lane = Aabb::from_center_size(
            middle + Vec3::Y * 1.0,
            vec3(STREET_LONG - 2.0, 2.0, WAGON_WIDE + body * 2.0),
        );

        for one in solid(REACHES) {
            assert!(
                !lane.intersects(&one),
                "something stands in the street: {:?} to {:?}",
                one.min,
                one.max
            );
        }
    }

    /// Spec 0068: no two buildings share ground.
    #[test]
    fn no_two_buildings_overlap() {
        let houses = built(REACHES);

        for (n, one) in houses.iter().enumerate() {
            for two in houses.iter().skip(n + 1) {
                assert!(
                    !one.footprint().intersects(&two.footprint()),
                    "two buildings share ground at {:?} and {:?}",
                    one.at,
                    two.at
                );
            }
        }
    }

    /// Spec 0068: it stands clear of everything else out there.
    ///
    /// **The garden, the bridge and the field's own stone.** Each of those
    /// was put where it is by its own spec and none of them knows this
    /// place exists, so the clearance is this file's to keep.
    #[test]
    fn it_stands_clear_of_everything_else() {
        let box_ = around(REACHES);
        let trail = {
            let mut trail = crate::trail::Trail::new();
            for one in crate::trail::Asked::every() {
                trail.gave(*one);
            }
            trail
        };

        for one in crate::bridge::solid(REACHES, &trail) {
            assert!(
                !box_.intersects(&one),
                "the bridge runs through the village at {:?}",
                one.min
            );
        }

        let garden = crate::garden::at(REACHES);
        let half = crate::garden::half();

        assert!(
            box_.max.x < garden.x - half.x,
            "the village reaches the garden: {} against {}",
            box_.max.x,
            garden.x - half.x
        );
    }

    /// Spec 0068: nobody on a round walks through anything.
    ///
    /// **Walked rather than measured**, as spec 0067's is at the steading
    /// and for the reason this repo keeps relearning: a stop that is clear
    /// and the stop after it that is clear say nothing about the cabin
    /// between them.
    #[test]
    fn nobody_walks_through_anything() {
        let solid = solid(REACHES);
        let body = vec3(
            crate::RADIUS * 2.0,
            crate::people::TALL,
            crate::RADIUS * 2.0,
        );

        for (n, round) in rounds(REACHES).into_iter().enumerate() {
            let whole: f32 = round
                .iter()
                .enumerate()
                .map(|(i, stop)| {
                    stop.stays
                        + stop.at.distance(round[(i + 1) % round.len()].at) / crate::people::PACE
                })
                .sum();
            let steps = (whole / 0.15).ceil() as usize;

            for step in 0..steps {
                let doing = crate::people::doing(&round, whole * step as f32 / steps as f32);
                let stands =
                    Aabb::from_center_size(doing.at + Vec3::Y * crate::people::TALL * 0.5, body);

                for one in solid.iter() {
                    assert!(
                        !stands.intersects(one),
                        "round {} is inside {:?} to {:?} at {:?}",
                        n,
                        one.min,
                        one.max,
                        doing.at
                    );
                }
            }
        }
    }

    /// Spec 0068: you can walk in from the garden and out again.
    ///
    /// **With the game's own `walk`**, because the baths' steps had four
    /// legal rises and were one solid block. A village you can see and
    /// cannot get into is scenery.
    #[test]
    fn you_can_walk_into_it() {
        let mut solid = solid(REACHES);
        solid.push(crate::field::ground(at(REACHES)));

        let middle = at(REACHES);
        let from = vec3(middle.x + STREET_LONG * 0.5 + 6.0, 1.0, middle.z);
        let to = vec3(middle.x - 10.0, 0.0, middle.z);
        let (mut body, mut falling) = (from, 0.0);

        // walked at the game's own step, dropped in from above rather than
        // placed, because a point on the ground can be inside the furniture
        for _ in 0..1_200 {
            let way = (to - body) * vec3(1.0, 0.0, 1.0);
            let wish = if way.length() < 1.0 {
                Vec3::ZERO
            } else {
                way.normalize() * crate::SPEED
            };

            (body, falling) =
                crate::walk::walk(body, wish, falling, crate::RADIUS, 1.0 / 60.0, &solid, true);
        }

        assert!(
            (body - to).length() < 2.5,
            "walking in from the east stopped at {:?}, short of {:?}",
            body,
            to
        );
    }

    /// Spec 0068: no tree grows inside it.
    #[test]
    fn nothing_is_sown_in_the_village() {
        let box_ = around(REACHES);

        for n in 0..40 {
            for m in 0..40 {
                let at = vec3(
                    box_.min.x + (box_.max.x - box_.min.x) * n as f32 / 39.0,
                    0.0,
                    box_.min.z + (box_.max.z - box_.min.z) * m as f32 / 39.0,
                );

                assert!(shades(REACHES, at), "a tree may grow at {:?}", at);
            }
        }

        // and the far side of the field is not shaded, or this says nothing
        assert!(!shades(REACHES, vec3(box_.min.x - 60.0, 0.0, box_.min.z)));
    }
}
