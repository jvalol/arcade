# 0067 People who are going somewhere

**Status:** implemented
**Date:** 2026-11-13

## Goal

The figures at the top of the bridge live there rather than stand there.

Jake, 2026-11-13: the people should move around meaningfully. walking from the
house to the outside firepit, the wagons to the horses, getting food. it should
feel like a true and believable village.

## What was there

A full walk cycle, in `people.rs`, used by exactly one figure: you, in the
mirrors. Spec 0046 built the gait and spec 0061 is the only thing that ever ran
it. Everybody else in this building has stood still since they were drawn.

So the work is not the walking. It is giving somebody a reason to be somewhere
else, and then a way to say where they are at a given second.

## A round

**Arithmetic and not a simulation**, like the strikes, the koi and the moon.
`people::doing(round, since)` is handed the round and the clock and says where
that body is. Nothing is stepped forward and nothing is remembered, which is
what lets any frame be drawn at any `since` at all: every picture in this repo
does that, and a village that had to be run up to would have no pictures.

A `Stop` is a place, a facing to hold while there, and how long to stay. A
round is a list of them and it loops.

- **A stay of nought is a corner.** It is walked straight through, which is how
  a round turns a gable without the turn being a pause.
- **`walked` runs on across the whole round**, so the gait never jumps
  mid-stride, and starts again at nought when the round comes round. That is
  only invisible because a round begins standing; one that opened with a walk
  would snap there once a loop.
- **The turn is eased.** `people::TURNS_IN` is 0.7 seconds at each end of a
  stay, short way round. Without it a figure walks up to the fire facing the
  way it came and is facing the fire on the next frame, which is the one thing
  no amount of leg work survives.
- **`PACE` is 1.1 metres a second**, which is an errand rather than a journey
  and slower than either of your own speeds.

## Where they go

Three rounds over the steading's own geography, which is a hall 24 by 9 with
its door on the gable you arrive at, three outbuildings off the lane, and now
a fire and a woodpile.

- One who keeps the door: out of the hall, up the lane to the north store and
  back. The longest stay is at the door looking out at the climb, which is what
  spec 0055's figure was doing standing still.
- One who keeps the fire: from the woodpile against the south store's gable to
  the ring out in the lane. That is the errand Jake named, and both ends of it
  are things you can see.
- One round the back, past the gable to the third store. It is the only round
  that leaves the lane, and what it is for is that somebody is always arriving
  from somewhere you were not looking.

They are started 4.3 seconds apart, because three rounds of about a length are
otherwise three people moving together.

## The two things they walk to

An errand with nothing at either end is a figure on a track. So:

- **A fire in the lane**, eleven stones round a bed of coals with three flames
  in it. Three because one is a cone: the hall's trench learnt that at sixteen
  metres and a yard fire is the same lesson at two. It throws a light, which is
  what puts somebody walking past it in front of something.
- **A woodpile** against the south store's gable, thirty split logs in five
  courses with every other course laid back a little, which is how a stack of
  split wood sits and what stops it reading as brick.

Both are solid. A fire you can walk through is not a fire, and the ring takes
the trench's own kerb height for the reason the trench has one: below that
`walk` rides over it and you stand in the coals.

## Acceptance criteria

- Nobody on a round walks through a wall, or the fire, or the woodpile. — `valhalla::tests::nobody_walks_through_a_wall`
- Nobody stands in the lane you arrive along, at any stop on any round. — `valhalla::tests::nothing_stands_in_the_way_in`
- A round loops, and where it puts somebody is the same at `t` and at `t` plus a whole round. — `people::tests::a_round_comes_round`
- Somebody standing has no gait and somebody walking has one. — `people::tests::the_legs_run_only_while_walking`
- A turn is taken the short way round. — `people::tests::a_turn_goes_the_short_way`
- Nobody teleports: where somebody is moves no faster than they walk. — `people::tests::nobody_moves_faster_than_they_walk`
- The steading with its people out on it matches its reference. — `tests::the_steading_is_lived_in`

### Verified by hand

- It reads as a place somebody lives rather than as figures on tracks. Walk up
  the bridge and stand in the lane.

## Out of scope

- **Avoiding each other.** Two rounds that crossed at the same second would
  walk through one another. They do not, because the stops are where they are,
  and nothing here would stop it if they did.
- **Doing anything at a stop.** They stand and face the thing. There is no
  carrying, no bending, no sitting: the body has no hands and the arms swing
  off the gait.
- **Going indoors.** Every round is outside. The hall's door is passable and a
  round through it is the obvious next thing, and it wants the figure to not
  be drawn while it is in there or it is a body inside a lit room seen through
  a window.
- **The fire in daylight.** It is written against the dark, like the hall's,
  and at noon the flames clip to white. Nobody is ever in that lane at noon in
  a picture but this one.
