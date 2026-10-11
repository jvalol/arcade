# 0014 Two faces in one plane

**Status:** implemented
**Date:** 2026-10-10

## Goal

Seven faults in this building have been two surfaces at the same depth:
stripes round the pool, a chequerboard down a wall of the baths, a border sunk
into a floor, a sign on panelling, the sky in patches, the hall's floor over
the garden, the underside of a neon can. Each was found by walking until you
stood somewhere that showed it, and each took several rounds because the
picture comes back before the place does.

Spec 0048 of the engine does the arithmetic over the frame. This puts it on a
key, in the building the arithmetic was written for.

## Behavior

**P asks for one check.** The engine compares every drawn box in the frame
with every other, writes the twenty biggest in the log, and hands back the
patches. The patches go up as wireframes in a hot colour, against spec 0013's
cool one, so the two debug lists are never read as one.

**A press and not a toggle.** There is nothing to turn off: the answer does
not change between frames, so running it again gives the same list. The
patches stay up until you ask again.

The repeats of a held key ask for nothing. A check over every drawn thing
against every other, taken off the repeat rate, is a building that stops while
you lean on a key.

**Off when the building opens**, like spec 0013's, and not mentioned in the
line at the top of the screen.

**It hands the engine a body**, and the engine fills out from where you are
standing to find where that body could be, per spec 0049 of the engine. Then it
throws out every pair nobody can get in front of. The body is this game's own:
the radius `walk` slides, a step of a quarter metre, and four metres of air over
a place to stand.

**It reports 2 pairs out of 221**, over 5215 drawn boxes, with 138 instances
turned off the axes and 814 whose shape is not a box, neither measured, and
66348 cells of a quarter metre a body can be in. Both of the two are at the
hall's ceiling, which is somewhere you can stand and look up.

**Shut doors mean rooms it says nothing about.** The way down is a secret door
and the way into the baths is another, so when the building opens the cellar and
the baths are not places a body can be and nothing in them is reported. Press
the key down there instead. This is right rather than a flaw, and it is a thing
to know before reading a short report as good news.

## Acceptance criteria

- P asks for a check, and the building opens not asking. — `tests::p_asks_for_one_coplanar_check`
- The repeats of a held P ask for nothing. — `tests::the_repeats_of_a_held_p_ask_for_nothing`
- Nothing else asks for it. — `tests::no_other_key_asks_for_the_check`
- The check catches this building's own worst case, the hall's floor and the room behind it in one plane over 5.40 by 3.00, passes the version that ships, and the filter does not throw the catch away. — `behind::tests::the_engines_check_catches_the_lap_and_passes_what_ships`
- The fill gets to the cellar and the baths once their doors are open, and to neither while they are shut. — `tests::the_fill_gets_to_the_rooms_that_are_open`

### Verified by hand

- Press P in the hall and the log comes back with the count and the twenty
  biggest patches.
- The patches stand where the surfaces are, in magenta against spec 0013's
  cyan.

## Out of scope

**Deciding which of them matter.** The fill says a body can be in front of a
face. It does not say anything is looking that way, or that a bookcase is not
in between. Walking up to the two it reports is still the last step.

**Anything that is not a box.** Three quarters of what this building draws is
refused, because a barrel's box is not a barrel's faces. Two turned things
flush with each other is a fault this will not find.

It is also why three of the baths' piers are in the report. Their back faces
are flat against a wall drawn as a tiled plane, which is subdivided, so it is
not box shaped, so it is not in the list, so nothing buries them. Spec 0048
says why that is left alone.

**Fixing them.** It says where to look.
