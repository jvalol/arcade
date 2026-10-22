//! The other corridor, where the same rooms are further apart. Spec 0035.
//!
//! **The hotel's list, re-spaced.** Spec 0029's corridor has a door every
//! 0.75 metres and a room for every counting number, and its sign says the
//! rooms do not run out. Nothing in that claim says they have to be
//! reachable. Door n is at the nth Fibonacci metre here, so the numbers go
//! on exactly as they did and you cannot count to twenty by walking.
//!
//! Both corridors are two kilometres long. One of them holds two thousand
//! six hundred rooms. This one holds sixteen.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::THICK;

/// How far along door `n` stands, in metres from the mouth.
///
/// The Fibonacci numbers with the repeated one dropped, so the doors are
/// 1, 2, 3, 5, 8, 13 and on rather than starting with two in the same place.
/// Counted from one, like the rooms.
pub fn reach(n: u64) -> u64 {
    let (mut was, mut is) = (1u64, 2u64);

    for _ in 1..n.max(1) {
        let next = was + is;

        was = is;
        is = next;
    }

    was
}

/// How long the corridor is, which is the hotel's own length.
///
/// **The same two kilometres, deliberately.** It is the comparison the whole
/// room is: the same hall, the same length, and sixteen doors in it instead
/// of two and a half thousand.
pub const LONG: f32 = 2000.0;

/// How many doors fit in it.
///
/// Sixteen, because `reach(16)` is 1597 and `reach(17)` is 2584, which is
/// past the end. Worked out rather than written down, so the two numbers
/// cannot drift.
pub fn doors() -> u64 {
    (1..).take_while(|n| reach(*n) as f32 <= LONG).count() as u64
}

/// How wide the corridor is.
///
/// **Narrower than the hotel's 3.2, because of where it has to leave from.**
/// The lobby's far wall is the only clear one and the desk is in the middle
/// of it, which leaves 2.7 at either end. Inset from the lobby's own east
/// wall so the two are not in one plane, that is 2.4.
///
/// The doors are the hotel's, which is spec 0035's own condition. A corridor
/// is not a door.
pub const ACROSS: f32 = 2.4;

/// Where the middle of it runs, which is between the desk and the lobby's
/// east wall.
pub fn middle_x() -> f32 {
    let (_, east) = crate::aleph::span();

    east - 0.2 - ACROSS * 0.5
}

/// Where it leaves the lobby, which is that room's far wall.
pub fn mouth(reaches: f32) -> f32 {
    crate::aleph::across(reaches).0
}

/// And how far it reaches, which is away from the lobby.
pub fn end(reaches: f32) -> f32 {
    mouth(reaches) - LONG
}

/// Which side door `n` is on, alternating the way the hotel's do.
pub fn on_the_left(n: u64) -> bool {
    n % 2 == 1
}

/// Where door `n` stands.
pub fn at(reaches: f32, n: u64) -> Vec3 {
    let out = if on_the_left(n) { -1.0 } else { 1.0 };

    vec3(
        middle_x() + out * ACROSS * 0.5,
        0.0,
        mouth(reaches) - reach(n) as f32,
    )
}

/// What the hall is called, over the line that counts for you.
pub const NAME: &str = "The long hall";

/// How far along somebody is, in metres from the mouth.
pub fn walked(reaches: f32, at: Vec3) -> f32 {
    (mouth(reaches) - at.z).max(0.0)
}

/// How many doors they have put behind them.
pub fn passed(reaches: f32, at: Vec3) -> u64 {
    let gone = walked(reaches, at);

    (1..=doors()).filter(|n| reach(*n) as f32 <= gone).count() as u64
}

/// And how many of the hotel's the same walk would have been.
///
/// **This is the whole room, and until now it was nowhere.** The hall's
/// claim is that these are the hotel's rooms re-spaced, and a reader could
/// only get that by walking both and remembering. Jake said it is a long
/// hall with doors you cannot open, which is what a claim nobody states
/// looks like from inside.
///
/// The hotel's doors are `APART * 0.5` apart, which is where the number
/// comes from rather than from a constant written twice.
pub fn the_hotel_in(metres: f32) -> u64 {
    (metres / (crate::aleph::APART * 0.5)) as u64
}

/// The fittings, one beside each door, and the lamps that are those lifted.
///
/// **Derived from the doors rather than spaced along the hall.** Lamps every
/// so many metres would light the near end evenly and leave everything past
/// door 10 in the dark, which is the wrong lie: the corridor is not empty out
/// there, it is just long. A sconce by every door means the light thins out
/// exactly as the doors do, so what you see going away from you is the
/// sequence itself.
///
/// The cellar was told once that every lamp sits in a fitting and the baths
/// did it again worse, so these come from one list.
pub fn sconces(reaches: f32) -> Vec<Vec3> {
    (1..=doors())
        .map(|n| {
            let stands = at(reaches, n);
            let out = if on_the_left(n) { -1.0 } else { 1.0 };

            vec3(
                stands.x - out * crate::aleph::SCONCE_OUT,
                crate::aleph::SCONCE_UP,
                stands.z + crate::aleph::PLAQUE_BESIDE * 1.6,
            )
        })
        .collect()
}

pub fn lamps(reaches: f32) -> Vec<Vec3> {
    sconces(reaches)
        .into_iter()
        .map(|at| at + Vec3::Y * crate::aleph::SCONCE.y * 0.3)
        .collect()
}

/// Everything solid in here: the floor, two walls, the ceiling.
///
/// Cut for the mouth the way the gallery is cut for the hotel, which the
/// lobby's own far wall does.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let middle = middle_x();
    let from = mouth(reaches);
    let to = end(reaches);
    let tall = crate::aleph::TALL;

    vec![
        // the floor, lapping under the lobby's rather than meeting it: two
        // floors at one height that meet edge to edge stop you dead, and
        // this building found that three times in one day
        Aabb::new(
            vec3(middle - ACROSS * 0.5 - THICK, -THICK, to - THICK),
            vec3(middle + ACROSS * 0.5 + THICK, 0.0, from + THICK),
        ),
        // the two long walls
        Aabb::new(
            vec3(middle - ACROSS * 0.5 - THICK, 0.0, to - THICK),
            vec3(middle - ACROSS * 0.5, tall, from),
        ),
        Aabb::new(
            vec3(middle + ACROSS * 0.5, 0.0, to - THICK),
            vec3(middle + ACROSS * 0.5 + THICK, tall, from),
        ),
        // the far end, and the ceiling over the lot
        Aabb::new(
            vec3(middle - ACROSS * 0.5 - THICK, 0.0, to - THICK),
            vec3(middle + ACROSS * 0.5 + THICK, tall, to),
        ),
        Aabb::new(
            vec3(middle - ACROSS * 0.5 - THICK, tall, to - THICK),
            vec3(middle + ACROSS * 0.5 + THICK, tall + THICK, from + THICK),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0035: door n stands at F(n) metres, and the gaps are the
    /// sequence again.
    #[test]
    fn the_doors_are_where_the_sequence_says() {
        let want = [
            1u64, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597,
        ];

        for (n, metres) in want.iter().enumerate() {
            assert_eq!(reach(n as u64 + 1), *metres, "door {}", n + 1);
        }

        // **The corridor is made of the same numbers twice.** The gap from
        // door n to door n+1 is itself a Fibonacci number, two places back.
        for n in 3..=14u64 {
            assert_eq!(
                reach(n + 1) - reach(n),
                reach(n - 1),
                "the gap after door {} is not in the sequence",
                n
            );
        }
    }

    /// Spec 0035: the same two kilometres that hold two thousand six hundred
    /// of the hotel's doors hold sixteen of these.
    ///
    /// That sentence is the room, so it is worth a test rather than a
    /// comment that stops being true.
    #[test]
    fn the_same_corridor_holds_sixteen() {
        assert_eq!(doors(), 16);
        assert!(reach(16) as f32 <= LONG);
        assert!(reach(17) as f32 > LONG);

        let hotel = (LONG / (crate::aleph::APART * 0.5)) as u64;

        assert!(
            hotel > doors() * 150,
            "the hotel holds {} in the same length and this holds {}",
            hotel,
            doors()
        );
    }

    /// Spec 0035: every gap is about 1.618 times the one before it, which is
    /// the thing you feel rather than read.
    #[test]
    fn every_gap_is_the_golden_ratio_of_the_last() {
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        let ratio = |n: u64| (reach(n + 1) - reach(n)) as f64 / (reach(n) - reach(n - 1)) as f64;

        // **It converges rather than being constant**, which is the true
        // claim and not the one this test made first. 8/5 is 1.600 and 13/8
        // is 1.625: the early gaps straddle the ratio rather than sitting on
        // it, and asserting the limit at door 6 failed on the sixth.
        for n in 4..=13u64 {
            assert!(
                (ratio(n + 1) - phi).abs() < (ratio(n) - phi).abs(),
                "door {} is no closer to the ratio than door {}",
                n + 1,
                n
            );
        }

        // and by the time anybody has walked that far it is the ratio
        for n in 8..=14u64 {
            assert!(
                (ratio(n) - phi).abs() < 0.005,
                "the gap after door {} is {:.4} times the last, not {:.4}",
                n,
                ratio(n),
                phi
            );
        }
    }

    /// Spec 0035: you can reach door 11 and not door 16.
    ///
    /// **The room is the giving up.** Eleven is a hundred and forty four
    /// metres, which is half a minute at a run and a thing somebody does.
    /// Sixteen is most of a mile and nobody is going to.
    #[test]
    fn you_can_reach_eleven_and_not_sixteen() {
        let running = crate::SPEED * crate::RUNS;

        assert!(
            reach(11) as f32 / running < 20.0,
            "door 11 is {:.0}s away at a run",
            reach(11) as f32 / running
        );
        assert!(
            reach(16) as f32 / running > 150.0,
            "door 16 is only {:.0}s away at a run",
            reach(16) as f32 / running
        );
    }

    /// Spec 0035: a door here is a door there.
    ///
    /// Nothing about a door says which corridor it is in, because if they
    /// looked different the room would be making some other claim.
    #[test]
    fn nothing_about_a_door_says_which_corridor_it_is_in() {
        let reaches = reaches();

        for n in 1..=doors() {
            let here = at(reaches, n);
            let out = if on_the_left(n) { -1.0 } else { 1.0 };

            // in the wall, on alternating sides, like the hotel's
            assert!(
                (here.x - (middle_x() + out * ACROSS * 0.5)).abs() < 1e-4,
                "door {} is not in a wall",
                n
            );
            assert_ne!(
                on_the_left(n),
                on_the_left(n + 1),
                "doors {} and {} are on the same side",
                n,
                n + 1
            );
        }

        // **And the hotel's own door fits where this corridor puts it**,
        // read off the walls that are built rather than off the constants
        // they are built from, which is an assertion with a constant value
        // and proves only that I can type.
        let walls: Vec<_> = solid(reaches)
            .into_iter()
            .filter(|box_| box_.min.y <= 0.0 && box_.max.y >= crate::aleph::DOOR_TALL)
            .collect();

        for n in 1..=doors() {
            let stands = at(reaches, n);
            let leaf = Aabb::new(
                vec3(
                    stands.x - crate::aleph::DOOR_THICK,
                    0.0,
                    stands.z - crate::aleph::DOOR_WIDE * 0.5,
                ),
                vec3(
                    stands.x + crate::aleph::DOOR_THICK,
                    crate::aleph::DOOR_TALL,
                    stands.z + crate::aleph::DOOR_WIDE * 0.5,
                ),
            );

            assert!(
                walls.iter().any(|wall| wall.intersects(&leaf)),
                "door {} is not in a wall of this corridor",
                n
            );
        }

        // and what is left to walk down is wider than a body, measured
        // between the inner faces of the two walls that are built
        let middle = middle_x();
        let left = walls
            .iter()
            .filter(|wall| wall.max.x <= middle)
            .map(|wall| wall.max.x)
            .fold(f32::MIN, f32::max);
        let right = walls
            .iter()
            .filter(|wall| wall.min.x >= middle)
            .map(|wall| wall.min.x)
            .fold(f32::MAX, f32::min);

        assert!(
            right - left - crate::RADIUS * 2.0 > 1.0,
            "the corridor is {:.2} clear for a body {:.2} across",
            right - left,
            crate::RADIUS * 2.0
        );
    }

    /// Spec 0035: the hall says what the same walk buys next door.
    ///
    /// **A hall of doors you cannot open is a diagram you walk through.**
    /// The comparison is the room and it was nowhere in it: you could only
    /// get it by walking both corridors and remembering. Now it counts for
    /// you, and the two numbers move at wildly different rates while you are
    /// the one moving both of them.
    #[test]
    fn it_says_what_the_same_walk_buys_next_door() {
        let reaches = reaches();
        let mouth = mouth(reaches);
        let stood = |metres: f32| glam::vec3(middle_x(), 0.0, mouth - metres);

        // at the mouth, nothing is behind you
        assert_eq!(passed(reaches, stood(0.0)), 0);
        assert_eq!(the_hotel_in(0.0), 0);

        // door 12 is 233 metres, and 233 metres of the hotel is room 310
        assert_eq!(passed(reaches, stood(233.0)), 12);
        assert_eq!(the_hotel_in(233.0), 310);

        // **The point is that the gap widens, not that it is wide.** It is
        // four to one at twenty metres and twenty six to one at two hundred,
        // so asserting a flat ratio fails at the near end, which is where
        // anybody actually is.
        let mut was = 0.0f32;

        for metres in [20.0f32, 60.0, 150.0, 400.0, 900.0] {
            let here = passed(reaches, stood(metres)).max(1);
            let there = the_hotel_in(metres);
            let ratio = there as f32 / here as f32;

            assert!(
                ratio > was,
                "at {:.0} metres the hall is {} to the hotel's {}, which is no further behind than before",
                metres,
                here,
                there
            );

            was = ratio;
        }

        // and by the time it matters it is not close
        assert!(the_hotel_in(400.0) > passed(reaches, stood(400.0)) * 20);

        // and walking back does not count a door twice
        assert!(passed(reaches, stood(-5.0)) == 0);
    }

    /// The building's own rule: every lamp sits in a fitting, and the light
    /// thins out the way the doors do.
    #[test]
    fn every_lamp_is_in_a_fitting_and_they_thin_out() {
        let reaches = reaches();
        let fittings = sconces(reaches);

        assert_eq!(fittings.len() as u64, doors(), "a fitting is missing");

        for lamp in lamps(reaches) {
            assert!(
                fittings
                    .iter()
                    .any(|at| (*at - lamp).length() < crate::aleph::SCONCE.y),
                "a lamp at {:?} is in no fitting",
                lamp
            );
        }

        // and the gaps between them grow, because they are the doors' gaps:
        // the light going away from you is the sequence
        for n in 4..=12u64 {
            let near = (fittings[n as usize] - fittings[n as usize - 1]).length();
            let far = (fittings[n as usize + 1] - fittings[n as usize]).length();

            assert!(
                far > near,
                "the lamps stop thinning out between {} and {}",
                n,
                n + 1
            );
        }
    }

    /// Spec 0035: it leaves the lobby and nothing of it is in another room.
    #[test]
    fn nothing_of_it_is_in_another_room() {
        let reaches = reaches();
        let mine = solid(reaches);

        // clear of the desk, which is on the same wall
        let desk = crate::aleph::desk(reaches);

        for one in mine.iter() {
            assert!(
                !one.intersects(&desk),
                "a piece of the corridor is inside the desk"
            );
        }

        // and clear of the register, which is the other thing out that way
        let (west, east) = crate::register::along(reaches);
        let (far, near) = crate::register::across(reaches);
        let book = Aabb::new(
            vec3(west - THICK, -1.0, far - THICK),
            vec3(east + THICK, crate::register::TALL + THICK, near + THICK),
        );

        for one in mine.iter() {
            assert!(
                !one.intersects(&book),
                "a piece of the corridor is inside the register: {:?} to {:?}",
                one.min,
                one.max
            );
        }

        // it leaves the lobby rather than starting somewhere nobody is
        let (_, lobby_east) = crate::aleph::span();

        assert!(
            middle_x() + ACROSS * 0.5 < lobby_east,
            "the corridor is not inside the lobby's width"
        );
        assert!(
            middle_x() - ACROSS * 0.5 > crate::aleph::desk(reaches).max.x,
            "the corridor's mouth runs into the desk"
        );
    }
}
