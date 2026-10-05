//! A gyroscope on a bench. Spec 0006.
//!
//! A rotor in two rings on a pedestal, pinned through its own middle, which is
//! what balanced means: gravity pulls on it with no arm to pull on, so it has no
//! reason to fall and none to walk. Left alone it holds its axis exactly, and
//! measured over twenty seconds it holds it to nought.
//!
//! The demonstration is what happens when you lean on it. Push the end of the
//! spindle and the axis does not go where you pushed. **It goes sideways**, at a
//! right angle to the push, and it keeps going sideways for exactly as long as
//! you keep pushing. Let go and it stops dead.
//!
//! Spin it harder and the same push moves it *slower*. The rate is the torque
//! you are applying over the spin's own momentum, so more spin is a bigger
//! denominator.
//!
//! Stop the rotor and push again and it does the obvious thing instead: it goes
//! where you pushed it, hard, and keeps going, because nothing in a balanced
//! rotor stops it. That is the comparison.
//!
//! None of this is put in by hand. The engine has carried Euler's equation since
//! spec 0034, solved implicitly in the body's own frame, and this is the first
//! thing in the project that shows what that buys: everything else it does is a
//! block toppling, where you cannot tell a right answer from a plausible one.
//!
//! Where it all sits, and how fast it ought to go round, is arithmetic and is
//! checked without a window.

use blitzkit::collision::Aabb;
use blitzkit::link::Link;
use blitzkit::mesh::{MeshData, Vertex};
use blitzkit::physics::{Body, Solver};
use glam::{vec3, Quat, Vec3};

/// What the bench it stands on is called.
pub const NAME: &str = "gyroscope";

/// The wheel: how wide across, how thick, and what it weighs.
pub const RADIUS: f32 = 0.117;
pub const THICK: f32 = 0.05;
pub const MASS: f32 = 0.3;

/// How far the rotor's middle is from where it is pinned.
///
/// Nought, and that is the whole of what balanced means. An earlier build hung
/// it off an arm, where gravity had something to pull on and it walked round the
/// stand on its own. That one is a gyroscope too, and it is a worse object: it
/// is a wheel on a stick rather than the thing in the rings everybody has
/// already seen, and the stand has to stand off to one side of it.
pub const OUT: f32 = 0.0;

/// How high above the bench top the rotor is pinned, and how thick the pedestal
/// and the spindle are drawn.
///
/// High enough that the outer ring clears the bench, which is what decides it.
pub const HUB_UP: f32 = 0.42;
pub const STAND: f32 = 0.03;
pub const AXLE: f32 = 0.012;

/// The ring round the wheel: how far out it stands, how thick it is, and how
/// many pieces it is drawn in.
///
/// Drawn and not weighed, and it is what makes the thing a gyroscope rather than
/// a desk fan. A wheel on a stick with a blade across it is a fan; a wheel
/// inside a ring is the object everybody has seen. It turns with the axle and
/// not with the wheel, which is what a gimbal does.
pub const GIMBAL: f32 = 0.2;
pub const GIMBAL_THICK: f32 = 0.009;
pub const GIMBAL_PIECES: usize = 32;

/// The outer ring, across the gimbal, which the spindle's ends reach.
pub const FRAME: f32 = 0.235;

/// How far out the mark on the rotor sits, and how many pieces the blur it
/// turns into is drawn in.
pub const MARK: f32 = 0.72;
pub const BLUR_PIECES: usize = 24;

/// The boss at the middle of the rotor, where the spindle goes through it.
///
/// A flat disc with a line drawn across it is a slice of lemon, whatever ring
/// you put round it. A thick wheel with a hub at the middle and one short spoke
/// is a flywheel, and the spoke still says it is turning.
pub const BOSS: f32 = 0.034;

/// How hard a held arrow leans on the end of the spindle.
///
/// Chosen so the axis walks at about three fifths of a radian a second in the
/// middle of the dial, which is slow enough to watch and fast enough to be sure
/// it is happening.
///
/// There is nothing to lean against in a balanced rotor, so the same push that
/// ambles a spun one sideways sends a stopped one end over end at nearly fifty
/// radians a second squared, which is forty times the fastest walk. The gap
/// between the two is the toy.
pub const PUSH: f32 = 0.21;

/// How far the spindle reaches past the rotor on each side: out to the frame,
/// which is what holds it.
///
/// A rotor that fills its ring is a slice of lemon. One a little over half the
/// ring across, on a spindle you can see going through it, inside two rings
/// across each other, is the thing everybody has already seen.
pub const SPINDLE_OUT: f32 = FRAME;

/// How many spins the dial carries, and the slowest and fastest of them.
///
/// The slow end is not slower than this on purpose. A gyroscope only behaves
/// like one while it goes round much more slowly than it spins; below that it
/// flops rather than precessing, and a toy whose point is a steady walk should
/// not have a setting where it wallows.
///
/// The fast end is not faster than this for a reason that has nothing to do with
/// physics. A frame is a sixtieth of a second, and a mark on a rim turning 240
/// radians a second moves 153 degrees between one frame and the next, which is
/// past the angle an eye can tell the direction of. It reads as a wheel standing
/// still. At 60 it moves 57 degrees a frame and reads as a wheel turning.
///
/// Nothing was lost in coming down. The walk is the torque over the spin's
/// momentum, so the push came down with it and the dial walks at exactly the
/// rates it walked at before.
pub const SPINS: usize = 5;
pub const SLOWEST: f32 = 20.0;
pub const FASTEST: f32 = 60.0;

/// Which spin it is found at.
pub const STARTS_AT: usize = 2;

/// The three rods that pin the end of the axle, and how big an anchor is.
///
/// Long, and for the same reason the metronome's are: the whole assembly sweeps
/// a ball of 0.46 about the hub, so anything closer in is something it goes
/// through.
pub const TIE: f32 = 0.75;
pub const PIN: f32 = 0.002;

/// How fast it is stepped, and how many passes a step gets.
///
/// Sixteen times the room's other toys, which is cheap, and being balanced is
/// why. The build that hung the rotor off an arm needed sixty thousand steps a
/// second and 4.5% of a core: turning a body is integrated to first order, and
/// with gravity working on an arm the whole behaviour sits inside that error. It
/// climbed when it should have held, +0.99 of the arm's own length in thirty
/// seconds at 960 a second, and halving the step halved the climb all the way
/// down to nothing at 30,720.
///
/// Balanced there is no arm, so there is no climb to integrate away. Measured at
/// a spin of 240, four times the top of the dial now, by where a two second push
/// left the axis:
///
/// ```text
///    960 a second   0.0299 rad, 99% of it sideways, 0.1% of a core
///  3,840            0.0293       100%                0.2%
/// 15,360            0.0292       100%                0.8%
/// 61,440            0.0290       100%                3.1%
/// ```
///
/// Four thousand a second, which is within a part in three hundred of the
/// finest and costs a fifth of a percent.
///
/// Six passes, because three rods holding one body converge at once: 6, 12 and
/// 24 gave the same answer to three decimals.
pub const STEP: f32 = 1.0 / 3840.0;
pub const PASSES: usize = 6;

/// How hard the gimbal's bearings drag on the axis as it turns.
///
/// Nutation, and almost nothing else. An axis that is pushed and let go does not
/// settle on its new heading, it rings: the wobble is only four tenths of a
/// degree wide and invisible to look at, but it is quick, and the speed it adds
/// is the same size as the walk's own. So the axis does not walk, it lurches,
/// seven to one and back, twelve times a second. That is what a gyroscope with
/// nothing damping it does, and a real one does not do it because its bearings
/// drag.
///
/// The drag is on the axis's turning and not on the spin, and the two are
/// separated by how fast they are rather than by being treated differently: the
/// wobble turns at about twice the spin and the walk at under a radian a second,
/// so a drag that kills the first barely touches the second. Swept, by how
/// unevenly the axis walked over five seconds and what the walk came out at
/// against the arithmetic's 0.6008:
///
/// ```text
/// 0.0000   lurches 23.1 to one   walks 0.7224
/// 0.0010            1.1           0.5987
/// 0.0035            1.0           0.5926
/// 0.0100            1.1           0.5499
/// ```
///
/// A thousandth. It is the least that settles it, and it costs the walk a third
/// of a percent. Note what the lurch does to the measurement as well as to the
/// look: undamped, the axis covers a fifth more ground than it travels, because
/// the wobble is path and not progress.
pub const DRAG: f32 = 0.001;

/// Drags on the axis's own turning for one step.
pub fn settle(bodies: &mut [Body], dt: f32) {
    let wheel = &bodies[WHEEL];
    let along = axle(bodies);
    let across = wheel.spin - along * wheel.spin.dot(along);

    let wheel = &mut bodies[WHEEL];
    wheel.spin += wheel.inverse_inertia() * (-across * DRAG) * dt;
}

/// How long a window the walk is measured over.
pub const SAMPLE: f32 = 0.1;

/// Nothing to collide with. The rods hold the axle and the wheel reaches
/// nothing.
pub const NO_WORLD: [Aabb; 0] = [];

/// Which body is which, in the order [`built`] makes them.
pub const PINS: usize = 3;
pub const WHEEL: usize = PINS;

/// Root three over two, which is where the wheel's half extents come from.
const DISC: f32 = 0.866_025_4;

/// The block that weighs what the drawn disc weighs.
///
/// The engine has spheres and blocks and no discs, and a sphere resists turning
/// the same way about every axis, which is the one thing a gyroscope must not
/// do. So the body is a block, and the drawn wheel is a disc.
///
/// They are not a compromise between each other. A block half as thick as the
/// disc and `√3/2` of its radius across has the disc's inertia exactly, about
/// its axle and across it both: `2mb²/3 = mr²/2` gives `b = √3r/2`, and that
/// same `b` leaves `ma²/3 = mt²/12` wanting `a = t/2`, which is the block's own
/// half thickness. Two equations, two unknowns, no remainder.
pub fn wheel_half() -> Vec3 {
    vec3(THICK * 0.5, RADIUS * DISC, RADIUS * DISC)
}

/// Where the axle is pinned, in the frame whose origin is the middle of the
/// bench top.
pub fn hub() -> Vec3 {
    Vec3::Y * HUB_UP
}

/// How fast the wheel spins at a notch on the dial.
pub fn spin_at(notch: usize) -> f32 {
    let notch = notch.min(SPINS - 1) as f32;

    SLOWEST + notch * (FASTEST - SLOWEST) / (SPINS - 1) as f32
}

/// How hard the rotor resists being turned about its own axle.
pub fn about_the_axle() -> f32 {
    let half = wheel_half();

    MASS / 3.0 * (half.y * half.y + half.z * half.z)
}

/// How fast the axis walks sideways while a push is held, in radians a second.
///
/// The torque you are leaning on it with, over the spin's own momentum. The
/// whole of the toy is in the division: the spin is underneath, so more of it is
/// less of this.
pub fn walks_at(notch: usize) -> f32 {
    PUSH * FRAME / (about_the_axle() * spin_at(notch))
}

/// Leans on the end of the spindle for one step.
///
/// The impulse is the force times the step, so holding the key is a steady push
/// however fast the toy is stepped, which is what lets the walk be a rate rather
/// than a number of presses.
///
/// The torque is applied rather than struck. `Body::strike` is for hitting a
/// thing: it takes a place in the world and pulls it onto the body's own
/// surface, which for a push on the end of a spindle rewrites the lever you
/// meant into whatever lever the block has. Used that way it came out
/// seventeen times too slow and the error was invisible, because any torque
/// across the axis sends a gyroscope sideways and sideways is what you were
/// looking for.
pub fn push(bodies: &mut [Body], way: Vec3, dt: f32) {
    let way = way.normalize_or_zero();
    if way == Vec3::ZERO {
        return;
    }

    let turning = (axle(bodies) * FRAME).cross(way * PUSH);
    let wheel = &mut bodies[WHEEL];
    wheel.spin += wheel.inverse_inertia() * turning * dt;
    wheel.wake();
}

/// The three places the end of the axle is held.
///
/// Three rods along three axes, which is the plainest way to be sure their pulls
/// cover every direction: three that lay in one plane would leave the point free
/// to walk out of it. Pinning it and no more is the point. A gyroscope is a
/// thing free to turn any way it likes about one held point, and anything that
/// also held the axle's bearing would be doing the work the spin is there to do.
fn pins() -> [Vec3; 3] {
    [
        hub() + Vec3::X * TIE,
        hub() + Vec3::Y * TIE,
        hub() + Vec3::Z * TIE,
    ]
}

/// Where the wheel sits with its axle pointing `along`.
pub fn posed(along: Vec3) -> (Vec3, Quat) {
    let along = along.normalize_or(Vec3::X);

    (hub() + along * OUT, Quat::from_rotation_arc(Vec3::X, along))
}

/// Which way the axle points to start with: out at the aisle, so the wheel is
/// face on to somebody standing at the bench.
pub const FIRST: Vec3 = Vec3::X;

/// The bodies and the rods, with the axle level.
pub fn built() -> (Vec<Body>, Vec<Link>) {
    let (at, facing) = posed(FIRST);
    let mut bodies: Vec<Body> = pins()
        .iter()
        .map(|anchor| Body::immovable(*anchor, PIN))
        .collect();
    bodies.push(Body::block(at, wheel_half(), MASS).facing(facing));

    // tied to the end of the axle, which is on the spin's own axis, so spinning
    // the wheel never moves the point the rods are holding
    let on_axle = Vec3::NEG_X * OUT;
    let rods = (0..PINS)
        .map(|n| Link::rod(n, WHEEL, TIE).tied_at(Vec3::ZERO, on_axle))
        .collect();

    (bodies, rods)
}

/// Which way the axle is pointing now.
pub fn axle(bodies: &[Body]) -> Vec3 {
    bodies[WHEEL].orientation * Vec3::X
}

/// Which way the gimbal is facing: along the axle, with the wheel's own spin
/// taken out, since a gimbal carries the wheel rather than turning with it.
pub fn gimbal(bodies: &[Body]) -> Quat {
    Quat::from_rotation_arc(Vec3::X, axle(bodies))
}

/// How far round the outer frame has swung, for an axis pointing `along`.
///
/// The frame turns about the upright and about nothing else, because it is
/// bolted to the pedestal. Following the axis instead is what it did first, and
/// then an axis tipped towards the upright carried the whole cage over with it
/// and lifted it off its own stand.
///
/// None when the axis is near enough upright that there is no direction left to
/// face. That is gimbal lock, and a real one has it too; here the frame simply
/// stays where it last was.
pub fn frame_round(along: Vec3) -> Option<f32> {
    let flat = vec3(along.x, 0.0, along.z);

    (flat.length() > 0.08).then(|| (-along.z).atan2(along.x))
}

/// A solver set up the way this wants one.
///
/// It never sleeps. Nothing in a gyroscope is still, and the one thing that
/// looks it from the outside, the axle holding its height, is the thing the
/// whole toy is about.
pub fn solver() -> Solver {
    let mut solver = Solver::new().with_passes(PASSES);
    solver.sleeps = false;

    solver
}

/// The wheel, drawn: a cylinder a unit across and a unit thick about the x
/// axis, so a transform scales it to whatever the body weighs like.
pub fn wheel_mesh(segments: u32) -> MeshData {
    let segments = segments.max(3);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let round = |n: u32| {
        let turn = n as f32 / segments as f32 * std::f32::consts::TAU;
        let (sin, cos) = turn.sin_cos();

        (cos * 0.5, sin * 0.5)
    };

    // the rim
    for n in 0..segments {
        let (y0, z0) = round(n);
        let (y1, z1) = round(n + 1);
        let first = vertices.len() as u32;

        for (x, y, z) in [(-0.5, y0, z0), (0.5, y0, z0), (0.5, y1, z1), (-0.5, y1, z1)] {
            let out = vec3(0.0, y, z).normalize_or(Vec3::Y);
            vertices.push(Vertex::new([x, y, z], out.to_array(), [0.0, 0.0]));
        }
        indices.extend_from_slice(&[first, first + 2, first + 1, first, first + 3, first + 2]);
    }

    // and a face at each end
    for (x, out) in [(-0.5f32, -1.0f32), (0.5, 1.0)] {
        let middle = vertices.len() as u32;
        vertices.push(Vertex::new([x, 0.0, 0.0], [out, 0.0, 0.0], [0.0, 0.0]));

        for n in 0..=segments {
            let (y, z) = round(n);
            vertices.push(Vertex::new([x, y, z], [out, 0.0, 0.0], [0.0, 0.0]));
        }
        for n in 0..segments {
            let (one, other) = (middle + 1 + n, middle + 2 + n);
            if out > 0.0 {
                indices.extend_from_slice(&[middle, one, other]);
            } else {
                indices.extend_from_slice(&[middle, other, one]);
            }
        }
    }

    MeshData::new(vertices, indices)
}

/// The one on the bench.
pub struct Gyro {
    pub bodies: Vec<Body>,
    pub rods: Vec<Link>,
    /// Which spin the dial is set to.
    pub notch: usize,
    pub going: bool,
    /// Which way a held arrow is leaning on the spindle, and nought for none.
    pub leaning: Vec3,
    /// How far round the outer frame is, kept because the axis can reach a
    /// place where it no longer says.
    pub facing: f32,
    /// Its own clock, where the axis was pointing when it was last sampled, how
    /// long ago that was, and how fast it is walking now.
    clock: f32,
    was: Vec3,
    since: f32,
    moving: f32,
    owed: f32,
    pub solver: Solver,
}

impl Default for Gyro {
    fn default() -> Self {
        Self::new()
    }
}

impl Gyro {
    pub fn new() -> Self {
        let (bodies, rods) = built();

        Self {
            bodies,
            rods,
            notch: STARTS_AT,
            going: false,
            leaning: Vec3::ZERO,
            facing: 0.0,
            clock: 0.0,
            was: Vec3::X,
            since: 0.0,
            moving: 0.0,
            owed: 0.0,
            solver: solver(),
        }
    }

    /// Spins it up, or stops it and lets it fall.
    pub fn press(&mut self) {
        if self.going {
            self.stop();
        } else {
            self.set_going();
        }
    }

    /// Stands the axle level again and spins the wheel.
    pub fn set_going(&mut self) {
        self.place(FIRST);
        self.bodies[WHEEL].spin = axle(&self.bodies) * spin_at(self.notch);
        self.going = true;
    }

    /// Squares it up and takes the spin out, which is the other half of the
    /// demonstration: the same push does something else entirely to a stopped
    /// one.
    pub fn stop(&mut self) {
        self.place(FIRST);
        self.going = false;
    }

    /// Puts the axle along a direction and takes all the speed out of it.
    fn place(&mut self, along: Vec3) {
        let (at, facing) = posed(along);

        self.bodies[WHEEL].position = at;
        self.bodies[WHEEL].orientation = facing;
        self.bodies[WHEEL].velocity = Vec3::ZERO;
        self.bodies[WHEEL].spin = Vec3::ZERO;
        self.bodies[WHEEL].wake();
        self.clock = 0.0;
        self.moving = 0.0;
        self.since = 0.0;
        self.was = axle(&self.bodies);
    }

    /// Moves the dial, which spins it up again at the new setting.
    pub fn turn_the_dial(&mut self, by: i32) {
        let want = (self.notch as i32 + by).clamp(0, SPINS as i32 - 1) as usize;
        if want == self.notch {
            return;
        }
        self.notch = want;

        if self.going {
            self.set_going();
        }
    }

    /// Steps it at its own rate, counting how far round it has walked.
    pub fn advance(&mut self, dt: f32) {
        self.owed = (self.owed + dt).min(0.2);

        while self.owed >= STEP {
            push(&mut self.bodies, self.leaning, STEP);
            self.solver.step_linked(
                &mut self.bodies,
                &self.rods,
                &NO_WORLD,
                crate::GRAVITY,
                STEP,
            );
            self.owed -= STEP;
            self.clock += STEP;

            settle(&mut self.bodies, STEP);

            if let Some(round) = frame_round(axle(&self.bodies)) {
                self.facing = round;
            }

            // how fast the axis is walking, over a window rather than over a
            // step.
            //
            // Over a step it reads the wobble instead. A pushed gyroscope keeps
            // nutating after the push stops, at about twice its own spin, and
            // although that wobble is two thousandths of a radian wide and
            // invisible to look at, two thousandths at three hundred a second is
            // a large speed. Measured a step at a time the bench said it was
            // still moving half a radian a second for ten seconds after it had
            // in fact stopped walking, which reads as the spin running out.
            //
            // A tenth of a second holds several wobbles, so they cancel and what
            // is left is the walk. The chord and not the angle, because a tenth
            // of a second of walking is six hundredths of a radian, whose cosine
            // an f32 holds to two digits.
            self.since += STEP;
            if self.since >= SAMPLE {
                let now = axle(&self.bodies);
                let by = (now - self.was).length() / self.since;

                self.moving = self.moving * 0.4 + by * 0.6;
                self.was = now;
                self.since = 0.0;
            }
        }
    }

    /// How fast the axis is moving now, counted rather than worked out.
    pub fn moving_at(&self) -> f32 {
        self.moving
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How fast the same push turns a stopped one, a second in.
    fn tumbles() -> f32 {
        let half = wheel_half();
        let across = MASS / 3.0 * (half.x * half.x + half.z * half.z);

        PUSH * FRAME / across
    }

    /// Spins one up and leans on the end of its spindle for a while. Gives back
    /// how far the axis moved, how much of that was sideways, and how fast it
    /// was moving at the end.
    fn leaned_on(notch: usize, spun: bool, way: Vec3, seconds: f32) -> (f32, f32, f32) {
        let mut one = Gyro::new();
        one.turn_the_dial(notch as i32 - STARTS_AT as i32);
        if spun {
            one.set_going();
        } else {
            one.stop();
        }

        let first = axle(&one.bodies);
        one.leaning = way;

        // sideways means across both the axis and the push, which is the one
        // direction nobody expects it to go. Taken over the first tenth of a
        // second, because the axis keeps turning and a quarter of a circle later
        // "the way it set off" is not a direction any more.
        let mut sideways = 0.0;
        for step in 0..(seconds / STEP) as u32 {
            one.advance(STEP);

            if step == (0.1 / STEP) as u32 {
                let across = first.cross(way).normalize_or_zero();
                sideways = (axle(&one.bodies) - first)
                    .normalize_or_zero()
                    .dot(across)
                    .abs();
            }
        }
        let now = axle(&one.bodies);

        (
            now.dot(first).clamp(-1.0, 1.0).acos(),
            sideways,
            one.moving_at(),
        )
    }

    /// Spec 0006: the block weighs what the drawn disc weighs, exactly.
    ///
    /// The engine has no disc, and a sphere resists turning the same way about
    /// every axis, which is the one thing a gyroscope must not do. So the body is
    /// a block, and this is what says the swap costs nothing.
    #[test]
    fn the_block_weighs_what_the_disc_weighs() {
        use blitzkit::physics::Shape;

        let block = Shape::Block { half: wheel_half() }.inertia(MASS);
        let about = MASS * RADIUS * RADIUS * 0.5;
        let across = MASS * (3.0 * RADIUS * RADIUS + THICK * THICK) / 12.0;

        assert!(
            (block.x - about).abs() < 1e-9,
            "about the axle the block is {} and a disc is {}",
            block.x,
            about
        );
        for side in [block.y, block.z] {
            assert!(
                (side - across).abs() < 1e-9,
                "across the axle the block is {} and a disc is {}",
                side,
                across
            );
        }

        // and it is the shape that makes the toy: a wheel resists turning about
        // its axle about twice as hard as across it, and that ratio is the whole
        // of why a push goes sideways
        assert!(
            block.x > block.y * 1.5,
            "the rotor is {} about its axle against {} across it",
            block.x,
            block.y
        );
        assert!(
            (about - about_the_axle()).abs() < 1e-9,
            "the arithmetic and the body disagree about the rotor"
        );
    }

    /// Spec 0006: it is pinned through its own middle, which is what balanced
    /// means.
    #[test]
    fn it_is_pinned_through_its_middle() {
        let (bodies, rods) = built();

        assert_eq!(bodies.len(), PINS + 1);
        assert_eq!(rods.len(), PINS);
        assert_eq!(OUT, 0.0, "it is hung off an arm, so gravity has a lever");

        for (n, rod) in rods.iter().enumerate() {
            assert!(
                rod.error(&bodies).abs() < 1e-5,
                "rod {} is built {} out",
                n,
                rod.error(&bodies)
            );
            assert!(rod.rigid, "rod {} only pulls", n);
            assert_eq!(bodies[rod.one].inverse_mass, 0.0, "anchor {} can move", n);
            assert_eq!(
                rod.at_other,
                Vec3::ZERO,
                "rod {} is tied somewhere other than the middle, so it can twist it",
                n
            );
        }
    }

    /// Spec 0006: and nothing stands where the rings sweep.
    #[test]
    fn the_anchors_are_out_of_the_way() {
        let (bodies, _) = built();

        for (pin, anchor) in bodies[..PINS].iter().enumerate() {
            let off = anchor.position.distance(hub());
            assert!(
                off > FRAME + PIN,
                "anchor {} is {} from the middle and the rings reach {}",
                pin,
                off,
                FRAME
            );
        }
    }

    /// Spec 0006: left alone it holds its axis, and holds it exactly.
    ///
    /// Spun or not. Balanced means gravity has no arm to pull on, so there is
    /// nothing to turn it and nothing for the solver to get wrong.
    #[test]
    fn left_alone_it_holds_its_axis() {
        for spun in [false, true] {
            let (moved, _, _) = leaned_on(SPINS - 1, spun, Vec3::ZERO, 20.0);

            assert!(
                moved < 1e-3,
                "{} it wandered {} radians in twenty seconds",
                if spun { "spun" } else { "stopped" },
                moved
            );
        }
    }

    /// Spec 0006: stopped, a push puts it where you pushed it.
    #[test]
    fn stopped_it_goes_where_you_push_it() {
        let (moved, sideways, _) = leaned_on(0, false, Vec3::NEG_Y, 0.4);

        assert!(moved > 0.3, "a push barely moved it: {} radians", moved);
        assert!(
            sideways < 0.05,
            "{} of a stopped one's movement went sideways",
            sideways
        );
    }

    /// Spec 0006: spun, the same push sends it sideways instead. The whole toy.
    #[test]
    fn spun_it_goes_sideways_instead() {
        for notch in 0..SPINS {
            let (moved, sideways, _) = leaned_on(notch, true, Vec3::NEG_Y, 2.0);

            assert!(
                sideways > 0.9,
                "at spin {} only {} of the movement went sideways",
                spin_at(notch),
                sideways
            );
            assert!(
                moved > 0.01,
                "at spin {} it hardly moved at all",
                spin_at(notch)
            );
        }
    }

    /// Spec 0006: and the harder it spins the slower it goes.
    #[test]
    fn spinning_it_harder_moves_it_slower() {
        let mut was = f32::INFINITY;
        for notch in 0..SPINS {
            let (moved, _, _) = leaned_on(notch, true, Vec3::NEG_Y, 2.0);

            assert!(
                moved < was,
                "at spin {} a push moved it {}, which is not less than {}",
                spin_at(notch),
                moved,
                was
            );
            was = moved;
        }

        // and by about the amount the arithmetic says: twice the spin is half
        // the walk, since the spin is the denominator
        let (slow, _, _) = leaned_on(0, true, Vec3::NEG_Y, 2.0);
        let (fast, _, _) = leaned_on(SPINS - 1, true, Vec3::NEG_Y, 2.0);
        let want = spin_at(SPINS - 1) / spin_at(0);

        assert!(
            (slow / fast - want).abs() < want * 0.2,
            "tripling the spin changed the walk by {} rather than {}",
            slow / fast,
            want
        );
    }

    /// Spec 0006: and the walk is the rate the arithmetic works out.
    #[test]
    fn the_walk_is_what_the_arithmetic_says() {
        for notch in 0..SPINS {
            let (moved, _, _) = leaned_on(notch, true, Vec3::NEG_Y, 2.0);
            let walked = moved / 2.0;
            let off = walked / walks_at(notch) - 1.0;

            assert!(
                off.abs() < 0.1,
                "at spin {} it works out {} a second and walked {}, {:.1}% out",
                spin_at(notch),
                walks_at(notch),
                walked,
                off * 100.0
            );
        }

        // and a stopped one, given the same push, turns far faster than any of
        // them walk. Across the axle rather than about it, because that is the
        // way a push turns something that is not spinning.
        assert!(
            tumbles() > walks_at(0) * 20.0,
            "stopped it turns at {} a second against the slowest walk of {}",
            tumbles(),
            walks_at(0)
        );
    }

    /// Spec 0006: letting go stops it dead.
    #[test]
    fn letting_go_stops_it() {
        let mut one = Gyro::new();
        one.set_going();
        one.leaning = Vec3::NEG_Y;
        for _ in 0..(2.0 / STEP) as u32 {
            one.advance(STEP);
        }
        assert!(one.moving_at() > 0.05, "it was not moving while pushed");
        assert!(
            axle(&one.bodies).dot(FIRST).clamp(-1.0, 1.0).acos() > 0.01,
            "it was not pushed anywhere"
        );

        one.leaning = Vec3::ZERO;
        let held = axle(&one.bodies);
        let mut worst: f32 = 0.0;
        for _ in 0..(5.0 / STEP) as u32 {
            one.advance(STEP);
            worst = worst.max(axle(&one.bodies).dot(held).clamp(-1.0, 1.0).acos());
        }

        // it stops walking. It does not stop moving altogether: a push leaves a
        // wobble in it, and a real one wobbles too. What it must not do is carry
        // on round, so this is against where five more seconds of pushing would
        // have put it.
        let carried = axle(&one.bodies).dot(held).clamp(-1.0, 1.0).acos();
        let pushed = walks_at(one.notch) * 5.0;

        assert!(
            carried < pushed * 0.05,
            "it went another {} radians, against the {} a push would have given it",
            carried,
            pushed
        );
        assert!(worst < pushed * 0.1, "its wobble reached {} radians", worst);
    }

    /// Spec 0006: the rods hold.
    #[test]
    fn the_rods_hold() {
        let mut one = Gyro::new();
        one.turn_the_dial(SPINS as i32);
        one.set_going();
        one.leaning = Vec3::NEG_Y;

        for _ in 0..(5.0 / STEP) as u32 {
            one.advance(STEP);

            for (n, rod) in one.rods.iter().enumerate() {
                let out = rod.error(&one.bodies).abs();
                assert!(
                    out < TIE * 0.02,
                    "rod {} is {} out of {}",
                    n,
                    out,
                    rod.length
                );
            }
        }
    }

    /// Spec 0006: the axis walks evenly rather than lurching.
    ///
    /// The wobble is four tenths of a degree wide and invisible to look at, and
    /// it is quick, so the speed it adds is the size of the walk's own. Undamped
    /// the axis went 23 to one between its fastest and its slowest, twelve times
    /// a second, which is what a gyroscope with nothing dragging on it does and
    /// what a real one's bearings prevent.
    #[test]
    fn it_walks_evenly() {
        let mut one = Gyro::new();
        one.set_going();
        one.leaning = Vec3::NEG_Z;
        for _ in 0..(3.0 / STEP) as u32 {
            one.advance(STEP);
        }

        let a_frame = (1.0 / 120.0 / STEP) as u32;
        let mut worst = 0.0f32;
        let mut least = f32::MAX;
        let mut was = axle(&one.bodies);
        for _ in 0..400 {
            for _ in 0..a_frame {
                one.advance(STEP);
            }
            let now = axle(&one.bodies);
            let step = now.distance(was);
            worst = worst.max(step);
            least = least.min(step);
            was = now;
        }

        assert!(
            worst / least < 1.5,
            "it walks {} to one between its fastest and its slowest",
            worst / least
        );
    }

    /// Spec 0006: the outer frame stays on its pedestal.
    ///
    /// It turns about the upright and about nothing else, because it is bolted
    /// to the stand. Drawn following the axis, an axis tipped towards the
    /// upright carried the whole cage over with it and lifted it off its own
    /// base, which looks like the thing coming apart.
    #[test]
    fn the_frame_stays_on_its_pedestal() {
        // level, pointing each way round, the frame faces the same way
        for round in [0.0f32, 1.0, 2.5, -2.0] {
            let along = vec3(round.cos(), 0.0, -round.sin());
            let faced = frame_round(along).expect("a level axis says which way");

            let off = (faced - round).rem_euclid(std::f32::consts::TAU);
            assert!(
                off < 1e-4 || std::f32::consts::TAU - off < 1e-4,
                "an axis at {} put the frame at {}",
                round,
                faced
            );
        }

        // tipping the axis does not turn the frame, it only tips the gimbal
        let level = frame_round(Vec3::X).expect("level");
        let tipped = frame_round(vec3(0.7, 0.7, 0.0)).expect("tipped");
        assert!((level - tipped).abs() < 1e-4, "tipping swung the frame");

        // and upright it says nothing, which is gimbal lock and is why the
        // last answer is kept
        assert_eq!(frame_round(Vec3::Y), None);
        assert_eq!(frame_round(Vec3::NEG_Y), None);
    }

    /// Spec 0006: the rotor turns slowly enough to be seen turning.
    ///
    /// Nothing to do with physics and everything to do with frames. A mark on a
    /// rim that moves more than half a turn between one frame and the next
    /// cannot be told from one going the other way, and at a sixtieth of a
    /// second a spin of 240 moves it 153 degrees, which is near enough that the
    /// rotor reads as standing still. It was 240, and it looked stopped the
    /// moment nothing else on the bench was moving.
    #[test]
    fn the_rotor_does_not_strobe() {
        const A_FRAME: f32 = 1.0 / 60.0;

        let turn = FASTEST * A_FRAME;
        assert!(
            turn < std::f32::consts::PI * 0.4,
            "the rotor turns {} radians a frame, and half a turn is where it stops reading",
            turn
        );

        // and the walk is untouched by it, because the push came down with the
        // spin and the rate is one over the other
        assert!(
            (walks_at(SPINS - 1) - 0.4).abs() < 0.01 && (walks_at(0) - 1.2).abs() < 0.01,
            "the dial walks {} to {} a second",
            walks_at(SPINS - 1),
            walks_at(0)
        );
    }

    /// Spec 0006: the drawn rotor is a closed disc, wound to be seen.
    #[test]
    fn the_rotor_is_a_closed_disc() {
        let mesh = wheel_mesh(24);

        let bounds = mesh.bounds();
        assert!((bounds.size() - Vec3::ONE).abs().max_element() < 0.01);

        for triangle in mesh.indices.chunks(3) {
            let [a, b, c] = [
                Vec3::from(mesh.vertices[triangle[0] as usize].position),
                Vec3::from(mesh.vertices[triangle[1] as usize].position),
                Vec3::from(mesh.vertices[triangle[2] as usize].position),
            ];
            let normal = Vec3::from(mesh.vertices[triangle[0] as usize].normal);
            let wound = (b - a).cross(c - a);

            assert!(
                wound.length() > 1e-9 && wound.normalize().dot(normal) > 0.9,
                "a face winds against its own normal and is culled"
            );
        }
    }

    /// How much does the drag settle the lurch, and what does it cost the walk?
    #[test]
    #[ignore]
    fn lurch() {
        let mut one = Gyro::new();
        one.set_going();
        one.leaning = Vec3::NEG_Z;

        // let the push settle in, then watch how evenly it walks
        for _ in 0..(3.0 / STEP) as u32 {
            one.advance(STEP);
        }

        let a_frame = (1.0 / 120.0 / STEP) as u32;
        let mut steps = Vec::new();
        let mut was = axle(&one.bodies);
        for _ in 0..600 {
            for _ in 0..a_frame {
                one.advance(STEP);
            }
            let now = axle(&one.bodies);
            steps.push(now.distance(was));
            was = now;
        }

        let mean: f32 = steps.iter().sum::<f32>() / steps.len() as f32;
        let worst = steps.iter().fold(0.0f32, |a, b| a.max(*b));
        let least = steps.iter().fold(f32::MAX, |a, b| a.min(*b));

        println!(
            "drag {:.4}: walks {:.4} a second, lurches {:.1} to one, worst {:.5} least {:.5}",
            DRAG,
            mean * 120.0,
            worst / least.max(1e-9),
            worst,
            least
        );
        println!(
            "   against an arithmetic walk of {:.4}",
            walks_at(STARTS_AT)
        );
    }

    /// What does one frame of the whole nook cost?
    #[test]
    #[ignore]
    fn frame_cost() {
        let mut gyro = Gyro::new();
        gyro.set_going();
        gyro.leaning = Vec3::NEG_Y;

        let mut metronome = crate::metronome::Metronome::new();
        metronome.set_going();
        let mut wrecker = crate::wrecker::Wrecker::new();
        wrecker.haul(Vec3::Z);
        let (mut cradle, ropes) = crate::cradle::strung();
        crate::cradle::set_going(&mut cradle, 0);
        let mut solver = crate::cradle::solver();

        for (what, mut run) in [
            (
                "gyroscope",
                Box::new(|| gyro.advance(1.0 / 60.0)) as Box<dyn FnMut()>,
            ),
            ("metronome", Box::new(|| metronome.advance(1.0 / 60.0))),
            ("ball and chain", Box::new(|| wrecker.advance(1.0 / 60.0))),
            (
                "cradle",
                Box::new(|| {
                    for _ in 0..(1.0 / 60.0 / crate::cradle::STEP) as u32 {
                        solver.step_linked(
                            &mut cradle,
                            &ropes,
                            &crate::cradle::NO_WORLD,
                            crate::GRAVITY,
                            crate::cradle::STEP,
                        );
                    }
                }),
            ),
        ] {
            run();
            let began = std::time::Instant::now();
            for _ in 0..300 {
                run();
            }
            let each = began.elapsed().as_secs_f32() / 300.0;

            println!(
                "{:>14}: {:.3} ms a frame, {:.1}% of a sixtieth",
                what,
                each * 1000.0,
                each / (1.0 / 60.0) * 100.0
            );
        }
    }

    /// Does the pinned middle hold still, or does it buzz against the rods?
    #[test]
    #[ignore]
    fn buzz() {
        for passes in [6usize, 24, 64] {
            let mut one = Gyro::new();
            one.solver = Solver::new().with_passes(passes);
            one.solver.sleeps = false;
            one.set_going();

            // settled first, then pushed, because the two load it differently
            for (what, lean) in [("still", Vec3::ZERO), ("pushed", Vec3::NEG_Y)] {
                one.leaning = lean;
                for _ in 0..(1.0 / STEP) as u32 {
                    one.advance(STEP);
                }

                let mut off: f32 = 0.0;
                let mut frame: f32 = 0.0;
                let mut was = one.bodies[WHEEL].position;
                for _ in 0..120 {
                    for _ in 0..((1.0 / 60.0) / STEP) as u32 {
                        one.advance(STEP);
                    }
                    let now = one.bodies[WHEEL].position;
                    off = off.max(now.distance(hub()));
                    frame = frame.max(now.distance(was));
                    was = now;
                }

                println!(
                    "{:>2} passes {}: middle sits {:.5} off, moves {:.5} a frame, which is {:.2}% of the rotor",
                    passes,
                    what,
                    off,
                    frame,
                    frame / (RADIUS * 2.0) * 100.0
                );
            }
        }
    }

    /// Do the rings sit still, or does the roll they are drawn with jump?
    #[test]
    #[ignore]
    fn ring_roll() {
        let mut one = Gyro::new();
        one.set_going();
        one.leaning = Vec3::NEG_Y;

        // a point on the outer ring, which is the one whose plane the roll
        // decides, sampled a frame apart
        let mut worst: f32 = 0.0;
        let mut was = gimbal(&one.bodies) * Vec3::Y;
        for frame in 0..180 {
            for _ in 0..((1.0 / 60.0) / STEP) as u32 {
                one.advance(STEP);
            }
            if frame == 90 {
                one.leaning = Vec3::ZERO;
            }

            let now = gimbal(&one.bodies) * Vec3::Y;
            worst = worst.max(now.distance(was));
            was = now;
        }

        println!(
            "the ring moves at most {:.4} of its radius between frames, against a walk of {:.4}",
            worst,
            walks_at(STARTS_AT) / 60.0
        );
    }

    /// How big is the wobble, and how fast?
    #[test]
    #[ignore]
    fn wobble() {
        for notch in [0usize, SPINS - 1] {
            let mut one = Gyro::new();
            one.turn_the_dial(notch as i32 - STARTS_AT as i32);
            one.set_going();

            one.leaning = Vec3::NEG_Y;
            for _ in 0..(1.0 / STEP) as u32 {
                one.advance(STEP);
            }
            one.leaning = Vec3::ZERO;

            // the axis against the straight line it would walk if it were not
            // wobbling: with no push it should sit still, so any swing is the
            // wobble
            let settled = axle(&one.bodies);
            let mut high: f32 = -1.0;
            let mut low: f32 = 1.0;
            let mut crossings = 0u32;
            let mut was = 0.0f32;
            for _ in 0..(2.0 / STEP) as u32 {
                one.advance(STEP);
                let off = (axle(&one.bodies) - settled).dot(Vec3::Y);
                high = high.max(off);
                low = low.min(off);
                if was <= 0.0 && off > 0.0 {
                    crossings += 1;
                }
                was = off;
            }

            println!(
                "spin {:>4.0}: wobble {:.4} rad across, {:.1} a second, which is {:.1} degrees",
                spin_at(notch),
                high - low,
                crossings as f32 / 2.0,
                (high - low).to_degrees()
            );
        }
    }

    /// Does the spin hold, and does the readout stop when the push does?
    #[test]
    #[ignore]
    fn holds() {
        let mut one = Gyro::new();
        one.set_going();
        let first = one.bodies[WHEEL].spin.length();
        let along = axle(&one.bodies);

        println!("spun up at {:.2}", first);
        one.leaning = Vec3::NEG_Y;
        for mark in [0.5f32, 1.0] {
            while one.clock < mark {
                one.advance(STEP);
            }
            println!(
                "  pushing, {:.1}s: spin {:.2} ({:+.2}%), reads {:.3} a second",
                mark,
                one.bodies[WHEEL].spin.length(),
                (one.bodies[WHEEL].spin.length() / first - 1.0) * 100.0,
                one.moving_at()
            );
        }

        one.leaning = Vec3::ZERO;
        let held = axle(&one.bodies);
        for mark in [1.2f32, 2.0, 5.0, 11.0, 31.0] {
            while one.clock < mark {
                one.advance(STEP);
            }
            println!(
                "  let go, {:.1}s: spin {:.2} ({:+.2}%), reads {:.3} a second, axis {:.4} from where it was let go, {:.4} from upright",
                mark,
                one.bodies[WHEEL].spin.length(),
                (one.bodies[WHEEL].spin.length() / first - 1.0) * 100.0,
                one.moving_at(),
                axle(&one.bodies).dot(held).clamp(-1.0, 1.0).acos(),
                axle(&one.bodies).dot(along).clamp(-1.0, 1.0).acos()
            );
        }
    }

    /// What the sweeps that set the push and the step saw.
    #[test]
    #[ignore]
    fn sweep() {
        for notch in 0..SPINS {
            let (moved, sideways, moving) = leaned_on(notch, true, Vec3::NEG_Y, 2.0);
            println!(
                "spin {:>5.0}: works out {:.3} rad/s, walked {:.3} ({:+.1}%), {:.0}% sideways, moving {:.3}",
                spin_at(notch),
                walks_at(notch),
                moved / 2.0,
                (moved / 2.0 / walks_at(notch) - 1.0) * 100.0,
                sideways * 100.0,
                moving
            );
        }
        println!(
            "stopped, the same push turns it at {:.1} a second squared",
            tumbles()
        );
    }
}
