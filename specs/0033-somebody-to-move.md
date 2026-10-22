# 0033 Somebody to move

**Status:** implemented
**Date:** 2026-10-21
**Built:** 2026-10-21, the same day it was written.

## Goal

Put people in the corridor, so that the move the hotel makes is a thing you
watch rather than a thing you are told.

Jake watched spec 0030's demonstration and said it would make more sense if
there were humanoid bodies actually moving rooms. He is right, and the reason
it needs saying is that this building has an answer on file which does not
work.

## The settlement this overturns

Spec 0029 decided what stands for a guest: **the light under a shut door.**
Nobody is drawn, a lit strip means somebody is in there, and the whole move is
forty strips going out and coming back on in a wave.

That was the right call for a corridor at rest and it is the wrong one for a
corridor in motion. A row of lamps blinking is a row of lamps blinking. It
reads as a lighting effect, which is what it is, and no amount of timing turns
it into people changing rooms.

**The light stays.** A guest at rest is still a strip under a shut door: there
is nothing to see in a room you cannot enter, and the corridor at rest looks
right. What changes is that a guest the wave has reached comes out, crosses,
and goes in next door.

## This is the building's first person, which is the part to decide first

Nothing in this arcade has ever drawn a human. Three things are waiting on the
answer and this spec does not get to settle it alone:

- The hall of mirrors, whose hard part is that the arcade has never drawn the
  player.
- The whole-building game, where a mystery with nobody in it is a trail of
  objects.
- This.

**A bad human is worse than a lamp**, because a lamp is not pretending. The
bar is that a figure reads as somebody at corridor distance, in the dark,
without inviting anybody to walk up and look at its face.

So the first build is **one figure, standing still, in one screenshot.** No
walking, no wave, no instancing. If it does not read, none of the rest is
worth writing.

## Behavior

### What a person is made of

Turned profiles, which is what this building makes anything round out of.
`cellar::turned` takes a profile and is already drawing barrels, bottles,
urns, sconce bowls and a fountain's basin.

A figure is a handful of them: a head, a body tapering to the shoulders, and
legs. **No face and no hands.** Both are detail the corridor's distance cannot
carry, and a smeared face is exactly the thing that turns a figure into a
mistake.

One mesh, drawn as an instance per person with a colour of its own, so a
corridor of them is one draw.

### They come out when the wave reaches them

The wave already has a front, at `aleph::TRAVELS_AT` of six rooms a second,
and `open_by` already stands a door open while the front is passing it. A
guest steps out of that open door, crosses the corridor, and goes in at the
next number.

**They cross, and that is the good part.** Rooms alternate sides, so room n
and room n+1 face each other: moving up a room is a diagonal walk across the
corridor, and a dozen of them at once is a criss-cross rather than a queue.
It will read as everybody moving at once, which is the thing the lamps could
never say.

Only the band near the front is out at any moment, so the count on screen is
small however long the corridor is. The hotel is still endless and still
nobody is counting it.

### Nothing to walk into

A person has no collider. Spec 0013 says a collider that disagrees with what
is drawn is this building's commonest fault, and this is a deliberate
exception rather than an oversight: a corridor where the demonstration can
pin you against a wall is worse than one you can walk through.

### The one at the desk

`at_the_door` is somebody wanting a room and has never been anything but a
line of text. A figure stands at the desk while there is one.

And when the desk takes the guest you walked out of the register with, that
one walks in and takes room 1, which is the first time anything in this
building arrives because of something you did.

## What this must not become

**A crowd.** The hotel is full of endlessly many people and we are drawing the
handful the wave is currently passing. Any attempt to show more is a lie about
what is on screen and costs a frame for it.

**A character.** No face, no name, nobody to talk to. The figures are what the
light under the door was: a sign that somebody is there.

**A reason to drop the lights.** A guest at rest is still a strip under a
door, and the corridor at rest must look exactly as it does now.

## What building it found

**It did not read, and the spec's own bar is what said so.** One figure
standing at the desk passed at a glance, and then Jake walked up to it and
said it is a creepy mannequin. The bar here was that a figure reads as
somebody at corridor distance without inviting a close look, and the lobby is
exactly where somebody stands next to one.

Two wrong answers first, both recorded where they were tried. Lighting it
better changed nothing, because the problem was never that it was dark.
Faceting it to seven sides made it worse in both rooms: a crown that closes
to a point is a spike, so a mannequin became something hooded.

**It was one colour from crown to toe.** That is what a mannequin is. A
single material in the shape of a person is the thing a shop window puts
clothes on, and nothing on the figure said coat, or trousers, or head. So it
is four parts in three colours now: a torso, a head, two legs and two arms,
and the coat is not the colour of the trousers and the head is not the colour
of either. Four sets of three, so a corridor is not one person repeated.

**And its head was a fifth of it, where a person's is a seventh.** Doll
proportions are most of what reads as uncanny before anybody is close enough
to look for a face. The head is a seventh now, and it is an ellipsoid rather
than a cone, which is the other half of why the old one read as hooded.

No face and no hands, still. What makes somebody read as dressed is where the
colour stops, not what is drawn on them.

**They were mannequins first.** The guests were given a colour around 0.3,
which is dark on paper, and came out near white. A colour here is an
unclamped multiplier rather than a paint, and the corridor's sconces are 1.85
at the red end, so a hallway of them read as a shop window. At about a tenth
they are in the dark the corridor is actually in.

**And their arms were inside out.** An arm is the leg's own mesh, shorter and
thinner, and the first build hung it from the shoulder with a y scale of minus
one. A negative scale turns every triangle in a mesh inside out, and a face
wound away from you is drawn nowhere at all, so what was on screen was the
inside of an arm: a pale slab flapping off each shoulder. The limb is wide at
the top anyway, so it only had to start lower.

**The legs swing rather than turn**, because `with_rotation` pivots a mesh
about its own origin and a leg's origin is its foot. Rotating one swings the
hip off the body, and flipping the lathe to move the origin reverses the
winding, which is the fault above. A foot that slides fore and aft and lifts
as it comes forward is what is left.

**The crowd was bounded twice**, once by the band the loop runs over and once
by throwing away anybody whose crossing had finished. Two guards that each do
the whole job mean neither is tested: breaking either one left both tests
passing. There is one now.

**And the first version of `they_cross_to_the_next_number` tested
`aleph::at`.** It worked out both doors itself and checked they were on
opposite sides, which is true of the corridor however wrong the crossing is.
Sending everybody to room n+2 instead left it green.

## Acceptance criteria

- A figure has a surface: the turned parts are not degenerate, which is how
  the candles, the clock dial and the fire's coals were all invisible at
  once. — `people::tests::a_figure_is_not_nothing`
- A head is about a seventh of somebody and it is domed rather than
  pointed. — `people::tests::a_head_is_a_seventh_of_somebody_and_it_is_domed`
- Nobody is one colour from crown to toe, and no two wear the same
  coat. — `people::tests::nobody_is_one_colour_from_crown_to_toe`
- Nobody is drawn for a guest the wave has not reached, and everybody the wave
  is passing is. — `people::tests::only_the_ones_the_wave_has_reached_are_out`
- They walk from their own door to the next room's, which is across the
  corridor. — `people::tests::they_cross_to_the_next_number`
- The number on screen is bounded by the wave's width rather than by how long
  the corridor is. — `people::tests::a_longer_corridor_is_not_a_bigger_crowd`
- The legs swap, and a foot is only off the floor on the way
  forward. — `people::tests::the_legs_swap`, `people::tests::a_foot_lifts_only_on_the_way_forward`
- They are in nothing you can walk into. — `people::tests::nobody_is_solid`
- The corridor at rest is unchanged: every taken room still has its strip
  lit. — `aleph::tests::a_room_at_rest_is_still_a_light_under_a_door`

### Verified by hand

- **One figure, standing still, before anything else is built.** Does it read
  as somebody, at corridor distance, in the dark. This is the whole spec and
  no test can see it. It failed this twice before it passed, and the thing
  that fixed it was not in the geometry.
- And at two metres, in a lit room, with somebody looking straight at it,
  which is the condition the hall of mirrors would be nothing but.
- A dozen of them crossing reads as everybody moving up a room, and not as
  traffic.
- The desk having somebody standing at it is better than the desk not having
  somebody standing at it. It may not be.

## Out of scope

**The player's own body.** Still nothing of you. That is the hall of mirrors'
question and it is a harder one, because a figure seen from outside and a
figure seen from inside your own eyes are not the same problem.

**Anybody in the register room.** The guests in that book are endless strings
and there is no sense in which one of them could stand in a hall.

**Animation worth the name.** Legs that swap is the ceiling here. Anything
beyond it is a character and this spec says not to build one.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
