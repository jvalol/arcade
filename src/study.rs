//! What makes the nook a study rather than more arcade: bookcases along the
//! ends of it, stocked, and candles on them. Spec 0006.
//!
//! None of it does anything. That is the point of it. The room outside is a dim
//! hall of neon and confetti, and the nook is the one place in the building that
//! is not trying to sell you a game, so it is furnished rather than lit up: dark
//! wood, filled shelves, a rug and candlelight.
//!
//! Where every shelf and every book stands is arithmetic and is checked without
//! a window.

use glam::{vec3, Vec3};

/// A bookcase: how wide across its face, how deep, how tall, and how thick its
/// boards are.
pub const CASE: Vec3 = vec3(1.1, 0.32, 1.9);
pub const BOARD: f32 = 0.035;

/// How many shelves it carries, counting the top and not the floor.
pub const SHELVES: usize = 5;

/// A book: how tall the tallest and shortest stand, how thick the widest and
/// thinnest are, and how far the row is set back from the front edge.
pub const BOOK_TALL: (f32, f32) = (0.17, 0.27);
pub const BOOK_THICK: (f32, f32) = (0.018, 0.045);
pub const BOOK_BACK: f32 = 0.06;

/// How much of a shelf is left empty at its ends.
pub const SHELF_SPARE: f32 = 0.04;

/// A sconce on the wall: how far out the bracket reaches, how big the shade is,
/// and how high it hangs.
///
/// Candles were tried first, standing on the shelves among the books. They were
/// pretty and they are the wrong thing: there is no lavishly furnished room in
/// the world with an open flame two inches from four hundred books, and a room
/// that makes you think about that is a room nobody relaxes in.
///
/// A sconce also answers the thing the candles never did, which is where the
/// light is coming from. The nook had two lamps in the ceiling and nothing to
/// hang them on, so what you saw was a bright patch on a surface and no reason
/// for it.
pub const SCONCE_OUT: f32 = 0.1;
pub const SCONCE_BACK: f32 = 0.04;
pub const SHADE: Vec3 = vec3(0.09, 0.13, 0.16);
pub const SCONCE_UP: f32 = 1.95;

/// How far apart the sconces are along a wall.
pub const SCONCE_EVERY: f32 = 2.2;

/// How bright one burns and how far it reaches.
///
/// Calibrated against lantern's candle, which is 0.5 over a range of 9. These
/// are wall lights in a panelled room rather than the one flame in a dark yard,
/// so they are softer and they do not reach as far.
pub const SCONCE_LIT: f32 = 0.55;
pub const SCONCE_RANGE: f32 = 4.2;
pub const SCONCE_COLOUR: Vec3 = vec3(1.0, 0.82, 0.56);

/// Where along a wall the sconces hang.
pub fn sconces(from: f32, to: f32) -> Vec<f32> {
    let run = to - from;
    let fits = (run / SCONCE_EVERY).floor().max(1.0);
    let along = fits * SCONCE_EVERY;
    let first = from + (run - along) * 0.5 + SCONCE_EVERY * 0.5;

    (0..fits as usize)
        .map(|n| first + n as f32 * SCONCE_EVERY)
        .collect()
}

/// Panelling: how deep the skirting and the cornice are, how high the dado rail
/// runs and how thick it is, and how far the panels stand proud of the wall.
///
/// Panelled below the rail and plain above it, which is what a panelled room is.
/// The walls were the same brown grain as the hall outside, and a wall is the
/// largest flat thing in any view of a room after the floor: it was the one
/// surface left in here still saying arcade.
pub const SKIRTING: f32 = 0.16;
pub const DADO: f32 = 0.98;
pub const RAIL: f32 = 0.07;
pub const CORNICE: f32 = 0.14;
pub const PROUD: f32 = 0.02;

/// A panel: how wide one is and how much stile is left between two.
pub const PANEL_WIDE: f32 = 0.74;
pub const PANEL_GAP: f32 = 0.08;

/// How far in from the skirting and the rail a panel's face is held.
pub const PANEL_INSET: f32 = 0.07;

/// Where the panels sit along a wall, as the middle of each.
///
/// Centred on the run, so a wall that did not divide evenly is short at both
/// ends rather than all at one, and the stiles at the corners match.
pub fn panels(from: f32, to: f32) -> Vec<f32> {
    let step = PANEL_WIDE + PANEL_GAP;
    let run = to - from;
    if run < step {
        return Vec::new();
    }

    let fits = (run / step).floor();
    let along = fits * step;
    let first = from + (run - along) * 0.5 + step * 0.5;

    (0..fits as usize)
        .map(|n| first + n as f32 * step)
        .collect()
}

/// How tall a panel stands, between the skirting and the rail.
pub fn panel_tall() -> f32 {
    DADO - RAIL * 0.5 - SKIRTING - PANEL_INSET * 2.0
}

/// How high its middle is.
pub fn panel_up() -> f32 {
    SKIRTING + PANEL_INSET + panel_tall() * 0.5
}

/// A number from a number, so the same shelf is stocked every time.
///
/// A bookcase restocked at random every frame is a bookcase that boils, and one
/// stocked from the clock is one nobody can write a test about.
fn from(seed: u32) -> u32 {
    let mut n = seed.wrapping_mul(0x9E37_79B9);
    n ^= n >> 15;
    n = n.wrapping_mul(0x85EB_CA6B);
    n ^= n >> 13;

    n
}

/// The spines, which are what a stocked shelf actually is from across a room.
///
/// Cloth and leather: oxblood, tan, green, navy, and the brown of a book nobody
/// has opened in years. No white, because a white spine reads as a gap.
const SPINES: [[f32; 3]; 7] = [
    [0.42, 0.14, 0.13],
    [0.52, 0.36, 0.18],
    [0.17, 0.30, 0.20],
    [0.14, 0.19, 0.34],
    [0.34, 0.22, 0.14],
    [0.46, 0.30, 0.12],
    [0.24, 0.13, 0.16],
];

/// What is on the spines.
///
/// Real books, all of them long out of copyright, which is the only kind a
/// room can be filled with four hundred of. Nothing invented: a shelf of
/// plausible-sounding titles is a shelf you read twice and stop believing,
/// and the whole point of this room is that it is the one place in the
/// building not pretending at you.
///
/// Short ones, mostly. A spine is three centimetres across and the title is
/// scaled to fit it, so `Emma` is legible from where you stand and
/// `The Life and Opinions of Tristram Shandy, Gentleman` would be a grey
/// smear however it was set. Real spines solve this the same way, by being
/// chosen or abbreviated, so the long ones here are the ones a binder would
/// actually have fitted.
///
/// ASCII only. The font is the engine's and an accent it has no glyph for
/// comes out as a hole, so `Les Miserables` is not here rather than being
/// here wrongly.
pub const TITLES: [&str; 56] = [
    "Moby-Dick",
    "Walden",
    "Emma",
    "Persuasion",
    "Middlemarch",
    "Bleak House",
    "Hard Times",
    "Jane Eyre",
    "Villette",
    "Dracula",
    "Frankenstein",
    "Kidnapped",
    "Treasure Island",
    "The Time Machine",
    "Heart of Darkness",
    "Lord Jim",
    "Silas Marner",
    "Adam Bede",
    "Cranford",
    "North and South",
    "Agnes Grey",
    "Shirley",
    "Tom Jones",
    "Moll Flanders",
    "Robinson Crusoe",
    "Paradise Lost",
    "The Odyssey",
    "The Iliad",
    "The Aeneid",
    "Metamorphoses",
    "On Liberty",
    "Leviathan",
    "The Prince",
    "Utopia",
    "Leaves of Grass",
    "The Scarlet Letter",
    "Great Expectations",
    "Oliver Twist",
    "David Copperfield",
    "Wuthering Heights",
    "Vanity Fair",
    "The Moonstone",
    "The Woman in White",
    "Sense and Sensibility",
    "Pride and Prejudice",
    "Mansfield Park",
    "Northanger Abbey",
    "The Warden",
    "Barchester Towers",
    "Daniel Deronda",
    "The Mill on the Floss",
    "Anna Karenina",
    "War and Peace",
    "Dead Souls",
    "Don Quixote",
    "Candide",
];

/// How tall the lettering on a spine is drawn, in texels.
///
/// The picture is made once a title and stretched onto every copy of it, so
/// this is about how it reads from a pace away rather than about memory.
pub const TITLE_TEXELS: f32 = 48.0;

/// How much of a spine the title is allowed: along the book, and across it.
pub const TITLE_LONG: f32 = 0.74;
pub const TITLE_ACROSS: f32 = 0.62;

/// One book: how thick it is, how tall it stands, and what its spine is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Book {
    pub thick: f32,
    pub tall: f32,
    pub spine: [f32; 3],
    /// Which of [`TITLES`] is on it.
    pub title: usize,
}

/// Where a shelf's boards sit above the bookcase's foot.
pub fn shelf_at(shelf: usize) -> f32 {
    let step = (CASE.z - BOARD) / SHELVES as f32;

    BOARD + shelf as f32 * step
}

/// How much room a book has to stand up in, on a given shelf.
pub fn headroom() -> f32 {
    (CASE.z - BOARD) / SHELVES as f32 - BOARD
}

/// The books on one shelf of one bookcase, left to right.
///
/// Stocked until the shelf runs out, so a shelf is full rather than a row of
/// books with a gap at the end. The last one that does not fit is left on the
/// floor of the imagination.
pub fn stock(case: u32, shelf: usize) -> Vec<Book> {
    let room = CASE.x - SHELF_SPARE * 2.0;
    let mut books = Vec::new();
    let mut used = 0.0;

    for n in 0.. {
        let roll = from(case.wrapping_mul(97).wrapping_add(shelf as u32 * 31 + n));
        let thick = BOOK_THICK.0 + (roll % 1000) as f32 / 1000.0 * (BOOK_THICK.1 - BOOK_THICK.0);
        if used + thick > room {
            break;
        }

        let tall =
            BOOK_TALL.0 + ((roll >> 10) % 1000) as f32 / 1000.0 * (BOOK_TALL.1 - BOOK_TALL.0);
        books.push(Book {
            thick,
            tall: tall.min(headroom() - 0.01),
            spine: SPINES[((roll >> 20) as usize) % SPINES.len()],
            // off a different part of the roll from the colour, or every
            // copy of a title would be bound the same and the shelf would
            // read as a pattern
            title: (roll.wrapping_mul(2_654_435_761) >> 7) as usize % TITLES.len(),
        });
        used += thick;
    }

    books
}

/// Where a book stands along its shelf, measured from the bookcase's left edge.
///
/// The row is centred on what it fills, so a shelf that came out a little short
/// is short at both ends rather than all at one.
pub fn along(books: &[Book], n: usize) -> f32 {
    let used: f32 = books.iter().map(|book| book.thick).sum();
    let mut at = (CASE.x - used) * 0.5;
    for book in books.iter().take(n) {
        at += book.thick;
    }

    at + books[n].thick * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0006: every book carries a title, and a shelf is not one title.
    #[test]
    fn a_shelf_is_a_library_and_not_a_pattern() {
        let books = stock(3, 2);
        assert!(books.len() > 6, "only {} books on a shelf", books.len());

        for book in books.iter() {
            assert!(
                book.title < TITLES.len(),
                "a book asks for title {} of {}",
                book.title,
                TITLES.len()
            );
        }

        let apart = books
            .iter()
            .map(|book| book.title)
            .collect::<std::collections::HashSet<_>>();

        assert!(
            apart.len() * 2 > books.len(),
            "{} books on a shelf and only {} titles between them",
            books.len(),
            apart.len()
        );
    }

    /// Spec 0006: and the title is not tied to the binding.
    ///
    /// Off the same part of the roll as the colour, every copy of a title
    /// comes out bound alike, and a shelf of that reads as a pattern rather
    /// than as a room where somebody bought books one at a time.
    #[test]
    fn a_title_is_not_always_bound_the_same() {
        let mut bindings: std::collections::HashMap<usize, std::collections::HashSet<[u32; 3]>> =
            std::collections::HashMap::new();

        for case in 0..12u32 {
            for shelf in 0..SHELVES {
                for book in stock(case, shelf) {
                    bindings
                        .entry(book.title)
                        .or_default()
                        .insert(book.spine.map(|n| n.to_bits()));
                }
            }
        }

        let twice = bindings.values().filter(|bound| bound.len() > 1).count();

        assert!(
            twice * 2 > bindings.len(),
            "only {} of {} titles turn up in more than one binding",
            twice,
            bindings.len()
        );
    }

    /// Spec 0006: the titles are ASCII, because the font has no accents.
    #[test]
    fn every_title_is_one_the_font_can_set() {
        for title in TITLES.iter() {
            assert!(
                title.is_ascii() && !title.is_empty(),
                "{:?} is not a title this font can set",
                title
            );
        }
    }

    /// Spec 0006: a shelf is stocked, and the books fit on it.
    #[test]
    fn the_shelves_are_full() {
        let room = CASE.x - SHELF_SPARE * 2.0;

        for case in 0..4u32 {
            for shelf in 0..SHELVES {
                let books = stock(case, shelf);
                let used: f32 = books.iter().map(|book| book.thick).sum();

                assert!(
                    books.len() > 15,
                    "shelf {} holds {} books",
                    shelf,
                    books.len()
                );
                assert!(
                    used <= room,
                    "shelf {} holds {} of books in {} of room",
                    shelf,
                    used,
                    room
                );
                assert!(
                    used > room - BOOK_THICK.1,
                    "shelf {} is {} short of full",
                    shelf,
                    room - used
                );
            }
        }
    }

    /// Spec 0006: and no book is taller than the shelf above it.
    #[test]
    fn nothing_is_taller_than_its_shelf() {
        for case in 0..4u32 {
            for shelf in 0..SHELVES {
                for book in stock(case, shelf) {
                    assert!(
                        book.tall < headroom(),
                        "a book {} tall is on a shelf {} high",
                        book.tall,
                        headroom()
                    );
                    assert!(book.thick >= BOOK_THICK.0 && book.thick <= BOOK_THICK.1);
                }
            }
        }
    }

    /// Spec 0006: the books stand side by side without overlapping.
    #[test]
    fn the_books_stand_in_a_row() {
        let books = stock(0, 0);

        for n in 1..books.len() {
            let gap = along(&books, n) - along(&books, n - 1);
            let want = (books[n].thick + books[n - 1].thick) * 0.5;

            assert!(
                (gap - want).abs() < 1e-5,
                "book {} is {} from its neighbour and they are {} across",
                n,
                gap,
                want
            );
        }

        // and the row sits inside the case
        let first = along(&books, 0) - books[0].thick * 0.5;
        let last = along(&books, books.len() - 1) + books[books.len() - 1].thick * 0.5;
        assert!(
            first > 0.0 && last < CASE.x,
            "the row runs {} to {}",
            first,
            last
        );
        assert!(
            (first - (CASE.x - last)).abs() < 1e-4,
            "the row is not centred"
        );
    }

    /// Spec 0006: the shelves are evenly spaced and all inside the case.
    #[test]
    fn the_shelves_fit_the_case() {
        for shelf in 1..SHELVES {
            let step = shelf_at(shelf) - shelf_at(shelf - 1);
            assert!(
                (step - (headroom() + BOARD)).abs() < 1e-5,
                "shelf {} is {} above the one under it",
                shelf,
                step
            );
        }

        assert!(shelf_at(0) >= BOARD, "the bottom shelf is below the foot");
        assert!(
            shelf_at(SHELVES - 1) + headroom() <= CASE.z,
            "the top shelf is through the top of the case"
        );
    }

    /// Spec 0006: the panelling fits between the skirting and the rail.
    #[test]
    fn the_panelling_fits_its_wall() {
        let tall = panel_tall();
        assert!(
            tall > 0.3,
            "a panel {} tall is a strip rather than a panel",
            tall
        );
        assert!(
            panel_up() - tall * 0.5 > SKIRTING,
            "a panel runs down into the skirting"
        );
        assert!(
            panel_up() + tall * 0.5 < DADO - RAIL * 0.5,
            "a panel runs up into the rail"
        );

        let top = DADO + CORNICE;
        assert!(
            top < crate::room::TALL,
            "the rail and the cornice do not both fit on a wall {} high",
            crate::room::TALL
        );

        // and the panels are spread along a wall and centred on it
        let along = panels(0.0, 8.0);
        assert!(along.len() > 6, "a wall got {} panels", along.len());
        for pair in along.windows(2) {
            assert!((pair[1] - pair[0] - PANEL_WIDE - PANEL_GAP).abs() < 1e-4);
        }
        assert!(
            (along[0] - (8.0 - along[along.len() - 1])).abs() < 1e-4,
            "the run is not centred"
        );

        // a wall too short for one gets none rather than one hanging off it
        assert!(panels(0.0, 0.4).is_empty());
    }

    /// Spec 0006: the sconces are spread along a wall, not bunched at one end.
    #[test]
    fn the_sconces_are_spread_along_the_wall() {
        let lit = sconces(0.0, 8.0);

        assert!(lit.len() > 2, "a wall got {} sconces", lit.len());
        for pair in lit.windows(2) {
            assert!(
                (pair[1] - pair[0] - SCONCE_EVERY).abs() < 1e-4,
                "two sconces are {} apart and they should be {}",
                pair[1] - pair[0],
                SCONCE_EVERY
            );
        }

        // and the row is centred, so a wall that did not divide evenly is short
        // at both ends rather than all at one
        let first = lit[0];
        let last = lit[lit.len() - 1];
        assert!(
            (first - (8.0 - last)).abs() < 1e-4,
            "the row runs {} from one end and {} from the other",
            first,
            8.0 - last
        );

        // high enough to be over a head and under the ceiling
        let up = SCONCE_UP;
        assert!(
            up > CASE.z && up < crate::room::TALL - 0.5,
            "a sconce at {} is not over the bookcases and under the ceiling",
            up
        );
    }

    /// Spec 0006: a bookcase is stocked the same way every time it is drawn.
    #[test]
    fn a_shelf_is_stocked_the_same_every_time() {
        for case in 0..3u32 {
            for shelf in 0..SHELVES {
                assert_eq!(
                    stock(case, shelf),
                    stock(case, shelf),
                    "the books moved between two readings"
                );
            }
        }

        // and two bookcases are not the same bookcase
        assert_ne!(stock(0, 0), stock(1, 0), "every case holds the same books");
    }
}
