# 0063 The rooms that ask before they show

**Status:** implemented
**Date:** 2026-11-10

## Goal

Take the hotel, the register and the imaginarium out of the building, and
keep every line of them compiling and tested under `src/parked/` so the work
is there when there is a way to make it land.

## Why

**Jake, 2026-11-10, having played it.** First: the hotel is arguably the
least interesting room or feature in the building. Then, after spec 0062 cut
it down to its desk: the hotel and the imaginarium aren't worth throwing
away, the code and logic are good, the composition in capriccio doesn't
land, and until there's a way to make them feel right for a user, keep them
out.

**They share one property and nothing else in the building has it.** Every
room here that works puts its idea in the world, where you meet it by doing
something ordinary. You walk at the horizon and never arrive. You look in
the mirror and you are there. You zoom the Mandelbrot with your own hands.
You climb the bridge and arrive at the top of it.

These two put the idea in your head first and then show you a room. Forty
doors are forty doors unless you are already holding "and there are
infinitely many". A staircase is a staircase unless you are already holding
"and three of these disagree about which way is down". No amount of
rebuilding the room fixes that, because the room is not where the idea is.

Spec 0062 is the evidence that rebuilding does not fix it. It was the fourth
round on one complaint, it was a good cut, and an hour after it shipped the
answer was still no.

## What goes out of the building

- **The hotel.** The corridor, the lobby, the desk, the arrivals and both
  waves. Spec 0029, 0030, 0034.
- **The long hall**, spec 0035's second corridor.
- **The register**, spec 0031 and 0032. Jake's call, and it is the right
  one: its way in is the hotel's door and its ending is the hotel's desk, so
  it cannot stand on its own, and it is the same mathematics in a labelled
  room.
- **The imaginarium**, specs 0037, 0038, 0047 and 0048, and the timber
  dog-leg in the nook that goes up to it.
- **The watchman's riddle**, spec 0058, and with it the typed line of spec
  0059, which has no other use.

## Parked, which here means still compiling and still tested

Everything above moves to `src/parked/`, keeps its arithmetic tests, and is
reached by nothing the building draws or collides. The module tree says the
state out loud, the way `games/aleph/Cargo.toml.not-yet` does for the crate
this hotel came from.

**Because history is not keeping it.** A deleted room is recoverable and
rotten: nothing compiles it, so the first engine change that touches its
corner of the API breaks it silently and nobody finds out until somebody
tries to bring it back. Parked and tested, it still works when it is wanted.

`tour.rs` is the reason this matters today. Spec 0049's route planner walks
inside a rotated gravity frame, and the whole of that machinery is the
imaginarium's: `walked_in_frame`, `out_of_frame`, `box_out_of_frame`,
`in_frame`. Deleting the room deletes a walk that nothing else in this
project knows how to do.

## The riddle, and why it goes rather than moving

The watchman asks what the building looks like from out here and accepts
"a light". That is not a riddle, it is an observation prompt: the answer
comes from turning round, not from having walked anything. Spec 0058 sets
its own test as "an answer you can only have from having walked the
building", and it fails that test. The gating is done by the bridge, which
only comes up once every station has been given, so the riddle is a second
lock on a door that is already locked.

The cut makes it worse rather than better. The riddle drew on what the
building had in it, and three rooms are leaving. There is nothing left to
ask about that a walker would know and a looker would not.

So the watchman keeps the threshold and lets you through. The gate swings as
you come up the last of the bridge, which is what a man keeping a gate does.

**What it costs is the typed line.** Spec 0059 is one of the few places in
this building where you put something in rather than look at something, and
it has no other use. It parks with the rest.

## The lantern stays

Spec 0057 lit one lantern on the imaginarium's roof so that the building
read as something rather than as a dark block from the top of the bridge,
and that is still the only thing that makes that view work. The imaginarium
is going and the hall's roof it stood on is not, so the lantern stands on
the hall's roof instead and is the building's rather than that room's.

It is a lamp standing on a roof and not a hole into a lit room, which is
what spec 0057 settled after getting it the other way round.

## What the trail becomes

Five stations instead of eight. `WalkedTheDiagonal` and `HousedTheCarried`
were the register and the desk; `WorkedAHatch` was the imaginarium. What is
left is riding to the sun, going inside the tesseract, opening the bookcase,
getting behind the hall's wall, and walking out into the field.

That makes the bridge easier to reach, which is a real change and is
probably the right one: eight was a checklist of a length nobody had
measured against how long the building takes to walk.

## Acceptance criteria

- Nothing the building collides with, draws, shows the sight or lists as a floor, a lid or a run comes from a parked room. — `room::tests::nothing_of_the_building_is_parked`
- The gallery has one doorway and its west wall is whole, swept along rather than checked at its ends. — `room::tests::the_gallery_has_one_doorway_and_one_whole_wall`
- The roof carries one lantern, standing on a lid, capped, and with no lamp in it. — `room::tests::the_roof_carries_one_lantern`
- The watchman opens the gate as you come up to it, keeps it shut while the bridge is not there, and does not open it from out of reach. — `gate::tests::the_watchman_lets_you_through`
- Every wall the sky can see is still dressed, including the nook's new south wall. — `room::tests::nothing_outside_is_plastered_indoors`, `room::tests::the_strip_over_a_shorter_room_is_dressed`
- The trail has five stations and `every` holds all of them. — `trail::tests::every_one_of_them_is_in_every`
- Every parked room still answers for itself, walking its own boxes and the building's shared walls. — `parked::aleph::tests::walked_from_the_lobby_into_the_corridor`, `parked::aloft::tests::you_can_walk_up_it`, `parked::register::tests::the_walk_starts_at_the_head_of_the_page`
- The planner still crosses a portal, handed the room directly. — `tour::tests::a_portal_edge_changes_which_way_is_down`
- The rooms that are left still look like themselves. — `tests::the_hall_is_drawn` and the thirteen beside it

### Verified by hand

- The building still reads as something from the top of the bridge. — climb
  it and look back east.

## Out of scope

**Finding the composition that makes them land.** That is the whole point of
parking rather than deleting, and it is not this spec. The one idea on the
table is that both rooms already have exactly one second in them that works
with no briefing at all: falling through the imaginarium's hatch and coming
up standing on a wall, and the desk saying yes to a guest it has just been
proved to have no room for. Keep the seconds, lose the architecture.

**Spec 0062.** It cut the hotel down to its desk and shipped, and this
supersedes it within the hour. It stays where it is rather than being
reverted out of the numbering, because it happened.
