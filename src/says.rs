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
