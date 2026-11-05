//! What is through the door at the top of the bridge. Spec 0056.
//!
//! **Nothing here exists until the bridge does**, which is until every room
//! has given up what it asks: spec 0054's rule, read from the same list, so
//! somebody who never finds the trail cannot tell any of this is here.
//!
//! **A hall and its steading and not a hall alone.** The myth gives Valhalla
//! as one great building and Jake asked for somewhere village-like, and a
//! Norse settlement is both at once: a longhouse with smaller buildings
//! round it. Spec 0055 carries the argument.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::trail::Trail;

/// How far the ground runs west from the landing's far edge, and how far
/// across.
///
/// **Thirty-two of the thirty-five metres there are.** The field's ground
/// stops at `field::GROUND_REACH` and the sky is sized to clear the ground,
/// so a place that runs past it is a place hanging out through the sky.
/// This does not need ground under it, being twenty-six metres up, but
/// staying inside that reach is what keeps it inside the weather.
pub const DEEP: f32 = 32.0;
pub const SPAN: f32 = 46.0;

/// How thick its ground is, which is only what you see of the edge.
pub const GROUND_DEEP: f32 = 1.4;

/// The hall: how long, how wide, how high to the eaves and how high to the
/// ridge.
///
/// **Twenty-four by nine**, which is a real longhouse and is also the only
/// size that makes the outbuildings read as outbuildings. Every room in
/// this building has been too small on its first build.
pub const HALL_LONG: f32 = 24.0;
pub const HALL_WIDE: f32 = 9.0;
pub const HALL_EAVES: f32 = 3.4;
pub const HALL_RIDGE: f32 = 6.6;

/// How wide the hall's door is and how high.
pub const DOOR_WIDE: f32 = 2.4;
pub const DOOR_HIGH: f32 = 2.8;

/// Where the middle of this place is: west of the landing by half its own
/// depth, on the line you walk in on.
pub fn at(reaches: f32) -> Vec3 {
    let top = crate::bridge::top(reaches);

    vec3(
        top.x - crate::bridge::LANDING_OUT - DEEP * 0.5,
        top.y,
        top.z,
    )
}

/// Its ground, as the one slab.
///
/// Flush with the landing, so stepping through is stepping across. Lapped
/// back under the landing by a stride, because two faces at the walking
/// surface stop a walk dead and `walk` reports nothing at all.
pub fn ground(reaches: f32) -> Aabb {
    let middle = at(reaches);
    let east = crate::bridge::top(reaches).x - crate::bridge::LANDING_OUT + 1.0;

    Aabb::new(
        vec3(
            middle.x - DEEP * 0.5,
            middle.y - GROUND_DEEP,
            middle.z - SPAN * 0.5,
        ),
        vec3(east, middle.y, middle.z + SPAN * 0.5),
    )
}

/// One building up here: where it stands, how big, and whether it is the
/// hall.
#[derive(Clone, Copy, Debug)]
pub struct Built {
    /// The middle of its footprint, on the ground.
    pub at: Vec3,
    /// How long it runs north and south, how wide east and west.
    pub long: f32,
    pub wide: f32,
    pub eaves: f32,
    pub ridge: f32,
    pub hall: bool,
}

impl Built {
    /// The box of its walls, which is what you walk round.
    pub fn walls(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.eaves * 0.5,
            vec3(self.wide, self.eaves, self.long),
        )
    }

    /// And the box its roof fills, taken as the whole of it, which is what
    /// one building must not share with another.
    ///
    /// Only the tests want it: what is drawn is the walls and the two
    /// pitches, and what holds you up is the walls.
    #[cfg(test)]
    pub fn footprint(&self) -> Aabb {
        Aabb::from_center_size(
            self.at + Vec3::Y * self.ridge * 0.5,
            vec3(self.wide, self.ridge, self.long),
        )
    }
}

/// The hall and its outbuildings.
///
/// **The hall faces the bridge**, so what you see on arriving is its door
/// and not its flank. The others stand off the line you walk in on, which
/// leaves that line open and gives the place a lane rather than a yard.
pub fn built(reaches: f32) -> Vec<Built> {
    let middle = at(reaches);
    let one =
        |off_x: f32, off_z: f32, long: f32, wide: f32, eaves: f32, ridge: f32, hall: bool| Built {
            at: vec3(middle.x + off_x, middle.y, middle.z + off_z),
            long,
            wide,
            eaves,
            ridge,
            hall,
        };

    vec![
        // the hall, laid across the way you come in so its gable end and
        // its door are what face you
        one(
            -4.0, 0.0, HALL_WIDE, HALL_LONG, HALL_EAVES, HALL_RIDGE, true,
        ),
        // and the steading, off the lane either side
        one(7.5, -13.0, 6.0, 8.0, 2.4, 4.0, false),
        one(6.0, 13.5, 5.0, 7.0, 2.2, 3.6, false),
        one(-10.0, -14.5, 7.0, 6.0, 2.3, 3.8, false),
    ]
}

/// Where the hall's door is, and how big.
///
/// Its own account, because the light that comes out of it and the walk
/// that goes in through it both ask, and two places saying where a door is
/// is the fault this building keeps making.
pub fn doorway(reaches: f32) -> (Vec3, f32, f32) {
    let hall = built(reaches)[0];

    (
        vec3(hall.at.x + hall.wide * 0.5, hall.at.y, hall.at.z),
        DOOR_WIDE,
        DOOR_HIGH,
    )
}

/// How deep an opening is set into its wall, and how far the jamb round it
/// stands proud.
///
/// **So that it is a hole and not a panel.** Set flush and lit, the hall's
/// door was a bright rectangle painted on a wall. Set back behind a dark
/// reveal, the same rectangle is a room you are not in.
pub const SET_BACK: f32 = 0.34;
pub const JAMB: f32 = 0.16;

/// How far a lit opening stands off the wall it is in, so the two are not
/// one surface, and how wide and high a window is.
pub const PROUD: f32 = 0.03;
pub const WINDOW_WIDE: f32 = 0.9;
pub const WINDOW_HIGH: f32 = 1.1;
pub const WINDOW_UP: f32 = 1.5;

/// One lit opening: where it is, how big, and which way it faces.
#[derive(Clone, Copy, Debug)]
pub struct Lit {
    pub at: Vec3,
    pub wide: f32,
    pub high: f32,
    pub looks: Vec3,
}

/// Every lit opening up here: the hall's door, and windows down the side of
/// everything.
///
/// **The light is in the openings and not on the walls.** The whole of spec
/// 0055's brief is that the source is out of frame: a wall that glows is a
/// lamp, and a window that glows is a room you are not in.
pub fn lit(reaches: f32) -> Vec<Lit> {
    let mut out = Vec::new();
    let (door, wide, high) = doorway(reaches);

    out.push(Lit {
        at: door - Vec3::X * SET_BACK + Vec3::Y * high * 0.5,
        wide,
        high,
        looks: Vec3::X,
    });

    for one in built(reaches) {
        // down the face that looks at the lane, which for everything here
        // is its east side
        let face = one.at.x + one.wide * 0.5 + PROUD;
        let count = ((one.long / 4.0).floor() as i32).max(1);

        for n in 0..count {
            let along = one.at.z + (n as f32 + 0.5) / count as f32 * one.long - one.long * 0.5;

            // not where the door is
            if one.hall && (along - door.z).abs() < wide {
                continue;
            }

            out.push(Lit {
                at: vec3(face - SET_BACK, one.at.y + WINDOW_UP, along),
                wide: WINDOW_WIDE,
                high: WINDOW_HIGH,
                looks: Vec3::X,
            });
        }
    }

    out
}

/// Where the people stand, and which way each faces.
///
/// **Nobody is fighting and nobody is waiting for a war.** Spec 0055: keep
/// the figures, drop the rest of what the myth puts in this hall.
pub fn people(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);
    let (door, _, _) = doorway(reaches);

    vec![
        // one by the door, looking out along the lane at whoever is coming
        (
            vec3(door.x + 1.8, door.y, door.z + 2.2),
            std::f32::consts::FRAC_PI_2,
        ),
        // and two further in, together, facing each other
        (vec3(middle.x + 6.0, middle.y, middle.z - 5.5), 2.6),
        (vec3(middle.x + 7.4, middle.y, middle.z - 6.4), -0.5),
    ]
}

/// What you cannot walk through up here.
pub fn solid(reaches: f32, trail: &Trail) -> Vec<Aabb> {
    if !crate::bridge::shows(trail) {
        return Vec::new();
    }

    let mut out = vec![ground(reaches)];
    out.extend(built(reaches).iter().map(Built::walls));

    out
}

/// Where the light that spills out of each building sits.
///
/// **One a building and not one a window.** The lamp is inside and the
/// openings are holes in the same room, so a light per hole is the same
/// room lit four times. Just outside the face the openings are in, so what
/// it throws lands on the lane rather than inside a wall.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    built(reaches)
        .into_iter()
        .map(|one| {
            // well out from the face and low, and further out than looks
            // right written down. Close in it shone back into the opening
            // it was supposed to be coming out of and burnt a white core
            // into the panel, and it lit the wall harder than the ground,
            // which washed a daubed wall to cream. Out here the wall is
            // five metres from it and the lane is one, and a light falls
            // off with the square of that.
            vec3(one.at.x + one.wide * 0.5 + 5.0, one.at.y + 1.0, one.at.z)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trail::Asked;

    const REACHES: f32 = 20.0;

    fn done() -> Trail {
        let mut trail = Trail::new();

        for one in Asked::every() {
            trail.gave(*one);
        }

        trail
    }

    /// Spec 0056: a hall in the sky over a field with no way up to it is
    /// worse than nothing, so it is there exactly when the bridge is.
    #[test]
    fn nothing_until_the_bridge_is_there() {
        assert!(solid(REACHES, &Trail::new()).is_empty());
        assert!(!solid(REACHES, &done()).is_empty());
    }

    /// Spec 0056: stepping through is stepping across and not up.
    #[test]
    fn the_ground_is_flush_with_the_landing() {
        let ours = ground(REACHES);
        let landing = crate::bridge::landing(REACHES, &done())[0];

        assert!(
            (ours.max.y - landing.max.y).abs() < 1e-4,
            "its ground is at {:.3} and the landing is at {:.3}",
            ours.max.y,
            landing.max.y
        );
        assert!(
            ours.max.x > landing.min.x + 1e-3,
            "it stops at {:.3} and the landing starts at {:.3}, which is \
             two faces in one plane at the walking surface",
            ours.max.x,
            landing.min.x
        );
    }

    /// Spec 0056: and you can actually get across.
    #[test]
    fn walked_from_the_landing_into_valhalla() {
        let mut solid = crate::bridge::solid(REACHES, &done());
        solid.extend(super::solid(REACHES, &done()));

        let landing = crate::bridge::landing(REACHES, &done())[0];
        let middle = at(REACHES);
        let start = vec3(landing.max.x - 1.5, landing.max.y + 0.4, middle.z);
        // in along the lane, which is east of the hall
        let want = vec3(middle.x + 9.0, middle.y, middle.z);
        let ended = crate::bridge::tests::walked(start, want, 14.0, &solid);

        assert!(
            ended.x < landing.min.x - 1.0,
            "it got to {:.2} and the landing ends at {:.2}, so it never \
             crossed",
            ended.x,
            landing.min.x
        );
        assert!(
            (ended.y - middle.y).abs() < 0.6,
            "it ended {:.2} up and the ground is at {:.2}",
            ended.y,
            middle.y
        );
    }

    /// Spec 0056: inside the ground the field draws, and so inside the sky.
    #[test]
    fn it_stays_inside_the_field() {
        let ours = ground(REACHES);
        let west = crate::field::starts(REACHES) - crate::field::GROUND_REACH;

        assert!(
            ours.min.x > west,
            "it runs out to {:.2} and the field's ground stops at {:.2}",
            ours.min.x,
            west
        );
    }

    /// Spec 0056: what you see on arriving is the door and not a flank.
    #[test]
    fn the_hall_faces_the_way_you_come_in() {
        let hall = built(REACHES)[0];
        let (door, _, _) = doorway(REACHES);

        assert!(hall.hall, "the first building is not the hall");
        assert!(
            door.x > hall.at.x,
            "the door at {:.2} is on the far side of a hall at {:.2}",
            door.x,
            hall.at.x
        );
        // and it is on the line you walk in on
        assert!(
            (door.z - at(REACHES).z).abs() < 1.0,
            "the door is {:.2} off the way in",
            (door.z - at(REACHES).z).abs()
        );
    }

    /// Spec 0056: nothing stands in the lane you arrive along.
    #[test]
    fn nothing_stands_in_the_way_in() {
        let middle = at(REACHES);
        let (door, _, _) = doorway(REACHES);
        let body = crate::RADIUS * 2.0;

        for one in built(REACHES) {
            if one.hall {
                continue;
            }

            let box_ = one.footprint();
            // the lane runs from the landing west to the hall's door, on
            // the middle line
            let across = box_.min.z - body < middle.z && box_.max.z + body > middle.z;
            let along = box_.max.x > door.x && box_.min.x < middle.x + DEEP * 0.5;

            assert!(
                !(across && along),
                "a building stands in the lane: {:?} to {:?}",
                box_.min,
                box_.max
            );
        }

        for (where_, _) in people(REACHES) {
            let off = (where_.z - middle.z).abs();

            assert!(
                off > crate::RADIUS || where_.x < door.x,
                "somebody stands in the doorway at {:?}",
                where_
            );
        }
    }

    /// Spec 0056: and no two of them are in the same place.
    #[test]
    fn no_two_buildings_share_a_space() {
        let all = built(REACHES);

        for (n, one) in all.iter().enumerate() {
            for other in all[n + 1..].iter() {
                let (a, b) = (one.footprint(), other.footprint());
                let apart = a.min.x > b.max.x
                    || b.min.x > a.max.x
                    || a.min.z > b.max.z
                    || b.min.z > a.max.z;

                assert!(apart, "two buildings overlap: {:?} and {:?}", a.min, b.min);
            }
        }
    }

    /// Spec 0056: the light is in the openings, and an opening is a hole
    /// in a wall rather than a panel on one.
    #[test]
    fn every_light_is_in_an_opening() {
        let all = built(REACHES);

        for one in lit(REACHES) {
            let found = all.iter().any(|built| {
                let walls = built.walls();

                // behind the face it is in, by the set-back and no more.
                // On the face it is a panel painted on a wall, and this
                // test said on the face until the day it stopped being
                // true.
                one.at.x < walls.max.x + 1e-3
                    && one.at.x > walls.max.x - SET_BACK - 0.1
                    && one.at.z > walls.min.z
                    && one.at.z < walls.max.z
                    && one.at.y + one.high * 0.5 < built.at.y + built.eaves
            });

            assert!(found, "a light hangs in the air at {:?}", one.at);
        }
    }

    /// Spec 0056: and everybody is standing on the ground, outdoors.
    #[test]
    fn everybody_stands_on_the_ground() {
        let floor = ground(REACHES);
        let all = built(REACHES);

        for (where_, _) in people(REACHES) {
            assert!(
                (where_.y - floor.max.y).abs() < 1e-3,
                "somebody stands at {:.3} and the ground is at {:.3}",
                where_.y,
                floor.max.y
            );
            assert!(
                where_.x > floor.min.x && where_.x < floor.max.x,
                "somebody stands off the edge at {:?}",
                where_
            );

            for one in all.iter() {
                let walls = one.walls();
                let inside = where_.x > walls.min.x - crate::RADIUS
                    && where_.x < walls.max.x + crate::RADIUS
                    && where_.z > walls.min.z - crate::RADIUS
                    && where_.z < walls.max.z + crate::RADIUS;

                assert!(!inside, "somebody stands in a wall at {:?}", where_);
            }
        }
    }
}
