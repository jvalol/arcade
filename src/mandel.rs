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

pub fn says(view: View) -> String {
    format!("{:.0} times in", view.deep())
}

/// The frame round the opening, and how far it stands proud of the wall.
///
/// **It used to be load bearing and now it is not.** Spec 0039 shipped
/// with a ladder of fixed views and no live picture at all: between two
/// steps it drew the one it had bigger, and the frame had to be wide
/// enough to hide the overflow. Spec 0074 of the engine gave a texture you
/// can write again, so the picture is worked out as you move and nothing
/// overflows. The frame stays because it reads as a window rather than a
/// poster, and it is a third of what it was.
pub const FRAME: f32 = 0.11;
pub const FRAME_PROUD: f32 = 0.06;

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
pub const TEXELS: u32 = 128;

/// Where the dive goes.
///
/// On the boundary, which is the only place with anything in it: the inside
/// is black and the outside thins out. This one is a seahorse valley point
/// and it has structure the whole way down.
pub const AT: (f32, f32) = (-0.743_643_9, 0.131_825_9);

/// How much the panel shows at the top of the dive.
pub const ACROSS: f32 = 3.2;

/// How fast the arrows move you, as a share of the view a second.
///
/// A share rather than a distance, so panning feels the same however far
/// in you are: half a panel a second at the top is half a panel a second
/// at the bottom.
pub const PANS: f32 = 0.5;

/// And how fast shift and up or down change the scale, as a multiple a
/// second.
///
/// **Shift, because the plain arrows move you now.** Spec 0039 shipped
/// with a ladder of fixed views and no way across the plane at all, which
/// Jake walked into on 2026-10-25.
pub const ZOOMS: f32 = 1.7;

/// How far out and how far in you can get.
///
/// Out is where the whole set is in the panel. In is where single
/// precision gives out: a texel's place is `corner + n * step`, and once
/// `step` is under the last bit of `corner` the two come out the same
/// float, so the same arithmetic runs twice and the picture goes to
/// blocks. The machine runs out before the mathematics does.
pub fn closest(at: (f32, f32)) -> f32 {
    at.0.abs().max(at.1.abs()).max(1.0) * f32::EPSILON * TEXELS as f32
}

/// How far the view can wander from the set before there is nothing to
/// look at.
pub const WANDER: f32 = 2.0;

/// Where you are looking and how much of the plane is in the panel./// Where you are looking and how much of the plane is in the panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub at: (f32, f32),
    pub across: f32,
}

impl Default for View {
    fn default() -> Self {
        View {
            at: AT,
            across: ACROSS,
        }
    }
}

impl View {
    /// Moves the view by the arrows, in its own terms.
    pub fn moved(self, pan: (f32, f32), zoom: f32, dt: f32) -> View {
        let across = (self.across * ZOOMS.powf(-zoom * dt)).clamp(closest(self.at), ACROSS);
        let step = self.across * PANS * dt;

        View {
            at: (
                (self.at.0 + pan.0 * step).clamp(AT.0 - WANDER, AT.0 + WANDER),
                (self.at.1 + pan.1 * step).clamp(AT.1 - WANDER, AT.1 + WANDER),
            ),
            across,
        }
    }

    /// How far in this is, which is the only thing the readout says.
    pub fn deep(self) -> f32 {
        ACROSS / self.across
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

    /// Spec 0039: moving feels the same however far in you are.
    ///
    /// A pan is a share of what is in the panel rather than a distance, or
    /// the same key crosses the whole set at the top and moves nothing at
    /// the bottom.
    #[test]
    fn moving_feels_the_same_however_far_in() {
        let shares: Vec<f32> = [1.0f32, 1e-2, 1e-4]
            .iter()
            .map(|across| {
                let view = View {
                    at: AT,
                    across: *across,
                };
                let moved = view.moved((1.0, 0.0), 0.0, 0.25);

                (moved.at.0 - view.at.0) / view.across
            })
            .collect();

        for share in shares.iter() {
            assert!(
                (share - shares[0]).abs() < shares[0] * 0.02,
                "a quarter second moves {:.6} of the panel at one scale and {:.6} at another",
                shares[0],
                share
            );
        }
    }

    /// Spec 0039: you cannot zoom in past what single precision can tell
    /// apart, or out past the whole set.
    ///
    /// **In is the room's one claim.** A texel's place is `corner + n *
    /// step`, and once `step` is under the last bit of `corner` the two
    /// come out the same float, so the same arithmetic runs twice and the
    /// picture goes to blocks. The machine runs out before the mathematics
    /// does.
    #[test]
    fn the_view_stops_where_the_numbers_run_together() {
        // taken at the clamp rather than walked to it: getting there is
        // twenty three seconds of holding a key, and what is being asked
        // is where it stops rather than how long that takes
        let view = View {
            at: AT,
            across: closest(AT),
        };

        assert!(
            view.moved((0.0, 0.0), 1.0, 10.0).across >= view.across,
            "held in from the stop, it went further"
        );

        let step = view.across / TEXELS as f32;
        let corner = view.at.0 - view.across * 0.5;

        assert!(
            (0..TEXELS - 1).all(|n| corner + n as f32 * step != corner + (n + 1) as f32 * step),
            "held in, it went past where two texels are the same number"
        );

        // and a few steps further in they do run together, so this is
        // where it gives out rather than somewhere short of it
        let past = View {
            at: view.at,
            across: view.across * 0.2,
        };
        let step = past.across / TEXELS as f32;
        let corner = past.at.0 - past.across * 0.5;

        assert!(
            (0..TEXELS - 1).any(|n| corner + n as f32 * step == corner + (n + 1) as f32 * step),
            "five times further in the numbers still tell texels apart"
        );

        // and out stops at the whole set
        let mut view = View::default();
        for _ in 0..600 {
            view = view.moved((0.0, 0.0), -1.0, 1.0 / 60.0);
        }

        assert!(
            (view.across - ACROSS).abs() < 1e-4,
            "held out, the panel shows {:.3} and the whole set is {:.3}",
            view.across,
            ACROSS
        );
    }

    /// Spec 0039: you cannot walk the view off into empty plane.
    #[test]
    fn you_cannot_wander_off_the_set() {
        for way in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let mut view = View::default();
            for _ in 0..600 {
                view = view.moved(way, 0.0, 1.0 / 60.0);
            }

            // **Against the set rather than against `WANDER`.** Read off
            // that constant this cannot fail: whatever it is, clamping to
            // it keeps you inside it. The set lives inside x -2.1 to 0.6
            // and y -1.2 to 1.2, so staying within three of the origin is
            // staying somewhere with something in it.
            assert!(
                view.at.0.abs() < 3.0 && view.at.1.abs() < 3.0,
                "held {:?} for ten seconds, the view wandered to {:?}, \
                 where there is nothing to look at",
                way,
                view.at
            );
        }
    }

    /// Spec 0039: a view that has moved is a different picture.
    ///
    /// Read off the pixels, because the point of the engine's spec 0074 is
    /// that the picture is worked out again as you move: a view that
    /// changes and a panel that does not is exactly the fault worth
    /// catching.
    #[test]
    fn a_moved_view_is_a_different_picture() {
        let view = View::default();
        let before = pixels(view);

        for moved in [
            view.moved((1.0, 0.0), 0.0, 0.5),
            view.moved((0.0, 1.0), 0.0, 0.5),
            view.moved((0.0, 0.0), 1.0, 0.5),
        ] {
            let after = pixels(moved);
            let same = before
                .iter()
                .zip(after.iter())
                .filter(|(one, other)| one == other)
                .count();

            assert!(
                (same as f32) < before.len() as f32 * 0.98,
                "moving to {:?} left the picture {:.1} per cent the same",
                moved,
                same as f32 / before.len() as f32 * 100.0
            );
        }
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
}
