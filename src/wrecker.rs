//! A ball on a chain, and a wall to swing it at, standing on a bench. Spec 0006.
//!
//! This is the toy the nook was built for. Spec 0041 gave the engine links and
//! `chain.rs` is where they were shown off, and the room had nowhere to put it:
//! you work it, so it is not one of the shapes on the far wall, and there is
//! nothing to win, so it is not one of the games in a cabinet.
//!
//! The example's own one hangs from eight metres up and this room is three and a
//! bit tall, so this is a model of it rather than the thing itself. The example
//! stays where it is, full size, at `cargo run --release --example chain`. What
//! is here is the same build at a thirteenth of the size: a gantry, a chain of
//! beads, a ball heavy enough to be worth swinging, and a wall of brick that
//! falls over.
//!
//! It stands on a tray with a lip. A model needs somewhere for the pieces to
//! end up, and without the lip a knocked brick slides off the bench and falls
//! through a room this solver knows nothing about.
//!
//! Where it all sits is arithmetic and is checked without a window.

use blitzkit::collision::Aabb;
use blitzkit::link::Link;
use blitzkit::physics::{Body, Solver};
use glam::{vec3, Quat, Vec3};

/// What the bench it stands on is called.
pub const NAME: &str = "ball and chain";

/// The tray: how much of the bench top it covers, and the lip round it.
pub const TRAY_WIDE: f32 = 0.8;
pub const TRAY_LONG: f32 = 1.3;
pub const LIP: f32 = 0.03;
pub const LIP_THICK: f32 = 0.02;

/// The chain: how many beads, how wide one is, and how far apart they hang.
pub const BEADS: usize = 10;
pub const BEAD: f32 = 0.03;
pub const SPACING: f32 = 0.045;

/// What one bead weighs, against the ball.
///
/// The example's ratio kept. A sequential solver works a chain a link at a
/// time, so what it finds hard is a heavy thing hung off light ones: the ball's
/// weight has to be passed up the chain and each pass moves it one link.
pub const BEAD_MASS: f32 = 0.09;

/// The ball. Heavy, because a wrecking ball that bounces off is a bauble.
pub const BALL: f32 = 0.065;
pub const BALL_MASS: f32 = 2.4;

/// Where it hangs from, above the middle of the tray, and how tall the gantry
/// that carries it is drawn.
pub const HANGS_FROM: Vec3 = vec3(0.0, 0.7, -0.35);
pub const POST: f32 = 0.028;

/// Where the gantry's post stands, and the jib reaches from it to the hook.
///
/// In the corner, not beside the hook. The arrows haul the ball both ways now,
/// so a post standing anywhere the ball can reach is a post the ball goes
/// through: it was level with the hook and the first pull across the tray put
/// the ball inside it. The corner is the furthest from the hook anything
/// standing on the tray can be.
///
/// Behind the model as well as off to the side. You come into the nook looking
/// along -x, so this is the far side and the post is not between you and the
/// wall you are swinging at.
pub const POST_AT: Vec3 = vec3(-0.37, 0.0, -0.58);

/// The wall: bricks across, bricks up, and how big one is.
///
/// Thin across the swing, so the wall stands face on to the ball.
pub const WIDE: usize = 5;
pub const HIGH: usize = 6;
pub const BRICK: Vec3 = vec3(0.0708, 0.0217, 0.035);

/// What a brick weighs.
///
/// The example's ball is two hundred and forty times its bricks and at this size
/// that does not survive. Swept, by what got off the tray and how fast anything
/// was still moving at the end:
///
/// ```text
/// 0.012  1 brick gone, something still doing 158 a second
/// 0.030  2 gone, 56 a second
/// 0.060  none gone, nothing moving, 23 of 27 down
/// 0.240  none gone, 15 of 27 down
/// ```
///
/// A fortieth of the ball. Lighter than that and a brick caught between the ball
/// and the wall behind it is squeezed by two hundred times its own weight, and
/// what comes out is not a brick falling over, it is a brick leaving at a
/// hundred and fifty a second. Heavier and the wall stops coming down.
pub const BRICK_MASS: f32 = 0.06;

/// Where the wall stands, down the swing from the hook.
pub const WALL_AT: f32 = 0.05;

/// What one press of an arrow adds to the ball.
///
/// Swept, pulling once every time the ball turned round, and counting the pulls
/// before the first brick moved against how much of the wall was down after
/// twenty seconds:
///
/// ```text
/// 0.6  6 pulls  11 of 27 down
/// 0.7  4 pulls  23 of 27
/// 0.8  2 pulls  10 of 27
/// 1.2  2 pulls  12 of 27
/// ```
///
/// 0.7. Past it two pulls do the whole job, which leaves nothing to work, and
/// under it the pumping goes on long enough to stop being a thing you are
/// doing.
pub const HAUL: f32 = 0.7;

/// How much of the chain a press winds in or out, and how far the winch goes
/// either way.
///
/// The one control a crane actually has, and the hardest thing to ask of a link:
/// the chain's length changes while the ball is hanging on it, so every link has
/// to take up slack or pay out under load rather than settle something that was
/// already still. It is on shift and the arrows rather than on the arrows
/// themselves, because hauling the ball about is what you do with this and
/// winding is what you do once.
pub const WINDS_BY: f32 = 0.06;
pub const WOUND_IN: f32 = 0.35;
pub const WOUND_OUT: f32 = 1.25;

/// How far a brick has to have moved from where it was built to count as
/// knocked down. Where it was, not how high it is.
pub const KNOCKED: f32 = BRICK.z;

/// How much room a brick wants round the ball before it will be laid.
pub const CLEAR: f32 = 0.006;

/// How fast it is stepped, and how many passes a step gets.
///
/// The engine's default number of passes. The chain hangs about 5.5% long under
/// the ball, which is the same order as the full size example's 4.6% and is the
/// thing spec 0041's passes are spent on. Swept: 64 passes take it to 4.8% and
/// 128 to 4.4%, and a heavier chain gets no further, so it bottoms out at about
/// four and the rest is not for sale.
pub const STEP: f32 = 1.0 / 240.0;
pub const PASSES: usize = 32;

/// Which body is which, in the order [`Wrecker::build`] makes them.
pub const HOOK: usize = 0;
pub const BALL_AT: usize = BEADS + 1;
pub const WALL_FROM: usize = BEADS + 2;

/// How many bricks a whole wall is.
///
/// Every other course is set in by half a brick, so the joints are staggered
/// the way a built wall's are, which costs that course one brick.
pub fn bricks() -> usize {
    (0..HIGH)
        .map(|row| if row % 2 == 1 { WIDE - 1 } else { WIDE })
        .sum()
}

/// The tray's floor and the lip round it, which is everything the pieces can
/// land on.
pub fn tray() -> Vec<Aabb> {
    let out = LIP_THICK * 0.5;

    vec![
        // the bench top itself, with its face at nought
        Aabb::from_center_size(vec3(0.0, -0.1, 0.0), vec3(TRAY_WIDE, 0.2, TRAY_LONG)),
        Aabb::from_center_size(
            vec3(TRAY_WIDE * 0.5 + out, LIP * 0.5, 0.0),
            vec3(LIP_THICK, LIP, TRAY_LONG + LIP_THICK * 2.0),
        ),
        Aabb::from_center_size(
            vec3(-TRAY_WIDE * 0.5 - out, LIP * 0.5, 0.0),
            vec3(LIP_THICK, LIP, TRAY_LONG + LIP_THICK * 2.0),
        ),
        Aabb::from_center_size(
            vec3(0.0, LIP * 0.5, TRAY_LONG * 0.5 + out),
            vec3(TRAY_WIDE, LIP, LIP_THICK),
        ),
        Aabb::from_center_size(
            vec3(0.0, LIP * 0.5, -TRAY_LONG * 0.5 - out),
            vec3(TRAY_WIDE, LIP, LIP_THICK),
        ),
    ]
}

/// Where a brick is built, given its course and its place along it.
pub fn brick_at(row: usize, column: usize) -> Vec3 {
    let across = if row % 2 == 1 { WIDE - 1 } else { WIDE };

    vec3(
        (column as f32 - (across - 1) as f32 * 0.5) * BRICK.x * 2.0,
        BRICK.y * (row as f32 * 2.0 + 1.0),
        WALL_AT,
    )
}

/// Where every brick comes to rest, worked out once by standing a wall up and
/// letting it go.
///
/// Laid where the arithmetic puts them the courses sink. Each contact gives up a
/// little before the solver pushes back, and stacked six deep that came to 0.032
/// by the top course, which is 73% of a brick's own height. Three things went
/// wrong with it: the wall slumped the moment it appeared, every brick sat
/// within a whisker of the distance that counts as knocked down, and rebuilding
/// snapped the lot back up, which is the hop you got pressing enter at a wall
/// that was already standing.
///
/// So the wall is built where it comes to rest rather than where it is drawn on
/// paper, and the resting place is measured rather than guessed at.
pub fn resting() -> Vec<(Vec3, Quat)> {
    let mut bodies = Vec::new();
    for row in 0..HIGH {
        let across = if row % 2 == 1 { WIDE - 1 } else { WIDE };
        for column in 0..across {
            bodies.push(Body::block(brick_at(row, column), BRICK, BRICK_MASS).with_friction(0.7));
        }
    }

    let world = tray();
    let mut solver = Solver::new().with_passes(PASSES);
    for _ in 0..(3.0 / STEP) as u32 {
        solver.step(&mut bodies, &world, crate::GRAVITY, STEP);
    }

    bodies
        .iter()
        .map(|brick| (brick.position, brick.orientation))
        .collect()
}

/// The one on the bench.
pub struct Wrecker {
    pub bodies: Vec<Body>,
    pub chain: Vec<Link>,
    /// Where each brick was built, which is what knocked down is measured
    /// against.
    built_at: Vec<Vec3>,
    /// What each link was built at, so the winch works from the originals
    /// rather than compounding its own rounding.
    built_long: Vec<f32>,
    /// The shortest each may be wound to: the two things it ties, touching.
    ///
    /// A link shorter than that is a link pulling two bodies together while the
    /// contact between them pushes them apart, and the two fight. Wound right in
    /// without it, the full size example reported its chain 55% stretched, which
    /// was the contacts winning rather than the links failing.
    shortest: Vec<f32>,
    /// How much of the chain is paid out, against what it was built at.
    pub wound: f32,
    /// Where each brick is laid, which is where one comes to rest.
    resting: Vec<(Vec3, Quat)>,
    world: Vec<Aabb>,
    owed: f32,
    pub solver: Solver,
}

impl Default for Wrecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Wrecker {
    pub fn new() -> Self {
        let mut one = Self {
            bodies: Vec::new(),
            chain: Vec::new(),
            built_at: Vec::new(),
            built_long: Vec::new(),
            shortest: Vec::new(),
            wound: 1.0,
            resting: resting(),
            world: tray(),
            owed: 0.0,
            solver: Solver::new().with_passes(PASSES),
        };
        one.build();

        one
    }

    /// The chain, the ball and the wall, all fresh.
    pub fn build(&mut self) {
        self.bodies.clear();
        self.chain.clear();
        self.built_at.clear();
        self.wound = 1.0;
        self.solver.forget();

        self.bodies.push(Body::immovable(HANGS_FROM, BEAD * 0.5));

        for bead in 1..=BEADS {
            let at = HANGS_FROM - Vec3::Y * bead as f32 * SPACING;
            self.bodies.push(Body::new(at, BEAD * 0.5, BEAD_MASS));
            self.chain.push(Link::rope(bead - 1, bead, SPACING));
        }

        let ball = HANGS_FROM - Vec3::Y * ((BEADS as f32 + 1.0) * SPACING + BALL);
        self.bodies
            .push(Body::new(ball, BALL, BALL_MASS).with_friction(0.4));
        self.chain.push(Link::rope(BEADS, BALL_AT, SPACING + BALL));

        self.built_long = self.chain.iter().map(|link| link.length).collect();
        self.shortest = self
            .chain
            .iter()
            .map(|link| self.bodies[link.one].radius() + self.bodies[link.other].radius())
            .collect();

        self.lay_the_wall();
    }

    /// The wall again, with whatever the chain is doing left alone.
    ///
    /// This is what the bench's enter does. It built the lot, which put the ball
    /// back on its hook wherever it had got to: press it mid swing and the ball
    /// jumped across the tray. Rebuilding the wall is the thing that was asked
    /// for, and the chain is not part of the wall.
    ///
    /// The chain stays where it is and stops moving.
    ///
    /// Stopping it is the catch. Leaving the swing in left the ball crossing a
    /// brick's clearance inside one step, so a brick laid clear of it was buried
    /// in it by the next, and out it went: one left at twenty three a second and
    /// ended up off the bench. You cannot build a wall round a swinging ball.
    /// You hold it still and then build.
    ///
    /// Stopping it is not moving it. The ball hangs where it got to and the
    /// swing is yours to put back, which is the whole of what pressing this used
    /// to get wrong.
    ///
    /// A wall with nothing wrong with it is left alone. There is nothing to put
    /// right, and the solver would still have to take up its contacts again from
    /// nothing.
    pub fn rebuild_wall(&mut self) {
        if self.whole() {
            return;
        }

        for n in 1..WALL_FROM {
            self.bodies[n].velocity = Vec3::ZERO;
            self.bodies[n].spin = Vec3::ZERO;
        }

        self.bodies.truncate(WALL_FROM);
        self.built_at.clear();

        // the solver is holding contacts against bricks that no longer exist
        self.solver.forget();
        self.lay_the_wall();
    }

    /// Winds the chain in or out.
    pub fn wind(&mut self, by: f32) {
        let was = self.wound;
        self.wound = (self.wound + by).clamp(WOUND_IN, WOUND_OUT);
        if (self.wound - was).abs() < 1e-6 {
            return;
        }

        for ((link, built), shortest) in self
            .chain
            .iter_mut()
            .zip(&self.built_long)
            .zip(&self.shortest)
        {
            link.length = (built * self.wound).max(*shortest);
        }
        for n in 1..WALL_FROM {
            self.bodies[n].wake();
        }
    }

    /// Whether the wall is all there and all of it standing.
    pub fn whole(&self) -> bool {
        self.bodies.len() - WALL_FROM == self.resting.len() && self.standing() == self.resting.len()
    }

    /// The bricks, laid where they rest.
    ///
    /// Nothing is laid through the chain. A brick built inside something is a
    /// brick the solver has to get out of there, and with the ball forty times
    /// its weight what it does is throw it: one went fifty six units down
    /// through the bench. So the wall comes back with a hole where the chain is,
    /// and pressing again once it has swung clear fills the hole.
    ///
    /// The whole chain and not just the ball. The beads hang from 0.25 down and
    /// the wall stands 0.26 high, so swinging the ball in puts beads over the
    /// wall as well, and a brick laid inside a bead is thrown exactly as far.
    fn lay_the_wall(&mut self) {
        let chain: Vec<(Vec3, f32)> = self.bodies[1..WALL_FROM]
            .iter()
            .map(|body| (body.position, body.radius()))
            .collect();

        for (at, facing) in self.resting.clone() {
            let in_the_way = chain.iter().any(|(on, radius)| {
                let off = ((*on - at).abs() - BRICK).max(Vec3::ZERO);
                off.length() < radius + CLEAR
            });
            if in_the_way {
                continue;
            }

            self.bodies.push(
                Body::block(at, BRICK, BRICK_MASS)
                    .facing(facing)
                    .with_friction(0.7),
            );
            self.built_at.push(at);
        }
    }

    /// Pulls the ball along the floor. One press, one pull, so a swing is pumped
    /// rather than driven.
    ///
    /// Both ways about, not just at the wall. Pulling along the swing alone
    /// means the ball only ever meets the wall head on in the middle, and the
    /// whole of what a wrecking ball asks of you is where to put it.
    pub fn haul(&mut self, way: Vec3) {
        self.bodies[BALL_AT].velocity += way.normalize_or_zero() * HAUL;
        for n in 1..WALL_FROM {
            self.bodies[n].wake();
        }
    }

    /// Steps it at its own rate, so a slow frame does not change how it falls.
    pub fn advance(&mut self, dt: f32) {
        self.owed = (self.owed + dt).min(0.2);

        while self.owed >= STEP {
            self.solver.step_linked(
                &mut self.bodies,
                &self.chain,
                &self.world,
                crate::GRAVITY,
                STEP,
            );
            self.owed -= STEP;
        }
    }

    /// How many bricks are still where they were built.
    pub fn standing(&self) -> usize {
        self.bodies[WALL_FROM..]
            .iter()
            .zip(&self.built_at)
            .filter(|(brick, built)| brick.position.distance(**built) < KNOCKED)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How far past its own length the chain is hanging, as a fraction.
    ///
    /// Spec 0041's measurement, and it lives here rather than on the toy. It is a
    /// number about the solver, and the person standing at a wrecking ball is not
    /// asking one.
    fn stretch(one: &Wrecker) -> f32 {
        let now: f32 = one.chain.iter().map(|link| link.span(&one.bodies)).sum();
        let want: f32 = one.chain.iter().map(|link| link.length).sum();

        now / want - 1.0
    }

    /// Swings it the way a hand does: a pull every time the ball turns round and
    /// starts back towards the wall. Gives back how many pulls it took to put
    /// the first brick down, how many were down at the end, and the furthest
    /// anything got from the middle of the tray.
    fn pumped(one: &mut Wrecker, seconds: f32) -> (Option<usize>, usize, f32) {
        let whole = bricks();
        let mut pulls = 0usize;
        let mut first = None;
        let mut was = 0.0f32;
        let mut furthest: f32 = 0.0;

        for _ in 0..(seconds / STEP) as u32 {
            let going = one.bodies[BALL_AT].velocity.z;
            if (pulls == 0 || (was <= 0.0 && going > 0.0)) && pulls < 40 {
                one.haul(Vec3::Z);
                pulls += 1;
            }
            was = going;

            one.advance(STEP);

            if first.is_none() && one.standing() < whole {
                first = Some(pulls);
            }
            for body in &one.bodies[1..] {
                furthest = furthest.max(
                    (body.position.x.abs() / (TRAY_WIDE * 0.5))
                        .max(body.position.z.abs() / (TRAY_LONG * 0.5)),
                );
            }
        }

        (first, whole - one.standing(), furthest)
    }

    /// What the chain hangs at, against the bead weight and the passes.
    /// How far a settled brick has moved from where it was built.
    #[test]
    #[ignore]
    fn settling() {
        let mut one = Wrecker::new();
        for seconds in [0.5f32, 1.0, 3.0, 6.0] {
            for _ in 0..(seconds / STEP) as u32 {
                one.advance(STEP);
            }
            let worst = one.bodies[WALL_FROM..]
                .iter()
                .zip(&one.built_at)
                .map(|(brick, built)| brick.position.distance(*built))
                .fold(0.0f32, f32::max);
            println!(
                "after {}s a brick has moved {:.5}, which is {:.1}% of its height",
                seconds,
                worst,
                worst / (BRICK.y * 2.0) * 100.0
            );
        }
    }

    #[test]
    #[ignore]
    fn hanging() {
        for mass in [0.09f32, 0.15, 0.24, 0.4] {
            for passes in [32usize, 64, 128] {
                let mut one = Wrecker::new();
                one.solver = Solver::new().with_passes(passes);
                for body in &mut one.bodies[1..=BEADS] {
                    body.inverse_mass = 1.0 / mass;
                }
                for _ in 0..(4.0 / STEP) as u32 {
                    one.advance(STEP);
                }
                println!(
                    "bead {:.2} passes {}: chain {:+.2}%",
                    mass,
                    passes,
                    stretch(&one) * 100.0
                );
            }
        }
    }

    /// What gets out of the tray, against the brick weight.
    #[test]
    #[ignore]
    fn escapes() {
        for mass in [0.012f32, 0.03, 0.06, 0.12, 0.24] {
            let mut one = Wrecker::new();
            for body in &mut one.bodies[WALL_FROM..] {
                body.inverse_mass = 1.0 / mass;
            }
            let (first, down, furthest) = pumped(&mut one, 20.0);
            let gone = one.bodies[WALL_FROM..]
                .iter()
                .filter(|b| {
                    b.position.y < -BRICK.y
                        || b.position.x.abs() > TRAY_WIDE
                        || b.position.z.abs() > TRAY_LONG
                })
                .count();
            let fastest = one.bodies[WALL_FROM..]
                .iter()
                .map(|b| b.velocity.length())
                .fold(0.0f32, f32::max);

            println!(
                "brick {:.3}: first {:?}, {} of {} down, {} gone, furthest {:.2} trays, fastest {:.2}",
                mass, first, down, bricks(), gone, furthest, fastest
            );
        }
    }

    #[test]
    #[ignore]
    fn sweep() {
        for haul in [0.6f32, 0.7, 0.8, 0.9, 1.0, 1.2] {
            let mut one = Wrecker::new();
            // the sweep works the constant through the same door a press does
            let mut pulls = 0usize;
            let mut was = 0.0f32;
            let mut first = None;
            let whole = bricks();
            for _ in 0..(20.0 / STEP) as u32 {
                let going = one.bodies[BALL_AT].velocity.z;
                if (pulls == 0 || (was <= 0.0 && going > 0.0)) && pulls < 40 {
                    one.bodies[BALL_AT].velocity += Vec3::Z * haul;
                    for n in 1..WALL_FROM {
                        one.bodies[n].wake();
                    }
                    pulls += 1;
                }
                was = going;
                one.advance(STEP);
                if first.is_none() && one.standing() < whole {
                    first = Some(pulls);
                }
            }

            let mut hanging = Wrecker::new();
            for _ in 0..(4.0 / STEP) as u32 {
                hanging.advance(STEP);
            }

            println!(
                "haul {:.1}: first brick after {:?} pulls, {} of {} down, chain {:+.1}% worked, {:+.2}% hanging",
                haul,
                first,
                whole - one.standing(),
                whole,
                stretch(&one) * 100.0,
                stretch(&hanging) * 100.0
            );
        }
    }

    /// Spec 0006: it is built where the arithmetic says, and hangs there.
    #[test]
    fn it_is_built_hanging_still() {
        let one = Wrecker::new();

        assert_eq!(one.bodies.len(), WALL_FROM + bricks());
        assert_eq!(one.chain.len(), BEADS + 1);
        assert_eq!(one.standing(), bricks(), "it was built knocked down");
        assert_eq!(one.bodies[HOOK].inverse_mass, 0.0, "the hook can move");

        for (n, link) in one.chain.iter().enumerate() {
            assert!(
                link.error(&one.bodies).abs() < 1e-5,
                "link {} is built {} out",
                n,
                link.error(&one.bodies)
            );
        }

        // the ball clears the tray and the lip round it
        let ball = one.bodies[BALL_AT].position;
        assert!(
            ball.y - BALL > LIP,
            "the ball hangs {} up and the lip is {}",
            ball.y - BALL,
            LIP
        );
    }

    /// Spec 0006: the wall stands until something hits it.
    ///
    /// A stack of light bricks under a solver is the thing most likely to fall
    /// over on its own, and a wall that collapses while you walk up to it is a
    /// wall nobody knocked down.
    #[test]
    fn the_wall_stands_until_something_hits_it() {
        let mut one = Wrecker::new();
        for _ in 0..(6.0 / STEP) as u32 {
            one.advance(STEP);
        }

        assert_eq!(
            one.standing(),
            bricks(),
            "{} bricks fell over on their own",
            bricks() - one.standing()
        );
    }

    /// Spec 0006: the wall is built where it rests, so it does not slump.
    ///
    /// Laid on the arithmetic's own spots a brick moved 0.032 settling, which is
    /// 73% of its own height and a whisker under the distance that counts as
    /// knocked down. Laid where it rests it moves 0.0008.
    #[test]
    fn the_wall_is_built_where_it_rests() {
        let mut one = Wrecker::new();
        for _ in 0..(3.0 / STEP) as u32 {
            one.advance(STEP);
        }

        let worst = one.bodies[WALL_FROM..]
            .iter()
            .zip(&one.built_at)
            .map(|(brick, built)| brick.position.distance(*built))
            .fold(0.0f32, f32::max);

        assert!(
            worst < KNOCKED * 0.1,
            "a brick settled {} from where it was laid, and knocked down is {}",
            worst,
            KNOCKED
        );
    }

    /// Spec 0006: and pressing enter at a wall with nothing wrong with it does
    /// nothing at all.
    ///
    /// It laid every brick again, which put a settled one back on its spot, so
    /// clicking at a standing wall made the whole thing hop.
    #[test]
    fn rebuilding_a_whole_wall_changes_nothing() {
        let mut one = Wrecker::new();
        for _ in 0..(3.0 / STEP) as u32 {
            one.advance(STEP);
        }

        assert!(one.whole(), "the wall is not whole to begin with");
        let was: Vec<Vec3> = one.bodies.iter().map(|b| b.position).collect();
        one.rebuild_wall();

        for (n, before) in was.iter().enumerate() {
            assert_eq!(one.bodies[n].position, *before, "body {} moved", n);
        }
    }

    /// Spec 0006: and hauling it brings the wall down.
    ///
    /// Swept over the pull: at 0.6 the ball never reached, and at 2.4 one pull
    /// threw it clean over the top.
    #[test]
    fn hauling_it_brings_the_wall_down() {
        let mut one = Wrecker::new();
        let (first, down, _) = pumped(&mut one, 20.0);

        assert!(first.is_some(), "the ball never reached the wall");
        assert!(
            first.is_some_and(|pulls| pulls > 2),
            "it took {:?} pulls, so there is nothing to work",
            first
        );
        assert!(
            down > bricks() / 3,
            "only {} of {} came down",
            down,
            bricks()
        );
    }

    /// Spec 0006: and nothing ends up off the tray.
    #[test]
    fn nothing_leaves_the_tray() {
        let mut one = Wrecker::new();
        let (_, _, furthest) = pumped(&mut one, 20.0);

        // the ball swings out past the lip, which it is high enough to clear,
        // so this is about where things come to rest rather than where they go
        for (n, body) in one.bodies.iter().enumerate().skip(WALL_FROM) {
            let at = body.position;
            assert!(
                at.x.abs() < TRAY_WIDE * 0.5 + BRICK.x && at.y > -BRICK.y,
                "brick {} ended up at {:?}",
                n - WALL_FROM,
                at
            );
            assert!(
                at.z.abs() < TRAY_LONG * 0.5 + BRICK.x,
                "brick {} ended up at {:?}",
                n - WALL_FROM,
                at
            );
        }

        assert!(
            furthest < 2.0,
            "something went {} tray widths out",
            furthest
        );
    }

    /// Spec 0041's own measurement, at a thirteenth of the size: a chain
    /// carrying something far heavier than itself holds its length.
    ///
    /// Holds, not exactly. It hangs 5.5% long and that is the figure, the same
    /// order as the full size example's 4.6%. What this catches is it getting
    /// worse, which is what a chain being worked by the solver one link at a
    /// time does when any of the weights change.
    #[test]
    fn the_chain_carries_the_ball_without_stretching() {
        let mut one = Wrecker::new();
        for _ in 0..(4.0 / STEP) as u32 {
            one.advance(STEP);
        }
        let hanging = stretch(&one);

        assert!(
            hanging > 0.0 && hanging < 0.07,
            "the chain hangs {:.2}% long",
            hanging * 100.0
        );

        // and swinging it is no worse than a little
        let mut worst = hanging;
        let (_, _, _) = pumped(&mut one, 12.0);
        for _ in 0..(2.0 / STEP) as u32 {
            one.advance(STEP);
            worst = worst.max(stretch(&one));
        }

        assert!(
            worst < 0.1,
            "worked, the chain went {:.1}% long",
            worst * 100.0
        );
    }

    /// Spec 0006: winding it in hauls the ball up, and winding out lets it down.
    #[test]
    fn the_winch_moves_the_ball() {
        let mut one = Wrecker::new();
        for _ in 0..(2.0 / STEP) as u32 {
            one.advance(STEP);
        }
        let hung = one.bodies[BALL_AT].position.y;

        for _ in 0..40 {
            one.wind(-WINDS_BY);
        }
        assert_eq!(one.wound, WOUND_IN, "the winch did not reach its stop");
        for _ in 0..(4.0 / STEP) as u32 {
            one.advance(STEP);
        }

        assert!(
            one.bodies[BALL_AT].position.y > hung + 0.1,
            "wound right in the ball is at {} against {}",
            one.bodies[BALL_AT].position.y,
            hung
        );

        // and no link was wound shorter than the two things it ties, which is a
        // link and a contact pulling against each other
        for (n, link) in one.chain.iter().enumerate() {
            let touching = one.bodies[link.one].radius() + one.bodies[link.other].radius();
            assert!(
                link.length >= touching - 1e-6,
                "link {} was wound to {} and the things it ties are {} across",
                n,
                link.length,
                touching
            );
        }
    }

    /// Spec 0006: and it can be stood back up.
    #[test]
    fn building_it_again_stands_the_wall_back_up() {
        let mut one = Wrecker::new();
        pumped(&mut one, 20.0);
        assert!(one.standing() < bricks());

        one.build();

        assert_eq!(one.standing(), bricks(), "the wall came back knocked down");
        for link in &one.chain {
            assert!(link.error(&one.bodies).abs() < 1e-5);
        }
    }

    /// Spec 0006: and standing it back up does not move the ball.
    ///
    /// Enter rebuilt everything, so pressing it mid swing put the ball back on
    /// its hook. What it says it does is build the wall again.
    #[test]
    fn rebuilding_the_wall_leaves_the_chain_where_it_is() {
        let mut one = Wrecker::new();
        pumped(&mut one, 14.0);
        assert!(one.standing() < bricks(), "nothing was knocked down");

        let chain: Vec<Vec3> = one.bodies[..WALL_FROM].iter().map(|b| b.position).collect();
        one.rebuild_wall();

        assert!(
            one.standing() > bricks() - 4,
            "the wall came back with {} of {} standing",
            one.standing(),
            bricks()
        );
        assert_eq!(
            one.standing(),
            one.bodies.len() - WALL_FROM,
            "a brick fell in"
        );
        for (n, was) in chain.iter().enumerate() {
            let moved = one.bodies[n].position.distance(*was);
            assert!(moved < 1e-5, "body {} jumped {}", n, moved);
        }

        // caught rather than carried on, since a wall cannot be built round a
        // swinging ball
        for n in 1..WALL_FROM {
            assert_eq!(
                one.bodies[n].velocity,
                Vec3::ZERO,
                "body {} is still going",
                n
            );
        }
    }

    /// Spec 0006: and rebuilding round the ball does not throw the wall.
    #[test]
    fn rebuilding_round_the_ball_throws_nothing() {
        let mut one = Wrecker::new();

        // swing it in and build the wall again while it is sitting in the way
        pumped(&mut one, 14.0);
        for _ in 0..12 {
            one.haul(Vec3::Z);
            for _ in 0..(0.1 / STEP) as u32 {
                one.advance(STEP);
            }
        }
        one.rebuild_wall();
        for _ in 0..(6.0 / STEP) as u32 {
            one.advance(STEP);
        }

        for (n, body) in one.bodies.iter().enumerate().skip(WALL_FROM) {
            let at = body.position;
            assert!(
                at.y > -BRICK.y
                    && at.x.abs() < TRAY_WIDE * 0.5 + BRICK.x
                    && at.z.abs() < TRAY_LONG * 0.5 + BRICK.x,
                "brick {} was thrown to {:?}",
                n - WALL_FROM,
                at
            );
        }
    }
}
