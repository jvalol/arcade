# 0057 The building from outside

**Status:** implemented
**Date:** 2026-11-05, amended 2026-11-07

## Goal

Make the view back east from the top of the bridge worth the climb, because
spec 0036 sold the climb on it: the prize is "height and the only sight of
the arcade from outside itself". Spec 0055 said in as many words that it does
not pay off yet, and its watchman is about to ask you what the place looks
like from out here, which is spec 0058.

## What you actually see from up there, measured

Posed on the landing at `bridge::top`, which is (-136.85, 26.00, 15.15),
looking east with the eye at 27.55:

- The arcade's ground storey is roofed at 3.21, rising to 4.50 over the
  corridor, across a footprint of 34 by 27.5 metres.
- The imaginarium stands on that roof and is lidded at **18.8**, 13.1 square.
- The near corner of the ground storey is **115 metres off and 21.5 metres
  down, which is 10.6 degrees**. The imaginarium's lid is 131 off and 7.2
  down, which is **3.8 degrees**.

Two things follow and both of them decide the work.

**You are not looking at a roof, you are looking at a skyline.** At four
degrees a flat roof is a sliver a few pixels deep. Nothing laid on it reads:
not a bay, not a seam, not a stain, not a fall to an outlet. What reads is
whatever stands up off it, as a shape against the sky, and whatever is lit.

**And there is one lantern up there, not two.** This was written here as two
before anybody counted. Spec 0038 puts a way out on each of the cube's six
faces, so one comes out upward and one downward; the downward one is the
hatch in the floor and stands outside nothing, which leaves a single box on
the roof. `aloft::outside_tests::one_lantern_stands_on_the_roof` holds the
count.

**It was blind, and it is not any more.** `door_recess` closes every one of
those openings with a back and four sides, on purpose, because a boxed
opening is what makes a doorway dark from inside the room. That one is now
the exception: no back, a kerb as deep as a recess and its back together, so
the box outside and the cap on it stand exactly where they stood. What is up
there is a roof lantern over a hole in the imaginarium's ceiling.

**The arcade is not what you see. The imaginarium is.** Shot from the
landing, the whole building is one tan block on the skyline, about 250 pixels
of a 1600 pixel frame, with a parapet and one bump on it. That block is the
imaginarium's tower. The ground storey is fifteen metres lower and reads as a
dark line along the horizon. So the roof that this spec is about is
`aloft::roof` at 18.8 and not the lids at 3.21, which is the opposite of
where anybody would start.

**And the bridge is in the way of its own view.** Standing on the landing and
looking back, the deck runs away under the sight and the two kerbs rise above
the horizon to left and right, so the bottom two thirds of the frame is the
thing you have just walked up. See *Out of scope*.

## Behavior

Nothing is added that would not read at 131 metres and four degrees. The test
of every piece here is its outline against the sky, or that it is lit.

**The lantern is lit.** Its four standing sides are glazed rather than
plastered and burn at `aim::ALOFT_LANTERN`. Warm, where the room under it is
white: the imaginarium's own six lamps are `Vec3::ONE` because the one room
with nothing above it should read as the pale one, and this is not that
light. Against pale lime at a hundred and thirty metres a white lamp is pale
lime.

**And a lamp hangs in the shaft**, `aloft::lantern_lamp`, white like the
room's own six because it is the room's light and not the roof's. Blind,
that lamp was impossible: it lit the imaginarium's ceiling straight through
the slab, which `tests::the_imaginarium_is_drawn` caught at once as a blob
over the stair with nothing above it to explain it. A point light in this
engine casts no shadow unless it takes one of the two slots that do, and
those are ranked by how deep a lamp sits inside its own reach, so a lamp on
an empty roof would take one on and off. With the hole cut there is nothing
to explain: light coming down a lantern is what a lantern is for.

**The glazing still burns on its own, and has to.** The lamp is inside the
kerb and these are its outside faces, so they point away from it and take
nothing off it. Colour here multiplies the lit result, so a panel with
nothing on it goes out when the world does, which is backwards for a lamp.
Lit far past one and biased hard warm, what survives the clamp is the
warmth: amber at night, apricot at dusk, pale yellow at noon, never white.
Shot from `bridge::top` at four times of day.

**Cutting the back out opens the shell, so the shaft has glass in it.**
That ceiling is a floor in one of the room's three frames, so an open shaft
is a hole somebody walks into and falls out of the building through, and
the hatch in the floor is the only way out meant to be there.
`aloft::lantern_lid` is a pane in the head of the kerb, not drawn, because
what you see from under it is the capstone's own underside. Putting that
collider up at the capstone instead was tried and three checks named it in
a minute: two faces in one plane, a thing outside the shell in no dressed
box, and a piece of the room outside the room.

**The capstone stays stone.** Glass over glazed sides was tried and the
whole lantern came out one flat tan mass at night, because the colour
clamps to the same value on both and the box loses its top. What says
there is a box up there is the dark slab and the shadow line under its
oversail.

## Acceptance criteria

- There is one lantern on the roof and it stands on the roof. — `aloft::outside_tests::one_lantern_stands_on_the_roof`
- The light in the room has a hole over it. — `tests::the_imaginarium_is_drawn`, the reference picture, which caught the lamp when there was no hole.
- Cutting the back out leaves no hole in the shell. — `aloft::lantern_lid` is in `aloft::solid`, and `aloft::outside_tests::everything_outside_the_shell_is_dressed`, `aloft::tests::nothing_drawn_in_here_is_in_one_plane_with_anything` and `standing::tests::a_run_is_one_room` all read it.
- Nothing stands on the roof that cannot be seen from the landing. — `tests::the_lantern_is_seen_from_the_landing`, which walks the sight line from the landing's eye to the top of the near glazing and finds no parapet in it. It clears by two centimetres.

### Verified by hand

- Posed at `bridge::top` looking east, the building reads as a building. — run with `--done` and `POSE=-136.85,26.0,15.15,1.5708,-0.05`.
- The lantern is warm and not white at every hour. — the same pose with `AT=0.45`, `AT=0.68`, `AT=0.80` and `AT=0.97`.
- From under it, the opening reads as a way out to the sky. — `FLY=1 POSE=-4.55,9.0,-5.75,1.5708,1.15`.

## What is not settled

**The near view.** Ten metres off, on the roof, the glazing is a flat tan
band and the lamp's only mark outside is a pencil of light at the kerb's
foot. The view this spec is for is the one at a hundred and thirty metres
and that one works, so this is recorded rather than owed.

**Whether anything stands above the parapet.** The skyline from the landing
is a flat top with one bump on it. This spec meant to break that with a mast
or a vane, and possibly a light on it, and left the decision until the
lantern was lit, on the argument that the lantern alone might be enough. The
lantern is lit and nothing has been added, so the question is open. Jake's,
because it is what the building looks like.

## Out of scope

**The ground storey's roof.** Fifteen metres below the thing you see and a
dark line at this distance. It wants doing when there is a reason to be
looking at it, and this is not one.

**Lighting the building's walls from outside.** A separate decision and a
pass over the whole exterior. Jake has it.

**The framing.** The deck and its kerbs take two thirds of the frame from the
landing. That is where you stand rather than what you look at, and moving it
is spec 0055's landing rather than this.

**The setting.** The field is endless and the building has nothing behind it,
so even a good building is a good building in a void.
