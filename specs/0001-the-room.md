# 0001 The room

**Status:** implemented
**Date:** 2026-10-03

## Goal

A room you walk into, with a cabinet for every game built on blitzkit, and a key
that plays the one you are standing at.

## Why

There are twelve games and the only way to meet them is to read a list and type
`cargo run` in the right folder. That is a directory, not a collection. A list
tells you the games exist; a room tells you they are a body of work, and it puts
them in front of someone who has not already decided to go looking.

It is also the best demo the engine has. Every other game shows one thing at a
time: lantern is lights and a maze, slider is a parametric surface, cascada is
contacts. A room with cabinets in it needs walking, collision, textures, three
kinds of light, shadows and text at once, which is the engine as a whole rather
than a feature of it.

## Behavior

**A room, walked in the first person.** Lantern's movement: the mouse looks, the
keys walk, and you cannot pass through a wall or a cabinet.

**A cabinet for every game**, found by reading `games/` rather than by keeping a
list. A new game gets a cabinet the moment its folder exists, which is the same
rule `list-repos` follows and for the same reason: a list in two places is a
list that disagrees with itself. Each cabinet wears that game's own screenshot. The screenshots are already in the repos and `check-listings` already
makes sure every game has one, so the arcade draws what is there rather than
keeping its own pictures.

**The one you are standing at is lit**, and named, so what a key would start is
never a guess.

**A key plays it**, by running the game's own binary. The games are not changed,
not linked, and not built into this: each still runs on its own from its own
folder exactly as before, and the arcade runs the same binary a person would.

**One at a time.** While a game is running the arcade starts nothing else. The
room keeps drawing behind it, so walking out of a game puts you back where you
were standing.

**A game that has not been built is a dark cabinet**, named but unlit, rather
than a key that does nothing or a crash. `./check-all` builds every binary, so
the usual state is all of them lit.

**It finds the games beside itself.** Every binary in this project is built into
one shared `target` directory, so the arcade looks in its own, rather than
guessing at paths or being told where the project is.

## Acceptance criteria

- There is a cabinet for every game the project has. — `room::tests::every_game_gets_a_cabinet`
- And it knows every game the project does, without being told. — `tests::it_knows_every_game_the_project_does`
- No two cabinets share a place, and none of them is inside a wall. — `room::tests::the_cabinets_all_fit`
- The one you are standing at is the nearest one you are facing. — `room::tests::the_nearest_one_in_front_is_the_one`
- Walking into a cabinet or a wall stops you. — `room::tests::you_cannot_walk_through_anything`
- A game that is not built is a cabinet that will not start. — `cabinet::tests::one_that_is_not_built_will_not_start`
- Only one game runs at a time. — `cabinet::tests::only_one_runs_at_a_time`
- And the room is free again once that one quits. — `cabinet::tests::it_is_free_again_once_that_one_quits`

### Verified by hand

- Walking the room and reading the cabinet names.
- Starting a game, quitting it, and walking on.

## Out of scope

The games playing on the cabinet screens, which would mean every one of them
becoming a library with its own render target rather than a binary that owns a
window. Scores. A coin slot. Sound of its own, since the game that is running
has its own.
