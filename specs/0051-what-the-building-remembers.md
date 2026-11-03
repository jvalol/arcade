# 0051 What the building remembers

**Status:** implemented
**Date:** 2026-11-03

## Goal

The building knows which rooms have given up what they ask, and in what
order. Nothing reads it and nothing draws it.

## Why it belongs here

Spec 0036 is a trail through every room, and it ends by saying it is waiting
on one decision that is Jake's: which room is at the end. It also says, in
the same breath, that the decision is not what the work is.

> The hall, the nook, the cellar, the baths, the garden, the space behind
> the wall and the back room are each sealed, and nothing anywhere knows
> what you have already done. That is the real work in this spec and it is
> not a trail, it is plumbing.

This is that plumbing, and it holds whichever room is picked.

## Behavior

**`trail::Trail` records that a room gave up its answer, in order.** A list
and not a set of flags: two rooms answered is not the same as two rooms
answered in that order, and a flag per room throws away the only thing in
the record that could mean anything.

**A room answering twice answered once.** You can walk the register's
diagonal again, and spec 0032 says so in as many words: walk it again and it
will have found somebody else. What the building wants to know is that the
room has been done, not how many times, and a count would be a score.

**Two rooms are wired, because two rooms already ask.** Walking the
register's diagonal to the last drawn row, and handing what that leaves you
to the desk. Both moments were already in the code and neither was invented
for this. Spec 0036 is plain that a room needing an object planted in it to
join the trail is a room the trail is wrong about, so the other nine arrive
as their rooms grow a moment worth recording.

**Nothing reads it but `WHERE`.** The record is printed on stderr behind the
same env var that already says where you are standing, which is a
diagnostic and not a window. Spec 0036's rule is that somebody who never
finds the trail should not be able to tell it is there, and a line drawn in
the building would be exactly that.

**There is no `len` and no `has`.** A room that wants either reads the list.
Writing them before anything asked would be two more ways to say what one
list already says, which is the fault this building keeps making.

**It is not `noise::Room`.** That enum says which room you are standing in,
worked out from your feet, for deciding what you can hear. It has a stair
and a field, which ask you nothing, and no hotel, register or gallery, which
ask the three hardest questions here. Two lists that overlap and answer
different questions, so each says its own.

## Acceptance criteria

- A new building has been given nothing. — `trail::tests::it_starts_with_nothing`
- A room that answers twice answered
  once. — `trail::tests::a_room_that_answers_twice_answered_once`
- The order they came in survives. — `trail::tests::it_keeps_the_order_they_came_in`
- The two moments that exist actually fill it, through the same calls a
  frame makes. — `tests::the_rooms_give_up_what_they_ask`

### Verified by hand

- **Three controls, each watched failing.** The duplicate check removed, so
  walking the diagonal twice counted twice. The list sorted on insert, so
  the order was thrown away. The desk's one line deleted, so the guest was
  taken and the building did not notice.
- **That the record's own tests prove nothing about the wiring.** They were
  written first and passed while nothing filled the record at all, which is
  why `the_rooms_give_up_what_they_ask` drives the real calls instead.

## Out of scope

**The trail.** Which room is at the end of it is spec 0036's open question
and remains Jake's. Nothing here picks it, and nothing here needs it picked.

**The other nine rooms.** Each asks something and most of those moments are
not in the code yet. Adding `Asked` variants for rooms that cannot yet say
when they were answered would be a list of promises.

**Anything on screen.** Spec 0036: no panel of ticks, no "3 of 8 found", and
the building has never had a HUD beyond one line under the sight.
