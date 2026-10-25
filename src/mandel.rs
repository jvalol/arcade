//! The Mandelbrot set, on a wall of the gallery. Spec 0039.

use blitzkit::texture::TextureData;

/// What the readout calls it, and what it says underneath.
///
/// **It does not say Mandelbrot and it does not say what you are looking
/// at.** Five shapes down the same wall are named for what they are, and
/// this one is not, because what it is is not the point: the point is how
/// far in you have got and that the number stops. The register next door
/// settles the same thing the same way, and is called the register rather
/// than Cantor's diagonal.
pub const NAME: &str = "A Window";

/// How far in step `level` is.
pub fn deep(level: usize) -> f32 {
    STEP.powi(level as i32)
}

pub fn says(level: usize) -> String {
    format!("{:.0} times in", deep(level))
}

/// How many steps a second the arrows take you, and how much shift adds.
///
/// Shift runs you everywhere else in this building, so it runs you here.
pub const DIVES: f32 = 0.9;
pub const RUNS: f32 = 3.5;

/// How much of a step you can be part way through and still have the frame
/// cover the picture behind it.
///
/// **The whole trick, and it costs nothing.** Each step in is a separate
/// picture and there are only as many as single precision has, so a frame
/// of them cannot be worked out while you watch: one is three to twenty
/// five milliseconds and a frame is eight. What can be done at no cost is
/// grow the one already on the wall. A quarter of the way into a step the
/// picture is `STEP` to the quarter bigger, and at the end of the step it
/// is exactly `STEP` bigger, which is the next picture's view, so swapping
/// to it changes nothing on screen.
///
/// What the frame is for is the overflow. The picture grows past the
/// opening and the frame has to be wide enough to hide where it has got to
/// at the end of a step, which is `(STEP - 1) / 2` of the opening each
/// side.
pub const FRAME: f32 = WIDE * (STEP - 1.0) * 0.5 + 0.04;
pub const FRAME_PROUD: f32 = 0.06;

/// How much bigger the picture is drawn, part way through a step.
pub fn grown(dove: f32) -> f32 {
    STEP.powf(dove.fract())
}

/// The frame round the opening: four sides, standing proud of the wall.
pub fn surround(reaches: f32) -> Vec<blitzkit::collision::Aabb> {
    let (at, size) = panel(reaches);
    let half = size.x * 0.5;
    let (near, far) = (at.z, at.z + FRAME_PROUD);

    [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)]
        .iter()
        .map(|(x, y)| {
            // the two uprights run the full height and the head and sill
            // reach over them, so no two of the four share a face
            let (wide, tall) = if *x == 0.0 {
                (half + FRAME, FRAME * 0.5)
            } else {
                (FRAME * 0.5, half)
            };
            let middle = glam::vec3(
                at.x + x * (half + FRAME * 0.5),
                at.y + y * (half + FRAME * 0.5),
                (near + far) * 0.5,
            );

            blitzkit::collision::Aabb::from_center_size(
                middle,
                glam::vec3(wide * 2.0, tall * 2.0, FRAME_PROUD),
            )
        })
        .collect()
}

/// How many texels across the panel is drawn.
pub const TEXELS: u32 = 256;

/// Where the dive goes.
///
/// On the boundary, which is the only place with anything in it: the inside
/// is black and the outside thins out. This one is a seahorse valley point
/// and it has structure the whole way down.
pub const AT: (f32, f32) = (-0.743_643_9, 0.131_825_9);

/// How much the panel shows at the top of the dive.
pub const ACROSS: f32 = 3.2;

/// How much one step in goes.
///
/// A root two, so a doubling is two steps and holding the key reads as a
/// dive rather than a slideshow.
pub const STEP: f32 = std::f32::consts::SQRT_2;

/// How many steps there are before single precision gives out.
///
/// **Worked out rather than chosen, and it is the room.** Two neighbouring
/// texels differ by `across / TEXELS`, and a float near the dive point can
/// only tell apart differences above its own last bit. Below that the
/// arithmetic is the same for both and the picture goes to blocks: the
/// machine runs out before the mathematics does, which is the only thing
/// this panel has to say.
pub fn levels() -> usize {
    let ulp = AT.0.abs().max(AT.1.abs()) * f32::EPSILON;
    let mut level = 0;

    while ACROSS / STEP.powi(level) / TEXELS as f32 > ulp {
        level += 1;
    }

    level as usize
}

/// What the panel shows at step `level`.
pub fn view(level: usize) -> View {
    View {
        at: AT,
        across: ACROSS / STEP.powi(level as i32),
    }
}

/// Where you are looking and how much of the plane is in the panel.
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub at: (f32, f32),
    pub across: f32,
}

impl Default for View {
    fn default() -> Self {
        view(0)
    }
}

/// How wide the panel is, and how far off the wall it stands.
///
/// **Small, and at the row's own height.** It is a window rather than an
/// exhibit: a plinth is for something you walk round and this is flat. It
/// goes at the free end of the far wall, which is where the row stops, so
/// you see it beside the three fractals it answers.
/// How wide the panel is, and how far off the wall it stands.
///
/// **Over the row and down the middle of the wall.** It went at the free
/// end of that wall first, at the shapes' own height, which is 0.7 of
/// panel in a 0.75 gap behind the end plinth. Jake walked in and said the
/// placing and the size are not good or accessible, and they were not: you
/// had to squeeze past a plinth to reach something the size of a book.
///
/// Up there it is the first thing you see on walking in, it is the only
/// flat thing in a room of objects, and the three fractals it answers are
/// in the same view underneath it.
pub const WIDE: f32 = 1.3;
pub const PROUD: f32 = 0.05;

/// Where the panel hangs, as its middle and how big it is.
///
/// Clear of the shapes under it and of the ceiling over it, both taken off
/// what is there rather than written out, so a row that grows taller
/// pushes this up instead of quietly standing behind it.
pub fn panel(reaches: f32) -> (glam::Vec3, glam::Vec3) {
    let wall = crate::room::gallery_wall(reaches);
    let over = crate::display::all_of_them(wall)
        .iter()
        .map(|one| one.at.y + one.half.y)
        .fold(crate::display::HIGH, f32::max);

    (
        glam::vec3(0.0, (over + crate::room::TALL) * 0.5, wall + PROUD * 0.5),
        glam::vec3(WIDE, WIDE, PROUD),
    )
}

/// How many times to iterate before calling a point inside.
///
/// It rises as you go in, because the deeper you are the longer the
/// outside takes to escape and a fixed cap paints the detail black.
pub fn rounds(view: View) -> u32 {
    let deep = (View::default().across / view.across).log2().max(0.0);

    120 + (deep * 32.0) as u32
}

/// The panel, ready for the engine.
pub fn drawn(view: View) -> TextureData {
    TextureData::from_pixels(TEXELS, TEXELS, pixels(view))
}

/// The panel as pixels: how far each point got before it left.
pub fn pixels(view: View) -> Vec<u8> {
    let rounds = rounds(view);
    let step = view.across / TEXELS as f32;
    let corner = (view.at.0 - view.across * 0.5, view.at.1 - view.across * 0.5);
    let mut out = Vec::with_capacity((TEXELS * TEXELS * 4) as usize);

    for down in 0..TEXELS {
        for along in 0..TEXELS {
            let c = (
                corner.0 + along as f32 * step,
                corner.1 + down as f32 * step,
            );
            let (mut x, mut y) = (0.0f32, 0.0f32);
            let mut n = 0;

            while n < rounds && x * x + y * y <= 4.0 {
                let next = x * x - y * y + c.0;
                y = 2.0 * x * y + c.1;
                x = next;
                n += 1;
            }

            let (r, g, b) = shade(n, rounds);
            out.extend([r, g, b, 255]);
        }
    }

    out
}

/// What a point that took `n` rounds is painted.
///
/// **On a log of the count, not a fraction of the cap.** Straight division
/// was the first build and the panel came out one flat blue: nearly
/// everything you can see escapes in a handful of rounds, so the whole
/// visible outside landed in the bottom thousandth of the range and the
/// detail was all in a band too narrow to paint. A log spreads the low
/// counts out, which is where every picture of this set lives.
fn shade(n: u32, rounds: u32) -> (u8, u8, u8) {
    if n >= rounds {
        return (5, 5, 9);
    }

    let out = (n as f32).max(1.0).ln() / (rounds as f32).ln();
    // round the bands rather than ramping once, so going in keeps turning
    // up new ones instead of washing to white
    let band = (out * 3.0 * std::f32::consts::TAU).sin() * 0.5 + 0.5;
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

    #[test]
    fn it_stops_where_single_precision_does() {
        let deep = view(levels() - 1);
        let step = deep.across / TEXELS as f32;
        let ulp = AT.0.abs().max(AT.1.abs()) * f32::EPSILON;

        assert!(
            step > ulp,
            "the last step is already past what a float can tell apart"
        );

        let past = ACROSS / STEP.powi(levels() as i32) / TEXELS as f32;
        assert!(
            past <= ulp,
            "there is another step in it, so this is not where it gives out"
        );
    }

    /// Spec 0039: the panel hangs clear of the shapes and of the ceiling,
    /// and you can reach it from in front of it.
    #[test]
    fn the_panel_hangs_clear_and_within_reach() {
        let reaches = 8.3;
        let (at, size) = panel(reaches);
        let wall = crate::room::gallery_wall(reaches);
        let over = crate::display::all_of_them(wall)
            .iter()
            .map(|one| one.at.y + one.half.y)
            .fold(0.0, f32::max);

        assert!(
            at.y - size.y * 0.5 > over,
            "the panel's bottom at {:.2} is in the row, which tops out at {:.2}",
            at.y - size.y * 0.5,
            over
        );
        assert!(
            at.y + size.y * 0.5 < crate::room::TALL,
            "the panel's top at {:.2} is through the ceiling at {:.2}",
            at.y + size.y * 0.5,
            crate::room::TALL
        );
        assert!(
            at.x.abs() + size.x * 0.5 < crate::room::GALLERY_WIDE * 0.5,
            "the panel is wider than the wall it is on"
        );

        // and from a pace and a half back it is inside the sight's reach
        let from = glam::vec3(at.x, crate::EYE, at.z + 1.6);

        assert!(
            from.distance(at) < crate::room::REACH,
            "standing in front of it, it is {:.2} away and the sight carries {:.2}",
            from.distance(at),
            crate::room::REACH
        );
    }

    /// Spec 0039: the frame covers the picture wherever it has got to
    /// inside a step.
    ///
    /// **This is what makes the dive run instead of jump.** Each step is
    /// its own picture and one of those takes three to twenty five
    /// milliseconds, which does not fit in a frame, so what moves between
    /// steps is not a new picture but the old one drawn bigger. At the end
    /// of a step it is exactly `STEP` bigger, which is the next picture's
    /// own view, so the swap changes nothing on screen. All the frame has
    /// to do is hide the overflow, and if it does not the picture is seen
    /// growing out of its own opening.
    #[test]
    fn the_frame_covers_the_picture_all_the_way_through_a_step() {
        let reaches = 8.3;
        let (at, size) = panel(reaches);
        let sides = surround(reaches);

        for n in 0..=40 {
            let part = n as f32 / 40.0;
            let half = size.x * 0.5 * grown(part);

            for (which, edge) in [
                (at + glam::vec3(half, 0.0, 0.0), 0),
                (at - glam::vec3(half, 0.0, 0.0), 1),
                (at + glam::vec3(0.0, half, 0.0), 2),
                (at - glam::vec3(0.0, half, 0.0), 3),
            ]
            .iter()
            .map(|(point, n)| (*n, *point))
            {
                assert!(
                    sides.iter().any(|side| side.contains_point(glam::vec3(
                        edge.x,
                        edge.y,
                        side.center().z
                    ))),
                    "{:.0} per cent through a step, edge {} is at {:?} with no frame over it",
                    part * 100.0,
                    which,
                    edge
                );
            }
        }
    }

    /// Spec 0039: the dive goes somewhere, and every step of it is a
    /// different place.
    ///
    /// Read off the drawn pictures rather than off the view, because a
    /// view that changes and a picture that does not is exactly the fault
    /// worth catching: every one of these is thrown away and only its
    /// pixels are compared.
    #[test]
    fn every_step_in_shows_something_else() {
        let mut last = pixels(view(0));

        for level in 1..levels() {
            let next = pixels(view(level));
            let same = last
                .iter()
                .zip(next.iter())
                .filter(|(one, other)| one == other)
                .count();

            assert!(
                (same as f32) < last.len() as f32 * 0.98,
                "step {} is {:.1} per cent the same as the one before it",
                level,
                same as f32 / last.len() as f32 * 100.0
            );
            last = next;
        }
    }

    /// Spec 0039: the dive stops at the last step where two neighbouring
    /// texels are still two different numbers.
    ///
    /// **The room's one claim, and counted rather than looked at.** A
    /// texel's place on the plane is `corner + n * step`, and once `step`
    /// is under the last bit of `corner` the two come out the same float,
    /// so the same arithmetic runs twice and the picture goes to blocks.
    /// The machine runs out before the mathematics does.
    ///
    /// Blockiness in the pixels was tried first and is not the same
    /// question: the shallow views are mostly inside the set, which is one
    /// flat colour, so they measure blockier than the deep ones while
    /// being perfectly sharp.
    #[test]
    fn the_dive_stops_where_the_numbers_run_together() {
        let apart = |level: usize| {
            let view = view(level);
            let step = view.across / TEXELS as f32;
            let corner = view.at.0 - view.across * 0.5;

            (0..TEXELS - 1)
                .filter(|n| corner + *n as f32 * step == corner + (*n + 1) as f32 * step)
                .count()
        };

        assert_eq!(
            apart(levels() - 1),
            0,
            "the last step already has texels that are the same number"
        );

        // and a few steps past the end they have run together
        let past = View {
            at: AT,
            across: ACROSS / STEP.powi(levels() as i32 + 3),
        };
        let step = past.across / TEXELS as f32;
        let corner = past.at.0 - past.across * 0.5;
        let run = (0..TEXELS - 1)
            .filter(|n| corner + *n as f32 * step == corner + (*n + 1) as f32 * step)
            .count();

        assert!(
            run > 0,
            "three steps past the end the numbers are still telling texels apart, \
             so this is not where it gives out"
        );
    }

    #[test]
    fn how_long_does_it_take() {
        for across in [3.2f32, 3.2e-2, 3.2e-5] {
            let view = View {
                at: (-0.743_643_9, 0.131_825_9),
                across,
            };
            let then = std::time::Instant::now();
            let made = drawn(view);
            eprintln!(
                "M across {:9.2e}  rounds {:5}  {:?}  {}x{}",
                across,
                rounds(view),
                then.elapsed(),
                made.width(),
                made.height()
            );
        }
    }
}
