# 0043 The building's floors as a set

**Status:** implemented
**Date:** 2026-10-28

## Goal

Ask one question of every floor in this building at once, rather than of one
pair of floors in one room's tests. Where one room's ground stops and the next
one's starts is a question nobody owned, and the rooms that happen to have a
test for it each wrote their own.

## Behavior

`Room::grounds` gives every floor in the building, said once each, with the
name of the room it belongs to. It sits in the same impl as `built`, because
`built` is the one place that knows what rooms there are: a room added to one
and not the other is a room whose ground nothing holds. Nothing can test for
the absence of something nobody wrote down, so the two being a screen apart is
the whole of the defence.

Each entry is the floor as the walk meets it, taken from the module that owns
the room. A room whose floor is in pieces gives its pieces, because what the
rules care about is where one room's ground ends, and a piece's edge is as real
as a whole slab's. The baths give four slabs round the pool and the imaginarium
gives the pieces left round the stair's slot.

The field is not in the set. `field::ground` re-centres on a tile grid around
wherever you stand, so asked from two places it gives two boxes, and a set that
changes under you is not a set.

Three rules read it.

**No hole between two rooms.** Where two floors at one storey face each other
across a strip narrower than a body, something has to floor that strip. Not the
pair: a third floor lying under the strip is a perfectly good answer, and the
gallery's floor stopping two tenths short of the other corridor's is exactly
that, with the lobby's running under both. The whole length of the seam is
swept at a step of `RADIUS`, because the first hole this rule found was in a
doorway and the middle of that seam is a wall.

**No ledge between two rooms.** Where two floors touch or lap in plan you may
be walking from one onto the other, so the step between them is at most
`walk::STEP`. Three rooms here stand over others and are not ledges at all: the
imaginarium six above the hall, the cellar three under the lobby, the baths
three under the garden. A floor more than a head above you is a ceiling and
cannot trip you, so the rule allows that too and bites in the band between.
That band is empty in this building today.

**Every ground is a box the building serves.** A ground nothing serves is a
ground the walk never meets, and a rule about it is a rule about nothing. This
one is here because the set got it wrong on the first build: the cellar has two
things it calls a floor, the clear floor of the room and the slab, they differ
by half a wall all round, and the set asked for the first.

### What it found

The cellar's doorway to the baths had no floor across the whole of its width.
`spa::near` is the shared wall's face and the cellar's slab stops at the wall's
middle, so half a wall's width had nothing under it. A body is wide enough to
bridge it, so nobody was ever going to fall in; it is the same fault the
garden's door had in spec 0010 and it is fixed the same way, by running the
baths' floor south under the wall to meet the cellar's slab.

## Acceptance criteria

- No two rooms leave a hole between their floors. — `room::tests::no_two_rooms_leave_a_hole_between_their_floors`
- No two rooms meet at a ledge. — `room::tests::no_two_rooms_meet_at_a_ledge`
- Every ground in the set is a box the building serves. — `room::tests::every_ground_is_a_box_the_building_serves`
- A floor and the hall's leave no hole and no ledge, in the room that used to own this question. — `behind::tests::the_floors_leave_no_hole_and_no_ledge`
- And in the back room. — `back::tests::the_floors_leave_no_hole_and_no_ledge`

## Out of scope

**A hole inside one room.** Two pieces of one floor are not asked about each
other, because the imaginarium's stair slot is a hole on purpose with a stair
in it. A room's own tests hold its own holes.

**Something that is not a floor poking up through one.** That is the fault the
baths' roof had against the garden's gravel, and it is one room's lid against
another room's ground rather than two grounds against each other.
`garden::tests::the_garden_sits_on_the_roof_of_the_baths` holds it, for the one
pair of stacked rooms where it has ever happened.

**A face at the walking surface.** Gone, and not because of anything here.
Spec 0007's step-over takes a face of no height in its stride, and
`behind::tests::the_floors_leave_no_hole_and_no_ledge` carries the measurement
that retired it.

**Whether you can get there.** A floor with no hole and no ledge can still be
walled off from everything else. `tests::the_fill_gets_to_the_rooms_that_are_open`
is the one that asks that, and it asks it of three rooms rather than all of
them.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
