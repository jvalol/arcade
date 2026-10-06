# 0007 A door in the bookcase

**Status:** implemented
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

## What it took

The risk was never the bookcase or the cellar. It was the stair, because the
arcade had no up or down: you were a sphere pushed horizontally against boxes
running floor to ceiling on a plane at y nought, nothing pulled you down,
nothing held you up, and `Room::solid()` had no floor in it at all.

**The engine was wrong, in a way that got righter the taller the step.**
`sweep_sphere` has a shortcut for "already in contact", which is what resting on
a floor is, and it took the way out from the nearest face of the box grown by
the radius. That agrees with the real contact against a face and disagrees
against every edge. Beside a step 0.05 high a walking sphere sits a quarter of a
unit inside the grown side and a twentieth under the grown top, so the nearest
grown face is the top: it answered "you are standing on this", sideways counted
as along the surface, and the sphere walked through the step. The lower the step
the more certain it was, so the one riser too small to be in anybody's way was
the one riser nothing could get over. It takes the direction from the box's own
closest point now. That is the engine's fix and the engine's spec 0014 carries
it, with the test that walks a sphere at a step and asks that it never end up
inside one.

**A step is climbed by being stepped over.** The slide turns the part of your
movement that is into a surface into movement along it, and at a walking pace
that is almost nothing: 0.07 of ground a frame against a rounded edge comes out
as hundredths of a unit of height. Measured over three seconds walking straight
at a step with gravity off, a 0.10 riser gained nothing and a 0.30 riser gained
0.01. It does not climb, it grinds.

Two things about that pass were wrong before it worked. It took any climb that
beat going at it flat, which against a wall too tall to climb means grazing the
top edge and landing in open air a few hundredths up: free height every frame,
and gravity cannot give it back, because falling starts at nothing each time you
land and its first frame is worth six thousandths against a lift of five
hundredths. Thirty frames of that and you are on top of anything in the
building. So it asks the floor: one more hair of a drop, and if you move there
was nothing under you. And it looked one frame ahead, which is 0.07 against the
quarter of a unit it takes to get your middle over a step's face, so with
landing enforced a real step stopped being climbable.

**Going down needed its own half.** You leave each tread at the top and fall to
the next, and a quarter unit riser takes nine frames to fall while a tread takes
four to cross, so you never land on the next tread: you sail out over the whole
flight. Sticking to the ground within a step fixes it, and the ground has to be
found with a ray rather than a sphere, because a sphere leaving a tread hangs on
its lip and a sphere on a lip cannot go down.

That snap then undid every step up, because you climb onto the next tread before
you are over it, so what is under your feet is still the one you left. One of
the two has to win and it is the step.

**A point for up and down, a radius for sideways.** The radius draws nothing, so
it looks like it could go. It cannot: the eye sits at your position and the near
plane is a tenth, so as a point you could walk your eye into a wall, and every
clearance in the building was measured for somebody 0.9 across. But every one of
the faults above was a sphere answering a question about the floor. The tread
had to be 0.7, deeper than the body is from its middle to its toe, until the
step up stopped being asked as a question about the body. It is 0.3 now, which
is what a stair looks like.

**The stairs were not janky, the view was.** At a walk you cross a tread in four
frames, so your feet drop a quarter of a unit about fourteen times a second, and
the eye was nailed to them. The feet stay exact, because the room is measured
against them. The eye lags by 0.09 seconds, which turns fourteen drops a second
into going down a slope.

## Behavior

**Behind the wall rather than through the floor.** A secret stair is a hole in a
wall, not a hole in a carpet, and it costs less: the nook's floor is one quad and
a quad has no hole in it, so a stairwell inside the room would have meant laying
the boards in pieces round an opening. Beyond the wall there is nothing to cut.

**The door is two bookcases**, each hinged at its own outer end so they open away
from each other and the way down is between them. One was 1.1 against a body 0.9
across, which is a tenth of a unit of daylight either side held for the nine
units it takes to get down. Walked exactly down the middle it fits, which is
what the test did and why the test was worth nothing.

**One book is the handle, and only that one.** The sight landed on the whole
case, so clicking anywhere on either leaf worked. That makes the door a pair of
very large buttons and makes the room's own sentence about it a lie: one of them
is not a book, and any of them would do.

It is the seventh along the third shelf of the left leaf, and finding where it
sits means working out the frame that shelf stands in, which the drawing already
does. Written out a second time those would be two lists of where a bookcase is,
and two lists of the same wall is what hung a sconce in a doorway in spec 0006.
They read one.

**It glows a little.** A shade over one in every channel, so it carries light
with nothing lighting it, the way a cabinet's band does, and nothing like as
much: a band is at 3.2 and that is neon. A book catching a light which is not
there is the most a secret door can give away and still be one. Its own colour
and not its spine brightened, because the spines come off a seed and some of
them are nearly black, so a multiplier would have made the handle a slightly
less dark book on a shelf of dark books.

**It takes its time.** A bookcase on a hinge with four hundred books in it, and
the whole of what makes a secret door worth having is the moment between pulling
the book and seeing what is behind it. The box follows the swing rather than
snapping at either end, so there is no point where it looks open and stops you.

**And it does not open through you.** You stand in front of the case to pull the
book and the case sweeps that floor. Everything else about getting about is you
moving and the room holding still, which is what `move_and_slide` is for, so
nothing pushed back and the shelves swung through the viewer. You are pushed out
sideways, in the leaf's own frame: on the room's axes a leaf at forty five
degrees fills a box half as big again as it is, so which way is out of that box
is not which way is out of the shelf, and a closing leaf shoved you along itself
and then swept past.

**Nothing down there is inside the room above it.** `back` is the wall's inner
face, the side the nook is on, so a box starting there reaches a third of a unit
into the wall. The stair's soffit did, and fought the lintel over the opening for
the same pixels: a grey rectangle with stippled edges hanging over the books. So
did both shaft walls.

**The pool table opens poolhall**, the way cascada's bench opens cascada. It is a
bench as far as the room is concerned, because a bench is a thing you walk up to,
point at and press a key at. The three tests that assumed every bench stands in
the nook say so now, and the flood fill skips it, because a grid at one height
cannot answer a question about a cellar. Getting down there is tested by walking
it.

**A lamp over the table**, which is both what a pool room is lit by and what the
cellar wanted: with no light of its own it came out as flat grey surfaces with no
shape to any of them. It goes in the same list as the nook's sconces and the
sign's pendant and is chosen off the same distance.

## Acceptance criteria

- You can walk up a step, and the shallowest is not the hardest. — `walk::tests::it_climbs_a_step`
- And one taller than a step is a wall. — `walk::tests::it_does_not_climb_a_wall`
- You come to rest on the floor rather than sinking or hovering. — `walk::tests::it_puts_you_on_the_floor`
- Walking off a step takes you down it. — `walk::tests::it_walks_you_back_down`
- And a wall still stops you, which is what the room is built of. — `walk::tests::a_wall_is_still_a_wall`
- A tread is well inside what you can climb. — `cellar::tests::a_tread_is_lower_than_a_step`
- And the run fits the shaft that holds it. — `cellar::tests::the_stair_fits_the_shaft`
- You can walk down the stair and back up it. — `cellar::tests::you_can_walk_down_it_and_back_up`
- Shut, the way down is not there; open, it is, off the middle as well as along it. — `room::tests::the_case_is_a_door`
- And you can get back up it through the room's own geometry. — `room::tests::you_can_get_back_up_the_stair`
- One book opens the wall, and the rest of the shelf does not. — `room::tests::one_book_opens_the_wall`
- The door does not open or close through you. — `room::tests::the_door_does_not_open_through_you`
- Nothing down the way down is inside the room above it. — `room::tests::the_way_down_keeps_out_of_the_room`
- Every repo is a cabinet in the hall, a bench in the nook, or a table in the cellar. — `tests::it_knows_every_game_the_project_does`

### Verified by hand

- The leaves swing rather than snapping, and read as open. — run the arcade and pull the book.
- The stair reads as a stair going down. — walk down it.
- The table reads as a pool table. — walk up to it.
- The book is findable without being marked. — look along the shelves in the nook.

## Out of scope

**Jumping.** Falling is needed because a stair without it is a ramp you float
over. Jumping is not needed by anything here and a jump button is a different
room's idea.

**A lift, a ladder, or a second stair.** One way down and the same way back up.

**Anything above the ground floor.** The ceiling stays a ceiling.

**Moving poolhall out of its repo.** It stays its own repo, its own specs and
its own window, exactly as cascada did. What moves is where in the building you
find it.
