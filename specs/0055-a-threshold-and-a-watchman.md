# 0055 A threshold and a watchman

**Status:** implemented
**Date:** 2026-11-04

## Goal

Jake, 2026-11-04, answering the question spec 0054 left at the top of the
bridge: a platform at the top of the arc, one figure on it, facing the
climb. Nothing to enter, nothing said.

## Why the bridge needed an end

Spec 0054 built Bifröst and stopped the deck level in mid-air, on the
argument that a crossing is not a room and that inventing a fourth room as
the prize is how you get a room nobody wanted. It said in the same breath
that the myth argues the other way: a rainbow arcs up and comes back down,
and Bifröst goes up and *arrives*, at a threshold with somebody standing on
it. Looked at from out in the field the first build reads as unfinished
rather than as deliberate, which is what settled it.

**A threshold is not the fourth room.** There is nothing to go into and no
door to open. You climb, you arrive somewhere that is a place rather than
the end of a plank, somebody is already there, and then you look back east
at the building you came out of, which is the whole of the prize spec 0054
named.

## Behavior

**A landing, flush with the top of the deck.** Wider than the deck and
running on past it, so that arriving is stepping out onto something rather
than reaching the last plank. The deck is six metres across and this is ten,
which is the difference you can feel underfoot without it reading as a
courtyard.

**Stone, and the only thing out here that is not light.** The bridge glows
because a rainbow is light rather than paint. The landing does not: it is
the thing the light arrives at, and a glowing platform at the end of a
glowing bridge is more bridge.

**A parapet on the three far sides and not on the fourth.** You arrive over
the open side. The other three are walled low, because a slab twenty-six
metres up with nothing round it is a diving board, and because a parapet is
what makes a platform read as somewhere to stand.

**One figure, facing the climb.** Standing, not walking. It is on the
landing and off to one side of the way you arrive, so that you are met
rather than blocked.

**Nothing is said and nothing happens.** No greeting, no gate opening, no
line of text. Spec 0036 is plain that this building has never had a HUD
beyond one line under the sight, and that nobody is rescued and nothing is
explained at the end. The watchman watches. That is the whole of it.

**And it fixes its gaze rather than following you.** Heimdall sees
everything and a head that tracks you up the climb is the strongest wordless
way to say you were expected. It is also the thing spec 0033 already got
wrong once, in the hotel lobby, where a figure that watched the entrance
read as creepy enough for Jake to say so on sight. The difference there was
that it watched a door nobody had earned. This one would watch a bridge only
somebody who finished the building can be on, which is an argument and not a
proof, so it stands still until somebody has seen it standing still.

**It is there exactly when the bridge is.** Same condition, read from the
same list, because a watchman on a bridge that is not there is a man
standing in a field.

## Acceptance criteria

- Nothing of it exists until every `Asked` has been given. — `bridge::tests::no_threshold_until_the_building_is_done`
- The landing's top is level with the top of the deck. — `bridge::tests::the_landing_is_flush_with_the_deck`
- It laps the deck rather than meeting it. — `bridge::tests::the_landing_laps_the_deck`
- It is wider than the deck on both sides. — `bridge::tests::the_landing_is_wider_than_the_deck`
- The parapet closes three sides and leaves the way you arrive open. — `bridge::tests::the_parapet_leaves_the_arriving_side_open`
- You can walk off the deck onto it. — `bridge::tests::walked_from_the_top_of_the_deck_onto_the_landing`
- And the parapet stops you walking off the far edge. — `bridge::tests::the_parapet_stops_you_at_the_far_edge`
- The watchman stands on the landing, not in the air and not in the parapet. — `bridge::tests::the_watchman_stands_on_the_landing`
- Facing back down the climb. — `bridge::tests::the_watchman_faces_the_climb`
- And clear of the way you arrive. — `bridge::tests::the_watchman_is_not_in_the_way`

### Verified by hand

- **Walked up to it**, with the trail filled in from a temporary block that
  was taken out again. It reads: the deck runs out onto stone, the parapet
  is a kerb at about waist height, and one figure is standing there with its
  back to the far edge.
- **Stood beside him on the landing.** He faces the climb, which from up
  there means his back is to you once you have passed, which is right.
- **And looked back east, which is what spec 0054 says the prize is.** It
  does not land. At a hundred and thirty-six metres and twenty-six up the
  arcade is a black slab with one tower off it, at noon as well as at dusk,
  because the roof is an unlit plane nobody has ever had a reason to look
  at. The threshold is not what is wrong there, and the fix is not this
  spec's.

## What Jake has since decided, and this does not build

**Heimdall is the last gate, and what is behind him is Valhalla.** Jake,
2026-11-04, after walking this: he lets you into a room, there is something
to answer first, and what is through there is where something stunning and
beautiful should be. So the far parapet is not the end of what the building
has, and the two lines this section used to open with, saying there is
nothing behind him and nothing to enter, are wrong rather than out of scope.

Nothing of it is built. What is built is the landing and the figure, and
both hold: a threshold is still a threshold when there is a door behind it.

**The gate is a riddle and not a game.** Jake's word is sphinx-like. That
rules out the thing this file argued for an hour earlier, which was the
arcade's own `starry` at full size, and he ruled it out on better grounds
than the ones it was argued on: Van Gogh is already in the building, in a
cabinet you walk past every time you cross the hall.

**What the riddle has to be, if spec 0036 is to hold.** That spec's rule is
that a room's clue is the answer to the thing that room already asks, and
that a room needing something planted in it is a room the trail is wrong
about. Read strictly that rule governs the eight rooms and not the gate at
the end, so a riddle here breaks nothing. Read for what it is for, it says
the useful thing: a riddle whose answer you could have looked up is a
trivia gate, and a riddle whose answer you can only have from having been
in this building is the trail asking you one last time what you saw. The
second is worth building and the first is not.

**The style of Valhalla is open.** Kinkade-like, a village, warm light with
its source out of frame, is Jake's own and is the one to beat. Van Gogh is
spent. Nothing is decided.

## Out of scope

**Talking.** See above.

**Getting down.** You walk back, as spec 0054 says.
