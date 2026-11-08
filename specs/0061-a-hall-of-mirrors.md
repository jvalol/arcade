# 0061 A hall of mirrors

**Status:** implemented
**Date:** 2026-11-08

## Goal

A corridor behind the nook's bookcases whose two long walls are mirrors, with
you in them. The first room in this building that shows the person walking it.

## Why it belongs here

Jake's idea, 2026-10-16, raised at the end of a message about something else,
and he named the hard part in the same breath: it would need some kind of
avatar, because everything here is seen from inside one head and there is no
body to reflect.

Two things changed. The building draws people now, so a figure exists where
none did. And on 2026-11-08 he chose a real reflection over a funhouse faked
from a warped duplicate.

**That choice cost four engine specs, and three of them exist because this
room went looking.** 0090 gave the renderer a texture it draws into. 0091
exists because 0090 was unreachable from a game. 0092 exists because 0090
claimed the mirror flip lives in a quad's uv and it does not. 0093 exists
because a reflection with no near plane on the glass needs several metres of
nothing behind it, and this building has nowhere like that.

**The hard half is unchanged.** Every version of a person in this building
was wrong at close range first, and a hall of mirrors is nothing but close
range. That is the part this spec cannot settle and the window can.

## Where it goes, and why there

**West of the nook: x -16.90 to -12.85, z -4.30 to 5.60.** Four metres across
and ten long. Asked of `room::grounds` and of the building's own boxes rather
than worked out from the constants, because this project has lost a sitting
to the second way.

**Three placements failed before this one**, and each failure was the
building saying something.

The hall's east wall is full. Its cabinets grow symmetrically outward from
z 0 with the game count, ±0.47 at one game and ±8.18 at sixteen, so the
middle is never free and the free ends move. Cutting that wall also broke the
plaster, the benches and the walk-through check, because unlike the west wall
it had never been cut.

The strip east of the hall is bounded by the gallery to the south and the
solar system from z 6.50 north, and a corridor placed to clear the cabinets
overlapped the gallery's floor.

And this corridor's first north end at z 6.80 both enclosed the garden's
outer face and left an unfloored strip short of it. Pulling back to 5.60
settles both, at the cost of one panel a wall.

**North of the cellar stair.** The nook's west wall was already in three
pieces round an opening at z -6.75 to -4.55, which is the way down: nine
steps marching west. This room begins just north of them.

**And west of it is empty, which still matters on one side.** The east wall
is the nook's own, borrowed the way the garden borrows the hall's, and the
nook stands right behind it. That is only tolerable because of engine spec
0093; without the clip plane, standing two metres off those panels would put
the reflected eye two metres into the nook.

## The way in is a pair of bookcases

**The nook's west wall is lined end to end with them, 1.10 apart.** There was
never a clear stretch to put a door in. Spec 0007 settled that for the way
down to the cellar: one bookcase swings, and a wall with a bookcase sized
hole in it is still a wall from the room, because a bookcase is standing in
the hole.

So this is the same, at `mirrors::CASE`, with its own latch, its own swing
and its own handle. Not shared with the cellar's, because two secret doors
sharing one handle is one secret door. Not beside the cellar's pair either,
and not at either end of the run.

**Two leaves and not one, which took building it wrong to learn.** A bookcase
is 1.10 wide and a body is 0.90 across, so a single leaf leaves ten
centimetres a side and the walk stops dead inside the opening. The cellar's
way down is a pair for exactly this reason and nothing said so until this
room could not get through its own door.

**With a lintel over it.** The opening is `mirrors::HIGH`, which is a
bookcase's height, not the wall's. Full height it showed as a bright strip
above the leaves, which the nook's own reference picture caught at 0.4% of
its pixels.

## What is in it

**Two mirrored walls, one target each, five panels to a wall.** Panels in one
plane are one reflection, so a wall of five costs one target rather than
five. Engine spec 0092 is what makes that true rather than hopeful: the
sample is by screen position, so each panel reads the part of its plane's
reflection standing behind it. Engine spec 0091 caps a frame at two mirrors
because each is a pass over the whole scene, which is exactly what this room
wants.

**The glass is a plain quad.** Its uv is never read, so there is no mirrored
mesh and nothing here decides the flip.

**Framed, and the frames are what make it panels rather than a wall of
glass.** Ten metres of unbroken mirror is one surface, which is the fault
this building keeps hitting in other forms. They stand further off the wall
than the glass does, so no two drawn faces share a plane.

**Lamps in the frames and not on the ceiling**, derived from the panels the
way `spa::lamps` comes off `spa::sconces`. The baths once had a glowing
ceiling and dark sconces because two lists said where the light was.

**You, in the glass and not on the screen.** The figure stands where you
stand, facing where you face, with the gait off `paced`, which the footsteps
already keep. It is pushed inside engine spec 0091's bracket, because the
camera is at eye height and the screen would otherwise show the inside of
your own head.

**The floor meets the nook's rather than lapping under it.** It began lapped
and the engine's coplanar check refused it: two floors at one height that
overlap put both tops in one plane, which is what `behind` came back as
carpet torn into bands. Here the seam is crossed level and one number does.

## Acceptance criteria

- The corridor's ground is a box the building serves, so something walks on it. — `room::tests::every_ground_is_a_box_the_building_serves`
- With the leaves open you get in from the nook and back out, and with them shut you do not. — `mirrors::tests::the_nook_lets_you_into_the_mirrors_and_back`
- The opening and the leaves standing in it are the same span. — `mirrors::tests::the_opening_is_exactly_the_leaves_that_swing`
- You can walk its whole length, and both sides of it close to the glass. — `mirrors::tests::nothing_blocks_the_walk_down_the_corridor`
- The floor meets the nook's without a lap, and the seam is crossed level. — `mirrors::tests::the_floor_meets_the_nooks_without_a_lap`, `mirrors::tests::the_floors_lap_without_a_step`
- No two drawn faces share a plane, which is this building's commonest fault. — `mirrors::tests::the_frames_stand_clear_of_the_glass`
- Each wall's panels are all in that wall's one plane, which is what makes one target right for five of them. — `mirrors::tests::every_panel_on_a_wall_is_in_the_same_plane`
- The plane handed to the engine is the glass's own and not the wall it hangs on. — `mirrors::tests::the_mirror_plane_is_the_glass_and_not_the_wall`
- The panels fit, do not overlap, and the run is centred. — `mirrors::tests::the_panels_fit_the_corridor_and_are_centred_in_it`
- Every lamp is over a panel rather than on the ceiling. — `mirrors::tests::every_lamp_sits_over_a_panel`
- The corridor reaches into no other room. — `mirrors::tests::the_corridor_is_clear_of_every_other_room`
- Nothing is built against the west wall, which would be two walls in one plane. — `mirrors::tests::nothing_is_built_against_the_west_wall`

### Verified by hand

**None of these is done, and the room is only implemented in the sense this
column cannot reach.** It is built, it walks and it holds thirteen tests.
Nobody has stood in it.

- The glass shows the corridor and not a black rectangle.
- You are in it, the right way round. — raise one hand and watch which hand the figure raises.
- The screen does not show the inside of your head. — walk at a panel and look down on the way.
- The nook reads unchanged until you pull the leaves.
- It is worth standing in. Colour, scale and whether the figure reads at close range are Jake's.

## Out of scope

**Curved glass.** Engine spec 0092 samples by screen position, so a curved
panel would read its plane's reflection as though it were flat, which gets
worse with curvature. A warped panel is its own spec, in the engine before it
is here.

**A third mirror.** The cap is two and this room is built for two.

**`way` deriving itself from the bookcases.** It holds its own numbers and a
test says they agree with `CASE`'s leaves. `cellar::opening` takes its place
from the bookcases instead, which is the better shape and the one to move to
if either ever drifts.

**The figure anywhere else in the building.** It exists in reflections, which
is the only place anything needs it. Spec 0033's people are a separate thing
and stay so.
