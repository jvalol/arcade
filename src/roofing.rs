//! A roof's courses, cut into pieces. Spec 0068.
//!
//! **Because a course in one piece is a plank.** A course that runs the whole
//! length of a building has no vertical joint anywhere in it, and an unbroken
//! band is what a sawn board looks like however many of them you stack. What
//! says shingle, or sod, is the short joints between one piece and the next,
//! staggered course to course so they never line up into a seam running up
//! the pitch.
//!
//! Here rather than in either roof, because the village is shingled and the
//! steading is turfed and the cutting is the same cutting. Two copies of this
//! is two copies that drift, which is the fault this building has written
//! down under six other names.

/// One course cut across a roof, each piece as (where its middle sits along
/// the roof, how long it is).
///
/// `long` is the whole roof end to end, `piece` is about how wide one piece
/// wants to be, `gap` is the joint between two, and `course` is which course
/// up the slope this is, which is all the stagger runs on.
///
/// **The stagger leaves a sliver at the near end**, and it gets a piece of
/// its own. Dropped, it is a notch out of every other course at one gable.
pub fn coursed(long: f32, piece: f32, gap: f32, course: usize) -> Vec<(f32, f32)> {
    let across = ((long / piece).round() as usize).max(2);
    let step = long / across as f32;
    let shift = if course.is_multiple_of(2) {
        0.0
    } else {
        step * 0.5
    };
    let mut out = Vec::new();

    if shift > gap {
        out.push(((shift - long) * 0.5, shift - gap));
    }

    for n in 0..across {
        let from = (n as f32 * step + shift - long * 0.5).max(-long * 0.5);
        let to = ((n as f32 + 1.0) * step + shift - long * 0.5).min(long * 0.5);

        if to - from > gap {
            out.push(((from + to) * 0.5, to - from - gap));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONG: f32 = 9.36;
    const PIECE: f32 = 0.8;
    const GAP: f32 = 0.06;

    /// The pieces cover the roof and none of them runs off it.
    #[test]
    fn a_course_covers_its_roof_and_no_more() {
        for course in 0..6 {
            let pieces = coursed(LONG, PIECE, GAP, course);
            let ends = |(at, long): &(f32, f32)| (at - long * 0.5, at + long * 0.5);

            assert!(pieces.len() > 1, "course {} is one piece", course);

            for one in pieces.iter() {
                let (from, to) = ends(one);

                assert!(
                    from >= -LONG * 0.5 - 1e-3 && to <= LONG * 0.5 + 1e-3,
                    "a piece runs from {:.3} to {:.3} on a roof {:.3} long",
                    from,
                    to,
                    LONG
                );
            }

            // and the gaps between them are gaps rather than holes
            let mut sorted: Vec<(f32, f32)> = pieces.iter().map(ends).collect();
            sorted.sort_by(|a, b| a.0.total_cmp(&b.0));

            // half a joint in from either gable, which is what keeps a
            // piece's end face out of the pitch's own end plane
            assert!(
                (sorted[0].0 + LONG * 0.5 - GAP * 0.5).abs() < 1e-3,
                "the first piece starts at {:.3} and the roof starts at {:.3}",
                sorted[0].0,
                -LONG * 0.5
            );
            assert!(
                (sorted[sorted.len() - 1].1 - LONG * 0.5 + GAP * 0.5).abs() < 1e-3,
                "the last piece ends at {:.3} and the roof ends at {:.3}",
                sorted[sorted.len() - 1].1,
                LONG * 0.5
            );

            for pair in sorted.windows(2) {
                assert!(
                    (pair[1].0 - pair[0].1 - GAP).abs() < 1e-3,
                    "two pieces are {:.3} apart and the joint is {:.3}",
                    pair[1].0 - pair[0].1,
                    GAP
                );
            }
        }
    }

    /// And no joint sits on the one in the course below it.
    #[test]
    fn the_joints_are_staggered_course_to_course() {
        let joints = |course: usize| -> Vec<f32> {
            coursed(LONG, PIECE, GAP, course)
                .into_iter()
                .flat_map(|(at, long)| [at - long * 0.5 - GAP * 0.5, at + long * 0.5 + GAP * 0.5])
                // the ends of the roof are its edge rather than a joint in it
                .filter(|z| (z.abs() - LONG * 0.5).abs() > GAP)
                .collect()
        };

        for course in 0..5 {
            for over in joints(course + 1).iter() {
                for under in joints(course).iter() {
                    assert!(
                        (over - under).abs() > GAP,
                        "a joint at {:.3} sits on the one at {:.3} below it",
                        over,
                        under
                    );
                }
            }
        }
    }
}
