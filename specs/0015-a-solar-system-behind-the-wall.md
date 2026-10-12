# 0015 A solar system behind the wall

**Status:** draft
**Date:** 2026-10-11, labels added the same day

## Goal

Spec 0011 built a space behind the hall's near wall and put nothing in it,
which was the joke: you walk through a wall you can see and find a room that
is only a room. A hidden empty space is worth one moment. A hidden solar
system is worth coming back for.

So: the sun at the middle of it, the eight planets out from it at their real
proportions, their moons round them, and stars on every surface. You come in
at the cold end, a hundred and eleven metres from the light, and walk in.

Spec 0011 ruled this out in as many words: "No toy, no cabinet, no key. It is
a place and not a room, and the moment it has a thing in it to work it is
another nook." Jake struck that on 2026-10-11. The reasoning there was sound
and the conclusion was wrong: what makes the room worth finding is not that it
is empty but that it is enormous.

## Behavior

### Two scales, and why there have to be two

One scale cannot work and the number says so. Put Neptune fifty metres out at
true proportion and the Earth is **0.14 mm** across: eight specks and a marble,
with fifty metres of nothing between them. That is the honest solar system and
it is not something anybody can look at.

So two scales, each exactly proportional inside itself:

| | one metre is | |
| --- | --- | --- |
| **bodies and their moons** | 557,080 km | the Earth is 2.3 cm across |
| **planets from the sun** | 56,769,000 km | one AU is 2.64 m |

The orbit scale is **101.9 times** the body scale. That ratio is the first lie
in the model and it is written here rather than hidden: every size is true
against every other size, every distance is true against every other distance,
and sizes are not true against distances.

The body scale was the sun's own diameter at first, which made the sun exactly
a metre and the Earth a nine millimetre pea. A pea in a room this size is a
thing you walk past without seeing, so the bodies went up two and a half
times, and the ratio went to 167.8 with them.

**The Moon sets the orbit scale.** This was the other way round until
2026-10-11 and it was wrong in a way that showed.

A moon is on the body scale, per the note under `Moon`, so a moon's orbit is
drawn a hundred times too wide against the planets' spacing. The Moon is the
worst of them: at an AU of 1.60 m it reached 0.690 m from the Earth against a
gap to Mars's orbit of 0.838 m, which is 82 per cent of it and 15 cm of
daylight between the Moon and a planet 1.2 cm across. Jake looked at it and
said the Moon's orbit gets really close to Mars. Callisto took 49 per cent of
the gap to Saturn and Titan 14, so the Moon was the one.

None of that was a slip and none of it was hidden. The rule here said "two and
a half and no further, and the Moon says so: at three times, the Moon's orbit
is as wide as the gap between the Earth's orbit and Mars's." What it got wrong
was the threshold. It stopped at the point where the two touch, and a model
starts lying about which things are near which a long way before that.

So `orrery::MOON_SHARE` is the threshold now, at a half, and `orrery::au` is
worked out from it: the Moon's orbit is fixed by the body scale, so the orbit
scale is whatever makes that orbit half the gap. The share is the constant and
the distance is derived, the same way round as `SUN_SHRUNK`, so growing the
bodies again moves the orbits with them instead of quietly eating the margin.

The half is a judgment and the only one in this. It is where the Moon's orbit
reads as belonging to the Earth rather than as reaching for Mars. Two fifths
wants a room two hundred metres across.

**What it cost and what it bought.** An AU of 2.64 m instead of 1.60, a room
159 m square instead of 97, and a walk from the door to the sun of 111 m
instead of 48: twenty seven seconds walked where it was eleven, thirteen at a
run. Shift was already there, from the running work earlier the same day, and
it is the only reason the bigger room is a distance rather than a trudge.

What it bought is every proportion in the model staying exactly what it was.
The Moon is still thirty Earths out, Callisto is still thirteen Jupiters out,
Neptune is still thirty times the Earth's distance. Nothing was made less
true. Two things improved on their own: the ratio between the scales fell
from 167.8 to 101.9, the closest they have been since the bodies grew, and
Mercury's clearance of the sun went from 11.8 cm to 51.6.
Callisto gives out at 5.2 times and Saturn's rings at 71, so the Moon is the
whole of the limit.

**And the sun is drawn at 40 per cent of true**, which is the second lie. Its
true size on the body scale is 2.50 m, and 2.50 m does not fit: Mercury has to
clear it at 0.387 AU, and that wants a disc two hundred and forty metres
across. Every planet walk in the world tells this lie for the same reason.

### Why the sun is drawn at one metre

In the orbiting arrangement, Mercury has to clear the sun's own ball. Mercury
sits at 0.387 AU, so the orbit scale cannot be less than about 1.3 times the
sun's diameter per AU, and Neptune is thirty AU out. The disc you need is
therefore **about seventy eight times the sun's diameter across**, and a sun
twice as wide costs four times the floor.

At one metre, Mercury's surface clears the sun's by 51.6 cm and the disc is
158.7 m across. At two and a half metres it would be 397 m across.

The cell cap this paragraph used to warn about is real and this room has now
hit it: the reach fill over a building 190 by 11 by 179 at a quarter-metre
step wants 24 million cells against the engine's 16 million, and it refused.
Two of the arcade's own tests caught it, because both assert the fill reached
something before asking where. The engine's cap went to 32 million rather
than each fill being bounded to the part of the building it expects to reach,
since a fill bounded to where you already expect to get cannot find the place
you did not. See the note on `blitzkit::reach::MOST`.

The price is small planets: Earth is a nine millimetre pea. If that turns out
to disappoint, drawing the sun alone at a third of its true size and labelling
it makes every planet two and a half times bigger in the same room. That is
what planet walks do. It is one constant and it is deliberately not taken here.

### The room

A hundred and fifty nine metres square, where spec 0011's forty four by fifty
two was and where this spec's own ninety seven was until the Moon moved the
orbit scale. The way in stays where it is, in the hall's near wall, and the
sun stands at the middle of the square, a hundred and eleven metres from the
door.

So you enter at Neptune and walk toward the light, which is the right way
round: the cold, dark, far end first.

### The bodies

Diameters, distances, tilts. Spin and year are given on the two clocks below.

| | ball | from the sun | tilt |
| --- | --- | --- | --- |
| Sun | 100.00 cm drawn, 250.00 true | the middle | 7.2° |
| Mercury | 0.88 cm | 0.62 m | 0.0° |
| Venus | 2.17 cm | 1.16 m | 177.4° |
| Earth | 2.29 cm | 1.60 m | 23.4° |
| Mars | 1.22 cm | 2.44 m | 25.2° |
| Jupiter | 25.67 cm | 8.32 m | 3.1° |
| Saturn | 21.64 cm | 15.29 m | 26.7° |
| Uranus | 9.18 cm | 30.75 m | 97.8° |
| Neptune | 8.89 cm | 48.18 m | 28.3° |

Venus and Uranus carry their real tilts, which are the two that look like
mistakes: Venus is upside down and Uranus lies on its side. Both are true and
both are worth seeing.

### The moons

On the **body** scale, not the orbit scale. The Moon at the orbit scale is
four millimetres from the Earth, which is inside it. On the body scale it is
2.5 mm across and 27.6 cm away, which is the famous proportion and reads.

| | of | ball | out |
| --- | --- | --- | --- |
| Moon | Earth | 0.62 cm | 69.0 cm |
| Io | Jupiter | 0.65 cm | 75.7 cm |
| Europa | Jupiter | 0.56 cm | 120.5 cm |
| Ganymede | Jupiter | 0.95 cm | 192.1 cm |
| Callisto | Jupiter | 0.87 cm | 338.0 cm |
| Titan | Saturn | 0.92 cm | 219.3 cm |
| Triton | Neptune | 0.49 cm | 63.7 cm |

Saturn's rings run from 5.36 cm to 9.82 cm in radius with the Cassini division
at 8.44 cm, against a planet 4.33 cm in radius. Those four numbers are what
makes a ringed ball read as Saturn and not as a ball with a hoop.

Phobos and Deimos are 22 km and 12 km across, which is four hundredths of a
millimetre. They are left out rather than cheated.

### Two clocks, for the same reason there are two scales

| | one second is | |
| --- | --- | --- |
| **spin** | 1.2 hours | Earth turns once in 20 s |
| **orbit** | 6.1 days | Earth goes round in 60 s |

On the spin clock Jupiter turns in 8.3 s and Venus in 81 minutes, which is
Venus turning slower than it orbits and is true. On the orbit clock Mercury
goes round in 14 s and Neptune in 2 hours 45 minutes, so you see the inner
planets racing and the outer ones hardly moving, which is the whole point of
the picture.

Tied to one clock, a day would be a sixth of a second and every planet would
strobe. Two clocks, each proportional inside itself, same bargain as the
lengths.

### The two arrangements, and the swing between them

**Lined up.** Every planet on one line from the sun to the door, at its own
distance. This is the walk, and it is the arrangement that fits a room.

**In orbit.** Every planet at the same distance, at the angle its orbit clock
puts it, going round.

A key swings between them and **it must not snap**. Each planet travels along
its own circle from one angle to the other over about three seconds, so the
line fans out into the orbits and folds back. Every planet keeps its own
radius throughout, which is what makes it read as the same model rearranged
rather than two models.

Started in the middle of a swing, the swing runs from where things actually
are. A toggle caught halfway that jumps is a toggle that breaks the illusion
the whole exhibit rests on.

### The light

The sun is the only lamp. Its own ball is drawn past white so it glows with
no light on it, the way the neon does.

Neptune really does get one nine hundredth of Earth's sunlight, so a falloff
that leaves the outer planets dim is correct rather than broken. Dim and not
invisible: the room carries a low starlight ambient so nothing is ever flat
black.

### The pictures

Everything is drawn in code, as everything in this building is.

- **Earth** has the only outside data in the project already: `data/land.txt`
  holds the world's coastlines as rings and `globe::drawn` makes a map from
  them. The one body that cannot be faked is the one that is done.
- **Jupiter**, banded, with the Red Spot. Bands are a stack of noise stretched
  along latitude, which is what bands are.
- **Saturn**, fainter bands, and the rings as a ring of varying opacity with
  the Cassini division cut out of it.
- **Mars**, rust with the dark albedo marks and two polar caps.
- **Moon**, grey with the maria as dark patches.
- **Mercury**, grey and cratered.
- **Venus**, featureless cream cloud, which is what it is.
- **Uranus** and **Neptune**, near featureless, Neptune the deeper blue.
- **The sun**, granulation, limb darkening, and a few spots.

### The stars

On all six surfaces, as one picture per surface rather than a tile repeated.
A plausible spread of magnitudes rather than a uniform sprinkle, since a sky
of equal dots reads as noise. Real constellations would need a catalogue and
are not in this spec.

**Every body is named, over the body, facing you.** The sun, the eight
planets and the seven moons: sixteen names off the same `orrery::every_body`
the pictures come from, walked with one index for both, so a name and a picture
cannot end up on two different bodies.

A name is a quad with the name drawn on its own black ground, the same as the
way out hangs on. It went in cut out of its ground, per the engine's spec
0047, on the reasoning that a plate out here would be a hole punched in the
stars. Both halves of that were wrong, and the window said so.

**A cut-out name is gone at a distance.** Thirty metres from Saturn, with the
columns taken out of the build so that nothing could be hiding it and the
multiplier run up to 24 so that nothing could be dimming it, there was no name
over Saturn at all. Changing the one line put it back, bright and sharp, from
the same spot. Walking back from the sun, cut-out names are there at two
metres and four, thin at eight, and gone by twenty.

Why is not established, and this spec is not going to pretend otherwise. The
engine's spec 0047 says mip levels average the alpha down until small text
dissolves, and the shader drops a fragment under `CUTOUT`, which is 0.35, so
that is the obvious candidate. It does not fit on its own: these names hold
the same size on the glass at every distance by construction, so the sampler
should be reaching the same mip at two metres as at thirty. Occlusion and
brightness are ruled out by the two runs above. What is left is a measurement
nobody has taken, and it is the engine's to take. The study's book spines are
the only other cut-out text in the building and they are read from arm's
length, which is why this has never shown.

**And the hole in the stars is not there.** The sky in this room is black, so
a black plate against it is invisible except where it crosses a star. What it
buys is a name that reads against the sun as well as against the dark, and the
sun is a metre of blown-out white that the inner names stand in front of from
anywhere near the middle of the model. It also makes two names that overlap
read as two cards rather than as one smear.

A name is multiplied past one so it carries its own light, because a name is a
thing somebody put there rather than a thing the sun is shining on, and out
past Uranus there is not much sun left. The multiplier is warm, because
`aim::FILL` is blue and three equal numbers come out of it blue.

**Planets hang their name over, moons hang theirs under.** In the lined up
arrangement every body is on one line, so read from along that line every name
lands on the same row and they pile into each other. A moon is always within a
stride of its planet and is the pair most likely to collide, so hanging the
moons underneath pulls the worst of it apart and says which body belongs to
which at the same time.

It does not fix the rest. Standing a metre from Earth, the names of Mercury,
Venus and Earth still stack, because the three bodies are within a metre of
each other and the names are the same size whatever they are naming. That is
the lined up arrangement being what it is, and in orbit the same three are
spread round the sun. A layout pass that pushed overlapping names apart is a
different and much larger piece of work.

**A name is the same size on the glass from wherever it is read.** This is the
whole of why it works. Triton is five millimetres across and sixty four metres
from the door, and a name that shrank with the ball would be one texel: what a
label out here has to beat is not the dark, it is the distance. Scaled with the
distance, every name in the room is a line 2.4 per cent of the screen's height,
which is 26 pixels on a 1080 window against a font that wants eight.

What it costs is that the far names are enormous in the room: Neptune's is a
metre and a third of lettering beside a ball nine centimetres across. That is
the trade and it is the right way round. A model you cannot find the planets in
is not a model of anything, which is what Jake said about it on the first walk
through.

**Written over every frame.** Where a name stands, how big it is and which way
it faces all come off where your eye is, so there is nothing to cache. Sixteen
quads is the cheapest thing in this room by a long way.

**The turn is `orrery::label_turn` and not `room::laid_on`.** `laid_on` is the
same job for a wall and answers only the six directions a wall faces. Writing
the general one against it turned up that `laid_on` is upright for +x and +z
and upside down for -x and -z: it faces the right way on all four but puts the
quad's top at -y on two of them. Nothing has ever shown it, because the three
surfaces it draws are tiling plaster, tile and pool lining, none of which has a
top, and the one sign that uses it faces +z. A sign on the other two walls
would read upside down.

The letters are made level by construction, `Vec3::Y` crossed into the normal,
rather than by projecting world up out of the normal. Projecting leaves a
residue at the poles: a body a thousandth off straight overhead came out rolled
by a ten-thousandth, which is nothing to look at and is still a quad whose
letters are not level.

**The sun is ranked by reach and not by distance, or the engine drops it.**
The building pushes every fitting it has and the engine carries 64 of them,
per its spec 0020, so the arcade sorts them and the engine keeps the front of
the list. That sort was plain distance from the eye.

Growing this room to 159 m took its ceiling grid from 36 lamps to 100. The sun
stands 111 m from the way in and every one of those lamps is nearer, so the
sun came out 76th of 133 and the engine dropped it: the one light this room
exists for, and the only thing lighting sixteen bodies, thrown away for a
ceiling lamp in the far corner.

A point light gives exactly nothing past its range, per the engine's
`PointLight::falloff`, so how far off a lamp is means nothing on its own. What
means something is how far off it is against how far it throws.
`inside_its_reach` is that ratio. The sun reaches 83 m and is 111 away, which
is 1.34 of its reach; a ceiling lamp reaches 9.5 and is 40 away, which is 4.2.

Measured after the change, with the sun's rank out of 133 pushed:

| standing at | how far off | the sun's rank |
| --- | --- | --- |
| the hall, outside the room | 114 m | 4 |
| the way in | 111 m | 3 |
| Neptune | 80 m | 1 |
| Uranus | 51 m | 0 |
| Saturn | 25 m | 0 |
| Earth | 4 m | 0 |

What the ratio does not do is promise the sun first place from everywhere, and
it should not. Past its own 83 m the sun gives nothing where you are standing,
whatever it is doing to the planets you are looking at, and a lamp spent by
exactly as much ranks level with it. The table is why that is survivable here:
the real lamps are far worse than level. A metric that read what a light gives
to *what you can see* rather than to where you stand would be the honest one
and is a different piece of work.

**Still over the cap.** 133 pushed against 64 means 69 dropped every frame,
and the engine says so once per dropped light: about 8,600 lines a second.
They are the right 69 now, but the log is unreadable and the formatting is
work done every frame. Not fixed here.

## Acceptance criteria

To be written with the code. Each of these is arithmetic and belongs in a test:

- Every body's ball is its real diameter on the body scale, and every planet's
  distance is its real distance on the orbit scale.
- Mercury's surface clears the sun's surface in the orbiting arrangement.
- Every moon is outside its planet and inside the next planet's distance.
- Saturn's rings start outside Saturn and the Cassini division falls between
  the two bright rings.
- The swing leaves every planet at its own radius at every moment of it.
- A swing interrupted halfway and sent back does not jump.
- Nothing in the room is close enough to anything else to fight it, per spec
  0048 of the engine.
- You can walk from the door to the sun, and round it.

The names are written, and these are theirs:

- A name turned at a wall is the turn that wall already uses, where that turn
  is upright. — `orrery::tests::a_name_turned_at_a_wall_is_the_wall`
- A name faces you, stands up, and does not roll, from straight under a body
  as well as from beside it. — `orrery::tests::a_name_faces_you_and_stands_up`
- A name is the same size on the glass however far off it is, and never under
  the font's grid. — `orrery::tests::a_name_is_the_same_size_however_far_off_it_is`
- A name stands clear of its own body, at every size, the sun included. — `orrery::tests::a_name_stands_clear_of_its_body`
- No column stands on the line the bodies are strung along. — `behind::tests::nothing_stands_on_the_line`
- And the width that clears it has room to move in, without gutting the grid. — `behind::tests::the_line_is_cleared_by_a_wide_margin`
- Lined up, the way from the doorway to every body runs through no column. — `behind::tests::every_body_is_in_view_from_the_doorway`
- The sun outranks a spent ceiling lamp from anywhere in the room, so the engine keeps it. — `tests::the_sun_outranks_a_ceiling_lamp_from_anywhere`
- Every body has a name, no two share one, and every one draws something. — `orrery::tests::every_body_is_named_once`

### Verified by hand

- Standing at the door, the names tell you where the planets are. Finding them
  was the complaint this answers.
- No name is upside down, mirrored, or rolled, from anywhere in the room.
- Nothing reads as a hole in the stars where a name is.
- Standing on the line in the lined up arrangement, the names of bodies near
  each other overlap. Known, and the pictures from 2026-10-11 show it.
- From the doorway you can see down the line to the sun, with no pillar in
  the way.
- The planets are lit from the doorway, not only once you are among them.
  This is what dropping the sun took away and no test sees it. This is what the clearing above bought and it is the one thing in
  this spec that a picture settles and a test cannot.

**The columns keep off the line the bodies stand on.** Found while
photographing the names, and the likeliest half of why the planets were hard
to find in the first place. `behind::columns` cleared a column that would sit
on an orbit and cleared the way in, and nothing cleared the view. Lined up,
the bodies are strung along the diagonal from the sun to the way out, and a
forty five degree line through a square grid passes near a grid point every
stride and a half. Nine stand on it, from a fifth of a metre off to two
thirds, and one of them is four metres inside the door: from the door, and
again from out by Uranus, the whole model was behind a pillar.

The width of the corridor is not a judgment call, which is the good part. The
nine that are on the line are all within 0.7 m of it and the nearest one that
is not is at 4.46 m, so every width from one metre to four drops exactly the
same nine of the three hundred and sixty eight. `behind::CLEAR` is reused rather than a fourth number
invented, and `behind::tests::the_line_is_cleared_by_a_wide_margin` is what
says how much room there is to move it in.

The whole line is cleared, sun to door, and not only the seventy nine metres
the planets take. The ones past Neptune are nearest the door and so the ones
that stand in front of everything.

This says nothing about the orbiting arrangement, where the bodies are off the
line by their own distance from the sun. A column between two orbits is what
the grid is for, and the orbit rule already keeps one off a body's path.

## Out of scope

**Elliptical orbits, inclinations and real positions.** Circles, coplanar, at
their mean distances. The eccentricity that would show at this size is
Mercury's, and the inclination that would show is Pluto's, and Pluto is not
here either.

**Pluto, the asteroid belt and the Kuiper belt.** The belt is a good idea and
is a different piece of work.

**Phobos, Deimos and the rest of the small moons.** Below the scale.

**Real constellations.** See the stars, above.

**Gravity.** The planets are placed, not simulated. A solver here would be a
solver producing numbers nobody can check against a number they know.
