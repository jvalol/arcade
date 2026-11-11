# 0064 A room with a glass sphere in it

**Status:** draft
**Date:** 2026-11-11

## Goal

A room at the top of a stair from the nook's far end, with one object in it: a
glass sphere, traced against the building's own geometry. What it demonstrates
is refraction, which is the one thing this engine can show that the hall of
mirrors cannot.

## Why this room

**Jake, 2026-11-11**, after agreeing the two blank walls want something built
rather than tiled: up, open to ideas on what the other side is, take
inspiration from what I've already built, maybe a demonstration of ray
tracing, something impressive for a graphics engine.

**It has to pass spec 0063's test**, which is the one this building has just
spent a morning on. Every room that works here puts its idea in the world,
where you meet it by doing something ordinary. The hotel and the imaginarium
put the idea in your head first and then showed you a room, and both are
parked. A glass ball is a thing everybody has already looked through, and
what it does is visible before anybody explains anything.

**The corridor of mirrors is the control and not a rival.** Spec 0061's two
walls are a render to texture from a fixed plane, so they can only ever show
what a second camera could see. That is the whole limit of the technique and
it is invisible while the only thing in front of a mirror is the room it is
in. A sphere shows the parts of the room nothing in it is pointed at, and
upside down. Downstairs is the trick, upstairs is the thing itself.

## Why the building is already a ray tracer's scene

This is the argument for doing it here rather than anywhere else.

Every room in this arcade is a list of axis-aligned boxes, because that is
what `walk` needs and what every room has been built out of since spec 0001.
A ray against an axis-aligned box is six divides and two compares. Measured
2026-11-11:

| | |
| --- | --- |
| solid boxes in the whole building | 631 |
| within a nook's reach of the nook | 168 |
| walls alone | 16 |
| lids | 4 |

So the scene description a tracer wants is the one `room::solid` already
returns. No mesh conversion, no bounding volume hierarchy, no acceleration
structure of any kind at this size. Almost no engine can say that about
itself, and it is true here by accident of how the building was built.

## What goes in the room

**One sphere, on a plinth, about as wide as a body and at waist height.** You
come up the stair and it is there. Through it the arcade is inverted and
reversed, and the things behind you are in it.

**Two bounces into the glass and one out**, which is what it takes for a
sphere to read as solid glass rather than as a bubble. Each bounce is another
pass over the box list and the budget below is what decides whether the third
survives.

**What it traces is the real building** and not a cube map of it. A cube map
is six rasterised faces and would show the same thing a second camera can
see, which is the limit this room exists to break. The boxes are right there.

## What you do in it

**Walk round it.** That is the whole verb, and it is the arcade's: the
tesseract is a thing on a plinth that turns you, the Mandelbrot is a thing
you zoom with your hands, this is a thing you walk round.

**No dial.** An index of refraction on the arrow keys is the first thing
anybody would reach for and it is the mistake aleph is parked for: the hall
was the good part and the dials were a menu with extra steps. If the sphere
needs a control to be interesting then the sphere is not interesting.

## The stair, and what changed about it

Spec 0037 has a section dated 2026-11-10 called "the stair at the far end,
and why it does not fit". It measured the chain and found 0.66 missing:

| | |
| --- | --- |
| nook floor | z -9.50 to 7.00 |
| nook end wall, inner face | z 6.85 |
| a reversed foot wants | z 7.51 |
| garden's doorway | z 7.55 to 9.25 |

Growing the nook covers 0.40 of the 0.66 before its end wall stands in the
way into the garden, and growing the hall is symmetric: tried at `END` 4.0 to
4.7, sixteen tests fell, six of them rules in six different rooms.

**Every number in that chain assumed a fixed landing.** The flight had to
reverse and keep its head where it already was, because its head was hard
against the imaginarium's north wall. Spec 0063 parked that room. There is no
landing to hit any more, so the flight is free: a run of 4.76 with its foot
at the wall's inner face of 6.85 puts the head at 2.09, which is inside the
nook with room to spare. The rise, the going and the step count can all be
chosen for the nook's north end rather than reversed out of somewhere else.

**So the stair is not the hard part, and the measurements that said it was
are still correct about the thing they measured.**

## What the engine needs, which is its own spec

The arcade cannot do this alone. Tracing needs the box list in a storage
buffer and a shader that reads it, and both of those are blitzkit's rather
than capriccio's. That is an engine spec in the engine's own numbering, and
this one should not be built until it exists.

The split is the usual one. The engine learns to trace a list of boxes and
shade what it hits. The arcade decides there is a sphere, where it stands,
and what is behind it.

## The budget, said before building rather than after

**2026-11-11 is why this paragraph is here.** The hall of mirrors was two
full renders of the world every frame from everywhere in the building, and
from the top of the bridge a frame was 139 ms. Nobody measured it until Jake
said the framerate drops.

So this room carries a number from its first commit. The sphere covers a
fraction of the screen rather than the whole of it twice, which bounds the
cost in a way the mirrors were not bounded, but each bounce is a full pass
over the box list and three bounces at a quarter of the screen is not free.
Measured with `Renderer::gpu_passes`, which already reports per pass and is
what found the mirrors.

## Acceptance criteria

Named here so the build has something to answer to. None of these exist yet.

- The sphere traces the room's own boxes, and the list it is given is the one `room::solid` returns rather than a second account of the building. — `glass::tests::it_traces_the_building_itself`
- A ray that leaves the sphere and hits nothing comes back as the room's ambient rather than as black. — `glass::tests::a_ray_that_hits_nothing_is_not_a_hole`
- What is behind the eye is in the sphere, which is the claim this room is for. — `glass::tests::the_room_behind_you_is_in_it`
- You can walk all the way round it with the passage this project has settled on. — `glass::tests::you_can_walk_round_it`
- The stair's foot is inside the nook's end wall and its head is clear of the carpet. — `glass::tests::the_stair_fits_the_nook`
- The room above does not stand over anything that is already there. — `room::tests::nothing_of_the_building_is_under_it`

### Verified by hand

- The sphere reads as glass rather than as a bubble or a marble. Walk round it.
- The arcade is recognisable in it, inverted. Stand where the fire is behind you.
- A frame with the sphere in shot against a frame without it, from the same spot.

## Out of scope

**Bringing the imaginarium back.** The stair is at the nook's far end and
goes somewhere new. Spec 0063 parked that room for reasons that have not
changed, and a stair is not a reason to unpark it.

**Ray tracing the room itself.** The walls, the floor and everything else are
rasterised as they are now. What is traced is what you see through the glass.

**Caustics.** The light the sphere would throw on the floor is a second piece
of work and a harder one, and the room is a room without it.

**A second sphere.** Two is a comparison and a comparison is an argument.
