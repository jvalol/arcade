# 0044 A turn with rounded corners

**Status:** implemented
**Date:** 2026-10-29

## Goal

An arrow held and let go turns you the way a head turns, easing up to speed
and easing off it, instead of snapping to full rate on the frame the key goes
down and to nothing on the frame it comes up.

## Why it belongs here

The arrows have turned at a flat 2.4 radians a second since spec 0001. On a
keyboard, under a hand, nobody notices: a person taps and holds and lets go on
their own clock, and the clock hides the corners.

The clip is where it shows. `refresh-examples` holds each key for an exact
number of seconds, so every turn in the arcade's clip starts and stops on a
frame boundary, and what that looks like is a tripod head being cranked. The
walk underneath it is the same walk a person does. The turns are not.

**It is the corners and not the rate.** The rate was tried first, slower and
faster, and both are the same stiffness at a different speed. What reads as
mechanical is a view that is at full speed in one sixtieth of a second and at
rest in the next.

**And a straight ramp has corners of its own.** The first build walked the
speed itself toward full at a steady rate. That is a flat acceleration, which
means the view goes from still to winding on as hard as it ever will inside
one frame, and does the mirror of it when the key comes up. Jake watched a
clip of it and said it still felt robotic, which is the same word and the
right one: the ramp was softer and the shape was still made of straight lines
and corners.

**And then the shape of the turn was never the whole of it.** With the easing
in, he watched it again, played it himself and sent back the sentence this
spec should have started from: the stop and turn is what makes it feel
robotic. Both clips walked to a corner, halted, pivoted ninety degrees on the
spot and set off again. Nobody does that. A turn you are standing still for
reads as a machine however smoothly it accelerates, and no amount of curve on
a pivot stops it being a pivot.

What a person does is turn while still moving, so the corner comes out as an
arc. The building could already do it: `walking` and `looking` are separate
held keys and the walk takes its direction from where you are facing each
frame, so holding w and an arrow together has always walked a curve. Nothing
could ask for it. `refresh-examples` held one key at a time and spec 0042's
checker read one key at a time, so every staged route in this project was
written in pivots because pivots were the only thing the grammar had.

## Behavior

Yaw and pitch each carry a winding from -1 to 1 rather than being moved
directly. Each frame the winding walks toward what the arrows ask for at
`1 / AIM_LAGS` a second, `eased` makes a smoothstep of it, and the angle moves
by that times `TURNS` or `TILTS`.

Smoothstep is what takes the corners out. It leaves nought with no slope and
reaches one with no slope, so the speed curves into the turn and out of it
instead of hinging into it. Carried through nought by its sign, so reversing a
turn is as smooth as starting one.

`AIM_LAGS` is 0.2 seconds, which is how long an arrow takes to reach full speed
from rest and how long the view takes to stop after the key is up. It sits
beside `EYE_LAGS` and is the same kind of number: a lag chosen so that a thing
nailed to the input does not read as nailed to it.

**A fifth of a second, which is most of a quarter turn.** A quarter turn is
0.655 seconds, so the view spends 0.2 winding on, 0.255 at speed and 0.2
winding off, and what that reads as is one arc. At 0.12 the flat middle was
longer than either end and the turn still read as a crank.

**A hold sweeps exactly what it always swept.** Smoothstep is symmetric about
a half, so the area under the curve winding on and the area missing winding
off are the same area, and a hold of `t` seconds loses `TURNS * AIM_LAGS / 2`
radians getting up to speed and gains the same back after the key is up. Every
route written against the old flat rate still ends up facing where it did, as
long as the tail after the key is allowed to run.

That proviso is real. Spec 0042's checker lets `AIM_LAGS` of no keys pass at
the end of every step before it reads where you are pointing, so a replay is
exact. On the machine `play` in `refresh-examples` leaves about four tenths of
a second between two keys, which is comfortably over it.

It is a soft edge either way. Smoothstep reaches nought with no slope, so the
end of the winding off is where the view is barely moving at all: a gap a hair
under `AIM_LAGS` loses about a thousandth of the turn. What would matter is a
gap of half of it or less. Spec 0042's checker had no such gap and ran one key straight
into the next, so it now lets `AIM_LAGS` of no keys pass at the end of every
step before it reads where you are pointing. Without that it reads every turn
in every route as short by 0.124 radians, which is the ramp it cut off.

That shortfall is a fixed amount and the checker's gate is a share, so what it
costs depends on the hold. A quarter turn comes out eight per cent short and
goes through a gate set at ten. The 0.22 second turn the arcade's own route
opens with comes out a quarter short and is refused outright: a route that
walks exactly where it says would be turned away for a fault in the checker.

The pitch limit is unchanged and clamps the angle, not the rate. An arrow held
into the limit stops moving the view and its rate decays as usual.

**The mouse is untouched.** It moves the view per pixel, which is a hand
already doing its own easing, and spec 0003's sight has to follow the hand
exactly. Lagging it would be lag.

**The panel is untouched too.** While the sight is on spec 0039's Mandelbrot
all four arrows go into the set, the arrows ask nothing of the view, and the
rate decays to nothing like any other release.

## Acceptance criteria

- A held arrow plus its tail sweeps the rate times the hold. — `tests::a_held_arrow_sweeps_what_it_always_swept`
- The first frame of a turn moves the view less than a frame at full speed. — `tests::a_turn_starts_slower_than_it_gets`
- And the start is a curve rather than a hinge, which a straight ramp is not. — `tests::the_turn_has_no_corner_where_it_starts`
- The view keeps turning after the arrow is up, and stops inside `AIM_LAGS`. — `tests::letting_go_does_not_stop_the_turn_dead`
- The mouse still turns you by its whole delta on the frame it arrives. — `tests::the_mouse_is_not_eased`
- A tilt held into the limit stops at the limit. — `tests::an_eased_tilt_still_stops_at_the_limit`
- Spec 0042's checker reads a route's turns after the tail, so a short turn in a route is still read as a turn. — `tests::a_short_turn_in_a_route_is_still_read_as_a_turn`

### Verified by hand

- A turn reads as a head turning rather than a crank. Run the arcade, hold an
  arrow, let go, and watch the last moment of it.

## Walking the corner

A hold in a staged route may name several keys joined by a plus, as
`hold:w+left:0.8`, and they go down together and come up together. The game
needed nothing for this: it is the grammar in `refresh-examples` and the
reader in spec 0042's checker that could only say one.

A quarter turn at a walk sweeps about 1.3 metres across and 2.1 along, which
is what a corner has to be laid out from: the turn starts two metres before
the opening rather than at it.

**It costs the walk some of its ground, and the checker had to be told.**
Spec 0042 calls a walk stopped under half of what `SPEED` times the hold would
cover in a straight line, and an arc keeps its chord rather than its length: a
quarter turn walked through keeps nine tenths and a half turn two thirds. Both
are clear of the half. A climb is the one that comes close, for a different
reason, and the arcade's own route holds the second flight longer than it
needs so the walk out onto the floor at the top earns the distance back.

## Acceptance criteria, walking the corner

- A hold may carry a walk and a turn together. — `tests::a_hold_can_carry_a_walk_and_a_turn_together`
- And held together they come round as far as the pivot and go somewhere. — `tests::a_walked_corner_turns_as_far_as_a_pivot_and_goes_somewhere`

## Out of scope

Easing the walk itself. Starting and stopping on the spot is the same
stiffness in the other axis, and it is a bigger change: the body is a sphere
pushed through the colliders a frame at a time, so a walk that coasts has to
coast into walls. The turn carries no such question.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
