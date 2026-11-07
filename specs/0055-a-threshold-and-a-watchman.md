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
  did not land, and the reason given here first was wrong. The arcade was a
  black slab with one tower off it, at noon as well as at dusk, and the
  guess was that its roof was an unlit plane. The roofs are drawn and lit.
  What stood behind them was spec 0011's room, a hundred and fifty nine
  metres square and as near black as this building draws, because from
  inside it that is space. It is not drawn from out of doors now and the
  slab is gone.

- **The building is still small from up there, and the reason is
  foreshortening rather than anything hiding it.** Said twice in
  conversation to be the garden's 4.2 metre wall standing in the way, and
  that is wrong: the ray grazing the top of that wall is at 1.24 metres by
  the time it reaches the hall, whose roof is at 3.2, so the roof clears it
  by two. What is actually true is that the hall is 134 metres off and 9.7
  degrees below the eye, and thirty metres of flat roof seen from 9.7
  degrees presents about five metres of apparent depth: two degrees of the
  view, call it forty-five pixels of that window.

  So a better roof would draw a better forty-five pixels. What reads at
  that angle is vertical extent and nothing else, which the imaginarium's
  tower proves by being the one part of the building you can make out. If
  the look back is ever to land, it wants things standing up on the roof
  rather than paint on it. Left alone for now, because spec 0055's own
  Valhalla goes at the top of this bridge and you will be facing the other
  way.

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

**The style of Valhalla.** Jake's own, and the nearest thing here to a
decision: Kinkade-like, a village, warm light with its source out of frame,
with more flourish than that and a touch of neon. Van Gogh is spent on the
hall's `starry`. Maxfield Parrish was offered and refused as dated.

**And the mythology stays**, which is the other half of what Jake wants and
looked at first like the opposite of the first half. It is not, and the
hinge is that what makes Valhalla worth arriving at in the myth is not the
fighting. It is the hall: fire, light, company, after the cold. A Norse
hall at dusk with the light coming out of its door is the warm-window
picture already, with none of the cottages.

Four things follow, and none of them is a compromise between the two.

**A hall with a steading round it.** The myth gives one great hall and Jake
wants somewhere village-like. A Norse settlement is a longhouse with
outbuildings round it, so that is both at once rather than a blend of them.

**Gold, and a roof of shields.** Grímnismál gives Valhalla spear rafters
and a roof of shields, and the sources call it gold-bright. Shields taking
a low light is the warm-window image arrived at from the myth's own detail
instead of from a painting.

**The neon is the aurora.** `aim::NEON` is 3.2 in cyan and magenta, which
is the aurora's palette. Northern lights over a steading is at home in the
myth, gives the register Jake is after, and rhymes with the bridge, which
plenty of people read as the aurora in the first place. With the sky doing
that, the made signs can be one or two.

**A few people about, and none of them fighting.** The myth fills the hall
with the dead Odin picked off battlefields, who fight each other all day,
are healed every evening and feast all night, waiting for the last war.
Keep the figures and drop all of that: spec 0033 draws people already and
spec 0038's whole room is other people using it. Somewhere peaceful is not
empty, and it is not a barracks either.

This also settles a wobble raised while talking it through, and settles it
better than the answer offered then. With a Norse place behind him the
watchman does not have to stop being Heimdall; he was only awkward while
what he guarded was an English village.

**The martial detail is architecture and not mood.** Shields on a roof and
spears for rafters, and then stop. No weapons left about, nobody waiting
for Ragnarök. The myth supplies the building. It does not get to supply how
the room feels, which is settled above and is rest.

**What the aurora will actually cost, which is more than was claimed.** It
was offered in conversation as cheap, on the grounds that the sky is a
picture on a dome and this would be paint. Two things in `garden::dome_mesh`
say otherwise and both were found by reading it rather than by guessing.

The weather moves by sliding the picture's own `uv` across the dome, at
`WEATHER` and `DRIFT`. Anything painted into that picture drifts with the
clouds, and an aurora that slides west all evening is a cloud that happens
to be green.

And there is one sky for the whole building. There were two, the garden's
and the field's, each plainly visible from under the other, and they were
made one deliberately. So an aurora over Valhalla is an aurora over the zen
garden as well. That may be worth having and it is a decision rather than a
detail, and whoever builds it should make it on purpose.

**The neon is why that is the building's answer and not an import.** This
arcade already has a light language of its own: `aim::NEON` is 3.2, cyan
and magenta, and it is over the hall's door and down in the cellar. A
village at the top of the bridge lit with warm windows and one or two of
those signs is the building turning up in its own heaven. Every other style
offered so far, this file's included, has been something borrowed from
outside and carried in.

**And it is a metaphor rather than a myth, which is the whole of why the
neon belongs.** Jake, answering the objection that neon says city and
commerce while Valhalla says hall of the slain: this one is an afterlife
the way a last room is an afterlife, and what it has to say is that you
made it somewhere peaceful after a journey. Read that way a little neon is
not a clash, it is what keeps the place happy rather than mournful for the
dead, in an engine built this decade.

That brief is also sharper than the name. Somewhere peaceful at the end of
a journey is a place to stand and be, which rules out the things Valhalla
would otherwise invite: no throne, no ceremony at the door, nothing further
to go on to.

**Rest is the reward**, which is Jake's correction to the first wording
here, and the two are not opposed. They answer different questions. What
is in the room: nothing handed to you, no throne and no ceremony. What the
climb buys: the rest itself. Both hold, and the first is why the second
has to carry the weight.

So the test is whether somebody stands up there a while before walking
back down.

**And it is the end.** Jake: nobody is ever going to leave, and unless it
is expanded on that is the end of the path and of the trail. So whatever
goes in there is the last thing anybody sees of this building, which is an
argument for spending on it and an argument against ever putting anything
past it. A thing beyond Valhalla turns Valhalla into a corridor, which is
the fault the bridge had when it stopped level in mid-air, one floor up.
That is cheap to decide now and expensive once it is built.

**The end of the trail and not the end of playing.** There is no win
screen in this building and nothing to quit to. The player stays as long as
they want and then goes back to the rooms. Spec 0036 asks for exactly that,
nobody rescued and nothing explained.

## The parapet is a parapet and not a bar of grey

Amended 2026-11-06. This built the landing with a kerb 0.62 high all the way
round and nothing marking its length, and a surface with nothing on it reads
as one thing however long it is: from the top of the deck the whole far side
was one bar of grey corner to corner. It also gave you a twenty-six metre
drop behind something knee high, which the walk tests say you cannot go over
and the eye does not believe for a moment.

So: piers at intervals in a warm stone, a railing of thin iron balusters
between them, and the kerb becomes the plinth it already was. The railing
reaches a hand's height and you can still see over it, which is what the
landing is for.

**A pier stands through the railing and not under it.** The first ones were
0.30 on a 0.62 kerb against a 1.05 rail, so the thing meant to carry the
railing stopped half a metre below it.

**And a baluster stands between two piers and never inside one.** The ends of
a run and the centres of the piers disagree by a quarter of a metre, so a
baluster placed off the run alone lands inside the pier at every corner.

This is where `bridge::piers` and `bridge::balusters` are written down. They
cited a spec 0059 that did not exist until the day this was amended, which is
the accident spec 0059 opens with.

- One pier at a corner and not two. — `bridge::railing_tests::a_corner_has_one_pier_and_not_two`
- A pier stands through the railing it carries. — `bridge::railing_tests::a_pier_is_sunk_into_the_kerb_it_stands_on`
- And no baluster stands inside one. — `bridge::railing_tests::no_baluster_stands_inside_a_pier`

## Out of scope

**Talking.** See above.

**Getting down.** You walk back, as spec 0054 says.
