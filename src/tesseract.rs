//! Eight rooms that are one room. Spec 0021.
//!
//! A tesseract's cells are its eight facets: the faces of a four dimensional
//! cube at `x = ±1`, `y = ±1`, `z = ±1` and `w = ±1`. Each is an ordinary
//! cube, and the inside of one is an ordinary room.
//!
//! Nothing here draws anything. Which cell meets which, and what happens to
//! somebody who walks out of one, are arithmetic, and arithmetic can be
//! checked without a window.

/// One of the four directions a tesseract has.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
    /// The one nobody has. Hinton called a step along it ana, and the other
    /// way kata, which is where this file's whole trick lives.
    W,
}

impl Axis {
    pub fn n(self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
            Axis::W => 3,
        }
    }
}

/// An axis with a sign: one of the eight directions, and one of the eight
/// cells, which are the same list for the same reason.
///
/// A cell *is* a direction, because a facet of a four dimensional cube is
/// named by the way it faces. The `+w` cell is the facet at `w = 1`, and the
/// direction `+w` is the way out of the whole shape through it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Way {
    pub axis: Axis,
    pub up: bool,
}

impl Way {
    pub const fn new(axis: Axis, up: bool) -> Way {
        Way { axis, up }
    }

    pub fn back(self) -> Way {
        Way::new(self.axis, !self.up)
    }

    pub fn signed(self, by: bool) -> Way {
        if by {
            self
        } else {
            self.back()
        }
    }

    /// Where this points, as four numbers.
    pub fn along(self) -> [f32; 4] {
        let mut out = [0.0; 4];
        out[self.axis.n()] = if self.up { 1.0 } else { -1.0 };

        out
    }
}

/// A cell is a way, and this name is here so the code can say which it means.
pub type Cell = Way;

/// Every cell, in a fixed order so two runs agree.
pub fn cells() -> [Cell; 8] {
    [
        Way::new(Axis::X, true),
        Way::new(Axis::X, false),
        Way::new(Axis::Y, true),
        Way::new(Axis::Y, false),
        Way::new(Axis::Z, true),
        Way::new(Axis::Z, false),
        Way::new(Axis::W, true),
        Way::new(Axis::W, false),
    ]
}

/// Whether two cells share a square face.
///
/// **They meet when their axes differ**, which is the one fact the whole
/// shape turns on and reads backwards from the three dimensional habit. `+x`
/// and `-x` are the two cells that do not touch, because they are the two
/// ends of the same direction and a cube's opposite faces share no edge.
/// Everything else does.
pub fn meets(one: Cell, other: Cell) -> bool {
    one.axis != other.axis
}

/// The one cell you cannot reach in a single step.
pub fn opposite(cell: Cell) -> Cell {
    cell.back()
}

/// The six that you can.
pub fn neighbours(cell: Cell) -> Vec<Cell> {
    cells()
        .iter()
        .copied()
        .filter(|it| meets(cell, *it))
        .collect()
}

/// Which way somebody is standing, as four directions of the world.
///
/// `ana` is the direction they are flattened against, which is to say the
/// cell they are in: a person inside the `+w` facet cannot move along `w`,
/// and the three they can move along are the other three.
///
/// The other three are an ordinary first person frame. `up` is which way
/// their head is and which way gravity is not, and `ahead` is the way they
/// walk when they walk forward.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Frame {
    pub right: Way,
    pub up: Way,
    pub ahead: Way,
    pub ana: Way,
}

/// The way somebody can walk out of a room.
///
/// Four and not six, and that is the design rather than a shortcut. A person
/// walks about on a floor; they do not walk into the floor or the ceiling.
/// The two faces across their own up are cells they cannot reach without
/// first becoming somebody standing another way up, which is what every other
/// crossing does to them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Ahead,
    Back,
    Right,
    Left,
}

impl Step {
    pub const ALL: [Step; 4] = [Step::Ahead, Step::Back, Step::Right, Step::Left];
}

impl Frame {
    /// Somebody standing in the `+w` cell the ordinary way up.
    pub fn new() -> Frame {
        Frame {
            right: Way::new(Axis::X, true),
            up: Way::new(Axis::Y, true),
            ahead: Way::new(Axis::Z, true),
            ana: Way::new(Axis::W, true),
        }
    }

    /// Which cell this is standing in.
    pub fn cell(self) -> Cell {
        self.ana
    }

    /// Which way a step actually goes, in the world.
    pub fn toward(self, step: Step) -> Way {
        match step {
            Step::Ahead => self.ahead,
            Step::Back => self.ahead.back(),
            Step::Right => self.right,
            Step::Left => self.right.back(),
        }
    }

    /// Which cell is through the wall a step would take you to.
    pub fn through(self, step: Step) -> Cell {
        self.toward(step)
    }

    /// Walking out of this cell and into the next one.
    ///
    /// **Two turns, one of which the geometry forces and one of which it does
    /// not.**
    ///
    /// The forced one is the corner. Walking along `ahead` reaches the square
    /// the two cells share, and carrying on means turning onto the other
    /// facet, which rotates `ana` onto `ahead` and `ahead` onto `-ana`. That
    /// is what the shape does and there is no choice in it.
    ///
    /// The second is a pitch, and it is this room's decision rather than four
    /// dimensional fact. The corner alone leaves `up` where it was: you would
    /// walk forward, keep walking forward, and the only thing that changed
    /// would be which direction had become unavailable. True, and nothing to
    /// look at. So the floor is put back under the wall you came through:
    /// `up` becomes the way you came from, so the wall you walked at is now
    /// the floor you are standing on and the room you left is overhead.
    ///
    /// What the fourth dimension forces is which cell. What is chosen is
    /// which way up it leaves you, and this spends the choice on the only
    /// part of it anybody can see.
    ///
    /// The axis you did not use is carried through unchanged, so a sidestep
    /// leaves you facing the way you faced and a walk forward leaves your
    /// shoulders where they were. The fourth is whatever keeps this a
    /// rotation rather than a reflection, which is `turned` below.
    pub fn step(self, step: Step) -> Frame {
        let toward = self.toward(step);
        let ana = toward;
        let up = self.ana.back();

        // the one of right and ahead that this step did not spend
        let (kept_is_right, kept) = match step {
            Step::Ahead | Step::Back => (true, self.right),
            Step::Right | Step::Left => (false, self.ahead),
        };

        // and the old up goes into the slot the step emptied, whichever sign
        // keeps a right handed frame
        let (right, ahead) = if kept_is_right {
            (kept, turned(kept, up, ana, self.up, false))
        } else {
            (turned(kept, up, ana, self.up, true), kept)
        };

        Frame {
            right,
            up,
            ahead,
            ana,
        }
    }
}

impl Default for Frame {
    fn default() -> Frame {
        Frame::new()
    }
}

/// Which sign the last direction takes, so a frame stays a rotation.
///
/// Three of the four are decided by the step. The fourth is on the one axis
/// nobody has claimed, and its sign is whatever makes the determinant one. A
/// frame with determinant minus one is a mirror, and walking through a wall
/// does not turn a person inside out.
///
/// `filling_right` says which slot it is going into, because the order of the
/// four matters to the sign and a frame is `right, up, ahead, ana`.
fn turned(kept: Way, up: Way, ana: Way, was: Way, filling_right: bool) -> Way {
    let guess = Way::new(was.axis, true);
    let order = |first: Way| {
        if filling_right {
            [first, up, kept, ana]
        } else {
            [kept, up, first, ana]
        }
    };

    if handed(order(guess)) {
        guess
    } else {
        guess.back()
    }
}

/// Whether four directions make a right handed frame.
///
/// They are four perpendicular signed axes, so written out as a matrix they
/// are a signed permutation and the determinant is the sign of that
/// permutation times the signs themselves. Nothing here needs a general
/// determinant, which was the first way this was written and is four three by
/// three expansions to say what counting pairs says.
///
/// Minus one is a mirror, and walking through a wall does not turn a person
/// inside out.
fn handed(ways: [Way; 4]) -> bool {
    let places: Vec<usize> = ways.iter().map(|way| way.axis.n()).collect();
    let mut swaps = 0;

    for (n, one) in places.iter().enumerate() {
        for other in places[n + 1..].iter() {
            if one > other {
                swaps += 1;
            }
        }
    }

    let flips = ways.iter().filter(|way| !way.up).count();

    (swaps + flips) % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0021: six neighbours and one opposite, and the neighbours are the
    /// cells on the other three axes.
    ///
    /// The fact the whole shape turns on, and the one that reads backwards
    /// from three dimensions: the two cells that do not touch are the two
    /// ends of one direction.
    #[test]
    fn every_cell_has_six_neighbours_and_one_opposite() {
        for cell in cells().iter().copied() {
            let near = neighbours(cell);

            assert_eq!(near.len(), 6, "{:?} has {} neighbours", cell, near.len());
            assert!(
                !near.contains(&opposite(cell)),
                "{:?} reaches its own opposite in one step",
                cell
            );
            assert!(
                near.iter().all(|it| it.axis != cell.axis),
                "{:?} has a neighbour on its own axis",
                cell
            );

            // and this measured something: eight cells, so six near, one
            // opposite and itself
            assert_eq!(cells().len(), near.len() + 2);
        }
    }

    /// Spec 0021: a step lands you in the cell its wall names.
    #[test]
    fn a_step_lands_in_the_cell_its_wall_names() {
        let frame = Frame::new();

        for step in Step::ALL.iter().copied() {
            let named = frame.through(step);
            let after = frame.step(step);

            assert_eq!(
                after.cell(),
                named,
                "{:?} named {:?} and arrived in {:?}",
                step,
                named,
                after.cell()
            );
            assert!(
                meets(frame.cell(), after.cell()),
                "{:?} left {:?} for {:?}, which it does not meet",
                step,
                frame.cell(),
                after.cell()
            );
        }
    }

    /// Spec 0021: you cannot walk back the way you came.
    ///
    /// **Because the wall you came through is the floor you are standing on**,
    /// and nobody walks into a floor. This was written the other way round
    /// first, as a step and the step back putting you where you were, and it
    /// cannot: the two halves of this room contradict each other and the
    /// measurement is what said so.
    ///
    /// It is the better rule. A door you can step back out of is a door, and
    /// a room nobody can retreat from is the one worth building.
    #[test]
    fn you_cannot_walk_back_the_way_you_came() {
        for first in Step::ALL.iter().copied() {
            let frame = Frame::new();
            let out = frame.step(first);

            assert!(
                !Step::ALL
                    .iter()
                    .copied()
                    .any(|step| out.through(step) == frame.cell()),
                "{:?} left a way straight back into {:?}",
                first,
                frame.cell()
            );
        }
    }

    /// Spec 0021: three steps bring you back exactly, and nothing shorter
    /// does.
    ///
    /// Measured rather than chosen, and a surprise: walking a triangle puts
    /// you in the room you left, the way up you left it, with every one of
    /// the four directions where it was. One step and two steps do not.
    ///
    /// It is the shortest way home and it is what makes the room playable
    /// rather than cruel, given nobody can retreat.
    #[test]
    fn three_steps_bring_you_back_exactly() {
        let start = Frame::new();
        let mut homes = 0;

        for one in Step::ALL.iter().copied() {
            let after_one = start.step(one);
            assert_ne!(after_one, start, "{:?} alone came back", one);

            for two in Step::ALL.iter().copied() {
                let after_two = after_one.step(two);
                assert_ne!(after_two, start, "{:?} then {:?} came back", one, two);

                for three in Step::ALL.iter().copied() {
                    if after_two.step(three) == start {
                        homes += 1;
                    }
                }
            }
        }

        assert!(homes > 0, "no three steps come back at all");
    }

    /// Spec 0021: the four walls a step can reach name four different cells.
    #[test]
    fn the_walls_name_four_different_cells() {
        let mut frame = Frame::new();

        // after a few steps as well as at the start, because a frame that has
        // been turned about is the one worth asking
        for step in [Step::Ahead, Step::Right, Step::Ahead, Step::Left]
            .iter()
            .copied()
        {
            let named: Vec<Cell> = Step::ALL
                .iter()
                .copied()
                .map(|it| frame.through(it))
                .collect();

            for (n, one) in named.iter().enumerate() {
                assert!(
                    meets(frame.cell(), *one),
                    "a wall of {:?} names {:?}, which it does not meet",
                    frame.cell(),
                    one
                );
                assert!(
                    !named[n + 1..].contains(one),
                    "two walls of {:?} name {:?}",
                    frame.cell(),
                    one
                );
            }

            frame = frame.step(step);
        }
    }

    /// Spec 0021: walking never turns anybody inside out.
    ///
    /// A frame is four signed perpendicular directions, so its determinant is
    /// one or minus one and nothing in between. Minus one is a mirror, and a
    /// person who walks through a wall and comes out mirrored is a bug nobody
    /// would name correctly from the picture: they would say the room was
    /// wrong.
    #[test]
    fn walking_never_turns_anybody_inside_out() {
        let mut frame = Frame::new();

        assert!(handed([frame.right, frame.up, frame.ahead, frame.ana]));

        for step in [
            Step::Ahead,
            Step::Right,
            Step::Right,
            Step::Back,
            Step::Left,
            Step::Ahead,
            Step::Left,
            Step::Back,
        ]
        .iter()
        .copied()
        {
            frame = frame.step(step);

            assert!(
                handed([frame.right, frame.up, frame.ahead, frame.ana]),
                "{:?} left a mirrored frame: {:?}",
                step,
                frame
            );
            assert!(
                [frame.right, frame.up, frame.ahead, frame.ana]
                    .iter()
                    .all(|way| [frame.right, frame.up, frame.ahead, frame.ana]
                        .iter()
                        .filter(|it| it.axis == way.axis)
                        .count()
                        == 1),
                "{:?} left two directions on one axis: {:?}",
                step,
                frame
            );
        }
    }

    /// Spec 0021: the wall you walked at is the floor you are standing on.
    ///
    /// The chosen half of the turn rather than the forced half. See `step`.
    #[test]
    fn the_wall_you_walked_at_is_the_floor_you_land_on() {
        let frame = Frame::new();

        for step in Step::ALL.iter().copied() {
            let after = frame.step(step);

            // down is the way you came from, which is where the old cell is
            assert_eq!(
                after.up.back(),
                frame.cell(),
                "{:?} left the floor pointing at {:?} rather than {:?}",
                step,
                after.up.back(),
                frame.cell()
            );
            // and the room you left is overhead
            assert_eq!(after.up, frame.cell().back());
        }
    }

    /// Spec 0021: every cell is reachable, and how far each one is.
    ///
    /// **Not the map the cells make.** Every cell meets six others, so the
    /// abstract answer is that everything is one step away but the opposite,
    /// which is two. That is not what a person walking finds, because a
    /// person walks on a floor and two of the six walls of any room are the
    /// floor and the ceiling.
    ///
    /// Four walls, so four cells in one step. The two across your own up take
    /// two, because you have to become somebody standing another way up
    /// first. And the opposite takes three, which is one more than the shape
    /// says and is the number that matters.
    #[test]
    fn every_cell_is_reachable_and_the_opposite_takes_three() {
        let start = Frame::new();
        let key = |f: Frame| (f.right, f.up, f.ahead, f.ana);
        let mut seen = std::collections::HashSet::new();
        let mut far = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::new();

        seen.insert(key(start));
        far.insert(start.cell(), 0usize);
        queue.push_back((start, 0usize));

        while let Some((frame, depth)) = queue.pop_front() {
            for step in Step::ALL.iter().copied() {
                let next = frame.step(step);
                if seen.insert(key(next)) {
                    far.entry(next.cell()).or_insert(depth + 1);
                    queue.push_back((next, depth + 1));
                }
            }
        }

        assert_eq!(far.len(), 8, "only {} cells can be reached", far.len());
        assert_eq!(
            far.get(&opposite(start.cell())),
            Some(&3),
            "the opposite cell is {:?} steps away",
            far.get(&opposite(start.cell()))
        );

        let near = far.values().filter(|it| **it == 1).count();
        let over = far.values().filter(|it| **it == 2).count();

        assert_eq!(near, 4, "{} cells are one step away", near);
        assert_eq!(over, 2, "{} cells are two steps away", over);
    }

    /// Spec 0021: twelve ways to stand in a room, and nobody is ever stuck.
    ///
    /// Ninety six states over eight cells. It is the size of the thing a
    /// player is actually holding in their head, against the eight the map
    /// would suggest, and it is why the map is not the puzzle.
    #[test]
    fn twelve_ways_to_stand_in_a_room_and_nobody_is_stuck() {
        let start = Frame::new();
        let key = |f: Frame| (f.right, f.up, f.ahead, f.ana);
        let mut seen = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        seen.insert(key(start));
        queue.push_back(start);

        while let Some(frame) = queue.pop_front() {
            for step in Step::ALL.iter().copied() {
                let next = frame.step(step);
                if seen.insert(key(next)) {
                    queue.push_back(next);
                }
            }
        }

        assert_eq!(seen.len(), 96, "{} states can be reached", seen.len());

        for cell in cells().iter().copied() {
            let ways = seen.iter().filter(|it| it.3 == cell).count();

            assert_eq!(ways, 12, "{:?} can be stood in {} ways", cell, ways);
        }

        // and every one of them can get home again, which is the thing that
        // would make this cruel if it were not true
        for one in seen.iter() {
            let from = Frame {
                right: one.0,
                up: one.1,
                ahead: one.2,
                ana: one.3,
            };
            let mut saw = std::collections::HashSet::new();
            let mut queue = std::collections::VecDeque::new();

            saw.insert(key(from));
            queue.push_back(from);

            let mut home = false;
            while let Some(frame) = queue.pop_front() {
                if key(frame) == key(start) {
                    home = true;
                    break;
                }
                for step in Step::ALL.iter().copied() {
                    let next = frame.step(step);
                    if saw.insert(key(next)) {
                        queue.push_back(next);
                    }
                }
            }

            assert!(home, "{:?} cannot get home", from);
        }
    }
}
