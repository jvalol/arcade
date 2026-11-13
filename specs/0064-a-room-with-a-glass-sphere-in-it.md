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

## Jake looked at it, 2026-11-12

*The stairs to it need a lot of work, and the room itself needs some
decoration to be more interesting.* Both are the first build of a room being
a blocking model, which is what the first build of a room here always is.

### The stair

It was fourteen bare slabs against the nook's stone, and that is all it was.
No string, no rail, no post, no nosing, and no light of its own. The open
side was a three and a half metre drop with nothing on it: you could walk
straight off the fourth step into the nook.

It has a closed string and a handrail now, each one box turned to the
stair's pitch, a turned baluster to every tread, and a newel at the foot and
at the head. The rail on the flight is lower than the rail round the well,
because a rail you hold going up is not a rail you lean on standing still,
and the newel at the head is what that step is for.

**The rake is named once.** What a body meets has to be upright boxes and
what a hand runs along wants to be at the stair's angle, so there are two
accounts of one balustrade. That is allowed here under this building's own
rule, because the two would not still agree if either changed: a collider is
axis aligned and a rail is not. So the pitch is written down in `rake`, taken
through the nosings rather than along the corners of the flight, and both are
set out from it.

**Through the nosings, which is not the same line.** The flight's own
diagonal runs floor to floor and the nosing line runs a riser higher at the
bottom and lands on the top tread. A rail set out from the first drifts a
quarter of a step away from the steps by the top.

### The lamp that was hanging in it

The nook's pendant grid had one in the middle of the flight, at the head
height of anybody climbing. That ceiling was solid when the grid was laid
out and this spec cut a well through it, so the lamp had been hanging from
nothing since the room was built.

It is dropped where the list is made, so its light goes with it. **The
flight and not the well**, which are two questions with similar answers: the
well is the whole north strip of the nook's ceiling and most of it is a void
you look up through, which still wants its lamps. Only the part with steps
under it is a place a head goes. Filtering on the well put the nook's north
end out and three other pictures with it.

The flight carries two sconces of its own instead, and their light goes into
`room::nook_lamps` rather than into a second list beside it, so the
measurement of what reaches that floor is taken over every lamp that reaches
it. That is what caught the next thing: with the pendant gone, the foot of
the stair was at 0.056 of a lamp against the 0.12 this building calls black.
The lower sconce came down the flight until it was not.

### The room

The columns were plain extrusions, the plinth was a cube and the floor was
one flat slab. Each column has a base and a capital with a bead necking it
to the shaft, the ceiling has a beam over every bay on the colonnade's own
pitch, the plinth has a base and a cap, and the floor has a panel laid on it
inside a border.

**Laid on and not cut in.** Two faces in one plane fight, and this building
has spent four sittings on that.

**All of it drawn and none of it walked.** A base is a hand's breadth of
overhang at ankle height and a capital is over your head. What a body meets
is the shaft, which is already in `solid` and already what the glass traces,
so adding the trim to the walk would be a ring of ledges round every column
for nothing.

### What is still a blocking model

The stone on the nook's wall beside the flight is one texture stretched over
the whole of it: soft scallops a metre across and texels you can count. That
is this project's own named fault and it belongs to the nook rather than to
this room.

### He walked it, and the first build of the balustrade was wrong three ways

Two pictures, 2026-11-12: the head of the stair, and the ball on its plinth.

**The rail and the string were set out from the rake.** The rake's own two
ends are the bottom and the top nosing, and neither of those is under a
newel: the foot's post stands a nosing further east and the head's a whole
going further west. So the boards overshot at the bottom and fell a fifth of
a metre short at the top, which left the last two balusters hanging under
nothing. They run newel centre to newel centre now, which is where joinery
meets a post.

**The post at the head ran from the nook's floor.** It stands just inside the
well's edge, where this room's own floor reaches, so the slab was threaded
through a five metre post at mid height. It stands on the floor it arrives at.

**And the string was dropped by half its depth.** A board at the stair's
pitch is turned, so two lines half a depth apart square to the board are half
a depth *divided by* the cosine of that pitch apart measured straight up.
Dropped by a plain half depth its top edge came five centimetres over the
line the balusters stand on and ate their feet; multiplying by the cosine,
which was the first guess at the fix, buried them twice as deep. A red string
is what found it, after two passes of reasoning about it got the direction
wrong both times.

### The blocky patch on the plinth was not a shadow

It read as one: a quantised grey staircase on the top of the plinth, exactly
where the ball meets it, which is what a point lamp's shadow map looks like
at the edge of its resolution.

It was the cap moulding and the plinth's own box, both topping out at the
same height and both drawn, fighting for the same pixels. The plinth is drawn
as a base, a die and a cap now, with the die stopping short of the cap and
the cap covering it. The box in `plinth` is still what a body walks into and
what the ball stands on, and it is no longer what is drawn.

**The engine's own coplanar check cannot answer this one, and it was asked.**
Over the whole building, the way `behind.rs` says to ask it, it passed the
broken version: that check throws out a face buried in another solid, and the
die's top is inside the cap it fights. The rule is right for what you can see
past and wrong for two things that are both drawn.

**Handed a fragment it reports nothing at all**, whatever is wrong with the
fragment. `behind.rs` wrote that down after losing an afternoon to it and the
first version of this test was written into it anyway, so its own control
passed. Two tests in one sitting passed their controls before either was
worth anything.

### A linear way out, 2026-11-12

Jake, having walked it: *the stair needs a lot of work. I think it should be
a linear way out, not a left turn. I fall through the floor when I try to
walk to the stairs.*

**The fall was the gap in the rail.** It was 1.3 wide, sized on the note that
"what has to pass through it is a body and not a step", and a body is 0.9.
Only the first 0.3 of that gap had the top tread under it; the rest was open
over treads a riser, two risers and three risers down. Walking at the stair
anywhere but dead on the top tread dropped you three quarters of a metre.
The question was never how wide a body is. It is what is under the gap.

**The flight runs along the room now**, on the plinth's own axis, descending
north. You walk at the sphere, past it, and straight on down. The spec's
first reason for crossing the room was that climbing back south costs 4.2
metres of the room's length and the length is the point of the room; that
reason held only because the well was taken at the full width. Cut as a slot
the width of the flight it costs the room nothing: the floor wraps round
three sides of it, the colonnade stands well outside it and runs the room's
whole length, and the only edges up here are the slot's own two. The
colonnade is longer than it was.

**The top tread is level with the floor it leaves.** Fourteen risers of 0.25
is 3.5, which is this floor, so there is no step at the head of the flight
at all and nothing to walk off.

**Two balustrades, because a flight down the middle of a room is open on
both sides.** That is double the joinery of one against a wall, and the
alternative was putting the flight against a wall, which buys one balustrade
and costs the turn Jake asked to have taken out.

The joinery stands in a gap of its own and is thinner than the gap. Filling
it exactly put the string's inner face on the treads' edge and its outer
face on the floor slab's cut edge, which is two faces in one plane twice
over: `aloft`'s check named a patch three and a half metres square. Two
centimetres of air each side is not a gap a body can find.

**The steps are drawn a step thick and walked as slabs.** A tread runs to the
floor because spec 0007 asks what is in front of your foot straight down, so
a tread with daylight under it is a tread you fall through. Running along
the room that makes the flight a solid wedge 3.5 high whose south end faces
the nook, and the room below got a blank pale slab in the middle of it with
the way up hidden behind. Drawn a step thick the underside is a stepped
rake, which is what a stair looks like from below, and consecutive boards
overlap by five centimetres so there is no daylight through it either.

**The lamps moved onto the newels**, because there is no wall beside this
flight any more. One at the head and one at the foot, on opposite sides, so
the light crosses the steps rather than running down them.

**It takes the space the imaginarium's own flight wanted.** `aloft` is
parked and its second flight is in the nook's air where this one now runs,
so its coplanar check found a tread of each sharing a plane. The two are
never both drawn, so that is not a fault; the conflict between the rooms is
real and is recorded where it belongs, in the test that names what took the
storey.

### Standing at the head of it, 2026-11-12

Jake: *why is there a light on the right and not on the left, it looks like
the ceiling from below is in the way of the stairs.*

**The ceiling was in the way, and the clipping for it was already written.**
`Room::ceiling` was given the well the day this room was built, so the slabs
had the hole; the bars and tiles that hang under them were given an empty
list, under a note from spec 0063 saying the clipping stays and whoever
brings a stair back will want it. Spec 0064 brought one back and nobody
handed it over. Every bar and tile ran straight across the stairwell. One
line.

**And a lamp on every newel, which is four.** The first pair went one at the
head and one at the foot on opposite sides, reasoning that the light would
cross the steps rather than run down them. Standing at the head that is a
lamp on one post and a bare post facing it. A pair of newels is a pair.

**What is not a fault:** from the head of the flight you see over it into
the room below rather than down onto the treads. The rake is 0.25 on 0.3,
which is forty degrees, and any line of sight shallower than that runs above
every nosing. It is what a steep stair looks like from the top.

### And it ended at a wall, 2026-11-12

Jake, standing on the flight: *now the stairs end at a wall.*

The flight was at the room's north end, so it came down four tenths of a
metre from the nook's north wall. That is the same turn he had asked to have
taken out of the top, moved to the bottom.

**The flight did not need to be at the end of the room.** The nook's open
floor runs its whole length, so stopping the flight short of the end lets it
arrive in the room instead of at its wall. `CLEAR_AT_THE_FOOT` is 3.2 and is
the one number that moves it.

**It trades against the sphere's own depth, a metre for a metre**, and
sixteen metres of room has to do both jobs now. The test that asked for five
metres of room behind the ball either way was written when the room was a
plain box and the ball could stand in the middle of the whole of it. The
stair takes four and a half metres down the middle, so the ball stands in
the middle of the floor the stair arrives at, which is eight and a half
long: five each way is not available anywhere in the room. The bar is four,
and buying back the fifth metre would leave a metre and a half of nook in
front of the bottom tread, which is the fault this move was made to fix.

The same test measured to the well, because the well used to be the room's
north end and the floor stopped at it. It is a slot in the middle now and
the walls, the colonnade and the ceiling run past it, so what is behind the
ball from the south is the whole room and not the floor's edge.

### The colonnade comes off the lathe, 2026-11-12

Jake: *the room should be more ornate and decorated.* Then, with the
columns turned: *the reflection doesn't show the details of the colonnade.*

Both are the same thing said twice. A box with two bands round it is a
pier, and this room is named for a colonnade. The lathe has been in this
building since the cellar's barrels: the columns are Tuscan now, a torus at
the foot, a shaft with a little entasis so it reads as carried rather than
extruded, a necking and an echinus under a square abacus.

**And spec 0098's tracer knows boxes and nothing else.** The sphere showed
twelve flat piers while the room had twelve columns, because `solid` still
held one box each. The profile is written once, in `column_waist`, and
`column_boxes` samples the same curve into six bands wide enough to hold
the lathe inside them. So the walk, the glass and the eye are all looking
at the same column, which is the rule this building keeps being caught by.

`colour` asked for a metre of height before it would call a box a column.
That was right while a shaft was one box and calls every band of a turned
one a wall.

The ceiling has ribs the other way as well now, because beams one way alone
are stripes, and the walls have a skirting and a frame round every bay.

### A mirror, and what it showed wrong, 2026-11-12

Jake: *the reflections aren't accurate, and the colour of the ball could
look a lot more like a mirror like reflective surface instead of the blue,
which isn't accurate for the room either.* Then, with it mirrored: *the
reflection gets the colour of the stairs wrong, the columns still don't
look like the ones in the room.*

**The ball is a mirror now rather than a lens**, per the engine's spec 0098:
an index at or under nought, which is not a refractive index. A solid glass
sphere bends the room twice, turns it over and squeezes it into a disc. That
is true and it reads as a smear of bands.

**The blue was the tint, and the tint was a flat multiplier.** `GLOW` was
3.5 of white, standing in for the point lights the tracer has not got. What
it could not stand in for is the colour of the ambient it was multiplying:
that ambient is `garden::filled`, which is the sky's, and at night the sky
is three hundredths red against nine hundredths blue. Every surface the ball
showed came back blue over a room of warm stone. It goes through
`garden::gives_off` now, which is the move the sky, the moon, the runestone
and this room's own bays all make, and what lands is the room's colours at
`SILVER` of them whatever hour it is outside.

**`colour` reads a box's shape, and no shape says timber.** The balustrade
is a run of upright boxes a column's width, so the glass showed a stone
handrail over a timber one, and a tread is a tall slab, which that function
calls a wall. `named` holds the pieces whose colour no shape can tell. It is
not the second account that function's note warns about: the boxes in it are
`guard` and `treads` themselves, asked by name, so there is nothing in it to
drift.

**And the column in the glass is still boxes.** It is ten bands now rather
than six, which is smoother and is still a stepped cylinder. The tracer
knows boxes and nothing else, so the honest fix is a round primitive in it,
which is engine work and is not this spec's.

### Cut the column where the lathe changes, 2026-11-12

Jake: *the columns in the room have wider and thinner bases, the reflection
doesn't match.*

`column_boxes` sliced the shaft's height into ten equal bands and made each
one as wide as the widest thing in it, which is right: a band has to hold
the lathe inside it. Equal bands put the necking and the echinus together
in the last one, so the top of the shaft came out as wide as its foot and
the taper disappeared. In the glass it was a tube.

The bands are cut at the profile's own joints now, which is the torus, five
down the shaft, the necking on its own and the echinus. The same count of
boxes, and a column rather than a tube.

## Acceptance criteria

- The sphere traces the room's own boxes, and the list it is given is `solid` plus `lids` rather than a second account of the building. — `glass::tests::it_traces_the_building_itself`
- A ray that leaves the sphere and hits nothing comes back as the room's ambient rather than as black. — `glass::tests::a_ray_that_hits_nothing_is_not_a_hole`
- What is behind the eye is in the sphere, which is the claim this room is for. — `glass::tests::the_room_behind_you_is_in_it`
- The plinth stands in the middle of the room's length, within a tread of it, and not against either end. — `glass::tests::the_sphere_has_length_behind_it_either_way`
- The frames run the room's length at the corridor's own pitch, so the cone finds something different at every angle. — `glass::tests::the_room_is_a_colonnade`
- You can walk its length and all the way round the plinth with the passage this project has settled on. — `glass::tests::you_can_walk_round_it`
- The stair's foot is inside the nook's end wall and its head is clear of the garden's way. — `glass::tests::the_stair_fits_the_nook`
- The room does not stand over anything that is already there. — `glass::tests::nothing_of_the_building_is_under_it`
- A body pushed at either open side of the flight is stopped by that side's balustrade rather than ending up on the nook's floor. — `glass::tests::you_cannot_walk_off_the_side_of_the_flight`
- And the balustrade costs the flight none of its going, because it stands outside the tread ends. — `glass::tests::the_balustrade_costs_the_flight_none_of_its_going`
- Nothing of the room below hangs in the flight. — `glass::tests::nothing_of_the_nook_hangs_in_the_flight`
- One piece of the plinth draws its top, and only one, so no two faces of it fight for the same pixels. — `glass::tests::one_piece_of_the_plinth_draws_its_top`
- Every part of the nook's open floor still has a lamp on it with the pendant over the flight gone. — `room::tests::every_part_of_the_open_floor_has_a_lamp_on_it`
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
