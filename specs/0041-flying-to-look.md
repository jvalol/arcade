# 0041 Flying to look

**Status:** implemented
**Date:** 2026-10-26

## Goal

A way to get to where a fault can be seen, which in this building is most of
the work of fixing one.

## Why it belongs here

`WHERE` solved half of this already. A fault here is reported as a picture,
and a picture has to be matched to a camera before anything can be measured or
shot again: six guesses at a pose found nothing on 2026-10-08 and one line of
`WHERE` found it at once.

This is the other half, which is getting to the place in the first place. Four
things got an outside in one sitting on 2026-10-25 and 26, and every one of
them was found by writing a test that poses a camera at a guessed point,
rendering it, and looking at the file. Four or five rounds apiece, each round a
build. The roof of the imaginarium had never been looked at by anybody, because
there is nowhere to stand that can see it.

## Behavior

`FLY=1` on the command line. Flight is the walk with the three things taken
out that make it a walk: gravity, the body's radius, and the floor.

- The arrows are still the arrows. Forward is where you are **looking** rather
  than where you are facing, so the pitch is what flies you up and down. There
  is no other way up, and a flight that only goes along the ground is a walk
  that ignores walls.
- `FLIES` times the walking pace, which is four. At `SPEED` that is seventeen
  metres a second, which the building needs: three hundred and twenty metres
  of field across and two kilometres of corridor along.
- No footfalls, because you are not walking.
- No climb budget spent. A flight to the roof would otherwise spend one meant
  for stairs and leave you unable to climb one for four seconds after landing.
- `walk` is not called at all, so nothing it does happens: no step budget, no
  slide along a wall, no fall.

**R puts you back at the door.** Flight can leave you somewhere with
nothing drawn at all: inside something, past everything, or out where no
room's gate is true, and the screen is black whichever way you turn, so
there is no landmark left to steer by and no way back. Jake found one the
first time he flew. The reset drops what you are inside as well as where
you are, because the tesseract and the ride out past the door each hold you
in a state of their own and a reset that left you in one would put you at
the door still riding.

Only while flying. In the game it is a key that teleports you, which is not
a thing this building does.

**The rooms will pop, and that is the price.** Seven things in here are
decided by which room the building thinks you are standing in, and every one
assumes you are in exactly one: `field::in_view` three times, `behind::inside`,
`twine::inside`, `garden::under_the_sky` and `noise::room_at` three times. Fly
out through a wall and the room behind you stops being drawn. Fly over the
hall's end wall and spec 0015's solar system appears, which is the gate that
keeps it a secret. Rewriting all seven is the same question as making spec
0040's field a plane rather than a half plane, and it is worth doing for its
own reasons rather than for this.

The one gate flight opens is the field's. A tool for looking at the outside of
a building that draws no ground and no sky the moment you leave the grass is
not a tool.

## Acceptance criteria

- Flight goes its whole distance with the building in the way, and the same
  journey walked is stopped. — `tests::flying_goes_through_what_walking_does_not`
- And it goes where you are looking, up and down
  included. — `tests::flying_follows_the_pitch`
- R puts you back where the game opens, from anywhere, holding
  nothing. — `tests::the_reset_puts_you_back_at_the_door`
- And does nothing at all on
  foot. — `tests::r_does_nothing_when_you_are_walking`

### Verified by hand

- `FLY=1 cargo run --release`, then fly into the dark until nothing draws
  and press R.
- `FLY=1 cargo run --release`, then look down at the building from forty
  metres. The field and the sky are drawn, the imaginarium's four plastered
  faces read, and the two lanterns and four bays stand on it.
- The same flight shows what flight is for: the imaginarium's roof is the
  pale stone of the room's ceiling, because until now nothing could be above
  it, and from up there it is the largest surface the building has.

## Out of scope

**A god mode in the game.** This is a tool, off unless asked for, costing a
string lookup a frame.

**Fixing what flight shows you.** It turns "the building has an outside" from
three walls into a list, and the list is not this spec's.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
