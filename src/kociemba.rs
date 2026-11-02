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
