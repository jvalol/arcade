# 0026 A building that stands still

**Status:** implemented, and it bought nothing in the hall, which is
worth as much as if it had
**Built:** 2026-10-18
**Date:** 2026-10-18

## Goal

Hand the engine the parts of the building that never move, once, instead of
describing them again sixty times a second.

## Why

The engine learned three things today and the arcade uses none of them.

Spec 0058 of the engine lets a game keep a run of instances: the saving is
the pushing, the hashing into batches, and the measuring and copying of
everything no view can see. Spec 0059 writes a kept run into the instance
buffer once and draws it where it lies. Spec 0061 asks the depth buffer
which kept runs a wall hides, and leaves those out of the picture.

All three work on kept runs, and the arcade keeps nothing. It pushes the
whole building every frame: eleven rooms, a carpet, two ceilings, a garden
and a pool table, and the only things in it that move are a ball on a chain,
some water, a wrecking ball and whatever the player is turning.

So the building is where all three of those specs were argued from and the
one place none of them has ever run.

## Behavior

**The shells first, and only the shells.** Every room in this building
already describes itself as a list of boxes, and the same list is what you
cannot walk through: `room::solid`, `back::solid`, `cellar::solid`,
`spa::solid`, `garden::solid`, `behind::solid`. Those lists are already the
one account of where a wall is, which is what makes them the safe thing to
keep: nothing has to be written twice to do it.

**A run per room, and only when the run is worth a draw call.** Not one run
for the building: spec 0058 says the engine will not divide a world and a
run is asked about whole, so a building kept as one run is a building that
is never behind anything. A room is the arcade's own patch.

But a kept run is a draw call of its own, and what is pushed is batched with
everything else of the same mesh and texture. The shells of this building
are lists of one, three, four, six, thirteen and three hundred and fifty one
boxes, so keeping all of them took the hall from 227 draws to 246 and saved
nothing. A run of thirty two or more is kept and the rest are pushed, which
leaves exactly one kept shell: the columns in the space behind the wall.

**What is kept is not pushed.** The fault this spec can produce is a wall
drawn twice, which looks exactly like a wall drawn once and costs double. A
piece belongs to the kept list or to the frame, never both, and a test says
so.

**What moves stays pushed, and that is most of the fittings.** A cabinet
that lights up when the sight finds it, a door that opens, a bookcase leaf
that swings, the water, the toys, the rig's cubes, the planets: all of them
go on being pushed every frame, because a kept run is for what does not
change. One instance of a kept run can be moved with `Scene::moved`, and
that is for a thing that moves rarely, not every frame.

**The rooms do not change shape, so nothing is rebuilt.** A doorway is
already a hole in the wall rather than a door that opens, per spec 0024, and
the two leaves that do swing are fittings rather than shells. If that stops
being true the run for that room is dropped and kept again, which is one
call.

## Acceptance criteria

- Every room's shell is kept once and never pushed, and the pushed list and
  the kept list do not overlap. — `tests::the_shells_are_kept_and_not_pushed`
- A kept run holds one room's boxes and nothing from another
  room's. — `tests::a_run_is_one_room`
- Standing still for two frames keeps the same runs, and changes
  none. — `tests::standing_still_changes_nothing`
- What a game can still walk through is what it could walk through before:
  the lists that are kept are the same lists the colliders come
  from. — `room::tests::the_building_is_where_it_was`

## What it came to

The hall, same pose, with the cost readout up:

| | draws | triangles | work | gpu |
| --- | --- | --- | --- | --- |
| pushed, as it was | 227 | 831,392 | 1.53 ms | 3.71 ms |
| every shell kept | 246 | 831,392 | 1.54 ms | 3.47 ms |
| only what is worth it | 228 | 831,824 | 2.23 ms | 1.96 ms |

**Nothing in the hall got faster, and the numbers bounce by more than the
difference.** What this building spends its frame on is the fittings, not
the shells: the cabinets, their screens, the plinths and the shapes on them,
the toys, the carpet. The shells are a few dozen boxes out of thousands of
instances, so handing them over once saves a few dozen pushes.

The triangle count moves by 432, which is thirty six cubes, because a kept
run is kept or dropped whole while a pushed instance is measured one at a
time. That is the granularity a patch costs, and it is the trade spec 0058
describes.

**And asking what the walls hide bought nothing either**, measured with the
engine's spec 0061 switched on in here: 246 draws and 1.66 milliseconds
against 1.54 without. From the hall, the one kept room ahead of you is
already outside the view, and the big one is behind you, so there is nothing
for the question to find. It is left off.

So this spec is a mechanism with its lists held to the colliders by tests,
and a measurement that says where the next one has to look. The fittings are
where the frame goes.

### Verified by hand

- The hall, with the readout behind F: the same picture, and 228 draws
  against 227. The draw count is the one to watch, because a kept run that
  is also pushed shows there and nowhere else, and 831,824 triangles against
  831,392 is the patch granularity rather than a wall drawn twice.
- The same with occlusion on, which found nothing from this pose, for the
  reason given above.
- Every room, walked, looking for a wall that is no longer there. A shell
  that was kept and not drawn is a hole you can see the sky through.

## Out of scope

**The fittings.** Benches, plinths, cabinets, shelves, barrels, trees and
the rest stay pushed. They are the next thing to keep if the shells pay, and
they are where the drawing is conditional, which is where this gets hard.

**Dividing a room further.** A room is the patch. If a room turns out to be
too big to be hidden by anything, that is a measurement, not a guess.

**The tesseract.** It is its own mode with its own room, and it has eight of
everything by construction. Spec 0021.

**Anything about what the engine does with a kept run.** That is specs 0058
to 0061, and this is a game using them.
