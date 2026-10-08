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

**There are koi in it.** Three or four, slow, each going its own way round, and
each one pushing the surface as it passes. They are the reason the pond is not
a mirror and they are the only moving things in the room.

Not built yet. Everything else in this spec is, and the pond is water with
nothing in it until they are.

**Trees**, turned on the lathe like everything else round in this building:
a trunk that tapers and leans, and a crown over it. Not a cone. A cone is a fir
and a fir is not what anybody draws in a zen garden; this wants the flat, wide,
layered crown of a pine or a maple, which is two or three discs of foliage at
different heights and widths.

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

It is wound to be seen from inside, which is the opposite of everything else
this building turns. Made two sided instead, every triangle had a twin a hair
away facing the other way, and the sky came out in patches of cloud and patches
of unlit dark: two surfaces in one plane, which is the fault this building has
now fought five times.

**The hall's wall is a metre shorter than the garden's**, so the garden carries
a parapet over it, in two pieces with the way in between them. In one piece it
crossed the opening, which put a beam back over the entrance the moment the
lintel came out of it, and from the garden that beam's underside was a ledge
hanging over the way in. Without one there was a metre of nothing above that wall and
over it you were looking at the roof of the hall from outside: a dark slab
hanging in the garden's sky with the lanterns catching its underside.

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

## Verified by hand

- The room reads as a garden and not as a room with plants in it.
- The pond reads as having depth.
- The koi are the reason you stand and watch it.
