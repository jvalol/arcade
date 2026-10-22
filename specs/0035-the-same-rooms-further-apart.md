# 0035 The same rooms, further apart

**Status:** implemented
**Date:** 2026-10-22
**Built:** 2026-10-22, the same day it was written.

## Goal

A second corridor off the hotel's lobby, with a room for every counting
number in it, and the rooms laid out so you cannot reach them.

Spec 0029's corridor has a door every 0.75 metres. Walk for a minute and you
have passed three hundred of them. This one puts door n at the nth Fibonacci
number of metres: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 and on.

**Both corridors are two kilometres long.** One of them holds two thousand
six hundred rooms. The other holds sixteen.

## Why it belongs beside the hotel and not somewhere else

The hotel's whole claim is that the rooms do not run out, which it makes by
having a room for every counting number and putting them where you can walk
past them. Nothing about that claim says they have to be reachable.

So this is the same list, re-spaced. Every room still has a number, the
numbers still go on for ever, and none of that changed. What changed is
sixty centimetres of plaster between each one, and now you cannot count to
twenty by walking.

That is a thing the building can show and nothing else in it does. Spec 0018
made a distance walkable, spec 0030 made a count walkable, spec 0031 made an
argument walkable. This makes a growth rate walkable, by being the first
thing here that outruns you.

## Behavior

### Where the doors are

Door n stands F(n) metres along, counted from the mouth:

| door | 1 | 4 | 8 | 10 | 12 | 14 | 16 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| metres | 1 | 5 | 34 | 89 | 233 | 610 | 1597 |

The first eight are a stroll: thirty four metres, eight seconds at a walk.
Door 12 is nearly four minutes there and back. Door 16 is most of a mile and
nobody is going to do it.

**The gaps are the sequence too.** The distance from door n to door n+1 is
F(n−1), so the corridor is made of the same numbers twice over, and each gap
is about 1.618 times the one before it. Walking at a constant speed, the
doors arrive at intervals that grow by the golden ratio: nothing about your
walking changes and the world keeps stretching.

### The doors are the hotel's doors

Same size, same plaque, same strip of light under them, same alternating
sides. **Nothing about a door says which corridor it is in.** That is the
point: the only difference between the two corridors is where the doors are,
and if the doors looked different the room would be making some other claim.

A guest behind each one, by spec 0029's settlement, which is a light under a
shut door.

### It counts for you while you walk

Jake walked it built and said it could use some utility, otherwise it is a
long hall with doors you cannot open. He is right, and the section below
talked itself into the thin version: a decision made by giving up is a
decision the way staring at a wall is an activity.

**The comparison is the room and it was nowhere in it.** The claim is that
these are the hotel's rooms re-spaced, and a reader could only get that by
walking both halls and remembering. So the hall counts: how many doors you
have passed, how far that is, and how many of the hotel's the same walk
would have been. At nine metres it is five against eleven. At a hundred and
forty six it is eleven against a hundred and ninety four. You are the one
moving both numbers and they are not moving together.

**It answers to standing somewhere, not to aiming.** Every other readout in
this building is what you get for pointing at a thing, which is right for a
cabinet and wrong for a hall whose whole content changes with every step.
Nobody is going to stop and aim at a wall to find out how far they have come.

### What you do is decide to stop

There is nothing to press. You walk, and at every door there is one more a
little further off, and the question is whether to go on. It is built so the
answer is yes about eleven times and then obviously no.

**That is the decision**, and it is the first room here whose decision is
made by giving up. The hotel is endless and strolling; this is endless and
exhausting, and they are the same rooms.

### Where it goes

~~On the wall the desk is not on.~~ Struck on the day it was written. The
desk is *on* the only clear wall, in the middle of it, which leaves 2.7
metres free at either end. The corridor leaves east of it, so the desk stands
between the two mouths, which is better than the spec asked for: you come in
and it is the thing between the two halls rather than beside one of them.

It is 2.4 wide where the hotel's is 3.2, because 2.7 is what there is and the
last 0.3 keeps its east wall out of the plane of the lobby's. A corridor is
not a door, and the doors are the condition.

### Where it goes, as written

Off the hotel's lobby, on the wall the desk is not on, so that two corridors
leave the same room and you can see both from the door. The probe says that
side is clear: six solids within forty metres, and those are the register's
east end, which a corridor leaving east of the desk misses.

Two corridors out of one lobby is the whole pairing. Sited anywhere else this
is a corridor with sparse doors.

## What this must not become

**Doors that grow.** If each door were 1.618 times the last, the corridor
would look identical from every door and the room would be about
self-similarity rather than growth. That is a good room and it is not this
one, and it probably belongs with the imaginarium.

**A ramp.** The doors are numbered and the numbers are the thing that makes
it bite: you have walked two hundred metres and the door says twelve.

**An explanation.** No panel about the golden ratio. The hotel next door
does not explain itself and neither does the register.

**A second window.** Spec 0030 draws forty of the hotel's doors around
wherever you stand. Sixteen doors is the whole corridor, so there is nothing
to window.

## What building it found

**The golden ratio converges rather than being constant.** 8/5 is 1.600 and
13/8 is 1.625: the early gaps straddle it. The first version of
`every_gap_is_the_golden_ratio_of_the_last` asserted the limit from door 6
and failed on the sixth. What is true, and what it says now, is that each
door is closer to the ratio than the one before.

**The flood fill came back empty.** `Room::fillable` keeps the hotel's two
kilometres out of the fill's cell budget by dropping boxes that reach far
west, on `x` alone. A second two kilometre hall running in `z` is invisible to
that, so the fill went over budget, refused, and returned nothing. The test
that reads it says `the fill reached nothing, so the next assertion proves
nothing`, which is why it was a failure rather than a pass.

**There were two box-count assertions, not one.** Adding the corridor's five
boxes to one of them left the other short, and a sloppy second pass added them
to the first one twice.

**And between the doors it was a featureless black tube.** Past door eight
the next one is fifty five metres on and a sconce reaches two and a half, so
there was nothing to see and, worse, nothing sliding past to say you were
moving. A skirting and a rail down each wall, cut at every door, is what the
hotel already uses and is what makes the distance legible. The spec was right
that the room is the walk; it had not noticed that a walk needs something to
walk past.

## Acceptance criteria

- Door n stands at F(n) metres, and the gaps are the sequence
  again. — `twine::tests::the_doors_are_where_the_sequence_says`
- Sixteen doors fit in the same two kilometres that hold two thousand six
  hundred of the hotel's. — `twine::tests::the_same_corridor_holds_sixteen`
- Each gap is about 1.618 times the one before, which is the thing you feel
  rather than read. — `twine::tests::every_gap_is_the_golden_ratio_of_the_last`
- A door here is a door there: same size, same plaque, same light, same
  side. — `twine::tests::nothing_about_a_door_says_which_corridor_it_is_in`
- You can walk to door 11 and not to door 16, which is the room working. —
  `twine::tests::you_can_reach_eleven_and_not_sixteen`
- It leaves the lobby and nothing of it is in another
  room. — `twine::tests::nothing_of_it_is_in_another_room`
- Every lamp sits in a fitting, and the lamps thin out the way the doors
  do. — `twine::tests::every_lamp_is_in_a_fitting_and_they_thin_out`
- The hall says what the same walk buys next door, and the gap between the
  two numbers widens. — `twine::tests::it_says_what_the_same_walk_buys_next_door`
- And it says it while you walk, without being pointed
  at. — `tests::the_long_hall_counts_while_you_walk`

### Verified by hand

- Standing at the mouth, the two corridors read as the same corridor laid out
  twice, rather than as two unrelated halls.
- Giving up is a thing you notice yourself doing.

## Out of scope

**Anybody moving.** The desk's moves are the hotel's and this corridor takes
no arrivals. Spec 0033's people stay next door.

**Saying what phi is.** Not here.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
