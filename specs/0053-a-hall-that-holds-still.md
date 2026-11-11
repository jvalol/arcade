# 0053 A hall that holds still

**Status:** implemented
**Date:** 2026-11-03

## Goal

A room's picture is a picture of this repo, and nothing outside it decides
what the reference looks like.

## Why it belongs here

Every cabinet in the hall shows that game's `media/screenshot.png`, read off
the disk by `art_of` as the test runs. So thirteen other repos are inside
this one's reference, and the glow each cabinet lends the room comes out of
the same file.

**It cost a release.** `./release` takes all thirteen screenshots before the
gate now, per the project's own ordering rule, so 0.15.0 retook them and
then failed on `the_hall_is_drawn` by 0.6% of its pixels. Nothing was wrong:
the room was correct and its reference was of art that no longer existed.
Clearing it took nine commits across nine repos, and it would have cost them
again at every release, because a staged shot of a game is a photograph of a
real window and never comes back quite the same.

## Behavior

**`Arcade::art_from` says where a cabinet's picture comes from**, and it is
`art_of` everywhere but a picture test.

**A picture test hands the hall art of its own.** `stand_in_art` is flat and
a different colour per cabinet, worked out from the name, so the screens are
still lit, still differ from each other, and are decided entirely in this
repo. Four references move with it: the hall, the cellar, the back room and
the imaginarium, which are the rooms with a cabinet in view or lit by one.

**And what that gives up is held somewhere else.** The picture no longer
notices a game whose screenshot has gone missing or stopped decoding, so
`every_cabinet_finds_its_art` asks that directly, of `art_of` itself. It is
arithmetic and it is asked as arithmetic.

The hall's light is partly the games' own colours and the reference no
longer shows that. That is the cost, and it buys a reference that only
changes when this repo does.

## Acceptance criteria

- Every cabinet finds the picture the hall would show on
  it. — `tests::every_cabinet_finds_its_art`
- The rooms still look like themselves. — `tests::the_hall_is_drawn` and the
  thirteen beside it

### Verified by hand

- **The four pictures that moved.** The hall still reads as the hall: the
  carpet, the sign, the aisle and a row of lit screens, each a different
  colour. The cellar, the back room and the imaginarium are unchanged but
  for the light a cabinet throws.
- **A control.** `art_of` pointed at a name no game has answers `None`, and
  `every_cabinet_finds_its_art` names the cabinets that found nothing
  rather than only counting them.

## Out of scope

**The browser build's copy of the art.** `build.rs` bakes all thirteen into
the wasm, so a game's screenshot changing makes the published arcade stale
and `check-fresh` cannot see it, because it counts commits inside one repo.
That is the same cross-repo dependency from the other end and it is not
fixed here.

**What the real art looks like on a cabinet.** Nothing automated holds that
and nothing here pretends to. It is a thing to look at.
