# 0013 Showing what is solid

**Status:** implemented
**Date:** 2026-10-10

## Goal

Four of the five faults fixed in this building in one week were the same
shape: a collider that disagreed with what was drawn. Every one was found by
playing until you happened to stand somewhere that showed it, and the last of
them was four pixels of sky over a doorway. Spec 0046 of the engine draws
collision boxes over the scene. This puts that on a key, in the building where
the faults are.

## Behavior

**O shows what is solid.** Every box of `room.solid()` goes up as a wireframe
over the finished scene, in one cool colour against a building lit warm. Press
it again and they go.

**Off when the building opens**, and not mentioned in the line at the top of
the screen. It is a tool for whoever is building the place, and the line at
the top is for whoever is standing in it.

**It outlines `room.solid()` itself** rather than a list assembled for the
purpose. A second list is two accounts of one thing, and two accounts of one
thing is the fault this is for finding.

**A toggle and not a hold.** What you do with it is walk to the thing you are
suspicious of and look at it from two or three places, and a key held down for
that is a key you are fighting.

The key repeats while it is down, and the repeats are ignored. Taken off every
press, a held O turns the outlines over at the repeat rate and reads as a key
that flickers. The test for this passed before the repeats were ignored,
because it sent five of them and five is odd.

## Acceptance criteria

- It is off when the building opens. — `tests::what_is_solid_starts_hidden`
- O turns it on, and off again. — `tests::o_toggles_what_is_solid`
- Holding O does not flicker it. — `tests::holding_o_shows_it_once`
- Nothing else turns it on. — `tests::no_other_key_shows_what_is_solid`

### Verified by hand

- The outlines land on the walls they trace rather than beside them.
- The way into the garden shows its opening, which is where this would have
  caught spec 0010's missing parapet.

## Out of scope

**Showing what is drawn.** The other half of the comparison is your eyes,
because this building draws from several dozen calls rather than one list.
Where the two disagree you see an outline standing off the thing it should be
tracing, which is what every one of those faults looked like.

**The body you walk around as.** It is a sphere, and spec 0046 draws boxes.

**Anything on the screen saying it is on.** The outlines are the signal that
it is on.
