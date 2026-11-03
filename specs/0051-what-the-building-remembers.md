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

**Eight rooms are wired, because eight rooms already ask.** Not one of the
moments was invented for this: each is a press, a ride or a place the code
already knew about, and a spec already called that room's question.

| What | Where it already was | Spec |
| --- | --- | --- |
| Walking the register's diagonal | `Carried::reach` reaching the last row | 0031 |
| Handing that guest to the desk | the `Seen::Desk` arm | 0032 |
| Riding out to the sun | a leg of the ride ending at that stop | 0018 |
| Working one of the three hatches | the `Seen::Hatch` arm | 0038 |
| Turning the sixth display | `go_inside` | 0021 |
| Opening the bookcase | the `Seen::Case` arm | 0007 |
| Getting behind the end wall | `noise::room_at` | 0011 |
| Walking out into the field | `noise::room_at` | 0040 |

**The other three were left, on purpose.** The hall, the nook and the baths
each have more than one thing you can work and no spec saying which of them
is the room's question, and the garden and the gallery have a case for
asking nothing at all. Spec 0036 is plain that a room needing an object
planted in it to join the trail is a room the trail is wrong about, and
picking one of several presses to mean the room would be that fault wearing
a different coat. They arrive when somebody decides what each asks.

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
- The register and the desk fill it, through the same calls a frame
  makes. — `tests::the_rooms_give_up_what_they_ask`
- A press fills it, and working the same thing twice does not fill it
  twice. — `tests::a_press_gives_its_room_up`
- A ride ending at the sun fills it, and ending anywhere else does
  not. — `tests::arriving_at_the_sun_gives_the_system_up`
- A room reached fills it, and standing in a room that asks nothing does
  not. — `tests::a_room_reached_gives_itself_up`

### Verified by hand

- **Three controls, each watched failing.** The duplicate check removed, so
  walking the diagonal twice counted twice. The list sorted on insert, so
  the order was thrown away. The desk's one line deleted, so the guest was
  taken and the building did not notice.
- **That the record's own tests prove nothing about the wiring.** They were
  written first and passed while nothing filled the record at all, which is
  why the four tests above drive the real calls instead.
- **Two more controls that passed, and should not have.** Breaking the
  case's `if self.room.open` guard changed nothing, because `gave` ignores
  a repeat: recording on both halves of a toggle and recording on one come
  out the same, so the guard was dead and the assertion written for it
  could not fail. Both it and the hatch's copy of it are gone. And
  `arriving_at_the_sun` set the stop to `SUN_STOP` and then asked whether
  `SUN_STOP` was the sun, which is one question twice: pointing the
  constant at the way back left it passing. It now finds the sun by name in
  the list `stops` builds, and fails when the two disagree.

## Out of scope

**The trail.** Which room is at the end of it is spec 0036's open question
and remains Jake's. Nothing here picks it, and nothing here needs it picked.

**The other nine rooms.** Each asks something and most of those moments are
not in the code yet. Adding `Asked` variants for rooms that cannot yet say
when they were answered would be a list of promises.

**Anything on screen.** Spec 0036: no panel of ticks, no "3 of 8 found", and
the building has never had a HUD beyond one line under the sight.
