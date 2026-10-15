# 0019 A pondoria

**Status:** implemented
**Date:** 2026-10-13, the garden given a day the same day

## Goal

A room you stay in, which this building does not have.

Jake's word, on a play on ponder. Every room here is for doing: a cabinet to
start, a bottle to turn, a ball to roll, a planet to walk to. The only thing
you can do in this one is stay, and the room has to be worth that.

## Behavior

### The experiment already ran once

Spec 0011 built a room with nothing in it, and said so in as many words: "a
space, very large and very dark, with nothing in it to look at and no reason
given for it being there", with "no toy, no cabinet, no key" ruled out
expressly. Spec 0015 struck that and put a solar system in it, and said why:
"A hidden empty space is worth one moment."

That is the finding and it is not in dispute. An empty room is a thing you
walk through once. So a pondoria is not an empty room, and the whole of this
spec is what it is instead.

### The second minute has to differ from the first

**Nothing here changes on its own.** The fire in the cellar flickers, which is
a loop: the tenth second and the hundredth are the same second. The koi swim a
circuit. The orrery turns, and turns back when you press L. Every room is in
the state you found it in, for ever, and that is why there is no reason to be
in any of them once you have worked out what they hold.

So the one thing this room must have is a change you cannot hurry. The test is
whether you can tell how long you have been in the room by looking at it.

**It may loop, and this spec said otherwise for an afternoon.** The first
draft ruled loops out, on the ground that a loop says nothing about how long
you have been watching. That is true of the cellar's fire, whose period is a
fraction of a second, and false of a day, and the thing that separates them is
not whether they come round but how long they take against a visit. Anything
that genuinely never repeats eventually stops changing, which fails the next
person through the door: a fire that burns down to nothing leaves a room with
cold ashes in it for ever.

### It asks nothing

No cabinet, no bench, no toy, nothing under the sight, no line in the corner
offering a key. `use_what_i_see` finds nothing here and the sight stays plain.

This is a real constraint rather than an absence, and it has to be defended in
code, because every other room in this building answers you and the machinery
to answer is reached from everywhere.

The one thing it may have is somewhere to sit. The building has benches and
every one of them is a table with a toy on it; there is nowhere in it to sit
and do nothing.

### Where it goes: the garden

Settled, and it is the garden. The other two readings are kept below because
they were real and may be again.

**A day in the garden lasts twenty minutes.** The sun goes round once, east to
west and under, and three things follow it: the light's direction, colour and
strength; what the sky's own picture is multiplied by; and how much of their
light the lanterns are giving.

Twenty minutes is the number that serves both halves of the rule. The sun
moves eighteen degrees a minute, which is a shadow you can watch swing, and no
one visit sees it come round.

**The hour it opens at matters as much as the length.** It opened just short of
noon, and Jake played it and could not tell the garden was doing anything: that
is the flattest part of the whole day, where two minutes takes the sun from
0.95 of its height to 0.95 of its height on the other side and the brightness
does not move at all. It opens in the late afternoon now. A minute in is
visibly dimmer, two and the sun is nearly down, four and the lanterns are up.
The first visit is the one that has to show the room is alive.

**The sun it moves is the garden's alone, and the ambient it moves is
everyone's.**

~~It changes the garden and nothing else. The directional light is the one
light in this building that keeps the day, and `room::outline` already records
as measured that the hall, the nook, the cellar and the baths are roofed and
the sun reaches none of them: "a measurement of both with the sun at full and
at nought came back the same to three places."~~

Struck 2026-10-14, on two counts.

The quotation is of a measurement of two rooms, the hall and the nook, and
this spec read it as four. The two it did not cover are the two the sun turned
out to reach: the baths were lit through their walls by a shadow map stretched
over the solar system, which is spec 0016's note and took an afternoon to
find.

And the day moves far more than the sun. `garden::filled` drives the scene's
ambient, which is added everywhere by definition, and it runs from (0.03,
0.04, 0.09) at night to (0.15, 0.18, 0.29) at noon, three to five times. The
cellar is three storeys from any sky and its median brightness moves 23 per
cent across a day, measured at one camera at five times of it.

Whether that is right is a question for somebody looking at it: a cellar that
dims at dusk is arguably what a cellar does. What it is not is nothing.

**Noon is the garden as it was.** The garden had one state for its whole life
and that state is now the top of the day; everything the day adds is either
side of it. Nothing was taken away to make room for this.

**The shadows had to stop crawling.** A sun that moves was the first thing in
this building to move the shadow map, and spec 0015 of the engine fitted that
map to the scene bounds as the light saw them, so every frame covered a
different patch of ground and every edge crawled. It is fitted to the bounds'
own sphere now, which is the same width from every direction. The trade is in
that spec: a texel over this building goes from 78 millimetres to 110.

**The sky is weather you can watch going over.** It drifted slowly enough that
nothing was seen to move, on the reasoning that a sky is different when you
look up again. Jake's answer to that is the one that matters: nobody is in the
garden long enough to look up again, and a sky that only changes while you are
not watching gives you no reason to stay.

A cloud crosses the whole sky in about a minute now, and it goes in two
directions at rates that do not divide. One alone is the picture sliding past
and the same cloud comes round as soon as it has gone by; crossways as well,
the dome's disc walks about the square picture and never takes the same path
twice, so the shapes themselves keep changing. The storm's noise wraps in both
axes, which is what lets this run for ever without a seam.

That is also the half of this room that works inside ten seconds. The day works
over minutes and the weather over seconds, and between them there is something
moving at whatever length of time somebody gives it.

**The lanterns earn their light.** They were lit at all hours against a sky
that was always the same. They come up over a minute and a half as the sun
goes down, and they are the only light in the garden by the time it is dark.

**The sky is never black.** What is overhead at night still has to read as
weather rather than as a lid, so the storm's picture stays and what comes down
is the day behind it.

### The two readings not taken

**Past Neptune.** The far end of spec 0018's line, where you arrive after 42
seconds of riding and there is nothing further. It is the loneliest place in
the building and it is already built. What it lacks is any change at all: out
there even the sun is fixed.

**A room of its own.** The most work and the most likely to end up as spec
0011 did, because a new room starts empty and the temptation is to furnish it.

### What is in it

Deliberately almost nothing, and what little there is must be the change
rather than scenery round it. One thing that moves and a place to sit facing
it. If a second thing is wanted, the right question is whether the first is
not enough rather than what else would look good.

## Acceptance criteria

- The weather goes over while you watch, in two directions that do not
  divide. — `garden::tests::the_weather_goes_over_while_you_watch`
- A day is long against a visit and plain inside a minute. — `garden::tests::the_day_is_long_against_a_visit_and_plain_within_one`
- And it comes round, which is the point rather than a flaw. — `garden::tests::a_day_comes_round`
- The lanterns come up as the sun goes down, and take their time over it. — `garden::tests::the_lanterns_come_up_as_the_sun_goes_down`
- The sky is never black. — `garden::tests::the_sky_is_never_black`
- Noon is the garden as it was, so the day took nothing away. — `garden::tests::noon_is_the_garden_as_it_was`
- The sun it moves reaches no room with no sky. — `noise::tests::the_sun_is_switched_off_where_there_is_no_sky`

### Verified by hand

- The sky is visibly different eighteen seconds later. Confirmed 2026-10-13.
- The garden at morning, dusk and midnight are three different rooms.
  Confirmed 2026-10-13: at dusk the lantern is the brightest thing in it and
  lays a pool on the wall behind, which it never did under a sky that was
  always the same.
- Nothing in the room answers the sight and the corner offers no key, which
  was already true of the garden and is why it was the right room.
- It is worth a second visit. Only Jake can say that, and the honest version
  of this criterion is that the room fails if he does not go back to it.

## Out of scope

Sound, which is its own question and a loop by nature. Anything with a state
the player sets, which is a toy. Day and night anywhere but in the room this
spec settles on, because a clock that runs in one room and not the next is two
accounts of what time it is.
