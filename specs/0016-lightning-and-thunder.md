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
everything else. One number into five, all moving together, which is what
makes it worth building rather than a light that blinks.

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
would light the cellar. The sun is shadowed by the roof and already measured in
`room::outline` as reaching no room but the garden, so the one light in this
building that can flash without getting indoors is the one that is already kept
out.

**Near lights the ground and far lights only the cloud.** The patch's falloff
and the garden's are different on purpose: what you are looking at overhead is
a mile of cloud lit from inside and dims slowly, and what reaches the ground
falls off the way light falls off. Measured at night on the surfaces that face
up: 21 code values dark, 27 under a strike 2,470 metres off, 40 under one at
297.

**The flash flickers.** Three strokes a few hundredths apart, because a bolt is
several strokes down one channel and a single rise and fall is a light being
switched on.

**The thunder is made rather than recorded**, per spec 0044: noise under a
wall, shaped into arrivals. How far off sets when it plays, how loud it is, how
long it runs, how many arrivals it is made of and how much of its top is left.

**A far rumble is longer, not just quieter.** This is the part people get
backwards. A distant rumble is not a near crack turned down: the bolt is
kilometres of channel and the sound off the far end arrives seconds after the
sound off the near end. Up close those arrivals are on top of each other and
you hear one crack.

**Kilometres of air are a wall and not a tilt**, and the first build of this
got it wrong. It was one running average, which is six decibels an octave, and
six an octave leaves the hiss roughly where it was: measured, a rumble from
2,800 metres still carried 0.29 of its body at 1,280 hertz. A noise burst with
its top still on is a waterfall. Four stages is twenty-four an octave and takes
it to 0.02. A near crack keeps its top, because that is what a crack is.

**A roll of thunder is several arrivals and not one fade**, which was the other
half of it. The first envelope fell 0.098, 0.056, 0.047, 0.040, 0.020 down the
run and never once turned back up, which is a noise burst going out. What a
roll is is the sound off one part of the channel arriving, then another, then
the ground and the cloud handing some of it back, at times with no rhythm to
them. A near strike is allowed to be one arrival, because up close they are all
on top of each other.

**And the body stays where a speaker can sound it.** Thunder's own spectrum
falls about six decibels an octave, so starting from brown noise rather than
white looked right and was not: with the wall over it as well the tilt is
counted twice, and every distance came out with all of it under eighty hertz.
That is honest about thunder and silent on the machines this will be heard on.
White under a wall is flat up to the corner and gone above it.

The corners are each stage's own, which is not where the wall ends up: four
stages are each three decibels down at their corner, so the run of them is
twelve down there and its real corner is about four tenths of the number in the
source. That was reasoned wrong first, by moving the number from 170 to 260 and
measuring the body move by a twentieth.

**It is mixed and not queued, and it comes from nowhere.** `play` hands a sound
straight to the mixer and it starts now; `play_at` is a queue, and a three
second rumble waiting its turn behind the room's note would arrive after the
next flash. It has nowhere to be played from in any case: the strike is a
chosen distance and not a place, so a sound coming from a point in this garden
would be the fiction told twice.

**A fifth of it indoors.** Thunder carries into the building, because a
building is not a sealed box and because a storm you can only hear in one room
is a feature of that room rather than weather over the place.

## Acceptance criteria

- How far off a strike is sets when it is heard, how much it lights the cloud, how much it lights the garden, and how loud, how long and how dull its thunder is. — `garden::tests::how_far_off_a_strike_is_sets_everything_about_it`
- The strikes come in order, keep inside their range, and no two running are the same strike. — `garden::tests::no_strike_overtakes_the_one_before_it`
- The flash comes back rather than fading once. — `garden::tests::the_flash_comes_back_rather_than_fading_once`
- What you can see and what you can hear are not the same strike. — `garden::tests::the_strike_you_see_is_not_the_strike_you_hear`
- The air takes the top off a far rumble and leaves a near crack its own, and both keep a body a speaker can sound. — `noise::tests::the_air_takes_the_top_off_a_far_rumble_and_leaves_a_near_crack_its_own`
- A far rumble rolls rather than fading once. — `noise::tests::a_far_rumble_rolls_rather_than_fading_once`

### Verified by hand

- It reads as a storm breaking rather than as a light blinking. Stand in the
  garden and wait.
- A near strike and a far one are plainly different things, and you can tell
  which before the thunder tells you.
- Counting the seconds works.
- The thunder indoors reads as weather outside rather than as a sound effect.

## Out of scope

**A bolt.** A drawn fork of lightning is a different and harder thing, and the
flash on the cloud is what ninety-nine strikes in a hundred actually look like
from under them.

**Rain.** Separate, and wants the pond to answer it.

**Anywhere but the garden.** It is the only room with a sky. The thunder is
heard everywhere, which is weather rather than a second storm.

**A soft-edged sprite.** Wanted here and not available: see the flash above.
If the engine ever grows one, the patch is worth another look.
