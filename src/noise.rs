//! Everything the building makes a sound with. Spec 0009.
//!
//! Worked out rather than recorded, the way the carpet, the wood grain and the
//! wine bottles are. There is no audio in this repo and there is not going to
//! be: a footstep is a short burst of noise under an envelope, a splash is the
//! same burst with water's colour on it, and a clack is two short tones a fifth
//! apart.
//!
//! None of these will be mistaken for the real thing and none of them has to
//! be. What they have to do is tell you that you are walking, that you went in
//! the water, and that something over there happened.
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

/// What you are walking on, which decides what you hear.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Underfoot {
    /// The hall, which is carpet.
    Carpet,
    /// The nook, which is boards.
    Boards,
    /// The stair and the cellar, which are stone.
    Stone,
    /// The baths, which are tile.
    Tile,
    /// And the pool, which is water up to your knees.
    Water,
}

/// How far you walk between footsteps, and how much further in water.
///
/// Paced by the distance walked and not by a clock, so slowing down slows them
/// without anybody telling them to. Wading is slower underfoot for free: the
/// water halves your pace, so it takes twice as long to cover a stride.
pub const STRIDE: f32 = 0.85;

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

/// How many of each footstep there are.
///
/// The thing that made the first ones sound like a machine was not their shape:
/// it was that there was one of each and you heard the same sample every
/// stride. No two steps anybody has ever taken were the same sound.
pub const STEPS: usize = 4;

/// How long the sole takes to come down, in seconds.
///
/// The one number that decides whether this is a footstep or a hammer. A hard
/// thing striking another is over in a thousandth of a second or two, and that
/// is what a hammer and a nail sounds like. A soft sole compresses: it takes a
/// fiftieth of a second to arrive, and nothing in it ever happens suddenly.
pub const CONTACT: f32 = 0.02;

/// A ring in something that has been struck: how fast it goes, how long it
/// takes to die, and how much of it there is.
///
/// What tells you tile from carpet is not how bright the noise is. It is that
/// hard things ring and soft things do not: a struck tile holds a note for a
/// few hundredths of a second and a carpet holds nothing at all. Noise alone
/// has no pitch to hold, which is why a step made of it sounds like a machine
/// however it is filtered.
struct Rings {
    hz: f32,
    dies: f32,
    level: f32,
}

/// A ring driven by being hit, rather than a sine switched on.
///
/// A pure sine faded out is a synthesiser: it has one frequency and nothing
/// else, and the ear hears a note being played. A real thing that has been
/// struck rings around a frequency, with the roughness of the blow still in it,
/// and that roughness is most of what says struck.
///
/// So this is a two pole resonator fed a short burst of noise, which is the
/// blow. `r` decides how long it rings and the angle decides where.
fn rung(hit: &[f32], hz: f32, dies: f32, rate: f32) -> Vec<f32> {
    let r = (-1.0 / (dies * rate)).exp();
    let turn = std::f32::consts::TAU * hz / rate;
    let (one, two) = (2.0 * r * turn.cos(), -r * r);

    let (mut back, mut further) = (0.0f32, 0.0f32);
    let out: Vec<f32> = hit
        .iter()
        .map(|x| {
            let out = x + one * back + two * further;
            further = back;
            back = out;

            out
        })
        .collect();

    // and brought back to a peak of one, because what a two pole resonator
    // hands back depends on how long it rings and how high: left as it came,
    // the tile's ring was quieter than the carpet's hiss at the same frequency,
    // which is a tile that does not ring.
    let most = out.iter().fold(0.0f32, |most, s| most.max(s.abs()));
    if most <= 1e-9 {
        return out;
    }

    out.into_iter().map(|sample| sample / most).collect()
}

/// A run that fades in and out over a few thousandths of a second.
///
/// Only the ends. Everything in between is whatever the sound itself is doing,
/// which for a footstep is a thing dying away and not a ramp: a ramp to nothing
/// is the sound of a fader being pulled.
fn tapered(samples: Vec<f32>, edge: usize) -> Samples {
    let count = samples.len();
    let edge = edge.max(1).min(count / 2);
    let shaped = samples
        .into_iter()
        .enumerate()
        .map(|(n, sample)| {
            let up = (n as f32 / edge as f32).min(1.0);
            let down = ((count - n) as f32 / edge as f32).min(1.0);

            (sample * up * down).clamp(-1.0, 1.0)
        })
        .collect();

    Samples::new(RATE, shaped)
}

/// The top end of a run: what is left when the dull part is taken away.
///
/// A running average keeps the bottom, so taking it off keeps the top. Carpet
/// needed this and was given the opposite: the pile's own sound is a rustle of
/// fibres, which is all top end, and the one thing a carpet has not got is
/// bottom. Low passed, it came out as a boom with the rustle thrown away.
fn brightened(dull: &[f32], raw: &[f32]) -> Vec<f32> {
    raw.iter()
        .zip(dull)
        .map(|(whole, low)| whole - low)
        .collect()
}

/// A footstep on something.
///
/// A soft shoe: a slipper or a sneaker, which is what anybody wandering around
/// their own basement has on. So there is no click anywhere in here. What there
/// is: the floor rung by the blow, a rubbery note from the sole, and the scuff
/// of the two meeting.
///
/// Carpet rings at nothing. It had one low note, and a pure low note on a
/// carpet is a synthesiser playing a bass string: what a carpet does is swallow
/// the blow and give back a soft rush of air out of the pile. Boards ring low
/// and long, stone and tile higher and shorter, and tile is the brightest thing
/// in the building. Water does not ring either: it is pushed aside rather than
/// struck.
pub fn step(on: Underfoot, seed: u32) -> Samples {
    let mut next = noise(seed);
    // no two the same: each is a little longer or shorter, a little higher or
    // lower, a little brighter and a little heavier than the last. People do
    // not weigh the same on both feet either.
    let stretch = 1.0 + next() * 0.2;
    let tuned = 1.0 + next() * 0.09;
    let colour = 1.0 + next() * 0.25;
    let weight = 1.0 + next() * 0.22;

    // how much of the scuff is its top end rather than its bottom, which is
    // most of what tells one floor from another
    let (rings, scuff, dies, sole, loud, seconds, top) = match on {
        // the floor under the carpet, heard through it, and the pile's own
        // rustle over that. The quietest of them by a long way: a carpet is
        // what you put down to stop hearing the floor.
        Underfoot::Carpet => (
            vec![Rings {
                hz: 78.0,
                dies: 0.05,
                level: 0.1,
            }],
            0.8,
            0.05,
            0.03,
            0.26,
            0.12,
            0.82,
        ),
        // a board, which is the one floor in the building with a note in it
        Underfoot::Boards => (
            vec![
                Rings {
                    hz: 96.0,
                    dies: 0.1,
                    level: 0.3,
                },
                Rings {
                    hz: 173.0,
                    dies: 0.055,
                    level: 0.13,
                },
                Rings {
                    hz: 402.0,
                    dies: 0.022,
                    level: 0.05,
                },
            ],
            0.9,
            0.035,
            0.08,
            0.38,
            0.15,
            0.3,
        ),
        Underfoot::Stone => (
            vec![
                Rings {
                    hz: 188.0,
                    dies: 0.03,
                    level: 0.22,
                },
                Rings {
                    hz: 445.0,
                    dies: 0.02,
                    level: 0.1,
                },
            ],
            0.95,
            0.028,
            0.07,
            0.36,
            0.1,
            0.42,
        ),
        Underfoot::Tile => (
            vec![
                Rings {
                    hz: 523.0,
                    dies: 0.022,
                    level: 0.2,
                },
                Rings {
                    hz: 1_140.0,
                    dies: 0.012,
                    level: 0.08,
                },
            ],
            0.95,
            0.024,
            0.06,
            0.34,
            0.085,
            0.55,
        ),
        // no ring, a slow swell, and a long wash after it
        Underfoot::Water => (Vec::new(), 1.0, 0.17, 0.0, 0.36, 0.26, 0.18),
    };

    let count = (RATE as f32 * seconds * stretch) as usize;
    let rate = RATE as f32;

    // the contact, which is what the floor is rung by and what everything with
    // a pitch in here is driven by.
    //
    // Not an impulse. Four thousandths of a second of noise at full height is
    // exactly what a hammer is, and Jake heard it as one: a hammer hitting a
    // nail. A soft sole does not strike, it compresses, so this swells over
    // about a fiftieth of a second and falls away over twice that.
    let blow: Vec<f32> = {
        let mut hit = noise(seed ^ 0x5BF0);
        let up = (rate * CONTACT) as usize;
        let over = up * 3;

        (0..count)
            .map(|n| {
                if n >= over {
                    return 0.0;
                }

                // in over the first third and out over the rest, both smoothly
                let shape = if n < up {
                    let along = n as f32 / up as f32;

                    0.5 - 0.5 * (along * std::f32::consts::PI).cos()
                } else {
                    let along = (n - up) as f32 / (over - up) as f32;

                    0.5 + 0.5 * (along * std::f32::consts::PI).cos()
                };

                hit() * shape
            })
            .collect()
    };

    let mut samples = vec![0.0f32; count];
    for ring in &rings {
        for (out, sample) in samples
            .iter_mut()
            .zip(rung(&blow, ring.hz * tuned, ring.dies, rate))
        {
            *out += sample * ring.level;
        }
    }

    // the sole's own rubbery note, which every soft shoe has and which is the
    // same sole whatever it is walking on
    if sole > 0.0 {
        for (out, sample) in samples
            .iter_mut()
            .zip(rung(&blow, 148.0 * tuned, 0.03, rate))
        {
            *out += sample * sole;
        }
    }

    // the scuff, which is most of what you hear, mixed from its own bottom and
    // its own top. How much of each is what a floor is made of: carpet is
    // nearly all rustle and water is nearly all rush.
    let raw: Vec<f32> = {
        let mut next = noise(seed ^ 0x9E37);

        (0..count).map(|_| next()).collect()
    };
    let dull = dulled(
        noise(seed ^ 0x9E37),
        count,
        (0.12 * colour).clamp(0.02, 1.0),
    );
    let bright = brightened(&dull, &raw);
    let hiss: Vec<f32> = dull
        .iter()
        .zip(&bright)
        .map(|(low, high)| low * (1.0 - top) + high * top)
        .collect();
    let samples: Vec<f32> = samples
        .into_iter()
        .enumerate()
        .map(|(n, sample)| {
            let t = n as f32 / rate;
            // the scuff arrives with the sole rather than before it, so
            // nothing in the step happens suddenly
            let swell = if on == Underfoot::Water {
                (t / (seconds * 0.4)).min(1.0)
            } else {
                (t / CONTACT).min(1.0)
            };
            let rush = swell * (-t / dies).exp();

            (sample + hiss[n] * scuff * rush) * loud * weight
        })
        .collect();

    tapered(samples, (rate * 0.0015) as usize)
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

    /// Everything the building makes a sound with, so a new one cannot be
    /// forgotten by the tests that follow.
    fn all() -> Vec<(&'static str, Samples)> {
        let mut out = vec![
            ("splash", splash(4.0, 0x51A5)),
            ("clack", clack(0xC1AC)),
            ("bubble", bubble(0xB0B1)),
            ("fire", fire(0xF12E)),
            ("trickle", trickle(0x7121)),
            ("hum", hum(0x4040)),
            ("steam", steam(0x57EA)),
        ];

        for (n, on) in [
            Underfoot::Carpet,
            Underfoot::Boards,
            Underfoot::Stone,
            Underfoot::Tile,
            Underfoot::Water,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            out.push(("step", step(on, 0x57E9 + n as u32 * 7919)));
        }

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

    /// Spec 0009: a footstep is paced by the walking and not by a clock.
    #[test]
    fn footsteps_are_paced_by_the_walking() {
        // the whole of it: a stride is a distance, so standing still is silence
        // however long you stand there. Read off a walk rather than off the
        // constant, which folds away to nothing a test can fail on.
        let walked = 4.3;
        assert!(
            (walked / STRIDE) as u32 > 0,
            "a walk of {} took no steps",
            walked
        );
        assert_eq!((walked / STRIDE) as u32, 5, "five strides in {}", walked);
        assert_eq!((0.0f32 / STRIDE) as u32, 0, "standing still took a step");
    }

    /// Spec 0009: wading is slower underfoot than walking the same ground.
    ///
    /// Which nothing has to arrange. The water takes most of your pace, and the
    /// steps are paced by distance, so they come further apart for free.
    #[test]
    fn wading_is_slower_underfoot() {
        let dry = crate::SPEED;
        let wet = crate::SPEED * crate::spa::WADE;

        assert!(wet < dry);
        assert!(
            dry / STRIDE > wet / STRIDE,
            "{} steps a second walking against {} wading",
            dry / STRIDE,
            wet / STRIDE
        );
    }

    /// Spec 0009: what is underfoot decides which step you hear.
    ///
    /// Four floors and water, and no two of them the same sound. Carpet is a
    /// thud and tile is a crack, and if they came out alike the room would have
    /// one floor.
    #[test]
    fn what_is_underfoot_decides_the_step() {
        let on = [
            Underfoot::Carpet,
            Underfoot::Boards,
            Underfoot::Stone,
            Underfoot::Tile,
            Underfoot::Water,
        ];

        let runs: Vec<Vec<f32>> = on
            .iter()
            .map(|floor| step(*floor, 0x57E9).collect())
            .collect();

        for (n, one) in runs.iter().enumerate() {
            for (m, other) in runs.iter().enumerate().skip(n + 1) {
                assert!(
                    one.len() != other.len() || one != other,
                    "{:?} and {:?} are the same sound",
                    on[n],
                    on[m]
                );
            }
        }

        // and water is the long one, because a slosh outlasts a knock
        let water: Vec<f32> = step(Underfoot::Water, 0x57E9).collect();
        for floor in [
            Underfoot::Carpet,
            Underfoot::Boards,
            Underfoot::Stone,
            Underfoot::Tile,
        ]
        .iter()
        .copied()
        {
            assert!(
                water.len() > step(floor, 0x57E9).len(),
                "{:?} outlasts the water",
                floor
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

    /// Spec 0009: no two of a floor's footsteps are the same sound.
    ///
    /// The thing that made the first of these unnatural was not their shape. It
    /// was that there was one of each, so you heard the same sample every
    /// stride, which no person has ever done and a machine always does.
    #[test]
    fn no_two_footsteps_are_the_same() {
        for floor in [
            Underfoot::Carpet,
            Underfoot::Boards,
            Underfoot::Stone,
            Underfoot::Tile,
            Underfoot::Water,
        ]
        .iter()
        .copied()
        {
            let runs: Vec<Vec<f32>> = (0..STEPS)
                .map(|n| step(floor, 0x57E9 + n as u32 * 7919).collect())
                .collect();

            for (n, one) in runs.iter().enumerate() {
                for (m, other) in runs.iter().enumerate().skip(n + 1) {
                    assert!(
                        one != other,
                        "{:?} steps {} and {} are the same sound",
                        floor,
                        n,
                        m
                    );
                    // and not merely different samples of the same length: a
                    // stride that is always the same length is a metronome
                    assert!(
                        one.len() != other.len(),
                        "{:?} steps {} and {} last exactly as long",
                        floor,
                        n,
                        m
                    );
                }
            }
        }
    }

    /// Spec 0009: nothing in a footstep happens suddenly.
    ///
    /// This test used to say the opposite: that a step opens with a sharp heel
    /// and dies back. That is a hard thing striking another hard thing, which
    /// is a hammer and a nail, and is exactly what Jake heard. A soft sole does
    /// not strike, it compresses, and the whole of the difference is that the
    /// loudest moment is a fiftieth of a second in rather than at the start.
    #[test]
    fn nothing_in_a_footstep_happens_suddenly() {
        let peak_at = |run: &[f32]| {
            let (mut most, mut at) = (0.0f32, 0usize);
            for (n, sample) in run.iter().enumerate() {
                if sample.abs() > most {
                    most = sample.abs();
                    at = n;
                }
            }

            (at as f32 / RATE as f32, most)
        };

        for floor in [
            Underfoot::Carpet,
            Underfoot::Boards,
            Underfoot::Stone,
            Underfoot::Tile,
            Underfoot::Water,
        ]
        .iter()
        .copied()
        {
            let run: Vec<f32> = step(floor, 0x57E9).collect();
            let (when, most) = peak_at(&run);

            // the loudest moment is not in the first thousandth of a second,
            // which is where a struck thing has it
            assert!(
                when > 0.004,
                "{:?} peaks {} seconds in, which is a blow and not a sole",
                floor,
                when
            );

            // and getting there is gradual: a thousandth of a second in, it is
            // nowhere near as loud as it is going to be
            let early = (RATE as f32 * 0.001) as usize;
            let opening = run[..early]
                .iter()
                .fold(0.0f32, |most, s| most.max(s.abs()));

            assert!(
                opening < most * 0.5,
                "{:?} is already at {} of its {} after a thousandth of a second",
                floor,
                opening,
                most
            );
        }

        // and water is the slowest of all of them to arrive, because water is
        // pushed aside rather than compressed
        let water = peak_at(&step(Underfoot::Water, 0x57E9).collect::<Vec<f32>>()).0;
        let tile = peak_at(&step(Underfoot::Tile, 0x57E9).collect::<Vec<f32>>()).0;

        assert!(
            water > tile,
            "water arrives at {} and tile at {}",
            water,
            tile
        );
    }

    /// Spec 0009: carpet is a rustle and tile is a crack.
    ///
    /// Which way up each floor's noise sits, and the thing carpet had backwards
    /// for three goes at this. The pile's own sound is fibres moving, which is
    /// all top end; the one thing a carpet has not got is bottom, because the
    /// pile is there to swallow it. Low passed it came out as a boom with the
    /// rustle thrown away.
    #[test]
    fn carpet_is_a_rustle_and_tile_is_a_crack() {
        // how much of a run sits above a given frequency, found by taking the
        // dull part off and seeing what is left
        let top_of = |run: &[f32]| {
            let mut last = 0.0f32;
            let (mut low, mut high) = (0.0f32, 0.0f32);

            for sample in run {
                last += (sample - last) * 0.05;
                low += last.abs();
                high += (sample - last).abs();
            }

            high / (low + high).max(1e-9)
        };

        let carpet: Vec<f32> = step(Underfoot::Carpet, 0x57E9).collect();
        let boards: Vec<f32> = step(Underfoot::Boards, 0x57E9).collect();

        assert!(
            top_of(&carpet) > 0.5,
            "carpet is {} top end, which is a boom and not a rustle",
            top_of(&carpet)
        );
        assert!(
            top_of(&carpet) > top_of(&boards),
            "carpet is {} top end against a board's {}",
            top_of(&carpet),
            top_of(&boards)
        );

        // and it is the quietest floor in the building, because a carpet is
        // what you put down to stop hearing the floor
        let loudest = |run: &[f32]| run.iter().fold(0.0f32, |most, s| most.max(s.abs()));
        for floor in [Underfoot::Boards, Underfoot::Stone, Underfoot::Tile]
            .iter()
            .copied()
        {
            let other: Vec<f32> = step(floor, 0x57E9).collect();

            assert!(
                loudest(&carpet) < loudest(&other),
                "carpet at {} is louder than {:?} at {}",
                loudest(&carpet),
                floor,
                loudest(&other)
            );
        }
    }

    /// Spec 0009: a hard floor rings and a soft one does not.
    ///
    /// The difference between tile and carpet is not how bright the noise on
    /// them is, which is what the first of these had and why they sounded like
    /// a machine: it is that a struck tile holds a note for a few hundredths of
    /// a second and a carpet holds nothing. Noise has no pitch to hold.
    #[test]
    fn a_hard_floor_rings_and_a_soft_one_does_not() {
        // how much of a run sits at one frequency, found by asking how well it
        // matches a sine at that frequency
        let rings_at = |run: &[f32], hz: f32| {
            let (mut re, mut im) = (0.0f32, 0.0f32);
            for (n, sample) in run.iter().enumerate() {
                let t = n as f32 / RATE as f32 * hz * std::f32::consts::TAU;
                re += sample * t.cos();
                im += sample * t.sin();
            }

            (re * re + im * im).sqrt() / run.len() as f32
        };

        let tile: Vec<f32> = step(Underfoot::Tile, 0x57E9).collect();
        let carpet: Vec<f32> = step(Underfoot::Carpet, 0x57E9).collect();
        let boards: Vec<f32> = step(Underfoot::Boards, 0x57E9).collect();

        // each hard floor against the one that does not ring, at that floor's
        // own note. Against its own other frequencies it would be measuring the
        // shoe as much as the floor: the sole has a note of its own and it is
        // the same sole on every floor.
        let stone: Vec<f32> = step(Underfoot::Stone, 0x57E9).collect();

        for (what, run, hz) in [
            ("tile", &tile, 523.0f32),
            ("boards", &boards, 96.0),
            ("stone", &stone, 188.0),
        ] {
            assert!(
                rings_at(run, hz) > rings_at(&carpet, hz) * 2.0,
                "{} rings at {} where carpet, which does not ring, manages {}",
                what,
                rings_at(run, hz),
                rings_at(&carpet, hz)
            );
        }

        // water rings at nothing at all
        let water: Vec<f32> = step(Underfoot::Water, 0x57E9).collect();
        assert!(
            rings_at(&water, 523.0) < rings_at(&tile, 523.0) * 0.5,
            "the water rang at {} against the tile's {}",
            rings_at(&water, 523.0),
            rings_at(&tile, 523.0)
        );
    }

    /// Spec 0009: a ring is driven by the blow, not switched on.
    ///
    /// A pure sine faded out is a synthesiser: one frequency and nothing else,
    /// and the ear hears a note being played rather than a thing being hit. A
    /// resonator fed a burst of noise keeps the roughness of the blow in it,
    /// and that roughness is most of what says struck.
    #[test]
    fn a_ring_is_driven_by_the_blow() {
        let rate = RATE as f32;
        let mut hit = noise(0x5BF0);
        let over = (rate * 0.004) as usize;
        let blow: Vec<f32> = (0..4_000)
            .map(|n| {
                if n >= over {
                    return 0.0;
                }

                hit() * (1.0 - n as f32 / over as f32)
            })
            .collect();

        let ring = rung(&blow, 500.0, 0.03, rate);

        // it rings on long after the blow has stopped, which is the whole point
        // of a resonator
        let after: f32 = ring[over..over + 400]
            .iter()
            .fold(0.0, |most, s| most.max(s.abs()));
        assert!(after > 0.1, "it stopped with the blow, at {}", after);

        // and it dies away rather than holding on
        let later: f32 = ring[ring.len() - 400..]
            .iter()
            .fold(0.0, |most, s| most.max(s.abs()));
        assert!(later < after * 0.2, "it was still going at {}", later);

        // it comes back at a peak of one whatever it was asked for, so what a
        // floor asks for is what a floor gets
        let most = ring.iter().fold(0.0f32, |most, s| most.max(s.abs()));
        assert!((most - 1.0).abs() < 1e-4, "it peaked at {}", most);

        // and it is not a sine: a sine has one frequency and nothing else, so
        // half a cycle in it is always the same shape. This is noise shaped by
        // a resonance, so no two of its peaks are the same height.
        let peaks: Vec<f32> = ring
            .windows(3)
            .filter(|w| w[1] > w[0] && w[1] > w[2] && w[1] > 0.05)
            .map(|w| w[1])
            .collect();
        assert!(peaks.len() > 4, "only {} peaks to look at", peaks.len());

        let even = peaks.windows(2).all(|w| (w[0] - w[1]).abs() < 1e-6);
        assert!(!even, "every peak the same height: that is a sine");
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
