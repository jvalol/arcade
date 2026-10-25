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

**Over the row, down the middle of the far wall.** Not in the row: that is
full and says so, since `WALL`'s own note works out that six shapes clear
the end wall by half a centimetre at each end and the hall cannot get
wider without the sight breaking.

A panel is what this wants anyway. It is flat, and a plinth is for
something you walk round.

~~On a side wall.~~ ~~At the free end of the far wall.~~ Both were built.
The side walls each have a door in them, per specs 0024 and 0029. The free
end is a 0.75 gap behind the end plinth, and 0.7 of panel in it is
something the size of a book that you squeeze past a plinth to reach: Jake
walked in and said the placing and the size are not good or accessible.

Above them it is the first thing in view on walking in, it is the only
flat thing in a room of objects, and the three fractals it answers are
underneath it in the same glance.

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

## Acceptance criteria

- Moving feels the same however far in you
  are. — `mandel::tests::moving_feels_the_same_however_far_in`
- It stops where single precision does, and not short of
  it. — `mandel::tests::the_view_stops_where_the_numbers_run_together`
- You cannot walk the view off into empty
  plane. — `mandel::tests::you_cannot_wander_off_the_set`
- A view that has moved is a different
  picture. — `mandel::tests::a_moved_view_is_a_different_picture`
- The panel hangs clear of the shapes and of the ceiling, and you can
  reach it from in front of
  it. — `mandel::tests::the_panel_hangs_clear_and_within_reach`

### Verified by hand

- It does not stutter while you move. The picture is worked out again
  whenever the view changes, which is 0.6 to 6 milliseconds at 128 texels
  against a frame of 8, and only while you are holding a key.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
