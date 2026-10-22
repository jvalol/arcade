# 0034 Endlessly many endless coaches

**Status:** implemented
**Date:** 2026-10-22
**Built:** 2026-10-22, the same day it was written.

## Goal

Give the desk the one arrival it cannot take, and the rule that takes it.

Spec 0030 built a cycle of three: one guest, a coach of forty two, and a coach
with a seat for every counting number. All three are handled by shuffling.
Everybody moves up by some stride, which is why every move in this building
draws as a wave running west down the corridor.

This is the fourth, and it is the one that breaks that.

## The rule, which is already written

`games/aleph/src/rule.rs:117`, in the game this room came out of, under a
heading that says what it is: **the rule that is not on the dials.**

- a resident in room n goes to room **2ⁿ**
- seat s of coach c goes to room **pˢ**, where p is the c-th odd prime

Nobody collides, because a number factors one way only. 2ⁿ is only ever 2ⁿ and
3ˢ is only ever 3ˢ, so every coach gets endlessly many rooms of its own rather
than queueing behind the one before it.

aleph's own dials admit the gap rather than hiding it: ask them about coaches
and the verdict is `OneAtATime`. The rule exists in that repo and nothing in
this one calls it.

## Why it is worth building and not just true

**It is a scatter, not a wave.** Every move here has been a slide along the
corridor with a front you can watch travel. This one sends room 1 to room 2,
room 3 to room 8, room 10 to room 1024 and room 40 to a number with thirteen
digits. The corridor in front of you empties almost completely in one go,
because nearly everybody has gone somewhere too far to walk to. Nothing in
this building has done that, and it is the first move that looks like what it
is.

**And it leaves the hotel full and still endlessly empty.** Afterwards the
occupied rooms are exactly the prime powers: 2, 3, 4, 5, 7, 8, 9, 11, 13, 16
and on. Room 6 is empty for ever, and so are 10, 12, 14, 15, every number with
two different primes in it. Of the forty doors you can see, nineteen are lit
and twenty one are dark. The desk takes endlessly many endless coaches and the
corridor afterwards has more dark doors than lit ones, which is a better
ending than a full hotel and is true.

## Behavior

### A move is a function

`Hotel::apply` takes a stride and a shift and works out `n * stride + shift`.
That is the shape every move has had, and prime powers do not fit through it.

It takes a function instead: where does the guest in room n go, or nowhere
that can be drawn. The three existing arrivals keep their arithmetic and lose
nothing; this one is `2ⁿ` and the coaches are `pˢ`.

**Overflow is a real answer here.** Room 64 doubling is fine and room 64 to
2⁶⁴ is not a number, so the function returns nothing for those and the guest
leaves the drawn window, which is what actually happens to them.

### They walk further than next door

Spec 0033 has guests crossing from their door to the next number's, because
every move so far was a shuffle of one. Under this rule room 3 goes to room 8,
which is five doors west and on the same side.

So a crossing takes its destination from the move rather than assuming `n + 1`,
and most of them walk out of the drawn window entirely. A corridor emptying
westward is the point rather than a side effect.

### What the desk says

Three lines, drafts, in the concierge's voice the rest of that desk speaks in:

- Arriving: `Coaches without number, each of them without end. One moment.
  [COPY - Jake]`
- While it runs: `Everybody to a prime power. Nobody shares. [COPY - Jake]`
- After: `All of them in, and the hotel is emptier than it was. [COPY - Jake]`

### Where it sits in the cycle

Fourth, after `Endless`, and then back to `One`. The cycle is the building's
argument in order: a shuffle of one, a shuffle of many, a doubling, and the
thing none of those can do.

## What this must not become

**A fourth entry in a list.** It is a different kind of move and the code
should say so. If it ends up as another `(stride, shift)` pair, something has
been fudged.

**An explanation.** The hotel has never explained itself and the register next
door does not either. There is no panel about unique factorisation.

**A reason to draw a thousand doors.** The window stays forty. Most of this
move's destinations are outside it and that is the thing you are meant to
notice.

## What building it found

**The desk said the wrong thing while you watched the right one.** `Everybody
is moving up a room` is a true sentence about a shuffle and a false one about
a scatter, so this move has its own two lines.

**And one of them was unreachable, which makes three.** Spec 0030 put
`DESK_FULL` behind an arm that always matched, so the sentence carrying the
whole point of the hotel was never once on screen. This put `DESK_SCATTERED`
behind the filling arm and then, after that was fixed, behind its own
travelling arm, because `Hotel::travelling` answers `Some` during *both*
waves and shadows everything below it. The pair that was already there only
works because they happen to be in the right order.

What stopped the third one shipping was pulling those forty lines out of
`update` into `what_the_desk_says`, so that a test could ask the desk what it
says. `every_line_the_desk_has_is_reachable` could not have been written while
it was an expression in the middle of a match, and it failed twice before it
passed.

**Two of the first controls tested nothing**, which is worth as much as the
tests. Replacing `2ⁿ` with `n * 2` left `a_prime_power_is_nobodys_but_its_own`
green, because that claim is injectivity and doubling is injective too; the
control has to collide. And `the_ones_who_go_too_far_leave` survived breaking
the `None` branch, because `2ⁿ` fits in a `u64` for every room the corridor
draws, so that branch never runs: what the test actually guards is the
`(1..=DRAWN)` bound, and clamping to it is what makes it fail.

## Acceptance criteria

- No two guests are sent to the same room, which is the whole rule. —
  `aleph::tests::a_prime_power_is_nobodys_but_its_own`
- Everybody who can be drawn is drawn, and the ones who cannot leave the
  window rather than piling up at its
  edge. — `aleph::tests::the_ones_who_go_too_far_leave`
- After it, the lit doors are exactly the prime powers and the dark ones are
  exactly what is left. — `aleph::tests::what_is_left_is_every_number_with_two_primes_in_it`
- The three older arrivals are unchanged by the move becoming a
  function. — `aleph::tests::the_older_moves_are_the_moves_they_were`
- A crossing walks to where the move actually sends somebody, rather than to
  the next number. — `people::tests::they_cross_to_where_the_move_sends_them`
- The cycle has four entries and returns to the first. — `aleph::tests::the_cycle_comes_round`
- Every line the desk has is one somebody can actually
  see. — `tests::every_line_the_desk_has_is_reachable`

### Verified by hand

- The corridor emptying reads as everybody leaving rather than as a bug.
- The dark doors afterwards are noticed without being pointed at.
- Four arrivals is still a cycle somebody watches to the end rather than a
  menu they stop caring about, which is what parked aleph's dials.

## Out of scope

**Drawing a coach, or anything arriving from outside.** Spec 0029 settled that
nobody is drawn until they are in the corridor and spec 0033 only drew the
ones already in it.

**Saying which infinity is which.** Not here and not next door.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
