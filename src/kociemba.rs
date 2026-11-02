//! A three by three cube on a bench, and the state that turning it moves.
//! Spec 0050.
//!
//! **Twenty pieces, not twenty-six.** The thing drawn is twenty-six cubies,
//! but the six centres never move: they are what the faces mean. So the
//! state is eight corners and twelve edges, each with a place and a twist,
//! and that is also the representation the two phase solver wants, which is
//! why this is the shape it is from the first commit rather than a soup of
//! cubies converted later.
//!
//! **The naming is Kociemba's**, because the tables in every published
//! account of the algorithm are written in it and a second convention here
//! would be a second account of which corner is which. Corners run URF, UFL,
//! ULB, UBR, DFR, DLF, DBL, DRB, and edges UR, UF, UL, UB, DR, DF, DL, DB,
//! FR, FL, BL, BR. A corner's twist is 0, 1 or 2 turns clockwise seen from
//! outside; an edge is flipped or it is not.
//!
//! Nothing here draws, decides a colour or says whether a cube is solved to
//! anybody. Spec 0050 is firm that the bench never tells you that you have
//! won, and the way to keep that true is for the part that knows to have no
//! way of saying so.

/// The eight corners and the twelve edges, by the names the tables use.
pub const CORNERS: usize = 8;
pub const EDGES: usize = 12;

/// The six faces, in the order the move tables are written in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back,
}

impl Face {
    pub const ALL: [Face; 6] = [
        Face::Up,
        Face::Right,
        Face::Front,
        Face::Down,
        Face::Left,
        Face::Back,
    ];

    /// The face across the cube from this one.
    ///
    /// Worth having rather than worth working out: two opposite faces are
    /// the only pair whose turns do not disturb each other, which is a
    /// property the tests lean on and the second phase of the solve is
    /// built out of.
    pub fn across(self) -> Face {
        match self {
            Face::Up => Face::Down,
            Face::Right => Face::Left,
            Face::Front => Face::Back,
            Face::Down => Face::Up,
            Face::Left => Face::Right,
            Face::Back => Face::Front,
        }
    }
}

/// Where every piece is and how it is turned.
///
/// `corner_at[n]` is which corner is sitting in slot `n`, not where corner
/// `n` has gone. The tables are written that way round and reading them the
/// other way is the commonest way to get a cube model subtly wrong: it
/// still turns, four turns still come home, and the solver never finds
/// anything.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cube {
    pub corner_at: [u8; CORNERS],
    pub corner_spun: [u8; CORNERS],
    pub edge_at: [u8; EDGES],
    pub edge_flipped: [u8; EDGES],
}

/// One quarter turn, as the cube it makes of a solved one.
struct Turn {
    corner_at: [u8; CORNERS],
    corner_spun: [u8; CORNERS],
    edge_at: [u8; EDGES],
    edge_flipped: [u8; EDGES],
}

/// A quarter turn of each face, clockwise seen from outside it.
///
/// These are the published tables rather than anything worked out here. The
/// tests are what say they were copied correctly: four of any turn comes
/// home, opposite faces do not disturb each other, adjacent ones do, the
/// twists stay divisible and `(R U R' U')` six times is a solved cube, which
/// is a fact about a cube and not about this code.
const TURNS: [Turn; 6] = [
    // U
    Turn {
        corner_at: [3, 0, 1, 2, 4, 5, 6, 7],
        corner_spun: [0; CORNERS],
        edge_at: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
        edge_flipped: [0; EDGES],
    },
    // R
    Turn {
        corner_at: [4, 1, 2, 0, 7, 5, 6, 3],
        corner_spun: [2, 0, 0, 1, 1, 0, 0, 2],
        edge_at: [8, 1, 2, 3, 11, 5, 6, 7, 4, 9, 10, 0],
        edge_flipped: [0; EDGES],
    },
    // F
    Turn {
        corner_at: [1, 5, 2, 3, 0, 4, 6, 7],
        corner_spun: [1, 2, 0, 0, 2, 1, 0, 0],
        edge_at: [0, 9, 2, 3, 4, 8, 6, 7, 1, 5, 10, 11],
        edge_flipped: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
    },
    // D
    Turn {
        corner_at: [0, 1, 2, 3, 5, 6, 7, 4],
        corner_spun: [0; CORNERS],
        edge_at: [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
        edge_flipped: [0; EDGES],
    },
    // L
    Turn {
        corner_at: [0, 2, 6, 3, 4, 1, 5, 7],
        corner_spun: [0, 1, 2, 0, 0, 2, 1, 0],
        edge_at: [0, 1, 10, 3, 4, 5, 9, 7, 8, 2, 6, 11],
        edge_flipped: [0; EDGES],
    },
    // B
    Turn {
        corner_at: [0, 1, 3, 7, 4, 5, 2, 6],
        corner_spun: [0, 0, 1, 2, 0, 0, 2, 1],
        edge_at: [0, 1, 2, 11, 4, 5, 6, 10, 8, 9, 3, 7],
        edge_flipped: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
    },
];

impl Cube {
    /// Every piece home and nothing twisted.
    pub const SOLVED: Cube = Cube {
        corner_at: [0, 1, 2, 3, 4, 5, 6, 7],
        corner_spun: [0; CORNERS],
        edge_at: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        edge_flipped: [0; EDGES],
    };

    /// This cube with a face turned a quarter turn clockwise, `quarters`
    /// times. Three is the same as one anticlockwise and four is nothing.
    pub fn turned(&self, face: Face, quarters: u8) -> Cube {
        let mut out = *self;

        for _ in 0..quarters % 4 {
            out = out.once(&TURNS[face as usize]);
        }

        out
    }

    /// **The twist adds and the place follows.** A piece arriving in a slot
    /// brings its own twist with it and the turn adds its own on top, which
    /// is why this is a sum rather than a swap, and why reading the table
    /// the other way round gives something that turns and never solves.
    fn once(&self, turn: &Turn) -> Cube {
        let mut out = Cube::SOLVED;

        for n in 0..CORNERS {
            let from = turn.corner_at[n] as usize;

            out.corner_at[n] = self.corner_at[from];
            out.corner_spun[n] = (self.corner_spun[from] + turn.corner_spun[n]) % 3;
        }

        for n in 0..EDGES {
            let from = turn.edge_at[n] as usize;

            out.edge_at[n] = self.edge_at[from];
            out.edge_flipped[n] = (self.edge_flipped[from] + turn.edge_flipped[n]) % 2;
        }

        out
    }

    /// Whether every piece is home and flat.
    ///
    /// For the tests and for the solver's own stopping, and deliberately not
    /// for anybody standing at the bench. Spec 0050.
    pub fn solved(&self) -> bool {
        *self == Cube::SOLVED
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::SOLVED
    }
}

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// What the bench it stands on is called, and what the sight says you are
/// looking at.
///
/// Jake's, 2026-11-02, and named for the algorithm rather than the puzzle:
/// the trademark on the latter is live, and this bench is the first in the
/// building whose subject is an algorithm, so the name is accurate as well
/// as clear of anybody's mark. Spec 0050 has the rest of that.
pub const NAME: &str = "kociemba";

/// How many of each coordinate there is.
///
/// **Every one of these is a count rather than a size.** A coordinate is a
/// whole state squeezed to one number, and the number of numbers is what
/// decides what a table over it costs. Named here so the table builders and
/// the tests that walk every value read the same figure.
pub const TWISTS: usize = 2187; // three ways each for seven corners, the eighth told by them
pub const FLIPS: usize = 2048; // two ways each for eleven edges, the twelfth told by them
pub const SLICES: usize = 495; // which four of the twelve slots hold the slice edges
pub const CORNER_WAYS: usize = 40320; // eight corners in order
pub const UD_EDGE_WAYS: usize = 40320; // the eight edges that are not the slice
pub const SLICE_WAYS: usize = 24; // the four slice edges among themselves

/// The eighteen turns: a quarter, a half and a quarter back, of each face.
pub const TURNS_IN_ALL: usize = 18;

/// The `n`th of them, as a face and a number of quarters.
pub fn turn_number(n: usize) -> (Face, u8) {
    (Face::ALL[n / 3], (n % 3) as u8 + 1)
}

/// Choosing `k` from `n`, which is the whole of the slice coordinate.
const fn choose(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut out = 1;
    let mut i = 0;

    while i < k {
        out = out * (n - i) / (i + 1);
        i += 1;
    }

    out
}

impl Cube {
    /// How the corners are twisted, as one number. Seven of them say it and
    /// the eighth is whatever makes the sum divisible by three.
    pub fn twist(&self) -> u16 {
        self.corner_spun[..CORNERS - 1]
            .iter()
            .fold(0u16, |n, &spun| n * 3 + spun as u16)
    }

    /// A solved cube twisted that way and nothing else.
    pub fn with_twist(n: u16) -> Cube {
        let mut out = Cube::SOLVED;
        let mut left = n;
        let mut sum = 0;

        for slot in (0..CORNERS - 1).rev() {
            out.corner_spun[slot] = (left % 3) as u8;
            sum += out.corner_spun[slot];
            left /= 3;
        }

        out.corner_spun[CORNERS - 1] = (3 - sum % 3) % 3;
        out
    }

    /// Which edges are flipped, as one number, on the same argument.
    pub fn flip(&self) -> u16 {
        self.edge_flipped[..EDGES - 1]
            .iter()
            .fold(0u16, |n, &flipped| n * 2 + flipped as u16)
    }

    pub fn with_flip(n: u16) -> Cube {
        let mut out = Cube::SOLVED;
        let mut left = n;
        let mut sum = 0;

        for slot in (0..EDGES - 1).rev() {
            out.edge_flipped[slot] = (left % 2) as u8;
            sum += out.edge_flipped[slot];
            left /= 2;
        }

        out.edge_flipped[EDGES - 1] = sum % 2;
        out
    }

    /// Which four slots hold the four slice edges, as one number, and not
    /// which of them is in which: that is the other coordinate and the
    /// second phase's business.
    ///
    /// **This is the whole of what the first phase is driving at.** Phase
    /// one ends when the twist is nothing, the flip is nothing and these
    /// four are home, and the number here is the only one of the three that
    /// a person could read off a cube.
    pub fn slice(&self) -> u16 {
        let mut out = 0;
        let mut found = 0;

        // **Counted from the far end, so that home is nought.** Counted
        // from the near end it is a bijection just the same, and every
        // round trip passes, and nought means the four slice edges are
        // sitting in the four slots at the top of the cube. Phase one
        // drove them there and phase two then read eight edges that were
        // not the eight it meant. Found by the one test that solves a
        // cube and looks at it.
        for slot in (0..EDGES).rev() {
            if self.edge_at[slot] >= 8 {
                out += choose(EDGES - 1 - slot, found + 1);
                found += 1;
            }
        }

        out as u16
    }

    pub fn with_slice(n: u16) -> Cube {
        let mut out = Cube::SOLVED;
        let mut left = n as usize;
        let mut slots = Vec::with_capacity(4);

        for found in (0..4).rev() {
            let mut from_the_end = EDGES - 1;

            while choose(from_the_end, found + 1) > left {
                from_the_end -= 1;
            }

            left -= choose(from_the_end, found + 1);
            slots.push(EDGES - 1 - from_the_end);
        }

        let mut slice = 8u8;
        let mut spare = 0u8;

        for slot in 0..EDGES {
            if slots.contains(&slot) {
                out.edge_at[slot] = slice;
                slice += 1;
            } else {
                out.edge_at[slot] = spare;
                spare += 1;
            }
        }

        out
    }

    /// The corners in order, as one number.
    pub fn corner_order(&self) -> u16 {
        rank(&self.corner_at) as u16
    }

    pub fn with_corner_order(n: u16) -> Cube {
        let mut out = Cube::SOLVED;

        out.corner_at = unrank::<CORNERS>(n as usize);
        out
    }

    /// The eight edges that are not the slice, in order. Only meaningful
    /// once the slice edges are home, which is what phase one is for.
    pub fn ud_edge_order(&self) -> u16 {
        let mut mine = [0u8; 8];

        mine.copy_from_slice(&self.edge_at[..8]);
        rank(&mine) as u16
    }

    pub fn with_ud_edge_order(n: u16) -> Cube {
        let mut out = Cube::SOLVED;
        let mine = unrank::<8>(n as usize);

        out.edge_at[..8].copy_from_slice(&mine);
        out
    }

    /// The four slice edges among themselves.
    pub fn slice_order(&self) -> u8 {
        let mut mine = [0u8; 4];

        for (slot, piece) in mine.iter_mut().enumerate() {
            *piece = self.edge_at[8 + slot] - 8;
        }

        rank(&mine) as u8
    }

    pub fn with_slice_order(n: u8) -> Cube {
        let mut out = Cube::SOLVED;
        let mine = unrank::<4>(n as usize);

        for (slot, &piece) in mine.iter().enumerate() {
            out.edge_at[8 + slot] = piece + 8;
        }

        out
    }
}

/// Where a run of distinct small numbers sits among all their orders.
///
/// Lehmer's, which is the one ranking of permutations that needs no table:
/// count how many of the numbers after each one are smaller than it, and
/// read the counts in factorial base.
fn rank<const N: usize>(of: &[u8; N]) -> usize {
    let mut out = 0;

    for n in 0..N {
        let mut smaller = 0;

        for m in n + 1..N {
            if of[m] < of[n] {
                smaller += 1;
            }
        }

        out = out * (N - n) + smaller;
    }

    out
}

fn unrank<const N: usize>(mut n: usize) -> [u8; N] {
    let mut counts = [0usize; N];

    for slot in (0..N).rev() {
        counts[slot] = n % (N - slot);
        n /= N - slot;
    }

    let mut left: Vec<u8> = (0..N as u8).collect();
    let mut out = [0u8; N];

    for (slot, &count) in counts.iter().enumerate() {
        out[slot] = left.remove(count);
    }

    out
}

/// The ten turns that stay inside the second phase's subgroup: any quarter
/// of the top and bottom, and half turns of the other four.
///
/// **This is the subgroup, written out.** Phase one's whole job is to reach
/// a cube these ten can finish, and phase two is a search over nothing but
/// these. It is also why two coordinates below are built over ten turns
/// rather than eighteen: outside this list a slice edge can leave the
/// slice, and once it has, "which order are the slice edges in" is not a
/// question with an answer.
pub const IN_THE_SUBGROUP: [usize; 10] = [0, 1, 2, 4, 7, 9, 10, 11, 13, 16];

/// What each turn does to each value of a coordinate.
///
/// **Generated, not copied.** The turn tables at the top of this file came
/// out of a book and needed six tests to say they arrived intact. These
/// come out of `Cube`, which those tests have already made good: make a
/// cube with that coordinate, turn it, read the coordinate back. There is
/// nothing here to transcribe wrong.
pub struct Table {
    pub over: Vec<u16>,
    pub turns: Vec<usize>,
    /// Which column a turn is, so a lookup is an index rather than a
    /// search. This was a `position` over the list, which reads well and
    /// sits in the innermost loop of every search and every breadth first
    /// sweep in the file.
    column: [usize; TURNS_IN_ALL],
}

impl Table {
    #[inline]
    pub fn at(&self, value: u16, turn: usize) -> u16 {
        self.over[value as usize * self.turns.len() + self.column[turn]]
    }

    /// How much memory it holds, which spec 0050 asks for by name.
    pub fn bytes(&self) -> usize {
        self.over.len() * std::mem::size_of::<u16>()
    }
}

fn tabled(
    count: usize,
    turns: &[usize],
    make: impl Fn(u16) -> Cube,
    read: impl Fn(&Cube) -> u16,
) -> Table {
    let mut over = vec![0u16; count * turns.len()];

    for value in 0..count {
        let cube = make(value as u16);

        for (which, &turn) in turns.iter().enumerate() {
            let (face, quarters) = turn_number(turn);

            over[value * turns.len() + which] = read(&cube.turned(face, quarters));
        }
    }

    let mut column = [usize::MAX; TURNS_IN_ALL];

    for (which, &turn) in turns.iter().enumerate() {
        column[turn] = which;
    }

    Table {
        over,
        turns: turns.to_vec(),
        column,
    }
}

/// Every turn there is, for the coordinates that mean something under all
/// of them.
fn every_turn() -> Vec<usize> {
    (0..TURNS_IN_ALL).collect()
}

pub fn twist_table() -> Table {
    tabled(TWISTS, &every_turn(), Cube::with_twist, |cube| cube.twist())
}

pub fn flip_table() -> Table {
    tabled(FLIPS, &every_turn(), Cube::with_flip, |cube| cube.flip())
}

pub fn slice_table() -> Table {
    tabled(SLICES, &every_turn(), Cube::with_slice, |cube| cube.slice())
}

pub fn corner_table() -> Table {
    tabled(
        CORNER_WAYS,
        &every_turn(),
        Cube::with_corner_order,
        |cube| cube.corner_order(),
    )
}

/// Over the subgroup's ten turns only, because outside them a slice edge
/// can leave the slice and the eight that are left are no longer eight.
pub fn ud_edge_table() -> Table {
    tabled(
        UD_EDGE_WAYS,
        &IN_THE_SUBGROUP,
        Cube::with_ud_edge_order,
        |cube| cube.ud_edge_order(),
    )
}

pub fn slice_order_table() -> Table {
    tabled(
        SLICE_WAYS,
        &IN_THE_SUBGROUP,
        |n| Cube::with_slice_order(n as u8),
        |cube| cube.slice_order() as u16,
    )
}

/// How far every state of a pair of coordinates is from home.
///
/// **A lower bound is the only thing a search can use.** The search needs
/// to know when a branch cannot possibly reach home inside the depth left,
/// and for that an estimate must never be too big. A distance over two
/// coordinates of the real cube is never too big by construction: the real
/// cube has to move both of them home and more besides, so whatever it
/// costs is at least this.
///
/// Breadth first from home, which fills every entry with the true distance
/// over that pair rather than a guess at it.
fn how_far(one: &Table, ones: usize, two: &Table, twos: usize, turns: &[usize]) -> Vec<u8> {
    let mut out = vec![u8::MAX; ones * twos];
    let mut edge = vec![0u32];

    out[0] = 0;

    let mut depth = 0u8;

    while !edge.is_empty() {
        let mut next = Vec::new();

        for &state in edge.iter() {
            let (a, b) = ((state as usize) / twos, (state as usize) % twos);

            for &turn in turns.iter() {
                let went = one.at(a as u16, turn) as usize * twos + two.at(b as u16, turn) as usize;

                if out[went] == u8::MAX {
                    out[went] = depth + 1;
                    next.push(went as u32);
                }
            }
        }

        edge = next;
        depth += 1;
    }

    out
}

/// Everything the solve needs, built once.
///
/// **Built rather than shipped, on the measurement.** Spec 0050 left that
/// open and asked for the numbers. `the_tables_say_what_they_cost` has
/// them, and `a_cold_start_is_measured` has the rest.
pub struct Solver {
    twist: Table,
    flip: Table,
    slice: Table,
    corner: Table,
    ud_edge: Table,
    slice_order: Table,
    /// how far from the subgroup, over the twist and the flip
    by_twist: Vec<u8>,
    by_flip: Vec<u8>,
    /// and how far from solved inside it
    by_corner: Vec<u8>,
    by_edge: Vec<u8>,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Solver {
        let twist = twist_table();
        let flip = flip_table();
        let slice = slice_table();
        let corner = corner_table();
        let ud_edge = ud_edge_table();
        let slice_order = slice_order_table();
        let every = every_turn();

        let by_twist = how_far(&twist, TWISTS, &slice, SLICES, &every);
        let by_flip = how_far(&flip, FLIPS, &slice, SLICES, &every);
        let by_corner = how_far(
            &corner,
            CORNER_WAYS,
            &slice_order,
            SLICE_WAYS,
            &IN_THE_SUBGROUP,
        );
        let by_edge = how_far(
            &ud_edge,
            UD_EDGE_WAYS,
            &slice_order,
            SLICE_WAYS,
            &IN_THE_SUBGROUP,
        );

        Solver {
            twist,
            flip,
            slice,
            corner,
            ud_edge,
            slice_order,
            by_twist,
            by_flip,
            by_corner,
            by_edge,
        }
    }

    /// What every table in here holds, which spec 0050 asks for by name.
    pub fn bytes(&self) -> usize {
        self.twist.bytes()
            + self.flip.bytes()
            + self.slice.bytes()
            + self.corner.bytes()
            + self.ud_edge.bytes()
            + self.slice_order.bytes()
            + self.by_twist.len()
            + self.by_flip.len()
            + self.by_corner.len()
            + self.by_edge.len()
    }

    /// Turns that bring this cube home, as turn numbers.
    ///
    /// **Two searches with a real cube between them.** The first drives the
    /// twist, the flip and the slice to nothing, which is the subgroup. The
    /// second works inside it. Between the two the moves are played onto an
    /// actual `Cube` and the second phase's coordinates are read off that,
    /// rather than carried through the first phase as coordinates nobody
    /// checked. It costs eighteen turns of a twenty piece struct and it
    /// means a coordinate slip in phase one cannot quietly become a wrong
    /// answer in phase two.
    pub fn solve(&self, cube: &Cube) -> Vec<usize> {
        if cube.solved() {
            return Vec::new();
        }

        let at = (cube.twist(), cube.flip(), cube.slice());

        // **The whole answer is bounded, not just each half.** Twenty
        // moves is enough for any cube, proved in 2010, so an answer of
        // twenty-three is a loose bound and a tight one is what makes this
        // finish. The first build let phase two have eighteen moves
        // whatever phase one had spent, and a bound that loose is a search
        // of a space with nothing in it: one solve took forty-five seconds
        // and came out at twenty-two moves. Bounding the total gives phase
        // two a cap of a dozen or so, which it either meets quickly or
        // fails at quickly, and failing quickly is what lets phase one go
        // and find a better place to hand over.
        for most in [23u8, 30] {
            for depth in self.at_least(at)..=12 {
                let mut found = Vec::new();

                if self.toward_the_subgroup(cube, at, depth, usize::MAX, &mut found, most) {
                    return found;
                }
            }
        }

        Vec::new()
    }

    /// Phase one, over coordinates and nothing else.
    ///
    /// **No cube is turned in here.** That is what the coordinates and
    /// their tables are for: a node is three numbers and a child is three
    /// lookups. The first build of this turned a real twenty piece cube at
    /// every node and read the rankings back off it, which is about four
    /// hundred operations a node where this is three, and a single solve
    /// did not finish in five minutes.
    ///
    /// A cube is turned once, at the moment the subgroup is reached, so
    /// that phase two starts from the building's own `Cube` rather than
    /// from coordinates that nothing has checked.
    fn toward_the_subgroup(
        &self,
        cube: &Cube,
        at: (u16, u16, u16),
        left: u8,
        last: usize,
        done: &mut Vec<usize>,
        most: u8,
    ) -> bool {
        if at == (0, 0, 0) {
            let mut went = *cube;

            for &turn in done.iter() {
                let (face, quarters) = turn_number(turn);

                went = went.turned(face, quarters);
            }

            let spare = most.saturating_sub(done.len() as u8);
            let mut rest = Vec::new();

            if self.inside_the_subgroup(&went, spare, &mut rest) {
                done.extend(rest);

                return true;
            }

            return false;
        }

        if left == 0 || self.at_least(at) > left {
            return false;
        }

        for turn in 0..TURNS_IN_ALL {
            if pointless(last, turn) {
                continue;
            }

            let next = (
                self.twist.at(at.0, turn),
                self.flip.at(at.1, turn),
                self.slice.at(at.2, turn),
            );

            done.push(turn);

            if self.toward_the_subgroup(cube, next, left - 1, turn, done, most) {
                return true;
            }

            done.pop();
        }

        false
    }

    /// How many turns this cube needs at the very least to reach the
    /// subgroup. Never too many, which is the only thing a search can use.
    fn at_least(&self, at: (u16, u16, u16)) -> u8 {
        let one = self.by_twist[at.0 as usize * SLICES + at.2 as usize];
        let two = self.by_flip[at.1 as usize * SLICES + at.2 as usize];

        one.max(two)
    }

    /// Phase two, deepening from what the tables already say it will cost
    /// rather than from nothing.
    fn inside_the_subgroup(&self, cube: &Cube, most: u8, done: &mut Vec<usize>) -> bool {
        let at = (
            cube.corner_order(),
            cube.ud_edge_order(),
            cube.slice_order() as u16,
        );

        for depth in self.still_to_go(at)..=most {
            if self.finish(at, depth, usize::MAX, done) {
                return true;
            }

            done.clear();
        }

        false
    }

    fn still_to_go(&self, at: (u16, u16, u16)) -> u8 {
        let one = self.by_corner[at.0 as usize * SLICE_WAYS + at.2 as usize];
        let two = self.by_edge[at.1 as usize * SLICE_WAYS + at.2 as usize];

        one.max(two)
    }

    fn finish(&self, at: (u16, u16, u16), left: u8, last: usize, done: &mut Vec<usize>) -> bool {
        if at == (0, 0, 0) {
            return true;
        }

        if left == 0 || self.still_to_go(at) > left {
            return false;
        }

        for &turn in IN_THE_SUBGROUP.iter() {
            if pointless(last, turn) {
                continue;
            }

            let next = (
                self.corner.at(at.0, turn),
                self.ud_edge.at(at.1, turn),
                self.slice_order.at(at.2, turn),
            );

            done.push(turn);

            if self.finish(next, left - 1, turn, done) {
                return true;
            }

            done.pop();
        }

        false
    }
}

/// Whether following `last` with `turn` is a move nobody needs to try.
///
/// Two turns of the same face in a row are one turn of that face, and the
/// search would find that one anyway. Two opposite faces commute, so the
/// pair comes up twice in either order and one of the two is enough: the
/// one with the lower face first.
fn pointless(last: usize, turn: usize) -> bool {
    if last == usize::MAX {
        return false;
    }

    let (before, now) = (Face::ALL[last / 3], Face::ALL[turn / 3]);

    before == now || (before == now.across() && last / 3 > turn / 3)
}

/// How big the whole cube is, across a face.
///
/// **Scaled to the room rather than to life.** A cube you could hold is
/// 57 millimetres and would be a speck on a bench 0.9 deep, which is the
/// same argument the Newton's cradle settled by taking a bench of 2.1.
/// Spec 0050 asks for the turning to be watchable from where you stand,
/// and 0.3 reads across the nook.
pub const ACROSS: f32 = 0.3;

/// The space between one cubie and the next, and so the black line you
/// see between the colours. Three cubies and two gaps make `ACROSS`.
pub const GAP: f32 = 0.004;

/// How far a sticker stands out from the cubie under it, and how far in
/// from its edges.
///
/// **Out, rather than flush.** Spec 0014: two drawn faces in one plane
/// speckle against each other as you move, and this building has hit that
/// seven times. A sticker laid on a cubie is that fault 54 times over, so
/// it stands off it, and the standing off is also what gives the colour
/// an edge to catch the light on.
pub const STICKER_OUT: f32 = 0.0015;
pub const STICKER_IN: f32 = 0.008;

/// A cubie's side, which is what is left of `ACROSS` after the gaps.
pub fn cubie() -> f32 {
    (ACROSS - GAP * 2.0) / 3.0
}

/// The six faces, in `Face::ALL`'s order: up, right, front, down, left,
/// back. Four hues to burn at the hall's own neon, and two that do not
/// burn at all.
///
/// **Neon against black and white.** Jake's, 2026-11-02.
///
/// **Saturated harder than the hall's own signs, and for a reason.** The
/// first pass used the ARCADE sign's exact pink and cyan, on the argument
/// that a cube glowing some other pink would be a stranger in the
/// building. The picture refuted it: those hues carry two channels near
/// one, and at 3.2 both clip, so a sign's thin tube reads as cyan while a
/// whole cube face reads as pale mint. A large glowing face keeps its
/// colour only if one channel does the glowing and the others stay under
/// one after the multiply, which means under about 0.31 before it.
const FACE_HUE: [glam::Vec3; 6] = [
    vec3(1.0, 1.0, 0.97),      // up, white
    vec3(1.0, 0.07, 0.34),     // right, pink
    vec3(0.04, 0.27, 1.0),     // front, blue
    vec3(0.055, 0.055, 0.065), // down, black
    vec3(0.10, 1.0, 0.16),     // left, green
    vec3(1.0, 0.40, 0.02),     // back, amber
];

/// How bright the white face is.
///
/// **Above one, because the nook is dim.** Colour here is an unclamped
/// multiplier: at one, a face is lit by the room and nothing else, and
/// four sconces at 0.55 make that a pale grey. The first picture of this
/// cube had a grey top for exactly that reason. Well under the neon
/// though, because what white is for here is being the bright thing that
/// is not glowing.
pub const WHITE_AT: f32 = 1.4;

/// What each face is drawn in.
///
/// A function rather than a table, so the four that glow are written as
/// the hall's own number rather than as that number already multiplied
/// out. `aim::NEON` is 3.2 and is the one place this room says how hot
/// its neon runs.
pub fn face_colour(n: usize) -> Vec3 {
    match n {
        0 => FACE_HUE[0] * WHITE_AT,
        3 => FACE_HUE[3],
        _ => FACE_HUE[n] * crate::aim::NEON,
    }
}

/// Which way each face looks, in the same order.
pub fn face_way(n: usize) -> Vec3 {
    [Vec3::Y, Vec3::X, Vec3::Z, -Vec3::Y, -Vec3::X, -Vec3::Z][n]
}

/// Every cubie of a cube standing with its middle here: twenty-six boxes,
/// the core left out because nobody can see it.
pub fn cubies(middle: Vec3) -> Vec<Aabb> {
    let side = cubie();
    let step = side + GAP;
    let mut out = Vec::with_capacity(26);

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if (x, y, z) == (0, 0, 0) {
                    continue;
                }

                let at = middle + vec3(x as f32, y as f32, z as f32) * step;

                out.push(Aabb::from_center_size(at, Vec3::splat(side)));
            }
        }
    }

    out
}

/// Every sticker on it, as a box and which face's colour it carries.
///
/// A cubie on the outside of the cube shows a sticker on each of its
/// faces that is on the outside, which is one for a middle, two for an
/// edge and three for a corner: nine on each of six faces, 54 in all.
///
/// This is the cube solved. What a scrambled one shows is the same
/// boxes with the colours moved about, which is the state's business and
/// not the geometry's.
pub fn stickers(middle: Vec3) -> Vec<(Aabb, usize)> {
    // the second of each pair is which of the fifty-four it is, not what
    // colour it is: the colour depends on how the cube is turned and is
    // `facelets`'s answer, read with this as the index
    let side = cubie();
    let step = side + GAP;
    let wide = side - STICKER_IN * 2.0;
    let mut out = Vec::with_capacity(54);

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if (x, y, z) == (0, 0, 0) {
                    continue;
                }

                let at = middle + vec3(x as f32, y as f32, z as f32) * step;

                let cell = vec3(x as f32, y as f32, z as f32);

                for n in 0..6 {
                    let way = face_way(n);

                    // a cubie carries a sticker on a face only when it is
                    // one of that face's nine, which is to say its
                    // coordinate along that way is the outer one
                    if cell.dot(way) < 0.5 {
                        continue;
                    }

                    let size = (Vec3::ONE - way.abs()) * wide + way.abs() * STICKER_OUT;

                    out.push((
                        Aabb::from_center_size(at + way * (side * 0.5 + STICKER_OUT * 0.5), size),
                        facelet_of((x, y, z), n),
                    ));
                }
            }
        }
    }

    out
}

/// Where each cubie slot sits, as a cell of the three by three, and which
/// faces its stickers are on.
///
/// **The order of the faces is the piece's own and is not arbitrary.** A
/// corner's three are read starting at the up or down face and going round
/// the corner the same way for every one of them, because that is what a
/// twist of one means: the sticker that was on the first face is now on
/// the second. Get the order wrong on one corner and that corner alone
/// shows the wrong colours whenever it is twisted.
const CORNER_CELL: [(i32, i32, i32); CORNERS] = [
    (1, 1, 1),
    (-1, 1, 1),
    (-1, 1, -1),
    (1, 1, -1),
    (1, -1, 1),
    (-1, -1, 1),
    (-1, -1, -1),
    (1, -1, -1),
];
const CORNER_FACE: [[usize; 3]; CORNERS] = [
    [0, 1, 2],
    [0, 2, 4],
    [0, 4, 5],
    [0, 5, 1],
    [3, 2, 1],
    [3, 4, 2],
    [3, 5, 4],
    [3, 1, 5],
];

const EDGE_CELL: [(i32, i32, i32); EDGES] = [
    (1, 1, 0),
    (0, 1, 1),
    (-1, 1, 0),
    (0, 1, -1),
    (1, -1, 0),
    (0, -1, 1),
    (-1, -1, 0),
    (0, -1, -1),
    (1, 0, 1),
    (-1, 0, 1),
    (-1, 0, -1),
    (1, 0, -1),
];
const EDGE_FACE: [[usize; 2]; EDGES] = [
    [0, 1],
    [0, 2],
    [0, 4],
    [0, 5],
    [3, 1],
    [3, 2],
    [3, 4],
    [3, 5],
    [2, 1],
    [2, 4],
    [5, 4],
    [5, 1],
];

/// Which of the fifty-four a cell's sticker on a given face is.
///
/// **Worked out rather than listed.** Every published account of this
/// writes out all fifty-four by hand, twice, once for corners and once for
/// edges. A face is a three by three read from outside it with up at the
/// top, so where a sticker falls on it is two of the cell's three numbers
/// with the signs the view puts them in, and that is all this is.
pub fn facelet_of(cell: (i32, i32, i32), face: usize) -> usize {
    let (x, y, z) = cell;
    let (row, col) = match face {
        0 => (z, x),
        1 => (-y, -z),
        2 => (-y, x),
        3 => (-z, x),
        4 => (-y, z),
        _ => (-y, -x),
    };

    face * 9 + ((row + 1) * 3 + (col + 1)) as usize
}

/// Which face's colour each of the fifty-four stickers carries, for this
/// cube.
///
/// The piece sitting in a slot brought its own colours with it, and its
/// twist says which of its faces ended up where.
pub fn facelets(cube: &Cube) -> [usize; 54] {
    let mut out = [0usize; 54];

    // the six centres, which never move and are what the faces mean
    for (n, face) in out.chunks_mut(9).enumerate() {
        face[4] = n;
    }

    for slot in 0..CORNERS {
        let piece = cube.corner_at[slot] as usize;
        let twist = cube.corner_spun[slot] as usize;

        for n in 0..3 {
            let here = facelet_of(CORNER_CELL[slot], CORNER_FACE[slot][(n + twist) % 3]);

            out[here] = CORNER_FACE[piece][n];
        }
    }

    for slot in 0..EDGES {
        let piece = cube.edge_at[slot] as usize;
        let flipped = cube.edge_flipped[slot] as usize;

        for n in 0..2 {
            let here = facelet_of(EDGE_CELL[slot], EDGE_FACE[slot][(n + flipped) % 2]);

            out[here] = EDGE_FACE[piece][n];
        }
    }

    out
}

/// How long one quarter turn takes on the bench.
///
/// **Slow enough to watch and no slower.** Spec 0050 settles that the
/// pace is a constant rather than a property of the algorithm: a solve is
/// twenty-odd turns, so this is twenty-odd seconds, which is the length
/// of the other things on that shelf. The gyroscope holds its axis for
/// about as long and the globe takes about as long to run down.
pub const TURNS_AT: f32 = 0.9;

/// How many turns a scramble is.
///
/// Past about twenty, more turns do not make a cube any more scrambled:
/// every position is twenty from home whatever you did to it.
pub const SCRAMBLE: u32 = 25;

/// The cube as the bench holds it: how it is turned, what it has left to
/// do, and the tables, once anybody has asked for them.
pub struct Toy {
    pub cube: Cube,
    /// **Built on first approach, not at load.** Spec 0050, on the
    /// measurement: 6.46 MB and 139 ms, against a wasm of 7.8. Nobody who
    /// never walks up to this bench pays for it, and nothing shipped can
    /// go stale, because it is made from `Cube` every time.
    solver: Option<Solver>,
    /// what is left to play, last first, so the next one is a pop
    left: Vec<usize>,
    /// the one it is in the middle of, and how far through it is
    doing: Option<(usize, f32)>,
    seed: u32,
}

impl Default for Toy {
    fn default() -> Self {
        Self::new()
    }
}

impl Toy {
    pub fn new() -> Toy {
        Toy {
            cube: Cube::SOLVED,
            solver: None,
            left: Vec::new(),
            doing: None,
            seed: 0x5eed,
        }
    }

    /// Whether it is in the middle of something.
    pub fn going(&self) -> bool {
        !self.left.is_empty() || self.doing.is_some()
    }

    /// The turn it is part way through: which face, how far round it has
    /// got in radians, and the sign the drawing turns by.
    ///
    /// **The short way round, not the way the move is written.** Three
    /// quarters clockwise is one quarter the other way, and a face that
    /// sweeps 270 degrees to end where a 90 would have put it is a thing
    /// nobody does to a cube. A half turn covers twice the angle in the
    /// same time, which is what keeps the pace one move a `TURNS_AT`
    /// however the move is written.
    pub fn part_way(&self) -> Option<(Face, f32)> {
        let (turn, through) = self.doing?;
        let (face, quarters) = turn_number(turn);
        let round = match quarters {
            1 => 1.0,
            2 => 2.0,
            _ => -1.0,
        };

        Some((face, through * round * std::f32::consts::FRAC_PI_2))
    }

    /// What the key does: scramble a solved cube, solve a scrambled one,
    /// and leave a cube that is already busy alone.
    ///
    /// **It never says you have won.** Spec 0050: the study is the one
    /// room not trying to sell you a game, so this has no score, keeps
    /// nothing, and the only thing it can tell you is what it is doing.
    pub fn worked(&mut self) {
        if self.going() {
            return;
        }

        if self.cube.solved() {
            for _ in 0..SCRAMBLE {
                self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
                self.left.push((self.seed >> 13) as usize % TURNS_IN_ALL);
            }

            return;
        }

        let solver = self.solver.get_or_insert_with(Solver::new);
        let mut moves = solver.solve(&self.cube);

        moves.reverse();
        self.left = moves;
    }

    /// One frame of it.
    ///
    /// A move is swept rather than snapped: it is taken off the list, run
    /// through over `TURNS_AT`, and only put into the cube's state at the
    /// end of it. Between those two the state is still what it was and
    /// `part_way` is what the drawing turns the face by, which means
    /// there is one account of where every piece is and the sweep is a
    /// way of looking at it rather than a second copy.
    pub fn advance(&mut self, dt: f32) {
        if self.doing.is_none() {
            match self.left.pop() {
                Some(turn) => self.doing = Some((turn, 0.0)),
                None => return,
            }
        }

        let Some((turn, through)) = self.doing.as_mut() else {
            return;
        };

        *through += dt / TURNS_AT;

        if *through < 1.0 {
            return;
        }

        let (face, quarters) = turn_number(*turn);

        self.cube = self.cube.turned(face, quarters);
        self.doing = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Read a sequence the way a cuber writes one: `R U R' U'`.
    ///
    /// Here rather than in the module because nothing in the game types a
    /// move, and a parser the game does not use is a parser nobody keeps
    /// working.
    fn done(moves: &str) -> Cube {
        let mut cube = Cube::SOLVED;

        for word in moves.split_whitespace() {
            let face = match &word[0..1] {
                "U" => Face::Up,
                "R" => Face::Right,
                "F" => Face::Front,
                "D" => Face::Down,
                "L" => Face::Left,
                "B" => Face::Back,
                other => panic!("{} is not a face", other),
            };
            let quarters = match &word[1..] {
                "" => 1,
                "2" => 2,
                "'" => 3,
                other => panic!("{} is not a number of quarters", other),
            };

            cube = cube.turned(face, quarters);
        }

        cube
    }

    /// Spec 0050: every coordinate is a bijection, walked end to end.
    ///
    /// **Not sampled.** Each of these is small enough to try every value,
    /// and a ranking that is wrong is usually wrong in one corner of its
    /// range: an off by one at the top, a carry that only bites once. The
    /// biggest of them is forty thousand and the whole test is instant, so
    /// there is no reason to guess.
    #[test]
    fn every_coordinate_goes_out_and_comes_back() {
        for n in 0..TWISTS as u16 {
            assert_eq!(Cube::with_twist(n).twist(), n, "twist {}", n);
        }

        for n in 0..FLIPS as u16 {
            assert_eq!(Cube::with_flip(n).flip(), n, "flip {}", n);
        }

        for n in 0..SLICES as u16 {
            assert_eq!(Cube::with_slice(n).slice(), n, "slice {}", n);
        }

        for n in 0..CORNER_WAYS as u16 {
            assert_eq!(
                Cube::with_corner_order(n).corner_order(),
                n,
                "corner order {}",
                n
            );
        }

        for n in 0..UD_EDGE_WAYS as u16 {
            assert_eq!(
                Cube::with_ud_edge_order(n).ud_edge_order(),
                n,
                "edge order {}",
                n
            );
        }

        for n in 0..SLICE_WAYS as u8 {
            assert_eq!(
                Cube::with_slice_order(n).slice_order(),
                n,
                "slice order {}",
                n
            );
        }
    }

    /// Spec 0050: twenty-six cubies, fifty-four stickers, nine a face.
    #[test]
    fn the_cube_is_the_shape_of_a_cube() {
        let at = vec3(1.0, 2.0, 3.0);
        let cubies = cubies(at);
        let stickers = stickers(at);

        assert_eq!(cubies.len(), 26, "cubies");
        assert_eq!(stickers.len(), 54, "stickers");

        for n in 0..6 {
            let mine = stickers.iter().filter(|(_, at)| at / 9 == n).count();

            assert_eq!(mine, 9, "face {} has {} stickers", n, mine);
        }

        // and each of the fifty-four is there once
        let mut seen = [false; 54];

        for (_, at) in stickers.iter() {
            assert!(!seen[*at], "two stickers are both facelet {}", at);
            seen[*at] = true;
        }

        assert!(seen.iter().all(|&hit| hit), "a facelet has no sticker");
    }

    /// Spec 0050: the key scrambles a solved cube and solves a scrambled
    /// one, and says nothing either way.
    #[test]
    fn working_it_scrambles_then_solves() {
        let mut toy = Toy::new();

        assert!(!toy.going(), "it starts busy");

        toy.worked();

        assert!(toy.going(), "working a solved cube did nothing");

        // a second press while it is going is ignored rather than queued
        let left = toy.left.len();

        toy.worked();

        assert_eq!(toy.left.len(), left, "pressing twice queued more");

        for _ in 0..60 * 60 {
            toy.advance(1.0 / 60.0);
        }

        assert!(!toy.going(), "the scramble never finished");
        assert!(!toy.cube.solved(), "it scrambled to nothing");

        toy.worked();

        for _ in 0..60 * 120 {
            toy.advance(1.0 / 60.0);
        }

        assert!(!toy.going(), "the solve never finished");
        assert!(toy.cube.solved(), "it did not come home");
    }

    /// Spec 0050: it turns at a pace somebody can watch.
    ///
    /// The pace is the whole of why this is worth standing in front of
    /// rather than a blur, so it is worth asking that it takes about as
    /// long as it should: a scramble is `SCRAMBLE` turns at `TURNS_AT`
    /// each.
    #[test]
    fn it_turns_at_a_pace_somebody_can_watch() {
        let mut toy = Toy::new();
        let step = 1.0 / 60.0;
        let mut took = 0.0;

        toy.worked();

        while toy.going() && took < 120.0 {
            toy.advance(step);
            took += step;
        }

        let wants = SCRAMBLE as f32 * TURNS_AT;

        assert!(
            (took - wants).abs() < wants * 0.1,
            "a scramble of {} turns took {:.1} seconds and not about {:.1}",
            SCRAMBLE,
            took,
            wants
        );
    }

    /// Spec 0050: a solved cube wears each face's own colour.
    #[test]
    fn a_solved_cube_is_six_plain_faces() {
        let on = facelets(&Cube::SOLVED);

        for (n, &colour) in on.iter().enumerate() {
            assert_eq!(colour, n / 9, "facelet {} carries face {}", n, colour);
        }
    }

    /// Spec 0050: turning moves colours about and never makes or loses
    /// one.
    ///
    /// Nine of each is what a cube is. A mapping that reads a twist the
    /// wrong way round still puts a colour on every sticker, and the count
    /// is what notices.
    #[test]
    fn there_are_always_nine_of_each_colour() {
        let mut cube = Cube::SOLVED;
        let mut seed = 0xfaceu32;

        for _ in 0..500 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            cube = cube.turned(Face::ALL[(seed >> 16) as usize % 6], 1);

            let on = facelets(&cube);
            let mut how_many = [0usize; 6];

            for &colour in on.iter() {
                how_many[colour] += 1;
            }

            assert_eq!(how_many, [9; 6], "the colours came out {:?}", how_many);

            for n in 0..6 {
                assert_eq!(on[n * 9 + 4], n, "the centre of face {} moved", n);
            }
        }
    }

    /// Spec 0050: one quarter turn changes twelve stickers and no more.
    ///
    /// **A fact about a cube rather than about this code.** Turning the
    /// top moves the top row of the four faces round it, which is twelve,
    /// and the top face's own nine come back the same colour because they
    /// were all that colour. So twelve is the whole of what a watcher
    /// sees, and a mapping that is out by a rotation somewhere changes a
    /// different number.
    #[test]
    fn a_quarter_turn_changes_twelve_stickers() {
        for face in Face::ALL {
            let before = facelets(&Cube::SOLVED);
            let after = facelets(&Cube::SOLVED.turned(face, 1));
            let moved = before
                .iter()
                .zip(after.iter())
                .filter(|(one, two)| one != two)
                .count();

            assert_eq!(moved, 12, "{:?} changed {} stickers", face, moved);
        }
    }

    /// Spec 0050: and what the solver hands back puts them all home.
    ///
    /// The mapping and the solver held against each other, which is the
    /// pair the bench actually runs.
    #[test]
    fn solving_a_scramble_puts_every_colour_home() {
        let solver = Solver::new();

        for seed in 1..12u32 {
            let cube = scrambled(seed, 20);
            let mut went = cube;

            for &turn in solver.solve(&cube).iter() {
                let (face, quarters) = turn_number(turn);

                went = went.turned(face, quarters);
            }

            assert_eq!(
                facelets(&went),
                facelets(&Cube::SOLVED),
                "seed {} solved to a cube that does not look solved",
                seed
            );
        }
    }

    /// Spec 0050: it is `ACROSS` across, and the cubies do not touch.
    ///
    /// **The gap is the black line you see between the colours**, so it
    /// being there is not a detail. Cubies that touch are a cube with no
    /// lines on it, which reads as a painted block.
    #[test]
    fn the_cubies_make_one_cube_without_touching() {
        let at = Vec3::ZERO;
        let cubies = cubies(at);
        let (mut low, mut high) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));

        for box_ in cubies.iter() {
            low = low.min(box_.min);
            high = high.max(box_.max);
        }

        assert!(
            (high - low - Vec3::splat(ACROSS)).abs().max_element() < 1e-5,
            "it measures {:?} and not {} across",
            high - low,
            ACROSS
        );

        for (n, one) in cubies.iter().enumerate() {
            for two in cubies[n + 1..].iter() {
                assert!(
                    !one.intersects(two),
                    "two cubies touch at {:?} and {:?}",
                    one.center(),
                    two.center()
                );
            }
        }
    }

    /// Spec 0014: no sticker lies in the same plane as the cubie under it.
    ///
    /// **This building has hit that seven times** and a cube laid flush is
    /// the fault fifty-four times over: two drawn faces in one plane
    /// speckle against each other as the camera moves, and no single frame
    /// of it looks wrong. So every sticker stands off its cubie, and what
    /// this asks is that the standing off is real on the axis it stands
    /// along and that the sticker is inside the cubie's outline on the
    /// other two.
    #[test]
    fn no_sticker_lies_in_its_cubie_s_face() {
        let at = Vec3::ZERO;
        let cubies = cubies(at);

        for (sticker, facelet) in stickers(at).iter() {
            let face = facelet / 9;
            let way = face_way(face);
            let under = cubies
                .iter()
                .min_by(|one, two| {
                    one.center()
                        .distance(sticker.center())
                        .total_cmp(&two.center().distance(sticker.center()))
                })
                .expect("a cubie under it");

            // clear of the face it sits on, along the way it faces
            let stands = (sticker.center() - under.center()).dot(way) - cubie() * 0.5;

            assert!(
                stands > 1e-6,
                "a sticker on face {} sits {:.5} into its cubie",
                face,
                -stands
            );

            // and inside the outline on the other two, so no edge of it
            // shares a plane with an edge of the cubie either
            let flat = Vec3::ONE - way.abs();
            let room = (under.size() - sticker.size()) * flat;

            assert!(
                room.max_element() > 1e-6 && room.min_element() >= -1e-6,
                "a sticker on face {} is not inside its cubie's outline",
                face
            );
        }
    }

    /// Spec 0050: four faces burn at the hall's neon and two do not burn.
    ///
    /// **Colour here is an unclamped multiplier**, so what a number past
    /// one buys is a face that is bright with no light on it. The first
    /// version of these sat at 1.9, which I argued for and a picture
    /// refuted: under four sconces at 0.55 it read as paint. The four are
    /// the hall's own 3.2 now.
    ///
    /// What keeps it a cube rather than a lantern is the other two. White
    /// is above one, because at one the nook's light is all it has and
    /// that came out grey, and well under the neon, because being the
    /// bright thing that is not glowing is its whole job. Black does not
    /// glow at all.
    #[test]
    fn four_faces_burn_and_two_do_not() {
        let hot: Vec<f32> = (0..6).map(|n| face_colour(n).max_element()).collect();
        let burning = hot.iter().filter(|&&n| n >= crate::aim::NEON).count();

        assert_eq!(burning, 4, "{} of the six burn at the hall's neon", burning);
        assert!(
            hot[0] > 1.0 && hot[0] < crate::aim::NEON * 0.5,
            "the white face is at {:.2}",
            hot[0]
        );
        assert!(hot[3] < 1.0, "the black face is at {:.2}", hot[3]);
    }

    /// Spec 0050: a solved cube is nought in every coordinate.
    ///
    /// **Which a round trip cannot see.** A ranking can be a perfect
    /// bijection and still put home somewhere other than nought, and every
    /// search in this file stops at nought. The slice coordinate was
    /// counted from the near end and read 494 for a solved cube: phase one
    /// drove the four slice edges into the top of the cube instead of into
    /// the slice, and only the test that solves a cube and then looks at it
    /// said so.
    #[test]
    fn a_solved_cube_is_nought_in_every_coordinate() {
        let cube = Cube::SOLVED;

        assert_eq!(cube.twist(), 0, "twist");
        assert_eq!(cube.flip(), 0, "flip");
        assert_eq!(cube.slice(), 0, "slice");
        assert_eq!(cube.corner_order(), 0, "corner order");
        assert_eq!(cube.ud_edge_order(), 0, "edge order");
        assert_eq!(cube.slice_order(), 0, "slice order");
    }

    /// Spec 0050: and no two states share a number.
    ///
    /// The other half of a bijection, and the half the round trip above
    /// cannot see: a ranking that sends two different cubes to the same
    /// number passes every round trip and loses the solver a branch.
    #[test]
    fn no_two_states_share_a_coordinate() {
        let mut seen = vec![false; CORNER_WAYS];

        for n in 0..CORNER_WAYS {
            let at = super::unrank::<CORNERS>(n);
            let got = super::rank(&at);

            assert!(!seen[got], "two orders both rank {}", got);
            seen[got] = true;
        }

        assert!(seen.iter().all(|&hit| hit), "some rank is never reached");
    }

    /// Spec 0050: a cube built from a coordinate has that coordinate and
    /// disturbs nothing else.
    ///
    /// **Because the table builders lean on it.** A move table is built by
    /// making a cube from a number, turning it and reading the number back,
    /// and that is only the move's own effect if the cube carried nothing
    /// else into the turn.
    #[test]
    fn a_cube_from_one_coordinate_carries_nothing_else() {
        for n in [0u16, 1, 100, (TWISTS - 1) as u16] {
            let cube = Cube::with_twist(n);

            assert_eq!(
                cube.corner_at,
                Cube::SOLVED.corner_at,
                "twist moved corners"
            );
            assert_eq!(cube.edge_at, Cube::SOLVED.edge_at, "twist moved edges");
            assert_eq!(cube.flip(), 0, "twist flipped an edge");
        }

        for n in [0u16, 1, 100, (SLICES - 1) as u16] {
            let cube = Cube::with_slice(n);

            assert_eq!(cube.twist(), 0, "slice twisted a corner");
            assert_eq!(cube.flip(), 0, "slice flipped an edge");
            assert_eq!(
                cube.corner_at,
                Cube::SOLVED.corner_at,
                "slice moved corners"
            );
        }
    }

    /// A scrambled cube, from a seed, so a failure can be gone back to.
    fn scrambled(seed: u32, turns: usize) -> Cube {
        let mut cube = Cube::SOLVED;
        let mut seed = seed | 1;

        for _ in 0..turns {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);

            let (face, quarters) = turn_number((seed >> 13) as usize % TURNS_IN_ALL);

            cube = cube.turned(face, quarters);
        }

        cube
    }

    /// Spec 0050: a coordinate's table holds whatever else the cube is
    /// doing.
    ///
    /// **Which is the whole claim a coordinate makes, and it is not the one
    /// the builder makes.** Building a table by making a cube from a
    /// number, turning it and reading the number back proves nothing: it is
    /// true by construction. What has to be true is that the answer does
    /// not depend on the rest of the cube, so a table built from bare cubes
    /// is still right about a scrambled one. A coordinate that fails this
    /// is not a coordinate, and the solver would search a graph that is not
    /// the cube's.
    #[test]
    fn a_coordinate_does_not_care_what_else_the_cube_is_doing() {
        let twist = twist_table();
        let flip = flip_table();
        let slice = slice_table();
        let corner = corner_table();

        for seed in 1..200u32 {
            let cube = scrambled(seed, 12);

            for n in 0..TURNS_IN_ALL {
                let (face, quarters) = turn_number(n);
                let went = cube.turned(face, quarters);

                assert_eq!(twist.at(cube.twist(), n), went.twist(), "twist, turn {}", n);
                assert_eq!(flip.at(cube.flip(), n), went.flip(), "flip, turn {}", n);
                assert_eq!(slice.at(cube.slice(), n), went.slice(), "slice, turn {}", n);
                assert_eq!(
                    corner.at(cube.corner_order(), n),
                    went.corner_order(),
                    "corner order, turn {}",
                    n
                );
            }
        }
    }

    /// Spec 0050: and the subgroup's two coordinates hold inside it.
    ///
    /// Scrambled with the subgroup's own turns, because that is the only
    /// place they mean anything, which is the reason their tables are ten
    /// turns wide rather than eighteen.
    #[test]
    fn the_subgroup_coordinates_hold_inside_the_subgroup() {
        let edges = ud_edge_table();
        let slice = slice_order_table();
        let mut seed = 7u32;

        for _ in 0..200 {
            let mut cube = Cube::SOLVED;

            for _ in 0..12 {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);

                let (face, quarters) =
                    turn_number(IN_THE_SUBGROUP[(seed >> 13) as usize % IN_THE_SUBGROUP.len()]);

                cube = cube.turned(face, quarters);
            }

            for &n in IN_THE_SUBGROUP.iter() {
                let (face, quarters) = turn_number(n);
                let went = cube.turned(face, quarters);

                assert_eq!(
                    edges.at(cube.ud_edge_order(), n),
                    went.ud_edge_order(),
                    "edge order, turn {}",
                    n
                );
                assert_eq!(
                    slice.at(cube.slice_order() as u16, n),
                    went.slice_order() as u16,
                    "slice order, turn {}",
                    n
                );
            }
        }
    }

    /// Spec 0050 asks for the tables' size and how long they take from
    /// cold, because the answer decides whether they ship or are built when
    /// the room loads. This is where that number comes from.
    #[test]
    fn the_tables_say_what_they_cost() {
        let began = std::time::Instant::now();
        let all = [
            ("twist", twist_table()),
            ("flip", flip_table()),
            ("slice", slice_table()),
            ("corner order", corner_table()),
            ("edge order", ud_edge_table()),
            ("slice order", slice_order_table()),
        ];
        let took = began.elapsed();
        let bytes: usize = all.iter().map(|(_, table)| table.bytes()).sum();

        for (name, table) in all.iter() {
            println!("  {:<12} {:>9} bytes", name, table.bytes());
        }

        println!(
            "  {:<12} {:>9} bytes, built in {} ms",
            "all six",
            bytes,
            took.as_millis()
        );

        assert!(
            bytes < 8 * 1024 * 1024,
            "the move tables alone are {} bytes, which is a download",
            bytes
        );
    }

    /// Spec 0050: a scramble goes in and a solved cube comes out.
    ///
    /// **The one that matters.** Everything else in this file checks a
    /// piece; this checks the answer. A wrong table, a wrong coordinate, a
    /// pruning value that is too big and cuts off the only way home: all of
    /// them end here as a cube that is not solved.
    #[test]
    fn every_scramble_comes_out() {
        let solver = Solver::new();
        let mut longest = 0;
        let mut total = 0;
        let mut how_many = 0;

        for seed in 1..60u32 {
            let cube = scrambled(seed, 25);

            assert!(!cube.solved(), "seed {} scrambled to nothing", seed);

            let moves = solver.solve(&cube);
            let mut went = cube;

            for &turn in moves.iter() {
                let (face, quarters) = turn_number(turn);

                went = went.turned(face, quarters);
            }

            assert!(
                went.solved(),
                "seed {} came out in {} moves and is not solved",
                seed,
                moves.len()
            );

            longest = longest.max(moves.len());
            total += moves.len();
            how_many += 1;
        }

        println!(
            "  {} scrambles solved, {} moves on average, {} at worst",
            how_many,
            total / how_many,
            longest
        );
    }

    /// Spec 0050: a solved cube needs no moves.
    #[test]
    fn a_solved_cube_needs_no_moves() {
        assert!(Solver::new().solve(&Cube::SOLVED).is_empty());
    }

    /// Spec 0050: every move it hands back is a legal turn of one face.
    #[test]
    fn nothing_but_turns_comes_out() {
        let solver = Solver::new();

        for seed in 1..20u32 {
            for &turn in solver.solve(&scrambled(seed, 20)).iter() {
                assert!(turn < TURNS_IN_ALL, "{} is not a turn", turn);

                let (_, quarters) = turn_number(turn);

                assert!(
                    (1..=3).contains(&quarters),
                    "{} quarters is not a turn",
                    quarters
                );
            }
        }
    }

    /// Spec 0050: the second phase keeps to its subgroup.
    ///
    /// Taken from the answer rather than from the code: once the cube is in
    /// the subgroup, every turn after that has to be one of the ten, or the
    /// cube leaves it again and the phase is not a phase.
    #[test]
    fn the_second_phase_keeps_to_its_subgroup() {
        let solver = Solver::new();

        for seed in 1..20u32 {
            let cube = scrambled(seed, 20);
            let moves = solver.solve(&cube);
            let mut went = cube;
            let mut arrived = false;

            for &turn in moves.iter() {
                if arrived {
                    assert!(
                        IN_THE_SUBGROUP.contains(&turn),
                        "turn {} is not one the second phase may use",
                        turn
                    );
                }

                let (face, quarters) = turn_number(turn);

                went = went.turned(face, quarters);
                arrived = arrived || (went.twist() == 0 && went.flip() == 0 && went.slice() == 0);
            }
        }
    }

    /// Spec 0050 asks how long the whole thing takes from cold, because
    /// that is what decides whether it is built when the room loads.
    #[test]
    fn a_cold_start_is_measured() {
        let began = std::time::Instant::now();
        let solver = Solver::new();
        let built = began.elapsed();

        let began = std::time::Instant::now();
        let moves = solver.solve(&scrambled(99, 25));
        let solved = began.elapsed();

        println!(
            "  built in {} ms, holding {} bytes; one solve of {} moves took {} ms",
            built.as_millis(),
            solver.bytes(),
            moves.len(),
            solved.as_millis()
        );
    }

    /// Spec 0050: four quarter turns of a face is no turn at all.
    ///
    /// The weakest of these and the first to fail on a table that is not a
    /// permutation at all.
    #[test]
    fn four_of_any_turn_comes_home() {
        for face in Face::ALL {
            let mut cube = Cube::SOLVED;

            for n in 1..=4 {
                cube = cube.turned(face, 1);

                assert_eq!(
                    cube.solved(),
                    n == 4,
                    "{:?} turned {} times is {}solved",
                    face,
                    n,
                    if cube.solved() { "" } else { "not " }
                );
            }
        }
    }

    /// Spec 0050: opposite faces do not disturb each other, and adjacent
    /// ones do.
    ///
    /// **Both halves, because the first alone passes on a table that moves
    /// nothing.** The second phase of the solve is built on exactly this
    /// property of the two faces it keeps quarter turns for.
    #[test]
    fn only_opposite_faces_leave_each_other_alone() {
        for one in Face::ALL {
            for two in Face::ALL {
                if one == two {
                    continue;
                }

                let there = Cube::SOLVED.turned(one, 1).turned(two, 1);
                let back = Cube::SOLVED.turned(two, 1).turned(one, 1);

                assert_eq!(
                    there == back,
                    two == one.across(),
                    "{:?} and {:?} {} each other",
                    one,
                    two,
                    if there == back {
                        "leave alone"
                    } else {
                        "disturb"
                    }
                );
            }
        }
    }

    /// Spec 0050: the twists stay divisible, whatever is done to it.
    ///
    /// A cube's corners always sum to a multiple of three turns and its
    /// edges to an even number of flips. No sequence of face turns can
    /// break that, so a table whose twists are wrong says so here even when
    /// every piece still lands in a legal place.
    #[test]
    fn the_twists_stay_divisible() {
        let mut cube = Cube::SOLVED;
        let mut seed = 0x5eedu32;

        for n in 0..2000 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);

            let face = Face::ALL[(seed >> 16) as usize % 6];

            cube = cube.turned(face, 1 + (seed >> 8) as u8 % 3);

            let spun: u32 = cube.corner_spun.iter().map(|&n| n as u32).sum();
            let flipped: u32 = cube.edge_flipped.iter().map(|&n| n as u32).sum();

            assert_eq!(spun % 3, 0, "after {} turns the corners sum to {}", n, spun);
            assert_eq!(
                flipped % 2,
                0,
                "after {} turns the edges sum to {}",
                n,
                flipped
            );
        }
    }

    /// Spec 0050: six of the sexy move is a solved cube.
    ///
    /// **A fact about a cube rather than about this code**, which is what
    /// makes it worth writing down. `R U R' U'` has order six, and reaching
    /// that needs the places, the twists, the flips and the inverses all
    /// right at once.
    ///
    /// It is the weakest of these in practice, which was checked rather
    /// than assumed. Dropping a flip from F and putting a twist wrong in R
    /// are both caught by the four turn test and the divisibility one, and
    /// this one passes through both of them: six of a wrong move can still
    /// come home. It stays because it is the only check here that is a
    /// claim about cubes, and whatever it catches will be something the
    /// structural tests cannot see.
    #[test]
    fn six_of_the_sexy_move_comes_home() {
        for n in 1..=6 {
            let cube = done(&"R U R' U' ".repeat(n));

            assert_eq!(
                cube.solved(),
                n == 6,
                "R U R' U' done {} times is {}solved",
                n,
                if cube.solved() { "" } else { "not " }
            );
        }
    }

    /// Spec 0050: a scramble undone leaves every piece home.
    ///
    /// Which is the shape of what the bench does, with the solver's answer
    /// standing in for the reversed scramble.
    #[test]
    fn a_scramble_walked_backwards_is_undone() {
        let scramble = "R U2 F' L D B2 R' U L2 F D' B R2 U' L' F2";
        let mut cube = done(scramble);

        assert!(!cube.solved(), "that scramble left it solved");

        for word in scramble.split_whitespace().rev() {
            let face = match &word[0..1] {
                "U" => Face::Up,
                "R" => Face::Right,
                "F" => Face::Front,
                "D" => Face::Down,
                "L" => Face::Left,
                _ => Face::Back,
            };
            let back = match &word[1..] {
                "" => 3,
                "2" => 2,
                _ => 1,
            };

            cube = cube.turned(face, back);
        }

        assert!(cube.solved(), "it did not come home");
    }

    /// Spec 0050: every piece is somewhere, and only once.
    ///
    /// The place arrays are permutations or they are nothing, and a table
    /// copied with a digit wrong is usually not one.
    #[test]
    fn every_piece_is_somewhere_and_only_once() {
        let mut cube = Cube::SOLVED;
        let mut seed = 0xc0ffeeu32;

        for _ in 0..500 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            cube = cube.turned(Face::ALL[(seed >> 16) as usize % 6], 1);

            let mut corners = cube.corner_at;
            let mut edges = cube.edge_at;

            corners.sort_unstable();
            edges.sort_unstable();

            assert_eq!(corners, Cube::SOLVED.corner_at, "a corner is doubled");
            assert_eq!(edges, Cube::SOLVED.edge_at, "an edge is doubled");
        }
    }
}
