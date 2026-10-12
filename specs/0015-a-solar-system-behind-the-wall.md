# 0015 A solar system behind the wall

**Status:** implemented
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
| **bodies and their moons** | 185,693 km | the Earth is 6.9 cm across |
| **planets from the sun** | 56,769,000 km | one AU is 2.64 m |

The orbit scale is **306 times** the body scale. That ratio is the first lie
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
Callisto is still thirteen Jupiters out and Neptune is still thirty times the
Earth's distance. Nothing was made less true. (The Moon was thirty Earths out
too, and is ten now; that was paid for separately, below.) Two things improved on their own: the ratio between the scales fell
from 167.8 to 101.9, the closest they have been since the bodies grew, and
Mercury's clearance of the sun went from 11.8 cm to 51.6. The ratio went back
out to 306 the next day; the clearance stayed.
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
158.7 m across. At seven and a half metres, its true size on the body scale,
it would be seven hundred across.

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

**Big enough to notice.** Jake stood among the planets and said you only
notice one because of the name over it, which is a model failing at its one
job. The Earth was 2.3 cm.

`BODY` is three times what it was: the Earth is 6.9 cm, Jupiter is 77 cm,
Mercury is 2.6 cm and Mars is 3.7, and none of them is a speck any more.

**Only this changes how big a planet looks**, which took a wrong turn to
learn and the wrong turn is worth keeping. Growing the bodies and the orbits
together was tried first, and it doubled every measurement in the room and
changed the picture not at all: you then stand twice as far from everything,
and a planet subtends exactly the angle it did. What a planet looks like is
its size against the distances it sits among, and that is `BODY` against
`au`, nothing else. The menu this spec's author put in front of Jake offered
the uniform scaling as a way to bigger planets. It is not one.

**What it cost.** The ratio between the two scales, the model's first lie,
goes from 102 to 306. And the Moon: its orbit is on the body scale, so at
three times the bodies it would be three times as wide and halfway to Mars,
which is the reading `MOON_SHARE` exists to stop. `orrery::MOONS_CLOSER`
holds the moons to a third of their true distance on this scale, so the Moon
sits exactly where it sat, 69 cm from the Earth, and reads as ten Earths out
rather than thirty. Ten still reads as a moon well clear of its planet, which
a cartoon's two does not. Within the moons nothing moved: Io against Europa
against Ganymede is exactly what it was.

**What it did not cost.** The room, the walk, `au`, and the sun. `au` comes
off the Moon's orbit and the Moon's orbit has not moved, so the room is the
same 159 m and the run to the sun is the same 13 seconds. `SUN_SHRUNK` came
down by the same three, from two fifths to two fifteenths, so the sun is
still a metre across and Mercury still clears it by 51.6 cm. Growing the sun
with the planets would only have crowded the frame and eaten that clearance.

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

**The shortest way round, and that was the bug.** `stands` multiplied
`angle_at` by the swing, and `angle_at` counts from the moment the building
opened and never comes back. So the sweep was however far the planet had
travelled since you walked in, crammed into three seconds: two turns of
Mercury half a minute in, twenty one at five minutes, sixty two at fifteen,
and worse for as long as the building stayed open. Jake watched it and said
switching modes was a bit chaotic. It was a planet whirling, not a line
fanning open.

`orrery::shortest_turn` wraps the angle into a half turn either way. Same
place, at most half a turn to reach it, whatever the clock says.

The test above this, that every planet keeps its own radius, passed
throughout and could not have done otherwise: a planet twenty turns round its
own circle is still exactly on it. It is the test that looked like it covered
the sweep.

**And then the wrap, which was the second wrong shape.** `shortest_turn`
flips from half a turn one way to half a turn the other, and partway through
a swing that moved the planet by up to the width of its orbit: 1.4 m of
Mercury in a frame where it could honestly travel 4 cm. Mercury passes the
wrap every fourteen seconds, so a three second swing met it often.

The shape that works carries one number per planet. Lined up is not an
angle, it is every multiple of a full turn at once, and the jump was the
code picking a different multiple halfway through. So `orrery::resting_near`
picks one at the ends of the swing and `orrery::stands` holds it across.
Both ends are safe to pick at, which is what makes this cheap: at rest the
planet sits exactly on it and a whole turn either way is the same place, and
swung fully out it carries no weight at all. In between nothing may change,
and nothing does. A swing turned round partway keeps the number it started
with, which is right: it is going back to the end it came from.

`shortest_turn` is gone with it.

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

A name is a quad with the name cut out of its ground, per the engine's spec
0047. It went in that way, moved to a black plate, and came back, and each
leg had a reason that was true when it was given.

**It went to a plate** because a cut-out name was gone at any distance:
thirty metres from Saturn, with the columns taken out of the build and the
multiplier run up to 24, there was no name over Saturn at all. And because a
plate read against the sun as well as against the dark, while being invisible
against a sky that is black anyway.

**It came back** because the first of those was never a property of cut-out
text. A cut-out surface wrote no depth anywhere, so whatever drew after it
painted over it however far behind it that was, and out at Saturn the thing
drawing after it was the star wall. The engine's spec 0047 has the whole of
it, including the note that blamed mip levels and was wrong. A cut-out word
stands up at any distance now, checked at twenty two metres where there had
been nothing.

So the plate went back to being what it always was: a black rectangle over
whatever it crosses. Free against the sky, which is what made it look free,
and a hole punched in Saturn's rings and in the planets. Jake saw the rings
first and called them clipped, which they were, by a label eleven metres
behind them.

What giving it up costs is the second reason: against the sun's own
blown-out metre of white a name is white on white. Milder than it sounds,
because the name's colour is cool and the sun is warm, so "Earth" over the
disc reads as pale blue on cream rather than disappearing. Trading a name
that is weak in one place for plates that are gone everywhere, when the
place in question is the one body in the room nobody needs told the name of.

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

It does not fix the rest on its own, and the swing showed how little it
fixed. Every name is the same size on the glass and every body sits at eye
height, so the names lie on two rows and slide through each other as you
move; swinging into the orbits they cross wholesale and the rows become one
band of letters, "ars EarVenMercury" and "Ga.medeCalli". Jake watched a
recording of the swing and called it chaotic, and the planets whirling was
only half of what he was looking at.

**The moons run on their own clock, and that was the janky one.** The moons
were on the planets' clock, which put the Moon round the Earth thirteen times
an Earth year and was written down here as the one fact about the Moon that
everybody has. The arithmetic was right and what it bought was a blur: a year
is sixty seconds, a month is a three hundred and sixty fifth of a year, and
Io went round Jupiter three and a half times a second.

`orrery::MOONS_DAWDLE` is twenty four, the third lie in this model after the
two scales and the sun's size. What it costs is the fact it was written for:
the Moon now goes round the Earth about half an Earth year. What it keeps is
the better one. Io, Europa and Ganymede come out at 7, 14 and 28 seconds,
which is the 1:2:4 those three are really in, and at those rates you can
stand and watch it happen. A resonance you can see beats a ratio you cannot.

**So the names are laid out on the glass, nearest first.**
`orrery::laid_out` projects each one, takes them in order of distance, and
drops any that would land on one already kept. Nearest wins because the near
body is the one you are looking at and the one whose name you can check
against a ball you can see. A name dropped is a name you get by taking a
step, which is how a planetarium behaves and is better than a smear that
names nothing.

It is cheap because `LABEL_SUBTENDS` already made every name the same size on
the glass: the distance cancels out of the height, so the layout needs only
where each name is and not how far off. Rectangles and not circles, because a
name is long and thin and a circle round one would reserve a square of sky it
does not use, and the width comes off each name's own picture so "the moon"
keeps more room than "Io".

Behind you and not wholly on the glass are both thrown out before anything is
compared. The first of those is not tidiness: a point behind the eye comes
back through the projection mirrored into the middle of the glass, where it
would take the place of a name that is actually there.

Wholly on, and not merely touching. The test threw out only a name that was
entirely off, so one straddling the frame was kept and drawn sliced:
"Ganymede" cut at the right edge reads as "anymede", which is worse than no
name because it looks like a name. Gone, it comes back the moment you turn a
few degrees.

A name's half width is divided by the aspect, which it was not. `tall` is a
fraction of the glass's *height* and the half width is across its width, so
on a window a third wider than it is tall every name was reserving a third
more room than it used and the layout was throwing out names that fitted.
Invisible in every test here before this, because they all run at 1:1, which
is `Camera::new`'s own viewport and the one aspect where it does nothing.

Two things steady it, and the measurement is what says they are worth having.
Standing still and touching nothing, the chosen set changed this many times a
second: 2.2 plain, 1.5 with `STICKS`, the hysteresis that lets a name already
up hold its place on a smaller gap, and 1.0 with `worth` as well, which ranks
the sun first, then planets, then moons, so nothing a moon does can take a
planet's name.

A dwell time was tried on top of those and is not here. It was put in while
the moons were still whipping round, took 36 changes a second down to 7, and
looked like a fix; the gaps between the remaining changes then sat at exactly
the dwell, which is a strobe slowed rather than a strobe stopped. Once
`MOONS_DAWDLE` was found it bought nothing at all over the other two, and it
cost a name overlapping for four tenths of a second before giving way. The
order the three were tried in is the wrong order, and the numbers above are
the record of it.

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

**And then not over the cap at all.** Ranking was the right answer to the
wrong question. The lamps that were spilling could not light anything you
could see: behind you, in another room, or a hundred metres off with a reach
of nine and a half. So the building throws those away before it ranks what
is left, by `Camera::frustum` and the engine's spec 0050, which is exact
rather than approximate because a lamp gives exactly nothing past its range.

A hundred and thirty three become twelve to fifty three depending on where
you stand and which way you look, against sixty four carried. Ten seconds of
the arcade now says nothing at all about lamps, where it was saying 8,600
lines a second before the counting went in and one line a frame after.

The ranking stays underneath as the fallback, so a corner nobody has
measured still spills in the right order rather than at random.

## Acceptance criteria

The model:

- Every body's ball is its real diameter on the body scale. — `orrery::tests::every_ball_is_true_against_every_other`
- And every distance is its real distance on the orbit scale. — `orrery::tests::every_distance_is_true_against_every_other`
- The two scales are a hundred times apart, which is the first lie, pinned. — `orrery::tests::the_orbit_scale_is_a_hundred_times_the_body_scale`
- Mercury's surface clears the sun's, which is what set the sun's size. — `orrery::tests::mercury_clears_the_sun_and_would_not_with_a_bigger_one`
- Every moon is outside its planet. — `orrery::tests::every_moon_stands_outside_its_planet`
- And inside the next planet's distance. — `orrery::tests::no_moon_reaches_the_next_planet`
- The Moon takes half the gap to Mars, which is what sets the orbit scale. — `orrery::tests::the_moon_takes_half_the_gap_to_mars`
- Saturn's rings start outside Saturn and the Cassini division falls between the two bright rings. — `orrery::tests::saturns_rings_start_outside_it_and_the_gap_falls_between`
- The planets are in order out from the sun. — `orrery::tests::the_planets_are_in_order`
- Venus turns slower than it goes round, which looks like a fault and is not. — `orrery::tests::venus_turns_slower_than_it_goes_round`
- The room holds the disc. — `orrery::tests::the_room_holds_the_disc`

The two arrangements and the swing:

- Lined up is lined up. — `orrery::tests::lined_up_is_lined_up`
- The swing leaves every planet at its own radius at every moment of it. — `orrery::tests::the_swing_keeps_every_planet_at_its_own_distance`
- And never jumps, from any clock, including across the wrap that used to make it. — `orrery::tests::a_swing_never_jumps`
- L swings it out and back. — `tests::l_swings_the_system_out_and_back`
- A swing interrupted halfway and sent back does not jump. — `tests::the_swing_turned_round_partway_does_not_jump`

The room:

- You can walk from the way in to the sun, and round it. — `behind::tests::you_can_walk_from_the_way_in_to_the_sun_and_round_it`
- The way out is signed, beside the slot and facing in. — `tests::the_sign_stands_beside_the_way_out_and_faces_in`
- The line at the top says which room you are in. — `tests::the_corner_says_the_room_you_are_in`

The names:

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
- The ceiling grid alone never fills the engine's lamp list, from anywhere, looking any way. — `behind::tests::the_ceiling_grid_stays_under_the_lamps_the_engine_carries`
- Every body has a name, no two share one, and every one draws something. — `orrery::tests::every_body_is_named_once`
- Two names that would land on each other are cut to the nearer one. — `orrery::tests::two_names_on_one_another_are_cut_to_one`
- And names with room to spare are all kept. — `orrery::tests::names_that_clear_each_other_are_all_kept`
- A name behind you cannot take the place of one in front. — `orrery::tests::a_name_behind_you_is_not_laid_out`
- A moon goes round slowly enough to watch, and the Galileans keep their 1:2:4. — `orrery::tests::a_moon_can_be_watched_and_the_galileans_keep_their_resonance`
- A name hanging off the edge is not drawn at all. — `orrery::tests::a_name_half_off_the_glass_is_not_drawn`
- A wider window fits more names, not fewer. — `orrery::tests::a_wider_window_fits_more_names`

### Verified by hand

**Nothing in the room is close enough to anything else to fight it**, per
spec 0048 of the engine. This had a line among the criteria above as though
it were arithmetic and it is not: 0048's check reads a drawn frame, so it
runs on the P key while you stand in the room and there is nothing for a test
to call. `tests::p_asks_for_one_coplanar_check` holds the key and says
nothing about the answer.


- Standing at the door, the names tell you where the planets are. Finding them
  was the complaint this answers.
- No name is upside down, mirrored, or rolled, from anywhere in the room.
- Nothing reads as a hole in the stars where a name is, and nothing reads as
  a hole in a planet or in Saturn's rings.
- A name over the sun's disc can still be read, which is the one place the
  cut-out is weak and the one this spec gave the plate up for.
- Swinging between the two arrangements, the names stay readable: fewer of
  them as bodies cross, and no two on top of each other.
- A name that goes, goes because something crossed it, and not every few
  frames. The churn is measured above; whether it reads as steady is not.
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
