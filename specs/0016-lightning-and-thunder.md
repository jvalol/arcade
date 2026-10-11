# 0016 Lightning and thunder

**Status:** draft
**Date:** 2026-10-11

Parked, not designed. Written down so the one idea in it is not lost.

## Goal

The garden's sky is already a storm. `garden::storm` draws it and `WEATHER`
turns it, and in all that weather nothing ever happens. A storm that never
breaks is wallpaper.

## The one idea worth keeping

**Thunder arrives late, and that is the whole exhibit.**

Everybody knows you count the seconds between the flash and the bang, and
almost nobody has watched a thing demonstrate it. Sound goes 343 metres a
second. Three seconds is a kilometre.

So a strike is **one number: how far away it is**, and that number sets
everything else:

- how long after the flash the thunder arrives, at 343 m/s
- how bright the flash is
- how loud the thunder is
- and what it sounds like. A near strike is a crack with the high end intact.
  A far one is a rumble, because the air has eaten the top of it on the way.

One number into four things that all move together is the kind of thing this
building is for. Get that right and it needs nothing else.

## What it would take

Most of the pieces are already here.

- **The flash.** A brief rise in the sun's light and the ambient, plus the
  sky's own picture going white for a frame or two. The engine has one sun
  and a game may change it per frame, so this is a number going up and coming
  down.
- **The thunder.** Made rather than recorded, per spec 0044: noise through a
  decaying envelope, low passed harder the further off the strike is.
- **The delay.** The game already tells the engine where the listener is, so
  the arithmetic is one division.

## The thing that needs deciding first

**A strike two kilometres away has nowhere to be.** The garden is sixteen
metres across and its sky is a vault eight metres out. There is no point in
the world two kilometres from the listener, so the distance cannot come from
a position: it has to be a number chosen per strike, and the flash has to be
drawn on the sky rather than lit from somewhere.

That is not a fudge as long as it is the only fiction. One chosen distance,
and everything else derived honestly from it.

Which also settles where the flash appears: a patch on the dome, in a
direction chosen per strike, bright in the middle and falling off. A near
strike lights the garden's walls; a far one only lights the cloud.

## Out of scope for now

**A bolt.** A drawn fork of lightning is a different and harder thing, and
the flash on the cloud is what ninety nine strikes in a hundred actually look
like from under them.

**Rain.** Separate, and wants the pond to answer it.

**Anywhere but the garden.** It is the only room with a sky.
