# 0037 A staircase that does not arrive

**Status:** draft
**Date:** 2026-10-22

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
  from the hall behind a book. Nothing has ever gone up.
- **A room whose whole content is climbing should be one you climb to.**
  Putting a staircase on the flat is the first thing it would contradict.

~~Out in the hidden room~~, which has a hundred and sixty metres of nothing
in it and would take no work at all. Ruled out: spec 0020's own goal calls
that wing "the part nobody finds", and putting the end of a trail there makes
it the part nobody finds twice over.

**Where the way up starts is not this spec's.** That is the last step of spec
0036's trail and belongs with it.

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

## What it is waiting on

Whether the loop is visible from the middle of the square, and that is the
whole engineering question rather than a detail.

Seen from inside one flight, nothing is strange and the crossing can hide the
seam. Seen from the middle, four flights that each climb are a helix, and the
place where the fourth meets the first is a join that has to be somewhere.
Hiding it, drawing the loop flat and lying about the climb, or letting the
join be visible and strange are three different rooms, and I do not know
which is right without building one.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
