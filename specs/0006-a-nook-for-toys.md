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
holds a shape. Waist high and deep enough to lean over. How tall and how deep
are the same for every bench, because that is what makes a row of them read as a
row. How long is not: the cradle's frame has to clear the swing of its end balls
and the metronome wants a square foot.

The **nook** is where the benches are. It runs behind the left wall from the far
end of the room, and the way in is the stretch of that wall past the last
cabinet.

That way in is the correction. The first build took the wall away over the whole
far end of it, which left the last cabinet on that side standing in the opening
with nothing behind it: from the aisle you saw through its back into the nook,
and from the nook you saw the back of a cabinet. The nook is a room behind that
wall and you get to it round the end of it.

Not in the aisle. The first attempt before that stood a bench down the middle of
the room and it was in front of the far wall from every angle, which is the one
place nothing should stand. A bench is solid, like a cabinet, so you cannot walk
through one.

Three of them, in the order you meet them coming through the way in: the ball
and chain, the Newton's cradle, the metronome.

### The arcade runs physics

This is the first thing in the room with solvers in it. Each toy keeps its own
step and its own solver, so a slow frame does not change how anything swings and
nothing one toy does reaches another. One solver holding all three would only
mean the cradle paying for the wall's contacts and the wall paying for the
cradle's forty eight passes.

### The ball and chain

The toy the nook was built for. The example's own one hangs from eight metres up
and this room is three and a bit tall, so what stands on the bench is the same
build at a thirteenth of the size: a gantry, a chain of ten beads, a ball forty
times the weight of a brick, and a wall of brick that falls over. The full size
one stays where it is, at `cargo run --release --example chain`.

It stands in a tray with a lip, because a knocked brick needs somewhere to end
up and without the lip it slides off the bench and falls through a room this
solver knows nothing about.

Two numbers came out of sweeps rather than out of the example. The pull is 0.7:
above it two presses do the whole job and there is nothing left to work, below it
the pumping goes on long enough to stop being a thing you are doing. A brick is a
fortieth of the ball rather than the example's two hundred and fortieth: lighter
than that and a brick caught between the ball and the wall behind it is squeezed
by two hundred times its own weight, and what comes out is not a brick falling
over, it is a brick leaving the bench at a hundred and fifty a second.

The chain hangs about 5.5% long under the ball, which is the same order as the
full size example's 4.6% and is the thing spec 0041's passes are spent on. It
bottoms out near four: 64 passes give 4.8%, 128 give 4.4%, and a heavier chain
gets no further.

### The Newton's cradle

Five steel balls hung in a line, touching. Lift the one on the end and let it go,
and it stops dead on the others while the one at the far end flies out.

A cradle is the case sequential impulses are worst at. A blow has to cross four
contacts, and each pass carries it one contact along, so four passes go on
delivery before anything is solved. It wants 240 steps a second and 48 passes,
against the engine's default of 32.

Restitution is 0.92 and not 1. At 1 the row never stops, which is a perpetual
motion machine and a bug. At 0.92 it runs down in about half a minute.

The frame is sized from the swing, not from the row. Built to the row it was
exactly as long as the balls hanging still, and the first thing the end one did
was swing clean through an upright: the ropes hold the balls and nothing holds
them off the frame, so the frame has to stand where they never reach.

### The metronome

An arm pivoted near its foot, with a heavy bob just below the pivot and a lighter
weight that slides up the needle above it. Sliding the weight up slows it down,
which is the thing about pendulums nobody knows until they are shown it: raising
the weight brings the whole thing's middle of mass closer to the pivot from
below, and what little is left to right it has to do the work over a bigger
swing. Eight notches, and the dial they come out at is a metronome's: 139 ticks
a minute at the bottom notch down to 39 at the top.

Spec 0041 gave the engine distance links and nothing else, and this says it did
not need anything else. **A hinge is five rods.** Pin one point on the arm three
ways and it can still turn any way it likes about that point, which is a ball
joint and lets the arm wander out of its plane and never come back. Hold a second
point on the same axis two more ways and the only turn left is the one about the
axis through them. Five rods for the five freedoms a hinge takes away, which is
exactly determined rather than piled on.

The anchors those rods run to are nowhere near the pivot, and they do not need to
be: a rod is a sphere its end rides, and where the middle of that sphere sits
makes no difference to the holding so long as the three of them pull along three
directions that between them cover everything. What it does decide is whether an
anchor stands in the arm's way, and an anchor in the arm's way is a contact
rather than a hinge. There is nowhere close in to put them. The bob sweeps the
space below the pivot, the weight occupies everything from a tenth of a unit
above it to four tenths, and the swing is the rest.

It keeps going, because a wound one does. At each pass through the upright every
speed in it is scaled by one number, to put the swing back where it was set. A
distance rod is a rule about speeds along a line and nothing else, so scaling
every speed by one number leaves every rule exactly as satisfied as it already
was: there is no jolt to take up and no pass spent taking it. Striking the arm
instead puts it somewhere the rods then have to haul it back from.

**The bench reports the rate it counts, not the rate it works out.** The two part
company by 1.7% at the bottom notch and 5.5% at the top. The mechanism is the
reason rather than the solver: at the top the thing is very nearly balanced, with
0.018 of righting left out of the bob's 0.09, so a fraction of a millimetre of
give in the pivot is a few percent of what is left. Thirty two passes and two
hundred and fifty six agree to a part in a thousand, which is what rules the
solver out. The closed form is kept for the explanation, and for sizing the
winding, which is an energy and is exact.

### Working them

Looking at a bench names the toy and says what to press. Enter does the toy's one
thing: it sets the cradle going, starts or stops the metronome, and builds the
wall again.

The arrow keys work whatever toy the sight is on, and walk you about when it is
on nothing. WASD walks whatever you are looking at, so there is never nothing
that does. Only a press is taken, so a release always goes on to clear the
walking: an arrow held down on the way to a bench would otherwise leave you
walking into it with nothing to let go of. A held arrow repeats and a repeat does
nothing, because a pull is a press and not forty a second.

## Acceptance criteria

- The nook is off the aisle, not in it. — `room::tests::the_nook_is_off_the_aisle_and_not_in_it`
- Nothing on a bench stands in front of a shape. — `room::tests::nothing_on_a_bench_stands_in_front_of_a_shape`
- The nook has a way in, wide enough to walk through. — `room::tests::the_nook_has_a_way_in`
- And every cabinet still has a wall behind it. — `room::tests::every_cabinet_has_a_wall_behind_it`
- The benches fit down the nook, inside it and clear of each other. — `room::tests::the_benches_fit_down_the_nook`
- A bench is something you bump into. — `room::tests::a_bench_is_something_you_bump_into`
- The sight lands on a bench. — `room::tests::the_sight_lands_on_a_bench`
- And not on one across the room. — `room::tests::a_bench_out_of_reach_is_not_seen`
- The chain is built hanging still. — `wrecker::tests::it_is_built_hanging_still`
- The wall stands until something hits it. — `wrecker::tests::the_wall_stands_until_something_hits_it`
- Hauling it brings the wall down, and takes more than a press. — `wrecker::tests::hauling_it_brings_the_wall_down`
- Nothing ends up off the tray. — `wrecker::tests::nothing_leaves_the_tray`
- The chain carries the ball without stretching further than it did. — `wrecker::tests::the_chain_carries_the_ball_without_stretching`
- The winch hauls the ball up, and never winds a link shorter than what it ties. — `wrecker::tests::the_winch_moves_the_ball`
- The wall can be stood back up. — `wrecker::tests::building_it_again_stands_the_wall_back_up`
- The balls hang touching. — `cradle::tests::the_balls_hang_touching`
- Each hangs from a hook that does not move. — `cradle::tests::every_ball_hangs_from_its_own_hook`
- A row left alone stays put. — `cradle::tests::a_row_left_alone_stays_put`
- Lifting one end swings the other. — `cradle::tests::lifting_one_end_swings_the_other`
- The middle is left alone. — `cradle::tests::the_middle_is_left_alone`
- It runs down rather than going on forever. — `cradle::tests::it_runs_down_rather_than_on_forever`
- A swinging ball never reaches an upright. — `cradle::tests::a_swinging_ball_never_reaches_an_upright`
- The frame is wider than the row. — `cradle::tests::the_frame_is_wider_than_the_row`
- The weight sits where the notches are, and stays on the needle. — `metronome::tests::the_weight_sits_where_the_notches_are`
- Every rod is built at its own length. — `metronome::tests::every_rod_is_built_at_its_own_length`
- No anchor stands where the arm swings. — `metronome::tests::the_anchors_are_out_of_the_way`
- It stands still until it is started. — `metronome::tests::it_stands_still_until_it_is_started`
- And swinging, it only swings the one way, which is what the hinge is for. — `metronome::tests::it_only_swings_the_one_way`
- The swing is near what the arithmetic says. — `metronome::tests::the_swing_is_near_what_the_arithmetic_says`
- The weight up is slower, over a dial a metronome would recognise. — `metronome::tests::the_weight_up_slows_it_down`
- The rate it reports is the rate it ticks. — `metronome::tests::the_rate_it_reports_is_the_rate_it_ticks`
- It keeps going, and keeps its swing rather than winding itself up. — `metronome::tests::it_keeps_going_and_keeps_its_swing`
- Pressing it again stops it dead and stands it up. — `metronome::tests::pressing_it_again_stops_it`
- The weight can be moved while it is going. — `metronome::tests::sliding_the_weight_while_it_goes_changes_the_rate`
- The rods hold. — `metronome::tests::the_rods_hold`

### Verified by hand

- Coming through the way in, the three toys are a row you walk along.
- Pressing enter sends the far cradle ball out and leaves the middle three still.
- The metronome's lit notch is where the weight is, read from across the nook.
- Four pulls on the ball and most of the wall is down.

## Out of scope

A toy that keeps its state between visits. All three start the way they were
built every time the arcade does.

Sound. A metronome ticks and a wall falling makes a noise, and the arcade is
silent throughout. The engine has sound and the arcade does not depend on rodio,
so this is a crate boundary rather than a missing feature.

The full size ball and chain in the room. It hangs from eight metres up and the
room is 3.2 tall. What is on the bench is a model of it and the example is where
the real one lives.
