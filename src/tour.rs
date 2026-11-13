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
        crate::parked::aloft::standing_on_each()
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
        let frame = crate::parked::aloft::frame_of_floor(ground);
        let (flat, there) = if frame == 0 {
            (*ground, *region)
        } else {
            (
                crate::parked::aloft::box_out_of_frame(frame, ground),
                crate::parked::aloft::box_out_of_frame(frame, region),
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
        let frame = crate::parked::aloft::frame_of_floor(ground);
        let turned = crate::parked::aloft::box_out_of_frame(frame, ground);
        let put = |local: Vec3| {
            if frame == 0 {
                local
            } else {
                crate::parked::aloft::in_frame(frame, local)
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
            crate::parked::aloft::box_out_of_frame(frame, &within)
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
        use crate::parked::aloft;

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
    // **And not inside a wall, which the drop cannot ask.** A sweep that
    // starts inside a box answers by direction: into it is blocked, out
    // of it or along it is free. So a body let go inside a wall is never
    // pushed out of one. It stands there, eight frames later it has not
    // moved, and the place reads as somewhere to stand. Nothing walking
    // can ever reach it, because the wall is in the way of every
    // approach, and a follower sent to one leans on that wall for the
    // rest of the film. Two sat 0.2 inside the garden's, four metres
    // from the door the clip starts at.
    //
    // **Taller than a step, which is what tells a wall from a stair.**
    // A body on a tread overlaps the riser in front of it every time,
    // and that riser is a thing to climb rather than a thing you are
    // inside. Asked of every box instead, the cellar's flight and the
    // imaginarium's both lost their treads and two legs of the clip lost
    // their way altogether.
    //
    // And `SLACK` of it, because nought is the wrong answer: this
    // building has gaps built to exactly a body's width, the nook's only
    // way in among them, so a place in one touches both jambs.
    let (here, boxes) = in_frame_terms(frame, at, solid);
    let middle = here + Vec3::Y * crate::RADIUS;
    let room_for = crate::RADIUS - SLACK;
    let walled = boxes.iter().any(|box_| {
        box_.max.y > here.y + crate::walk::STEP
            && box_.closest_point(middle).distance_squared(middle) < room_for * room_for
    });

    if walled {
        return false;
    }

    let step = 1.0 / 60.0;
    let (mut went, mut falling) = (at, 0.0);

    for _ in 0..8 {
        let (next, fell) = crate::parked::aloft::walked_in_frame(
            frame,
            went,
            Vec3::ZERO,
            falling,
            step,
            solid,
            true,
        );

        went = next;
        falling = fell;
    }

    went.distance(at) < NEAR
}

/// How far inside something a place may be and still be a place.
///
/// **Nought is the wrong answer, and it was tried.** A body is 0.9
/// across and this building has gaps built to exactly that: the nook's
/// only way in is 0.9 wide, so a place in it touches both jambs and any
/// overlap at all refuses it. Asked that way the nook had no way in and
/// three more legs went with it. Five centimetres tells that apart from
/// the places that had to go, which were a fifth of a metre in.
const SLACK: f32 = 0.05;

/// A place and the building, both turned into the frame's own terms,
/// where `walk` does its arithmetic. Spec 0047.
fn in_frame_terms(frame: u64, at: Vec3, solid: &[Aabb]) -> (Vec3, Vec<Aabb>) {
    if frame == 0 {
        return (at, solid.to_vec());
    }

    (
        crate::parked::aloft::out_of_frame(frame, at),
        solid
            .iter()
            .map(|box_| crate::parked::aloft::box_out_of_frame(frame, box_))
            .collect(),
    )
}

/// How far round the two ends of a leg the lattice is laid.
///
/// **One account, because three disagreed.** The film said eight by five
/// by eight, `--tour` said eight by four by eight, and the test that
/// walks the whole route said six by three by six. They are all the same
/// question: a region holds what a leg may plan through, and a leg that
/// has to step round something needs room either side of the straight
/// line. Three answers meant the test was planning a route the film
/// never walks, and the first leg it failed on was a plan nothing in the
/// film had ever made.
///
/// Taller than it is wide would be wrong the other way: the imaginarium
/// is twelve metres of height over the nook, so a leg into it carries
/// five rather than three.
pub fn spare() -> Vec3 {
    vec3(8.0, 5.0, 8.0)
}

/// The region a leg of a route is planned in: the two ends and `spare`.
pub fn region(from: Vec3, to: Vec3) -> Aabb {
    Aabb::new(from.min(to) - spare(), from.max(to) + spare())
}

/// Whether the walker gets from one place to another, asked of the walker.
///
/// **On the walker's own climbing budget, which is the whole of it.** A
/// step costs its own height out of a budget that fills at `walk::CLIMBS`
/// a second, and until it covers the next riser the walker simply walks
/// into it. Asked with an unlimited budget this said yes to a straight
/// line from the eleventh tread of the imaginarium's way up to the floor
/// at the top of it, because a body that may climb every frame ratchets
/// up a flight however little of its pace is pointed along it. `thinned`
/// then dropped the twenty treads between the two, the follower aimed at
/// the far one, and the camera walked into the shaft wall at an angle for
/// the last hundred seconds of the film with four risers to go.
///
/// It is the same arithmetic as the game's, in the frame's own up per
/// spec 0047, because the one thing this has to be is the walker.
fn walked(frame: u64, from: Vec3, to: Vec3, solid: &[Aabb]) -> bool {
    walk_between(frame, from, to, solid, f32::INFINITY)
}

/// Whether the walker gets there **without going round anything**.
///
/// **Which is the question thinning is really asking.** `walked` lets the
/// body lean on a wall and slide along it, and a body that arrives that
/// way did get there, so an edge half a metre long is right to count it.
/// A place dropped out of a plan is a different matter: the follower that
/// walks the thinned way is never standing exactly where the thinning
/// stood, so a run that only works by sliding works from one spot and not
/// from the one a metre off it.
///
/// It cost two takes. Thinning collapsed the garden to the hall into one
/// eight metre diagonal, which `walked` allowed by grazing the wall
/// between them; the camera started the leg a hand's width from where the
/// plan did, met the wall at a slightly different angle, and leaned on it
/// for the rest of the film.
fn walked_straight(frame: u64, from: Vec3, to: Vec3, solid: &[Aabb]) -> bool {
    walk_between(frame, from, to, solid, crate::RADIUS)
}

/// The walk both of those are, with how far it may stray from the straight
/// line between the two places before it does not count.
fn walk_between(frame: u64, from: Vec3, to: Vec3, solid: &[Aabb], stray: f32) -> bool {
    let step = 1.0 / 60.0;
    let over = crate::parked::aloft::frame(frame) * Vec3::Y;
    let up = (to - from).dot(over).max(0.0);
    let line = to - from;
    let along = line.length();
    let off = |at: Vec3| {
        if along < 1e-4 {
            0.0
        } else {
            let how_far = ((at - from).dot(line) / (along * along)).clamp(0.0, 1.0);

            at.distance(from + line * how_far)
        }
    };
    // the ground it asks for, and half as long again for a graze on a
    // jamb, plus what the climbing in it is rationed to take
    let rounds = (((from.distance(to) / crate::SPEED) * 1.5 + up / crate::walk::CLIMBS) / step)
        .ceil() as u32
        + 8;
    let (mut at, mut falling) = (from, 0.0);
    let mut climbs = 0.0;

    for _ in 0..rounds {
        let left = to - at;
        let up = left.dot(over);
        let flat = left - over * up;

        if flat.length() < NEAR && up.abs() < crate::walk::STEP {
            return true;
        }

        let wish = flat.normalize_or_zero() * crate::SPEED;
        let (went, fell) = crate::parked::aloft::walked_in_frame(
            frame,
            at,
            wish,
            falling,
            step,
            solid,
            climbs <= 0.0,
        );

        climbs = (climbs - crate::walk::CLIMBS * step).max(0.0) + (went - at).dot(over).max(0.0);
        at = went;
        falling = fell;

        if off(at) > stray {
            return false;
        }
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

/// How long a day lasts while the clip is being filmed, and what hour it
/// opens at.
///
/// **A film wants one evening, not three days.** The clip runs fifty-six
/// seconds and `garden::DAY` is twenty, so filmed at the working length the
/// sky would go round nearly three times: the garden would be a different
/// hour each time the route passes it, and the one outdoor shot would get
/// whichever sky its second happened to fall on. At twenty minutes the whole
/// film moves the clock by a twentieth of a day and every shot gets the hour
/// this names, give or take: fifty-eight seconds of this is a twentieth of a
/// day, which carries the moon seventeen degrees across the sky over the
/// whole film and a degree and a half over the five seconds of standing that
/// matter. `the_clip_looks_at_the_moon` asks the whole span rather than the
/// moment, because where in the film the shot falls is not a thing the
/// constants here know.
pub const FILM_DAY: f32 = 20.0 * 60.0;

/// And the hour, which is chosen by the one shot that can see a sky.
///
/// The route stands in the garden's way out and looks west at Bifrost, so
/// the sky in that frame is the western one. Late in the night the moon is
/// low in the west, which puts it in the same stretch of sky as the arc.
/// Anywhere else in the night it is overhead or behind you, and by day it is
/// not there at all.
///
/// **Chosen by looking, and it is not the hour of the shot.** The hour
/// decides where the moon sits against things that do not move, and the
/// frame that works has it in open sky between a trunk and the garden's
/// wall, with the arc to the left. That frame is hour 0.95. A take at 0.94
/// puts a trunk straight across the moon and one at 0.96 puts it against the
/// wall's edge, so there is about a hundredth of a day of room either side.
///
/// **This is that hour less the time the camera takes to get there**, which
/// is about fifteen seconds of a twenty minute day. The first take set this
/// to the hour of the shot and the film arrived thirteen thousandths of a
/// day late, with the moon already sliding behind the wall. A start hour is
/// not a shot hour and nothing but looking at the film says so.
pub const FILM_AT: f32 = 0.937;

/// How far back a place stands from the thing it is there to look at.
///
/// A body is 0.9 across and the lattice steps half a metre, so anything
/// much under a metre is a place that may turn out not to exist. This is
/// what "in front of" means everywhere in the route below.
const STANDS_BACK: f32 = 1.4;

/// How far back from the garden's west wall the clip stands to look at
/// Bifrost, on the garden's own side of it.
///
/// **Inside, because the field is not ground the planner knows.**
/// `tour::ways` lays its lattice over `room::grounds`, and the field is not
/// in that list: it follows the camera and is three hundred metres across,
/// so it is not a floor anybody can lay half metre places over. A stop two
/// metres the other side of the wall planned no way at all and the walk
/// never left the garden.
///
/// And it costs nothing to be inside. At two and a half metres back the
/// opening is wider than the view, so the jambs are in the corners of the
/// frame and the arc, the field and the trees either side of it are not
/// touched. Walking out to the foot is thirty-five metres further, over the
/// dark ground that cost the solarium its place on this route.
const BACK_FROM_THE_WAY: f32 = 2.5;

/// How far past the solarium's threshold the clip stands to look down it.
///
/// Its own number rather than `room::INSIDE`, which is how far inside the
/// front door you wake. The two agree to within a metre and are answering
/// different questions: that one is where a body fits past a door, this
/// one is where the jamb stops being in shot.
const INSIDE_THE_FIELD: f32 = 2.0;

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
pub fn clip(room: &crate::room::Room, done: bool) -> Vec<Step> {
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

    // **And through the garden's west wall at the bridge, when there is
    // one.** Spec 0054 puts Bifrost up only once the trail has been given,
    // so this is the one place on the route that is not simply there: a run
    // without `--done` stands here and looks at grass, which is why the
    // route is told rather than left to guess.
    //
    // It is the end of the trail and the one thing the building is for that
    // nothing published has ever shown.
    //
    // Lined up on the way out rather than on the bridge, which are the same
    // z and are not the same question: `field::way` is a hole in a wall and
    // `bridge::foot` is a place in a field, and the day one of them moves
    // this wants to be standing in the opening and not beside it.
    if done {
        let (from, to) = crate::field::way(reaches);

        let stand = vec3(
            crate::field::starts(reaches) + BACK_FROM_THE_WAY,
            0.0,
            (from + to) * 0.5,
        );

        route.push(Step::At(stand, 0));
        route.push(Step::Look(crate::bridge::top(reaches)));
        route.push(Step::Stand(3.0));

        // **And up at the moon, which is the only thing on this route that
        // is not in the building.** Spec 0065 gave it a face and nothing
        // published had ever shown it. It hangs off the eye rather than
        // standing anywhere, so where to look is a direction from where you
        // are standing and not a place: `moon::at` at the hour the film
        // opens, which `FILM_DAY` holds still for the length of the clip.
        //
        // After the bridge rather than before it. Both are west, the moon is
        // the higher of the two, and a turn that ends looking up is a better
        // last thing to see of the field than a turn that ends looking along
        // it.
        let eye = vec3(stand.x, stand.y + crate::EYE, stand.z);
        let moon = crate::moon::at(crate::garden::when(FILM_AT));

        route.push(Step::Look(eye + moon * crate::moon::off(eye)));
        route.push(Step::Stand(2.0));
    }

    // **The solarium, down the line rather than along it.** Every planet
    // rests on the line between its sun and its way in, which is
    // `orrery::stands` with no swing on it, so the whole room is in one
    // view from inside the door: the planets recede along that line in
    // order, the sun at the end of it.
    //
    // **Walked to, it was half the film.** The sun stands a hundred and
    // nine metres from this threshold, so going to it and coming back is
    // a hundred and sixty of the route's three hundred and forty-eight
    // metres, and almost all of it across a dark empty field. Worse, a
    // leg that long is a lattice ninety-four metres by a hundred and six:
    // sixty-seven thousand places, laid on the frame that asks for them,
    // which stopped the picture for twenty-five seconds of a hundred and
    // forty-five second take. Both of those are the same eighty metres.
    // Jake's call, on 2026-11-01, with the take in front of him.
    let middle = crate::behind::at(reaches);
    let sun = vec3(middle.x, crate::orrery::PLANE, middle.z);
    let (west, east) = crate::behind::way();

    route.push(Step::At(
        vec3((west + east) * 0.5, 0.0, reaches + INSIDE_THE_FIELD),
        0,
    ));
    route.push(Step::Look(sun));
    route.push(Step::Stand(3.0));

    // **The imaginarium is not on this route, and that is the second cut
    // of the same kind.** It was a place in each frame, one circuit of
    // it, both apertures, and the planner's to work out which.
    //
    // It cost twenty-two of the clip's sixty-six seconds and about eight
    // of those were worth having. The rest is the dog-leg, climbed and
    // then descended, seven seconds of a pale shaft with nothing on its
    // walls, and the walks across the room's own floors, which are
    // plaster. Measured by edge density over the take of 2026-11-01, the
    // room and its stair held nearly every blank frame in the film.
    // Jake's call, with the take in front of him, the same call and the
    // same reason as the solarium's above.
    //
    // What goes with it is the only part of the film that crossed a
    // frame, so nothing in the clip exercises spec 0047's apertures now.
    // `aloft`'s own tests and
    // `tour::tests::a_mouth_you_have_crossed_is_not_aimed_at_again` are
    // what hold that, and they were already what held it: the crossing
    // fault that wedged the camera for a whole take got through this
    // route without a word.

    // **The cube, before the way down, because the row is on the way in.**
    // Spec 0050. The nook's door is at the foot of the bench row and the
    // bookcase is across the room from it, so a stop at the cube costs the
    // walk up the row and back rather than a journey of its own.
    //
    // **Working it scrambles it**, because it stands solved, and a
    // scramble is twenty-five quarter turns at the toy's own pace. What
    // the clip shows is the first four or five of them: nine cubies
    // turning about an axis that belongs to neither them nor the whole,
    // which is the thing nothing else on that shelf does and the whole of
    // why it has a bench.
    //
    // Stood where the row's toys are worked from and aimed where they are
    // aimed at, each of which the bench is the one account of. Not at the
    // cube: `looked_at` says why, and it is the bookcase's fault below in
    // another room.
    let cube = room
        .benches
        .iter()
        .find(|one| one.name == crate::kociemba::NAME)
        .expect("the nook's row has the cube in it");

    route.push(Step::At(cube.worked_at(), 0));
    route.push(Step::Look(cube.looked_at()));
    route.push(Step::Work);
    route.push(Step::Stand(4.0));

    // **The bookcase, which is worked and not walked through.** Spec
    // 0007: pull the book and four hundred of them swing out of the way.
    // `cellar::opening` is the one account of which of them it is.
    let nook = room.floors()[1];
    let down = vec3(nook.min.x, 0.0, crate::cellar::opening(reaches));

    route.push(Step::At(
        down + vec3(STANDS_BACK + crate::RADIUS, 0.0, 0.0),
        0,
    ));
    // **Aimed at the middle of the leaf and not at the middle of the
    // opening.** Those are not the same point. `opening` is the mean of
    // the cases that swing, so it is the seam between the pair of them,
    // and `handle` is the one of the two you can point at: its box ends
    // exactly on that seam. A look along it was a ray laid on the box's
    // own edge, and the body does not stand where the route says it does
    // because a leg ends at the lattice place nearest the place it names.
    // Thirty centimetres east of here the ray cleared the north face by
    // four millimetres, the sight found nothing, `Work` worked nothing,
    // and the bookcase the next leg walks through stayed shut.
    //
    // Any interior point is proof against that, because the aim is taken
    // from wherever the body actually is, every frame. The middle is the
    // one that stays interior whatever the leaf's size.
    //
    // Read off a shut room, which is what `clip` is handed. The swing is
    // nought until the route's own `Work` opens it.
    let leaf = room.handle().expect("the cellar has a way down into it");

    route.push(Step::Look((leaf.min + leaf.max) * 0.5));
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

    // and aimed at the middle of that leaf, so the rule above is written
    // one way in both places. `doorway` is not a point on the leaf
    // either: it stands fifteen centimetres short of it and a metre
    // under its middle.
    //
    // **No control fails on this half of it.** Put back, the sight still
    // finds the leaf, because the ray is level and a leaf two metres tall
    // is still in front of it fifteen centimetres further on. It is the
    // bookcase's aim that the test holds.
    let leaf = crate::spa::leaf_box(reaches, 0.0);

    route.push(Step::Look((leaf.min + leaf.max) * 0.5));
    route.push(Step::Work);
    route.push(Step::Stand(1.4));

    // and it finishes at the pool, looking the length of it.
    //
    // **Stood at the floor's height and not the pool's.** `pool.y` is the
    // water, which is seven centimetres under the coping you stand on, so
    // a place taken from it is a place in the floor: the leg planned,
    // walked most of the way and stopped three and a half metres short.
    // The leaf's own height is the one account of what that floor is.
    let (pool, pool_size, _) = crate::spa::pool(reaches);

    route.push(Step::At(
        vec3(pool.x, baths.y, pool.z - pool_size.y * 0.5 - STANDS_BACK),
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
///
/// **Level ground only, because a stair is not ground you cross.** On the
/// flat a place is a corner to cut and the walk either gets there or does
/// not. On a flight it is a tread, the walker climbs one riser at a time
/// out of a rationed budget, and the far end of a thinned run is up and
/// away rather than along: aim at it and you lean on the wall beside the
/// stair for the whole climb, taking what little of your pace still
/// points along the treads. That is what emptied the last hundred seconds
/// of the film, four risers from the top of the imaginarium's way up.
fn thinned(way: Vec<Spot>, solid: &[Aabb]) -> Vec<Spot> {
    let mut out: Vec<Spot> = Vec::new();

    for spot in way.into_iter() {
        out.push(spot);

        while out.len() >= 3 {
            let last = out.len() - 1;
            let over = crate::parked::aloft::frame(out[last].frame) * Vec3::Y;
            let level = |one: Vec3, two: Vec3| (two - one).dot(over).abs() < SAME_HEIGHT;

            if out[last - 2].frame != out[last].frame
                || !level(out[last - 2].at, out[last - 1].at)
                || !level(out[last - 1].at, out[last].at)
                || !walked_straight(out[last].frame, out[last - 2].at, out[last].at, solid)
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
    /// The closest it has come to the place it is aiming at, and how many
    /// frames since that last improved.
    nearest: f32,
    stuck: u32,
    /// How many places it has given up on this way.
    gave_up: u32,
}

impl Follower {
    pub fn new(way: Vec<Spot>) -> Self {
        Self {
            way,
            at: 0,
            nearest: f32::INFINITY,
            stuck: 0,
            gave_up: 0,
        }
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
    /// `walking` is whether the body was walking the last time it asked,
    /// which is what tells a jam from a turn.
    pub fn steer(&mut self, from: Vec3, frame: u64, walking: bool) -> Option<Steer> {
        let out = |at: Vec3| {
            if frame == 0 {
                at
            } else {
                crate::parked::aloft::out_of_frame(frame, at)
            }
        };
        let from = out(from);

        while let Some(spot) = self.way.get(self.at) {
            // **A place in a frame you are not standing in is a place you
            // have been carried away from.** A portal's mouth is used by
            // walking into it, and walking into it puts you somewhere
            // else, so the mouth is never arrived at: the distance test
            // that retires an ordinary place can never fire on one. The
            // follower went on aiming at the aperture it had just come
            // through, walked back into it, and crossed again. Nine round
            // trips between two of the imaginarium's three frames, driven
            // on 2026-11-01, and then it wedged in the aperture and never
            // walked again: every step of the route past the imaginarium
            // was taken standing in one spot.
            //
            // Frames only go round by crossing, so the frame changing
            // under you is itself the news that the mouth is behind you.
            if spot.frame != frame {
                self.at += 1;
                self.nearest = f32::INFINITY;
                self.stuck = 0;

                continue;
            }

            let there = out(spot.at);
            let flat = vec3(there.x - from.x, 0.0, there.z - from.z);

            if flat.length() < NEAR {
                self.at += 1;
                self.nearest = f32::INFINITY;
                self.stuck = 0;

                continue;
            }

            // **And a place you cannot reach is a place you give up.**
            // The lattice lays its places on a grid and asks of each one
            // that a body dropped there stays put, which a body standing
            // inside a wall does: nothing pushes it out, so nothing says
            // the place is no good. The edge into it then turns on
            // whether the walk ends a hair inside `NEAR` or a hair
            // outside, and the follower coming in from somewhere else
            // ends the hair the other way and leans on the wall for the
            // rest of the film. One such place sat 0.2 from the garden's
            // wall, two metres from the door the clip starts at.
            //
            // A second of no progress, which is four metres of walking,
            // so nothing that is still getting anywhere gives up.
            // **And it only counts while the body is walking.** A body
            // finishing its turn is standing still on purpose and gets no
            // closer to anything while it does, so counting those frames
            // makes a turn look exactly like a wall. The rate is 2.4
            // radians a second, so a second of it is 137 degrees: every
            // leg that opens with an about-turn gave up the place it was
            // turning toward before it took a step, and set off for the
            // one after instead.
            if flat.length() < self.nearest - 1e-3 {
                self.nearest = flat.length();
                self.stuck = 0;
            } else if walking {
                self.stuck += 1;

                if self.stuck > 60 {
                    // **And the second one ends the leg.** Giving up a
                    // place and aiming at the next is right for one bad
                    // place in an otherwise good way. Keep doing it and
                    // the places left are all on the far side of
                    // whatever is in the way, so each is given up in
                    // turn and the body walks the length of the wall and
                    // back for as long as the film runs. A camera that
                    // moves on to the next thing is a cut; a camera
                    // pacing a wall is a fault anybody can see.
                    self.gave_up += 1;

                    if self.gave_up > 1 {
                        self.at = self.way.len();

                        return None;
                    }

                    self.at += 1;
                    self.nearest = f32::INFINITY;
                    self.stuck = 0;

                    continue;
                }
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

    /// Spec 0049. The clip stands where it stands and the orrery's bodies
    /// stand where they stand, and nothing checked the two against each
    /// other.
    ///
    /// **A guard, and not the fix for anything.** It was written chasing
    /// frame 851 of the published film, which draws nothing: the thought was
    /// that the camera steps through a planet, since inside a sphere you see
    /// backfaces and those are culled. Neptune is at thirty astronomical
    /// units, which sounded like the threshold. It is not. The nearest the
    /// clip ever comes to any body is 30.7 metres, off a planet drawn 0.27
    /// across, because at rest the outer ones sit outside the room
    /// altogether. That is the third idea about frame 851 to come apart, and
    /// it is still open.
    ///
    /// Kept anyway, because the orrery is at true scale and a waypoint moved
    /// a few metres is a waypoint that could land inside a world. It passes
    /// by a long way today and is worth nothing until the day it does not.
    ///
    /// **At rest only.** `stands` is asked for the unswung arrangement, so
    /// every planet lies on one ray out of the sun. That is the line the
    /// walk out through the door crosses, which is the case worth asking
    /// Spec 0065: the clip looks at the moon, and the moon is there when it
    /// does.
    ///
    /// **Three things have to agree and they are written in three places.**
    /// The route has to make the look, `FILM_AT` has to be an hour the moon
    /// is up and west at, and `FILM_DAY` has to be long enough that it has
    /// not moved by the time the camera gets there. Any one of them changing
    /// alone puts the film back to showing a patch of sky.
    #[test]
    fn the_clip_looks_at_the_moon() {
        let room = crate::room::Room::of(crate::games());
        let since = crate::garden::when(FILM_AT);

        assert!(
            crate::moon::showing(since) > 0.9,
            "at the film's hour the moon is only {:.2} lit",
            crate::moon::showing(since)
        );

        // where the route stands for its one outdoor shot, and the two
        // things it looks at from there
        let steps = clip(&room, true);
        let mut stand = None;
        let mut looks: Vec<Vec3> = Vec::new();

        for (n, step) in steps.iter().enumerate() {
            if let Step::Look(at) = step {
                // the stand this look is made from is the last `At` before it
                let from = steps[..n].iter().rev().find_map(|one| match one {
                    Step::At(at, _) => Some(*at),
                    _ => None,
                });

                if from == Some(the_way_out(&room)) {
                    stand = from;
                    looks.push(*at);
                }
            }
        }

        let stand = stand.expect("the route stands in the garden's way out");

        assert_eq!(
            looks.len(),
            2,
            "it looks {} ways from the way out, not two",
            looks.len()
        );

        let eye = vec3(stand.x, stand.y + crate::EYE, stand.z);
        let way = (looks[1] - eye).normalize();
        let moon = crate::moon::at(since);

        assert!(
            way.dot(moon) > 0.9999,
            "the second look is {:?} and the moon is {:?}",
            way,
            moon
        );

        // and it is still there whenever the camera gets to it. Where in
        // the film the shot falls is not this file's to know, so this asks
        // the whole span rather than a moment in it: the moon has to be lit
        // and in the west from the first frame to the last.
        //
        // **Asked as hours and not as seconds.** `garden::day` is whatever
        // this run is using and a test is not a film, so 58 seconds here is
        // nearly three days at `garden::DAY`. What the film gets is 58
        // seconds of `FILM_DAY`, and `when` then `at` cancel the length out
        // between them.
        let film = 58.0 / FILM_DAY;

        for n in 0..=20 {
            let hour = FILM_AT + film * n as f32 / 20.0;
            let then = crate::garden::when(hour);
            let lit = crate::moon::showing(then);
            let where_ = crate::moon::at(then);

            assert!(
                lit > 0.5,
                "a fifth of the way through the film the moon is {:.2} lit",
                lit
            );
            assert!(
                where_.x < -0.5,
                "at hour {:.3} the moon is at {:?}, which is not the west",
                hour,
                where_
            );
        }
    }

    /// Where that shot is taken from, which `clip` works out the same way.
    fn the_way_out(room: &crate::room::Room) -> Vec3 {
        let (from, to) = crate::field::way(room.reaches);

        vec3(
            crate::field::starts(room.reaches) + BACK_FROM_THE_WAY,
            0.0,
            (from + to) * 0.5,
        )
    }

    /// about; swung, they are somewhere else and this says nothing.
    #[test]
    fn the_clip_never_stands_inside_a_body() {
        let room = crate::room::Room::of(crate::games());
        let reaches = room.reaches;
        // the line the bodies rest on, taken the way the drawing takes it
        // rather than guessed: `Game::the_sun`. Guessing it put every
        // planet somewhere it is not and this passed while the clip walked
        // through Neptune.
        let middle = crate::behind::at(reaches);
        let sun = vec3(middle.x, crate::orrery::PLANE, middle.z);
        let (from_, to_) = crate::behind::way();
        let door = vec3((from_ + to_) * 0.5, crate::orrery::PLANE, reaches);
        let toward = (door - sun).normalize_or_zero();

        // **The legs and not only the ends of them.** The first version of
        // this asked about each `Step::At` and passed: the waypoint inside
        // the orrery's door is two metres clear of Neptune. What is not
        // clear is the walk out through it, which crosses Neptune's own
        // plane, and that is where the black frame is.
        let stops: Vec<Vec3> = clip(&room, true)
            .into_iter()
            .filter_map(|step| match step {
                Step::At(at, _) => Some(at),
                _ => None,
            })
            .collect();

        for leg in stops.windows(2) {
            let (from, to) = (leg[0], leg[1]);

            for planet in crate::orrery::PLANETS.iter() {
                let stood = sun + crate::orrery::stands(planet, toward, 0.0, 0.0, 0.0);
                let near = nearest_on(from, to, stood);
                let clear = near.distance(stood) - planet.body.drawn() * 0.5;

                assert!(
                    clear > crate::RADIUS,
                    "walking from {:.1} {:.1} to {:.1} {:.1} passes {:.2} from the \
                     middle of {}, which is drawn {:.2} across",
                    from.x,
                    from.z,
                    to.x,
                    to.z,
                    near.distance(stood),
                    planet.body.name,
                    planet.body.drawn()
                );
            }
        }
    }

    /// The point on the segment `from`..`to` nearest `at`.
    fn nearest_on(from: Vec3, to: Vec3, at: Vec3) -> Vec3 {
        let way = to - from;
        let length = way.length_squared();

        if length <= f32::EPSILON {
            return from;
        }

        from + way * ((at - from).dot(way) / length).clamp(0.0, 1.0)
    }

    /// A box round two places with room to get between them, which is
    /// `region` and is the film's own.
    fn between(one: Vec3, two: Vec3) -> Aabb {
        region(one, two)
    }

    /// What `Arcade::standing_on` is: the room, and the field under you
    /// when you are out in it.
    fn underfoot(solid: &[Aabb], at: Vec3, reaches: f32) -> Vec<Aabb> {
        let mut out = solid.to_vec();

        if crate::field::afoot(&out, at) {
            out.extend(crate::field::solid(reaches, at));
        }

        out
    }

    /// Walk a planned way the way the game walks it, and say where it
    /// ended up.
    ///
    /// **Where it ended up, rather than whether it finished.** The first
    /// build of this returned nothing when the follower stopped steering
    /// and read that as having arrived, which is exactly what a follower
    /// does when it gives a leg up: every leg of the route passed while
    /// the film walked a third of the first one and stopped. What a leg
    /// is owed is the place it was going to, so that is what the caller
    /// is handed and what it asks about.
    ///
    /// **The game's own driving and not the planner's.** It turns at the
    /// rate the arrows turn and only walks once it is roughly pointed at
    /// the next place, and it climbs out of a ration that fills at
    /// `walk::CLIMBS` a second. Both of those are in `Arcade::update` and
    /// neither is in `walk`, so a follower simulated without them is a
    /// different body from the one in the film.
    ///
    /// **And it starts facing where the step before left it facing.** A
    /// leg begun facing nowhere in particular has a turn in it that the
    /// film does not have, and the turn is not free: a body standing
    /// still to finish one is a body not walking. This read the last leg
    /// of the clip as a body that walks into the doorjamb, when what the
    /// film does there is turn seven degrees and go through.
    fn followed(
        from: (Vec3, u64),
        facing: f32,
        way: Vec<Spot>,
        solid: &[Aabb],
        reaches: f32,
    ) -> Vec3 {
        let step = 1.0 / 60.0;
        let over = crate::parked::aloft::frame(from.1) * Vec3::Y;
        let mut following = Follower::new(way);
        let (mut at, mut falling) = (from.0, 0.0);
        let mut yaw = facing;
        let mut climbs = 0.0;
        let mut walking = false;

        for _ in 0..(60 * 180) {
            let Some(steer) = following.steer(at, from.1, walking) else {
                break;
            };

            // `turned_toward`, which is the same arithmetic in `main`
            let turn = std::f32::consts::TAU;
            let mut by = (steer.yaw - yaw).rem_euclid(turn);

            if by > turn * 0.5 {
                by -= turn;
            }

            let most = crate::TURNS * step;
            let left = if by.abs() <= most {
                yaw = steer.yaw;

                0.0
            } else {
                yaw += most * by.signum();

                by.abs() - most
            };

            let (sin, cos) = yaw.sin_cos();

            walking = left < crate::WALKS_WHEN;

            let wish = if walking {
                crate::parked::aloft::frame(from.1) * vec3(sin, 0.0, -cos) * crate::SPEED
            } else {
                Vec3::ZERO
            };
            // **What the game is standing on, which is not the room.**
            // `Arcade::standing_on` adds the field under you whenever
            // there is sky over you, and the field is ground, a wall and
            // trees. A follower walked against the room's boxes alone is
            // a body in a different building: it walks through the
            // garden's trees, and the plan it is following was laid
            // against them.
            let standing_on = underfoot(solid, at, reaches);
            let (went, fell) = crate::parked::aloft::walked_in_frame(
                from.1,
                at,
                wish,
                falling,
                step,
                &standing_on,
                climbs <= 0.0,
            );

            climbs =
                (climbs - crate::walk::CLIMBS * step).max(0.0) + (went - at).dot(over).max(0.0);
            at = went;
            falling = fell;
        }

        at
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
            let Some(steer) = follower.steer(at, 0, true) else {
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

    /// Spec 0049: a place the body cannot reach is given up rather than
    /// leaned on.
    ///
    /// **Hand-laid, because the fault is about one place and not about a
    /// plan.** A wall, a way whose middle place is set a fifth of a metre
    /// inside it, and a last place out in the open beyond. The body has
    /// to finish at the last one.
    ///
    /// Its control is the middle place itself: the walk must never come
    /// within `NEAR` of it, or the way was walkable all along and this
    /// proves nothing.
    #[test]
    fn a_place_it_cannot_reach_is_given_up() {
        let floor = Aabb::new(vec3(-10.0, -1.0, -10.0), vec3(10.0, 0.0, 10.0));
        let wall = Aabb::new(vec3(-1.0, 0.0, 2.0), vec3(1.0, 3.0, 4.0));
        let solid = [floor, wall];
        let hidden = vec3(0.0, 0.0, 1.8);
        let way = vec![
            Spot {
                at: vec3(0.0, 0.0, -2.0),
                frame: 0,
                ground: "the floor",
            },
            Spot {
                at: hidden,
                frame: 0,
                ground: "the floor",
            },
            Spot {
                at: vec3(4.0, 0.0, -2.0),
                frame: 0,
                ground: "the floor",
            },
        ];
        let mut follower = Follower::new(way);
        let mut at = vec3(0.0, 0.0, -4.0);
        let mut falling = 0.0;
        let step = 1.0 / 60.0;
        let mut closest = f32::INFINITY;

        for _ in 0..1200 {
            let Some(steer) = follower.steer(at, 0, true) else {
                break;
            };
            let (went, fell) =
                crate::walk::walk(at, steer.wish, falling, crate::RADIUS, step, &solid, true);

            at = went;
            falling = fell;
            closest = closest.min(vec3(at.x - hidden.x, 0.0, at.z - hidden.z).length());
        }

        assert!(
            closest > NEAR,
            "the place in the wall came within {:.3}, so nothing was ever given up",
            closest
        );
        assert!(
            follower.arrived() && at.distance(vec3(4.0, 0.0, -2.0)) < 1.0,
            "it ended {:7.2}{:7.2}{:7.2}, still wanting {:?}",
            at.x,
            at.y,
            at.z,
            follower.toward()
        );
    }

    /// Specs 0047 and 0049: a mouth you have come through is behind you.
    ///
    /// **Which the distance test cannot tell you.** Every other place is
    /// retired by being got to; a portal's mouth is used by being walked
    /// into, and walking into it carries you somewhere else, so the body
    /// is further from it after using it than before. Aiming at it again
    /// walks back into it. The film crossed between the imaginarium's
    /// frames four times over and wedged in the aperture.
    ///
    /// The mouth and where it lands you are the building's own, out of
    /// `join_the_portals`, rather than a pair of invented points.
    #[test]
    fn a_mouth_you_have_crossed_is_not_aimed_at_again() {
        let out = crate::parked::aloft::round_the_other_way(1);
        let mouth = crate::parked::aloft::doorway(1, 0).center();
        let lands = crate::parked::aloft::foot_of_the_flight(out);
        let on = lands + crate::parked::aloft::frame(out) * vec3(1.0, 0.0, 0.0);
        let way = vec![
            Spot {
                at: mouth,
                ground: "a way through",
                frame: 1,
            },
            Spot {
                at: on,
                ground: "the imaginarium",
                frame: out,
            },
        ];
        let mut follower = Follower::new(way);

        // walking up to it in its own frame, it is what you are aiming at
        follower
            .steer(crate::parked::aloft::foot_of_the_flight(1), 1, true)
            .expect("a heading toward the mouth");

        assert_eq!(
            follower.toward().map(|spot| spot.at),
            Some(mouth),
            "the mouth is not what it is walking into"
        );

        // and then the crossing happens, which is a change of frame
        follower
            .steer(lands, out, false)
            .expect("a heading after the crossing");

        assert_eq!(
            follower.toward().map(|spot| spot.at),
            Some(on),
            "it is still aiming at the mouth it came through"
        );
    }

    /// Spec 0049: a place behind you is still a place you get to.
    ///
    /// **Which is a test about the turn and not about the walk.** An
    /// about-turn is 180 degrees and the arrows turn 2.4 radians a
    /// second, so it is 78 frames in which the body stands still and gets
    /// no closer to anything. Counted as frames of no progress that is a
    /// second and a half of wall, and the place is given up before the
    /// first step. Nothing in the clip's route opens with a turn that
    /// big, which is why this is written out rather than left to the
    /// route to catch.
    #[test]
    fn a_place_behind_you_is_walked_to_and_not_given_up() {
        let solid = [Aabb::new(vec3(-10.0, -1.0, -10.0), vec3(10.0, 0.0, 10.0))];
        let behind = vec3(0.0, 0.0, 4.0);
        let way = vec![Spot {
            at: behind,
            frame: 0,
            ground: "the floor",
        }];
        let mut follower = Follower::new(way);
        let mut at = Vec3::ZERO;
        let mut falling = 0.0;
        // nought faces -z, so the place is a half turn away
        let mut yaw = 0.0f32;
        let mut walking = false;
        let step = 1.0 / 60.0;

        for _ in 0..600 {
            let Some(steer) = follower.steer(at, 0, walking) else {
                break;
            };
            let turn = std::f32::consts::TAU;
            let mut by = (steer.yaw - yaw).rem_euclid(turn);

            if by > turn * 0.5 {
                by -= turn;
            }

            let most = crate::TURNS * step;
            let left = if by.abs() <= most {
                yaw = steer.yaw;

                0.0
            } else {
                yaw += most * by.signum();

                by.abs() - most
            };
            let (sin, cos) = yaw.sin_cos();

            walking = left < crate::WALKS_WHEN;

            let wish = if walking {
                vec3(sin, 0.0, -cos) * crate::SPEED
            } else {
                Vec3::ZERO
            };
            let (went, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

            at = went;
            falling = fell;
        }

        assert!(
            at.distance(behind) < NEAR,
            "it ended {:7.2}{:7.2}{:7.2}, {:.2} from a place on open floor",
            at.x,
            at.y,
            at.z,
            at.distance(behind)
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
            let Some(steer) = follower.steer(at, 0, true) else {
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
        // **Its own floors and its own boxes.** Spec 0063 parked the
        // imaginarium, so the building's grounds no longer hold the three
        // frames and a planner given them would find nothing to cross.
        // What is being tested is the planner's portal edge, which is
        // `tour`'s and is not parked, so the room is handed to it directly.
        let grounds: Vec<(&str, Aabb)> = crate::parked::aloft::floors()
            .into_iter()
            .map(|slab| ("the imaginarium", slab))
            .collect();
        let solid = crate::parked::aloft::solid();
        let middle = crate::parked::aloft::middle();
        let whole = Aabb::new(
            middle - Vec3::splat(crate::parked::aloft::HALF + 1.0),
            middle + Vec3::splat(crate::parked::aloft::HALF + 1.0),
        );
        let ways = ways(&grounds, &solid, whole);

        let from = crate::parked::aloft::in_frame(0, vec3(0.0, -crate::parked::aloft::HALF, 0.0));
        let to = crate::parked::aloft::arrive_through(1, 1);
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
        // **The far end of the nook, which the stair used to stand at.**
        // The journey that sent this wrong went to the top of the way up
        // to the imaginarium, and spec 0063 parked that stair. What made
        // the fault was the region, not the stair: a box from the front
        // door to anything at the nook's far end stops short of the gap
        // the nook opens through, and that is still true of this one.
        let to = vec3(-7.85, 0.0, 2.2);

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
            "there is no way from the door to the far end of the nook"
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
        let steps = clip(&room, true);
        let reaches = room.reaches;

        // the two things on this route that swing, read off the same
        // accessors the route reads, so there is one account of each
        let case = vec3(room.floors()[1].min.x, 0.0, crate::cellar::opening(reaches));
        let leaf = crate::spa::doorway(reaches);
        // **And how near a `Work` has to be to one of them to be one.** Not
        // every `Work` on this route is a door: spec 0050's cube is worked
        // too, and asked which of the two doors it was nearer this opened
        // the bookcase ten seconds before the route reaches it. The room
        // reads as having been through a door nobody has been through yet.
        let at_a_door = crate::room::REACH;

        let mut solid = room.solid();
        let mut sight = Vec3::ZERO;
        let mut facing = 0.0;
        let mut last: Option<(Vec3, u64)> = None;
        let mut legs = 0;
        let mut walking = 0.0;
        let mut standing = 0.0;
        let mut broken = Vec::new();

        for step in steps.iter() {
            match step {
                Step::Look(at) => {
                    sight = *at;

                    // which is where the body is left pointing, worked
                    // out the way the film's own `Look` works it out
                    if let Some((here, frame)) = last {
                        let way = crate::parked::aloft::frame(frame).inverse() * (sight - here);

                        facing = way.x.atan2(-way.z);
                    }
                }
                Step::Work => {
                    if sight.distance(case) < at_a_door {
                        room.open = true;
                        room.swing = 1.0;
                    } else if sight.distance(leaf) < at_a_door {
                        room.baths_open = true;
                        room.baths_swing = 1.0;
                    }

                    solid = room.solid();
                }
                Step::Stand(how_long) => standing += *how_long,
                Step::At(at, frame) => {
                    if let Some(from) = last {
                        legs += 1;

                        // the leg's boxes are the game's: it asks
                        // `standing_on` once, where it is standing, and
                        // lays the lattice against that
                        let here = underfoot(&solid, from.0, reaches);
                        let ways = ways(&room.grounds(), &here, between(from.0, *at));

                        if let Some(way) = ways.plan(from, (*at, *frame), &here) {
                            // how far the walking is, which is what the
                            // clip's length in `refresh-examples` is made of
                            let mut was = from.0;

                            for spot in way.iter() {
                                walking += was.distance(spot.at);
                                was = spot.at;
                            }

                            // and then walk it, which is a different
                            // question. A leg the planner is happy with
                            // is a leg the body may still not get down:
                            // the plan's places were joined by a walker
                            // that could climb every frame, and the one
                            // in the game climbs out of a ration. That
                            // gap stood the camera on a seam in the
                            // imaginarium's floor for the last hundred
                            // seconds of a take, with every leg of this
                            // route planning clean.
                            //
                            // The legs that change frame are left out,
                            // because what carries you between frames is
                            // a crossing, which is the room's move rather
                            // than the walker's and is not in `walk` at
                            // all.
                            //
                            // **Which is a hole in this, and the film has
                            // already fallen through it.** The one leg
                            // left out was the one that broke, and it
                            // broke in the follower rather than in the
                            // aperture: a mouth crossed and then aimed at
                            // again, which `aloft`'s aperture tests have
                            // no reason to look for.
                            // `a_mouth_you_have_crossed_is_not_aimed_at_again`
                            // holds that rule now, and
                            // `the_clip_walks_the_whole_route` in `main`
                            // walks the route through a real `Arcade`,
                            // crossings and doors and all, which is what
                            // this cannot do. The clip has not crossed a
                            // frame since the imaginarium came out of it,
                            // so nothing is exercising one today.
                            if from.1 == *frame {
                                let ended = followed(from, facing, way, &solid, reaches);

                                // a stride short is a body that walked
                                // it; anything further is a leg the film
                                // does not finish
                                if ended.distance(*at) > 1.0 {
                                    broken.push(format!(
                                        "{:?} to {:?}, which planned but ended {:.1} away at {:7.2}{:7.2}{:7.2}",
                                        from,
                                        (*at, *frame),
                                        ended.distance(*at),
                                        ended.x,
                                        ended.y,
                                        ended.z
                                    ));
                                }
                            }
                        } else {
                            broken.push(format!("{:?} to {:?}", from, (*at, *frame)));

                            // **And where it stopped, not just that it
                            // did.** Which rooms a leg got to is the
                            // question; a height is what sent this up a
                            // staircase three times for a fault two rooms
                            // earlier.
                            if let (Some(one), Some(two)) = (
                                ways.nearest(from.0, from.1, &here),
                                ways.nearest(*at, *frame, &here),
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

        // the garden, the solarium, the bookcase, the baths and the pool:
        // five, and the point of counting is that a room cannot drop out
        // of the clip without this saying so. It was eight until the
        // imaginarium came out of the route on 2026-11-02.
        assert!(legs > 4, "the route only has {} legs", legs);

        // **And it leaves room for the turns.** How long the camera runs
        // is in `refresh-examples` and is deliberately not repeated here:
        // this crate is its own repo and a clone of it has no such file.
        // What this can say is the shape of the answer. The turning is
        // not counted above and is worth a fair part of the walking on a
        // route with this many stops, so anything approaching a minute
        // before a single turn is a route somebody has to go and check
        // the camera against.
        let takes = walking / crate::SPEED + standing;

        assert!(
            takes < 60.0,
            "the route is {:.0} seconds of walking and standing before any \
             turning, which is long enough that the camera's own run in \
             refresh-examples has to be looked at",
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
        let half = crate::parked::aloft::HALF;
        let here = crate::parked::aloft::in_frame(1, vec3(0.0, -half, 0.0));
        let there = crate::parked::aloft::in_frame(1, vec3(0.0, -half, 2.0));

        let mut follower = Follower::new(vec![Spot {
            at: there,
            ground: "the imaginarium",
            frame: 1,
        }]);

        let steer = follower
            .steer(here, 1, true)
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
