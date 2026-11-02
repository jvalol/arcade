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
