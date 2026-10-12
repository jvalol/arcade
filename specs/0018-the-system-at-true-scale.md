# 0018 The system at true scale

**Status:** implemented
**Date:** 2026-10-12, the ride added the same day

## Goal

Spec 0015 says, in as many words, that the honest solar system "is not
something anybody can look at". That was true of a solar system that has to
fit in a room. Through the door of spec 0017 there is no room, so this is the
honest one: every body and every distance at one scale, with nothing bent
except the two things named below.

## Behavior

### One scale, and the one number that sets it

The smallest body in the tables is Triton, 2,707 km across, and it is drawn as
a two centimetre marble. That is the whole of the scale and everything else
follows from it:

**one metre is 135,350 kilometres.**

Derived from the tables rather than written down, so that a smaller moon
joining them moves the whole model rather than quietly breaking the one
promise this spec makes.

| | across | out from the sun |
| --- | --- | --- |
| the sun | 10.29 m | |
| Mercury | 3.60 cm | 427.8 m |
| Venus | 8.94 cm | 799.4 m |
| Earth | 9.42 cm | 1,105.3 m |
| Mars | 5.02 cm | 1,684.1 m |
| Jupiter | 1.056 m | 5,750.3 m |
| Saturn | 0.891 m | 10,560.7 m |
| Uranus | 37.77 cm | 21,241.5 m |
| Neptune | 36.59 cm | 33,280.0 m |

And the moons, which are the part nobody believes: the Moon is a 2.57 cm
marble **2.84 metres** from a 9.42 cm Earth. Callisto is 13.91 m out from
Jupiter, Titan 9.03 m from Saturn, Triton 2.62 m from Neptune.

### What it looks like

A line, not orbits. An orbit at this scale is a circle 66 kilometres across
and there is nothing to be gained by walking one. The bodies stand out from
the sun along a straight line running north from the door, each at its true
distance, and each planet's moons stand out from it at right angles to that
line, at their true distances.

Every planet's and moon's centre is at 1.6 metres, which is eye height and is
`orrery::PLANE`. The sun's is at its own radius, so the sun rests on the floor
rather than being buried in it.

**The sun is the one body not at eye height, and that is the point.** It is
ten metres across. No single height can hold a ten metre ball and a two
centimetre marble, and which of the two had to give is the fact this whole
space exists to show.

### The lie, and there is one

A point light in this engine gives `(1 - d/range)²` and nothing past its
range, per spec 0020 of the engine. Real sunlight falls as the inverse square
and never reaches nothing: Neptune gets about a nine hundredth of what the
Earth gets. Drawn honestly, everything past Jupiter is black.

So the sun's range is 100 kilometres against a model 33 long. That leaves
Mercury at 0.99 of full light and Neptune at 0.45, a spread of two where the
truth is a spread of nine hundred.

This is the only thing bent here, and it is bent because the thing this space
is for survives it. What you are meant to come away with is the **sun
shrinking**: 22 pixels tall from Mercury on a 1080 screen, 9 from the Earth,
under 2 from Jupiter, and a third of a pixel from Neptune, which is to say a
star. That reading needs the planets lit enough to find and does not need the
light to be right.

### Getting about

### The ride, which is the point

**Hold Shift and you travel down the line**, at 185 metres a second, which is
three minutes from the sun to Neptune. Running it is 73 minutes and walking it
is two and a quarter hours.

The speed is constant, so the time between two bodies is in the same proportion
as the distance between them:

| | |
|---|---|
| Mercury | 2.3 s |
| Venus | 4.3 s |
| Earth | 6.0 s |
| Mars | 9.1 s |
| Jupiter | 31 s |
| Saturn | 57 s |
| Uranus | 115 s |
| Neptune | 180 s |

The inner four are over in nine seconds and then nothing happens for
twenty-two. That is the solar system, and it is the one thing no diagram, no
photograph and neither of the other two scales can do. The jump below was built
first and it skips every metre of it, which is to say it skips the subject.

It would be wrong to ease it. An acceleration would put the inner planets in
slow motion and hurry the gulf, which is the shape of the lie every drawing of
the solar system tells.

**You do not steer.** The ride runs along the line and you keep the mouse,
which is the whole point of riding rather than jumping, because the thing to
watch is the sun going small behind you. At 185 metres a second a ten degree
turn crosses the twenty metres to the edge of the floor in under a second, and
over the edge there is no floor at all. It stops at the door rather than going
through the wall, and at the far end rather than off it, and it says both
itself: the ride does not go through `walk`, because that sweep is three metres
a frame looking for something to catch on over a slab with nothing on it.

### What you ride past

A spine down the middle of the line, and a pair of posts every hundred metres,
which at the ride's speed is nearly two a second. Without them the ride is
three minutes of an unchanging black screen, which reads as broken rather than
as vast.

**They stand up, and the first version did not.** Laid flat on the floor
nothing was drawn at all: a mark on the floor seen from eye height is edge-on
past twenty metres, and at ninety the depression angle is one degree, so three
tenths of a metre of mark came to five thousandths of a metre of picture. It is
the lesson the hall's columns already teach, which spec 0011 puts as the only
way to read a distance being to see the same thing at several of them.

A taller pair every astronomical unit, 1,105.3 metres, thirty of them over the
whole line. They glow, because a post two metres tall is a pixel at seven
hundred metres and nothing at a kilometre, and the gulf between Uranus and
Neptune is twelve thousand. Nothing says what they are and nothing needs to:
the Earth stands on the first.

### And the jump, for when you have had enough

**Return steps to the next body.** One press, one body, in the order they come
out from the sun with each planet's moons after it: the door, the sun,
Mercury, Venus, Earth, the Moon, Mars, Jupiter, Io, Europa, Ganymede,
Callisto, Saturn, Titan, Uranus, Neptune, Triton. Seventeen stops, and the
door is one of them, so stepping in from the sun is how you leave.

Which way you go is which way you are facing, measured against **the way the
chain was already going** and not against where the next stop is. The chain
turns a right angle at every moon, because a planet's moons stand out at right
angles to the line: arriving at the Moon you are facing along the Earth-to-Moon
line and Mars is 579 metres off at ninety degrees to it, so compared with the
way to Mars, facing on from the Moon pointed at nothing and the chain stopped
dead there.

You arrive standing short of a body on the line you travelled in along, looking
at it, so pressing again carries on the way you were going and turning round
takes you back.

**Where you are in the chain is remembered**, which took two goes to accept.
Worked out as the nearest stop in metres it is wrong at the sun: you arrive
25.7 short of it and that is 4.6 from the door, so a second press takes you to
the sun you are already standing at. Measured in standoffs instead, so that the
stop you travelled to always reads about one, it is wrong at every moon:
standing at Io you are 1.0 of Io's standoff from Io and 0.95 of Jupiter's from
Jupiter. There is no distance that means "here" in a model whose largest body
is five hundred times its smallest. It is one `usize`, and crossing the
threshold puts it back to the door.

How far short is the body's own size: far enough off that it fills the view
and no further. A marble is 60 cm away and the sun is 26 metres.

No menu, no list, no key but the one the rest of this building already uses
for "do the thing in front of you".

### What else had to give

**The camera sees further out here.** The building's far plane is a hundred
metres, which is further than any of it goes and is what keeps the far wall of
the hidden room a thing you find rather than a thing you see. Out here a
hundred metres culls the sun from everywhere past Mercury, which throws away
the one reading this space is for, so past the door it reaches the far end of
the floor instead.

**The sun is pushed only while you are out there with it.** A point light goes
through a wall in this engine unless it casts, and this one has a hundred
kilometres of range: left on, it lit every surface in the building from the
north at full strength, and from a thousand metres out the back of the hidden
room's north wall was a grey bar across the sky. What pops as you cross the
threshold is the lit floor seen through a doorway 1.5 metres wide, and the
sun's own ball glows and so is drawn either way.

**The floor takes the tightest highlight there is**, `aim::VOID`, for the same
reason the hidden room's floor does. A broad lobe on a floor you are looking
along is metres of it: from the Earth the sun laid a searchlight down the whole
1,105 metres back to itself, which is the opposite of a sun that has shrunk to
a speck.

**The 33 kilometre floor is kept out of the building's outline.** In it, the
directional shadow map's texel goes from thirty-six millimetres to seventy
metres, which is no shadow anywhere in the building, and spec 0049's reach fill
goes past the engine's cap of 32 million cells, which is no fill at all and a
test that quietly proves nothing.

**The labels are laid out for your side of the wall only.** Both systems are
drawn every frame and `orrery::laid_out` reads the frustum and not what is in
the way: from the Earth out here, the room's whole model is 1,200 metres behind
you through a wall, and its eight names came up stacked on the one speck the
sun had become. It is also what keeps the two lists apart, since a name is
picked by which body it is for and the same body stands in both systems.

### The floor

One slab, 40 metres across and running from the door to past Triton. It laps
back under the hidden room's own floor, for the reason spec 0017 gives: two
floors at one height that meet edge to edge put a vertical face at the surface
you are walking on, and a body resting on a surface counts as touching it.

40 across because Callisto stands 13.91 metres off the line and wants room to
walk round.

This replaces the 24 by 24 landing of spec 0017, which was that spec saying
where it stopped. Two floors beyond that door would be two accounts of one
thing, and this building knows what those do.

## Acceptance criteria

- The scale comes off the smallest body in the tables, and that body is drawn
  as a two centimetre marble. — `beyond::tests::the_smallest_body_is_a_marble`
- Every body and every distance is in one proportion. — `beyond::tests::one_scale_and_no_other`
- Every stop stands on the floor. — `beyond::tests::nothing_stands_off_the_floor`
- You arrive looking at the body you asked for, with it filling a sensible
  part of the view. — `beyond::tests::you_arrive_looking_at_it`
- Pressing on carries you out and turning round takes you back. —
  `beyond::tests::the_way_you_face_is_the_way_you_go`
- The door is a stop, so there is a way home. — `beyond::tests::the_door_is_the_last_stop_in`
- The floor laps under the hidden room's. — `beyond::tests::the_floor_laps_under_the_room`
- The ride crosses in the same proportion as the distance. — `beyond::tests::the_ride_crosses_in_proportion`
- And stays on the line whatever you are looking at. — `beyond::tests::the_ride_stays_on_the_line`
- And stops at the door and at the far end. — `beyond::tests::the_ride_stops_at_both_ends`
- The posts stand up and come often enough to read as speed. — `beyond::tests::the_posts_stand_up_and_come_often`
- Shift rides out there and runs in here, asked of the one place that chooses
  between them. — `tests::shift_rides_out_there_and_runs_in_here`
- The sun reaches Neptune with something left. — `beyond::tests::the_sun_still_reaches_neptune`
- And you can see it from there, which the building's own far plane does not
  allow. — `beyond::tests::the_far_plane_reaches_the_sun_from_neptune`

### Verified by hand

- The sun from the doorway reads as a ten metre ball resting on the floor.
  Confirmed 2026-10-12.
- The sun from Neptune reads as a speck. Confirmed 2026-10-12: a dot under its
  own name, 33.3 kilometres off.
- A marble at a jump's standoff is worth looking at. Confirmed 2026-10-12 at
  the Earth, 9.4 cm across at 60 cm, with its coastlines and the sun's
  highlight on it.
- The building is unchanged. Confirmed 2026-10-12 against the staged shot of
  the hall.

## What is still wrong

Jake rode it on 2026-10-12 and called it a good starting point. Three things
he asked for, written down here rather than fixed, because each is a change to
what the ride is:

- **It should stop at each body on its own.** At the moment you hold and watch
  and let go, and judging when to let go in the dark is a thing nobody can do.
- **The posts should say what they measure.** They are a rhythm and nothing
  else right now, so what you get from them is speed and not distance.
- **Minutes to Neptune is too long.** Three was chosen to keep Mercury
  perceptible at 2.3 seconds, and the two wants pull against each other only
  while the ride does not stop.

The three fit together, and that is the shape of the fix rather than three
separate ones. A ride that stops on its own can be far faster without losing
the inner planets, because they are no longer things to catch sight of in
passing. Nothing above changes: the time between two stops stays in the same
proportion as the distance between them, which is the one claim this spec
makes about the ride.

## Out of scope

Orbits, which at this scale are not a thing to walk. Moving bodies: they stand
where they stand. Any body not in spec 0015's tables, which is to say Pluto,
the asteroids, and the two specks round Mars that spec 0015 left out for the
same reason they would be left out here.
