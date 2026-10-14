# 0018 The system at true scale

**Status:** implemented
**Date:** 2026-10-12, the ride added and given its stops the same day, sped up and given a map, numbered posts, a sky and a list 2026-10-13

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

**Press Return and you ride to the next body on the line**, at 800 metres a
second. A second press stops you where you are.

It was shift held down, and two things were wrong with that. Shift everywhere
else in this building means run, and out there it meant something else. And
Jake could not take a picture of the place, because the shift in his own
screenshot shortcut set him off down the line every time.

A hold is the wrong shape for a thing that stops itself anyway: the leg ends
where it was going whether or not the key is still down, so what the holding
bought was stopping early, which one more press buys instead. Running it is 73 minutes to Neptune and walking it is two and a quarter
hours; this is 42 seconds of travel for the whole thing.

The speed is constant, so the time of a leg is in the same proportion as its
length:

| | |
|---|---|
| the sun to Mercury | 0.6 s |
| Mercury to Venus | 0.5 s |
| Venus to the Earth | 0.4 s |
| the Earth to Mars | 0.7 s |
| Mars to Jupiter | 5.1 s |
| Jupiter to Saturn | 6.0 s |
| Saturn to Uranus | 13.4 s |
| Uranus to Neptune | 15.1 s |

The inner four are over in two seconds and then nothing happens for five. The
longest leg is forty times the shortest crossing between two planets, and that
ratio is the thing, not the seconds.

It was 185, three minutes to Neptune, and it was held there to keep Mercury
perceptible at a couple of seconds. The stops took the constraint away: a body
you are set down at is not something you have to catch sight of in passing, so
the inner legs are free to be as short as they truly are. That is the solar system, and it is the one thing no diagram, no
photograph and neither of the other two scales can do. The jump below was built
first and it skips every metre of it, which is to say it skips the subject.

**Where the leg is going is settled when it sets off**, rather than read from
the facing each frame. Looking about while you travel is the point of riding
rather than being put there, and a ride that read the facing every frame turned
round when you did.

**A leg ends where it was going.** The ride was a throttle first: you held it
and watched and let go. Nobody can make that judgment in the dark, with the
thing you are aiming at a speck that is still a speck a second before you reach
it. So a leg carries you to the next body and sets you down exactly where the
jump would have, and the hold that carried you is spent: going on is another
press. Without that, holding on at the sun runs you through the whole system
and the stop is a stutter rather than a stop.

A frame at this speed is three metres and the standoff at a marble is sixty
centimetres, so arriving is a crossing and not a nearness: the landing is
clamped to the resting place rather than tested against it.

**The ride steps over the moons**, which stand off the line at right angles and
are the jump's to reach. From Jupiter the next body the ride knows is Saturn,
four moons later.

**Which way it goes is the facing's own**, and this is where the ride differs
from the jump. The jump reads the facing against the way the chain was going,
because the chain turns a right angle at every moon. The ride never turns one:
it runs along the line and nowhere else. Read the chain's way here and riding
out of Callisto went to Saturn whichever way you faced, because the way in to
Callisto was at right angles to both answers.

A ride that starts at a moon slides back onto the line at 8 metres a second as
it goes. Callisto is furthest off at 13.9 metres, which is under two seconds of
a leg that is twenty-six.

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

A spine down the middle of the line, and a pair of posts every 250 metres,
which at the ride's speed is three a second. It was every hundred when the ride
was 185 a second; at 800 that is eight a second, which is not a rhythm and is a
flicker rate to keep away from besides. Without them the ride is
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
Neptune is twelve thousand.

**They say which they are.** "1 AU" through "30 AU", written the same way the
bodies' names are, so the two read as one hand. The unit itself needs no
explaining because the Earth stands on the first of them.

One at a time, and the one **ahead** of you. A name out here is drawn at a
fixed angle whatever its distance, per `orrery::label_tall`, so thirty of them
down one line are thirty the same size stacked on the vanishing point. Nearest
was the first try and it is the one you have just gone past: riding out you
never see a number at all, because it comes up behind your shoulder a frame
after you pass the post it belongs to.

Ten of its own heights to the side of the line, which is ten on the glass at
any distance. The same fixed angle puts a number a kilometre off and a body ten
kilometres off in the same place: "4 AU" came up written through the middle of
"Saturn".

Ten because that is sixteen degrees, and **every body subtends the same angle
where the ride sets you down**: the standoff is two and a half times the body
whatever the body is, so it is 11.3 degrees for the sun and for Jupiter and for
a marble alike. One offset clears all of them. At one the number overlapped, at
three and a half it touched, and at six it came out from behind Jupiter's edge
as a stray "U".

To the world's west, which is away from the moons, since they all stand east of
their planets and their names with them.

There is no nought. That is where you are measuring from and the sun is already
named there.

### The map at the foot of the screen

Two bars, out past the door and nowhere else.

The thin one is the whole line, the door at the left and Neptune at the right,
linear, with a tick for every body on it. Mercury sits at a hundredth of the
way across and the four inner planets are all inside the first twentieth. That
picture is true, and it is the reason the solar system is hard to draw.

It is also why it cannot be the whole map. A bar on which Mercury is one pixel
from the door cannot tell you anything about crossing to it. So the thick bar
under it is **the leg you are on, measured again on its own**: nought where you
left, one where you are going, filling the width whatever its length, with both
bodies named at the ends. That is the one that answers how far.

Standing at a moon it shows the moon's planet's leg, because the ride runs on
the line and the moons stand off it. At the far end facing out there is no leg,
because there is nowhere to go, and the thin bar still says where you are.

It carries a plate, for the reason the corner's text carries one: what is
behind it out here is a lit line running to a vanishing point directly under
the middle of it.

### The list, for when you have had enough

**Tab opens a list of every stop**, down the left of the glass where neither
the help across the top nor the map along the bottom goes. It opens on the one
you are at, W and S walk it, and Return takes you there outright and closes it.
It stops at the ends rather than wrapping, because a list that wraps is a list
you can lose your place in.

Jake asked for it after stepping through sixteen bodies to reach the
seventeenth. The ride is still what the space is for, and the help line names
it first for that reason, but a model you can only cross one body at a time is
a model you stop crossing.

### And the step, which the list replaced

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

### The staged shot

`--screenshot` stages Saturn and Titan, which is what `media/screenshot.png`
holds, so that the project's own `refresh-screenshots` reproduces the committed
picture rather than quietly replacing it. It staged the hall for a day while
the committed picture was the system, which is a script that undoes your work
the first time anybody runs it. The hall's pose is still there behind
`--screenshot=hall`.

Saturn and Titan because Saturn is the one body in the tables that is not a
ball, and a moon at its true distance is what this space is for.

**Past the moon, looking back**, which is the only place both are visible at
once, and the two pull against each other to get there. The ring plane is
tilted about the line, so a view along the line lies in it and the rings come
out as a black streak; a moon stands out at right angles to the line, so a view
across it puts the moon behind its planet. From past the moon the rings open 25
degrees and the two sit 38 apart.

Worked out from the bodies and not written down, the way the hall's own pose is
worked out from a cabinet. A bearing goes stale the day anything moves.

### The sky, and the floor under it

**The spine stands on the stars rather than beside them.** Both were at 0.02,
so the spine's top face and the whole star sheet were one plane: the spine came
and went in pieces as you moved, which Jake called the track clipping and is
what two exposed faces in one plane have done to this building seven times now.


Jake said the black was empty when it should be full of stars and galaxies,
and that the planets were standing against something that read as a road.

**Six faces of stars round the eye**, which are the same six the room of spec
0015 has. They are drawn 36 kilometres off, which is further than anything out
here can be from anywhere else: the line runs 33.4, so standing at one end the
other is 33.4 away and the sky is still behind it. Nearer than that and a body
at the far end would be hidden inside the sky.

They move with you, so walking does not shift a star, which is what a star
does. The far plane goes out past the sky's furthest corner, its half-size by
the root of three, because short of that the corners are clipped away and you
are looking at four triangles of nothing where they met.

**The floor takes stars too**, laid in 128 metre patches and repeated, which
is exactly the biggest picture `starred` will make at `STAR_PER_UNIT`, so the
stars underfoot are the same size as the room's. A floor that is the one dark
surface in a sky of them is a floor, and what you are standing on out here is
not meant to be one.

**The galaxies are the sky's alone.** A galaxy is a soft ellipse a few per
cent of the picture across, and a picture laid on something you can walk up to
puts that at a few metres: on the floor they came out as six metre smudges at
your feet. The sky is the one surface anywhere in this building that is
genuinely far off.

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
- The floor laps under the hidden room's. — `beyond::tests::the_floor_laps_under_the_room`
- The ride crosses in the same proportion as the distance. — `beyond::tests::the_ride_crosses_in_proportion`
- And stays on the line whatever you are looking at. — `beyond::tests::the_ride_stays_on_the_line`
- A leg ends at the next body on the line, where the jump would have put you. — `beyond::tests::a_leg_of_the_ride_ends_at_the_next_body`
- And lands on it rather than near it, however long the step. — `beyond::tests::the_ride_does_not_overshoot`
- The ride steps over the moons. — `beyond::tests::the_ride_steps_over_the_moons`
- One press is one leg, and a second press stops it. — `tests::a_press_rides_one_leg`
- The list goes straight to any body, opens where you are and does not wrap. — `tests::the_list_goes_straight_to_a_body`
- Shift runs out there, the way it does everywhere. — `tests::shift_runs_out_there_too`
- The staged shot sees the rings and the moon at once. — `beyond::tests::the_staged_shot_sees_the_rings_and_the_moon`
- The spine stands on the stars rather than beside them. — `beyond::tests::the_spine_stands_on_the_stars`
- And it stops at the door and at the far end. — `beyond::tests::the_ride_stops_at_both_ends`
- The posts stand up and come often enough to read as speed. — `beyond::tests::the_posts_stand_up_and_come_often`
- The unit posts say which unit they are, and the one that speaks is the one
  ahead of you. — `beyond::tests::the_number_is_the_post_ahead`
- The Earth stands on the first of them, which is what says what a unit is
  without a word being spent on it. — `beyond::tests::the_earth_stands_on_the_first_unit`
- And it stands clear of whatever body you are at. — `beyond::tests::the_number_stands_clear_of_the_body_you_are_at`
- And there is no number before the first or past the last. — `beyond::tests::there_is_no_number_off_the_ends`
- The map's whole-line bar is linear, which crushes the inner planets. — `beyond::tests::the_whole_line_crushes_the_inner_planets`
- And the leg fills the map whatever its length, which is what answers how
  far. — `beyond::tests::the_leg_fills_the_map_whatever_its_length`
- Standing at a moon the map reads its planet's leg. — `beyond::tests::the_map_reads_a_moon_as_its_planet`
- And there is no leg past the end. — `beyond::tests::the_map_has_no_leg_past_the_end`
- The map fits on the glass at every window the building opens at. — `aim::tests::the_map_fits_on_the_glass`
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

## What was still wrong

Jake rode it on 2026-10-12 and called it a good starting point. Three things he
asked for, all three done since:

- ~~**It should stop at each body on its own.**~~ Done 2026-10-12.
- ~~**Minutes to Neptune is too long.**~~ Done 2026-10-13: 42 seconds of
  travel for the whole line, the longest leg fifteen.
- ~~**The posts should say what they measure.**~~ Done 2026-10-13: every
  taller pair carries its own number, and the Earth standing on the first is
  what says what the unit is.

## Out of scope

Orbits, which at this scale are not a thing to walk. Moving bodies: they stand
where they stand. Any body not in spec 0015's tables, which is to say Pluto,
the asteroids, and the two specks round Mars that spec 0015 left out for the
same reason they would be left out here.

**Galaxies anywhere but on this sky.** The hidden room of spec 0011 is the
other dark place in this building and its standing walls look like the obvious
second home for them, being the one dark surface that is nowhere near your
feet. They are also 3.2 metres high against 159 long, and a galaxy is a share
of a picture's smaller side, so on that letterbox it comes out between eight
and twenty-five centimetres. Built and photographed: two frames of the same
spot, differenced, came back 91 pixels of two million changed by three code
values or more.

Sizing them off the long side instead only moves the problem, to 12.7 metres
on a wall 3.2 high. A galaxy's whole job is saying the dark goes on past the
stars, and a wall you can walk up to and touch says the opposite. The two
surfaces in that room that are big whichever way you measure them are its
floor and its lid, and a galaxy on those is the six metre smudge at your feet
that `orrery::galaxied` already records. — `orrery::tests::a_galaxy_wants_a_surface_that_is_big_whichever_way_it_is_measured`
