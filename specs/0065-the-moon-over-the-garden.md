# 0065 The moon over the garden

**Status:** implemented
**Date:** 2026-11-12

## Goal

The moon has a face. It is worked out rather than painted: highlands under a
saturated crater field, the near side's real seas flooded over them, the ray
systems of the young craters, and the sun somewhere off to one side so that
any of it casts a shadow.

## Why

**Jake, 2026-11-11**: there should be an entire room dedicated to a highly,
extremely detailed moon of the earth including atmosphere, so clouds, fog,
stars. Then, on being asked which way round it stood: the idea is for the moon
to be off in the sky from earth, where there is atmosphere and clouds.

**It is not a room, and it cannot be one.** The moon is drawn where
`Room::out_of_doors` is true, which is the garden, the field, the bridge and
Valhalla's doorstep, and nowhere with a lid. Every blank wall the walk has
found is under a lid. So the first half of this is the face, which pays off in
all four places at once; whether a court is worth building to frame it is a
separate question and is not this spec.

## What was measured first

A spike on 2026-11-11 drew the face on the CPU at several sizes, and the
garden was photographed to see what the engine actually does with it.

- The moon's disc is 1483 pixels of a 900 by 720 frame: 43 across.
- Its blue channel sat at 255 on 1407 of those 1483. At `aim::MOON`'s flat
  multiplier of 28 against a night fill of 0.09 blue, a third of the face's
  range was gone before anything was drawn on it.
- At 43 pixels not one crater reads. The seas are the whole of what is there.
  Craters start reading at about 180.
- A full moon is a flat disc at every scale. The moon scatters light back the
  way it came, so at zero phase nothing on it casts a shadow, which is why
  the real one looks pasted on.

## Behavior

**Where it is.** Unchanged: 230 metres off the eye, in the direction
`moon::at` gives, which is the sun's own place turned half a day. It hangs off
the eye like the sky does and is drawn only out of doors.

**How big.** 80 metres at 230, which is 20 degrees across and comes to 188
pixels in a 900 by 720 frame. Four times what it was. This is the number to
change and the one thing in here that is pure judgement: at 43 pixels no
crater reads at all, at 188 the terminator and the large craters do, and a
moon the size it really is would be 2 pixels.

**Which way it faces.** The ball is turned so the same face keeps to the eye,
by the shortest turn, with no up to hold the roll to. It had no rotation at
all before, so the hemisphere pointed at you swung round through the night.
With nothing on the face nobody could tell.

**What is on it.** A 2048 by 1024 picture of the near side, baked once at load
in about 540 milliseconds, in the order the ground happened:

- highlands, as fractal noise, pale and rough
- 45,000 craters on a power law from 6 kilometres up, each with an age: an
  old one is a shallow soft bowl with no rim and a young one has a rim,
  terraced walls, a central peak and bright ejecta
- the seas, flooded over all of that, dark and smooth, their shores taken off
  the union of seventeen basins so that they run into each other, with
  wrinkle ridges on them
- the craters that fell after the seas, which are thin on them
- twelve craters put where they really are, four of them young enough to throw
  rays

**Only the near side.** The moon keeps one face to the eye and a ball of this
size at this distance shows the cap within 80 degrees of the middle of it, so
nothing past 100 degrees round is worked out at all. That is 44 percent of
every pass over the picture, and it is why 2048 is affordable.

**How it is lit.** The shading is baked into the picture, because a mesh here
carries a colour and a texture and nothing that says which way its surface
tilts. Lommel-Seeliger with an eighth of Lambert in it, which is what makes a
full moon a disc rather than a ball and keeps Lambert's dark ring off the
limb.

**What lands on screen.** `aim::MOON` is now what the lit face comes to rather
than what it is multiplied by, through `garden::gives_off`, so all three
channels have their range.

## The three lies

Each of these is here because the truth is worse to look at, and each is
written up where it is told.

- **The size.** Twenty degrees instead of half of one. It was already five
  times life before this.
- **The phase.** `moon::at` hangs the moon opposite the sun, so it is full
  every night and a full moon has no relief at all. `SUN_OFF` puts the sun 60
  degrees round anyway. The terminator never moves, because the picture is
  baked once.
- **The dark side.** Lit at `EARTHSHINE`, a thirtieth, because without it the
  unlit part is a hole cut in the sky at a sharp edge.

## The bug worth recording

Four bakes were looked at before anybody noticed that **not one crater had
ever been drawn**. `crater` worked out for itself where on the picture a place
on the ball falls, and its arithmetic named the antipode, so every footprint
was laid over the far side and everything inside it was too far from the
crater to be touched.

Both guesses at the cause were wrong and both were plausible. The picture was
doubled and the crater count raised sevenfold, which drew the same face. Then
the sun was moved twice, which did bring relief out at the terminator, so the
lighting looked like the answer. Two measurements were written down as
findings off those runs and both had to be struck.

What was missing was a test that a place on the picture and a place on the
ball are the same place. There is one now, and the inverse is written once in
`moon::uv` instead of twice.

## Acceptance criteria

- A place on the picture and a place on the ball are the same place, both ways round. — `moon::tests::the_picture_and_the_ball_are_the_same_ball`
- A crater is cut into the ground where it was put. — `moon::tests::a_crater_leaves_a_hole_where_it_was_put`
- East points east and north points north. — `moon::tests::east_is_east_and_north_is_north`
- The seas read darker than the ground around them. — `moon::tests::the_seas_are_darker_than_the_highlands`
- The face is not one shade. — `moon::tests::the_face_is_not_one_shade`
- The side away from the sun is dark and the side under it is lit. — `moon::tests::the_sun_leaves_a_dark_edge`
- The dark side is still a body rather than a hole in the sky. — `moon::tests::the_dark_side_is_not_a_hole`
- The same face keeps to the eye at every hour. — `moon::tests::one_face_keeps_to_the_eye`
- What is asked for is what lands, with nothing clipping. — `moon::tests::the_face_lands_at_what_it_is_asked_for`
- It is up at midnight and gone at noon. — `moon::tests::it_is_a_night_thing`
- The moon over the garden matches its reference. — `tests::the_moon_is_drawn`

### Verified by hand

- The seas read as the moon's rather than as blobs. Run with `AT=0.75` and
  look up from the garden.
- The craters read at the terminator and not in the flat light, which is
  correct and is also the reason the phase is a lie.

## It is in the film, 2026-11-12

Jake: the arcade clip should show the moon. It does now. Spec 0049's route
already stood in the garden's way out and looked west at Bifrost, which is
the one shot on it that can see a sky, and it gained a second look from the
same place, up at the moon. The film runs at its own hour and its own day
length so that the moon is up and in the west for the whole of it, which
spec 0049 explains and `tour::FILM_AT` holds.

## Out of scope

- **A room for it.** The moon cannot be seen from under a lid, so a room that
  holds it is a court with no roof. That is a separate build and a separate
  decision.
- **A phase that changes.** It would mean baking a picture per phase or
  lighting the ball in the engine, and the engine has no normal map.
- **The far side.** Half the picture is never drawn. It is generated anyway
  because the ground is one field and splitting it would be two accounts of
  one thing.
- **Clouds and fog over it.** Spec 0019's storm already drifts across the
  dome; whether it crosses the moon is not answered here.
