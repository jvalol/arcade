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

**The shapes on the far wall name themselves and offer no key.** Spec 0002 says
looking at one says what it is and nothing else, and that does not change.

**The corner keeps the controls**, and gains the one sentence that says what the
room is for. Walking and looking were already there and being able to walk was
never the part that was unclear.

## What it asks of blitzkit

Nothing. `RenderText` already has `centered` and a colour, and `resized` already
reports the window, so the sight is text at a position this crate works out.

## Acceptance criteria

- The sight is in the middle of the window. — `aim::tests::the_sight_is_in_the_middle`
- And stays there when the window changes size. — `aim::tests::the_sight_follows_a_resize`
- What to press is under the sight rather than over it. — `aim::tests::the_prompt_sits_under_the_sight`
- The sight is brighter on something than on nothing. — `aim::tests::the_sight_brightens_on_something`
- A cabinet you are pointing at is brighter than one you are not, body and screen both. — `aim::tests::the_one_you_point_at_is_brighter`
- Far enough brighter to read across the room. — `aim::tests::the_difference_is_worth_seeing`
- A cabinet that has not been built does not light up. — `aim::tests::an_unbuilt_cabinet_stays_dark`
- Only the thing you are pointing at is lit. — `room::tests::the_one_you_are_looking_at_is_the_one`

### Verified by hand

- Standing in the doorway and being able to tell which cabinet the sight is on.
- Walking the aisle and watching the screens come on one at a time.
- Reading the key under the sight and pressing it.

## Out of scope

An outline or a glow around the lit cabinet, which wants a render pass the
engine does not have. A sight that changes shape. Naming a cabinet from behind
it. Sound on pointing at one.
