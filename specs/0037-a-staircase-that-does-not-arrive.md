# 0037 A staircase that does not arrive

**Status:** implemented
**Date:** 2026-10-22
**Built:** 2026-10-22, the same day it was written. The room, the loop, the
four landings and the way up to it. Two things in here are not built and are
named at the end.
**Superseded in part:** 2026-10-24 by
[0038](0038-three-ways-to-be-the-right-way-up.md), which replaces the loop
and keeps the room, the siting, the way up and the lamps. Walked on the
23rd, the loop does not read as a staircase and cannot be made to, for a
reason that is in its shape rather than in its build. Read 0038 before
building anything against the four flights or the things standing on them.

## Goal

An imaginarium, with an Escher staircase in it. Jake's, 2026-10-14, thrown
out in the middle of a long session about something else and left alone since
because nobody had said what shape it should be.

Picking it as what is at the end of spec 0036's trail answers that: it is a
room in this building, like the baths and the garden, and not a game beside
them.

## Why it belongs here and not in a different project

Every substantial thing in this arcade is something that cannot be pictured,
made walkable. The solar system is a distance. The hotel is a count that does
not end. The register is a list that cannot be finished. The long hall is a
growth rate.

A Penrose staircase is the same move one step further: a path that gains
height all the way round and comes back to where it started. You cannot draw
it honestly and you can walk it, which is this building's entire habit.

## The lesson this room has to obey, which is already written down

Spec 0021 built eight rooms that are one room, and parked it the same day
because nobody could tell where they were. It came back with a map and it
still did not work, through three rewrites, and the reason is in that spec in
one line:

> A map can only point at places that already mean something.

Eight rooms alike in everything but a word on a wall. The fix was not a
better map: it was giving each room its own object so that there was
something for a map to be a map of.

**Four flights that look alike is the same room again.** So each landing is
its own, told apart by what is on it and not by a sign, and the thing that
tells you how far round you have come is the landings rather than a counter.

## What it is

Four flights round a square, each climbing, and the top of the fourth meets
the foot of the first. You go up all the way round and arrive where you set
off.

**The machinery exists.** Spec 0021's crossing already does this: you reach a
wall, the frame turns, and the room is laid out again with the new floor at y
nought. `tesseract::crossing` and `tesseract::Turn` are that, and a flight
whose top hands you to the foot of the next is the same event with a quarter
turn in it.

## Where it is

**Above.** Over the hall, about x −9 to 4 and z −15 to 8, with the loop
climbing from around y 6.

The plan is full. West is the cellar and the baths, north is the garden, east
is the hidden room and the back room, south is the whole hotel wing, and the
two corridors off its lobby already run two kilometres each. Measured, every
room in this building sits between y −4.5 and y 4.5, and **nothing at all is
above 4.5.** It is the one direction unused everywhere.

Three reasons, and only one of them is that the space is free:

- **The building already stacks.** The garden sits on the roof of the baths,
  per spec 0010, so a room on top of another is not a new idea here.
- **It mirrors the only vertical move there is.** Spec 0007 hid a stair down
  from the nook behind a book. Nothing has ever gone up.
- **A room whose whole content is climbing should be one you climb to.**
  Putting a staircase on the flat is the first thing it would contradict.

~~Out in the hidden room~~, which has a hundred and sixty metres of nothing
in it and would take no work at all. Ruled out: spec 0020's own goal calls
that wing "the part nobody finds", and putting the end of a trail there makes
it the part nobody finds twice over.

### The way up

~~Where the way up starts is not this spec's. That is the last step of spec
0036's trail and belongs with it.~~

Struck 2026-10-22, the day it was written, because it contradicts this same
spec forty lines above. Jake, with the room built: how do I get to it in the
app. The answer was that you cannot, and nothing walkable in this building is
above 4.5 metres.

**If the only way in is the trail, the room is exactly a lock and the trail
is exactly a key**, which the section below says it must not be, and a
visitor who never does anything else gets nothing rather than a whole room.
The second pair of sentences was written about the thing *above* the loop
being unreachable, and then the way into the room itself was handed to the
trail without noticing that had just been ruled out.

**So the way up is ordinary and anybody can find it.** What the trail changes
is what the fourth flight does once you are in here, and nothing else.

It starts in ~~the hall~~ **the nook**, mirroring spec 0007, which hid a
stair *down* behind a book in that room's wall of books and is the only
vertical move this building has ever made. Same room, the opposite
direction, and it puts the way in directly under this one rather than
sending anybody across the building to reach something that is over their
head where they started.

~~the hall~~ was written twice and was wrong twice: 0007's bookcase is in
the nook and always was. Corrected the same day, before anything was built
on it.

**And the nook is the only room under here with the floor for it.** Measured
rather than chosen. The footprint above runs x −9.3 to 3.8 and z −10.05 to
3.05, and three things are under it:

- **The nook**, clear from x −12.23 to −3.77, which is 8.46 across, and
  13.7 along. Books down the west face at 0.32 deep, benches down the east
  at 0.9. Room for a flight and a lane past it. It was 4.86 across when this
  was written and the flight went in on that; with the parapet either side
  the lanes came out at 1.75 and 1.23, so the room is 3.6 deeper than it was
  and they are 3.63 and 3.03.
- **The hall's aisle**, clear from x −1.45 to 1.45. That is 2.9, it is the
  spine of the building, and `WALL`'s own note says the aisle cannot change
  width without the sight breaking. A stair in it is a stair in the only
  way through.
- **A slot east of the hall** a shade over a body wide. Not a room.

So the nook, which is also the room the mirror asks for. The two arguments
land in the same place, which is the only reason this is a short section.

**It is a dog-leg and not a spiral.** A spiral was drawn first and thrown
away: every collider in this building is a box, a spiral tread is a wedge,
and the box round a wedge is most of a cylinder. Treads that overlap near
the newel lift you to whichever is highest, so the stair either refuses you
or slides you up it, and `walk` reports neither. Spec 0007 learned the same
thing from the other end, where nested treads made a flight of four legal
rises you could not get out of. Two straight flights are exactly what a box
describes.

## What you do, and the one thing you cannot

You climb. You pass four landings and you are back at the first, higher than
you started and at the same height, and nothing about any single flight was
odd.

**And there is something at the top that you do not reach.** Visible from the
middle of the square, obviously above the loop, and no flight goes to it. The
staircase is complete as an impossible object with it unreachable, which is
what a Penrose staircase is, so a visitor who never does anything else gets a
whole room.

**Unless you came the long way.** Spec 0036's trail ends here, and what it
buys is not finding this room, which anybody can do. It is that the loop
opens: having walked the building properly, the fourth flight goes somewhere
it has never gone before.

That is the shape worth having. The room is not a lock and the trail is not a
key: the room is finished either way and the trail changes what finished
means.

## What this must not become

**A disorienting corridor.** The tesseract's fault, the one it was parked
for. If somebody cannot say which landing they are on, the room has failed,
and no map will save it.

**A puzzle with an answer.** Nothing is typed, chosen or guessed, the same as
the register next door. You climb and the building does the rest.

**A trick that only works from one spot.** Forced perspective, the way the
real sculptures do it, looks right from one mark on the floor and broken from
everywhere else. This is a building you walk, so it has to hold up while
walking.

**An explanation.** Nobody says Penrose and nothing says what you are looking
at.

## Acceptance criteria

~~The loop:~~

~~The climb is gentle enough that the loop reads as closed. And the treads
are that gradient. The fourth flight's top is the first flight's foot.
Nothing of it sits in the floor's own plane. No two landings are in the same
place, which is what tells them apart.~~

**Struck 2026-10-24 with the loop, per
[0038](0038-three-ways-to-be-the-right-way-up.md).** Six tests, all passing,
all honest about what they measured, and the room still did not read. The
first of them is the one worth keeping in mind: it asked whether the
gradient was gentle enough to look level, which it was, and nothing
anywhere asked whether a stair looked like a stair.

What survives of that list, because it is about the room rather than the
loop:

- Nothing else in the building is up
  here. — `aloft::tests::nothing_in_the_building_is_up_here`

**Walking up it was not a thing anything asked.** `you_can_walk_up_it` walks
one body up the middle, which is the easy case and the only one that was ever
tried. Coming at the foot from an angle, a body settled against the bottom
step and stood there for ever. Jake found it by playing, twice; sampling it by
hand called it two lanes out of fifteen, and easing the turns in spec 0044
moved it somewhere else entirely, because it was never about the lane.

It is in the engine: a sweep that starts in contact answers at no distance at
all, so `move_and_slide` turns the movement along the face, comes round to the
same contact four times, and the frame ends where it began. The fix is there
and the proof is here, because this is the room that has the geometry to show
it: seven hundred ways at the flight, every lane across its width by four
distances back by seven headings. Eight of them stood still before and none
after.

The way up:

- It runs from the nook's floor to this room's, and meets its own landing at
  both ends of it. — `aloft::tests::the_stair_runs_from_one_floor_to_the_other`
- The landing laps the flight rather than meeting it edge
  on. — `aloft::tests::the_landing_laps_the_flight`
- Every rise is one you can get
  up. — `aloft::tests::every_rise_is_one_you_can_climb`
- It does not stand in the swing of spec 0007's door, and you can still
  reach the shelf that opens
  it. — `aloft::tests::the_stair_is_not_in_the_swing_of_the_door`
- Nothing in the room stands where the way up
  arrives. — `aloft::tests::nothing_stands_where_the_way_up_arrives`
- The slot in the floor is filled by the stair, so there is nothing beside
  it to fall down. — `aloft::tests::the_slot_is_filled_by_the_stair`
- Where the floor is not open the flight passes under it with room over your
  head. — `aloft::tests::the_second_flight_clears_the_floor_it_comes_up_through`
- All of it is under this room and none of it is
  outside. — `aloft::tests::the_whole_stair_is_under_the_room`
- Every lamp on it is in a fitting, in the shaft, and not inside the
  stair. — `aloft::tests::every_lamp_on_the_way_up_is_in_a_fitting`
- And somebody your size, walking, gets from the nook to the
  room. — `aloft::tests::you_can_walk_up_it`
- The hall's ceiling has nothing of the way up hanging under
  it. — `aloft::tests::the_way_up_leaves_the_halls_ceiling_alone`
- And a body pressed into either side of the first flight stays on the
  stair. — `aloft::tests::you_cannot_walk_off_the_side_of_the_way_up`
- And nobody coming at the foot of it from any lane or any angle ends up
  standing still on it. — `tests::nobody_sticks_at_the_foot_of_the_way_up`
- The landing is at the nook's
  ceiling. — `aloft::tests::the_landing_is_at_the_nooks_ceiling`

The ceiling one is the test that had to be aimed twice. The shaft has to
begin inside the thickness of a ceiling tile: level with the bars it
speckles against them, level with the tiles it speckles against those, and
anywhere lower it hangs across the hall as a pale slab the length of the
second flight. The height it needs is the tile's, and it was worked out
from the bars' underside instead, which is eleven millimetres lower. The
test measured the same wrong surface, so it passed while the slab was on
screen. `room::tile_under` and `room::tile_middle` are now the one place
either number is said.

**Seventeen steps up and fifteen on, rather than sixteen each.** Thirty two
rises over six metres either way, so every step in here is the same step.
What the split decides is where the landing is.

Sixteen put it at 3.00 and the nook's ceiling is at 3.18, so the second
flight set off from under that ceiling and had to climb through it. Its
first two treads hung 0.18 into the room below, in a hole cut in the tiles
for them, and from the lane underneath that is a wooden soffit coming out of
the ceiling. The same 0.18 showed as a slot running round the landing,
between the stair's top and the foot of the shaft. Both were dressed twice,
with a trim under the landing and then with every tread carried down to it,
and neither was a dressing fault.

Seventeen puts the landing at 3.1875, inside the thickness of a ceiling
tile. The second flight starts above the ceiling, nothing hangs under it,
and the shaft comes down onto the landing rather than stopping short of it.
Those two heights are worked out apart, one from the stair's own rise and
one from where spec 0012 hangs its tiles, and they agree to a millimetre, so
a test holds them together rather than one of them being written in terms of
the other.

It costs 0.28 at each end: the foot of the flight is a tread further south
and the arrival a tread further west.

**The first flight has solid sides.** A parapet, standing on the nook's
floor and stopping a rail's height over each tread, stepped because every
collider in this building is a box.

Without them the shaft hung from the nook's ceiling at 3.19 and everything
under it was open air. A body here is a sphere 0.9 across sitting on your
feet, so on the twelfth tread its middle is at 2.70, under the bottom edge
of the lowest wall there was, and pressing sideways took it under that edge
and off into a three metre drop. Jake walked into it twice. The arithmetic
was never wrong: every wall was where the plan said, and what was missing
was anything between the lowest wall and the floor.

**A parapet and not a wall to the ceiling.** Both stop the fall, and the
wall was built first. It also stands between the lane and the stair, and
that lane is the one view this building has of its own staircase: the
nook's two reference pictures both came back as a grey slab and both
cameras had to move to see round it. A metre over the treads you look over
the sides at a flight of stairs.

## What it is waiting on

~~Whether the loop is visible from the middle of the square, and that is the
whole engineering question rather than a detail.~~

Answered by building it. Three were built and looked at: a twelve metre
climb is 20.6 degrees and is plainly a spiral going away from you, four
separate flights from the floor are an X of ramps, and three metres over
thirty two of run is 5.4 degrees and reads as a square walkway. The join is
nowhere, because at that gradient there is nothing to see.

Two things in this spec are still only written down.

**The thing above the loop that no flight reaches.** The room is eight
metres tall and the loop climbs three of them, so there is height for it and
nothing in it. What it is has to be something the room already means, per
spec 0021's lesson, and I do not have that yet.

**What spec 0036's trail changes when it gets here.** The trail is a draft
and its last step is this room, so nothing can be built against it until it
is settled.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
