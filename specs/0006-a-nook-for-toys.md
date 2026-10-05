# 0006 A nook for toys

**Status:** implemented
**Date:** 2026-10-05

## Goal

The arcade held two kinds of thing: games in cabinets, and the engine's own
shapes on plinths. The chain example is neither. You work it, so it is not a
shape; there is nothing to win, so it is not a game. This gives the room a third
kind and somewhere to put it.

## Behavior

### Toys, benches, a nook

A **toy** is a demonstration you drive. You work the controls, the engine shows
you what it does, and there is nothing to win.

A **bench** is what a toy stands on, the way a cabinet holds a game and a plinth
holds a shape. Waist high and deep enough to lean over.

The **nook** is where the benches are. It opens off the left wall past the last
cabinet, so you walk the cabinets, reach the shapes, and the toys are in the
corner beside them.

Not in the aisle. The first attempt stood a bench down the middle of the room
and it was in front of the far wall from every angle, which is the one place
nothing should stand. A bench is solid, like a cabinet, so you cannot walk
through one.

### The first toy

A Newton's cradle. Five steel balls hung in a line, touching. Lift the one on
the end and let it go, and it stops dead on the others while the one at the far
end flies out.

Looking at the bench names the toy and says enter sets it going. Enter lifts the
end ball through nine tenths of a radian and lets it fall. It is placed rather
than shoved: a ball given a velocity in a row that is already touching spends it
on the contact in front of it before it has anywhere to swing from.

### The arcade runs physics

This is the first thing in the room with a solver in it. The toy steps at its
own rate rather than the frame's, so a slow frame does not change how it swings.

A cradle is the case sequential impulses are worst at. A blow has to cross four
contacts, and each pass carries it one contact along, so four passes go on
delivery before anything is solved. It wants 240 steps a second and 48 passes,
against the engine's default of 32 passes at whatever rate the game picks.

Restitution is 0.92 and not 1. At 1 the row never stops, which is a perpetual
motion machine and a bug. At 0.92 it runs down in about half a minute.

## Acceptance criteria

- The nook is off the aisle, not in it. — `room::tests::the_nook_is_off_the_aisle_and_not_in_it`
- Nothing on a bench stands in front of a shape. — `room::tests::nothing_on_a_bench_stands_in_front_of_a_shape`
- The nook has a way in. — `room::tests::the_nook_has_a_way_in`
- A bench is something you bump into. — `room::tests::a_bench_is_something_you_bump_into`
- The sight lands on a bench. — `room::tests::the_sight_lands_on_a_bench`
- And not on one across the room. — `room::tests::a_bench_out_of_reach_is_not_seen`
- The balls hang touching. — `cradle::tests::the_balls_hang_touching`
- Each hangs from a hook that does not move. — `cradle::tests::every_ball_hangs_from_its_own_hook`
- A row left alone stays put. — `cradle::tests::a_row_left_alone_stays_put`
- Lifting one end swings the other. — `cradle::tests::lifting_one_end_swings_the_other`
- The middle is left alone. — `cradle::tests::the_middle_is_left_alone`
- It runs down rather than going on forever. — `cradle::tests::it_runs_down_rather_than_on_forever`

### Verified by hand

- Walking in from the aisle, the cradle is the thing you see.
- Pressing enter sends the far ball out and leaves the middle three still.

## Out of scope

More than one toy. The row of benches is built to take a second, and there is
one.

A toy that keeps its state between visits. The cradle starts hanging still every
time the arcade does.

Any toy that needs a shape the engine does not have. A metronome wants a hinge
and spec 0041 gave the engine distance links only.
