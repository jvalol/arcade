# 0017 A door at the far end

**Status:** implemented
**Date:** 2026-10-12, the landing handed to spec 0018 the same day

## Goal

Spec 0015 built a solar system you can look at by telling one lie about it:
the bodies are drawn at one scale and their distances at another, 306 times
apart. The lie is what makes it a model rather than a diagram, and it is
written down and admitted to.

The true proportions are a different thing to show, and they cannot share a
room with the model, so they get their own. This spec is the way through: a
door in the far wall of the room spec 0011 hid.

## Behavior

### Where it is

In the north wall, on the room's own axis, which is `behind::at(reaches).x`.
That is dead opposite the sun and 79.5 metres past it.

Those two numbers are the whole of the staging and neither was chosen.

- From the way in, the door is 177 metres off. The camera's far plane is a
  hundred, so it is not there.
- From the sun it is 79.5 metres off, so it is.

You come in at the cold end, you walk the planets in toward the light, and the
way on appears only once you have reached it. Nothing hides the door and
nothing announces it. The room is simply too big to see across, which is the
one thing spec 0011 built it to be.

### What it is

An opening 1.5 wide and 2.2 high, with a frame round it standing 0.12 proud of
the wall on the room's side and sunk 0.04 into it, so no face of the frame
shares a plane with the wall's own. Two faces in one plane fight and this
building has lost that fight five times; a face buried in another solid is
hidden and is fine.

Four members and not three. A sill closes the bottom, laid on the floor rather
than sunk into it, three centimetres high: at the frame's own 0.16 it would be
something to trip on in a doorway, and left out the door is three sides of a
rectangle from every distance you look at it from.

1.5 because that is what `behind::WAY_WIDE` already is, and that number was
paid for: at 1.15 the walk through the way in wedged half way, because a sweep
with an eighth of a unit either side of a body catches on a shoulder.

The frame glows. Colour is an unclamped multiplier in this engine, so the
frame carries the lamps' own colour at 2.4 and gives off more than it is
given. It is not a light: it lights nothing and casts no shadow, which is what
`Instance::glows` means in the engine, and it costs four drawn boxes and no
light slot.

**A light was hung in the opening first and it did nothing.** Not a matter of
intensity: at 0.85 over a range of 7.5 the floor two metres away stayed black,
and at 3.0 over 14 it stayed exactly as black. Instrumenting the list showed
the lamp surviving the cull and ranking first of the two that reach the shader
at that end of the room, so it was being drawn with and giving nothing.

The reason is the palette. Every surface in this room is between 0.018 and
0.045, and a hundredth of white is black on a screen whatever is shining on
it. What lights the columns in here is the sun, which throws 83 metres against
a ceiling lamp's 9.5; the grid lamps light almost nothing and the room was
built that way on purpose. So the door carries its own light in its colour,
which is the one thing that works in a room this dark.

The lamps' colour and not a warmer one. Everything in this room is the same
cold blue-white, hung on the same regular grid, and the door was put in by
whoever hung the lamps. A warm door would say somewhere else is through there,
and somewhere else through there is darker and colder than this, not warmer.

### What is beyond it

Spec 0018, which is where this spec stops.

It was a landing here, 24 by 24, which was this spec saying where it ended.
Spec 0018 took it out and ran its own floor from this threshold to past
Neptune. Two floors beyond one door would have been two accounts of one thing,
and they would have shared a top plane over the whole 24 by 24, which is the
fault this building has lost to five times.

The room's lid stops at the north wall, so through the door is open above,
which is what a ten metre sun needs.

### The seam under your feet

The floor beyond laps three metres back under the room's, the way the room's
own floor laps under the hall's. Two slabs at one height that meet edge to edge
put a vertical face at the surface you are walking on, and a body resting on a
surface counts as touching it: that is what stopped the walk through the way in
four centimetres short of the seam. Spec 0018 owns that lap now and
`beyond::tests::the_floor_laps_under_the_room` is where it is checked.

## Acceptance criteria

- The far wall is a wall everywhere except the opening. — `behind::tests::the_far_wall_is_solid_either_side_of_the_door`
- You can walk from the sun through the door and out the other side. — `behind::tests::the_door_can_be_walked_through`
- And back in again, which is not the same walk. — `behind::tests::the_landing_leads_back_into_the_room`
- The door is out of sight from the way in and in sight from the sun. — `behind::tests::the_sun_is_what_shows_you_the_door`
- Nothing stands in the way between the sun and the door. — `behind::tests::the_way_to_the_door_is_clear_of_columns`
- The frame glows rather than lighting. — `behind::tests::the_frame_gives_off_light_and_casts_none`
- The sill is nothing to trip on, measured as the same walk over open floor
  and through the door. — `behind::tests::the_sill_costs_nothing_to_cross`

### Verified by hand

- The door reads as a door from the sun. — run the arcade, find the way in,
  walk to the sun, look north. Confirmed 2026-10-12: a lit rectangle at the end
  of the room with the columns running away either side of it.
- The frame is not two surfaces in one plane with the wall. — run the arcade,
  stand at the door and move; the frame does not speckle.

## Out of scope

What is beyond the door, which is spec 0018. Any door in any other wall of
this room. Opening and closing: it is an opening, not a thing on hinges.
