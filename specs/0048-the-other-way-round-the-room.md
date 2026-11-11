# 0048 The other way round the room

**Status:** parked on 2026-11-10 by spec 0063, which took this room out of the building and kept every line of it compiling and tested under `src/parked/`. What is written below is the record of a room that was built and stood, and the tests it cites went out of the suite with it.
**Date:** 2026-11-01
**Built:** 2026-11-01, the same day it was written. Two things the filming
found and the tests could not are at the end of the behaviour below.

## Goal

Give the door at the top of every flight somewhere to go. Walk up your own
stairs and out of it, the room turns around you, and it sets you down at the
foot of another frame's flight.

Spec 0047 made the hatch in your floor a way through and left the three
doors at the tops of the flights exactly as they were, which is painted
scenery. Jake walked straight at one and asked why it would not open. Until
that morning neither kind of door went anywhere and they were at least
consistent; now one works and three identical-looking ones do not.

## What makes these different from 0047's

The room is the cube. Its six faces are each exactly one frame's floor or
one frame's ceiling.

The three second doors take the three floors. That is why 0047 was
straightforward: each of those apertures has a real far side with real
ground on it, and all the spec had to do was let you step across and change
which way was down.

The three top doors take the three ceilings. Measured:

```
frame 0's top door is cut in frame 1's ceiling
frame 1's top door is cut in frame 2's ceiling
frame 2's top door is cut in frame 0's ceiling
```

Nobody walks on their ceiling, and past the face is `DOOR_BACK`, eighteen
centimetres of dark, and then the outside of the room. **So there is no far
side to find and any destination is invented.** That is the whole of why
this is harder than 0047 and it is worth saying plainly rather than
pretending the geometry offers an answer.

## The move this borrows

This building already makes one crossing that is not a step across a gap.
Spec 0021's tesseract is eight cells that are the same cube drawn eight
ways, and you cross between them by walking at an opening while the world
turns around you: `crossing` to notice, `arrive` to place you, `Turn` to
carry it, `rolled` to roll the camera. The player has been taught that move
once already in this building.

**Not borrowed from 0047**, which swaps your frame in one frame of
animation because a frame half way between two is one nobody can stand in.
That is right for a step over a threshold of 0.7 metres and wrong for this,
where the two ends are a dozen metres apart and an instant swap is a blink
from one side of the room to the other.

## Behavior

### Which way it goes

The hatch in your floor takes you from frame `n` to frame `n + 1`, which is
`hatch_is`. **The top of your flight takes you to frame `n - 1`**, the other
way round the same three.

Jake's call, 2026-11-01, and the reason is that it makes the room somewhere
you can move about deliberately rather than a one way carousel. It also
settles something 0047 left open: you could get into frame one and be
stranded, because from the floor you land on, the way back is a doorway a
metre to one side of where you are looking, and walked near rather than at
it you grind along the wall. Every flight now ends in a way out.

Where you arrive has nothing to do with which frame's ceiling the door
happens to be cut in. There is no far side, so the destination is chosen,
and choosing the one that makes the room traversable is better than
choosing the one that looks like it follows from the geometry when it does
not.

### Where you come out

At the foot of the destination frame's flight, on their floor, a stride
clear of the first tread and facing up it. You left at the top of a
staircase and you arrive at the bottom of one, which is the shape of the
thing.

### What the crossing is

Over `TURNS_ROUND`, your position eases from the doorway to the arrival and
your frame turns with it. Walking is suspended for the duration, the way
the tesseract suspends it: there is nothing to collide with on a path that
is not in the room.

The camera's up comes from the turning frame rather than chasing it, so the
roll is the scripted one and not 0047's lag.

### The path bows through the middle, and you look along it

Both of these came off a render and neither is in the idea.

A straight line from a door high on one face to the foot of a flight on
another goes through whatever is in the way, and half the crossing was
spent inside a flight with the camera against grey. The middle of this
cube is the one part of it that is open by construction, because all three
flights run along the faces, so the path is a curve through the middle.

And you trigger this standing in the doorway with your nose against the
eighteen centimetres of dark behind it, so the first third was a grey
panel at arm's length however the path ran. You turn at the off to look
where the crossing is taking you, which is into the room.

### Your place, your up and your heading are on one clock

**And this is the thing that made both portals hard to read.** Jake said
he could not tell what was happening walking through one, so rather than
watch it again, a harness drives a crossing, renders every fourth frame,
and asks four questions of each pair: how far the eye moved, how far the
heading turned, how far the up rolled, and how much of the picture changed.

What came back was one fault at both. The place and the up moved smoothly
and the heading snapped. The hatch turned ninety degrees in a single
frame, taking ninety three per cent of the picture with it, and then spent
the next second and a half righting a horizon that had already arrived.
The top door did a hundred and fifty degrees at each end with a clean
sweep in between. **The middle of a crossing was never the problem**,
which is not what either of us would have guessed from watching one.

So the heading is carried by the crossing like everything else, and spec
0047's hatch becomes a crossing too rather than a swap in one frame with
the camera rolling after it. Measured again afterwards, the hatch's worst
sample turns 14 degrees where it turned 90, and nothing in either crossing
is flagged.

### A top door needs no working

The second doors have leaves and a state. These have neither and never did.
An aperture with nothing in it is open, so walking into one is the whole of
the interaction.

## Acceptance criteria

- Walked up its own flight and into the door at the top, you come out in
  the frame one back round the three. — `tests::the_top_of_a_flight_goes_the_other_way`
- **Its control:** the hatch in your floor still goes the other
  way. — `tests::the_hatch_and_the_top_door_go_opposite_ways`
- You arrive at the foot of a flight, on that frame's floor, not inside
  anything. — `tests::you_come_out_at_the_foot_of_a_flight`
- The crossing takes `TURNS_ROUND` and moves you nowhere else while it
  runs. — `tests::a_crossing_cannot_be_walked_out_of`
- Three top doors, three destinations, each frame reachable from each
  other. — `parked::aloft::tests::the_two_ways_round_cover_all_three_frames`
- No crossing of this room turns, rolls or changes the picture faster than
  somebody can follow, measured frame against
  frame. — `tests::no_crossing_in_here_snaps`

### Verified by hand

- Walk up a flight and out of the top, and watch the room roll.

## Out of scope

**The walkers.** They still stop at the top door and go back. Giving the
figures the same crossing is a separate question and a bigger one, because
their paths are built once per round and this would make a path that
changes frame twice.

**Anything outside the cube.** The eighteen centimetres of dark behind each
door stays. Nothing is built out there and nothing is seen through it.

**Getting out of the imaginarium.** The way down to the nook is still the
dog-leg and still only reachable from frame nought.

## 2026-11-10: parked by spec 0063

**Jake, having played it: the code and logic are good, the composition in
capriccio does not land.** This room is out of the building and under
`src/parked/`, compiling and tested, until there is a way to make it feel
right for a user.

The reasons are in [0063](0063-the-rooms-that-ask-before-they-show.md). The
short of it is that this room puts its idea in your head first and then
shows you a room, where every room in this building that works puts its idea
in the world and lets you meet it by doing something ordinary.

Everything above is left as written. The arithmetic in it held up and still
runs.
