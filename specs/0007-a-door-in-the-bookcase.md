# 0007 A door in the bookcase

**Status:** draft
**Date:** 2026-10-05

## Goal

One book in the nook's wall of books that is not a book. Pull it and the case it
stands in swings off the wall, and behind it is a stair going down to a cellar
with a pool table in it.

The nook answered a question the room had: where does a thing go that is neither
a game nor a shape. This answers a different one. Spec 0005 said the arcade
should be a room you want to be in rather than a menu with walls, and the nook
is now the room in the building that is most worth standing in, which is all the
more reason it should not be the end of it. A building you can find the end of
is a menu again.

A cellar is also the right home for one particular game. poolhall is a game and
there is something to win, so by the hall's own rule it belongs in a cabinet out
there. But a cabinet with pool on the screen is the wrong object: pool is a
table, and the thing you want is to walk up to the table. cascada showed the
shape of this. A bench that opens a window is a toy too big for a bench, and a
pool table that opens a window is a game too big for a cabinet.

## What has to be found out first

This is the first spec in the arcade that needs something the arcade has never
done, and the size of it is not in the bookcase or the cellar. It is in the
stair.

**You have no up or down.** You are a sphere of radius 0.45 pushed horizontally
by `move_and_slide` against boxes that run from the floor to the ceiling, on a
plane at y nought. Nothing pulls you down, nothing holds you up, and
`Room::solid()` has no floor in it at all. Your height never changes because
nothing in the room has ever been at a different height.

**The engine is not the problem.** `sweep_sphere` already answers a step
correctly. Walked at a riser of 0.10 it reports the top edge rather than the
face, at a point of (2.0, 0.10, 0), with a normal of (-0.63, 0.78, 0). That
normal leans up. The engine knows the shape of a step.

**The slide is.** `move_and_slide` turns the part of your movement that is into
a surface into movement along it, and at a walking pace that is almost nothing
per frame. You cover 0.07 of ground in a frame, and 0.07 pushed against a
rounded edge comes out as a few hundredths of a unit of height. Measured over
three seconds of walking straight at a step, with gravity turned off so nothing
could undo it:

```text
riser   reached
0.10     0.00
0.20     0.00
0.30     0.01
0.45     0.00
```

It does not climb. It grinds. A character gets up a step because something
lifts it over, not because it slides up the corner, and that pass is what the
arcade has to grow. The engine gives the right normal to build it on.

So this spec is in three parts and the first is the whole of the risk.

## Behavior

**You stand on things and you fall.** The floor becomes something solid rather
than an assumption, the stair is a run of boxes, and a downward pull settles you
onto whichever of them is under you. Walking off the top of the stair takes you
down it rather than out into the air.

**A step is climbed by being stepped on.** Blocked by something no taller than a
riser, the move is tried again from a little higher and then settled back down.
The riser follows from that number and not the other way about: a stair of
twenty is a different length from a stair of eight, and the length decides
whether the stairwell fits under the nook at all.

**The nook's floor gets a hole in it.** It is one quad today, and a quad has no
hole in it, so the boards become pieces laid round the opening.

**One book is a handle.** It stands in the stocked shelves like the rest, and
the sight lands on it the way it lands on a cabinet, a shape or a bench, which
is a fourth thing for the sight to name. Pulling it swings its case off the
wall.

**The case that swings is solid both ways.** Shut, it is a bookcase and the way
down is not there. Open, it is a bookcase standing at an angle and the way down
is. `Room::solid()` has been a fixed list since spec 0001 and now has a state in
it, which every walkability test reads.

**The cellar is its own room**, with its own floor, its own walls and its own
light. The engine carries eight lamps and the nook's sconces take six of them
while you are in it, so down there is a third room wanting a share.

**The pool table opens poolhall**, the way cascada's bench opens cascada. The
test that keeps a repo from going missing asks that every folder under `games`
is a cabinet in the hall or a bench in the nook; it gains a third place to be.

## Acceptance criteria

To be written with the code. The ones that are already clear:

- You can walk up a step of the stair's own riser.
- You come to rest on the floor rather than passing through it or floating.
- Walking off the top of the stair puts you at the bottom of it, not in the air.
- Every sconce and every lamp in the cellar is on something.
- With the case shut there is no route from the nook to the cellar, and with it
  open there is.
- Every repo under `games` is a cabinet, a bench or a table.

### Verified by hand

- The case swings rather than vanishing. — run the arcade and pull the book.
- The stair reads as a stair going down and not as a ramp. — walk down it.

## Out of scope

**Jumping.** Falling is needed because a stair without it is a ramp you float
over. Jumping is not needed by anything here and a jump button is a different
room's idea.

**A lift, a ladder, or a second stair.** One way down and the same way back up.

**Anything above the ground floor.** The ceiling stays a ceiling.

**Moving poolhall out of its repo.** It stays its own repo, its own specs and
its own window, exactly as cascada did. What moves is where in the building you
find it.
