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

/// How close you have to be for a cabinet to be the one you are at, and how
/// nearly you have to be facing it.
///
/// Less than the room is wide, which is the whole of it. The cabinets stand
/// `WALL` from the middle, so anything more generous than that makes one across
/// the room yours while you are standing in the aisle, and the room stops being
/// something you walk through.
pub const WITHIN: f32 = WALL - 0.4;
pub const FACING: f32 = 0.35;

/// The screen itself: one quad facing +x, so the screenshot sits on it the way
/// it was taken.
///
/// `MeshData::cube` would do for the shape, and it is what the cabinets are
/// made of, but its faces carry the texture whichever way round they please and
/// the pictures came out turned on their sides. A quad with the corners written
/// down is three lines and leaves nothing to find out.
pub fn screen_mesh() -> MeshData {
    let normal = [1.0, 0.0, 0.0];
    // u runs -z to +z. Worked out the other way round first, from which way
    // the room's axes point, and the pictures came out mirrored: every
    // screenshot has a window title bar in it and it was on the wrong side.
    let corners = [
        ([0.0, 0.5, -0.5], [0.0, 0.0]),
        ([0.0, 0.5, 0.5], [1.0, 0.0]),
        ([0.0, -0.5, 0.5], [1.0, 1.0]),
        ([0.0, -0.5, -0.5], [0.0, 1.0]),
    ];

    // both ways round, because half the cabinets are turned to face the other
    // side of the aisle and a one sided quad is an invisible screen on those
    let back = [-1.0, 0.0, 0.0];
    let mut vertices: Vec<Vertex> = corners
        .iter()
        .map(|(at, uv)| Vertex::new(*at, normal, *uv))
        .collect();
    vertices.extend(corners.iter().map(|(at, uv)| Vertex::new(*at, back, *uv)));

    MeshData::new(vertices, vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7])
}

/// A cabinet, where it stands and which way it faces.
#[derive(Debug, Clone)]
pub struct Stood {
    pub cabinet: Cabinet,
    pub at: Vec3,
    /// Which way its screen looks, flat on the floor.
    pub facing: Vec3,
}

pub struct Room {
    pub stood: Vec<Stood>,
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

    /// Which cabinet you are at: the nearest one in front of you, near enough
    /// to reach and nearly enough faced to mean it.
    ///
    /// Both halves matter. Nearest alone picks the one behind you when you walk
    /// past; facing alone picks one across the room.
    pub fn at(&self, you: Vec3, looking: Vec3) -> Option<usize> {
        let flat = vec3(looking.x, 0.0, looking.z).normalize_or_zero();

        self.stood
            .iter()
            .enumerate()
            .filter_map(|(n, stood)| {
                let out = stood.at - vec3(you.x, 0.0, you.z);
                let away = out.length();
                if away > WITHIN || away < 1e-4 {
                    return None;
                }
                if out.normalize().dot(flat) < FACING {
                    return None;
                }

                Some((n, away))
            })
            .min_by(|one, other| one.1.total_cmp(&other.1))
            .map(|(n, _)| n)
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

    /// Spec 0001: the one you are at is the nearest in front of you.
    #[test]
    fn the_nearest_one_in_front_is_the_one() {
        let room = Room::of(some(12));
        let first = room.stood[0].at;

        // standing in front of it, looking at it
        let you = first - vec3(1.4, 0.0, 0.0);
        assert_eq!(room.at(you, vec3(1.0, 0.0, 0.0)), Some(0));

        // the same spot, looking away
        assert_eq!(room.at(you, vec3(-1.0, 0.0, 0.0)), None);

        // and from the middle of the room, nothing is close enough to be yours
        assert_eq!(room.at(Vec3::ZERO, vec3(0.0, 0.0, 1.0)), None);
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
