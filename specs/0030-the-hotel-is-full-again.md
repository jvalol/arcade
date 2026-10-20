# 0030 The hotel is full again

**Status:** implemented
**Date:** 2026-10-20
**Built:** 2026-10-20, the same day it was written, because Jake walked spec
0029's corridor again and the four things he found were this spec: the lights
clip, the doors stop while the hallway goes on, opening a door does not seem
to work, and there is no obvious way to check a customer in.

## Goal

Make the corridor of spec 0029 demonstrate the thing it is about. It does not
yet. You can make its doors open in a wave, and Jake walked it and said
opening and closing doors does not really demonstrate the point.

The point is one sentence and the room has to produce it:

> The hotel was full. A coach arrived. Nobody left. The hotel is full again,
> and there are more people in it.

Spec 0029 stops at the middle of that. It frees the odd rooms and leaves them
free, which reads as arithmetic, and arithmetic is what aleph the game already
had on a dial.

## The two wrong answers, so nobody writes a third

**The dials.** aleph set `a` and `b` on two dials and pressed a button. Its
own parked note says they are a menu with extra steps: there are three right
answers, a player has them in a couple of minutes, and no setting makes a
stride of four better than a stride of two. Carrying that in here carries the
reason the game was put down.

**A key that checks a guest in and out.** Written and taken out the same hour
on 2026-10-20. Jake said there was no way to add a guest and this answered him
with a light switch. A door that can be shut is not a paradox, and a hotel you
fill one room at a time is a hotel with a finite number of rooms.

What both have in common is that they let you set the state. The paradox is
not a state. It is a **move**, and a move needs somebody it is done for.

## Behavior

### Somebody is at the door

The desk says who wants a room. aleph writes this line already and it is the
right one: *a coach of 42 is at the door*, or *one guest is at the door*.

Until somebody is at the door there is nothing to do, and the corridor is what
it is now: full, every door shut, every threshold lit. That is the premise and
it should be the resting state rather than a thing you set up.

### One move, and it is the guest's problem to pose

A rule frees rooms. Which rule is the question aleph could not make
interesting, and the reason is that it asked the player to find a rule that
works when every rule works.

**So the arrival sets the difficulty rather than the dials setting the
answer.** One guest needs one room. A coach of forty two needs forty two. A
coach with a room for every counting number needs infinitely many, and that is
the one that cannot be done by shuffling and has to be done by doubling. The
move is the same key each time; what changes is whether it is enough.

That is the shape to aim for and it is not settled here. What is settled is
that the question comes from the door, not from a dial.

### And then the rooms fill

**This is the step that turns emptying into the paradox and it is the one
spec 0029 has no part of.**

The wave of spec 0029 runs away from room 1 and leaves the odd side open. A
second wave follows it, the arrivals walking in: the freed rooms shut one
after another, their thresholds light, and the corridor goes back to how it
looked before.

Then the desk says it. The hotel is full. Nobody left. There are forty two
more people in it than there were.

A reader who has watched that does not need the arithmetic explained, and a
reader who has only watched the odd side empty has been shown half a
subtraction.

### The doors must stop ending

Spec 0029 draws forty rooms and walls off what is past them, which Jake found
by walking to room 36. Freeing infinitely many rooms is a sentence you cannot
say in a corridor with a last door in it.

**Rooms drawn around wherever you are standing**, rather than a fixed forty
from the lobby. The arithmetic for where room `n` stands already takes any
`n`, so what changes is which range is drawn and that the shell follows.

The constraint that stopped spec 0029 making the shell long is written down
there and holds here: at a hundred and ten metres it put the reach fill over
its cell budget and two of this building's own tests came back saying "the
fill reached nothing at all". A shell that follows the camera is a shell that
is never long, which is why this is the way through rather than simply
building more of it.

## What building it found

**The desk had to become the thing you work.** The only thing you could point
at in that corridor was a door, which is why there was no obvious way to check
anybody in: there was nothing to check them in *at*. The desk names itself
under the sight now and says who is at the door.

**Somebody is always at the door**, which is the resting state rather than a
thing you set up. One guest, then a coach of forty two, then a coach with a
room for every number, and round again. You do not choose the move; you find
out whether shuffling is enough, and for the third one it is not.

**And the corridor's length cost something, which is named in `room::fillable`.**
The shell is two kilometres so the doors never stop, and the reach fill has a
cell budget that length puts it over. The fill is given the building with the
corridor pulled back out of it, so it does not reach down there, and spec
0048's coplanar narrowing does not narrow inside the hotel. What is in there
is forty doors of one shape repeated.

## Acceptance criteria

- A coach of `n` is housed by the move, and after it the hotel is as full as
  it was. — `aleph::tests::the_hotel_is_full_again`
- And it has `n` more people in it than before. — `aleph::tests::nobody_left_and_more_arrived`
- One guest is housed without doubling anything, because a shuffle is enough
  and the room should not say otherwise. — `aleph::tests::one_guest_does_not_need_the_doubling`
- A coach with a room for every counting number is housed, which is the case
  only doubling answers. — `aleph::tests::an_endless_coach_is_housed`
- The arrivals walk in after the rule rather than with it, so the two waves
  are two. — `aleph::tests::the_filling_follows_the_freeing`
- There is no last door: whatever room you stand outside, there are rooms
  drawn past it. — `aleph::tests::there_is_always_another_door`
- And the numbers on them are right wherever you stand, not only near the
  lobby. — `aleph::tests::the_numbers_are_right_far_from_the_desk`

### Verified by hand

- Watching it happen says what it means without anybody explaining it.
- The second wave reads as people arriving rather than as doors shutting.
- You cannot reach a last door by walking.

## Out of scope

**Drawing guests.** Spec 0029 settled what stands for one: light under a shut
door. Nothing here draws a person and this building still never has.

**The dials.** See above. One key, and the arrival sets what it has to be
enough for.

**The corridor's fittings, lighting and shell**, which are spec 0029's and
are built.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
