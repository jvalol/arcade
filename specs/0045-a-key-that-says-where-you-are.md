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
and leaves it there. M again takes it off. Either way it writes the line to
the error stream, because a press that turned the readout off is still a
press that was made somewhere and a log is what gets grepped afterwards.
Held, it does nothing more: the repeats of a held key ask for nothing, as
with spec 0048's P.

**On the screen and not only in the log**, which was the first build and was
the wrong shape. The whole point is that a screenshot carries its own pose,
and a line in a terminal behind the window does not: Jake pressed it, saw
nothing, and said so. The log line stays because a log can be grepped.

The line is seven bare numbers: the feet as three, the eye's height, the yaw
and the pitch in degrees, and the clock. Two places on the lengths and one on
the angles, which is finer than anybody can stand still. The log line carries
what `what_i_see` found as well, as a cross check.

**Bare numbers because the plate is user facing.** A readout that says `at`
and `yaw` is copy, it goes in under a marker, and it holds the gate until
Jake has rewritten it in his own voice. Seven numbers in a fixed order are
not copy, and whoever reads them off a screenshot already knows the order.

**It ran out on its own until 2026-11-12.** The argument was that a readout
which stays is a mode, and a mode wants a second key. Jake, having used it:
it shouldn't disappear after a few seconds, it should be more like a toggle.

The second key is the same key, which is what that argument was missing. And
the eight seconds were chosen to be long enough to take a screenshot without
hurrying, which is a guess about how long somebody takes to do something, of
exactly the kind this building gets wrong by sitting on the other side of
the screen from the person doing it.

**A string and not a print.** `where_you_are` gives the line, the key shows
it and logs it, so a test can read what it says. A key whose whole output is
a side effect is a key nothing can check.

## The clock, 2026-11-12

The line carried the place and nothing about when, and a day here runs its
whole round while you stand in it. The sun swings through it, the sky's picture slides by
`since` at 0.011 of itself a second, the lanterns come on and go out, and
spec 0065's moon rises and sets. Two frames from one spot are two different
pictures.

**What that cost, on the day it was written.** Jake sent a screenshot of a
pale shape in the sky over the field, with this line on it. The place was
exact and useless on its own: that pose was drawn at every tenth of the day,
then at fine steps through the late morning, then across ten days of weather
drift, at his window size as well as mine, and the shape never appeared in
any of them. The hunt was searching a dimension he had already measured and
had no way to write down.

So the clock goes on the end. `since` is a plain sum of frame times and
everything the sky does is worked out from it, so these seven numbers are the
frame.

**And the line goes back in.** `POSE` now reads exactly what the plate
prints, in the same order and the same units, with commas or with the gaps
left as they are, so a screenshot is a recipe rather than a clue. That took
two corrections nothing was watching:

- The plate printed degrees and `POSE` read radians. A pasted readout faced
  somewhere else entirely.
- The plate's fourth number is the camera's height and `eye` is the feet's,
  which are `EYE` apart. Fed straight back, the camera stood a metre and a
  half too high, and the number came back out of the readout looking exactly
  like the one that went in.

Both are the fault this building keeps writing down under other names: two
accounts of one thing. There is one account now, `Pose`, and a test that
takes the line the plate prints, puts it back in, and asks for the line
again.

## Acceptance criteria

- The line carries where you are standing, how high your eye is, which way
  you are looking and what the clock says, and says something different when
  you have moved. — `tests::m_says_where_you_are`
- The line the plate prints is the line `POSE` takes, all the way round. — `tests::the_plate_says_what_pose_takes`
- And a line that is not a pose is not read as one. — `tests::a_line_that_is_not_a_pose_is_not_read_as_one`
- It goes on the screen on the press, stays there, and comes off when it is
  asked again. — `tests::m_shows_where_you_are_until_it_is_asked_again`
- And asking moves nothing. — `tests::m_moves_nothing`
- The repeats of a held key ask for nothing. — `tests::the_repeats_of_a_held_p_ask_for_nothing`

### Verified by hand

- Run the arcade, stand somewhere, press M, and read the line under the
  help. Walk, and watch it keep up. Press M again and watch it go.

## Out of scope

**Recording a walk.** This says where you were and when, which is enough to
stand a frame up again. It is not enough to reproduce how you got there, so
anything that depends on the path rather than the place, the pool's surface
or the cradle's swing, still wants a recorder. That is not built, because
every fault chased so far has been "stand here, at this time, look that
way".

**Every other readout.** This one is a place and a clock. What is solid,
what the sight is on and what a frame costs are spec 0013's business and
have their own keys.
