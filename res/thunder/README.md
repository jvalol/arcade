# Thunder

Recordings, not arithmetic, and the second thing in this building to go that
way after the footsteps. It was made first and did not land: a running average
over noise under an envelope is a running average over noise under an envelope,
however carefully the envelope was chosen, and thunder is a sound every ear
already knows by heart. Same reason, same answer.

Both sources are **CC0**, which asks for nothing. The credit below is courtesy
and not a condition.

| Band | Files | From |
| --- | --- | --- |
| `near` | 1 | [thunder crack.wav](https://freesound.org/people/cognito%20perceptu/sounds/347853/) by cognito perceptu, "a loud clap of thunder and the subsequent rumble" |
| `mid` | 1 | [Thunder Clap And Rumble #1](https://freesound.org/people/Kinoton/sounds/397952/) by Kinoton, "dry version of a single lightning strike at medium distance" |
| `far` | 2 | the rolls off the back of those same two |

**The far band is the tail of a close strike**, which is not a shortcut. What
you hear from kilometres away is the roll and not the strike, and the roll off
the back of a near one is the same sound arriving by a longer road. It also
costs no third source. If it ever stops convincing, the answer is a genuine
distant recording and not a filter over these.

## What was done to them

Each one is mono, 44.1 kHz, 16-bit PCM, and otherwise the recording. The whole
of the processing is:

- converted to mono at 44.1 kHz, since the engine plays one channel and the
  storm has nowhere to be played from in any case: a strike is a chosen
  distance and not a place, per spec 0016
- cut to the part worth keeping, with a 120 ms fade at each end so neither is
  a step in the waveform
- the peak brought to −3 dBFS, so the code can set how loud a strike is and
  not have that undone by which file it reached for

No filtering, no pitching, no layering. A recording shaped into something it
is not is halfway back to arithmetic, and arithmetic is what this replaced.
How loud a given strike is and the small wander from one to the next are
applied at run time by `Samples::gain` and `Samples::pitched`, the way the
footsteps' are.

**No length cap**, which is the one place this differs from the footsteps.
Those are cut to 0.32 seconds because the engine queues what `play_at` is
given and a step that outlasts the gap to the next one falls behind for ever.
Thunder goes through `play`, which hands it to the mixer and starts it now, so
two strikes overlapping are two strikes overlapping. Real thunder rolls for
ten or twenty seconds and these are six to eight.

## Where they came from

Freesound's preview mp3s and not the original wavs, which need an account. At
128 kbps mono 44.1 kHz that is one generation of mp3, and the pipeline above
converts everything to mono 44.1 kHz anyway. CC0 covers the preview as fully
as the original.

128 is lower than the 198 the water footsteps came at, and mp3's weak point is
a broadband transient, which is exactly what the front of a near clap is. It
measured clean enough: both recordings have nothing above 1.3 kHz worth
counting, so there is little up there for an mp3 to spoil. Worth replacing from
the originals if anybody ever has an account.

## What the recordings said that the synthesis had wrong

Measured the same way, over the loudest window of each:

| | 640 Hz | 1,280 Hz | 2,560 Hz |
| --- | --- | --- | --- |
| the close recording | 0.08 | 0.00 | 0.00 |
| what was synthesised for a near strike | 0.25 | 0.17 | 0.13 |

Real thunder is far darker than it was built to be. The near strikes were not
failing for want of a crack, which is what they were tuned for. They were
hissing.
