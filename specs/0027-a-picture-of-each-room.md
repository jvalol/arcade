# 0027 A picture of each room

**Status:** implemented
**Date:** 2026-10-19
**Built:** 2026-10-19 as four rooms of seven, and finished 2026-10-20 when
Jake sent a photograph of the baths with a ceiling in it.

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

### All seven, and three faults in the harness that took them

The cellar, the baths and the space behind the wall were left out on the
first build because every pose tried for them photographed the wrong
thing. The building was never wrong. The harness was, three times.

**A room drawn without `update` has its lights off.** The lamps are worked
out from where you are standing and a frame drawn straight after
`initialize` has none of them, which is why the cellar came out black.

**A point off a room's bounding box is not a place to stand.** The
cellar's was inside its own floor slab, and dropping in from the top of
the box landed in the hall above it. Dropping from a quarter up settles on
the cellar's floor.

**And the camera's height is not `at.y`.** It is `eye`, which eases toward
`at.y` over `EYE_LAGS` seconds so that walking down a step is not a jump.
With `dt` at nought it never eases at all, so every pose set through `at`
alone stood at whatever height the game opened on. That is why the baths
showed the garden's trees: the camera was on the hall's floor, above the
baths' ceiling, looking out. The four rooms that did work are all at ground
level, where the fault is invisible.

What settled it was a photograph. Jake stood in the baths and sent one,
and it has a ceiling in it, which is a thing no amount of looking at my own
pictures would have told me.

Three more things came out of finishing it. The angles are worked out from
where the eyes are rather than where the feet are, or a target on the floor
a few paces off is a picture of the floor. The control was comparing the
darkened hall against the reference **file**, so on a run where that file
did not exist it blessed the dark hall as the reference: it draws both and
compares them now, and `blitzkit::picture::how_far_apart` exists for that.
And the garden is aimed level at the pond rather than down at it, which is
the difference between the room and two fish.

**The three rooms that were not in it.** The cellar, the baths and the space behind
the wall were posed from the anchors their modules expose and all three
came out wrong, so they were walked into instead: a body dropped inside
each room's own boxes and left to settle, then photographed four ways from
where it landed. That is the method this project already has written down
and it is cheaper than asking anybody to stand anywhere.

Walking found two faults in this harness and did not produce three
pictures.

- **A room drawn without `update` has its lights off.** The lamps are
  worked out from where you are standing, and a frame drawn straight after
  `initialize` has none of them. One `update` at `dt` of nought is in the
  harness now, which is what this spec said to do and what the first build
  did not.
- **A point off a room's bounding box is not a place to stand.** The
  cellar's was inside its own floor slab, and dropping from the top of the
  box landed in the hall above it. Dropping from a quarter up the box
  settles on the cellar's floor at -3.0.

What walking did not give is a view worth committing. From the middle of
the cellar's floor the four axes are four pictures of panelling; aimed at
its pool table the frame is a dark wall. The baths from their own floor
look up at the garden's trees through what should be a ceiling, which is
either a fault worth somebody's eye or the light well working as intended,
and this file cannot tell. And `behind::solid` is not the space behind the
wall at all: it is the far space with the orrery in it, which is why that
pose landed ninety metres out.

**Choosing a view that shows a room is a judgement**, and three attempts at
each of these did not make it. The five seconds it takes to stand in the
room and read the line back is worth more than another hour of guessing.

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
  `tests::the_nook_is_drawn`, `tests::the_cellar_is_drawn`,
  `tests::the_baths_are_drawn`, `tests::the_garden_is_drawn`,
  `tests::what_is_behind_the_wall_is_drawn`,
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
- The three rooms walked, four ways each, three times over as the harness
  was fixed. Something over twenty pictures looked at, and the last three
  of them kept.
- The baths against Jake's own photograph of the room, which is what says
  the view is of the thing the room is for.

## Out of scope

**The games in the cabinets.** They animate and they are their own repos.
A cabinet's screen in these pictures is whatever it shows standing still.

**The tesseract.** Its own mode, its own room, eight of everything, and
spec 0021 already says it is measured differently.

**Anything timed.** One frame, standing still. The same rule spec 0062 set
for the engine.

**The readout.** `update` fills the help text and the 2D overlay, and at
320 by 240 the help fills the frame. These pictures are of the building, so
the quads and the text are left out of them, which also means a reference
does not change when a line of help does.

**A picture per pose.** One a room. A room that needs two pictures to say
whether it is right is a room that wants a test, not a second photograph.
