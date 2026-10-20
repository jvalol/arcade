# 0028 The fittings stay pushed

**Status:** not built, and not going to be. The measurement is the point.
**Date:** 2026-10-20

## What this was going to be

Spec 0026 handed the engine the rooms' shells and put the rest aside in
one paragraph:

> **The fittings.** Benches, plinths, cabinets, shelves, barrels, trees and
> the rest stay pushed. They are the next thing to keep if the shells pay,
> and they are where the drawing is conditional, which is where this gets
> hard.

Engine spec 0064 then made keeping one call with a name and a version,
which is as easy to write as pushing, so the hard part had got smaller.
This was going to be that spec.

## What the measurement says

The whole of `Arcade::draw` is **0.74 to 0.82 ms**, and it pushes 7,421
instances. The shells it already hands over are 351 more.

The frame is 8.33 ms, pinned to the display, with room to spare.

So the prize is at most three quarters of a millisecond out of a frame
that is already waiting for the screen, and at most is the right word:

- Part of that time is the building working out its own geometry, not
  pushing it. `standing::runs`, `back::clutter` and the rest build their
  boxes each frame, and keeping would only stop the ones whose runs never
  change.
- Everything that animates stays pushed whatever happens. The water, the
  toys, the planets, the metronome, the gyro.

## Why it is not worth it

**The complexity is where spec 0026 said it was.** A cabinet lights under
the sight. A door opens. A leaf swings. Each of those is a fitting whose
appearance depends on something, so each needs that something folded into
a version, and the version has to be right or the building draws a frame
from the past.

**And a version that changes every frame is worse than pushing.** Engine
spec 0068 measured exactly this and wrote it down: a run handed over and
then changed costs the handing over as well as the drawing, and the engine
cannot tell which kind it has until it has already paid.

Three quarters of a millisecond, from a frame with four to spare, against
that. It is the same shape as four things the engine measured and threw
away on 2026-10-19: a small prize, a real cost, and a hypothesis that only
looked good before anybody took a number.

## The one argument that is not about speed

This building never changes, and it is recomputed, allocated and pushed
sixty times a second anyway. Keeping it would stop that, and would delete
the last of the hand-rolled bookkeeping in this repo, after
`standing::worth_keeping` went in spec 0064's wake.

That is a tidiness argument. It is worth about 0.75 ms, and tidiness that
costs a version field on every conditional fitting is not tidiness.

## What would change this

- **The arcade stops holding its display rate.** Then 0.75 ms is a real
  share of what is missing rather than a share of what is spare.
- **A room gets much bigger**, or a game is added whose cabinet is dense
  enough to push the count up by an order.
- **A per-pass measurement finds the geometry rather than the pushing.**
  Engine spec 0067 can time a pass but not a function; if the boxes being
  rebuilt every frame turn out to be most of the 0.75, that is a different
  spec and a simpler one, because caching a `Vec` of boxes needs no
  versions at all.

## What was measured, and how

`Arcade::draw` timed end to end behind `WATCH_DRAW`, with
`Scene::pushed_count` reporting what it had pushed by the end of it. Four
frames at a time, from the doorway, over nine hundred frames.

The first attempt timed to `keep_the_building`, which is in the middle of
`draw` rather than at the end, and reported 0.57 ms and 908 instances. The
count at the next `reset` said 7,421, and the two not agreeing is what
showed the instrument was in the wrong place.
