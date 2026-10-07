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
to be. A footstep is a short burst of noise shaped by an envelope, a splash is
the same burst with water's own colour on it, and a clack is two short tones a
fifth apart. None of these will be mistaken for the real thing, and none of them
has to be: what they have to do is tell you that you are walking, that you went
in the water, and that something over there happened.

**A footstep is the floor ringing.** Filtered noise under an envelope is a
machine, however the noise is filtered, because noise has no pitch to hold and
holding a pitch is the whole of what being struck sounds like. A struck tile
rings for a few hundredths of a second, a board rings lower and longer, and a
carpet rings at nothing at all. So each floor has its own notes, each dying away
at its own rate, and the noise over them is only the scuff of the two meeting.

The notes are rung rather than played. A pure sine faded out is a synthesiser:
one frequency and nothing else, and the ear hears a note being sounded instead
of a thing being hit. Each note here is a resonator fed a few thousandths of a
second of noise, which is the blow, so the roughness of the blow stays in the
ring and that roughness is most of what says struck.

**Which way up a floor's noise sits** is most of what tells one from another.
The pile of a carpet is fibres moving, which is all top end, and the one thing a
carpet has not got is bottom, because the pile is there to swallow it. Taken the
other way up it came out as a boom with the rustle thrown away, which is what it
was for three goes at this. Tile is the other way about and water is the furthest
from either.

Carpet keeps one note, very low and very quiet: the floor under it, heard
through it. And it is the quietest floor in the building, because a carpet is
what you put down to stop hearing the floor.

The shoe is a soft one, a slipper or a sneaker, which is what anybody wandering
round their own basement has on, so there is no click anywhere in here. The sole
has a rubbery note of its own and it is the same sole on every floor.

**Nothing in a step happens suddenly.** A hard thing striking another is over in
a thousandth of a second, and that is a hammer hitting a nail, which is what
these sounded like when the floor was rung by an impulse. A sole does not
strike, it compresses: it swells over about a fiftieth of a second and falls
away over twice that, and the floor's own note is a hint under the scuff rather
than the main event.

Water does not ring. It is pushed aside rather than struck, so its step swells
instead of striking and washes for much longer than any of them.

There are several of each, and this matters more than their shape: one sample
heard every stride is a machine, and no two steps anybody has ever taken were
the same sound.

**Footsteps** come from under you, at a pace taken from how far you have
actually walked rather than from a timer, so slowing down slows them. What you
are walking on decides which one you hear: carpet in the hall, boards in the
nook, stone on the stair, tile in the baths, and water when you are in the pool.
In the water they are slower and duller, which falls out of the pace being
distance rather than time.

**Water** answers what you do to it. Going in is a splash, and its size comes
from how fast you went in. Wading on is a wash under the footsteps. The fountain
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
eye and pointed where you are looking, every frame. A sound with no place, which
is only the footsteps under you, goes through `play`.

## Acceptance criteria

- Every sound the building makes has samples in it and ends. — `noise::tests::every_sound_is_a_sound`
- None of them clips: no sample past one. — `noise::tests::nothing_is_louder_than_one`
- None of them starts or ends with a jump, which is a click. — `noise::tests::nothing_begins_or_ends_with_a_click`
- A footstep is taken from distance walked, not from a clock. — `noise::tests::footsteps_are_paced_by_the_walking`
- Wading is slower underfoot than walking the same ground. — `noise::tests::wading_is_slower_underfoot`
- What you are standing on decides which step you hear. — `noise::tests::what_is_underfoot_decides_the_step`
- A splash is louder the faster you went in. — `noise::tests::a_splash_is_as_big_as_the_fall`
- No two of a floor's footsteps are the same sound. — `noise::tests::no_two_footsteps_are_the_same`
- Nothing in a footstep happens suddenly: the sole compresses rather than strikes. — `noise::tests::nothing_in_a_footstep_happens_suddenly`
- A hard floor rings at its own note and a soft one does not ring. — `noise::tests::a_hard_floor_rings_and_a_soft_one_does_not`
- Carpet is a rustle, not a boom, and is the quietest floor. — `noise::tests::carpet_is_a_rustle_and_tile_is_a_crack`
- A ring is driven by the blow and dies away, and is not a sine. — `noise::tests::a_ring_is_driven_by_the_blow`
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
