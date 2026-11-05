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

## Out of scope

**Anything behind him.** No hall, no gate, no door. The landing's far
parapet is the end of what this building has.

**Talking.** See above.

**Getting down.** You walk back, as spec 0054 says.
