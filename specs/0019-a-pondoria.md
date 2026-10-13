# 0019 A pondoria

**Status:** draft
**Date:** 2026-10-13

## Goal

A room you stay in, which this building does not have.

Jake's word, on a play on ponder. Every room here is for doing: a cabinet to
start, a bottle to turn, a ball to roll, a planet to walk to. The only thing
you can do in this one is stay, and the room has to be worth that.

## Behavior

### The experiment already ran once

Spec 0011 built a room with nothing in it, and said so in as many words: "a
space, very large and very dark, with nothing in it to look at and no reason
given for it being there", with "no toy, no cabinet, no key" ruled out
expressly. Spec 0015 struck that and put a solar system in it, and said why:
"A hidden empty space is worth one moment."

That is the finding and it is not in dispute. An empty room is a thing you
walk through once. So a pondoria is not an empty room, and the whole of this
spec is what it is instead.

### The second minute has to differ from the first

**Nothing here changes on its own.** The fire in the cellar flickers, which is
a loop: the tenth second and the hundredth are the same second. The koi swim a
circuit. The orrery turns, and turns back when you press L. Every room is in
the state you found it in, for ever, and that is why there is no reason to be
in any of them once you have worked out what they hold.

So the one thing this room must have is a change you cannot hurry and cannot
reverse. Light crossing a floor. Water going still after something fell in it.
A fire burning down to nothing and staying there. The test is whether you can
tell how long you have been in the room by looking at it.

That is also what tells a pondoria from a screensaver: a loop says nothing
about how long you have been watching, and a change does.

### It asks nothing

No cabinet, no bench, no toy, nothing under the sight, no line in the corner
offering a key. `use_what_i_see` finds nothing here and the sight stays plain.

This is a real constraint rather than an absence, and it has to be defended in
code, because every other room in this building answers you and the machinery
to answer is reached from everywhere.

The one thing it may have is somewhere to sit. The building has benches and
every one of them is a table with a toy on it; there is nowhere in it to sit
and do nothing.

### Where it goes

Undecided, and these are the three readings.

**The garden, given time.** The cheapest and probably the best. The garden is
already the room nobody is doing anything in: moss, a koi pond, stone lanterns,
and a sky. It has one thing the others do not, which is a sky, and a sky is the
natural home of a change you cannot hurry. A sun that crosses it over ten
minutes turns the garden into a pondoria without a wall being built, and the
lanterns start earning their light.

**Past Neptune.** The far end of spec 0018's line, where you arrive after 42
seconds of riding and there is nothing further. It is the loneliest place in
the building and it is already built. What it lacks is any change at all: out
there even the sun is fixed.

**A room of its own.** The most work and the most likely to end up as spec
0011 did, because a new room starts empty and the temptation is to furnish it.

### What is in it

Deliberately almost nothing, and what little there is must be the change
rather than scenery round it. One thing that moves and a place to sit facing
it. If a second thing is wanted, the right question is whether the first is
not enough rather than what else would look good.

## Acceptance criteria

Nothing is built yet, so these are the claims to prove and not the proof.

- You can tell how long you have been in the room by looking at it, which is
  to say the state at ten minutes is not the state at one.
- The change does not loop and does not reverse.
- Nothing in the room answers the sight, and the corner offers no key.
- The walk in and the walk out are the same walk, so it is somewhere you go
  rather than somewhere you end up.

### Verified by hand

- It is worth a second visit. Only Jake can say that, and the honest version
  of this criterion is that the room fails if he does not go back to it.

## Out of scope

Sound, which is its own question and a loop by nature. Anything with a state
the player sets, which is a toy. Day and night anywhere but in the room this
spec settles on, because a clock that runs in one room and not the next is two
accounts of what time it is.
