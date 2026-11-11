# 0060 Inside the hall

**Status:** implemented
**Date:** 2026-11-06

## Goal

Make the end of the trail worth arriving at. Spec 0056 built Valhalla's
ground, its longhouse, its steading and its people, and built all of it from
outside: the hall is one solid box with lit panels standing in for openings,
and the door you walk up to is a door you cannot go through.

Spec 0036 is what is owed here, in Jake's words from 2026-11-04: Heimdall
should let you into a room, there should be a final puzzle with him, and the
room should be Van Gogh or Kinkade inspired. The puzzle is spec 0058 and is
built. This is the room.

Jake, 2026-11-06: Kinkade inside, Van Gogh overhead.

## Why Kinkade is the inside and Van Gogh is the hole in the roof

**Kinkade is a lighting problem and this engine does lighting.** Warm source
out of frame, the surfaces round it taking the light, a falloff you can see
across a room. That is spec 0055's brief in his own words, and the runestone
on the same day proved the engine can do it: a thing that is its own light
and lights nothing reads as neon, and a thing whose light lands on what holds
it reads as glowing.

**Van Gogh is brushwork and this engine has no answer for brushwork.** What
would arrive is his palette on flat surfaces, which is a hall painted odd
colours rather than a hall that looks like a painting. The Night Café is also
the wrong painting for the brief: it is about not being able to sleep, and
this room is rest.

**So Van Gogh gets the one surface that is allowed to be a picture.** A
louvre in the ridge over the fire, and through it a painted night. It is not
the sky and is not pretending to be: it is a panel in the hall, which is why
it can be a painting.

**And that closes a deferred decision.** Spec 0056 parked the aurora with the
cost written down: the weather moves by sliding the sky's own picture, there
is one sky for the whole building, so an aurora over Valhalla is an aurora
over the zen garden. A painted panel under a hole costs nothing anywhere
else. The aurora proper stays parked and this is not it.

## Which way round the hall is, because it is not what it says

`Built` has a `wide` and a `long`, and for the hall `wide` is **24** and
`long` is **9**. `built` passes `HALL_WIDE` into the `long` slot and
`HALL_LONG` into the `wide` slot, so the names have been the wrong way round
since spec 0056 and nothing read them closely enough to notice.

What is actually there: twenty-four metres east to west, nine north to south,
the door in the east gable, and the ridge running north to south across the
middle of the length with a twelve metre pitch each side of it. You arrive at
the gable, you go in at the end, and the hall runs away from you.

The first build of this room was written to the names rather than to the
numbers and every piece of it came out across the hall instead of along it: a
sixteen metre fire through both walls, posts in the wrong rows, benches across
the door. It was caught by printing the hall's own corners rather than by
reading the code, which is the only thing that would have caught it.

**The names are left alone.** Renaming a field of `Built` touches the
outbuildings, the roof, the lit openings and four tests, for no change to
anything anybody sees. What is written down instead is this paragraph and a
note where it bites.

## What is in it

**A fire trench down the length, and it is the only light.** Sixteen metres
of it, stone-kerbed, running away from the door down the middle of the
twenty-four. Not a hearth at the far end: that is a bright wall with a dark
walk to it. A trench is the thing you walk along.

**Posts in two rows, carrying the roof.** A longhouse is a nave and two
aisles, and the posts are the thing that makes the length read as length. They
are also what the firelight has to fall across: a room lit from the floor with
nothing standing up in it is a lit floor.

**Benches down both flanks**, and a high seat at the far gable, which is what
the walk is towards. Somewhere to have arrived at, rather than a hall with
nothing in it.

**The roof's underside, with rafters.** Looked up at, which is what the
smoke hole makes you do. Lit from below by the fire, which is the whole
Kinkade trick in one surface.

**Dark timber and soot.** The ambient in this building is one number
everywhere and at noon it is a third of white, so a pale interior is a flat
lit box whatever the fire does. Built dark, the fire is the only thing in here
past the ambient at any hour, which is the same answer the runestone took.

## The light

**One source, on the fire, and nothing else inside.** Three lamps along the
trench rather than one, because sixteen metres of fire lit by a point in the
middle of it is a hearth with a long dark trench either side.

They flicker on `bridge::burning`, which already exists and is already two
slow breaths crossed at lengths that do not divide. A second flicker written
here would be a second answer to a question that has one.

## The hole and what is through it

**A louvre in the ridge over the fire, above the door.** So that looking up
is what you do as you come in, and so that the thing overhead is over the
thing at your feet.

Cutting it means the two pitches stop being one box each. Each side becomes
three: the run before the hole, the run after it, and the part below it in
between.

**The painted night sits inside the hall and not above the roof.** A panel
under the ridge, spanning more than the hole so no edge of it shows. From
below it is what is through the hole. From outside and above it is facing
away and is culled, so the roof has a hole in it and a light inside, which is
what a lit louvre looks like.

### Which is the whole ceiling now, 2026-11-11

**A small rectangle of sky does not land, whatever is painted on it.** Jake,
twice in a morning: is it supposed to be the northern lights, it looks like
a rectangle of swirling colour, and then a small rectangle doesn't land,
just make it the whole ceiling. A hole in a ceiling is a thing in a ceiling.
What a longhouse gives you when you lie back on a bench is the whole
underside of its roof, and if that is night then the room is open to it.

So the louvre is gone and the night is the two slopes of the roof, the full
length of the hall, with the rafters crossing it. The pitches are one piece
each again, `HOLE_LONG`, `HOLE_DOWN` and `hole_wide` with them.

**Still drawn rather than opened.** From outside this has to stay a roof,
because the building reads from the top of the bridge by its roofline and
one lantern, and a hole in the ridge was a hole in that too.

**And the rafters hang under it rather than in it.** A plane through a box
leaves a seam along it, so at their old height the sky cut every rafter in
the hall lengthways. `RAFTER_UNDER` is written as `NIGHT_UNDER` plus half a
rafter and a margin, said at build time rather than in a test the way
`back::GAUGE_UP` says its own.

**The sky itself was the other half of the fault.** It ran to a cobalt that
clipped in blue once `NIGHT_BURNS` had multiplied it, so the swirl was
carrying the picture and a swirl carrying a picture is a pattern. It is
darker now, the swirl survives at 0.42 of its old strength, and there are
fifty-four stars off a hash rather than seven in a table: a night has a
great many faint ones and a few that catch you.

**Drawn rather than photographed.** Cobalt ground, swirl, and stars that are
a bright core in a wide warm halo, which is the same construction as the
runestone's glow because the reason is the same: this canvas clips at one, so
a star that is only bright is a white dot.

## Behavior

- The hall's door is a way through. You walk in.
- Inside: a fire down the middle, posts, benches, the roof overhead.
- The fire is the only light in the room and it moves.
- Overhead, the whole underside of the roof painted as night.
- Nothing is asked of you in here and nothing is counted.

## Acceptance criteria

- You can walk in through the door and out again. — `valhalla::tests::the_hall_is_a_way_in_and_a_way_out`, walked with the game's own `walk` from the lane to the middle of the floor and back.
- And the hall is hollow rather than a box with a hole in the front of it. — `valhalla::tests::the_hall_is_hollow`, walked from inside the door to each gable.
- The posts stand clear of the way in. — `valhalla::tests::nothing_stands_in_the_doorway`
- Everything in here is inside the hall, which is what the axes being the wrong way round broke. — `valhalla::tests::everything_inside_is_inside`
- Nothing drawn in here shares a plane with anything else drawn in here. — `valhalla::tests::nothing_inside_is_in_one_plane_with_anything`
- The fire is the only light inside the hall. — `valhalla::tests::the_fire_is_the_only_light_in_here`
- The night is the whole ceiling: a quad down each slope, the length of the room, hung under the roof and rolled to the pitch. — `valhalla::tests::the_night_is_the_whole_ceiling`
- And the roof over it is whole. — `valhalla::tests::the_roof_is_one_piece_down_each_slope`
- The night is dark and its stars are the only thing in it that burns. — `valhalla::tests::the_night_is_dark_and_only_its_stars_burn`
- And the room as a whole is drawn. — `tests::the_hall_inside_is_drawn`, which is the check for the three things a number cannot see: a fire that washes every surface to cream, a triangle of open sky at each gable, and a lit panel painted across a doorway you can walk through. All three were in the first build.

### Verified by hand

- **Walking up to it along the lane**, `POSE=-145.0,26.0,15.15,-1.5708,0.0`
  with `--done` and `AT=0.78`. The door is an opening with the fire down the
  length of the hall through it. It was a lit panel until the hall was
  hollow, and then it was a painted rectangle in front of a room.
- **Down the hall from just inside**, `POSE=-153.5,26.0,15.15,-1.5708,0.02`.
  Three rounds: the first washed every surface to cream, the second had the
  coals as a sixteen metre slab of white, the third is a trench.
- **Looking up at the hole**, `POSE=-162.0,26.0,17.5,-0.667,1.013`. The
  rafters ran across it in the first build, which is a grating and not a
  hole.
- **And at noon**, the same pose with `AT=0.22`. A dim timber room with a
  fire in it rather than a flat lit box, which is what the dark surfaces are
  for.

## What this turned up and did not fix

**Spec 0059 did not exist**, and twelve places in this crate cited it: the
typed field, `aim::CARET`, the field's leaf litter, and the bridge's piers and
balusters. Three pieces of work took the same number and none of them wrote
the file. Fixed the same day: 0059 is the typed field, which is what the
engine's 0087 points at; the litter went back to spec 0040 and the parapet to
spec 0055, which own the places they are in. `check-specs` reads the gap in
the numbering now, which is the one unambiguous part of this.

**`Built::wide` and `Built::long` are the wrong way round for every building
up here**, not only the hall. Left alone, with a paragraph above instead,
because renaming them touches the outbuildings, the roof, the openings and
four tests for no change to anything anybody sees.

## Out of scope

**A puzzle in here.** Spec 0036 asks for a final puzzle with Heimdall and
spec 0058 is it. A second one past the gate is a corridor, which spec 0055
already refused.

**Anybody sitting.** `people` draws figures standing and a sitting pose is a
different piece of work. They stand about the fire, which is what people do
round a fire.

**Smoke.** There is no volumetric anything in this engine and a grey box over
a fire is a grey box.

**The outbuildings.** Three of them and none of them is a room. The hall is
the one you are meant to arrive in.

**The aurora.** Still parked, with spec 0056's reasoning intact. The panel
here is a panel.
