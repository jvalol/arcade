# 0056 Valhalla

**Status:** implemented
**Date:** 2026-11-04

## Goal

What is through the door spec 0055's watchman keeps. The end of the trail,
of the path and of the game: somewhere peaceful after a journey, and the
last thing anybody sees of this building.

Spec 0055 settled what it is for and what it looks like, by talking it
through with Jake, and that section is the brief this one builds to. In
short: a Norse hall and its steading, warm light with its source out of
frame, a touch of neon, a few people and nobody fighting, and rest as the
reward rather than instead of one.

## Where it stands

**On its own ground at the bridge's height**, running west from the
landing's far parapet. The bridge climbs 26 metres over 100 and stops
level; this begins where it stops, so stepping through is stepping across
and not up.

**Thirty-five metres of field is left west of the landing** before the
ground runs out at `field::GROUND_REACH`, and this takes most of it. It
does not need ground under it, being 26 metres up, but staying inside the
field's own reach keeps it inside the sky as well, and a place that pokes
out through the sky is not a place.

**Whether it reads as a plateau or as an island is Jake's to say** from a
picture. It is built as its own ground either way.

## What is on it

**A longhouse, and outbuildings round it.** The myth gives one great hall
and the brief asks for somewhere village-like. A Norse settlement is both:
a longhouse with smaller buildings about it. The hall faces the bridge, so
what you see on arriving is the door.

**Light out of the openings and not off the walls.** The whole of the
brief's warmth is that the source is out of frame. Doors and windows are
lit; the walls take what falls on them.

**A line of neon along the ridge.** One, and wordless. `aim::NEON` is 3.2
in cyan and magenta and it is what the hall's own sign is made of, so the
building turns up in its own heaven. Anything with words on it is copy and
is Jake's.

**A few figures, none of them fighting.** Spec 0033 draws people and spec
0038's whole room is other people using it. Somewhere peaceful is not empty
and it is not a barracks.

## What this build is not

**The aurora.** Spec 0055 records what it will cost and it is more than was
first claimed: the weather moves by sliding the sky's own picture, so
anything painted into it drifts, and there is one sky for the whole
building, so an aurora here is an aurora over the zen garden. Worth doing
and worth deciding on purpose, and not in the first build of a place.

**The riddle.** Spec 0055 leaves what Heimdall asks open, and a gate on a
place nobody can reach without finishing the building is a second lock on a
locked door. The place first.

**Anything past it.** Spec 0055: a thing beyond Valhalla turns Valhalla
into a corridor.

## Acceptance criteria

- None of it is there until the bridge is. — `valhalla::tests::nothing_until_the_bridge_is_there`
- Its ground is flush with the top of the deck and the landing. — `valhalla::tests::the_ground_is_flush_with_the_landing`
- You can walk from the landing onto it. — `valhalla::tests::walked_from_the_landing_into_valhalla`
- It stays inside the ground the field draws, and so inside the sky. — `valhalla::tests::it_stays_inside_the_field`
- The hall faces the bridge. — `valhalla::tests::the_hall_faces_the_way_you_come_in`
- Nothing stands in the way in. — `valhalla::tests::nothing_stands_in_the_way_in`
- No two buildings overlap. — `valhalla::tests::no_two_buildings_share_a_space`
- Every lit opening is on a wall of a building. — `valhalla::tests::every_light_is_in_an_opening`
- Every figure stands on the ground and not in a wall. — `valhalla::tests::everybody_stands_on_the_ground`
- And the landing's far parapet has a way through it. — `bridge::tests::the_far_parapet_has_a_way_through_it`

### Verified by hand

- **Stood on the landing and looked through**, with the trail filled by
  `--done`. The hall faces you with its door lit, the outbuildings stand
  off the lane either side with their windows lit, and the light lands on
  the walls and on the people. It is a blocking model and reads as one.
- **The door and the lane are done, and both took the same lesson twice
  over.** The door was a flat white-gold slab: the panel glowed at 3.4 on
  its own and then took the lamp outside as well, and a surface past one
  that is also lit goes white. It is dimmer now and set back behind a dark
  reveal, so it reads as a hole with a room behind it rather than as a
  panel painted on a wall.

  The lane was pale because the ground was. At 0.22 it came out grey under
  the field's dusk, so a warm pool landed on something already bright and
  read as nothing, which is the garden's bark again. The ground is 0.085
  now and takes a broad highlight, and what the door throws runs back along
  it wet.

  And the lamp is five metres out from the face, which looks wrong written
  down. Close in it shone back into the opening it was meant to be coming
  out of, and lit the wall harder than the ground, which washed a daubed
  wall to cream. Out there the wall is five metres from it and the lane is
  one.

- **The hall still shows you its gable end**, which is nine metres wide, so
  it reads as a shed rather than as something long. Not touched.
- **The neon along the ridge is not visible** from the ground in front of
  the hall, being on top of the roof. It wants to be on the gable or over
  the door.
