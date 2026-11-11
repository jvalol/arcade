# 0062 The hotel comes down to its desk

**Status:** superseded on 2026-11-10 by spec 0063, within the hour, which took the rest of this wing out as well and parked all of it. The tests cited below went out of the suite with the lobby they held up. Kept in the numbering because it happened and because its reasons are the ones 0063 builds on.
**Date:** 2026-11-10

## Goal

Take the endless corridor and the long hall out of the building. Keep the
lobby, the desk and the register behind it, which is Cantor's diagonal and is
the only part of that wing where you do something and something happens.

## Why

**Jake, 2026-11-10, having played it: the hotel is arguably the least
interesting room or feature, and worth dropping entirely.** Said of the
hotel and the imaginarium together, the complaint being that both are
confusing to anyone who is not him. The imaginarium stays and gets made
legible. This one goes.

He is right, and the repo has been telling him so for three weeks in its own
words. Four rounds on one complaint:

- Spec 0029 shipped a corridor whose doors open in a wave, and records him
  walking it: opening and closing doors does not demonstrate the point.
- A key that checked a guest into a room and out again went in and came out
  within the hour. It answered "there is no way to add a guest" with a light
  switch.
- `aleph::SIGN_SAYS` exists because he stood at the desk and said the room
  does not make sense to a user. Two lines of signage were the fix.
- Spec 0030 answered 0029 with the arrival and the refilling, which is a
  better wave, and spec 0029 took a fourth amendment today for occupancy
  read as a predicate rather than a list.

Each of those made the thing it fixed correct. None of them changed what the
room is.

**What the room is: you walk to a desk, press a key, and watch.** Every room
in this building that works is something you do. You zoom the Mandelbrot
with your own hands, you fall through a hatch and come up standing on a wall,
you type an answer to a watchman, you walk at the horizon and never reach it.
The hotel is the only one that is an animation with a trigger on it.

**And the astonishing part is the one thing a corridor cannot show.** The
premise is that the hotel is full, which is a claim about an infinity. What
is on screen is forty doors. Forty doors that all open and shut is forty
doors, however correct the arithmetic behind them, and no amount of drawing
more of them helps, because the number that matters is not forty and never
will be.

## What goes

- **The corridor.** Two thousand metres of shell, the forty doors drawn
  round wherever you stand, the plaques, the strips of light under the
  doors, the sconces along it, its lid and its floor.
- **The four arrivals and both waves.** `Arrival`, `Who`, the occupancy
  predicate, `tick`, the travelling front, the refilling, and the desk's
  five lines about them.
- **The people crossing**, which is `people::crossing` and its tests. The
  figures themselves stay: every other room with a person in it uses them.
- **The long hall**, spec 0035's second corridor, where door n stands at the
  nth Fibonacci metre.

## What stays, and why it is enough

The lobby, the desk, the board of keys, the bell, the sign, and the register.

**The premise does not need the corridor.** It needs a claim and a thing
that would have to honour it. The sign is the claim: every room taken, and
there are always more rooms. The board behind the desk is thirty-two key
holes with nothing in them, which says every room is taken in one thing you
can see all of at once. Two kilometres of corridor could not do that, which
is the whole of why it was two kilometres.

**And Cantor never needed Hilbert.** The register is an endless list of
endless entries, which is what a hotel with a room for every number would
have to keep. You walk along it, each step takes the character on the
diagonal and changes it, and at the end of the walk you are carrying a guest
who is on no line of the book. You hand them to the desk. The desk houses
them, writes them in, and that is spec 0032's point exactly: the desk says
yes and it does not help.

So the two trail stations in this wing both keep working.
`Asked::WalkedTheDiagonal` is the walk and `Asked::HousedTheCarried` is the
handover, and the trail still has eight.

## Where the register's door goes

It was room nought's place: `at(reaches, 0)`, three quarters of a metre
inside the corridor, on the even side, the first door you pass. Spec 0031
chose that for the joke, which is that the room proving not everything can
be numbered stands behind the one door with no number on it.

With no numbered doors left the joke has nothing to work against, so the
door moves to the lobby's west wall, where the corridor's mouth was. One
door in a wall instead of a door a stride down a corridor. The sign moves
with it and hangs over it, sized to that wall rather than to the corridor's
mouth, which is what `SIGN_FITS` was counted off.

That reads better than where it was. The sign claims the rooms never run
out, and the door under it is the way to the book that says otherwise.

## What this costs

**The Fibonacci comparison, which was the best thing in the wing.** Spec
0035's hall is the same two kilometres holding sixteen doors instead of two
and a half thousand. Room 17 is twenty-five metres down one corridor and two
thousand five hundred and eighty-four down the other, which is past the end
of the floor: a room that exists, is numbered, and cannot be reached. That
is a sharper statement of the premise than any wave of doors, and an hour
before this spec was written it was the argument for keeping the wing.

It goes because it is a comparison between two corridors and after this
there are no corridors. Written down here rather than deleted quietly, so
whoever wants it back knows what it was and that it was good.

**Eight specs stop describing the building.** 0029, 0030, 0034 and 0035
describe rooms that will not exist; 0031, 0032 and 0036 keep their rooms and
need the way in amended. Each gets a dated section pointing here rather than
a rewrite, because the reasons in them are the useful part and this spec is
only the last of them.

## What the cut turned up

**The register's row numerals were being supplied by the corridor's doors.**
Row n of the book is room n's entry and wears that room's numeral rather
than keeping a second account of what a number looks like, which is spec
0031 doing the right thing. What nothing said is where those numerals came
from: the forty door plaques were made for whichever rooms were drawn round
you, rows 1 to 14 fell inside that window from the moment the building
opened, and the book borrowed them without either room knowing it.

Take the doors away and the left hand column of the book goes blank. No test
of either room noticed, because every one of them is arithmetic and this is
a texture. `the_register_rules_out_what_it_has_answered` caught it by being
a picture, which is the argument for keeping pictures in the suite at all.

The fix is to want those fourteen numerals always. They never change and
they do not depend on where you stand, so asking for them once is cheaper
than asking where you are.

## Acceptance criteria

- The lobby is closed but for the gallery's doorway and the register's door, swept along each wall rather than checked at its corners. — `aleph::tests::one_way_out_of_the_lobby`
- You can walk in from the gallery, cross the lobby, go through that door and reach the head of the page. — `aleph::tests::walked_from_the_gallery_into_the_register`
- The door is wide enough to walk through with its leaf standing open. — `parked::aleph::tests::the_one_door_you_walk_through_is_wide_enough`
- And it is solid, so it is not a door you walk through. — `register::tests::the_book_door_is_solid`
- The sign hangs clear of the door's head below it and the ceiling above it. — `parked::aleph::tests::the_sign_hangs_in_the_band`
- Every lamp in the room sits in a fitting, now that the corridor's set is gone. — `parked::aleph::tests::every_lamp_is_in_a_fitting`
- The lobby's ceiling still reaches its walls and still crosses no doorway. — `parked::aleph::tests::the_lobbys_ceiling_reaches_its_walls`, `parked::aleph::tests::no_beam_crosses_the_way_in`
- The register and the lobby do not both build the wall between them. — `parked::register::tests::the_wall_between_is_built_once`
- And the strip over that wall, where the register stands taller, is closed. — `parked::register::tests::the_strip_over_the_borrowed_wall_is_closed`
- The diagonal still starts at the far end of the page rather than at the door. — `parked::register::tests::the_walk_starts_at_the_head_of_the_page`
- Pressing at the desk empty-handed does nothing, and handing the carried guest over houses them and writes them in. — `tests::the_desk_houses_the_diagonal`
- Each of the desk's three remaining lines shows, and the last one clears itself. — `tests::every_line_the_desk_has_is_reachable`
- The trail still has eight stations and this wing still answers two of them. — `trail::tests::every_one_of_them_is_in_every`

### Verified by hand

- The lobby reads as a room rather than as the stump of a corridor. — walk
  in from the gallery and look west.

## Out of scope

**The imaginarium.** It stays. It holds a trail station, it carries the
lantern that answers the watchman's riddle, and it is the only thing you see
of this building from the top of the bridge. What is wrong with it is that
nothing tells you a hatch is a way you can take, which is a sign and a first
hatch you cannot miss rather than a demolition.

**Rehousing the Fibonacci hall.** Named above as the loss. Putting it
somewhere else is a room's worth of work and a spec of its own.

**The desk's own arrivals.** Gone with the waves. A hotel lobby where
somebody is always at the door and the desk always says yes is the thing
this spec is taking out, and keeping one of the four as a button would be
the same room with three quarters of the code.
