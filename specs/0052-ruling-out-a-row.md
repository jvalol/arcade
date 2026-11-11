# 0052 Ruling out a row

**Status:** implemented
**Date:** 2026-11-03

## Goal

The register shows you that what you are carrying is on no line of the book,
rather than telling you at the end.

## Why it belongs here

Jake walked it and said the room does not land: what it means does not come
across.

**The proof was compiled out of the building.** `Carried::differs_from` is
the argument, it is tested, and it was `#[cfg(test)]`: nothing in the game
could call it. So the room showed the construction, with marks lighting
behind the walk, and the desk asserted the conclusion at the end of it, and
nothing in between let you check anything.

**And Cantor's argument is an absence.** What you hold is on no line, and an
absence cannot be looked at. Next door Hilbert lands because it is a
presence: the corridor has no end, everybody visibly moves up, and you watch
it happen. This room had nothing to watch.

## Behavior

**A row the walk has answered is ruled out, with a line drawn through it.**
As you pass row three, what you carry already differs from row three, so the
wall says so. By the end of the page every row you have passed is struck and
the rows run on into the end wall, which spec 0032 put there so that none of
them finishes. What you are looking at is a list where every line you have
checked is ruled out and the lines keep going.

**It asks the proof, not the walk.** `differs_from` answers false for a row
the walk has not reached, so the drawing asks it rather than asking whether
`row < reach`. The two are the same answer by the theorem, and the theorem
is the thing being shown.

**The rule stands proud of the marks**, by its own thickness, rather than
sitting at their depth. Two faces in one plane is this building's oldest
fault.

**It is lit a little past one.** The first version was 0.95 and unlit, on
the reasoning that ink on a page is not a thing switched on. The room is
dark and the marks carry their own brightness, so an unlit box standing in
front of them came out as a shadow band across the rows. Well under
`REGISTER_DIAGONAL`'s 1.9, which is a figure a cell wide rather than a line
a metre long.

## Acceptance criteria

- ~~The wall rules out what the walk has answered and nothing further.~~
  Struck 2026-11-10: spec 0063 parked this room. The arithmetic under it is
  unchanged and still runs, in `parked::register`; what went is the picture
  of the wall, which needed the room to be in the building to be taken.

### Verified by hand

- **In the window, at the size a player sees.** Rows one to seven struck,
  eight to fourteen not, the line between them plain. The 320 by 240
  reference is for catching change and is too small to judge this by, which
  is why it was not judged by it.
- **Measured against the same frame with the rule turned off.** 4,537
  pixels brighter and 44 darker, which is what said the rule was drawing
  while it still looked like a shadow at reference size.
- **Two faults the pose found.** `yaw` of PI photographed the far wall of
  an empty room, since nought already looks down `-z` here. And standing
  exactly on the boundary of cell seven walks six rows, because the
  multiply comes back a hair under and `walked` truncates: the pose stands
  mid-cell, and the test asserts the walk happened, which is what caught
  it.

## Out of scope

**What the room says.** `BOOK_SAYS` gives the mechanic and `DESK_TAKES`
gives the verdict, and nothing between them says what the walk gave you or
why it cannot be in the book. That is copy and it is Jake's.

**Striking the rows from the desk's side.** The book is written into when a
guest is housed, and what a rule would mean on a row that has moved down one
is a different question.
