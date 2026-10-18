//! The parts of the building that never move, as the lists the engine is
//! handed once. Spec 0026.
//!
//! **Because the building is what all of this was argued from.** The engine
//! learned to keep a run of instances rather than be told it every frame
//! (its spec 0058), to draw a kept run where it lies (0059), and to ask the
//! depth buffer which kept runs a wall hides (0061). Every one of those
//! works on kept runs and this building kept nothing: it pushed eleven
//! rooms, a carpet and two ceilings, sixty times a second, for a building
//! in which the only things that move are a ball on a chain, some water, a
//! wrecking ball and whatever the player is turning.
//!
//! **The shells and nothing else, to begin with.** Every room here already
//! says where its walls are as a list of boxes, and that same list is what
//! you cannot walk through. Those are the safe thing to keep, because
//! keeping them writes nothing down twice. The fittings are conditional:
//! a cabinet lights up when the sight finds it, a door opens, a leaf
//! swings. They go on being pushed.

use blitzkit::collision::Aabb;

use crate::behind;

/// Which room a kept run belongs to.
///
/// A run a room, because a run is asked about whole: a building kept as one
/// run is a building that is never behind anything, and the room is this
/// building's own patch. Spec 0058 of the engine on why the engine will not
/// divide a world itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    /// The hall's own walls, per spec 0001.
    Hall,
    /// The space behind the near wall, per spec 0011, which is the largest
    /// room in the building and the one most often behind a wall.
    Behind,
    /// The back room through the gallery's door, per spec 0024.
    Back,
}

/// What a piece is made of, which is the colour it is drawn in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Made {
    Wall,
    Ground,
    Column,
    Frame,
    BackWall,
    BackCeiling,
    Lining,
}

/// One run: where it is, what it is made of, and the boxes in it.
#[derive(Debug)]
pub struct Run {
    pub at: Where,
    pub made: Made,
    pub boxes: Vec<Aabb>,
}

impl Run {
    /// Whether this one is worth handing over rather than pushing.
    ///
    /// **Because a kept run is a draw call of its own.** What is pushed is
    /// batched with everything else of the same mesh and texture, so three
    /// boxes kept are three boxes taken out of a batch and given a draw
    /// call, a sphere, a question and a range. The first build of this kept
    /// everything and the hall went from 227 draws to 246 for no saving at
    /// all.
    ///
    /// Thirty two because the lists in this building are one, three, four,
    /// six, thirteen, and three hundred and fifty one: the columns in the
    /// space behind the wall are the one shell worth keeping and the number
    /// only has to fall between.
    pub fn worth_keeping(&self) -> bool {
        self.boxes.len() >= KEPT_AT
    }
}

/// How many boxes a run needs before it is worth its own draw call.
pub const KEPT_AT: usize = 32;

/// Every run the building hands over once.
///
/// Grouped by room and then by what it is made of, because a run is one
/// mesh and one texture and one colour: two colours in a run would be two
/// draws of the same range or one colour drawn over the other.
pub fn runs(reaches: f32) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::new();
    let mut put = |at: Where, made: Made, boxes: Vec<Aabb>| {
        if !boxes.is_empty() {
            out.push(Run { at, made, boxes });
        }
    };

    // the hall's walls, which are the room's own list and the same one the
    // colliders come from
    put(
        Where::Hall,
        Made::Wall,
        crate::room::Room::of(crate::games()).walls.clone(),
    );

    // the space behind the near wall: its floor and lid, its walls, and the
    // columns. The frame round the far door glows, so it is its own run.
    for (made, want) in [
        (Made::Ground, behind::Made::Ground),
        (Made::Wall, behind::Made::Wall),
        (Made::Column, behind::Made::Column),
        (Made::Frame, behind::Made::Frame),
    ] {
        put(
            Where::Behind,
            made,
            behind::built(reaches)
                .into_iter()
                .filter(|(_, of)| *of == want)
                .map(|(box_, _)| box_)
                .collect(),
        );
    }

    // and the back room: its shell, its lining, and nothing of what stands
    // about in it, which is clutter and moves when somebody moves it
    let (walls, ceiling): (Vec<Aabb>, Vec<Aabb>) = crate::back::solid(reaches)
        .into_iter()
        // the floor is drawn as a plane and the bench has its own legs and
        // top, the same two the drawing has always skipped
        .filter(|box_| box_.max.y > 0.0 && *box_ != crate::back::bench_box(reaches))
        .filter(|box_| !crate::back::clutter_solid(reaches).contains(box_))
        .partition(|box_| box_.min.y < crate::back::TALL);

    put(Where::Back, Made::BackWall, walls);
    put(Where::Back, Made::BackCeiling, ceiling);
    put(Where::Back, Made::Lining, crate::back::lining(reaches));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0026: a run holds one room's boxes and nothing from another's.
    ///
    /// What makes a run worth keeping is that a view can be asked about the
    /// whole of it at once, and a run that reaches into two rooms is a run
    /// that is never wholly behind anything.
    #[test]
    fn a_run_is_one_room() {
        let reaches = reaches();
        let hall = crate::room::Room::of(crate::games());
        let behind = behind::solid(reaches);
        let (west, east) = crate::back::span();
        let (far, near) = crate::back::deep(reaches);
        let back = Aabb::new(
            glam::vec3(
                west - crate::room::THICK * 2.0,
                -1.0,
                far - crate::room::THICK * 2.0,
            ),
            glam::vec3(
                east + crate::room::THICK * 2.0,
                crate::back::TALL + crate::room::THICK * 2.0,
                near + crate::room::THICK * 2.0,
            ),
        );

        for run in runs(reaches) {
            for box_ in run.boxes.iter() {
                let inside = match run.at {
                    Where::Hall => hall
                        .walls
                        .iter()
                        .any(|wall| (wall.min - box_.min).length() < 1e-4),
                    Where::Behind => behind.iter().any(|room| room.intersects(box_)),
                    Where::Back => back.contains_point(box_.center()),
                };

                assert!(
                    inside,
                    "a {:?} piece of the {:?} is not in that room: {:?} to {:?}",
                    run.made, run.at, box_.min, box_.max
                );
            }
        }
    }

    /// Spec 0026: the same lists, every time they are asked for.
    ///
    /// A run is kept once and drawn for as long as it lives, so a list that
    /// came out differently on the second frame would be a building that
    /// rebuilt itself for ever.
    #[test]
    fn standing_still_changes_nothing() {
        let reaches = reaches();
        let once = runs(reaches);
        let twice = runs(reaches);

        assert_eq!(once.len(), twice.len());

        for (one, two) in once.iter().zip(twice.iter()) {
            assert_eq!(one.at, two.at);
            assert_eq!(one.made, two.made);
            assert_eq!(one.boxes.len(), two.boxes.len());

            for (a, b) in one.boxes.iter().zip(two.boxes.iter()) {
                assert_eq!(a.min, b.min);
                assert_eq!(a.max, b.max);
            }
        }
    }

    /// Spec 0026: what is kept is a wall you cannot walk through, and not
    /// something made up for the drawing.
    ///
    /// The kept lists come from the same functions the colliders come from,
    /// which is what keeps the two from drifting: a wall drawn where nothing
    /// stops you is this building's oldest fault, per spec 0013.
    #[test]
    fn the_shells_are_the_colliders() {
        // and the two that are not: they are drawn where the building is
        // all the same, which is the weaker claim this makes about them
        for run in runs(reaches())
            .iter()
            .filter(|run| matches!(run.made, Made::Lining | Made::Ground))
        {
            for box_ in run.boxes.iter() {
                assert!(
                    box_.size().max_element() > 0.0,
                    "a {:?} piece of the {:?} is nothing at all",
                    run.made,
                    run.at
                );
            }
        }

        let reaches = reaches();
        let solid = crate::room::Room::of(crate::games()).building();

        for run in runs(reaches) {
            // two kinds are drawn to a different extent than they are
            // walked, and say so where they are built: the lining is paint
            // on a wall rather than a wall, and the hidden room's floor is
            // drawn only as far as the hall's floor reaches and walked a
            // good deal further, per `behind::built`
            if matches!(run.made, Made::Lining | Made::Ground) {
                continue;
            }

            for box_ in run.boxes.iter() {
                assert!(
                    solid.iter().any(|one| (one.min - box_.min).length() < 1e-3
                        && (one.max - box_.max).length() < 1e-3),
                    "a {:?} piece of the {:?} is drawn where nothing stops you",
                    run.made,
                    run.at
                );
            }
        }
    }

    /// Spec 0026: a run is kept only when it is big enough to pay for the
    /// draw call it costs, and something is.
    #[test]
    fn only_a_run_worth_a_draw_is_kept() {
        let runs = runs(reaches());
        let kept: Vec<&Run> = runs.iter().filter(|run| run.worth_keeping()).collect();

        assert!(!kept.is_empty(), "nothing at all is worth keeping");

        for run in runs.iter() {
            assert_eq!(
                run.worth_keeping(),
                run.boxes.len() >= KEPT_AT,
                "{:?} of the {:?} is kept on something other than its size",
                run.made,
                run.at
            );
        }

        // and the one that is worth it is the one the measurement found:
        // the columns in the space behind the wall
        assert!(
            kept.iter()
                .any(|run| run.at == Where::Behind && run.made == Made::Column),
            "the columns stopped being worth keeping"
        );
    }

    /// And there is something in each room to keep, or this spec has quietly
    /// stopped doing anything.
    #[test]
    fn every_room_hands_something_over() {
        let runs = runs(reaches());

        for at in [Where::Hall, Where::Behind, Where::Back] {
            let boxes: usize = runs
                .iter()
                .filter(|run| run.at == at)
                .map(|run| run.boxes.len())
                .sum();

            assert!(boxes > 0, "the {:?} keeps nothing at all", at);
        }
    }
}
