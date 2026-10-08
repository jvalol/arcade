//! The space behind the wall you wake with your back to. Spec 0011.
//!
//! There is no door, no sign, and nothing in the hall that says it is here.
//! You find it by walking into the wall off centre, or because somebody told
//! you.
//!
//! Every other room in this building is furnished and this is the opposite: a
//! space, very large and very dark, with nothing in it to look at and no
//! reason given for it being there.
//!
//! **The way in is a hole in the collider that the drawn wall covers.** That
//! is a fault everywhere else in this building and the point here. If a test
//! fails over it, the test is right to ask and the answer is spec 0011.
//!
//! Nothing here draws. Where the gap is, how big the space is and where the
//! columns stand are arithmetic, and arithmetic can be checked without a
//! window.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

use crate::room::{CABINET, THICK, WALL};

/// How far the space runs east of the hall's left wall, and how far back from
/// the hall's near end.
///
/// East, because west is taken. The garden is off the hall's left and reaches
/// well past the near end; east of the hall is unbuilt ground.
///
/// Further than the light goes, which is the only size that matters. The
/// camera's far plane is a hundred and a point light falls off long before
/// that, so what you see is dark in every direction and the far wall is a
/// thing you find rather than a thing you see.
pub const SPAN: f32 = 44.0;
pub const DEEP: f32 = 52.0;

/// How high the ceiling is.
///
/// Lower than the hall's, which is 3.2. Low and vast is uneasy in a way that
/// tall and vast is not: tall and vast is a cathedral, and a cathedral is a
/// thing somebody built on purpose and is proud of.
pub const HIGH: f32 = 2.7;

/// How far this floor runs back under the hall's.
///
/// Far enough that neither slab's end face is anywhere near the join. See the
/// floor in `built`.
pub const LAPS: f32 = 3.0;

/// How wide the way through the wall is, and how far off the middle of it.
///
/// Off centre, so walking straight back from where you wake meets solid wall.
/// A gap in the middle is found by everybody on their first turn round and a
/// gap at the very end is found by nobody.
///
/// A body is 0.9 across. At 1.15 it was a gap you had to line up with: the walk
/// through it wedged half way, because a sweep with an eighth of a unit either
/// side of a sphere is a sweep that catches on a shoulder. A third of a unit
/// either side is a gap you walk through.
pub const WAY_WIDE: f32 = 1.5;
pub const WAY_OFF: f32 = -1.4;

/// Where the way through is, as the two x it runs between.
///
/// The one account of it. The wall is built round it, the test that says the
/// rest of the wall is a wall reads it, and the columns are kept out of it.
pub fn way() -> (f32, f32) {
    (WAY_OFF - WAY_WIDE * 0.5, WAY_OFF + WAY_WIDE * 0.5)
}

/// The hall's near end wall, in two pieces with the way in between them.
///
/// Only the collider is in two pieces. What is drawn is one wall across the
/// whole of it, per spec 0011, which is the whole trick and is the one place
/// in this building where the two are meant to disagree.
pub fn near_wall(reaches: f32) -> Vec<Aabb> {
    let side = WALL + CABINET.x;
    let (from, to) = way();

    vec![
        Aabb::from_center_size(
            vec3((-side + from) * 0.5, crate::room::TALL * 0.5, reaches),
            vec3(from + side, crate::room::TALL, THICK),
        ),
        Aabb::from_center_size(
            vec3((to + side) * 0.5, crate::room::TALL * 0.5, reaches),
            vec3(side - to, crate::room::TALL, THICK),
        ),
    ]
}

/// The middle of the floor, and how far it reaches from there.
///
/// Its near edge is the hall's far edge, meeting it rather than lapping over
/// it. Two floors at one height that overlap put one's end face in the middle
/// of the other's floor, and a body resting on a surface counts as touching
/// it: that is what jammed the walk across the garden where the baths below
/// happened to end.
pub fn at(reaches: f32) -> Vec3 {
    let side = WALL + CABINET.x;

    vec3(-side + SPAN * 0.5, 0.0, reaches + DEEP * 0.5)
}

pub fn half() -> Vec2 {
    vec2(SPAN * 0.5, DEEP * 0.5)
}

/// What a box of this space is, so the drawing does not have to count them.
///
/// Told apart here rather than by where they fall in the list. A list read by
/// index is two accounts of one thing, and this building knows what those do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Made {
    /// The floor and the lid, which are the same thing twice.
    Ground,
    Wall,
    Column,
}

/// The floor, running `back` past the near wall and under the hall's own.
///
/// It is drawn with no lap and walked with one, and the two numbers are not
/// the same because they are answering different questions.
///
/// Walked: two floors at one height that meet edge to edge put a vertical
/// face at the surface you are walking on, and a body resting on a surface
/// counts as touching it, so the walk through the gap stopped dead four
/// centimetres short of the seam. Lapped a good way under, both faces are
/// deep inside the other slab and there is no edge at the join at all.
///
/// Drawn: lapped, its top and the hall's are one plane over five metres by
/// three, and the two fight. That patch is the floor you wake standing on,
/// and it came back as the carpet torn into bands with the dark of this room
/// showing between them. Nothing under the hall's floor needs drawing at all,
/// so the drawn slab stops where the hall's starts.
fn ground(middle: Vec3, back: f32) -> Aabb {
    Aabb::from_center_size(
        vec3(middle.x, -THICK * 0.5, middle.z - back * 0.5),
        vec3(SPAN, THICK, DEEP + back),
    )
}

/// Everything solid, and what each of it is: the floor, the lid, the four
/// walls, and the columns.
///
/// Four walls and not three. The hall's near end wall is in the way of the
/// south side only where the hall is, and the hall is five units of
/// forty-four.
pub fn built(reaches: f32) -> Vec<(Aabb, Made)> {
    let middle = at(reaches);
    let half = half();
    let side = WALL + CABINET.x;
    let (west, east) = (middle.x - half.x, middle.x + half.x);
    let (south, north) = (middle.z - half.y, middle.z + half.y);

    let mut out = vec![
        // the floor, drawn only as far as the hall's own floor reaches and
        // walked a good deal further. See `ground`.
        (ground(middle, 0.0), Made::Ground),
        // and a lid, because a space with no top is a space you are standing
        // outside of, which is the fault the hall was fixed for in spec 0001
        (
            Aabb::from_center_size(
                vec3(middle.x, HIGH + THICK * 0.5, middle.z),
                vec3(SPAN, THICK, DEEP),
            ),
            Made::Ground,
        ),
        // the walls go to the top of the lid and not to its underside. Stopped
        // at the underside they leave a slot the lid's own thickness, the
        // whole length of the room, and through it you see the lit side of the
        // building: it showed as a bright sliver along the ceiling.
        (
            Aabb::from_center_size(
                vec3(west - THICK * 0.5, (HIGH + THICK) * 0.5, middle.z),
                vec3(THICK, HIGH + THICK, DEEP + THICK * 2.0),
            ),
            Made::Wall,
        ),
        (
            Aabb::from_center_size(
                vec3(east + THICK * 0.5, (HIGH + THICK) * 0.5, middle.z),
                vec3(THICK, HIGH + THICK, DEEP + THICK * 2.0),
            ),
            Made::Wall,
        ),
        (
            Aabb::from_center_size(
                vec3(middle.x, (HIGH + THICK) * 0.5, north + THICK * 0.5),
                vec3(SPAN, HIGH + THICK, THICK),
            ),
            Made::Wall,
        ),
        // the south side, which is the hall's near end wall for the width of
        // the hall and this room's own wall for the rest of it
        (
            Aabb::from_center_size(
                vec3((east + side) * 0.5, (HIGH + THICK) * 0.5, south),
                vec3(east - side, HIGH + THICK, THICK),
            ),
            Made::Wall,
        ),
    ];

    out.extend(columns(reaches).into_iter().map(|at_| {
        (
            Aabb::from_center_size(at_ + Vec3::Y * HIGH * 0.5, vec3(COLUMN, HIGH, COLUMN)),
            Made::Column,
        )
    }));

    out
}

/// The same, as what you cannot walk through, with the floor lapped back
/// under the hall's. See `ground`.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let middle = at(reaches);

    built(reaches)
        .into_iter()
        .map(|(box_, made)| match made {
            // the floor, which is the Ground that is not the lid
            Made::Ground if box_.max.y <= 0.0 => ground(middle, LAPS),
            _ => box_,
        })
        .collect()
}

/// How far apart the columns stand, how thick they are, and how far from the
/// way in the nearest of them may be.
///
/// A regular grid. Everywhere else in this building the rule is that nothing
/// lines up, because nothing in a furnished room ever does. This is not a
/// furnished room, and the regularity is what makes it feel like nobody is
/// meant to be here.
pub const STRIDE: f32 = 7.5;
pub const COLUMN: f32 = 0.72;
pub const CLEAR: f32 = 2.2;

/// Where the columns stand.
///
/// They are what gives the size away. An empty floor going off into the dark
/// is a corridor with the lights off, and the only way to read a distance is
/// to see the same thing at several of them.
pub fn columns(reaches: f32) -> Vec<Vec3> {
    let middle = at(reaches);
    let half = half();
    let (from, to) = way();
    let door = vec2((from + to) * 0.5, reaches);

    let across = (SPAN / STRIDE) as i32;
    let along = (DEEP / STRIDE) as i32;
    let mut out = Vec::new();

    for i in 0..across {
        for j in 0..along {
            let at_ = vec2(
                middle.x - half.x + STRIDE * (i as f32 + 0.5),
                middle.z - half.y + STRIDE * (j as f32 + 0.5),
            );

            // nothing in the way in. One column there and the whole thing is a
            // wall you walked into twice.
            if at_.distance(door) < CLEAR {
                continue;
            }

            out.push(vec3(at_.x, 0.0, at_.y));
        }
    }

    out
}

/// How far apart the lamps are, how far each throws and how hard.
///
/// Far apart and faint, and cool where every other light in this building is
/// warm. Most of this is not lit at all, which is the point: there is no fog
/// in this engine and none is wanted, because a point light falls off and the
/// far end goes black on its own.
pub const LAMPS: f32 = 15.0;
pub const LAMP_RANGE: f32 = 9.5;
pub const LAMP_LIT: f32 = 0.42;
pub const LAMP_COLOUR: Vec3 = vec3(0.72, 0.78, 0.92);

/// Where the lamps are, on the ceiling.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    let middle = at(reaches);
    let half = half();
    let across = (SPAN / LAMPS).max(1.0) as i32;
    let along = (DEEP / LAMPS).max(1.0) as i32;
    let mut out = Vec::new();

    for i in 0..across {
        for j in 0..along {
            out.push(vec3(
                middle.x - half.x + LAMPS * (i as f32 + 0.5),
                HIGH - 0.08,
                middle.z - half.y + LAMPS * (j as f32 + 0.5),
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The room these tests walk, taken from the room rather than written down.
    fn room() -> crate::room::Room {
        crate::room::Room::of(
            (0..12)
                .map(|n| {
                    crate::cabinet::Cabinet::found(
                        &format!("game{}", n),
                        std::path::Path::new("/nowhere"),
                    )
                })
                .collect(),
        )
    }

    /// Walks a body your size from one place to another and says where it got
    /// to. The whole building's boxes, because the wall it goes through is the
    /// hall's.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid = room().solid();
        let step = 1.0 / 60.0;
        let (mut at_, mut falling, mut owed) = (from, 0.0f32, 0.0f32);

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at_.x, 0.0, to.z - at_.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (next, fell) =
                crate::walk::walk(at_, wish, falling, crate::RADIUS, step, &solid, owed <= 0.0);
            owed = (owed - crate::walk::CLIMBS * step).max(0.0) + (next.y - at_.y).max(0.0);
            at_ = next;
            falling = fell;
        }

        at_
    }

    /// Spec 0011: you can walk through the wall where the gap is.
    ///
    /// The whole room is this one walk. Everything else in here is scenery for
    /// it.
    #[test]
    fn you_can_walk_through_where_the_gap_is() {
        let reaches = room().reaches;
        let (from, to) = way();
        let on = (from + to) * 0.5;
        let got = walked(
            vec3(on, 0.0, reaches - 2.0),
            vec3(on, 0.0, reaches + 4.0),
            3.0,
        );

        assert!(got.z > reaches + THICK, "it did not get through: {:?}", got);
    }

    /// Spec 0011: and not where it is not, which is most of the wall.
    ///
    /// Swept across the whole of it, because a wall with one hole in it is a
    /// wall everywhere else and that is what makes the hole worth finding.
    #[test]
    fn the_rest_of_the_wall_is_a_wall() {
        let reaches = room().reaches;
        let side = WALL + CABINET.x;
        let (from, to) = way();

        for step in 0..=20 {
            let on = -side + (side * 2.0) * step as f32 / 20.0;

            // the gap itself, and a body's width either side of it, where you
            // get through by sliding rather than by walking straight
            if on > from - crate::RADIUS - 0.1 && on < to + crate::RADIUS + 0.1 {
                continue;
            }

            let got = walked(
                vec3(on, 0.0, reaches - 2.0),
                vec3(on, 0.0, reaches + 4.0),
                3.0,
            );

            assert!(
                got.z < reaches - THICK,
                "the wall let a body through at x {:.2}: {:?}",
                on,
                got
            );
        }
    }

    /// Spec 0011: the way in is not in front of you when you wake.
    ///
    /// You wake on the middle of the hall facing away from this wall. A gap on
    /// the middle is found by everybody on their first turn round, and the
    /// whole of the room is that it is not.
    #[test]
    fn the_way_in_is_not_in_front_of_you() {
        let (from, to) = way();

        assert!(
            from > crate::RADIUS || to < -crate::RADIUS,
            "the way in is straight behind where you wake: {:.2} to {:.2}",
            from,
            to
        );
        // and not so far over that it is in a corner nobody walks into
        let side = WALL + CABINET.x;
        assert!(
            from > -side + 0.5 && to < side - 0.5,
            "the way in is in a corner: {:.2} to {:.2}",
            from,
            to
        );
    }

    /// Spec 0011: the space is closed but for that gap.
    ///
    /// Walked at every wall from inside, because a room this big with one way
    /// out has a lot of wall to get wrong.
    #[test]
    fn nothing_else_gets_out() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let inside = vec3(middle.x, 0.0, middle.z);

        for (what, to) in [
            (
                "the west wall",
                vec3(middle.x - half.x - 6.0, 0.0, middle.z),
            ),
            (
                "the east wall",
                vec3(middle.x + half.x + 6.0, 0.0, middle.z),
            ),
            (
                "the north wall",
                vec3(middle.x, 0.0, middle.z + half.y + 6.0),
            ),
            (
                "the south wall",
                vec3(middle.x, 0.0, middle.z - half.y - 6.0),
            ),
        ] {
            let got = walked(inside, to, 20.0);
            let off = vec2(got.x - middle.x, got.z - middle.z).abs();

            assert!(
                off.x < half.x && off.y < half.y,
                "it got out through {} to {:?}",
                what,
                got
            );
        }
    }

    /// Spec 0011: it keeps out of every other room.
    #[test]
    fn it_keeps_out_of_every_other_room() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let garden = crate::garden::at(reaches);
        let gap = crate::garden::half();

        let apart = |a: Vec2, ah: Vec2, b: Vec2, bh: Vec2| {
            (a.x - b.x).abs() >= ah.x + bh.x - 1e-4 || (a.y - b.y).abs() >= ah.y + bh.y - 1e-4
        };

        assert!(
            apart(
                vec2(middle.x, middle.z),
                half,
                vec2(garden.x, garden.z),
                gap
            ),
            "it is over the garden"
        );

        // the hall and the nook are the other side of the wall it is behind
        assert!(
            middle.z - half.y >= reaches - 1e-4,
            "it comes through into the hall: its near edge is at {:.2} and the \
             hall's near end wall is at {:.2}",
            middle.z - half.y,
            reaches
        );
    }

    /// Spec 0011: its floor laps under the hall's rather than meeting it.
    ///
    /// The other way round from how this was first written, and the walk is
    /// what settled it. Two floors at one height that meet edge to edge put a
    /// vertical face at the surface you are walking on, and a body resting on a
    /// surface counts as touching it: the walk through the gap stopped dead
    /// four centimetres short of the seam. Lapped a good way under, both end
    /// faces are deep inside the other slab and the join is nothing at all.
    ///
    /// The lap has to be a good way, not a hair. A face just under the join is
    /// still a face you cross at the walking surface, only harder to find.
    #[test]
    fn the_floors_lap_rather_than_meet() {
        let room = room();
        let hall = room.floors()[0];
        // the floor as walked, found under the foot of somebody who has just
        // stepped through the gap, rather than by its place in the list
        let (from, to) = way();
        let foot = vec3((from + to) * 0.5, -0.05, hall.max.z - crate::RADIUS);
        let ours = solid(room.reaches)
            .into_iter()
            .find(|box_| box_.contains_point(foot))
            .expect("a floor under the way in");

        assert!(
            (ours.max.y - hall.max.y).abs() < 1e-4,
            "the floors are at different heights"
        );

        let over = hall.max.z - ours.min.z;
        assert!(
            over > crate::RADIUS * 2.0,
            "they lap by {:.2} and a body is {:.2} across",
            over,
            crate::RADIUS * 2.0
        );
        // and the lap is under the hall rather than the hall under it: this
        // floor's near face has to be inside the hall's slab in x as well, or
        // it is a face in the open after all
        assert!(
            ours.min.x <= hall.min.x + 1e-4,
            "this floor starts east of the hall's, so its near face is in the open"
        );
    }

    /// The lap is walked and not drawn.
    ///
    /// Drawn, it put this room's floor and the hall's in one plane over five
    /// metres by three, and the patch they fight over is the floor you wake
    /// standing on: the carpet came back torn into bands with the dark of
    /// this room showing between them.
    #[test]
    fn nothing_drawn_lies_in_the_halls_floor() {
        let room = room();

        for hall in room.floors() {
            for (box_, made) in built(room.reaches) {
                let level = (box_.max.y - hall.max.y).abs() < 1e-4;
                let over = box_.min.x < hall.max.x - 1e-4
                    && box_.max.x > hall.min.x + 1e-4
                    && box_.min.z < hall.max.z - 1e-4
                    && box_.max.z > hall.min.z + 1e-4;

                assert!(
                    !(level && over),
                    "a {made:?} drawn at {box_:?} tops out in the hall's floor plane and \
                     laps over it by {:.2} by {:.2}",
                    box_.max.x.min(hall.max.x) - box_.min.x.max(hall.min.x),
                    box_.max.z.min(hall.max.z) - box_.min.z.max(hall.min.z),
                );
            }
        }
    }

    /// Spec 0011: no column stands in the way in.
    ///
    /// One there and the whole thing is a wall you walked into twice.
    #[test]
    fn nothing_stands_in_the_way_in() {
        let reaches = room().reaches;
        let (from, to) = way();
        let door = vec2((from + to) * 0.5, reaches);

        for at_ in columns(reaches) {
            let off = vec2(at_.x, at_.z).distance(door);

            assert!(
                off > COLUMN * 0.5 + crate::RADIUS,
                "a column stands in the way in, {:.2} from it",
                off
            );
        }
    }

    /// Spec 0011: you can get back out.
    ///
    /// The way in is the way out and there is no other. A space you can get
    /// into and not out of is not a secret, it is a hole.
    #[test]
    fn you_can_walk_back_out() {
        let reaches = room().reaches;
        let (from, to) = way();
        let on = (from + to) * 0.5;
        let got = walked(
            vec3(on, 0.0, reaches + 3.0),
            vec3(on, 0.0, reaches - 4.0),
            4.0,
        );

        assert!(got.z < reaches - THICK, "it could not get out: {:?}", got);
    }
}
