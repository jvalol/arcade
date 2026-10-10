# 0010 A garden off the entrance

**Status:** implemented
**Date:** 2026-10-07

## Goal

A zen garden with trees and a koi pond, through the wall on your left as you
wake up. Every room in this building so far has been a room: plastered, lit,
and full of made things. This one is ground, water and growing things, and it
is the first place in the arcade that is not somebody's furniture.

## What it is

**Where.** Behind the near end of the left wall, through an open gap in the
stretch past the last cabinet. Open to the top, with no beam across it: a
garden you step into under a lintel is a garden through a door, and a doorway
is a thing a building does. The wall simply stops. You wake a step inside the near wall looking
down the hall, and the way in is at your left hand before you have taken a
step. The nook is also off the left wall and is nowhere near it: the nook is
the far end, past six cabinets, and its end wall at `shut` is the garden's near
boundary. Nothing else has ever been built at this end.

The ground beyond that wall is empty in a way no other room's was. The nook cut
into the building's own footprint and the baths were dug off the cellar, but
past `shut` on the left there is no floor, no wall and no ceiling, so this room
is not carved out of anything and can be the size it wants to be.

**It is big.** Eleven deep and eleven along, against the nook's six by
fourteen. A garden read as a corridor is a planter, and every room in this
building has been too small on its first build: the nook twice, the baths three
times. The pond sits off centre with walking room on all four sides, and the
widest walk is a little over three bodies.

**The floor is gravel**, raked in lines, with a stone border where it meets the
walls and flagstones where you walk. Raked gravel is the one thing a zen garden
is always drawn with and it is a texture rather than geometry, so it costs
nothing and reads at once.

**The pond holds engine water**, per spec 0043 of blitzkit, the same
heightfield the baths use. It is sunk into the ground in a stone basin with a
tiled lining, which is what spec 0008 learned gives water its depth: a flat
colour under a surface is a flat colour at any depth, and the baths read as a
blanket until the basin was tiled.

Still water, not a pool. The baths are stirred by wading and by the fountain;
this one is stirred by the fish and by nothing else, and most of the time it is
a mirror.

**There are koi in it.** Three, slow, each going its own way round, and each
one pushing the surface as it passes. They are the reason the pond is not a
mirror and they are the only moving things in the room.

Three and not four, like the trees and for the same reason: two is a pair and
four is a shoal. The big one swims the outer circle near the surface, the
small quick one swims a tight circle near the bottom, and that is the order a
pond puts them in by itself.

Each is a turned body with a fan of a tail hung off the stalk, swinging. A
body on its own drifts like a leaf; what says fish, before you can make out a
fish, is the tail going over.

Patched, not coloured. One colour per fish is three fish the colour of
plastic, and what a koi is, is the patching: a white one with red over its
shoulders is a different animal from a red one. Each wears a painted skin,
whose patches run round the body the short way so one falling off the side
comes back on the other.

The pond's water had to be opened up for any of this. It was drawn opaque,
which is a sheet of slate with a kerb round it, and there is no point putting
fish in a pond you cannot see into. Short of clear, because this is rainwater
under a storm: the deepest koi is a shape going by and the shallowest is a
fish.

The pond's bed is silt and not stone. Cut from the same grey as the kerb, a
pale floor under see-through water is a tiled bath with a coping round it,
which is exactly what this came out as the moment the water was opened up.

The water's highlight is a tight one, like the baths'. The default is a broad
one, and over a surface this size that is a sheen across the whole pond, which
was the other half of the bath.

And the stir falls off with depth. Stirred the same by all three, the pond has
a mechanism in it. What the surface is for is saying that something is down
there and roughly how far down, so the shallow one drags a dimple after it and
the deep one drags nothing.

**Set stones, and the raking that goes round them.** This is what the garden
was missing. A zen garden is rocks; the gravel is the thing around them, and
raking with nothing to break round is corduroy.

Three groups, of three, three and two. Odd numbers and never the same number
twice. Each group is one tall stone, one low and broad, and in the threes a
small one set nearer the tall one than the broad one is, so the group reads as
two things one of which is two things rather than as a bus queue. A third of
every stone is under the gravel, because set on the surface they are pebbles
on a tray and the one thing every account of these gardens insists on is that
a stone has to look like it came up rather than having been put down.

In the open gravel, which in this garden is the south strip and the east one.
The pond takes the middle and the walk round it takes a body's width outside
that; what is left either side of the water is a corridor and not a place to
put anything.

Carved and faceted. `compute_normals` averages a face's normal into the
corners it shares, which is right for a barrel and wrong for a stone: a
boulder of a dozen facets shaded smooth is a potato. A rock is the one thing
in this garden whose faces you are meant to see.

Out of round two ways, because one is not enough. Roundness that varies the
way round the stone has to come back to nothing at the poles or the stone is
torn open there, and on its own it leaves a lump that is still a ball in
outline. What makes the outline lopsided is leaning the whole of it over,
which a function of the height alone can do because it takes the poles with
it.

The raking runs in rings round each group and straight everywhere else, so the
stones are islands and the gravel is water. That means a bed is painted its
own picture in the garden's own coordinates rather than wearing one tile
repeated, since a tile cannot know where a stone is. The rings stop ten lines
out, a whole number on purpose, so the handover lands on a line and reads as
the outermost ring rather than as a cut edge.

The lines are a hand's width apart. They were a metre and a half, which is
what a tractor leaves: standing next to it you were inside a single line, the
gravel read as a wash with a gradient across it, and the raking was not
visible at any distance at all.

**The lip is set stones, not a run of coping.** One band of pale stone round a
rectangle of water is a swimming bath whatever it is cut from, and that is what
this was. The lip is a bank now with stones set on it: flats mostly, varying in
length, height and how far across the band they sit, a boulder every third one
and a bigger boulder at each corner, thrown a little off the line and turned a
few degrees each so the run is set and not sawn. They lean out over the water,
because what breaks the line of a kerb is the shadow under an overhang, and
there are gaps between them where the dark of the bank shows.

The bank and the stones are two things and not one. The bank is what holds the
water and what stops you walking into it, so it has to be continuous; the
stones vary, and a thing that varies cannot also be the thing that is
continuous.

**The pond is raised, and the bank is higher than a step.** That is not a
choice about how it looks. The garden's ground is one slab under the whole of
it, the pond included, so there is no hole to fall into: the water is drawn
over ground you can stand on, and anything round it low enough to climb is a
lip you walk over to stand on the pond. `walk::STEP` is 0.42 and the bank
clears it.

Raised rather than sunk with a wall round it, because the second is a trough.
The water comes up to a hand's width under the lip, which is where a koi pond's
water is: near enough the top to be the thing you see, and near enough to lean
on.

**The garden stands on the roof of the baths.** It was laid over them without
either knowing, and it was wrong in both directions at once. The baths' ceiling
landed at nought and the slab over it stood from nought up, so their roof came
through the garden's gravel as a ledge you could not see, could walk up, and
could then step off into the koi pond. And the garden's ground was two units
thick, so it hung that far down into the far corner of the baths, where it is a
block of nothing at chest height.

Both are the one statement: over the baths, the garden's ground is their roof.
The ground is a slab the thickness of a floor now, and the baths' ceiling is
below it rather than level with it. Below and not level: two floors whose tops
are in one plane put the lower one's end face in the middle of the upper one's
floor, a body resting on a surface counts as touching it, and the sweep against
that face lets you most of the way through and then jams. Walking west across
the garden stuck on nothing, at the line where the baths below happen to end.

The baths lost a third of a unit of ceiling to it, and spec 0008's room is that
much lower.

**Stepping stones from the way in to the water.** You arrive and the whole
floor is equally walkable, so you wander. A path of irregular flats tells you
where to stand, and it turns the gravel from something you cross into
something you look at.

A stride apart, middle to middle, because the spacing is the pace the garden
wants you to go at. Thrown left and right of the line by turns, each its own
size and its own way round: a path of one stone repeated is a row of tiles.
Bowed, and bowed away from the water, since a straight line between two points
is a kerb.

Walked rather than divided. The path is a curve, so cutting its own parameter
into equal pieces bunches the stones up round the bend and spreads them on the
straights, which is the one thing the spacing is not allowed to do.

Both ends are taken from the room and neither is written down. The first
version wrote them down as places in the garden, and the opening sits a fixed
distance from the near end of the hall while the garden's middle does not: at
twelve cabinets instead of fourteen the first stone was a stride and a half
outside the door. The test walks the path at five different cabinet counts for
that reason.

A flat slab and not a squashed boulder. A turned lump flattened has a domed
top, and the top is the one face of a stepping stone anybody ever sees. It
stands a knuckle out of the gravel: flush, it is a pattern on the floor, and
any higher it is something to trip on. Low enough that it is not in `solid`
either, because a thing you step on is not a thing you walk round.

**Trees**, turned on the lathe like everything else round in this building:
a trunk that tapers and leans, and pads of foliage held out from it. Not a
cone. A cone is a fir and a fir is not what anybody draws in a zen garden.

Not three discs threaded on a straight trunk either, which is what it was, and
which is a lollipop. A tree in a garden like this is pruned for years to lean
and pruned clear along its limbs so the foliage is only at their ends: what is
left is a handful of clouds at different heights with sky between them, and the
gaps are the point.

So the trunk bends. The bend comes on with the square and a bit of the height,
so the foot is upright and the lean is in the top half; a straight slope is a
mast guyed over. Which way each one leans is not written down. Every one of
them leans towards the water, which is what they are pruned to do and what puts
something over the pond to look through, and a direction per tree is three more
numbers to get wrong.

One account of the trunk's line, and everything hangs off it: the mesh is built
from it, the pads sit on it, the limbs start on it and the collider follows it.
The first build had the lean baked into the mesh and the pads placed on the
straight line the trunk used to be on, so every pad floated off the side of its
own tree.

Five pads, going round as they go up, each a different size, the last on the
trunk's own line because the top of a pruned pine is its apex. Each hangs off a
limb that leaves the trunk a little below it, because a branch goes out and up.
A pad with no limb is a cloud.

The pads wander a good deal. A smooth one is a pebble, and worse, a squashed
sphere carries the one highlight it is given all the way round its rim as a wet
green streak.

And the trunk is a fifth thinner than it was. That number had never been
measured against anything: the collider round a tree was a box 0.34 across and
the trunk it stood for was 0.42 at the foot, so the tree you walked round was
narrower than the tree you could see. Taking the collider off the trunk's own
thickness made it show up as three trees too close to the walls, and the trunk
was what was wrong.

The collider follows the lean as far as head height and no further. A box at
the foot lets you walk through the part of a leaning trunk that is in front of
your face; a box round the whole lean is a tree you cannot get near on the side
it leans away from.

**Stone lanterns** stand among the trees and on the pond's edge, and they are
the room's only light. Per the lesson of the cellar and the baths, the lamps
are derived from the lanterns rather than written down a second time: there is
no lamp that is not inside a lantern.

**A sky, and a storm in it.** A garden wants sky and this engine has none, so
outside a window is black. The answer is a dome over the garden with cloud
painted on the inside of it, past one in every channel so it glows rather than
waiting to be lit, and turning slowly so the weather goes over.

Wider than the room and sprung below the tops of its walls, so the walls hide
its rim and every way you can look up lands on cloud. Sprung level with them
and only as wide as the garden, there was a wedge of black over one corner
where the sky ran out.

**And sprung above the building's ceilings**, which is the other end of the
same decision and was missed. The dome is nineteen across against a garden of
sixteen, so it reaches well past the garden and over the hall and the nook, and
anything of it below their ceilings is inside those rooms. At a little over
half the garden's height the rim sat at 2.31 and every ceiling in here is at
3.2: the rim ran through the nook and the hall at head height, and from in
there it was a pale blade hanging down through the ceiling.

There is no radius that fixes it, which is why the height is the thing that
moves. The nook is nearer the garden's middle than the garden's own corners
are, so a dome small enough to keep its rim out of the nook is too small to
cover the garden. It springs between the building's ceilings and the top of the
garden's walls instead, and both ends of that are load bearing.

It took a recording, a build that logged where the camera was so the one view
that shows it could be stood in exactly, and a build with the dome left out, to
find. Six guesses at where to point a camera found nothing.

It is wound to be seen from inside, which is the opposite of everything else
this building turns. Made two sided instead, every triangle had a twin a hair
away facing the other way, and the sky came out in patches of cloud and patches
of unlit dark: two surfaces in one plane, which is the fault this building has
now fought five times.

**The picture goes on as a disc seen from below**, not wrapped round like a
map of the world. Wrapped, all forty-eight columns of the dome ran into its
apex and the whole top edge of the picture was pulled into that one point: a
dark star hanging over the middle of the garden, which is the part of the sky
you lie in it and look at. The middle of a disc is a point already, so there
is nothing left there to pinch. The seam goes with it, since the two sides of
the picture that used to meet round the back now fall on the same texels.

The angle down from the apex gives the radius, so the scale is even from the
middle out. Only the circle inside the picture is used and its four corners
are not, which is a fifth of it spent on having no pole, and the cloud cells
are half the size they were to land the same size on the dome.

Its normals come off the shape rather than averaged off the faces, for the
same reason in a different place. The forty-nine vertices at the apex are all
in one spot, so the triangles between them have no area and every normal there
came out of rounding: forty-eight pointed down and the one at the seam pointed
up. That one was never what you could see, which a build with the sky drawn
in flat colour settled, and it is still wrong.

**The hall's wall is a metre shorter than the garden's**, so the garden carries
a parapet over it. Without one there was a metre of nothing above that wall and
over it you were looking at the roof of the hall from outside: a dark slab
hanging in the garden's sky with the lanterns catching its underside.

**In one piece and not two, which this spec had the wrong way round.** It was
two, either side of the way in, because a parapet across the opening was a
beam over the entrance while that opening still had a lintel in it. The lintel
came out and the two pieces stayed, so the band stopped at the doorway and
started again past it, and over the way in there was a notch of sky. The
opening runs to the hall's own ceiling now, so the band over it is the head of
the doorway and nothing is hanging.

Both pieces were sprung from a height written out twice, one from the midpoint
of the two ceilings and the other from `HIGH * 0.55`, which is 3.70 against
2.31. The low one sat inside the wall and left the whole metre open behind it.
Two accounts of one number, which is the fault under most of the others in
this building, and the height is named once now.

**The parapet over the nook sits on the nook's wall**, which is not the same
plane as the garden's own south side. The nook's end wall is at
`-reaches + NOOK_SPAN` and the garden's south side is half a wall further out,
a sixth of a metre apart. Sprung from the garden's, the parapet overhung into
the garden along two of its sides and stopped short on the other two, which
from underneath is a ledge running the width of the nook. It comes off
`room::nook_end` now, which is the one place that says where that wall is.

It is wider than that wall by the same sixth of a metre, because it has to
reach the garden's own south wall as well, and that wall stops where the
nook's begins rather than where it ends. Moved onto the nook's wall and left
at the nook's width, it left a hole of exactly that width where the two meet.
The sweep in `the_garden_is_closed_all_the_way_up` steps thirteen centimetres
and allows six, so a fifteen centimetre hole went straight through it; the
sweep that caught it steps one.

**The head of the way in is drawn through the wall and not skinned.** Every
other face of this garden is a skin a hair proud of a wall that already
exists. Above the hall's head there is no wall to be a skin on, so a skin left
the wall's own thickness open behind it: a hand's breadth of channel, and from
the garden, at the angle you look up at a doorway from, a line of sky through
the slot. Four pixels, found the same way the metre was, by standing in the
garden and looking at the door.

**The hall's lid and its carpet are the shape of the hall.** Both were one
rectangle, wide enough to take in the nook and as long as the hall. The hall's
floor is an L, and a rectangle round an L is bigger than the L by the corner
the two arms do not share. That corner is out of doors. It put a slab of
ceiling six by five in the air over the garden, three metres up and dark, and
a patch of the arcade's carpet flat on the gravel under it. Both are drawn
from `Room::floors` now, one quad per slab, which is the same single account
of a number that split the floor itself.

The carpet goes over the aisle alone, because the nook has boards of its own
over every inch of its floor and always did. Its tile count comes down with its
width so the weave is the size it has always been.

**Both borrowed walls are topped, and only one of them was.** The hall's wall
got its parapet. The nook's end wall, which is the garden's near side, did not,
and it stops at the nook's ceiling while the garden is a metre taller: there was
a strip of nothing the whole width of the nook. From inside the nook, looking up
at that end, a wedge of the garden's sky came through the ceiling, and from the
garden you were looking down into the nook.

The test for it sweeps every side of the garden at the height between the rooms'
ceilings and the garden's own, rather than checking the corners, because a gap
in the middle of a wall is what this was.

**The wall it borrows is finished on both sides.** The nook's end wall is the
garden's near wall, and from in here it is a garden wall, not the back of a
bookcase. The baths made exactly this mistake with the cellar's wall.

That finish stops either side of the way in. Run across it, it stood a hair
behind the opening and closed it: from the hall you looked through the gap at a
flat panel and the passage read as an alcove rather than a way anywhere.

## What it is not

**Not a path through to anywhere.** It is a room you go into and come back out
of. The building is a line, hall to nook to cellar to baths, and a garden on
the end of a branch is a place rather than a corridor.

**No weather, no day.** The light does not change and nothing falls.

**The koi are not simulated.** They swim their own circles at their own speeds.
A fish that avoids another fish is a spec of its own and this is not it.

## Acceptance criteria

- You can walk into it from where you wake. — `garden::tests::you_can_walk_in_from_where_you_wake`
- The walk round the pond goes all the way round. — `garden::tests::the_walkway_goes_right_round_the_pond`
- And out again from the far side, by way of the door. — `garden::tests::you_can_walk_back_out_from_the_far_side`
- Nothing stands in the pond, against a wall, in the way in, or on the walk. — `garden::tests::nothing_stands_where_you_have_to_walk`
- No part of the hall is laid over the garden. — `garden::tests::nothing_of_the_hall_is_laid_over_the_garden`
- Every koi stays in the water, nose and tail, all the way round. — `garden::tests::every_koi_stays_in_the_water`
- No two of them ever swim through each other. — `garden::tests::no_two_koi_swim_through_each_other`
- Each points the way it is going. — `garden::tests::a_koi_points_the_way_it_swims`
- A koi is fattest at the shoulders and comes to something at both ends. — `garden::tests::the_koi_is_fattest_at_its_shoulders`
- The shallow one stirs the water and the deep one does not. — `garden::tests::only_the_koi_near_the_surface_stir_it`
- A stone is cut, has a surface, and is out of round. — `garden::tests::a_stone_is_cut_and_out_of_round`
- It has no crack up the back of it and no tear at either pole. — `garden::tests::a_stone_has_no_crack_up_the_back_of_it`
- No group is stones of a size in a row. — `garden::tests::a_group_is_not_a_bus_queue`
- The raking rings the stones and runs straight away from them. — `garden::tests::the_raking_runs_round_the_stones`
- The path runs from the way in to the water, at any size of room. — `garden::tests::the_path_runs_from_the_door_to_the_water`
- Every step along it is a stride, and it is not a straight line. — `garden::tests::every_step_on_the_path_is_a_stride`
- No stepping stone is laid on a stone, a tree, a lantern or a wall. — `garden::tests::no_stepping_stone_lands_on_anything`
- You cannot walk into the pond, from any side. — `garden::tests::you_cannot_walk_into_the_pond`
- The lip is set stones and not a run of coping. — `garden::tests::the_lip_is_set_stones_and_not_a_kerb`
- The garden stands on the roof of the baths and neither is inside the other. — `garden::tests::the_garden_sits_on_the_roof_of_the_baths`
- The garden is closed all the way up, on every side. — `garden::tests::the_garden_is_closed_all_the_way_up`
- And the head of the way in is drawn through the wall. — `garden::tests::the_head_of_the_way_in_is_drawn_through_the_wall`
- The parapet sits on the wall it tops. — `garden::tests::the_parapet_sits_on_the_wall_it_tops`
- And the south side is closed at parapet height, all the way. — `garden::tests::the_south_side_is_closed_at_parapet_height`
- No part of the sky is inside the building. — `garden::tests::no_part_of_the_sky_is_inside_the_building`
- The sky's picture is not wedged into a point anywhere. — `garden::tests::the_sky_has_no_point_where_its_picture_is_wedged`
- And it faces inward everywhere, its apex included. — `garden::tests::the_sky_faces_inward_everywhere_including_its_apex`
- Every tree leans over the water, and bends rather than slopes. — `garden::tests::every_tree_leans_over_the_water`
- Its foliage is pads going round it, not a stack of plates. — `garden::tests::a_tree_is_pads_and_not_a_stack_of_plates`
- Every pad hangs off a limb that starts on the trunk. — `garden::tests::every_pad_hangs_off_the_trunk`

## Verified by hand

- The room reads as a garden and not as a room with plants in it.
- The pond reads as having depth.
- The koi are the reason you stand and watch it.
