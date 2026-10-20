# 0029 A corridor with a room for every number

**Status:** draft
**Date:** 2026-10-20

## Goal

A second door in the far wall of the hidden room, opening onto a corridor of
numbered doors that does not end. It is aleph's hotel, which was built as a
game on 2026-10-04 and parked the same night, put where it actually belongs:
not in a cabinet, because it is not a game, but in the wing of this building
that already exists to make a quantity too large to picture into something you
can walk at.

## Why it is through that door and not in the hall

Spec 0001 gives the hall a cabinet per game. aleph is not a game and its own
spec says so in its first paragraph, so a cabinet that launched it would be a
cabinet that lies about what is behind it.

The back room of spec 0024 is the near miss. It holds the building's own
instruments, which are neither games nor shapes, and that sounds like the
slot. But an instrument is a thing the building uses on itself, and this is an
exhibit. On a bench beside the test rig it reads as tooling.

The wing behind the wall is already this. Spec 0011 put a vast dark space
there with no door and no sign. Spec 0015 filled it with a solar system that
fits by telling one lie, written down and admitted to, and spec 0018 put the
true proportions through a door in the far wall: one scale, Neptune thirty
three kilometres out, and three minutes to ride there. That is the same move.
You cannot hold an astronomical unit in your head and you cannot hold
aleph-null in your head, so the building lets you walk at one until the size
stops being a number and starts being a distance.

And the clinching reason is structural. **A corridor is not a thing you put in
a room.** Everything else in this building is an object with a room round it.
This is architecture, so it should be the way through rather than a stop on
the way: a second doorway beside spec 0017's, where one leads to a distance
too big to picture and the other to a count that is.

## Behavior

### The second door

In the far wall of spec 0011's room, beside the one spec 0017 hung. That one
sits on the room's own axis, dead opposite the sun and 79.5 metres past it,
`DOOR_WIDE` at 1.5. This one is its pair, off the axis by enough that the two
read as a choice rather than as one wide opening.

A pair is the point. You arrive at the far wall having walked a hundred and
fifty nine metres of nothing, and there are two ways on.

### The corridor

aleph's own numbers, because they work and because two lists of one thing
drift:

| | |
| --- | --- |
| rooms apart | 1.5 m |
| corridor across | 3.2 m |
| corridor tall | 2.4 m |
| door | 1.05 by 1.9 |
| a door swings to | 1.75 rad, over 0.4 s |

Rooms run 1, 2, 3 and on without end, odd down the left and even down the
right. That split is not decoration: the rule that doubles everyone sends
every guest into an even room, so the entire left wall opens in one move, and
it is the clearest picture this thing has of freeing infinitely many rooms.
Every door carries a plaque with its number beside the frame.

A shut door is a room with somebody in it. An open one is free and dark.

### How long it is

There is no end and the building does not pretend there is one by moving the
corridor while you are not looking. It is built long and the numbers are real.

Where it actually stops is f32. Past 16,777,216 metres a float cannot tell one
metre from the next, which at 1.5 metres a room is **11.2 million rooms**, and
at this building's walking speed of 4.2 m/s it is 46 days of holding W. Spec
0051's eye-relative rendering is what buys that, by keeping the arithmetic
near the camera instead of near the origin.

Eleven million is not infinity. Say so on the plaque at the threshold rather
than hiding it, the way spec 0015 writes down its lie.

### The rule, and watching it travel

This is the half aleph could not have and a walkable corridor can.

In the game, applying a rule swings the doors and the result appears. aleph's
own parked note calls that the lesser half: the guests do not travel from room
n to room a·n + b, so the transformation itself is invisible.

Here you are standing in the corridor when it fires. The doors do not all
swing at once. The change starts at room 1 and runs away from you down the
corridor at a speed you can watch, so what you see is a wave going somewhere
you cannot see the end of. Stand near the door and it leaves you behind. Stand
far down and it arrives.

## The thing to settle before any of this is built

**There are no guests.** That is the first of aleph's three parked reasons and
it is the one that moving the thing into a building makes worse, not better.

A shut door is the only evidence anyone is in there. In a game seen down a
camera that is thin. In a building you walk, with your own footsteps, it is
thinner: now there is visibly one person in the hotel and every door is shut.

This building has never drawn a person. That is the same wall the hall of
mirrors idea runs into, and it is not a thing to discover again halfway
through building a corridor. The options are to draw guests and walk them,
which is the largest piece of work anywhere in this repo, or to settle on what
stands for a guest without being one: light under a door, a sound behind it,
a shadow crossing it.

**Nothing below is worth building until that is decided**, because it changes
what a door is.

## Acceptance criteria

Arithmetic, which is what this building tests. None of it is written.

- Odd rooms are on the left and even on the right, all the way along. — `aleph::tests::the_sides_alternate`
- Doubling frees every room on the left side and no room on the right. — `aleph::tests::doubling_empties_the_odd_side`
- A body your size walks the corridor from the threshold with doors open on
  both sides, which is the case that fails: two facing doors at 1.75 rad leave
  1.13 m of a 3.2 m corridor, against a body of 0.9 and this building's usual
  1.05 of clearance. — `aleph::tests::walked_with_every_door_open`
- The wave moves away from room 1 rather than arriving everywhere at once. — `aleph::tests::the_change_travels`
- Room numbers are right at the far end as well as the near one, where a float
  is at its worst. — `aleph::tests::the_last_room_is_numbered_correctly`
- The second door does not overlap spec 0017's, and neither is blocked by the
  columns spec 0015 keeps off the way. — `behind::tests::both_far_doors_are_clear`
- Nothing of the corridor is drawn inside the hidden room, which is the fault
  this building has fought seven times. — `aleph::tests::nothing_of_the_corridor_is_in_the_room`

### Verified by hand

- The corridor reads as having no end rather than as being long.
- The wave is worth standing still for.
- Walking through a doorway with the door swung open does not catch on a
  shoulder, which 1.13 m says it should not and which this building has been
  wrong about before.

## Out of scope

**The dials.** aleph's third parked reason is that they are a menu with extra
steps: three right answers and a player works them out in a couple of minutes.
Nothing is gained by carrying that in here. What the rule is and how it is
chosen in a building you walk is its own question and belongs in its own spec.

**aleph the crate.** It stays parked, its manifest stays renamed, and it stays
out of this project's gate. This is the corridor rebuilt here, not the game
launched from here.

**Guests.** Named above because it blocks this, not because this spec answers
it.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
