# 0016 Lightning and thunder

**Status:** implemented
**Date:** 2026-10-14

## Goal

The garden's sky was already a storm. `garden::storm` drew it and `WEATHER`
turned it, and in all that weather nothing ever happened. A storm that never
breaks is wallpaper.

## The one idea

**Thunder arrives late, and that is the whole exhibit.**

Everybody knows you count the seconds between the flash and the bang, and
almost nobody has watched a thing demonstrate it. Sound goes 343 metres a
second. Three seconds is a kilometre.

So a strike is **one number: how far away it is**, and that number sets
everything else: when the bang lands, how much the cloud lights up, how much
the garden does, how loud it is and what kind of sound it is. All moving
together, which is what makes it worth building rather than a light that
blinks.

Four of those five are worked out from the number. The fifth, what kind of
sound it is, is **picked** from it, and that is the one concession in here.
See the thunder below.

## Behavior

**A strike is its own number and nothing is remembered.** `garden::strike(n)`
works the nth one out from its index, so there is no state to keep and a check
can ask about the four hundredth strike without waiting an hour for it. The
storm is the same storm every time the building opens, which nobody can tell
and every test can rely on.

**Between 250 and 2,800 metres**, which is three quarters of a second to eight.
Near enough at one end that the bang is nearly on the flash, far enough at the
other that you can count to eight, and spread between so no two strikes running
teach the same lesson. Not further: at eleven seconds between strikes a longer
delay puts a flash before the last thunder has landed, which real storms do and
an exhibit about one flash and one bang should not.

**The flash lights the whole cloud deck**, not a patch of it, and that is the
one place the first draft of this spec was wrong. It said a patch on the dome
in a direction chosen per strike. A soft bright patch wants a picture whose
alpha fades, and in this engine a texture's alpha is a stencil rather than a
blend: `blitzkit::text::CUTOUT` throws away every texel under 0.35 before
anything is lit, and the one blend the translucent pass has is alpha over
rather than additive. A patch drawn that way is a disc with a rim you can point
at, which is a moon. It was built and photographed before this paragraph was
written.

Lighting the whole deck loses little. Ninety-nine strikes in a hundred are a
flash behind cloud rather than a bolt, and the cloud's own shape is a better
picture of one than any patch. Measured at night, a near strike takes the sky
from 7 code values of 255 to 70 and a far one to 32.

**And the direction survives anyway**, in the light rather than the picture.
A flash throws the garden's shadows from the strike's own bearing, which is the
half of the direction you can read without being told.

**Its light comes mostly down.** If the flash were the bolt its light would
come from wherever the bolt was, and a bolt low on the horizon would light
nothing in a walled garden: that was measured, with a strike 297 metres off
lifting the sky nine times over and the ground under it by nothing at all,
because the garden's west wall stood in the way of a light arriving at twenty
degrees. What is glowing is the deck, and a deck is overhead, so the light
comes down out of it and leans toward the strike by about half.

**It rides on the sun and not on the ambient**, which is the whole reason it
can be done at all. The ambient is added everywhere and a flash riding on it
would light the cellar. The sun is switched off in any room with no sky
in it, per `noise::Room::sees_the_sky`, so the one light in this building that
can flash without getting indoors is the one that cannot get indoors.

That sentence used to say the sun was shadowed by the roof and leave it there,
which was nearly true and not true. A shadow map holds one depth a texel, and
along the edge of anything thin it leaves a fringe one texel wide that it
cannot resolve away. The bath house ribs are thin fins lit nearly edge on, so
the fringe ran the length of every beam and wobbled as the beam crossed the
map's grid. Jake called it ripples and reported it, in one form or another,
for most of an afternoon.

Only the texel size moved it. Halving it halved the fringe; the grazing bias
swept twelvefold did nothing, and the normal offset taken to nothing or to
four times did nothing. Out-resolving it is a losing trade: the fringe is
about six pixels at a normal viewing distance, a map twice as wide costs
67 MB to halve that, and one four times as wide costs 268 MB.

Switching the sun off where it cannot reach costs nothing. There is no
sightline from the baths, the cellar, the stair or the nook to the sky.

**Near lights the ground and far lights only the cloud.** The patch's falloff
and the garden's are different on purpose: what you are looking at overhead is
a mile of cloud lit from inside and dims slowly, and what reaches the ground
falls off the way light falls off. Measured at night on the surfaces that face
up: 21 code values dark, 27 under a strike 2,470 metres off, 40 under one at
297.

**The flash flickers.** Three strokes a few hundredths apart, because a bolt is
several strokes down one channel and a single rise and fall is a light being
switched on.

**The thunder is recordings and not arithmetic**, which is the second thing
in this building to go that way after the footsteps and for the same reason.
It was made first, out of noise under a wall shaped into arrivals, and it did
not land. A noise burst under an envelope is a noise burst under an envelope
however carefully the envelope was chosen, and thunder is a sound every ear
already knows by heart. `res/thunder/README.md` has the sources and what was
done to them.

**So the character is picked and the loudness is worked out.** Three bands
over the range, near, middling and far, and how far off a strike is chooses
which. That is the concession: how long the bang runs and how much of its top
is left stop being derived and become a consequence of which file was reached
for. They still change with distance, in three steps rather than smoothly, and
how loud it is still moves smoothly across a band. The alternative was to
filter a recording by distance, and `res/steps/README.md` already settled that
one: a recording shaped into something it is not is halfway back to the
arithmetic it replaced.

**A far rumble is longer, not just quieter.** This is the part people get
backwards, and the recordings bear it out. A distant rumble is not a near
crack turned down: the bolt is kilometres of channel and the sound off the far
end arrives seconds after the sound off the near end. Up close those arrivals
are on top of each other and you hear one crack.

**What the recordings said the synthesis had wrong.** Not what it was tuned
for. The near strikes were built bright, on the reasoning that a crack keeps
its high end, and real close thunder has 0.08 of its body at 640 hertz and
nothing at all above 1,300. The synthesis had 0.25 and 0.17. They were not
failing for want of a crack. They were hissing, and no amount of turning the
dials that existed would have found that.

**It is mixed and not queued, and it comes from nowhere.** `play` hands a sound
straight to the mixer and it starts now; `play_at` is a queue, and a three
second rumble waiting its turn behind the room's note would arrive after the
next flash. It has nowhere to be played from in any case: the strike is a
chosen distance and not a place, so a sound coming from a point in this garden
would be the fiction told twice.

**The garden and nowhere else.** This was built the other way first, at a
fifth of itself indoors, on the reasoning that a building is not a sealed box
and that a storm heard in one room is a feature of that room rather than
weather over the place. That overran this spec's own out of scope and Jake's
answer was no.

The reasoning was wrong anyway. The rooms that would have heard it are the
hall, the nook, the cellar and the baths, and three of those are underground.
A fifth of a bang with no wall between you and it reads as a sound effect
rather than as weather, which is the opposite of what it was for.

**And where the sky is, not where the gravel is.** The first answer to that
asked what was under your feet and sounded wherever it was gravel, which works
in this building for a reason that is a coincidence: the garden's floor is
gravel and nothing else here is. A later room with a gravel path through it
would have been in a thunderstorm. Jake's point, and it is the right one.

So it is a place. `garden::under_the_sky` is the one account of where the
weather reaches, and `noise::underfoot` reads it too: what you are walking on
and what is over you are different questions about the same spot, and the
second is the one the first was standing in for. Above ground as well as on
the right side of the wall, because the cellar and the baths run under the
garden, and a point in the garden's own footprint three metres down has the
garden's floor between it and any weather.

A strike whose bang falls due while you are indoors is a strike you missed. It
is marked heard either way, so walking out afterwards does not set off a bang
that should already have gone.

## Acceptance criteria

- How far off a strike is sets when it is heard, how much it lights the cloud, how much it lights the garden, and how loud, how long and how dull its thunder is. — `garden::tests::how_far_off_a_strike_is_sets_everything_about_it`
- The strikes come in order, keep inside their range, and no two running are the same strike. — `garden::tests::no_strike_overtakes_the_one_before_it`
- The flash comes back rather than fading once. — `garden::tests::the_flash_comes_back_rather_than_fading_once`
- What you can see and what you can hear are not the same strike. — `garden::tests::the_strike_you_see_is_not_the_strike_you_hear`
- Every band has thunder, and every one of it reads. — `noise::tests::every_band_has_thunder_that_reads`
- Every distance in the range lands in a band, and the bands run in order. — `noise::tests::every_strike_lands_in_a_band_and_the_bands_run_in_order`
- The storm is heard where the sky is, and the sky is a place rather than a floor. — `noise::tests::the_storm_is_heard_where_the_sky_is_and_not_where_the_gravel_is`

### Verified by hand

- It reads as a storm breaking rather than as a light blinking. Stand in the
  garden and wait.
- A near strike and a far one are plainly different things, and you can tell
  which before the thunder tells you.
- Counting the seconds works.

## Out of scope

**A bolt.** A drawn fork of lightning is a different and harder thing, and the
flash on the cloud is what ninety-nine strikes in a hundred actually look like
from under them.

**Rain.** Separate, and wants the pond to answer it.

**Anywhere but the garden.** It is the only room with a sky, and the thunder
is the garden's too. See above for the version of this that overran the line.

**A soft-edged sprite.** Wanted here and not available: see the flash above.
If the engine ever grows one, the patch is worth another look.
