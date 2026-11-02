# 0050 A cube that solves itself

**Status:** draft
**Date:** 2026-11-02

## Goal

A seventh bench in the nook, holding a three by three turning cube. You
scramble it with the arrows, step back, and it undoes itself while you
stand there. It never tells you that you have solved it, because the
study is the one room that is not trying to sell you a game.

Named for the two-phase algorithm rather than for the puzzle. Jake's
call, 2026-11-02.

## Why this earns a bench

Every object on that shelf pays its way by showing the engine doing one
thing, and `globe.rs` says so in as many words: it is the one with no
solver in it, and it defends itself on wrapping a picture on a surface
and lighting it instead. The gyroscope is a rigid body. The cradle is
constraints.

**A cube is the one that is all hierarchy.** Turning a face is nine
cubies rotating about an axis that belongs to neither them nor to the
whole, and then belonging to a different nine. Nothing else in this
building does that, and it is an exercise rather than a decoration.

It is also the first object here that demonstrates an algorithm instead
of a solver, which is why the name is the algorithm's.

## What it is and is not

**Not a game.** Spec 0006's nook exists for the things that are worked
rather than watched and have nothing to win. A cube that scores you is a
cabinet's job, out in the hall, by the rule `cellar.rs` states for
poolhall. So: no score, no congratulation, nothing kept between visits.

**There is no visible progress, and that is the point.** Phase one ends
when the cube is in the subgroup where four of the six faces need only
half turns, and that state looks exactly like a scrambled cube. So the
watching is twenty-odd seconds of apparent randomness and then it is
suddenly done. Every other object on that shelf is the same: the
gyroscope holds its axis and nothing happens, the globe runs down.

**The structure shows in the moves rather than in the cube.** Phase two
may only half turn four of the faces, so the character of the moves
changes partway through: quarter turns anywhere, then quarter turns on
two faces and half turns on the rest. Somebody who watches it twice
notices. That is the bench's real subject and the reason the name is not
a dodge.

**Pace is a constant, not a property.** Twenty moves at a second a move
is twenty seconds, which is longer than the globe takes to run down. An
early draft of this argued that an optimal solve is too quick to watch
and that a long layer-by-layer solve would be the better object. That
was wrong, and only the legibility half of it survived.

## Behaviour

**The cube.** Twenty-six cubies and no core. Each face's stickers stand
clear of the body rather than lying in it, per spec 0014: two coplanar
faces both on show is the fault this building has hit seven times.

**Colours: glowing neon against black and white.** Jake's, 2026-11-02.
The hall outside is already neon over a dark carpet, so the cube is the
study's one loud object rather than a stranger in the building, and a
face that glows is a face that reads in a room lit by four sconces.

It also steps clear of the only thing a United States court has held
protectable in this shape, which is a black body carrying six specific
Pantones. Neon on black and white is not that, and the distance is a
side effect of a choice made for the room rather than a choice made for
a lawyer.

Two things to watch when it is built. Colour here is an unclamped
multiplier and the hall's neon is 3.2, so a glowing face needs no light
on it and will throw its own onto the bench. And six faces that all glow
is a lamp rather than a cube: the black and the white are what give the
neon something to be bright against, so they want to be most of it.

**A turn** is a quarter turn of one face over a fixed time, and nothing
else on the bench moves while one is running. Turns queue rather than
overlap.

**The arrows work it**, the way the row's other toys are worked, through
`wanted_by` and the `Spin` the gallery's shapes already use. Which
arrows, and what picks the face, is for the implementation.

**It shares the globe's bench.** Jake's, 2026-11-02, and the room leaves
no other option: the nook's row is full. Its usable run is 11.38 metres,
six benches take 10.80, and a seventh of the narrowest size there is
takes 12.40. Growing the nook needs the building to grow, because the
garden's door sits in the same wall 0.5 north of the nook's end with
only 0.25 of wall north of it, so `END` and `NOOK_SPAN` have to move
together and four rooms' pictures move with them. Tightening the gaps
instead takes 0.2 off every space in a row that is already too narrow to
stand in, which is the thing this building does not do.

The globe's bench goes from 0.9 to about 1.3, which puts the row at
11.20 and leaves 0.18, the same clearance it has at the doorway. The
cube stands in the 0.4 beside the globe.

**What that costs is a bench that holds two toys, which this building
has never had.** `Benched` carries one name, `looking_at` casts at one
box per bench and hands back one `Seen::Bench`, and every line under the
sight is chosen by that name. So pointing at the cube would say globe
unless a bench can hold two things and the sight can tell which half you
are looking at. That is the work, and it is more than the geometry.

**Solving is Kociemba's two phase.** Phase one reduces the cube to the
subgroup generated by quarter turns of two opposite faces and half turns
of the other four. Phase two solves inside it. Each phase is an IDA*
search over coordinates with precomputed pruning tables.

**Near optimal, not optimal.** Every position is solvable in twenty
moves or fewer in the half turn metric, which was proved in 2010, but
two phase does not promise that number on the first solution it finds.
It is the right trade for a bench and the wrong claim to make in a sign.

**The tables are generated on first approach, not shipped.** Jake's,
2026-11-02, on the measurement. They come to 6,460,365 bytes and build
from cold in 139 ms native, against an arcade wasm of 7.8 MB inside a
`web/` holding 40: shipping them is most of a doubling of what somebody
downloads to play it. Expect two to four times the 139 ms in wasm, which
is a third of a second against a game that already takes twenty to come
up.

The deciding reason is neither of those. Shipped tables are a second
copy of the truth. They are derived from `Cube`, so the day that model
changes they are stale, the solver returns moves that do not solve, and
nothing says so. Generated tables cannot go stale.

On first approach rather than at room load, so nobody pays the third of
a second unless they walk up to the bench. Off the frame's thread if the
stall shows; measure before assuming it does.

**When it is already solved** there is nothing to undo, so working it
scrambles it instead.

## Numbers to measure rather than assume

- how much memory the pruning tables take
- how long they take to generate from cold
- how many moves a typical solve comes out at, over a few hundred
  scrambles
- what the whole thing adds to the wasm

## Acceptance criteria

- A scramble followed by a solve leaves every cubie home. — `kociemba::tests::a_scramble_is_undone`
- Every move the solver emits is a legal quarter or half turn of one face. — `kociemba::tests::nothing_but_turns_comes_out`
- Phase two uses only half turns on the four faces it is allowed to half turn. — `kociemba::tests::the_second_phase_keeps_to_its_subgroup`
- A solved cube is left alone by the solver rather than churned. — `kociemba::tests::a_solved_cube_needs_no_moves`
- A few hundred random scrambles all solve, and none exceeds the search's own bound. — `kociemba::tests::every_scramble_comes_out`
- The bench is in the nook's row and a body can stand where it is worked from. — `room::tests::the_benches_are_reachable`
- No two drawn faces of a cubie share a plane with its neighbour's. — `kociemba::tests::no_two_stickers_fight`

### Verified by hand

- The colours read as six distinct faces under the nook's sconces, which are warm and dim. — run it and stand at the bench.
- The change of move character partway through a solve is noticeable. — watch one solve twice.
- Twenty seconds of it is worth standing through. — watch one and see.

## Out of scope

- Telling you that you have solved it. That is what makes it not a game.
- Any cube but the three by three.
- Guaranteeing the shortest solution. That is a different search and a
  slower one, and nothing on a bench needs it.
- Letting the solver run while you are turning a face. One at a time.

## The name, and what it is avoiding

`RUBIK'S CUBE` is a live United States word mark, registered 1983 and
renewed, so the name stays out of the module, the bench, the sign and
the alt text. The European 3D shape marks have been falling: the
greyscale one went invalid in 2019 and the colour ones on 9 July 2025,
both on the ground that the shape and the colours are functional. The
United States went the other way. In `Rubik's Brand v Flambeau` a
magistrate held in 2021 that the trade dress is not functional, and that
dress is narrow: a black base carrying specific Pantone numbers. Choose
colours that are not those, on a body that is not black, and nothing a
court has protected is in play.

None of the above is legal advice and the 2025 disposition is reported
inconsistently by the commentaries. If it ever matters, read the
judgments.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
