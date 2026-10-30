# 0038 Three ways to be the right way up

**Status:** draft
**Date:** 2026-10-24

## Goal

Replace spec 0037's loop with the other Escher. One room, three directions
that are all down, and you keep yours while everybody else keeps theirs.

Jake's, 2026-10-24, after walking 0037 and asking whether it would make sense
to an educated human. It would not, and the reason it would not is in the
shape of the thing rather than in how it was built.

## What 0037 got wrong, and what it got right

Right: where the room is, how you get to it, how big it is, how it is lit.
None of that is in question and none of it changes.

Wrong: the loop.

**You cannot have all three of honest geometry, a loop that reads as closed,
and steps that read as steps.** Shallow enough to look closed is too shallow
to look like a staircase. 0037 took the first two and gave up the third
without noticing, because the one test it has asks whether the gradient is
gentle enough to read as level and nothing asks whether anything reads as a
stair.

Walked on 2026-10-23. From the middle of the square the four flights read as
floating shelves, the loop cannot be seen to climb so it cannot be seen to
return, and the room is a grey box with a cone in it.

And 0037 had already ruled out the way round it. Its own words: *it is not
forced perspective, the geometry is honest and holds up from every angle.*
That is the right call and it is also exactly what makes a single Penrose
loop unbuildable here. Escher's own is a drawing and closes from one
viewpoint only.

## What this is instead

**Relativity, 1953, and not Ascending and Descending.** One space with three
mutually perpendicular downs, staircases belonging to each, and doorways in
surfaces that are walls to everybody but the person walking through them.

The geometry is honest from every angle, which 0037 required and this one can
actually keep. What is impossible is not a shape. It is that three floors
disagree about which way is down and all three are in use at once.

**You keep your own down.** You walk in on the floor, the floor stays the
floor, and nothing you can do changes it. The other two frames are theirs.

Jake chose the version you watch over the version you solve, 2026-10-24, and
there is a second reason to want it. Spec 0021 built eight rooms you walk
between with the frame turning under you, and they are parked, because
nobody could tell where they were and three rewrites of a map failed before
anybody saw why. Three frames in one room is that problem again with the
lights on. This version adds no movement rule at all.

## The figures are the room

Without people on them, three staircases at three angles are slabs stuck to
walls, and a player reads that as the building being broken rather than as
the joke. With people on them it is legible in one second and needs nothing
said.

Spec 0033 built the figures because forty lamps blinking is not forty guests
changing rooms. The same argument lands harder here: this is the first room
in the building whose whole content is other people using it.

So the room does not ship without them, and that goes in the acceptance
criteria rather than in a wish list.

## Three, and the vantage

**Three frames.** Jake, 2026-10-24: a computer drawing 3d rooms is not
fundamentally different from a drawing by human hand on paper. Three is what
fills a picture and this is a picture, so three.

**The one place the two come apart is not the count, it is the viewpoint.**
Escher's viewer cannot walk. His three compose in his one projection and
they stay composed for ever. A player's projection changes every frame, so a
layout tuned for where you come in falls apart as you walk off that spot.

So the vantage is a constraint and it is a harder one than a drawing has.
The way up arrives at one known place, which spec 0037 already fixed and
tested, and the picture has to assemble there. It also has to survive the
walk away from there, which means the three frames want to be legible from
the room's middle and from the far side as well, not composed for a single
pair of eyes standing still.

That is the thing most likely to be wrong on the first build, and it is
worth taking a picture from four places before believing any of it.

## What is kept and what goes

Kept: the room shell, its siting over the nook and the hall, the way up to
it, and its lamps.

Struck: the four flights round the square, the four landings, the things
standing on them, and the thing above the loop that no flight reaches.

## What this must not become

**An explanation.** Nobody says Escher and nothing says what you are looking
at. Same rule 0037 had and the only one of its rules worth keeping whole.

**A diorama.** A room you walk into, look at, and leave is against a lesson
this project has already written down: show what nothing else shows, and
give the player something to decide. Being a good picture is not enough and
is the obvious failure mode of the version you only watch.

## What you do in here, which is nothing yet and on purpose

Jake, 2026-10-24: start simple and make it more goal oriented or interesting
incrementally.

So the first build is the picture and nothing else. That is not a hole in
this spec, it is the order of work, and it is the order every other room in
this building went in: the first build of a room is a blocking model and
everybody knows it.

**First, the room reads.** Three frames, a flight and a door or two in each
so the flights go somewhere, and figures walking circuits on all three. It
is finished when somebody who has never heard the word Escher walks in and
sees people on the walls rather than a bug.

**Then, something to decide.** Not specced, because what a room like this
can ask of you depends on what it turns out to feel like, and guessing now
is guessing. Spec 0036's trail ends in this room and may be the answer.

### Why this order needs a guard and the other rooms did not

Every other room here leaves a hole you can see when it is unfinished.
Benches with nothing on them look wrong. A corridor with no doors looks
wrong. The register with no book in it looked wrong. The gap is in the
picture, so the room asks to be finished every time you walk through it.

**This room will not do that.** Three frames, figures walking them, the
composition landing from the arrival: that is complete as an image. Nothing
about it is missing. This spec can say the room is unfinished and the room
will not say it, so the only thing carrying the second half is intention,
and intention is the thing that decays.

There is a worse version of the same problem. A room that looks finished
gets defended. Tune the layout so the picture assembles from the arrival and
any purpose added afterwards has to fit round a composition that was
optimised without it, so the picture becomes the constraint and the doing
works around it. That is backwards from the rest of this building, where the
thing you do came first and the dressing came after. Spec 0031 is the sharp
case: the room was built, the mathematics under it was wrong, and fixing it
meant re-deriving the whole room as spec 0032. That was only possible
because what the room was for was settled enough to rebuild from. Here that
is the part being deferred.

It will also pass half a rule convincingly. `demos-need-a-decision` says
show what nothing else shows and give the player something to decide. This
room does the first better than anything else in the building and the second
not at all, and half a rule passed well is how things get called done.

### So the gate carries the reminder instead of anybody's memory

`check-specs` flags a draft only when every test it cites exists. A draft
citing one test nobody has written is a real draft and the gate is happy
with it; the moment that test exists the gate says so and asks for the
status to change.

So this spec cites one test that does not exist, below, and **0038 stays a
draft until the room asks you something**, however good the picture gets.
Marking it implemented when the picture lands is the failure this section is
about, and doing it would need the citation deleted by hand, which is a
thing somebody has to decide to do rather than a thing that happens.

## What it is waiting on

Nothing. Both open questions were answered on 2026-10-24, the day after it
was written: three frames, and no goal in the first build.

## Making it read

Two things went in on the 30th of October after Jake walked it and called the
walls and the floor bland, and both come off the same note this project keeps
about furnishing a room.

**A floor you can tell from the walls**, which is the first thing that note
asks for and the one this room never had. In here that means three floors:
three of the six faces are somebody's down and the other three are over
somebody's head. Nothing said which was which, so all six were drawn as wall
and the picture came out one value from end to end, in the one room whose
whole subject is that three surfaces disagree about which way is down.

Splitting them also stopped this room's own floor being drawn twice, once as
part of the shell and once by hand, the same boxes in the same colour. Two
faces in one plane that spec 0014's check cannot see, because `solid` is the
list both of them read.

**And something breaking the long walls**, which is the other thing that note
asks for. Piers, and there is only one place they can go: on a floor a pier is
a block in the way of whoever walks there, so they are on the three faces
nobody stands on, where a beam over your head is a pilaster up the wall to the
two frames that read that face as a wall.

A pier that meets a doorway or the way up is dropped whole rather than cut
round. Cutting was tried and makes slivers, and slivers put two drawn faces in
one plane. A missing pier reads as a gap in a rhythm, which is what a stair
coming through a wall should look like.

- No pier stands in anybody's way. — `aloft::tests::no_pier_stands_in_anybodys_way`
- And the way up still arrives, which a pier walled off on the first
  build. — `aloft::tests::you_can_walk_up_it`
- And nothing in here shares a plane with anything, which the slivers
  did. — `aloft::tests::nothing_drawn_in_here_is_in_one_plane_with_anything`

## Acceptance criteria

The first build's are not written yet. This one is, and it is the only one
that matters for the status:

- There is something in this room to decide, and it is reachable from the
  way up. — `aloft::tests::there_is_something_to_do_in_here`

**That name is a placeholder and nobody has written it.** Its whole job is
to hold this spec at draft while the room is only a picture, per the section
above. Whoever builds the second half will write the real test and should
rename this line to match it rather than leaving both.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
