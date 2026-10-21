# 0031 The one door with no number

**Status:** draft
**Date:** 2026-10-20

## Goal

Cantor's diagonal, walked rather than explained, as the exception to the sign
hanging over the hotel's corridor.

Spec 0029 put that sign up and it says THE ROOMS DO NOT RUN OUT. Spec 0030
made that true in front of you: the hotel is full, somebody arrives, one move
houses them, and it is full again with nobody gone. Every arrival so far has
been housed.

This is the one that cannot be.

## Why it is the sequel and not a new direction

The hotel is ℵ₀: countable infinity, and its whole trick is that a full hotel
always has room. The diagonal is the other half of that story, and it is the
more startling half, because it says some infinities cannot be listed at all.

It belongs here rather than anywhere because **it is a proof you can walk**,
and walking is what this building is for. Spec 0018 made a distance walkable,
thirty three kilometres of solar system. Spec 0030 made a count walkable.
This makes an argument walkable, which is a thing none of the rooms have
tried.

And it pairs with the room next door rather than standing beside it. The
hotel's sign is thirty paces away and this is its exception. Neither is as
good alone.

## Behavior

### The door with no number

**In the corridor, among the numbered ones, a short walk in.**

Every door in that corridor has a plaque with a number on it, which spec 0029
built and spec 0030 made follow you. This one has a plaque and nothing on it.

That is the whole of the signposting and it is enough, because it is the one
thing in forty doors that is different, and because it is also the joke: the
room that proves not everything can be numbered is behind the one door that
is not.

Near enough the lobby that you pass it. The building's own record on things
nobody finds is bad and spec 0020 says so in as many words.

### The register

An endless list of endless entries, which is what a hotel with a room for
every number would have to keep.

Rows stacked, each row a string of characters running away from you. The rows
recede the way the corridor does and for the same reason: drawn around
wherever you are standing rather than a fixed count, per spec 0030, which is
the known answer to making a thing read as having no end.

**One texture an alphabet, not one a row.** The corridor's plaques are a
texture each because every one says a different number. A register is the
same handful of characters over and over, so it is one small texture per
character and an instance per place, which is what this engine is for.

### The walk

You walk along the register and the diagonal climbs away from you: the first
character of the first row, the second of the second, the third of the third.

At each step the character you pass is taken, changed, and added to what you
are carrying. That is the whole of the proof and it is five words long: change
the diagonal, and what you have differs from every row in at least one place.

**It has to be yours rather than watched.** A camera that flies the diagonal
while you hold still is a cutscene, and this building has never had one. What
you are holding grows as you walk and stops when you stop.

### And then you take it to the desk

This is the end of it and the reason the two rooms are thirty paces apart.

You carry what you made back to the hotel's desk. The desk has housed every
arrival since the building opened, by shuffling, by doubling, and by the
prime powers if spec 0030 ever gains them.

It cannot house this one.

That is the first time that desk says no, and it says it about a guest you
wrote yourself, one line at a time, out of the register that was supposed to
have everybody in it.

## What this must not become

**A puzzle with an answer.** Nothing here is typed, chosen or guessed. The
diagonal is not a riddle, it is a construction, and the player's part is to
walk it and watch the thing come into their hands.

**A cutscene.** See above. If it plays while you stand there it is a video
with a room round it.

**An explanation.** There is no panel of text stating Cantor's theorem. The
hotel does not explain itself either: its sign is six words and the rest is
what you watch happen.

## Acceptance criteria

None of this is written.

- The unnumbered door is in the drawn window wherever you stand near the
  lobby, so it is passed rather than hunted. — `aleph::tests::the_blank_door_is_passed`
- Its plaque is blank and every other plaque in view is
  not. — `tests::one_door_in_the_corridor_has_no_number`
- The register has no last row and no last column, by the same window the
  corridor uses. — `register::tests::there_is_always_another_row`
- The diagonal takes the nth character of the nth row, wherever the walk
  starts. — `register::tests::the_diagonal_climbs_as_it_recedes`
- What you carry differs from every row that has been walked past, in at
  least one place, which is the proof
  itself. — `register::tests::what_you_hold_is_on_no_row`
- And it grows only while you move, so it is walked rather than
  watched. — `register::tests::standing_still_writes_nothing`
- The desk refuses this one and has refused no other. — `aleph::tests::the_desk_cannot_house_the_diagonal`

### Verified by hand

- The blank plaque is noticed without anybody being told about it.
- Walking the diagonal feels like doing something rather than triggering
  something.
- The desk saying no lands, having said yes every time before.

## Out of scope

**Drawing anybody.** Still. Spec 0029 settled what stands for a guest and
this building has never drawn a person.

**The prime power arrival**, which is spec 0030's cycle gaining a fourth
entry and is written in aleph's own `rule.rs` already. Related and separate.

**Explaining what ℵ₀ is.** The hotel does not and neither does this.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
