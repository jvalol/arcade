//! Everything the building makes a sound with. Spec 0009.
//!
//! Mostly worked out rather than recorded, the way the carpet, the wood grain
//! and the wine bottles are: a splash is a short burst of noise with water's
//! colour on it and a clack is two short tones a fifth apart. None of those
//! will be mistaken for the real thing and none of them has to be. What they
//! have to do is tell you that you went in the water and that something over
//! there happened.
//!
//! The footsteps are the exception and are recordings. They were the best
//! measured and worst sounding thing in the building and were taken out
//! entirely until they sounded like footsteps. A step is the one sound here you
//! hear hundreds of times an hour and the one your ear already knows by heart,
//! and a noise burst under an envelope is a noise burst under an envelope
//! however carefully its envelope was chosen. See `res/steps/README.md`.
//!
//! Nothing here plays anything. It makes runs of samples, which is arithmetic,
//! and arithmetic can be checked without a speaker.

use blitzkit::sound::Samples;
use glam::Vec3;

/// How many samples a second everything here is made at.
///
/// The engine primes its output at this rate, so a sound made at it needs no
/// resampling and cannot be pitched by one.
pub const RATE: u32 = 44_100;

/// What you are standing on, which decides which footstep you hear.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underfoot {
    /// The hall, which is carpet.
    Carpet,
    /// The nook, the cellar and the stair, all of which are boards.
    ///
    /// Three rooms on one floor, which is not a shortcut: the cellar's floor is
    /// hardwood laid over the stone it sits on and the stair's treads are drawn
    /// with the same boards. Called stone, which they were, they came out as
    /// walking on pebbles in a room with a wooden floor.
    Boards,
    /// The baths, which are tile.
    Tile,
    /// And the garden, which is raked gravel.
    Gravel,
}

/// What a body at `at` is standing on.
///
/// Taken from where you are rather than from a flag somebody sets on the way
/// through a door, because a flag is a second account of where you are and this
/// building has learned what two accounts of one thing do.
///
/// Read in the order the building is stacked: everything below the hall first,
/// then the stair, then the two rooms off the hall's left wall, then the hall.
/// Each test is the one that is true of nowhere else.
pub fn underfoot(reaches: f32, at: Vec3) -> Underfoot {
    let floor = -crate::cellar::DOWN;

    if at.y < floor + 0.5 {
        // the baths and the cellar are both down here, and the baths are the
        // far side of the cellar's own wall
        if at.z > crate::cellar::opening(reaches) + crate::cellar::SPAN * 0.5 {
            return Underfoot::Tile;
        }

        return Underfoot::Boards;
    }
    if at.y < -0.1 {
        // on the stair, whose treads are drawn with the same boards as the
        // floor they go down to
        return Underfoot::Boards;
    }

    // the garden and the nook are both through the hall's left wall, and the
    // nook stops short of the garden: which side of that wall you are on is not
    // enough on its own to tell them apart.
    if at.x < crate::garden::wall_at() {
        let middle = crate::garden::at(reaches);

        if at.z > middle.z - crate::garden::SPAN * 0.5 {
            return Underfoot::Gravel;
        }

        return Underfoot::Boards;
    }

    Underfoot::Carpet
}

/// How far you walk between steps.
///
/// Paced by distance and not by a clock, so slowing down slows them and
/// standing still is silence.
///
/// A stride and not a pace. This was 0.85, which is a pace, and at the speed
/// this building walks you that is nearly five steps a second: a sprint
/// cadence, under a body that is plainly not sprinting. A stride at four units
/// a second is a jog's stride.
pub const STRIDE: f32 = 1.6;

/// How long a footstep may last.
///
/// Not a taste. The engine plays what it is given one after another and is not
/// a mixer, so a step longer than the gap to the next one never finishes before
/// the next is due and the queue falls behind for ever. At the old stride the
/// gap was a fifth of a second and the recordings ran to most of a second: ten
/// seconds of walking left half a minute of footsteps still to play, which is
/// why they went on after you stopped.
///
/// So this comes off the stride and the walking speed rather than being chosen,
/// with enough under the gap that a step has finished before the next lands.
pub const SPARE: f32 = 0.06;

pub fn longest() -> f32 {
    (STRIDE / crate::SPEED - SPARE).max(0.05)
}

/// The recordings, per the engine's spec 0045 and `res/steps/README.md`.
///
/// Bundled rather than read off disk. Every other asset in this building is
/// either worked out by the code that draws it or an `include_bytes!`, and a
/// game that looks for a folder beside itself is a game that runs from one
/// directory.
///
/// Recordings and not arithmetic, which is the whole of why they are here.
/// These were the best measured and worst sounding thing in the building and
/// were taken out in `e187000` until they sounded like footsteps. What was
/// wrong was never the pacing or the number of variants: it was that a footstep
/// made of a noise burst under an envelope is a noise burst under an envelope.
pub fn steps(on: Underfoot) -> &'static [&'static [u8]] {
    match on {
        Underfoot::Carpet => &[
            include_bytes!("../res/steps/carpet1.wav"),
            include_bytes!("../res/steps/carpet2.wav"),
            include_bytes!("../res/steps/carpet3.wav"),
            include_bytes!("../res/steps/carpet4.wav"),
        ],
        // six, where the others have four, because this floor is under three
        // rooms and two of them are walked end to end every time anybody goes
        // down to the cellar
        Underfoot::Boards => &[
            include_bytes!("../res/steps/boards1.wav"),
            include_bytes!("../res/steps/boards2.wav"),
            include_bytes!("../res/steps/boards3.wav"),
            include_bytes!("../res/steps/boards4.wav"),
            include_bytes!("../res/steps/boards5.wav"),
            include_bytes!("../res/steps/boards6.wav"),
        ],
        Underfoot::Tile => &[
            include_bytes!("../res/steps/tile1.wav"),
            include_bytes!("../res/steps/tile2.wav"),
            include_bytes!("../res/steps/tile3.wav"),
            include_bytes!("../res/steps/tile4.wav"),
        ],
        Underfoot::Gravel => &[
            include_bytes!("../res/steps/gravel1.wav"),
            include_bytes!("../res/steps/gravel2.wav"),
            include_bytes!("../res/steps/gravel3.wav"),
            include_bytes!("../res/steps/gravel4.wav"),
            include_bytes!("../res/steps/gravel5.wav"),
            include_bytes!("../res/steps/gravel6.wav"),
        ],
    }
}

/// Every floor there is, so the one list of them is this one.
pub const FLOORS: [Underfoot; 4] = [
    Underfoot::Carpet,
    Underfoot::Boards,
    Underfoot::Tile,
    Underfoot::Gravel,
];

/// How loud each floor is against the others.
///
/// The files are all normalised to the same peak, so which floor is the quiet
/// one is a decision and not a property of whichever recording came to hand.
/// Carpet is the quiet one and stone is the loud one, which is the one thing
/// about floors everybody already knows.
pub fn loudness(on: Underfoot) -> f32 {
    match on {
        Underfoot::Carpet => 0.34,
        Underfoot::Boards => 0.48,
        Underfoot::Tile => 0.58,
        Underfoot::Gravel => 0.5,
    }
}

/// How far a step is moved in pitch and in level from one to the next.
///
/// Small, and both. One sample at one pitch and one level every stride is a
/// machine however well it was recorded, and it is the repetition that gives it
/// away rather than the sample. A twelfth is about a semitone, which is as far
/// as a step can move before it is a different shoe.
pub const PITCHED: f32 = 0.06;
pub const QUIETER: f32 = 0.16;

/// Which of a floor's recordings to play, and how to bend it.
///
/// Never the one just played. Picked at random out of all of them, the same one
/// twice running comes up once every few steps and that is the one pair anybody
/// notices; picked in order, the whole run is a loop you learn in ten paces.
/// So it walks the list in order and starts from a different place each time it
/// changes floor, which is neither.
pub fn bend(step: usize, many: usize) -> (usize, f32, f32) {
    let many = many.max(1);
    // a hash of the step number rather than a generator carried about, so this
    // is arithmetic and can be checked without running the game
    let mut state = (step as u32).wrapping_mul(2_654_435_761) ^ 0x9E37_79B9;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        (state >> 8) as f32 / ((1u32 << 24) as f32) * 2.0 - 1.0
    };

    (
        step % many,
        1.0 + next() * PITCHED,
        1.0 - next().abs() * QUIETER,
    )
}

/// How far a room's own sound carries. The cellar's fire is not something you
/// hear from the hall.
pub const CARRIES: f32 = 14.0;

/// Which of the room's notes you can hear from here, if any.
///
/// One, and the nearest. The engine plays what it is given one after another,
/// per spec 0044, and that is not a mixer: four notes a second each a second
/// long is a queue growing by three seconds every second, which after an hour
/// or two is a gigabyte of audio waiting its turn and a machine on its knees.
/// It is also why each of them is short.
///
/// So the building plays the note of the room you are in, and the room you are
/// in is the one whose note is nearest.
pub fn nearest(ear: Vec3, airs: &[Vec3]) -> Option<usize> {
    airs.iter()
        .enumerate()
        .filter(|(_, at)| ear.distance(**at) <= CARRIES)
        .min_by(|one, other| {
            ear.distance_squared(*one.1)
                .total_cmp(&ear.distance_squared(*other.1))
        })
        .map(|(n, _)| n)
}

/// The one random number generator in here, which is the same one the textures
/// use.
fn noise(seed: u32) -> impl FnMut() -> f32 {
    let mut rng = seed | 1;

    move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;

        (rng >> 8) as f32 / 8_388_608.0 - 1.0
    }
}

/// A run that begins and ends at nothing.
///
/// Every sound in here goes through this. A run that starts at anything but
/// nothing starts with a step in the waveform, and a step is a click: the one
/// sound nothing in a building makes.
fn shaped(samples: Vec<f32>, attack: f32) -> Samples {
    let count = samples.len();
    if count == 0 {
        return Samples::new(RATE, samples);
    }

    let rise = ((count as f32 * attack) as usize).max(1);
    let shaped = samples
        .into_iter()
        .enumerate()
        .map(|(n, sample)| {
            let along = n as f32 / count as f32;
            let up = (n as f32 / rise as f32).min(1.0);
            // in quickly and out over everything that is left, which is the
            // shape of a thing being struck
            let down = (1.0 - along).powf(1.6);

            (sample * up * down).clamp(-1.0, 1.0)
        })
        .collect();

    Samples::new(RATE, shaped)
}

/// Noise with the high end taken off it, which is what makes one surface sound
/// unlike another.
///
/// A one lets everything through and a small number lets almost nothing: it is
/// a running average, so the softer the surface the more of itself each sample
/// is made of.
fn dulled(mut next: impl FnMut() -> f32, count: usize, through: f32) -> Vec<f32> {
    let mut last = 0.0;

    (0..count)
        .map(|_| {
            last += (next() - last) * through.clamp(0.01, 1.0);

            last
        })
        .collect()
}

/// A splash, as big as whatever made it.
///
/// `fell` is how fast you went in. Going in faster is louder and longer, which
/// is the whole of what a splash tells you.
pub fn splash(fell: f32, seed: u32) -> Samples {
    let hard = fell.clamp(0.0, 6.0) / 6.0;
    let count = 4_000 + (hard * 9_000.0) as usize;
    let samples: Vec<f32> = dulled(noise(seed), count, 0.2 + hard * 0.3)
        .into_iter()
        .map(|sample| sample * (0.25 + hard * 0.6))
        .collect();

    shaped(samples, 0.03)
}

/// Two hard things meeting: two short tones a fifth apart, which is what one
/// struck by another is.
///
/// The ball and chain knocking a brick off the wall, which is the one thing in
/// this building that hits anything. The pool table has no balls on it: pool is
/// played in poolhall's own window and that game makes its own noise.
pub fn clack(seed: u32) -> Samples {
    let mut next = noise(seed);
    let wobble = next() * 60.0;
    let count = 1_600;
    let samples: Vec<f32> = (0..count)
        .map(|n| {
            let t = n as f32 / RATE as f32;
            let one = (t * (1_480.0 + wobble) * std::f32::consts::TAU).sin();
            let fifth = (t * (2_220.0 + wobble) * std::f32::consts::TAU).sin();

            (one + fifth * 0.6) * 0.3
        })
        .collect();

    shaped(samples, 0.002)
}

/// A bubble coming up: a short tone that rises, which is a bubble getting
/// smaller as it goes.
pub fn bubble(seed: u32) -> Samples {
    let mut next = noise(seed);
    let from = 280.0 + next().abs() * 420.0;
    let count = 2_600;
    let samples: Vec<f32> = (0..count)
        .map(|n| {
            let along = n as f32 / count as f32;
            let t = n as f32 / RATE as f32;

            ((t * (from * (1.0 + along * 1.4))) * std::f32::consts::TAU).sin() * 0.22
        })
        .collect();

    shaped(samples, 0.04)
}

/// How long a room's own note runs.
///
/// Short, because one of them is playing at a time and anything else from
/// somewhere in the world waits behind it: a splash a whole second late is a
/// splash for something you have forgotten doing.
pub const NOTE: f32 = 0.4;

/// The fire: a low roll with the odd crack in it.
pub fn fire(seed: u32) -> Samples {
    let mut next = noise(seed);
    let count = (RATE as f32 * NOTE) as usize;
    let mut last = 0.0f32;
    let samples: Vec<f32> = (0..count)
        .map(|n| {
            last += (next() - last) * 0.03;
            // a crack now and then, which is what says fire rather than wind
            let crack = if next() > 0.9988 { next() * 0.5 } else { 0.0 };
            let breathe = 0.7 + ((n as f32 / count as f32) * 6.3).sin() * 0.3;

            (last * 2.2 * breathe + crack).clamp(-1.0, 1.0) * 0.3
        })
        .collect();

    shaped(samples, 0.08)
}

/// Water falling into water, which is the fountain and goes on for ever.
pub fn trickle(seed: u32) -> Samples {
    let count = (RATE as f32 * NOTE) as usize;
    let samples: Vec<f32> = dulled(noise(seed), count, 0.45)
        .into_iter()
        .enumerate()
        .map(|(n, sample)| {
            let beat = ((n as f32 / RATE as f32) * 11.0).sin() * 0.3 + 0.7;

            sample * 0.18 * beat
        })
        .collect();

    shaped(samples, 0.1)
}

/// The hum a hall full of cabinets has.
pub fn hum(seed: u32) -> Samples {
    let mut next = noise(seed);
    let count = (RATE as f32 * NOTE) as usize;
    let samples: Vec<f32> = (0..count)
        .map(|n| {
            let t = n as f32 / RATE as f32;
            // mains hum and the whine of a tube, and a little hiss over both
            let low = (t * 60.0 * std::f32::consts::TAU).sin() * 0.1;
            let whine = (t * 15_734.0 * std::f32::consts::TAU).sin() * 0.012;

            low + whine + next() * 0.01
        })
        .collect();

    shaped(samples, 0.12)
}

/// The sauna: the stove ticking and what is on it hissing.
pub fn steam(seed: u32) -> Samples {
    let count = (RATE as f32 * NOTE) as usize;
    let samples: Vec<f32> = dulled(noise(seed), count, 0.8)
        .into_iter()
        .enumerate()
        .map(|(n, sample)| {
            let tick = if n % 9_000 < 120 { 0.25 } else { 0.0 };
            let along = n as f32 / count as f32;

            sample * 0.12 + tick * (1.0 - along)
        })
        .collect();

    shaped(samples, 0.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0009: each room sounds like what it is floored with.
    ///
    /// Stood in the middle of each one, which is the only place every room has.
    /// This moved out of `main` to be testable at all: a method on the whole
    /// game needs a window and a sound card to ask one question about a point.
    #[test]
    fn each_room_sounds_like_its_own_floor() {
        use glam::vec3;

        let room = crate::room::Room::of(
            (0..12)
                .map(|n| {
                    crate::cabinet::Cabinet::found(
                        &format!("game{}", n),
                        std::path::Path::new("/nowhere"),
                    )
                })
                .collect(),
        );
        let reaches = room.reaches;
        let garden = crate::garden::at(reaches);
        let (pond, _, _) = crate::garden::pond(reaches);
        let nook = crate::room::nook_floor(reaches).0;
        let cellar = crate::cellar::floor(reaches).0;
        let baths = crate::spa::at(reaches);

        for (what, at, want) in [
            ("the aisle", vec3(0.0, 0.0, 0.0), Underfoot::Carpet),
            (
                "the doorway",
                vec3(0.0, 0.0, reaches - crate::room::INSIDE),
                Underfoot::Carpet,
            ),
            ("the nook", vec3(nook.x, 0.0, nook.z), Underfoot::Boards),
            (
                "the cellar",
                vec3(cellar.x, -crate::cellar::DOWN, cellar.z),
                Underfoot::Boards,
            ),
            (
                "the baths",
                vec3(baths.x, -crate::cellar::DOWN, baths.z),
                Underfoot::Tile,
            ),
            (
                "the garden",
                vec3(garden.x, 0.0, garden.z),
                Underfoot::Gravel,
            ),
            // the walk round the pond, which is the other end of the garden
            (
                "the pond's edge",
                vec3(pond.x, 0.0, pond.z - 3.6),
                Underfoot::Gravel,
            ),
            // half way down the stair, which is stone whatever is at the foot
            (
                "the stair",
                vec3(cellar.x, -crate::cellar::DOWN * 0.5, cellar.z),
                Underfoot::Boards,
            ),
        ] {
            assert_eq!(
                underfoot(reaches, at),
                want,
                "{} at {:?} sounded like {:?}",
                what,
                at,
                underfoot(reaches, at)
            );
        }
    }

    /// Spec 0009: every floor has footsteps, and every one of them reads.
    ///
    /// The one test that matters for a bundled file. A WAV that will not parse
    /// is a floor that walks in silence, and the loading drops what it cannot
    /// read rather than crashing, so nothing at run time would ever say so.
    #[test]
    fn every_floor_has_footsteps_that_read() {
        for on in FLOORS.iter().copied() {
            let files = steps(on);

            assert!(files.len() >= 3, "{:?} has only {} steps", on, files.len());

            for (n, bytes) in files.iter().enumerate() {
                let got = Samples::from_wav(bytes)
                    .unwrap_or_else(|why| panic!("{:?} step {}: {}", on, n, why));

                assert!(!got.is_empty(), "{:?} step {} is silence", on, n);
                // short enough that one does not still be playing when the next
                // is due. The engine plays what it is given one after another
                // rather than mixing, so a tail longer than a stride is a queue
                // that grows for as long as you keep walking.
                // the one that matters. The engine queues rather than mixes,
                // so a step longer than the gap to the next one is a queue that
                // grows for as long as you keep walking, and footsteps that go
                // on after you stop.
                assert!(
                    got.seconds() <= longest(),
                    "{:?} step {} lasts {:.3}s and there is {:.3}s between steps",
                    on,
                    n,
                    got.seconds(),
                    longest()
                );
                assert!(
                    got.seconds() > 0.08,
                    "{:?} step {} lasts {:.3}s, which is a click",
                    on,
                    n,
                    got.seconds()
                );
            }
        }
    }

    /// Spec 0009: walking never queues more footsteps than there is time to
    /// play them.
    ///
    /// This is the fault the lengths are cut to, stated as the thing that was
    /// wrong rather than as the number that fixes it. The engine appends to a
    /// player that runs one sound after another, so a step that outlasts the
    /// gap to the next one is a backlog that grows for as long as you keep
    /// walking and goes on playing after you stop.
    ///
    /// At the stride and lengths this shipped with first, half a minute of
    /// walking left a minute and a half of footsteps still to come.
    #[test]
    fn walking_does_not_run_the_queue_behind() {
        let gap = STRIDE / crate::SPEED;

        assert!(
            longest() < gap,
            "a step lasts {:.3}s and lands every {:.3}s",
            longest(),
            gap
        );

        for seconds in [1.0f32, 10.0, 60.0, 600.0] {
            let taken = (seconds * crate::SPEED / STRIDE).floor();
            let queued = taken * longest();

            assert!(
                queued < seconds,
                "{}s of walking queues {:.1}s of footsteps",
                seconds,
                queued
            );
        }

        // and the cadence is a cadence and not a drum roll
        let a_second = crate::SPEED / STRIDE;
        assert!(
            (1.5..=3.5).contains(&a_second),
            "{:.1} steps a second is not a person walking",
            a_second
        );
    }

    /// Spec 0009: no floor's files are another floor's files.
    ///
    /// Two floors sharing a recording is two rooms that sound the same, and it
    /// is the kind of thing a copy and paste does quietly.
    #[test]
    fn no_two_floors_sound_the_same() {
        for (n, on) in FLOORS.iter().copied().enumerate() {
            for other in FLOORS.iter().copied().skip(n + 1) {
                for one in steps(on) {
                    for two in steps(other) {
                        assert!(
                            !std::ptr::eq(*one, *two),
                            "{:?} and {:?} share a recording",
                            on,
                            other
                        );
                    }
                }
            }
        }
    }

    /// Spec 0009: no step is the one before it.
    ///
    /// The repetition is what gives a footstep away, not the recording. Walked
    /// a hundred paces, the same file must never come up twice running, and
    /// neither the pitch nor the level may land on the last one either.
    #[test]
    fn no_step_is_the_one_before_it() {
        for many in 3..=6usize {
            let mut was = bend(0, many);

            for step in 1..100usize {
                let now = bend(step, many);

                assert_ne!(now.0, was.0, "step {} is the file before it", step);
                assert!(
                    (now.1 - was.1).abs() > 1e-4,
                    "step {} is the pitch before it",
                    step
                );
                assert!(
                    (now.2 - was.2).abs() > 1e-4,
                    "step {} is the level before it",
                    step
                );
                was = now;
            }
        }
    }

    /// Spec 0009: the bend stays inside what a step can be.
    ///
    /// A semitone and a sixth of the level. Further than that and it is not the
    /// same shoe on the same floor, which is a different fault from sounding
    /// like a machine and just as obvious.
    #[test]
    fn a_step_is_bent_and_not_mangled() {
        let (mut low, mut high) = (f32::MAX, 0.0f32);
        let (mut quiet, mut loud) = (f32::MAX, 0.0f32);

        for step in 0..1_000usize {
            let (which, pitch, level) = bend(step, 4);

            assert!(which < 4, "it reached for file {} of four", which);
            low = low.min(pitch);
            high = high.max(pitch);
            quiet = quiet.min(level);
            loud = loud.max(level);
        }

        assert!(low > 1.0 - PITCHED - 1e-6 && high < 1.0 + PITCHED + 1e-6);
        assert!(quiet > 1.0 - QUIETER - 1e-6 && loud <= 1.0 + 1e-6);
        // and it uses the range it is given rather than hovering in the middle
        assert!(
            high - low > PITCHED,
            "the pitch barely moves: {} to {}",
            low,
            high
        );
        assert!(loud - quiet > QUIETER * 0.6, "the level barely moves");
    }

    /// Spec 0009: the quiet floor is quiet and the loud one is loud.
    ///
    /// Which is a decision, because the files are all normalised to one peak.
    /// Left alone, how loud a floor is would be whichever recording came to
    /// hand.
    #[test]
    fn carpet_is_the_quiet_floor_and_tile_the_loud_one() {
        let on = |floor| loudness(floor);

        assert!(on(Underfoot::Carpet) < on(Underfoot::Boards));
        assert!(on(Underfoot::Boards) < on(Underfoot::Tile));
        for floor in FLOORS.iter().copied() {
            assert!(
                on(floor) > 0.0 && on(floor) <= 1.0,
                "{:?} is at {}",
                floor,
                on(floor)
            );
        }
    }

    /// Everything the building makes a sound with, so a new one cannot be
    /// forgotten by the tests that follow.
    fn all() -> Vec<(&'static str, Samples)> {
        let out = vec![
            ("splash", splash(4.0, 0x51A5)),
            ("clack", clack(0xC1AC)),
            ("bubble", bubble(0xB0B1)),
            ("fire", fire(0xF12E)),
            ("trickle", trickle(0x7121)),
            ("hum", hum(0x4040)),
            ("steam", steam(0x57EA)),
        ];

        out
    }

    /// Spec 0009: every sound the building makes is a sound: it has samples in
    /// it, and it ends.
    #[test]
    fn every_sound_is_a_sound() {
        for (what, sound) in all() {
            assert!(!sound.is_empty(), "the {} is silence", what);
            assert!(
                sound.seconds() > 0.01 && sound.seconds() < 2.0,
                "the {} lasts {} seconds",
                what,
                sound.seconds()
            );

            // and it is not a run of nothing dressed as a sound
            let loudest = sound.clone().fold(0.0f32, |most, s| most.max(s.abs()));
            assert!(loudest > 0.02, "the {} is {} loud", what, loudest);
        }
    }

    /// Spec 0009: and none of them clips.
    ///
    /// The engine does not police loudness, per spec 0044: a sample past one is
    /// clipping, and clipping is a buzz over everything else in the room.
    #[test]
    fn nothing_is_louder_than_one() {
        for (what, sound) in all() {
            for sample in sound {
                assert!(sample.abs() <= 1.0, "the {} reaches {}", what, sample.abs());
            }
        }
    }

    /// Spec 0009: and none of them begins or ends with a jump.
    ///
    /// A run that starts at anything but nothing starts with a step in the
    /// waveform, and a step is a click: the one sound nothing in a building
    /// makes.
    #[test]
    fn nothing_begins_or_ends_with_a_click() {
        for (what, sound) in all() {
            let samples: Vec<f32> = sound.collect();

            assert!(
                samples[0].abs() < 0.02,
                "the {} opens at {}",
                what,
                samples[0]
            );
            assert!(
                samples[samples.len() - 1].abs() < 0.02,
                "the {} closes at {}",
                what,
                samples[samples.len() - 1]
            );
        }
    }

    /// Spec 0009: one note plays at a time, and it is the nearest.
    ///
    /// The whole of why this function exists. The engine plays what it is given
    /// one after another, per spec 0044, and that is not a mixer. Five notes
    /// each a second long, each queued once a second, is a queue growing by
    /// four seconds every second: after two hours it was a gigabyte of audio
    /// waiting its turn and a machine with nothing left to draw with.
    #[test]
    fn one_note_plays_at_a_time_and_it_is_the_nearest() {
        let airs = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(40.0, 0.0, 0.0),
        ];

        // the nearest of the ones you can hear, and only ever one
        assert_eq!(nearest(Vec3::new(2.6, 0.0, 0.0), &airs), Some(1));
        assert_eq!(nearest(Vec3::new(-1.0, 0.0, 0.0), &airs), Some(0));

        // and nothing at all when they are all out of earshot
        assert_eq!(nearest(Vec3::new(200.0, 0.0, 0.0), &airs), None);
        assert_eq!(nearest(Vec3::ZERO, &[]), None);

        // the far one is past earshot even standing next to the others
        assert_ne!(nearest(Vec3::new(20.0, 0.0, 0.0), &airs), Some(2));

        // and a note is short, because anything else from the world waits
        // behind it
        for (what, sound) in all() {
            if !matches!(what, "fire" | "trickle" | "hum" | "steam" | "bubble") {
                continue;
            }

            assert!(
                sound.seconds() <= NOTE + 1e-3,
                "the {} runs {} seconds and a note is {}",
                what,
                sound.seconds(),
                NOTE
            );
        }
    }

    /// Spec 0009: a splash is as big as the fall that made it.
    #[test]
    fn a_splash_is_as_big_as_the_fall() {
        let gentle = splash(0.5, 0x51A5);
        let hard = splash(5.0, 0x51A5);

        assert!(
            hard.len() > gentle.len(),
            "a hard splash is {} samples and a gentle one {}",
            hard.len(),
            gentle.len()
        );

        let loudest = |sound: Samples| sound.fold(0.0f32, |most, s| most.max(s.abs()));
        assert!(
            loudest(hard) > loudest(gentle),
            "going in faster was no louder"
        );

        // and standing in it is not a splash at all
        assert!(splash(0.0, 0x51A5).len() < splash(6.0, 0x51A5).len());
    }
}
