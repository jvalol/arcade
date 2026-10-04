//! What is on the far wall: the engine's own shapes, turning. Spec 0002.
//!
//! Built from `blitzkit-shapes`, which is the same library the examples build
//! from, so these are the objects themselves rather than pictures of them and
//! nothing here can drift from what the examples draw.

use blitzkit::mesh::MeshData;
use glam::{vec3, Quat, Vec3};

/// How high a shape's middle sits and how fast it turns.
pub const HIGH: f32 = 1.35;
pub const TURNS: f32 = 0.35;

/// How far apart they stand along the wall, and how far off it.
///
/// This is what holds the row's width. The cabinets at the end of the rows
/// stand 1.65 either side of the middle, so a row any wider than three and a
/// bit puts its outer two behind them and the wall looks like three things
/// rather than five.
pub const APART: f32 = 0.72;
pub const OFF_THE_WALL: f32 = 0.75;

/// How wide each one is drawn, across its own widest way.
///
/// A shape turning about its middle sweeps a ball as wide as its box's space
/// diagonal, so what has to fit between two of them is the sweep rather than
/// the width. A step divided by that is the largest that never reaches its
/// neighbour.
pub const ACROSS: f32 = APART / SWEEP;

/// How much wider than itself a shape reaches while it is turning.
///
/// Root three, the space diagonal of its box. It was root two, the horizontal
/// one, which was right while spec 0002 turned everything about the upright
/// and only that. Spec 0004 turns one about every axis, and at root two the
/// Sierpinski tetrahedron tipped on its side reached into the Menger sponge
/// beside it and down through its own plinth.
///
/// The row cannot widen to pay for it. Five steps of 0.88 is 4.03 across and
/// the clear span between the end cabinets is 3.3, measured by standing in the
/// doorway and looking: the teapot and the hilbert tube went behind them. So
/// the shapes come down from 0.509 to 0.416 instead, which is still three
/// times the Klein bottle's old size and a little over half again the
/// teapot's.
pub const SWEEP: f32 = 1.7320508;

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
const SHAPES: [&str; 5] = ["teapot", "klein", "sierpinski", "menger", "hilbert"];

/// Everything the engine's shape library carries, in the order it is laid out.
///
/// The depths and step counts are the ones the examples open at. Deeper is
/// prettier and slower, and a room with five of them turning at once pays for
/// all of them every frame.
/// How big each one is drawn is measured off its own mesh rather than written
/// down. Five numbers picked by eye drew the Klein bottle at 0.136 and the
/// hilbert tube at 0.522, a quarter of each other, while the library hands all
/// five back within a twentieth of the same size already.
pub fn all_of_them(wall: f32) -> Vec<Display> {
    let along = (SHAPES.len() - 1) as f32 * APART;

    SHAPES
        .iter()
        .zip(meshes())
        .enumerate()
        .map(|(n, (name, mesh))| Display {
            name,
            at: vec3(n as f32 * APART - along * 0.5, HIGH, wall + OFF_THE_WALL),
            scale: ACROSS / mesh.bounds().size().max_element().max(1e-4),
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

/// Where a shape's lowest point is, which is where its plinth has to reach.
///
/// All five are drawn the same size, so this is one number rather than one
/// each.
pub fn stands_at() -> f32 {
    HIGH - ACROSS * 0.5
}

/// The plinth under a shape: where its middle is and how big it is.
///
/// It used to be written down in the drawing code as a box 1.35 high ending at
/// 0.75, which left a shape floating a third of a unit over it. Nothing showed
/// that until the sun's shadow map was fitted to the room and the shadow on the
/// plinth's top landed clear of the thing casting it.
///
/// A little wider than the shape it carries, so a turning one does not hang
/// over the edge.
pub fn plinth_under(one: &Display) -> (Vec3, Vec3) {
    let top = stands_at();
    let wide = ACROSS * 1.15;

    (vec3(one.at.x, top * 0.5, one.at.z), vec3(wide, top, wide))
}

/// How far round the row has turned by now. All of them together, because five
/// things turning out of step with each other reads as five things rather than
/// as a row.
pub fn turned(seconds: f32) -> f32 {
    seconds * TURNS
}

/// How far a shape turns for each pixel the mouse moves, per spec 0004.
///
/// Faster than looking around, which is 0.0022. A shape is at arm's length and
/// you are turning it to see the far side, so a flick should get you there.
pub const TURN_BY: f32 = 0.008;

/// Which way each shape is facing: the slow turn they share, and whatever a
/// hand has turned one of them to.
///
/// Spec 0004. All of it is arithmetic, so taking hold of one, turning it and
/// letting go are checked without a window.
///
/// A whole orientation rather than one angle about the upright. It was an
/// angle, and a Klein bottle turned only about its own axis never shows you
/// the handle going in through the wall, which is the one thing worth seeing.
#[derive(Debug, Clone, Default)]
pub struct Spin {
    /// Where a hand has turned each one, which is nowhere until someone does.
    by_hand: Vec<Quat>,
    /// Which one is in your hands and the way it is frozen facing.
    held: Option<(usize, Quat)>,
}

impl Spin {
    pub fn of(count: usize) -> Self {
        Self {
            by_hand: vec![Quat::IDENTITY; count],
            held: None,
        }
    }

    /// Which one is in your hands, if any.
    pub fn holding(&self) -> Option<usize> {
        self.held.map(|(n, _)| n)
    }

    /// Which way one is facing now.
    ///
    /// The row's own turn about the upright, applied over wherever a hand left
    /// this one, so a shape you have tilted goes on turning tilted rather than
    /// righting itself.
    pub fn facing(&self, n: usize, seconds: f32) -> Quat {
        match self.held {
            Some((held, at)) if held == n => at,
            _ => {
                Quat::from_rotation_y(turned(seconds))
                    * self.by_hand.get(n).copied().unwrap_or(Quat::IDENTITY)
            }
        }
    }

    /// Take hold of one, facing the way it already is, so nothing jumps.
    ///
    /// Lets go of whatever was in your hands first. Nothing in the room can
    /// reach a second one without letting go of the first, but a type that
    /// threw away the way you had just turned one is a trap waiting for the
    /// room to change.
    pub fn take(&mut self, n: usize, seconds: f32) {
        if n < self.by_hand.len() {
            self.let_go(seconds);
            self.held = Some((n, self.facing(n, seconds)));
        }
    }

    /// Turn the one in your hands, about whichever axes the hand asked for.
    /// Nothing in your hands, nothing turns.
    ///
    /// Applied over what it was already facing rather than under it, so the
    /// axes are the ones on the screen in front of you however far it has
    /// already been turned.
    pub fn turn(&mut self, by: Quat) {
        if let Some((_, at)) = self.held.as_mut() {
            *at = (by * *at).normalize();
        }
    }

    /// How far a drag turns a shape: across the screen about the upright, up
    /// and down about whichever way is right on the screen.
    ///
    /// The camera's right rather than a fixed axis, so turning one stays the
    /// way it looks whichever side of the aisle you are standing on.
    pub fn drag(delta: glam::Vec2, right: Vec3) -> Quat {
        Quat::from_axis_angle(Vec3::Y, delta.x * TURN_BY)
            * Quat::from_axis_angle(right.normalize_or(Vec3::X), delta.y * TURN_BY)
    }

    /// Let go, leaving it where you left it rather than where it would have
    /// been. It carries on from there.
    pub fn let_go(&mut self, seconds: f32) {
        if let Some((n, at)) = self.held.take() {
            if let Some(by_hand) = self.by_hand.get_mut(n) {
                *by_hand = (Quat::from_rotation_y(turned(seconds)).inverse() * at).normalize();
            }
        }
    }
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

    /// Spec 0002: all five come out the same size, whatever their own
    /// arithmetic gave them.
    ///
    /// Five scales picked by eye drew the Klein bottle at 0.136 and the
    /// hilbert tube at 0.522, and nothing failed.
    #[test]
    fn they_are_all_the_same_size() {
        for (one, mesh) in all_of_them(-6.0).iter().zip(meshes()) {
            let drawn = mesh.bounds().size().max_element() * one.scale;

            assert!(
                (drawn - ACROSS).abs() < 1e-3,
                "{} is drawn {} across where the others are {}",
                one.name,
                drawn,
                ACROSS
            );
        }
    }

    /// Spec 0002: and a turning one never reaches the one beside it.
    ///
    /// The sweep rather than the width, since a shape turning about its middle
    /// covers a circle as wide as its diagonal. Checked against the gap
    /// because that is what it has to fit in.
    #[test]
    fn a_turning_one_keeps_to_itself() {
        let sweep = ACROSS * SWEEP;

        assert!(
            sweep <= APART,
            "a shape sweeps {} across a gap of {}",
            sweep,
            APART
        );
    }

    /// Spec 0002: a shape sits on its plinth rather than over it.
    #[test]
    fn each_one_sits_on_its_plinth() {
        for (one, mesh) in all_of_them(-6.0).iter().zip(meshes()) {
            let (at, size) = plinth_under(one);
            let top = at.y + size.y * 0.5;
            let bottom = one.at.y - mesh.bounds().size().max_element() * one.scale * 0.5;

            assert!(
                (top - bottom).abs() < 1e-3,
                "{} stands at {} over a plinth reaching {}",
                one.name,
                bottom,
                top
            );
            assert!(
                at.y - size.y * 0.5 > -1e-3,
                "{}'s plinth goes through the floor",
                one.name
            );
        }
    }

    /// Spec 0002: and the plinth is wider than what it carries, so a turning
    /// shape does not hang over the edge.
    #[test]
    fn a_plinth_is_wider_than_its_shape() {
        for one in all_of_them(-6.0).iter() {
            let (_, size) = plinth_under(one);

            assert!(
                size.x > ACROSS && size.z > ACROSS,
                "{}'s plinth is {} across under a shape {} across",
                one.name,
                size.x,
                ACROSS
            );
            assert!(
                size.x < APART,
                "{}'s plinth is {} across in a gap of {}",
                one.name,
                size.x,
                APART
            );
        }
    }

    /// How far apart two ways of facing are, in radians.
    fn apart(one: Quat, other: Quat) -> f32 {
        one.angle_between(other)
    }

    /// A drag of the mouse, with the camera looking down the room.
    fn drag(x: f32, y: f32) -> Quat {
        Spin::drag(glam::vec2(x, y), Vec3::X)
    }

    /// Spec 0004: taking hold of one and letting go leaves it where it was.
    #[test]
    fn letting_go_does_not_jump() {
        let mut spin = Spin::of(5);
        let was = spin.facing(2, 3.0);

        spin.take(2, 3.0);
        assert!(
            apart(spin.facing(2, 3.0), was) < 1e-4,
            "it jumped on taking"
        );

        spin.let_go(3.0);
        assert!(
            apart(spin.facing(2, 3.0), was) < 1e-4,
            "it jumped on letting go"
        );
    }

    /// Spec 0004: a held one does not turn on its own.
    #[test]
    fn a_held_one_holds_still() {
        let mut spin = Spin::of(5);
        spin.take(1, 2.0);
        let at = spin.facing(1, 2.0);

        for later in [2.5, 4.0, 10.0] {
            assert!(
                apart(spin.facing(1, later), at) < 1e-4,
                "it drifted by {}",
                later
            );
        }
    }

    /// Spec 0004: a free one does.
    #[test]
    fn a_free_one_turns() {
        let spin = Spin::of(5);

        assert!(
            apart(spin.facing(0, 2.0), spin.facing(0, 1.0)) > 1e-3,
            "it stopped"
        );
    }

    /// Spec 0004: a shape turns about every axis, not only the upright.
    ///
    /// A Klein bottle turned only about its own axis never shows you the
    /// handle going in through the wall, which is the one thing worth seeing.
    #[test]
    fn it_turns_in_every_direction() {
        let mut spin = Spin::of(5);
        spin.take(0, 0.0);

        // dragging across takes the upright with it and leaves it upright
        spin.turn(drag(100.0, 0.0));
        let across = spin.facing(0, 0.0);
        assert!(
            (across * Vec3::Y).dot(Vec3::Y) > 0.999,
            "a drag across the screen tipped it: up is now {:?}",
            across * Vec3::Y
        );
        assert!(
            (across * Vec3::Z).dot(Vec3::Z) < 0.999,
            "a drag across the screen turned nothing"
        );

        // and dragging up or down tips it
        spin.turn(drag(0.0, 100.0));
        let tipped = spin.facing(0, 0.0);
        assert!(
            (tipped * Vec3::Y).dot(Vec3::Y) < 0.9,
            "a drag down the screen did not tip it"
        );
    }

    /// Spec 0004: turning moves the one in your hands and no other.
    #[test]
    fn turning_moves_only_the_held_one() {
        let mut spin = Spin::of(5);
        let others: Vec<Quat> = (0..5).map(|n| spin.facing(n, 1.0)).collect();

        spin.take(3, 1.0);
        spin.turn(drag(80.0, 40.0));

        assert!(
            apart(spin.facing(3, 1.0), others[3]) > 1e-3,
            "the held one did not turn"
        );
        for n in [0, 1, 2, 4] {
            assert!(
                apart(spin.facing(n, 1.0), others[n]) < 1e-4,
                "{} moved as well",
                n
            );
        }
    }

    /// Spec 0004: letting go carries on from where you left it.
    #[test]
    fn it_carries_on_from_where_you_left_it() {
        let mut spin = Spin::of(5);
        spin.take(0, 1.0);
        spin.turn(drag(120.0, 60.0));
        let left_at = spin.facing(0, 1.0);
        spin.let_go(1.0);

        assert!(
            apart(spin.facing(0, 1.0), left_at) < 1e-4,
            "it moved the moment you let go"
        );
        let on = apart(spin.facing(0, 2.0), left_at);
        assert!(
            (on - TURNS).abs() < 1e-3,
            "a second later it had gone {} rather than {}",
            on,
            TURNS
        );
    }

    /// Spec 0004: and the tilt you gave it survives the turning.
    #[test]
    fn a_tilted_one_goes_on_turning_tilted() {
        let mut spin = Spin::of(5);
        spin.take(0, 1.0);
        spin.turn(drag(0.0, 100.0));
        let tipped = spin.facing(0, 1.0) * Vec3::Y;
        spin.let_go(1.0);

        // the row turns about the upright, so a tilt stays as far off upright
        // as it was however long it goes on turning
        for later in [1.5, 3.0, 7.0] {
            let up = spin.facing(0, later) * Vec3::Y;
            assert!(
                (up.y - tipped.y).abs() < 1e-3,
                "at {} it had righted itself to {:?}",
                later,
                up
            );
        }
    }

    /// Spec 0004: nothing can be turned while nothing is held.
    #[test]
    fn turning_nothing_turns_nothing() {
        let mut spin = Spin::of(5);
        let before: Vec<Quat> = (0..5).map(|n| spin.facing(n, 1.0)).collect();

        spin.turn(drag(200.0, 200.0));

        for (n, was) in before.iter().enumerate() {
            assert!(apart(spin.facing(n, 1.0), *was) < 1e-4, "{} moved", n);
        }
    }

    /// Spec 0004: only one is held at a time.
    #[test]
    fn only_one_is_held() {
        let mut spin = Spin::of(5);

        assert_eq!(spin.holding(), None);
        spin.take(1, 1.0);
        assert_eq!(spin.holding(), Some(1));
        spin.turn(drag(70.0, 0.0));
        let first = spin.facing(1, 1.0);

        spin.take(4, 1.0);
        assert_eq!(spin.holding(), Some(4), "the second did not take over");
        assert!(
            apart(spin.facing(1, 1.0), first) < 1e-4,
            "taking the second threw away where the first was turned to"
        );

        spin.let_go(1.0);
        assert_eq!(spin.holding(), None);
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
