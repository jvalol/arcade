//! cascada, which is a toy and had a cabinet. Spec 0006.
//!
//! Stand dominoes up on a floor wherever you like, push the first one, and the
//! number at the end is how many fell. There is nothing to win and nothing to
//! lose, which is the whole of what separates the nook from the hall. It had a
//! cabinet in the hall for the same reason everything else does, which is that
//! it is a repo under `games`, and that was the only reason.
//!
//! It is the one toy in the nook that opens a window. The other five are worked
//! where they stand because a bench is enough room for them. This one is not: a
//! domino run wants a floor and wants you to put every domino exactly where you
//! want it, and four arrow keys at a bench cannot do that. So the bench is what
//! it is and the window is where you do it, which is also the honest reading of
//! a toy that already has a repo, a spec folder and a readme of its own.
//!
//! The dominoes on the bench do nothing. They are a picture of the toy, the way
//! a cabinet's screen is a picture of the game, and the first of them is leaning
//! into the second because that is the only part of a domino run anybody needs
//! explaining.

use glam::{vec3, Vec3};

/// What the bench is called, which is what the room says when you look at it.
pub const NAME: &str = "cascada";

/// How much of the nook's length its bench takes.
pub const LONG: f32 = 1.1;

/// A domino: wide across the run, tall, and thin the way it falls, which is
/// along the run.
///
/// Which axis is the thin one is the whole of whether this is a domino run or
/// eleven dominoes in a row. Thin across the run they stand with their faces
/// to you and fall sideways into nothing, and it looks right from beside the
/// bench, because from beside a real run you see edges and not faces.
pub const DOMINO: Vec3 = vec3(0.042, 0.082, 0.016);

/// How many stand on the bench, and how far apart along the run.
///
/// Closer than a domino is tall, which is cascada's own rule and the one thing
/// its readme spends its words on: too far and a falling one lands short of the
/// next, too close and it has nowhere to swing before it meets it.
pub const STANDING: usize = 11;
pub const APART: f32 = 0.062;

/// How far the run bends off straight, and how far over the first one is.
///
/// The bend is paid for in spacing and it is not cheap. `APART` is the step
/// along the run and the gap between two dominoes is that step times the arc,
/// so a curve steep enough to look laid by hand pushes them further apart than
/// the number says. At 0.1 the gap came out 0.085 against a domino 0.082 tall,
/// which in cascada is a run that stops. At 0.06 it is 0.073.
pub const BEND: f32 = 0.06;
pub const TIPPED: f32 = 0.5;

/// How a domino standing here is turned, given its yaw and how far over it is.
///
/// The lean goes on inside the yaw, so it tips along the run rather than across
/// it. Which sign of the lean is forwards is not worth arguing about from the
/// handedness of a quaternion: [`tops_at`] says where the top of it ends up and
/// a test reads that.
pub fn stood(turn: f32, lean: f32) -> glam::Quat {
    glam::Quat::from_rotation_y(turn) * glam::Quat::from_rotation_x(lean)
}

/// Where the top of one ends up, and how far its middle drops getting there.
///
/// A domino going over pivots on an edge, so it sits lower than one standing up
/// and leans from there.
pub fn tops_at(at: Vec3, turn: f32, lean: f32) -> (Vec3, f32) {
    let drop = DOMINO.y * 0.5 * (1.0 - lean.cos());
    let middle = at - Vec3::Y * drop;

    (
        middle + stood(turn, lean) * (Vec3::Y * DOMINO.y * 0.5),
        drop,
    )
}

/// Where each domino stands on the bench top, how it is turned, and how far it
/// is over.
///
/// A curve rather than a line, because a line of dominoes is a picture of a
/// row and a curve is a picture of somebody having laid them.
pub fn laid() -> Vec<(Vec3, f32, f32)> {
    let last = (STANDING - 1) as f32;
    let spots: Vec<Vec3> = (0..STANDING)
        .map(|n| {
            let along = (n as f32 - last * 0.5) * APART;
            let across = BEND * (along / (last * APART) * std::f32::consts::TAU).sin();

            vec3(across, DOMINO.y * 0.5, along)
        })
        .collect();

    (0..STANDING)
        .map(|n| {
            // turned to face the one in front of it rather than along the curve
            // it is standing on. Those are not the same thing: the tangent where
            // a domino stands and the line to the next one are a few degrees
            // apart on any curve, and what a domino has to fall onto is the next
            // one. The last stands the way the one behind it points.
            let (from, to) = if n + 1 < STANDING {
                (n, n + 1)
            } else {
                (n - 1, n)
            };
            let way = spots[to] - spots[from];

            (
                spots[n],
                way.x.atan2(way.z),
                if n == 0 { TIPPED } else { 0.0 },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0006: they stand on the bench and none of them hangs off it.
    #[test]
    fn they_all_stand_on_the_bench() {
        let bench = crate::room::BENCH_WIDE;

        for (at, _, _) in laid() {
            let reach = DOMINO.x.max(DOMINO.z) * 0.5;

            assert!(
                at.x.abs() + reach < bench * 0.5,
                "a domino at {} is over the edge of a bench {} across",
                at.x,
                bench
            );
            assert!(
                at.z.abs() + reach < LONG * 0.5,
                "a domino at {} is off the end of a bench {} long",
                at.z,
                LONG
            );
            assert!(at.y > 0.0, "a domino is under the bench top");
        }
    }

    /// Spec 0006: and they are spaced the way cascada says to space them.
    ///
    /// Closer together than one is tall, or a falling domino lands short of the
    /// next and the run stops. The toy's own readme is about nothing else.
    #[test]
    fn they_are_close_enough_to_knock_each_other_over() {
        let laid = laid();

        assert_eq!(laid.len(), STANDING);
        for (n, (at, _, _)) in laid.iter().enumerate().skip(1) {
            let gap = at.distance(laid[n - 1].0);

            assert!(
                gap < DOMINO.y,
                "two are {} apart and a domino is {} tall",
                gap,
                DOMINO.y
            );
            assert!(
                gap > DOMINO.z * 2.0,
                "two are {} apart and a domino is {} thick",
                gap,
                DOMINO.z
            );
        }
    }

    /// Spec 0006: and each one falls onto the next rather than past it.
    ///
    /// The thin axis and the turn have to agree with where the run goes. They
    /// did not: a domino was thin across the run, so every one of them would
    /// have gone over sideways into nothing. From beside the bench it looked
    /// right, because from beside a real run you see edges and not faces, and
    /// this showed eleven faces and read as a curve of dominoes anyway.
    #[test]
    fn each_one_falls_onto_the_next() {
        let laid = laid();

        for (n, (at, turn, _)) in laid.iter().enumerate().take(laid.len() - 1) {
            let falls = glam::Quat::from_rotation_y(*turn) * Vec3::Z;
            let next = (laid[n + 1].0 - *at).normalize();

            assert!(
                falls.dot(next) > 0.99,
                "domino {} falls {:?} and the next one is {:?}",
                n,
                falls,
                next
            );
        }
    }

    /// Spec 0006: and it is over towards the next one, not away from it.
    #[test]
    fn the_leaning_one_leans_the_right_way() {
        let laid = laid();
        let (at, turn, lean) = laid[0];
        let (top, drop) = tops_at(at, turn, lean);

        assert!(drop > 0.0, "a domino going over does not pivot on an edge");
        assert!(
            top.distance(laid[1].0) < at.distance(laid[1].0),
            "the top is {} from the next one and the base is {}",
            top.distance(laid[1].0),
            at.distance(laid[1].0)
        );
        // and the ones behind it stand up
        let (upright, _) = tops_at(laid[4].0, laid[4].1, laid[4].2);
        assert!((upright.x - laid[4].0.x).abs() < 1e-5);
        assert!((upright.z - laid[4].0.z).abs() < 1e-5);
    }

    /// Spec 0006: the first one is over, because that is the part that needs
    /// explaining.
    #[test]
    fn the_first_one_is_leaning() {
        let laid = laid();

        assert!(laid[0].2 > 0.1, "nothing on the bench says to push it");
        assert!(
            laid[1..].iter().all(|(_, _, lean)| *lean == 0.0),
            "the run has already gone over"
        );
    }
}
