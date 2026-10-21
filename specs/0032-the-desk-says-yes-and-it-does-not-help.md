# 0032 The desk says yes and it does not help

**Status:** implemented
**Date:** 2026-10-21
**Built:** 2026-10-21, the same day it was written.

## Goal

Make the register room prove the thing it claims to prove. Spec 0031 built a
room that hands you a number and has the desk refuse it, and Jake walked up to
the desk and took the argument apart in one sentence: if what I am holding is a
number, and there is no limit to how many rooms there are, I should be able to
get a room.

He is right. The desk should say yes. The room is what happens next.

## What 0031 got wrong

Its own comment on the alphabet gives it away: *a row is a number written out
and the thing you build by changing them is another number.*

If the thing you build is a number then the hotel has a room for it. The rooms
are the counting numbers, 13608155081728 is a counting number, and the move
spec 0030 already built puts somebody there without anybody leaving. The
refusal was false and the player who tried it found out faster than the author
who wrote it.

The diagonal does not work on numbers. **A number is a finite string of
characters, and there are only countably many of those, so a list of all of
them has every one somewhere in it.** The thing that cannot be listed has to
never end. That is the whole of what 0031 missed, and everything below follows
from it.

Two faults carry it:

- **The entries end.** `COLUMNS` is 24 and the row stops.
- **What you carry ends.** `Carried::walked_to` grows the string to however
  many rows you have walked past, the wall is 14 rows, and you reach the desk
  holding fourteen digits.

## The other thing it got wrong, which is the better half

Cantor's argument does not say *this guest cannot have a room*. One guest can
always have a room, by the move the hotel has been making since spec 0030.
What it says is that **no list is ever complete**, and it says it by handing
you a machine that produces somebody the list has missed, however the list was
written.

So the desk says yes, and it does not help. That is a better room than a
refusal, because it uses the hotel's own trick rather than contradicting it,
and because the player does the disproving rather than being told the answer.

## Behavior

### The entries do not end

That is the whole requirement and there is nothing else in it.

~~The second column stops being digits and `MARKS` becomes the twenty six
letters, so that a room has a number and a guest has a name and the wall shows
they are not the same kind of thing.~~

Struck 2026-10-21, hours after it was written. Jake said he was dubious about
names, because a list of either letters or numbers can be infinite. He is
right twice over: a finite string of letters is as listable as a finite string
of digits, and an endless string of digits is as unlistable as an endless
string of letters. **Countability turns on whether the entry ends, and never
on the alphabet it is written in.** The letters were a way of stopping a
player saying "that is a number, give it a room", which is a presentational
trick wearing a proof's clothes.

**So the digits stay**, and they are the better version anyway. An endless
string of digits is a real number, which makes this room Cantor's 1891
argument and not a retelling of it, and it meets the objection rather than
preventing it: yes, it is a number, and no, it is not a counting number, and
the rooms are the counting numbers.

What has to change is only that the entries run on. `mark` already answers for
any row at any column, so **the book is endless already and nothing about the
data changes.** What changes is the drawing: the entries run to the edge of
the page and are cut there mid-number, with no right margin. An entry that
ends tidily is an entry with a last digit, and that is the fault this spec
exists to fix.

The first column stays the room's number, borrowed from the corridor's plaque,
and the difference between the two columns is now the only thing on the wall
that matters: **the left one ends and the right one does not.**

### What you carry is a prefix

You still walk the diagonal and still take a digit from each row as you pass
it. What you hold is not a finished thing and never will be. It is as much of
an entry as you have walked, and the guarantee that comes with it: it differs
from row n at place n, for every n you have been past.

What is drawn in your hand runs off the same way the wall's entries do.

### The desk houses them

You take it to the desk and **the desk says yes.** One guest, one shuffle, the
same move as spec 0030's `Arrival::One`: everybody up a room, room 1 free, the
new name in it.

Nothing about that is grudging and nothing about it is a trick. It is the
correct answer.

### And then you are a row

Housing them writes them onto the register, which is what a register is for.
Your name is room 1 now. Every other guest moved up a room, so **every row of
the page moves down one**, and the bottom one falls off the window the way the
corridor's furthest door does.

The wall does not grow and does not slide along. It gains a line at the top.
That keeps 0031's answer to the fault Jake found on 2026-10-21, which was a
page that moved with the reader.

Then you walk the diagonal again. The new one differs from row 1, which is
you, because that is what the diagonal does to whatever is written. You can go
back to the desk with it. The desk will say yes again.

**That loop is the room.** The register is never finished, the desk is never
beaten, and nobody ever says a word about ℵ₁.

### What the desk says

Three lines, and they are drafts:

- Arriving with an entry nobody has: `Nobody in this hotel is that. Room 1,
  and everybody moves up. [COPY - Jake]`
- Once they are in: `Room 1 is theirs. The register has them now. [COPY -
  Jake]`
- And the one that does the work, after the second time: `That is twice. It
  will be twice again. [COPY - Jake]`

## What this must not become

**A refusal.** Struck from 0031 and struck on purpose. See above.

**A counter.** "4 housed who were on no line" is a scoreboard, and the thing
that matters is not how many times it happened but that it cannot stop
happening.

**An explanation.** Unchanged from 0031. No panel states the theorem.

**A page that moves with you.** Unchanged from 0031, which already struck it
once.

## What building it found

**The walk was over before it started.** The page begins at the room's west
end and the way in is at its east, so `walked` answered fourteen at the
doorway: what spec 0031 called a walk was handed over in the first frame you
were in the room, without a step. Nothing is taken now until you have stood at
the head of the page, which is what reading a book from the beginning means,
and it makes the length of that hall worth having. You go in empty, walk down
to the start of the book, and read your way back toward the door carrying it.

**The entries stopped five metres short of the wall.** Twenty four columns of
0.19 on a nine metre room, with bare plaster after them. They run into the end
wall now and the last cell straddles its face, so it is cut rather than
finished.

**Running them to the wall made the wall unreadable.** Forty four columns of
digits at one brightness is a texture rather than a page, and making each mark
a full cube at the same time put its digit on two faces at once. Jake looked at
it and said it is unintelligible, and he was looking at six hundred marks with
no line in them anywhere.

Three things fixed it and none of them is a compromise on the entries running
on. The marks are a third of a cell deep rather than a whole one, which keeps
the sliver of side digit he liked about the accident and leaves the front of
each one as what you see. The part of a row the diagonal can reach is lit and
the rest falls away to about a fifth, which tells the eye where to start and
still never goes out, because a mark that fades to nothing is a page that ends
after all. And the marks touch along a row, so each row is one dark band and
the only gap on the wall is the gap between lines.

**The diagonal had to stop reading `mark`.** Once the desk writes somebody in,
row 1 is them, and a diagonal still consulting the original book would walk
straight past the one guest it has to differ from. It reads the book.

**And somebody written in needed a seed rather than a row.** Their row moves
down every time another is written in above them, so an entry keyed on its row
number would change under them. It is keyed on what they walked.

## Acceptance criteria

- The part of a row that is read is lit and the run-on falls away without
  going out. — `register::tests::the_run_on_falls_away_without_going_out`
- No entry in the register ever ends: the book answers for any row at any
  column, however far along, and nothing is drawn with a last
  digit. — `register::tests::no_entry_ever_ends`
- The left column ends and the right one does not, which is the two columns'
  whole difference and the only thing the alphabet was ever standing in
  for. — `register::tests::one_column_ends_and_the_other_does_not`
- What you carry differs from every row you have walked past, at the place you
  passed it. — `register::tests::what_you_hold_is_on_no_row`
- The desk houses it, by the same one-guest move spec 0030
  built. — `tests::the_desk_houses_the_diagonal`
- Housing it puts them on the register as room 1 and moves every other row down
  one. — `register::tests::housing_them_writes_them_in`
- And the diagonal walked after that differs from them too, which is why
  housing them did not
  help. — `register::tests::the_next_diagonal_differs_from_the_last_guest`
- Walking it twice gives two different guests, so the loop is a loop and not
  the same entry over
  again. — `register::tests::the_second_walk_is_a_different_guest`
- The page still does not move with the
  reader. — `register::tests::the_page_is_fixed_to_the_wall`
- And nothing is taken until you have stood at the head of it, which is the
  fault found while building
  this. — `register::tests::the_walk_starts_at_the_head_of_the_page`

### Verified by hand

- The right column reads as something that has not finished. This is the one
  the whole spec now rests on, and it is the one no test can see.
- The entries being cut at the page's edge read as continuing rather than as
  text that failed to draw. This building has made that mistake once already
  with a blank plaque, so it is worth standing there and looking.
- Going back a second time lands as the point rather than as a repeat.

## Out of scope

**Drawing anybody.** Still, and for the third spec running.

**The prime power arrival.** Still spec 0030's, still unwritten.

**Any claim about how many names there are.** The room shows that the list
cannot be finished. It does not name what it is a list of, and `aleph` next
door does not either.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
