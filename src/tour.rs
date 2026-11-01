//! Planning a walk and following it. Spec 0049.
//!
//! **Because an open loop long enough to be a clip drifts.** The filmed
//! route is a string of held keys with nothing anywhere asking where the
//! body got to. Driven with the real keys on 2026-11-01 and the game
//! logging where it was, it agreed with spec 0042's checker for 24 seconds
//! and then stopped dead against a cabinet in the hall and sat there for
//! seventy more, over one about-turn coming back seven degrees short of its
//! 180.
//!
//! Seven degrees is not a bug to fix. So: a path worked out over the
//! building's own floors, and a follower that aims at the next place every
//! frame, which is the half that stops the drift. A turn that sweeps short
//! is simply aimed again on the next frame.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How far apart the lattice's places are.
///
/// Half a metre, which is a little over a body's width and small enough
/// that a gap a body fits through has a place in it. Finer costs a walk
/// test per neighbour and this already does thousands.
pub const APART: f32 = 0.5;

/// How close counts as arrived, both for the planner's own walk tests and
/// for the follower.
pub const NEAR: f32 = APART * 0.45;

/// A place the lattice knows: where a pair of feet can be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spot {
    pub at: Vec3,
    /// Which room's floor it is on, for saying where a path goes.
    pub ground: &'static str,
    /// And which way is down there, which is nought everywhere but the
    /// imaginarium. Spec 0047.
    pub frame: u64,
}

/// The places and the ways between them.
///
/// **A way exists when the walker can walk it.** Not when the boxes say so.
/// A graph that reads geometry is a second account of what `walk` does, and
/// the day the two disagree a planned path walks into a step it cannot
/// climb. Asking `walk` also means the step height, the climb budget and
/// the sliding are all taken into account without being restated here.
pub struct Ways {
    spots: Vec<Spot>,
    next: Vec<Vec<usize>>,
}

/// Lay places over the grounds that lie inside `within`, and join the ones
/// the walker can get between.
///
/// Bounded on purpose: a lattice over the whole building is tens of
/// thousands of walk tests, and a journey across the hall should not pay
/// for the field.
pub fn ways(grounds: &[(&'static str, Aabb)], solid: &[Aabb], within: Aabb) -> Ways {
    let mut spots = Vec::new();
    // **The imaginarium's other two floors, which `grounds` cannot give.**
    // That list is what you can stand on the world's way up, and in that
    // room that is one face out of six. Spec 0049.
    let mut all: Vec<(&'static str, Aabb)> = grounds.to_vec();

    all.extend(
        crate::aloft::standing_on_each()
            .into_iter()
            .filter(|(n, _)| *n != 0)
            .map(|(_, slab)| ("the imaginarium", slab)),
    );

    for (name, ground) in all.iter() {
        if ground.min.x > within.max.x
            || ground.max.x < within.min.x
            || ground.min.z > within.max.z
            || ground.max.z < within.min.z
            || ground.max.y > within.max.y
            || ground.max.y < within.min.y
        {
            continue;
        }

        // **Laid out in the frame whose floor it is**, so the imaginarium's
        // three get a lattice apiece and everywhere else gets the world's.
        let frame = crate::aloft::frame_of_floor(ground);
        let turned = crate::aloft::box_out_of_frame(frame, ground);
        let put = |local: Vec3| {
            if frame == 0 {
                local
            } else {
                crate::aloft::in_frame(frame, local)
            }
        };
        let flat = if frame == 0 { *ground } else { turned };
        // **Clipped to the region in the frame's own terms.** Without this
        // a ground that merely overlaps the region gets a lattice from end
        // to end, and the first build of this laid a hundred and four
        // thousand places for a nine metre walk across the hall.
        let bounds = if frame == 0 {
            within
        } else {
            crate::aloft::box_out_of_frame(frame, &within)
        };

        // **Not inset by a body.** The first build held the lattice a
        // radius back from each ground's edge, which is right about a wall
        // and wrong about a doorway: two grounds that abut then have 0.9
        // metres of nothing between their nearest places, which is further
        // than a neighbour, and the hall and the garden came out as two
        // islands with no way between them. A place a body cannot stand in
        // simply joins nothing, and the walk test is what says so.
        let low = flat.min.max(bounds.min);
        let high = flat.max.min(bounds.max);
        let (across, along) = (
            ((high.x - low.x) / APART).floor() as i32,
            ((high.z - low.z) / APART).floor() as i32,
        );

        for i in 0..=across.max(0) {
            for j in 0..=along.max(0) {
                let local = vec3(
                    low.x + i as f32 * APART,
                    flat.max.y,
                    low.z + j as f32 * APART,
                );

                spots.push(Spot {
                    at: put(local),
                    ground: name,
                    frame,
                });
            }
        }
    }

    // **Only the boxes anywhere near**, because every walk test below reads
    // the whole list and the building has hundreds.
    let near: Vec<Aabb> = solid
        .iter()
        .filter(|box_| {
            box_.min.x <= within.max.x + 2.0
                && box_.max.x >= within.min.x - 2.0
                && box_.min.z <= within.max.z + 2.0
                && box_.max.z >= within.min.z - 2.0
        })
        .copied()
        .collect();

    let mut next = vec![Vec::new(); spots.len()];
    // **Bucketed, because this is the expensive part.** Every pair that is
    // close enough costs a walk, and asking every pair whether it is close
    // enough costs the square of the lattice. The buckets are the lattice's
    // own spacing, so a place's neighbours are in the nine round it.
    let mut buckets: std::collections::HashMap<(i32, i32, i32), Vec<usize>> =
        std::collections::HashMap::new();
    let cell = |at: Vec3| {
        (
            (at.x / APART).floor() as i32,
            (at.y / APART).floor() as i32,
            (at.z / APART).floor() as i32,
        )
    };

    for (n, spot) in spots.iter().enumerate() {
        buckets.entry(cell(spot.at)).or_default().push(n);
    }

    for one in 0..spots.len() {
        let (i, j, k) = cell(spots[one].at);

        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    let Some(near_by) = buckets.get(&(i + di, j + dj, k + dk)) else {
                        continue;
                    };

                    for &two in near_by.iter() {
                        if one == two || spots[one].frame != spots[two].frame {
                            continue;
                        }

                        // neighbours only, which at this spacing is the
                        // eight round you and anything a step up or down
                        if spots[one].at.distance(spots[two].at) > APART * 1.5 {
                            continue;
                        }

                        if walked(spots[one].frame, spots[one].at, spots[two].at, &near) {
                            next[one].push(two);
                        }
                    }
                }
            }
        }
    }

    let mut ways = Ways { spots, next };

    ways.join_the_portals(&near);
    ways
}

/// The imaginarium's two kinds of crossing, as edges. Specs 0047 and 0048.
///
/// **A portal is an edge you are a passenger on.** It leaves you in a
/// different frame, it costs its own fixed time, and nothing can steer
/// during it. A navmesh has no way to say "and now down is somewhere
/// else", which is why these are put in by hand rather than falling out of
/// the lattice.
///
/// Each one is two places: the mouth, which is a real spot in the frame
/// you are leaving and is where walking into it sets the crossing off, and
/// the far side, which is wherever the crossing puts you down. The mouth
/// joins its own frame's lattice the ordinary way, by being walked to, so
/// a path reaches it like anywhere else.
impl Ways {
    fn join_the_portals(&mut self, solid: &[Aabb]) {
        use crate::aloft;

        let mut mouths: Vec<(Vec3, u64, Vec3, u64)> = Vec::new();

        for k in 0..aloft::FRAMES {
            // the hatch: cut in `floor_of(k)`'s floor, and the two frames
            // that share it swap through it either way
            let hole = aloft::doorway(k, 1);
            let under = aloft::floor_of(k);

            for (leaving, joining) in [(under, k), (k, under)] {
                mouths.push((
                    hole.center(),
                    leaving,
                    aloft::arrive_through(k, joining),
                    joining,
                ));
            }

            // and the door at the top of frame k's own flight
            let out = aloft::round_the_other_way(k);

            mouths.push((
                aloft::doorway(k, 0).center(),
                k,
                aloft::foot_of_the_flight(out),
                out,
            ));
        }

        for (mouth, leaving, lands, joining) in mouths {
            let far = match self.nearest_in(lands, joining) {
                Some(far) => far,
                None => continue,
            };

            let here = self.spots.len();

            self.spots.push(Spot {
                at: mouth,
                ground: "a way through",
                frame: leaving,
            });
            self.next.push(vec![far]);

            // and joined to its own frame's lattice by being walked to,
            // like anywhere else
            for n in 0..here {
                if self.spots[n].frame != leaving {
                    continue;
                }

                if self.spots[n].at.distance(mouth) > APART * 3.0 {
                    continue;
                }

                if walked(leaving, self.spots[n].at, mouth, solid) {
                    self.next[n].push(here);
                }
            }
        }
    }

    fn nearest_in(&self, to: Vec3, frame: u64) -> Option<usize> {
        self.spots
            .iter()
            .enumerate()
            .filter(|(_, spot)| spot.frame == frame && spot.at.distance(to) < APART * 4.0)
            .min_by(|one, two| one.1.at.distance(to).total_cmp(&two.1.at.distance(to)))
            .map(|(n, _)| n)
    }
}

/// Whether the walker gets from one place to another, asked of the walker.
fn walked(frame: u64, from: Vec3, to: Vec3, solid: &[Aabb]) -> bool {
    let step = 1.0 / 60.0;
    // the ground it asks for, and half as long again for a step up or a
    // graze on a jamb
    let rounds = ((from.distance(to) / crate::SPEED / step) * 1.5).ceil() as u32 + 8;
    let (mut at, mut falling) = (from, 0.0);

    let over = crate::aloft::frame(frame) * Vec3::Y;

    for _ in 0..rounds {
        let left = to - at;
        let up = left.dot(over);
        let flat = left - over * up;

        if flat.length() < NEAR && up.abs() < crate::walk::STEP {
            return true;
        }

        let wish = flat.normalize_or_zero() * crate::SPEED;
        let (went, fell) =
            crate::aloft::walked_in_frame(frame, at, wish, falling, step, solid, true);

        at = went;
        falling = fell;
    }

    false
}

impl Ways {
    pub fn places(&self) -> usize {
        self.spots.len()
    }

    /// The place nearest a point that the walker can actually get to from
    /// it.
    ///
    /// **Nearest by distance is not enough.** Rooms in this building sit
    /// over and beside each other, and the closest place to the hall's own
    /// doorway turned out to be one on the floor of the space behind the
    /// wall, which joins three hundred places and none of them the garden.
    /// So the candidates are tried in order of distance and the first one
    /// that can be walked to wins.
    pub fn nearest(&self, to: Vec3, frame: u64, solid: &[Aabb]) -> Option<usize> {
        let mut near: Vec<usize> = (0..self.spots.len())
            .filter(|n| {
                self.spots[*n].frame == frame && self.spots[*n].at.distance(to) < APART * 4.0
            })
            .collect();

        near.sort_by(|one, two| {
            self.spots[*one]
                .at
                .distance(to)
                .total_cmp(&self.spots[*two].at.distance(to))
        });

        near.into_iter()
            .find(|n| walked(self.spots[*n].frame, to, self.spots[*n].at, solid))
    }

    /// The shortest way from one place to another, as the places along it.
    ///
    /// A* with the straight line as the guess, which never overstates what
    /// is left because no way between two places is shorter than the line
    /// between them.
    /// **Both ends say which frame they are in**, because a point on
    /// frame one's floor is a few centimetres from a point on frame
    /// nought's where the two meet, and nearest by distance picked the
    /// wrong one of those every time.
    pub fn plan(&self, from: (Vec3, u64), to: (Vec3, u64), solid: &[Aabb]) -> Option<Vec<Spot>> {
        let (start, end) = (
            self.nearest(from.0, from.1, solid)?,
            self.nearest(to.0, to.1, solid)?,
        );
        let mut cost = vec![f32::MAX; self.spots.len()];
        let mut came = vec![usize::MAX; self.spots.len()];
        let mut open: Vec<usize> = vec![start];

        cost[start] = 0.0;

        while !open.is_empty() {
            // the open place that looks cheapest all the way through
            let (which, _) = open
                .iter()
                .enumerate()
                .min_by(|one, two| {
                    let guess = |n: usize| cost[n] + self.spots[n].at.distance(self.spots[end].at);

                    guess(*one.1).total_cmp(&guess(*two.1))
                })
                .expect("the open list is not empty");
            let here = open.swap_remove(which);

            if here == end {
                let mut way = vec![self.spots[end]];
                let mut n = end;

                while came[n] != usize::MAX {
                    n = came[n];
                    way.push(self.spots[n]);
                }

                way.reverse();

                return Some(way);
            }

            for &there in self.next[here].iter() {
                let so_far = cost[here] + self.spots[here].at.distance(self.spots[there].at);

                if so_far < cost[there] {
                    cost[there] = so_far;
                    came[there] = here;
                    open.push(there);
                }
            }
        }

        None
    }
}

/// Which way to walk and which way to look, this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Steer {
    pub wish: Vec3,
    pub yaw: f32,
}

/// Somebody walking a planned way.
///
/// **This is the half that earns the spec.** Nothing in here is timed. A
/// turn that sweeps short, a graze on a jamb, a tread that costs a frame of
/// the climbing budget: each is only a slightly different place to aim from
/// on the next frame.
pub struct Follower {
    way: Vec<Spot>,
    at: usize,
}

impl Follower {
    pub fn new(way: Vec<Spot>) -> Self {
        Self { way, at: 0 }
    }

    /// Where it is trying to get to, if it is still going anywhere.
    pub fn toward(&self) -> Option<Spot> {
        self.way.get(self.at).copied()
    }

    pub fn arrived(&self) -> bool {
        self.at >= self.way.len()
    }

    /// Aim, from wherever you actually are.
    ///
    /// The places behind you are dropped rather than walked back to: a
    /// follower that overshoots a corner should carry on to the next place
    /// rather than turn round for one it has already passed.
    pub fn steer(&mut self, from: Vec3) -> Option<Steer> {
        while let Some(spot) = self.way.get(self.at) {
            let flat = vec3(spot.at.x - from.x, 0.0, spot.at.z - from.z);

            if flat.length() < NEAR {
                self.at += 1;

                continue;
            }

            let wish = flat.normalize_or_zero();

            return Some(Steer {
                wish: wish * crate::SPEED,
                // yaw nought faces -z and grows toward +x
                yaw: wish.x.atan2(-wish.z),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A box round two places with room to get between them.
    fn between(one: Vec3, two: Vec3) -> Aabb {
        let spare = vec3(6.0, 3.0, 6.0);

        Aabb::new(one.min(two) - spare, one.max(two) + spare)
    }

    /// Spec 0049: every step of a planned path is one the walker can walk.
    ///
    /// **Which is not the same as saying the path avoids the boxes.** The
    /// lattice decides what joins to what by asking `walk`, so this is
    /// asking the same question a second time, from the answer rather than
    /// from the graph: take the path that came out and walk every leg of
    /// it.
    #[test]
    fn every_step_of_a_path_can_be_walked() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let from = room.doorway();
        let to = from + vec3(-9.0, 0.0, 0.0);
        let ways = ways(&room.grounds(), &solid, between(from, to));

        let way = ways
            .plan((from, 0), (to, 0), &solid)
            .expect("no way across the hall");

        assert!(way.len() > 4, "the path is {} places long", way.len());

        for pair in way.windows(2) {
            assert!(
                walked(pair[0].frame, pair[0].at, pair[1].at, &solid),
                "the walker cannot get from {:?} to {:?}",
                pair[0].at,
                pair[1].at
            );
        }
    }

    /// **Its control:** somewhere with no way to it gets no path rather
    /// than a path through a wall.
    #[test]
    fn a_shut_room_has_no_way_in() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let from = room.doorway();
        // inside the hall's own end wall, where nobody can stand
        let to = vec3(from.x, from.y, from.z - 40.0);
        let ways = ways(&room.grounds(), &solid, between(from, to));

        assert!(
            ways.plan((from, 0), (to, 0), &solid).is_none(),
            "a path was found to somewhere there is no floor"
        );
    }

    /// Spec 0049: a follower arrives from a place and a heading the plan
    /// did not expect.
    ///
    /// **The whole point of the spec.** The route that is filmed today is
    /// fired open loop, and half a metre of error at the start is half a
    /// metre of error at the end plus whatever it picks up on the way.
    #[test]
    fn a_follower_arrives_from_the_wrong_place() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let from = room.doorway();
        let to = from + vec3(-9.0, 0.0, 0.0);
        let ways = ways(&room.grounds(), &solid, between(from, to));
        let way = ways
            .plan((from, 0), (to, 0), &solid)
            .expect("no way across the hall");
        let mut follower = Follower::new(way);

        // pushed half a metre sideways from where the plan starts
        let mut at = from + vec3(0.0, 0.0, 0.5);
        let mut falling = 0.0;
        let step = 1.0 / 60.0;

        for _ in 0..1200 {
            let Some(steer) = follower.steer(at) else {
                break;
            };
            let (went, fell) =
                crate::walk::walk(at, steer.wish, falling, crate::RADIUS, step, &solid, true);

            at = went;
            falling = fell;
        }

        assert!(
            follower.arrived(),
            "it is still trying to get to {:?}, from {:?}",
            follower.toward(),
            at
        );
        assert!(
            at.distance(to) < 1.0,
            "it came to rest {:.2} metres from where it was going",
            at.distance(to)
        );
    }

    /// **Its control:** the same plan fired open loop does not.
    ///
    /// **Fired the way the filmed route is**, which is the comparison that
    /// matters: each leg of the plan held for exactly as long as its own
    /// length asks, aimed once from where the plan thought the body would
    /// be rather than from where it is. That is what `refresh-examples`
    /// does today, and it is why half a metre at the start is still there
    /// at the end along with everything collected on the way.
    #[test]
    fn the_open_loop_does_not() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let from = room.doorway();
        let to = from + vec3(-9.0, 0.0, 0.0);
        let ways = ways(&room.grounds(), &solid, between(from, to));
        let way = ways
            .plan((from, 0), (to, 0), &solid)
            .expect("no way across the hall");

        let step = 1.0 / 60.0;
        let mut at = from + vec3(0.0, 0.0, 0.5);
        let mut falling = 0.0;

        for pair in way.windows(2) {
            // aimed off the plan, not off the body
            let leg = pair[1].at - pair[0].at;
            let wish = vec3(leg.x, 0.0, leg.z).normalize_or_zero() * crate::SPEED;

            for _ in 0..((leg.length() / crate::SPEED / step).round() as u32).max(1) {
                let (went, fell) =
                    crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

                at = went;
                falling = fell;
            }
        }

        let open = at.distance(to);

        // and the follower over the same plan, from the same wrong place
        let mut follower = Follower::new(way);
        let mut at = from + vec3(0.0, 0.0, 0.5);
        let mut falling = 0.0;

        for _ in 0..1200 {
            let Some(steer) = follower.steer(at) else {
                break;
            };
            let (went, fell) =
                crate::walk::walk(at, steer.wish, falling, crate::RADIUS, step, &solid, true);

            at = went;
            falling = fell;
        }

        let followed = at.distance(to);

        assert!(
            followed < open * 0.5,
            "the follower missed by {:.2} and the open loop by {:.2}, which is not a difference worth a spec",
            followed,
            open
        );
    }

    /// Spec 0049: a way through the imaginarium's hatch comes out in the
    /// frame on the other side.
    #[test]
    fn a_portal_edge_changes_which_way_is_down() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let middle = crate::aloft::middle();
        let whole = Aabb::new(
            middle - Vec3::splat(crate::aloft::HALF + 1.0),
            middle + Vec3::splat(crate::aloft::HALF + 1.0),
        );
        let ways = ways(&room.grounds(), &solid, whole);

        let from = crate::aloft::in_frame(0, vec3(0.0, -crate::aloft::HALF, 0.0));
        let to = crate::aloft::arrive_through(1, 1);
        let way = ways
            .plan((from, 0), (to, 1), &solid)
            .expect("no way through the hatch");

        assert_eq!(
            way.first().map(|spot| spot.frame),
            Some(0),
            "it does not start in frame nought"
        );
        assert_eq!(
            way.last().map(|spot| spot.frame),
            Some(1),
            "it does not end in frame one, which is the far side of that hatch"
        );
        assert!(
            way.iter().any(|spot| spot.ground == "a way through"),
            "it got there without going through a portal"
        );
    }
}
