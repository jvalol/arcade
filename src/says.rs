//! What the building says, in the words it says it in.
//!
//! **One file, because the signature is a hash of a file.** Every string
//! drafted for somebody to read carries a marker until Jake takes it out,
//! and `sign-copy` takes it out by showing him the words and writing the
//! file's hash into `.copy-signed`. That works on a file whose contents
//! are the copy. Pointed at `src/bridge.rs` it would make him re-approve
//! the Bifröst inscription every time somebody moved a vertex in the same
//! file, which is the opposite of easy.
//!
//! The marker is written above the strings below and nowhere in this
//! prose, because `sign-copy` strikes the token off whatever line it finds
//! it on. A sentence that quotes it comes back with a hole in it, which is
//! what the sentence above had.
//!
//! So drafted copy is written here and read from here. The hash moves
//! when the words move and at no other time.
//!
//! **And `.copy-files` names this file**, which is what makes
//! `sign-copy --all` find it. Before that there were two scanners with
//! two file sets: `check-all` read `src` and held the gate, while
//! `check-voice` read the readme and reported everything signed. The
//! one with the reading and editing in it was looking at the other
//! half.

/// The name on the runestone at the foot of the bridge, in the Latin
/// letters under the runes. Spec 0054.
pub const BIFROST: &str = "BIFROST";

/// What the watchman says when you come up to him.
///
/// **He asked a riddle until spec 0063.** It was "tell me what it looks
/// like from here", and what it took for an answer was "a light": the
/// building is a dark block with one lit lantern on it from up there, per
/// spec 0057. Jake read that and said the riddle is very weak, and he is
/// right. The answer comes from turning round rather than from having
/// walked anything, and spec 0058 set its own test as "an answer you can
/// only have from having walked the building", which it fails. The bridge
/// is already the gate: it only comes up once every station has been
/// given, so the riddle was a second lock on a door that was locked.
///
/// Four of his lines went with it: the question, the line after an answer,
/// the line after a wrong one, and the four answers it took.
///
/// What stands here is his own: he struck the line I drafted out of those
/// four and wrote this one in its place.
pub const WATCHMAN_LETS_YOU_BY: &str =
    "I have stood at this gate since the beginning of time. You may go on through.";

/// What `--help` says about the arcade's own options, above the engine's.
///
/// **One block with one marker on a line of its own**, which is the form
/// that leaves nothing behind when `sign-copy` takes the marker out. A
/// marker per line would sign off into eight stray gaps.
///
/// Spec 0069: every one of these was an environment variable until Jake
/// said the arguments seem to belong in different places. They do not any
/// more, and `tests::the_help_names_every_option_the_arcade_takes` holds
/// this list against the one the code actually reads.
pub const OPTIONS: &str = "\
The arcade takes these of its own:

  --done                   every question on the trail already answered, so
                           the bridge and the hall at the top of it are there
  --pose <seven numbers>   stand here, at this moment, rather than at the
                           door in the afternoon: the line the plate prints
                           under M, with or without commas
  --where                  print where you are standing, twice a second, on
                           stderr
  --at <0 to 1>            what time it is when the run starts: nought dawn,
                           a quarter noon, a half dusk
  --day <seconds>          how long a day lasts
  --sky <name>             hold the weather: clear, fair, overcast or storm
  --fly                    fly rather than walk
  --open-case              hold the hall of mirrors' bookcase open
  --dump-mirrors           write every mirror's view into /tmp, once
  --screenshot [<name>]    take the poster, or one of the others by name
  --screenshots            print the pictures and their files, and stop
  --walk <route>           walk a written route with no window, and fail if
                           it no longer goes where it says
  --tour <x,y,z>           work a route out to a place, and print it
  --film <clip>            film a clip

";
