# 0040 A field with no far side

**Status:** implemented
**Date:** 2026-10-25
**Built:** 2026-10-25, the blocking model

## Goal

An open field off the garden, made as you walk into it: grass, trees,
flowers, a sky and a sun, and no edge to any of it. Jake's, 2026-10-25.

## Why it belongs here

This building is a set of infinities you can walk. The hotel has endlessly
many rooms, the register has a list that cannot be finished, the long hall
has the same rooms spaced further and further apart, the imaginarium has a
climb that arrives where it began, and the gallery has the one that goes
inwards for ever.

Every one of them is indoors, and every one is a wall away from the next.
This is the first that goes outwards, and the first that is endless in the
ordinary direction people mean when they say the word. A visitor who has
understood nothing else in the building understands a field with no far
side.

## Where

Through the garden's far wall, which is the only wall in the building with
sky over it already.

The garden is 16 by 16 with walls 4.2 high, and spec 0010 put a dome 19
across over it because a garden has to have a sky. That dome is the first
thing the field breaks: see below.

## What it must not become

**A diorama.** Spec 0027 gives every room a picture and none of the
pictures is a room. A field of grass and trees with nothing in it to
decide would be the best picture in this building and the least room, and
"what you do there, later" is exactly how a diorama stays one. Spec 0038
is already carrying that rule for the imaginarium, and this would be
carrying it further and with better scenery.

**Scenery that proves nothing.** Grass that is dressing is grass. The
arcade has not built a thing yet that is only nice to look at, and the
reason the garden gets away with being nice to look at is that it is a
garden, which is a thing a building has, rather than a room this one
claims is about something.

## What the field asks

**The horizon.** Jake's, 2026-10-25, from four candidates.

Walk at a tree and you reach it. Walk at the horizon and you do not, and
no amount of walking changes that. Nothing else in this building has
anything in it you cannot reach, because nothing else in it has a distance
in it: the longest sightline indoors is the hall, and you can be at the
end of it in twelve seconds.

~~That it never repeats, and that you can tell.~~ ~~The day.~~ ~~Getting
lost.~~ The second is a claim you cannot check, because you cannot compare
a field with a field. The third and fourth are things the field will do
anyway and neither is a reason for it to exist.

**And the horizon is shown by what is near, not by what is far.** You
learn that a thing is distant by walking and watching nearer things go by
it. A field whose grass streams past while the skyline holds still is the
whole demonstration, and it needs no words, no prompt and nothing to press.
That is what makes this a room rather than a view: the content is
something you do, and what you do is walk.

**So what is on the horizon has no place, only a direction.** A thing you
cannot reach is not a thing a long way off; a thing a long way off is
reached in an hour. It is at infinity, which in a drawing means its
position follows you exactly and only its bearing is real. Turning changes
it and walking does not. The sky is already the clearest case of this and
the field's sky is the garden's sky, drawn where you stand.

That is also the honest arithmetic. The horizon of a flat plane is at no
finite distance at all: it is the line where parallel lines meet, which is
a direction rather than a place. The field is not pretending about
distance. It is drawing the thing correctly.

## What the first build is

The blocking model, in the order the rooms in this building have always
been built:

- The garden's west wall opens, and beyond it the ground does not stop.
- Flat ground that follows you, so there is no edge to walk to.
- The garden's sky and the garden's day, drawn where you stand.
- Trees, flowers and tufts scattered by cells, each cell seeded from its
  own coordinates, built as it comes near and forgotten as it goes.
- The scatter thins with distance into ground and haze, so the far edge
  reads as a horizon rather than as the end of the floor.

The trees are the garden's trees. `garden::Tree` already says what one is,
with a trunk mesh, limbs and pads, and a field cannot have a mesh a tree:
so the field stands a handful of them many times over. One account of what
a tree is, which is the rule this building keeps breaking.

## What the engine already has for it

**The chunk loader is half written.** `Scene::standing` takes a run under a
name and keeps it, and `forget_what_nobody_mentions` drops any name that
has not been mentioned for `REMEMBERED` frames. A field is cells named by
their own coordinates, mentioned while they are near you and forgotten
when they are not, which is that function's existing behaviour with
nothing added.

**There is no instance budget on a desktop**, and sixteen million in a
browser, per spec 0071. The browser build is the one that decides how much
grass there is.

**And the arcade has drowned once already** on something that only
happened while nobody was watching, so a generator that builds cells and
never drops them is a known way to take this machine down. That belongs in
a test with a watchdog before the first tree goes in.

## What else it is waiting on

**The sky.** The garden's dome is 19 across, sprung over the garden, and
spec 0010 records what happened the first time a dome tried to cover more
than its own room: it ran through the hall and the nook at head height.
A field needs either a dome that follows you or a shell far enough off
that walking does not close on it, and the second is a lie that only works
until somebody walks for a minute. Whichever it is, the building's rooms
must still not see it.

**Flat or not.** A flat field is one `walk` away from working, because
`walk` is collision against boxes and the ground is one box. Ground that
rolls is a height field, which `walk` has never been asked for, and that
is an engine change rather than an arcade one.

**What grass is.** Blades are not a question worth asking: a field of
them is tens of millions of instances. Ground that is textured, with
tufts standing in it near you and nothing but texture far off, is the
shape of the answer, and where the near part stops is a number somebody
has to look at.

**How far it actually goes.** Positions are `f32`, so at eight kilometres
out the smallest step the arithmetic can tell apart is about a
millimetre, and at sixty-five it is near a centimetre. The field is
endless the way spec 0039's dive is endless: the mathematics does not
stop and the machine does. That is a true thing about computers and this
building has said it once already, so it should be said the same way
rather than hidden.

## Out of scope

Weather beyond what spec 0016 already does. Animals. Anything built in
the field. Seasons.

## What the field showed about the rest of the building

**Nothing in here had an outside.** Walked on 2026-10-25, the first evening
it existed, and three faults came back at once, all of them the same fault:
every surface in this building faces inward, because until the field there
was nowhere to stand that was not a room.

- **The garden had no walls from out there.** A wall here is a collider and
  a skin on its inner face, with nothing on the back, so from the field you
  looked through the garden's west wall at its pond, its stones and its
  lanterns. Narrowing the way out to twenty centimetres changed not one
  pixel of it, which is how it was told apart from a hole.
- **The sky was a blue bubble sitting on the roof.** Spec 0010's vault, 19
  metres across over a 16 metre garden, is a sky from underneath and an
  object from outside. The field had a second sky of its own, so the garden
  and the field, which are one outdoors with one sun and one day over them,
  had two skies with two rims and two scales and each could be seen from
  under the other. There is one now, drawn round the eye at 22 times the old
  size, springing from the ground. `garden::springs`, `garden::dome_at` and
  `garden::vault_at` went with the old one.
- **The solar system stood above the roofline with its planets named.**
  Spec 0011 is a room nobody is meant to know is there and spec 0015 is what
  is in it, and from forty metres out you could read Saturn's name over the
  roof of a building three metres tall. `draw_the_system` was gated on
  nothing but its meshes existing, and a hundred metre far plane and a
  sealed room had been doing the work. It is gated on standing past the
  hall's near end wall now.

The garden's three outward walls have outer faces, from the same
`garden::facing` list that gives them inner ones. The rest of the building
does not: see below.

## What the first build left out

**What is on the horizon.** The horizon itself is there and the ground meets
the sky in a line, but there is nothing standing on that line to set out for,
and setting out for a thing is what makes the walk a walk rather than a
stroll. Hills, a tower, a single tree: Jake's call, and the only part of this
spec that is still a question.

**Ground that rolls.** Flat, as the spec said it would be. `walk` has never
been asked for a height field.

**What grass sounds like.** `noise::underfoot` has no grass in it, so the
field is walked on gravel.

**Anything at all behind you.** You can walk back into the garden, and from
far enough out the building is below the horizon, and that is the whole of
getting lost as it stands.

**What the building looks like from its own field, above the garden's
walls.** The garden is plastered on both sides and reads as a walled
compound. What stood over it was the imaginarium, in the pale stone of its
own inside, with no roof and no articulation, because nothing had ever seen
it from outside.

**And it is a storey now, standing on the hall's roof.** Spec 0038's room
is a cube twelve and a half metres each way whose floor is at six, and the
hall's lid is at three point two, so the pale slab on the skyline was also
floating two and a half metres clear of the building with daylight under it.
What was in that daylight was the shaft the way up climbs. `aloft::outside`
is one mass from the lid to the top of the room, in the garden's plaster
with the garden's cope, and the shaft and the gap are both inside it. A gap
under a room is not a thing a building has.

Two things it taught, both of them about reusing the field's dressing on
something taller than the field has dressed before.

The plaster is one picture per face, not the field's tile. That tile carries
the splash the foot of a wall gets, which is the point of it on a run three
hundred metres long, and tiled up a face fifteen and a half metres high it
is a dark band every six metres with three of them one above another. The
compound wall and the hotel's corridor are both under one tile tall, so
nothing had shown it. One picture per wall is what the garden's own walls
have always had.

The cope is a parapet round the edge rather than a lid over the top, and it
stands back from the room's inner face. Spec 0038 puts a way out on all six
faces of that cube, so two of them stand on the ceiling: a cope across the
footprint seals both, and the garden's two-sided oversail lays eleven
centimetres of stone across the opening of each, because those doorways begin
exactly at that face. A garden wall has two outsides and this one has a roof
on its inside.

**And all six of those doorways stand outside the shell.** A closed recess
behind a hole is a box on the far side of the wall it is cut into, so the two
in the ceiling are crates on the roof and the four in the walls are bays off
them, and all six were out there in the pale stone of the room's inside.
`aloft::jutting` takes the bounds of the recess itself rather than writing the
box out again. Each gets plaster on its four faces that are neither buried in
the wall nor on top, and stone over the top: a capstone on a lantern, a hood
on a bay, the parapet's course on a smaller plan either way. The underside of
a bay is as much on show as its front from a field fifteen metres below, which
is the face that would have been forgotten.

Which of the box's three extents a quad is scaled by depends on which way the
quad was turned to face, and worked out at the call it is the third slot that
gets it wrong, silently. `room::quad_on` is the one account of it.

**And the storey under it had no roof at all.** The hall's lid is drawn as
one quad per piece turned over to face down, because from underneath an
upward plane is a back face and the opaque pass culls it. From above it is
the other way about, so flying over the hall you looked straight down into
it at the top of its own suspended ceiling. The same screed, one picture per
piece.

The stairwell stays cut out of those pieces, because the hole is under the
imaginarium's mass and that is the only thing up here allowed to cover it.

The rooms that adjoin at that level are a different job and not this one.
The hotel's lobby, the register and the back room each have a lid of their
own at their own height, and those lids are boxes rather than downward
quads, so they are there and lit and simply undressed, along with the walls
under them. Spec 0015's is the fifth and it is a hundred and fifty nine
metres square: it is the black plane you can read Neptune over, which spec
0041 says is a gate to be fixed rather than a surface to be dressed.

**And the roof, which spec 0041 found the first hour it existed.** Thirteen
metres square, the largest surface this building has, and it was the pale stone
of the room's own ceiling, because until there was a way to be above it nobody
had been. It is lime laid flat in bays now, and the quad runs the whole
footprint so the cope buries its edge.

`garden::screeded` rather than `plastered` turned on its side. Every mark in
that picture is about a wall standing up: the rain streaks down it, the ground
splashes its foot, and the course under the cope dries first. Laid flat the
streaks are stripes along one axis, the splashed foot is a dark band along one
edge and the dry course a pale band along the other, which on a roof this size
is three wrong edges. What a flat lime roof looks like from above is bays with
a joint between them, bleached by the sun, and dirtiest in the middle of each
bay where the water stands. Only in the bays that hold water: pooling the same
in every one is seven by seven smudges on a grid, which is the fault
`plastered`'s streaks had before they were made to pick their columns.

**Both faults in the parapet came from the engine and not from looking at
it.** `aloft::tests::nothing_drawn_in_here_is_in_one_plane_with_anything`
already ran spec 0048's coplanar check over this room, and the outside is in
its list now: it found the course flush with the inner face against the
reveal of each ceiling doorway, 0.13 by 1.60 twice over, and then the four
runs of the ring lapping at the corners, 0.30 by 0.30 four times. The runs
along x take the corners and the runs along z stop at their faces, where two
end faces look opposite ways and neither is hidden.

The dashed line under every cope in this building, the compound wall's
included, is older than any of this and is not the sun's shadow map:
shrinking that map from a hundred and sixty metres to thirty changes it by
nothing.

**Where the field meets the rooms that are bigger than it.** The field is a
half plane west of the garden's west face. Making it a plane means deciding
what happens where it meets the three rooms in this building that run a
hundred metres or more: spec 0029's corridor is two kilometres long so that
its doors never stop, spec 0035's is the same, and spec 0015's room is a
hundred and fifty nine. A field that went round all three would start past
the horizon; one that goes over them has two kilometres of wall running
away across the grass.

**Over them, and the wall is the point.** Jake's, 2026-10-25, after
standing in the field and seeing two pale wedges on the skyline that turned
out to be the two corridors end on. Spec 0029's is dressed: the garden's
own plaster, the garden's own cope, running to the vanishing point. The
endlessness is a thing you look at out here rather than a thing the room
tells you, and this is the only place in the building you can see the whole
length of it.

Spec 0035's is not drawn from outside at all. It is spec 0029's corridor
re-spaced, and two of them on one skyline is the same sentence said twice.
See `twine::inside`, which is the same gate `draw_the_system` got for the
solar system and for the same reason.

Spec 0015's room is the third, and it stays hidden: it is a secret, which
is what gating it was for.

**And where the ground is is not asked as a region at all.** Three times
the field had an edge somewhere nobody had thought to look, and each time
the fix moved the edge rather than removing it: first east of the garden's
west face, then round the garden's corner, then the strip north of it,
eleven hundred square metres with grass drawn on it and nothing under it,
because `garden::under_the_sky` is a quarter plane and `open` was a half
plane and the two are not the same shape.

A region cannot help being a region. So the question the walk asks is
`field::afoot`: if nothing of the building is under your feet, the field
is. There is nowhere left for an edge to be, and
`field::tests::there_is_ground_under_everywhere` sweeps sixty metres each
way round the building every half metre to say so.

**And what is drawn asks the same question.** It did not. The ground was
solid as a half plane and drawn as a quarter one, `garden::under_the_sky`,
which stops at the nook's end wall: south of that line you stood on grass
nobody had painted and looked straight into the nook with black where the
sky should be, four seconds' walk from the gap. `field::in_view` is
`afoot` plus the garden now, and the sweep holds the two together, because
being the same shape by accident is how this has gone wrong twice.

Two things that took finding in `afoot` itself. A floor only counts if it
is at least as high as the field's would be, since the baths' roof is
thirty centimetres down and runs out past the garden and counted as a
floor it took the ground away over the whole of it. And a body inside a
box of the building is in the building whatever is under it, which is what
the picture of the space behind the wall is: it stands a floor's thickness
under that room's floor, where nothing of the building is under the feet
because the feet are in the floor.

Two things that took finding. A floor only counts if it is at least as
high as the field's would be: the baths' roof is thirty centimetres down
and runs out past the garden, and counted as a floor it took the ground
away over the whole of it. And it is asked at any height, because it used
to test that you were above the grass first, so the moment anything put a
body a hand's breadth under it the grass stopped being there and the body
fell for ever.

What the half plane gets instead is an edge you cannot walk through. The
first draft left it at the garden's own west wall and called the rest
unreachable, because the wall runs away north and south further than you
could get round. The garden is sixteen metres long. Jake walked out of the
gap, round its corner and east into no ground at all inside a few seconds,
and came back standing on the hall's ceiling.

So the wall runs the whole length of the ground, in the garden's own
plaster with the garden's own cope, with the garden's stretch left to the
garden and the way out the one gap in all of it.

**And the plaster on it is one run a side.** The garden used to plaster its
own outer face from its own list, one picture for the whole face the way
its other walls get one each, and the field tiled a different picture over
the rest: two pictures at two phases meeting at the corner, which reads as
a seam. It is one wall, so it is one run, through the garden's stretch,
with the way out the only break in it. The cope on it is the garden's own
two courses, the tiles and the roll set in from them, because one course
and no roll was a step along the top of a single wall.

A run's length is a whole number of tiles, and the quad carries its tile
count in its own corners, so the near end is the way out, which does not
move, and the far end is snapped out to the next tile past where the ground
stops. Unsnapped, a run whose length wanders by a metre is a run whose
tiles breathe as you walk.

**And where the garden's stretch ends is said once.** It was said twice: the
plaster on the garden's outer face ran to the garden's own edge and
`field::wall` began a wall's thickness further out, so there was a slot one
wall thick at each end of the garden with nothing drawn in it. Jake looked
through the north one at the grass beyond. The colliders had no such gap,
which is why nothing stopped him and why only looking found it.
`garden::west_ends` is the one account now, and
`field::tests::the_wall_has_one_gap_in_it` walks the line every ten
centimetres to say the only hole is the way out. It follows you and ends
where the ground does. That is also the answer to what the building looks
like from its field at eye level: a long wall with one way through it.
`field::tests::you_cannot_walk_out_of_the_field` walks a body at it from
four places for twenty seconds; without the wall that walk ends seventy
eight metres inside the building.

## Acceptance criteria

- You can walk out of the garden into it. — `field::tests::you_can_walk_out_of_the_garden`
- The ground never gets nearer its own edge, a kilometre out or
  eight. — `field::tests::the_ground_never_gets_nearer_its_edge`
- The sky is further off than the ground on every bearing, corners
  included. — `field::tests::the_sky_is_further_off_than_the_ground`
- And the camera sees as far as the sky. — `field::tests::you_can_see_as_far_as_the_sky`
- A cell holds the same things every time it is asked, and two cells do
  not hold the same things. — `field::tests::a_cell_is_itself_and_nothing_else`
- Nothing of the field stands inside the garden or the
  building. — `field::tests::nothing_of_the_field_is_indoors`
- How much is asked for at once does not grow with how far you have
  walked. — `field::tests::walking_does_not_fill_the_machine`
- The grass is a silhouette and is mostly not
  there. — `field::tests::the_grass_is_mostly_not_there`
- The field has one edge and you cannot walk to
  it. — `field::tests::the_field_has_one_edge_and_you_cannot_reach_it`
- And you cannot walk out of it, from anywhere, in twenty
  seconds. — `field::tests::you_cannot_walk_out_of_the_field`
- There is ground under every spot a body could
  be. — `field::tests::there_is_ground_under_everywhere`
- And the wall along its edge has one gap in it and no
  others. — `field::tests::the_wall_has_one_gap_in_it`
- And a body let loose outdoors for half a minute stays in the
  world. — `field::tests::a_body_let_loose_outdoors_stays_in_the_world`
- And the whole of it, from outside the building looking
  back. — `tests::the_field_is_drawn`
- The hotel's corridor, from out in the field, dressed and running to the
  vanishing point. — `tests::the_corridor_from_the_field_is_drawn`
- The imaginarium stands on the roof under it, with no daylight between
  them. — `aloft::outside_tests::the_mass_reaches_the_roof_it_stands_on`
- And its faces run from that roof to the top of the
  room. — `aloft::outside_tests::the_imaginarium_stands_on_the_roof_under_it`
- The shaft the way up climbs is inside that
  mass. — `aloft::outside_tests::the_way_up_is_inside_what_the_field_sees`
- Its cope sits on the top and nothing of it hangs
  below. — `aloft::outside_tests::the_cope_sits_on_the_top`
- And it stays out of the room's own plan, so the two doorways on the
  ceiling are
  clear. — `aloft::outside_tests::the_cope_is_clear_of_what_stands_on_the_roof`
- Nothing drawn out there shares a plane with anything drawn in
  here. — `aloft::tests::nothing_drawn_in_here_is_in_one_plane_with_anything`
- And nothing of the room stands outside its own shell
  undressed. — `aloft::outside_tests::everything_outside_the_shell_is_dressed`
- The roof reaches under the cope and is laid inside the course's own
  thickness. — `aloft::outside_tests::the_roof_runs_under_the_cope`
- And what is laid on it has no dark edge, which a wall's plaster laid flat
  would have. — `garden::screed_tests::a_roof_is_not_darker_at_one_edge`
- And two pieces of one roof agree where they
  meet. — `garden::screed_tests::two_pieces_of_a_roof_meet`

### Verified by hand

- No line across the grass where the sun's shadow map ends. It was at
  thirty six metres, which is a hard edge between lit and unlit grass with
  the trees standing beyond it: the map follows you out here and was sized
  for what casts rather than for what you can see. It is the ground's own
  reach now, and a texel of it is sixteen centimetres against a tree four
  metres wide.
- No line where the trees stop. They ran to a hundred and twelve metres
  against a ground of a hundred and sixty, which is a tree line with a band
  of bare grass beyond it. They run to the ground's edge now.
- Walk at a tree and you reach it; walk at the horizon and you do not.
- The grass streams past and the skyline does not.
- The ground has no lit path down the middle of it that turns with you.
  The first build had one: a broad specular highlight on a surface this
  size, which is the garden pond's note on a field.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
