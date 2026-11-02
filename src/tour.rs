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

/// How close in height two places have to be before they are the same
/// place.
///
/// **Not `APART`.** Laying places on the tops of solids means laying one
/// on every tread, and the dog-leg's risers are 0.1875. Rounded to the
/// lattice's own half metre, the first tread of a flight lands in the
/// same cell as the floor it rises from and is thrown away as a
/// duplicate, which takes the one place that bridges the floor to the
/// stair. The whole flight was then an island: seventeen treads joined to
/// each other and to nothing else, and no way up from the nook.
pub const SAME_HEIGHT: f32 = 0.06;

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

    // **Grown to hold every meeting of the rooms in play, because that is
    // where the doorways are.** The nook opens through one gap south of the
    // last cabinet, at z -8.3 to -5.975, and a journey to the top of its
    // stair bounded the region at z -5.8. That cut the doorway off by 0.175
    // metres, so the only edges from the hall to the nook ran through
    // places buried in the dividing wall, and the nook came out 0 of 504
    // reachable. The stair looked like the fault for three rounds.
    //
    // Laying an admitted ground whole was the first fix and is not
    // available: the field behind the wall is a hundred and sixty metres
    // square and the space beyond the door runs to thirty three thousand,
    // so whole rooms came out at a hundred and seven thousand places.
    // Growing the region itself cascades, because a bigger region admits
    // more rooms, which grow it again: two hundred and twelve thousand.
    //
    // Where two rooms meet is neither. It is the strip they share, it is
    // the only place a way between them can be, and it is small.
    // **Asked in the ground's own frame, because thin upward is a claim
    // about a floor and not about the world.** The imaginarium's other two
    // floors are walls in world terms: three tenths thick in x or z and
    // twelve metres tall in y. Asked in world terms, frame one's floor has
    // a `max.y` of eighteen, so a journey across that floor at y twelve
    // threw the floor out and found no way across a room it was standing
    // in.
    let admits = |ground: &Aabb, region: &Aabb| {
        let frame = crate::aloft::frame_of_floor(ground);
        let (flat, there) = if frame == 0 {
            (*ground, *region)
        } else {
            (
                crate::aloft::box_out_of_frame(frame, ground),
                crate::aloft::box_out_of_frame(frame, region),
            )
        };

        !(flat.min.x > there.max.x
            || flat.max.x < there.min.x
            || flat.min.z > there.max.z
            || flat.max.z < there.min.z
            || flat.max.y > there.max.y
            || flat.max.y < there.min.y)
    };

    let within = {
        let in_play: Vec<Aabb> = all
            .iter()
            .filter(|(_, ground)| admits(ground, &within))
            .map(|(_, ground)| *ground)
            .collect();
        let mut grown = within;

        for (n, one) in in_play.iter().enumerate() {
            for two in in_play[n + 1..].iter() {
                // abutting counts as meeting: a shared wall's two floors
                // stop at the same plane and share no volume at all
                if one.min.x > two.max.x + APART
                    || two.min.x > one.max.x + APART
                    || one.min.z > two.max.z + APART
                    || two.min.z > one.max.z + APART
                {
                    continue;
                }

                let low = one.min.max(two.min) - vec3(APART, 0.0, APART);
                let high = one.max.min(two.max) + vec3(APART, 0.0, APART);

                grown.min.x = grown.min.x.min(low.x);
                grown.min.z = grown.min.z.min(low.z);
                grown.max.x = grown.max.x.max(high.x);
                grown.max.z = grown.max.z.max(high.z);
            }
        }

        grown
    };

    for (name, ground) in all.iter() {
        if !admits(ground, &within) {
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

    // **And the top of anything else you could stand on.** `grounds` is
    // spec 0043's list of rooms' floors, and a stair's treads are not a
    // room: planned off that list alone there was no way from the hall's
    // door to the dog-leg's own landing, because nothing in the lattice
    // was on a tread. A walkable place is the top of any solid, and the
    // walk test still decides what joins to what, so a wall's top simply
    // joins nothing.
    //
    // Skipped where a place is already laid, or every floor gets a second
    // lattice over the first.
    let cell_of = |at: Vec3| {
        (
            (at.x / APART).round() as i32,
            (at.y / SAME_HEIGHT).round() as i32,
            (at.z / APART).round() as i32,
        )
    };
    let mut laid: std::collections::HashSet<(i32, i32, i32)> =
        spots.iter().map(|spot| cell_of(spot.at)).collect();
    let mut more = Vec::new();

    for box_ in near.iter() {
        if box_.max.y < within.min.y || box_.max.y > within.max.y {
            continue;
        }

        let low = box_.min.max(within.min);
        let high = box_.max.min(within.max);
        let (across, along) = (
            ((high.x - low.x) / APART).floor() as i32,
            ((high.z - low.z) / APART).floor() as i32,
        );

        // **A box narrower than the lattice step puts its one place in
        // the middle.** From the corner is right for a floor, where the
        // next place along is half a metre away, and wrong for a stair
        // tread: the cellar's treads are 0.35 across, so the one place on
        // each sat on its west edge with a body 0.9 wide hanging over the
        // tread below, and `stood` threw two treads in every three away.
        // The way down came out 0 of 399 cellar places.
        let put = |lo: f32, hi: f32, n: i32| {
            if hi - lo < APART {
                (lo + hi) * 0.5
            } else {
                lo + n as f32 * APART
            }
        };

        for i in 0..=across.max(0) {
            for j in 0..=along.max(0) {
                let at = vec3(put(low.x, high.x, i), box_.max.y, put(low.z, high.z, j));
                if !laid.insert(cell_of(at)) {
                    continue;
                }

                more.push(Spot {
                    at,
                    ground: "a step",
                    frame: 0,
                });
            }
        }
    }

    spots.extend(more);

    // **And a place is only a place if a body fits in it.** This was an
    // inset of a radius from each ground's edge to begin with, which is
    // right about a wall and wrong about a doorway: two grounds that abut
    // ended up with 0.9 metres of nothing between them and the hall and
    // the garden were two islands. Taking the inset out fixed that and
    // broke stairs, because a tread's own edge is where its side wall is,
    // so every place on the dog-leg sat half inside the flight and the
    // walker lifted out of it and fell. The arcade's own stair test says
    // the same thing from the other side: it walks bodies up this flight
    // at `west + RADIUS` and no closer.
    //
    // Neither inset nor no inset is the answer. The question is whether a
    // body fits, so that is what is asked, of every place.
    spots.retain(|spot| stood(spot.frame, spot.at, &near));

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

    /// Everywhere you can get to from a place, for saying why a plan
    /// failed.
    ///
    /// **A planner that only says no is a planner you cannot debug.** The
    /// first time this said there was no way up the dog-leg, the question
    /// was whether the stair had no places on it or places that joined
    /// nothing, and nothing in the answer told them apart.
    pub fn reached_from(&self, start: usize) -> Vec<usize> {
        let mut seen = vec![false; self.spots.len()];
        let mut queue = vec![start];

        seen[start] = true;

        let mut n = 0;

        while n < queue.len() {
            let here = queue[n];
            n += 1;

            for &there in self.next[here].iter() {
                if !seen[there] {
                    seen[there] = true;
                    queue.push(there);
                }
            }
        }

        queue
    }

    /// And which place is where, for the same reason.
    pub fn spot(&self, n: usize) -> Spot {
        self.spots[n]
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

/// Whether somebody can stand here, asked of the walker.
///
/// **A sphere that does not overlap anything is the wrong question.** On
/// any stair a body standing on a tread overlaps the next riser, because
/// the riser is 0.1875 tall and 0.28 away while the body is 0.45 across,
/// and the walker deals with that by stepping. Asked geometrically, every
/// tread in the building is "does not fit" and no flight has a place on
/// it. Asked of the walker, standing still on a tread stays put.
fn stood(frame: u64, at: Vec3, solid: &[Aabb]) -> bool {
    let step = 1.0 / 60.0;
    let (mut went, mut falling) = (at, 0.0);

    for _ in 0..8 {
        let (next, fell) =
            crate::aloft::walked_in_frame(frame, went, Vec3::ZERO, falling, step, solid, true);

        went = next;
        falling = fell;
    }

    went.distance(at) < NEAR
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

                return Some(thinned(way, solid));
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

/// One step of a filmed tour. Spec 0049.
///
/// **Places and what to do at them, which is what a route is now.** How
/// long a leg takes stops being written down, because it stops being
/// something anybody sets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Step {
    /// Walk to here, in this frame.
    At(Vec3, u64),
    /// Stand still for this long.
    Stand(f32),
    /// Turn to face this point, however long that takes.
    Look(Vec3),
    /// And work whatever the sight is on, which is the bookcase and the
    /// baths' leaf.
    Work,
}

/// Read a tour off a line of text, the way `refresh-examples` writes one.
///
/// `at:x,y,z` or `at:x,y,z:frame`, `stand:seconds`, `look:x,y,z`, `work`.
pub fn read(tour: &str) -> Result<Vec<Step>, String> {
    let point = |bit: &str| -> Result<Vec3, String> {
        let n: Vec<f32> = bit
            .split(',')
            .map(|one| {
                one.trim()
                    .parse::<f32>()
                    .map_err(|_| format!("{} is not a number", one))
            })
            .collect::<Result<_, _>>()?;

        match n[..] {
            [x, y, z] => Ok(vec3(x, y, z)),
            _ => Err(format!("{} is not three numbers", bit)),
        }
    };

    tour.split_whitespace()
        .map(|word| {
            let mut bits = word.split(':');

            match (bits.next(), bits.next(), bits.next()) {
                (Some("at"), Some(where_), frame) => Ok(Step::At(
                    point(where_)?,
                    frame.unwrap_or("0").parse().unwrap_or(0),
                )),
                (Some("stand"), Some(how_long), _) => how_long
                    .parse()
                    .map(Step::Stand)
                    .map_err(|_| format!("{} is not a number of seconds", how_long)),
                (Some("look"), Some(where_), _) => Ok(Step::Look(point(where_)?)),
                (Some("work"), None, _) => Ok(Step::Work),
                _ => Err(format!("{} is not a step of a tour", word)),
            }
        })
        .collect()
}

/// How far back a place stands from the thing it is there to look at.
///
/// A body is 0.9 across and the lattice steps half a metre, so anything
/// much under a metre is a place that may turn out not to exist. This is
/// what "in front of" means everywhere in the route below.
const STANDS_BACK: f32 = 1.4;

/// The clip's route, as places in the building rather than as keys.
///
/// **Out of the building's own accessors, every one of them.** A route
/// written as coordinates would be a second account of where everything
/// is, and it would be the longest such list in the project: a score of
/// places that each already have an owner elsewhere. `cellar::stair_foot`
/// and `spa::doorway` exist because two lists of one thing drift, and a
/// route is where that drift would show up as a camera walking into a
/// wall six months after somebody moved the wall.
///
/// What this does not say is how to get from any one place to the next.
/// That is the planner's, and it is the whole of spec 0049. It is also why
/// the imaginarium's hatch and its top door are not named here: specs 0047
/// and 0048 made both of them ways you take rather than things you press,
/// so a place in frame one is the whole of the instruction and the portal
/// edges are what know where the way through is.
///
/// The order is the building's and not a preference. The garden's way in
/// and the hole into the solarium are both a couple of metres from the
/// front door, so they come first. The hall is walked once, end to end.
/// The nook is at the far end of it and holds both of the remaining ways
/// on, so it is walked twice with the climb between the visits.
pub fn clip(room: &crate::room::Room) -> Vec<Step> {
    let reaches = room.reaches;
    let mut route = vec![Step::At(room.doorway(), 0)];

    // **The garden, and the koi in it.** Stood off the pond's east bank,
    // which is the side the way in is on, with the garden's depth behind
    // it rather than its wall.
    let (pond, pond_size, _) = crate::garden::pond(reaches);

    route.push(Step::At(
        vec3(pond.x + pond_size.x * 0.5 + STANDS_BACK, 0.0, pond.z),
        0,
    ));
    route.push(Step::Look(pond));
    route.push(Step::Stand(2.0));

    // **The solarium, in along the planet line.** Every planet rests on
    // the line between its sun and its way in, which is `orrery::stands`
    // with no swing on it, so walking in is walking past them in order.
    // Jake watched a take on 2026-11-01 that turned along the right wall
    // instead and said so: the planets and the moons and the sun are the
    // room, and the wall is not.
    let middle = crate::behind::at(reaches);
    let sun = vec3(middle.x, crate::orrery::PLANE, middle.z);
    let (west, east) = crate::behind::way();
    let along =
        (vec3((west + east) * 0.5, crate::orrery::PLANE, reaches) - sun).normalize_or_zero();
    let on_the_floor = |at: Vec3| vec3(at.x, 0.0, at.z);

    // the two with something to see on them, outermost first, which is
    // the order you meet them walking in
    for named in ["Saturn", "Jupiter"] {
        let planet = crate::orrery::PLANETS
            .iter()
            .find(|planet| planet.body.name == named)
            .expect("the solar system has lost a planet");
        let at = sun + along * planet.far();

        route.push(Step::At(on_the_floor(at + along * STANDS_BACK), 0));
        route.push(Step::Look(at));
        route.push(Step::Stand(1.4));
    }

    route.push(Step::At(
        on_the_floor(sun + along * (crate::orrery::sun_drawn() * 0.5 + STANDS_BACK)),
        0,
    ));
    route.push(Step::Look(sun));
    route.push(Step::Stand(2.4));

    // **The imaginarium, named as a place in each frame.** One circuit of
    // it is both apertures, and which aperture that takes is the
    // planner's to work out.
    let floor_of = |n: u64| crate::aloft::in_frame(n, vec3(0.0, -crate::aloft::HALF, 0.0));

    route.push(Step::At(floor_of(0), 0));
    route.push(Step::Look(crate::aloft::middle()));
    route.push(Step::Stand(2.4));
    route.push(Step::At(floor_of(1), 1));
    route.push(Step::Look(crate::aloft::middle()));
    route.push(Step::Stand(2.0));
    route.push(Step::At(crate::aloft::foot_of_the_flight(1), 1));

    // **The bookcase, which is worked and not walked through.** Spec
    // 0007: pull the book and four hundred of them swing out of the way.
    // `cellar::opening` is the one account of which of them it is.
    let nook = room.floors()[1];
    let down = vec3(nook.min.x, 0.0, crate::cellar::opening(reaches));

    route.push(Step::At(
        down + vec3(STANDS_BACK + crate::RADIUS, 0.0, 0.0),
        0,
    ));
    route.push(Step::Look(vec3(down.x, crate::EYE, down.z)));
    route.push(Step::Work);
    route.push(Step::Stand(1.6));

    // **The baths' leaf, which is the other one.** Spec 0008. Spec 0042's
    // checker could not replay either of these until a route needed them,
    // and said so in as many words.
    let baths = crate::spa::doorway(reaches);

    // Stood short of it on the cellar's side, which is the side you
    // arrive from. The first version of this stood on the baths' side,
    // so the leg had to walk through the leaf to reach the place it was
    // going in order to open the leaf.
    route.push(Step::At(baths - vec3(0.0, 0.0, STANDS_BACK), 0));
    route.push(Step::Look(baths + vec3(0.0, crate::EYE, 0.0)));
    route.push(Step::Work);
    route.push(Step::Stand(1.4));

    // and it finishes at the pool, looking the length of it
    let (pool, pool_size, _) = crate::spa::pool(reaches);

    route.push(Step::At(
        vec3(pool.x, pool.y, pool.z - pool_size.y * 0.5 - STANDS_BACK),
        0,
    ));
    route.push(Step::Look(pool));
    route.push(Step::Stand(2.4));

    route
}

/// Drop the places a walk does not need.
///
/// **A path of half metre steps is a path of half metre steps.** Each one
/// is a thing to aim at, and aiming at something half a metre away while
/// walking is how the first build of the follower came to orbit its own
/// waypoints. A place can go whenever the walker gets from the one before
/// it to the one after without it, which is asked of `walk` like
/// everything else here.
fn thinned(way: Vec<Spot>, solid: &[Aabb]) -> Vec<Spot> {
    let mut out: Vec<Spot> = Vec::new();

    for spot in way.into_iter() {
        out.push(spot);

        while out.len() >= 3 {
            let last = out.len() - 1;

            if out[last - 2].frame != out[last].frame
                || !walked(out[last].frame, out[last - 2].at, out[last].at, solid)
            {
                break;
            }

            out.remove(last - 1);
        }
    }

    out
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
    /// `frame` is the frame the walker is standing in, and it matters.
    ///
    /// **Flat is a claim about a floor, not about the world.** In the
    /// imaginarium's other two frames the floor's plane is not x and z,
    /// so a heading worked out from world x and z throws away the part of
    /// the journey that is world y and keeps a part that is straight into
    /// the floor. `Step::Look` already turns its target out of the frame
    /// before taking an angle off it; this did not, and the game hands
    /// this yaw to a camera whose yaw is the frame's own.
    pub fn steer(&mut self, from: Vec3, frame: u64) -> Option<Steer> {
        let out = |at: Vec3| {
            if frame == 0 {
                at
            } else {
                crate::aloft::out_of_frame(frame, at)
            }
        };
        let from = out(from);

        while let Some(spot) = self.way.get(self.at) {
            let there = out(spot.at);
            let flat = vec3(there.x - from.x, 0.0, there.z - from.z);

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

        // **Not a count.** The first version of this asked for more than
        // four places and then thinning, which is a good thing, took the
        // path down to two and failed it. What is wanted is that the path
        // spans the journey, which is a thing about its ends.
        assert!(
            way[0].at.distance(from) < APART * 4.0,
            "the path starts {:.2} from where it was asked to",
            way[0].at.distance(from)
        );
        assert!(
            way[way.len() - 1].at.distance(to) < APART * 4.0,
            "the path ends {:.2} from where it was going",
            way[way.len() - 1].at.distance(to)
        );

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
            let Some(steer) = follower.steer(at, 0) else {
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
            let Some(steer) = follower.steer(at, 0) else {
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

    /// Spec 0049: a journey's region may not clip the doorway it needs.
    ///
    /// **The nook opens at one end and the journey is at the other.** Its
    /// only way in is the gap south of the last cabinet, which `room`
    /// puts at `-reaches + NOOK_DOOR`, and a walk from the front door to
    /// the top of the nook's stair has a bounding box that stops short of
    /// it by about two tenths of a metre. With the doorway outside the
    /// lattice the hall and the nook joined only through places buried in
    /// the dividing wall, which join nothing walkable, so the nook came
    /// out 0 of 504 reachable and the stair looked like the fault.
    ///
    /// Asked of the nook's own doorway rather than of a route, because a
    /// route that happens to work is not the claim. The claim is that the
    /// region a journey lays its lattice over holds the way in.
    #[test]
    fn the_region_holds_the_way_into_the_nook() {
        let room = crate::room::Room::of(crate::games());
        let solid = room.solid();
        let from = room.doorway();
        // the top of the nook's stair, which is what sent this wrong
        let to = vec3(-7.85, crate::aloft::UP_LANDING, 2.2);

        let ways = ways(&room.grounds(), &solid, between(from, to));

        // the gap itself: the stretch of the nook's open side south of the
        // last cabinet, at the plane the two rooms share
        let door = -room.reaches + crate::room::NOOK_DOOR;
        let mut through = 0;

        for n in 0..ways.places() {
            let at = ways.spot(n).at;

            if at.z < door && at.x > -crate::room::WALL - crate::room::CABINET.x - 1.0 {
                through += 1;
            }
        }

        assert!(
            through > 0,
            "nothing is laid in the way into the nook, which is z below {:.2}",
            door
        );

        assert!(
            ways.plan((from, 0), (to, 0), &solid).is_some(),
            "there is no way from the door to the top of the nook's stair"
        );
    }

    /// Spec 0049: the whole filmed tour reaches its last place.
    ///
    /// **Leg by leg, and it says which legs failed rather than that one
    /// did.** A route is a score of places, and the useful answer when it
    /// breaks is which pair of them has no way between it. Four legs
    /// failed the first time this ran and three of them were one fault in
    /// `admits`, which a single pass or fail would have hidden.
    ///
    /// **And it works the doors on the way, because the planner does not.**
    /// Spec 0049 puts opening things out of scope on purpose: the bookcase
    /// and the baths' leaf are pointed at and pressed, and until they are,
    /// there is genuinely no way through them. So this walks the route's
    /// own state rather than the building's resting state, which is the
    /// thing `Room::open` was made for.
    ///
    /// The region is built per leg because that is how the clip runs it. A
    /// lattice over the whole building is the hundred thousand places
    /// `ways` is commented about.
    #[test]
    fn the_clip_s_tour_gets_all_the_way_round() {
        let mut room = crate::room::Room::of(crate::games());
        let steps = clip(&room);
        let reaches = room.reaches;

        // the two things on this route that swing, read off the same
        // accessors the route reads, so there is one account of each
        let case = vec3(room.floors()[1].min.x, 0.0, crate::cellar::opening(reaches));
        let leaf = crate::spa::doorway(reaches);

        let mut solid = room.solid();
        let mut sight = Vec3::ZERO;
        let mut last: Option<(Vec3, u64)> = None;
        let mut legs = 0;
        let mut walking = 0.0;
        let mut standing = 0.0;
        let mut broken = Vec::new();

        for step in steps.iter() {
            match step {
                Step::Look(at) => sight = *at,
                Step::Work => {
                    if sight.distance(case) < sight.distance(leaf) {
                        room.open = true;
                        room.swing = 1.0;
                    } else {
                        room.baths_open = true;
                        room.baths_swing = 1.0;
                    }

                    solid = room.solid();
                }
                Step::Stand(how_long) => standing += *how_long,
                Step::At(at, frame) => {
                    if let Some(from) = last {
                        legs += 1;

                        let ways = ways(&room.grounds(), &solid, between(from.0, *at));

                        if let Some(way) = ways.plan(from, (*at, *frame), &solid) {
                            // how far the walking is, which is what the
                            // clip's length in `refresh-examples` is made of
                            let mut was = from.0;

                            for spot in way.iter() {
                                walking += was.distance(spot.at);
                                was = spot.at;
                            }
                        } else {
                            broken.push(format!("{:?} to {:?}", from, (*at, *frame)));

                            // **And where it stopped, not just that it
                            // did.** Which rooms a leg got to is the
                            // question; a height is what sent this up a
                            // staircase three times for a fault two rooms
                            // earlier.
                            if let (Some(one), Some(two)) = (
                                ways.nearest(from.0, from.1, &solid),
                                ways.nearest(*at, *frame, &solid),
                            ) {
                                let got = ways.reached_from(one);
                                let mut all: std::collections::BTreeMap<&str, (usize, usize)> =
                                    std::collections::BTreeMap::new();

                                for n in 0..ways.places() {
                                    all.entry(ways.spot(n).ground).or_default().1 += 1;
                                }

                                for n in got.iter() {
                                    all.entry(ways.spot(*n).ground).or_default().0 += 1;
                                }

                                println!(
                                    "  leg to {:?} on {}: {} of {} reachable",
                                    ways.spot(two).at,
                                    ways.spot(two).ground,
                                    got.len(),
                                    ways.places()
                                );

                                for (ground, (reached, laid)) in all {
                                    println!("    {:28} {:5} of {:5}", ground, reached, laid);
                                }
                            }
                        }
                    }

                    last = Some((*at, *frame));
                }
            }
        }

        println!(
            "{:.0} metres of walking at {} plus {:.0} of standing is about {:.0} seconds",
            walking,
            crate::SPEED,
            standing,
            walking / crate::SPEED + standing
        );

        assert!(legs > 8, "the route only has {} legs", legs);

        // **And it fits in the film.** `refresh-examples` runs the camera
        // for 145 seconds on this row. A route that outgrows that is not a
        // route that fails, it is a clip that stops in the middle of the
        // baths, and no frame of it looks wrong. The slack is the turning,
        // which this does not count, and the margin.
        let takes = walking / crate::SPEED + standing;

        assert!(
            takes < 120.0,
            "the route is {:.0} seconds of walking and standing against a camera \
             that runs for 145, which leaves nothing for the turns",
            takes
        );
        assert!(
            broken.is_empty(),
            "{} of {} legs have no way: {}",
            broken.len(),
            legs,
            broken.join("; ")
        );
    }

    /// Spec 0049: a heading is worked out in the frame the walker is in.
    ///
    /// **Because flat is a claim about a floor.** Frame one carries local y
    /// to world x, so world x is up in there and the floor's plane is world
    /// z and world y. A heading taken off world x and z keeps the first of
    /// those and throws the second away. The game hands this yaw to a
    /// camera whose yaw is the frame's own, and `Step::Look` beside it
    /// already turns its target out of the frame before taking an angle.
    ///
    /// Asked of a step along the frame's own z, which is world y, and
    /// which world x and z cannot see at all: the two places are two
    /// metres apart and a flat world reading of them is nought apart. So
    /// it was not a worse heading, it was no heading.
    #[test]
    fn a_heading_is_taken_in_the_frame_you_are_standing_in() {
        let half = crate::aloft::HALF;
        let here = crate::aloft::in_frame(1, vec3(0.0, -half, 0.0));
        let there = crate::aloft::in_frame(1, vec3(0.0, -half, 2.0));

        let mut follower = Follower::new(vec![Spot {
            at: there,
            ground: "the imaginarium",
            frame: 1,
        }]);

        let steer = follower
            .steer(here, 1)
            .expect("it will not steer to a place two metres away");

        // local +z is a half turn, by the convention `steer` writes down:
        // yaw nought faces -z and grows toward +x
        assert!(
            (steer.yaw.abs() - std::f32::consts::PI).abs() < 0.05,
            "it steers on {:.3} where the way across the frame is a half turn",
            steer.yaw
        );

        // **Its control.** Read in world x and z those two places are the
        // same place, so there is no heading in them to get right and this
        // test would pass on any reading that happened to be in the frame.
        let flat = vec3(there.x - here.x, 0.0, there.z - here.z);

        assert!(
            flat.length() < NEAR,
            "world x and z tell these two places apart by {:.3}, so this test \
             is not asking what it says it asks",
            flat.length()
        );
    }
}
