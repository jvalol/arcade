# 0021 Eight rooms that are one room

**Status:** built, 2026-10-16. The room, the marks, the crossing and the way
in and out. What it looks like is Jake's to say.
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
wall and arrive standing on that wall, with the room you came from under your
feet.

**Half of that is forced and half of it is chosen**, and the spec should say
which. Walking at the shared square and carrying on round the corner rotates
`ana` onto the way you walked: that is the shape, and there is no choice in
it. What the shape does *not* say is which way up it leaves you. The corner
alone would leave your head where it was, and you would walk forward, keep
walking forward, and the only thing that had changed would be which direction
had become unavailable. True, and nothing to look at.

So the floor is put back under the wall you came through. That is this room's
decision, spent on the only part of the turn anybody can see.

**And it puts the room you left under you rather than over you**, which this
spec said the other way round twice before the arithmetic was asked. The
square you walked at is a face of both cells. Once it is the floor you are
standing on, the cell on its far side is the one you were just in. `up`
becomes `ana.back()`, which points away from where you came from, so what is
overhead is the cell opposite the one you left and what is underfoot is the
one you left.

It is the better version of the rule below, because it gives it a reason you
can see. You cannot walk back the way you came, and the way you came is the
floor.

**Gravity is yours and not the building's.** All six faces of a cell are the
same face as far as the tesseract is concerned, and which one is the floor is a
fact about the person standing in it rather than about the room.

**And the walk never learns about it.** This spec expected to teach `walk` to
hold somebody up on whichever face was the floor, which is a change to the
one function every room in the building shares and to all of spec 0007's
tests. It is not needed. A crossing is an event rather than a slide: you
reach the wall, the frame steps, and the room is laid out again with the new
floor at y nought. Inside a cell you are always the ordinary way up, so the
ordinary walk works untouched and the fourth dimension lives entirely in the
crossing.

The room is a cube because of this and not for tidiness. The wall you walk at
becomes the floor you stand on, so a room four wide and three tall would land
you in a room of different proportions and the second crossing in another one
again. Only a cube comes back to itself, which makes the height the price of
the trick rather than a choice: `CELL` is one number and the ceiling is as
far up as the far wall is away.

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

## What the first build found

Four things, and none of them was the fourth dimension.

**A cell is a closed box, so there is no sun in it.** The room was lit by a
directional light like every other room in the building, and that light
contributes exactly nothing: the shell shadows its own inside. Measured rather
than argued, because the first two guesses were wrong. The floor came out at
(73, 76, 89) at an intensity of 0.38 and at (73, 76, 89) at 3.0, to the byte,
which is what a shadow term of nought looks like from outside the shader. So
the room carries a lamp of its own, which is the only thing in here that makes
one face differ from another.

**The ambient multiplies the surface rather than adding to it**, which is
`base.rgb * (ambient + diffuse * lit)` in the engine's mesh shader and is the
ordinary way round. It is also what broke the first idea of this room. The air
was meant to say which cell you are standing in, so it was 0.42 of that cell's
hue and nothing else; a green floor multiplied by a violet is a grey, and
every panel in the room was being asked to survive that. The tint is down to
0.20 with a plain fill of 0.13 under it, and the colour of the room is in the
lamp's reach and the panels rather than in the air alone.

**Six stripes were drawn inside the shell**, every frame, and never once seen.
The mark stands off its panel by `PROUD` along the face's own way, and the
face's own way points out of the room, so adding it buried the lot. The test
that was meant to hold this checked that the stripe is inside its panel across
the two ways that lie in the panel, and deliberately skipped the third,
because that is the one the stripe stands out along. It skipped the only axis
the bug was on. There is now a test for standing in front rather than behind,
and it fails when the sign goes back.

**Landing 0.6 from a wall made the room unusable** in a way nothing could
test. You arrive against the face you came through; at 0.6 the first step
backwards crosses again, so every arrival was a faceful of panel and any drift
put you through another one. `INSET` is 1.25 now, which is far enough in to
turn round.

## What walking it found

Jake walked the room and brought back three things. One was a bug and two
were the room not working.

**The building's sight followed you in.** `self.seen` was worked out every
frame from `room::looking_at`, which reads the hall's cabinets and plinths
against wherever you are standing, and a cell's coordinates are not the
hall's. A cell three steps from the plinth lined up with a cabinet, so a wall
of the tesseract offered you slider, click or press enter to play. Enter would
not have started it, because the enter handler returns early in here, but a
click would have. Inside, the sight is nothing.

**The marks were a code with no key.** A hue for the axis and a bar high or
low for the end, and nothing anywhere taught either half. Jake's question was
"what are the white bars", which is the whole of the answer to whether a mark
needs a legend.

So **the walls say it in words**. Four pairs, one to an axis: east and west,
zenith and nadir, north and south, and ana and kata for the direction nobody
has. The pairing is knowledge a person already has, which carries the one rule
of the shape without anybody being told it: the room you cannot reach is the
other end of your own name. The colours stay, because a colour is read across
a room faster than a word and costs nothing, but what a wall means is now
written on it.

Three things went wrong drawing them, all of them about `text::drawn` rather
than about the room. It makes an **opaque** texture with a dark ground and the
letters picked out, so dark lettering on it is a black slab, which is what the
first build drew. It makes the texture **the shape of the word in it**, so a
quad sized without asking stretches "ana" to the width of "zenith"; the height
is chosen and the width follows from `text::room_for`. And a pale plate behind
a dark label behind a word is three things stacked and reads as a sticker on a
sticker, so the label's own ground is the sign, which is what the nook's plank
already does.

**And two of the six names were drawn sideways and backwards.** Pointing a
quad at a face takes one turn, and the letters then run along whichever way
that turn happened to leave them, which for the floor and the ceiling is away
down `ahead`: "nadir" written across the floor and read side on.

The turn that fixes it cannot be derived, or not by me. Two builds went on
reasoning about which way `u` runs against which way the quad is wound, and
`room::screen_mesh` says in its own comment that this has been got backwards
twice before and that the only reliable way to settle it is to draw a word and
read it. Four candidate turns drawn side by side on the floor settled it in
one picture.

The test for it was written twice as well. The first version asserted that a
flat name is flat, which the broken turn also satisfies: breaking the fix on
purpose left the test green. What it was missing is that a name laid flat has
to run **across** the room rather than along it, and with that in it the
broken turn fails. Whether a word comes out mirrored is still not testable and
still wants an eye.

**The room gave you nothing to hold.** Eight rooms alike in everything but
their marks is eight rooms nobody can navigate, which this spec said, and the
marks are on the walls you are looking away from. The corner now says which
cell you are standing in and where the way out is: "You are in north. The way
out is in ana." Naming the cell with the plinth in it is enough, and Jake
asked for exactly that. A count of steps to it would be a hint system, and
working out the route is the game.

## What it cost the nook

The sixth plinth does not fit. Spec 0002 sized that row by the clear span
between the end cabinets, which is 3.3 and cannot grow because what bounds it
is where the cabinets stand. Five shapes fitted with room to spare and six do
not, so the step came down from 0.72 to 0.59, which is the largest that still
fits: `5 * APART + APART / SWEEP` under 3.3.

Every shape in the nook is 18 per cent smaller for it. The teapot, the Klein
bottle and the three fractals all came down so that the thing which turns you
can stand with them. That is a real cost and it is the one this spec argued
for when it said the sixth belongs in that line on its own merits.

## Looking at it

`--screenshot=tesseract` opens in the room rather than in the building,
because there is no walking to it: a pose is a place and a yaw in the hall,
and the eight cells are nowhere in the hall.

## Acceptance criteria

The arithmetic and the room's geometry are written and these are their tests.
The drawing is not built.

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

And the room:

- The six faces of a cell name its six neighbours, and the two they do not
  name are the one you are in and its opposite. — `tesseract::tests::the_six_faces_name_the_six_neighbours`
- The room you came from is under the floor, and its opposite is overhead. — `tesseract::tests::the_room_you_came_from_is_under_the_floor`
- A crossing puts you on the floor just inside the far wall, with the way you
  did not spend carried over. — `tesseract::tests::a_crossing_lands_you_inside_the_far_wall`
- Standing anywhere in the room crosses nothing. — `tesseract::tests::standing_in_the_room_crosses_nothing`
- Three crossings bring your feet back as well as your frame. — `tesseract::tests::three_steps_bring_your_feet_back_too`
- The floor and the ceiling are solid and nothing else is, because the walls
  are the way out. — `tesseract::tests::only_the_floor_and_the_ceiling_are_solid`

The two that fail when they are broken, watched rather than assumed: swapping
the floor and the ceiling in `Frame::beyond` fails the second, and landing
you at the near wall instead of the far one fails the third.

And the drawing:

- A panel hangs inside the room, standing into it and with its back inside the
  shell, because two surfaces in one plane fight. — `tesseract::tests::the_panels_hang_inside_the_room`
- A cell's written name hangs on the panel, stands in front of it rather than
  behind, and the two ends of an axis put it on opposite sides. — `tesseract::tests::the_name_hangs_where_it_can_be_read`
- A name is square to its face, stands upright on a wall, and runs across the
  room rather than along it when it is laid flat. — `tesseract::tests::a_name_faces_into_the_room_and_stands_up`
- A short name is drawn the shape of its own letters rather than stretched to
  the width of a long one. — `tesseract::tests::a_short_name_is_not_stretched_to_a_long_one`
- The thing on the plinth is a four dimensional cube: sixteen corners, thirty
  two edges, four at every corner, and one cube inside another. — `tesseract::tests::the_shape_on_the_plinth_is_a_four_cube`
- There is a display for each shape on the wall, which is six now. — `display::tests::every_shape_is_on_the_wall`

Still to answer, and not written:

- The room has no sound of its own. Spec 0009 gives every room in the building
  a voice and this one is silent.
- There is no line of help in here, because the draw returns before the
  building's does, so the room says nothing about what to do.
- Nothing says you have reached the cell opposite the one you came in by,
  which is the thing there is to do.

## Verified by hand

- Walking into a wall leaves you standing on it with the room you left under
  your feet, and does not drop you.
- You can tell which cell you are in without being told.
- Eight rooms that look the same are eight rooms nobody can navigate, so they
  do not.
- Turning the sixth plinth reads as a surprise rather than as a fault, which
  is what the five before it are for.
