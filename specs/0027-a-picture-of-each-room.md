# 0027 A picture of each room

**Status:** implemented
**Date:** 2026-10-19
**Built:** 2026-10-19, four rooms of seven. The other three are waiting on
poses, for the reason under **What was built**.

## Goal

Draw each room in a test and hold it against a picture that was right, so a
wall that stops being drawn is caught by running the tests rather than by
Jake walking into the room.

## Why

**This building is checked by being played.** Every fault in the
walk-the-building notes was found that way: the sky dome reaching through
the hall, a chequerboard down the wall of the baths, a gap above a borrowed
wall, a ledge you could not see. The loop is Jake plays, Jake sends a
picture, and then somebody has to find where in the building that picture
is. It is the slowest loop in this project and it is the only thing
watching the part of the engine a test cannot reach.

There are 455 tests in this repo and not one of them has looked at
anything. They check where a box is, whether you fit through a gap, whether
two faces share a plane. All of that is arithmetic and all of it passes
whether or not the room is drawn.

**Three changes went through the engine yesterday that could each have
emptied a room**, and the only check on this building either time was one
photograph of the hall, taken by hand, because somebody thought to. Spec
0063 changed where a kept instance lives and what the vertex stage adds to
it. Spec 0064 changed which runs are kept and which are pushed. Spec 0026
had already handed this building's shells to the engine. A room nobody
photographed could have gone dark on any of them.

Engine spec 0062 built the machinery: a renderer with no window, a frame
read back as pixels, a comparison with a tolerance, and `BLESS=1`. It put
the games' own pictures out of scope and said they were the game's spec to
write. This is that spec.

## Behavior

**One picture a room.** The hall, the nook, the cellar, the baths, the
garden, the space behind the wall, and the back room. Each from a pose this
file names, and each looking at the thing the room is for rather than at a
blank wall.

**The pose is the game's own.** `Arcade` already carries `at`, `yaw` and
`pitch`, and already knows how to stand somewhere for a photograph: that is
what `--screenshot=back` does. A test sets the same three fields rather
than inventing a second way to stand somewhere, because two ways to say
where the camera is will come apart.

**Inside `src/main.rs`.** This crate is a binary, so an integration test
cannot reach `Arcade` at all. The pictures go in the test module that is
already there.

**Nothing in the picture may move.** The pool has a surface, the neon
flickers, the metronome swings, the gyro turns and the toys run. A picture
of any of those is a picture of what time it was. Each room's shot either
stands where none of them is in frame, or the test holds the clock still:
one `update` at `dt` of nought, from a game that has just been built, and
no second frame. **This is the hard part of this spec** and the reason it
is one room at a time rather than all seven at once.

**The comparison belongs to the engine.** Spec 0062's tolerance, its diff
image and its `BLESS=1` live in `blitzkit/tests/pictures.rs`, where no game
can reach them. Copying sixty lines into this repo would give two tolerances
that drift, which is the fault this building has written down four times
under other names. So the engine grows a small public thing for holding a
picture against a file, and both use it.

## What was built

Four rooms: the hall, the nook, the garden and the back room. Each is one
call naming where to stand and what to look at, and each reference was
looked at before it was committed.

**Three rooms are not in it, and that is a judgement rather than a
shortfall.** The cellar, the baths and the space behind the wall were posed
from the anchors their modules expose and all three came out wrong: the
cellar was a black frame with a floor edge in it, the baths were a view of
the garden's trees, and `behind::at` of this building's reach lands at
(76.8, 0, 87.8), which is nowhere a person stands. Committing a reference
of the wrong room is worse than having none, because it passes.

The way to get those three is the one this project already has written
down: Jake stands in the room, `WHERE` prints the line, and the pose is
read off it rather than guessed. Each is then one call, in the same shape
as the four that are in.

## What each room's picture is of

| room | what it has to show |
| --- | --- |
| hall | the aisle, the cabinets down both sides, the sign |
| nook | the toys, and that the way in is a way through |
| cellar | the panelling, the barrels, the stair |
| baths | the pool, the tile, the piers |
| garden | the trees, the pond, and the sky where the sky goes |
| behind | the columns, which is the one kept run this building has |
| back room | the bay, the gauge board, the bench |

## Acceptance criteria

- Each room matches its reference. — `tests::the_hall_is_drawn`,
  `tests::the_nook_is_drawn`, `tests::the_garden_is_drawn`,
  `tests::the_back_room_is_drawn`
- The comparison can fail: the same room with its lights turned off does
  not match. — `tests::a_room_picture_can_fail`
- Drawing a room twice gives the same pixels, which is what makes a
  reference worth keeping. — `tests::a_room_drawn_twice_is_the_same`
- A machine with no adapter skips rather than
  failing. — `tests::no_device_is_not_a_failure`

### Verified by hand

- Every reference looked at before it was committed, which is spec 0062's
  rule and the only thing that stops this becoming seven files nobody has
  seen. It is also what caught the three rooms that are not here: they were
  pictures of a wall, of the wrong room, and of nothing.
- The back room's first blessing was a picture of the ceiling lamp, because
  the thing being looked at had standing height added to it as well as to
  the camera. Looking at it is how that was found.

## Out of scope

**The games in the cabinets.** They animate and they are their own repos.
A cabinet's screen in these pictures is whatever it shows standing still.

**The tesseract.** Its own mode, its own room, eight of everything, and
spec 0021 already says it is measured differently.

**Anything timed.** One frame, standing still. The same rule spec 0062 set
for the engine.

**The three rooms above**, until somebody stands in them.

**A picture per pose.** One a room. A room that needs two pictures to say
whether it is right is a room that wants a test, not a second photograph.
