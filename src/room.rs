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

/// What the middle of the screen is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    Cabinet(usize),
    Display(usize),
}

pub struct Room {
    pub stood: Vec<Stood>,
    /// The engine's own shapes along the far wall, per spec 0002. Where they
    /// stand, not what they are made of.
    pub displays: Vec<crate::display::Display>,
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
        let reaches = along * 0.5 + APART;

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
        let walls = vec![
            // the two long walls the cabinets back onto
            Aabb::from_center_size(
                vec3(WALL + CABINET.x, TALL * 0.5, 0.0),
                vec3(thick, TALL, reaches * 2.0),
            ),
            Aabb::from_center_size(
                vec3(-WALL - CABINET.x, TALL * 0.5, 0.0),
                vec3(thick, TALL, reaches * 2.0),
            ),
            // and the ends
            Aabb::from_center_size(
                vec3(0.0, TALL * 0.5, reaches),
                vec3((WALL + CABINET.x) * 2.0, TALL, thick),
            ),
            Aabb::from_center_size(
                vec3(0.0, TALL * 0.5, -reaches),
                vec3((WALL + CABINET.x) * 2.0, TALL, thick),
            ),
        ];

        Self {
            stood,
            displays: crate::display::all_of_them(-reaches),
            walls,
            reaches,
        }
    }

    /// Everything you cannot walk through: the walls and the cabinets.
    pub fn solid(&self) -> Vec<Aabb> {
        let mut out = self.walls.clone();
        out.extend(
            self.stood
                .iter()
                .map(|stood| Aabb::from_center_size(stood.at + Vec3::Y * CABINET.y * 0.5, CABINET)),
        );

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

        cabinets
            .chain(shapes)
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
        assert_eq!(solid.len(), room.walls.len() + 12);

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
}
