# 0037 A staircase that does not arrive

**Status:** implemented
**Date:** 2026-10-22
**Built:** 2026-10-22, the same day it was written. The room, the loop, the
four landings and the way up to it. Two things in here are not built and are
named at the end.

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

- **The nook**, clear from x −8.63 to −3.77, which is 4.86 across, and
  13.7 along. Books down the west face at 0.32 deep, benches down the east
  at 0.9. Room for a flight and a lane past it.
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

The loop:

- The climb is gentle enough that the loop reads as
  closed. — `aloft::tests::it_is_shallow_enough_to_read_as_closed`
- And the treads are that gradient rather than a number beside
  it. — `aloft::tests::the_steps_are_what_that_gradient_is`
- The fourth flight's top is the first flight's
  foot. — `aloft::tests::the_fourth_flight_comes_back_to_the_first`
- Nothing of it sits in the floor's own
  plane. — `aloft::tests::nothing_of_it_is_in_the_floors_plane`
- No two landings are in the same place, which is what tells them
  apart. — `aloft::tests::no_two_landings_are_in_the_same_place`
- And nothing else in the building is up
  here. — `aloft::tests::nothing_in_the_building_is_up_here`

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
- You can get past the second flight to the rest of the
  room. — `aloft::tests::you_can_get_past_the_second_flight`
- The slot in the floor is filled by the stair, so there is nothing beside
  it to fall down. — `aloft::tests::the_slot_is_filled_by_the_stair`
- Where the floor is not open the flight passes under it with room over your
  head. — `aloft::tests::the_second_flight_clears_the_floor_it_comes_up_through`
- The stairwell does not cut the
  loop. — `aloft::tests::the_stairwell_does_not_cut_the_loop`
- All of it is under this room and none of it is
  outside. — `aloft::tests::the_whole_stair_is_under_the_room`
- Every lamp on it is in a fitting, in the shaft, and not inside the
  stair. — `aloft::tests::every_lamp_on_the_way_up_is_in_a_fitting`
- And somebody your size, walking, gets from the nook to the
  room. — `aloft::tests::you_can_walk_up_it`

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
