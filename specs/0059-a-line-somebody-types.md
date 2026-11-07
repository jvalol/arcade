# 0059 A line somebody types

**Status:** implemented
**Date:** 2026-11-06, written after the fact

## Why this is late

**It was built and never written down.** The engine's spec 0087 names this
one in its own Out of scope: "What holds them, draws them, shows a caret and
decides what Return means is the game's. The arcade's spec 0059 is one." The
code went in the same evening and the file did not.

Twelve places in this crate cite 0059 and nothing noticed, because
`check-specs` reads a spec and asks whether the tests it names exist. Nothing
read the code and asked whether the spec it names exists, so the citation rots
in the one direction nothing looks. Found 2026-11-06 while picking the number
for spec 0060, and the check now goes both ways.

**Three different pieces of work claimed this number**, which is the other
half of the same accident: the typed field, the field's leaf litter, and the
landing's piers and balusters were built in one stretch and all three wrote
"0059" into their comments. This spec is the first, because that is what the
engine's 0087 points at. The other two have gone back to the specs that own
the places they are in: the litter to spec 0040 and the parapet to spec 0055.

## Goal

One thing in this building asks you to write something, and spec 0058's
watchman is it. What this is: the line, the caret, and what Return does.

## Behavior

**One field, not a widget set.** No layout, no scrolling, no selection and no
paste. What is here is what a line of typing needs: the characters, where the
caret is, and what to do when Return lands. A text box that could go anywhere
is a toolkit nobody asked for, and this building has one place to type.

**The characters come off `Game::typed`**, which is the engine's spec 0087:
what a press came to after the layout, the modifiers and the dead keys have
had their say, rather than which key it was. Backspace, Return and Escape are
keys and arrive the other way, which is where a field has to look for them
anyway.

**The question travels with the field.** It is drawn over the words you are
writing, and a field that held only the answer would make whoever draws it
work out what was asked, which is the same fact in two places.

**An answer is read the way somebody means it.** Trimmed, folded to lower
case, runs of space squeezed. A capital, a stray space at the end or two in
the middle is not somebody being wrong. One function does it, used on both
sides of the comparison: what is typed and what is accepted have to be folded
the same way, or the fold is the thing that refuses a right answer. The hazard
is on record: `sign-copy`'s own header says that striking a marker out of a
Rust string left the space after it, so seven of the watchman's strings
signed as " a light" on 2026-11-06. The committed `src/says.rs` carries no
leading space and hashes to its signature, so that one was put right rather
than survived by the fold.

**It stops at `MOST` rather than scrolling.** Forty-eight characters, which
is long enough for a sentence somebody means and short enough that the line
never wraps. A field that runs off its own plate is a field you cannot read
what you wrote in.

**Backspace takes a character and not a byte.** `truncate(len - 1)` on
anything with an umlaut in it cuts a character in half and Rust refuses,
which is a crash in a field anybody can type into.

**A wrong line is a reply and not a failure.** The line clears, the question
stays, and the field says it has been answered before so whoever draws it can
say so. Spec 0058: no fail state, because a gate that can be failed is a
score.

**And the keys that walk you stop while it is open.** W is a letter now. A
field you can type into while WASD still drives is a field that walks you off
the landing one character at a time.

## Acceptance criteria

- What arrives is what was typed. — `typed::tests::it_takes_what_was_typed`
- An answer is read the way it was meant, in all three ways it can be written. — `typed::tests::an_answer_is_read_the_way_it_was_meant`
- A field with nothing in it says so, whatever the spacebar has been doing. — `typed::tests::a_field_of_spaces_is_empty`
- Backspace takes a character and not a byte. — `typed::tests::it_rubs_out_a_character_and_not_a_byte`
- It holds no more than a line, whether that arrives one character at a time or all at once. — `typed::tests::it_holds_no_more_than_a_line`
- Looking at the watchman opens it, W does not walk you while it is open, a wrong line clears without moving the gate, and one of his answers opens it. — `tests::the_watchman_is_answered_in_writing`

### Verified by hand

- Typed at the gate with a wrong answer and then a right one, 2026-11-06.

## Out of scope

**Anywhere else to type.** Jake named admin mode as a reason to build this at
all, and nothing here is admin mode. What this is is the one field the
watchman asks for; the second caller is what decides whether any of it has to
move.

**Composition.** Dead keys arrive already combined, which is the engine's
doing and is the right answer for a Latin keyboard.

**Clipboard.** Nothing here pastes.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
