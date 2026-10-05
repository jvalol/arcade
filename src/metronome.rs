//! A metronome on a bench. Spec 0006.
//!
//! An arm pivoted near its foot, with a heavy bob just below the pivot and a
//! lighter weight that slides up and down the needle above it. Sliding the
//! weight up slows it down, which is the thing about pendulums people do not
//! know until they are shown it: raising the weight brings the whole thing's
//! middle of mass closer to the pivot from below, and what little is left to
//! right it has to do the work over a bigger swing.
//!
//! Spec 0041 gave the engine distance links and nothing else, and this says it
//! did not need anything else. A hinge is five rods. Pin one point on the arm
//! three ways and it can still turn any way it likes about that point; hold a
//! second point on the same axis two more ways and the only turn left is the one
//! about the axis through them. Five rods for the five freedoms a hinge takes
//! away, which is exactly determined rather than piled on.
//!
//! Where everything sits, and how fast it ought to swing, is arithmetic and is
//! checked without a window.

use blitzkit::collision::Aabb;
use blitzkit::link::Link;
use blitzkit::physics::{Body, Solver};
use glam::{vec3, Quat, Vec3};

/// What the bench it stands on is called.
pub const NAME: &str = "metronome";

/// The bob under the pivot: how big it is and what it weighs.
///
/// It is what rights the thing. Without it the arm's own middle of mass is
/// above a low pivot, which is a pendulum balanced upside down and falls over
/// at the first chance.
pub const BOB: Vec3 = vec3(0.038, 0.055, 0.038);
pub const BOB_MASS: f32 = 1.0;

/// How far above the bob's middle the pivot is, and how high above the bench
/// top it sits.
pub const PIVOT_UP: f32 = 0.09;
pub const PIVOT_Y: f32 = 0.24;

/// The needle above the pivot: how far it reaches, and how thick it is drawn.
///
/// Drawn and not weighed. A real one's needle is a wire, and what a wire adds
/// to a kilogram of brass is nothing worth carrying through the arithmetic.
pub const NEEDLE: f32 = 0.52;
pub const NEEDLE_THICK: f32 = 0.012;

/// The weight that slides: how big, what it weighs, and how many notches it
/// has to sit in.
///
/// Notches rather than anywhere, because the rate is the point and a rate you
/// cannot come back to is not one you can hear the difference against.
pub const WEIGHT: f32 = 0.042;
pub const WEIGHT_MASS: f32 = 0.18;
pub const NOTCHES: usize = 8;

/// The lowest and highest the weight goes, as a height above the pivot.
pub const LOWEST: f32 = 0.12;
pub const HIGHEST: f32 = 0.4;

/// Which notch it is found at.
pub const STARTS_AT: usize = 3;

/// How far it swings each way, in radians. About fifteen degrees, which is what
/// one of these does.
pub const SWING: f32 = 0.26;

/// The hinge: how long each holding rod is, how far along the axis the second
/// held point sits, and how big an anchor is as a body.
///
/// The rods are long, and much longer than the thing they hold. They do not need
/// to be short: a rod is a sphere its end rides, and where the middle of that
/// sphere sits makes no difference to the holding so long as the three of them
/// pull along three directions that between them cover everything. What it does
/// decide is whether an anchor is standing in the arm's way, and an anchor in
/// the arm's way is a contact rather than a hinge. Tried close in first, and
/// there is nowhere close in to put them: the bob sweeps the space below the
/// pivot, the weight occupies everything from a tenth of a unit above it to four
/// tenths, and the swing is the rest.
pub const TIE: f32 = 0.5;
pub const HINGE_HALF: f32 = 0.06;
pub const PIN: f32 = 0.002;

/// How far from the seat the three rods that carry the weight are tied on.
pub const SEAT_TIE: f32 = 0.06;

/// The case, which is drawn and not weighed: the foot it stands on, and the
/// plate the needle swings in front of with the notches marked up it.
///
/// The plate stands clear of the weight rather than against it. At a finger's
/// width behind the needle it was inside the weight, which is 0.042 across.
pub const FOOT: Vec3 = vec3(0.18, 0.05, 0.16);
pub const PLATE_BACK: f32 = 0.062;
pub const PLATE_THICK: f32 = 0.014;
pub const PLATE_WIDE: f32 = 0.2;
pub const PLATE_TALL: f32 = PIVOT_Y + NEEDLE + 0.02 - FOOT.y;

/// The notches up the plate: how long a mark is, and how far to the side of the
/// needle it sits.
///
/// Beside the weight and not behind it. Marked down the middle, the one the
/// weight is in is the one the weight covers, so the lit mark that says where
/// the setting is was the only mark you could never see.
/// Far enough out that the weight does not sit on top of the mark: at a weight's
/// width the lit one was a sliver showing past its edge.
pub const NOTCH_LONG: f32 = 0.04;
pub const NOTCH_AT: f32 = 0.072;

/// How fast it is stepped, and how many passes a step gets.
pub const STEP: f32 = 1.0 / 240.0;
pub const PASSES: usize = 32;

/// Nothing to collide with. The arm is held by its rods and reaches nothing.
pub const NO_WORLD: [Aabb; 0] = [];

/// Which body is which, in the order [`built`] makes them.
pub const PINS: usize = 5;
pub const ARM: usize = PINS;
pub const SLIDER: usize = PINS + 1;

/// Where the pivot is, in the frame whose origin is the middle of the bench top.
pub fn pivot() -> Vec3 {
    Vec3::Y * PIVOT_Y
}

/// How high above the pivot the weight sits at a notch.
pub fn weight_at(notch: usize) -> f32 {
    let notch = notch.min(NOTCHES - 1) as f32;

    LOWEST + notch * (HIGHEST - LOWEST) / (NOTCHES - 1) as f32
}

/// The five places the hinge is held: an anchor in the world, and where on the
/// arm the rod to it is tied.
fn hinge() -> [(Vec3, Vec3); 5] {
    // the three axes, which is the plainest way to be sure the three pulls cover
    // every direction: three rods pulling along one plane would leave the point
    // free to walk out of that plane and never be hauled back
    let back = Vec3::NEG_X;
    let up = Vec3::Y;
    let side = Vec3::Z;

    // the pivot itself, held every way
    let one = Vec3::Y * PIVOT_UP;
    // and a second point along the hinge's own axis, held two ways. It is on the
    // axis, so swinging never moves it at all: what these two refuse is the arm
    // turning any way but about that axis.
    let along = Vec3::NEG_X * HINGE_HALF;
    let other = one + along;

    [
        (pivot() + back * TIE, one),
        (pivot() + up * TIE, one),
        (pivot() + side * TIE, one),
        (pivot() + along + up * TIE, other),
        (pivot() + along + side * TIE, other),
    ]
}

/// Where on the arm the three rods carrying the weight are tied, for a notch.
///
/// A tripod, which is what pins a point to a body: three rods along three
/// directions that between them cover every way the weight could slip.
fn seats(notch: usize) -> [Vec3; 3] {
    let seat = Vec3::Y * (PIVOT_UP + weight_at(notch));

    [
        seat + Vec3::X * SEAT_TIE,
        seat + Vec3::Y * SEAT_TIE,
        seat + Vec3::Z * SEAT_TIE,
    ]
}

/// Where the arm and the weight are, leaning by `lean` with the weight at
/// `notch`.
pub fn posed(notch: usize, lean: f32) -> (Vec3, Quat, Vec3) {
    let turn = Quat::from_rotation_x(lean);

    (
        pivot() - turn * (Vec3::Y * PIVOT_UP),
        turn,
        pivot() + turn * (Vec3::Y * weight_at(notch)),
    )
}

/// The bodies and the rods, upright and still.
pub fn built(notch: usize) -> (Vec<Body>, Vec<Link>) {
    let (arm_at, turn, slider_at) = posed(notch, 0.0);
    let mut bodies = Vec::new();
    let mut rods = Vec::new();

    for (anchor, _) in hinge() {
        bodies.push(Body::immovable(anchor, PIN));
    }
    bodies.push(Body::block(arm_at, BOB, BOB_MASS).facing(turn));
    bodies.push(Body::new(slider_at, WEIGHT, WEIGHT_MASS));

    for (n, (_, on_arm)) in hinge().iter().copied().enumerate() {
        rods.push(Link::rod(n, ARM, TIE).tied_at(Vec3::ZERO, on_arm));
    }
    for tie in seats(notch) {
        rods.push(Link::rod(ARM, SLIDER, SEAT_TIE).tied_at(tie, Vec3::ZERO));
    }

    (bodies, rods)
}

/// How far over the needle is leaning, and which way.
pub fn lean(bodies: &[Body]) -> f32 {
    let up = bodies[ARM].orientation * Vec3::Y;

    up.z.atan2(up.y)
}

/// How long one swing out and back takes, worked out rather than watched.
///
/// The bob below the pivot rights it and the weight above the pivot un-rights
/// it, so what is left to pull it back is the difference. That difference is
/// what shrinks as the weight goes up, while the weight's own reluctance to be
/// turned grows, and the period has the one on top of the other.
pub fn period(notch: usize) -> f32 {
    let up = weight_at(notch);
    let pull = crate::GRAVITY.y.abs();

    let righting = pull * (BOB_MASS * PIVOT_UP - WEIGHT_MASS * up);
    let reluctance = BOB_MASS / 3.0 * (BOB.y * BOB.y + BOB.z * BOB.z)
        + BOB_MASS * PIVOT_UP * PIVOT_UP
        + WEIGHT_MASS * up * up;

    std::f32::consts::TAU * (reluctance / righting).sqrt()
}

/// Ticks a minute the arithmetic above works out to. One a swing, so two to a
/// period.
///
/// What the arithmetic says and not what the rods do. The two part company at
/// the slow end of the dial, by 2% at the bottom notch and 5% at the top, and
/// the reason is the mechanism itself rather than the solver: up there the thing
/// is very nearly balanced, with 0.018 of righting left out of the bob's 0.09,
/// so a tenth of a millimetre of give in the pivot is a few percent of what is
/// left. More passes do not touch it. Thirty-two and two hundred and fifty-six
/// came out the same to a part in a thousand.
///
/// So the bench reports what it measures and this is kept for the explanation,
/// and for sizing the winding, which is an energy and is exact.
pub fn beats(notch: usize) -> f32 {
    120.0 / period(notch)
}

/// What a full swing is worth, measured at the upright where none of it is
/// height.
fn worth(notch: usize) -> f32 {
    let pull = crate::GRAVITY.y.abs();
    let righting = pull * (BOB_MASS * PIVOT_UP - WEIGHT_MASS * weight_at(notch));

    righting * (1.0 - SWING.cos())
}

/// What it is carrying now.
fn carrying(bodies: &[Body]) -> f32 {
    let arm = &bodies[ARM];
    let slider = &bodies[SLIDER];

    0.5 * (BOB_MASS * arm.velocity.length_squared()
        + arm.spin.dot(arm.inertia() * arm.spin)
        + WEIGHT_MASS * slider.velocity.length_squared())
}

/// Tops the swing up as it goes through the upright, which is what the spring
/// in a wound one does.
///
/// Every speed is scaled by the same amount. A distance rod is a rule about
/// speeds along a line and nothing else, so scaling every speed by one number
/// leaves every rule exactly as satisfied as it already was: there is no jolt
/// to take up and no pass spent taking it. Striking the arm instead puts it
/// somewhere the rods then have to haul it back from.
pub fn wind(bodies: &mut [Body], notch: usize) {
    let now = carrying(bodies);
    if now < 1e-9 {
        return;
    }

    let by = (worth(notch) / now).sqrt().clamp(0.5, 1.5);
    for n in [ARM, SLIDER] {
        bodies[n].velocity *= by;
        bodies[n].spin *= by;
        bodies[n].wake();
    }
}

/// A solver set up the way this wants one.
///
/// It never sleeps. A pendulum is motionless at each end of its swing and this
/// one is there twice a second, which is exactly what a sleep test is looking
/// for, and a metronome that nods off at the top of a swing is a bug.
pub fn solver() -> Solver {
    let mut solver = Solver::new().with_passes(PASSES);
    solver.sleeps = false;

    solver
}

/// The one on the bench.
pub struct Metronome {
    pub bodies: Vec<Body>,
    pub rods: Vec<Link>,
    /// Which notch the weight is in.
    pub notch: usize,
    pub going: bool,
    /// How many times it has gone through the upright, which is the ticking.
    pub ticks: u32,
    /// Which side it was on, so going through can be noticed.
    was: f32,
    /// Seconds owed, so it steps at its own rate and not the frame's.
    owed: f32,
    /// Its own clock, and when it last ticked by it.
    clock: f32,
    last: Option<f32>,
    /// How long a swing is taking, smoothed. Measured rather than worked out,
    /// because the two differ by a few percent and the one on the bench should
    /// be the one you can count against.
    swung: f32,
    solver: Solver,
}

impl Default for Metronome {
    fn default() -> Self {
        Self::new()
    }
}

impl Metronome {
    pub fn new() -> Self {
        let (bodies, rods) = built(STARTS_AT);

        Self {
            bodies,
            rods,
            notch: STARTS_AT,
            going: false,
            ticks: 0,
            was: 0.0,
            owed: 0.0,
            clock: 0.0,
            last: None,
            swung: 0.0,
            solver: solver(),
        }
    }

    /// Sets it going, or stops it dead and stands it up again.
    pub fn press(&mut self) {
        if self.going {
            self.stop();
        } else {
            self.set_going();
        }
    }

    pub fn set_going(&mut self) {
        self.place(SWING);
        self.going = true;
        self.was = SWING;
    }

    pub fn stop(&mut self) {
        self.place(0.0);
        self.going = false;
        self.was = 0.0;
    }

    /// Puts the arm at a lean and takes all the speed out of it.
    fn place(&mut self, lean: f32) {
        let (arm, turn, slider) = posed(self.notch, lean);

        self.bodies[ARM].position = arm;
        self.bodies[ARM].orientation = turn;
        self.bodies[SLIDER].position = slider;
        for n in [ARM, SLIDER] {
            self.bodies[n].velocity = Vec3::ZERO;
            self.bodies[n].spin = Vec3::ZERO;
            self.bodies[n].wake();
        }
    }

    /// Slides the weight by a notch or two, keeping the lean it had.
    ///
    /// A hand moves the weight while the thing is going and the rate changes
    /// under it, which is the only way to hear that it changed.
    pub fn slide(&mut self, by: i32) {
        let want = (self.notch as i32 + by).clamp(0, NOTCHES as i32 - 1) as usize;
        if want == self.notch {
            return;
        }
        self.notch = want;
        // the rate it was keeping is not the rate it is about to keep
        self.last = None;
        self.swung = 0.0;

        for (rod, tie) in self.rods[PINS..].iter_mut().zip(seats(self.notch)) {
            rod.at_one = tie;
        }

        // and the weight goes with it, carrying whatever the arm is doing where
        // it lands rather than arriving stopped
        let arm = self.bodies[ARM];
        let at = arm.position + arm.orientation * (Vec3::Y * (PIVOT_UP + weight_at(self.notch)));

        self.bodies[SLIDER].position = at;
        self.bodies[SLIDER].velocity = arm.velocity + arm.spin.cross(at - arm.position);
        self.bodies[SLIDER].wake();
        self.bodies[ARM].wake();
    }

    /// Steps it at its own rate, and winds it each time it goes through.
    pub fn advance(&mut self, dt: f32) {
        self.owed = (self.owed + dt).min(0.2);

        while self.owed >= STEP {
            self.solver.step_linked(
                &mut self.bodies,
                &self.rods,
                &NO_WORLD,
                crate::GRAVITY,
                STEP,
            );
            self.owed -= STEP;
            self.clock += STEP;

            let now = lean(&self.bodies);
            if self.going && now != 0.0 && now.signum() != self.was.signum() {
                wind(&mut self.bodies, self.notch);
                self.ticks += 1;
                self.time_it();
            }
            self.was = now;
        }
    }

    /// Notes how long that swing took.
    ///
    /// Smoothed, because a single swing is timed to the nearest step and a
    /// number that twitches in the last digit reads as a number that is wrong.
    fn time_it(&mut self) {
        if let Some(last) = self.last {
            let took = self.clock - last;
            self.swung = if self.swung == 0.0 {
                took
            } else {
                self.swung * 0.6 + took * 0.4
            };
        }
        self.last = Some(self.clock);
    }

    /// Ticks a minute, counted rather than worked out. None until it has swung
    /// twice and there is something to count.
    pub fn ticking(&self) -> Option<f32> {
        (self.swung > 1e-4).then(|| 60.0 / self.swung)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs one for a while and gives back how long a period measured out at,
    /// and the worst the swing ever got.
    ///
    /// Timed between crossings rather than from the start. It is let go from the
    /// top, so the first crossing is a quarter of a period in, and counting that
    /// quarter as if it were a whole one put the answer a percent high over a
    /// dozen swings.
    fn run(notch: usize, seconds: f32) -> (f32, f32) {
        let mut one = Metronome::new();
        one.slide(notch as i32 - STARTS_AT as i32);
        one.set_going();

        let mut widest: f32 = 0.0;
        let mut crossed: Vec<u32> = Vec::new();
        for step in 0..(seconds / STEP) as u32 {
            let was = one.ticks;
            one.advance(STEP);
            widest = widest.max(lean(&one.bodies).abs());

            if one.ticks != was {
                crossed.push(step);
            }
        }

        // two swings to a period, and it ticks once a swing. The first two are
        // dropped: one is the quarter period it is let go through and the next
        // is the first the winding has had a say in.
        let period = match (crossed.get(2), crossed.last()) {
            (Some(first), Some(last)) if last > first => {
                2.0 * (last - first) as f32 * STEP / (crossed.len() - 3) as f32
            }
            _ => f32::INFINITY,
        };

        (period, widest)
    }

    /// Free swing, nothing winding it: how long a period takes against what the
    /// arithmetic says, at several amplitudes.
    #[test]
    #[ignore]
    fn free() {
        for notch in [0usize, 4, 7] {
            for start in [0.02f32, 0.08, 0.26] {
                let (bodies, rods) = built(notch);
                let mut bodies = bodies;
                let (arm, turn, slider) = posed(notch, start);
                bodies[ARM].position = arm;
                bodies[ARM].orientation = turn;
                bodies[SLIDER].position = slider;

                let mut solver = solver();
                let want = period(notch);
                let mut crossed: Vec<u32> = Vec::new();
                let mut was = start;
                for step in 0..((want * 20.0) / STEP) as u32 {
                    solver.step_linked(&mut bodies, &rods, &NO_WORLD, crate::GRAVITY, STEP);
                    let now = lean(&bodies);
                    if now != 0.0 && now.signum() != was.signum() {
                        crossed.push(step);
                    }
                    was = now;
                }

                let got = match (crossed.get(1), crossed.last()) {
                    (Some(a), Some(b)) if b > a => {
                        2.0 * (b - a) as f32 * STEP / (crossed.len() - 2) as f32
                    }
                    _ => f32::NAN,
                };
                println!(
                    "notch {} from {:.2}: works out {:.4}s, swings {:.4}s ({:+.2}%), ended at {:.4}",
                    notch,
                    start,
                    want,
                    got,
                    (got - want) / want * 100.0,
                    lean(&bodies).abs()
                );
            }
        }
    }

    /// What the sweep that set the tolerance saw.
    #[test]
    #[ignore]
    fn sweep() {
        for notch in 0..NOTCHES {
            let want = period(notch);
            let (got, widest) = run(notch, want * 20.0);
            println!(
                "notch {} weight {:.2} works out {:.1}/min, ticks {:.1}/min ({:+.2}%), widest {:.3} of {:.3}",
                notch,
                weight_at(notch),
                beats(notch),
                120.0 / got,
                (got - want) / want * 100.0,
                widest,
                SWING
            );
        }
    }

    #[test]
    fn the_weight_sits_where_the_notches_are() {
        assert_eq!(weight_at(0), LOWEST);
        assert_eq!(weight_at(NOTCHES - 1), HIGHEST);
        assert_eq!(weight_at(NOTCHES + 9), HIGHEST, "a notch past the last one");

        for notch in 1..NOTCHES {
            assert!(weight_at(notch) > weight_at(notch - 1));
        }
        // and it stays on the needle
        let highest = HIGHEST + WEIGHT;
        assert!(
            highest < NEEDLE,
            "the weight reaches {} of {}",
            highest,
            NEEDLE
        );
    }

    /// Spec 0006: the hinge is five rods, and they are tied where the arithmetic
    /// says.
    #[test]
    fn every_rod_is_built_at_its_own_length() {
        let (bodies, rods) = built(STARTS_AT);

        assert_eq!(bodies.len(), PINS + 2);
        assert_eq!(
            rods.len(),
            PINS + 3,
            "five for the hinge and three for the weight"
        );

        for (n, rod) in rods.iter().enumerate() {
            assert!(
                (rod.error(&bodies)).abs() < 1e-5,
                "rod {} is built {} out",
                n,
                rod.error(&bodies)
            );
            assert!(rod.rigid, "rod {} is a rope, so it only pulls", n);
        }

        for (pin, anchor) in bodies[..PINS].iter().enumerate() {
            assert_eq!(anchor.inverse_mass, 0.0, "anchor {} can move", pin);
        }
    }

    /// Spec 0006: nothing is standing where the arm swings.
    ///
    /// The anchors are bodies, and a body in the way of the arm is a contact
    /// rather than a hinge.
    #[test]
    fn the_anchors_are_out_of_the_way() {
        let (bodies, _) = built(0);

        for lean in [-SWING, -SWING * 0.5, 0.0, SWING * 0.5, SWING] {
            for notch in [0, NOTCHES - 1] {
                let (arm_at, turn, slider_at) = posed(notch, lean);

                for (pin, anchor) in bodies[..PINS].iter().enumerate() {
                    let at = anchor.position;

                    // clear of the bob, which turns with the arm
                    let in_arm = turn.inverse() * (at - arm_at);
                    let out = (in_arm.abs() - BOB).max_element();
                    assert!(out > PIN, "anchor {} is {} inside the bob", pin, -out);

                    // and clear of the weight
                    let off = at.distance(slider_at);
                    assert!(
                        off > WEIGHT + PIN,
                        "anchor {} is {} from the weight, which is {} across",
                        pin,
                        off,
                        WEIGHT
                    );
                }
            }
        }
    }

    /// Spec 0006: left alone it stands up and stays there.
    #[test]
    fn it_stands_still_until_it_is_started() {
        let mut one = Metronome::new();
        for _ in 0..(5.0 / STEP) as u32 {
            one.advance(STEP);
        }

        assert!(
            lean(&one.bodies).abs() < 0.02,
            "it leaned {} with nobody touching it",
            lean(&one.bodies)
        );
        assert_eq!(one.ticks, 0, "it ticked without being started");
    }

    /// Spec 0006: and swinging, it only swings the one way.
    ///
    /// This is what the hinge is for. Three rods on one point is a ball joint,
    /// and a ball joint lets the arm wander out of its plane and never come
    /// back. The two extra rods on a second point along the axis are what stop
    /// it.
    #[test]
    fn it_only_swings_the_one_way() {
        let mut one = Metronome::new();
        one.set_going();

        let (arm_at, _, _) = posed(one.notch, 0.0);
        for _ in 0..(20.0 / STEP) as u32 {
            one.advance(STEP);

            let wandered = (one.bodies[ARM].position.x - arm_at.x).abs();
            assert!(
                wandered < 0.01,
                "the arm wandered {} out of its plane",
                wandered
            );

            let up = one.bodies[ARM].orientation * Vec3::Y;
            assert!(up.x.abs() < 0.03, "the needle leaned {} sideways", up.x);
        }
    }

    /// Spec 0006: the swing is near what the arithmetic works out to.
    ///
    /// Near, not equal. The two part company by 1.7% at the bottom notch and
    /// 5.5% at the top, and the mechanism is the reason rather than the solver:
    /// at the top the thing is nearly balanced, so a tenth of a millimetre of
    /// give in the pivot is a few percent of the righting that is left. Thirty
    /// two passes and two hundred and fifty six give the same answer to a part
    /// in a thousand, which is what rules the solver out.
    ///
    /// Held anyway, loosely, because it is what catches the model being wrong
    /// rather than slack: a ball joint instead of a hinge, a weight hung
    /// somewhere other than where it is drawn, or a mass that is not the mass
    /// the arithmetic was given would all land nowhere near.
    #[test]
    fn the_swing_is_near_what_the_arithmetic_says() {
        for notch in [0, 2, 4, NOTCHES - 1] {
            let want = period(notch);
            let (measured, _) = run(notch, want * 12.0);
            let off = (measured - want) / want;

            assert!(
                off > 0.0 && off < 0.08,
                "notch {} works out at {}s and swings at {}s, {:.1}% out",
                notch,
                want,
                measured,
                off * 100.0
            );
        }
    }

    /// Spec 0006: the weight up is slower. The whole point of the toy.
    ///
    /// Counted, not worked out, since counted is what the bench shows.
    #[test]
    fn the_weight_up_slows_it_down() {
        let mut was = f32::INFINITY;
        for notch in 0..NOTCHES {
            let (period, _) = run(notch, period(notch) * 10.0);
            let ticks = 120.0 / period;

            assert!(
                ticks < was,
                "notch {} ticks {} a minute, which is not slower than {}",
                notch,
                ticks,
                was
            );
            was = ticks;
        }

        // and the dial it ends up covering is a metronome's: this ran out at
        // 139 a minute at the bottom notch and 39 at the top
        let (fastest, _) = run(0, period(0) * 10.0);
        let (slowest, _) = run(NOTCHES - 1, period(NOTCHES - 1) * 10.0);

        assert!(
            120.0 / fastest > 130.0 && 120.0 / slowest < 45.0,
            "the dial runs {} to {} a minute",
            120.0 / slowest,
            120.0 / fastest
        );
    }

    /// Spec 0006: and the rate it puts on the bench is the rate it keeps.
    #[test]
    fn the_rate_it_reports_is_the_rate_it_ticks() {
        let mut one = Metronome::new();
        assert_eq!(one.ticking(), None, "it reported a rate before it swung");

        one.set_going();
        for _ in 0..(30.0 / STEP) as u32 {
            one.advance(STEP);
        }

        let counted = one.ticks as f32 * 2.0;
        let said = one.ticking().expect("a rate after thirty seconds");
        let off = (said - counted).abs() / counted;

        assert!(
            off < 0.03,
            "it says {} a minute and ticked {} times in thirty seconds",
            said,
            one.ticks
        );

        // and moving the weight drops the old reading rather than carrying it
        one.slide(3);
        assert_eq!(one.ticking(), None, "it kept the rate from the old notch");
    }

    /// Spec 0006: it keeps going, because a wound one does.
    #[test]
    fn it_keeps_going_and_keeps_its_swing() {
        let mut one = Metronome::new();
        one.set_going();
        for _ in 0..(60.0 / STEP) as u32 {
            one.advance(STEP);
        }

        assert!(one.ticks > 60, "it ticked {} times in a minute", one.ticks);
        assert!(
            lean(&one.bodies).abs() > 0.01 || one.bodies[ARM].spin.length() > 0.1,
            "it stopped"
        );

        // and it has not wound itself up into something it was never set to
        let (_, widest) = run(STARTS_AT, 40.0);
        assert!(
            widest < SWING * 1.3,
            "the swing grew to {} from {}",
            widest,
            SWING
        );
        assert!(widest > SWING * 0.7, "the swing fell to {}", widest);
    }

    /// Spec 0006: pressing it again stops it dead and stands it up.
    #[test]
    fn pressing_it_again_stops_it() {
        let mut one = Metronome::new();
        one.press();
        assert!(one.going);
        for _ in 0..(2.0 / STEP) as u32 {
            one.advance(STEP);
        }

        one.press();
        assert!(!one.going);
        assert!(
            lean(&one.bodies).abs() < 1e-5,
            "it stopped leaning at {}",
            lean(&one.bodies)
        );

        let was = one.ticks;
        for _ in 0..(5.0 / STEP) as u32 {
            one.advance(STEP);
        }
        assert_eq!(one.ticks, was, "it went on ticking after it was stopped");
    }

    /// Spec 0006: and the weight can be moved while it is going.
    #[test]
    fn sliding_the_weight_while_it_goes_changes_the_rate() {
        let mut one = Metronome::new();
        one.set_going();

        let count = |one: &mut Metronome, seconds: f32| {
            let was = one.ticks;
            for _ in 0..(seconds / STEP) as u32 {
                one.advance(STEP);
            }
            one.ticks - was
        };

        one.slide(-(STARTS_AT as i32));
        assert_eq!(one.notch, 0);
        let fast = count(&mut one, 12.0);

        one.slide(NOTCHES as i32);
        assert_eq!(one.notch, NOTCHES - 1, "the slide ran off the end");
        let slow = count(&mut one, 12.0);

        assert!(
            fast > slow * 2,
            "{} ticks at the bottom against {} at the top",
            fast,
            slow
        );

        // and it survived being moved mid swing
        assert!(
            slow > 3,
            "it only ticked {} times after the weight moved",
            slow
        );
        for rod in &one.rods {
            assert!(
                rod.error(&one.bodies).abs() < 0.01,
                "a rod is {} out after the weight moved",
                rod.error(&one.bodies)
            );
        }
    }

    /// Spec 0006: the rods hold, which is what says the hinge is a hinge rather
    /// than something stretching quietly.
    #[test]
    fn the_rods_hold() {
        let mut one = Metronome::new();
        one.set_going();

        for _ in 0..(20.0 / STEP) as u32 {
            one.advance(STEP);

            for (n, rod) in one.rods.iter().enumerate() {
                let out = rod.error(&one.bodies).abs();
                assert!(
                    out < TIE * 0.05,
                    "rod {} is {} out of {}",
                    n,
                    out,
                    rod.length
                );
            }
        }
    }
}
