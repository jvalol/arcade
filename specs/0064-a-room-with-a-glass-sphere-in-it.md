# 0064 A room with a glass sphere in it

**Status:** implemented
**Date:** 2026-11-11, built 2026-11-12

## Goal

A room over the nook, reached by stairs from the nook's far wall, with one
object in it: a glass sphere traced against the building's own geometry.
What it demonstrates is refraction, which is the one thing this engine can
show that the hall of mirrors cannot.

## Why this room

**Jake, 2026-11-11**, on the two blank walls the walk found: it would be
more interesting to put the stairs or another room there rather than tiling.
Then, on what the other side should be: open to ideas, take inspiration from
what I've already built, maybe a demonstration of ray tracing, something
impressive for a graphics engine.

**It has to pass spec 0063's test.** Every room that works here puts its
idea in the world, where you meet it by doing something ordinary. The hotel
and the imaginarium put the idea in your head first and then showed you a
room, and both are parked. A glass ball is a thing everybody has already
looked through.

**The corridor of mirrors is the control and not a rival.** Spec 0061's two
walls are a render to texture from a fixed plane, so they can only show what
a second camera could see. A sphere shows the parts of the room nothing is
pointed at, and upside down.

## Why the building is already a ray tracer's scene

Every room here is a list of axis-aligned boxes, because that is what `walk`
needs and what every room has been built out of since spec 0001. A ray
against one is six divides and two compares. Measured 2026-11-11:

| | |
| --- | --- |
| solid boxes and lids in the whole building | 635 |
| within a nook's reach of the nook | 168 |
| walls alone | 16 |

So the scene a tracer wants is the list the building already keeps. No mesh
conversion and no acceleration structure at this size.

## The room is long, and the sphere stands in the middle of it

**Both of those are the spike's, and both were found by failing.** The
record is at the end of this spec. The short version is the whole design:

**A sphere squeezes a wide cone of the world into a disc.** So what it needs
behind it is not objects but depth and variety with angle. A quiet room with
one thing in it, which is what this spec first proposed, gives a flat disc
on a flat wall and the object disappears.

So the room is fifteen metres over the nook with frames down it at the
corridor's own `mirrors::PITCH` of 1.80, which is a colonnade. The arcade
already owns the proof that this is what works: the hall's aisle is sixteen
metres of repeating cabinets, and the sphere reads there and nowhere else.

**And the plinth is in the middle of the room, not against its end wall.** A
sphere shows what is behind it, so a plinth at the end puts the room's length
behind your eye instead. In the middle there is length behind it whichever
end you come from, and the walk is the same from either.

**Two bounces into the glass and one out**, which is what it takes to read
as solid glass rather than as a bubble.

**What it traces is the real building** and not a cube map of it. A cube map
is six rasterised faces and shows what a second camera could see, which is
the limit this room exists to break.

## What you do in it

**Walk its length and round it.** That is the whole verb, and it is the
arcade's: the tesseract is a thing on a plinth that turns you, the Mandelbrot
is a thing you zoom with your hands, this is a thing you walk at.

**No dial.** An index of refraction on the arrow keys is the first thing
anybody would reach for and it is the mistake aleph is parked for: the hall
was the good part and the dials were a menu with extra steps.

## The stair, and what changed about it

Spec 0037 has a section dated 2026-11-10 called "the stair at the far end,
and why it does not fit". It measured the chain and found 0.66 missing:

| | |
| --- | --- |
| nook floor | z -9.50 to 7.00 |
| nook end wall, inner face | z 6.85 |
| a reversed foot wants | z 7.51 |
| garden's doorway | z 7.55 to 9.25 |

**Every number in that chain assumed a fixed landing**, hard against the
imaginarium's north wall. Spec 0063 parked that room, so the flight is free:
the rise, the going and the step count can be chosen for the nook's north end
rather than reversed out of somewhere else.

**The routing is not settled.** Only 0.40 separates the far wall's inner face
from the garden's way, so a flight cannot simply run north behind it. It
turns and climbs back south over the nook, with this room above it. That is
geometry to work out at build time, and the measurements above are what it
has to satisfy.

## What the engine needs, which is its own spec

Blitzkit's spec 0098, a ray against a list of boxes. The box list in a
storage buffer and a shader that reads it are the engine's rather than the
arcade's, and this should not be built before that exists.

## The budget, said before building rather than after

**2026-11-11 is why this paragraph is here.** The hall of mirrors was two
full renders of the world every frame from everywhere in the building, and
from the top of the bridge a frame was 139 ms. Nobody measured it until Jake
said the framerate drops.

So this room carries a number from its first commit, measured with
`Renderer::gpu_passes`, which already reports per pass and is what found the
mirrors.

## What it came to, 2026-11-12

Built. What follows is what the build changed about the spec above, and what
it cost.

**The flight runs across the nook and not along it.** Spec 0037 measured the
chain for a flight running north, found 0.40 between the end wall and the
garden's way, and this spec inherited that and said it turns and climbs back
south. Climbing back south costs the room 4.2 metres of its own length for
the going, and the length is the whole point. The nook is ten metres wide
and sixteen and a half long, so crossing it costs the width of the flight,
which is 1.8, and nothing has to turn. The room is fourteen metres.

**The foot is read off the bench row.** The first one stood against the east
wall, which is where spec 0006's row is: four treads stood on the kociemba
and the cascada, and the walk said so by reaching neither of them.

**The colonnade stands free in the room**, a stride in from each wall, and
that took three goes. Panels on the wall 0.12 deep read as one slab with
lines on it. Piers a third of a metre proud read in the room and not in the
ball. Painting every traced box a colour of its own showed why: the sphere
was finding five surfaces, the floor, the ceiling and the three walls,
because that is all a room with flat walls has. The spike said it first and
this spec quotes it: what works is the hall's aisle, two rows of things
standing in a room with floor behind, under and between them.

**The room is dark and the bays are lit.** The first build used this
building's own floor, wall and ceiling, which are a pale room, and
everything in the glass was within a shade of everything else. A sphere
squeezes a room into a disc, so it wants a few bright things on a dark
ground.

**And the glass was handed a brightness.** Spec 0098's shader shades what a
ray meets with the ambient and the sun, and this room is indoors where the
sun does not reach and the light comes from lamps. See `glass::GLOW`. The
honest fix is point lights in the tracer, which is engine work and its own
spec.

### The lens was twice the size it thought it was

**An engine fault, found by building on it.** `trace.wgsl` took the lens's
radius as `length(model_0)` on the argument that a unit sphere scaled has
one column's length for its radius. `MeshData::sphere` is a unit *diameter*:
every vertex is half a unit out. So every lens ever drawn thought itself
twice its real size, worked out an exit point that was not on the glass, and
returned two flat regions instead of a room.

The engine's own picture was blessed by eye and a wrong bend looks like a
bend. It moved by 0.7% of its pixels when this was fixed, which is why
nobody caught it there. Blitzkit's spec 0098 carries the correction.

### What it shows the field, 2026-11-12

**It was a blank box on the roof**, and Jake said so by looking at a picture
of it: four faces with nothing on any of them, the tallest thing on the
building, which from the field reads as a plant room. From spec 0057's own
pose on the bridge it is only an outline at a hundred and thirty metres and
nobody would pick it out, but the field is where you meet it.

It has a cornice now, which is what every other roofline here has, and a slot
of glazing on the outside of every bay, lined up with the lamp that is
already in there. By day the slots break the slab into the same rhythm the
colonnade has inside it, and by night the room's own light comes out of the
building, which nothing else in this building does.

Two things that took finding:

**A bay written as a flat colour is five times brighter by day than by
night.** A colour here multiplies the light, and the one light the engine
hands a scene is the world's. So from the field at night the glazing was all
but out, which is the opposite of what a lit room looks like from outside.
`glass::lit` divides the fill back out, the same answer `aim::MOON`, the
runestone and Valhalla's hearth all reached.

**And the building's outside skin does not know about openings.**
`Room::outside` samples columns and asks what is over each one, so it knows
walls and plasters straight over a hole in one. The slots are real: nothing
solid stands in them, you can see out, and the sphere traces through them.
The glazing is drawn a little proud of the skin rather than inside the
opening, because inside the opening is behind the plaster, and that is how
this was built first and why the elevation came back blank twice. Teaching
`Room::outside` to cut round an opening is the real fix, spec 0038's ways
out would want it too, and it is its own piece of work.

### The budget

Measured 2026-11-12 at 1200 by 948, wall clock over twenty frames, because
this machine's adapter returns nothing for `gpu_passes` offscreen and a
number that is zero is not a number.

| | |
| --- | --- |
| down the room, the ball in shot | 8.5 ms |
| up close at the glass | 7.9 ms |
| turned away from it, in the same room | 1.7 ms |
| the hall, none of it in shot | 2.0 ms |

So the glass is about six and a half milliseconds and nothing at all when it
is not being looked at, against the hall of mirrors' 139. The gate is the
same three questions `mirrors::draw` asks, with a fourth reason of its own:
the lens pass traces every box it is given for every pixel of the sphere.

**And the imaginarium's site is gone.** Spec 0063 parked that room and kept
it tested so it would not rot; its floor is at 6.0 and this room's lid is at
7.0, and 3.5 to 5.7 is a crawl space. A parked room cannot hold a storey
empty against one being built, so the space is this one's. Its test says so
by name now, and bringing it back means finding it a new height or a new
place first.

## Acceptance criteria

- The sphere traces the room's own boxes, and the list it is given is `solid` plus `lids` rather than a second account of the building. — `glass::tests::it_traces_the_building_itself`
- A ray that leaves the sphere and hits nothing comes back as the room's ambient rather than as black. — `glass::tests::a_ray_that_hits_nothing_is_not_a_hole`
- What is behind the eye is in the sphere, which is the claim this room is for. — `glass::tests::the_room_behind_you_is_in_it`
- The plinth stands in the middle of the room's length, within a tread of it, and not against either end. — `glass::tests::the_sphere_has_length_behind_it_either_way`
- The frames run the room's length at the corridor's own pitch, so the cone finds something different at every angle. — `glass::tests::the_room_is_a_colonnade`
- You can walk its length and all the way round the plinth with the passage this project has settled on. — `glass::tests::you_can_walk_round_it`
- The stair's foot is inside the nook's end wall and its head is clear of the garden's way. — `glass::tests::the_stair_fits_the_nook`
- The room does not stand over anything that is already there. — `glass::tests::nothing_of_the_building_is_under_it`
- And the room over the nook is drawn. — `tests::the_glass_room_is_drawn`

### Verified by hand

- The sphere reads as glass rather than as a bubble or a marble.
- The room is recognisable in it, inverted, from either end of the walk.
- A frame with the sphere in shot against a frame without it, from the same spot.

## Out of scope

**Bringing the imaginarium back.** The stair goes somewhere new. Spec 0063
parked that room for reasons that have not changed.

**Ray tracing the room itself.** The walls, the floor and the frames are
rasterised as they are now. What is traced is what you see through the glass.

**Caustics.** The light the sphere would throw is a second piece of work and
a harder one, and the room is a room without it.

**A second sphere.** Two is a comparison and a comparison is an argument.

**Opening the nook's ceiling.** Tried in the spike on 2026-11-11 and it is a
good room, but it is a room rather than a demonstration and it has nothing
to do with this one. Its own spec if it is wanted.

## 2026-11-11: the spike, and the three things it was wrong about

`capriccio::tests::zz_spike` traces the arcade's own boxes on one CPU core
and writes pictures. It answered blitzkit's spec 0098's question first, which
was whether a box-only arcade reads as the arcade, and it does. Then it was
pointed at this room, and the first version of this spec did not survive it.

**Wrong about where.** The sphere was to go in a quiet room at the top of a
stair. Four placements, in the order they were tried:

| where | what the sphere came back as |
| --- | --- |
| the hall's aisle | the aisle inverted, both rows of cabinets round the ring |
| a sealed room over the nook | a flat disc on a flat wall |
| the same with its floor opened to the nook | a flat disc on a flat wall |
| a gallery's rail over an opened nook | the nook in the top third, the underside of the gallery floor in the rest |

**Wrong about why the hall worked.** Not because it is the hall. Because it
is sixteen metres of aisle with two rows of cabinets running to a vanishing
point, and a sphere wants depth and variety with angle rather than objects.
A rail is the worst case of all: half the cone is the room and half is the
underside of the floor the plinth stands on.

**Wrong about the middle.** Given a long room with frames down it, the first
plinth went against the end wall and came back as a flat disc in a corner,
because that puts the room's length behind the eye rather than behind the
sphere.

**And then it worked.** Fifteen metres, frames at 1.80, plinth in the middle.
From the door the room is a colonnade with something small and bright at its
centre; at it, the room is inside the glass, with the frames wrapped round
the ring and the floor and ceiling swapped.

### What the spike does not answer

Anything about the renderer. The storage buffer, the shader, the bind group
and what three bounces cost at a quarter of the screen are untried, and those
are the work.
