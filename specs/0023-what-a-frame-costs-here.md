# 0023 What a frame costs here

**Status:** implemented
**Built:** 2026-10-17
**Date:** 2026-10-17

## Goal

Know what this building costs to draw, room by room, while standing in it.

The engine learned to time a frame for its own `load` example, which is a wall
of cubes with dials on it and exists because there was nothing else to
measure. This is the thing worth measuring. It is the largest program anybody
has built on this engine: eleven rooms, a sky, two pools, shadow maps, a
system of planets and eight cells that are one cell. The number moves as you
walk from one room to the next, and until now nothing could see it move.

## Behavior

F puts a readout in the bottom left corner and F again takes it away. It says:

- the work this frame cost, as the middle of a window of recent frames, with
  its 95th percentile and its worst
- the time between frames, which is the display's number rather than the
  engine's
- what the GPU said its own frame took, where the device will say
- how many draw calls went in, and how many triangles

All of it comes from `blitzkit::cost` and the renderer's own counters, per
spec 0056 of the engine. Nothing here works out a second account of any of it.

**Work rather than the time between frames.** Spec 0056 has this at length:
on this machine a window is redrawn when the display says so, so the gap
between frames is 8.33 milliseconds in every room that fits inside a frame,
which is all of them. It tells you the display is 120 Hz. The work is what
tells you the hall is dearer than the cellar.

**The window fills whether or not it is being looked at.** A readout that
starts measuring when the key is pressed spends its first seconds reporting
the cost of pressing the key. The window is a few seconds long, so it empties
itself of wherever you were a moment ago without being told to.

**And the device is asked to time a frame whether or not the readout is up.**
That feature is asked for once, before the device exists, and a key pressed
later cannot go back and ask again. It costs two timestamps and a sixteen
byte read a frame against a building that draws several hundred things with
shadows.

**Not in the help line.** O and P are not in it either. The line in the corner
is for somebody walking round the building, and these three are for whoever
is building it.

## Acceptance criteria

- F shows the readout and F puts it away, and the window of frames it reads
  from is untouched either way. — `tests::f_toggles_what_a_frame_costs`
- The frames are counted at the top of every frame rather than only while the
  readout is up. — `tests::the_frames_are_counted_before_anybody_asks`

### Verified by hand

- Standing in the hall with the cabinets lit: 1.60 ms of work, 2.49 on the
  GPU, 522 draws and 2,493,684 triangles, with 8.33 between frames.
- The number moves between rooms. If it ever does not, the readout is broken
  rather than the building being uniform.

## Out of scope

**Anything that makes it faster.** This says what it costs. A change that
comes out of it is its own spec, and doing both at once is how a measurement
ends up flattering the change.

**A budget, or a warning when a room is dear.** There is no threshold worth
failing on yet, which spec 0056 found out the hard way: the same sweep run
twice twenty minutes apart differed by a factor of two, because the second run
shared the machine with two browsers.

**A room to put it in.** Jake, on where an instrument belongs in a building
whose cabinets are games: maybe like a tech room, or a maintenance closet.
That is a room and this is a key, and the key is the part that is useful
today.
