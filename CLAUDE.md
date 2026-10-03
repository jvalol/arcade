# arcade

A room of cabinets, one per game, walked in the first person. Press enter at one
and it runs that game's own binary.

Not a game, and not in `games/`. It sits beside that folder because it is the
front of the collection rather than part of it, and `list-repos` names it in the
full listing so the gate tests it, but not in `games` so the game lists are not
asked for a bullet about it.

## Building and running

```
cargo run --release
```

```
cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt --check
```

Or `./check-all` from `blitzkit-project`, which runs that for every crate and
then checks the copy markers, the spec citations and the listings.

## The spec flow

A spec before the code, in `specs/`, numbered in the order written. Each
acceptance criterion names the test that proves it.

## Decisions worth defending

**The games are not linked in.** A cabinet knows a name and spawns a binary. The
alternative is every game becoming a library with its own render target instead
of a binary that owns a window, which is a change to all twelve and to the
engine's `Game` and `start`. It would be needed for cabinets that actually play,
and that is deliberately out of scope.

**It finds the binaries beside its own.** Every crate here builds into one
shared `target`, per the `[build]` in `.cargo/config.toml`, so the arcade's
neighbours in its own directory are the games. Walking up from the working
directory or being told where the project is are both guesses that break the
moment it is run from somewhere else.

**The list of games is read from the disk, not kept here.** `games/` is the
list, the same way `list-repos` reads it. A list in two places is a list that
disagrees with itself, which is what the comment at the top of `list-repos` is
about.

**One at a time.** A room where every cabinet starts a window is a dozen windows
and a dozen sound devices, and the thing meant to show the games off becomes a
way to lose them behind each other.

---

I asked AI to draft this for me. I've edited it. Any surviving AI smells are my oversight.
