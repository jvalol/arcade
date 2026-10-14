# 0020 A track in the carpet

**Status:** implemented
**Date:** 2026-10-13

## Goal

Give the hall one hint that the way through the near wall is there, and make
it a thing that happened rather than a thing somebody put up.

Spec 0011 built that way in with nothing at all pointing at it, and said so in
as many words. What is behind it now is spec 0015's orrery and spec 0018's
thirty-three kilometres of solar system, which is most of what has been built
in this building and the part nobody finds.

## Behavior

**It is wear in the carpet.** A lane worn across the near bay, from the middle
of the aisle to the foot of the wall at the hole in it. Nothing is written,
nothing glows, nothing is hung up. Everything else in this hall is a thing
somebody installed. This is the one thing that is a trace of somebody else
having been here, which is why it can say where to go without saying anything.

**It is on the floor because the floor is where it can be seen.** The hole is
in the wall you wake with your back to and nobody looks at that wall, so a
hint at the hole only reaches somebody already standing at it. What everybody
does do is walk back up the aisle, and that walk is the whole of this hint's
chance. It is 3.2 metres long for that reason and the length is the dial on
it: longer is more of the walk with the lane in front of you.

**A lane, not a patch.** It leaves the middle of the aisle, swings across and
arrives at the hole square on, which is the shape of somebody turning into a
doorway. It was a quadratic curve first and a quadratic moves most of the way
across in its first third, so by the time the wear had come up out of nothing
the lane was already over at the wall and the thing read as a rug laid at the
skirting. A cubic with a control point out at each end holds the lane straight
up the aisle, turns in the middle and arrives face on.

**It comes up out of nothing over its first three tenths** and is at full for
the rest. Wear is heaviest where the traffic converges, so it has to begin
somewhere, and a lane with a beginning you can see is a painted line.

**Dark grey at a third of an alpha, and that is both halves of a mistake.**
Wear is flattened pile and flattened pile is paler than standing pile, so the
first reading of it was a pale wash. The hardware blends in linear light and
this carpet's ground is five thousandths of full once it is linear, so a pale
wash at any alpha worth having comes out twenty times the floor it is lying
on: a searchlight down the aisle.

The second reading was dark enough not to do that and then invisible. Two
frames of the same bay with the wear on and off, differenced, came back four
code values of 255 apart at the middle of the lane. A doubling in linear light
is nothing on a screen. What it is set by now is that difference: eighteen of
255, measured the same way, which is a floor plainly a different tone where
the feet have been and nowhere near a lamp.

The alpha does the other half. A third of it takes the carpet's brightest
threads down a quarter, so the confetti goes quiet inside the lane rather than
being painted over, which is what worn carpet actually looks like.

**Nothing drawn has a straight edge but the one at the wall**, and that one is
straight because the wall is standing on it. The band fades out well inside
the quad it is painted on, and the quad carries a hand's width past the wall's
inner face so the picture's own repeat seam is buried in the wall rather than
showing as a hair of half strength wear along the skirting.

**Two millimetres above the carpet**, the hair the stars out past the door lie
above their own floor. In the carpet's own plane it would be two exposed faces
sharing one, which is this building's commonest fault.

**It does not show from the other side.** The quad stops inside the wall, so
the dark room's floor has none of it. There is no carpet through there to be
worn and no reason for a patch of one.

## Acceptance criteria

- The wall end of the track is in the hole, and the aisle end is out of it. — `behind::tests::the_worn_track_ends_at_the_hole`
- The wear fades out inside its own quad on every side but the wall's. — `behind::tests::the_worn_track_fades_out_before_the_edge_of_its_quad`
- All of it lies on the hall's own carpet and none past the wall. — `behind::tests::the_worn_track_lies_on_the_halls_carpet`
- The curve stays inside the box its two ends make, which is what lets the two ends speak for the whole of it. — `carpet::tests::the_wear_keeps_between_its_two_ends`
- The wear is heaviest at the wall, nothing where it leaves the aisle, and gone a stride to either side. — `carpet::tests::the_wear_is_heaviest_at_the_wall_and_gone_a_stride_off_it`
- There is none of it at the edge of its picture. — `carpet::tests::the_wear_runs_out_before_the_edge_of_its_picture`
- It lifts the carpet far enough to see and not far enough to read as a light, counted in code values rather than in linear light. — `aim::tests::the_worn_track_can_be_seen_and_is_not_a_spotlight`

### Verified by hand

- It reads as wear and not as a spill, a rug or a lamp. Walk up the aisle
  towards the near wall and look at the floor in front of the wall.
- From the far end of the hall it is a quieter patch and no more, and it gets
  plainer as you come up the aisle.
- Nothing of it shows from inside the dark room.

## Out of scope

**No second hint.** One is the whole of it. A hint that is backed up by
another hint is a sign.

**Nothing on the wall.** No mark, no draught, no sound through it. The floor
is where this lives, and the wall stays what spec 0011 made it.

**No wear anywhere else in the building.** Every other room would have some,
and once it is everywhere it is a texture rather than a trace. This is the one
place where it is carrying something.
