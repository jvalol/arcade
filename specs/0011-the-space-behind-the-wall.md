# 0011 The space behind the wall

**Status:** implemented
**Date:** 2026-10-08

## Goal

Put something behind the wall you wake with your back to, with no door, no
sign and nothing in the hall that says it is there. You find it by walking into
the wall, or because somebody told you.

Every other room in this building is furnished. This one is the opposite: a
space, very large and very dark, with nothing in it to look at and no reason
given for it being there.

## Behavior

**Behind the near end of the hall.** You wake a unit inside that wall facing
away from it, and the arcade's sign hangs over the far end pulling your eye
down the aisle. The room already points you away from this. That is the whole
reason it goes here and not anywhere else.

**The way in is a hole in the wall that the wall is drawn across.** The near
end wall is two boxes now with a gap between them, and the drawn wall is one
piece over the whole of it. From the hall it is a blank wall. Walk into it off
centre and you go through.

This is deliberately the same mechanic as several faults fixed in this building
in one day: a collider that disagrees with what is drawn. It is a fault
everywhere else and the point here, so it is written down in capitals and the
tests that would otherwise catch it carry an exception that names this spec. If
you are reading this because a test failed, the test is right to ask and the
answer is that this one is on purpose.

Off centre, so walking straight back from where you wake meets solid wall. A
gap in the middle would be found by everybody on their first turn round, and a
gap at the very end would be found by nobody.

**It is low and it is wide.** The ceiling is lower than the hall's and the
floor runs further than the light reaches, in every direction. Low and vast is
uneasy in a way that tall and vast is not: tall and vast is a cathedral, which
is a thing somebody built on purpose and is proud of.

It spreads east, away from the garden. The garden is off the hall's left and
reaches well past the near end, so west is taken; east of the hall is
unbuilt ground.

**Columns**, on a grid, plain and square, floor to ceiling. They are what gives
the size away. An empty floor going off into the dark is a corridor with the
lights off, and the only way to read a distance is to see the same thing at
several of them.

The grid is regular. Everywhere else in this building the rule is that nothing
lines up, because nothing in a furnished room ever does. This is not a
furnished room and the regularity is what makes it feel like nobody is meant to
be here.

**Barely lit, and most of it not at all.** A few lamps far apart, dim and cool
against every other light in the building being warm. There is no fog in this
engine and none is wanted: a point light falls off, so the far end goes black
on its own and the camera's hundred unit far plane is never reached by eye.

The lamps go through the same sorting as every other fitting, nearest first, so
the engine keeps the ones near you and drops the rest. A vast space is the one
room where that is not a compromise.

**No voice.** Every room in this building has one, per spec 0009. This has
none, and the silence on walking through the wall is the loudest thing in here.

It was not silent. `noise::room_at` ended by answering "the hall" for anything
it had not been taught about, and nobody had taught it about this room, so for
sound and for light this was the hall. The hall's hum of cabinets carries
fourteen metres and its voice stands nine from the hole, so the first stretch
of the room with no voice hummed. The sun was switched on in here as well, a
sealed room with no sky, for 11,096 lit pixels of the staged shot.

Found on 2026-10-14 by driving the building rather than by a picture: the
`WHERE` log said Hall while the screen showed these columns. It is its own
room now, `Room::Behind`, and both halves are held by tests.

**It is very much larger than its floor.** `behind::inside` is "past the
hall's reach and above the floors below", and not the drawn footprint, which
was the first answer and was wrong by four orders of magnitude. `SPAN` and
`DEEP` are what is built and lit; spec 0018 put the system in at true scale
and Saturn stands ten and a half kilometres out, so the staged shot of Saturn
came back as the hall. Nothing else in this building is out there, so being
past the wall is the whole question.

**Footsteps are the hall's.** Its floor is at the hall's height, and the one
thing that should not change when you step through is you.

**Its floor laps a long way under the hall's** rather than meeting it at the
wall, and that is not tidiness. Two floors at one height that meet edge to edge
put a vertical face at the surface you are walking on, and a body resting on a
surface counts as touching it: the walk through the gap stopped dead four
centimetres short of the seam. Lapped a good way under, both end faces are deep
inside the other slab and the join is nothing at all.

This is the third time in one day that a vertical face at the walking surface
has stopped a body. The garden jammed where the baths below happened to end,
and that one was fixed by dropping the roof so the tops were no longer level.
Here both are floors and neither can move, so the faces are buried instead.
Written down because the first version of this spec said the opposite, in as
many words, and the walk is what settled it.

**The lap is walked and not drawn.** Drawn as well, it puts this floor's top
and the hall's in one plane over five and a half metres by three, and two
exposed faces in one plane fight. The patch they fight over is the floor you
wake standing on: the carpet came back torn into horizontal bands with the
dark of this room showing between them. Nothing under the hall's floor needs
drawing at all, so the drawn slab stops where the hall's starts and only the
collider runs back.

Two numbers for one slab is the thing this building is otherwise careful not
to do, so they come off one function with the lap as its argument. They are
answering different questions: one is what you can see and one is what you can
stand on.

## Acceptance criteria

- You can walk through the wall where the gap is. — `behind::tests::you_can_walk_through_where_the_gap_is`
- And not where it is not, which is most of it. — `behind::tests::the_rest_of_the_wall_is_a_wall`
- The gap is off centre of where you wake. — `behind::tests::the_way_in_is_not_in_front_of_you`
- The space is closed but for that gap. — `behind::tests::nothing_else_gets_out`
- It does not reach into the garden, the hall or the nook. — `behind::tests::it_keeps_out_of_every_other_room`
- Its floor laps under the hall's rather than meeting it. — `behind::tests::the_floors_lap_rather_than_meet`
- And the lap is walked and not drawn. — `behind::tests::nothing_drawn_lies_in_the_halls_floor`
- No column stands in the way in. — `behind::tests::nothing_stands_in_the_way_in`
- You can get back out. — `behind::tests::you_can_walk_back_out`
- It is its own room, as far out as Saturn. — `noise::tests::the_space_behind_the_wall_is_its_own_room`
- And nothing sounds in it. — `noise::tests::nothing_sounds_in_the_space_behind_the_wall`

### Verified by hand

- It reads as vast rather than as unfinished.
- Walking in is a surprise and walking out is a relief.
- The regular grid reads as nobody's rather than as lazy.

## Out of scope

**Nothing to do in here.** ~~No toy, no cabinet, no key. It is a place and not
a room, and the moment it has a thing in it to work it is another nook.~~

Struck 2026-10-11. Spec 0015 puts the solar system in here, at true
proportion, seventy nine metres from the sun out to Neptune and a hundred and
eleven from the sun to the door. The reasoning above
was sound and the conclusion was wrong: what makes this room worth finding is
not that it is empty but that it is enormous, and there is exactly one thing
worth building that needs somewhere enormous and dark. A hidden empty space is
worth one moment. A hidden solar system is worth the walk back.

**No way of telling you it exists.** ~~No mark on the wall, no draught, no
sound through it. Somebody who never walks into that wall never finds it, and
that is the design rather than a gap in it.~~

Struck 2026-10-13. Spec 0020 wears a lane in the carpet across the near bay,
from the middle of the aisle to the foot of the wall at the hole. The
reasoning above held while there was nothing behind the wall worth the walk,
and by the time the orrery and thirty-three kilometres of solar system were in
there it had become a way of hiding most of what has been built in this
building from everyone who uses it.

The letter of it survives: there is still no mark on the wall, no draught and
no sound through it, because the hint is on the floor and is wear. Everything
else in this hall is a thing somebody installed and that is a trace of
somebody having been here, which is the one thing that can say where to go
without saying anything at all.

**No second way out.** The way in is the way out.
