//! The Mandelbrot set, in the middle of the gallery's back wall. Spec 0039.

use blitzkit::collision::Aabb;
use blitzkit::texture::TextureData;
use glam::{vec3, Vec3};

/// What the picture opens on: the whole set, with room round it.
///
/// **Framed to the picture rather than to a square.** The scale is set by
/// whichever way is tighter: the set runs from -1.2 to 1.2 and has to fit
/// that with a margin. Centred on the dive point, which is what it did
/// when the picture was square, the set came out cropped top and bottom
/// and sitting off to one side.
///
/// This was 6.4 while the picture was the whole wall and nearly two and a
/// half times wider than it was tall. On a picture barely wider than it is
/// tall, 6.4 leaves the set adrift in the middle of a great deal of empty
/// plane, so it comes down with the wall.
pub const HOME: (f64, f64) = (-0.65, 0.0);
pub const ACROSS: f64 = 3.6;

/// How far the picture can get from the set before there is nothing in it.
pub const WANDER: f64 = 2.0;

/// How fast the arrows move you, as a share of the view a second, and how
/// fast shift with up or down changes the scale, as a multiple a second.
pub const PANS: f64 = 0.5;
pub const ZOOMS: f64 = 1.7;

/// How far in it can get.
///
/// **In double rather than single, which is twenty nine more doublings.**
/// It shipped in single and stopped at eighteen: a texel's place is
/// `corner + n * step`, and once `step` is under the last bit of `corner`
/// the two come out the same number, so the same arithmetic runs twice and
/// the picture goes to blocks. Single really stops at 18.0 doublings and
/// the clamp was at 17.7, so there was a quarter of a doubling left in it
/// and no more. Double reaches 47.
///
/// The type is nearly free. What costs is rounds, and that is dealt with
/// in [`rounds`].
pub fn closest(at: (f64, f64)) -> f64 {
    at.0.abs().max(at.1.abs()).max(1.0) * f64::EPSILON * WIDE as f64
}

/// How big the picture is.
///
/// **The whole back wall was two faults wearing one coat.** The six stand
/// on their plinths 0.75 off that wall, so from anywhere far enough back
/// to see all of it, three of them were across the middle of it. And 6.36
/// metres of wall is 160 texels a metre whatever the grid is, because the
/// picture is worked out while you watch and the grid is what a frame can
/// afford. Jake called that blocky, then called a smoothed version of the
/// same thing blurry.
///
/// Both answers are the same answer: a smaller picture, on a piece of wall
/// nothing stands in front of. The row of six opens round it rather than
/// across it, per [`crate::display::gap`], and the room widens to hold
/// that.
///
/// ~~A 1.3 square over the row.~~ ~~The whole back wall.~~ This is the
/// third placing and the second size. What is different from the first is
/// the hole in the row: a picture over the shapes is a picture you look
/// past, and one between them is one you walk up to.
pub const PICTURE_WIDE: f32 = 2.6;
pub const PICTURE_TALL: f32 = 2.0;

/// How wide the picture is with its moulding, which is what the row has to
/// open by.
pub fn framed() -> f32 {
    PICTURE_WIDE + STEPS[STEPS.len() - 1].0 * 2.0
}

/// How many texels a metre of picture gets.
///
/// **A metre of picture and not a picture.** Both ways round were the same
/// grid before: a count across, with down worked out from the shape of the
/// wall, so a picture that changed size changed how fine it was and
/// nothing said so. Said as a density it is the thing that was actually
/// wrong, and the picture's two sides cannot come out stretched, which was
/// the other fault in the same line: picked by hand they were 256 by 112
/// against an opening of 2.446, a seven per cent stretch.
///
/// **What a frame can afford is what sets it.** Every texel is a point
/// worked out while you watch, so this is a budget and not a preference.
/// At 280 the picture is 408 thousand points and the threads clear them
/// in 7.9 milliseconds at the deepest the arithmetic goes, against a
/// frame of eight; at 300 it is 8.6 and the frame is gone. The wall it
/// replaced was 428 thousand over six and a third metres, so this is the
/// same work at nearly twice the density, which is the whole trick: the
/// picture got smaller.
///
/// **And it is still soft with your nose on it, which is arithmetic
/// rather than a setting.** Half a metre from a picture you see about
/// half a metre of it across the window, so matching the screen there
/// would want three thousand texels across. This is sharp from where the
/// whole picture is in frame and soft closer than that. A picture worked
/// out while you watch cannot be both.
///
/// A quarter of it in a browser, which has one thread.
#[cfg(not(target_arch = "wasm32"))]
pub const TEXELS: f32 = 280.0;
#[cfg(target_arch = "wasm32")]
pub const TEXELS: f32 = 70.0;
pub const WIDE: u32 = (PICTURE_WIDE * TEXELS) as u32;
pub const TALL: u32 = (PICTURE_TALL * TEXELS) as u32;

/// And how many it gets once you have stopped moving it.
///
/// **What a frame can afford is the wrong budget for a picture nobody is
/// moving.** 280 a metre is sharp from where the whole picture is in
/// frame and blocky with your nose on it, and standing with your nose on
/// it is what the thing invites: Jake walked up to it and the texels were
/// the size of the detail. There is no density that answers both while
/// every texel is worked out inside the frame that draws it.
///
/// So it is worked out outside that frame instead, a slice at a time,
/// into a second and much finer picture, which is drawn in place of the
/// first the moment it is whole. 787 a metre is 2048 across: more than a
/// window has pixels for the whole picture, so it is the window and not
/// the texels you are looking at from anywhere you can stand.
///
/// Moving throws the slices away and goes back to the coarse one, which
/// costs nothing, because it was being drawn all along.
#[cfg(not(target_arch = "wasm32"))]
pub const SHARP: f32 = 787.0;
#[cfg(target_arch = "wasm32")]
pub const SHARP: f32 = 140.0;
pub const SHARP_WIDE: u32 = (PICTURE_WIDE * SHARP) as u32;
pub const SHARP_TALL: u32 = (PICTURE_TALL * SHARP) as u32;

/// How much of the sharp picture one frame may work out, as points times
/// rounds.
///
/// **Measured, in the same points-times-rounds as [`A_FRAME`] and
/// deliberately looser.** A settling happens while nobody is moving
/// anything, so what it costs is a frame that is slower rather than a
/// frame that stutters: a slice runs from 0.7 milliseconds at the top to
/// 6.7 at the floor of double precision, which is eight rows of 2048
/// down there and a hundred and ten up here. The picture is whole in 15
/// frames at the top, 93 at twenty doublings and 197 at the bottom,
/// which at a hundred and twenty a second is a tenth of a second and a
/// second and a half.
///
/// Tightening it to hold every settling frame under four milliseconds
/// would double that wait, and a wall that takes three seconds to come
/// in is a wall you have walked away from.
const A_SLICE: u64 = 27_000_000;

/// The sharp picture, worked out a slice at a time.
///
/// It holds the view it is for, so a view that moves under it is a
/// settling thrown away rather than a picture of two places at once.
pub struct Settling {
    pub view: View,
    counts: Vec<f32>,
    done: u32,
}

impl Settling {
    pub fn new(view: View) -> Self {
        Settling {
            view,
            counts: Vec::with_capacity((SHARP_WIDE * SHARP_TALL) as usize),
            done: 0,
        }
    }

    /// Works out the next slice.
    pub fn step(&mut self) {
        if self.whole() {
            return;
        }

        let rounds = rounds(self.view);
        let deep = (A_SLICE / (SHARP_WIDE as u64 * rounds as u64)).max(1) as u32;
        let rows = self.done..(self.done + deep).min(SHARP_TALL);

        self.counts.extend(worked_out(
            self.view,
            rounds,
            rows.clone(),
            SHARP_WIDE,
            SHARP_TALL,
        ));
        self.done = rows.end;
    }

    /// Whether every row of it has been worked out.
    pub fn whole(&self) -> bool {
        self.done >= SHARP_TALL
    }

    /// The picture, which is only a picture once [`Self::whole`] is true.
    ///
    /// Painted here rather than slice by slice, because what a count is
    /// painted is decided by where the picture's other counts lie and
    /// there is no telling that until they are all in.
    pub fn drawn(&self) -> TextureData {
        let spread = Spread::of(&self.counts, rounds(self.view));

        TextureData::from_pixels(SHARP_WIDE, SHARP_TALL, painted(&self.counts, &spread))
    }
}

/// How many rounds a point gets before it is called inside.
///
/// It rises with depth, because the deeper you are the longer a point
/// just outside the set takes to get away, and anything that has not got
/// away by the last round is painted as though it never will.
///
/// **The same count whether you are moving or not, which it was not.**
/// Moving was capped at 90 to hold the frame, and 90 is below what the
/// outside needs past about twenty doublings in: every point was still in
/// hand at the last round, so every point was painted inside, so the
/// picture went black the moment you touched an arrow and came back when
/// you let go. Jake zoomed and the wall went out. What gives way while
/// you move is [`coarse`], which is how many points there are, and not
/// how long each one is allowed.
pub fn rounds(view: View) -> u32 {
    let deep = (ACROSS / view.across).log2().max(0.0);

    120 + (deep * 32.0) as u32
}

/// How much work one frame of moving may do, as points times rounds.
///
/// **Calibrated where the work actually is.** Points times rounds is a
/// ceiling and not a cost: at the top of the dive nearly everything
/// escapes in two rounds and the whole picture comes out at a fifth of
/// what the ceiling says. Near the boundary, which is where anybody who
/// has zoomed is standing, almost every point runs its full count and the
/// ceiling is the cost: 2.4e-10 seconds a point a round, measured at the
/// floor of double precision. This is six and a half milliseconds of
/// that, against a frame of eight.
///
/// A first draft read the rate off the top of the dive instead, came out
/// at 1.5e-10, and left a frame of moving at twelve milliseconds thirty
/// doublings in. The cheap end of a thing is the wrong end to measure it
/// from.
const A_FRAME: u64 = 33_000_000;

/// How many texels of the picture one worked-out point covers while you
/// are moving it.
///
/// Two at the top, three ten doublings in, four at twenty and thirty
/// three, five at the floor: the picture you pan is 364 across up there
/// and 146 at the bottom, and the one that arrives when you let go is
/// 2048 wherever you are. Blocky while a key is down and sharp a moment
/// after, which is the trade this whole panel is built on.
///
/// It holds a frame of moving between 1.1 and 7.0 milliseconds the whole
/// way down, measured along the boundary, which is the expensive line to
/// measure along.
pub fn coarse(view: View) -> u32 {
    let wants = WIDE as u64 * TALL as u64 * rounds(view) as u64;
    let over = wants.div_ceil(A_FRAME).max(1) as f64;

    over.sqrt().ceil() as u32
}

/// Where you are looking and how much of the plane is in the panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub at: (f64, f64),
    pub across: f64,
}

impl Default for View {
    fn default() -> Self {
        View {
            at: HOME,
            across: ACROSS,
        }
    }
}

impl View {
    /// Moves the view by the arrows, in its own terms.
    pub fn moved(self, pan: (f64, f64), zoom: f64, dt: f32) -> View {
        let dt = dt as f64;
        let across = (self.across * ZOOMS.powf(-zoom * dt)).clamp(closest(self.at), ACROSS);
        let step = self.across * PANS * dt;

        View {
            at: (
                (self.at.0 + pan.0 * step).clamp(HOME.0 - WANDER, HOME.0 + WANDER),
                (self.at.1 + pan.1 * step).clamp(HOME.1 - WANDER, HOME.1 + WANDER),
            ),
            across,
        }
    }

    /// How far in this is, which is the only thing the readout says.
    pub fn deep(self) -> f64 {
        ACROSS / self.across
    }

    /// How much of the plane the picture covers, down as well as across.
    ///
    /// **Off the picture's own shape and not the grid's.** Both grids are
    /// the picture's shape to within a rounding, but they are not each
    /// other's to the last texel, and a coarse grid that worked its own
    /// height out from its own width would cover a slightly different
    /// piece of the plane from the fine one. The wall would shift a hair
    /// every time you let go.
    pub fn plane(self) -> (f64, f64) {
        (
            self.across,
            self.across * PICTURE_TALL as f64 / PICTURE_WIDE as f64,
        )
    }
}

/// What the readout calls it, and what it says underneath.
pub const NAME: &str = "A Window";
pub const KEYS: &str = "Arrows to move, shift with up or down to zoom";

pub fn says(view: View) -> String {
    format!("{}. {:.0} times in", KEYS, view.deep())
}

/// The picture itself: where its middle is and how big it is drawn.
///
/// Halfway up the wall and halfway along it, which is also halfway across
/// the way in from the hall: the doorway is 3.3 wide and this is 2.6, so
/// the picture is what you see through it from down the aisle.
pub fn panel(reaches: f32) -> (Vec3, Vec3) {
    let wall = crate::room::gallery_wall(reaches);

    (
        vec3(0.0, crate::room::TALL * 0.5, wall + PROUD * 0.5),
        vec3(PICTURE_WIDE, PICTURE_TALL, PROUD),
    )
}

/// How far the picture and each step of the moulding stand off the wall.
pub const PROUD: f32 = 0.04;
pub const STEPS: [(f32, f32); 3] = [(0.07, 0.05), (0.13, 0.09), (0.22, 0.05)];

/// The moulding round it: three steps out from the picture, each wider and
/// each standing further proud.
///
/// **A profile rather than a border.** One box round a picture is a border
/// and reads as one, which is what the pool table's rails were before spec
/// 0005 gave them three boards and a cap that oversails. What does the
/// work is the shadow line at each step.
pub fn surround(reaches: f32) -> Vec<Aabb> {
    let (at, size) = panel(reaches);
    let mut out = Vec::new();

    for (n, (out_by, deep)) in STEPS.iter().enumerate() {
        let inner = if n == 0 {
            (size.x * 0.5, size.y * 0.5)
        } else {
            (size.x * 0.5 + STEPS[n - 1].0, size.y * 0.5 + STEPS[n - 1].0)
        };
        let (wide, tall) = (size.x * 0.5 + out_by, size.y * 0.5 + out_by);
        let (near, far) = (at.z, at.z + deep);

        for (x, y) in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            // the head and the sill reach over the two uprights, so no two
            // of the four share a face
            let (half_x, half_y) = if x == 0.0 {
                (wide, (tall - inner.1) * 0.5)
            } else {
                ((wide - inner.0) * 0.5, inner.1)
            };
            let middle = vec3(
                at.x + x * (inner.0 + wide) * 0.5,
                at.y + y * (inner.1 + tall) * 0.5,
                (near + far) * 0.5,
            );

            out.push(Aabb::from_center_size(
                middle,
                vec3(half_x * 2.0, half_y * 2.0, far - near),
            ));
        }
    }

    out
}

/// A fine picture of nothing, which is what the fine texture is built
/// with.
///
/// [`blitzkit::renderer::Renderer::write_texture`] refuses a size the
/// texture was not built at, so the size has to be settled before a single
/// point of it has been worked out. Nothing ever draws this: the texture
/// is not reached for until a settling has filled it.
pub fn blank() -> TextureData {
    TextureData::from_pixels(
        SHARP_WIDE,
        SHARP_TALL,
        vec![0; (SHARP_WIDE * SHARP_TALL * 4) as usize],
    )
}

/// The picture, ready for the engine.
pub fn drawn(view: View, moving: bool) -> TextureData {
    TextureData::from_pixels(WIDE, TALL, pixels(view, moving))
}

/// How far out a point has to get before it is called gone.
///
/// **Two was right for counting and wrong for colouring.** The round a
/// point leaves on is an integer, so a picture painted from it is painted
/// in whole steps: flat rings with a staircase along every edge, at any
/// depth and at any number of texels, which is what a wall of this made
/// look low. The fraction of a round a point is past the edge recovers the
/// rest, and that fraction is only worth having when the point is well
/// clear: at a radius of two the first term swamps it and the staircase
/// comes back faintly. At 256 it is smooth.
///
/// Squared, because that is what the loop has in hand.
const ESCAPES: f64 = 256.0 * 256.0;

/// The round a point left on, and the part of a round it was already past.
///
/// `n + 1 - log2(log2 |z|)`, which is the standard continuous escape time:
/// |z| doubles its logarithm every round out here, so how far past
/// `ESCAPES` the point overshot says where in the round it crossed.
fn how_far(n: u32, away: f64) -> f32 {
    if away <= 1.0 {
        return n as f32;
    }

    (n as f64 + 1.0 - (away.ln() * 0.5).ln() / std::f64::consts::LN_2) as f32
}

/// One band of rows of the picture.
fn band(view: View, rounds: u32, rows: std::ops::Range<u32>, wide: u32, tall: u32) -> Vec<f32> {
    let plane = view.plane();
    let step = (plane.0 / wide as f64, plane.1 / tall as f64);
    let corner = (view.at.0 - plane.0 * 0.5, view.at.1 - plane.1 * 0.5);
    let mut out = Vec::with_capacity(rows.len() * wide as usize);

    for down in rows {
        for along in 0..wide {
            // the middle of the texel and not its corner, so a coarse
            // grid and a fine one over the same plane agree about where
            // the middle of the picture is
            let c = (
                corner.0 + (along as f64 + 0.5) * step.0,
                corner.1 + (down as f64 + 0.5) * step.1,
            );
            let (mut x, mut y) = (0.0f64, 0.0f64);
            let mut n = 0;
            let mut away = 0.0f64;

            while n < rounds && away <= ESCAPES {
                let next = x * x - y * y + c.0;
                y = 2.0 * x * y + c.1;
                x = next;
                away = x * x + y * y;
                n += 1;
            }

            out.push(if n >= rounds {
                INSIDE
            } else {
                how_far(n, away)
            });
        }
    }

    out
}

/// What a point that never got away is counted as.
///
/// Not a round, so nothing that measures where the counts lie has to
/// leave it out by hand.
const INSIDE: f32 = f32::INFINITY;

/// How many steps the spread below cuts the counts into.
///
/// Four thousand across the whole range of rounds, which at the top of
/// the dive is a thirtieth of a round and at the bottom is two fifths of
/// one. Finer than either the eye or the colours can tell apart.
const STEPS_OF_SPREAD: usize = 4096;

/// Where the counts in one picture actually lie.
///
/// **The colours have to be laid over the counts a picture has, and not
/// over the counts it could have had.** They were a fraction of the cap:
/// `ln(n) / ln(rounds)`, which is fine near the top, where a point gets
/// 120 rounds and the picture's counts run from 1 to 119. Thirty
/// doublings in it gets 1080 and the picture's counts run from 1021 to
/// 1073, so the whole picture landed in seven thousandths of the colour
/// range and came out as one flat blue with a mottle in it. Jake zoomed
/// to 84 billion and said it barely looks like anything, and that is the
/// arithmetic of it: the deeper you are, the tighter the counts bunch
/// under the cap.
///
/// So each count is painted by how many of the picture's own points came
/// in at or below it. Every colour the picture has is used at every
/// depth, by construction, which is the only way a thing worked out while
/// you watch can promise it. The classical histogram colouring, which
/// every explorer of this set arrives at for the same reason.
pub struct Spread {
    /// What a count in each step of the range comes out as, worked out
    /// once rather than three million times. A sine and a fractional
    /// power a texel is fifteen milliseconds over the fine picture and
    /// four thousand lookups is nothing.
    looks: Vec<[u8; 4]>,
    rounds: f32,
}

impl Spread {
    pub fn of(counts: &[f32], rounds: u32) -> Self {
        let mut hit = vec![0u32; STEPS_OF_SPREAD];
        let mut got_away = 0u32;

        for n in counts.iter() {
            if *n == INSIDE {
                continue;
            }

            hit[Self::step(*n, rounds as f32)] += 1;
            got_away += 1;
        }

        let mut looks = Vec::with_capacity(STEPS_OF_SPREAD);
        let mut so_far = 0u32;

        for many in hit {
            so_far += many;

            let (r, g, b) = shade(so_far as f32 / got_away.max(1) as f32);

            looks.push([r, g, b, 255]);
        }

        Spread {
            looks,
            rounds: rounds as f32,
        }
    }

    fn step(n: f32, rounds: f32) -> usize {
        ((n / rounds * STEPS_OF_SPREAD as f32) as usize).min(STEPS_OF_SPREAD - 1)
    }

    /// What a count is painted: where it sits among the picture's own.
    fn at(&self, n: f32) -> [u8; 4] {
        if n == INSIDE {
            return INSIDE_LOOK;
        }

        self.looks[Self::step(n, self.rounds)]
    }
}

/// What the set itself is painted, which is nearly black.
const INSIDE_LOOK: [u8; 4] = [5, 5, 9, 255];

/// The counts painted, which is the picture.
fn painted(counts: &[f32], spread: &Spread) -> Vec<u8> {
    let mut out = Vec::with_capacity(counts.len() * 4);

    for n in counts.iter() {
        out.extend_from_slice(&spread.at(*n));
    }

    out
}

/// The picture as pixels: how far each point got before it left.
///
/// **A band of rows to a thread.** Every point here is worked out from its
/// own coordinates and nothing else, so the work splits perfectly and the
/// only cost is handing it out. At depth that is 308 milliseconds on one
/// thread against 25 on eighteen, which is the difference between a wall
/// that settles and a wall that stops the game while it thinks.
#[cfg(not(target_arch = "wasm32"))]
fn worked_out(
    view: View,
    rounds: u32,
    rows: std::ops::Range<u32>,
    wide: u32,
    tall: u32,
) -> Vec<f32> {
    let hands = std::thread::available_parallelism().map_or(4, |n| n.get()) as u32;
    let deep = (rows.end - rows.start).div_ceil(hands).max(1);

    std::thread::scope(|scope| {
        let bands: Vec<_> = (0..hands)
            .map(|n| {
                let from = (rows.start + n * deep).min(rows.end);
                let to = (rows.start + (n + 1) * deep).min(rows.end);

                scope.spawn(move || band(view, rounds, from..to, wide, tall))
            })
            .collect();

        bands
            .into_iter()
            .flat_map(|hand| hand.join().expect("a band of the wall"))
            .collect()
    })
}

/// The same in a browser, which has one thread.
#[cfg(target_arch = "wasm32")]
fn worked_out(
    view: View,
    rounds: u32,
    rows: std::ops::Range<u32>,
    wide: u32,
    tall: u32,
) -> Vec<f32> {
    band(view, rounds, rows, wide, tall)
}

/// The picture as pixels.
///
/// While you are moving it, worked out on a grid [`coarse`] texels to the
/// point and blown up to fill the picture, which is what lets every point
/// have all the rounds it needs without the frame paying for all of them.
pub fn pixels(view: View, moving: bool) -> Vec<u8> {
    let rounds = rounds(view);
    let by = if moving { coarse(view) } else { 1 };
    let (wide, tall) = (WIDE.div_ceil(by), TALL.div_ceil(by));
    let counts = worked_out(view, rounds, 0..tall, wide, tall);
    let few = painted(&counts, &Spread::of(&counts, rounds));

    if by <= 1 {
        return few;
    }

    let mut out = Vec::with_capacity((WIDE * TALL * 4) as usize);

    for down in 0..TALL {
        let row = (down / by).min(tall - 1) as usize;

        for along in 0..WIDE {
            let at = (row * wide as usize + (along / by).min(wide - 1) as usize) * 4;

            out.extend_from_slice(&few[at..at + 4]);
        }
    }

    out
}

/// What a point that took `n` rounds is painted.
///
/// **On where the count sits among the picture's own, not on a fraction
/// of the cap.** Straight division was the first build and the wall came
/// out one flat blue: nearly everything you can see escapes in a handful
/// of rounds, so the whole visible outside landed in the bottom
/// thousandth of the range. A log of the count fixed that at the top and
/// not at depth, where the counts bunch under the cap instead. See
/// [`Spread`].
///
/// **One sweep of colour and not three, which is not a change.** `out`
/// used to span about a third of its range on the view you walk in on,
/// and three sweeps over a third of the range is one sweep of colour.
/// `out` spans all of it now, at every depth, so three would be three
/// everywhere and the opening view would be banded where it was a single
/// gradient.
fn shade(out: f32) -> (u8, u8, u8) {
    let band = (out * std::f32::consts::TAU).sin() * 0.5 + 0.5;
    let lit = out.powf(0.55);

    (
        (lit * 90.0 + band * 150.0) as u8,
        (lit * 150.0 + band * 90.0) as u8,
        (lit * 70.0 + 110.0 - band * 60.0) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0039: splitting the work across threads gives the same
    /// picture as doing it in one go.
    ///
    /// **The thing a band-per-thread gets wrong.** Each band has to know
    /// which rows it is, and a band that starts its own coordinates from
    /// zero draws the top of the picture over and over. Read off the
    /// pixels, because that is the only place it would show.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_bands_make_one_picture() {
        let view = View::default();
        let rounds = rounds(view);
        let whole = band(view, rounds, 0..TALL, WIDE, TALL);

        assert_eq!(
            pixels(view, false),
            painted(&whole, &Spread::of(&whole, rounds)),
            "the threaded picture is not the one thread's"
        );
        assert_eq!(
            whole.len(),
            (WIDE * TALL) as usize,
            "a picture of the wrong size"
        );
    }

    /// Spec 0039: the picture that arrives when you stop is the same
    /// picture, worked out in pieces.
    ///
    /// **The thing a slice at a time gets wrong.** Every slice has to
    /// know which rows it is, and one that starts its own coordinates
    /// from zero draws the top of the picture over and over. It is the
    /// fault `the_bands_make_one_picture` holds for the threads, in the
    /// one other place the picture is cut up, and it shows nowhere but in
    /// the pixels.
    #[test]
    fn the_slices_make_one_picture() {
        let view = View::default();
        let mut settling = Settling::new(view);
        let mut steps = 0;

        while !settling.whole() {
            settling.step();
            steps += 1;

            assert!(steps < SHARP_TALL, "a settling that is not getting on");
        }

        let whole = band(view, rounds(view), 0..SHARP_TALL, SHARP_WIDE, SHARP_TALL);

        assert_eq!(
            settling.counts.len(),
            whole.len(),
            "the settled picture is {} counts and the whole one is {}",
            settling.counts.len(),
            whole.len()
        );
        assert!(
            settling.counts == whole,
            "the settled picture is not the one the slices were cut from"
        );
    }

    /// Spec 0039: the fine picture arrives while you are still standing
    /// there.
    ///
    /// **A slice is a budget divided by the rounds, and the rounds climb
    /// with the depth**, so the deeper you are the fewer rows a frame
    /// buys and the longer the wait. At the bottom it is eight rows a
    /// frame, which is a second and a half of standing still at 120
    /// frames a second, and that is the number this holds: the picture
    /// sharpening is a thing you notice, and the same thing taking ten
    /// seconds is a picture that never arrives.
    ///
    /// Counted rather than timed, because a clock in a test says
    /// different things on different machines and on the same one twice.
    #[test]
    fn the_fine_picture_arrives_while_you_wait() {
        /// Frames, at the hundred and twenty a second this draws at.
        const WAIT: u32 = 240;

        for deep in [0.0f64, 20.0, 46.0] {
            let view = View {
                across: ACROSS / 2f64.powf(deep),
                ..View::default()
            };
            // one slice, and what that rate comes to. Every slice of a
            // view is the same height, so this is the whole settling
            // without paying for the whole settling: at the bottom that
            // is three seconds of test for a thing already measured.
            let mut settling = Settling::new(view);

            settling.step();

            assert!(
                settling.done > 0,
                "at {} doublings in, a slice is no rows at all",
                deep
            );

            let steps = SHARP_TALL.div_ceil(settling.done);

            assert!(
                steps <= WAIT,
                "at {} doublings in the picture wants {} frames, and {} is the wait",
                deep,
                steps,
                WAIT
            );
        }
    }

    /// Spec 0039: the picture is not stretched.
    ///
    /// **The one fault about this set that everybody would spot.** The
    /// texel grid was picked by hand at 256 by 112 against an opening of
    /// 2.446, which flattens every circle in it by seven per cent. Down is
    /// worked out from across now and what is left is integer rounding.
    #[test]
    fn the_picture_is_not_stretched() {
        let (_, size) = panel(8.3);
        let opening = size.x / size.y;

        // both of them: the coarse one you move and the fine one that
        // arrives when you stop, which are two grids over one opening
        for (what, texels) in [
            ("moving", WIDE as f32 / TALL as f32),
            ("settled", SHARP_WIDE as f32 / SHARP_TALL as f32),
        ] {
            assert!(
                (opening - texels).abs() < opening * 0.01,
                "the opening is {:.4} and the {} grid is {:.4}, a {:.1} per cent stretch",
                opening,
                what,
                texels,
                (texels / opening - 1.0).abs() * 100.0
            );
        }
    }

    /// Spec 0039: the whole set is in the view you walk in on.
    ///
    /// Framed to the wall's height, which is what decides it: the set is
    /// 2.4 tall and the wall is two and a half times wider than it is
    /// tall, so filling the width would cut the top and bottom off.
    #[test]
    fn the_whole_set_is_in_the_view_you_walk_in_on() {
        let view = View::default();
        let tall = view.plane().1;

        for (x, y) in [(-2.05f64, 0.0f64), (0.47, 0.0), (-0.1, 1.12), (-0.1, -1.12)] {
            assert!(
                (x - view.at.0).abs() < view.across * 0.5 && (y - view.at.1).abs() < tall * 0.5,
                "the set reaches {:?} and the opening view does not",
                (x, y)
            );
        }

        // **And it fills that height rather than floating in it.** Fitting
        // alone passes at any scale large enough, so it says nothing about
        // framing: the set is 2.24 tall and wants most of the wall.
        let fills = 2.24 / tall;
        assert!(
            (0.7..1.0).contains(&fills),
            "the set fills {:.0} per cent of the wall's height",
            fills * 100.0
        );
    }

    /// Spec 0039: moving feels the same however far in you are.
    #[test]
    fn moving_feels_the_same_however_far_in() {
        let shares: Vec<f64> = [1.0f64, 1e-4, 1e-9]
            .iter()
            .map(|across| {
                let view = View {
                    at: HOME,
                    across: *across,
                };

                (view.moved((1.0, 0.0), 0.0, 0.25).at.0 - view.at.0) / view.across
            })
            .collect();

        for share in shares.iter() {
            assert!(
                (share - shares[0]).abs() < shares[0] * 0.02,
                "a quarter second moves {:.6} of the wall at one scale and {:.6} at another",
                shares[0],
                share
            );
        }
    }

    /// Spec 0039: it goes as deep as double precision does, and stops
    /// there.
    ///
    /// **Twenty nine doublings further than it shipped.** A texel's place
    /// is `corner + n * step`, and once `step` is under the last bit of
    /// `corner` the two come out the same number, so the same arithmetic
    /// runs twice and the picture goes to blocks. Single really stops at
    /// 18 doublings; this reaches 47.
    #[test]
    fn it_goes_as_deep_as_double_precision_does() {
        let view = View {
            at: HOME,
            across: closest(HOME),
        };

        assert!(
            view.deep().log2() > 40.0,
            "it stops at {:.1} doublings, which is not much past what single managed",
            view.deep().log2()
        );
        assert!(
            view.moved((0.0, 0.0), 1.0, 10.0).across >= view.across,
            "held in from the stop, it went further"
        );

        let apart = |across: f64| {
            let step = across / WIDE as f64;
            let corner = HOME.0 - across * 0.5;

            (0..WIDE - 1).all(|n| corner + n as f64 * step != corner + (n + 1) as f64 * step)
        };

        assert!(
            apart(view.across),
            "at the stop two texels are the same number"
        );
        assert!(
            !apart(view.across * 0.05),
            "twenty times further in they still tell texels apart, so this is not the end"
        );
    }

    /// Spec 0039: you cannot walk the view off into empty plane.
    #[test]
    fn you_cannot_wander_off_the_set() {
        for way in [(1.0f64, 0.0f64), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let mut view = View::default();
            for _ in 0..600 {
                view = view.moved(way, 0.0, 1.0 / 60.0);
            }

            // against the set rather than against the clamp, or whatever
            // the clamp is would make this pass
            assert!(
                view.at.0.abs() < 4.0 && view.at.1.abs() < 4.0,
                "held {:?} for ten seconds, the view wandered to {:?}",
                way,
                view.at
            );
        }
    }

    /// Spec 0039: moving is cheap however deep you are, and settling is
    /// where the detail comes back.
    ///
    /// At the bottom the full count is 1624 rounds and 35 milliseconds
    /// against a frame of 8, so a picture worked out properly every frame
    /// is a picture that stutters.
    #[test]
    fn moving_is_cheap_however_deep_you_are() {
        for deep in [0.0f64, 10.0, 20.0, 33.0, 46.0] {
            let view = View {
                at: HOME,
                across: ACROSS / 2f64.powf(deep),
            };
            let by = coarse(view) as u64;
            let points = (WIDE.div_ceil(by as u32) as u64) * (TALL.div_ceil(by as u32) as u64);
            let work = points * rounds(view) as u64;

            assert!(
                work <= A_FRAME,
                "at {} doublings in a frame of moving is {} million against a budget of {}",
                deep,
                work / 1_000_000,
                A_FRAME / 1_000_000
            );
        }
    }

    /// Spec 0039: the picture you are moving is not black.
    ///
    /// **It was, past about twenty doublings in.** Moving was capped at
    /// 90 rounds to hold the frame, and 90 is below what a point just
    /// outside the set needs down there, so every point was still in hand
    /// at the last round and every point was painted inside. Jake zoomed
    /// and the wall went out; it came back the moment he let go, which is
    /// the tell that it was the cap and not the arithmetic.
    ///
    /// What is held here is that the picture you pan is the picture you
    /// settle on, coarsely: the same plane, the same colours, fewer
    /// points.
    ///
    #[test]
    fn the_picture_you_move_is_not_black() {
        /// A point the boundary stays near all the way down, which is
        /// what a view has to straddle for any of this to show. Dive on
        /// the middle of the set and the picture is black at every depth
        /// and every count of rounds; dive a little off it and the whole
        /// view escapes in a handful. A first draft of this held two
        /// black pictures against each other and passed.
        const BOUNDARY: (f64, f64) = (-0.743643887037151, 0.131825904205330);

        for deep in [0.0f64, 10.0, 20.0, 33.0] {
            let view = View {
                at: BOUNDARY,
                across: ACROSS / 2f64.powf(deep),
            };
            let moving = pixels(view, true);
            let settled = pixels(view, false);
            let lit = |picture: &[u8]| {
                picture
                    .chunks(4)
                    .filter(|texel| texel[0] as u32 + texel[1] as u32 + texel[2] as u32 > 60)
                    .count() as f32
                    / (WIDE * TALL) as f32
            };

            assert!(
                (lit(&moving) - lit(&settled)).abs() < 0.1,
                "at {} doublings in, {:.0} per cent of the picture you move is lit against {:.0} \
                 of the one you settle on",
                deep,
                lit(&moving) * 100.0,
                lit(&settled) * 100.0
            );
        }
    }

    /// Spec 0039: a view that has moved is a different picture.
    #[test]
    fn a_moved_view_is_a_different_picture() {
        let view = View::default();
        let before = pixels(view, false);

        for moved in [
            view.moved((1.0, 0.0), 0.0, 0.5),
            view.moved((0.0, 1.0), 0.0, 0.5),
            view.moved((0.0, 0.0), 1.0, 0.5),
        ] {
            let after = pixels(moved, false);
            let same = before
                .iter()
                .zip(after.iter())
                .filter(|(one, other)| one == other)
                .count();

            assert!(
                (same as f32) < before.len() as f32 * 0.98,
                "moving to {:?} left the wall {:.1} per cent the same",
                moved,
                same as f32 / before.len() as f32 * 100.0
            );
        }
    }

    /// Spec 0039: the moulding surrounds the picture and stands off the
    /// wall in steps.
    #[test]
    fn the_moulding_surrounds_the_picture() {
        let (at, size) = panel(8.3);
        let sides = surround(8.3);

        assert_eq!(
            sides.len(),
            STEPS.len() * 4,
            "a moulding of {} pieces is not four to a step",
            sides.len()
        );

        for (x, y) in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            // **Out to a moulding's width, not a hair past the edge.** One
            // thin box passes a probe two centimetres out and is a border
            // rather than a moulding, which is what the pool table's rails
            // were before spec 0005 gave them a profile. Fifteen
            // centimetres is read off nothing in here on purpose.
            for out_by in [0.02f32, 0.08, 0.15] {
                let edge = at
                    + vec3(
                        x * (size.x * 0.5 + out_by),
                        y * (size.y * 0.5 + out_by),
                        0.0,
                    );

                assert!(
                    sides.iter().any(|side| side.contains_point(vec3(
                        edge.x,
                        edge.y,
                        side.center().z
                    ))),
                    "nothing covers {:.2} outside the picture's edge at {:?}",
                    out_by,
                    (x, y)
                );
            }
        }

        // and each step stands further off the wall than the last
        let proud: Vec<f32> = STEPS.iter().map(|(_, deep)| *deep).collect();
        assert!(proud.iter().all(|deep| *deep > 0.0), "a step with no depth");
    }

    /// Spec 0039: the picture uses its colours however deep you are.
    ///
    /// **The fault the spread exists to fix.** The colours were a
    /// fraction of the cap, and the deeper you go the tighter a
    /// picture's counts bunch under it: thirty doublings in, a point
    /// gets 1080 rounds and the counts run from 1021 to 1073, which is
    /// seven thousandths of the range. Jake zoomed to 84 billion and got
    /// one flat blue with a mottle in it.
    ///
    /// Counted as how many different colours the picture holds, over a
    /// sample of it. Painted off the cap it is 31 at thirty doublings
    /// in; painted off the spread it is 253, and never under 250
    /// anywhere on the way down.
    #[test]
    fn the_colours_are_used_at_every_depth() {
        const BOUNDARY: (f64, f64) = (-0.743643887037151, 0.131825904205330);

        for (at, deep) in [
            (HOME, 0.0f64),
            (BOUNDARY, 10.0),
            (BOUNDARY, 20.0),
            (BOUNDARY, 30.0),
            (BOUNDARY, 36.0),
        ] {
            let view = View {
                at,
                across: ACROSS / 2f64.powf(deep),
            };
            let picture = pixels(view, false);
            let mut seen = std::collections::HashSet::new();

            for texel in picture.chunks(4).step_by(37) {
                seen.insert((texel[0], texel[1], texel[2]));
            }

            assert!(
                seen.len() > 150,
                "at {} doublings in the picture holds {} colours, which is a wash",
                deep,
                seen.len()
            );
        }
    }

    /// Spec 0039: nothing stands in front of the picture.
    ///
    /// **The fault the third placing exists to fix.** On the whole back
    /// wall the row of six stood 0.75 off it, and from anywhere far
    /// enough back to see all of the picture, three of them were across
    /// the middle of it. Measured in plan, which is where it has to be
    /// measured: a shape between the room and the wall is in front of the
    /// wall from every standing place at once, so no camera can settle
    /// it.
    ///
    /// Against the sweep and not the width. A shape turns, and what it
    /// reaches while it turns is what is in the way.
    #[test]
    fn nothing_stands_in_front_of_the_picture() {
        let reaches = 8.3;
        let frame = framed() * 0.5;
        let sweep = crate::display::sweep();

        for one in crate::display::all_of_them(crate::room::gallery_wall(reaches)) {
            let clear = one.at.x.abs() - sweep - frame;

            assert!(
                clear >= crate::display::BESIDE - 1e-4,
                "{} sweeps to {:.3} of the middle and the picture's frame reaches {:.3}",
                one.name,
                one.at.x.abs() - sweep,
                frame
            );
        }
    }

    /// Spec 0039: the outside is a gradient and not a staircase.
    ///
    /// **The one thing that made the wall look low.** The round a point
    /// leaves on is a whole number, so a picture painted straight off it
    /// is painted in whole steps: the view you walk in on came out as a
    /// handful of flat rings with a staircase down every edge, and no
    /// number of texels fixes that, because the texels are not what is
    /// coarse. [`how_far`] takes the fraction of a round back off the
    /// overshoot.
    ///
    /// Counted rather than looked at: how many colours a line across the
    /// middle of the picture passes through. Painted from the round
    /// alone it is twenty, and the twenty are what you see. With the
    /// fraction it is a few hundred, which is as many as the line has
    /// room for.
    #[test]
    fn the_outside_is_a_gradient() {
        let view = View::default();
        let whole = pixels(view, false);
        let row = (TALL / 2) as usize;
        let mut seen = std::collections::HashSet::new();

        for along in 0..WIDE as usize {
            let at = (row * WIDE as usize + along) * 4;

            seen.insert((whole[at], whole[at + 1], whole[at + 2]));
        }

        assert!(
            seen.len() > 100,
            "a line across the middle passes through {} colours, which is a staircase",
            seen.len()
        );
    }
}
