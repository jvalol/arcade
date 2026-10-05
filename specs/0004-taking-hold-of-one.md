# 0004 Taking hold of one

**Status:** implemented
**Date:** 2026-10-04

## Goal

Turn a shape on the far wall with the mouse, so you can see what it is.

## Why

Spec 0002 put five of the engine's own objects at the end of the room, turning
slowly on their own. They are the graphics canon: the Utah teapot has been the
field's standard test model since 1975, a Klein bottle has no inside, a Menger
sponge has infinite surface area and no volume, and a Hilbert curve is one line
reaching every point in a cube without crossing itself.

That is the joke those five are, and from one angle at one speed you cannot
check it. A Klein bottle from the wrong side is a lump. The thing worth seeing
is the handle passing through the wall rather than around it, and the only way
to see it is to turn the thing.

So the room gains a verb it did not have, and the shapes become the one place
it is used.

## Behavior

**Click a shape and you are holding it.** The room already has one rule for the
button: it acts on whatever the sight is on, and what that thing is decides what
it means. A cabinet plays. A shape is taken hold of. Enter does the same, as it
already does for a cabinet.

**While you hold one, the mouse turns it instead of the view.** The view is
where you were pointing when you took hold, which is at the thing in your hands,
so nothing is lost by stopping it. You can still walk.

**It turns about every axis, not only the upright.** Across the screen turns it
about the upright and up or down tips it, about whichever way is right on the
screen rather than a fixed axis, so it behaves the same from either side of the
aisle.

One axis was the first build and it does not answer the spec's own reason for
existing. A Klein bottle turned only about its own axis never shows the handle
going in through the wall, which is the one thing there is to see.

**The shapes come down to pay for it.** A shape turning about one axis sweeps a
circle as wide as its box's horizontal diagonal. Turning about every axis it
sweeps a ball as wide as the space diagonal, which is root three rather than
root two, so the size spec 0002 settled on was 0.509 wide sweeping 0.88 across a
gap of 0.72. Tipped on its side the Sierpinski tetrahedron reached into the
Menger sponge and down through its own plinth.

The row cannot widen to pay for it. Five steps of 0.88 is 4.03 across and the
clear span between the end cabinets is 3.3. Measured rather than argued: built
it that way, stood in the doorway and looked, and the teapot and the hilbert
tube were behind the cabinets. So the shapes are 0.416 rather than 0.509, which
is the largest that a step of 0.72 will hold, and the row comes to 3.296.

That is smaller than they were, and they were made bigger one change ago for a
reason that still holds. It buys free rotation, which is worth more: a Klein
bottle you can turn over is worth more than a slightly larger one you cannot.

**And it rests on its plinth whichever way it is turned.** A turned shape reaches
lower than an upright one by however much its own corners hang down, which is
its half extents projected onto the upright, so where its middle goes follows
which way it is facing.

This was written down as a trade first: a tipped shape dipping 0.152 into its
plinth, against a plinth clearing the whole sweep and every upright shape
floating that far over it. Both of those are wrong and the choice between them
was false. A thing rests on the thing it is standing on, whichever way up it is,
and the arithmetic for where that puts it is three multiplications.

**A tilt survives letting go.** The row's slow turn is about the upright, and it
is applied over wherever a hand left a shape rather than under it, so one you
have tipped goes on turning tipped instead of righting itself.

**A held one stops its own spin.** It would otherwise drift out from under you
while you were turning it.

**Click again and you let go**, and it carries on spinning from where you left
it rather than snapping back. You turned it to look at something; putting it
back is the room undoing what you just did.

That means the one you touched is out of step with the other four from then on.
Spec 0002 turned all five together, because five things out of step read as five
things rather than as a row. The one in your hands is already not one of the
row, and leaving your angle alone is worth more than the row being tidy.

**Walking away lets go.** Nothing else does: looking away cannot happen, since
while you hold one the mouse is not moving the view.

**The wink goes on the wall, once.** "showing off the engine, if you know you
know" was said under every shape, five times, which is five times too many for a
joke. It is one line, in the corner, while the sight is on any of them, and it
is the same line for all five. Its last clause is what tells you the verb
exists.

It is the two clauses and nothing in front of them. Two drafts opened with a
phrase saying what the five were, "the engine drawing things it did not have
to" and then "what you draw to prove a renderer works", and both were wrong the
same way. "If you know, you know" works by not explaining, so a clause
explaining is the one thing that cannot go in front of it.

The name of the thing is already under the sight when the line is up, so the
line has something to point at without naming it.

**A shape still says its own name under the sight**, and now also what the
button would do, as a cabinet does. Spec 0002 said a shape names itself "and
nothing else", which was right while there was nothing to press. There is now.

## What it asks of blitzkit

Nothing. The engine already reports raw mouse motion, and what a game does with
it is the game's.

## Acceptance criteria

- Taking hold of one and letting go leaves it where it was. — `display::tests::letting_go_does_not_jump`
- A held one does not turn on its own. — `display::tests::a_held_one_holds_still`
- A free one does. — `display::tests::a_free_one_turns`
- It turns about every axis, not only the upright. — `display::tests::it_turns_in_every_direction`
- A tilt survives letting go. — `display::tests::a_tilted_one_goes_on_turning_tilted`
- And a turned one still rests on its plinth. — `display::tests::a_turned_one_still_rests_on_its_plinth`
- A screen reads forwards from the front and backwards from behind. — `room::tests::a_screen_reads_backwards_from_behind`
- Turning moves the one in your hands and no other. — `display::tests::turning_moves_only_the_held_one`
- Letting go carries on from where you left it. — `display::tests::it_carries_on_from_where_you_left_it`
- Nothing can be turned while nothing is held. — `display::tests::turning_nothing_turns_nothing`
- Only one is held at a time. — `display::tests::only_one_is_held`

### Verified by hand

- Turning the Klein bottle over until the handle passes through the wall.
- Walking away from one you are holding.
- Taking hold of one, letting go, and watching it carry on.

## Out of scope

Zooming in on one. Taking a
cabinet's screen off its cabinet. A shape that remembers its angle between runs.
Turning one with the keyboard.
