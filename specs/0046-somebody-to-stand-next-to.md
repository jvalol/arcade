# 0046 Somebody to stand next to

**Status:** implemented
**Date:** 2026-11-01
**Built:** 2026-11-01, the same day it was written. Two things turned up in
the building that are not in the complaint and are named below: the legs
were cycling at twice the rate their own stride says, and a sine can never
plant a foot.

## Goal

Make the figures hold up close. Spec 0033 built them for a corridor seen from
one end in the dark, and spec 0038 put them in a lit room you walk into and
stand among.

Jake watched the imaginarium clip on 2026-11-01 and said they only vaguely
resemble a humanoid, that the joints are stiff and disconnected from the
body, and that the movement is very robotic. All three are in the code, and
one of them causes the other two.

## The ceiling this raises

Spec 0033's last line of scope reads:

> **Animation worth the name.** Legs that swap is the ceiling here. Anything
> beyond it is a character and this spec says not to build one.

That was right for what it was deciding. Its premise is in the sentence
before it: the bar was that a figure reads as somebody **at corridor
distance, in the dark**, without inviting anybody to walk up and look at its
face. Nothing in the building then let you walk up.

The imaginarium does. You arrive on the same floor as four of them, the room
is lit, and they pass within a metre of the camera on their way through a
door. The invitation 0033 was careful not to issue has been issued by the
room, so the bar moves with it.

**What does not move is the reason.** A bad human is still worse than a lamp,
and the thing that makes one is detail that promises more than it can keep. A
face at a metre is worse than no face at a metre. This spec adds joints and
takes nothing away from that.

## What is actually wrong

### There are no joints at all

A limb is a rigid lathe placed by translation. `people::swing` slides a whole
leg up to 0.34 metres fore and aft, `people::lift` raises it 0.07, and
nothing in a figure rotates except the figure. A leg at the end of its swing
is a stick standing a third of a metre clear of the hip it hangs from, which
is exactly what Jake saw.

It is not an oversight and the reason is written down in the code: rotating a
limb about its hip needs the mesh's origin at the top of it, flipping the
lathe to put it there reverses the winding, and a limb wound away from you is
drawn nowhere at all. Sliding the foot was what was left.

### One bone in each limb

No knee, no elbow, no ankle, no foot, no hand. A leg is one taper and an arm
is the same taper at 0.62 scale.

### The body does not rise

The torso sits at a fixed height for the whole cycle. A walk where the hips
hold still is a glide with the feet moving, and that reads as machinery
whatever the legs do.

### The legs run at twice the rate they are paced at

Found on the way in rather than complained about. `aloft::STRIDE` is 0.7 and
its own comment calls it how far one pace carries somebody, but the phase
put a whole cycle of both legs into each of them. A cycle is two paces, so
every figure in the building took two steps for each one it was moving at
and minced. It is a good part of what reads as robotic and it is one
character in the fix.

### What is already right, and must stay right

The proportions. The head is a seventh of the height, which 0033 paid for.
And the amplitude: the swing's travel is 0.68 against a 0.70 step, which is
the right number arrived at by hand. It survives as a derivation.

## Behavior

### A limb hangs from its joint and turns about it

`limb_profile` runs downward from the origin rather than up from the foot, so
the origin is the hip and the profile is still read in the same order. The
winding is unchanged, which is the trap the first attempt fell into.

The leg then turns about the hip and the arm about the shoulder, both about
the walker's own right axis. `swing` returns an angle. `lift` goes: a foot
lifts because the knee bends, not because the leg is raised.

### Two bones in each limb

Thigh and shin, upper arm and forearm, with one bend each off the same phase
as the swing. **The knee bends backwards only and the elbow forwards only.**
With no foot and no hand, that is the only thing in the figure that tells a
leg from an arm, so it is worth more than its cost.

### A planted foot does not move, and a sine cannot do that

This is the one that changes the most and was not in the complaint.

A foot on the ground does not travel. While the body covers a step, the foot
has to cover exactly one step backwards relative to it, at a constant rate.
A sine is only ever momentarily still, so no figure driven by one has ever
had a foot that stayed put, whatever its amplitude. **The stance is a
straight line.** The swing is the curve that joins its two ends, and the hip
angle is read off where the foot has to be rather than set.

That curve lands at the speed the body is going, because a foot arriving
with any speed of its own has to stop dead at the moment it touches, and the
plant is the one place in a walk the eye is looking. It leaves at rest
instead. A cubic made to match at both ends takes the foot three centimetres
further back while the knee has already lifted it ten, which reads as
dragging, and that was tried.

### The numbers come out of the step, not out of taste

The widest a straight leg gets is `asin` of half a step over the leg's
length, rather than a literal near a third of a metre that happens to agree
with it. The step is 0.7 and it lives in `aloft::STRIDE`, which is the wrong
place: how far a leg carries you is a fact about walking and not about the
imaginarium. It moves to `people::STEP` and the rooms read it. Two places
meaning the same number is how this building's faults start.

That makes the phase a function of ground covered, which the imaginarium
already does and the hotel corridor does not. `people::crossing` gives
`along` as progress through a crossing, so everybody takes `STEPS` paces
whether the move sends them three metres or four hundred. Under the old
slide that was a skate on top of a skate. Under a foot that is meant to stay
put it is simply wrong, so a walker carries the metres they have walked and
the gait reads those.

Nobody walks four hundred metres on camera, as it turns out. A crossing caps
what it covers at `WALKS * CROSSES_IN / TRAVELS_AT`, which is 6.3 metres,
and whoever is bound for room 1024 finishes the journey out of the window.
So the spread the fix is worth is about one to one and a half rather than
one to a hundred.

### The hips rise and fall twice a stride

Highest when the legs pass each other and the one holding you up is
upright, lowest at either end of its stance, where you are standing on the
long side of the swing. It is not a number to pick: the hips are the planted
leg's length times the cosine of its angle, so the rise falls out and comes
to about seven centimetres. It is the difference between walking and
gliding.

A foot's height off the floor falls out of the same arithmetic rather than
being set, which is what makes the planted foot land at exactly nothing.

### Feet and hands

End caps, small. A foot is most of what says which way somebody is facing
when you are behind them, which in the imaginarium you usually are.

### Nothing is per person

Every figure in the building runs off the same phase. No individual gait, no
idle motion, nobody slower than anybody else. That is where a character
starts and this is where the spec stops.

## Acceptance criteria

- The top of a leg stays at its hip through the whole cycle, and the top of
  an arm at its shoulder. — `people::tests::a_limb_never_leaves_its_joint`
- **Its control:** the lathe this replaces stood on its own foot, so turning
  it took the hip with
  it. — `people::tests::a_limb_stood_on_its_foot_comes_away_from_the_body`
- A planted foot does not move: through its stance the ground under it never
  changes. — `people::tests::a_planted_foot_does_not_slide`
- **Its control:** the sine this replaces does slide, by more than a tenth
  of a metre. — `people::tests::a_slid_foot_does_not_stay_put`
- A knee bends through the swing and an elbow through the stance, neither
  ever goes past straight, and both of them do
  bend. — `people::tests::a_knee_and_an_elbow_bend_in_opposite_halves`
- A foot leaves the floor only while it is going
  forward. — `people::tests::a_foot_lifts_only_on_the_way_forward`
- The hips reach their high once a step, with a leg upright under them, and
  the rise is big enough to
  see. — `people::tests::the_hips_rise_twice_a_stride`
- The legs swap rather than hold still. — `people::tests::the_legs_swap`
- Every part of a figure has a
  surface. — `people::tests::a_figure_is_not_nothing`
- Somebody walking in the corridor paces off the ground they cover rather
  than off the crossing, and the longest and shortest moves in one crowd are
  far enough apart to tell the
  difference. — `people::tests::a_long_move_takes_the_same_step_as_a_short_one`

## What standing next to one found

None of these is in the complaint and none of them was visible from the
reading. They came out of rendering a figure from two and a half metres,
which is the distance the spec is named for.

**Every limb was swinging mirrored.** `turn` is `rot_y(facing)` and this
building's yaw convention is `(sin yaw, 0, -cos yaw)`, which is that
rotation flipped in x. So `turn * rot_x` swung a limb the wrong way across
the body while the foot, placed off `ahead`, went the right way, and the
feet came off the legs. They turn about `side` now, which is the axis the
rest of the drawing already agrees with. Nothing caught this because
nothing before it rotated a limb at all.

**The shin still closed to a point.** Right when there was nothing under it
and wrong the moment there is a foot: a point over a flat sole reads as a
tile lying beside a leg. It has an ankle.

**The figures wore epaulettes.** An arm hung level with the shoulders put
the flat top of its lathe out over each one. The limb narrows into the
joint now, which is also truer of a hip than a flat cap was.

**And the arms moving out cost something elsewhere.** The imaginarium's
crowd test borrowed `ARMS_APART` as how wide somebody is, so widening the
arms by four centimetres tightened a test about people passing each other
by eight, and it failed. How far out to hang an arm and how much room two
people need to pass are two questions that happened to have one answer.
They each say their own now, `people::PASSES`.

### Verified by hand

- Stand in the imaginarium beside the trapdoor and let one walk past within a
  metre. This is the view the spec is for.
- Then the hotel corridor from one end, which is what 0033 was tuned for and
  what this must not spoil.

## Out of scope

**Faces and hands with fingers.** Still none, and for 0033's reason rather
than out of deference to it: at a metre a smeared face is worse than no face.

**Anything that varies between people.** One gait, no idling, no head turning,
nobody who looks at you. Each of those is a character and none of them is
asked for by a room you walk through.

**The player's own body.** Still the hall of mirrors' question, and still a
different one.

**Standing still well.** This is about the walk. A figure at rest is the same
figure with the phase held, and whether that reads is a separate question
nobody has asked yet.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
