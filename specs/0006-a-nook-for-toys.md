# 0006 A nook for toys

**Status:** implemented
**Date:** 2026-10-05

## Goal

The arcade held two kinds of thing: games in cabinets, and the engine's own
shapes on plinths. The chain example is neither. You work it, so it is not a
shape; there is nothing to win, so it is not a game. This gives the room a third
kind and somewhere to put it.

## Behavior

### Toys, benches, a nook

A **toy** is a demonstration you drive. You work the controls, the engine shows
you what it does, and there is nothing to win.

A **bench** is what a toy stands on, the way a cabinet holds a game and a plinth
holds a shape. Waist high and deep enough to lean over. How tall and how deep
are the same for every bench, because that is what makes a row of them read as a
row. How long is not: the cradle's frame has to clear the swing of its end balls
and the metronome wants a square foot.

The **nook** is where the benches are. It runs behind the left wall from the far
end of the room, and the way in is the stretch of that wall past the last
cabinet.

That way in is the correction. The first build took the wall away over the whole
far end of it, which left the last cabinet on that side standing in the opening
with nothing behind it: from the aisle you saw through its back into the nook,
and from the nook you saw the back of a cabinet. The nook is a room behind that
wall and you get to it round the end of it.

The far end of the room carries more than it used to. It was one cabinet step
past the last cabinet, which was enough while that end held nothing but the
shapes on their plinths and you could walk straight through them. Making the
plinths solid, per spec 0002, closed the only route in: it runs between the last
cabinet and the nearest plinth, and that gap was 0.74 wide for someone 0.9
across. It is 2.8 now, flooded rather than worked out, and the way opens between
2.35 and 2.4.

The reaching is flooded too. A grid of the floor is filled from where you come
in, and every bench, and the floor in front of every bookcase, has to be
somewhere that fill got to. Arithmetic about one wall at a time cannot see a
pinch between two things that were never thought about together, and this one
was invisible until somebody walked at it.

Reachable and not merely free. That distinction is worth a whole layout, and it
went unnoticed for three goes at the room. The benches ran the length of the back
wall with 0.3 between them and stood off it far enough to walk behind, so the
test asked whether there was standing room behind each one and there was, along
every inch of it. The strip was sealed at both ends by the first and last bench
and the only way into it was between two benches. Nobody is 0.3 across. Widening
it from 1.1 to 1.5 to 1.9 to 2.4 made the part of the room nobody could reach
bigger each time, and the test went green for all four.

Not in the aisle. The first attempt before that stood a bench down the middle of
the room and it was in front of the far wall from every angle, which is the one
place nothing should stand. A bench is solid, like a cabinet, so you cannot walk
through one.

Six of them, in the order you meet them coming through the way in: the ball and
chain, the gyroscope, the Newton's cradle, the metronome, the globe, cascada.

**cascada is a toy and had a cabinet.** You stand dominoes up on a floor
wherever you like, push the first one, and the number at the end is how many
fell. There is nothing to win and nothing to lose, which is the whole of what
separates this room from the hall. It had a cabinet out there for the one reason
everything else has one, which is that it is a folder under `games`, and being a
folder under `games` is not an argument about anything.

It is the one toy in here that opens a window. The other five are worked where
they stand because a bench is enough room for them, and this one is not: a domino
run wants a floor, and it wants you to put every domino exactly where you want
it. Four arrow keys at a bench cannot do that, and a version that could would be
a line of dominoes you push, which is a demonstration of the engine's waking rule
rather than the toy. So the bench is what it is and the window is where you do
it, which is also the honest reading of something that already has a repo, a
spec folder and a readme of its own.

The dominoes on its bench do nothing. They are a picture of the toy the way a
cabinet's screen is a picture of the game, and the first of them is leaning into
the second because that is the only part of a domino run anybody needs
explaining.

Two things about that picture were wrong in ways that looked right. A domino was
thin across the run rather than along it, so every one of them stood with its
face to you and would have gone over sideways into nothing: from beside the
bench it read as a curve of dominoes anyway, because from beside a real run you
see edges and not faces. And each one was turned along the tangent of the curve
it stands on rather than at the one in front of it, which are a few degrees apart
on any curve. What a domino has to fall onto is the next one.

The arcade reads `games` off the disk and gives every folder in it a cabinet,
less this one. The test that keeps a repo from going missing used to compare two
lists of cabinets, which was the same thing as the real question only while every
repo had a cabinet. Moving cascada would have passed it and taken the toy out of
the building. It now asks that every repo is a cabinet in the hall or a bench in
the nook.

### It is furnished

The nook is a study off an arcade, and for a while it was a corridor with a
counter down it: benches end to end against a wall, under the same brown grain
and the same confetti carpet as the hall outside. The room out there is a dim
neon box and that is right for it. This one is the one place in the building
that is not selling you a game, so it is furnished rather than lit up.

It is 6.4 by 13.0 rather than 3.0 by 8.0, which is 83 square units of floor
against the first 24.

**The benches are set against the open side**, and the bookcases against the
back wall opposite them, so the floor between the two is one piece 4.9 across
and every part of it is somewhere you can be. Out in the floor is where the
benches were, and the aim of that was floor on both sides of them rather than a
corridor with a counter down it. It got floor on both sides and the far side was
no use to anybody, for the reason above. Against one wall with the books against
the other is the same amount of walking and all of it connected, and the toys
face the books across it, which is also what the room is for.

The row starts past the way in rather than centred in the span. The open side is
the wall the way in is a gap in, and a bench centred down that is a bench in the
doorway.

Which side of a bench you stand on is the bench's own business, and so is which
way your right hand is from there. Both were written down instead, in two files,
as "you come in off the aisle looking along -x, so your right hand is -z". That
was a true sentence about where the benches used to be, and a compiler has
nothing to say about a true sentence concerning the wrong room: moving them left
both of them reading and left left meaning right.

**A wall of bookcases** runs the length of the back wall, across the floor from
the benches. One wall and no more: the open side is the benches now, the far end
is the way in and a bookcase in a doorway is a door, and the closed end is where
the last bench stands. The books are generated and stocked from a seed, so a
shelf holds the same books every time it is drawn. A bookcase restocked every
frame is a bookcase that boils.

**Every spine carries a real title.** Fifty-six of them, all long out of
copyright, down the spine the way an English binding reads. Nothing invented:
a shelf of plausible-sounding titles is one you read twice and stop believing,
and this room is the one place in the building not pretending at you.

Short ones mostly, because a spine is three centimetres across. The title is
scaled to fit the book it is on, which is what a binder does, so `Emma` is set
large and `The Mill on the Floss` small. ASCII only: the engine's font has no
glyph for an accent and would leave a hole where one went.

The title comes off a different part of the same roll as the binding colour,
or every copy of a title would be bound alike and the shelf would read as a
pattern rather than as a library.

It is `text::stencilled` and not `text::drawn`, per the engine's spec 0047. A
drawn word carries a black ground, which on a dark panel is a plate behind the
letters at no cost and on four hundred coloured spines is four hundred black
labels: readable with your nose against the shelf and a wall of dashes from
where you stand, which is worse than no titles at all.

**Sconces** down both long walls are what the room is lit by. There were candles
on the shelves first and they were the wrong thing twice over. There is no
lavishly furnished room in the world with an open flame two inches from four
hundred books, and a room that makes you think about that is a room nobody
relaxes in.

They also answer something the nook never did, which is where the light comes
from. It had two bare lamps in the ceiling with nothing to hang them on, so what
you saw was a bright patch on a surface and no reason for it. A sconce is a thing
you can look at.

Each wall over its own run. They shared one, the length of the nook, and the
open side is not that long, because the way in is a gap in it. The far end of
that shared run hung a lit sconce in mid air out over the aisle, with nothing
behind it, throwing light on nothing. The panelling had been told where that
wall starts and the lighting had not, which is what two lists of the same wall
gets you.

Both walls and not one. Lit from the open side alone the light fell on the wall
it came out of and the four hundred books across the room sat in the dark, which
is the wrong way round. The ones on the back wall hang above the cases rather
than beside them, which is how a library lights a wall of shelves. Six of the
engine's eight go to the nearest of them while you are in here; the cabinets out
in the aisle are too far to be throwing anything you could see from this room.

**A floor of boards**, which is what the nook stands on instead of the arcade's
carpet. The floor is the largest single thing in anybody's view of a room, so it
is the largest single thing saying which of the two rooms you are standing in,
and the carpet out there is confetti on black and reached under here. Planks
down the long way with a dark line between them and a butt joint across each one
at a place of its own, because boards jointed in a line is a grid and a grid is
the one thing the eye reads as a texture rather than as a thing. The plank count
is taken off the nook and not off the quad, so a plank is the same width
whichever way the room is longer.

**A rug** on top of them, laid on the open floor between the benches and the
books with bare boards showing round it. Wall to wall is what it was, which is
not a rug, it is a floor, and it ran under four hundred books where nobody would
ever see it. It was also hiding the confetti rather than fixing it: shrink it to
something a room would actually have and the neon comes back out at the
skirting, which is worse than before.

Twice as long as it is wide and no longer, because the nook is nearly three times
as long as the open floor is wide and a rug that shape is a runner. The band
round the outside is inset the same number of pixels from every edge rather than
the same fraction of one, which is the whole reason the drawing takes a shape: a
square rug stretched over a quad twice as long as it is wide has a border twice
as thick across the ends as down the sides, and nothing else in the picture says
which way it was stretched, so it reads as a badly made rug rather than as a bug.
The medallion stretches where the border does not. A long rug has a long
medallion; a long border is just a mistake.

**Panelled walls**, with a skirting, a dado rail, panels between the two and a
cornice at the top. Panelled below the rail and plain above it, which is what a
panelled room is. Every wall of it you can actually see: not the back one, which
is four hundred books deep, and not the stretch of the open side that is the way
in, because panelling across a doorway is a door. A wall is the largest flat
thing in any view of a room after the floor, and the grain in here was the same
grain as the hall outside, which was the last surface still saying arcade.

None of it does anything. That is what furniture is.

### It says it is there

Nothing in the hall said the nook existed. You found it by walking to the end of
the aisle and happening to look left, which is finding it by accident, and
everything else in that room announces itself: a cabinet has the game's name lit
across the top and that is most of what makes anybody walk up to one.

**A sign hung over the aisle**, level with the end of the cabinet rows. That is
where the hall stops offering you anything and you have to decide whether to
keep going. Hung at the nook's own doorway instead it would only ever be read by
somebody already standing in it.

Not neon. Neon is what the hall is, and the nook is the one room in the building
that is not, so a neon arrow pointing at a panelled study is a sign for the wrong
room. This one is a painted plank on two chains, gilt round the edge, with a
pendant over it. The gilt is the rug's thread and the globe's brass, which are
the only other warm things in the building.

**The end of it is cut to a point**, and the point is the whole of the direction.
An arrow drawn on the face would have to be mirrored on the back to go on
pointing the same way in the world, and a mirrored face is mirrored lettering.
A plank cut to a point is the same shape from both sides and from underneath,
which is the thing a drawn arrow cannot do and is why every pointing sign ever
nailed to a post is cut this way.

The lettering reads forwards from both sides, which the cabinets' does not. A
marquee is a lit sign seen through itself and from behind you read it the wrong
way round, which is right for one. A sign hung across a corridor is not that:
people come at it from both ends.

**The pendant over it** is why you can read it. The lettering could carry a
multiplier over one and read from the far end of the hall with nothing lighting
it, which is the exact thing this room was told off for: a bright patch on a
surface and no reason for it. It goes into the same list as the nook's sconces
and is chosen off the same distance, so it takes a slot off the nearest sconce
while you are out in the hall and loses to them the moment you are in the nook.
A fitting of its own would have had to come out of the cabinets' two, and the
cabinets are the hall.

The typeface is still the arcade's. The engine carries one font and it is a
pixel font, which is the right one for a marquee and the wrong one for a painted
plank. That is a change to the engine rather than to this room.

### The arcade runs physics

This is the first thing in the room with solvers in it. Each toy keeps its own
step and its own solver, so a slow frame does not change how anything swings and
nothing one toy does reaches another. One solver holding all three would only
mean the cradle paying for the wall's contacts and the wall paying for the
cradle's forty eight passes.

### The ball and chain

The toy the nook was built for. The example's own one hangs from eight metres up
and this room is three and a bit tall, so what stands on the bench is the same
build at a thirteenth of the size: a gantry, a chain of ten beads, a ball forty
times the weight of a brick, and a wall of brick that falls over. The full size
one stays where it is, at `cargo run --release --example chain`.

It stands in a tray with a lip, because a knocked brick needs somewhere to end
up and without the lip it slides off the bench and falls through a room this
solver knows nothing about.

The four arrows haul the ball about the tray, both ways about rather than only
at the wall, because the whole of what a wrecking ball asks of you is where to
put it. Shift and up or down wind the chain in and out, which is the one control
a crane actually has and the hardest thing to ask of a link: the length changes
while the ball is hanging on it, so every link has to take up slack or pay out
under load rather than settle something that was already still.

Enter builds the wall again and nothing else. It built the lot, which put the
ball back on its hook wherever it had got to, so pressing it mid swing threw the
ball across the tray. The chain stays where it is now, and stops moving: you
cannot build a wall round a swinging ball, so pressing this catches it first. The
swing is yours to put back.

Nothing is laid through the chain either. A brick built inside something forty
times its weight is a brick the solver has to get out of there, and what it does
is throw it, one of them fifty six units down through the bench. The whole chain
and not just the ball: the beads hang from 0.25 down and the wall stands 0.26
high, so swinging the ball in puts beads over the wall as well. The wall comes
back with a hole where the chain is, and pressing again once it has swung clear
fills the hole.

**The wall is built where it comes to rest**, not where the arithmetic puts the
courses. Laid on the arithmetic's own spots each contact gives up a little before
the solver pushes back, and stacked six deep that came to 0.032 by the top
course, which is 73% of a brick's own height. The wall slumped the moment it
appeared, every brick sat within a whisker of the distance that counts as knocked
down, and rebuilding snapped the lot back up, which was a hop you got pressing
enter at a wall that was already standing. A wall stood up and let go once, with
the resting places read off it, settles 0.0008 instead. A wall with nothing wrong
with it is now left alone entirely.

The gantry's post stands in the corner of the tray rather than beside the hook.
Once the arrows hauled both ways, a post level with the hook was a post the first
pull across the tray put the ball inside.

Two numbers came out of sweeps rather than out of the example. The pull is 0.7:
above it two presses do the whole job and there is nothing left to work, below it
the pumping goes on long enough to stop being a thing you are doing. A brick is a
fortieth of the ball rather than the example's two hundred and fortieth: lighter
than that and a brick caught between the ball and the wall behind it is squeezed
by two hundred times its own weight, and what comes out is not a brick falling
over, it is a brick leaving the bench at a hundred and fifty a second.

The chain hangs about 5.5% long under the ball, which is the same order as the
full size example's 4.6% and is the thing spec 0041's passes are spent on. It
bottoms out near four: 64 passes give 4.8%, 128 give 4.4%, and a heavier chain
gets no further.

### The Newton's cradle

Five steel balls hung in a line, touching. Lift the one on the end and let it go,
and it stops dead on the others while the one at the far end flies out.

A cradle is the case sequential impulses are worst at. A blow has to cross four
contacts, and each pass carries it one contact along, so four passes go on
delivery before anything is solved. It wants 240 steps a second and 48 passes,
against the engine's default of 32.

Restitution is 0.92 and not 1. At 1 the row never stops, which is a perpetual
motion machine and a bug. At 0.92 it runs down in about half a minute.

The frame is sized from the swing, not from the row. Built to the row it was
exactly as long as the balls hanging still, and the first thing the end one did
was swing clean through an upright: the ropes hold the balls and nothing holds
them off the frame, so the frame has to stand where they never reach.

### The gyroscope

A rotor in two rings on a pedestal, pinned through its own middle, which is what
balanced means: gravity pulls on it with no arm to pull on, so it has no reason
to fall and none to walk. Left alone it holds its axis, and holds it to nought
over twenty seconds, spun or stopped.

The demonstration is what happens when you lean on it. Hold an arrow and it
pushes the end of the spindle, and the axis does not go where you pushed. **It
goes sideways**, at a right angle to the push, and it keeps going sideways for
exactly as long as you keep pushing. Let go and it stops walking.

Spin it harder and the same push moves it *slower*. The rate is the torque you
are leaning on it with over the spin's own momentum, so more spin is a bigger
denominator. The dial carries five, from 10 to 20, and the walk comes out within
4% of `τ / I ω` at every one of them, walking 1.32 a second at the bottom and
0.67 at the top, which is round in five to nine seconds. The axis takes about half a second to settle into
walking after a push starts, because the wobble's own period is that long at
these spins, so you watch it dip and then go.

**The push is as hard as it can be while the axis still goes sideways**, which is
the whole demonstration. Swept, by how much of the movement came out sideways
against how fast the axis walked compared to its own spin:

```text
walk / spin    sideways
     0.013       99%
     0.025       98%
     0.051       92%
     0.080       87%
     0.300       60%
     0.410       29%
```

It has been a quarter of this and half of it, and both made a toy you lean on for
ten or thirty seconds before anything happens, which is not one anybody leans
on.

**The push eases on and off over a quarter of a second** rather than switching.
A hand leaning on something is a ramp and not a hammer, and a hammer is exactly
what sets a gyroscope ringing: a torque switched on in one step excites the
nutation with everything it has. Slowing the rotor down to where its spoke could
be read as turning made that ring-up thirty times bigger, six point nine degrees
at the bottom of the dial against four tenths before, and easing the push is what
took it out. The speed the axis walks at swings 1.3 to one over the first second
now rather than 1.6, and the movement comes out *more* sideways rather than less,
91% at the bottom of the dial against 87%.

Dragging harder on the bearings was tried first and is the wrong lever. It
flattened the ring-up a little and took up to 39% off the walk doing it, which is
paying in the thing the toy is for.

**The gimbal's bearings drag**, by a thousandth, and that is what makes it watch
properly. A pushed axis does not settle on its new heading, it rings. The wobble
is four tenths of a degree wide and invisible to look at, so it was measured and
dismissed; what matters is that it is quick, so the speed it adds is the size of
the walk's own. The axis does not walk, it lurches, 23 to one between its fastest
and its slowest, twelve times a second. A real gyroscope does not do that because
its bearings drag.

The drag is on the axis's turning and not on the spin, and the two separate
themselves by speed: the wobble turns at about twice the spin and the walk at
under a radian a second. Swept, against an arithmetic walk of 0.6008:

```text
0.0000   lurches 23.1 to one   walks 0.7224
0.0010            1.1           0.5987
0.0035            1.0           0.5926
0.0100            1.1           0.5499
```

A thousandth is the least that settles it and it costs the walk a third of a
percent. Worth noting what the lurch does to the measurement as well as to the
look: undamped, the axis covers a fifth more ground than it travels, because the
wobble is path and not progress.

**The dial's top end is set by the frame rate and not by the physics.** The spoke
on the rotor has to move little enough between one frame and the next to be read
as going round:

```text
240 radians a second   153 degrees a frame   reads as standing still
 60                     57                   reads as flickering
 20                     19                   reads as turning
```

Drawing the spoke as the ring it would blur into was tried in between, and it is
steadier than steady: nothing moves at all and the rotor looks stopped. What
coming down costs is the walk, since the push came down with the spin to keep the
response gyroscopic. The dial runs 10 to 20 rather than 6 to 15 because the
slow end is what gives first: leaned on hard enough to walk in five seconds, a
rotor at 6 goes only 65% sideways while one at 10 goes 86%.

Stop the rotor and push again and it does the obvious thing instead: it goes
where you pushed, at 26 radians a second squared, which is twenty times the
fastest walk. That is the comparison, and it is why the push is a third of the
rotor's own weight. There is nothing to lean against in a balanced rotor.

The engine has carried Euler's equation since spec 0034, solved implicitly in
the body's own frame, and this is the first thing in the project that shows what
that bought. Everywhere else it is a block toppling, where a right answer and a
plausible one look the same.

**The body is a block and the drawn rotor is a disc.** The engine has spheres and
blocks and nothing else, and a sphere resists turning the same way about every
axis, which is the one thing a gyroscope must not do. They are not a compromise
between each other: a block half as thick as the disc and `√3/2` of its radius
across has the disc's inertia exactly, about its axle and across it both. Two
equations, two unknowns, no remainder.

**The rings carry no weight and are not in the arithmetic at all.** Without them
the thing is a disc on a post, which is a desk fan. They follow the axis and not
the rotor, because that is what a gimbal does: it carries the wheel rather than
turning with it.

**Being balanced is also why it is cheap.** The first build hung the rotor off an
arm, where gravity had something to pull on and it walked round the stand by
itself. That one needed 61,440 steps a second and 4.5% of a core: turning a body
is integrated to first order, and with gravity working on an arm the whole
behaviour sits inside that error. Let go level it must stay level, since the
upright part of its momentum is nought and gravity cannot change it, and instead
the axle climbed +0.99 of the arm's own length in thirty seconds at 960 a second.
Halving the step halved the climb, all the way to nothing at 30,720.

Balanced there is no arm and no climb to integrate away. At the top of the dial,
by where a two second push left the axis:

```text
   960 a second   0.0299 rad, 99% of it sideways, 0.1% of a core
 3,840            0.0293       100%                0.2%
15,360            0.0292       100%                0.8%
61,440            0.0290       100%                3.1%
```

It runs at 3,840. Six passes, because three rods holding one body converge at
once: 6, 12 and 24 gave the same answer to three decimals.

**The walk is measured over a window and not over a step.** Over a step it reads
the wobble instead. A pushed gyroscope goes on nutating after the push stops, at
about twice its own spin, and although that wobble is two thousandths of a radian
wide and invisible to look at, two thousandths at three hundred a second is a
large speed. Measured a step at a time the bench said it was still moving half a
radian a second for ten seconds after it had stopped walking, which reads as the
spin running out. Over a tenth of a second the wobbles cancel and what is left is
the walk: it now reads 0.006 a second within a second of letting go. The spin
itself holds to +1.5% over thirty one seconds, and the axis moves another two
thousandths of a radian and stays there.

**The outer ring turns about the upright and about nothing else**, because it is
bolted to the pedestal, and only the gimbal inside it tips with the axis. Drawn
following the axis, both of them swung, so an axis tipped towards the upright
carried the whole cage over and lifted it clean off its own base. Upright the
axis no longer says which way the frame should face, which is gimbal lock and is
real, so the frame keeps the last answer it had.

**The spindle is drawn turning with the axis and not with the rotor.** It is a
square bar and a real one is round, so spinning it about its own length does
nothing a round rod would do. What it does instead is swell and shrink by the 41%
between a square's side and its diagonal, forty times a second, and since the
spindle is the spine of the whole object the object reads as shaking. Three
things were measured and cleared before this was found: the wobble is 0.4 degrees
at the slow end of the dial and nought at the fast end, the rings' orientation
moves at the walk's own rate and no faster, and the pinned middle sits 0.002 off
and moves 0.00000 between frames at six passes or at sixty four.

The push is applied as a torque rather than through `Body::strike`. Strike is for
hitting a thing: it takes a place in the world and pulls it onto the body's own
surface, which for a push on the end of a spindle rewrites the lever you meant
into whatever lever the block has. Used that way the walk came out seventeen
times too slow, and the error was invisible, because any torque across the axis
sends a gyroscope sideways and sideways was what we were looking for.

### The globe

The kind that stands in an office: a tilted sphere in a brass meridian ring, on
a stand, that you spin with a finger and watch run down.

**It is the one thing on a bench with no solver in it**, and that is not
laziness. A sphere resists turning the same way about every axis, so there is
nothing for a solver to find out: the whole of its motion is one angle and one
rate, and the engine's own `turn` short circuits a sphere for exactly that
reason. What this one is for is the other two things the engine does, which are
wrapping a picture on a surface and lighting it.

It leans 23.44 degrees, which is the Earth's own lean and is why every office
globe in the world leans. The same number draws the two tropics, at 23.44 out
from the middle, and the two polar circles, at 23.44 short of the poles.

**The map is the only thing in this project that came from somewhere else.**
Natural Earth's 1:110 million land, which its makers put in the public domain,
cut down from their GeoJSON to 127 rings of longitude and latitude in
`data/land.txt`. Five thousand numbers nobody here can justify one at a time:
they are where the land is. Everything else about the picture is drawn here, the
sea, the fill, the coast, the grid and the named circles, into an equirectangular
texture 2048 by 1024, which is the layout the engine's own sphere is already
wrapped for.

Rings and not coastlines. Coastlines were taken first and drawn as lines, and a
coastline set gives a blue ball with faint scratches on it. Rings can be filled.

There is one test on it worth naming. **The world has to be where it says it
is**: the test names the Congo, the Amazon, Mongolia, the middle of Australia,
Colorado and Antarctica and asks the map what is drawn there, and then does the
same for five stretches of open sea. Twice this project has laid a picture on a
surface the wrong way round and found out by reading a word off it. A globe has
no word on it, and a mirrored Earth, or one a quarter turn out, looks like an
Earth.

### The metronome

An arm pivoted near its foot, with a heavy bob just below the pivot and a lighter
weight that slides up the needle above it. Sliding the weight up slows it down,
which is the thing about pendulums nobody knows until they are shown it: raising
the weight brings the whole thing's middle of mass closer to the pivot from
below, and what little is left to right it has to do the work over a bigger
swing. Eight notches, and the dial they come out at is a metronome's: 139 ticks
a minute at the bottom notch down to 39 at the top.

Spec 0041 gave the engine distance links and nothing else, and this says it did
not need anything else. **A hinge is five rods.** Pin one point on the arm three
ways and it can still turn any way it likes about that point, which is a ball
joint and lets the arm wander out of its plane and never come back. Hold a second
point on the same axis two more ways and the only turn left is the one about the
axis through them. Five rods for the five freedoms a hinge takes away, which is
exactly determined rather than piled on.

The anchors those rods run to are nowhere near the pivot, and they do not need to
be: a rod is a sphere its end rides, and where the middle of that sphere sits
makes no difference to the holding so long as the three of them pull along three
directions that between them cover everything. What it does decide is whether an
anchor stands in the arm's way, and an anchor in the arm's way is a contact
rather than a hinge. There is nowhere close in to put them. The bob sweeps the
space below the pivot, the weight occupies everything from a tenth of a unit
above it to four tenths, and the swing is the rest.

It keeps going, because a wound one does. At each pass through the upright every
speed in it is scaled by one number, to put the swing back where it was set. A
distance rod is a rule about speeds along a line and nothing else, so scaling
every speed by one number leaves every rule exactly as satisfied as it already
was: there is no jolt to take up and no pass spent taking it. Striking the arm
instead puts it somewhere the rods then have to haul it back from.

**The bench reports the rate it counts, not the rate it works out.** The two part
company by 1.7% at the bottom notch and 5.5% at the top. The mechanism is the
reason rather than the solver: at the top the thing is very nearly balanced, with
0.018 of righting left out of the bob's 0.09, so a fraction of a millimetre of
give in the pivot is a few percent of what is left. Thirty two passes and two
hundred and fifty six agree to a part in a thousand, which is what rules the
solver out. The closed form is kept for the explanation, and for sizing the
winding, which is an energy and is exact.

### Working them

Looking at a bench names the toy and says what to press. Enter does the toy's one
thing: it sets the cradle going, starts or stops the metronome, and builds the
wall again.

The arrow keys work whatever toy the sight is on, and walk you about when it is
on nothing. WASD walks whatever you are looking at, so there is never nothing
that does. Only a press is taken, so a release always goes on to clear the
walking: an arrow held down on the way to a bench would otherwise leave you
walking into it with nothing to let go of. A held arrow repeats and a repeat does
nothing, because a pull is a press and not forty a second.

## Acceptance criteria

- The nook is off the aisle, not in it. — `room::tests::the_nook_is_off_the_aisle_and_not_in_it`
- Nothing on a bench stands in front of a shape. — `room::tests::nothing_on_a_bench_stands_in_front_of_a_shape`
- The nook has a way in, wide enough to walk through. — `room::tests::the_nook_has_a_way_in`
- And every cabinet still has a wall behind it. — `room::tests::every_cabinet_has_a_wall_behind_it`
- The benches fit down the nook, inside it and clear of each other. — `room::tests::the_benches_fit_down_the_nook`
- And no bench stands in the way in. — `room::tests::no_bench_stands_in_the_way_in`
- And you can walk from where you come in to every one of them. — `room::tests::you_can_walk_to_every_bench`
- And from there to the books, which is reachable and not merely free. — `room::tests::you_can_walk_to_the_books`
- A bench knows which side of it you stand on, and which way your right hand is. — `room::tests::a_bench_knows_which_way_your_right_hand_is`
- The rug lies on the open floor, not under the benches or the books. — `room::tests::the_rug_lies_between_the_benches_and_the_books`
- A long rug has an even border rather than one that stretches with it. — `carpet::tests::the_rug_keeps_its_border_even`
- The boards are planks running one way, jointed out of line. — `carpet::tests::the_boards_are_planks_and_not_a_grid`
- Every sconce is on a wall. — `room::tests::every_sconce_is_on_a_wall`
- Every repo is a cabinet in the hall or a bench in the nook. — `tests::it_knows_every_game_the_project_does`
- cascada's dominoes stand on its bench and none hangs off it. — `cascada::tests::they_all_stand_on_the_bench`
- And they are spaced the way cascada says to space them. — `cascada::tests::they_are_close_enough_to_knock_each_other_over`
- And each one falls onto the next rather than past it. — `cascada::tests::each_one_falls_onto_the_next`
- The first one is over, towards the next and not away from it. — `cascada::tests::the_leaning_one_leans_the_right_way`
- And only the first. — `cascada::tests::the_first_one_is_leaning`
- The sign hangs over your head and under the ceiling. — `sign::tests::it_hangs_over_your_head`
- And in the aisle, not through a cabinet. — `sign::tests::it_hangs_clear_of_the_cabinets`
- Its pointed end is a solid with every face wound to be seen. — `sign::tests::the_point_is_wound_to_be_seen`
- And it points the way the nook is. — `sign::tests::the_point_points_at_the_nook`
- A bench is something you bump into. — `room::tests::a_bench_is_something_you_bump_into`
- The sight lands on a bench. — `room::tests::the_sight_lands_on_a_bench`
- And not on one across the room. — `room::tests::a_bench_out_of_reach_is_not_seen`
- The chain is built hanging still. — `wrecker::tests::it_is_built_hanging_still`
- The wall stands until something hits it. — `wrecker::tests::the_wall_stands_until_something_hits_it`
- Hauling it brings the wall down, and takes more than a press. — `wrecker::tests::hauling_it_brings_the_wall_down`
- Nothing ends up off the tray. — `wrecker::tests::nothing_leaves_the_tray`
- The chain carries the ball without stretching further than it did. — `wrecker::tests::the_chain_carries_the_ball_without_stretching`
- The winch hauls the ball up, and never winds a link shorter than what it ties. — `wrecker::tests::the_winch_moves_the_ball`
- The wall can be stood back up. — `wrecker::tests::building_it_again_stands_the_wall_back_up`
- And standing it back up leaves the chain where it is. — `wrecker::tests::rebuilding_the_wall_leaves_the_chain_where_it_is`
- And throws nothing when the ball is in the way. — `wrecker::tests::rebuilding_round_the_ball_throws_nothing`
- The wall is built where it rests, so it does not slump. — `wrecker::tests::the_wall_is_built_where_it_rests`
- And a wall with nothing wrong with it is left alone. — `wrecker::tests::rebuilding_a_whole_wall_changes_nothing`
- The balls hang touching. — `cradle::tests::the_balls_hang_touching`
- Each hangs from a hook that does not move. — `cradle::tests::every_ball_hangs_from_its_own_hook`
- A row left alone stays put. — `cradle::tests::a_row_left_alone_stays_put`
- Lifting one end swings the other. — `cradle::tests::lifting_one_end_swings_the_other`
- The middle is left alone. — `cradle::tests::the_middle_is_left_alone`
- It runs down rather than going on forever. — `cradle::tests::it_runs_down_rather_than_on_forever`
- A swinging ball never reaches an upright. — `cradle::tests::a_swinging_ball_never_reaches_an_upright`
- The frame is wider than the row. — `cradle::tests::the_frame_is_wider_than_the_row`
- The weight sits where the notches are, and stays on the needle. — `metronome::tests::the_weight_sits_where_the_notches_are`
- Every rod is built at its own length. — `metronome::tests::every_rod_is_built_at_its_own_length`
- No anchor stands where the arm swings. — `metronome::tests::the_anchors_are_out_of_the_way`
- It stands still until it is started. — `metronome::tests::it_stands_still_until_it_is_started`
- And swinging, it only swings the one way, which is what the hinge is for. — `metronome::tests::it_only_swings_the_one_way`
- The swing is near what the arithmetic says. — `metronome::tests::the_swing_is_near_what_the_arithmetic_says`
- The weight up is slower, over a dial a metronome would recognise. — `metronome::tests::the_weight_up_slows_it_down`
- The rate it reports is the rate it ticks. — `metronome::tests::the_rate_it_reports_is_the_rate_it_ticks`
- It keeps going, and keeps its swing rather than winding itself up. — `metronome::tests::it_keeps_going_and_keeps_its_swing`
- Pressing it again stops it dead and stands it up. — `metronome::tests::pressing_it_again_stops_it`
- The weight can be moved while it is going. — `metronome::tests::sliding_the_weight_while_it_goes_changes_the_rate`
- The rods hold. — `metronome::tests::the_rods_hold`
- The block weighs what the drawn disc weighs, exactly. — `gyro::tests::the_block_weighs_what_the_disc_weighs`
- It is pinned through its own middle, which is what balanced means. — `gyro::tests::it_is_pinned_through_its_middle`
- No anchor stands where the rings sweep. — `gyro::tests::the_anchors_are_out_of_the_way`
- Left alone it holds its axis, spun or not. — `gyro::tests::left_alone_it_holds_its_axis`
- Stopped, a push puts it where you pushed it. — `gyro::tests::stopped_it_goes_where_you_push_it`
- Spun, the same push sends it sideways instead. — `gyro::tests::spun_it_goes_sideways_instead`
- And the harder it spins the slower it goes. — `gyro::tests::spinning_it_harder_moves_it_slower`
- The walk is the rate the arithmetic works out. — `gyro::tests::the_walk_is_what_the_arithmetic_says`
- Letting go stops it walking. — `gyro::tests::letting_go_stops_it`
- Its rods hold. — `gyro::tests::the_rods_hold`
- The axis walks evenly rather than lurching. — `gyro::tests::it_walks_evenly`
- The outer frame stays on its pedestal. — `gyro::tests::the_frame_stays_on_its_pedestal`
- The rotor turns slowly enough to be read as turning. — `gyro::tests::the_rotor_can_be_seen_turning`
- The drawn rotor is a closed disc, wound to be seen. — `gyro::tests::the_rotor_is_a_closed_disc`
- The world is where it says it is. — `globe::tests::the_world_is_where_it_says_it_is`
- The map is the shape a wrapped sphere wants. — `globe::tests::the_map_is_two_to_one`
- The circles with names are the lean and nothing else. — `globe::tests::the_named_circles_are_the_lean`
- The land is all there, and every ring closes. — `globe::tests::the_land_is_all_there`
- Flicked, it spins and runs down. — `globe::tests::it_runs_down`
- A shelf is stocked, and the books fit on it. — `study::tests::the_shelves_are_full`
- No book is taller than the shelf above it. — `study::tests::nothing_is_taller_than_its_shelf`
- The books stand in a row without overlapping, centred on the case. — `study::tests::the_books_stand_in_a_row`
- The shelves are evenly spaced and all inside the case. — `study::tests::the_shelves_fit_the_case`
- The sconces are spread along a wall, over the bookcases and under the ceiling. — `study::tests::the_sconces_are_spread_along_the_wall`
- The panelling fits between the skirting and the rail. — `study::tests::the_panelling_fits_its_wall`
- And a case holds the same books every time it is drawn. — `study::tests::a_shelf_is_stocked_the_same_every_time`

### Verified by hand

- Coming through the way in, the three toys are a row you walk along.
- Pressing enter sends the far cradle ball out and leaves the middle three still.
- The metronome's lit notch is where the weight is, read from across the nook.
- Four pulls on the ball and most of the wall is down.
- Leaning on the gyroscope and watching the axis go the other way.
- Finding where you live on the globe.
- Walking in and seeing a study rather than more arcade.
- And stopping it and leaning on it again, which sends it tumbling.
- The chain reads as a chain. The beads are half a bead apart, so the links
  between them are drawn as well, or it is a dotted line.

## Out of scope

A toy that keeps its state between visits. All three start the way they were
built every time the arcade does.

Sound. A metronome ticks and a wall falling makes a noise, and the arcade is
silent throughout. The engine has sound and the arcade does not depend on rodio,
so this is a crate boundary rather than a missing feature.

The full size ball and chain in the room. It hangs from eight metres up and the
room is 3.2 tall. What is on the bench is a model of it and the example is where
the real one lives.
