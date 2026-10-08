# Footsteps

Recordings, not arithmetic. Every sound in this building was worked out by the
code that plays it until these, and the footsteps were the one thing that could
not be: they were the best measured and worst sounding thing in here, and they
were taken out in `e187000` until they sounded like footsteps.

All three sources are **CC0**, which asks for nothing. The credit below is
courtesy and not a condition.

| Surface | Variants | From |
| --- | --- | --- |
| `carpet` | 4 | [Owlish Media Sound Effects](https://opengameart.org/content/sound-effects-pack), `hard-footstep1`–`4` |
| `boards` | 6 | [Different steps on wood, stone, leaves, gravel and mud](https://opengameart.org/content/different-steps-on-wood-stone-leaves-gravel-and-mud) by TinyWorlds, `wood01`–`03`, and Owlish Media's `step1`, `step3`, `step4` |
| `tile` | 4 | Owlish Media again, the four reverberant ones |
| `gravel` | 6 | Fantozzi's Footsteps, the sand half |

## What was done to them

Each one is mono, 44.1 kHz, 16-bit PCM, and otherwise the recording. The whole
of the processing is:

- converted to mono at 44.1 kHz, since the engine plays one channel and places
  it in the world itself
- the lead-in silence trimmed, so a step lands when the foot does and not a
  tenth of a second later
- cut at 0.32 seconds with a 60 ms fade, which is not a taste. The engine plays
  sounds one after another and is not a mixer, so a step that outlasts the gap
  to the next one never finishes before the next is due: the queue falls behind
  for ever and the footsteps go on after you stop. At this building's walking
  speed and stride there is 0.38 of a second between steps. `noise::longest`
  works the cap out from those two rather than having it written down, and the
  tests hold every file to it
- the peak brought to −3 dBFS, so the code can set how loud a floor is and not
  have that undone by which file it reached for

No filtering, no pitching, no layering. A recording shaped into something it
is not is halfway back to arithmetic, and arithmetic is what this replaced. The
pitch and level that vary from step to step are applied at run time by
`Samples::pitched` and `Samples::gain`, per the engine's spec 0045.

## What is missing

**Water.** The pool in the baths has no wading sound, and none of these three
packs has one. Spec 0009 says what it should be. Fantozzi's sand is standing in
for the garden's gravel, which is close enough to be right; nothing in here is
close enough to be water.

**Boards carries three rooms**, which is why it has six where the others have
four: the nook, the cellar and the stair. There was a `stone` set, and it is
gone, because no floor in this building is stone and it sounded like walking on
pebbles in a room with a wooden floor.

**Carpet is the weakest match.** It is a hard-floor recording picked for being
the dullest of what was available, and the hall is the most walked floor in the
building. Worth replacing from Freesound, filtered to CC0.
