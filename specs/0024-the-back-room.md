# 0024 The back room

**Status:** built, 2026-10-17.
**Date:** 2026-10-17

## What it is, plainly

There is a door in the side wall of the room where the shapes are. Nothing is
written on it.

Behind it is where the arcade keeps its things. Buckets, cable on a reel, a
stepladder, paint tins, a stack of spare chairs, boxes nobody has opened.

Against the far wall is a workbench. Clamped in a jig above it is a block of
small white cubes, standing against a black card so you can see them, and on
the wall over the bench is a board headed what a frame costs, with three
gauges on it: time, worst and draws, each with a bar and, past the end of its
track, what a full bar would mean. Stand at the bench and press a key and
there are more cubes, press another and there are fewer. The gauges say how
hard the arcade is working to draw them, and the bars get longer as the cubes
do.

That is all it does. It is the room where you can watch the building work.

## Goal

A room for the building's own instruments, which have nowhere to stand.

Jake, on where the engine's `load` example belongs in here: cabinets are for
games, the load isn't a game, arguably needs its own category. He is right,
and the category has been missing for as long as there has been anything in
it. A cabinet in the hall is a game, from `games()`, which reads the disk. A
plinth in the gallery is a shape the engine can draw. An instrument is a third
kind of thing and borrowing either of the other two would be a category error
you would notice every time you walked past it.

The name is his, and so is the reason for it: the back room leaves a bit of
mystery, anything could be in there.

## What goes in it

A workbench with the test rig on it: a block of cubes held in a jig, the
dials that change it, and the gauges on the wall above. Then whatever else
turns out to be about the building rather than in it.

Jake's framing of what it must not be, which is what settled the form: load
as a thing you interact with as it is, without launching anything.

It was briefly a small theatre, with seats and a stage, which he then read
back as the wrong room: it's a maintenance room, a theatre might be a good
different room though. He is right. A theatre is a room you are an audience
in, and nobody is an audience here: you are whoever came in to find out what
it costs. The theatre is written down as its own idea and is not this.

**Not a cabinet, and not a launcher.** The first draft of this spec put the
engine's `load` example in a cabinet and called it a service cabinet. Jake:
I don't like load being in a cabinet the same as the games, that blurs mental
models. He had already said the thing I needed to hear once, that cabinets are
for games, and I wrote it into the goal and then built the room around a
cabinet anyway.

The deeper fault is that a cabinet is a shortcut. You press a key and a second
window opens somewhere else, which is a menu with extra steps however the
furniture is carved. This building is worth something because the hall is a
hall rather than a list of games, and a room that exists to launch one program
is the list again with a door on it.

So the rig is in the room, on the bench. Nothing is spawned and there is no
second window. You walk in and it is already running, because it is the same
frame everything else is drawn in.

**The cubes are really there, and not a picture of cubes.** A panel showing a
live view of another scene means drawing that scene into a texture and
sampling it, and this engine cannot: the only textures it draws into are its
shadow maps, and `Renderer::add_texture` takes an image from the CPU. Render
to texture is a real engine spec with a real cost, and it would buy a picture
of a thing instead of the thing.

Which is the better answer anyway. What you are looking at is this building
drawing those cubes, at the building's own samples and lights and shadows. A
picture could lie about what it costs to draw. A block of cubes on a bench
cannot.

**What that measures is this building.** The cubes are drawn by the arcade's
own renderer, so the number on the gauges is what the arcade costs with four
thousand cubes added to it rather than what the engine's example costs on its
own. That is the more useful of the two questions and the one that cannot be
asked from outside.

**On a bench, rather than standing in the middle of the room.** A rig you
stand inside is a wall of cubes you are inside, and every time the dial moves
the view changes and the numbers are about a different picture. A block held
in a jig at arm's length is a thing you look at while you work the dial. The
view holds still, which is the whole of what makes two readings comparable.

**And the spec stops there on purpose.** The room's whole character is that
you do not know what is behind the door until you open it, and a spec that
lists its contents three releases ahead takes that away as surely as a sign
on the door would. What it fixes is the room, the door and the kind of thing
that belongs; what goes in it is decided one thing at a time.

## Where it is

Through a door in the side wall of the gallery, spec 0022's room across the
end of the hall.

**Not off the hall**, because there is no wall left. That is not a preference:
spec 0022 exists because the strip between the cabinets ran out, which is why
the shapes moved to a room of their own. A door in the hall means a cabinet
comes out, and a game losing its place to a toolbox is the wrong trade.

**Off the gallery**, because the two rooms are about the same thing from
different sides. The gallery is where the engine shows what it can draw. The
back room is where it shows what that costs. Standing in one you can see the
door to the other, and that is the whole of the explanation anybody gets.

The gallery is `GALLERY_WIDE` across, which is 6.8, so its side walls are at
plus and minus 3.4. The door goes in one of them, near the end the shapes are
on, so the room behind it sits beside the far end of the building rather than
alongside the aisle. Which side is free of the garden, the cellar and spec
0011's space is a thing to check rather than assume, and the check is one of
the criteria below.

**The door is unmarked.** Every other way into a hidden part of this building
announces itself somehow: the nook has a sign with an arrow, the cellar has
stairs, the space behind the wall has a bookcase that swings, the room out
past the door has a frame around it. This one is a door in a wall with nothing
written on it, which in a building full of lit signs is the loudest thing a
door can be.

## What is in it besides the bench

Clutter. This was going to be a bare room, on the thinking that the back room
is the one the building does not dress up, and Jake: bare is boring, there
could be random clutter and detritus like a normal maintenance closet would
have. He is right, and the first draft of this section had already admitted
half of it, that a bare room and an unfinished room are a thumb's width apart
in a screenshot. Bare is not a decision anybody can see.

So: buckets, a mop, cable coiled on a reel, a stepladder against a wall,
paint tins with the lids half on, a stack of spare chairs, crates, a shelf of
jars of fixings, a broom. The things a room like this fills up with when
nobody is looking, left where somebody put them down rather than arranged.

**Which is a trap this building has fallen into before.** Boxes are
placeholders and Jake has said so every time: a grey block on a plinth is
what a thing looks like before it is built. Clutter made of grey blocks is
nine of them.

The wheel has its own trap, which the jars found. A profile gives the point
along the axis and the width there, and a profile that returns nought for
the first of those puts every ring in one plane: the three jars on the bench
were a flat splat each until somebody looked. Jake: there's weird circles on
the desk. Everything round or turned here comes off `cellar::turned`,
which already makes barrels, bottles, urns and basins, and a crate is a crate
because it has boards and a lid rather than because it is a box with a label.

**And it has to stay out of the way.** Spec 0005's walkways: a body is 0.9
across and wants about 1.05 clear of everything, so the clutter belongs
against the walls and in the corners, not between the door and the bench. A
room you have to pick your way through is a room nobody goes back into.

The one tidy thing in it is the bench. That contrast is the whole character of
the room: a working instrument set up in the middle of somebody's junk.

## What it is not

**A second examples page.** The site lists every example and plays them in
the browser, per spec 0053 of the engine. This room holds the ones that are
instruments, not the ones that are demonstrations, and the two sets overlap
in exactly one thing today.

**A games room.** Nothing in here is scored, won or lost. If something in the
back room turns out to be fun, it has outgrown the room and belongs in a
cabinet in the hall.

**A way to run the engine's examples.** They are on the site and they are in
the repo, and a door in here that opens one of them is the shortcut this spec
spent a section refusing.

**Somewhere to put a thing that has a proper home.** The temptation with a
room like this is that it becomes where anything awkward goes. The test is
whether the thing is about the building. A shape the engine can draw is not;
it goes on a plinth.

## What has to change to build it

**The rig, which is the work.** A grid of cubes with a count, standing on the
bench rather than at the origin; dials that move it; and gauges on the wall
fed by the same `blitzkit::cost` window spec 0023 already fills every frame.
The cubes want their own colour and material so they read as a thing under
test rather than as furniture, and the jig has to bound them: at every count
the dial reaches they stay over the bench, or the room has a wall of cubes
through it.

**White cubes on a black card.** They were brass, against a grey wall, under
one bulb. Jake, with the engine's own example beside it: the maintenance
room version doesn't land as well as the example, which is a stunning bright
white. The example is white cubes in an empty black window, and what makes
it read is the edge between the two rather than the colour of either. A
workshop has a dark card behind the thing on the bench for the same reason,
which is that you cannot see an edge against its own background.

The card has to be wider than the block by half its width each side, because
it stands behind it and a person walks about: at a width and a half the far
side of the block came off the edge of the card as soon as you stood anywhere
but straight in front. And the gauges hang at a height worked out from the
card's top rather than a number of their own, because widening the card
covered two of the three of them, and that one is checked at compile time:
both are constants, so a build that got it wrong does not build.

Each gauge is a bar in a track with its name on its left and what a full bar
would mean on its right, under a heading saying what the whole board is
about. Three coloured bars mean nothing on their own: Jake, standing at the
rig in the browser build, what do the bars mean. They were then named work,
worst and draws, and he read that back as still not conveying meaning, which
is the same fault one step along. A name without a scale is half a label: a
bar half full of nothing in particular. So the board says what it measures,
each row says what it measures, and the end of each track says what filling
it would cost. Time is in milliseconds because the heading says this is a
frame, and a frame is measured in them.

All of those words are drawn into textures at startup, which the readings
could not be. A word that never changes is one texture for the life of the
program; a number that changes every frame is a texture a frame, and this
building has already grown until it stopped once. Time is full at eight
milliseconds, worst at sixteen and draws at four thousand, so a bar pinned at
the end means off the scale.

The board is sized by what is written on it rather than by eye. Every word is
drawn at one height in a font of square cells, so a row's width is
arithmetic: the longest name, a track, the longest reading, and a margin each
side. At a width of one and a half metres the longest reading hung off the
right edge, and the test below found it rather than a picture.

**A dial a person can work.** The nook's toys are turned with the sight and a
key, per spec 0004, and the dials here want the same verbs rather than new
ones: look at the thing, press the key, the number moves. The building
already teaches that.

And the thing you look at is the block, not the bench. Every other bench in
here answers to its own box, which is right for a toy standing on one and
wrong for a thing held in a jig above it. Jake, pointing at the cubes and
getting nothing: if I'm pointing at the thing demonstrated it should be
interactable, same as if I press enter pointing at the table. The sight
takes the block as well, and answers with the bench either way, because the
rig is the bench.

**Nothing else.** No new kind of furniture that launches a process, which is
what the first draft of this asked for. Everything else is a room: a floor,
four walls, a ceiling, a doorway, a lamp in a fitting, and the walk tests
that go with them.

## Acceptance criteria

- The room stands east of the gallery and nothing of the building is in its
  air. — `back::tests::the_back_room_stands_where_nothing_else_does`
- It is closed, and the doorway is the only way out of it. — `back::tests::the_only_way_out_is_the_door`
- Its floor laps under the gallery's rather than meeting it, so the threshold
  is not a face you walk into. — `back::tests::the_floors_lap_rather_than_meet`
- You can walk in from the gallery and out again, with both rooms' colliders
  in the walk. — `back::tests::you_can_walk_in_and_out_again`
- The block stays over the bench at every step of the dial, above its top and
  inside its footprint, with the gauges above the whole of it. — `back::tests::the_block_stays_over_the_bench`
- Every step of the dial is a cube number, so what it draws is a block with
  nothing missing off it. — `back::tests::every_step_of_the_dial_is_a_cube`
- Nothing in the clutter stands on the way from the door to the bench,
  nothing of it is inside anything else, and all of it is in the room. — `back::tests::the_clutter_is_against_the_walls`
- There is room to stand at the bench, clear of it and of everything
  else. — `back::tests::there_is_room_to_stand_at_the_bench`
- The lamp is in its fitting. — `back::tests::the_lamp_is_in_the_shade`
- Every word on the gauge board is on the board: each name level with its own
  bar and ending before that bar's track begins, each full-scale reading level
  with its bar and starting after the track ends, the heading above every bar,
  and the three rows evenly spaced under
  it. — `back::tests::every_word_on_the_board_is_on_the_board`
- The card stands behind the whole block with a margin each way, and is
  behind it rather than a wall away. — `back::tests::the_card_is_behind_the_block_and_under_the_gauges`
- Pointing at the cubes finds the rig, pointing at the bench finds it too,
  and pointing at a wall across the room finds neither. — `back::tests::pointing_at_the_cubes_finds_the_rig`
- Nothing in the room starts a process, and the gauges read the window the
  building already fills rather than a second count of the same
  frames. — `tests::the_back_room_starts_nothing`

Five of those were written after the thing they guard had already gone wrong
in a picture: a block 1.5 across standing through the far wall and the bench,
a door placed where the only way to it was a seventy centimetre gap beside
the last plinth, a floor that stopped at the threshold, gauges drawn on the
back of their own board, and a dial whose steps were round numbers rather
than cube numbers.

That last one is the one worth remembering. 256 cubes in a seven a side grid
is five full layers and a sixth with a third missing, filled from the bottom,
so what stands on the bench is a block with a bite out of the top. Every test
here passed and the room had been photographed four times. Jake saw it in a
picture: why aren't the cubes organised as a cube, there are more layers at
the bottom than higher up. The dial is four, eight, twelve, sixteen and
twenty four a side now.

### Verified by hand

- The door reads as a door and not as a seam in the wall, from the middle of
  the gallery.
- The clutter reads as clutter rather than as placeholders. The crates needed
  a post at each corner before they stopped reading as stacks of boards.
- The bench says what it is without a sign explaining the category.

Still to look at, which is the part only Jake can answer: the room is lit by
one bulb over the middle of the floor and the corners are dark, the clutter
is all along two walls so the west side is bare, and nothing has been done
to the walls at all. Bare was the thing he rejected, and a room with four
things in one corner is bare with a pile in it.

## Out of scope

**Naming the category in the game.** There is no word on the door and no line
in the corner calling it a tooling room. The room is the category.

**Taking the F readout away.** Spec 0023 puts what a frame costs behind a key,
which works anywhere in the building and goes on working anywhere. The panel
in this room is the same numbers bolted to a wall beside the thing that moves
them, which is a different use for one instrument rather than a second
instrument.

**A theatre.** Jake's idea, raised while this one was being written and
better off on its own: a room you sit in and watch something in. Nobody sits
in a maintenance room.

**Instruments that do not exist yet.** A profiler, a collider browser, a
texture memory readout: all plausible, none built, and naming them here would
be the listing this spec just refused to write.

**Render to texture.** A real engine spec, and the thing that would turn this
bench into a screen showing anything the engine can draw. Worth wanting. Not
needed for this, and the real cubes are better than a picture of them for the
one thing in here now, because a block of cubes cannot lie about what it
costs to draw.
