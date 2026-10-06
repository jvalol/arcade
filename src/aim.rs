//! The sight in the middle of the screen, and how bright a cabinet is. Spec
//! 0003.
//!
//! Nothing here draws. Where the sight goes and how much brighter the lit
//! cabinet is are arithmetic, so both can be checked without a window.

use glam::{vec2, vec4, Vec2, Vec3, Vec4};

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

/// How far the shadow under the sight and the prompt is offset, and what
/// colour it is.
///
/// The prompt lands on whatever you are pointing at, and what you are pointing
/// at is the brightest thing in the room because the sight just lit it. White
/// on white is what that gives you: the name of the Sierpinski tetrahedron was
/// legible only where a dark face happened to sit behind a letter. Two pushes
/// rather than a pass the engine does not have.
///
/// The sight carries one too, and needs it more. On nothing it is dim on
/// purpose, and dim grey on a lit grey wall is nothing at all: from the
/// doorway it disappeared outright.
pub const SHADOW: f32 = 2.0;
pub const SHADOW_COLOUR: Vec4 = vec4(0.0, 0.0, 0.0, 0.85);

/// Where the shadow under a line of the prompt goes.
pub fn shadow_at(at: Vec2) -> Vec2 {
    at + Vec2::splat(SHADOW)
}

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

/// The light in the room: a warm overhead sun and a cool fill under it.
///
/// This is what was making the room grey, and tinting the surfaces first was
/// treating the symptom. The sun was pure white and the fill was
/// `Vec3::splat(0.28)`, a perfectly neutral grey, and under a neutral light a
/// tinted surface comes back the colour it started as, at a lower brightness.
/// Nothing in the room could be any colour until the light was one.
///
/// Warm from above and cool in what it does not reach, which is how a room
/// with a light in it looks and is why a shadow reads as blue.
pub const SUN: Vec3 = glam::vec3(1.0, 0.88, 0.72);
pub const SUN_STRENGTH: f32 = 0.62;
pub const FILL: Vec3 = glam::vec3(0.15, 0.18, 0.29);

/// The room's own surfaces.
///
/// The floor, the walls and the plinths were all one colour at four
/// brightnesses, every one of them between 0.12 and 0.24 of the same hue, and
/// a plinth was the wall behind it exactly, so the row of shapes had nothing
/// to stand against.
///
/// FLOOR is a tint on the carpet rather than a colour of its own, near white so
/// the weave comes through as it was made. The walls are properly warm rather
/// than nudged, so the cabinets read as cool objects in a warm room. The
/// plinths are lighter than both, because what stands on them is pale.
pub const FLOOR: Vec4 = vec4(0.95, 0.95, 1.0, 1.0);
pub const WALL: Vec4 = vec4(0.30, 0.22, 0.16, 1.0);
/// The cradle on its bench: its frame, its ropes and its balls. Spec 0006.
pub const CRADLE_FRAME: Vec4 = vec4(0.20, 0.21, 0.25, 1.0);
pub const CRADLE_ROPE: Vec4 = vec4(0.62, 0.60, 0.56, 1.0);
pub const CRADLE_BALL: Vec4 = vec4(0.80, 0.82, 0.88, 1.0);

/// The metronome: its case, the plate the needle swings against, the needle,
/// the bob under the pivot and the weight that slides up it. Spec 0006.
///
/// Wood and brass, which is what one of these is made of, and the one thing in
/// the nook that is not grey or steel.
pub const CASE: Vec4 = vec4(0.32, 0.19, 0.11, 1.0);
pub const CASE_PLATE: Vec4 = vec4(0.50, 0.44, 0.36, 1.0);
pub const NEEDLE: Vec4 = vec4(0.72, 0.70, 0.64, 1.0);
pub const PENDULUM: Vec4 = vec4(0.22, 0.23, 0.27, 1.0);
pub const SLIDING_WEIGHT: Vec4 = vec4(0.88, 0.72, 0.32, 1.0);

/// The notches up the plate, and the one the weight is in.
///
/// The one it is in glows, which is the only way to see where the weight is set
/// from more than a step away. Colour past 1 glows with no light on it, as the
/// cabinets' bands do.
pub const NOTCH: Vec4 = vec4(0.34, 0.30, 0.25, 1.0);
pub const NOTCH_ON: Vec4 = vec4(1.8, 1.5, 0.7, 1.0);

/// The gyroscope: its stand, its axle, the wheel, and the stud on the rim that
/// says it is turning. Spec 0006.
///
/// A wheel is a round thing and a round thing spinning looks still, so without
/// the stud the only way to tell a spun one from a stopped one is that it has
/// not fallen over. Which is the point, but it should not be the only evidence.
pub const STAND: Vec4 = vec4(0.30, 0.29, 0.33, 1.0);
pub const SPINDLE: Vec4 = vec4(0.62, 0.60, 0.56, 1.0);
pub const FLYWHEEL: Vec4 = vec4(0.72, 0.60, 0.30, 1.0);
pub const STUD: Vec4 = vec4(1.7, 0.9, 0.5, 1.0);
pub const GIMBAL: Vec4 = vec4(0.78, 0.79, 0.84, 1.0);

/// The study the nook is furnished as: the bookcases, the candles and their
/// flames. Spec 0006.
///
/// Dark wood against the arcade's cool grey, because the whole point of the nook
/// is that it is not the arcade. A flame is past 1 in every channel, so it glows
/// with no light on it, the way a cabinet's band does.
pub const BOOKCASE: Vec4 = vec4(0.26, 0.15, 0.09, 1.0);
pub const SHELF: Vec4 = vec4(0.32, 0.19, 0.11, 1.0);
/// The panelling: the panels themselves, and the skirting, rail and cornice
/// that frame them.
///
/// Dark wood against the arcade's grain. Panelled below the rail and plain
/// above it, which is what a panelled room is, and the trim a shade lighter
/// than the panels so the mouldings read as mouldings rather than as a flat
/// wall of one brown.
pub const PANEL: Vec4 = vec4(0.23, 0.13, 0.08, 1.0);
pub const TRIM: Vec4 = vec4(0.34, 0.21, 0.12, 1.0);

/// The sconces on the wall: the bracket, and the shade that glows.
///
/// A shade past 1 in every channel burns with no light on it, the way a
/// cabinet's band does, so the fitting looks lit rather than painted.
pub const SCONCE: Vec4 = vec4(0.46, 0.34, 0.14, 1.0);
pub const SHADE: Vec4 = vec4(2.6, 1.9, 1.1, 1.0);

/// How many lights the sconces may take.
///
/// The engine carries eight. These are what lights the nook, so they get the
/// larger share of them while you are in it, and the cabinets out in the aisle
/// are too far away to be throwing anything you can see from here anyway.
pub const SCONCE_LAMPS: usize = 6;

/// The nook's floor of boards, under the rug. Spec 0006.
///
/// Darker than the panelling and warmer than the benches, so the room has a
/// floor, a dado and a worktop and they read as three different woods rather
/// than as one wood lit three ways.
pub const BOARDS: Vec4 = vec4(0.30, 0.19, 0.12, 1.0);

/// The globe: its stand and the meridian ring round it. Spec 0006.
///
/// Brass, because that is what they are made of, and because the nook is
/// otherwise steel and dark wood and the globe is the one thing in it that is
/// meant to be looked at rather than worked.
pub const GLOBE_STAND: Vec4 = vec4(0.62, 0.48, 0.22, 1.0);
pub const GLOBE_RING: Vec4 = vec4(0.78, 0.62, 0.30, 1.0);

/// The ball and chain: the gantry it hangs from, the chain, the ball, the brick
/// and the tray it all stands in. Spec 0006.
///
/// The chain, the ball and the brick are the example's own three colours, so
/// the model on the bench and the full size one in its own window are the same
/// thing to look at.
pub const GANTRY: Vec4 = vec4(0.34, 0.33, 0.30, 1.0);
pub const CHAIN: Vec4 = vec4(0.42, 0.44, 0.50, 1.0);
pub const WRECKING_BALL: Vec4 = vec4(0.30, 0.31, 0.36, 1.0);
pub const BRICK: Vec4 = vec4(0.70, 0.56, 0.38, 1.0);
pub const TRAY: Vec4 = vec4(0.19, 0.20, 0.23, 1.0);

/// A bench and its legs, and how thick its top is. Spec 0006.
///
/// Warmer than the cabinets, which are cool grey, because a bench is furniture
/// you lean on rather than a machine you feed.
pub const BENCH: Vec4 = vec4(0.46, 0.33, 0.24, 1.0);
pub const BENCH_ON: Vec4 = vec4(0.72, 0.54, 0.34, 1.0);
pub const BENCH_LEG_LOOK: Vec4 = vec4(0.24, 0.23, 0.26, 1.0);
pub const BENCH_TOP: f32 = 0.09;
pub const BENCH_LEG: f32 = 0.07;

pub const PLINTH: Vec4 = vec4(0.34, 0.33, 0.36, 1.0);

/// What the carpet is multiplied by, and the lid over the room.
///
/// The floor is a texture now, so this is a tint on it rather than its colour,
/// and it is near white so the carpet comes through as it was woven. The
/// ceiling is darker than anything else, which is where a dim room's light
/// does not reach.
pub const CEILING: Vec4 = vec4(0.09, 0.08, 0.10, 1.0);

/// How tight a highlight a matte surface gets.
///
/// Shininess is the power the half vector is raised to, so a small number is a
/// highlight spread over everything. The carpet went down at 4.0 and its dark
/// ground came back pale grey, because every texel of it was catching the sun.
pub const MATTE: f32 = 64.0;

/// The lamp a cabinet's band throws into the room: how far it reaches, how
/// bright it burns, and how much brighter the one you are standing at is.
///
/// A band on its own is a bright rectangle and nothing else. The engine has no
/// bloom, so what makes a tube read as a tube is not the tube, it is the
/// colour it puts on the wall behind it and the floor under it. Thirteen
/// glowing bars that lit nothing were thirteen coloured bars, which is what
/// Jake called them.
///
/// Calibrated against lantern's candle, which is 0.5 over a range of 9.
pub const LAMP_RANGE: f32 = 3.0;
pub const LAMP_INTENSITY: f32 = 0.5;
pub const LAMP_LIT: f32 = 2.2;

/// How many cabinets can be lighting the room at once.
///
/// The engine carries eight point lights and drops the rest with a warning, and
/// the room has a cabinet per game, which is well past eight. The nearest eight
/// are the ones that can be seen to be lighting anything.
pub const LAMPS: usize = 8;

/// How much of the engine's eight the nook's sconces take.
///
/// It had two bare lamps in the ceiling with nothing to hang them on, so what
/// you saw was a bright patch on a surface and no reason for it. The sconces are
/// the light in here now, they are a thing you can look at, and they take most
/// of the eight while you are in the room. The cabinets out in the aisle are too
/// far to be throwing anything you could see from here, which is why they get
/// what is left.
///
/// The neon on a cabinet: how bright the band burns, how thick it is, and
/// where up the front it sits.
///
/// Colour here is an unclamped multiplier, so anything past 1 glows with no
/// light on it. securitysweep's tripwires are 7.0 and its finish line 5.6,
/// which is where this number comes from.
///
/// It is the one thing in the room that is lit from the doorway. Spec 0003
/// keeps a screen dark until you are within reach of it, and that is about
/// what the room is offering you rather than about what it looks like. A real
/// cabinet's trim is on whether or not anyone is standing at it.
pub const NEON: f32 = 3.2;
pub const NEON_THICK: f32 = 0.055;
pub const NEON_UP: f32 = 0.93;

/// The marquee: how tall the name is drawn in texels, how tall the band is in
/// the world, and how much of the cabinet's front it spans.
///
/// A lit bar with nothing on it is a lit bar, which is what Jake called it.
/// Every cabinet ever built has the game's name across the top, and that is
/// most of what tells you to walk up to one. Spec 0037 of the engine is what
/// lets a word go on a thing in the world rather than on the glass.
pub const SIGN_TEXELS: f32 = 48.0;
pub const SIGN_TALL: f32 = 0.17;
pub const SIGN_SPAN: f32 = 0.86;

/// How much of its grey a tint loses before it is burnt into a band.
///
/// A tint straight off a picture is pale, something like (0.5, 1.0, 0.75), and
/// multiplying that by anything bright puts every channel past 1 and the
/// screen clamps all three to white. Thirteen white tubes is what the first
/// build of this gave.
///
/// So the grey comes out first: each channel is remapped from its own weakest
/// up to its brightest, which leaves the weakest at nothing and the brightest
/// at full, and only then is it burnt. Neon is a saturated colour in life as
/// well.
pub const NEON_FLOOR: f32 = 0.2;

/// A cabinet's band, in its own game's colour.
pub fn neon_of(glow: Vec3) -> Vec4 {
    let dullest = glow.min_element();
    let range = (1.0 - dullest).max(NEON_FLOOR);
    let saturated = ((glow - Vec3::splat(dullest)) / range).clamp(Vec3::ZERO, Vec3::ONE);

    // a picture with no colour in it gets a white tube rather than a black one
    let colour = if saturated.max_element() < 0.15 {
        Vec3::ONE
    } else {
        saturated
    };

    (colour * NEON).extend(1.0)
}

/// What colour a screenshot lends the room.
///
/// The smallest level of its own mip chain is the picture averaged down to
/// almost nothing, which is the average colour and costs nothing to read.
/// Normalised to its brightest channel, so this is a tint and `LAMP_INTENSITY`
/// alone says how bright the lamp is. A picture with no colour in it at all
/// lends white rather than nothing.
pub fn glow_of(art: &blitzkit::texture::TextureData) -> Vec3 {
    let Some(level) = art.levels.last() else {
        return Vec3::ONE;
    };

    let mut sum = Vec3::ZERO;
    let mut seen = 0.0;
    for texel in level.pixels.chunks_exact(4) {
        sum += glam::vec3(texel[0] as f32, texel[1] as f32, texel[2] as f32);
        seen += 1.0;
    }
    if seen == 0.0 {
        return Vec3::ONE;
    }

    let mean = sum / (seen * 255.0);
    let brightest = mean.max_element();
    if brightest < 1e-3 {
        Vec3::ONE
    } else {
        mean / brightest
    }
}

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

/// How bright one of the far wall's shapes is.
///
/// A smaller step than a cabinet's, and that is the whole point. Brightening
/// is this room's way of saying press enter, and a shape has nothing to press:
/// it lit up exactly as a cabinet does and then did nothing, which is the same
/// promise an unbuilt cabinet is kept dark to avoid making.
///
/// Not none, though. Five of them stand 0.72 apart and the name under the
/// sight belongs to one of them. Enough to say which, not enough to say press.
pub fn shape_colour(lit: bool) -> Vec4 {
    if lit {
        vec4(0.58, 0.56, 0.66, 1.0)
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

    /// Spec 0003: the sight is drawn whether or not it is on anything, so the
    /// dim one has to survive a light wall.
    #[test]
    fn the_dim_sight_is_still_there() {
        let dim = sight_colour(false);

        assert!(dim.w > 0.3, "at {} alpha there is nothing to see", dim.w);

        let sight = sight_at(WINDOW);
        assert!(
            shadow_at(sight) != sight,
            "the sight has no shadow to stand against a light wall"
        );
    }

    /// Spec 0003: the prompt is readable against the thing it lands on, which
    /// is the brightest thing in the room.
    #[test]
    fn the_prompt_carries_its_own_background() {
        let text = prompt_at(WINDOW, 0);
        let shadow = shadow_at(text);

        assert!(shadow != text, "the shadow is not offset from the text");
        assert!(
            shadow.y > text.y,
            "the shadow is above the text rather than under it"
        );

        let under = SHADOW_COLOUR;
        assert!(
            under.w > 0.5,
            "a shadow at {} alpha is not a background",
            under.w
        );
        assert!(
            under.x + under.y + under.z < sight_colour(true).truncate().element_sum(),
            "the shadow is no darker than what it sits under"
        );
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
    }

    /// Spec 0003: a shape says which one it is and does not say press.
    ///
    /// Held as a fraction of a cabinet's step rather than as a number of its
    /// own, since what matters is that the two do not read as the same offer.
    /// They were 0.51 and 0.58, near enough the same, and a shape that comes
    /// on like a cabinet and then does nothing is the promise an unbuilt
    /// cabinet is kept dark to avoid making.
    #[test]
    fn a_shape_answers_more_quietly_than_a_cabinet() {
        let shapes = shape_colour(true).x - shape_colour(false).x;
        let cabinets = look_of(true, true).screen.x - look_of(true, false).screen.x;

        assert!(
            shapes > 0.05,
            "a step of {} does not say which one the name belongs to",
            shapes
        );
        assert!(
            shapes < cabinets * 0.5,
            "a shape steps {} against a cabinet's {}, which reads as the same offer",
            shapes,
            cabinets
        );
    }

    /// Spec 0003: the room's surfaces are told apart.
    ///
    /// Floor, walls and plinths were one colour at four brightnesses, every
    /// one between 0.12 and 0.24 of the same hue, and a plinth was the wall
    /// behind it exactly. The room read as a grey box and the row of shapes
    /// had nothing to stand against.
    #[test]
    fn the_room_is_not_one_colour() {
        let body = look_of(true, false).body;
        let surfaces = [("wall", WALL), ("plinth", PLINTH), ("cabinet", body)];

        for (n, (name, one)) in surfaces.iter().enumerate() {
            for (other_name, other) in surfaces.iter().skip(n + 1) {
                let apart = (one.truncate() - other.truncate()).abs().max_element();
                assert!(
                    apart > 0.03,
                    "{} and {} are {} apart, which is the same colour",
                    name,
                    other_name,
                    apart
                );
            }
        }

        // and the carpet's own ground is darker than anything standing on it,
        // so what stands on it reads as standing on it. FLOOR is a tint on a
        // texture now rather than the floor's colour, so the ground is where
        // the floor's darkness actually lives.
        let ground = crate::carpet::ground();
        for (name, one) in surfaces {
            assert!(
                ground.element_sum() < one.truncate().element_sum(),
                "the carpet's ground {:?} is no darker than the {}",
                ground,
                name
            );
        }
    }

    /// Spec 0003: the light itself has a colour, warm above and cool below.
    ///
    /// A pure white sun over a neutral fill is what made the room grey: under
    /// a neutral light a tinted surface comes back the colour it started as,
    /// so tinting the walls first did nothing.
    /// How warm a light is: how much more red it carries than blue. Negative
    /// is cool, nought is the neutral that greyed the room.
    fn warmth(light: Vec3) -> f32 {
        light.x - light.z
    }

    #[test]
    fn the_light_is_not_neutral() {
        assert!(
            warmth(SUN) > 0.1,
            "the sun is {:?}, which is neutral enough to grey the room",
            SUN
        );
        assert!(
            warmth(FILL) < -0.05,
            "the fill is {:?}, which does not cool what the sun misses",
            FILL
        );
    }

    /// Spec 0003: a band keeps its game's colour rather than burning white.
    ///
    /// A tint off a picture is pale, and multiplying a pale colour up puts
    /// every channel past 1 and the screen clamps all three. The first build
    /// of this gave thirteen white tubes.
    #[test]
    fn a_band_burns_in_its_own_colour() {
        let green = neon_of(glam::vec3(0.5, 1.0, 0.75)).truncate();

        assert!(
            green.y > green.x * 2.0,
            "a green picture gave a band of {:?}",
            green
        );
        assert!(green.y > 1.0, "the band does not glow: {:?}", green);
        assert!(
            green.x < 1.0,
            "every channel is past 1, so it clamps to white: {:?}",
            green
        );

        // and a picture with no colour in it lights a white tube
        let grey = neon_of(Vec3::splat(0.6)).truncate();
        assert!(
            (grey.x - grey.y).abs() < 1e-4 && grey.x > 1.0,
            "a colourless picture gave {:?}",
            grey
        );
    }

    /// Spec 0003: a screenshot lends its own colour and not its brightness.
    #[test]
    fn a_picture_lends_a_tint_rather_than_a_light() {
        use blitzkit::texture::TextureData;

        // a dim green picture and a bright one lend the same colour
        let dim = TextureData::from_pixels(1, 1, vec![0, 60, 0, 255]);
        let bright = TextureData::from_pixels(1, 1, vec![0, 255, 0, 255]);
        let apart = (glow_of(&dim) - glow_of(&bright)).abs().max_element();

        assert!(apart < 1e-3, "the dim one lends {:?}", glow_of(&dim));
        assert!(
            (glow_of(&bright).max_element() - 1.0).abs() < 1e-3,
            "a tint should reach 1.0 somewhere"
        );

        // and a black one lends white rather than nothing at all
        let black = TextureData::from_pixels(1, 1, vec![0, 0, 0, 255]);
        assert_eq!(glow_of(&black), Vec3::ONE);
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
