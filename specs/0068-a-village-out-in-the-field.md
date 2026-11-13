# 0068 A village out in the field

**Status:** implemented
**Date:** 2026-11-13

## Goal

A second place to walk to, out in the field, with people living in it.

Jake, 2026-11-13: a western pioneer folksy style village, and the people should
move around meaningfully, walking from the house to the outside firepit, the
wagons to the horses, getting food.

## Where it is, and why there

**South of the way out, where the bridge is west.** Spec 0054's bridge runs
west from the garden's gap along that gap's own middle line, and the whole of
spec 0036's trail ends up there. A village on that line is on the way to
something; a village off it is a place you choose instead, which is what makes
it worth walking to.

Far enough out that the garden does not frame it. The field's ground reaches
160 metres and follows you, so there is no edge to fall off and no cost to
going further.

**Not a station on the trail.** Spec 0063 is the argument: a room that puts
its idea in your head before it shows you anything is a room that does not
land. This one puts its idea on the ground, which is people doing things, and
you meet it by walking in.

## What is in it

A street running east and west, so the first thing you see on walking up is
down it rather than at the back of something.

- **Six cabins and a barn**, gabled, off both sides of the street, none of them
  the same size. The barn closes the west end.
- **A firepit** in the middle of the street with a cook pot over it. This is
  the one Jake named and it is what the rounds are built round.
- **A well** with a trough beside it.
- **Two wagons** and a hitching rail, with horses at it.
- **Fences** between the cabins, so the street is a street and not a gap
  between buildings.

Nothing is behind a door. Every fitting here is outside, because what this
place is for is being walked through and looked at.

## What it is made of

**Geometry and not textures**, which is this building's own answer and the
cellar's panelling gives the reason: a cube's corners run nought to one
however big the cube is, so a timber texture on a cabin's wall would be a
single plank seven metres long. What makes sawn boards read is the shadow
line between one and the next, and a shadow line wants an edge.

- **The cabins are boarded**, horizontal courses standing `BOARD_OUT` proud.
- **The roofs are shingled**, courses laid from the eaves up, each standing
  `SHINGLE_OUT` off the face of the pitch, with a ridge board over the join.
  Two flat slabs leaning together is a roof nobody has been on.
- **And a course is shingles rather than one band.** It ran the whole length
  of the building at first, gable to gable with no vertical joint anywhere in
  it, which is what a sawn board looks like however many of them you stack.
  Jake, 2026-11-13: make the village roofs read as shingle, not planks. They
  are `SHINGLE_WIDE` across now, with every other course stepped half a
  shingle along so the joints never stack into a seam running up the pitch.
- **The street has ruts**, at a wagon's own wheel spacing, in pieces that
  wander: a pair of straight lines is road markings, and what a wheel does is
  follow the last wheel without following it exactly.
- **No two pieces are the same shade.** Every board, shingle and course takes
  a tone off its own place, through two sines that do not divide, so the
  pattern up a wall never repeats and no two walls of one building agree. A
  wall of identical boards is a wall with lines ruled on it.

**And the steading is staved where this is boarded.** Spec 0056's longhouse
got upright split staves and courses of turf at the same time, which is what
you build with an axe and a wood where this is what you build with a saw and
a wagonload of planks. The grain running the other way does more to tell the
two places apart than any colour would.

## The people

Spec 0067's rounds, over this geography. What Jake named, one errand each:

- From a cabin to the firepit and back, which is cooking.
- From the wagons to the horses, which is loading.
- From the well to a cabin, which is water.
- Up and down the street, which is somebody going somewhere you are not.

## Acceptance criteria

- Nothing here stands in the street. — `village::tests::the_street_is_clear`
- No two buildings share ground. — `village::tests::no_two_buildings_overlap`
- You can walk in from the garden and out again. — `village::tests::you_can_walk_into_it`
- Nobody on a round walks through anything. — `village::tests::nobody_walks_through_anything`
- No tree grows inside it. — `village::tests::nothing_is_sown_in_the_village`
- It is clear of the bridge and of the garden. — `village::tests::it_stands_clear_of_everything_else`
- A course lies on the face of its pitch rather than inside it. — `village::tests::every_course_stands_proud_of_its_pitch`
- A course is shingles and their joints are staggered. — `village::tests::a_course_is_shingles_and_their_joints_are_staggered`
- The street matches its reference. — `tests::the_village_is_drawn`
- The common and its cook fire match theirs. — `tests::the_village_common_is_drawn`
- And the horses at the rail. — `tests::the_village_horses_are_drawn`

### What the tests caught, before anything was drawn

Four faults, all of them found by running the layout tests against a village
that existed only as arithmetic.

- **The barn lay across its own street.** It was 15 deep against a street 10
  wide and placed inside the street's own run, so the thing meant to close the
  west end blocked it. It stands beyond the end now.
- **The village reached the garden.** `around` was a written-down size, 38
  metres, while the barn stood at 43. It is worked out from the buildings now,
  because a second number for how big a place is drifts from the first the
  moment anything in it moves.
- **The firepit was in the middle of the street.** Which is where a village
  puts a fire in a picture and nowhere a village would put one. It is on a
  common off the street, in a gap the fences are told to leave open: a gap wide
  enough to take a wagon is a way in and not a hole to patch.
- **Somebody walked through their own cabin.** Twice: once for cutting the
  corner from the doorstep to the fire, and once for not cutting it far enough.
  The round goes out past the gable and turns up a line nothing stands on.

And three after it was drawn, which the pictures caught and the arithmetic
could not:

- **Every door and window was on the back of its cabin.** A facing of nought
  in this building looks down -z, so the row standing north of the street has
  to be PI to look back at it, and it was written the obvious way round. The
  street was a run of blank gables.
- **The street was grass**, which reads as a mown gap between two rows rather
  than as the way everybody walks. It has beaten earth on it now, and the
  common with it.
- **The horses were benches.** Three faults in one animal: the neck was leaned
  about x where a horse's forward here is its own x, so both of them lolled
  sideways; the body was built along +x while a facing of `f` looks down
  `(sin f, 0, -cos f)`, ninety degrees away, so they stood broadside to the
  rail they were tied to; and the legs were given the back's height and hung
  from the barrel's middle, so they ran twenty centimetres into the ground.
  Every measurement on a horse is a height off the ground now.

### Verified by hand

- It reads as folksy and lived in rather than as sheds in a row.

## Out of scope

- **Going inside.** The cabins are solid lumps with lit windows, as spec 0056's
  outbuildings are. The hall at the top of the bridge is the one building in
  this game you go into and it should stay that way.
- **Anything on the trail.** No station, no asking, nothing to answer.
- **A picture of a board.** Everything with character on it here is geometry,
  not texture, which is the cellar's own argument: a cube's corners run nought
  to one however big it is, so a timber texture on a cabin's wall would be one
  plank seven metres long. What is missing is grain *within* a board, and that
  wants `room::tiled_plane` and a bake, which is its own change.
- **A horse that moves.** Spec 0067's gait is a two-legged walk and a horse
  that used it would be a person on all fours.
- **People avoiding each other.** Four rounds that crossed at the same second
  would walk through one another. They do not, and nothing here would stop it
  if they did.
