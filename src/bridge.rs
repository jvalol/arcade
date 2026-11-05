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
pub const TREAD: f32 = 0.6;

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

/// What the stone at its foot says.
///
/// **Because nobody can tell what any of this is.** Jake, 2026-11-05: a
/// person seeing it for the first time would have no idea it is meant to
/// be Norse. Spec 0006 is the precedent and says it plainly about the
/// nook: everything else in the hall announces itself, and a room nothing
/// names is a room you find by accident.
///
/// **Carved and not hung or lit.** Spec 0006 picked a painted plank over
/// neon for the nook because a neon arrow pointing at a panelled study is
/// a sign for the wrong room. The same test sends this one to stone.
///
/// Without the umlaut. The font has one and draws it as a lowercase ö
/// among capitals, which on a carved stone reads as a mistake rather than
/// as a spelling.
pub const SAYS: &str = "BIFROST [COPY - Jake]";

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

    0.76 + slow * 0.17 + under * 0.07
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
    let across = ((u - 0.5).abs() / (FACE_WIDE * 1.4)).min(1.0);
    let middle = (FACE_FROM + FACE_TO) * 0.5;
    let along = ((v - middle).abs() / ((FACE_TO - FACE_FROM) * 0.72)).min(1.0);
    let off = across.max(along);

    (1.0 - off * off).max(0.0)
}

/// Where the stone's skin is at `u` round and `v` up, in its own unit.
///
/// **One account of the surface, which is the whole point.** The carving
/// is built from this too, so it lies on the rock instead of over it: it
/// was a flat slab standing proud of a lump, and you could see the plate.
pub fn stone_point(seed: u32, u: f32, v: f32) -> Vec3 {
    use std::f32::consts::{PI, TAU};

    // the face looks along +x at `u` of a half, which keeps the seam round
    // the back where nothing is written
    let round = -(u - 0.5) * TAU;
    let up = v * PI;
    let out = (0.5 + knobble(seed, u, v)) * up.sin();
    let lean = crate::garden::tip(seed, v);

    vec3(
        round.cos() * out + lean.x,
        -up.cos() * 0.5,
        round.sin() * out + lean.y,
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
pub const FACE_WIDE: f32 = 0.085;
pub const FACE_FROM: f32 = 0.28;
pub const FACE_TO: f32 = 0.76;
/// And how far the carving stands off the skin it is laid on.
pub const FACE_PROUD: f32 = 0.012;

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
        let u = 0.5 + (s - 0.5) * FACE_WIDE * 2.0;
        // down the stone as the picture goes down, because the picture's
        // row nought is its top and `v` nought is this lump's foot.
        // Taken the other way the band stood on its head and the name
        // read along the crown.
        let v = FACE_TO - t * (FACE_TO - FACE_FROM);
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
    let (near, far) = (0.11, 0.89);
    let mut out = vec![
        // the band, round the face and open at the foot the way a
        // serpent's body leaves its own tail
        ((near, 0.08), (near, far)),
        ((near, far), (0.5, 0.965)),
        ((0.5, 0.965), (far, far)),
        ((far, far), (far, 0.08)),
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
    let tall = 0.069;
    let wide = tall * 2.05;
    let gap = 0.031;
    let top = far - 0.06;

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
    let want = (n as f32 * 0.62, n as f32 * 0.62 * was_tall / was_wide);
    let (at_x, at_y) = ((n as f32 - want.0) * 0.5, n as f32 * 0.045);

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

    blitzkit::texture::TextureData::from_pixels(n, n, pixels)
}

/// How tall the lettering is cut, how big the stone is, and how far it
/// leans.
///
/// **Off the side of the way on, not across it.** The deck is six metres
/// and you walk up the middle of it; a stone in that is a stone you walk
/// into on the way to the thing it names.
pub const RUNE_TEXELS: f32 = 64.0;
pub const RUNE_TALL: f32 = 2.7;
pub const RUNE_WIDE: f32 = 1.12;
pub const RUNE_THICK: f32 = 0.34;
pub const RUNE_LEANS: f32 = 0.06;
pub const RUNE_ASIDE: f32 = 1.4;

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
    if (at.x - stone.x).abs() < size.x * 0.5 + CLEARS
        && (at.z - stone.z).abs() < size.z * 0.5 + CLEARS
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
            foot.x + RUNE_WIDE,
            RUNE_TALL * 0.5,
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
                + landing(REACHES, &done()).len()
                + runestone(REACHES, &done()).iter().count()
        );
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
        let coarse = TREAD * 4.0;
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
                way.normalize() * crate::SPEED
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

    #[test]
    fn walked_from_the_foot_to_the_top() {
        let solid = solid(REACHES, &done());
        let foot = foot(REACHES);
        let top = vec3(foot.x - RUN + 2.0, RISE, foot.z);

        // dropped in from above the foot rather than placed, because a
        // point on a deck can be inside it
        let start = vec3(foot.x - 1.0, 1.4, foot.z);
        let ended = walked(start, top, 60.0, &solid);

        assert!(
            ended.y > RISE * 0.9,
            "walked for a minute and got to {} up, and the top is {}",
            ended.y,
            RISE
        );
        assert!(
            foot.x - ended.x > RUN * 0.85,
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
