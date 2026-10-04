# 0002 What is on the far wall

**Status:** draft
**Date:** 2026-10-03

## Goal

The engine's own demonstrations, running at the end of the room, turning.

## Why

Spec 0001 gave every game a cabinet. The engine also carries examples, and they
are not games: nobody wins a Klein bottle. A cabinet says play me and these are
things to watch, so they want somewhere else and something else to be.

They can genuinely run rather than be photographs of themselves, because
`blitzkit/shapes` is already a library and already exposes exactly these: the
teapot, the Klein bottle, the Sierpinski tetrahedron, the Menger sponge and the
Hilbert curve. The arcade builds the same meshes the examples build, from the
same code, so nothing is copied and nothing can drift.

## Behavior

**Five objects along the far wall**, each turning slowly on a plinth, built from
`blitzkit-shapes` rather than drawn from a picture.

**Named when you look at one**, as a cabinet is, so the room reads one way
throughout.

**They do not start anything.** A cabinet plays a game; these are a
demonstration and there is nothing to play. Looking at one says what it is and
nothing else.

**The other five examples are not here.** cubes, rolling, stacking, tower and
tunnel are scenes with physics and input rather than shapes, and putting those
on a wall needs every example to become a library with its own render target,
which spec 0001 ruled out and this does not change.

## Acceptance criteria

- There is a display for each shape the engine's library carries. — `display::tests::every_shape_is_on_the_wall`
- They stand along the far wall, clear of each other and of the cabinets. — `display::tests::they_stand_clear`
- Each one turns. — `display::tests::they_turn`
- Looking at one names it, and names a cabinet when you look at a cabinet. — `room::tests::looking_at_a_display_names_it`

### Verified by hand

- Walking to the end of the room and watching them turn.

## Out of scope

Playing anything from the wall. The five examples that are scenes rather than
shapes. Choosing how deep a recursive solid goes.
