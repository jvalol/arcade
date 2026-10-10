//! The sight in the middle of the screen, and how bright a cabinet is. Spec
//! 0003.
//!
//! Nothing here draws. Where the sight goes and how much brighter the lit
//! cabinet is are arithmetic, so both can be checked without a window.

use glam::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

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

/// The lettering on a book's spine, per spec 0006.
///
/// Gilt, and dimmer than it looks written down: these are small enough that
/// bright lettering reads as a row of white dashes rather than as words.
pub const SPINE_LETTERS: Vec4 = vec4(0.86, 0.74, 0.44, 1.0);
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

/// What the room says about something nobody has compiled.
///
/// One sentence and not two. A cabinet in the hall and cascada's bench in the
/// nook are the same condition, which is a fresh checkout where only the arcade
/// has been built, and they had a line each. Two lists of the same thing is how
/// a sconce ended up hanging in a doorway.
///
/// It said "Not built. Run ./check-all", which is a shell command standing in
/// a room. True, and useful to exactly one person, who already knows. An
/// arcade has its own word for a machine that will not start and it is taped
/// to the glass: anyone can read it, and the one person who needs to know why
/// is the one person who can work it out.
pub const NOT_BUILT: &str = "Out of order";

/// What the cellar is furnished with: the racks, the bottles in them, the
/// barrels, and the hoops round the barrels. Spec 0007.
///
/// Dark. The one bright thing down there is the table and everything else is
/// what you can make out round it, which is what a room downstairs is for.
/// Glass is nearly black and takes its shape from the light that catches it.
pub const RACK: Vec4 = vec4(0.26, 0.17, 0.11, 1.0);
/// What a bottle of wine is made of, which is nearly black glass.
///
/// Four of them and not one. Bottles come in dead leaf green for reds, a
/// browner glass for some of them, and a pale one for whites, and a rack of
/// four hundred identical bright green ones is a crate of lager. These are all
/// darker than they look written down: glass holding red wine passes almost no
/// light, and what you see of a bottle in a cellar is the highlight on it
/// rather than the colour of it.
pub const BOTTLES: [Vec4; 4] = [
    vec4(0.035, 0.070, 0.040, 1.0),
    vec4(0.055, 0.045, 0.028, 1.0),
    vec4(0.030, 0.052, 0.055, 1.0),
    vec4(0.105, 0.115, 0.075, 1.0),
];

/// And the cork in the end of it, which is the one pale thing on a bottle and
/// most of what says the bottle is full.
pub const CORK: Vec4 = vec4(0.52, 0.40, 0.25, 1.0);

pub const BARREL: Vec4 = vec4(0.33, 0.21, 0.13, 1.0);
pub const HOOP: Vec4 = vec4(0.20, 0.19, 0.18, 1.0);
pub const LID: Vec4 = vec4(0.25, 0.16, 0.10, 1.0);

/// The one book that opens the wall. Spec 0007.
///
/// Over one in every channel, so it carries a little light of its own with
/// nothing lighting it, the way a cabinet's band does. Nothing like as much:
/// a band is at 3.2 and that is neon. This is a book that catches a light which
/// is not there, which is the most a secret door can give away and still be one.
///
/// Its own colour rather than its spine brightened. The spines come off a seed
/// and some of them are nearly black, so a multiplier would have made the
/// handle a slightly less dark book on a shelf of dark books, which is no tell
/// at all.
pub const BOOK_HANDLE: Vec4 = vec4(1.55, 1.30, 0.80, 1.0);

/// The pool table in the cellar: the cloth, and how thick a rail is. Spec 0007.
///
/// Green, because a pool table is green, and it is the one saturated thing in a
/// room of stone. The hall is neon and the nook is wood and books, so a cellar
/// with one green rectangle in it is a room with a pool table in it and not a
/// room with furniture.
/// And the mouth of a pocket, which is darker than anything else in the room,
/// because what you are looking at is a hole.
pub const POCKET: Vec4 = vec4(0.04, 0.035, 0.03, 1.0);
pub const CLOTH: Vec4 = vec4(0.10, 0.42, 0.22, 1.0);

/// The sights down the rails, in bone, and the net under a pocket, which is
/// leather and string and reads as neither if it is lit like timber.
pub const DIAMOND: Vec4 = vec4(0.88, 0.85, 0.76, 1.0);
pub const NET: Vec4 = vec4(0.17, 0.13, 0.10, 1.0);

/// What stands on the mantel. Spec 0007.
///
/// Brass, wax, a clock in a dark case with a pale face, and cut glass. A mantel
/// with nothing on it is a shelf, and the one thing a room with a fire in it
/// always has is a few things lined up over the fire.
pub const BRASS: Vec4 = vec4(0.60, 0.44, 0.17, 1.0);
pub const WAX: Vec4 = vec4(0.86, 0.80, 0.66, 1.0);
pub const WICK: Vec4 = vec4(2.8, 1.5, 0.5, 1.0);
pub const CLOCK_CASE: Vec4 = vec4(0.22, 0.10, 0.07, 1.0);
/// A shade over one in every channel, so the dial carries its own light the way
/// the book that opens the wall does. It faces into the room and the fire is in
/// the wall behind it, so lit only by what reaches it a clock face sits in its
/// own shadow, which is the one part of a clock that has to be readable.
pub const CLOCK_FACE: Vec4 = vec4(1.22, 1.14, 0.94, 1.0);
pub const CRYSTAL: Vec4 = vec4(0.54, 0.46, 0.40, 1.0);
pub const PORT: Vec4 = vec4(0.30, 0.05, 0.07, 1.0);

/// The way down and the room at the bottom of it. Spec 0007.
///
/// Stone, because a cellar is the one room in the building that is not finished.
/// The nook is panelled and carpeted and lit by its own sconces and the hall is
/// neon; this is under both of them and looks it. The treads are the nook's own
/// boards carried down the stair, because that is where the stair starts.
/// Warm. It was a neutral grey, which is what a basement is: the point of this
/// room is that it is the warmest in the building, warmer than the nook, and a
/// grey room with a fire in it is a grey room.
pub const CELLAR: Vec4 = vec4(0.40, 0.25, 0.20, 1.0);
pub const CELLAR_FLOOR: Vec4 = vec4(0.42, 0.21, 0.14, 1.0);

/// The panelling down there, in four timbers.
///
/// Oak, elm, a darker oak and something closer to walnut. One brown repeated
/// down a wall is hardboard; what makes a panelled room read as wood is that no
/// two boards beside each other are quite the same.
pub const TIMBER: [Vec4; 4] = [
    // mahogany, and three things that sit beside it: rosewood, a redder
    // mahogany, and walnut for the darkest bays
    vec4(0.46, 0.17, 0.11, 1.0),
    vec4(0.38, 0.14, 0.11, 1.0),
    vec4(0.52, 0.21, 0.13, 1.0),
    vec4(0.31, 0.15, 0.10, 1.0),
];

/// The stiles, rails, skirting and cornice, which are darker than the panels
/// they hold. A frame the same colour as its panel is not a frame.
pub const TIMBER_TRIM: Vec4 = vec4(0.26, 0.11, 0.08, 1.0);

/// The ceiling: its beams, the panel sunk in each coffer, and the carved boss
/// where four beams meet.
///
/// Darker than the walls, because a ceiling is further from every light in a
/// room and a ceiling the same value as the walls reads as no ceiling at all.
pub const RAFTER: Vec4 = vec4(0.30, 0.13, 0.09, 1.0);
pub const COFFER: Vec4 = vec4(0.38, 0.17, 0.12, 1.0);
pub const BOSS: Vec4 = vec4(0.52, 0.38, 0.17, 1.0);

/// The fireplace: its stone, the dark of the opening, the logs, and the fire.
///
/// The fire is far over one, because fire is the thing in a room that is
/// brighter than the room. Colour here is an unclamped multiplier, so this is
/// the one surface in the building that is a light as well as a thing.
pub const HEARTH: Vec4 = vec4(0.58, 0.40, 0.31, 1.0);
pub const FIREBOX: Vec4 = vec4(0.05, 0.03, 0.02, 1.0);
pub const LOG: Vec4 = vec4(0.17, 0.10, 0.06, 1.0);
pub const FIRE: Vec4 = vec4(5.2, 2.1, 0.5, 1.0);
pub const EMBER: Vec4 = vec4(3.4, 0.9, 0.2, 1.0);

/// A coal that is not currently alight, which is most of them.
pub const COAL: Vec4 = vec4(0.16, 0.08, 0.06, 1.0);

/// The dominoes on cascada's bench. Spec 0006.
///
/// Pale, because they are the one thing on a bench in that room meant to be
/// read as a picture rather than worked, and the benches under them are dark.
pub const DOMINO: Vec4 = vec4(0.84, 0.82, 0.76, 1.0);

/// The sign hung over the aisle. Spec 0006.
///
/// Painted wood and gilt rather than neon and a tube. The hall is a dim neon
/// box and the nook is the one room in the building that is not, so the thing
/// that sends you to it is made of what is down there and not of what is out
/// here. The gilt is the rug's thread and the globe's brass, which are the two
/// other warm things in the building.
/// The room's own neon sign, per `neon`.
///
/// The can is nearly black. It is the one surface in the room that wants to be
/// a hole with light in front of it, since a lit tube against a lit box is a
/// box.
///
/// Two colours and not one. A single tube reads as a lit bar, which is what the
/// cabinets were called before they had names on them, and every neon sign ever
/// bent has the outline in one colour and the word in another.
pub const ARCADE_CAN: Vec4 = vec4(0.05, 0.045, 0.06, 1.0);
pub const ARCADE_STEM: Vec4 = vec4(0.26, 0.26, 0.29, 1.0);

/// The tube and the lettering, both burnt at the multiplier the cabinets' bands
/// use, so one number says how hot the neon in this room runs.
pub fn arcade_tube() -> Vec4 {
    (glam::vec3(0.22, 0.95, 1.0) * NEON).extend(1.0)
}

pub fn arcade_letters() -> Vec4 {
    (glam::vec3(1.0, 0.26, 0.62) * NEON).extend(1.0)
}

/// The spa, per spec 0008. Tile and plaster rather than the cellar's mahogany:
/// panelling belongs in a library and would not last a week over a pool, which
/// is the kind of reason a room should be able to give for what it is made of.
pub const SPA_TILE: Vec4 = vec4(0.62, 0.59, 0.55, 1.0);
pub const SPA_WALL: Vec4 = vec4(0.74, 0.71, 0.67, 1.0);

/// The inside of the basin, which is what gives the water its colour: a
/// heightfield is see-through and most of what you look at through it is this.
pub const SPA_BASIN: Vec4 = vec4(0.30, 0.52, 0.56, 1.0);
pub const SPA_STEP: Vec4 = vec4(0.38, 0.58, 0.60, 1.0);

/// Cedar for the tub, pine for the sauna, iron for the stove.
pub const SPA_TUB: Vec4 = vec4(0.43, 0.25, 0.16, 1.0);
pub const SAUNA_TIMBER: Vec4 = vec4(0.64, 0.46, 0.27, 1.0);
pub const SAUNA_BENCH: Vec4 = vec4(0.72, 0.54, 0.33, 1.0);
pub const SAUNA_STOVE: Vec4 = vec4(0.15, 0.14, 0.14, 1.0);

/// The water itself, and what it is lit by.
///
/// Alpha under one, so spec 0018 of the engine draws it see-through and the
/// basin under it shows. A pool you cannot see into is a blue lid.
pub const WATER: Vec4 = vec4(0.20, 0.50, 0.60, 0.62);
pub const WATER_HOT: Vec4 = vec4(0.32, 0.56, 0.60, 0.74);
/// The sauna's glass, and the steam over hot water.
///
/// Both lean on spec 0018: alpha under one, drawn after everything solid. The
/// glass is barely tinted, because a sauna door is clear and what makes it read
/// as glass is the frame round it and the shine on it rather than its colour.
pub const SAUNA_GLASS: Vec4 = vec4(0.80, 0.86, 0.84, 0.26);
pub const SAUNA_FRAME: Vec4 = vec4(0.52, 0.37, 0.21, 1.0);
pub const STEAM: Vec4 = vec4(0.94, 0.95, 0.96, 1.0);

/// The sign over the door to the baths: glazed tile, a border round it, and the
/// name cut into it dark. Not neon, which is the hall, and not a painted plank,
/// which is the nook.
pub const BATHS_TILE: Vec4 = vec4(0.86, 0.88, 0.86, 1.0);
pub const BATHS_EDGE: Vec4 = vec4(0.33, 0.47, 0.50, 1.0);
pub const BATHS_LETTERS: Vec4 = vec4(0.13, 0.26, 0.30, 1.0);

/// And the door itself, which is a door between two rooms and made of what a
/// door is made of.
pub const BATHS_DOOR: Vec4 = vec4(0.36, 0.24, 0.15, 1.0);

/// And its glass, which is frosted rather than clear: you can see there is a
/// lit room through it and not what is in the room, which is the whole of what
/// a bathroom door's glass is for. Thicker than the sauna's, since frosting is
/// what stops you seeing through.
pub const FROSTED: Vec4 = vec4(0.80, 0.86, 0.85, 0.62);

/// The tiling and the fittings in the baths, per spec 0008.
///
/// Glazed tile is not plaster: it is darker, greener and it shines, which is
/// the difference between a bath house and a corridor.
pub const SPA_DADO: Vec4 = vec4(0.40, 0.56, 0.57, 1.0);
pub const SPA_BAND: Vec4 = vec4(0.20, 0.38, 0.42, 1.0);
pub const SPA_PIER: Vec4 = vec4(0.68, 0.66, 0.62, 1.0);
pub const SPA_INLAY: Vec4 = vec4(0.30, 0.44, 0.47, 1.0);

/// Cut stone for the plinths, the seat and the fountain's basin, terracotta for
/// the urns, and brass for everything a hand would touch.
pub const SPA_CUT: Vec4 = vec4(0.78, 0.75, 0.70, 1.0);
pub const SPA_URN: Vec4 = vec4(0.56, 0.32, 0.22, 1.0);

pub const SPA_LAMP: Vec3 = glam::vec3(0.86, 0.94, 1.0);
pub const SAUNA_LAMP: Vec3 = glam::vec3(1.0, 0.68, 0.34);

pub const SIGN_BOARD: Vec4 = vec4(0.20, 0.09, 0.07, 1.0);
pub const SIGN_GILT: Vec4 = vec4(0.70, 0.55, 0.26, 1.0);
pub const SIGN_CHAIN: Vec4 = vec4(0.22, 0.21, 0.23, 1.0);

/// The lettering, which is a shade over one so it carries from the far end of
/// the hall. Over one and no further: the cabinets' bands are at 3.2 and that
/// is neon, which is the one thing this sign is not.
pub const SIGN_LETTERS: Vec4 = vec4(1.35, 1.06, 0.50, 1.0);

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

/// The grid hung under it: the bars, and the tile behind them. Spec 0012.
///
/// The bar is darker than the tile and the tile is lighter than the lid, so
/// what you see is a lattice of dark lines running away down the aisle. Both
/// a shade off the lid itself, which is still what shows through where a
/// tile is missing.
/// What a collider is outlined in, per spec 0013.
///
/// Cool and past white, against a building lit warm by everything in it, so
/// an outline is never mistaken for a thing that is actually there.
pub const SOLID: Vec4 = vec4(0.0, 1.6, 0.9, 1.0);

pub const CEIL_BAR: Vec4 = vec4(0.055, 0.050, 0.065, 1.0);
pub const CEIL_TILE: Vec4 = vec4(0.150, 0.145, 0.165, 1.0);

/// How tight a highlight a matte surface gets.
///
/// Shininess is the power the half vector is raised to, so a small number is a
/// highlight spread over everything. The carpet went down at 4.0 and its dark
/// ground came back pale grey, because every texel of it was catching the sun.
pub const MATTE: f32 = 64.0;

/// For things that do not shine at all.
///
/// Shininess is a power, so a bigger number is a smaller highlight, and 64 is
/// small and bright: a hot spot. On a cask that is varnish. Oak does not do
/// that, and neither does stone.
pub const DULL: f32 = 900.0;

/// The garden, per spec 0010.
///
/// The sky is past one in every channel, which is how this engine says a thing
/// is lit rather than being lit: a storm is the brightest surface in the
/// building and nothing is shining on it. Neon is 3.2 and this is nearer to
/// daylight coming through cloud, so it is gentler than that and still over
/// the line.
pub const SKY: Vec4 = vec4(1.35, 1.40, 1.52, 1.0);

/// The gravel, which is pale stone, and the wall behind it, which is the
/// weathered plaster of a garden wall rather than the hall's.
pub const GRAVEL: Vec4 = vec4(0.78, 0.76, 0.70, 1.0);
pub const GARDEN_WALL: Vec4 = vec4(0.44, 0.43, 0.40, 1.0);

/// The pond, the stone round it, and what grows in the garden.
///
/// The water is darker than the baths', which is a lit pool with a tiled
/// floor. This one is rainwater under a storm, and what it mostly does is
/// hold the sky.
///
/// See-through, which it was not. Opaque it is a sheet of slate with a kerb
/// round it, and everything a pond is for is under it: there is no point
/// putting fish in a pond you cannot see into. Short of clear, because this is
/// rainwater and not a swimming bath, so the deepest koi is a shape going by
/// and the shallowest is a fish.
pub const POND: Vec4 = vec4(0.16, 0.23, 0.26, 0.78);

/// What a koi's skin is tinted by, which is as near nothing as this engine
/// gets. The patching is painted into the texture, so a tint here is a fish
/// seen through coloured glass.
pub const KOI: Vec4 = vec4(0.96, 0.94, 0.92, 1.0);

/// The bed of the pond, which is silt and not stone.
///
/// Cut from the same grey as the kerb, a pale floor under see-through water is
/// a tiled bath with a coping round it, which is what this came out as the
/// first time it was opened up. What is at the bottom of a pond is what has
/// fallen into it.
pub const POND_BED: Vec4 = vec4(0.11, 0.12, 0.09, 1.0);

/// The bank the water is held in, and the stones set round its lip.
///
/// The bank is dark, because what you see of it is the gaps between the stones
/// and a gap wants to read as shadow. The stones are between the bank and the
/// gravel: a pale run of coping is a swimming bath whatever shape it is cut
/// into, and the garden's own stonework is paler still.
pub const POND_BANK: Vec4 = vec4(0.16, 0.16, 0.15, 1.0);
pub const EDGE_STONE: Vec4 = vec4(0.25, 0.245, 0.23, 1.0);
pub const GARDEN_STONE: Vec4 = vec4(0.52, 0.51, 0.47, 1.0);
pub const BARK: Vec4 = vec4(0.27, 0.21, 0.17, 1.0);

/// The set stones, which are darker than anything else out here.
///
/// A rock reads against the gravel or it does not read. Cut near the kerb's
/// grey it is a lighter patch of floor; what every one of these gardens does
/// is put something nearly black on something nearly white.
pub const ROCK: Vec4 = vec4(0.21, 0.20, 0.19, 1.0);

/// The space behind the wall, per spec 0011: its floor, its walls and the
/// columns holding its lid up.
///
/// All near enough one colour, and that is the point. Every other room in here
/// is told apart from the next by what it is made of; this is told apart by
/// having nothing to tell apart.
///
/// Very dark, and darker than looks right written down. The sun and the fill
/// are the whole building's and reach in here too, so a surface that is merely
/// dim comes out as a lit floor running away to a lit horizon, which is a car
/// park rather than a thing you have found. At a twentieth the lamps are the
/// only light that does anything and everything past them is black.
///
/// The columns a shade off the walls and no more. Picked out, they are
/// furniture.
/// The plate behind the lines of text at the top left.
///
/// The text is drawn straight over whatever the room is doing, and the room is
/// sometimes a neon sign four feet from your face. White on cyan is not
/// readable, and which bit of the building happens to be behind a line of help
/// is not something the help can know.
///
/// Dark and see-through, so it is a shade over the room rather than a bar
/// across it. Black rather than grey, because grey at this alpha over a dark
/// room is a grey box and over a bright one is still grey: black takes
/// whatever is behind it down by the same amount wherever you stand, which is
/// the whole point.
pub const PLATE: Vec4 = vec4(0.0, 0.0, 0.0, 0.62);

/// How much wider and taller the plate is than the words on it.
pub const PLATE_PAD: f32 = 7.0;

/// Where the first line of help sits, how big it is drawn and how far apart
/// the lines are.
///
/// One account of all three. They were three: a line at twenty, a line at
/// forty-four, and nothing saying the second was the first plus a line.
pub const SAYS_AT: Vec2 = vec2(20.0, 20.0);
pub const SAYS_SIZE: f32 = 14.0;
pub const SAYS_LINE: f32 = 24.0;

/// Where the nth line of help goes.
pub fn says_at(n: usize) -> Vec2 {
    SAYS_AT + Vec2::Y * SAYS_LINE * n as f32
}

/// How thick the line round a plate is, and what colour it is.
///
/// A border and not only a shade. Jake asked for both: a shade alone over a
/// busy room still leaves the text sitting in the middle of whatever is behind
/// it, and the line is what says the words are a thing in front rather than a
/// thing in the room.
pub const PLATE_EDGE: f32 = 1.5;
pub const PLATE_LINE: Vec4 = vec4(1.0, 1.0, 1.0, 0.22);

/// A plate under a block of text: where its middle is, how big the shade is,
/// and how big the line round it is.
///
/// A middle and not a corner, because a `Quad`'s position is its middle. Given
/// a corner it draws the plate half its own width to the left of the words,
/// which is what it did: the first one of these was a corner and the shade sat
/// beside the text rather than under it.
pub struct Plate {
    pub at: Vec2,
    pub size: Vec2,
    pub line_size: Vec2,
}

/// The plate round a block of text, measured off the words.
///
/// Takes the lines as they are about to be drawn, each with where it goes and
/// how big it is, because the two blocks of text in this room are laid out
/// differently and neither of them should be laid out twice.
///
/// One plate for the block and not one a line. A line of this font is taller
/// than the gap between lines, so plates a line, each padded, lap over one
/// another, and two see-through quads in the same place are twice as dark as
/// one: the overlap shows as a band through the middle of the words.
pub fn plate_round(lines: &[(Vec2, &str, f32)], centred: bool) -> Option<Plate> {
    let (mut low, mut high) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));

    for (at, words, size) in lines.iter() {
        let room = blitzkit::text::room_for(words, *size);
        let from = if centred {
            vec2(at.x - room.x * 0.5, at.y)
        } else {
            *at
        };

        low = low.min(from);
        high = high.max(from + room);
    }

    if low.x > high.x {
        return None;
    }

    let size = high - low + Vec2::splat(PLATE_PAD * 2.0);

    Some(Plate {
        at: (low + high) * 0.5,
        size,
        line_size: size + Vec2::splat(PLATE_EDGE * 2.0),
    })
}

pub const BEHIND_FLOOR: Vec4 = vec4(0.013, 0.013, 0.016, 1.0);
pub const BEHIND_WALL: Vec4 = vec4(0.065, 0.065, 0.072, 1.0);
pub const BEHIND_COLUMN: Vec4 = vec4(0.085, 0.085, 0.092, 1.0);
pub const LEAF: Vec4 = vec4(0.17, 0.31, 0.19, 1.0);

/// What burns in a stone lantern: a candle behind paper, which is warm against
/// everything else out here being the colour of rain.
pub const LANTERN_LIT: Vec3 = vec3(1.0, 0.82, 0.52);

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

    /// Spec 0003: every line of text sits inside its plate.
    ///
    /// The text is drawn straight over whatever the room is doing, and the
    /// room is sometimes a neon sign four feet from your face or a pale table
    /// under a lamp. Which bit of the building is behind a line is not
    /// something the line can know, so it brings its own background.
    ///
    /// Both blocks, laid out two different ways. The one that was hardest to
    /// read was the centred pair under the sight, and the first plate written
    /// for this only went under the corner.
    #[test]
    fn every_line_of_text_sits_on_its_plate() {
        let help = [
            "WASD and the mouse to get about. Arrow keys look around, and work the toy you are looking at. Escape quits.",
            "If you know, you know. If you don't, play with one.",
        ];
        let window = (1600.0f32, 1200.0f32);

        for many in 1..=help.len() {
            let laid: Vec<(Vec2, &str, f32)> = help[..many]
                .iter()
                .copied()
                .enumerate()
                .map(|(n, words)| (says_at(n), words, SAYS_SIZE))
                .collect();

            inside(&laid, false, "the corner");
        }

        let said = [
            (prompt_at(window, 0), "ball and chain", PROMPT_SIZE),
            (
                prompt_at(window, 1),
                "27 of 27 standing. Swing the ball with arrow keys.",
                DETAIL_SIZE,
            ),
        ];
        for many in 1..=said.len() {
            inside(&said[..many], true, "the prompt");
        }
    }

    /// Every line of a block falls inside the plate, and the line is round it.
    fn inside(lines: &[(Vec2, &str, f32)], centred: bool, what: &str) {
        let plate = plate_round(lines, centred).expect("a plate");

        for (at, words, size) in lines.iter() {
            let room = blitzkit::text::room_for(words, *size);
            let from = if centred {
                vec2(at.x - room.x * 0.5, at.y)
            } else {
                *at
            };

            // the plate is given as a middle, so it runs half its size either
            // way from there
            let low = plate.at - plate.size * 0.5;
            let high = plate.at + plate.size * 0.5;

            assert!(
                low.x <= from.x + 1e-4 && low.y <= from.y + 1e-4,
                "{} with {} lines: one starts outside the plate",
                what,
                lines.len()
            );
            assert!(
                high.x >= from.x + room.x - 1e-4 && high.y >= from.y + room.y - 1e-4,
                "{} with {} lines: one runs off the plate",
                what,
                lines.len()
            );
        }

        assert!(
            plate.line_size.x > plate.size.x && plate.line_size.y > plate.size.y,
            "{}: the line is not round the plate",
            what
        );
    }

    /// Nothing to plate is no plate, rather than a dot in the corner.
    #[test]
    fn no_words_is_no_plate() {
        assert!(plate_round(&[], false).is_none());
    }

    /// The plate is a shade with a line round it, not a bar across the top.
    #[test]
    fn the_plate_is_a_shade_and_not_a_bar() {
        assert!(
            PLATE.w > 0.3 && PLATE.w < 0.9,
            "a plate at {} alpha is either nothing or a bar",
            PLATE.w
        );
        assert!(
            PLATE.x < 0.1 && PLATE.y < 0.1 && PLATE.z < 0.1,
            "the plate is not dark: {:?}",
            PLATE
        );
        assert!(
            PLATE_LINE.w > 0.05 && PLATE_LINE.w < PLATE.w,
            "the line round it is either invisible or heavier than the shade"
        );

        // and it fits the words, so it cannot be the width of the window
        let narrow = plate_round(&[(Vec2::ZERO, "short", SAYS_SIZE)], false).expect("a plate");
        let wide = plate_round(
            &[(
                Vec2::ZERO,
                "a good deal longer than that one was",
                SAYS_SIZE,
            )],
            false,
        )
        .expect("a plate");

        assert!(
            wide.size.x > narrow.size.x,
            "the plate does not follow the words"
        );
    }

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
    ///
    /// It used to answer that with a drop shadow of itself, and this test used
    /// to check the shadow. The shadow is gone: a shade with a line round it
    /// does the same job and does it over a pale table as well as a dark one,
    /// and two mechanisms for one job is one too many. The test kept its name
    /// and changed what it reads, because the name is the promise and the
    /// shadow was only ever how it was kept.
    #[test]
    fn the_prompt_carries_its_own_background() {
        let window = (1280.0f32, 800.0f32);
        let said = [
            (prompt_at(window, 0), "a thing with a name", PROMPT_SIZE),
            (
                prompt_at(window, 1),
                "and a line saying what to press",
                DETAIL_SIZE,
            ),
        ];
        let plate = plate_round(&said, true).expect("a plate");

        assert!(
            plate.size.x > 0.0 && plate.size.y > 0.0,
            "the prompt has no plate under it"
        );
        assert!(
            PLATE.w > 0.5,
            "a plate at {} alpha is not a background",
            PLATE.w
        );
        assert!(
            PLATE.x + PLATE.y + PLATE.z < sight_colour(true).truncate().element_sum(),
            "the plate is no darker than what it sits under"
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
