//! The sight in the middle of the screen, and how bright a cabinet is. Spec
//! 0003.
//!
//! Nothing here draws. Where the sight goes and how much brighter the lit
//! cabinet is are arithmetic, so both can be checked without a window.

use glam::{vec2, vec4, Vec2, Vec4};

/// The sight itself. A cross, because it is one glyph in the font the engine
/// already loads and it has a middle.
pub const SIGHT: &str = "+";

/// How big the sight is drawn, the name under it, and the line under that.
pub const SIGHT_SIZE: f32 = 22.0;
pub const PROMPT_SIZE: f32 = 22.0;
pub const DETAIL_SIZE: f32 = 15.0;

/// How much of the window's width a line under the sight may use before it
/// wraps.
///
/// Centred text with no bounds is laid out from the middle in both directions
/// and runs off both edges, which is what Jake's line about the Sierpinski
/// tetrahedron did: ninety characters, centred, at twenty points.
pub const PROMPT_WIDTH: f32 = 0.55;

/// How far under the middle the first line of the prompt sits, and how far
/// apart the lines are.
///
/// Clear of the sight rather than touching it: the sight is what you are
/// reading the room through and a word against it is a word in the way.
pub const PROMPT_DROP: f32 = 34.0;
pub const PROMPT_STEP: f32 = 26.0;

/// The middle of the window, where the sight goes.
///
/// Text is laid out from its baseline down, so the glyph is lifted by half its
/// size to put the cross itself on the middle rather than under it.
pub fn sight_at(window: (f32, f32)) -> Vec2 {
    vec2(window.0 * 0.5, window.1 * 0.5 - SIGHT_SIZE * 0.5)
}

/// Where a line of the prompt goes, counted from the first.
pub fn prompt_at(window: (f32, f32), line: usize) -> Vec2 {
    vec2(
        window.0 * 0.5,
        window.1 * 0.5 + PROMPT_DROP + line as f32 * PROMPT_STEP,
    )
}

/// How wide a line under the sight may be before it wraps.
pub fn prompt_bounds(window: (f32, f32)) -> Vec2 {
    vec2(window.0 * PROMPT_WIDTH, crate::aim::UNBOUNDED)
}

/// What the engine calls a bound that does not bind.
pub const UNBOUNDED: f32 = blitzkit::renderer::render_text::UNBOUNDED_F32;

/// The sight's colour. Dim on nothing so it is not a thing in the way, and up
/// to full when it is on something, which is the room answering you.
pub fn sight_colour(on_something: bool) -> Vec4 {
    if on_something {
        vec4(1.0, 0.98, 0.9, 0.95)
    } else {
        vec4(0.72, 0.72, 0.78, 0.45)
    }
}

/// How a cabinet is drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Look {
    /// The box itself.
    pub body: Vec4,
    /// What the screenshot is multiplied by, so 1.0 is the picture as taken.
    pub screen: Vec4,
}

/// How much of itself an unlit screen keeps.
///
/// Spec 0003's number. 0.78 was what spec 0001 drew and it could not be seen
/// at all: against a picture at full strength that is a difference of under a
/// third, in a room lit at 0.55, across an aisle. This is a little over half.
pub const DIMMED: f32 = 0.42;

/// A cabinet's body, lit and not, and the dark one a game that will not run
/// gets.
const BODY: Vec4 = vec4(0.20, 0.19, 0.24, 1.0);
const BODY_LIT: Vec4 = vec4(0.46, 0.44, 0.52, 1.0);
const BODY_UNBUILT: Vec4 = vec4(0.13, 0.13, 0.14, 1.0);

/// How bright this cabinet is, given whether it can be played and whether the
/// sight is on it.
///
/// One that will not run is never lit, however long you point at it. Spec 0001
/// already refuses to start it, and a cabinet that lights up and then does
/// nothing is worse than one that never lit.
pub fn look_of(built: bool, lit: bool) -> Look {
    if !built {
        return Look {
            body: BODY_UNBUILT,
            screen: Vec4::splat(DIMMED).with_w(1.0),
        };
    }

    if lit {
        Look {
            body: BODY_LIT,
            screen: Vec4::ONE,
        }
    } else {
        Look {
            body: BODY,
            screen: Vec4::splat(DIMMED).with_w(1.0),
        }
    }
}

/// How bright one of the far wall's shapes is, which follows the same rule.
pub fn shape_colour(lit: bool) -> Vec4 {
    if lit {
        vec4(0.95, 0.93, 1.0, 1.0)
    } else {
        vec4(0.44, 0.42, 0.52, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: (f32, f32) = (1280.0, 800.0);

    /// How far apart lit and unlit have to be before the difference reads from
    /// across the room.
    ///
    /// Held here rather than beside the colours, so tuning a colour cannot
    /// quietly undo the point of spec 0003: 0.78 against 1.0 was the right
    /// direction and still invisible.
    const WORTH_SEEING: f32 = 0.4;

    /// Spec 0003: the sight is in the middle of the window.
    ///
    /// Within half a line vertically, since text hangs from its baseline and
    /// the glyph is lifted to sit on the middle rather than under it.
    #[test]
    fn the_sight_is_in_the_middle() {
        let at = sight_at(WINDOW);

        assert!(
            (at.x - WINDOW.0 * 0.5).abs() < 1e-3,
            "sight is at x {} for a window {} wide",
            at.x,
            WINDOW.0
        );
        assert!(
            (at.y - WINDOW.1 * 0.5).abs() <= SIGHT_SIZE * 0.5,
            "sight is at y {} for a window {} high",
            at.y,
            WINDOW.1
        );
    }

    /// Spec 0003: and it stays there when the window changes size.
    #[test]
    fn the_sight_follows_a_resize() {
        for window in [
            (800.0, 600.0),
            (1280.0, 800.0),
            (2560.0, 1440.0),
            (640.0, 480.0),
        ] {
            let at = sight_at(window);

            assert!(
                (at.x - window.0 * 0.5).abs() < 1e-3,
                "{:?} put the sight at x {}",
                window,
                at.x
            );
            assert!(
                (at.y - window.1 * 0.5).abs() <= SIGHT_SIZE * 0.5,
                "{:?} put the sight at y {}",
                window,
                at.y
            );
        }
    }

    /// Spec 0003: a line under the sight wraps rather than running off both
    /// edges of the window.
    #[test]
    fn a_long_line_is_held_inside_the_window() {
        for window in [(640.0, 480.0), (1280.0, 800.0), (2560.0, 1440.0)] {
            let wide = prompt_bounds(window).x;

            assert!(
                wide < window.0,
                "{:?} lets a line use {} of its width",
                window,
                wide
            );
            assert!(
                wide > window.0 * 0.25,
                "{:?} leaves no room to read",
                window
            );
        }
    }

    /// Spec 0003: what to press is under the sight, not over it, and the lines
    /// go downwards.
    #[test]
    fn the_prompt_sits_under_the_sight() {
        let sight = sight_at(WINDOW);
        let first = prompt_at(WINDOW, 0);
        let second = prompt_at(WINDOW, 1);

        assert!(
            first.y > sight.y + SIGHT_SIZE,
            "the first line is at {} and the sight at {}",
            first.y,
            sight.y
        );
        assert!(second.y > first.y, "the second line is not under the first");
        assert!(
            (first.x - sight.x).abs() < 1e-3,
            "the prompt is not under the sight"
        );
    }

    /// Spec 0003: the sight answers you by brightening.
    #[test]
    fn the_sight_brightens_on_something() {
        let on = sight_colour(true);
        let off = sight_colour(false);

        assert!(on.w > off.w, "it is no more solid on something");
        assert!(
            on.x + on.y + on.z > off.x + off.y + off.z,
            "it is no brighter on something"
        );
    }

    /// Spec 0003: the one you are pointing at is brighter, body and screen
    /// both.
    #[test]
    fn the_one_you_point_at_is_brighter() {
        let lit = look_of(true, true);
        let not = look_of(true, false);

        assert!(
            lit.screen.x > not.screen.x,
            "the lit screen is {} against {}",
            lit.screen.x,
            not.screen.x
        );
        assert!(
            lit.body.x > not.body.x && lit.body.y > not.body.y && lit.body.z > not.body.z,
            "the lit body is {} against {}",
            lit.body,
            not.body
        );
    }

    /// Spec 0003: and far enough brighter to read across the room.
    ///
    /// The number rather than the direction, because spec 0001 had the
    /// direction right and still could not be seen.
    #[test]
    fn the_difference_is_worth_seeing() {
        let lit = look_of(true, true);
        let not = look_of(true, false);

        assert!(
            lit.screen.x - not.screen.x >= WORTH_SEEING,
            "the screens are {} apart, which spec 0003 says is not enough",
            lit.screen.x - not.screen.x
        );

        let shapes = shape_colour(true).x - shape_colour(false).x;
        assert!(shapes >= WORTH_SEEING, "the shapes are {} apart", shapes);
    }

    /// Spec 0003: one that will not run never lights up, however long you
    /// point at it.
    #[test]
    fn an_unbuilt_cabinet_stays_dark() {
        let pointed = look_of(false, true);
        let not = look_of(false, false);

        assert_eq!(pointed, not, "pointing at an unbuilt cabinet changed it");
        assert!(
            pointed.body.x < look_of(true, false).body.x,
            "it is no darker than one that can be played"
        );
        assert!(
            pointed.screen.x < look_of(true, true).screen.x,
            "its screen is as bright as a lit one"
        );
    }
}
