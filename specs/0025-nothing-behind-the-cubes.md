# 0025 Nothing behind the cubes

**Status:** implemented
**Built:** 2026-10-18
**Date:** 2026-10-18

## What it is, plainly

You come through the unmarked door into the back room. The near half is what
it was: a bench along the side wall, buckets, a cable reel, tins, crates, one
bulb hanging over the floor.

The far end is a bay, and the bay is black. Black back wall, black sides,
black floor, black ceiling. Standing in it is a frame holding a block of
white cubes as wide as your arms, lit by one lamp at the lip and by nothing
else. Look at it and press a key and there are more cubes, press another and
there are fewer. The gauges hang on the black above the block.

Walk toward it and the room behind you stops mattering, which is the whole
point of the change.

## Goal

The engine's own `load` example is the best looking thing in this project.
Jake's word for it is stunning. The same cubes in the back room are not, and
his reading after they were already made white: the example is more stunning
than the closet.

He is right, and the reason is composition rather than colour. `load` is a
black window with one bright thing filling it. The back room is a lit grey
room in which the block is 62 centimetres on a bench, seen from two metres,
against a card the size of a tea tray, with walls as bright as the cubes
either side of it. White cubes were necessary and were never going to be
enough.

So the demonstration gets the three things the example has for nothing:

1. It fills your view.
2. There is nothing behind it.
3. It is the only lit thing.

## What changes

**The block gets big.** 0.62 across becomes 1.2. At the working spot that is
about forty degrees of view, which is roughly what the block fills in the
example's window. The cube count and its dial do not change: still four,
eight, twelve, sixteen and twenty four a side, still cube numbers, so each
cube is 1.2 divided by the side and the block stays solid.

**The card becomes a bay.** The far 1.6 metres of the room, floor to ceiling
and wall to wall, is lined in the colour the card already uses, which is as
near black as this building draws. 1.6 because the block is 1.2 deep as well
as wide and has to stand inside the lining with room behind it and in front:
anything shallower and the cubes hang out into the grey.

**The rig stands on the floor.** The block is held in a frame in the middle
of the bay with its middle at 1.2, so it is in front of you rather than over
a bench. The bench moves to the east wall and keeps the clutter company. It
is furniture after this: the dials are worked by looking at the cubes, which
is already how the sight finds the rig.

**The lamp moves into the bay and the room's own bulb is pulled back.** One
lamp at the lip, over the opening, with a range that dies before it reaches
the near half. The room's bulb keeps the door and the clutter visible and is
shortened so it does not wash the lining grey. Every lamp in this building
sits in a fitting, per the rule the cellar and the baths both broke first,
and this one is no exception.

**The gauge board hangs on the black.** Same board, same words, same bars,
above the block. On a black wall under no light of its own, what you see is
the words and the three bars, which is what it is for.

**The building's ambient is not touched.** `scene.light.ambient` is one
number for the whole scene and the door is open: dimming it for this room
dims the gallery through the doorway. A black material under the ambient the
building already has is already black. The darkness here is paint and
falloff, not a global.

## What it is not

**Not a theatre.** You stand, you work a dial, you watch a number move. The
room where you sit down and something plays is still its own idea.

**Not a new instrument.** Nothing is measured here that spec 0023 does not
measure already, and nothing in this room starts a process.

## Acceptance criteria

- The block stands inside the bay with clearance on all six sides, at every
  step of the dial. — `back::tests::the_block_stands_clear_inside_the_bay`
- Nothing grey is behind the cubes: a ray from the eye at the working spot
  through each corner of the block lands on the bay's lining, not past
  it. — `back::tests::nothing_grey_is_behind_the_cubes`
- From the working spot the block takes at least forty degrees of the
  view. — `back::tests::the_block_fills_the_view_from_where_you_work`
- At the block and at the lining behind it, the bay's lamp gives four times
  what the room's bulb does, and the room's bulb still reaches the
  door. — `back::tests::the_rig_is_lit_by_its_own_lamp`
- The bay's lamp is inside its fitting and inside the
  bay. — `back::tests::the_bay_lamp_is_in_its_fitting`
- The bench, the clutter and the door's swing are all out of the
  bay. — `back::tests::the_bay_holds_nothing_but_the_rig`
- You can walk in from the gallery to the working spot and back out
  again. — `back::tests::you_can_walk_in_and_out_again`
- There is room to stand at the rig, clear of it and out of the
  dark. — `back::tests::there_is_room_to_stand_at_the_rig`
- Nothing in the clutter stands on the way from the door to the
  rig. — `back::tests::the_clutter_is_against_the_walls`
- Pointing at the cubes finds the rig; pointing at the workbench, which is
  the thing in here that most looks like a bench, does
  not. — `back::tests::pointing_at_the_cubes_finds_the_rig`
- Every word on the gauge board is still on the
  board. — `back::tests::every_word_on_the_board_is_on_the_board`

### Verified by hand

- Whether it is stunning, which is the only reason this spec exists and the
  one thing no test here can answer.
- Whether the lining reads as a space you could step into or as a black
  rectangle painted on a wall. If it reads flat, the lining is lit and the
  lamp's range is the first thing to cut.
- Whether the room is now too dark to find the door from the bay.
- Whether the gauge board floating on black reads as part of the rig or as
  something hanging in space.

## What it took that the spec did not say

Three things the writing missed and the building found.

**A bench record draws a bench.** The rig is one of the building's benches,
which is what gives it a name under the sight and arrows that work it, and
the code that draws benches drew it: a wooden table top hanging in the air at
the top of the block, 1.5 across, lit. The pool table in the cellar already
had its own branch for the same reason. The rig has one now too, which is a
`continue`.

**The lamp arithmetic had to move twice.** The room's bulb at its old range
washed the lining grey, and short enough to leave the lining alone was short
enough to leave the door dark. What settled it was measuring both lamps with
the engine's own falloff at the block, at the lining and at the door, rather
than reasoning about distances.

**The clutter was all in the dark.** Every piece of it was along the far wall
or the east wall, which are the bay and the bench now. The broom then stood
in the walking line, which the clutter test caught, and the ladder stood
inside the broom, which it caught next.

## Out of scope

**A spotlight.** The engine has `SpotLight` with a cone and allows four, and
this building has never used one. A point lamp at the lip with a short range
is the simpler thing and is what this specifies. If the lining comes out grey
anyway, the spot is the next move, and it is an arcade change rather than an
engine one.

**Render to texture.** Still the thing that would turn the back wall into a
screen, still not needed: the cubes in here are real cubes, and a block of
real cubes cannot lie about what it costs to draw.

**The example itself.** `load` is not changed by any of this. It is the
reference, and the room is what has to come up to it.

**Making the rest of the building darker.** One room gets a dark end. The
hall, the gallery and the cellar are lit the way they are for reasons of
their own.
