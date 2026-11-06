# 0058 A gate and a riddle

**Status:** in build
**Date:** 2026-11-06

## Goal

Put the last thing in the trail between you and Valhalla. Spec 0036 names
the riddle as what is still open; spec 0055 settled that the gate is a
riddle and not a game, and that it has to be one you can only answer from
having walked the building; spec 0056 deferred it on the argument that a
gate on a place nobody can reach without finishing the building is a second
lock on a locked door. The place is built, so that argument has expired.

Jake, 2026-11-06: Valhalla should be behind a large ornate wooden double
door, castle style, which swings open when the riddle is solved.

## Where it stands

The way through is already there and already three metres wide.
`bridge::landing` splits the far parapet round it, so the deck, the gap and
the hall's door are one straight way in. The gate goes in that gap, at
`bridge::top().x - bridge::LANDING_OUT`, which is the line where the
landing's slab ends and Valhalla's ground laps back under it.

The watchman stands 3.4 metres east of it and 2 metres north of its middle,
facing the climb. He is beside the way in rather than across it, which spec
0055 chose so you never have to walk through him, and it is also what lets
him watch you arrive at a gate he is not blocking.

## The riddle

**He has never seen the building.** He has stood at this end of the bridge
the whole time. What he asks is what it looks like from out here, and the
answer is to turn round and look at it.

That is the one thing spec 0055 asks of a riddle. There is nothing to look
up: the answer is not a word, it is the act of having turned round, and
nobody who has not climbed the bridge is ever asked. It also spends what
spec 0057 just built, which is the only thing of the arcade you can see
from this end: one lit lantern on the imaginarium's roof, a hundred and
thirty one metres off and four degrees down.

**You answer by looking, and the game reads it off the camera.** The
lantern is in view or it is not. `aloft::lantern` is a box with a middle,
the camera has a frustum, and the question is whether the one is in the
other. Nothing is typed, nothing is chosen off a list, and there is no
prompt telling you to turn round.

**He asks when you look at him**, which is how everything else in this
building is worked. Spec 0003's rule is that a room offers what you are
looking at; a man you have not looked at has not spoken to you. So the
exchange is: you look at him, he asks, you look away at the building, the
gate opens behind you.

**And there is no fail state.** Nothing is wrong to look at, nothing is
counted, nobody is turned away, and the gate has no second question if you
wander off. Jake left this open on 2026-11-05 and asked for suggestions;
this is the suggestion, and the reason is spec 0036's own, that the trail
must never become a checklist. A gate that can be failed is a score. What
this gate does is wait.

## The gate

**Two leaves in a frame, in a wall.** The parapet there is 0.62 high,
which is a kerb, so the gate brings a frame: two posts either side of the
three metre gap, a head across them, and the leaves hung off the posts.

**And the kerb either side of it is carried up to the head.** The first
build left the gate standing in the kerb, and the whole steading was in
plain sight to left and right of it over knee height, so the gate shut
nothing. The far parapet is a wall now, from the gate to each corner of
the landing, with the kerb as its plinth and a course along its top.

What that does and does not do is worth being exact about. It closes the
way through and it puts a wall in front of you when you face Valhalla. It
does not hide the place: the landing's other two sides are still kerbs, so
looking north-west or south-west over them the steading is there. A gate
is what you may not walk through, not a blindfold.

**It swings and the collider does not.** A box in this engine cannot tilt,
per spec 0007 and every door in the building, and that is why the
imaginarium's hatch slides instead. This one swings because Jake asked for
a swing and because the drawing is free: a leaf is drawn at whatever angle
it has reached. What cannot follow it is the collider, so there are two,
and which one is in the list is decided by the one number that says how far
open the gate is:

- shut, one slab across the whole opening
- anything else, two thin slabs against the posts, where the open leaves
  come to rest

**They swing away from you**, west, into Valhalla's ground, so a leaf never
sweeps through where you are standing while you watch it.

**One number is the whole state.** How far the gate has swung, nought to
one. Nought is shut and unanswered, and it never goes back, so there is no
second flag saying whether the riddle is answered and no way for the two to
disagree.

## Behavior

- Arriving on the landing, the gate is shut and the way to Valhalla is
  closed. Heimdall is the only thing up here to look at.
- Looking at him, he asks. The line is his and it is copy.
- Turning round and putting the arcade's lantern in view answers it.
- The leaves swing open over about three seconds and stay open.
- Looking at him after that, he says something else, once.

## Acceptance criteria

- Shut, the way to Valhalla is closed. — `gate::tests::the_way_through_is_shut`, walked from the middle of the landing straight at the opening rather than measured, because a collider that fills an opening on paper is one with a body-width gap beside it if a post is a finger out.
- Open, it is not. — `gate::tests::the_way_through_opens`
- It stays shut until the riddle is answered, and once it is moving it does not stop. — `gate::tests::it_stays_shut_until_the_riddle_is_answered`
- No part of a leaf ever sweeps the landing you stand on to watch it. — `gate::tests::the_leaves_open_away_from_the_landing`, at every tenth of the swing.
- Looking at the watchman is how he asks, and there is no watchman before there is a bridge. — `gate::tests::the_watchman_is_looked_at_from_where_you_arrive`
- Turning round to the building is the answer, and looking through the gate or at the sky is not. — `gate::tests::the_answer_is_looking_back_at_the_building`

### Verified by hand

- Shut, from the landing's east edge: a wall with a gate in it, and Valhalla behind it. — `POSE=-137.2,26.0,15.15,-1.5708,0.06` with `--done`.
- Open, from the same place: the leaves back against the jambs and the hall's door lit through the opening.
- The watchman asking. — `POSE=-138.0,26.0,15.6,-2.182,-0.248`. He is 4.03 metres off where the deck ends and `room::REACH` is 4.0, so you take a step towards him before he speaks.

## What is not settled

**The words.** Both of his lines are drafts and carry `[COPY - Jake]`, so
`check-all` holds until Jake edits them or signs them. They live in
`src/says.rs` with the rest of the copy, which is the file `sign-copy`
hashes.

## Out of scope

**A gate anywhere else.** This is the one in the building.

**Shutting it again.** Nothing in this building takes back a thing you have
done, and a gate that closes behind you is a corridor, which spec 0055
already refused for a different reason.

**A second riddle.** One gate, one question.

**Anything Heimdall does besides ask and answer.** He does not follow you,
he does not repeat himself, and he does not comment on how long you took.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
