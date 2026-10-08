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
