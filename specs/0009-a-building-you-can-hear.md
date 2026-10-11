# 0009 A building you can hear

**Status:** implemented
**Date:** 2026-10-06, amended 2026-10-11

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

**The footsteps are recordings.** Everything else in here is worked out by the
code that plays it; these are the exception, and the exception was earned.

They were synthesised first and they were the best measured and worst sounding
thing in the building. The floor was rung as a struck object, the ring was
driven by a swelling contact rather than an impulse, each floor had its own note
and decay, there were several of every one so no two strides repeated, and the
carpet was given a rustle instead of a boom. Every one of those was a real fault
found and fixed, and it still did not sound like a person walking, so they came
out entirely: a sound that is nearly right is worse than no sound, because
silence is a choice a room can carry and a footstep that is not quite a footstep
is a thing the ear keeps going back to.

What was wrong was never the pacing or the number of variants. A step is the one
sound in here you hear hundreds of times an hour and the one your ear already
knows by heart, and a noise burst under an envelope is a noise burst under an
envelope however carefully the envelope was chosen.

So there are twenty recordings bundled in `res/steps`, all of them CC0, from
three sources that `res/steps/README.md` names. Four floors: carpet in the hall,
boards in the nook and the cellar and on the stair, tile in the baths, gravel in
the garden. Four of each, and six of the boards.

Four floors and not five. There was a stone, under the cellar and the stair, and
no floor in this building is stone: the cellar's is hardwood laid over the stone
it sits on and the stair's treads are drawn with the same boards. What it
sounded like was walking on pebbles in a room with a wooden floor, which is what
it was.

**What you are standing on is worked out from where you are**, and not from a
flag set on the way through a door. A flag is a second account of where you are
and this building has learned what two accounts of one thing do. It reads in the
order the place is stacked: everything below the hall, then the stair, then the
two rooms off the hall's left wall, then the hall. The garden and the nook are
both through that wall and the nook stops short of the garden, so which side of
the wall you are on is not enough on its own.

**One is played every stride**, paced by how far you have actually walked rather
than by a clock, so slowing down slows them and standing still is silence.

**A stride and not a pace.** The stride was 0.85 of a unit, which is a pace,
and at the speed this building walks you that is nearly five steps a second: a
sprint cadence under a body that is plainly not sprinting. It is 1.6 now, which
is a jog's stride at a jog's speed and about two and a half steps a second.

**Footsteps are allowed to overlap, and the engine is why.** This was the
reverse until 2026-10-11, and it was wrong twice over.

A player in rodio is a queue, so a step handed to it while one is still
playing waits its turn. A step that outlasts the gap to the next one therefore
never finishes before the next is due, and the backlog grows for as long as you
walk. At the old 0.85 stride the recordings ran to most of a second against a
gap of a fifth of one, so ten seconds of walking left half a minute of
footsteps still to play: they went on after you had stopped. The answer then
was `noise::longest`, a cap taken from the stride and the walking speed, with
any file over it dropped at load.

That cap broke the day shift was added, in the way a derived number does. It
came off `noise::pace`, which is the walking speed, so running halved the real
gap and left the cap measured against a walk. The recordings are 0.320s and
the gap at a run is 0.190s, and the fault came back exactly as before. The
test that was supposed to hold this took both sides of its comparison from
`pace`, so it was an identity and passed throughout.

The fix is in the engine rather than here. `SoundSystem::play` hands the sound
to the sink's mixer instead of appending it to a player, per spec 0004 of the
engine, so two steps that overlap are two steps overlapping and nothing
accumulates. Spec 0044 already promised this in as many words, that a sound
"can be played again before the first has finished", and `play` was the one
path where it was not true.

`noise::longest` stays, with a different job and an honest derivation. It is
`noise::OVERLAP` strides at `noise::fastest`, the quickest that floor can be
crossed, so a step is over before the step after next lands. Three shoes going
at once is a smear whatever is playing them. `fastest` is named apart from
`pace` because that is the pair that came apart: everything wanting the
shortest gap asked `pace` for it, and `pace` quietly stopped being the shortest
gap.

**No step is the one before it.** The repetition is what gives a footstep away,
not the recording, so the file walks the list in order and both the pitch and
the level move a little every time. Picked at random instead, the same file
comes up twice running every few steps and that is the one pair anybody notices.
A twelfth of a tone and a sixth of the level, which is as far as a step can go
before it is a different shoe on a different floor.

Both of those are applied as the sound goes out, not baked into the files: the
engine's `Samples::pitched` and `Samples::gain` share the buffer they came from,
so a step that is never quite the last step costs nothing. Spec 0045 of the
engine.

**How loud a floor is, is a decision.** The files are all normalised to one peak
so that it can be: left alone, carpet would be as loud as stone if the carpet
recording happened to be the hotter file. Carpet is the quiet one and stone is
the loud one.

**The pool has no sound underfoot.** None of the three CC0 packs has a wade, and
nothing in them is close enough to stand in for one. Going in still splashes;
walking about in it does not.

**Water** answers what you do to it. Going in is a splash, and its size comes
from how fast you went in. Wading on is a recording like any other floor's.
The fountain runs for as long as there is a fountain, which is always, so that
one is a sound the room has rather than one anything causes.

**The pool is a floor.** Standing in it, what you hear is the water you are
pushing through and not the tile under your feet, so it takes the tile's place
rather than sounding over it. A floor and its water sounding together would be
available now that `play` mixes, and would still be wrong: you do not hear your
shoe on the bottom of a pool.

Which floor you are on is read off where you are, and the pool asks the same
`spa::wading` that slows you down in it. A second account of being in the
water is a second thing to come apart.

**A step in water is allowed to be longer.** The gap between steps is not the
same on every floor: you wade at `spa::WADE` of your walking pace and shift
does nothing in the water at all, so a step in the pool lands every nine
tenths of a second against a fifth of one on a dry floor at a run. One cap for
every floor would either cut the splash off short or hold every dry floor to a
splash's length. `noise::fastest` names the speeds and `noise::longest` works
the cap out from whichever applies.

**One note at a time.** The room notes go out through `play_at`, which still
appends to a player and runs them one after another. Four notes a second, each
a second long, is a queue growing by three seconds every second: left running
for two hours it was a gigabyte of audio waiting its turn and a machine with
nothing left to draw with, which is what the pool looking like a wall of
streaks turned out to be. The notes are paced to start as the last one ends and
rely on the queue to hold that, so moving them to the mixer would change what a
room sounds like and is not part of this.

**Several of each note, picked and bent.** One note queued again the moment
it runs out is the same four tenths of a second played two and a half times a
second, which is a loop and is heard as one whatever is in it. Each room has
`noise::AIRS` of its note, each off its own seed, and the one to play is
picked by `noise::adrift` and bent in pitch and level the way a footstep is.

Picked and not cycled. The floors cycle, on purpose, because you do not take
two steps from the same place and the room moves between them. A room's note
comes from one place at a fixed rate, so five in a fixed order is a bar of
five beats and a listener catches the order long before the sample.

A pick does come up twice in a row now and then, and that is fine: two of the
same four tenths running is a slightly longer sound. It is the order that
gives a loop away rather than the repeat.

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
- A room's note does not come round in a bar. — `noise::tests::a_rooms_note_does_not_come_round_in_a_bar`
- And it is bent within the bounds a step is. — `noise::tests::a_rooms_note_is_bent_like_a_step`
- The pool sounds like water and its edge like tile. — `noise::tests::each_room_sounds_like_its_own_floor`
- No more than two footsteps sound at once, at a run and in the water. — `noise::tests::no_more_than_two_steps_sound_at_once`
- None of them clips: no sample past one. — `noise::tests::nothing_is_louder_than_one`
- None of them starts or ends with a jump, which is a click. — `noise::tests::nothing_begins_or_ends_with_a_click`
- A splash is louder the faster you went in. — `noise::tests::a_splash_is_as_big_as_the_fall`
- One note plays at a time, and it is the nearest one within earshot. — `noise::tests::one_note_plays_at_a_time_and_it_is_the_nearest`
- Every floor has footsteps and every one of them reads. — `noise::tests::every_floor_has_footsteps_that_read`
- No two floors share a recording. — `noise::tests::no_two_floors_sound_the_same`
- Each room sounds like what it is floored with. — `noise::tests::each_room_sounds_like_its_own_floor`
- No step is the file, the pitch or the level of the one before it. — `noise::tests::no_step_is_the_one_before_it`
- A step is bent and not mangled. — `noise::tests::a_step_is_bent_and_not_mangled`
- A step is over before the step after next lands, read off the files. — `noise::tests::no_more_than_two_steps_sound_at_once`
- Carpet is the quiet floor and tile the loud one. — `noise::tests::carpet_is_the_quiet_floor_and_tile_the_loud_one`

### Verified by hand

Run the arcade and walk from the hall to the baths.

- The floor changes under you four times and you can hear each one.
- The fire, the fountain and the tub come from where they are and swing as you
  turn, which is spec 0019 of the engine doing its job through this.
- Going into the pool is a splash and walking in it is a wash.
- Running across the hall and stopping stops the footsteps with you, rather
  than leaving them to run on. This is the one that was wrong on 2026-10-11 and
  it cannot be heard in a test.
- A sprint sounds like a sprint rather than a smear, with two steps' worth of
  overlap in it.

## Out of scope

**Recordings.** Nothing here reads a file, and nothing in this repo is a sound
anybody made with a microphone.

**Music.** The building has no opinion about what it sounds like beyond what is
in it.

**Mixing.** The engine takes a sound and plays it, per spec 0044. Anything about
how loud two of them are together is a thing this game does by choosing how
loud it makes each one.
