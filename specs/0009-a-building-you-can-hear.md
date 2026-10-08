# 0009 A building you can hear

**Status:** implemented
**Date:** 2026-10-06

## What it is

Four rooms, and until now every one of them silent. The engine could play a
sound from a place and point your ears at it, per specs 0004 and 0019, and this
game never once asked it to. Spec 0044 of the engine is what makes it possible
to ask: the arcade works its sounds out the way it works out its carpet, its
wood grain and its wine bottles, which is arithmetic and no files.

**Nothing is recorded.** There is no audio in this repo and there is not going
to be. A splash is a short burst of noise with water's own colour on it, and a
clack is two short tones a fifth apart. Neither will be mistaken for the real
thing, and neither has to be: what they have to do is tell you that you went in
the water, and that something over there happened.

**There are no footsteps.** There were, and they were the best measured and
worst sounding thing in this building. The floor was rung as a struck object,
the ring was driven by a swelling contact rather than an impulse, each floor
had its own note and decay, there were several of every one so no two strides
repeated, and the carpet was given a rustle instead of a boom. Every one of
those was a real fault found and fixed, and the result still did not sound like
a person walking.

Jake pinned them, played on, and then said to take them out until they feel
natural. A sound that is nearly right is worse than no sound: silence is a
choice a room can carry, and a footstep that is not quite a footstep is a thing
the ear keeps going back to.

So they are gone rather than disabled. `Underfoot`, `step`, the per floor
table, the resonators that drove it and the eight tests that held it are all
out of the source, where git has them if they are ever worth another go. What
is left is what nobody has complained about: the splash, the clack, the bubble,
the fire, the trickle, the hum and the steam.

**Water** answers what you do to it. Going in is a splash, and its size comes
from how fast you went in. Wading on is a wash. The fountain
runs for as long as there is a fountain, which is always, so that one is a
sound the room has rather than one anything causes.

**One at a time.** The engine plays what it is given one after another, per
spec 0044, and that is not a mixer. Four notes a second, each a second long, is
a queue growing by three seconds every second: left running for two hours it was
a gigabyte of audio waiting its turn and a machine with nothing left to draw
with, which is what the pool looking like a wall of streaks turned out to be.

So the building plays the note of the room you are in, which is the nearest one
within earshot, and nothing else until it has finished. The notes are short for
the same reason: anything else from somewhere in the world waits behind one, and
a splash a whole second late is a splash for something you have forgotten doing.

**The rooms each have a note of their own.** The cellar has its fire. The baths
have the fountain and the tub's blower. The sauna, with its door shut, has the
stove ticking and the hiss of what is on it. The hall has the cabinets, which
hum the way a room full of them hums.

**Where a sound is heard from** is the engine's business, so everything that
comes from somewhere goes through `play_at` and the listener is moved to your
eye and pointed where you are looking, every frame. A sound with no place goes through
`play`. Nothing is left that has none, now the footsteps are out, but `play` is
the engine's and stays available.

## Acceptance criteria

- Every sound the building makes has samples in it and ends. — `noise::tests::every_sound_is_a_sound`
- None of them clips: no sample past one. — `noise::tests::nothing_is_louder_than_one`
- None of them starts or ends with a jump, which is a click. — `noise::tests::nothing_begins_or_ends_with_a_click`
- A splash is louder the faster you went in. — `noise::tests::a_splash_is_as_big_as_the_fall`
- One note plays at a time, and it is the nearest one within earshot. — `noise::tests::one_note_plays_at_a_time_and_it_is_the_nearest`

### Verified by hand

Run the arcade and walk from the hall to the baths.

- The floor changes under you four times and you can hear each one.
- The fire, the fountain and the tub come from where they are and swing as you
  turn, which is spec 0019 of the engine doing its job through this.
- Going into the pool is a splash and walking in it is a wash.

## Out of scope

**Recordings.** Nothing here reads a file, and nothing in this repo is a sound
anybody made with a microphone.

**Music.** The building has no opinion about what it sounds like beyond what is
in it.

**Mixing.** The engine takes a sound and plays it, per spec 0044. Anything about
how loud two of them are together is a thing this game does by choosing how
loud it makes each one.
