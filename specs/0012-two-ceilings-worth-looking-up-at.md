# 0012 Two ceilings worth looking up at

**Status:** implemented
**Date:** 2026-10-09
**Read against the code:** 2026-10-14, claim by claim. Eight claims and three
criteria, so five claims had nothing under them, and two of those five were
worth holding. Both now have a test.
**Emptied and restored:** the commit that did that reading, 5950890, deleted
all 113 lines of this file while correctly updating spec 0019 and `spa.rs`
beside it. The tests it wrote survived; this text did not, and nothing said so
for six days, because `check-specs` decides whether to read a spec at all from
its `**Status:**` line and an empty file has none. Restored 2026-10-20 from
80b4022, which is the state before the truncation, and the reading written
into it here.

## Goal

The hall's lid and the baths' are each one flat colour over the whole room.
Spec 0005 named that fault about the cellar and fixed it there, with coffers
and beams and a boss where four of them meet. The other two never got the same
treatment, and walking the building with a camera pointed at the ceiling is
what made it obvious: five rooms look up at something and two look up at
nothing.

A ceiling is the one surface in a room nobody spends anything on, which is why
spending anything on it reads.

## Behavior

**The hall gets a suspended grid.** Bars one way and then the other, with a
tile sunk behind every square. That is what is over a real arcade, and the
reason is not only that: the aisle is sixteen units long and three wide, and a
grid of lines running away from you down a corridor is depth you get for
nothing. One flat colour over the same shape is a colour.

The grid is laid to the world and not to each piece of lid. The floor is an L,
the hall end to end and the nook beside the near half of it, and the ceiling is
drawn as one quad per piece. Each piece gridded from its own corner puts two
grids meeting out of step along the join, which is the join you walk through.
Snapped to the world the two are one grid with a wall standing in it.

**The bars hang below the tiles.** A tile flush with the bar under it is two
faces in one plane, which is the fault this building has fought seven times.
A suspended ceiling is built that way regardless: the bars are a lattice and
the tiles drop onto it from above, so the bar's underside is the lowest thing
up there and nothing is level with anything.

**Everything that hangs from the hall's ceiling hangs from the grid.** The
neon's stem and the sign's pendant both ran to the structural lid. With a
grid under it they would come through the tiles, which is right for a service
and wrong for the two things in the room that are meant to be hung. They take
the height of the visible soffit instead, named once and called twice.

**The baths get a border and ribs across it.** Not tile, which was the first
answer and was wrong. Every other surface in that room is tiled and the lid was
the one left in plaster, so tiling it looked obvious: laid, it came back as
noise. You see a ceiling at a glancing angle, the picture compresses to nothing
along the way you are looking, and what is left is a grid beating against the
pixels. The cellar's lid is the one in this building that works and it is built
rather than painted. So is this one now.

The ribs run across the room and not along it. It is eighteen by twelve and you
walk the eighteen, so ribs across are a thing you pass under and ribs along are
two lines going nowhere.

The border mirrors the one laid into the floor and takes its numbers from the
same two constants, so the two cannot drift apart. It starts past the sauna at
the same place the floor's does, because the sauna's roof is a hand's breadth
under the ceiling and a border drawn over it is a border nobody sees. The ribs
stop at it rather than crossing it, so a rib reads as landing on the border and
no two undersides share a plane.

**The ceiling's work is pale, and that is a drawing decision rather than a
decorative one.** The ribs and the border they land on were the dado band's
dark teal, which is right for a band at shoulder height and wrong hanging under
plaster. A rib is a long shallow edge at a glancing angle, which is the worst
case there is for putting a straight line on a grid of pixels: the boundary can
only sit on whole pixels, so it runs flat for five and drops one. Spec 0052's
multisampling shades the single pixel at each drop and cannot move the boundary,
and eight samples a pixel took eight per cent off what four left.

So the contrast goes instead. At the band's colour the jump across a rib edge
was 38 of 255 near and 25 far; at the ceiling's own colour it is 22 and 10. The
staircase is still there and there is not enough difference across it left to
read as one. This is the only fix of the three that was cheap, and it is a
fix for this ceiling rather than for the engine, which still draws shallow
edges the way it always did.

**This building draws no contact shadows**, per blitzkit's spec 0029, and this
ceiling is why. See the note at `set_contact_shadows` in `main.rs`.

Held by a test since 2026-10-14, which is the second of the two worth holding.
The bath house ceiling is thin fins lit nearly edge on, which is the one shape
that march cannot answer, and what it does there is ripple along every beam.
One line turns it back on, and that is the sort of line somebody adds while
wondering why a room looks flat.

**Both borders are mitred, the floor's included.** Four runs each at their full
length meet twice over at every corner, which is two faces in one plane four
times round the room, and that is the fault this building has fought seven
times. The ceiling's border was written mitred, and the floor's border had been
built the same way since spec 0008.

~~the test caught it on its first run~~

Struck 2026-10-14. No test answered to that, and the floor's border had never
had one at all. A claim that names a test which caught something reads as
covered and was not: the mitring held because somebody wrote it that way and
nothing anywhere would have noticed if a later hand had not. Both borders have
a test now. It takes the runs off the room's own geometry rather than naming
them, and checks every pair of them for the overlap that a mitre is the
absence of.

**No vault.** A bath house wants one and this one cannot have it. The baths'
lid is at -0.60 and the garden's floor slab starts at -0.30, so there are
thirty centimetres over the room and a rise of thirty over a span of twelve is
not a vault, it is a flat ceiling with a slight mistake in it. Written down
because it was the first answer and the measurement is what settled it.

## Acceptance criteria

- The hall's grid is laid to the world and not to each piece of the lid. — `room::tests::the_lid_is_one_grid_laid_to_the_world`
- No tile is level with a bar, or over one. — `room::tests::no_ceiling_tile_is_level_with_a_bar`
- Nothing hung from the baths' ceiling runs into anything. — `spa::tests::nothing_hung_from_the_ceiling_runs_into_anything`
- Both borders are mitred, the ceiling's and the floor's, so no two runs of
  either cross at a corner. — `spa::tests::both_borders_are_mitred`
- And nothing turns the contact march back on. — `spa::tests::the_building_draws_no_contact_shadows`

### And the three the reading left alone

Five claims had nothing under them. Two are above. These three stay where they
are, which is a decision rather than an omission:

- **Everything that hangs from the hall's ceiling hangs from the grid.** The
  soffit's height is named once and called twice, so the thing a test would
  check is the thing the code already cannot get wrong twice. It is in
  "verified by hand" below and belongs there.
- **The ceiling's work is pale.** A contrast judgement with its own numbers in
  the text, 22 and 10 against 38 and 25. A test would pin a colour nobody has
  agreed is final.
- **No vault.** A measurement that settled an argument, not a behaviour. The
  thirty centimetres are either there or the garden's floor has moved, and if
  it moves this paragraph is wrong in a way a reader can see.

### Verified by hand

- What hangs in the hall reaches the grid rather than coming through it.
- Looking up in either room is worth doing.
- The grid reads as a ceiling rather than as a pattern painted on one.
- The aisle is longer for having lines running down it.

## Out of scope

**Light panels in the hall's grid.** An arcade is dim and the lamps it has are
in fittings already, per spec 0005. A lit tile is another lamp with no fitting.

**The cellar's ceiling**, which has been right since spec 0007, and the
garden's, which is the sky.
