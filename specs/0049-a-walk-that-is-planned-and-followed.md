# 0049 A walk that is planned and followed

**Status:** implemented
**Date:** 2026-11-01

## Goal

Say where the clip should go and let the building work out how. A route
becomes a list of places and what to do at each, the arcade plans a path
between them, and it follows that path with its eyes open instead of
firing a fixed number of seconds of each key and hoping.

Jake's, 2026-11-01, after a morning of takes that passed the checker and
failed on the machine.

## Why, with the numbers

The route in `refresh-examples` is open loop. It is a string of
`hold:w:2.4` and `hold:left:0.655`, replayed as real keys into a window,
with nothing anywhere that asks where the body actually got to.

Driven with the real keys and the game logging its position twice a second,
against the same route in spec 0042's checker: the two agree for 24 seconds,
through the garden and the solarium, and then **the live run stops dead in
the hall at z 4.23, x -2.10**, against a cabinet, and sits there for the
remaining seventy seconds. It never reaches the nook, never climbs, and
never touches the imaginarium at all.

What put it there was one about-turn at the far end of the solarium coming
back seven degrees short of its 180. Over the seven metres back to the door
that is most of a metre sideways, and the hall is narrower than that
mistake.

**Seven degrees is not a bug to fix.** Spec 0044 eases the arrows on and off
over `AIM_LAGS`, `play` waits its own margin after every key, and the
machine is not a clock. Any open loop long enough to be a clip will drift,
and this building asks the route to thread a bookcase one case wide, a
doorway a couple of degrees across and, since spec 0047, a hatch of 1.6
metres. The present answer is `pin:`, which walks into a wall to fix one
coordinate, and it works and it is not enough.

## What this is and is not

**A planner is half of it and the smaller half.** A path worked out by A*
and then replayed as fixed key durations drifts exactly as far as the
present one does. What kills drift is the follower: aim at the next place
every frame, so a turn that comes back short is simply aimed again on the
next one.

So: A* for the path, a follower for the walking, and the follower is the
part that earns the spec.

## Behavior

### The graph is the building's own floors

Spec 0043's `Room::grounds` already gives every floor in the building with
the room it belongs to. That is the node set at room scale, and the
doorways between them are the edges.

Within a ground, a grid of half a metre, built only over the rooms the
room-scale path passes through, so a journey across the hall does not pay
for the field.

### An edge exists if the walker can walk it

**Asked rather than worked out.** Two cells are joined when
`walk::walk` gets a body from one to the other in the time the distance
asks for. The repo's own tooling is the oracle: a graph that decides
reachability by looking at boxes is a second account of what `walk` does,
and the day they disagree is the day a planned path walks into a step it
cannot climb. It is also the only way the climb budget, the step height
and the slide all get taken into account without being restated.

### Following is aiming, every frame

The follower holds forward and turns toward the next place at the game's
own rate, which is `2.4` radians a second, so it looks like somebody
walking rather than a camera on rails. A place is reached when the body is
within a stride of it, and then the next one is aimed at.

**Which is why it cannot drift.** Nothing is timed. A turn that sweeps
short, a graze on a jamb, a tread that costs a frame of climbing budget:
each is just a slightly different position to aim from next frame.

### A portal is an edge you are a passenger on

Specs 0047 and 0048 put two kinds of crossing in the imaginarium, and
neither is a walk. The hatch changes which way is down. The door at the top
of a flight hands you to a scripted crossing for `TURNS_ROUND` during which
walking is suspended on purpose.

So a portal is an edge with three things the others do not have: it leaves
you in a different frame, it costs its own fixed time, and the follower is
a passenger for the whole of it and must not try to steer. Coming out, the
follower picks up from wherever the crossing put it, which is a known
place, so a portal is also the one edge that resets every error before it.

**This is the part that does not come free with A\*.** A navmesh has no
way to say "and now down is somewhere else".

### A route says places, not seconds

The table in `refresh-examples` keeps its shape and changes what is in the
last column: the pond, the solarium's planets, the trapdoor, the bookcase,
the baths, with a stand or a look or a working at each. How long each leg
takes stops being written down, because it stops being something anybody
sets.

**The places are named in the arcade and not in the table**, which is a
change from the paragraph above it and the reason is the drift rule. Twenty
places written into a shell script would be a second account of where the
pond, the planets, the bookcase and the baths are, and every one of them
already has an owner: `garden::pond`, `orrery::stands`, `cellar::opening`,
`spa::doorway`, `spa::pool`. So the route is `tour::clip`, which calls
those, and the column says `film:clip`.

What the table still sets is how long to run the camera, because that is a
fact about the film and not about the route. The route's own length is
printed by the test below: 341 metres of walking, which is about 98
seconds before the turns.

## What stops being tested, which is the cost

Today the clip's keys go through System Events into a window, and that is
the only thing in this project that exercises the real input path. Spec
0042 exists because that path broke: spec 0039's panel took all four arrows
while the sight was on it, the staged walk met the panel on its way to the
hotel, every turn after it was eaten, and nine seconds of a window pressed
against a fractal shipped.

A tour that steers itself cannot catch that, because there is no input to
eat.

So spec 0042's `--walk` stays exactly as it is, and the route it checks
stays a keyed one. It stops being what the clip is filmed from and becomes
what the input path is tested with: a short keyed walk that presses
something and reads the readout, run by the gate rather than by the camera.

## Acceptance criteria

- A path is found between two places in different rooms, and every step of
  it is one the walker can walk. — `tour::tests::every_step_of_a_path_can_be_walked`
- **Its control:** two places with no way between them get no path rather
  than a path through a wall. — `tour::tests::a_shut_room_has_no_way_in`
- Following a path arrives, from a start pushed half a metre off and a
  heading forty degrees out. — `tour::tests::a_follower_arrives_from_the_wrong_place`
- **Its control:** the same push with the present open loop does
  not. — `tour::tests::the_open_loop_does_not`
- A path through the imaginarium's hatch comes out in the frame on the
  other side. — `tour::tests::a_portal_edge_changes_which_way_is_down`
- The follower does not steer during a crossing. — `tests::a_passenger_does_not_steer`
- The region a journey lays its lattice over holds the way into a room
  whose only doorway is at the far end from the
  journey. — `tour::tests::the_region_holds_the_way_into_the_nook`
- The whole filmed tour reaches its last place. — `tour::tests::the_clip_s_tour_gets_all_the_way_round`

### Verified by hand

- Film it and watch. The thing to look for is whether it still reads as
  somebody walking rather than something being driven.

## Out of scope

**Anything the player does.** This is how a camera gets about for a clip
and a test. Nothing here changes walking for somebody at the keyboard.

**Doors that need working on the way.** The bookcase and the baths are
opened by pointing and pressing, and a route still says so at that place.
The planner is not told how to open things.

**Making the route pretty.** Spec 0042's notes are full of hard-won staging
about arcs rather than pivots and standing still in dead ends, and a
follower that only knows how to arrive will undo all of it. What a place
can say about how to approach it is a question for after this works at all.

**The field.** It is made as you walk into it and has no floors to put in a
graph. A tour that goes out there wants something else.
