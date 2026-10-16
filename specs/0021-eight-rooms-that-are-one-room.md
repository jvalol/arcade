# 0021 Eight rooms that are one room

**Status:** the arithmetic is implemented, the room is not
**Date:** 2026-10-16

## Goal

Stand inside a tesseract. A cube with six walls, each of which is a different
cube, and the one you cannot reach in a single step is the one opposite you.

## Why it is a room and not a game

The building already settled this and the reasoning is in `games()`: poolhall
is a game with something to win, and it still got the cellar rather than a
cabinet, because "Pool is a table. The thing you want is to walk up to the
table." A tesseract is a space you walk into, so a screen is the wrong object
for it twice over.

The rest follows. This building already does walking, looking, collision and a
line of help at the top of the window, and a repo of its own would rebuild all
of that to carry one idea. And spec 0011 already hid one space behind a wall
that does not obey the rest of the building, so there is a precedent for a
second, and the second is a better joke than the first because by then you
know the building does that.

What it is not is the thing everybody makes. A tesseract drawn as a spinning
wireframe is the most made object in graphics, and the engine already has the
better version: klein is a bottle whose neck you can watch pass through its own
wall, which makes the "will not fit in three dimensions" point with an object
that has a punchline you can point at. A hypercube has a shape you have to be
told about. What nobody has made is the inside.

## What it is

### The eight cells

A tesseract's cells are its eight facets: the faces of a four dimensional cube
at `x = ±1`, `y = ±1`, `z = ±1` and `w = ±1`. Each is an ordinary cube and the
inside of one is an ordinary room.

Two cells meet along a square face when they sit on **different** axes. So the
`+w` cell meets `+x`, `-x`, `+y`, `-y`, `+z` and `-z`, and does not meet `-w`
at all: every cell has six neighbours and one opposite.

That is the whole map and you will hold it in your head in a minute. **The map
is not the puzzle**, and a spec that stopped here would have built a ring of
eight rooms with nothing in it.

It is also not what a person walking finds, which this spec said it was until
the arithmetic was written and asked. See "How far everything actually is".

### The wall you walk through becomes the floor

This is the puzzle, and it is the only thing the fourth dimension actually does
to a person standing in it.

In the `+w` cell, `w` is the direction you cannot move in. You are flattened
against it and the three you can move in are `x`, `y` and `z`. Cross into `+x`
and the axes trade: `x` becomes the one you cannot move in and `w` becomes one
you can. The direction you were walking is now the direction you are flattened
against.

In the room that is a quarter turn. You walk across the floor into the north
wall and arrive standing on that wall, with the room you came from overhead.

**Half of that is forced and half of it is chosen**, and the spec should say
which. Walking at the shared square and carrying on round the corner rotates
`ana` onto the way you walked: that is the shape, and there is no choice in
it. What the shape does *not* say is which way up it leaves you. The corner
alone would leave your head where it was, and you would walk forward, keep
walking forward, and the only thing that had changed would be which direction
had become unavailable. True, and nothing to look at.

So the floor is put back under the wall you came through. That is this room's
decision, spent on the only part of the turn anybody can see.

**Gravity is yours and not the building's.** All six faces of a cell are the
same face as far as the tesseract is concerned, and which one is the floor is a
fact about the person standing in it rather than about the room.

### Coming back is not retracing

**You cannot walk back the way you came.** The wall you came through is the
floor you are standing on, and nobody walks into a floor. This spec said the
opposite of that in its first draft, as a crossing and the crossing back
putting you where you were, and the two halves of it contradicted each other.
The arithmetic is what noticed.

It is the better rule. A door you can step back out of is a door, and a room
nobody can retreat from is the one worth building.

What gets you home instead is **three steps**, measured rather than chosen:
walking a triangle puts you in the room you left, the way up you left it, with
all four directions where they were. One step and two steps never do.

### How far everything actually is

A person walks on a floor, so two of any room's six walls are the floor and
the ceiling and are not walls you walk into. Four walls, so four cells in one
step. The two across your own up take two, because you have to become somebody
standing another way up first. **And the opposite takes three**, which is one
more than the shape says.

The whole of it is ninety six states over the eight cells, twelve ways to
stand in each, and nothing is stuck: every one of the ninety six can get home.
That ninety six, and not the eight, is the size of the thing a player is
holding in their head, and it is the answer to why the map is not the puzzle.

### What is written on the walls

Every cell is marked and every wall says which cell is through it.

A room with nothing in it is a room nobody can navigate, and the difficulty
should be holding the turning in your head rather than remembering which blank
wall you came through. The building already believes this: spec 0020 wore a
lane in the carpet rather than leave the hidden rooms to luck.

### The way in is a sixth plinth

**You turn it, and it turns you.**

Spec 0006's nook holds five things on plinths and one verb: teapot, klein,
sierpinski, menger, hilbert, and "click or press enter to turn it". The
tesseract is the sixth, and it belongs in that line on its own merits before
any of this is built: it is the object the other five are reaching towards, and
klein is already half of it.

Turning it is a four dimensional rotation, and a four dimensional rotation is
exactly the thing that changes which cell you are standing in. So the verb the
building has already taught you is the mechanic, which is why it is this and
not a door.

The setup is free, because the nook spends five objects teaching it. You turn
five harmless things, each of which turns on its plinth while you stand and
watch. The sixth turns you instead.

It is not another wall to walk into, which is half the point of choosing it.
Spec 0011's space is through the blank near end wall, and a second blank wall
would be the first one copied. The building's other ways in are a gap where a
wall stops, a flight of stairs, a bookcase that swings and a framed door, and
none of them is a thing on a plinth.

**What is through it is a room, and what sits on the plinth is a toy**, which
is the distinction the nook needs. `games()` separates the nook from the hall
by there being nothing to win in the nook, and there is something to win in
here. The nook holds the door; it does not hold the game, any more than the
cellar's bookcase is the baths.

**Coming back out** is one cell with the plinth standing in it. The goal below
is already "get back to where you started", and the plinth is what that looks
like from the inside: eight rooms alike in everything but their marks, and
exactly one of them with the way out in the middle of the floor.

### What there is to do

Reach the cell opposite the one you came in by, which is three steps off
however you go and is the furthest thing in here. Then get back to the plinth,
which you cannot do by turning round.

## What it is not

**Seeing through.** A wall says what is beyond it and does not show it.
Looking into the next cell is a portal render and most of the work.

**A view from outside.** Outside is the thing this is built to avoid.

**Four dimensional physics.** Nothing collides in four dimensions. You move in
three, in one cell at a time, and the fourth dimension is the crossing and
nothing else.

**A second tesseract.** Eight cells, one of each.

## Acceptance criteria

Nothing is built, so these are what the first build has to answer rather than
tests that exist.

The arithmetic is written and these are its tests. The rest is not built.

- Each cell has six neighbours and one opposite, and the neighbours are the
  cells on the other three axes. — `tesseract::tests::every_cell_has_six_neighbours_and_one_opposite`
- A step lands you in the cell its wall names. — `tesseract::tests::a_step_lands_in_the_cell_its_wall_names`
- You cannot walk back the way you came. — `tesseract::tests::you_cannot_walk_back_the_way_you_came`
- Three steps bring you back exactly, and nothing shorter does. — `tesseract::tests::three_steps_bring_you_back_exactly`
- The four walls you can walk at name four different cells. — `tesseract::tests::the_walls_name_four_different_cells`
- Walking never turns anybody inside out. — `tesseract::tests::walking_never_turns_anybody_inside_out`
- The wall you walked at is the floor you land on. — `tesseract::tests::the_wall_you_walked_at_is_the_floor_you_land_on`
- Every cell is reachable, four in one step, two in two, the opposite in three. — `tesseract::tests::every_cell_is_reachable_and_the_opposite_takes_three`
- Twelve ways to stand in each room, ninety six in all, and nothing stuck. — `tesseract::tests::twelve_ways_to_stand_in_a_room_and_nobody_is_stuck`

Still to answer, and not written:

- The walk works on whichever face is the floor, so a floor that was a wall a
  moment ago holds you up.
- Exactly one cell has the plinth in it, and turning it there puts you back in
  the nook.

## Verified by hand

- Walking into a wall leaves you standing on it with the room you left
  overhead, and does not drop you.
- You can tell which cell you are in without being told.
- Eight rooms that look the same are eight rooms nobody can navigate, so they
  do not.
- Turning the sixth plinth reads as a surprise rather than as a fault, which
  is what the five before it are for.
