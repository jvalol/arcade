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

/// What the watchman asks, and what he says once you have answered.
/// Spec 0058.
///
/// **He has never seen the building.** He has stood at this end of the
/// bridge the whole time, which is the one thing he can honestly want
/// from somebody arriving over it. The answer is not a word: it is
/// turning round, and the line has to ask for that without saying
/// "turn round", or the riddle is an instruction.
///
/// The second line is said once, after the gate has started to move. It
/// does not say well done and it does not say what is through the gate.
pub const WATCHMAN_ASKS: &str =
    "I have stood at this end since it was built. Tell me what it looks like from here.";
pub const WATCHMAN_ANSWERED: &str = "So that is it. Go on through.";

/// And what he says to an answer that is not one of his.
///
/// **It is not a wrong buzzer.** Spec 0058: a gate that can be failed is
/// a score, and nothing in this building counts at you. He says this,
/// the line clears, and the field stays open for as long as anybody
/// wants to stand there.
pub const WATCHMAN_AGAIN: &str = "That is not what I see from here either. Look again.";

/// What he takes for an answer, folded the way `typed::Field::answer`
/// folds what you write: trimmed, lower case, single spaces.
///
/// **Jake's, and placeholder until he writes it.** What the riddle is
/// worth turns on these words and on nothing else in the file: too few
/// and a right answer is refused, too many and the gate is a door. They
/// are drafted off what spec 0057 measured you can actually see from up
/// there, which is a dark block with one lit lantern on it.
///
/// Kept beside the question because a question and the answers it takes
/// are one thing, and the two drifting apart is a riddle that cannot be
/// solved.
pub const WATCHMAN_ACCEPTS: &[&str] = &["a light", "a lantern", "a light in the dark", "one light"];
