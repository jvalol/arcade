//! What the building says, in the words it says it in.
//!
//! **One file, because the signature is a hash of a file.** Every string
//! drafted for somebody to read carries `` until Jake takes
//! it out, and `sign-copy` takes it out by showing him the words and
//! writing the file's hash into `.copy-signed`. That works on a file
//! whose contents are the copy. Pointed at `src/bridge.rs` it would
//! make him re-approve the Bifröst inscription every time somebody
//! moved a vertex in the same file, which is the opposite of easy.
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
