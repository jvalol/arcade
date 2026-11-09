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

One over each panel, and they go to the scene through the ranked and frustum
culled list in `main` rather than straight at it from this room's own drawing.
Pushed straight, they went in ahead of that list, so eight lamps behind a
bookcase held eight of the building's sixty four slots from everywhere in it:
the count was halved to five for it and the entrance was still three over the
cap. Through the list they are culled like every other room's, there is no
overflow anywhere in the building, and the corridor has its eight back. Jake,
2026-11-08, why not up the lamps from 8 instead of down to 5.

What that does not fix is the leak: these lamps do not cast, so what they put
on the corridor's side of the nook's bookcases they put on the nook's side
too. It was there at five and is a little stronger at eight. Visible in
`tests/rooms/nook.png` and `tests/rooms/wayup.png`, which both moved by under
two per cent of their pixels when the count went up.

**And facing the way you face, which took two goes.** Every account of
which way a figure points is `(sin facing, 0, -cos facing)`: `Arcade::forward`,
the `way.x.atan2(-way.z)` that `people::walking` and `aloft::walking` build a
facing with, and the `ahead` and `side` that `stand` places eyes and arms
along. The body's mesh was turned with `Quat::from_rotation_y(facing)`, which
sends its own `-Z` the other way, so every figure in the building stood
mirrored about the north line. It showed up nowhere until a figure was looked
at straight on, because a torso is a symmetric box and a guest walking a
corridor is near enough sin or cos of nought for the two to agree.
`tests::a_figure_faces_the_way_its_own_facing_says` holds them together now.

What that did not change is any picture, because the eyes were already placed
along `ahead` and they are the only part of a figure that says which way it
is pointing. Measured by making them large: at Jake's own pose the face is on
the side of the head you can see. The corridor's reflection is the right way
round too, measured by painting the north end red and looking west, where it
comes back on the left, which is north.

**The west wall's glass leans a degree and a half**, so the recession
bends away instead of stacking. Images in a facing pair turn by twice the
angle between the glass each bounce, so the nth is `2n` times that off true;
at nought they are collinear and every copy lands dead behind the one in
front, which is eight of your own head in a column. Jake, 2026-11-09, the
reflections would naturally curve away, not stack, and he is right about why:
real facing mirrors are never parallel to better than a degree, which is what
makes the chain in a lift arc off and leave.

One wall and not both, because the angle between them is what bends the
chain and one is enough to make it; the east wall is the nook's and leaning
it would lean the bookcases.

**The frames lean with the glass and the wall does not.** A panel is propped
against the wall the way a mirror is propped against a wall: `proud` and
`framed` stand the whole assembly out by `swing` first, so the head still
clears the wall it leans into, and the drawing turns the frames about the
glass's own middle. The wall, the floor, the lid and every box you can walk
into stay upright, which keeps the corridor square, keeps the collision
honest and keeps a gap from opening at the ceiling. `solid` and the drawing
therefore disagree about those frames by `swing`, on purpose and once.

Six tenths was the most the glass could lean on its own, with the frame
upright: past about a degree its head was in the wall. With the frame leaning
too, geometry stops bounding it and what is left is what a frame's foot puts
in your way. At a degree and a half that is 10.8 cm at the sill, against the
15 that `mirrors::tests::the_leaning_glass_clears_both_the_wall_and_the_frame`
allows, and it fails at four degrees. The other cost is that the reflection is
three degrees out of true, so the reflected floor meets the real one at a
slight kink where a pane stops.

**A frame is four pieces that touch and do not overlap.** The stiles ran the
full height of the opening and the sill runs its full width, so at each
corner two boxes sat in the same place with their fronts in one plane and
both on show. That is the seventh time this building has put two drawn faces
in one plane, and it reads as speckle that crawls as you move: Jake,
2026-11-09, there is stippling around the mirrors. It was there from the
first build of this room; the lean is what made it show, by standing the
frames out where the light catches them.

**A lamp over the way through.** The nook's sconce run is spaced every 2.2
and the opening is 2.18 wide, so the two that bracket it fall a centimetre
outside each end and nothing lands between them: the threshold had a lamp
either side and none over it. Jake, 2026-11-09, the nook floor still reads
black.

It is the nook's own fitting, higher, hung from the middle of `way` so the
door and its lamp cannot come apart, and `Arcade::bookcase_wall` finds which
of the nook's two sconce walls it belongs to rather than indexing one.

**What it does not fix is the floor**, and that was worth finding out. The
same patch looked at directly from inside the corridor is just as dark as it
is in the glass, and the nook's own reference picture has a dark carpet from
end to end. The mirror was telling the truth about a dark room. The lamp
lifts the threshold and the leaves; a lighter floor in there is the nook's
decision and not this room's.

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
- The leaning glass stays off the wall and inside the frame. — `mirrors::tests::the_leaning_glass_clears_both_the_wall_and_the_frame`
- No two pieces of one frame sit in the same place. — `mirrors::tests::no_two_pieces_of_a_frame_overlap`
- The lamp over the way through has wall behind it. — `mirrors::tests::the_lamp_over_the_way_through_is_on_wall`
- The corridor reaches into no other room. — `mirrors::tests::the_corridor_is_clear_of_every_other_room`
- Nothing is built against the west wall, which would be two walls in one plane. — `mirrors::tests::nothing_is_built_against_the_west_wall`

- The glass shows the corridor and not a black rectangle. — `tests::the_mirrors_are_drawn`

**The one that was black was the wall behind you.** Its target is filled
from the eye reflected in its own glass, which for a wall you have your back
to is a camera looking away from everything engine spec 0093 lets it draw.
On the screen that cost nothing; inside the other wall's glass it was the
whole picture, and Jake reported it four times in a day. Engine spec 0095 is
the fix.

**And it was found by hanging both targets on the wall as ordinary quads.**
One held the corridor, the other was black, which named the faulty one in a
single build. The position came off his own `WHERE` log rather than being
guessed at, so the first picture reproduced his screenshot exactly.

### Verified by hand

- The nook reads unchanged until you pull the leaves.
- You are in it, the right way round. — raise one hand and watch which hand the figure raises.
- The screen does not show the inside of your head. — walk at a panel and look down on the way.
- It is worth standing in. Colour, scale and whether the figure reads at close range are Jake's.

## Out of scope

**Curved glass.** Engine spec 0092 samples by screen position, so a curved
panel would read its plane's reflection as though it were flat, which gets
worse with curvature. A warped panel is its own spec, in the engine before it
is here.

**A third mirror.** The cap is two and this room is built for two.

**The second bounce, which is still black.** Spec 0095 moved the black one
level deeper and did not get rid of it: the pane in front of you now holds
the corridor, and the panes inside that reflection are solid black. Measured
in the window on 2026-11-09, standing in the middle of the corridor square
on to the west wall, and at Jake's own pose off the `WHERE` log. What the two
mirrors should give is a recession that goes until the light runs out, and
what this gives is one bounce and then nothing.

Three things are ruled out, each by a build: the glass is drawn inside a
mirror's pass, because tinting every glass surface green turned those black
rectangles vivid green; the uv it samples with is inside the picture, because
drawing the uv as colour showed no fragment outside nought to one anywhere;
and both targets are filled every frame from the right cameras, because
logging each fill showed the east wall's drawn first from a camera six metres
east of its glass looking west, which is the reflection that can see it. So
the second target is being written and read, and what it holds where it
matters is black. What that target holds was then looked at, by hanging it in the nook as an
ordinary textured quad, where the east mirror's clip plane cuts it out and
the nook's own wall hides it from the west one, and by clearing every mirror
pass to red instead of black so that undrawn reads differently from dark. It
holds the corridor. Dark, small, in the middle of a frame that is otherwise
the clear colour, but drawn.

The control is what makes that worth anything: hung the same way from the
same spot, the west wall's target, which is the one that works, is the same
shape. The corridor in the middle and nothing around it. So a mostly empty
frame is what a mirror target looks like from this far back, and the
difference between the bounce that reads and the bounce that does not is
how brightly the corridor in it is lit.

**And it is dark because what it draws is lit from behind.** Changing the
engine's diffuse term from `max(dot(normal, to_light), 0.0)` to `abs(...)`
brings the corridor back in that target: walls, floor, the frames and their
lamps. The geometry is drawn and the lamps are in the scene; the clamp throws
away every surface in it, which leaves the ambient and nothing else. Engine
spec 0095 carries what that rules out.

Both cameras check out on paper besides. Projecting the corridor's own points
through the second one puts them at the middle of its picture, with depth
between 0.02 and 0.40, and its cull frustum holds the corridor and correctly
rejects the hotel behind it. Three things in the lighting are already ruled
out: the scene uniform is rebuilt per pass from that pass's own camera, so
spec 0051's shift is right for it; the lamps are the scene's and not the
screen's; and the corridor's lamps do not cast, so no shadow map is involved.

**`way` deriving itself from the bookcases.** It holds its own numbers and a
test says they agree with `CASE`'s leaves. `cellar::opening` takes its place
from the bookcases instead, which is the better shape and the one to move to
if either ever drifts.

**The figure anywhere else in the building.** It exists in reflections, which
is the only place anything needs it. Spec 0033's people are a separate thing
and stay so.
