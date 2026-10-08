# 0005 A room you want to be in

**Status:** implemented
**Date:** 2026-10-04

## Goal

Make the room somewhere worth walking into, rather than a corridor of boxes
that happens to have games in it.

## Why

Jake said the room was drab four times, and each answer was better than the one
before and still answered the wrong question.

First the surfaces were tinted apart: floor cool, walls warm, plinths lighter
than both. They had all been one hue between 0.12 and 0.24 and a plinth was the
wall behind it exactly. That helped and the room stayed grey, because the light
was white over a neutral grey fill, and under a neutral light a tinted surface
comes back the colour it started as. So the sun went warm and the fill cool.

Then the cabinets got bands in their games' own colours, and bands that lit
nothing are coloured bars, so every band got a lamp. Then the names went on the
marquees, which is most of what tells you to walk up to a cabinet.

All of that is real and none of it was the complaint. "Flat colours aren't
inviting" is about surfaces with nothing on them, and a box is a box at any
hue. Every face in the room was one colour across the whole of it, which is the
thing an eye has nothing to land on.

## Behavior

**The floor is carpet, woven here.** A dark ground with bright shapes thrown
over it, which is the pattern that exists in a real arcade so the carpet does
not show what gets spilled on it. Procedural and deterministic, the way cairn
makes its wood grain and cascada draws its pips: nothing is loaded.

It tiles. The engine's sampler repeats, so the floor quad's own u and v run past
one rather than a single tile being stretched over thirty units, and the tile
wraps so a floor of them has no seams.

**It is matte.** Shininess is the power the half vector is raised to, so a small
number spreads a highlight over everything. The carpet went down at 4.0 and its
dark ground came back pale grey, because every texel of it was catching the sun.

**The room has a ceiling.** It opened onto nothing, and a corridor with no lid
is a corridor you are standing outside of. Dark, because that is where a dim
room's light does not reach, and it takes the colour the bands throw up at it.

**The carpet's ground is darker than anything standing on it**, so what stands
on it reads as standing on it. The floor's darkness used to live in a constant;
it lives in the weave now.

**The sun's map covers the building and not just the hall.** Outside that map
the engine calls a fragment lit and lets nothing cast, which is the forgiving
answer for a world bigger than its map and the wrong one for a roofed room.
Fitted to the hall, the map had three rooms outside it. The cellar and the
baths are under the hall and under the box. The space behind the wall runs
forty-four by fifty-two past the end of it. All three were taking full sun
through solid ground, and in the room that is meant to be lit by nothing but
its own six lamps it showed as a warm wedge on the back of the hall's end
wall, which is the first thing you see on turning round in there.

The building is nine times the hall across, so a texel of the map goes from
eight millimetres to thirty-six, and the old comment on these bounds argued
hard for keeping them tight. Nothing is lost by it. The hall and the nook are
roofed and the sun reaches neither: measured with the sun at full and at
nought, from three cameras, both rooms came back the same to three places.
The garden is the one room the sun is for, it is roofless, and thirty-six
millimetres is finer than the edge of anything in it.

## What it asks of blitzkit

Nothing. `TextureData::from_pixels` builds the tile and the sampler already
repeats.

## Acceptance criteria

- The carpet has a pattern on it and is still mostly dark ground. — `carpet::tests::it_has_a_pattern_and_is_still_dark`
- It wraps, so a floor of them has no seams. — `carpet::tests::it_wraps_rather_than_seaming`
- The same tile is woven every time. — `carpet::tests::it_is_woven_the_same_every_time`
- And it is square and opaque. — `carpet::tests::it_is_square_and_opaque`
- The floor repeats the carpet rather than stretching one tile over it. — `room::tests::the_floor_tiles_its_carpet`
- The room's surfaces are told apart, and the carpet's ground is the darkest. — `aim::tests::the_room_is_not_one_colour`
- The light itself has a colour, warm above and cool below. — `aim::tests::the_light_is_not_neutral`
- A band burns in its own game's colour rather than white. — `aim::tests::a_band_burns_in_its_own_colour`
- The sun's map covers every room that has a roof on it. — `room::tests::the_suns_map_covers_every_roofed_room`

### Verified by hand

- Walking in and wanting to stay.

## Out of scope

Anything on the walls or the cabinets, which are still one colour each. A
forest, which was the first idea and would have put the room in competition
with thirteen screenshots that each have their own palette. Carpet that differs
room to room, since there is one room.
