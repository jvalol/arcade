//! The room, the cabinets in it, and where you are standing. Spec 0001.
//!
//! Nothing here draws or reads a key. The shape of the room and which cabinet
//! you are at are arithmetic, so all of it can be checked without a window.

use crate::cabinet::Cabinet;
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

/// How far from the nook's back wall a bench stands, and how much floor is
/// left between two of them.
pub const BENCH_OFF: f32 = 1.1;
pub const BENCH_GAP: f32 = 0.3;

/// The nook the toys live in: how far it cuts in behind the left wall and how
/// much of that wall it runs along.
///
/// A room off the room, not a table in it. Standing the benches in the aisle
/// put them in front of the plinths from every angle, which is the one place
/// nothing should stand.
pub const NOOK_DEEP: f32 = 3.0;
pub const NOOK_SPAN: f32 = 6.8;

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

/// How far the room's walls are from its middle, and how high.
pub const WALL: f32 = 2.0;
pub const TALL: f32 = 3.2;

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
    let normal = [0.0, 1.0, 0.0];
    let corners = [
        ([-0.5, 0.0, 0.5], [0.0, tiles]),
        ([0.5, 0.0, 0.5], [tiles, tiles]),
        ([0.5, 0.0, -0.5], [tiles, 0.0]),
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
}

/// Where the benches stand: a row along the nook's back wall, centred in it,
/// in the order you meet them coming through the way in.
///
/// The ball and chain first. It is the reason there is a nook: it is worked
/// rather than watched, so it is not one of the shapes, and there is nothing to
/// win, so it is not one of the games. The room had nowhere to put it and that
/// is what this is.
pub fn benches(far: f32) -> Vec<Benched> {
    /// What is on them, and how much of the nook's length each one takes.
    const ON_THEM: [(&str, f32); 4] = [
        ("ball and chain", 1.4),
        ("gyroscope", 0.9),
        ("newton's cradle", 2.1),
        ("metronome", 0.9),
    ];

    let back = -(WALL + CABINET.x) - NOOK_DEEP + BENCH_OFF;
    let row: f32 =
        ON_THEM.iter().map(|(_, long)| long).sum::<f32>() + (ON_THEM.len() - 1) as f32 * BENCH_GAP;
    let mut z = far + NOOK_SPAN * 0.5 - row * 0.5;

    ON_THEM
        .iter()
        .map(|(name, long)| {
            let at = vec3(back, 0.0, z + long * 0.5);
            z += long + BENCH_GAP;

            Benched {
                name,
                at,
                size: vec3(BENCH_WIDE, BENCH_TALL, *long),
            }
        })
        .collect()
}

/// What the middle of the screen is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    Cabinet(usize),
    Display(usize),
    Bench(usize),
}

pub struct Room {
    pub stood: Vec<Stood>,
    /// The engine's own shapes along the far wall, per spec 0002. Where they
    /// stand, not what they are made of.
    pub displays: Vec<crate::display::Display>,
    /// The benches in front of that wall, per spec 0006.
    pub benches: Vec<Benched>,
    pub walls: Vec<Aabb>,
    /// How far the room reaches from its middle, worked out from how many
    /// cabinets there are.
    pub reaches: f32,
}

impl Room {
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

        let thick = 0.3;
        let side = WALL + CABINET.x;
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
            // the nook: its back, and the end that is not the way in
            Aabb::from_center_size(
                vec3(-side - NOOK_DEEP, TALL * 0.5, -reaches + NOOK_SPAN * 0.5),
                vec3(thick, TALL, NOOK_SPAN),
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

        Self {
            stood,
            displays: crate::display::all_of_them(-reaches),
            benches: benches(-reaches),
            walls,
            reaches,
        }
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

    /// Everything you cannot walk through: the walls, the cabinets, the benches
    /// and the plinths.
    ///
    /// The plinths were the one thing in the room that let you through. You
    /// could walk into the far wall's row and stand inside a Klein bottle,
    /// which is a thing the room says you may pick up and turn.
    pub fn solid(&self) -> Vec<Aabb> {
        let mut out = self.walls.clone();
        out.extend(
            self.stood
                .iter()
                .map(|stood| Aabb::from_center_size(stood.at + Vec3::Y * CABINET.y * 0.5, CABINET)),
        );
        out.extend(self.benches.iter().map(Self::bench_box));
        out.extend(self.displays.iter().map(Self::plinth_box));

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

        let benches = self.benches.iter().enumerate().filter_map(|(n, one)| {
            slab(from, way, &Self::bench_box(one)).map(|far| (Seen::Bench(n), far))
        });

        cabinets
            .chain(shapes)
            .chain(benches)
            .filter(|(_, far)| *far <= REACH)
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(what, _)| what)
    }

    /// Where you start: the middle of the room, looking down it.
    pub fn doorway(&self) -> Vec3 {
        vec3(0.0, 0.0, self.reaches - 1.0)
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

        // the near one wins when two are in line
        let behind = room
            .stood
            .iter()
            .position(|s| s.at.x < 0.0)
            .expect("a far side");
        let through = room.stood[behind].at + eye - (first + eye);
        assert_eq!(
            cabinet_at(&room, first + eye - through.normalize() * 3.0, through),
            Some(0)
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
        assert_eq!(
            solid.len(),
            room.walls.len() + 12 + room.benches.len() + room.displays.len()
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

    /// Spec 0006: you can walk from where you come in to every bench.
    ///
    /// Flooded rather than reasoned about. Making the plinths solid closed the
    /// only way into the nook and nothing said so: the route runs between the
    /// last cabinet and the nearest plinth, and that gap was 0.74 wide for
    /// someone 0.9 across. Arithmetic about one wall at a time cannot see a
    /// pinch between two things that were never thought about together.
    #[test]
    fn you_can_walk_to_every_bench() {
        const GRID: f32 = 0.12;

        let room = Room::of(some(13));
        let solid = room.solid();
        let radius = crate::RADIUS;
        let free = |at: Vec3| solid.iter().all(|box_| clear_of(at, box_) > radius);

        let low = vec3(-(WALL + CABINET.x) - NOOK_DEEP, 0.0, -room.reaches);
        let wide = ((WALL + CABINET.x) * 2.0 + NOOK_DEEP) / GRID;
        let long = room.reaches * 2.0 / GRID;
        let (wide, long) = (wide as usize + 1, long as usize + 1);
        let cell = |x: usize, z: usize| low + vec3(x as f32 * GRID, radius, z as f32 * GRID);

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

        for bench in &room.benches {
            // the floor in front of it, on the side you stand to work it
            let at = bench.at + Vec3::X * (bench.size.x * 0.5 + radius + 0.1);
            let (x, z) = (
                ((at.x - low.x) / GRID).round() as usize,
                ((at.z - low.z) / GRID).round() as usize,
            );

            assert!(
                free(cell(x, z)),
                "there is no standing at the {}",
                bench.name
            );
            assert!(
                reached[x * long + z],
                "you cannot walk from the door to the {}",
                bench.name
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

    #[test]
    fn the_nook_is_off_the_aisle_and_not_in_it() {
        let room = Room::of(some(12));

        for bench in &room.benches {
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
            room.walls.len() + 13 + room.benches.len() + room.displays.len()
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

        // stood in the nook, looking at it
        let from = bench.at + Vec3::X * 1.4 + Vec3::Y * 1.5;
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
