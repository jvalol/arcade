//! A Newton's cradle on a bench. Spec 0006.
//!
//! Five balls hung in a line, touching. Lift the one on the end and let it go,
//! and it stops dead on the others while the one at the far end flies out. The
//! middle three barely move.
//!
//! It is the first thing in this project that asks the link solver and the
//! contact solver to work together rather than take turns: the ropes hold the
//! balls on their arcs while a blow travels the row through four contacts. Spec
//! 0041 gave the engine the ropes; this is what they were for.
//!
//! Where everything sits is arithmetic and is checked without a window.

use blitzkit::collision::Aabb;
use blitzkit::link::Link;
use blitzkit::physics::{Body, Solver};
use glam::{vec3, Vec3};

/// How many balls hang.
pub const BALLS: usize = 5;

pub const BALL: f32 = 0.085;
pub const BALL_MASS: f32 = 1.0;

/// How long a rope is, from its hook to the middle of its ball.
pub const ROPE: f32 = 0.52;

/// The frame: how high the bar sits above the bench, and how thick the
/// uprights and the bar are.
pub const BAR_UP: f32 = ROPE + BALL * 2.0 + 0.18;
pub const STOCK: f32 = 0.045;

/// How much of a blow a ball gives back.
///
/// High, because the whole point is that the row passes it along rather than
/// swallowing it. Not one: at one the thing never stops and the row drifts,
/// which is a perpetual motion machine and a bug.
pub const BOUNCE: f32 = 0.92;

/// How far the end ball is lifted when it is set going, as an angle.
pub const LIFTED: f32 = 0.9;

/// How fast the toy is stepped, and how many passes a step gets.
///
/// Faster and harder than the engine's default. A row of touching balls is the
/// case sequential impulses are worst at: each pass moves a blow one contact
/// along, so four contacts want four passes just to carry it end to end, and
/// that is before anything is solved.
pub const STEP: f32 = 1.0 / 240.0;
pub const PASSES: usize = 48;

/// Where a ball hangs when the row is at rest, measured from the middle of the
/// bench top.
///
/// Touching, not spaced: the gap between two balls is what a blow has to cross,
/// and a cradle with gaps is a row of pendulums that clack at random.
pub fn resting(ball: usize) -> Vec3 {
    let across = (ball as f32 - (BALLS - 1) as f32 * 0.5) * BALL * 2.0;

    vec3(0.0, -ROPE, across)
}

/// Where a ball's rope is hooked to the bar.
pub fn hook(ball: usize) -> Vec3 {
    let at = resting(ball);

    vec3(at.x, BAR_UP, at.z)
}

/// The bodies and the ropes, in a frame whose origin is the middle of the
/// bench top. The hooks come first, then the balls.
pub fn strung() -> (Vec<Body>, Vec<Link>) {
    let mut bodies = Vec::new();
    let mut links = Vec::new();

    for ball in 0..BALLS {
        bodies.push(Body::immovable(hook(ball), STOCK * 0.5));
    }
    for ball in 0..BALLS {
        bodies.push(
            Body::new(hook(ball) + Vec3::NEG_Y * ROPE, BALL, BALL_MASS)
                .with_restitution(BOUNCE)
                .with_friction(0.02),
        );
    }
    for ball in 0..BALLS {
        links.push(Link::rope(ball, BALLS + ball, ROPE));
    }

    (bodies, links)
}

/// Which body in `strung` is a given ball.
pub fn ball_at(ball: usize) -> usize {
    BALLS + ball
}

/// Swings the end ball up and lets it go.
///
/// Placed rather than shoved: a ball given a velocity in a row that is already
/// touching spends it on the contact in front of it before it has anywhere to
/// swing from.
pub fn set_going(bodies: &mut [Body], from_the_end: usize) {
    let ball = ball_at(from_the_end.min(BALLS - 1));
    let hook = hook(from_the_end.min(BALLS - 1));
    let away = if from_the_end == 0 { -1.0 } else { 1.0 };

    bodies[ball].position = hook + vec3(0.0, -ROPE * LIFTED.cos(), away * ROPE * LIFTED.sin());
    bodies[ball].velocity = Vec3::ZERO;
    bodies[ball].spin = Vec3::ZERO;
    bodies[ball].wake();
}

/// A solver set up the way a cradle wants one.
pub fn solver() -> Solver {
    Solver::new().with_passes(PASSES)
}

/// How much of the row is moving, which is what says whether it has settled.
pub fn stirring(bodies: &[Body]) -> f32 {
    (0..BALLS)
        .map(|ball| bodies[ball_at(ball)].velocity.length())
        .fold(0.0f32, f32::max)
}

/// Nothing to collide with: the balls meet each other and the ropes hold them.
/// The bench is below them and they never reach it.
pub const NO_WORLD: [Aabb; 0] = [];

#[cfg(test)]
mod tests {
    use super::*;

    const GRAVITY: Vec3 = Vec3::new(0.0, -9.81, 0.0);

    fn run(bodies: &mut [Body], links: &[Link], seconds: f32) {
        let mut solver = solver();
        for _ in 0..(seconds / STEP) as u32 {
            solver.step_linked(bodies, links, &NO_WORLD, GRAVITY, STEP);
        }
    }

    #[test]
    fn the_balls_hang_touching() {
        let (bodies, _) = strung();

        for ball in 1..BALLS {
            let apart = bodies[ball_at(ball)]
                .position
                .distance(bodies[ball_at(ball - 1)].position);

            assert!(
                (apart - BALL * 2.0).abs() < 1e-5,
                "balls {} and {} hang {} apart",
                ball - 1,
                ball,
                apart
            );
        }
    }

    #[test]
    fn every_ball_hangs_from_its_own_hook() {
        let (bodies, links) = strung();

        assert_eq!(links.len(), BALLS);
        for (ball, link) in links.iter().enumerate() {
            assert!((link.span(&bodies) - ROPE).abs() < 1e-5);
            assert_eq!(link.one, ball);
            assert_eq!(link.other, ball_at(ball));
            assert_eq!(bodies[link.one].inverse_mass, 0.0, "the hook moved");
        }
    }

    #[test]
    fn a_row_left_alone_stays_put() {
        let (mut bodies, links) = strung();
        run(&mut bodies, &links, 3.0);

        for ball in 0..BALLS {
            let moved = bodies[ball_at(ball)]
                .position
                .distance(hook(ball) + Vec3::NEG_Y * ROPE);
            assert!(moved < 0.02, "ball {} drifted {}", ball, moved);
        }
    }

    #[test]
    fn lifting_one_end_swings_the_other() {
        let (mut bodies, links) = strung();
        set_going(&mut bodies, 0);

        // the blow has to cross four contacts, so give it the time to
        let mut furthest: f32 = 0.0;
        let mut solver = solver();
        for _ in 0..(1.2 / STEP) as u32 {
            solver.step_linked(&mut bodies, &links, &NO_WORLD, GRAVITY, STEP);

            let far =
                bodies[ball_at(BALLS - 1)].position.z - (hook(BALLS - 1) + Vec3::NEG_Y * ROPE).z;
            furthest = furthest.max(far);
        }

        assert!(
            furthest > BALL * 2.0,
            "the far ball only moved {}",
            furthest
        );
    }

    #[test]
    fn the_middle_is_left_alone() {
        let (mut bodies, links) = strung();
        let settled = hook(2) + Vec3::NEG_Y * ROPE;
        set_going(&mut bodies, 0);

        let mut worst: f32 = 0.0;
        let mut solver = solver();
        for _ in 0..(1.2 / STEP) as u32 {
            solver.step_linked(&mut bodies, &links, &NO_WORLD, GRAVITY, STEP);
            worst = worst.max(bodies[ball_at(2)].position.distance(settled));
        }

        // it is shoved about a little, and it does not swing out
        assert!(worst < BALL * 2.0, "the middle swung {}", worst);
    }

    #[test]
    fn it_runs_down_rather_than_on_forever() {
        let (mut bodies, links) = strung();
        set_going(&mut bodies, 0);
        run(&mut bodies, &links, 30.0);

        assert!(
            stirring(&bodies) < 0.4,
            "still going at {} after half a minute",
            stirring(&bodies)
        );
    }
}
