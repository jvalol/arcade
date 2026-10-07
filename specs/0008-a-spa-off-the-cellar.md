# 0008 A spa off the cellar

**Status:** implemented
**Date:** 2026-10-06

## Goal

A room with water in it: a pool you can walk into, a hot tub, and a sauna. The
building has had nothing you could be in rather than look at, and water is the
first thing in it that answers back.

## What it is

**Where.** Beside the cellar, through a door cut in one of its long walls. Not
the far wall, which is the fireplace: a door through a chimney breast is a door
into a fire, and the first layout of this room had one. The cellar is already the warm, lamplit end of the building and the
way down is already built, so the spa costs no second descent. Walking it is
hall, nook, bookcase, stair, cellar, spa, and each of those is a room you have a
reason to cross.

It is one room holding three things, not three rooms. A pool, a tub beside it,
and the sauna as a timber box standing in the corner with its own door.

**The floor** is tiled rather than boarded, and wet underfoot near the water.
The cellar's mahogany stops at the door: panelling belongs in a library and
would not last a week in here, which is the kind of reason a room should be
able to give for what it is made of.

**The pool** is sunk into the floor, six by four and a little over a unit deep,
with steps down at the near end. It holds engine water, per spec 0043 of
blitzkit: a heightfield that waves cross and that answers what goes into it.
The surface is one mesh written over every frame, per spec 0042.

**Walking into it.** The pool is not a wall. The floor of the room is solid,
the basin's floor is solid, and the lip between them is a step like any other,
so walking in is walking down. In the water you are slowed, your eye drops with
the floor under you, and you push the surface as you go: the ripples follow you
about, which is the whole of what this room is for.

The wake is a ring round your own body and not a disc under it, per spec 0043's
`ring`. The water you are standing in is where you are, so a disc centred on you
spends the wake holding down the one patch of surface you can never watch move.
It is sized from your own radius rather than from a number of its own, and it is
strong enough to see: the measure is the share of the pool's depth a ball makes
in the engine's own example, which is about a seventh.

Out of your depth is not swimming. The basin is shallow enough to stand in
everywhere, because a spec that says swimming is a spec about a camera and a
stroke and a surface you can be under, and this one is about water.

**The hot tub** is square, cedar, raised above the floor rather than sunk into
it, and its own smaller body of water. Square because a heightfield is a
rectangle, and a round shell round one is a lie at four corners: either the
water runs out through the staves or a rectangle of water floats inside the tub.
It is bubbling, which is a push in the middle of it every so often rather than
anything simulated, and it steams.

The bubbling lifts. A jet is air on its way out and it carries water with it, so
the middle of a hot tub stands above its own rim. A disc and not a ring, unlike
the wake, because nothing sits in the middle of a plume.

**The sauna** is timber: a box in the corner with a glass door, two tiers of
bench inside, and a stove. The door is a real door, hinged on one edge of the
opening and swung by clicking it, the way the bookcase is, and lighter: it eases
in 0.8 against a bookcase's 1.3. Shut behind you, the rest of the spa is still
there and is on the other side of something, which is most of what makes the
sauna read as a room rather than a recess. What makes it
read is that it is lit differently from the room, close and orange, and that
you can be inside it with the door between you and the rest.

**Steam** rises over the tub and over the stove: soft translucent things that
drift outwards, fade in over the first fifth of their life and out over the
rest, so nothing appears or vanishes. A pure function of the clock, so there is
nothing kept and nothing to reset. Not particles: the engine has no particle
system and eight puffs do not need one.

## Acceptance criteria

- The door is in a long wall of the cellar, clear of the fireplace. — `spa::tests::the_way_through_is_in_a_long_wall`
- You can walk from the cellar into the spa. — `spa::tests::you_can_walk_in_from_the_cellar`
- The pool is sunk into the floor, not standing on it. — `spa::tests::the_pool_is_sunk_into_the_floor`
- Its steps are a flight you can climb, by the same rule the stair is. — `spa::tests::the_steps_are_steps_you_can_take`
- You can stand anywhere in the basin. — `spa::tests::you_can_stand_up_anywhere_in_it`
- Nothing in the room stands inside the pool, the tub or the sauna. — `spa::tests::nothing_stands_where_the_water_is`
- The tub holds its own water, clear of the pool's. — `spa::tests::the_tub_is_its_own_pool`
- The sauna is a room with a way in. — `spa::tests::the_sauna_has_a_door_you_fit_through`
- Its benches are under its ceiling and clear of its stove. — `spa::tests::the_benches_fit_in_the_sauna`
- Every lamp is inside the room it lights. — `spa::tests::every_lamp_is_in_the_room`
- Water slows you and the water knows you are in it. — `spa::tests::wading_is_slower_than_walking`
- Your eye goes down with the floor under you. — `spa::tests::your_eye_drops_as_you_wade`
- The sauna's glass fills its doorway when it is shut. — `spa::tests::the_glass_fills_the_doorway_when_it_is_shut`
- And swings clear of it, hinged rather than slid, when it is open. — `spa::tests::the_glass_swings_out_of_the_way`
- Steam rises, drifts, fades at both ends, and is the same at the same moment. — `spa::tests::the_steam_rises_and_fades`

### Verified by hand

Run the arcade and walk down.

- The surface moves, and the ripples follow you about rather than arriving on a
  timer.
- The tub steams and the sauna is a different warmth from the room.
- Nothing is drawn in two places at once where the basin meets the floor.

## Out of scope

**Swimming.** You can stand everywhere. Being out of your depth is a camera
problem and a stroke and a surface you can be under, which is a different spec.

**Getting wet.** Nothing changes because you were in the water. No footprints,
no dripping, no towel.

**Heat as a number.** The sauna is warm because of how it is lit and what it is
made of. Nothing in the building has a temperature.

**Water that goes anywhere.** Per spec 0043 of the engine, the surface moves up
and down and that is all. Nothing splashes out and nothing drains.
