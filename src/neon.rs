//! The room's own sign, hung over the near end of the aisle. Spec 0001.
//!
//! You wake a step inside the near wall looking down the hall, and nothing in
//! that view said what the place was. The nook got a sign for the same reason
//! and that one is a painted plank on chains, because the nook is a study and
//! neon would be a sign for the wrong room. This is the other way about. The
//! hall is thirteen neon cabinets, so the hall's own sign is neon: a dark can
//! with a tube run round it and the name burning on both faces.
//!
//! Hung at the near end of the cabinet rows, which is where `sign` hangs at the
//! far end. One piece of arithmetic for both, `END` past the last cabinet, so
//! the two ends of the hall are marked the same way and neither of them has to
//! be placed by hand.
//!
//! The colours are `aim::ARCADE_*`, apart from the multiplier, which is
//! `aim::NEON`: the same number the cabinets' bands burn at, since this is the
//! same stuff.
//!
//! Nothing here draws or reads a key, so where it hangs and what of it is in
//! front of you when you open your eyes are arithmetic and can be checked
//! without a window.

use glam::{vec3, Vec3};

/// What it says. The room is the arcade, and this is its name over the door.
pub const SAYS: &str = "ARCADE";

/// The can: across the aisle, down, and through.
pub const WIDE: f32 = 2.4;
pub const TALL: f32 = 0.5;
pub const THICK: f32 = 0.12;

/// The middle of the can off the floor.
///
/// Set by what is in the opening view rather than by what clears your head,
/// which it does with room to spare. The frame is sixty degrees top to bottom,
/// so from where you wake there is about a unit of picture above your eye and
/// the whole sign has to live in it. A sign jammed against the top edge is a
/// sign you have to look up to read on a frame you are not looking up on.
pub const HANGS: f32 = 2.2;

/// The tube run round the can: how thick a tube is and how far it stands out of
/// each face.
///
/// Proud of both faces rather than bent round the front, because the can is
/// read from both ends of the hall and a tube on one side only is a lit edge
/// from behind.
pub const TUBE: f32 = 0.036;
pub const PROUD: f32 = 0.026;

/// The lettering: how many texels tall it is drawn, how tall it is hung, and
/// how much of the can it crosses.
pub const TEXELS: f32 = 64.0;
pub const LETTERS: f32 = 0.26;
pub const MARGIN: f32 = 0.14;

/// The two stems to the ceiling: how far out from the middle each one is and
/// how thick.
///
/// Rigid, where the nook's sign is on chains. A chain says the thing under it
/// is light and painted, and this one is a steel box with a transformer in it.
pub const STEM_AT: f32 = 0.86;
pub const STEM: f32 = 0.05;

/// What it throws into the near end of the hall.
///
/// A sign burning at `aim::NEON` with no light of its own is the thing this
/// room has been told off for twice: a bright patch and no reason for it. Neon
/// of this size puts its colour on the ceiling and on the floor under it, and
/// that is most of what says the sign is a light rather than a picture of one.
pub const LIT: f32 = 0.78;
pub const RANGE: f32 = 6.2;
pub const COLOUR: Vec3 = vec3(1.0, 0.46, 0.76);

/// Where it hangs: over the middle of the aisle, level with the near end of the
/// cabinet rows.
pub fn at(reaches: f32) -> Vec3 {
    vec3(0.0, HANGS, reaches - crate::room::END)
}

/// The tube run, as four bars: a middle relative to the can and a size.
///
/// Inside the can's outline rather than around it, so the can reads as the
/// frame and the tube as what is fitted into it. Four bars and not a ring
/// mesh: the corners overlap, which is what a bent tube does anyway.
pub fn tubes() -> [(Vec3, Vec3); 4] {
    let through = THICK + PROUD * 2.0;
    let across = (WIDE - TUBE) * 0.5;
    let up = (TALL - TUBE) * 0.5;

    [
        (Vec3::Y * up, vec3(WIDE, TUBE, through)),
        (Vec3::NEG_Y * up, vec3(WIDE, TUBE, through)),
        (Vec3::NEG_X * across, vec3(TUBE, TALL, through)),
        (Vec3::X * across, vec3(TUBE, TALL, through)),
    ]
}

/// How wide the lettering is drawn, which is the can inside its tubes.
pub fn span() -> f32 {
    WIDE - (TUBE + MARGIN) * 2.0
}

/// How long a stem is, from the top of the can to the ceiling.
pub fn stem(ceiling: f32) -> f32 {
    (ceiling - (HANGS + TALL * 0.5)).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0001: the whole of it is in front of you when you open your eyes.
    ///
    /// This is the one thing asked of this sign. It is not "there is a sign in
    /// the room": it is that you wake up to it, which is a claim about a
    /// distance, a height and a field of view together, and all three can move
    /// without anybody touching this file.
    #[test]
    fn you_wake_up_looking_at_it() {
        // how far down the hall it is from where you wake, which does not
        // depend on how long the hall is: both ends are measured off `reaches`
        let away = crate::room::END - crate::room::INSIDE;
        assert!(away > 0.0, "the sign is behind you at {}", away);

        // the frame is sixty degrees top to bottom, so this is half of it at
        // that distance, measured from the eye looking level
        let frame = away * (30f32.to_radians()).tan();
        let top = HANGS + TALL * 0.5 + PROUD;

        assert!(
            top < crate::EYE + frame,
            "the top of the sign is {} and the picture stops at {}",
            top,
            crate::EYE + frame
        );
        // and not so far under the edge that it is in the middle of the view,
        // which is where the hall is
        assert!(
            top > crate::EYE + frame * 0.5,
            "the sign is at {}, low in a picture that reaches {}",
            top,
            crate::EYE + frame
        );
    }

    /// Spec 0001: and you walk under it rather than into it.
    #[test]
    fn you_can_walk_under_it() {
        let under = HANGS - TALL * 0.5 - PROUD;

        assert!(
            under > crate::EYE + 0.2,
            "the sign hangs at {} and your eye is at {}",
            under,
            crate::EYE
        );
        assert!(
            stem(crate::room::TALL) > STEM,
            "the can is against the ceiling, with {} of stem",
            stem(crate::room::TALL)
        );
    }

    /// Spec 0001: and it hangs in the aisle, not through a cabinet.
    #[test]
    fn it_hangs_clear_of_the_cabinets() {
        let aisle = crate::room::WALL - crate::room::CABINET.x * 0.5;

        assert!(
            WIDE * 0.5 < aisle,
            "the sign reaches {} across an aisle {} wide",
            WIDE * 0.5,
            aisle
        );
        // against the tube run rather than against `WIDE`, so this is read off
        // the thing the stems have to land on
        let (_, top) = tubes()[0];
        assert!(
            STEM_AT + STEM * 0.5 < top.x * 0.5,
            "a stem at {} hangs off a can {} across",
            STEM_AT,
            top.x
        );
    }

    /// Spec 0001: the tube run rings the can and shows from both sides.
    ///
    /// The nook's lamp is a fitting you can look at and this has none, so the
    /// tube is the whole of what makes it read as neon rather than as a lit
    /// rectangle. A bar that does not reach the corner leaves the ring open,
    /// and one that does not stand out of the can is buried in it.
    #[test]
    fn the_tubes_ring_the_can() {
        for (middle, size) in tubes() {
            assert!(
                size.z > THICK,
                "a tube {} through a can {} through is inside it",
                size.z,
                THICK
            );
            // it stays on the can rather than hanging off the edge of it
            assert!(middle.x.abs() + size.x * 0.5 <= WIDE * 0.5 + 1e-6);
            assert!(middle.y.abs() + size.y * 0.5 <= TALL * 0.5 + 1e-6);
            // and it reaches both corners of the side it is on
            let long = if size.x > size.y { size.x } else { size.y };
            let whole = if size.x > size.y { WIDE } else { TALL };
            assert!(
                (long - whole).abs() < 1e-6,
                "a tube {} long on a side {} long",
                long,
                whole
            );
        }
    }

    /// Spec 0001: and the lettering fits between the tubes.
    #[test]
    fn the_name_fits_inside_the_tubes() {
        assert!(span() > 0.0 && span() < WIDE - TUBE * 2.0);
        let (_, side) = tubes()[2];
        assert!(
            LETTERS < side.y - TUBE * 2.0,
            "letters {} tall in a can {} tall",
            LETTERS,
            side.y
        );
        assert!(!SAYS.is_empty(), "a sign with nothing on it");
    }
}
