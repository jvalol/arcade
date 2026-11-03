//! What the building has given up to you, room by room. Spec 0036.
//!
//! **This is the plumbing and not the trail.** Spec 0036 is a trail through
//! every room, and it is waiting on which room is at the end of it. What it
//! says can be built first is the part underneath: the building has eleven
//! rooms and no idea what you have already done in any of them. Four refer
//! to each other and the other seven are sealed.
//!
//! So this records, and nothing more. Nothing here is drawn, nothing is
//! counted at you, and a run that never finds the trail is a run where this
//! fills up and is never read. That is deliberate: spec 0036's rule is that
//! somebody who never finds the trail should not be able to tell it is
//! there.
//!
//! **A room gives up its answer, it does not get a tick.** The register
//! already does the whole pattern and is the reason this has the shape it
//! has: you walk the diagonal, you come away carrying a guest the hotel
//! cannot house, and the desk takes it out of your hand. The clue is the
//! receipt for doing the room properly, so what is recorded is that the
//! room gave something up, in the order the rooms gave.
//!
//! **Why not `noise::Room`.** That enum says which room you are standing
//! in, worked out from where your feet are, for deciding what you can hear.
//! It has a stair and a field, which ask you nothing, and it has no hotel,
//! no register and no gallery, which ask you the three hardest questions in
//! the building. The two lists overlap and answer different questions, so
//! each says its own, and this one is named for the asking rather than for
//! the standing.

/// A room that asks you something, named by what it asks.
///
/// **Only the rooms whose question is already in the code.** Spec 0036 is
/// plain that a room needing an object planted in it to join the trail is a
/// room the trail is wrong about, so nothing is listed here on the strength
/// of a room being nice to visit. The rest arrive as their rooms grow a
/// moment worth recording, and `Trail` does not care how many there are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asked {
    /// Walking the register's diagonal to the end of the wall, which leaves
    /// you carrying a guest on no line of the book. Spec 0031.
    WalkedTheDiagonal,
    /// Handing that guest to the desk, which houses them and writes them
    /// into the book. Spec 0032.
    HousedTheCarried,
    /// Riding out to the sun, which is what the space behind the wall is
    /// for: the system at true scale, where the reading is the distance.
    /// Specs 0015 and 0018, and one of the four spec 0036 names itself.
    RodeToTheSun,
    /// Working one of the imaginarium's second doors. Spec 0038, where
    /// `room::Seen::Hatch` already says in as many words that this is the
    /// only thing in that room that asks you anything.
    WorkedAHatch,
    /// Turning the sixth display, which turns you instead. Spec 0021: five
    /// of them spin on their plinths while you watch, and the way into the
    /// hidden wing is a thing on a plinth rather than another wall.
    WentInsideTheTesseract,
    /// Opening the bookcase in the cellar, by the one leaf in it that is a
    /// handle. Spec 0007.
    OpenedTheCase,
    /// Getting behind the hall's near end wall. Spec 0011, and the answer
    /// to the question spec 0020's worn track asks by existing.
    GotBehindTheWall,
    /// Walking out past the garden's west wall, where the ground does not
    /// stop. Spec 0040.
    WentOutIntoTheField,
}

/// What the building has given up, in the order it gave.
///
/// A list rather than a set of flags, because the order is the only thing
/// here that could ever mean anything: two rooms answered is not the same
/// as two rooms answered in that order, and a flag per room throws that
/// away. It is also what stops this becoming the checklist spec 0036 says
/// it must not be, since a list of what happened is not a list of what has
/// not.
#[derive(Clone, Debug, Default)]
pub struct Trail {
    given: Vec<Asked>,
}

impl Trail {
    pub fn new() -> Self {
        Self { given: Vec::new() }
    }

    /// A room gave up its answer.
    ///
    /// **Saying it twice is not two answers.** You can walk the diagonal
    /// again, and spec 0032 says so in as many words: walk it again and it
    /// will have found somebody else. The building wants to know that the
    /// room has been done, not how many times, and a count here would be a
    /// score.
    pub fn gave(&mut self, what: Asked) {
        if !self.given.contains(&what) {
            self.given.push(what);
        }
    }

    /// In the order they were given.
    pub fn given(&self) -> &[Asked] {
        &self.given
    }

    /// Whether the building has given up anything at all.
    ///
    /// **There is no `len` and no `has`.** A room that wants either can
    /// read `given`, and writing them before anything asked would be two
    /// more ways to say what one list already says. Spec 0036 is also
    /// plain about what a count must never become: no panel of ticks, no
    /// "3 of 8 found".
    pub fn is_empty(&self) -> bool {
        self.given.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_with_nothing() {
        let trail = Trail::new();

        assert!(trail.is_empty());
        assert!(trail.given().is_empty());
    }

    #[test]
    fn a_room_that_answers_twice_answered_once() {
        let mut trail = Trail::new();

        trail.gave(Asked::WalkedTheDiagonal);
        trail.gave(Asked::WalkedTheDiagonal);

        assert_eq!(
            trail.given().len(),
            1,
            "walking the diagonal again scored again"
        );
    }

    #[test]
    fn it_keeps_the_order_they_came_in() {
        let mut one = Trail::new();
        one.gave(Asked::WalkedTheDiagonal);
        one.gave(Asked::HousedTheCarried);

        let mut other = Trail::new();
        other.gave(Asked::HousedTheCarried);
        other.gave(Asked::WalkedTheDiagonal);

        assert_ne!(
            one.given(),
            other.given(),
            "the same two rooms in either order came out the same, so the \
             order is not kept and a trail cannot be walked backwards"
        );
    }
}
