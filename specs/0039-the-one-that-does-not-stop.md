# 0039 The one that does not stop

**Status:** implemented
**Date:** 2026-10-24
**Built:** 2026-10-24, with the way across the plane added on the 25th.

## Goal

A Mandelbrot set in the gallery, zoomed with the arrows. Jake's, 2026-10-24.

## Why it goes in the gallery

The gallery already holds this building's mathematical objects, and three
of its six are fractals: sierpinski, menger and hilbert.

**All three are meshes cut off at a fixed depth.** They are pictures of
self-similarity rather than the thing itself, because a mesh has to end
somewhere. The Mandelbrot is the one that does not, and standing it beside
three that do is the whole reason it belongs here rather than anywhere
else.

It is also what this building keeps doing. The hotel is endless rooms, the
register is a list that cannot be finished, the long hall is the same rooms
further apart. Every room here is one infinity made walkable, and this is
the one that is infinite inwards.

## Where exactly

**In the middle of the back wall, in a moulding, with the row opened
round it.** Three of the six stand either side and the wall between them
is clear, so there is nothing between you and it from anywhere in the
room. The way in from the hall is 3.3 wide and the picture is 2.6, dead
ahead of it, which means you meet it from down the aisle before you are in
the room at all.

The reading the spec was built on survives the move and reads better for
it: three of the six are fractals cut off at a fixed depth, and the one
that does not stop is among them rather than behind them.

~~On a side wall.~~ ~~At the free end of the far wall.~~ ~~A 1.3 square
over the row.~~ ~~The whole back wall.~~ Four were built, in that order.
The side walls each have a door in them, per specs 0024 and 0029. The free
end is a 0.75 gap behind the end plinth, and 0.7 of panel in it is
something the size of a book that you squeeze past a plinth to reach: Jake
walked in and said the placing and the size are not good or accessible.
Over the row was better and still a picture on a wall rather than the
wall.

**And the whole wall was two faults in one coat.** The six stand 0.75 off
that wall, so from anywhere far enough back to see all of the picture,
three of them were across the middle of it. That is not a thing a camera
can be moved to fix, which is why the test for it is measured in plan.

The second fault is the same fault: 6.36 metres of wall is 160 texels a
metre however many texels there are, because the picture is worked out
while you watch and the grid is what a frame can afford. Jake called that
blocky, and when the shading was smoothed he called the same thing blurry.

One answer to both: a smaller picture, 2.6 by 2.0, with clear wall round
it. The row opens by the picture, its moulding and a clear stride each
side, and the room widens to hold what the row came to. That is spec
0022's own rule working, which is what it is there for.

**The texel grid is a density and not a count.** How fine the picture is
was a number across with the other way worked out from the shape of the
wall, so a picture that changed size changed how fine it was and nothing
said so. Texels a metre is the thing that was actually wrong, and said
that way the two sides cannot come out stretched, which was the other
fault in the same line: picked by hand they were 256 by 112 against an
opening of 2.446, a seven per cent flattening of every circle in the set.

**What a frame can afford is what sets the density of the one you move.**
Every texel of it is a point worked out inside the frame that draws it. At
280 a metre the picture is 408 thousand points, which the threads clear in
7.9 milliseconds at the deepest the arithmetic goes, against a frame of
eight; at 300 it is 8.6 and the frame is gone.

**And that budget is the wrong budget for a picture nobody is moving.**
280 a metre is sharp from where the whole picture is in frame and blocky
with your nose on it, which is exactly what the thing invites you to do:
Jake walked up to it and the texels were the size of the detail he had
gone in for. There is no density that answers both while every texel is
worked out inside the frame that draws it.

So there are two pictures. The one you move is 280 a metre and is written
every frame, as before. The one that arrives when you stop is 787, which
is 2048 across: more texels than a window has pixels for the whole
picture, so from anywhere you can stand it is the window you are looking
at and not the texels. It is worked out a slice a frame while nothing is
moving, into a second texture, and drawn in place of the first the moment
it is whole.

A slice is four milliseconds, which is its texels times its rounds against
a measured budget, so it holds a frame wherever you are: eight rows at the
bottom and a hundred and ten at the top. The picture is whole in fifteen
frames at the top and a hundred and ninety seven at the bottom, which at
120 frames a second is a tenth of a second and a second and a half.

**An arrow throws it away.** A settling is of one view, and slices of two
views in one picture is a window onto two places at once. What is drawn in
the meantime is the coarse picture, which costs nothing to fall back to
because it is written every frame anyway.

**What gives way while you move is how many points there are, not how
long each one is allowed.** The rounds are one curve now, whether an arrow
is down or not. They were two, with moving capped at 90 to hold the frame,
and 90 is below what a point just outside the set needs past about twenty
doublings in: every point was still in hand at the last round, so every
point was painted inside, so the picture went black the moment you touched
an arrow and came back when you let go. Jake zoomed and the wall went out.

So the grid coarsens instead. One worked-out point covers two texels at
the top, three ten doublings in, four at twenty and thirty three and five
at the floor, which holds a frame of moving between 1.1 and 7.0
milliseconds the whole way down against a frame of 8. The picture you pan
is 364 across at the top and 146 at the bottom; the one that arrives when
you let go is 2048 wherever you are. Blocky while a key is down, sharp a
moment later, and never black.

**The budget for that is measured along the boundary and not at the top.**
Points times rounds is a ceiling rather than a cost, and at the top nearly
everything escapes in two rounds, so the picture comes out at a fifth of
what the ceiling says. Anybody who has zoomed is standing on the boundary,
where almost every point runs its full count and the ceiling is the cost.
Read off the top it came to 1.5e-10 seconds a point a round and left a
frame of moving at twelve milliseconds thirty doublings in; read off the
floor it is 2.4e-10. The cheap end of a thing is the wrong end to measure
it from.

## What a count is painted

**Where it sits among the picture's own counts, and not a fraction of the
cap.** The cap climbs with the depth and the counts climb with it, but
they bunch as they go: thirty doublings in a point gets 1080 rounds and
the picture's counts run from 1021 to 1073, which is seven thousandths of
a range of 1080. Every colour the picture had landed in that seven
thousandths. Jake zoomed to 84 billion and said it barely looks like
anything, and he was reading the arithmetic off the screen.

So each count is painted by how many of the picture's own points came in
at or below it, over four thousand steps of a histogram built from the
picture itself. Every colour is used at every depth, by construction,
which is the only way a thing worked out while you watch can promise it.
Counted as how many different colours a sample of the picture holds: 31
thirty doublings in before, 253 after, and never under 250 anywhere on the
way down.

The colour runs one sweep over that range rather than three. That is not
a change: the fraction of the cap spanned about a third of its range on
the view you walk in on, and three sweeps over a third is one sweep. It
spans all of it now, so three would have banded the opening view where it
had a single gradient.

A settling is painted when it is whole rather than slice by slice, because
what a count is painted depends on where the picture's other counts lie
and there is no telling that until they are all in.

A browser has one thread and gets a quarter of the density of each.

## What you do

**You hold an arrow and it keeps going.** The gallery already teaches that
verb on five shapes and spends it on the sixth, so this needs no new one
and no explaining.

The arrows move you about the plane and shift with up or down changes the
scale. Enter puts you back at the whole set, since climbing out by hand is
a long hold with nothing learned on the way.

~~Up and down go in and out. Left and right move along the boundary.~~
Struck 2026-10-25. It shipped with no way across the plane at all, because
the first build had no live picture: it pre-computed a ladder of fixed
views, each the last scaled about the same centre, and between two of them
drew the one it had bigger behind a frame wide enough to hide the
overflow. That is a good trick and it only existed because the engine
could add a texture and not write one again. Spec 0074 of the engine fixed
that, and the picture is worked out as you move.

A pan is a share of what is in the panel rather than a distance, so moving
feels the same however far in you are.

## What this must not become

**An explanation.** Nobody says Mandelbrot and nothing says what you are
looking at.

**A screensaver.** A thing that zooms on its own, prettily, asking nothing.
If it moves without you it is a picture that happens to be animated, and
the arcade already decided what a picture is worth: spec 0027 gives every
room one and none of them is a room.

## What it is waiting on

**How deep it goes before the arithmetic gives out, and what happens then.**
Single precision carries about seven digits, which is twenty-odd doublings
and under a minute of holding a key. Double carries fifteen and would take
a while to reach. Either way there is a floor, and the room either meets it
in silence or says something.

Worth deciding by building one rather than by arguing: the interesting
version may be that the machine runs out before the mathematics does, which
is a true thing about computers and a close cousin of what the register's
desk already does.

## What makes it look like a photograph and not a poster

**The colour comes off a fraction of a round, not off the round.** The
number a point escapes on is a whole number, so a picture painted straight
from it is painted in whole steps: flat rings with a staircase along every
edge. That is what the first build was, and it looked low however many
texels the wall had, because the texels were not what was coarse. Jake
called it blocky, then called the smoothed-over version of the same thing
blurry, which is the same complaint twice.

The fraction is recoverable: a point that is well clear of the edge
overshoots it by an amount that says where in its last round it crossed.
Well clear is the condition that matters, so the escape radius is 256 and
not 2. It costs a round or two a point and nothing else.

## Acceptance criteria

- Moving feels the same however far in you
  are. — `mandel::tests::moving_feels_the_same_however_far_in`
- It goes as deep as double precision does, and stops
  there. — `mandel::tests::it_goes_as_deep_as_double_precision_does`
- The picture is not stretched. — `mandel::tests::the_picture_is_not_stretched`
- The whole set is in the view you walk in
  on. — `mandel::tests::the_whole_set_is_in_the_view_you_walk_in_on`
- Moving is cheap however deep you
  are. — `mandel::tests::moving_is_cheap_however_deep_you_are`
- And the picture you move is not
  black. — `mandel::tests::the_picture_you_move_is_not_black`
- The picture uses its colours however deep you
  are. — `mandel::tests::the_colours_are_used_at_every_depth`
- The moulding surrounds the
  picture. — `mandel::tests::the_moulding_surrounds_the_picture`
- Splitting the work across threads gives the same picture as doing it in
  one go. — `mandel::tests::the_bands_make_one_picture`
- You cannot walk the view off into empty
  plane. — `mandel::tests::you_cannot_wander_off_the_set`
- A view that has moved is a different
  picture. — `mandel::tests::a_moved_view_is_a_different_picture`
- The outside is a gradient and not a
  staircase. — `mandel::tests::the_outside_is_a_gradient`
- Nothing stands in front of
  it. — `mandel::tests::nothing_stands_in_front_of_the_picture`
- The picture worked out a slice at a time is the same picture. —
  `mandel::tests::the_slices_make_one_picture`
- And it arrives while you are still standing
  there. — `mandel::tests::the_fine_picture_arrives_while_you_wait`
- An arrow on it throws the settled one
  away. — `tests::moving_throws_the_settled_picture_away`


### Verified by hand

- It does not stutter while you move, and the detail arrives when you
  stop. At the bottom a point gets 1624 rounds, which over the whole
  picture is 663 million points-times-rounds against a frame's budget of
  60, and that is why the grid you pan on coarsens by four down there.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
