//! A corridor off the hall whose two long walls are mirrors. Spec 0061.
//!
//! The first room in this building that shows the person walking it. Jake's
//! idea from 2026-10-16, which named its own hard part: everything here is
//! seen from inside one head and there was no body to reflect.
//!
//! **Where the numbers come from.** `room::grounds` was asked where the
//! building actually is rather than the constants being read and added up,
//! because this project has lost a sitting to the second way. Every ground
//! at floor level is taken except the strip east of the hall, below the
//! solar system's z 6.50 and above the gallery's z -9.50, and this is in
//! that strip.
//!
//! **And the doorway was asked too.** The hall's east wall is one box from
//! z -9.50 to 9.50 with four cabinets standing against it, and there is a
//! gap between the middle two from z -0.62 to 0.63. That gap is the way in,
//! so the corridor is centred on it rather than on anything tidier, and
//! `no_cabinet_stands_in_the_doorway` is what says it stays a gap when a
//! fourteenth game moves the cabinets.

use crate::room::{TALL, THICK};
use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// The nook's west wall, which is this corridor's east side.
///
/// Borrowed rather than built, the way the garden borrows the hall's wall:
/// a second box in the same place is two surfaces in one plane.
pub const EAST: f32 = -12.85;

/// How far west the corridor reaches.
///
/// **Stopped short of x -17.15, which is where the cellar's footprint
/// starts.** The cellar is three metres down, so there is no collision, but
/// its lid and this floor would abut and the question of whose top face is
/// whose is the fault this building keeps hitting.
pub const WEST: f32 = -16.90;

/// Where the corridor starts and stops along the nook.
///
/// **North of the cellar stair, which is the thing that was in the way.**
/// The nook's west wall is already in three pieces with an opening at
/// z -6.75 to -4.55, and that opening is the way down: nine steps marching
/// west from x -13.85 to -16.25 at z -6.85 to -4.45. So this begins just
/// north of them.
///
/// And it stops short of the garden, whose west wall reaches down to z 6.75
/// two metres behind this corridor's own. That sliver was in the reflection
/// until `nothing_stands_behind_the_glass` said so.
pub const SOUTH: f32 = -4.30;
pub const NORTH: f32 = 5.60;

/// Where the nook's own floor slab ends, which is where this one meets it.
const NOOK_FLOOR_ENDS: f32 = -12.70;

/// The middle of the corridor across its width, and half that width.
///
/// Derived rather than written, because the walls are where they are for
/// reasons of their own and the middle is wherever that leaves it.
pub fn middle() -> f32 {
    (WEST + EAST) * 0.5
}

pub fn wide() -> f32 {
    (EAST - WEST) * 0.5
}

/// How far the glass stands off the wall it hangs on.
///
/// Two centimetres, and it matters more than it looks: the plane handed to
/// the engine is the glass's and not the wall's, and reflecting in the wall
/// would put the whole corridor two centimetres deeper than it is.
pub const PROUD: f32 = 0.02;

/// How far the frames stand off, which is further than the glass.
///
/// So no two drawn faces share a plane, which is the fault this building has
/// hit seven times, and so the frame reads as holding the glass rather than
/// being level with it.
pub const FRAMED: f32 = 0.05;

/// The doorway's span in z, cut out of the nook's west wall.
///
/// **One place says where**, the way `garden::way` is the only place that
/// says where the garden's opening is. The nook's wall is built from this.
///
/// **Exactly one bookcase wide, and behind that bookcase.** The nook's west
/// wall is lined end to end with them, 1.10 apart, so there is no clear
/// stretch to put a door in and never was. Spec 0007 already settled what to
/// do about that for the way down to the cellar: one of the bookcases swings,
/// and behind it the wall is not there. A wall with a bookcase sized hole in
/// it is still a wall from the room, because a bookcase is standing in the
/// hole.
///
/// So this is the span of the pair at z 0.95 to 3.15, and not a round
/// number. Sized to anything else and the hole shows past the bookcase's
/// edge, which the nook's own reference picture noticed at 1.20.
pub fn way() -> (f32, f32) {
    // Where the pair at `CASE` stands, taken from a bookcase's own width
    // rather than written out: 3.14 as a literal is not a number anybody
    // should have to recognise, and clippy reads it as pi.
    //
    // A centimetre inside the pair at each end, so a sconce sitting exactly
    // on a bookcase's edge still has wall behind it. The one at z 2.05 came
    // out at 2.0499992 and was inside the hole.
    const FIRST: f32 = 0.95;
    const INSET: f32 = 0.01;
    let span = crate::study::CASE.x * CASES as f32;

    (FIRST + INSET, FIRST + span - INSET)
}

/// Which bookcase along the nook's west wall is this room's leaf.
///
/// **Nine, and the opening is its span exactly.** The wall is lined end to
/// end with them 1.10 apart, so there was never a clear stretch to put a door
/// in. Spec 0007 settled that for the way down: one swings, and a wall with a
/// bookcase sized hole in it is still a wall from the room.
///
/// Not next to the cellar's pair at 2 and 3, and not at either end of the
/// run. Two secret doors side by side is one secret door twice as wide.
pub const CASE: usize = 9;

/// How many of them swing, from [`CASE`].
///
/// **Two, and the cellar's pair is why.** A bookcase is 1.10 wide and a body
/// is 0.90 across, so one leaf leaves ten centimetres a side and the walk
/// stops dead in the opening. The cellar's way down is two leaves for the
/// same reason; it was not obvious until this room tried one and could not
/// get through its own door.
pub const CASES: usize = 2;

/// Whether a bookcase is one of this room's leaves.
pub fn swings(n: usize) -> bool {
    (CASE..CASE + CASES).contains(&n)
}

/// Which way a leaf swings: away from the middle of the opening, so the two
/// open outwards and leave the way through between them, as the cellar's do.
pub fn swings_out(n: usize) -> f32 {
    let middle = CASE as f32 + (CASES as f32 - 1.0) * 0.5;

    if (n as f32) < middle {
        -1.0
    } else {
        1.0
    }
}

/// The one book on this pair that is not a book: which shelf, and which
/// along it. Spec 0061, copying spec 0007's own handle.
///
/// **Without it the copy lies.** The prompt says another of these is not a
/// book, and nothing on the case was marked, so there was nothing to find:
/// Jake, 2026-11-08, aside from the text it's not obvious this is a secret
/// passageway. The cellar's pair has carried a marked spine since spec 0007
/// and this one had none.
///
/// A different shelf and a different place along it from the cellar's, so
/// finding one does not tell you where the other is.
pub const BOOK_SHELF: usize = 3;
pub const BOOK: usize = 2;

/// How high the opening is, which is how tall a bookcase is.
///
/// **Not the wall's full height.** The bookcase standing in the hole is 1.90
/// and the wall is 3.20, so a full-height opening shows as a bright strip
/// above the bookcase. The nook's own reference picture caught it at 0.4% of
/// its pixels, which is a strip you would notice standing there. The cellar's
/// way down has a lintel over it for the same reason.
pub const HIGH: f32 = 1.90;

/// How many panels to a wall.
pub const PANELS: usize = 5;

/// How wide one panel of glass is, and how tall, and where its foot sits.
pub const PANEL_WIDE: f32 = 1.60;
pub const PANEL_FOOT: f32 = 0.35;
pub const PANEL_HEAD: f32 = 2.55;

/// Middle to middle along the corridor.
pub const PITCH: f32 = 1.80;

/// Where the first panel's middle is, so the run is centred on the corridor.
fn first() -> f32 {
    let run = PANEL_WIDE + PITCH * (PANELS - 1) as f32;
    SOUTH + (NORTH - SOUTH - run) * 0.5 + PANEL_WIDE * 0.5
}

/// Where each panel's middle sits along the corridor.
pub fn along() -> Vec<f32> {
    (0..PANELS).map(|n| first() + PITCH * n as f32).collect()
}

/// The same, for one wall, with any panel that would cover the way through
/// left out.
///
/// **The east wall is the nook's, and the way through is a hole in it.** Two
/// panels landed across that hole, so from inside the corridor the way out
/// was behind a mirror: Jake, 2026-11-08, the room doesn't seem to have an
/// exit even though one is visible in one of the mirrors. He could see the
/// opening reflected in the west wall and could not find it in the east.
///
/// Where the panels go and where the way through is are two questions that
/// happen to be on one wall, so each asks its own. The west wall has no hole
/// in it and keeps all five.
pub fn along_wall(side: f32) -> Vec<f32> {
    let (south, north) = way();

    along()
        .into_iter()
        .filter(|at| {
            // the west wall has nothing to keep clear of
            if side < 0.0 {
                return true;
            }

            let (from, to) = (at - PANEL_WIDE * 0.5, at + PANEL_WIDE * 0.5);

            from >= north || to <= south
        })
        .collect()
}

/// Which wall a side is: -1 for the west one, 1 for the east, which is the
/// nook's own.
pub const SIDES: [f32; 2] = [-1.0, 1.0];

/// The plane one wall's glass lies in, and which way it faces.
///
/// This is what goes to `Scene::mirror`, and it is the glass's own plane
/// rather than the wall's.
pub fn plane(side: f32) -> (Vec3, Vec3) {
    (
        vec3(middle() + side * (wide() - PROUD), 0.0, 0.0),
        vec3(-side, 0.0, 0.0),
    )
}

/// One panel of glass: where it stands and how big it is.
///
/// Zero thickness in x, because it is a pane and not a slab.
pub fn pane(side: f32, at: f32) -> (Vec3, Vec3) {
    (
        vec3(
            middle() + side * (wide() - PROUD),
            (PANEL_FOOT + PANEL_HEAD) * 0.5,
            at,
        ),
        vec3(0.0, PANEL_HEAD - PANEL_FOOT, PANEL_WIDE),
    )
}

/// The floor, which meets the nook's rather than lapping under it.
///
/// **It started out lapped and the engine's own coplanar check refused it.**
/// Two floors at one height that overlap put both their tops in one plane,
/// and `behind` came back as carpet torn into bands for exactly that. There
/// the answer is two numbers for two questions, because its lap runs five
/// metres under a wall; here the seam is a doorway in the open and the walk
/// crosses it level, which `the_floors_lap_without_a_step` holds.
pub fn floor() -> Aabb {
    Aabb::new(
        vec3(WEST - THICK, -THICK, SOUTH - THICK),
        vec3(NOOK_FLOOR_ENDS, 0.0, NORTH + THICK),
    )
}

/// The same, which the drawing uses.
///
/// One function would do today. It is two because the moment they come
/// apart, which is the moment anything laps, the drawing must not be the one
/// that reaches for the walk's number.
pub fn drawn_floor() -> Aabb {
    floor()
}

/// The ceiling, hung at the height every other room's is.
pub fn lid() -> Aabb {
    let soffit = crate::room::soffit();

    Aabb::new(
        vec3(WEST - THICK, soffit, SOUTH - THICK),
        vec3(NOOK_FLOOR_ENDS, soffit + THICK, NORTH + THICK),
    )
}

/// Everything solid: the floor, the two long walls, the end, and the lid.
///
/// The west end is open, because that is the hall, the way the back room
/// leaves its own west end open onto the gallery.
pub fn solid() -> Vec<Aabb> {
    let mut out = vec![floor(), lid()];

    // the west wall, the long one this room builds for itself
    out.push(Aabb::new(
        vec3(WEST - THICK, 0.0, SOUTH - THICK),
        vec3(WEST, TALL, NORTH + THICK),
    ));

    // and the two ends. They run east into the nook's own wall rather than
    // stopping against it: two walls that meet exactly share a plane, and
    // two drawn faces in one plane is the fault this building has hit seven
    // times.
    for (near, far) in [(SOUTH - THICK, SOUTH), (NORTH, NORTH + THICK)] {
        out.push(Aabb::new(
            vec3(WEST - THICK, 0.0, near),
            vec3(NOOK_FLOOR_ENDS, TALL, far),
        ));
    }

    out.extend(frames());

    out
}

/// The frames round each panel: a sill, a head and a stile either side.
///
/// **What makes it panels rather than one sheet of glass.** Fourteen metres
/// of unbroken mirror is one surface, and a long surface with nothing
/// breaking it is the thing this building has been told about in every room.
/// They stand further off the wall than the glass does, so nothing shares a
/// plane with anything.
/// Everything to draw: the same shell, with the drawn floor in place of the
/// walked one.
///
/// **A list of its own rather than the caller remembering**, because the one
/// difference between the two is the single thing in here that is easy to get
/// wrong and invisible when you do: a drawing that reached for `solid` would
/// put the lap back and tear the carpet.
pub fn drawn() -> Vec<Aabb> {
    solid()
        .into_iter()
        .map(|box_| if box_ == floor() { drawn_floor() } else { box_ })
        .collect()
}

pub fn frames() -> Vec<Aabb> {
    let mut out = Vec::new();
    let bead = 0.09;

    for side in SIDES {
        let face = middle() + side * wide();
        let stood = middle() + side * (wide() - FRAMED);
        let (near, far) = (face.min(stood), face.max(stood));

        for at in along_wall(side) {
            let (south, north) = (at - PANEL_WIDE * 0.5, at + PANEL_WIDE * 0.5);

            // the sill and the head
            for (low, high) in [
                (PANEL_FOOT - bead, PANEL_FOOT),
                (PANEL_HEAD, PANEL_HEAD + bead),
            ] {
                out.push(Aabb::new(
                    vec3(near, low, south - bead),
                    vec3(far, high, north + bead),
                ));
            }

            // and a stile each side
            for (near_z, far_z) in [(south - bead, south), (north, north + bead)] {
                out.push(Aabb::new(
                    vec3(near, PANEL_FOOT - bead, near_z),
                    vec3(far, PANEL_HEAD + bead, far_z),
                ));
            }
        }
    }

    out
}

/// Where the lamps go, which is in the frames rather than on the ceiling.
///
/// **Derived from the frames and not written beside them**, for the reason
/// `spa::lamps` is `spa::sconces` lifted: two lists of where the light is
/// come apart, and the baths once had a glowing ceiling and dark sconces.
/// One over each panel's head, a little out into the room.
pub fn lamps() -> Vec<Vec3> {
    let mut out = Vec::new();

    for side in SIDES {
        for at in along_wall(side) {
            out.push(vec3(
                middle() + side * (wide() - 0.28),
                PANEL_HEAD + 0.18,
                at,
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0061. The plane handed to the engine is the glass's own.
    ///
    /// **Two centimetres, and it is the difference between a reflection and
    /// a reflection of the wrong room.** Reflecting in the wall rather than
    /// in the glass puts the whole corridor `PROUD` deeper than it is, and
    /// what that looks like is a mirror very slightly out, which is the kind
    /// of wrong nobody can name from a picture.
    #[test]
    fn the_mirror_plane_is_the_glass_and_not_the_wall() {
        for side in SIDES {
            let (on, normal) = plane(side);
            let (pane_at, _) = pane(side, along()[0]);

            assert!(
                (on.x - pane_at.x).abs() < 1e-6,
                "the mirror plane is at x {} and the glass at {}",
                on.x,
                pane_at.x
            );
            assert!(
                (on.x - (middle() + side * wide())).abs() > 1e-6,
                "the mirror plane is the wall's at x {}, not the glass's",
                on.x
            );
            // and it faces into the room
            assert!(
                normal.x * side < 0.0,
                "the glass on side {} faces {}, which is into the wall",
                side,
                normal.x
            );
        }
    }

    /// Spec 0061. Every panel on a wall is in that wall's one plane, which
    /// is what makes one target right for six of them.
    #[test]
    fn every_panel_on_a_wall_is_in_the_same_plane() {
        for side in SIDES {
            let (on, _) = plane(side);

            for at in along() {
                let (pane_at, size) = pane(side, at);
                assert!(
                    (pane_at.x - on.x).abs() < 1e-6,
                    "a panel at z {} sits at x {} and the wall's plane is {}",
                    at,
                    pane_at.x,
                    on.x
                );
                assert!(size.x.abs() < 1e-6, "a pane has thickness, so it is a slab");
            }
        }
    }

    /// Spec 0061. The panels fit between the walls with the run centred.
    #[test]
    fn the_panels_fit_the_corridor_and_are_centred_in_it() {
        let places = along();
        assert_eq!(places.len(), PANELS);

        let first_edge = places[0] - PANEL_WIDE * 0.5;
        let last_edge = places[PANELS - 1] + PANEL_WIDE * 0.5;

        assert!(
            first_edge > SOUTH,
            "the first panel starts at {} and the corridor at {}",
            first_edge,
            SOUTH
        );
        assert!(
            last_edge < NORTH,
            "the last panel ends at {} and the corridor at {}",
            last_edge,
            NORTH
        );
        assert!(
            ((first_edge - SOUTH) - (NORTH - last_edge)).abs() < 1e-4,
            "the run is not centred: {} spare at the south end and {} at the north",
            first_edge - SOUTH,
            NORTH - last_edge
        );
        // and they do not overlap each other
        const _: () = assert!(PITCH > PANEL_WIDE);
    }

    /// Spec 0061. The frames stand clear of the glass, so no two drawn faces
    /// share a plane.
    #[test]
    fn the_frames_stand_clear_of_the_glass() {
        // `const`s, so this is a compile-time truth stated where a reader
        // of this test will look for it rather than an assertion that can
        // fail
        const _: () = assert!(FRAMED > PROUD);

        for side in SIDES {
            let (on, _) = plane(side);
            for frame in frames() {
                let faces = [frame.min.z, frame.max.z];
                for face in faces {
                    assert!(
                        (face - on.z).abs() > 1e-4,
                        "a frame face at z {} is in the glass's own plane",
                        face
                    );
                }
            }
        }
    }

    /// Spec 0061. Every lamp is in a fitting, which here means over a panel.
    ///
    /// The baths were built with the lamps along the ceiling and the sconces
    /// on a wall, so the ceiling glowed at nothing and every sconce was dark.
    #[test]
    fn every_lamp_sits_over_a_panel() {
        let places = along();
        // per wall, because the east one leaves out the panels that would
        // cover the way through
        let want: usize = SIDES.iter().map(|side| along_wall(*side).len()).sum();
        assert_eq!(lamps().len(), want);
        assert!(want < places.len() * SIDES.len(), "no panel was left out");

        for lamp in lamps() {
            assert!(
                places.iter().any(|at| (at - lamp.z).abs() < 1e-4),
                "a lamp at z {} is over no panel",
                lamp.z
            );
            assert!(
                lamp.y > PANEL_HEAD,
                "a lamp at y {} is behind the glass rather than over it",
                lamp.y
            );
            assert!(
                (lamp.x - middle()).abs() < wide(),
                "a lamp at x {} is inside the wall",
                lamp.x
            );
        }
    }

    /// Spec 0061. The floor meets the hall's rather than lapping under it,
    /// and the walk is level across the seam.
    ///
    /// **This started out lapped and was wrong.** Two floors at one height
    /// that overlap put both their tops in one plane, and the engine's own
    /// coplanar check found it at once: x 2.40 to 2.70 at y 0, which is the
    /// floor you walk in on. `behind` came back as carpet torn into bands for
    /// the same reason, and its answer was two numbers for two questions.
    ///
    /// Here one number does, which is what the walk says: the seam at the
    /// hall's edge is crossed level, and `the_floors_lap_without_a_step`
    /// walks it. The lap `behind` needs is for a five metre gap under a wall;
    /// this is a doorway in the open.
    #[test]
    fn the_floor_meets_the_nooks_without_a_lap() {
        let ours = floor();

        assert!(
            (ours.max.x - NOOK_FLOOR_ENDS).abs() < 1e-6,
            "this floor ends at {} and the nook's starts at {}: overlapping, \
             their tops are one plane and the two fight",
            ours.max.x,
            NOOK_FLOOR_ENDS
        );
        assert_eq!(
            drawn_floor(),
            ours,
            "the drawn floor and the walked one have come apart, which is two \
             numbers where the walk says one will do"
        );
        assert!(
            ours.max.y <= 0.0 && ours.min.y < 0.0,
            "the floor's top is not where every other floor's is"
        );
    }

    /// Spec 0061. The corridor is clear of everything else in the building.
    ///
    /// **Asked of the building rather than of the constants**, which is the
    /// whole reason the placement was asked of `room::grounds` first. A room
    /// that overlaps another is the fault that put a sky inside the building
    /// and a garden under a tongue of the hall's floor.
    #[test]
    fn the_corridor_is_clear_of_every_other_room() {
        let room = crate::room::Room::of(crate::games());
        let inside = Aabb::new(
            vec3(WEST + 0.01, 0.01, SOUTH + 0.01),
            vec3(EAST - 0.01, crate::room::soffit() - 0.01, NORTH - 0.01),
        );

        let ours = solid();

        for (name, ground) in room.grounds() {
            let meets = ground.min.x < inside.max.x
                && ground.max.x > inside.min.x
                && ground.min.z < inside.max.z
                && ground.max.z > inside.min.z
                && ground.min.y < inside.max.y
                && ground.max.y > inside.min.y;

            assert!(
                !meets || ours.contains(&ground),
                "{} reaches into the corridor: x {:.2}..{:.2} z {:.2}..{:.2}",
                name,
                ground.min.x,
                ground.max.x,
                ground.min.z,
                ground.max.z
            );
        }
    }

    /// Spec 0061. Nothing is built behind the wall this room builds for
    /// itself, which is the west one.
    ///
    /// **The east wall is the nook's, and the nook is right behind it.** That
    /// is fine now and was not: engine spec 0090 drew a reflection from the
    /// reflected eye with no near plane on the glass, so standing two metres
    /// off a panel put that eye two metres into the nook and everything
    /// between was in the picture. A fitting 30 cm the far side of the
    /// borrowed wall is what found it, and engine spec 0093 is the answer.
    ///
    /// So this checks only the side where emptiness is still doing work: the
    /// west wall backs onto open ground, and a room built out there would
    /// not show through, but it would be two walls in one plane.
    #[test]
    fn nothing_is_built_against_the_west_wall() {
        let room = crate::room::Room::of(crate::games());
        let ours = solid();

        for box_ in room.building() {
            if ours.contains(&box_) {
                continue;
            }
            let meets = box_.min.z < NORTH
                && box_.max.z > SOUTH
                && box_.min.x < WEST
                && box_.max.x > WEST - THICK - 0.5
                && box_.min.y < TALL
                && box_.max.y > 0.0;

            assert!(
                !meets,
                "something is against the west wall: x {:.2}..{:.2} \
                 z {:.2}..{:.2}, which is two walls in one plane",
                box_.min.x, box_.max.x, box_.min.z, box_.max.z
            );
        }
    }

    /// Spec 0061. No panel, frame or lamp covers the way through.
    ///
    /// **This is the one Jake found by standing in the room.** Two of the
    /// five panels landed across the opening, so the way out was behind a
    /// mirror: he could see the gap reflected in the west wall and could not
    /// find it in the east. Every other test passed, because they all asked
    /// about panels or about the opening and none asked about both.
    #[test]
    fn nothing_hangs_over_the_way_through() {
        let (south, north) = way();

        for at in along_wall(1.0) {
            let (from, to) = (at - PANEL_WIDE * 0.5, at + PANEL_WIDE * 0.5);
            assert!(
                from >= north || to <= south,
                "a panel covering z {:.2}..{:.2} hangs over the way through at \
                 {:.2}..{:.2}, so the way out is behind a mirror",
                from,
                to,
                south,
                north
            );
        }

        // the frames follow the panels, so they must be clear too
        for frame in frames() {
            let on_the_nooks_wall = frame.min.x > middle();
            if !on_the_nooks_wall {
                continue;
            }
            assert!(
                frame.min.z >= north || frame.max.z <= south,
                "a frame at z {:.2}..{:.2} hangs over the way through",
                frame.min.z,
                frame.max.z
            );
        }

        // and the west wall keeps all of them, because it has no hole
        assert_eq!(
            along_wall(-1.0).len(),
            PANELS,
            "the west wall lost a panel to an opening it does not have"
        );
        assert!(
            along_wall(1.0).len() < PANELS,
            "the east wall kept every panel, so none of them was in the way \
             and this test is checking nothing"
        );
    }

    /// Spec 0061. The opening is exactly the pair of bookcases that swing.
    ///
    /// **This replaced a check on the hall's cabinets.** The room was going
    /// to be off the hall's east wall and that wall turned out to be full:
    /// its cabinets grow symmetrically out from z 0 with the game count, so
    /// the middle is never free and the free ends move. When the room moved
    /// to the nook the old test went on passing while checking a wall
    /// nothing was in, which is worse than no test.
    ///
    /// What matters here instead is that the hole and the leaves standing in
    /// it are the same span. `way` holds its own numbers, so if a bookcase
    /// ever moves this is what says the hole no longer matches the thing
    /// hiding it.
    #[test]
    fn the_opening_is_exactly_the_leaves_that_swing() {
        let room = crate::room::Room::of(crate::games());
        let (south, north) = way();

        let leaves: Vec<_> = (CASE..CASE + CASES)
            .filter_map(|n| room.bookcases.get(n))
            .map(crate::room::Room::bookcase_box)
            .collect();

        assert_eq!(
            leaves.len(),
            CASES,
            "there are not {} leaves to swing",
            CASES
        );

        let from = leaves.iter().map(|b| b.min.z).fold(f32::INFINITY, f32::min);
        let to = leaves
            .iter()
            .map(|b| b.max.z)
            .fold(f32::NEG_INFINITY, f32::max);

        assert!(
            south > from && north < to,
            "the opening is z {:.2}..{:.2} and the leaves cover {:.2}..{:.2}, \
             so the hole shows past the bookcases standing in it",
            south,
            north,
            from,
            to
        );
        assert!(
            (south - from) < 0.05 && (to - north) < 0.05,
            "the opening is z {:.2}..{:.2} inside leaves covering {:.2}..{:.2}, \
             which is a wall where the way through should be",
            south,
            north,
            from,
            to
        );
    }

    /// The whole building's boxes, because a walk that only knows this
    /// corridor's own walks through the hall's wall.
    fn everything() -> Vec<Aabb> {
        opened(1.0)
    }

    /// The building with this room's leaf however far open.
    fn opened(by: f32) -> Vec<Aabb> {
        let mut room = crate::room::Room::of(crate::games());
        room.mirror_swing = by;
        room.solid()
    }

    /// Drives a body through a building whose leaf is however far open.
    fn walked_with(solid: &[Aabb], from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
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

    /// Drives a body this size with the game's own `walk` from one place
    /// towards another, and says where it ended up.
    ///
    /// **Walked and not measured**, which is what this building has learned
    /// the hard way: the pool's steps had four legal rises and were one solid
    /// block you could not climb, and every rise passed.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid = everything();
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
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    /// Spec 0061. You can get in from the hall and back out again.
    ///
    /// **Both ways, and from more than one spot.** One path in is not a way
    /// in: you walk at a door from wherever you happen to be standing, and
    /// the doorway here is a 1.20 gap between two cabinets.
    /// Spec 0061. With the leaf open you can get in from the nook and back
    /// out again, and with it shut you cannot.
    ///
    /// **Both halves, because a secret door that is always open is a
    /// doorway.** The wall is lined end to end with bookcases 1.10 apart, so
    /// there was never a clear stretch for a door and spec 0007 had already
    /// settled what to do: one swings, and a wall with a bookcase sized hole
    /// in it is still a wall from the room.
    #[test]
    fn the_nook_lets_you_into_the_mirrors_and_back() {
        let through = (way().0 + way().1) * 0.5;
        let inside = vec3(middle(), 1.0, through);

        // in, from three places in the nook, not one: you walk at a door
        // from wherever you happen to be standing
        // spread in front of the opening rather than along the whole wall:
        // the rest of that wall is bookcases that do not swing, and walking
        // into one is the door working
        for from in [
            vec3(-11.0, 1.0, through),
            vec3(-11.0, 1.0, through - 0.4),
            vec3(-11.0, 1.0, through + 0.4),
        ] {
            let got = walked(from, inside, 10.0);

            assert!(
                got.x < EAST - 0.5,
                "from the nook at z {:.1} you got to x {:.2}, which is still \
                 outside the corridor",
                from.z,
                got.x
            );
        }

        // and back out into the nook
        let out = walked(inside, vec3(-11.0, 1.0, through), 10.0);
        assert!(
            out.x > EAST + 0.3,
            "you ended at x {:.2}, so you cannot get back out",
            out.x
        );

        // **and shut, you cannot.** A secret door that is always open is a
        // doorway, and nothing about a picture of a bookcase says which.
        let shut = opened(0.0);
        let stopped = walked_with(
            &shut,
            vec3(-11.0, 1.0, through),
            vec3(middle(), 1.0, through),
            10.0,
        );
        assert!(
            stopped.x > EAST,
            "with the leaf shut you still got to x {:.2}, so the bookcase is \
             not standing in the hole",
            stopped.x
        );
    }

    /// Spec 0061. You can walk the whole length without being stopped.
    ///
    /// The panels stand proud of the walls and the frames stand proud of the
    /// panels, so the clear width is less than the four metres between the
    /// walls. This walks it rather than subtracting.
    #[test]
    fn nothing_blocks_the_walk_down_the_corridor() {
        let start = vec3(WEST + 0.6, 1.0, 0.0);
        let far = vec3(EAST - 0.6, 1.0, 0.0);

        let got = walked(start, far, 20.0);
        assert!(
            got.x > EAST - 1.2,
            "you got to x {:.2} of {:.2}, so something stops you partway",
            got.x,
            EAST - 0.6
        );

        // and along each side, close to the glass, where the frames are
        for side in SIDES {
            let near = middle() + side * (wide() - FRAMED - crate::RADIUS - 0.05);
            let got = walked(
                vec3(near, 1.0, SOUTH + 0.6),
                vec3(near, 1.0, NORTH - 0.6),
                20.0,
            );
            assert!(
                got.z > NORTH - 1.2,
                "walking the side at x {:.2} stopped at z {:.2}",
                near,
                got.z
            );
        }
    }

    /// Spec 0061. You stand on the floor rather than falling through it or
    /// rising onto a ledge where the two floors lap.
    #[test]
    fn the_floors_lap_without_a_step() {
        let mut last: Option<f32> = None;

        // across the seam between the nook's floor and this one, which is
        // at NOOK_FLOOR_ENDS, standing on a line through the doorway
        let through = (way().0 + way().1) * 0.5;

        for x in [-11.5f32, -12.2, -12.7, -13.0, -13.6, -14.5] {
            let at = walked(vec3(x, 1.2, through), vec3(x, 1.2, through), 1.5);
            if let Some(was) = last {
                assert!(
                    (at.y - was).abs() < 1e-3,
                    "standing at x {} puts you at y {:.3} where the step \
                     before was {:.3}, so there is a ledge at the seam",
                    x,
                    at.y,
                    was
                );
            }
            last = Some(at.y);
        }
    }
}
