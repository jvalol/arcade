# 0022 A room at the end of the aisle

**Status:** implemented
**Built:** 2026-10-16
**Supersedes nothing.** Spec 0002 put the shapes on the far wall and this
moves the wall.

## Goal

Give the engine's own shapes a room to stand in, instead of the strip of end
wall they have been squeezed onto.

## Why the strip ran out

**Because what bounds it is where the cabinets stand, and nothing said so.**
The row sits on the hall's far wall between the two rows of cabinets, whose
inner faces are at `room::WALL - room::CABINET.x * 0.5` either side. At a
`WALL` of 2.0 that gap is 3.30. Five shapes fitted it with room to spare. Six
do not: at 0.59 apart they are 3.29 of row and clear the gap by half a
centimetre at each end, which Jake read off the screen as the plinths growing
and wanting a wider berth.

**And the hall cannot be widened to pay for it.** That was tried first and it
is the wrong variable. The aisle is 4.0 across and `room::REACH`, how far the
sight carries, is also 4.0, and spec 0003 chose that so a thing comes alive as
you reach it rather than when you merely aim at it. Widening the room by as
little as 0.2 breaks the one thing that pairing buys, which is being able to
work the cabinet opposite you from where you stand. Measured rather than
argued: `room::tests::the_one_you_are_looking_at_is_the_one` fails at every
width from 2.2 up.

Jake's answer was the one I had not considered: expand the wall into a larger
room. A room at the end of the aisle touches the aisle not at all. The
cabinets stay where they are, the sight carries exactly as far as it did, and
the row is bounded by the new room instead of by the old one's furniture.

## What it is

A room across the end of the hall, wider than the aisle and deep enough to
stand back in, with the shapes along its far wall.

**Wider than the aisle on purpose.** Walking down the aisle between two rows
of cabinets and out into something broader is the whole of what this room is
for: it says the shapes are the thing at the end rather than more of the same
wall. The hall's far wall becomes an opening the width of the aisle, so the
room is entered through a gap rather than by the wall simply stopping.

**Deep enough to see the row whole.** The row is a little over five metres
across, and a person standing a metre from it sees two shapes. The depth is
whatever puts the whole row in view from the back of the room.

## What it is not

**Not the nook.** That is spec 0006, it is off the left wall, it has benches
and bookcases and toys in it, and it stays exactly as it is. This room has
one thing in it.

**Not a corridor.** If it comes out narrow and deep it is the aisle again
with a kink in it.

## What it came out as

6.8 wide by 6.3 deep, which is nearly square and is the row's doing twice
over. The width is six shapes at 0.95 apart, 5.30 of row, with three quarters
of a metre clear at each end. The depth is derived rather than chosen:
`room::IN_FRAME` says how much wall is in the window per unit you stand back,
measured off a picture at 3.65 back with 4.2 of row in frame, and the depth is
what that needs plus room to stand in behind it. The first guess was 4.2 and
cut both ends off the row.

The shapes are 39 per cent bigger than the squeezed six and 32 per cent
bigger than the five ever were.

Three downlights over the row rather than one over the middle of the floor, so
a shape is lit from in front and above and throws its shadow onto the wall
behind it. Each has a shade and a stem to the ceiling, because a lamp with no
fitting is a bright patch on a ceiling with nothing making it.

The arcade's own carpet, carried through the way in. It had no floor at all on
its first build, which from inside is a room standing on black.

## Acceptance criteria

- The shapes fit along its far wall with real clearance at each end, derived
  from the room rather than written down. — `display::tests::the_row_fits_between_the_end_cabinets`
- Nothing in the building stands in it except the room and what holds the
  shapes up, and the shapes really are in it. — `room::tests::nothing_stands_in_the_gallery`
- You cannot walk through any of it. — `room::tests::you_cannot_walk_through_anything`
- The sight still reaches across the aisle, which is the thing widening the
  hall would have cost. — `room::tests::the_one_you_are_looking_at_is_the_one`

Two counts of the building's solid boxes had the number of floors written into
them as `+ 2`, and a third slab made both wrong. They count `floors().len()`
now, which is the only reason anything noticed.

## Verified by hand

`--screenshot=shapes` stands just inside the way in and looks down the room,
which is the furthest back it goes and the one place the whole row is in
frame. Its own slug because the hall's pose is worked out from a cabinet and
the shapes are through a doorway from any cabinet.

- Walking out of the aisle into it reads as arriving somewhere rather than as
  the room getting wider.
- The whole row is in view from the back of it.
