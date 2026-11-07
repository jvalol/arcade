# 0058 A gate and a riddle

**Status:** implemented
**Date:** 2026-11-06, amended 2026-11-07

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
answer is a thing you can only have by turning round and looking at it.

That is the one thing spec 0055 asks of a riddle. There is nothing to look
up, because the answer is behind you and nowhere else, and nobody who has
not climbed the bridge is ever asked. It also spends what spec 0057 just
built, which is the only thing of the arcade you can see from this end: one
lit lantern on the imaginarium's roof, a hundred and thirty one metres off
and four degrees down.

**You answer in writing, and turning round is what tells you the answer.**
The first build read it off the camera: `aloft::lantern` is a box with a
middle, the camera has a frustum, and whether the one was inside the other
was the whole of it. Jake read that as a proof you had walked the place
rather than as a riddle, which is what it was, so it types now. He asks for
a word for a thing he cannot see, turning round is how you get the word,
and spec 0059 is the line you write it on. What he takes is
`says::WATCHMAN_ACCEPTS`, kept beside the question because a question and
the answers it accepts are one thing.

**He asks when you look at him**, which is how everything else in this
building is worked. Spec 0003's rule is that a room offers what you are
looking at; a man you have not looked at has not spoken to you. So the
exchange is: you look at him, he asks and a field opens, you turn round and
look, you write what is out there, and the gate opens behind you.

**And there is no fail state.** A line that is not one of his gets a reply
and the field stays open, nothing is counted, nobody is turned away, and
the gate has no second question if you wander off. Jake left this open on
2026-11-05 and asked for suggestions; this is the suggestion, and the
reason is spec 0036's own, that the trail must never become a checklist. A
gate that can be failed is a score. What this gate does is wait.

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
- Turning round tells you the answer, and a field takes it. A line that is
  not one of his gets a reply and the field stays open.
- The leaves swing open over about three seconds and stay open.
- Looking at him after that, he says something else, once.

## Acceptance criteria

- Shut, the way to Valhalla is closed. — `gate::tests::the_way_through_is_shut`, walked from the middle of the landing straight at the opening rather than measured, because a collider that fills an opening on paper is one with a body-width gap beside it if a post is a finger out.
- Open, it is not. — `gate::tests::the_way_through_opens`
- It stays shut until the riddle is answered, and once it is moving it does not stop. — `gate::tests::it_stays_shut_until_the_riddle_is_answered`
- No part of a leaf ever sweeps the landing you stand on to watch it. — `gate::tests::the_leaves_open_away_from_the_landing`, at every tenth of the swing.
- Looking at the watchman is how he asks, and there is no watchman before there is a bridge. — `gate::tests::the_watchman_is_looked_at_from_where_you_arrive`
- Looking at him opens a field, a wrong line clears it without moving the gate, and one of his words opens it. — `tests::the_watchman_is_answered_in_writing`, which types both in turn. How a line is read is spec 0059's, held by `typed::tests::an_answer_is_read_the_way_it_was_meant`.

### Verified by hand

- Shut, from the landing's east edge: a wall with a gate in it, and Valhalla behind it. — `POSE=-137.2,26.0,15.15,-1.5708,0.06` with `--done`.
- Open, from the same place: the leaves back against the jambs and the hall's door lit through the opening.
- The watchman asking. — `POSE=-138.0,26.0,15.6,-2.182,-0.248`. He is 4.03 metres off where the deck ends and `room::REACH` is 4.0, so you take a step towards him before he speaks.

## What is not settled

**Nothing of the gate itself.** His two lines and the list of words he
takes were drafts carrying `[COPY - Jake]` when this was written. Jake
signed `src/says.rs` with `sign-copy` on 2026-11-06, which is what took the
marker out, and `check-voice` reads the hash in `.copy-signed` rather than
the marker from then on.

**And that signature is in the way of one thing.** The doc comment over
`says::WATCHMAN_ASKS` still says the answer is not a word but the act of
turning round, which was true of the first build and is not true of the
file three lines under it. The signature is of the whole file's bytes, so
correcting a comment in it turns `check-voice` red, and signing it again is
Jake's. Left wrong and written down here rather than quietly broken.

## Out of scope

**A gate anywhere else.** This is the one in the building.

**Shutting it again.** Nothing in this building takes back a thing you have
done, and a gate that closes behind you is a corridor, which spec 0055
already refused for a different reason.

**A second riddle.** One gate, one question.

**Anything Heimdall does besides ask and answer.** He does not follow you,
he does not repeat himself, and he does not comment on how long you took.
