# 0047 A door that turns you over

**Status:** implemented
**Date:** 2026-11-01
**Built:** 2026-11-01, the same day it was written. Two things it had to
take on that are not in the goal are named below: the leaf had to stop
hanging in the aperture, and the arrival had to step past the threshold.

## Goal

Make the trapdoor in the imaginarium's floor a way you can take. Step
through an open one and the room turns you over: you come out standing on
what was the west wall, with frame one's floor under you and your own floor
now a wall.

## Why

Spec 0038 gave the room a decision and it does not land. Jake, twice: "way
through and trapdoor don't seem to do much beyond moving a panel", and then
"it doesn't seem like it opens a way to anything".

The mechanism is not missing. Measured over a long sweep, with a door open
somebody from frame one is standing on your floor seventy per cent of the
time and with it shut, never. What is missing is you. The transit through
the aperture is 0.7 metres of a 20.3 metre round, under a second, in the
corner against the west wall, and the moment somebody is through it they
are upright like you and indistinguishable from your own frame's people.

**Every other door in this building opens a way for you.** Spec 0007's
bookcase opens the stair down. Spec 0008's leaf opens the baths. You pull
it and you walk through it. This one opens a way for somebody else, so the
honest account of your part is that you pressed enter and a panel moved.

And the room is called three ways to be the right way up. The player has
had one since the building began.

## The settlement this overturns

Down is `-y` everywhere in this arcade. `walk::walk` is written in it: the
body is a sphere at `at + Y * radius`, the ground is `at.y`, a step is
measured up and the fall is along `NEG_Y`. Nothing above suggests changing
that, and this spec does not.

## Behavior

### You carry a frame rather than an up

Not a vector. The imaginarium's three frames are the room's own, a cyclic
permutation of the axes about its long diagonal, and because every one of
them is a right angle **a box turns into a box exactly**. `box_in_frame`
already relies on this and the inverse is the same two corners the other
way. That is the whole reason this is affordable: nothing has to be refitted
and no bounding box grows.

Outside the imaginarium your frame is nought and every turn in here is the
identity, so the rest of the building walks the same arithmetic it always
did. That is a thing to test rather than to assert.

### Walking happens in your own terms

The solid boxes are turned into your frame, `walk` runs on them unchanged,
and the answer is turned back. Your yaw and pitch are in your frame too, so
the heading you steer with is the one you see, and the facing the world gets
is yours turned out.

The camera's up is your frame's. It already takes one: the tesseract turns
it, which is the precedent this follows.

### Going through is walking through

No key. The second doors are already worked with enter and that is how they
open; this is what you do with one once it is open. Walk into the aperture
and your frame becomes the one whose floor that door is cut in. You are a
metre off that floor when it happens, so what follows is a fall of about a
metre onto what was a wall, which is the room doing the explaining.

It goes both ways, by the same rule: an open door is an aperture and whoever
is in it changes frames.

A shut door is a floor. You stand on it, and nothing happens.

### The leaf slides out of the way rather than under it

Spec 0038 slid a leaf back out of its own doorway, which is how a door in
a wall swings clear and which in a door cut in a floor means forty
centimetres straight down. Open was a kerb with a panel sitting in a tray
under it, and what Jake read off the screen was a panel shifting back and
forth, which is what it was.

It mattered less while nobody could use the thing. Now it is a shelf
hanging in the middle of the way through, and what you do with it is land
on it. It slides sideways into the slab beside the opening instead, which
is what a floor hatch does and which leaves the aperture empty.

### You arrive past the threshold

All three second doors are in one corner of the room, which is the corner
where all three floors meet, so coming through one puts you on the join of
two of them. A body is 0.45 of radius and the floor you left is right
there: `move_and_slide` starts inside something, answers with no distance
at all, and you stand wedged in the jamb unable to walk in any direction.
Found by walking it rather than by reading it.

So an arrival has a place, `aloft::arrive_through`, a stride into the room
on the floor you have come out onto. A doorway has a threshold and you
arrive past it.

### You face the way you went

Carrying your old heading through was the first build and it is a mess: a
hundred and twenty degrees of turn puts whatever you were looking at into a
surface, so you arrive with your nose against a wall and no horizon
anywhere. You walked through a door, so you come out looking the way you
went, which is into the room, and what is in front of you is the whole of
it with your own floor standing up in the middle.

### The turn is eased, and only in the view

Gravity and walking change the moment you are through, because a frame that
eases is a frame that is wrong for the duration. The camera's up slerps to
its new one over `TURNS_YOU`, so what you see is the room rolling under you
rather than snapping.

## Acceptance criteria

- Walked into an open second door, you come out standing in the frame whose
  floor it is cut in. — `tests::an_open_door_turns_you_over`
- **Its control:** walked into the same door shut, you stay in your own
  frame and on your own floor. — `tests::a_shut_door_is_a_floor`
- The same aperture works from the other side: walked at from frame one's
  floor it puts you back on frame
  nought's. — `tests::the_same_aperture_works_both_ways`
- With your frame at nought, walking down the hall leaves you in frame
  nought, on the hall's floor, having covered the
  ground. — `tests::the_building_walks_as_it_did`
- A box turned into a frame and back is the box it started as, and the
  same size rather than a bounding box of the
  turn. — `aloft::tests::a_box_turned_into_a_frame_comes_back`

### Verified by hand

- Open the trapdoor, walk into it, and watch the room roll.

## Out of scope

**The other two frames from the ground.** You reach frame one through the
door in your own floor. The other two doors are cut in frames one and two's
floors, so they are how you get between those once you are there, and
nothing promises a route to all three from where you come in.

**Anywhere else in the building.** Your frame is nought everywhere outside
this room and nothing else cuts a door in a floor.

**Finding the way back.** The aperture works from both sides and nothing
makes it easy to find from the one you land on: you come out on a floor
that was a wall, and the doorway is a metre to one side of where you are
looking. Walked into, it works. Walked near, you grind along the wall
beside it, which is what a person does with a door they cannot see.

**Telling you that you can walk through it.** The readout still offers
only to open and shut. The words for a way you can take are Jake's.

**The map, and the key that says where you are.** Both report a position and
a heading and neither knows about a frame. They will be wrong in here and
that is a smaller thing than this.

**Carrying it out of the room.** Leaving by the way up puts you back in
frame nought. Walking out of an imaginarium sideways is not a thing this
building needs to answer.
