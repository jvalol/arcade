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

**It reports 221 pairs over 5214 boxes**, with 139 instances turned off the
axes and 770 whose shape is not a box, neither measured. That is not a list to
read, and it is not meant to be: the patches you cannot get to never come on
screen, so what you see when you press it is the ones in the room you are
standing in.

## Acceptance criteria

- P asks for a check, and the building opens not asking. — `tests::p_asks_for_one_coplanar_check`
- The repeats of a held P ask for nothing. — `tests::the_repeats_of_a_held_p_ask_for_nothing`
- Nothing else asks for it. — `tests::no_other_key_asks_for_the_check`
- The check catches this building's own worst case, the hall's floor and the room behind it in one plane over 5.40 by 3.00, and passes the version that ships. — `behind::tests::the_engines_check_catches_the_lap_and_passes_what_ships`

### Verified by hand

- Press P in the hall and the log comes back with the count and the twenty
  biggest patches.
- The patches stand where the surfaces are, in magenta against spec 0013's
  cyan.

## Out of scope

**Deciding which of them matter.** The check knows whether a face looks into a
room or out at the weather. It does not know whether anybody can stand
anywhere that sees it, so the ceiling's ribs are in the list. Spec 0048 says
what answering that would take.

**Anything that is not a box.** Three quarters of what this building draws is
refused, because a barrel's box is not a barrel's faces. Two turned things
flush with each other is a fault this will not find.

It is also why three of the baths' piers are in the report. Their back faces
are flat against a wall drawn as a tiled plane, which is subdivided, so it is
not box shaped, so it is not in the list, so nothing buries them. Spec 0048
says why that is left alone.

**Fixing them.** It says where to look.
