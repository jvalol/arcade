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
    /// The hotel through the gallery's other door, per spec 0029.
    Hotel,
    /// The register behind the hotel's one unnumbered door, per spec 0031.
    Register,
    /// The other corridor off the hotel's lobby, per spec 0035.
    Twine,
    /// The imaginarium over the hall, per spec 0037.
    Aloft,
    /// The way up to it, per spec 0037, which is in neither room it joins:
    /// it starts on the nook's floor and ends on the imaginarium's.
    Ascent,
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
    HotelWall,
    HotelCeiling,
    HotelEnd,
    RegisterWall,
    RegisterCeiling,
    TwineWall,
    TwineCeiling,
    AloftWall,
    AloftCeiling,
    AscentStair,
    AscentShaft,
}

/// One run: where it is, what it is made of, and the boxes in it.
#[derive(Debug)]
pub struct Run {
    pub at: Where,
    pub made: Made,
    pub boxes: Vec<Aabb>,
}

// **Which run is worth a draw call of its own is no longer asked here.**
// This file carried a `worth_keeping` and a `KEPT_AT` of 32, with a
// comment reasoning that the lists in this building are one, three, four,
// six, thirteen and three hundred and fifty one, so the threshold only had
// to fall between. Engine spec 0064 took that decision into the engine,
// where it is one number with one account of itself rather than one per
// game. Every run is handed over now and the engine keeps the ones worth
// keeping.

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

    // and the hotel: its shell, and nothing of the doors, the plaques, the
    // strips or the desk. A door swings and a strip goes out, and spec 0026
    // keeps what stands still.
    let (walls, ceiling): (Vec<Aabb>, Vec<Aabb>) = crate::aleph::solid(reaches)
        .into_iter()
        // the floors are drawn as planes and the desk is furniture, the same
        // two kinds of thing the back room's drawing skips. The far end is
        // its own run, because it is the one piece in here that is not the
        // colour of a wall.
        .filter(|box_| {
            box_.max.y > 0.0
                && *box_ != crate::aleph::desk(reaches)
                && *box_ != crate::aleph::end(reaches)
        })
        .partition(|box_| box_.min.y < crate::aleph::TALL);

    put(Where::Hotel, Made::HotelWall, walls);
    put(Where::Hotel, Made::HotelCeiling, ceiling);
    put(
        Where::Hotel,
        Made::HotelEnd,
        vec![crate::aleph::end(reaches)],
    );

    // and the register behind the blank door, per spec 0031. Its floor is
    // drawn as a plane, the same as every other floor in this building.
    let (walls, ceiling): (Vec<Aabb>, Vec<Aabb>) = crate::register::solid(reaches)
        .into_iter()
        .filter(|box_| box_.max.y > 0.0)
        .partition(|box_| box_.min.y < crate::register::TALL);

    put(Where::Register, Made::RegisterWall, walls);
    put(Where::Register, Made::RegisterCeiling, ceiling);

    // and the other corridor off the hotel's lobby, per spec 0035. Its
    // floor is drawn as a plane like every other floor in this building.
    let (walls, ceiling): (Vec<Aabb>, Vec<Aabb>) = crate::twine::solid(reaches)
        .into_iter()
        .filter(|box_| box_.max.y > 0.0)
        .partition(|box_| box_.min.y < crate::aleph::TALL);

    put(Where::Twine, Made::TwineWall, walls);
    put(Where::Twine, Made::TwineCeiling, ceiling);

    // and the imaginarium over the hall, per spec 0037. Its treads and
    // landings are the room rather than its shell and are pushed, because
    // what they are made of is what tells them apart.
    //
    // The way up comes out of the same list and is its own place. It runs
    // from the nook's floor to this room's, so it is in neither of them,
    // and the test that every run is one room is what said so.
    let up = crate::aloft::way_up();
    let (walls, ceiling): (Vec<Aabb>, Vec<Aabb>) = crate::aloft::solid()
        .into_iter()
        .filter(|box_| {
            !(0..crate::aloft::FRAMES).any(|n| crate::aloft::treads(n).contains(box_))
                && !up.contains(box_)
        })
        .partition(|box_| box_.min.y < crate::aloft::BASE + crate::aloft::TALL);

    put(Where::Aloft, Made::AloftWall, walls);
    put(Where::Aloft, Made::AloftCeiling, ceiling);

    // **Drawn without the lap.** `up_treads` reaches its landing a tread
    // back under the flight so that no two walking surfaces meet in a
    // plane, which `walk` wants; drawn, that leaves two faces in one plane,
    // which speckles. Spec 0014's settlement is to keep the lap in the
    // colliders and out of the picture.
    let walked = crate::aloft::up_treads();
    let shaft: Vec<Aabb> = up
        .into_iter()
        .filter(|box_| !walked.contains(box_))
        .collect();

    put(Where::Ascent, Made::AscentStair, crate::aloft::up_drawn());
    put(Where::Ascent, Made::AscentShaft, shaft);

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

        // the hotel, which is a lobby and a corridor leaving it, so its box
        // is both and runs west as far as the corridor's shell does
        let hotel = Aabb::new(
            glam::vec3(
                crate::aleph::corridor_west() - crate::room::THICK * 2.0,
                -1.0,
                crate::aleph::across(reaches).0 - crate::room::THICK * 2.0,
            ),
            glam::vec3(
                -crate::room::GALLERY_WIDE * 0.5 + crate::room::THICK * 2.0,
                crate::aleph::LOBBY_TALL + crate::room::THICK * 2.0,
                crate::aleph::across(reaches).1 + crate::room::THICK * 2.0,
            ),
        );

        // and the register, which is its own room off that corridor
        let register = {
            let (west, east) = crate::register::along(reaches);
            let (far, near) = crate::register::across(reaches);

            Aabb::new(
                glam::vec3(
                    west - crate::room::THICK * 2.0,
                    -1.0,
                    far - crate::room::THICK * 2.0,
                ),
                glam::vec3(
                    east + crate::room::THICK * 2.0,
                    crate::register::TALL + crate::room::THICK * 2.0,
                    near + crate::room::THICK * 2.0,
                ),
            )
        };

        // and spec 0035's corridor, which leaves the hotel's lobby and runs
        // two kilometres away from it
        let twine = {
            let middle = crate::twine::middle_x();
            let half = crate::twine::ACROSS * 0.5 + crate::room::THICK * 2.0;

            Aabb::new(
                glam::vec3(
                    middle - half,
                    -1.0,
                    crate::twine::end(reaches) - crate::room::THICK * 2.0,
                ),
                glam::vec3(
                    middle + half,
                    crate::aleph::TALL + crate::room::THICK * 2.0,
                    crate::twine::mouth(reaches) + crate::room::THICK * 2.0,
                ),
            )
        };

        // and the imaginarium, which is the only room above the rest
        let aloft = {
            // out as far as the dark behind the three doors, which stands
            // past the shell on purpose: it is what stops each opening
            // being an opening on to nothing. Spec 0038 put one of those
            // doors in the ceiling, so the room reaches past its own lid as
            // well as past its own walls, and it is the same distance every
            // way because the three frames are one layout turned.
            let out = crate::aloft::DOOR_BACK + crate::room::THICK * 2.0;
            let middle = crate::aloft::middle();

            Aabb::new(
                middle - glam::Vec3::splat(crate::aloft::HALF + out),
                middle + glam::Vec3::splat(crate::aloft::HALF + out),
            )
        };

        // the way up, which is a box round the stair and its shaft and
        // nothing else: it is in neither room it joins, so neither room's
        // own box can hold it.
        let ascent = {
            let boxes = crate::aloft::way_up();
            let (mut low, mut high) = (boxes[0].min, boxes[0].max);

            for box_ in boxes.iter() {
                low = low.min(box_.min);
                high = high.max(box_.max);
            }

            Aabb::new(low, high)
        };

        for run in runs(reaches) {
            for box_ in run.boxes.iter() {
                let inside = match run.at {
                    Where::Hall => hall
                        .walls
                        .iter()
                        .any(|wall| (wall.min - box_.min).length() < 1e-4),
                    Where::Behind => behind.iter().any(|room| room.intersects(box_)),
                    Where::Back => back.contains_point(box_.center()),
                    Where::Hotel => hotel.contains_point(box_.center()),
                    Where::Register => register.contains_point(box_.center()),
                    Where::Twine => twine.contains_point(box_.center()),
                    Where::Aloft => aloft.contains_point(box_.center()),
                    Where::Ascent => ascent.contains_point(box_.center()),
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

            // and a third, which is held to more than the other two. The way
            // up's landing laps the flight in the colliders so that no two
            // walking surfaces meet in a plane, and is drawn without that
            // lap because two drawn faces in one plane speckle, per spec
            // 0014. So its drawn boxes are not the walked ones, but every
            // one of them has to be inside a walked one: drawn short of
            // what stops you is a picture of a stair, drawn past it is a
            // stair you fall through.
            if run.made == Made::AscentStair {
                for box_ in run.boxes.iter() {
                    assert!(
                        solid.iter().any(|one| {
                            one.min.cmple(box_.min + 1e-3).all()
                                && one.max.cmpge(box_.max - 1e-3).all()
                        }),
                        "a drawn piece of the way up is outside everything that stops you"
                    );
                }

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
