//! The bridge at the end of the trail: a rainbow out of the field, going up.
//! Spec 0054.
//!
//! **Nothing here exists until the building has been done.** Every room that
//! asks you something has to have given up its answer, which is what
//! `trail::Asked` lists and what [`shows`] reads. Until then this draws
//! nothing and stands nothing up, and spec 0036's rule holds: somebody who
//! never finds the trail cannot tell it is there.
//!
//! **A crossing and not a room.** Spec 0036 warns that inventing a fourth
//! room as the prize is how you get a room nobody wanted, and the warning is
//! why there is nothing at the top of this. What the climb gives you is
//! height and the only sight of the building from outside it: eleven rooms
//! walked and nobody has ever seen the arcade whole.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3, Vec4};

use crate::trail::{Asked, Trail};

/// How far west of the garden's ground the foot of it stands.
///
/// Far enough to see the whole arc from the gap you come out of, near
/// enough to walk to without the building going behind you.
pub const OUT: f32 = 18.0;

/// How far west it runs from there, and how high it climbs over that run.
///
/// **A hundred and sixty metres is the field's own reach**, so a hundred of
/// run with eighteen of standing-off leaves the top well inside the ground
/// rather than hanging over the edge of it. Twenty-six up is six storeys of
/// the hall.
///
/// The two are a pair and not two numbers: the rise over the run is what
/// decides whether this can be walked at all, and at these it is steepest
/// in the middle at a shade under half, which is twenty-five degrees.
pub const RUN: f32 = 100.0;
pub const RISE: f32 = 26.0;

/// How wide the deck is.
///
/// **Six metres, which is deliberately more than it needs.** A body is 0.9
/// across and every walkway in this building has been built too narrow
/// first: the nook went 3.0 to 6.4, the baths 9 to 11 and then wide enough
/// for walkways. A bridge you edge along is not a prize.
pub const WIDE: f32 = 6.0;

/// How many colours lie along it, and how thick the deck is.
pub const BANDS: usize = 7;
pub const DEEP: f32 = 0.35;

/// How many pieces the drawn deck is cut into.
///
/// Only the drawing. What holds you up is [`treads`], which is a different
/// and much finer list for a different reason, and both come out of
/// [`high`] so there is one curve rather than two.
pub const PIECES: usize = 48;

/// How long one tread of the collider is.
///
/// **This is what makes it walkable or not.** Colliders here are boxes that
/// cannot tilt, so a slope is a staircase to the thing walking up it, and a
/// riser taller than `walk::STEP` is a wall. At the steepest part of the
/// arc, a tread of 0.6 rises 0.28, and the walk steps 0.42.
///
/// **Walkable is not the same as smooth, which is what 0.6 missed.**
/// Jake: the walk up is jittery. Every riser at the steep part was most
/// of a foot, taken at seven metres a second, which is a dozen
/// twenty-eight centimetre hops a second and reads exactly as a jitter.
/// Clearing `walk::STEP` only says the stairs are climbable.
///
/// At 0.12 the riser at the steepest point is under six centimetres,
/// which is a kerb rather than a stair, and the staircase is close
/// enough to the ramp it is standing in for. It costs eight hundred
/// boxes where there were a hundred and sixty seven, which is nothing:
/// they are axis aligned and the walk tests them with six comparisons
/// each.
pub const TREAD: f32 = 0.12;

/// Where the last of the climb is done, as a share of the run.
///
/// **It ends level.** Arriving somewhere is standing on it, and a deck
/// still climbing at its last piece is a thing you walk off the end of.
pub const LEVELS_AT: f32 = 0.86;

/// The seven, in order from the foot.
///
/// Past one on purpose. Colour is an unclamped multiplier in this engine
/// and anything over one glows with no light on it, which is what a rainbow
/// is: light rather than paint. Neon in the cellar is 3.2 and this is
/// quieter than that, because the field has a sky behind it and a sun on it.
pub const COLOURS: [Vec3; BANDS] = [
    vec3(2.0, 0.42, 0.42),
    vec3(2.0, 1.02, 0.36),
    vec3(1.90, 1.74, 0.40),
    vec3(0.46, 1.86, 0.62),
    vec3(0.42, 1.10, 2.0),
    vec3(0.52, 0.56, 1.95),
    vec3(1.20, 0.52, 1.90),
];

/// What every piece of the deck is multiplied by, and so what the colours
/// in [`rainbow`] are divided by to get there.
///
/// **Because a picture cannot hold a colour past one and this needs to.**
/// Colour is an unclamped multiplier in this engine and anything over one
/// glows with no light on it, which is what a rainbow is. A texture is
/// eight bits a channel and stops at one. So the picture carries the hue
/// and this carries the glow, and `COLOURS` is still the one place the
/// seven are written.
pub const GLOW: f32 = 2.0;

/// How many texels across the rainbow's picture is drawn.
///
/// **Enough that the blend is a blend.** Seven flat bands is what this was
/// and Jake said the colours should run together the way a real one's do.
/// At 192 across six metres a texel is three centimetres, which is finer
/// than the eye resolves at the distance you stand from it, and the step
/// between two neighbouring texels is about a fortieth of the step between
/// two of the seven.
pub const RAINBOW_WIDE: u32 = 192;

/// The rainbow across the deck, as a picture that blends.
///
/// **The seven are stops and no longer bands.** Each sits at the middle of
/// the width it used to fill and the picture runs between them, so the
/// colours at those seven places are exactly what they always were and
/// everything between them is new. Outside the first and last stop it holds
/// the end colour rather than fading, because a rainbow's edges are where
/// it stops being a rainbow and not where it stops being bright.
pub fn rainbow() -> blitzkit::texture::TextureData {
    let mut pixels = vec![0u8; (RAINBOW_WIDE * 4) as usize];

    for at in 0..RAINBOW_WIDE {
        // where this texel falls among the stops, which sit at the middles
        // of the seven bands
        let across = (at as f32 + 0.5) / RAINBOW_WIDE as f32 * BANDS as f32 - 0.5;
        let below = across.floor().clamp(0.0, (BANDS - 1) as f32) as usize;
        let above = (below + 1).min(BANDS - 1);
        let share = (across - below as f32).clamp(0.0, 1.0);

        let colour = COLOURS[below] + (COLOURS[above] - COLOURS[below]) * share;
        let at = (at * 4) as usize;

        for (n, one) in [colour.x, colour.y, colour.z].iter().enumerate() {
            pixels[at + n] = ((one / GLOW).clamp(0.0, 1.0) * 255.0).round() as u8;
        }
        pixels[at + 3] = 255;
    }

    blitzkit::texture::TextureData::from_pixels(RAINBOW_WIDE, 1, pixels)
}

/// Whether the building has given up everything it asks.
///
/// **Every variant rather than a count.** A number here would go stale the
/// first time a room grows a question, and spec 0036 is plain that this
/// must never become "3 of 8 found". `Asked::every` is the one list.
pub fn shows(trail: &Trail) -> bool {
    Asked::every().iter().all(|one| trail.given().contains(one))
}

/// The name under the runes, which lives in [`crate::says`] with the
/// rest of the drafted copy so that signing it off is one command.
pub use crate::says::BIFROST as SAYS;

/// How hard the carving is burning, which breathes rather than holding
/// still.
///
/// **A steady glow is a lamp and a slow one is a thing that is alive.**
/// Jake asked for mystical, and what reads as that is not the brightness
/// but that it will not sit still: the garden's lanterns already come up
/// and go out over a long minute rather than switching, for the same
/// reason.
///
/// Two breaths crossed at lengths that do not divide, so it never repeats
/// on a count anybody could feel. Never down to nothing, because a rune
/// that goes out is a rune that was turned off.
pub fn burning(since: f32) -> f32 {
    let slow = (since * 0.41).sin();
    let under = (since * 0.17 + 1.3).sin();

    BURNS + slow * 0.16 + under * 0.07
}

/// What it burns at before the flicker.
///
/// **Raised, because the troughs were taking it out of the glow.**
/// `RUNE_CUT` is past one in red and under one in green and blue, which
/// is what makes it read as a hot colour rather than a white one. At
/// 0.76 the low end of the flicker pulled the red down near one as
/// well, so on a dark night there was nothing over one at all and the
/// carving was lit like any other paint: Jake's picture of it is a
/// brown mark on a grey stone.
pub const BURNS: f32 = 0.95;

/// And how much harder it burns at night.
///
/// Jake: the glyphs should especially glow at night. Which is also the
/// only time it can be seen to: a thing that glows is a thing brighter
/// than what is round it, and at noon nothing on a field is.
pub const NIGHTLY: f32 = 1.7;

pub fn glows(since: f32) -> f32 {
    burning(since) * (1.0 + NIGHTLY * nightly(since))
}

/// How much of the night it is: nought while the sun is up, one at the
/// bottom of it, and squared so the two twilights are the day's side of
/// halfway rather than the night's.
pub fn nightly(since: f32) -> f32 {
    let dark = 1.0 - (crate::garden::sun_up(since) * 0.5 + 0.5).clamp(0.0, 1.0);

    dark * dark
}

/// How white the heart of the glow goes at night.
///
/// **Because there is no bloom in this engine.** A colour past one is a
/// thing that is lit with nothing lighting it, and that is the whole of
/// it: nothing bleeds into the pixels round it. So a red of eight and a
/// green of three is not a light, it is an orange mark, and Jake's
/// first night picture of it reads exactly that way.
///
/// What a hot thing actually looks like is white in the middle and its
/// own colour at the edge. With no bloom to do that across the picture,
/// it is done in the colour: a pale amount added to every channel at
/// night, so the glyph goes from orange towards white as the light
/// goes. Jake: the glyphs should especially glow at night.
pub const RUNE_HOT: f32 = 0.45;

/// The colour the carved face is drawn in, glow and all.
///
/// **Alpha held under one on purpose**, per `aim::RUNE_GLAZE`: at one
/// the face goes through the engine opaque and is not drawn at all.
pub fn alight(since: f32) -> glam::Vec4 {
    let lit = crate::aim::RUNE_CUT * glows(since);
    let hot = RUNE_HOT * nightly(since);

    glam::vec4(
        lit.x + hot,
        lit.y + hot,
        lit.z + hot,
        crate::aim::RUNE_GLAZE,
    )
}

/// How fine the carved face is drawn.
pub const CARVED: u32 = 256;

/// The runes of the name, in younger futhark, as the strokes each is made
/// of in its own square.
///
/// **Because Latin capitals on a grey slab are a gravestone.** Jake,
/// 2026-11-05: it looks like a random grey slab, it does not land as a
/// nordic runestone. The lettering was the arcade's own pixel font, which
/// says arcade and not Norse however it is coloured.
///
/// Runes are strokes and nothing else, which is why they were cut into
/// stone and why they can be drawn here without a font: a stave down the
/// middle and branches off it. These are b, i, f, r, o, s, t, the name
/// spelled the way the sound goes rather than the way the Latin does.
///
/// Each is a list of lines in its own square, nought to one, y up.
const RUNES: [&[(f32, f32, f32, f32)]; 7] = [
    // bjarkan, b
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 1.0, 0.72, 0.78),
        (0.72, 0.78, 0.3, 0.55),
        (0.3, 0.55, 0.72, 0.33),
        (0.72, 0.33, 0.3, 0.1),
    ],
    // iss, i
    &[(0.5, 0.0, 0.5, 1.0)],
    // fe, f
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 0.86, 0.74, 0.62),
        (0.3, 0.58, 0.74, 0.34),
    ],
    // reid, r
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 1.0, 0.72, 0.8),
        (0.72, 0.8, 0.3, 0.56),
        (0.34, 0.52, 0.74, 0.05),
    ],
    // oss, o
    &[
        (0.3, 0.0, 0.3, 1.0),
        (0.3, 0.9, 0.72, 0.68),
        (0.3, 0.66, 0.72, 0.44),
    ],
    // sol, s
    &[
        (0.68, 1.0, 0.32, 0.72),
        (0.32, 0.72, 0.68, 0.44),
        (0.68, 0.44, 0.32, 0.14),
    ],
    // tyr, t
    &[
        (0.5, 0.0, 0.5, 1.0),
        (0.5, 1.0, 0.22, 0.68),
        (0.5, 1.0, 0.78, 0.68),
    ],
];

/// How wide a cut stroke is, as a share of the carved face.
/// How wide a cut stroke is, as a share of the carved face.
///
/// **Thin, and fattening it was the wrong fix.** At 0.026 a stroke was
/// half the width of the rune it belonged to, so every branch ran into
/// its own stave and consecutive runes touched: the chisel was wider than
/// the gaps. What the writing needed was to be bigger, not bolder.
pub const STROKE: f32 = 0.0115;

/// How finely the stone itself is cut, and how far its skin wanders at
/// each scale.
///
/// **Jake, 2026-11-05: it needs to be impressive, and the kind of
/// texturing that would take would be worth the vertices.** This is the
/// start of the last part of the trail. The lump the garden's set stones
/// are cut from is eleven round by seven up, which is right for something
/// the size of a loaf lying in gravel and is a smooth egg at two metres
/// tall: the granite picture gave it the look of rock and none of the
/// shape of one.
///
/// Twelve thousand quads for one object, which this building would not
/// spend anywhere else and spends here.
pub const STONE_ROUND: u32 = 176;
pub const STONE_STEPS: u32 = 224;
pub const KNOBBLE: f32 = 0.235;

/// How far the stone wanders from a ball at `u`, `v`.
///
/// **Five scales, each a whole number of waves.** Whole, because a wave
/// that does not come back to itself leaves a crack up the seam and a
/// tear at the poles, which is the rule `garden::wandered` already works
/// to; this one just goes finer, and the fine scales are what rock is.
pub fn knobble(seed: u32, u: f32, v: f32) -> f32 {
    use std::f32::consts::{PI, TAU};

    let mut state = seed | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };

    let mut out = 0.0;
    let mut weight = 0.0;

    // **Wave counts that are not multiples of each other.** Doubling
    // them, 1 2 4 8 16, lines every scale's crests up on the same
    // meridians and the stone comes out fluted. These share no factors,
    // so the scales never agree twice round.
    //
    // And not past about a quarter of the steps there are, or the finest
    // scale has two samples a wave and draws a shimmer rather than a
    // bump.
    for (waves, share) in [
        (1.0, 1.0),
        (3.0, 0.62),
        (7.0, 0.40),
        (13.0, 0.30),
        (23.0, 0.24),
        (41.0, 0.19),
        (67.0, 0.14),
    ] {
        let round = waves + (next() % 2) as f32;
        let up = waves + (next() % 3) as f32;
        let phase = (next() % 360) as f32 / 360.0 * TAU;

        out += (u * TAU * round + phase).sin() * (v * PI * up).sin() * share;
        weight += share;
    }

    // **The fine scales keep most of their weight.** Halving each one
    // against the last is how a landscape is made, and it draws a boulder
    // with gentle angles, which is what Jake called this. Real stone is
    // mostly small bumps with a shape somewhere under them.
    //
    // And the written part is taken back down to smooth, because a thin
    // line over that surface breaks up and stops being readable, which it
    // did. A runestone's face is dressed before it is cut.
    out / weight * KNOBBLE * (1.0 - 0.74 * dressed_by(u, v))
}

/// How far the face has been dressed flat at `u`, `v`: one in the middle
/// of the written part and nothing outside it.
///
/// **A rough stone and legible writing are one question.** Jake asked for
/// both in the same breath, and they pull against each other everywhere
/// except here: the shape of the stone carries on under the face and only
/// the small stuff comes off it.
pub fn dressed_by(u: f32, v: f32) -> f32 {
    // **Flat across the whole panel, and falling off outside it.** This
    // measured out from the middle of the face, so the dressing was
    // strongest at the centre and half gone at the edges. That was
    // survivable while the panel was a thin strip. Widened to a proper
    // dressed face, its sides sat where the stone is still half as
    // bumpy as it is anywhere, the bumps came through the carving, and
    // the ribbon vanished at every hour of the day: Jake's night
    // picture of a blank stone is this and not the glow.
    let across = over(1.0, (u - 0.5).abs() / face_wide(v));
    let middle = (FACE_FROM + FACE_TO) * 0.5;
    let along = over(0.5, (v - middle).abs() / (FACE_TO - FACE_FROM));
    let off = across.max(along);

    (1.0 - off * off).max(0.0)
}

/// Nought up to `edge`, one by half again past it: how far outside a
/// thing you are, for something that has to be wholly flat inside it.
fn over(edge: f32, at: f32) -> f32 {
    ((at - edge) / (edge * 0.6)).clamp(0.0, 1.0)
}

/// Where the stone's skin is at `u` round and `v` up, in its own unit.
///
/// **One account of the surface, which is the whole point.** The carving
/// is built from this too, so it lies on the rock instead of over it: it
/// was a flat slab standing proud of a lump, and you could see the plate.
/// How broad the stone is a given way up itself.
///
/// **A slab and not a column.** An egg is as narrow at the top as it is
/// at the foot, and a tall narrow lump with two round stones either side
/// of it looks like exactly what Jake said it looked like. A runestone is
/// a broad flat slab that holds its width nearly to the top and is
/// shouldered off above that.
///
/// Full width from the grass to better than two thirds up, then an arc
/// over. The foot closes to a point under the ground, where nobody sees
/// it.
pub fn girth(v: f32) -> f32 {
    // **Blunt at the crown, not pointed.** Smoothstepped off over the
    // top third it came out a cone, and a cone on a field is a haystack.
    // A stone that has stood a thousand years has a weathered top, so
    // this holds full width to near the head and then rounds over fast.
    let crown = ((0.995 - v) / 0.17).clamp(0.0, 1.0).powf(0.38);
    // and loses a little going up, the way a split block does
    let waist = 1.0 - 0.18 * (v - 0.25).max(0.0);
    let foot = (v / 0.04).clamp(0.0, 1.0).sqrt();

    crown * waist * foot
}

/// How much wider than its body the stone is at a height `v`: the
/// swelling at its foot.
///
/// **Jake: it needs to be a lot thicker at the bottom.** The first go at
/// that multiplied the whole profile by a term falling off over the
/// stone's entire height, which is the shape of a cone and read as one
/// from thirty metres. Weight low down is not a taper from top to
/// bottom; it is a body of roughly one width with a plinth swelling out
/// of the ground under it. This is the swelling, and it is gone by
/// `SWELLS_TO`.
pub const SWELLS: f32 = 1.05;
pub const SWELLS_TO: f32 = 0.62;

pub fn flare(v: f32) -> f32 {
    let low = ((SWELLS_TO - v) / SWELLS_TO).clamp(0.0, 1.0);

    1.0 + SWELLS * low * low
}

/// How much of the swelling the broad face gets.
///
/// **Less than the thin one.** The stone is a slab: 0.92 through and 1.6
/// across, and a swelling applied evenly to both keeps that ratio, so
/// the foot is a bigger slab rather than a heavier stone. Given mostly
/// to the thickness, the base reads as a block and the head still reads
/// as a face to carve.
pub const BROADENS: f32 = 0.45;

/// The stone's half-extents in metres at a height `v`, through the face
/// and across it. The shape as the eye reads it, which is the body, the
/// swelling and the two scales together and not any one of them.
pub fn half_at(v: f32) -> (f32, f32) {
    let body = girth(v) * 0.5;
    let swell = flare(v);

    (
        RUNE_THICK * body * swell,
        RUNE_WIDE * body * (1.0 + (swell - 1.0) * BROADENS),
    )
}

/// And where it meets the grass, which is what the heaped footing banks
/// against.
pub fn foot_half() -> (f32, f32) {
    half_at(RUNE_SUNK / RUNE_TALL)
}

pub fn stone_point(seed: u32, u: f32, v: f32) -> Vec3 {
    use std::f32::consts::TAU;

    // the face looks along +x at `u` of a half, which keeps the seam round
    // the back where nothing is written
    let round = -(u - 0.5) * TAU;
    let out = (0.5 + knobble(seed, u, v)) * girth(v);
    let lean = crate::garden::tip(seed, v);
    let swell = flare(v);

    vec3(
        round.cos() * out * swell + lean.x,
        v - 0.5,
        round.sin() * out * (1.0 + (swell - 1.0) * BROADENS) + lean.y,
    )
}

/// The stone.
pub fn stone_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        STONE_ROUND,
        STONE_STEPS,
        |u, v| stone_point(seed, u, v),
    ))
}

/// How much of the stone the carved face takes: half its width in `u`,
/// and the two `v` it runs between.
/// **A dressed face, with the ribbon a narrow thing down the middle of
/// it.** The band used to be drawn at the edges of this, which made the
/// frame three times wider than the seven runes it framed: a chain hung
/// on a hoarding. Seven runes stacked down a metre and a fifth cannot be
/// made much bigger than a hand, so what had to change was the frame,
/// not the lettering. The panel is wider than it was and the ribbon
/// hugs the text, leaving plain dressed stone either side of it, which
/// is what the carved stones actually look like.
pub const FACE_WIDE: f32 = 0.105;
pub const FACE_FROM: f32 = 0.52;
pub const FACE_TO: f32 = 0.86;
/// And how far the carving stands off the skin it is laid on.
/// **Proud enough to clear what is left of the stone's grain.** A
/// hundredth was the number while the panel sat in the flattest part of
/// the face. It is a dressed panel standing off a rough stone, and a
/// real one stands off it by more than a millimetre.
pub const FACE_PROUD: f32 = 0.03;

/// How much of the way round the stone the dressed face takes, at a
/// height `v`.
///
/// **Widening as the stone narrows, so the panel does not keystone.**
/// `FACE_WIDE` is a share of the way round, and a share of the way round
/// a tapering stone is a narrowing strip: once the foot was flared the
/// band's bottom row was twice the width of its top, and the name read
/// as if seen from below. Held to a constant arc instead, by giving back
/// exactly what the taper takes, the panel is a straight-sided board
/// with the stone falling away behind it.
pub fn face_wide(v: f32) -> f32 {
    let middle = (FACE_FROM + FACE_TO) * 0.5;

    FACE_WIDE * girth(middle) / girth(v).max(0.05)
}

/// The carved face: the stone's own surface over the written part of it,
/// lifted off itself by a hair.
///
/// **Built from `stone_point`, so every bump under it is the bump the
/// stone has.** A quad laid across a rock is a plate stuck on a rock, and
/// that is what Jake saw: the panel's own edge was visible against the
/// lump. This one is the rock.
pub fn carved_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let mut mesh = blitzkit::mesh::MeshData::surface(56, 72, |s, t| {
        // down the stone as the picture goes down, because the picture's
        // row nought is its top and `v` nought is this lump's foot.
        // Taken the other way the band stood on its head and the name
        // read along the crown.
        let v = FACE_TO - t * (FACE_TO - FACE_FROM);
        let u = 0.5 + (s - 0.5) * face_wide(v) * 2.0;
        let round = -(u - 0.5) * TAU;
        let at = stone_point(seed, u, v);

        at + vec3(round.cos(), 0.0, round.sin()) * FACE_PROUD
    });
    mesh.compute_normals();

    mesh
}

/// Every stroke of the carving, in the face's own square, nought to one.
///
/// **One account, because the stone is cut and not printed.** Jake,
/// 2026-11-05: there could be geometrical texture on which the glyphs are
/// laid, so they do not look so flat. A picture on a rock is a decal
/// however it is lit: what makes a carving read is that its edges take the
/// light differently from the face round them, and that wants geometry.
///
/// So this is the lines, once, and both the picture and the cut bars are
/// stood along them.
pub fn strokes() -> Vec<((f32, f32), (f32, f32))> {
    // **The ribbon's own numbers, not the runes'.** Its top used to be
    // `far`, which was also its width, and the two agreed only because
    // the band happened to be as wide as it was tall. Narrowed to the
    // lettering, `far` fell to 0.615 and took the top down with it: the
    // column ran out of the bottom of the picture and three of the
    // seven runes were drawn off the stone.
    let (near, far) = (0.385, 0.615);
    let (foot, shoulder, apex) = (0.065, 0.885, 0.945);
    let mut out = vec![
        // the band, round the face and open at the foot the way a
        // serpent's body leaves its own tail
        ((near, foot), (near, shoulder)),
        ((near, shoulder), (0.5, apex)),
        ((0.5, apex), (far, shoulder)),
        ((far, shoulder), (far, foot)),
    ];

    // and the runes down the middle of it, reading from the top. Taller
    // than wide, and the strokes run from 0.22 to 0.78 inside their own
    // square, so the width factor is near one to get a rune half as wide
    // as it is high.
    // **Drawn squat, because the face they land on is not square.** The
    // picture is a square and the patch it is laid on is about twice as
    // tall as it is wide, so anything drawn square comes out stretched
    // into a smear: the column of runes merged into one wiggling line and
    // Jake could not read a letter of it.
    //
    // So a rune is written about twice as wide as it is high here, to
    // arrive about twice as high as it is wide out there. The strokes run
    // from 0.22 to 0.78 inside their own square, which is in the factor.
    //
    // **And filling the ribbon.** At 2.05 against the old wide band the
    // column used a fifth of it. The ribbon moved rather than the
    // runes; this is the width that fills the ribbon and still arrives
    // about twice as high as it is wide out on the stone.
    let tall = 0.0778;
    let wide = tall * 1.3;
    let gap = 0.035;
    let top = 0.875;

    for (n, lines) in RUNES.iter().enumerate() {
        let foot = top - (n as f32 + 1.0) * (tall + gap);

        for (x0, y0, x1, y1) in lines.iter() {
            out.push((
                (0.5 + (x0 - 0.5) * wide, foot + y0 * tall),
                (0.5 + (x1 - 0.5) * wide, foot + y1 * tall),
            ));
        }
    }

    out
}

/// The carved face of the stone: a band round the edge with the runes of
/// the name standing in it.
///
/// **The band is what says runestone more than the runes do.** An Uppland
/// stone is an inscription inside a looping ribbon that runs round the
/// face, and the eye reads that outline before it reads a single
/// character. Without it this is strokes on a rock.
///
/// Cut away rather than painted on: the alpha is nought everywhere the
/// chisel did not go, so the stone shows between the strokes and the
/// engine's cutout throws the rest away.
pub fn carving() -> blitzkit::texture::TextureData {
    let n = CARVED;
    let mut pixels = vec![0u8; (n * n * 4) as usize];

    let mut ink = |x: f32, y: f32| {
        let (at_x, at_y) = ((x * n as f32) as i32, ((1.0 - y) * n as f32) as i32);

        if at_x < 0 || at_x >= n as i32 || at_y < 0 || at_y >= n as i32 {
            return;
        }

        let at = ((at_y as u32 * n + at_x as u32) * 4) as usize;

        for c in 0..3 {
            pixels[at + c] = 255;
        }
        pixels[at + 3] = 255;
    };

    let mut line = |from: (f32, f32), to: (f32, f32), thick: f32| {
        let steps = ((to.0 - from.0).hypot(to.1 - from.1) * n as f32 * 2.0).ceil() as i32;
        let wide = (thick * n as f32 * 0.5).ceil() as i32;

        for step in 0..=steps.max(1) {
            let on = step as f32 / steps.max(1) as f32;
            let (x, y) = (from.0 + (to.0 - from.0) * on, from.1 + (to.1 - from.1) * on);

            for dy in -wide..=wide {
                for dx in -wide..=wide {
                    if dx * dx + dy * dy > wide * wide {
                        continue;
                    }

                    ink(x + dx as f32 / n as f32, y + dy as f32 / n as f32);
                }
            }
        }
    };

    // the band, round the face and open at the foot the way a serpent's
    // body leaves its own tail
    for (from, to) in strokes() {
        line(from, to, STROKE);
    }

    // and the name in letters along the foot, composited into the same
    // picture rather than hung on a second quad. **One carved face.** A
    // separate plate for the Latin is the plate this whole change is
    // getting rid of.
    let said = blitzkit::text::stencilled(SAYS, RUNE_TEXELS);
    let (was_wide, was_tall) = (said.width() as f32, said.height() as f32);
    // **Sized by its height, not its width.** It was laid out to a
    // fixed share of the picture across, with the height following from
    // the aspect, which means the fewer letters there are the taller
    // they get. The marker came off when Jake signed the copy, the
    // string went from twenty one characters to seven, and the block
    // trebled in height and ran up into the rune ribbon. A cap height
    // is the thing that should not move when the words change.
    let want = (
        n as f32 * LATIN_TALL * was_wide / was_tall,
        n as f32 * LATIN_TALL,
    );
    let (at_x, at_y) = ((n as f32 - want.0) * 0.5, n as f32 * LATIN_FOOT);

    for down in 0..want.1 as u32 {
        for across in 0..want.0 as u32 {
            let from = (
                (across as f32 / want.0 * was_wide) as u32,
                (down as f32 / want.1 * was_tall) as u32,
            );
            let took = ((from.1.min(said.height() - 1) * said.width()
                + from.0.min(said.width() - 1))
                * 4) as usize;

            if said.levels[0].pixels[took + 3] < 128 {
                continue;
            }

            // the picture's row nought is its top and so is the text's,
            // so the block is placed by its top row and read forwards.
            // Walking the destination upwards instead turned every
            // letter over.
            let (to_x, to_y) = (
                at_x as u32 + across,
                n.saturating_sub(at_y as u32 + want.1 as u32) + down,
            );
            let at = ((to_y.min(n - 1) * n + to_x.min(n - 1)) * 4) as usize;

            for c in 0..3 {
                pixels[at + c] = 255;
            }
            pixels[at + 3] = 255;
        }
    }

    halo(&mut pixels, n);

    blitzkit::texture::TextureData::from_pixels(n, n, pixels)
}

/// How far the halo reaches round a cut, in texels, and how strongly.
pub const HALO_OUT: i32 = 4;
pub const HALO: f32 = 0.5;

/// Spreads a soft skirt of the same colour round everything already cut,
/// in the picture's own alpha.
///
/// **Because this engine has no bloom.** A colour past one is a thing
/// lit with nothing lighting it, and that is all it is: it does not
/// bleed into the pixels round it. So turning the carving up for the
/// night only made a brighter mark, and adding white to it only made a
/// paler one. Jake's two night pictures are those two answers.
///
/// What a glow actually is on screen is the thing plus light around the
/// thing, and here the cheapest place to put the light around it is in
/// the same picture: the glyph at full alpha, and a soft skirt of the
/// same amber at part alpha spreading off it. One texture, one draw,
/// and it is the glyph's own shape rather than a disc behind it.
///
/// The skirt goes under the cut and never over it: a glyph whose own
/// edge has been softened is a blurry glyph, and legibility is the one
/// thing this carving has cost the most rounds.
fn halo(pixels: &mut [u8], n: u32) {
    let was: Vec<u8> = (0..(n * n) as usize).map(|i| pixels[i * 4 + 3]).collect();
    let reach = (HALO_OUT * HALO_OUT) as f32;

    for y in 0..n as i32 {
        for x in 0..n as i32 {
            let at = ((y as u32 * n + x as u32) * 4) as usize;

            if was[(y as u32 * n + x as u32) as usize] > 0 {
                continue;
            }

            let mut near = 0.0f32;

            for dy in -HALO_OUT..=HALO_OUT {
                for dx in -HALO_OUT..=HALO_OUT {
                    let (sx, sy) = (x + dx, y + dy);

                    if sx < 0 || sy < 0 || sx >= n as i32 || sy >= n as i32 {
                        continue;
                    }

                    let off = (dx * dx + dy * dy) as f32;

                    if off > reach || was[(sy as u32 * n + sx as u32) as usize] == 0 {
                        continue;
                    }

                    near = near.max(1.0 - off / reach);
                }
            }

            if near <= 0.0 {
                continue;
            }

            for c in 0..3 {
                pixels[at + c] = 255;
            }

            pixels[at + 3] = (255.0 * near * near * HALO) as u8;
        }
    }
}

/// How tall the lettering is cut, how big the stone is, and how far it
/// leans.
///
/// **Off the side of the way on, not across it.** The deck is six metres
/// and you walk up the middle of it; a stone in that is a stone you walk
/// into on the way to the thing it names.
pub const RUNE_TEXELS: f32 = 64.0;

/// How tall the Latin name at the foot is, as a share of the carved
/// picture, and how far it sits off the bottom of it. Held rather than
/// derived, so signing the copy cannot move it.
pub const LATIN_TALL: f32 = 0.046;
pub const LATIN_FOOT: f32 = 0.012;
/// The whole length of the stone, foot and all, and how much of that is
/// under the grass.
///
/// **Jake: it has no reason to be standing that way.** A lump the shape
/// of an egg with its bottom pole exactly at ground level is an egg
/// balanced on its tip, and nothing says whether there is more of it
/// down there. The garden's set stones are the same shape and get away
/// with it because a third of each is under the gravel.
///
/// Better than a quarter of this one is, which is what a standing stone
/// needs to stay standing and is why the ones still up are still up. The
/// width at the grass line is what says so.
pub const RUNE_TALL: f32 = 3.6;
pub const RUNE_SUNK: f32 = 1.0;
pub const RUNE_WIDE: f32 = 1.6;
/// And how deep, which is the one that decides whether it looks like it
/// is about to fall over.
///
/// **Jake: it is still a really thin slab, and rocks around the base do
/// not make it look like it is not about to fall over.** Both true. At
/// 0.34 against two and a half metres standing it is a blade seen edge
/// on, and no amount of spoil round the foot buys what mass buys. The
/// stones that are still up after a thousand years are the thick ones.
pub const RUNE_THICK: f32 = 0.92;
/// How far from the stone's middle the heaped footing reaches, how high
/// it banks where it meets the stone, how many pieces it is broken into,
/// and how much grass there is between its rim and the bridge.
pub const RUBBLE_OUT: f32 = 2.15;
pub const RUBBLE_UP: f32 = 0.62;
pub const RUBBLE: usize = 160;
pub const RUBBLE_CLEARS: f32 = 0.5;

pub const RUNE_LEANS: f32 = 0.06;
/// How far the stone stands off the deck, measured from the deck's edge
/// on one side and from the deck's foot on the other.
///
/// **Which is the footing's reach and not the stone's.** It was 1.4,
/// back when the footing was a handful of stones that could be put
/// wherever there was room. A heaped bank has a footprint of its own,
/// and at 1.4 its edge came within two centimetres of the ramp's
/// corner. None of it is solid, so touching the ramp is rubble you walk
/// through. The clearance the stone needs is the clearance the footing
/// needs, and it is written here once and used for both directions.
///
/// Conservative, because the sum is measured along one axis while the
/// thing it has to miss is a half strip: the shortest line to the ramp
/// is a diagonal, so this leaves about 1.7 metres of grass where
/// `RUBBLE_CLEARS` only asks for half of one. Half a metre is the
/// requirement and `the_bank_keeps_off_the_bridge` is what holds it;
/// this is one way of satisfying it and not the tightest.
pub const RUNE_ASIDE: f32 = RUBBLE_OUT + RUBBLE_CLEARS;

/// How high the heaped footing stands a share `share` of the way from
/// the stone's foot to its rim, and how ragged its edge is at `round`.
///
/// One function because the bank and the loose pieces lying on it have
/// to agree: a stone floating a hand over the gravel is the fault that
/// comes of working them out twice.
pub fn mound_high(share: f32) -> f32 {
    RUBBLE_UP * (1.0 - share.clamp(0.0, 1.0)).powf(1.6)
}

pub fn mound_rim(round: f32) -> f32 {
    // **No one wave allowed to carry it.** At 0.11 the three-wave did,
    // and three big lobes round a low bank is the outline of a spill
    // rather than a heap. Spread over more waves, none of them large,
    // the edge wanders without taking a shape.
    //
    // The shares add to one, so the whole wander is `RUBBLE_RAGGED` and
    // the furthest the bank can reach is a number rather than a guess.
    // It has to be: the clearance from the bridge is worked out from it.
    let wobble = (round * 3.0).sin() * 0.26
        + (round * 7.0 + 1.9).sin() * 0.33
        + (round * 13.0 + 0.4).sin() * 0.24
        + (round * 23.0 + 2.6).sin() * 0.17;

    1.0 + wobble * RUBBLE_RAGGED
}

/// How far past `RUBBLE_OUT` the bank's edge can wander.
pub const RUBBLE_RAGGED: f32 = 0.21;

/// And so the furthest anything in the footing can be from the stone.
pub fn rubble_reaches() -> f32 {
    RUBBLE_OUT * (1.0 + RUBBLE_RAGGED)
}

/// The bank of spoil the stone is set into, as one skin.
///
/// **Because a scatter of pebbles on flat grass is a scatter of pebbles
/// on flat grass.** The first go at this was ninety loose stones in a
/// ring, and from thirty metres it read as a ring: you could see the
/// grass between them and the ground under them was dead level. What
/// says a stone was dug in and packed is that the ground itself is
/// raised round it. So the ground is raised, as a surface, and the loose
/// pieces lie on that rather than standing in for it.
///
/// Built in metres about the stone's middle, with its rim wandering, and
/// faceted, because a smooth dome of earth is a smooth dome of earth.
pub fn mound_mesh() -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let (hug_x, hug_z) = foot_half();

    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        MOUND_ROUND,
        MOUND_STEPS,
        |u, v| {
            let round = -u * TAU;
            let (cos, sin) = (round.cos(), round.sin());
            // from under the stone's own foot, so there is no seam of
            // grass showing between the bank and what it banks against
            let inner = (hug_x * cos).hypot(hug_z * sin) * 0.86;
            let rim = RUBBLE_OUT * mound_rim(round);
            let out = inner + v * (rim - inner).max(0.0);
            // and it thins to nothing at the rim rather than ending in a
            // step, with the surface itself broken up on the way down
            let rough = ((u * TAU * 9.0).sin() * (v * 11.0).sin()) * 0.035 * (1.0 - v);

            vec3(cos * out, mound_high(v) + rough, sin * out)
        },
    ))
}

pub const MOUND_ROUND: u32 = 112;
pub const MOUND_STEPS: u32 = 14;

/// The loose pieces lying on that bank: where each sits, how big it is,
/// and which way round.
///
/// Finest in close where it was packed and trodden, coarsest at the rim
/// where it was thrown clear, and every one of them sitting down into
/// the bank rather than resting on top of it.
///
/// A hundred and sixty pieces, which is cheap: these are the garden's
/// own eleven by seven lump, not the hundred and seventy six by two
/// hundred and twenty four the standing stone is. Seven of *those* put
/// more geometry in the footing than in the monument.
pub fn rubble(reaches: f32, trail: &Trail) -> Vec<(Vec3, Vec3, f32)> {
    if !shows(trail) {
        return Vec::new();
    }

    let (at, _) = runestone_at(reaches);
    let (hug_x, hug_z) = foot_half();

    let mut state = 0x5EED_u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };
    let roll = |v: u32| (v % 10_000) as f32 / 10_000.0;
    let mut out = Vec::with_capacity(RUBBLE);

    for _ in 0..RUBBLE {
        let round = roll(next()) * std::f32::consts::TAU;
        let (cos, sin) = (round.cos(), round.sin());

        let inner = (hug_x * cos).hypot(hug_z * sin) * 0.86;
        // **At its own direction, not the opposite one.** Written
        // `-round` the pieces read a mirrored rim, so round half the
        // heap they lay out past the bank's edge and round the other
        // half they stopped short of it.
        let rim = RUBBLE_OUT * mound_rim(round);
        let share = roll(next()).powf(1.2);
        let off = inner + share * (rim - inner).max(0.0);

        // **Coarse at the collar and fine at the rim, which is the
        // way round it was not.** Thrown-clear spoil at the edge and
        // gravel in close is one real thing, but the thing that ties a
        // raised stone to the ground is the packing: the wedges driven
        // in round the collar to hold it plumb. Those are the biggest
        // pieces on the site and they are the ones against the stone.
        let across = (0.30 - share * 0.20) * (0.7 + roll(next()) * 0.8);
        // sitting into the bank at that radius, by about a third of
        // itself, which is what stops them reading as dropped
        let up = mound_high(share) - across * 0.34;

        out.push((
            vec3(at.x + cos * off, up, at.z + sin * off),
            vec3(
                across,
                across * (0.45 + roll(next()) * 0.4),
                across * (0.8 + roll(next()) * 0.6),
            ),
            roll(next()) * std::f32::consts::TAU,
        ));
    }

    out
}

/// Gjallarhorn, in the watchman's hand. Spec 0055.
///
/// **The horn that has not been blown.** It is the one thing in the
/// myth that is only ever about to be used: Heimdall holds it through
/// the whole of the world's history and sounds it once, at the end. So
/// it hangs at his side with the bell forward and down rather than
/// being raised to his lips. A horn at the mouth is a warning already
/// given, and nothing about the walk up here should say the end has
/// come.
///
/// Built as a circle swept along an arc, widening from the mouthpiece
/// to the bell. Flat shaded with the rest of him, so it reads as a
/// carved horn rather than a brass instrument.
/// **Short and heavy, not long and thin.** At three quarters of a
/// metre of spine with an eighth of that at the bell it came out a
/// blade: from the side it read as a scythe hanging off his hand. A
/// horn is a stubby thing. Shorter, fatter at the mouth and curled
/// harder, it is a horn from any angle.
pub const HORN_LONG: f32 = 0.52;
pub const HORN_CURLS: f32 = 1.5;
pub const HORN_TIP: f32 = 0.028;
pub const HORN_BELL: f32 = 0.125;
/// How big it is in his hand. **Bigger than life**, because an ox horn
/// at true size on a figure at the top of a hundred metre bridge is a
/// stick, and this is the object the last of the trail is named for.
pub const HORN_HELD: f32 = 1.35;
pub const HORN_ROUND: u32 = 11;
pub const HORN_STEPS: u32 = 20;

/// Where the horn's spine is a share `along` of the way to the bell,
/// which way it is pointing there, and how thick it is.
pub fn horn_at(along: f32) -> (Vec3, Vec3, f32) {
    let turn = along * HORN_CURLS;
    let spine = vec3(
        HORN_LONG * turn.sin() / HORN_CURLS,
        HORN_LONG * (turn.cos() - 1.0) / HORN_CURLS,
        0.0,
    );
    let ahead = vec3(turn.cos(), -turn.sin(), 0.0);
    // **Cubed rather than tapered straight.** An ox horn is nearly a
    // point for its first third and nearly all bell in its last
    // quarter, and a straight cone between the two ends is a traffic
    // cone whichever way round you put it.
    let thick = HORN_TIP + (HORN_BELL - HORN_TIP) * along.powf(2.3);

    (spine, ahead, thick)
}

pub fn horn_mesh() -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    crate::garden::faceted(blitzkit::mesh::MeshData::surface(
        HORN_ROUND,
        HORN_STEPS,
        |u, v| {
            let (spine, ahead, thick) = horn_at(v);
            let round = -u * TAU;
            // the circle standing across the spine. `ahead` turns in
            // the x y plane, so across it is that turned a quarter,
            // and z is the one axis the arc never leaves.
            let side = vec3(-ahead.y, ahead.x, 0.0);

            spine + (side * round.cos() + Vec3::Z * round.sin()) * thick
        },
    ))
}

/// The stone at the foot of the bridge: where it stands and how big.
///
/// Empty until the bridge is there, like everything else here. A stone
/// naming a bridge that does not exist is a grave marker.
pub fn runestone(reaches: f32, trail: &Trail) -> Option<(Vec3, Vec3)> {
    shows(trail).then(|| runestone_at(reaches))
}

/// Whether a spot in the field is one the bridge keeps clear of trees.
///
/// **Because the field's scatter has never known the bridge is there.**
/// `field::open` keeps trees out of the building and out of nothing else,
/// so one grew through the runestone, and the low stretch of the deck runs
/// through the canopy of anything sown under it.
///
/// **Always, and not only once the bridge is up.** A clearing that appears
/// the moment somebody finishes the building is trees vanishing, which is
/// worse to watch than trees in a rainbow.
///
/// **And only the low stretch.** Past the point where the deck clears a
/// tree's head there is nothing to grow through, and a six metre avenue
/// running the whole hundred is a lane pointing at the one thing spec 0036
/// says nobody should be able to tell is there. This reaches as far as the
/// deck is lower than a tree and no further, which is about a fifth of it.
pub fn shades(reaches: f32, at: Vec3) -> bool {
    let foot = foot(reaches);
    let (stone, size) = runestone_at(reaches);

    // round the stone. **Its own test and not an else.** Written as the
    // far branch of how far off the deck's line a spot is, the stone fell
    // on the near side of that and so was never asked about: it sits four
    // and a half metres off a line whose own clearance is five.
    // measured off the heap and not the stone, which is the smaller
    // of the two now that the stone is set in a bank
    let footing = rubble_reaches();

    if (at.x - stone.x).abs() < (size.x * 0.5).max(footing) + CLEARS
        && (at.z - stone.z).abs() < (size.z * 0.5).max(footing) + CLEARS
    {
        return true;
    }

    // or under the low stretch of the deck
    if (at.z - foot.z).abs() > WIDE * 0.5 + CLEARS {
        return false;
    }

    let along = (foot.x - at.x) / RUN;

    (0.0..=1.0).contains(&along) && high(along) < crate::field::TREE_TALL
}

/// How much faster you get about while you are on the deck.
///
/// **Because the bridge is a road and not a hill.** Jake: it takes way
/// too long to run up the rainbow bridge. A hundred metres of span and
/// twenty six of climb is eleven or twelve seconds of holding one key
/// with nothing to do, every time, and the span is the thing that makes
/// it read from the far side of the field. Shortening it fixes the
/// climb by spoiling what the climb is for.
///
/// So the deck carries you. Bifröst is the road the gods ride to the
/// well every day, and a road that carries its traffic is the most
/// ordinary thing a mythic road could do.
///
/// **And the budget was the bigger half of it.** `walk::CLIMBS`
/// rations height, so twenty six metres of rise was floored at eleven
/// seconds whatever the pace, and the carry alone only made you walk
/// into risers faster. `walk::RAMPS` is the other half and the one
/// that mattered: the climb is three seconds at a walk now, measured
/// through the game's own loop by
/// `tests::the_bridge_is_walked_up_rather_than_climbed`.
///
/// **Only going up.** Coming down you have gravity and a view, and a
/// bridge that flings you back towards the building is a bridge
/// fighting you.
/// **Jake, twice: still laborious.** 1.8 was about five seconds at a
/// walk, which was my judgement of what a ride should feel like
/// against his of what it actually felt like. His wins.
pub const CARRIES: f32 = 2.8;

/// Whether a spot is on the deck, for the purpose of being carried.
///
/// Measured off the same `treads` the walking is done on, rather than
/// off the deck's drawn shape: two answers to where the bridge is would
/// come apart the first time either moved.
pub fn carries(reaches: f32, at: Vec3, going: Vec3, trail: &Trail) -> f32 {
    if !shows(trail) {
        return 1.0;
    }

    let foot = foot(reaches);

    if (at.z - foot.z).abs() > WIDE * 0.5 || at.y < 0.4 {
        return 1.0;
    }

    let along = (foot.x - at.x) / RUN;

    if !(0.0..=1.0).contains(&along) {
        return 1.0;
    }

    // and only while you are going up it, faded rather than switched.
    //
    // **A hard test here was its own jitter.** `going` is where the
    // camera looks, not where the feet go, so walking up the deck while
    // glancing to one side crossed `going.x == 0` and the carry
    // snapped on and off with the mouse. Faded, the worst a glance does
    // is slow you.
    let up = (-going.x).clamp(0.0, 1.0);

    1.0 + (CARRIES - 1.0) * up
}

/// How far from either a tree has to be sown.
///
/// **A canopy and not a trunk.** The field's trees are `TREE_TALL` high
/// and their crowns are about half that across, so a metre and a half of
/// clearance is a tree whose trunk misses the stone and whose branches sit
/// over it, which is what the first go at this drew.
pub const CLEARS: f32 = 3.0;

/// Where the stone stands, whether or not the bridge is up.
///
/// The clearing round it has to hold either way, so the place it stands is
/// its own question and [`runestone`] is that place plus the bridge.
fn runestone_at(reaches: f32) -> (Vec3, Vec3) {
    let foot = foot(reaches);

    (
        vec3(
            // behind the deck's foot by the same clearance it stands
            // aside by, because in both directions what has to miss the
            // bridge is the rubble's rim and not the stone. This was
            // `RUNE_WIDE`, which only ever agreed with the right answer
            // by accident.
            foot.x + RUNE_ASIDE,
            RUNE_TALL * 0.5 - RUNE_SUNK,
            foot.z + WIDE * 0.5 + RUNE_ASIDE,
        ),
        vec3(RUNE_THICK, RUNE_TALL, RUNE_WIDE),
    )
}

/// Where the foot of it stands, in the field.
pub fn foot(reaches: f32) -> Vec3 {
    let out = crate::field::starts(reaches) - OUT;
    let (from, to) = crate::field::way(reaches);

    vec3(out, 0.0, (from + to) * 0.5)
}

/// How high the deck is a share `along` of the way out.
///
/// **It eases out of the ground and eases into the top**, so the steepest
/// part is the middle and both ends are nearly flat. The first shape tried
/// here was a quarter circle, which rises fastest where it leaves the
/// ground: at the numbers above it went up 1.29 in its first 1.46, which is
/// a wall to walk at. `it_leaves_the_ground_at_a_walkable_angle` is that
/// mistake, kept.
pub fn high(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);
    let climbing = (along / LEVELS_AT).min(1.0);

    RISE * (1.0 - (climbing * std::f32::consts::PI).cos()) * 0.5
}

/// How steeply it climbs there, as a rise over the run.
///
/// The slope of [`high`], which is what a drawn piece is tilted by.
pub fn pitch(along: f32) -> f32 {
    let along = along.clamp(0.0, 1.0);

    if along >= LEVELS_AT {
        return 0.0;
    }

    let climbing = along / LEVELS_AT;

    RISE * std::f32::consts::PI * (climbing * std::f32::consts::PI).sin() / (2.0 * RUN * LEVELS_AT)
}

/// The kerb down either side of the ramp: how high it stands, how
/// thick it is, and how long each piece is.
///
/// **Because the deck had no edge at all.** Six metres across, a
/// hundred long, and nothing to either side of it. At four metres a
/// second that was survivable; carried at eleven it is not, because
/// three degrees off the straight is half a metre a second sideways
/// and the whole climb is five seconds of it. Jake went over the side
/// near the top and his recording catches him down in the field
/// looking back up at the bridge.
///
/// So the road gets a verge, which is the other thing that makes a
/// road a road. Higher than `walk::STEP`, so it stops you rather than
/// being walked up, and the two of them leave the deck's middle five
/// metres clear, which is wider than anything else you walk along in
/// this building.
///
/// Its pieces are ten treads long. Nothing stands on a kerb, so it
/// does not need the fine tread the deck needs to be smooth, and at
/// `TREAD` it would have been seventeen hundred boxes for a thing
/// whose whole job is to be in the way sideways.
pub const KERB_UP: f32 = 0.52;
pub const KERB_THICK: f32 = 0.5;
pub const KERB_LONG: f32 = TREAD * 10.0;
/// How far the drawn kerb stands outside the deck's edge, which is
/// only so that no face of it lies in a plane the deck already has.
pub const KERB_PROUD: f32 = 0.04;

/// The kerb as one swept ribbon per side, for looking at.
///
/// **Drawn as a sweep and collided as boxes.** `kerbs` is a row of
/// axis aligned blocks because that is the only thing the walk
/// understands, and drawing those blocks is a flight of stairs running
/// up either side of a rainbow: each piece a foot and a half longer
/// than the last is level on top, so the kerb went up in steps of half
/// a metre. Swept along the same curve the deck is swept along, it is
/// a line.
pub fn kerb_mesh(reaches: f32, side: f32) -> blitzkit::mesh::MeshData {
    let foot = foot(reaches);
    let middle = foot.z + side * (WIDE * 0.5 - KERB_THICK * 0.5);
    let half = KERB_THICK * 0.5;

    let mut mesh = blitzkit::mesh::MeshData::surface(PIECES as u32, 3, |u, v| {
        let along = u;
        let on = high(along);
        let x = foot.x - along * RUN;
        // round the kerb's cross section: down the outer face, over the
        // top, down the inner face
        // **Clear of the deck's own faces.** Standing exactly on the
        // deck's edge, the kerb's outer face shared a plane with it and
        // its underside shared one with the deck's underside, and both
        // came out speckled: the fault this building has found eight
        // times. Nudged out and cut short, neither touches.
        let out = middle + half + KERB_PROUD;
        let under = on - DEEP * 0.55;
        let (z, y) = match v {
            v if v < 0.34 => (out, under),
            v if v < 0.67 => (out, on + KERB_UP),
            v if v < 0.99 => (middle - half, on + KERB_UP),
            _ => (middle - half, under),
        };
        vec3(x, y, z)
    });
    mesh.compute_normals();

    mesh
}

/// Where each piece of kerb sits: its box, in the same terms as
/// [`treads`].
pub fn kerbs(reaches: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let pieces = (RUN / KERB_LONG).ceil() as usize;
    let mut out = Vec::with_capacity(pieces * 2);

    for piece in 0..pieces {
        let from = piece as f32 * KERB_LONG / RUN;
        let to = ((piece + 1) as f32 * KERB_LONG / RUN).min(1.0);
        let top = high(to).max(high(from)) + KERB_UP;

        for side in [-1.0f32, 1.0] {
            let middle = foot.z + side * (WIDE * 0.5 - KERB_THICK * 0.5);

            out.push(Aabb::new(
                vec3(
                    foot.x - to * RUN,
                    top - KERB_UP - DEEP,
                    middle - KERB_THICK * 0.5,
                ),
                vec3(foot.x - from * RUN, top, middle + KERB_THICK * 0.5),
            ));
        }
    }

    out
}

/// What holds you up: one box the full width, every [`TREAD`] along.
///
/// **A different list from the drawn deck, and the reason is the engine.**
/// A collider is an axis-aligned box and cannot tilt, so a slope has to be
/// a staircase to walk on, and a staircase fine enough to walk is far too
/// fine to draw as boxes. Both come out of `high`, which is the one place
/// the shape is written.
pub fn treads(reaches: f32) -> Vec<Aabb> {
    let foot = foot(reaches);
    let steps = (RUN / TREAD).ceil() as usize;
    let mut out = Vec::with_capacity(steps);

    for step in 0..steps {
        let from = step as f32 * TREAD / RUN;
        let to = ((step + 1) as f32 * TREAD / RUN).min(1.0);
        let top = high(to).max(high(from));

        out.push(Aabb::new(
            vec3(foot.x - to * RUN, top - DEEP, foot.z - WIDE * 0.5),
            vec3(foot.x - from * RUN, top, foot.z + WIDE * 0.5),
        ));
    }

    out
}

/// The middle of the deck's top surface, every piece of the way along.
///
/// **The one account of where the deck is**, which the mesh is built from
/// and the tests read. It is the same curve [`treads`] steps, so what you
/// see and what holds you up are two readings of [`high`] and not two
/// shapes.
pub fn stations(reaches: f32) -> Vec<Vec3> {
    let foot = foot(reaches);

    (0..=PIECES)
        .map(|at| {
            let along = at as f32 / PIECES as f32;

            vec3(foot.x - along * RUN, high(along), foot.z)
        })
        .collect()
}

/// What every bit of the deck is multiplied by.
///
/// The rainbow's own colours live in [`rainbow`], which is a picture and so
/// cannot hold anything past one. This carries the glow that the picture
/// cannot.
pub fn glow() -> Vec4 {
    Vec4::new(GLOW, GLOW, GLOW, 1.0)
}

/// The whole deck as one ribbon, swept along the arc.
///
/// **One mesh and not forty-eight boxes.** It was boxes, each tilted to the
/// pitch where it lay and lapping the next, and that worked for exactly as
/// long as every face of one was a single flat colour: a join you cannot
/// see is a join nobody minds. The moment the rainbow became a picture laid
/// across the width, every one of those joins turned into a bright chevron
/// and every end cap into a stripe, because an end cap wears the picture
/// too. Shrinking the lap only swapped chevrons for seams.
///
/// A ribbon has no joins to show. It is also one instance rather than
/// forty-eight, and its top surface passes through [`high`] at every
/// station rather than near it.
///
/// **Built for a given building rather than scaled to one.** The pitch
/// changes along the arc, so a unit mesh stretched to length is a straight
/// plank. `garden::trunk_mesh` is built per tree for the same reason.
///
/// **Its own faces, each with its own normal.** Four strips run the length
/// of it and the corners between them are sharp, so the vertices are not
/// shared across a corner: averaged round one, the top of a rainbow catches
/// the same light as its edge and the whole thing reads as a hose.
pub fn deck_mesh(reaches: f32) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let along = stations(reaches);
    let half = WIDE * 0.5;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // the four strips, as which corner of the cross-section each runs
    // between and which way it looks. The corners are numbered round it:
    // 0 top south, 1 top north, 2 under north, 3 under south.
    let strips: [(usize, usize, i32); 4] = [(0, 1, 0), (1, 2, 1), (2, 3, 2), (3, 0, 3)];

    for (near, far, looks) in strips {
        let first = vertices.len() as u32;

        for (at, middle) in along.iter().enumerate() {
            // how the deck lies here, which is the slope of the curve: the
            // cross-section stands square to it rather than square to the
            // world, or the deck is thicker where it is steeper
            let slope = pitch(at as f32 / PIECES as f32);
            let over = vec3(slope, 1.0, 0.0).normalize();
            let under = *middle - over * DEEP;

            let corner = |which: usize| match which {
                0 => *middle - Vec3::Z * half,
                1 => *middle + Vec3::Z * half,
                2 => under + Vec3::Z * half,
                _ => under - Vec3::Z * half,
            };
            // across the width, so the picture lies the same way on the
            // top, the underside and both edges
            let across = |which: usize| if which == 1 || which == 2 { 1.0 } else { 0.0 };
            let normal = match looks {
                0 => over,
                1 => Vec3::Z,
                2 => -over,
                _ => -Vec3::Z,
            };
            let down = at as f32 / PIECES as f32;

            for which in [near, far] {
                let put = corner(which);

                vertices.push(Vertex::new(
                    [put.x, put.y, put.z],
                    [normal.x, normal.y, normal.z],
                    [across(which), down],
                ));
            }
        }

        for at in 0..PIECES as u32 {
            let one = first + at * 2;

            indices.extend([one, one + 1, one + 3, one, one + 3, one + 2]);
        }
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// The landing at the top: how far it runs on past the deck, how wide it is
/// across, and how thick its slab is.
///
/// **Ten across against the deck's six.** Arriving wants to be stepping out
/// onto something rather than reaching the last plank, and two metres either
/// side is the difference you feel underfoot without it reading as a
/// courtyard. Seven deep is three strides, which is room to turn round and
/// look back and nothing more.
pub const LANDING_OUT: f32 = 7.0;
pub const LANDING_WIDE: f32 = 10.0;
pub const LANDING_DEEP: f32 = 0.5;

/// The parapet round it: how high it stands over the landing and how thick.
///
/// **Low, because it is a kerb and not a wall.** What it is for is that a
/// slab twenty-six metres up with nothing round it is a diving board, and
/// that a platform with an edge reads as somewhere to stand. Head height
/// would take away the view, which is the prize.
pub const RAIL_UP: f32 = 0.62;
pub const RAIL_THICK: f32 = 0.34;

/// How wide the way through the far parapet is.
///
/// **Because spec 0056 is through there.** The parapet was built on three
/// sides to stop somebody walking off a twenty-six metre drop, which it
/// does, and that included the side Valhalla is on: the walk test for
/// getting across fetched up eight tenths of a metre short of the edge,
/// against the rail. A gate with no gap in it is a wall.
///
/// On the line you walk in on, so the deck, the gap and the hall's door
/// are one straight way through.
pub const WAY_WIDE: f32 = 3.0;

/// Where the top of the deck is: the far end of the run, at full height.
pub fn top(reaches: f32) -> Vec3 {
    let foot = foot(reaches);

    vec3(foot.x - RUN, high(1.0), foot.z)
}

/// The landing, as its slab and the three sides of parapet round it.
///
/// Empty until the building has been done, like everything else here.
///
/// **It laps the deck rather than meeting it.** Two faces at the walking
/// surface stop a walk dead and `walk` reports nothing whatever, which this
/// building has been caught by three times in one day before now. The slab
/// runs back under the last stretch of deck by a tread's length.
pub fn landing(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !shows(trail) {
        return Vec::new();
    }

    let top = top(reaches);
    // east is where you arrive from, so the slab starts back under the deck
    let east = top.x + TREAD;
    let west = top.x - LANDING_OUT;
    let (south, north) = (top.z - LANDING_WIDE * 0.5, top.z + LANDING_WIDE * 0.5);

    vec![
        // the slab, its top flush with the deck's
        Aabb::new(
            vec3(west, top.y - LANDING_DEEP, south),
            vec3(east, top.y, north),
        ),
        // and the parapet on the three sides you do not arrive over. Each
        // one runs the full corner so that two of them meet rather than
        // leaving a notch you could step through.
        //
        // The far one is in two pieces with the way through between them,
        // the way the garden's west wall is split round the way out to the
        // field.
        Aabb::new(
            vec3(west, top.y, south),
            vec3(west + RAIL_THICK, top.y + RAIL_UP, top.z - WAY_WIDE * 0.5),
        ),
        Aabb::new(
            vec3(west, top.y, top.z + WAY_WIDE * 0.5),
            vec3(west + RAIL_THICK, top.y + RAIL_UP, north),
        ),
        Aabb::new(
            vec3(west, top.y, south),
            vec3(east, top.y + RAIL_UP, south + RAIL_THICK),
        ),
        Aabb::new(
            vec3(west, top.y, north - RAIL_THICK),
            vec3(east, top.y + RAIL_UP, north),
        ),
    ]
}

/// Where the watchman stands, and which way he faces.
///
/// **Off the line you walk in on, and not off the end of it.** Dead centre
/// he is a man standing in a doorway and you would walk into him: a body is
/// 0.9 across. Out at a third of the landing's width he is exactly on the
/// edge of the deck's own six, which is past the lane you come up and so is
/// somebody you never pass. A fifth of it leaves a metre of daylight either
/// side of two bodies and still puts him in front of you.
///
/// **Facing the climb**, which is back east down the deck. The yaw is the
/// one `draw_the_people` reads, where ahead is `(sin, 0, -cos)`, so east is
/// a quarter turn.
pub fn watchman(reaches: f32) -> (Vec3, f32) {
    let top = top(reaches);

    (
        vec3(
            top.x - LANDING_OUT * 0.48,
            top.y,
            top.z + LANDING_WIDE * 0.20,
        ),
        std::f32::consts::FRAC_PI_2,
    )
}

/// What you can stand on, which is the treads and nothing else.
pub fn solid(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !shows(trail) {
        return Vec::new();
    }

    let mut out = treads(reaches);
    out.extend(kerbs(reaches));
    out.extend(landing(reaches, trail));
    out.extend(runestone(reaches, trail).map(|(at, size)| Aabb::from_center_size(at, size)));

    out
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// One question short of done, which is what the bridge must not show
    /// for.
    fn nearly() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every().iter().skip(1) {
            trail.gave(*one);
        }

        trail
    }

    #[test]
    fn nothing_until_the_building_is_done() {
        let empty = Trail::new();

        assert!(!shows(&empty));
        assert!(solid(REACHES, &empty).is_empty());
    }

    #[test]
    fn one_short_is_not_done() {
        let nearly = nearly();

        assert!(!shows(&nearly), "one answer short and it is still there");
        assert!(solid(REACHES, &nearly).is_empty());
    }

    #[test]
    fn it_is_there_once_every_room_has_answered() {
        assert!(shows(&done()));
        assert_eq!(
            solid(REACHES, &done()).len(),
            (RUN / TREAD).ceil() as usize
                + kerbs(REACHES).len()
                + landing(REACHES, &done()).len()
                + runestone(REACHES, &done()).iter().count()
        );
    }

    /// Jake, twice: it reads as a slab with no reason to be standing, and
    /// then that it is still a thin slab. The answer both times was mass
    /// low down, so this holds the profile's shape rather than a number.
    #[test]
    fn the_stone_is_heaviest_where_it_leaves_the_ground() {
        let grass = RUNE_SUNK / RUNE_TALL;
        let (foot, _) = half_at(grass);
        let (shoulder, _) = half_at(0.78);

        assert!(
            foot > shoulder * 1.4,
            "{:.3} through at the grass line against {:.3} at the shoulder",
            foot,
            shoulder
        );

        // and it only ever loses width going up from there: a bulge
        // anywhere above the grass is a stone balanced on a waist
        let mut last = f32::INFINITY;

        for step in 0..=60 {
            let v = grass + (0.92 - grass) * step as f32 / 60.0;
            let (now, _) = half_at(v);

            assert!(now <= last + 1e-4, "it widens again at {:.3}", v);
            last = now;
        }

        // and it is a slab and not a post: wider across the face than it
        // is through, all the way up
        for step in 0..=20 {
            let v = grass + (0.92 - grass) * step as f32 / 20.0;
            let (through, across) = half_at(v);

            assert!(across > through * 1.15, "it is square at {:.3}", v);
        }
    }

    /// The carved panel keeps one width down a stone that does not, which
    /// is what stops the name reading as if seen from below.
    #[test]
    fn the_carved_panel_is_one_width_all_the_way_down() {
        let (mut least, mut most) = (f32::INFINITY, 0.0f32);

        for step in 0..=40 {
            let v = FACE_FROM + (FACE_TO - FACE_FROM) * step as f32 / 40.0;
            let arc = face_wide(v) * girth(v);

            least = least.min(arc);
            most = most.max(arc);
        }

        assert!(
            most - least < least * 0.01,
            "the band runs {:.4} to {:.4} across the face",
            least,
            most
        );
    }

    /// Jake: the stones around it do not make visual sense unless it was a
    /// gravel or rubble pile it sat on. A pile banks against what it was
    /// heaped against and thins to nothing at its edge; a ring of stones
    /// at one height is the thing he was looking at.
    #[test]
    fn the_rubble_banks_against_the_stone_and_thins_outward() {
        let (at, _) = runestone_at(30.0);
        let heap = rubble(30.0, &done());

        assert_eq!(heap.len(), RUBBLE);

        let reach = |where_: Vec3| {
            ((where_.x - at.x).powi(2) + (where_.z - at.z).powi(2)).sqrt() / RUBBLE_OUT
        };
        let mut near = (0.0f32, 0);
        let mut far = (0.0f32, 0);

        for (where_, size, _) in &heap {
            let top = where_.y + size.y * 0.5;

            if reach(*where_) < 0.5 {
                near = (near.0 + top, near.1 + 1);
            } else {
                far = (far.0 + top, far.1 + 1);
            }
        }

        assert!(
            near.1 > 10 && far.1 > 10,
            "{} in close, {} out",
            near.1,
            far.1
        );

        let (near, far) = (near.0 / near.1 as f32, far.0 / far.1 as f32);

        assert!(
            near > far * 1.8,
            "it stands {:.3} in close and {:.3} at the rim",
            near,
            far
        );
    }

    /// The bank and the pieces lying on it have to agree about where the
    /// edge is. They did not: the pieces read the rim at the opposite
    /// direction, so round half the heap they lay out past it.
    #[test]
    fn nothing_lies_off_the_edge_of_the_bank() {
        let (at, _) = runestone_at(30.0);

        for (where_, size, _) in rubble(30.0, &done()) {
            let off = ((where_.x - at.x).powi(2) + (where_.z - at.z).powi(2)).sqrt();
            let round = (where_.z - at.z).atan2(where_.x - at.x);
            let rim = RUBBLE_OUT * mound_rim(round);

            assert!(
                off <= rim + size.x * 0.5,
                "a piece sits {:.3} out where the bank ends at {:.3}",
                off,
                rim
            );
        }
    }

    /// And the bank itself keeps off the ramp, which the pieces alone
    /// cannot say: its edge wanders a fifth further than its nominal
    /// reach, and the clearance was worked out without that.
    #[test]
    fn the_bank_keeps_off_the_bridge() {
        let foot = foot(30.0);
        let (at, _) = runestone_at(30.0);

        for step in 0..360 {
            let round = step as f32 * std::f32::consts::TAU / 360.0;
            let out = RUBBLE_OUT * mound_rim(round);

            assert!(out <= rubble_reaches() + 1e-4, "the rim reaches {:.3}", out);

            let edge = vec3(at.x + round.cos() * out, 0.0, at.z + round.sin() * out);
            let aside = (edge.z - foot.z) - WIDE * 0.5;
            let behind = edge.x - foot.x;

            // how far that is from the deck's own footprint, which is
            // the half strip running out in -x within `WIDE` of the
            // line. **Not `aside > 0 || behind > 0`**, which is only
            // whether it is off the deck: at the offset this had, the
            // bank came within five centimetres of the ramp and that
            // test said nothing.
            let clear = match (aside > 0.0, behind > 0.0) {
                (true, true) => aside.hypot(behind),
                (true, false) => aside,
                (false, true) => behind,
                (false, false) => 0.0,
            };

            assert!(
                clear >= RUBBLE_CLEARS,
                "the bank comes within {:.3} of the ramp at {:.2} radians",
                clear,
                round
            );
        }
    }

    /// And it stops under the runes. A footing lapping the first row is a
    /// name that has to be read round.
    #[test]
    fn no_rubble_laps_the_carved_band() {
        let (at, _) = runestone_at(30.0);
        let band = at.y + (FACE_FROM - 0.5) * RUNE_TALL;

        for (where_, size, _) in rubble(30.0, &done()) {
            let top = where_.y + size.y * 0.5;

            assert!(
                top < band - 0.05,
                "a piece tops out at {:.3}, band at {:.3}",
                top,
                band
            );
        }
    }

    /// None of the heap is solid, so any of it over the ramp is rubble you
    /// walk through. It ran a metre onto the deck when the stone's offset
    /// was its own number rather than the heap's reach.
    #[test]
    fn the_rubble_keeps_off_the_bridge() {
        let foot = foot(30.0);

        for (where_, size, _) in rubble(30.0, &done()) {
            let (wide, deep) = (size.x * 0.5, size.z * 0.5);
            let aside = (where_.z - deep - foot.z) - WIDE * 0.5;
            let behind = (where_.x - wide) - foot.x;

            assert!(
                aside > 0.1 || behind > 0.1,
                "a piece sits {:.3} aside and {:.3} behind the foot",
                aside,
                behind
            );
        }
    }

    #[test]
    fn it_rises_out_of_the_field_and_levels_off() {
        assert_eq!(high(0.0), 0.0, "it starts on the ground");
        assert!((high(1.0) - RISE).abs() < 0.001, "it reaches its full rise");

        // level over the last stretch, which is what makes the top a place
        // to stand rather than an edge
        assert!((high(LEVELS_AT) - high(1.0)).abs() < 0.001);

        // and climbing all the way up to there
        let mut was = -1.0;
        for step in 0..=40 {
            let now = high(step as f32 / 40.0 * LEVELS_AT);
            assert!(now > was, "it climbs the whole way to the level part");
            was = now;
        }
    }

    /// The first shape here was a quarter circle, which rises fastest where
    /// it leaves the ground: 1.29 over its first 1.46, a wall to walk at.
    #[test]
    fn it_leaves_the_ground_at_a_walkable_angle() {
        let step = RUN / PIECES as f32;
        let up = high(1.0 / PIECES as f32);

        assert!(
            up < step * 0.34,
            "the foot rises {} over {}, which is a wall to walk at",
            up,
            step
        );
    }

    /// The one that decides whether any of this can be used. A collider is
    /// a box that cannot tilt, so the arc is a staircase to walk on, and a
    /// riser over `walk::STEP` is a wall with a view.
    #[test]
    fn no_tread_rises_more_than_a_step() {
        let treads = treads(REACHES);
        let mut worst: f32 = 0.0;

        for pair in treads.windows(2) {
            let up = pair[1].max.y - pair[0].max.y;
            worst = worst.max(up);
        }

        assert!(
            worst < crate::walk::STEP,
            "the steepest riser is {} and a walk steps {}",
            worst,
            crate::walk::STEP
        );
    }

    /// And the control for it, so the test above is known to be able to
    /// fail: four times the tread is a staircase nobody climbs.
    #[test]
    fn a_coarse_tread_would_not_be_walkable() {
        let foot = foot(REACHES);
        // **A length of its own, not a multiple of `TREAD`.** Written
        // as four times the real one, this control tracked whatever
        // `TREAD` became: when the tread went to 0.12 to take the
        // jitter out, the control went to 0.48, which is still
        // walkable, and the test that was meant to prove the fine
        // tread matters stopped proving anything. A metre is a coarse
        // tread whatever the real one is doing.
        let coarse = 1.0;
        let steps = (RUN / coarse).ceil() as usize;
        let mut worst: f32 = 0.0;
        let mut was = 0.0;

        for step in 0..steps {
            let to = ((step + 1) as f32 * coarse / RUN).min(1.0);
            let top = high(to);
            worst = worst.max(top - was);
            was = top;
        }

        let _ = foot;
        assert!(
            worst > crate::walk::STEP,
            "the control did not go past a step"
        );
    }

    #[test]
    fn the_treads_lie_under_the_drawn_deck() {
        let treads = treads(REACHES);
        let foot = foot(REACHES);

        // one curve, two lists off it: a tread's top and the deck's height
        // at the same place have to be the same thing
        for box_ in &treads {
            let along = (foot.x - box_.center().x) / RUN;

            assert!(
                (box_.max.y - high(along)).abs() < 0.2,
                "a tread at {} along tops out at {} and the deck is at {}",
                along,
                box_.max.y,
                high(along)
            );
        }
    }

    #[test]
    fn every_band_is_the_same_width_and_they_fill_it() {
        let made = deck_mesh(REACHES);
        let foot = foot(REACHES);
        let (mut south, mut north) = (f32::MAX, f32::MIN);

        for one in made.vertices.iter() {
            south = south.min(one.position[2]);
            north = north.max(one.position[2]);
        }

        assert!(
            (north - south - WIDE).abs() < 0.001,
            "the deck is {:.3} across and should be {:.3}",
            north - south,
            WIDE
        );
        assert!(
            ((north + south) * 0.5 - foot.z).abs() < 0.001,
            "and it is centred on {:.3} rather than {:.3}",
            (north + south) * 0.5,
            foot.z
        );
    }

    /// Spec 0054: what you see is what you stand on.
    ///
    /// **Jake, 2026-11-04: the base of the bridge should be flush with the
    /// grass, not a step.** It was worse than the base. The drawn deck was
    /// hung `DEEP * 0.5` above its own middle rather than below it, so the
    /// whole of it floated a third of a metre over its own treads: a step
    /// up out of the field at the foot, a step down onto spec 0055's
    /// landing at the top, and shin deep the length of it.
    #[test]
    fn the_deck_you_see_is_the_deck_you_stand_on() {
        let foot = foot(REACHES);

        for middle in stations(REACHES) {
            let along = (foot.x - middle.x) / RUN;

            assert!(
                (middle.y - high(along)).abs() < 1e-4,
                "at {:.3} along, what you see tops out at {:.3} and what \
                 holds you up is at {:.3}",
                along,
                middle.y,
                high(along)
            );
        }
    }

    /// Spec 0054: and it comes out of the grass rather than off a kerb.
    #[test]
    fn it_springs_flush_from_the_grass() {
        // the top of what is drawn, which is what you would step up onto
        let lowest = deck_mesh(REACHES)
            .vertices
            .iter()
            .map(|one| one.position[1])
            .fold(f32::MIN, f32::max)
            .min(
                stations(REACHES)
                    .into_iter()
                    .map(|middle| middle.y)
                    .fold(f32::MAX, f32::min),
            );

        assert!(
            lowest < 0.05,
            "the near end of it stands {:.3} proud of the field",
            lowest
        );
    }

    /// Spec 0054: the seven run into each other rather than meeting at an
    /// edge.
    ///
    /// **Jake's other half of the same note**, on 2026-11-04: the colours
    /// between the bands should blend together, like a real rainbow.
    ///
    /// Two halves, and the first alone passes on a picture of one colour.
    /// Neighbouring texels have to be close, which is the blend, *and* the
    /// picture has to go somewhere over its width, which is what says
    /// there is still a rainbow in it.
    #[test]
    fn the_rainbow_blends_rather_than_steps() {
        let made = rainbow();
        let pixels = &made.levels[0].pixels;
        let mut biggest = 0i32;
        let mut over_all = 0i32;

        for at in 1..RAINBOW_WIDE as usize {
            for channel in 0..3 {
                let one = pixels[at * 4 + channel] as i32;
                let last = pixels[(at - 1) * 4 + channel] as i32;

                biggest = biggest.max((one - last).abs());
            }
        }

        for channel in 0..3 {
            let (mut low, mut high) = (255i32, 0i32);

            for at in 0..RAINBOW_WIDE as usize {
                let one = pixels[at * 4 + channel] as i32;
                low = low.min(one);
                high = high.max(one);
            }

            over_all = over_all.max(high - low);
        }

        assert!(
            biggest < 12,
            "two texels next to each other differ by {}, which is a band \
             edge and not a blend",
            biggest
        );
        assert!(
            over_all > 120,
            "the whole picture only moves by {}, so it is one colour and \
             not a rainbow",
            over_all
        );
    }

    /// Spec 0054: and every one of the seven is still in it, where it was.
    ///
    /// The stops sit at the middles of the bands they used to fill, so the
    /// colour at those seven places is what it always was and only what is
    /// between them is new.
    #[test]
    fn every_colour_of_the_rainbow_is_still_in_the_picture() {
        let made = rainbow();
        let pixels = &made.levels[0].pixels;

        for (n, want) in COLOURS.iter().enumerate() {
            let at = ((n as f32 + 0.5) / BANDS as f32 * RAINBOW_WIDE as f32) as usize;
            let at = at.min(RAINBOW_WIDE as usize - 1) * 4;

            for (channel, one) in [want.x, want.y, want.z].iter().enumerate() {
                let got = pixels[at + channel] as f32 / 255.0 * GLOW;

                assert!(
                    (got - one).abs() < 0.03,
                    "band {} channel {} came out {:.3} and is {:.3}",
                    n,
                    channel,
                    got,
                    one
                );
            }
        }
    }

    /// Walked rather than measured, which is the lesson of the pool's
    /// steps: four legal risers in a row were one solid block you could
    /// not get out of, and every rise passed.
    /// Spec 0055: nothing of the threshold exists until the building has
    /// been done, on the same condition and from the same list as the deck.
    ///
    /// A watchman on a bridge that is not there is a man standing in a
    /// field.
    #[test]
    fn no_threshold_until_the_building_is_done() {
        assert!(
            landing(REACHES, &Trail::new()).is_empty(),
            "the landing is up before anything has been given"
        );
        assert!(
            landing(REACHES, &nearly()).is_empty(),
            "the landing is up one question short"
        );
        assert!(
            !landing(REACHES, &done()).is_empty(),
            "the building is done and there is nothing at the top"
        );
    }

    /// Spec 0055: you step out onto it rather than up onto it.
    #[test]
    fn the_landing_is_flush_with_the_deck() {
        let slab = landing(REACHES, &done())[0];

        assert!(
            (slab.max.y - top(REACHES).y).abs() < 1e-4,
            "the landing's top is {:.3} and the deck's is {:.3}",
            slab.max.y,
            top(REACHES).y
        );
    }

    /// Spec 0055, and the fault this building keeps making: two faces at
    /// the walking surface stop a walk dead and `walk` reports nothing.
    #[test]
    fn the_landing_laps_the_deck() {
        let slab = landing(REACHES, &done())[0];
        let top = top(REACHES);

        assert!(
            slab.max.x > top.x + 1e-3,
            "the landing stops at {:.3} and the deck ends at {:.3}, which \
             is two faces in one plane",
            slab.max.x,
            top.x
        );
    }

    /// Spec 0055: wider than the deck, on both sides.
    #[test]
    fn the_landing_is_wider_than_the_deck() {
        let slab = landing(REACHES, &done())[0];
        let top = top(REACHES);

        for (side, got) in [
            ("north", slab.max.z - (top.z + WIDE * 0.5)),
            ("south", (top.z - WIDE * 0.5) - slab.min.z),
        ] {
            assert!(
                got > 1.0,
                "the landing stands {:.2} proud of the deck on the {}",
                got,
                side
            );
        }
    }

    /// Spec 0055: the parapet closes three sides and leaves the way you
    /// arrive open, and spec 0056's way through is cut in the far one.
    #[test]
    fn the_parapet_leaves_the_arriving_side_open() {
        let built = landing(REACHES, &done());
        let top = top(REACHES);
        let rails = &built[1..];

        // four pieces over three sides, because the far one is split
        // round the way through to spec 0056
        assert_eq!(rails.len(), 4, "a parapet of {} pieces", rails.len());

        // nothing of it stands across the lane you walk in on, which is the
        // deck's own width at the landing's east edge
        for rail in rails {
            let across = rail.min.z < top.z + WIDE * 0.5 && rail.max.z > top.z - WIDE * 0.5;
            let at_the_east = rail.max.x > built[0].max.x - 1e-3;

            assert!(
                !(across && at_the_east),
                "a rail closes the way you arrive over: {:?} to {:?}",
                rail.min,
                rail.max
            );
        }
    }

    /// Spec 0056: and there is a way through the far side.
    ///
    /// A gate with no gap in it is a wall. This was found by the walk that
    /// crosses into Valhalla fetching up eight tenths of a metre short of
    /// the edge, against a rail built to stop somebody falling.
    #[test]
    fn the_far_parapet_has_a_way_through_it() {
        let built = landing(REACHES, &done());
        let top = top(REACHES);
        // the far ones are the thin-in-x pieces. Taken by where they
        // start instead, the north and south rails come too: they run the
        // full corner, so they begin at the same west face.
        let far: Vec<_> = built[1..]
            .iter()
            .filter(|rail| rail.max.x - rail.min.x < RAIL_THICK * 1.5)
            .collect();

        assert_eq!(far.len(), 2, "the far side is in {} pieces", far.len());

        let gap = far
            .iter()
            .map(|rail| {
                if rail.max.z < top.z {
                    rail.max.z
                } else {
                    rail.min.z
                }
            })
            .collect::<Vec<_>>();
        let wide = (gap[0] - gap[1]).abs();

        assert!(
            wide > crate::RADIUS * 3.0,
            "the way through is {:.2} and a body is {:.2}",
            wide,
            crate::RADIUS * 2.0
        );
        // and it is on the line you walk in on
        assert!(
            ((gap[0] + gap[1]) * 0.5 - top.z).abs() < 1e-3,
            "the way through is off the middle by {:.2}",
            ((gap[0] + gap[1]) * 0.5 - top.z).abs()
        );
    }

    /// Spec 0054: nothing is sown where the bridge comes down, or where
    /// the stone stands.
    ///
    /// **The field's scatter has only ever known about the building.**
    /// `field::open` keeps trees west of the garden's face and asks
    /// nothing else, so one grew through the runestone and the low
    /// stretch of the deck ran through whatever was under it.
    ///
    /// Swept over the cells round the foot rather than checked at a
    /// point, because a scatter is the one thing you cannot reason about
    /// from one sample.
    #[test]
    fn nothing_is_sown_where_the_bridge_comes_down() {
        let foot = foot(REACHES);
        let (stone, size) = runestone_at(REACHES);
        let mut looked = 0;

        for cell in crate::field::cells_round(foot, 46.0) {
            for stood in crate::field::trees(REACHES, cell) {
                looked += 1;

                let off_stone = (stood.at.x - stone.x)
                    .abs()
                    .max((stood.at.z - stone.z).abs() - size.z * 0.5);

                assert!(
                    off_stone > 1.0,
                    "a tree is sown {:.2} from the runestone, at {:?}",
                    off_stone,
                    stood.at
                );

                let along = (foot.x - stood.at.x) / RUN;

                if (0.0..=1.0).contains(&along)
                    && (stood.at.z - foot.z).abs() < WIDE * 0.5
                    && high(along) < crate::field::TREE_TALL
                {
                    panic!(
                        "a tree is sown under the deck at {:?}, where it is \
                         {:.2} up and a tree is {:.2}",
                        stood.at,
                        high(along),
                        crate::field::TREE_TALL
                    );
                }
            }
        }

        assert!(
            looked > 20,
            "only {} trees anywhere near the bridge, so this test is not \
             looking at a field",
            looked
        );
    }

    /// Spec 0055: you can walk off the end of the deck onto it.
    #[test]
    fn walked_from_the_top_of_the_deck_onto_the_landing() {
        let solid = solid(REACHES, &done());
        let top = top(REACHES);
        // standing on the last of the level stretch, walking on west
        let start = vec3(top.x + 6.0, top.y + 0.6, top.z);
        let ended = walked(
            start,
            vec3(top.x - LANDING_OUT * 0.5, top.y, top.z),
            12.0,
            &solid,
        );

        assert!(
            ended.x < top.x - 1.0,
            "it got to {:.2} and the deck ends at {:.2}, so it never \
             stepped off onto the landing",
            ended.x,
            top.x
        );
        assert!(
            (ended.y - top.y).abs() < 0.4,
            "it ended {:.2} up and the landing is at {:.2}",
            ended.y,
            top.y
        );
    }

    /// Spec 0055: and the parapet stops you walking off the far edge.
    ///
    /// Twenty-six metres up, which is the whole reason for a kerb.
    #[test]
    fn the_parapet_stops_you_at_the_far_edge() {
        let solid = solid(REACHES, &done());
        let top = top(REACHES);
        let slab = landing(REACHES, &done())[0];
        // off the middle line, because the middle of the far side is the
        // way through to spec 0056 and is meant to be open
        let aside = top.z + WAY_WIDE * 0.5 + crate::RADIUS * 2.0;
        let start = vec3(top.x - 1.0, top.y + 0.4, aside);
        // aimed well past the far parapet
        let ended = walked(start, vec3(slab.min.x - 20.0, top.y, aside), 10.0, &solid);

        assert!(
            ended.x > slab.min.x,
            "it walked out to {:.2} and the landing stops at {:.2}",
            ended.x,
            slab.min.x
        );
        assert!(
            ended.y > top.y - 1.0,
            "it ended {:.2} up, off a landing at {:.2}",
            ended.y,
            top.y
        );
    }

    /// Spec 0055: he is on the landing, not in the air and not in the
    /// parapet.
    #[test]
    fn the_watchman_stands_on_the_landing() {
        let built = landing(REACHES, &done());
        let slab = built[0];
        let (at, _) = watchman(REACHES);

        assert!(
            (at.y - slab.max.y).abs() < 1e-4,
            "he stands at {:.3} and the landing's top is {:.3}",
            at.y,
            slab.max.y
        );
        assert!(
            at.x > slab.min.x && at.x < slab.max.x && at.z > slab.min.z && at.z < slab.max.z,
            "he stands at {:?}, off a landing from {:?} to {:?}",
            at,
            slab.min,
            slab.max
        );

        // and his own width is clear of every rail, since a body is 0.9
        for rail in &built[1..] {
            let near = (at.x - at.x.clamp(rail.min.x, rail.max.x)).abs()
                + (at.z - at.z.clamp(rail.min.z, rail.max.z)).abs();

            assert!(
                near > crate::RADIUS,
                "he stands {:.2} from a rail, and he is {:.2} across",
                near,
                crate::RADIUS * 2.0
            );
        }
    }

    /// Spec 0055: facing back down the climb.
    ///
    /// Taken the way `draw_the_people` takes it, where ahead of a figure is
    /// `(sin, 0, -cos)` of its yaw. Written as a number here it would say
    /// nothing: a quarter turn is only east because of that line.
    #[test]
    fn the_watchman_faces_the_climb() {
        let (at, facing) = watchman(REACHES);
        let ahead = vec3(facing.sin(), 0.0, -facing.cos());
        let climb = (foot(REACHES) - at).normalize_or_zero();

        assert!(
            ahead.dot(climb) > 0.9,
            "he faces {:?} and the climb comes up from {:?}",
            ahead,
            climb
        );
    }

    /// Spec 0055: and clear of the way you arrive, so that you are met
    /// rather than blocked.
    #[test]
    fn the_watchman_is_not_in_the_way() {
        let (at, _) = watchman(REACHES);
        let top = top(REACHES);
        let off = (at.z - top.z).abs();

        assert!(
            off > crate::RADIUS * 2.0,
            "he stands {:.2} off the middle of a lane {:.2} wide, and a \
             body is {:.2} across",
            off,
            WIDE,
            crate::RADIUS * 2.0
        );
        assert!(
            off < WIDE * 0.5,
            "he stands {:.2} off the middle, which is outside the lane \
             you walk up: you would never pass him",
            off
        );
    }

    pub fn walked(from: Vec3, to: Vec3, seconds: f32, solid: &[Aabb]) -> Vec3 {
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                let going = way.normalize();

                // **Carried where the game carries you**, or this walks
                // the bridge at a pace nobody playing ever walks it at
                // and the time it reports is a time nobody waits.
                going * crate::SPEED * super::carries(REACHES, at, going, &done())
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    /// The deck carries you up itself, and only up, and only on itself.
    #[test]
    fn the_deck_carries_you_up_and_nothing_else_does() {
        let foot = foot(REACHES);
        let up = vec3(-1.0, 0.0, 0.0);
        let down = vec3(1.0, 0.0, 0.0);
        let on = vec3(foot.x - RUN * 0.5, high(0.5), foot.z);

        assert_eq!(carries(REACHES, on, up, &done()), CARRIES);
        assert_eq!(
            carries(REACHES, on, down, &done()),
            1.0,
            "it pushes you back"
        );
        assert_eq!(
            carries(REACHES, on, up, &nearly()),
            1.0,
            "it carries you before it is there"
        );

        let aside = vec3(on.x, on.y, foot.z + WIDE);
        assert_eq!(carries(REACHES, aside, up, &done()), 1.0, "off the side");

        let under = vec3(on.x, 0.0, foot.z);
        assert_eq!(carries(REACHES, under, up, &done()), 1.0, "underneath it");

        let past = vec3(foot.x - RUN * 1.4, on.y, foot.z);
        assert_eq!(carries(REACHES, past, up, &done()), 1.0, "past the top");

        let behind = vec3(foot.x + 6.0, 1.4, foot.z);
        assert_eq!(
            carries(REACHES, behind, up, &done()),
            1.0,
            "short of the foot"
        );
    }

    #[test]
    fn walked_from_the_foot_to_the_top() {
        let solid = solid(REACHES, &done());
        let foot = foot(REACHES);
        let top = vec3(foot.x - RUN + 2.0, RISE, foot.z);

        // dropped in from above the foot rather than placed, because a
        // point on a deck can be inside it
        // **Nine seconds and not a minute.** Jake: it takes way too long
        // to run up the rainbow bridge, and a test that gave it a
        // minute could never have told anybody that. This is a walk
        // rather than a run, so `CARRIES` is the whole of what gets it
        // up inside the window: unassisted, 4.2 a second over a hundred
        // metres is twenty four.
        //
        // It was six, which fitted while the climb was getting a reach
        // of free travel out of `walk::over_a_step` at every one of its
        // 0.12 treads. Taking that away is what `walk::RIDES` did, and
        // a walk up here is now the 8.5 seconds the pace says it is.
        let start = vec3(foot.x - 1.0, 1.4, foot.z);
        let ended = walked(start, top, 9.0, &solid);

        assert!(
            ended.y > RISE * 0.9,
            "walked for nine seconds and got to {} up, and the top is {}",
            ended.y,
            RISE
        );
        // **0.92 and not 0.4.** Unassisted this walk covers 37.8 in nine
        // seconds, so a bound under that passed whether the deck carried
        // you or not and said nothing about the thing it is there for.
        assert!(
            foot.x - ended.x > RUN * 0.92,
            "got {} of the way along a run of {}",
            foot.x - ended.x,
            RUN
        );
    }

    /// The control. Without it the walk above passes for a bridge that is
    /// one flat slab at the height of the foot.
    #[test]
    fn a_walk_off_the_side_does_not_climb() {
        let solid = solid(REACHES, &done());
        let foot = foot(REACHES);
        let beside = vec3(foot.x - 1.0, 1.4, foot.z + WIDE);
        let ended = walked(beside, vec3(foot.x - RUN, 0.0, foot.z + WIDE), 60.0, &solid);

        assert!(
            ended.y < 1.0,
            "walking beside it climbed to {}, so the deck is wider than it says",
            ended.y
        );
    }

    #[test]
    fn it_crosses_nothing() {
        // west of where the garden's ground stops, every piece of it, so
        // it never lands on the garden or on the building behind it
        let stops = crate::field::starts(REACHES);

        for one in deck_mesh(REACHES).vertices.iter() {
            assert!(
                one.position[0] < stops,
                "a corner of it at {} is east of the field's start at {}",
                one.position[0],
                stops
            );
        }
    }

    #[test]
    fn it_stays_inside_the_ground_it_stands_on() {
        let stops = crate::field::starts(REACHES);
        let out = treads(REACHES)
            .iter()
            .map(|box_| stops - box_.min.x)
            .fold(f32::MIN, f32::max);

        assert!(
            out < crate::field::GROUND_REACH,
            "it runs {} out and the ground reaches {}",
            out,
            crate::field::GROUND_REACH
        );
    }
}

#[cfg(test)]
mod look {
    #[test]
    #[ignore]
    fn write_the_carving() {
        let made = super::carving();
        let (wide, tall) = (made.width(), made.height());
        let mut out = format!("P6\n{} {}\n255\n", wide, tall).into_bytes();

        for texel in made.levels[0].pixels.chunks(4) {
            let a = texel[3] as f32 / 255.0;

            for under in [120u8, 60, 44] {
                out.push((255.0 * a + under as f32 * (1.0 - a)) as u8);
            }
        }

        let to = std::env::temp_dir().join("carving.ppm");
        std::fs::write(&to, out).unwrap();
        eprintln!("WROTE {}", to.display());
    }
}
