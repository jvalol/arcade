# 0045 A key that says where you are

**Status:** implemented
**Date:** 2026-10-30

## Goal

M prints where you are standing, which way you are looking and what the sight
is on, so a screenshot of something wrong arrives with the camera that took
it.

## Why it belongs here

Jake finds the faults in this building by playing it. He plays, something
looks wrong, he takes a picture and sends it, and then somebody has to find
the place. That last step is the expensive one. A picture of a grey wall in a
building of grey walls carries no coordinates, and the only way back to the
camera is to guess at it: pose, render, hold it against the picture, guess
again.

On the 30th of October 2026 that cost the best part of an hour on one
screenshot of the way up, over four wrong guesses, and the fault in the end
was not a fault at all. It was the nook's ceiling with no light on it. The
same afternoon a second picture of the same stair arrived and the guessing
started over.

`WALK` and `WHERE` in the walking skill already do this. Both are environment
variables, which means a rebuild and a second run, which is two steps more
than somebody standing in the thing they want to show you is going to take.

**The feet and the eye are both in the line, because they are not the same
number.** The camera stands at `EYE` over the feet and eases toward it over
`EYE_LAGS`, so a pose set from one alone draws the room from whatever height
the game opened at. Spec 0042 paid for that: a replay with the eye left
behind reported a cabinet in the hall as what you were looking at while
standing in the imaginarium.

## Behavior

M, pressed, puts one line on the screen under the help, on the same plate,
for `PLACE_SHOWS` seconds, and writes it to the error stream as well. Held,
it does nothing more: the repeats of a held key ask for nothing, as with spec
0048's P.

**On the screen and not only in the log**, which was the first build and was
the wrong shape. The whole point is that a screenshot carries its own pose,
and a line in a terminal behind the window does not: Jake pressed it, saw
nothing, and said so. The log line stays because a log can be grepped.

The line is six bare numbers: the feet as three, the eye's height, the yaw
and the pitch in degrees. Two places on the lengths and one on the angles,
which is finer than anybody can stand still. The log line carries what
`what_i_see` found as well, as a cross check.

**Bare numbers because the plate is user facing.** A readout that says `at`
and `yaw` is copy, it goes in under a marker, and it holds the gate until
Jake has rewritten it in his own voice. Six numbers in a fixed order are not
copy, and whoever reads them off a screenshot already knows the order.

It runs out on its own rather than being turned off. A readout that stays is
a mode and a mode wants a second key.

**A string and not a print.** `where_you_are` gives the line, the key shows
it and logs it, so a test can read what it says. A key whose whole output is
a side effect is a key nothing can check.

## Acceptance criteria

- The line carries where you are standing, how high your eye is, and which
  way you are looking, and says something different when you have
  moved. — `tests::m_says_where_you_are`
- It goes on the screen on the press and runs out on its
  own. — `tests::m_shows_where_you_are_for_a_while`
- And asking moves nothing. — `tests::m_moves_nothing`
- The repeats of a held key ask for nothing. — `tests::the_repeats_of_a_held_p_ask_for_nothing`

### Verified by hand

- Run the arcade, stand somewhere, press M, and read the line under the
  help.

## Out of scope

**Putting the camera back.** This says where you were. Flying there is spec
0041's and posing a test camera there is what the line is for.

**A readout you leave up.** This one runs out. Something that stays on the
screen while you walk is spec 0013's business and has its own key.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
