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
use glam::{vec3, Vec3};

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
    /// The pool in the baths, when you are standing in it.
    ///
    /// Not a floor so much as a thing between you and one: the tile is still
    /// under your feet, and what you hear is the water you are pushing
    /// through. It takes the place of the tile rather than sounding over it,
    /// because the engine queues sounds and does not mix them.
    Water,
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

/// Which room of the building a point is in.
///
/// Taken from where you are rather than from a flag somebody sets on the way
/// through a door, because a flag is a second account of where you are and this
/// building has learned what two accounts of one thing do.
///
/// **The one account of where the rooms are**, and the thing several others
/// were standing in for. What is under your feet was the nearest this building
/// had and it is not the same question: three rooms are floored in the same
/// boards, and the pool is tile you are standing in water on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Room {
    Hall,
    Nook,
    /// Its own, between the hall and the cellar, and floored like the cellar.
    Stair,
    Cellar,
    /// Which holds the pool, the fountain and the sauna.
    Baths,
    Garden,
}

/// Which room a point is in.
///
/// Read in the order the building is stacked: everything below the hall
/// first, then the stair, then the rooms through the hall's left wall, then
/// the hall itself.
pub fn room_at(reaches: f32, at: Vec3) -> Room {
    // below the hall's floor is the cellar, the baths, or the stair between
    // them, and which one is a question about where you are and not how high.
    //
    // It was how high: anything in the three metres between the two floors
    // counted as the stair. That is true of a body, whose feet are on one
    // floor or the other, and false of a point. The sauna's own voice sounds
    // from the middle of its room at 1.55 down, and the building put it on
    // the stair.
    if at.y < -0.1 {
        // the stair runs from the hall's floor down to its foot, and nothing
        // else in the building is over that strip of the cellar
        if at.x > crate::cellar::stair_foot() - crate::cellar::TREAD
            && at.z > crate::cellar::opening(reaches) - crate::cellar::WIDE * 0.5
            && at.z < crate::cellar::opening(reaches) + crate::cellar::WIDE * 0.5
        {
            return Room::Stair;
        }
        // and the baths are the far side of the cellar's own wall
        if at.z > crate::cellar::opening(reaches) + crate::cellar::SPAN * 0.5 {
            return Room::Baths;
        }

        return Room::Cellar;
    }

    // the garden and the nook are both through the hall's left wall, and the
    // nook stops short of the garden: which side of that wall you are on is not
    // enough on its own to tell them apart.
    //
    // Asked of `garden::under_the_sky`, which is the same question the storm
    // asks and so is answered once.
    if at.x < crate::garden::wall_at() {
        if crate::garden::under_the_sky(reaches, at) {
            return Room::Garden;
        }

        return Room::Nook;
    }

    Room::Hall
}

/// What a body at `at` is standing on.
///
/// Which room it is in, and then what that room is floored with. It used to
/// be the other way about, with the rooms worked out inside here and only the
/// floor coming back out, which left every other question about where you are
/// to be answered again somewhere else.
pub fn underfoot(reaches: f32, at: Vec3) -> Underfoot {
    match room_at(reaches, at) {
        // in the pool it is the water you hear and not the tile it is
        // standing on. Asked of the same function that slows you down in it,
        // so what you hear and what you can do cannot disagree.
        Room::Baths if crate::spa::wading(reaches, at) => Underfoot::Water,
        Room::Baths => Underfoot::Tile,
        // the stair's treads are drawn with the same boards as the floor they
        // go down to
        Room::Nook | Room::Stair | Room::Cellar => Underfoot::Boards,
        Room::Garden => Underfoot::Gravel,
        Room::Hall => Underfoot::Carpet,
    }
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

/// How far you cover between steps at a run.
///
/// A unit is a metre here: a body is 0.9 across and your eye is at 1.55. So
/// `crate::SPEED` times `crate::RUNS` is 8.4 metres a second, which is a
/// sprint, and a sprinter at that speed is not taking the same step twice as
/// often. They are taking a longer one: about two metres, four times a second.
///
/// This came out of the footstep work. `RUNS` doubled the speed and the stride
/// stayed where it was, so the whole of it went into the cadence and a run was
/// 5.3 steps a second, which is a buzz rather than a sprint. 2.1 at 8.4 is
/// four a second, against two and a half at a walk.
///
/// Longer and not shorter, which is the half of this that is easy to get
/// backwards: a faster thing sounds like more of it, and the reason a sprint
/// does not sound like a drum roll is that each step covers more ground.
pub const RUN_STRIDE: f32 = 2.1;

/// How many steps may be sounding at once.
///
/// Two, so a step is over before the step after next lands. Three shoes going
/// at the same time is not a run, it is a smear, and whichever way the engine
/// plays them there is a limit past which more overlap is less footstep.
///
/// This used to be a cap under the gap itself, because the engine queued what
/// it was given and a step that outlasted the gap put the queue a little
/// further behind on every stride. That was the fault Jake heard twice: the
/// first time at the old 0.85 stride, where ten seconds of walking left half a
/// minute of footsteps still to play, and the second time the day shift was
/// added, because the cap was worked out from a walk and running halved the
/// gap it was measured against. `SoundSystem::play` mixes now, per spec 0004
/// of the engine, so steps that overlap are steps overlapping and nothing
/// accumulates. The cap stays because a smear is still worth not having, but
/// it is a cap on how much overlap sounds like a footstep rather than the
/// thing standing between the building and a runaway queue.
pub const OVERLAP: f32 = 2.0;

/// How far you cover between steps, going as you are.
///
/// The one place the two strides are chosen between, so nothing else has to
/// know there are two.
pub fn stride(running: bool) -> f32 {
    if running {
        RUN_STRIDE
    } else {
        STRIDE
    }
}

/// Whether shift does anything on a floor.
///
/// Not in the water. Wading is the whole of what tells you that you are in the
/// pool rather than beside it, per spec 0008, and a sprint through it takes
/// that away. Said once here, because `crate::Game::pace` and everything that
/// works out the gap between steps both need it and two accounts of it is two
/// things to come apart.
pub fn can_run(on: Underfoot) -> bool {
    !matches!(on, Underfoot::Water)
}

/// How fast you cross a floor at a walk, which is half of what sets the gap
/// between steps on it.
///
/// The same `spa::WADE` the pool slows you by, so the sound and the walking
/// cannot disagree about how fast you are getting through the water.
pub fn pace(on: Underfoot) -> f32 {
    match on {
        Underfoot::Water => crate::SPEED * crate::spa::WADE,
        _ => crate::SPEED,
    }
}

/// The fastest you can cross it.
///
/// Named apart from `pace` because the two came apart once. Everything that
/// wanted the shortest gap asked `pace` for it, and the day shift arrived
/// `pace` stopped being the fastest and no test said so.
pub fn fastest(on: Underfoot) -> f32 {
    if can_run(on) {
        pace(on) * crate::RUNS
    } else {
        pace(on)
    }
}

/// The shortest the gap between steps can get on a floor.
///
/// Both halves at once, which is the point of it being one function: going
/// faster shortens the gap and the longer stride at a run gives some of that
/// back. On a dry floor it is 2.1 over 8.4, a quarter of a second, where the
/// speed alone would have said a fifth.
///
/// The stride asked for is the one you would use at the fastest you can go
/// here, so in the water it is the walking stride: you cannot run in there.
pub fn shortest_gap(on: Underfoot) -> f32 {
    stride(can_run(on)) / fastest(on)
}

/// How long a footstep may last: `OVERLAP` of the shortest gap.
///
/// Per floor, because the gap is not the same on all of them. You wade at
/// `spa::WADE` of your walking pace and cannot run in it at all, so a step in
/// the pool lands every nine tenths of a second rather than every quarter of
/// one, and a splash wants that tail. One number for every floor would either
/// cut the water off short or hold every dry floor to a splash's length.
pub fn longest(on: Underfoot) -> f32 {
    (OVERLAP * shortest_gap(on)).max(0.05)
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
        // six, and longer than any other floor: see `longest`
        Underfoot::Water => &[
            include_bytes!("../res/steps/water1.wav"),
            include_bytes!("../res/steps/water2.wav"),
            include_bytes!("../res/steps/water3.wav"),
            include_bytes!("../res/steps/water4.wav"),
            include_bytes!("../res/steps/water5.wav"),
            include_bytes!("../res/steps/water6.wav"),
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
pub const FLOORS: [Underfoot; 5] = [
    Underfoot::Carpet,
    Underfoot::Water,
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
        Underfoot::Water => 0.52,
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

/// How many pieces of each voice there are. Spec 0009.
///
/// One was not enough and the reason is the rate. A piece is four tenths of a
/// second and is queued again the moment it runs out, so the same four tenths
/// played two and a half times a second: that is a loop, and a loop is heard
/// as a loop whatever is in it. Every one of these comes off a seed already,
/// so more of them costs a few kilobytes and nothing else.
pub const PIECES: usize = 5;

/// Which piece of a voice to play next, and how to bend it.
///
/// Like [`bend`], which the floors use, except that it picks rather than
/// cycling. A floor's steps cycle because you do not take two steps from the
/// same place and the room moves between them. A voice comes from one place at
/// a fixed rate, so a cycle of five is a bar of five beats and you hear it by
/// the second time round.
///
/// Picked and not shuffled, so the same piece does come up twice in a row now
/// and then. That is fine and a cycle is not: two of the same four tenths of
/// a second running is a slightly longer sound, while five in a fixed order
/// is a bar of five beats, and it is the order a listener catches rather than
/// the repeat. The first version of this tried to rule out the back to back
/// case and compared against a previous pick it had not worked out.
pub fn adrift(piece: usize, many: usize) -> (usize, f32, f32) {
    let many = many.max(1);
    let mut state = (piece as u32).wrapping_mul(2_246_822_519) ^ 0x85EB_CA6B;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        (state >> 8) as f32 / ((1u32 << 24) as f32) * 2.0 - 1.0
    };

    let picked = ((((next() + 1.0) * 0.5) * many as f32) as usize).min(many - 1);

    (picked, 1.0 + next() * PITCHED, 1.0 - next().abs() * QUIETER)
}

/// How far a room's own sound carries. The cellar's fire is not something you
/// hear from the hall.
pub const CARRIES: f32 = 14.0;

/// Which voice you can hear from here, if any.
///
/// One, and the nearest in your own room. The engine plays what it is given
/// one after another, per spec 0044, and that is not a mixer: four pieces a
/// second each a second long is a queue growing by three seconds every second,
/// which after an hour or two is a gigabyte of audio waiting its turn and a
/// machine on its knees. It is also why each of them is short.
///
/// **In your own room, which is the half this was missing.** It used to be
/// the nearest voice full stop, on the reasoning that the nearest voice is the
/// one in the room you are in. That is not true of a building with rooms under
/// other rooms: the baths sit beneath the garden, and from the middle of the
/// garden the tub is 9.4 metres away and from its south end 5.4, both of them
/// straight down through a floor. Nothing else is nearer, because the garden
/// has no voice of its own, so the garden's background sound was a hot tub
/// underneath it.
///
/// A distance with nothing checking what is between the two points is this
/// building's commonest mistake. It has cost it a light through a wall, a
/// storm through the whole place, and this.
///
/// **A doorway does not carry**, which is the simplification left in. Stand in
/// the hall by the garden's opening and you hear the hall, where in life you
/// would hear both. One voice at a time is already the rule, and a room you
/// can see into is a harder question than the one this was wrong about.
pub fn nearest(ear: Vec3, here: Room, voices: &[(Room, Vec3)]) -> Option<usize> {
    voices
        .iter()
        .enumerate()
        .filter(|(_, (room, at))| *room == here && ear.distance(*at) <= CARRIES)
        .min_by(|one, other| {
            ear.distance_squared(one.1 .1)
                .total_cmp(&ear.distance_squared(other.1 .1))
        })
        .map(|(n, _)| n)
}

/// One voice: which room it is in, where it sounds from, what makes its
/// pieces and the seed they come off.
pub type Voice = (Room, Vec3, fn(u32) -> Samples, u32);

/// Every voice in the building.
///
/// Here rather than in the drawing, so that where a voice is and which room
/// that puts it in are one account and a test can walk them.
pub fn voices(reaches: f32) -> Vec<Voice> {
    vec![
        (Room::Cellar, crate::cellar::hearth(reaches), fire, 0xF12E),
        (Room::Baths, crate::spa::stream(reaches).0, trickle, 0x7121),
        (Room::Baths, crate::spa::tub(reaches).0, bubble, 0xB0B1),
        (
            Room::Baths,
            crate::spa::sauna(reaches).center(),
            steam,
            0x57EA,
        ),
        (Room::Hall, vec3(0.0, crate::EYE, 0.0), hum, 0x4040),
    ]
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

/// How long one piece of a voice runs.
///
/// **A piece and not the voice.** The fire does not stop and has no length;
/// what has a length is the buffer the engine is handed, and this is that. The
/// voice is the fire, and the slicing is plumbing. This was called `NOTE`,
/// which named the slice and left the thing it is a slice of with no name at
/// all, and `PIECES` then got stretched over the gap.
///
/// Short, because one voice is playing at a time and anything else from
/// somewhere in the world waits behind it: a splash a whole second late is a
/// splash for something you have forgotten doing.
pub const PIECE: f32 = 0.4;

/// The fire: a low roll with the odd crack in it.
pub fn fire(seed: u32) -> Samples {
    let mut next = noise(seed);
    let count = (RATE as f32 * PIECE) as usize;
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
    let count = (RATE as f32 * PIECE) as usize;
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

/// How far off a strike has to be to count as the next band out.
///
/// Three bands over 250 to 2,800 metres. The character of thunder changes with
/// distance in a way a level cannot carry, and with recordings that change is
/// which file is picked rather than a filter turned further: see
/// `res/thunder/README.md`, and spec 0016 for what that costs.
pub const MIDDLING_AT: f32 = 700.0;
pub const FAR_AT: f32 = 1_600.0;

/// Which band a strike this far off is heard in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rolled {
    /// Close enough that the strike and its own roll are one sound.
    Near,
    Middling,
    /// Far enough that the strike itself never arrives, only the roll.
    Far,
}

/// All of them, so a check can walk the bands rather than name them.
pub const ROLLS: [Rolled; 3] = [Rolled::Near, Rolled::Middling, Rolled::Far];

pub fn rolled(away: f32) -> Rolled {
    if away < MIDDLING_AT {
        Rolled::Near
    } else if away < FAR_AT {
        Rolled::Middling
    } else {
        Rolled::Far
    }
}

/// The recordings for a band, in the order they are picked from.
///
/// Recordings and not arithmetic, which is the second time this building has
/// come to that conclusion and for the same reason as the first. A noise burst
/// under an envelope is a noise burst under an envelope however carefully the
/// envelope was chosen, and thunder is a sound every ear knows by heart. See
/// `res/thunder/README.md`.
pub fn claps(band: Rolled) -> &'static [&'static [u8]] {
    match band {
        Rolled::Near => &[include_bytes!("../res/thunder/near1.wav")],
        Rolled::Middling => &[include_bytes!("../res/thunder/mid1.wav")],
        // two, because this is the band most strikes land in: the range runs
        // to 2,800 and everything past 1,600 is here
        Rolled::Far => &[
            include_bytes!("../res/thunder/far1.wav"),
            include_bytes!("../res/thunder/far2.wav"),
        ],
    }
}

/// How loud a strike this far off is, against one at the nearest.
///
/// The one thing about the sound that is still worked out from the distance
/// rather than picked, and so the one that still moves smoothly across a band
/// rather than in steps. Sound falls off faster than this in open air; the
/// exponent softens it, because a rumble that is honestly a fiftieth of a near
/// clap is a rumble nobody hears over an arcade.
pub fn carries(away: f32) -> f32 {
    (crate::garden::NEAR / away.max(1.0)).powf(0.7)
}

/// The hum a hall full of cabinets has.
pub fn hum(seed: u32) -> Samples {
    let mut next = noise(seed);
    let count = (RATE as f32 * PIECE) as usize;
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
    let count = (RATE as f32 * PIECE) as usize;
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

    /// The building, for the tests that ask about where things are in it.
    fn a_room() -> crate::room::Room {
        crate::room::Room::of(
            (0..12)
                .map(|n| {
                    crate::cabinet::Cabinet::found(
                        &format!("game{}", n),
                        std::path::Path::new("/nowhere"),
                    )
                })
                .collect(),
        )
    }

    /// Spec 0009: every voice is in the room it says it is in.
    ///
    /// Which room a voice is in and where it sounds from are one account now,
    /// and this is what holds them together: move the tub and the room it
    /// claims follows, or this fails and says so.
    #[test]
    fn every_voice_is_in_the_room_it_says_it_is() {
        let reaches = a_room().reaches;

        for (room, at, _, _) in voices(reaches) {
            assert_eq!(
                room_at(reaches, at),
                room,
                "a voice at {:?} says it is in the {:?} and stands in the {:?}",
                at,
                room,
                room_at(reaches, at)
            );
        }
    }

    /// Spec 0009: the garden does not hear the baths through its own floor.
    ///
    /// The fault this rule was written for, in the building's own numbers. The
    /// baths sit under the garden: from the middle of the garden the tub is
    /// 9.4 metres away and from its south end 5.4, both of them straight down
    /// through a floor, and both inside the 14 a voice carries. Nothing else
    /// was nearer, because the garden has no voice of its own, so the garden's
    /// background sound was a hot tub underneath it.
    ///
    /// Walked across the garden rather than checked in its middle, because the
    /// south end is where it was worst and the middle is where it was obvious.
    #[test]
    fn the_garden_does_not_hear_the_baths_through_its_floor() {
        let reaches = a_room().reaches;
        let places: Vec<(Room, Vec3)> = voices(reaches)
            .into_iter()
            .map(|(room, at, _, _)| (room, at))
            .collect();
        let middle = crate::garden::at(reaches);
        let mut near = 0;

        for step in 0..=10 {
            let along = middle.z - 7.0 + 14.0 * step as f32 / 10.0;
            let stood = glam::vec3(middle.x, 0.0, along);

            assert_eq!(
                room_at(reaches, stood),
                Room::Garden,
                "the test walked out of the garden at z {:.1}",
                along
            );

            let heard = nearest(stood, Room::Garden, &places);

            assert!(
                heard.is_none(),
                "from the garden at z {:.1} you hear the {:?}",
                along,
                places[heard.unwrap()].0
            );

            // and the thing that made it wrong is still true: something in the
            // baths is well inside earshot from here
            if places
                .iter()
                .any(|(room, at)| *room == Room::Baths && stood.distance(*at) <= CARRIES)
            {
                near += 1;
            }
        }

        assert!(
            near > 5,
            "only {} of eleven places in the garden have the baths within earshot, \
             so this test is no longer about anything",
            near
        );
    }

    /// Spec 0016: the storm is heard where the sky is, and the sky is a place
    /// rather than a floor.
    ///
    /// Written after getting it the other way about. The storm asked what was
    /// under your feet and sounded wherever that was gravel, which works in
    /// this building for a reason that is a coincidence: the garden's floor is
    /// gravel and nothing else here is. A later room with a gravel path
    /// through it would have been in a thunderstorm.
    ///
    /// So the two halves are here. Every room is checked, and the garden's own
    /// footprint is checked again from the cellar's depth, where the thing
    /// overhead is the garden's floor and not any weather.
    #[test]
    fn the_storm_is_heard_where_the_sky_is_and_not_where_the_gravel_is() {
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
        let middle = crate::garden::at(reaches);
        let hall = glam::vec3(0.0, 0.0, 0.0);

        assert!(
            crate::garden::under_the_sky(reaches, middle),
            "the middle of the garden is not under the sky"
        );
        for (what, stood) in [
            ("the hall", hall),
            (
                "the nook",
                glam::vec3(
                    -crate::room::WALL - crate::room::CABINET.x - 1.0,
                    0.0,
                    -reaches + 2.0,
                ),
            ),
            (
                "the cellar",
                glam::vec3(0.0, -crate::cellar::DOWN, -reaches + 4.0),
            ),
            // and the one that matters: the garden's own ground plan, from
            // under its floor. Whatever is down there, it is not weather.
            (
                "under the garden",
                glam::vec3(middle.x, -crate::cellar::DOWN, middle.z),
            ),
        ]
        .iter()
        .copied()
        {
            assert!(
                !crate::garden::under_the_sky(reaches, stood),
                "{} at {:?} is under the sky",
                what,
                stood
            );
        }

        // and the floor is read from the place rather than the place from the
        // floor, which is what keeps the two in step
        assert_eq!(underfoot(reaches, middle), Underfoot::Gravel);
        assert_eq!(underfoot(reaches, hall), Underfoot::Carpet);
    }

    /// Spec 0016: every band has thunder, and every one of it reads.
    ///
    /// The one test that matters for a bundled file. A WAV that will not parse
    /// is a band that strikes in silence, and the loading drops what it cannot
    /// read rather than crashing, so nothing at run time would ever say so.
    #[test]
    fn every_band_has_thunder_that_reads() {
        for band in ROLLS.iter().copied() {
            let files = claps(band);

            assert!(!files.is_empty(), "{:?} has no thunder at all", band);

            for (n, bytes) in files.iter().enumerate() {
                let got = Samples::from_wav(bytes)
                    .unwrap_or_else(|why| panic!("{:?} clap {}: {}", band, n, why));

                assert!(!got.is_empty(), "{:?} clap {} is silence", band, n);
                // long, and that is the point of them. Thunder is mixed and
                // not queued, so nothing here has the footsteps' cap, and a
                // roll that is over in half a second is a door slamming.
                assert!(
                    got.seconds() > 2.0,
                    "{:?} clap {} lasts {:.2}s, which is too short to roll",
                    band,
                    n,
                    got.seconds()
                );
                // and not so long that one is still going two strikes later
                assert!(
                    got.seconds() < crate::garden::STRIKES,
                    "{:?} clap {} lasts {:.2}s against {:.2}s between strikes",
                    band,
                    n,
                    got.seconds(),
                    crate::garden::STRIKES
                );
            }
        }
    }

    /// Spec 0016: every distance in the storm's range lands in a band, and the
    /// bands run in order.
    ///
    /// Walked across the whole range rather than checked at the two ends. The
    /// boundaries are the part worth walking: a band nobody can reach is a
    /// recording nobody hears.
    #[test]
    fn every_strike_lands_in_a_band_and_the_bands_run_in_order() {
        let mut seen = Vec::new();

        for step in 0..=200 {
            let away = crate::garden::NEAR
                + (crate::garden::FAR - crate::garden::NEAR) * step as f32 / 200.0;
            let band = rolled(away);

            if seen.last() != Some(&band) {
                assert!(
                    !seen.contains(&band),
                    "the bands go {:?} and then back to {:?} at {:.0} m",
                    seen,
                    band,
                    away
                );
                seen.push(band);
            }
        }

        assert_eq!(
            seen,
            ROLLS.to_vec(),
            "the range reaches {:?} of the three bands",
            seen.len()
        );
    }

    /// Spec 0009: a voice's pieces are picked rather than cycled.
    ///
    /// A cycle of five at two and a half notes a second is a bar of five
    /// beats, and a listener catches the order long before they catch the
    /// sample. `bend`, which the floors use, cycles on purpose: you do not
    /// take two steps from the same place and the room moves between them.
    #[test]
    fn a_voices_pieces_do_not_come_round_in_a_bar() {
        let many = PIECES;
        let picks: Vec<usize> = (0..600).map(|n| adrift(n, many).0).collect();

        for which in picks.iter() {
            assert!(*which < many, "picked piece {} of {}", which, many);
        }

        // a cycle would have every piece the same as the one `many` before it
        let same = picks
            .iter()
            .skip(many)
            .zip(picks.iter())
            .filter(|(now, before)| now == before)
            .count();

        assert!(
            same * 3 < picks.len(),
            "{} of {} notes repeat a bar later, which is a bar",
            same,
            picks.len()
        );

        // and every one of them gets used
        for which in 0..many {
            assert!(
                picks.contains(&which),
                "note {} of {} never comes up",
                which,
                many
            );
        }
    }

    /// Spec 0009: and it is bent, within the same bounds a step is.
    #[test]
    fn a_voices_piece_is_bent_like_a_step() {
        let (mut moved, mut quietened) = (false, false);

        for n in 0..200 {
            let (_, pitch, quieter) = adrift(n, PIECES);

            assert!(
                (1.0 - PITCHED..=1.0 + PITCHED).contains(&pitch),
                "a note was pitched by {}",
                pitch
            );
            assert!(
                (1.0 - QUIETER..=1.0).contains(&quieter),
                "a note was taken to {}",
                quieter
            );

            moved |= (pitch - 1.0).abs() > PITCHED * 0.5;
            quietened |= quieter < 1.0 - QUIETER * 0.5;
        }

        assert!(moved, "no note was pitched much");
        assert!(quietened, "no note was quietened much");
    }

    /// Spec 0009: each room's notes differ from one another.
    ///
    /// Five of a note is only worth having if they are five sounds. Every one
    /// of these comes off a seed, so what this really asks is that the seed
    /// reaches the sound: a generator that took one and ignored it would give
    /// a room five copies of one note and look exactly like variety from the
    /// outside.
    ///
    /// A `Samples` is an `Iterator<Item = f32>`, so this is how you look
    /// inside one. It is not in the method list and I concluded it could not
    /// be done, deleted this test, and went off to add three accessors to the
    /// engine that already existed. Written down because the next person will
    /// read the same method list.
    #[test]
    fn a_voices_pieces_are_not_all_one_piece() {
        for (what, make) in [
            ("the fire", fire as fn(u32) -> Samples),
            ("the spout", trickle),
            ("the tub", bubble),
            ("the sauna", steam),
            ("the hall", hum),
        ] {
            let built: Vec<Vec<f32>> = (0..PIECES)
                .map(|n| make(0x1234u32.wrapping_add(n as u32 * 7919)).collect())
                .collect();

            for (n, one) in built.iter().enumerate() {
                assert!(
                    one.iter().any(|sample| sample.abs() > 1e-4),
                    "one of {} notes is silence",
                    what
                );

                for other in built.iter().skip(n + 1) {
                    assert!(
                        one != other,
                        "two of {}'s five notes are the same sound",
                        what
                    );
                }
            }
        }
    }

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
            // in the pool, where it is the water you hear and not the tile
            // it stands on. Spec 0009.
            (
                "the pool",
                {
                    let (middle, _, deep) = crate::spa::pool(reaches);
                    vec3(middle.x, middle.y - deep * 0.5, middle.z)
                },
                Underfoot::Water,
            ),
            // and a stride from its coping, where it is the tile again
            (
                "the poolside",
                {
                    let (middle, size, _) = crate::spa::pool(reaches);
                    vec3(
                        middle.x,
                        -crate::cellar::DOWN,
                        middle.z + size.y * 0.5 + 0.6,
                    )
                },
                Underfoot::Tile,
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
                // and short enough that no more than `OVERLAP` of them are
                // sounding at once at the fastest this floor can be crossed.
                // Two comments stood here saying this twice, one of them left
                // over from when the cap was there to keep a queue from
                // falling behind.
                assert!(
                    got.seconds() <= longest(on),
                    "{:?} step {} lasts {:.3}s against {:.3}s a step may last",
                    on,
                    n,
                    got.seconds(),
                    longest(on)
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

    /// Spec 0009: no more than two footsteps are ever sounding at once.
    ///
    /// Read off the files rather than off `longest`, which is where this test
    /// earns its keep. The version before it asked whether `longest` was under
    /// the gap and both sides of that came out of `pace`, so it was an
    /// arithmetic identity dressed as an invariant: it passed on the day
    /// running was added and halved the real gap, and it would have passed
    /// whatever `RUNS` was set to.
    ///
    /// This one takes a recording's actual length, divides it by the shortest
    /// gap the building can produce on that floor, and counts the shoes. Turn
    /// `RUNS` up to three and it fails, which is the whole point.
    #[test]
    fn no_more_than_two_steps_sound_at_once() {
        for on in FLOORS {
            let gap = shortest_gap(on);

            for (n, bytes) in steps(on).iter().enumerate() {
                let got = Samples::from_wav(bytes).expect("a step that reads");
                let shoes = (got.seconds() / gap).ceil();

                assert!(
                    shoes <= OVERLAP,
                    "{:?} step {} lasts {:.3}s, lands every {:.3}s, so {} go at once",
                    on,
                    n,
                    got.seconds(),
                    gap,
                    shoes
                );
            }
        }

        // and the cadence is a cadence and not a drum roll, at either speed
        let walked = crate::SPEED / stride(false);
        assert!(
            (1.5..=3.5).contains(&walked),
            "{:.1} steps a second is not a person walking",
            walked
        );

        let run = crate::SPEED * crate::RUNS / stride(true);
        assert!(
            (3.0..=4.5).contains(&run),
            "{:.1} steps a second is not a person running",
            run
        );
    }

    /// Spec 0009: the stride lengthens when you run, so the cadence rises by
    /// less than the speed does.
    ///
    /// A sprinter covers more ground per step rather than taking the same step
    /// twice as often, and this is the half of it that is easy to get
    /// backwards. Running arrived with the stride left alone, so the whole of
    /// `crate::RUNS` went into the cadence and a sprint across the hall was
    /// 5.3 steps a second: a buzz, and not what anybody has ever heard a
    /// person do.
    ///
    /// The control is `RUN_STRIDE` set back to `STRIDE`: the second assertion
    /// then reads a cadence doubled against a speed doubled and fails.
    #[test]
    fn a_run_is_a_longer_step_and_not_just_a_faster_one() {
        assert!(
            stride(true) > stride(false),
            "a running stride of {:.2} against a walking {:.2}",
            stride(true),
            stride(false)
        );

        let faster = crate::RUNS;
        let more_often =
            (crate::SPEED * crate::RUNS / stride(true)) / (crate::SPEED / stride(false));

        assert!(
            more_often < faster * 0.85,
            "running is {:.2} times as fast and {:.2} times as many steps, so              nearly all of it went into the cadence",
            faster,
            more_often
        );

        // and the longer stride is not so long that a step at a run is rarer
        // than one at a walk, which would be a stride growing faster than the
        // speed
        assert!(
            more_often > 1.0,
            "running takes {:.2} times as many steps, which is fewer",
            more_often
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
    fn one_voice_plays_at_a_time_and_it_is_the_nearest() {
        let here = Room::Baths;
        let voices = [
            (here, Vec3::new(0.0, 0.0, 0.0)),
            (here, Vec3::new(3.0, 0.0, 0.0)),
            (here, Vec3::new(40.0, 0.0, 0.0)),
            // and one a couple of metres away in another room, which is the
            // case the garden was losing to: near enough to win on distance
            // and not yours to hear
            (Room::Garden, Vec3::new(2.0, 0.0, 0.0)),
        ];

        // the nearest of the ones you can hear in your own room, and only one
        assert_eq!(nearest(Vec3::new(2.6, 0.0, 0.0), here, &voices), Some(1));
        assert_eq!(nearest(Vec3::new(-1.0, 0.0, 0.0), here, &voices), Some(0));

        // the next room's is nearest of the lot at 2.1 and is not heard
        assert_eq!(nearest(Vec3::new(2.1, 0.0, 0.0), here, &voices), Some(1));
        // and from in that room it is the only one heard
        assert_eq!(
            nearest(Vec3::new(2.1, 0.0, 0.0), Room::Garden, &voices),
            Some(3)
        );
        // a room with nothing in it is silent rather than borrowing
        assert_eq!(nearest(Vec3::ZERO, Room::Nook, &voices), None);

        // and nothing at all when they are all out of earshot
        assert_eq!(nearest(Vec3::new(200.0, 0.0, 0.0), here, &voices), None);
        assert_eq!(nearest(Vec3::ZERO, here, &[]), None);

        // the far one is past earshot even standing next to the others
        assert_ne!(nearest(Vec3::new(20.0, 0.0, 0.0), here, &voices), Some(2));

        // and a piece is short, because anything else from the world waits
        // behind it
        for (what, sound) in all() {
            if !matches!(what, "fire" | "trickle" | "hum" | "steam" | "bubble") {
                continue;
            }

            assert!(
                sound.seconds() <= PIECE + 1e-3,
                "the {} runs {} seconds and a piece is {}",
                what,
                sound.seconds(),
                PIECE
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
