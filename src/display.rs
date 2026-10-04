//! What is on the far wall: the engine's own shapes, turning. Spec 0002.
//!
//! Built from `blitzkit-shapes`, which is the same library the examples build
//! from, so these are the objects themselves rather than pictures of them and
//! nothing here can drift from what the examples draw.

use blitzkit::mesh::MeshData;
use glam::{vec3, Vec3};

/// How wide a shape is drawn, how high its middle sits, and how fast it turns.
/// Narrow enough that all five are in view at once. The cabinets at the end of
/// the rows stand 1.65 either side of the middle, so a row any wider than three
/// and a bit puts its outer two behind them and the wall looks like three
/// things rather than five.
pub const ACROSS: f32 = 0.62;
pub const HIGH: f32 = 1.35;
pub const TURNS: f32 = 0.35;

/// How far apart they stand along the wall, and how far off it.
pub const APART: f32 = 0.72;
pub const OFF_THE_WALL: f32 = 0.75;

/// A thing on the wall: what it is called, where it stands and how big it is
/// drawn, since the shapes come out at whatever size their own arithmetic gives
/// them.
///
/// The mesh is not in here. Where a thing stands is what the room needs to
/// answer what you are looking at, and that wants checking without a window;
/// the mesh is the renderer's business and is built once beside it.
#[derive(Debug, Clone, Copy)]
pub struct Display {
    pub name: &'static str,
    pub at: Vec3,
    pub scale: f32,
}

/// What each one is, in the order they are laid out along the wall.
const SHAPES: [(&str, f32); 5] = [
    ("teapot", 0.42),
    ("klein", 0.22),
    ("sierpinski", 0.55),
    ("menger", 0.55),
    ("hilbert", 0.9),
];

/// Everything the engine's shape library carries, in the order it is laid out.
///
/// The depths and step counts are the ones the examples open at. Deeper is
/// prettier and slower, and a room with five of them turning at once pays for
/// all of them every frame.
pub fn all_of_them(wall: f32) -> Vec<Display> {
    let along = (SHAPES.len() - 1) as f32 * APART;

    SHAPES
        .iter()
        .enumerate()
        .map(|(n, (name, scale))| Display {
            name,
            at: vec3(n as f32 * APART - along * 0.5, HIGH, wall + OFF_THE_WALL),
            scale: scale * ACROSS,
        })
        .collect()
}

/// The meshes, in the same order.
///
/// The depths and step counts are the ones the examples open at. Deeper is
/// prettier and slower, and a room with five of them turning at once pays for
/// all of them every frame.
pub fn meshes() -> Vec<MeshData> {
    vec![
        blitzkit_shapes::teapot::teapot(6),
        blitzkit_shapes::klein::klein_bottle(48, 24),
        blitzkit_shapes::sierpinski::sierpinski(4),
        blitzkit_shapes::menger::menger(2),
        blitzkit_shapes::hilbert::hilbert_tube(3, 8, 0.03),
    ]
}

/// How far round one has turned by now. All of them together, because five
/// things turning out of step with each other reads as five things rather than
/// as a row.
pub fn turned(seconds: f32) -> f32 {
    seconds * TURNS
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0002: there is a display for each shape the library carries.
    #[test]
    fn every_shape_is_on_the_wall() {
        let on = all_of_them(-6.0);
        let names: Vec<&str> = on.iter().map(|one| one.name).collect();

        assert_eq!(
            names,
            ["teapot", "klein", "sierpinski", "menger", "hilbert"],
            "the wall carries {:?}",
            names
        );
        let built = meshes();
        assert_eq!(
            built.len(),
            on.len(),
            "a shape with no mesh or the other way"
        );
        for (one, mesh) in on.iter().zip(&built) {
            assert!(
                !mesh.vertices.is_empty(),
                "{} came out with no mesh at all",
                one.name
            );
            assert!(one.scale > 0.0);
        }
    }

    /// Spec 0002: and they stand clear of each other.
    #[test]
    fn they_stand_clear() {
        let on = all_of_them(-6.0);

        for (n, one) in on.iter().enumerate() {
            assert!(
                (one.at.z + 6.0 - OFF_THE_WALL).abs() < 1e-4,
                "{} is not off the wall",
                one.name
            );

            for other in &on[n + 1..] {
                let apart = one.at.distance(other.at);
                assert!(
                    apart > ACROSS,
                    "{} and {} are {} apart and are drawn {} across",
                    one.name,
                    other.name,
                    apart,
                    ACROSS
                );
            }
        }
    }

    /// Spec 0002: each one turns.
    #[test]
    fn they_turn() {
        assert_eq!(turned(0.0), 0.0);
        assert!(turned(1.0) > 0.0);
        assert!(turned(2.0) > turned(1.0), "it stopped");

        // and goes right round in a reasonable while rather than a minute
        let round = std::f32::consts::TAU / TURNS;
        assert!(
            (6.0..40.0).contains(&round),
            "a turn takes {} seconds",
            round
        );
    }
}
