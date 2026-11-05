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
- The deck fills the width and is centred on it. — `bridge::tests::every_band_is_the_same_width_and_they_fill_it`
- The top of what you see is the surface you walk on. — `bridge::tests::the_deck_you_see_is_the_deck_you_stand_on`
- And it springs flush from the grass rather than off a kerb. — `bridge::tests::it_springs_flush_from_the_grass`
- The colours run into each other rather than meeting at an edge. — `bridge::tests::the_rainbow_blends_rather_than_steps`
- And all seven are still in it, where they were. — `bridge::tests::every_colour_of_the_rainbow_is_still_in_the_picture`
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

## Out of scope

**What is on the far side.** Sky. See above.

**Weather on it.** The field has a sky and spec 0016 has a storm. Rain on a
rainbow is a question for whoever wants it.

**Getting down.** You walk back.
