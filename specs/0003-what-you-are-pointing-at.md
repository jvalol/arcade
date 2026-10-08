# 0003 What you are pointing at

**Status:** implemented
**Date:** 2026-10-04

## Goal

Make it obvious what the middle of the screen is on, and what pressing enter
would do about it.

## Why

Spec 0001 made looking at a thing the same as pointing at it, because the window
holds the cursor and there is nothing else to point with. It did not draw the
pointer. So the room has an aim and no sight, and the only way to find out what
you are on is to read a line of text in the corner.

The highlight it did build is too small to see. A lit cabinet's screen goes from
a tint of 0.78 to 1.0, which is 28 per cent more of a photograph in a room lit
at 0.55, and the cabinet's own body does not change. Jake walked the room and
could not tell the arcade was interactive at all.

## Behavior

**A sight in the middle of the screen, always.** It is where the room is asking
you to point, so it is drawn whether or not you are on anything. Dim on nothing
and bright on something.

**The sight only reaches what you could walk up to.** It carried the length of
the room: from the doorway it lit a cabinet thirteen units off, named it, and
offered to play it, and after spec 0004 it offered to turn a Klein bottle at a
size where you cannot see what you would be turning.

Spec 0001 replaced "the nearest one in front of you, within reach and nearly
enough faced" with pointing and nothing else, because having to face a thing
meant walking the aisle to learn what anything was. Dropping the facing was
right. Dropping the reach along with it is what put the far wall in your hands
from the door.

One reach for everything, so the room has one rule, and it governs the light,
the name and the click together. Beyond it the sight is on nothing, which is
what it already draws when it is on nothing.

**The screens are dark until you point at one.** Dimming the rest is what makes
one stand out, and it is what an arcade looks like anyway. The one you are on
shows its picture at full strength and the others sit well under it, far enough
apart to read across the room rather than from in front of the cabinet.

**The cabinet you are on brightens too**, not only its screen. A screenshot can
be dark all over, and when it is, the screen alone carries nothing.

**What to press is said under the sight**, where you are already looking, rather
than only in the corner. The name of the thing, and the key.

**A cabinet that has not been built says so and does not light up.** It is not
playable, so lighting it would be a promise the room cannot keep. Spec 0001
already refuses to start it.

**The shapes on the far wall answer more quietly.** Brightening is this room's
way of saying press enter, and a shape has nothing to press. Lighting one as a
cabinet lights is the same promise an unbuilt cabinet is kept dark to avoid
making, and it was: 0.51 against a cabinet's 0.58.

Not none, though, which was the first thing tried on paper. Five of them stand
0.72 apart and the name under the sight belongs to exactly one. Enough to say
which, held at under half a cabinet's step so the two never read as the same
offer. Spec 0002 still stands: looking at one says what it is and nothing else.

**The corner keeps the controls and nothing else.** It gained a line saying to
look at a cabinet and press enter, and that is the same instruction the sight
already gives, in the place you are not looking. Said once.

## What it asks of blitzkit

Nothing. `RenderText` already has `centered` and a colour, and `resized` already
reports the window, so the sight is text at a position this crate works out.

**Every line of text brings its own background.** They are drawn
straight over whatever the room is doing, and the room is sometimes a neon
sign four feet from your face: white on cyan is not readable, and which bit of
the building is behind a line of help is not something the help can know.

A plate under them, dark and see-through, so it is a shade over the room
rather than a bar across it. Black rather than grey, because grey at this
alpha is a grey box over a dark room and still grey over a bright one, where
black takes whatever is behind it down by the same amount wherever you are
standing.

A line round it as well as the shade. A shade alone over a busy room still
leaves the words sitting in the middle of whatever is behind them; the line is
what says they are a thing in front rather than a thing in the room. Two quads
and not a frame of four, because a frame of four is four chances to be a pixel
out.

Both blocks, and the first plate written for this only went under the corner.
The pair under the sight was the harder one to read: it lands on whatever the
sight has just lit, which is the brightest thing in the room, and it used to
answer that with a drop shadow of itself. A shade with a line round it does
the job the shadow was doing and does it over a pale table as well as a dark
one, so the shadow is gone.

Measured off the words rather than run to the edge of the window, and one
plate for the block rather than one a line. A line of this font is taller
than the gap between lines, so a plate a line, each padded, lap over one
another, and two see-through quads in the same place are twice as dark as one:
the overlap showed as a band through the middle of the help.

## Acceptance criteria

- The sight is in the middle of the window. — `aim::tests::the_sight_is_in_the_middle`
- And stays there when the window changes size. — `aim::tests::the_sight_follows_a_resize`
- What to press is under the sight rather than over it. — `aim::tests::the_prompt_sits_under_the_sight`
- The sight is brighter on something than on nothing. — `aim::tests::the_sight_brightens_on_something`
- A cabinet you are pointing at is brighter than one you are not, body and screen both. — `aim::tests::the_one_you_point_at_is_brighter`
- Far enough brighter to read across the room. — `aim::tests::the_difference_is_worth_seeing`
- A shape says which one it is without saying press. — `aim::tests::a_shape_answers_more_quietly_than_a_cabinet`
- A cabinet that has not been built does not light up. — `aim::tests::an_unbuilt_cabinet_stays_dark`
- Only the thing you are pointing at is lit. — `room::tests::the_one_you_are_looking_at_is_the_one`
- The sight does not reach the far wall from the doorway. — `room::tests::the_sight_does_not_reach_across_the_room`
- Every line of text sits inside its plate, in the corner and under the sight. — `aim::tests::every_line_of_text_sits_on_its_plate`
- The plate is a shade with a line round it, not a bar. — `aim::tests::the_plate_is_a_shade_and_not_a_bar`
- Nothing to plate is no plate. — `aim::tests::no_words_is_no_plate`

### Verified by hand

- Standing in the doorway and being able to tell which cabinet the sight is on.
- Walking the aisle and watching the screens come on one at a time.
- Reading the key under the sight and pressing it.

## Out of scope

An outline or a glow around the lit cabinet, which wants a render pass the
engine does not have. A sight that changes shape. Naming a cabinet from behind
it. Sound on pointing at one.
