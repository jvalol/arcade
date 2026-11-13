# 0066 Weather that is not always a storm

**Status:** implemented
**Date:** 2026-11-13

## Goal

The sky over this garden has been one storm since spec 0019 built it, baked
once at load off a seed written into `main.rs`. Jake, 2026-11-13: the sky
doesn't always have to be stormy, it can be cloudy, or cloudless blue, etc.

## Why one picture became four

A sky here costs 3.1 milliseconds to bake, measured. Four of them at load is
twelve milliseconds against a moon that takes five hundred and forty, so the
set is free and nothing has to be rebuilt while the building is running.

That is the whole argument for a set rather than a rebake: a texture rebuilt
at runtime is a hitch. The other half of the first draft's argument, that a
cross-fade would need a second backdrop the engine does not have, stopped
being true when engine spec 0099 made a sky something you can push. Two decks
over each other is two draws of a 48 by 24 dome, and only while it is
turning.

## What varies

One knob and two colours, which is what the existing bake already had buried
in it as constants.

- **`cover`**, nought to one, how much of the dome is cloud. It enters the
  bake as `cloud * 1.6 - 0.25 - (1 - cover) * 1.6`, which is the storm's own
  bias with a term that is nothing at a cover of one, so the storm's picture
  is unchanged to the byte. Written the other way round, as a bias worked
  out from the cover, it is the same number in arithmetic and not in f32:
  `-1.85 + 1.6` is a hair off `-0.25` and that hair moved three blessed
  pictures under a sky nobody had changed.

  **It is not a share and it is not linear.** Measured in twentieths:
  everything up to 0.20 is bit-for-bit clear, the first cloud appears at
  0.55, and the dome is 0.5 per cent clouded at 0.62, 4 at 0.70, 16 at 0.80
  and 66 at one. The useful range is the top half, and the four numbers
  below are chosen off that table rather than off the words. Fair was 0.62
  first, which is wisps.

  Overcast is 1.5, past where the bias clears the noise outright, so its deck
  is solid with no break anywhere. At one it was the storm in a lighter grey
  with the same holes in the same places, which is not a lid.
- **`sky`**, the colour of the sky itself. It goes on the backdrop.
- **`cloud`**, the colour of the cloud. It goes on the deck, one layer in
  front, with the sun and the moon between the two. Engine spec 0099.

Both are tints on an instance rather than values in a picture, so both lerp
from one weather to the next. Nothing about a sky is baked except the shape of
its cloud.

**The storm's deck is darker than its sky now**, which spec 0019 ruled out and
gave its reason for: a storm is dark for a sky and bright for a room, a slate
chosen against daylight is the only light this garden has, and those pull
opposite ways. They do not pull on each other any more. What fills the shade
is `garden::filled`, which reads the hour and not the weather, so the deck can
be as dark as a storm wants and the garden under it is lit exactly as it was.
Drawn with the old numbers against the new layers it came back a bright white
overcast with blue holes in it.

**And the colours were measured rather than chosen.** They are multiplied by
`skylit`, 1.35, 1.40, 1.52 at noon, and written to an sRGB target, so a value
lands a good deal paler than it reads in the source. The first set gave a
washed-out overcast and a blue clipped flat at 255. These were read back off
the blessed pictures a pixel at a time and worked backwards.

Four of them. Clear and fair share a blue and differ in their cloud; overcast
is the lid; storm is the dark deck with pale breaks.

## The atlas, and what it was

Jake, playing it: the sky reads like an atlas of continents. It did. Five
things made it, and the first four were all one thing: the picture had stopped
carrying any tone.

- **The cloud's colour was flat.** The whole of the weather had moved into the
  alpha and the texture was white. What the old opaque picture had, and this
  had thrown away, was shading inside the cloud, which came for free while the
  colour ramped from the sky's to the cloud's across the same curve. A flat
  tint behind a clean outline against a flat sky is a map.

  It is off the noise now, not off the alpha. `DEEPEST`, how dark the thickest
  part comes to against the thinnest. Read off the alpha first, which looks
  right and is not: the alpha saturates, so a shade taken from it goes flat
  exactly where there is most cloud to shade.

- **And the two were the same brightness.** Measured: cloud at 149, 152, 160
  against a sky of 116, 165, 234, so the mass was very slightly *darker* than
  what it sat on. Two areas of one luminance in two hues is how a map colours
  its countries.

  The cause was not the tint. A fair-weather cloud only reached about a third
  opaque through its middle, so what you saw was two parts sky to one part
  cloud. `heavy` only arrives at one where the noise does, and the noise does
  not. The ramp has nearly twice the slope now, which puts the core solid and
  leaves the bottom half of it to the edge.

- **The curve was hard.** Spec 0019 wrote `heavy * heavy * (3 - 2 * heavy)`
  and gave its reason: a gradient of grey is haze, and what a storm wants is
  cloud with edges. Right while the picture was a ramp between two colours a
  shade apart. Wrong the moment the edge became an alpha against open sky,
  where it crosses from clear to solid inside a texel and every break has a
  staircase round it.

  A power was tried in its place, to spend more of the ramp near nothing and
  feather the thin end. It does the opposite here: the thin parts are most of
  a fair-weather cloud, so taking them down left small hard islands and read
  more like a map, not less.

- **The picture was coarse and its edges were smooth.** 512 over a whole sky
  is the better part of a degree a texel, and the finest octave was 96 cells,
  so the outline of a break was a curve a few cells long with nothing ragged
  about it. A coastline is smooth and a cloud edge is not. 1024 now, and six
  octaves, the two added carrying a sixteenth and a thirty-second between them
  so the weather's shape is unchanged and only its edges move.

- **And the covers all moved**, because the ramp's slope changed what a number
  means. Fair went 0.78 to 0.60 and the storm one to 0.88: at the old numbers
  fair became a broken overcast and the storm closed every break and became
  the lid. What tells a storm from a lid is that you can see sky through it.

**And none of it was the whole cause.** Jake sent a recording of the running
game after all five were in and the atlas was still there, worse in the window
than in any blessed picture. The rest of it was two faults in the engine, both
found by measuring one row of the garden's sky with the other in place, and
both written up in engine spec 0099: a sky was going through the cut-out depth
prepass and cutting a hard-edged hole in the backdrop behind every cloud, and
the backdrop was appended to the instance list rather than put at its head.
The five above are all improvements and not one of them would have been
noticed under those.

And a third, found the same way a day later: there's a moment where the moon
appears to just clip through the clouds. The deck's instance alpha is the
share of the weather it is drawing, which is exactly one for every second
outside a turn, and the engine reads translucency off that alpha. Sorted
solid, the deck was drawn before the moon rather than over it, and it flipped
back for the two and a half seconds of each turn. That one was bisected by
sweeping the clock across a night at a twentieth of a second a step and
finding the one step where the moon's own middle went from 37 to 158.

And a fourth, from the top of the bridge: the clouds look like they cover the
floor at Valhalla and Heimdall. They did, and everything else past a hundred
metres with them. The deck was scaled by `DECK_FAR / DOME`, 160 over 19, and
19 is not the dome mesh's reach: the mesh is 8 at the middle of an edge and 12
at the apex, so the deck stood at 101 metres rather than 160, which is nearer
than the horizon's own forms at 147.

Fixing it turned up a second thing of the same kind that nobody had looked
for. `field::sees` grows with the room you stand in, 420 in the garden and 682
out on the bridge, and the horizon is a share of it while the moon was a fixed
230: out there the hills stand at 239 and the moon was in front of them.

Everything hung off the eye takes a share of that one number now. The horizon
at 0.35, the deck's nearest point at `DECK_OUT`, 0.40, the moon and the sun at
`moon::OUT`, 0.80, and the backdrop at 0.9. The bodies' sizes became angles
rather than widths in metres, because a width at one distance is a different
moon at another, and both were measured back off what they had been so neither
changed. `garden::tests::the_sky_is_layered_in_order` holds the order at three
standing points, measured off the deck's own vertices, which is the lesson of
the 19: a number that is near the right one is not the right one.

The hour lost to it is worth writing down. The first screenshot taken of the
running game still showed the atlas, and it was a stale binary: `cargo test`
does not rebuild `target/release/capriccio`, so what was photographed was the
engine from before the fix. That is the same trap as the restored backup with
an old mtime, and the tell is the same, a result that disagrees with a
measurement.

Four of the nine tests in this spec are about weather that none of this
touches. The one that was about the picture, `the_storm_is_unchanged`, held
spec 0019's bytes, and this changed every one of them: it is
`the_four_skies_stand_in_order` now, which asks the thing a reader would, that
the four are four kinds of day and not four tints.

## When it changes

Over the first eighth of a day that runs from midnight, one weather
dissolving into the next.

**The first build swapped it, and that was wrong.** It changed the picture on
the dome in one frame at midnight, on the argument that `garden::skylit` is at
its floor there and nobody could see it. Jake, straight away: the sky changes
abruptly, and not smoothly. They can see it. At a day of twenty seconds you
are looking at the sky for a good share of every night, and an instant is an
instant whatever the multiplier is.

`garden::TURNS_OVER` is 0.12, two and a half seconds at `garden::DAY`, and the
whole of the turn is between midnight and hour 0.87, so none of it is in the
light. `garden::overhead` gives back one sky colour, one cloud colour and the
one or two decks to draw: the colours lerp, and the decks are the old picture
and the new one with the share of each as its own alpha. Outside a turn the
second share is nought and nothing is drawn for it.

The day boundary `garden::hour` already has is at dawn, which is the one hour
of the night this could not use, so the weather counts its own days midnight
to midnight.

A day is `garden::DAY`, twenty seconds, so this is a new sky every twenty
seconds. That is a lot of weather and the day length is the lever, not this.

## Thunder belongs to the storm

`garden::strike` has run every eleven seconds since spec 0019 whatever the sky
was doing. A strike now happens only if the weather at its own hour thunders,
which is one filter in `garden::struck` and `garden::heard`: everything
downstream of those, the flash on the dome, the light thrown into the garden,
the thunder and its delay, follows without being told.

## And the sun behind it

Spec 0065's disc is drawn in front of the dome whatever is on it. It is out
under heavy cover, through a smoothstep from 0.85 to 0.98, which sits above
fair's 0.78 and below overcast's one: the sun survives a fair day whole and is
gone under overcast and storm.

A step rather than a fade, because under broken cloud the sun is out or it is
behind a cloud, and which one depends on where the breaks are rather than on
how many there are. Faded by `1 - cover` a fair day would have a permanently
weak sun. And a step between two covers rather than a match on the name,
because the name is a label and the cover is the fact.

## Acceptance criteria

- A clear sky has no cloud on it anywhere. — `garden::tests::a_clear_sky_is_clear_everywhere`
- The four are four kinds of day, and only overcast is a lid. — `garden::tests::the_four_skies_stand_in_order`
- Nothing about the sky ever jumps. — `garden::tests::the_sky_never_jumps`
- The sky stands in layers, in order, wherever you are. — `garden::tests::the_sky_is_layered_in_order`
- A cloud crosses the moon. — `tests::the_cloud_crosses_the_moon`
- Every sky in the set is a different picture. — `garden::tests::no_two_skies_are_the_same`
- The weather holds for a day and turns at midnight. — `garden::tests::the_sky_turns_in_the_dark`
- All four come up over a run. — `garden::tests::every_sky_is_used`
- Thunder only happens under a sky that thunders. — `garden::tests::only_a_storm_thunders`
- The sun is out on a fair day and gone under overcast. — `garden::tests::cloud_puts_the_sun_out`
- The garden under each sky matches its reference. — `tests::the_skies_are_drawn`

### What the controls found

Every one of these was broken on purpose and watched to fail. Two passed their
controls first and were the same mistake twice, which is the one this repo
keeps making: asserting against what I meant rather than against what the code
holds.

- `the_sky_turns_in_the_dark` read the hour of each turn and held it against
  `MIDNIGHT`, which is the constant the turn is computed from. Moving
  `MIDNIGHT` to dawn moved both sides and the test passed on a sky swapping in
  broad daylight. It asks `skylit` now, which is the thing that decides
  whether anybody can see the swap.
- `a_clear_sky_is_clear_everywhere` passed with clear's cover set to 0.2, and
  that one was the control's fault rather than the test's: 0.2 **is** clear, as
  the table above says. The control is 0.7 now, which is where cloud starts.
- `the_sky_never_jumps` passed with `TURNS_OVER` set to nought, and that was a
  third thing again. A turn of no width divides by nothing, the one step where
  the swap happened came back NaN, and every comparison against a NaN is false
  while `f32::max` drops it: the test went blind at exactly the step it exists
  to catch. It asserts every reading is finite now, and the control is a hard
  step that does not divide by nothing.

### Verified by hand

- The four read as four different days rather than as four greys. Run with
  `AT=0.25` and watch the sky turn, or pin one with `SKY=`.

## What it cost the pictures

Six blessed references over the two rounds this spec took, and most of them
are the building looking different, which is the point. `garden`, `field` and `horizon` take whichever
sky the hour they open on rolls, and that is fair rather than storm now: a
picture of the garden as the building opens, with the opening sky in it. If
the roll is ever retuned they change again, and they should.

`the_moon_is_drawn` and `the_sun_is_drawn` are the two that needed fixing
rather than reblessing, and for one reason between them. It
pinned an hour, and an hour is no longer a frame: it landed on a storm, the
sun was correctly not drawn, and the picture of the sun with no sun in it
passed. It names a day as well now, through `garden::under`, and
`standing_when` is the hook that lets a picture ask for a second rather than
an hour.

The moon's went the same way once the cloud was in front of it: a picture of
the moon's face under a fair sky is a picture of cloud, and for an hour it
and `the_cloud_crosses_the_moon` were the same frame. The face is on a clear
night and the cloud has its own picture.

## Out of scope

- **The light following the weather.** `garden::sunlit` is full sun at every
  hour and under every sky, which it already was: the storm has never dimmed
  it. Making an overcast day's light flat and grey is the obvious next thing
  and it reblesses every outdoor picture in the repo, so it is its own change.
- **Rain, and anything falling.** No particles here.
- **Weather that persists.** Each day is rolled on its own, so clear can
  follow storm. Real weather does not and nothing here is asking for a model
  of it.
- **The lightning's own layer.** The flash lights the whole deck, which is
  spec 0016's decision and stands: ninety-nine strikes in a hundred are a
  flash behind cloud, and the cloud's own shape is a better picture of one
  than any patch. It is added after the tint now rather than multiplied into
  it, because a storm's dark deck was putting out its own lightning.
- **Stars.** Jake named them alongside the moon and the sun. A clear night
  here is an empty navy dome.
