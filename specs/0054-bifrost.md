# 0054 Bifröst

**Status:** implemented
**Date:** 2026-11-04

## Goal

Jake, 2026-11-04, answering the one question spec 0036 left open: the end of
the trail should be a reward, a prize, something like a rainbow bridge to
Elysium or Valhalla.

**Which is Bifröst, named.** A rainbow bridge to Valhalla is the Norse one
out of the human world up to Asgard, with Heimdall standing guard at the top
of it, and at Ragnarök it breaks under the riders crossing. Jake's idea
arrived at the myth on its own and he took the name when it was pointed out.

Finish what every room asks and a bridge comes up out of the field, west of
the garden, and goes up into the sky. You can walk it.

## What this settles, and what it does not

**Spec 0036 said the end should be one of the three rooms waiting without a
reason to be built**, and warned in the same breath that inventing a fourth
room as the prize is how you get a room nobody wanted. This is not one of the
three, and the warning is the reason to be careful rather than a reason to
stop: a bridge is a crossing and a view, not a place to furnish. Nothing is
put at the far end of it in this spec.

**So what is the prize?** Height, and the only sight of the building from
outside it. Eleven rooms and you have been inside all of them; nobody has
ever seen the arcade whole. The climb gives that and nothing else gives it,
which is spec 0036's own test for whether a thing is worth adding: it uses
the whole building rather than adding to it.

**What is at the far end stays open.** A hall at the top is the fourth room
0036 names, and it is Jake's to want or not. This builds the bridge and
leaves the top as sky.

The myth argues against leaving it there. A rainbow arcs up and comes back
down to the ground; Bifröst goes up and *arrives*, and what it arrives at is
a threshold with somebody standing on it. The first build ends level and in
mid-air, which reads as unfinished rather than as deliberate, and the story
says why. A threshold and a watchman would answer it without being a room to
furnish, and the building has drawn people since spec 0033. That is the next
decision and it is Jake's.

## Behavior

**It is not there until the building has been done.** Every variant of
`trail::Asked` has to have been given. Eight of them today, and the list is
written to grow: the rule is every variant, not a number, so a room that
starts asking something joins the condition without this spec being touched.

**No announcement.** Spec 0036 is plain that there is no HUD and no panel of
ticks, and that a clue must be a thing that happened rather than a thing
somebody put up. The bridge is simply there the next time you look west, and
nothing says so.

**Where it springs from.** The field, out past the garden's west wall, which
is the one part of this building with open sky and distance in it. It starts
18 metres west of where the garden's ground stops, on the line you walk out
through, and runs 100 metres further west while it climbs 26. A bridge off an
indoor floor would want a hole in a roof.

**The run and the rise are a pair and not two numbers**, because what they
come to is whether this can be walked at all. A collider in this engine is a
box that cannot tilt, so a slope is a staircase to the thing climbing it, and
a riser over `walk::STEP` is a wall. At 26 over 100 the steepest riser is
0.28 against a step of 0.42. Steeper and it is a thing to look at.

**It eases out of the ground and eases into the top.** The first curve here
was a quarter circle, which rises fastest where it leaves the ground: 1.29
metres up in its first 1.46 along, which is a wall to walk at.

**What it looks like.** Six metres across, with seven colours running into
each other across that width the way a real one's do. The colours are past
one, which in this engine is a thing that glows with no light on it: a
rainbow is light rather than paint. Nothing holds it up.

**Seven stops and not seven bands.** They were bands, each 0.86 wide and
flat, and Jake said the colours should blend. They are a picture now, with
the seven at the middles of the widths they used to fill and the picture
running between them, so those seven places are exactly the colours they
always were. A picture cannot hold anything past one, so it carries the hue
and the instance colour carries the glow.

**Six metres across** because a body is 0.9 and every walkway in this
building has been built too narrow first. A bridge you edge along is not a
prize.

**You can walk it.** Walking up it is the whole of the reward, so a bridge
you can only look at would be a picture hung in a field.

**What holds you up is not what you see.** What you stand on is one box the
full width every 0.6 metres, which is a staircase fine enough to walk and
far too fine to draw. What you see is one ribbon swept along the arc. Both
come out of `bridge::high`, which is the one place the shape is written.

**One ribbon and not a row of boxes.** It was 48 lengths of 7 bands, each
turned to the pitch where it lay and lapping the next, and that held up for
exactly as long as every face of one was a single flat colour: a join you
cannot see is a join nobody minds. The moment the rainbow became a picture
laid across the width, every join turned into a bright chevron and every end
cap into a stripe, because an end cap wears the picture too. Shrinking the
lap swapped chevrons for seams. A ribbon has no joins to show.

**And the top of what you see is the surface you walk on.** The drawn deck
used to hang its own thickness above its treads, which is a step up out of
the grass at the foot, a step down onto spec 0055's landing at the top, and
shin deep for the hundred metres in between. Jake saw the step at the foot.

**The top is an end and not a drop.** It stops climbing and the last stretch
runs level, so that arriving is standing somewhere rather than walking off
the end of something.

**And the eye keeps up with the feet.** The camera's height eases toward
the body's over `EYE_LAGS`, which turns a stair into a slope and is the
right thing everywhere else in the building. It is a time constant, so
what it settles at is the rate you are climbing times that, and nothing
here climbed fast enough for the number to matter until this did: run up,
the feet gain better than twenty a second and the eye parks two metres
under them, which is further than the eye is above the feet to begin with.
The camera went below the deck and stayed there for the steep half of the
climb, looking up at the underside of the rainbow. Jake, with a recording:
there is some clipping where the player ends up beneath the bridge for a
bit, and he said he went straight, which he had.

What hid it is that his feet never left the deck. Three tests already
walked, ran and lurched up here and all of them measured the body. The
lag is now capped at one step, which is the discontinuity the easing
exists for and so changes nothing about a stair.

It is also the first fault in this building whose size depended on the
refresh rate. The climb was quicker the more frames it got, because a step
is taken at most once a frame and carries you along by a share of your own
width: the foot to the top was 2.38 seconds at sixty and 1.87 at a hundred
and forty four. Measured with the cap off, the least the camera cleared
the deck by was 0.18 at sixty, and under it at everything above: -0.13 at
ninety, -0.63 at a hundred and twenty, -1.14 at a hundred and forty four.
Every test ran at a sixtieth. Jake's screen does not.

**And the climb runs at the pace it was given.** That free reach per
frame was the whole of why the deck was quick, and it was an accident
rather than a choice: against a run of 23.5 a second the climb measured
39.9 at sixty frames and 50.9 at a hundred and forty four. `walk::RIDES`
takes the risers out of the way instead of stepping over them, so the
deck is walked, and the climb is 4.5 seconds foot to top at every rate.
A walk up it is 8.5. `CARRIES` is the lever on both, and it is now the
only one.

**Steering is still quicker than going straight, and nothing here stops
it.** Pressed against the kerb the body is stopped by something it cannot
climb; the probe in `walk::over_a_step` looks the way it was refused,
reads the deck through the verge, calls it a step and pays the reach out
again. Measured mid deck at a run of 23.5: straight covers 23.5 a second,
and against the kerb up to 39.7. Three guards against it were written and
each took a real stair down with it, so this is recorded rather than
fixed.

## The stone at its foot

**Because nobody can tell what any of this is.** Jake, 2026-11-05: a person
seeing it for the first time would have no idea it is meant to be Norse.

Spec 0006 is the precedent and settles it. That spec exists because the
nook is off the end of a wall and nothing in the hall says so, and its own
words are that everything else in the room announces itself: a cabinet has
the game's name lit across the top of it, and that is most of what makes
anybody walk up to one. A place nothing names is found by accident.

**Carved, not hung and not lit.** Spec 0006 chose a painted plank over
neon for the nook, on the grounds that a neon arrow pointing at a panelled
study is a sign for the wrong room. The same test sends this one to stone.

**Runes, a band, and a stone that is out of round.** The first build was
the arcade's own pixel font, in Latin capitals, on a box. Jake: it looks
like a random grey slab, it does not land as a nordic runestone. All three
parts of that were the fault.

The font says arcade however it is coloured, so the name is cut in younger
futhark instead, which needs no font at all: a rune is a stave and some
branches, and that is why they were cut into stone. The band round the
face is doing more work than the runes are, because an Uppland stone is an
inscription inside a ribbon and the eye reads that outline first. And the
stone is `garden::rock_mesh`, the same out-of-round lump the garden's set
stones are cut from, squashed thin so its face is flat enough to carve.

The name in letters stays, small, under the band. That is every runestone
at every site in Scandinavia, and it is the only part of this anybody can
read.

**Deep, which is the one that decides whether it looks like it will stay
up.** Jake: it is still a really thin slab, and rocks around the base do
not make it look like it is not about to fall over. Both true. At a third
of a metre against two and a half standing it is a blade seen edge on,
and no amount of spoil round the foot buys what mass buys. It is nearly a
metre through now.

That also says what the packing is and is not for. It is the mark of a
thing that was dug in, and it was reached for as an answer to stability,
which it never was: the stones still up after a thousand years are the
thick ones.

**A slab, and not a column with two stones beside it.** The first setting
read as genitalia and Jake said so. Both halves of it were mine: the lump
is an ellipsoid, so it narrows at the top exactly as it narrows at the
foot, and the packing was five stones with the two largest either side of
the base, which is a matched pair flanking a column.

So the stone has a profile of its own now instead of a sine: full width
from the grass to better than two thirds up and shouldered over above
that, which is what a runestone is, and it is half again as broad. And
the packing is seven small ones, none of them large, none opposite
another, most off to one side the way a thing that was dug and backfilled
is.

**It is set in the ground, not balanced on it.** Jake: it feels like a
slab with no reason to be standing that way, no support, and no telling
whether there is more under the ground like an iceberg. The lump is an egg
and its bottom pole sat exactly at the grass, so it was an egg on its tip.
The garden's set stones are the same shape and get away with it because a
third of each is under the gravel; this was the only one standing on
nothing.

A metre is under now, which is better than a quarter and is what keeps a
standing stone standing. What says so is the width at the grass line and
the packing round it: five stones, none the same size, none square to each
other, each sunk past half. A clean line where rock meets grass is a prop
on a lawn, and what a raised stone has round it is the spoil and the
wedges that hold it up.

**A plinth, not a cone.** Jake: it needs to be a lot thicker at the
bottom. The first answer multiplied the whole profile by a term that
fell off over the stone's entire height, which is the shape of a cone,
and from thirty metres it read as a haystack. Weight low down is not a
taper from top to bottom. It is a body of roughly one width with a
swelling coming out of the ground under it, and a crown that is blunt
rather than pointed, because a stone that has stood a thousand years has
a weathered top.

So the shape is three things rather than one. A body that holds its
width to near the head and then rounds over fast. A swelling at the
foot, gone by two thirds of the way up. And that swelling given mostly
to the thin axis, because a slab that fattens evenly stays the same slab
and only gets bigger. A metre and a fifth through and one and four
fifths across where it leaves the grass, eight tenths by one and a half
through the body.

**A heap, not seven boulders.** Jake: the stones around it do not make
visual sense unless it was a gravel or rubble pile it sat on. Right, and
the reason is that the ground under the seven was dead level. A handful
of rocks placed near a base is a handful of rocks that happen to be near
a stone, however they are arranged.

What a raised stone sits in is everything that came out of the hole and
everything that got wedged back in after. So the ground itself is raised:
a bank, as one skin, banking against the stone's own foot ellipse and
thinning to nothing at a wandering rim, with the loose pieces lying on
that rather than standing in for it. Coarse at the collar, where the
packing wedges that hold it plumb would be, and fine at the rim.

Two things fell out of building it. The seven were drawn with the
standing stone's own mesh, which is a hundred and seventy six by two
hundred and twenty four: a quarter of a million quads of footing under a
forty thousand quad monument. They are the garden's eleven by seven lump
now. And the heap has a footprint, which the stone's offset from the bridge
never allowed for: at the offset it had, the bank's edge came within two
centimetres of the ramp's corner, and none of it is solid, so touching
the ramp is rubble you walk through. The offset is the heap's reach plus
a margin now, in both directions, rather than a number of its own that
happened to agree.

Worth writing down how that number was got wrong twice on the way. The
first reading of it said the apron ran a metre onto the ramp, which it
never did: the deck is a half strip and not a plane, so where the heap
reaches furthest across the deck's line it is already well past the
deck's end, and the shortest line between them is a diagonal. The test
measures that distance rather than either axis, which is the only reason
the second reading was caught.

The carved face moves up the stone with it, or the writing goes
underground.

**A ribbon on a dressed face, not a frame round an empty board.** Two
faults, both of them a width worked out in the wrong frame. The panel is
a share of the way round the stone, and a share of the way round a stone
that now tapers is a strip that narrows going up: the band's bottom row
came out twice the width of its top and the name read as if seen from
below. It is held to a constant arc now, giving back exactly what the
taper takes.

And the band was drawn at the edges of that panel, which made the frame
three times wider than the seven runes inside it. Seven runes stacked
down a metre and a fifth cannot be made much bigger than a hand, so what
had to move was the frame. The dressed face is wider than it was and the
ribbon hugs the lettering, leaving plain stone either side, which is
what the carved stones actually look like. Narrowing it caught a third:
the ribbon's top was the same number as its width, and they had only
ever agreed by accident, so the column ran out of the bottom of the
picture and three of the seven runes were drawn off the stone.

**Bumpy, and the writing lies on the bumps.** Jake, looking at the first
build: the rune is on a slab on a stone, and the stone has a picture of
rock on it but is not shaped like one. Both were the same fault. The
carving was a flat quad standing off a smooth lump, and you could see the
plate's own edge.

So the stone is forty thousand quads of its own surface, seven scales of
wander over a ball, flat shaded. Wave counts that share no factors:
doubling them, one two four eight, lines every scale's crests up on the
same meridians and the stone comes out fluted. None finer than a quarter
of the steps there are, or the last scale has two samples to a wave and
shimmers instead of bulging.

And `stone_point` is the one account of where that surface is, so the
carved face is built from it too: the same patch of the same rock, lifted
off itself by a hair. The band bends over every lump because it is on
every lump. This building would not spend twelve thousand quads on one
object anywhere else, and Jake asked for it here, because this is the
start of the last part of the trail.

**And the small scales keep most of their weight.** Halving each against
the last is how a landscape is made, and it draws a boulder with gentle
angles, which is what Jake called the first rough cut. Real stone is
mostly small bumps with a shape somewhere under them.

**The face is dressed, because a rough stone and legible writing are one
question.** A thin bright line over that surface breaks up and stops being
readable, which it did: almost unintelligible, in Jake's words. So the
small stuff comes off the written part and the shape of the stone carries
on under it, which is what was done to them before they were cut.

**The writing is drawn squat, because the face it lands on is not
square.** The picture is a square and the patch it is laid on is about
twice as tall as it is wide, so anything drawn square arrives stretched:
the column of runes merged into one wiggling line and not a letter of it
could be read. A rune is written about twice as wide as it is high in the
picture, to arrive about twice as high as it is wide on the stone.

Fattening the chisel was the wrong answer and was tried first. At 0.026 a
stroke was half the width of the rune it belonged to, so every branch ran
into its own stave and consecutive runes touched. What the writing needed
was to be bigger, not bolder.

**And nothing lights the stone.** There was a lamp off the face, put there
to say the glow came out of the groove. What it did was wash the face pale
and take with it the contrast the glyphs were relying on. Jake: glowing
from within was the wrong instruction, the glyph has to read as lit from
inside and the source does not have to be seen at all.

**Granite, which is what they are.** Uppland's stones are granite and
gneiss almost without exception. Drawn as countable crystals rather than
as noise, because noise at this size is television static and what says
granite is grains you could pick out: feldspar, quartz and mica over a
ground that is none of them, with a little lichen because a stone that has
stood in a field for nine hundred years has some.

Closer together in value than a mineral chart would have them. At the true
spread, mica against feldspar, a stone two metres off reads as camouflage
and the carving is lost in it.

**And alight.** Jake asked for the runes to glow, somewhat mystically.
Colour past one burns with nothing on it in this engine, which is what
the rainbow and the cellar's neon already are, and the groove throws a
little of it on the stone round it.

What reads as mystical is not the brightness but that it will not sit
still: two slow breaths crossed at lengths that do not divide, so it
never repeats on a count anybody could feel, and never down to nothing,
because a rune that goes out is a rune somebody switched off.

The pigment's own colour taken up rather than a new one. Red earth in a
groove, lit from inside, is the stone doing something. A blue-white would
be a different object sitting in the same hole.

**Painted, which is what was done to them.** Darker letters were tried
first, since what you read in a carved groove is its shadow, and there is
no shadow in a groove here: at 0.14 against a 0.33 stone under a night
field the lettering could not be seen at all, and it took turning it
scarlet to prove it was being drawn. Runestones were painted, commonly in
red earth, so the thing that fixes it is the historical answer rather than
a workaround.

**Stencilled and not drawn.** The engine's `text::drawn` keeps the ground
opaque, which is free and right for the nook's dark board and is a black
patch on a pale stone. `text::stencilled` is there for exactly this.

**And the field keeps clear of it.** `field::open` has only ever kept trees
out of the building, so one grew through the stone, and the low stretch of
the deck runs through the canopy of anything sown under it. The clearing
holds whether or not the bridge is up, because a clearing that appears when
somebody finishes the building is trees vanishing. It reaches as far as the
deck is lower than a tree and no further: a six metre avenue running the
whole hundred is a lane pointing at the thing spec 0036 says nobody should
be able to tell is there.

## Acceptance criteria

- Nothing is drawn and nothing is solid until every `Asked` has been given. — `bridge::tests::nothing_until_the_building_is_done`
- One `Asked` short leaves it away. — `bridge::tests::one_short_is_not_done`
- Every question the building asks is in the list the bridge reads. — `trail::tests::every_one_of_them_is_in_every`
- It springs from the ground and ends level. — `bridge::tests::it_rises_out_of_the_field_and_levels_off`
- It leaves the ground at an angle you can walk at. — `bridge::tests::it_leaves_the_ground_at_a_walkable_angle`
- No tread rises more than `walk::STEP`. — `bridge::tests::no_tread_rises_more_than_a_step`
- And a coarser tread would not, so that check can fail. — `bridge::tests::a_coarse_tread_would_not_be_walkable`
- The treads lie under the deck that is drawn. — `bridge::tests::the_treads_lie_under_the_drawn_deck`
- You can walk from the foot to the top. — `bridge::tests::walked_from_the_foot_to_the_top`
- And walking beside it does not climb, so that walk is of the deck. — `bridge::tests::a_walk_off_the_side_does_not_climb`
- It is walked up, run up and lurched up without going under it. — `tests::the_bridge_is_walked_up_rather_than_climbed`, `tests::the_bridge_is_run_up_without_falling_through_it`, `tests::a_long_frame_does_not_drop_you_through_the_deck`
- And off the straight. — `tests::the_bridge_is_climbed_off_the_straight_without_falling_through`
- The kerb keeps you on it. — `tests::the_kerb_keeps_you_on_the_bridge`
- And the camera stays over the deck at every refresh rate, not only sixty. — `tests::the_camera_stays_above_the_deck_however_fast_the_screen_is`
- And it is climbed at the pace it was given, at every refresh rate. — `tests::the_bridge_is_climbed_at_the_pace_it_was_given`
- The deck fills the width and is centred on it. — `bridge::tests::every_band_is_the_same_width_and_they_fill_it`
- The top of what you see is the surface you walk on. — `bridge::tests::the_deck_you_see_is_the_deck_you_stand_on`
- And it springs flush from the grass rather than off a kerb. — `bridge::tests::it_springs_flush_from_the_grass`
- The colours run into each other rather than meeting at an edge. — `bridge::tests::the_rainbow_blends_rather_than_steps`
- And all seven are still in it, where they were. — `bridge::tests::every_colour_of_the_rainbow_is_still_in_the_picture`
- Nothing is sown where it comes down, or where the stone stands. — `bridge::tests::nothing_is_sown_where_the_bridge_comes_down`
- It is clear of the garden. — `bridge::tests::it_crosses_nothing`
- And inside the ground it stands on. — `bridge::tests::it_stays_inside_the_ground_it_stands_on`

### Verified by hand

- **Looked at from out in the field**, 13 metres east of the foot and 35 to
  the north, with the trail filled in and the camera posed from a temporary
  block that was taken out again. The arc reads, the seven bands read, and
  the glow carries against the dusk sky.
- **Not yet walked by hand.** `walked_from_the_foot_to_the_top` drives the
  game's own walk up it, which is the check that matters, but nobody has
  stood on it.
- **Seen from the garden's west gap**, which is where you come out and so is
  the view that decides whether you notice it at all. You cannot miss it: it
  glows against a dusk sky with the field's trees either side of it. Read
  end on, though, it is a wedge of colour rather than a way up. The seven
  bands run along its length, which is how a rainbow is coloured and is the
  one direction that gives the eye nothing to measure distance by from the
  one place everybody will first stand.

- **The stone photographed from three places**, posed through `POSE` with
  the trail filled in: face on from four metres, three quarters on from
  seven, and from twenty out where the silhouette is all there is. Face
  on the seven runes read and the ribbon frames them. At twenty metres it
  reads as a blunt slab set into a packed bank, which is what the three
  rounds before it did not: a cone, then a stone in a flat ring of
  cobbles, then a frame round an empty board.

## Out of scope

**What is on the far side.** Sky. See above.

**Weather on it.** The field has a sky and spec 0016 has a storm. Rain on a
rainbow is a question for whoever wants it.

**Getting down.** You walk back.
