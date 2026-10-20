# 0029 A corridor with a room for every number

**Status:** implemented
**Built:** 2026-10-20, the same day, and short of what this spec says in one way that matters. See "What this shipped short".
**Date:** 2026-10-20
**Amended:** 2026-10-20, twice, both on the day it was written.

Jake, having read the first draft: where is the hotel. It specified a corridor
and called it a hotel, which is the fault aleph has and which this was
supposed to be the fix for. That is "What you arrive in" below.

Then, having been shown the walk: it is unfindable and no hints lead to it.
That one took the whole placement with it. See "Why it is off the gallery",
which replaces an argument for putting it behind the wall.

**Unblocked** the same day: light under the door is what stands for a guest.
Jake's call, and it is cheaper and carries further than what this spec had
proposed.

## Goal

A second door in the far wall of the hidden room, opening onto a corridor of
numbered doors that does not end. It is aleph's hotel, which was built as a
game on 2026-10-04 and parked the same night, put where it actually belongs:
not in a cabinet, because it is not a game, but in the wing of this building
that already exists to make a quantity too large to picture into something you
can walk at.

## Why it is off the gallery

Spec 0001 gives the hall a cabinet per game. aleph is not a game and its own
spec says so in its first paragraph, so a cabinet that launched it would be a
cabinet that lies about what is behind it.

**It goes through the gallery's west wall**, which is spec 0022's room across
the end of the aisle.

### What the first draft said, and why it was wrong

~~A second door in the far wall of the hidden room, beside spec 0017's.~~

Struck 2026-10-20, on two counts, both found by looking at the building
instead of reasoning about it.

**Nothing leads there.** The argument was that spec 0011's wing is where a
quantity too large to picture already lives: spec 0015's orrery, and spec
0018's true scale through the far door, Neptune thirty three kilometres out.
That is true and it is not the only thing that is true about the wing. It is
also where things go to not be found, and spec 0020's own goal says so in as
many words: the wing is "most of what has been built in this building and the
part nobody finds."

The walk is worse than that sentence. The way in is signposted and works, a
lane worn across the carpet with the gap it runs to visible in the same view.
What has no hint of any kind is the crossing. One step inside, there is no
horizon, no wall, no direction, nothing but Neptune's label floating in black
and a scatter of columns. A hundred and fifty nine metres of that, and the
far wall is a hundred and seventy seven metres off against a far plane of a
hundred, so the way on is not merely unmarked, it is not drawn.

The solar system was designed around that cost and is worth it anyway. A
hotel cannot be. A thing nobody can find is a thing nobody arrives at, and
arriving is what a hotel is.

**And the far wall is already spoken for.** From the sun, that wall is a
colonnade running away to one lit rectangle dead centre, and the rectangle is
a galaxy in a pale frame. It is the vanishing point of the longest sightline
in this building. The draft said "a pair is the point". It is not: a second
opening beside it does not read as a choice, it turns the one composed image
in that room into two holes in a wall.

### What the gallery has that the wing does not

You cannot miss the gallery. You start in the doorway under the sign, the
aisle runs away in front of you, and this is what it runs to. No hint is
needed because nothing is hidden.

And the pair is already half built there. The **east** wall has the doorway
to spec 0024's back room, lit, with the test rig's bench visible through it.
The **west** wall is brick, blank, with a spotlight already falling on it, and
it is the most conspicuous empty surface in the arcade.

That also sorts what each side is for. The back room holds the building's own
instruments, things it uses on itself. Opposite it belongs the other kind of
thing, which is not a tool but an arrival.

### Space is not the constraint

The first draft sited this around sixteen thousand kilometres of corridor
needing somewhere to go, and concluded only the wing was long enough. That is
reasoning about a building as though it were made of land. It is not. The
corridor does not have to fit anywhere. It needs a door in a wall people
reach, and then it runs west through space nothing is using.

What is actually west, measured rather than assumed, with the gallery's west
wall at x -3.40 running z -14.55 to -8.30:

| | |
| --- | --- |
| the garden, the baths, the back room, the wing, the tesseract | nothing across that line at all |
| the hall | only its own shell, the wall and the floor it straddles |
| the cellar | 10.00 m west, and below: y -3.30 to -0.15 |

So at the gallery's own level the way west is clear, and the one thing to
watch is underfoot. **The cellar runs 10.30 m of itself under that line,
starting ten metres out, and its top is at y -0.15 against a gallery floor at
0.** Fifteen centimetres. A corridor floor slab of this building's usual 0.3
goes straight through the cellar's ceiling.

That is the ledge fault, which this building has had before and which no
amount of looking finds: you walk and your feet rise, or you stop dead on a
face at the walking surface. It is arithmetic and it belongs in a test before
anything is drawn.

## Behavior

### The doorway

In the gallery's west wall, opposite the back room's. The two are a pair and
should be built as one thing so they cannot drift: a doorway either side of a
room, at the same height, in walls the same thickness.

The gallery's spotlight already falls on that wall. A doorway cut into a lit
wall is a dark rectangle in a bright field, which is as much signposting as
this needs and is the opposite of the wing, where the way on is not drawn at
all until you are on top of it.

### What you arrive in

Not the corridor. You come through the door into the place the corridor runs
away from, and it has to say what this is before you are in it.

There is no such place in aleph and that is the hole. `desk.rs` draws nothing:
its own first line says it is "kept apart from the drawing, so a night can be
played without a window", and what it holds is the dials, the rule and how far
each door has swung. The only other trace is `corridor::eye`, a camera parked
two and a bit rooms back so rooms 1 and 2 are both in frame. That position is
called the desk and there is nothing at it. No counter, no lobby, no way you
came in. The hotel is one sentence in that spec's goal and a module name.

So: a desk, standing where that camera stands, with the corridor running away
from it. A counter you walk up to and stand behind, which is the side of it
the clerk is on and the side this is played from.

The furnishing rules of this building apply and are not a detail. A fitting
needs a reason, a box is a placeholder and Jake will say so, and the room is
twice as small as whoever builds it thinks. Everything turned comes off
`cellar::turned`.

### The corridor

aleph's own numbers, because they work and because two lists of one thing
drift:

| | |
| --- | --- |
| rooms apart | 1.5 m |
| corridor across | 3.2 m |
| corridor tall | 2.4 m |
| door | 1.05 by 1.9 |
| a door swings to | 1.75 rad, over 0.4 s |

Rooms run 1, 2, 3 and on without end, odd down the left and even down the
right. That split is not decoration: the rule that doubles everyone sends
every guest into an even room, so the entire left wall opens in one move, and
it is the clearest picture this thing has of freeing infinitely many rooms.
Every door carries a plaque with its number beside the frame.

A shut door is a room with somebody in it. An open one is free and dark.

### How long it is

There is no end and the building does not pretend there is one by moving the
corridor while you are not looking. It is built long and the numbers are real.

Where it actually stops is f32. Past 16,777,216 metres a float cannot tell one
metre from the next, which at 1.5 metres a room is **11.2 million rooms**, and
at this building's walking speed of 4.2 m/s it is 46 days of holding W. Spec
0051's eye-relative rendering is what buys that, by keeping the arithmetic
near the camera instead of near the origin.

Eleven million is not infinity. Say so on the plaque at the threshold rather
than hiding it, the way spec 0015 writes down its lie.

### The rule, and watching it travel

This is the half aleph could not have and a walkable corridor can.

In the game, applying a rule swings the doors and the result appears. aleph's
own parked note calls that the lesser half: the guests do not travel from room
n to room a·n + b, so the transformation itself is invisible.

Here you are standing in the corridor when it fires. The doors do not all
swing at once. The change starts at room 1 and runs away from you down the
corridor at a speed you can watch, so what you see is a wave going somewhere
you cannot see the end of. Stand near the door and it leaves you behind. Stand
far down and it arrives.

## What stands for a guest: light under the door

**Light under the door.** Jake, 2026-10-20, which unblocks this.

A shut door with a line of light under it is somebody in. A door standing
open on a dark room is nobody. Nothing is drawn that is a person, the paradox
stays about people who are never shown, and the building keeps its record of
never having drawn one.

It is also the right thing for this hotel in particular. aleph's own spec is
called "The night". A corridor of shut doors at night with light under some of
them is what a hotel is when everybody is in.

### Why it is cheap, which matters at this length

It is not a light. It is a bright strip on the floor at the threshold, drawn
and not lit.

This engine grants two shadow casting point lights and a short list of plain
ones, per `lighting::MAX_SHADOWING_POINT_LIGHTS`, so a lamp in every occupied
room is not a thing that can be asked for at forty doors, let alone at eleven
million. A strip is one more instance per door in a corridor that is instanced
already, and this building's own habit covers the rest: colour is an unclamped
multiplier and anything past one glows with no light falling on it, which is
how the neon works at 3.2.

### What it buys that the doors do not

**The corridor stays readable after the doors stop being.** A door at the far
end is smaller than a pixel. A bright strip against a dark floor is still
there, because what carries at distance is contrast and not size. So the
corridor reads as a column of lights running away, and the thing you watch
when the rule fires is a wave of them going out.

That is the half aleph never had, and it turns out the light is what delivers
it rather than the doors. The doors are the near-field version of the same
fact.

### Two ways it can go wrong, both of which this building has had

**Laid in the floor rather than on it.** A glowing quad coplanar with the
floor is two faces in one plane, which is the fault this building has fought
seven times, and it speckles as you move. Spec 0012 says it of a border and
the answer is the same: lay it on, not in.

**A long shallow edge at a glancing angle.** Spec 0012 found that is the worst
case there is for putting a straight line on a grid of pixels, that
multisampling shades the pixel at each drop and cannot move the boundary, and
that eight samples took eight per cent off what four left. Its fix was to drop
the contrast until the staircase stopped reading.

**That fix is not available here**, because the contrast is the point: a dim
line under a door is not light under a door. The near strips are the ones to
watch, since the far ones foreshorten to a mark. Whoever builds this should
expect to measure it and should not expect spec 0012's answer to work twice.

### What this does to the desk

The desk stays. It is where you arrive, where the rule is set, and what the
corridor runs away from, and none of that was ever about the guests.

~~Give the desk a board of keys, one hook per drawn room. A key on its hook is
a room standing empty, a key gone is somebody upstairs.~~

Struck 2026-10-20, the day it was proposed. It was an answer to the guest
question and the guest question has a better one. Keeping both would be two
accounts of who is in, and two lists of one thing drift, which is the fault
under most of the others in this building.

A board of keys is still good furniture for a hotel desk and may well end up
on the wall behind it. What it must not be is a second record of occupancy.

## Acceptance criteria

Arithmetic, which is what this building tests. None of it is written.

- Odd rooms are on the left and even on the right, all the way along. — `aleph::tests::the_sides_alternate`
- Doubling frees every room on the left side and no room on the right. — `aleph::tests::doubling_empties_the_odd_side`
- A body your size walks the corridor from the threshold with doors open on
  both sides, which is the case that fails: two facing doors at 1.75 rad leave
  1.13 m of a 3.2 m corridor, against a body of 0.9 and this building's usual
  1.05 of clearance. — `aleph::tests::walked_with_every_door_open`
- The wave moves away from room 1 rather than arriving everywhere at once. — `aleph::tests::the_change_travels`
- Every shut room is lit under its door and every open one is not, so the
  light and the door never disagree. — `aleph::tests::the_light_says_what_the_door_says`
- No strip is level with the floor it lies on. — `aleph::tests::no_strip_is_in_the_floor`
- The light falls off rather than stopping, which is three quads over each
  other and not one. — `aleph::tests::the_light_falls_off_rather_than_stopping`
- A door is solid, so you cannot walk through one, and an open one takes up
  more corridor than a shut one. — `aleph::tests::a_door_is_solid`
- And it swings into the corridor rather than through the
  wall. — `aleph::tests::an_open_door_swings_into_the_corridor`
- The corridor runs away from the lobby rather than doubling
  back. — `aleph::tests::the_corridor_runs_away_from_the_lobby`
- Every strip lies on the corridor's floor rather than through a
  wall. — `aleph::tests::every_strip_is_on_the_corridors_floor`
- Nothing travels unless a rule is applied. — `aleph::tests::nothing_travels_on_its_own`
- Room numbers are right at the far end as well as the near one, where a float
  is at its worst. — `aleph::tests::the_last_room_is_numbered_correctly`
- The gallery has a doorway either side and the two are one account, so the
  pair cannot drift. — `room::tests::the_gallery_has_a_doorway_either_side`
- Nothing of the corridor is drawn inside another room, which is the fault
  this building has fought seven times. — `aleph::tests::nothing_of_the_corridor_is_in_another_room`
- The corridor's floor clears the cellar's ceiling, which it does not at this
  building's usual slab: 0.15 m of headroom against 0.3 of slab, from ten
  metres west. — `aleph::tests::the_corridor_clears_the_cellar`
- You walk from the gallery into the corridor without your feet rising, which
  is what a ledge feels like and what no amount of looking
  finds. — `aleph::tests::walked_from_the_gallery_into_the_corridor`
- You can stand at the desk and see room 1, which is what the camera in aleph
  was placed for. — `aleph::tests::room_one_is_in_view_from_the_desk`
- The doorway here and the back room's are one account, so the pair cannot
  drift. — `aleph::tests::the_two_doorways_are_one_account`

### Verified by hand

- You can tell it is a hotel before you are in the corridor.
- The doorway reads from the aisle, without anybody being told it is there.
- The corridor reads as having no end rather than as being long.
- The lights carry further than the doors do, which is what this choice is
  for. Walk until the doors stop reading and check the strips still do.
- The near strips at a glancing angle, for the staircase spec 0012 describes
  and cannot fix twice.
- The wave is worth standing still for.
- Walking through a doorway with the door swung open does not catch on a
  shoulder, which 1.13 m says it should not and which this building has been
  wrong about before.

## What this shipped short

**The doors end.** This spec says the corridor has no end and that the
numbers are real as far as a float carries them, which is 11.2 million rooms.
What was built draws forty and runs its shell four rooms past that, so you can
walk to room 40 and find a wall. In a hotel whose premise is that there is
always more of it than you can see, reaching the last door is the one thing
that must not happen. Jake walked to room 36 and said so.

The cause is that the shell cannot simply be made long: at a hundred and ten
metres it put the reach fill over its cell budget and two of this building's
tests came back saying "the fill reached nothing at all". What it wants
instead is rooms drawn around wherever you are standing rather than a fixed
forty, and that is spec 0030's.

**And the demonstration is not here.** What this spec built is a corridor
whose doors can be made to open in a wave. Jake, having walked it: opening and
closing doors does not really demonstrate the point. He is right, and it is
worth writing down why, because the first fix for it was wrong too.

A key that checked a guest in and out of a room was added and taken back out
the same hour. It answered "there is no way to add a guest" with a light
switch. A door that can be shut is not a paradox.

The paradox is that the hotel is **full**, a coach arrives, one rule moves
everybody, and afterwards the hotel is full again with more people in it and
nobody has left. What is built stops at "the odd side emptied", which is half
of it and the half that reads as arithmetic. The missing half is the arrival
and the refilling, and it is spec 0030.

## Out of scope

**The dials.** aleph's third parked reason is that they are a menu with extra
steps: three right answers and a player works them out in a couple of minutes.
Nothing is gained by carrying that in here. What the rule is and how it is
chosen in a building you walk is its own question and belongs in its own spec.

**aleph the crate.** It stays parked, its manifest stays renamed, and it stays
out of this project's gate. This is the corridor rebuilt here, not the game
launched from here.

**Guests.** Named above because it blocks this, not because this spec answers
it.

**The hidden wing.** Nothing here touches it. Spec 0017's door keeps the far
wall to itself and stays the one lit thing at the end of that sightline.

**Moving the cellar.** The headroom above it is fifteen centimetres and the
corridor has to deal with that, by running over it on a thinner slab, by
stepping, or by leaving the floor's height alone and taking the cellar's
ceiling down. Which of those is a question for whoever builds it, and the
test is the same either way.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
