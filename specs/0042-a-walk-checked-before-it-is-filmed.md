# 0042 A walk checked before it is filmed

**Status:** implemented
**Date:** 2026-10-26

## Goal

A staged route that does not go where it says should be refused before the
camera runs, rather than published.

## Why it belongs here

Spec 0039's panel takes all four arrows while the sight is on it. That is the
right answer for somebody working the panel and the wrong one for somebody
walking past it, and nothing in the building can tell the two apart.

`refresh-examples` in the project above films the arcade by holding keys at
it, and its route met the panel on the way to the hotel. Every turn after that
went into the panel instead of the camera, so the walk never turned, the last
nine seconds are a window pressed against the end wall, and the hotel the
route was written to show was never reached. It was filmed, looked at and
published on the 26th of October 2026. No frame of it looks broken. What it
looks like is somebody choosing to zoom a fractal.

The route lives in that table and the rule about the arrows lives in here, so
neither repo can check this alone, and a copy of either in the other is the
drift this building keeps writing down.

**A walk that does not go is the same fault wearing other clothes.** The
threshold out of the hotel's lobby stops you dead on one heading and lets you
through on both of the ones either side of it, which is a two degree band and
nobody could have guessed it. The route crosses it. The check said every turn
turned and the walk went four metres of the eleven it asked for, and said
nothing at all, because it was only ever asked about turns.

## Behavior

`--walk=<route>` takes a route in `refresh-examples`' own grammar, replays it
with no window, prints where it went and what it was looking at, and exits
nonzero if a turn in it did not happen or a walk in it went nowhere.

**A `pin:` is a walk meant to end against something**, written like a hold and
exempt from the rule that a walk goes somewhere. It is for squaring up on the
building rather than aiming at a point in it.

That distinction was bought twice. The clip of the four rooms walks twenty
metres down the hall, turns into the nook and points at the bookcase that
opens. Dead reckoning over that distance put it one case along, which is a
camera reading the spines of the wrong bookcase for twenty seconds, and this
check called the route sound both times because the route is sound: the
arithmetic is exact here and the machine's is not. Walking into the wall first
pins the coordinate the error is in, and a short strafe off that lands where
it should. Use the building as the datum wherever a target is tight.

**A hold may name several keys joined by a plus**, as `hold:w+left:0.8`, and
the reader sets each of their slots. Spec 0044 is why: a route that can only
say one key at a time can only pivot, and a corner you stand still to turn is
what reads as a machine.

**Where it went is three numbers, not two.** It printed x and z while every
room in the building stood on one floor. The way up and the imaginarium are
two storeys of it, and a trace without the height cannot tell the nook from
the room above it.

**And the eye goes with the feet.** The catching up of spec 0005's lagged eye
sat near the end of `update`, so a replay never moved it: the sight was worked
out from a camera on the hall's floor however high the body climbed, and this
reported a cabinet in the hall as what you were looking at while standing in
the imaginarium. It is in `aimed_and_moved` now, with everything else a replay
and the game have to agree about. `update` works the sight out straight after
calling that, so the order inside a frame is unchanged.

- The replay is the game's own. `aimed_and_moved` and `what_i_see` came out of
  `update` for this and are called by both, so there is one account of how
  aiming works and the check is of the building people walk.
- A held arrow asks for `TURNS` or `TILTS` times its seconds. Less than nine
  tenths of that and it was swallowed. A tilt is measured against what is left
  before the pitch stops, because an arrow held into the limit moves less than
  it asked for and nothing ate it.
- A held walking key asks for `SPEED` times its seconds, and less than half
  of that means something stopped you. Half is wide on purpose. A walk down a
  corridor keeps all of its ground, one that grazes a jamb or threads a
  doorway loses a few per cent, and the walk this was written for kept 4.3
  metres of the 11.5 it asked for. There is no exemption for the last key:
  walking into a wall is nobody's ending.
- **A swallowed arrow is a fault only when a key comes after it.** The
  arcade's own route ends by holding one into the panel, which is the single
  thing the panel can be filmed doing: zooming is shift and an arrow, and
  there is no way to hold shift from a script. A route may end that way and
  may rest there. It may not carry on.
- A route it cannot read is refused rather than guessed at.
- It does not replay the door easing open or the shove that keeps you clear of
  its leaf. Neither aims you.

## Acceptance criteria

- A turn the panel eats, with the route carrying on past it, is
  refused. — `tests::a_turn_the_panel_eats_is_a_fault_when_the_route_carries_on`
- The same walk into the same panel with nothing after it is let through, and
  so is a rest on
  it. — `tests::a_route_may_end_by_holding_an_arrow_into_the_panel`
- A walk that keeps out of the panel's reach is nobody's
  business. — `tests::a_route_that_stays_out_of_the_panels_reach_is_let_through`
- A hold with no seconds, or of a key that neither aims nor walks, is
  refused. — `tests::a_route_this_cannot_read_is_refused`
- A walk that spends most of itself against a wall is
  refused. — `tests::a_walk_a_wall_stops_is_a_fault`
- And the same walk with room to finish in is let
  through. — `tests::a_walk_with_room_to_finish_is_let_through`
- The eye goes up with the feet, so a replayed climb's sight is taken from
  the height it is at. — `tests::the_eye_sets_off_after_the_feet_in_one_frame_of_aiming`
- And what that sight finds is what you would see standing
  there. — `tests::the_sight_is_taken_from_the_floor_you_are_standing_on`
- A `pin:` may walk into a wall and the same walk as a `hold:` may
  not. — `tests::a_pin_may_walk_into_a_wall_and_a_hold_may_not`

### Verified by hand

- `--walk` on the route as it shipped: the left arrow at 5.25s is swallowed
  and the walk after it is named.
- `--walk` on the route as restaged: twenty one steps, the desk and both
  corridors in the trace, and the one swallowed key is the last.
- `refresh-examples arcade` refuses to film when the check fails.
- The lobby threshold, walked at every heading the frame rate can land on:
  89.4 degrees is held at the wall for six seconds of walking, and 87.1 and
  91.7 both go the whole eleven metres. The check names the first and passes
  the other two.

## Out of scope

**What the clip shows.** This says the camera went where the route said. It
says nothing about whether that was worth filming, which is Jake's and comes
off the screen.

**Every other example.** They turn with the arrows and have no panel, so there
is nothing in them for this to find.

**A walk stopped in its last stride.** It keeps enough ground to pass, and
widening the gate until it does not would start refusing the honest walks that
graze a jamb. This catches a walk that was stopped, not one that arrived
slightly short.

**Wading.** `spa::WADE` is 0.42, so a walk through the pool covers less than
half of what it asks for and this would call it stopped. No route goes there.
The day one does, this asks `pace` where it now assumes `SPEED`.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
