//! The register, behind the one door in the hotel's corridor with no number
//! on it. Spec 0031.
//!
//! An endless list of endless entries, which is what a hotel with a room for
//! every number would have to keep. You walk along it, and at each step the
//! character on the diagonal is taken, changed, and added to what you carry.
//! At the end of the walk you hold a guest who is on no line of the book.
//!
//! That is Cantor's diagonal and it is the exception to the sign hanging over
//! the corridor outside, which says the rooms do not run out. They do not.
//! This is a guest that cannot be given one.
//!
//! Everything here is arithmetic, so it can be checked without a window.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

use crate::room::THICK;

/// How many rows are drawn.
///
/// A window, not a count: the book has no last row, and the diagonal can be
/// no longer than the wall is high.
pub const ROWS: u64 = 14;

/// How many characters of a row are drawn, which is however many reach the
/// end of the wall.
///
/// **The entries do not end, and this is the whole of spec 0032.** It was a
/// flat 24, which put the last digit of every row five metres short of the
/// room's east wall with bare plaster after it. An entry that stops in the
/// middle of a wall is an entry with a last digit, and a list of those is a
/// list of numbers, and the hotel has a room for every number: Jake walked up
/// to the desk holding one and said so.
///
/// So they run into the end wall and are cut by it. The last cell straddles
/// that wall's face, which swallows half of it, and nothing in the room shows
/// where any entry finishes because none of them does.
pub fn columns(reaches: f32) -> u64 {
    let (_, east) = along(reaches);

    (((east - page_left(reaches)) / CELL).ceil() as u64) + 1
}

/// How big a character's cell is on the wall.
pub const CELL: f32 = 0.19;

/// How far a mark stands out of the wall.
///
/// **A cube, not a plate.** Each mark was 0.02 deep, and because a cube's
/// texture runs nought to one on every face however thin the face is, the
/// digit was on the edges too: a smear a couple of centimetres wide down the
/// side of each one. Jake saw it at a glancing angle, said it was kind of
/// cool, and asked for cubes. So they are cubes, and the digit is on the
/// sides on purpose, which means the wall reads as a number from anywhere in
/// the room rather than only from square on.
///
/// **A third of a cell and not a whole one.** At `CELL * 0.82` they were
/// cubes in the proper sense and the wall stopped being readable: every one
/// of six hundred marks showed its digit on two faces at once, and Jake
/// looked at it and said it is unintelligible. This is deep enough to catch
/// the light and show a sliver of the side, which is the thing he liked
/// about the accident, and shallow enough that the front of each one is what
/// you see.
pub const MARK_DEEP: f32 = CELL * 0.3;

/// What is written after a thing that has not finished.
///
/// Not copy and not Jake's to strike: it is punctuation, and it means the
/// same in every line it appears in.
pub const RUNS_ON: &str = "...";

/// The alphabet the book is written in.
///
/// Ten characters is ten small textures for the whole wall, where the
/// corridor's plaques are one a door: a register is the same handful of marks
/// over and over, which is what this engine is for.
///
/// **The alphabet is not the lever and this comment used to say it was.** It
/// read "a row is a number written out and the thing you build by changing
/// them is another number", which is the fault spec 0032 exists to fix,
/// written down by its author and then walked past for a day. The first fix
/// drafted was to make these the twenty six letters so that a guest had a
/// name and a room had a number. Jake said he was dubious, because a list of
/// either letters or numbers can be infinite, and he is right twice:
/// a finite string of letters is as listable as a finite string of digits,
/// and an endless string of digits is as unlistable as an endless string of
/// letters. **What matters is whether the entry ends, never what it is
/// written in.**
///
/// So the digits stay, and they are the better version: an endless string of
/// them is a real number, which makes this wall Cantor's own argument rather
/// than a retelling, and it answers the objection instead of dodging it. Yes
/// it is a number. No, it is not a counting number. The rooms are the
/// counting numbers.
pub const MARKS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// What stands at row `row`, column `column` of the book.
///
/// Worked out rather than stored, so the book is endless in both directions
/// and the same every time it is read. A register you could hold in memory
/// would have a last row.
pub fn mark(row: u64, column: u64) -> u8 {
    mark_from(row, column)
}

/// The same, for an entry that is nobody's row number.
///
/// Somebody written into the book by the desk has a seed of their own rather
/// than a row, because the row they are on moves down every time another one
/// is written in and their entry must not change under them.
pub fn mark_from(seed: u64, column: u64) -> u8 {
    // a cheap mix, good enough that the wall does not read as a pattern and
    // deterministic so two readings of a row agree
    let mut at = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(column.wrapping_mul(0xBF58_476D_1CE4_E5B9));

    at ^= at >> 31;
    at = at.wrapping_mul(0x94D0_49BB_1331_11EB);
    at ^= at >> 29;

    (at % MARKS.len() as u64) as u8
}

/// The character on the diagonal at row `row`, which is its own column.
///
/// **Read out of the book rather than out of `mark`.** Spec 0032: once the
/// desk has written somebody in, row 1 is them, and a diagonal that went on
/// consulting the original book would walk straight past the one guest it has
/// to differ from.
pub fn on_the_diagonal(book: &Book, row: u64) -> u8 {
    book.at(row, row)
}

/// And what it becomes when it is changed.
///
/// **Any change will do and this one is the plainest.** What matters is only
/// that it differs from what was there, because that is the whole proof: the
/// thing you build differs from row n at place n, for every n, so it is on no
/// row.
pub fn changed(was: u8) -> u8 {
    (was + 1) % MARKS.len() as u8
}

// How far along the book a row is, is one cell a row, because the diagonal's
// nth cell is n along as well as n down. It was its own `ROWS_A_METRE` while
// the page slid along under the reader and is `CELL` now that it does not.

/// What you are carrying.
///
/// Built a character at a time as you go along, and it is the guest the desk
/// cannot house.
#[derive(Clone, Debug, Default)]
pub struct Carried {
    marks: Vec<u8>,
    started: bool,
}

impl Carried {
    pub fn new() -> Self {
        Self::default()
    }

    /// How many characters of it there are.
    ///
    /// This and the two below are the vocabulary the proof is written in and
    /// nothing the game draws needs them: what the room shows is `says`, and
    /// what it asks is `is_empty`. They are here so the claim in the tests
    /// can be stated in the same terms Cantor's argument is.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.marks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }

    /// How far along the diagonal the walk has got.
    ///
    /// **So the wall can show it.** Spec 0031 lit the whole diagonal from
    /// the moment you walked in, which means walking changed nothing on
    /// screen: Jake walked the room and said he could not see any actions in
    /// it. The cells behind the walk are lit now and the ones ahead are only
    /// marked, so the staircase fills in as you go.
    pub fn reach(&self) -> u64 {
        self.marks.len() as u64
    }

    /// What is at place `n`, counted from nought, if it has been written yet.
    #[cfg(test)]
    pub fn at(&self, n: usize) -> Option<u8> {
        self.marks.get(n).copied()
    }

    /// Written out, which is what the desk is handed.
    ///
    /// **It runs on.** What you hold is a prefix and never becomes anything
    /// else, so it is written with the three dots that say so. Spec 0032:
    /// a thing with a last digit is a number and the hotel has a room for
    /// every number.
    pub fn says(&self) -> String {
        self.marks
            .iter()
            .map(|mark| MARKS[*mark as usize])
            .chain(RUNS_ON.chars())
            .collect()
    }

    /// Hand it over and start again.
    ///
    /// Emptied rather than kept, because the next one is walked out of a book
    /// that now has this one in it. `started` goes with it: you read the new
    /// book from its head, the same as the old one.
    pub fn taken(&mut self) -> Vec<u8> {
        self.started = false;

        std::mem::take(&mut self.marks)
    }

    /// Take the diagonal as far as `rows`, which is however far along the
    /// book you have walked.
    ///
    /// Only ever grows, and only to where you have been. Walking back does
    /// not unwrite what you took.
    pub fn walked_to(&mut self, book: &Book, rows: u64) {
        // **A book is read from its first row.** The page begins at the
        // room's west end and the way in is at its east, so walking through
        // the door puts you at the far end of the book with the whole
        // diagonal already behind you: `walked` answered fourteen at the
        // doorway, and what spec 0031 called a walk was handed over in the
        // first frame you were in the room. Nothing is taken until you have
        // stood at the head of the page.
        if rows == 0 {
            self.started = true;
        }

        if !self.started {
            return;
        }

        while (self.marks.len() as u64) < rows {
            let row = self.marks.len() as u64;

            self.marks.push(changed(on_the_diagonal(book, row)));
        }
    }

    /// Whether what is carried differs from row `row`, which is the proof.
    ///
    /// It differs at place `row`, by construction, because that is the place
    /// the diagonal was changed at.
    #[cfg(test)]
    pub fn differs_from(&self, book: &Book, row: u64) -> bool {
        match self.at(row as usize) {
            Some(mine) => mine != book.at(row, row),
            // a row past where you have walked has not been answered yet
            None => false,
        }
    }
}

/// The register itself, which is everything written in it.
///
/// **Not a pure function any more, and that is spec 0032.** Spec 0031's book
/// was `mark` and nothing else: endless, the same every reading, and with no
/// way for anything that happened in the building to appear in it. A register
/// nobody can be written into is not a register, and it is also why that room
/// had to end in a refusal.
///
/// The desk writes a guest in. Everybody moved up a room to make space, so
/// every row moves down one, and the one just written in is room 1.
#[derive(Clone, Debug, Default)]
pub struct Book {
    written: Vec<Vec<u8>>,
}

impl Book {
    pub fn new() -> Self {
        Self::default()
    }

    /// How many the desk has written in since the building opened.
    pub fn written(&self) -> usize {
        self.written.len()
    }

    /// What stands at row `row`, column `column`.
    ///
    /// The rows written in sit at the top, most recent first, because each
    /// one arrived into room 1 and pushed the rest down. Below them the book
    /// goes on as it always did, offset by however many are above it.
    pub fn at(&self, row: u64, column: u64) -> u8 {
        let by_hand = self.written.len() as u64;

        if row < by_hand {
            let who = &self.written[(by_hand - 1 - row) as usize];

            return match who.get(column as usize) {
                Some(mark) => *mark,
                // **Past where they walked their entry runs on**, like every
                // other entry on the wall. What they handed over is a prefix
                // and a prefix is all anybody ever has.
                None => mark_from(seed_of(who), column),
            };
        }

        mark(row - by_hand, column)
    }

    /// Write somebody in as room 1, which moves every other row down one.
    pub fn write_in(&mut self, who: Vec<u8>) {
        self.written.push(who);
    }
}

/// A seed for somebody who is on no row of the original book.
///
/// Taken from what they walked, so their entry is the same every time it is
/// read and does not change when another guest is written in above them.
fn seed_of(who: &[u8]) -> u64 {
    let mut at = 0xA24B_AED4_963E_E407u64;

    for mark in who {
        at = at
            .wrapping_mul(0x0000_0100_0000_01B3)
            .wrapping_add(*mark as u64 + 1);
    }

    at
}

/// How big the room is inside, west to east and along `z`.
///
/// Long rather than wide, because what is in it is a wall you walk the length
/// of. The book is on the far side and you go along it.
pub const DEEP: f32 = 3.0;
pub const LONG: f32 = 9.0;
/// And how high, which is nearly twice the corridor it is off.
///
/// **The diagonal can be no longer along the wall than the wall is high.**
/// Rows stack, so the cell taken from row n is n down and n along: the walk
/// is the height. At the corridor's 2.4 that is a two metre stroll, and the
/// first build hid it by sliding the page along under the reader, which is
/// why Jake said the wall seemed to move with him. It did.
///
/// A hall of records is a tall room anyway, and at this height the diagonal
/// is a walk of nearly four metres along a wall that stays where it is.
pub const TALL: f32 = 4.2;

/// Where the room's inside begins and ends along `z`, which is away from the
/// corridor.
pub fn across(reaches: f32) -> (f32, f32) {
    let door = crate::aleph::blank_door(reaches);

    (door.z - THICK - DEEP, door.z - THICK)
}

/// And along `x`, which is the length you walk.
///
/// It runs west from the door, the same way the corridor does, so turning in
/// is a quarter turn rather than a doubling back.
pub fn along(reaches: f32) -> (f32, f32) {
    let door = crate::aleph::blank_door(reaches);

    (door.x - LONG, door.x + crate::aleph::DOOR_WIDE)
}

/// Everything solid in here: the floor, three walls, the ceiling.
///
/// The fourth wall is the corridor's, which stands already and is cut for the
/// blank door.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let (west, east) = along(reaches);
    let (far, near) = across(reaches);

    vec![
        // the floor, lapping under the corridor's rather than meeting it:
        // two floors at one height that meet edge to edge stop you dead, and
        // this building found that three times in one day
        Aabb::new(
            vec3(west - THICK, -THICK, far - THICK),
            vec3(east + THICK, 0.0, near + THICK),
        ),
        // the far wall, which is the one the book is on
        Aabb::new(
            vec3(west - THICK, 0.0, far - THICK),
            vec3(east + THICK, TALL, far),
        ),
        // and the two ends, which stop short of the corridor's wall rather
        // than meeting it.
        //
        // **Neither buried nor flush.** Running them into it put their end
        // faces in the same plane as its face, which is the same fault as
        // building the wall twice and was the second half of it. Stopping
        // half a thickness short leaves a slot too narrow to stand in,
        // closed on every side, and no two faces anywhere near each other.
        Aabb::new(
            vec3(west - THICK, 0.0, far - THICK),
            vec3(west, TALL, near - THICK * 0.5),
        ),
        Aabb::new(
            vec3(east, 0.0, far - THICK),
            vec3(east + THICK, TALL, near - THICK * 0.5),
        ),
        // **No near wall.** That is the corridor's own, which stands already
        // and is cut for the blank door in `aleph::solid`. Building one here
        // too put two walls in exactly the same place, which is two faces in
        // one plane and speckles as you move: the fault this building has
        // fought seven times, and the eighth was mine.
        //
        // **Except above it.** That wall is the corridor's and stops at the
        // corridor's height, and this room is nearly twice as tall, so the
        // strip over it is open unless somebody tops it. Spec 0012 did this
        // twice and missed it once each time, and the warning is in the
        // building's own notes: where a wall is borrowed from a shorter
        // room, sweep the boundary at the height between the two ceilings.
        // **Starting above the corridor's ceiling, not at its wall's top.**
        // That ceiling is 0.3 thick and lies in this same z, so a strip from
        // the wall's top ran through it with both their near faces in one
        // plane, which is what Jake saw dithering along the top of this room.
        // The ceiling closes those three hundred millimetres itself.
        Aabb::new(
            vec3(west - THICK, crate::aleph::TALL + THICK, near),
            vec3(east + THICK, TALL, near + THICK),
        ),
        // Below that it closes this room's near side along the whole of it,
        // since it runs two kilometres west and is cut only at the doorway.
        // the ceiling
        Aabb::new(
            vec3(west - THICK, TALL, far - THICK),
            vec3(east + THICK, TALL + THICK, near + THICK),
        ),
    ]
}

/// How far along the book you are, in rows, from where you are standing.
///
/// Nought at the east end, where you come in, and climbing as you go west.
pub fn walked(reaches: f32, at: Vec3) -> u64 {
    let gone = at.x - page_left(reaches);

    if gone < 0.0 {
        return 0;
    }

    // one row a cell, because the diagonal's nth cell is n along as well as
    // n down: walking a cell's width is walking a row of the book
    ((gone / CELL) as u64).min(ROWS)
}

/// Where the cell at window place `(down, along)` sits on the wall.
///
/// **A page with a diagonal through it, not a diagonal band.** The first
/// build stepped each row along *and* up, so the book came out as a staircase
/// rather than something written on a wall.
///
/// And it does not move. The second build slid the page along to keep the
/// diagonal in front of whoever was walking, which Jake read as the wall
/// moving with him, because it was.
pub fn cell_at(reaches: f32, down: u64, along_by: u64) -> Vec3 {
    let (far, _) = across(reaches);

    vec3(
        // **Along `+x`, which is the reader's left to right.** Facing this
        // wall you look down `-z`, so your right hand points along `+x`, and
        // a row laid out the other way is a row that reads backwards. The
        // first build did, and the room numbers went on the wrong end of it.
        page_left(reaches) + along_by as f32 * CELL,
        // **A cell lower than it was.** At `CELL * 1.6` the top row sat
        // 0.23 under the ceiling, which is not enough to hang a heading in:
        // "WHO IS IN IT" came out 0.19 tall centred 0.02 under it and ran
        // 76mm into the ceiling. Jake looked up and said the ceiling clips
        // the text, and at `CELL * 2.4` said it again: the heading had
        // stopped touching the ceiling and was still up against it, which is
        // a steep thing to read from the floor of a room four metres high.
        //
        // **The page had 0.46 of wall over it and 1.18 under it.** It is
        // nearer the middle now: 0.72 above the top row, 0.93 under the
        // bottom one, and a heading that is a heading rather than a line
        // squeezed into what was left. Held by
        // `the_headings_hang_in_the_band`.
        TALL - CELL * 3.8 - down as f32 * CELL,
        // **Back face buried.** Sat flush on the wall it would be two faces
        // in one plane, which is this building's oldest fault and is worth
        // 15mm to avoid.
        far + MARK_DEEP * 0.5 - 0.015,
    )
}

/// Where the first character of every row is.
///
/// **Fixed to the wall and not to the reader.** Row n is in the same place
/// every time you look at it, which is what a book written on a wall is. The
/// first build slid the page along to keep the diagonal in front of whoever
/// was walking, so the wall was never the same twice and read as moving.
/// Jake walked it and said so.
pub fn page_left(reaches: f32) -> f32 {
    let (west, _) = along(reaches);

    west + CELL * 6.0
}

/// The one fitting in this room, over the head of the page.
///
/// **It is a signpost before it is a lamp.** The page begins at the west end
/// and you come in at the east, so the first thing this room asks is that
/// you walk the length of it to a corner nothing points at. Jake walked in
/// and said there are no actions in the register room, and this is half of
/// why: he was never at the place where the walk starts.
///
/// So the head of the page is the lit end of a dark hall, which is a thing
/// you go and look at without being told to. The room had no fittings at all
/// before this, which the building's own notes say is a room that reads as a
/// corridor however right everything in it is.
pub const SCONCE: Vec3 = vec3(0.1, 0.3, 0.1);
pub const SCONCE_RANGE: f32 = 5.5;
pub const SCONCE_LIT: f32 = 1.15;

pub fn sconces(reaches: f32) -> Vec<Vec3> {
    let (west, _) = along(reaches);
    let (far, _) = across(reaches);

    vec![vec3(
        west + SCONCE.x * 0.5 + THICK * 0.1,
        heading_band(reaches).1 - SCONCE.y * 0.5,
        far + 0.5,
    )]
}

/// And the lamps, which are the sconces lifted.
///
/// Derived rather than placed, so there is no lamp in here that is not in a
/// fitting. The cellar was told that once and the baths did it again worse.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    sconces(reaches)
        .into_iter()
        .map(|at| at + vec3(0.0, SCONCE.y * 0.3, 0.0))
        .collect()
}

/// How brightly the mark at this column is drawn.
///
/// **The other half of why that wall could not be read.** The entries were
/// made to run to the end of the room so that nothing on the wall shows
/// where one finishes, and they ran there at full strength: forty four
/// columns of equal digits, which is a texture rather than a page. The
/// diagonal was lost in it and so was every line.
///
/// So the part a reader uses is lit and the rest falls away. It still runs
/// to the wall and is still cut by it, so no entry ends, and the eye is told
/// which end of it to start at. `READS` is the diagonal's own reach plus a
/// couple, since nothing past that is ever taken.
pub const READS: u64 = ROWS + 2;

pub fn lit_at(column: u64) -> f32 {
    if column < READS {
        return 1.0;
    }

    let past = (column - READS) as f32;
    // down to about a fifth over the next twenty, and no further: a mark
    // that fades to nothing is a page that ends after all
    (1.0 - past / 20.0).clamp(0.22, 1.0)
}

/// What the two columns are, written over them.
///
/// **Without this the wall is numbers and nothing else.** Jake walked it and
/// said it is not clear what they mean, which is the point of demonstrating
/// them. The rows being rooms was half of it: a reader can see the left
/// column is a room number and still have no idea what the long one is.
///
/// Two words, in the world, over the columns they name. The hotel next door
/// explains itself with six words on a sign and the rest is what you watch
/// happen, and this is the same bargain.
pub const HEADINGS: [&str; 2] = ["ROOM", "WHO IS IN IT"];

/// What the book says when you are looking at it and carrying nothing.
///
/// It said "Every guest, every room. Walk it.", which names the claim and
/// then asks for something without saying what it does. Jake read it and
/// said walk it does not explain much.
///
/// **Then it said "you take a mark from each", which is worse.** `mark` is
/// this file's own word for a digit, out of `MARKS` and `mark()`, and it
/// went in front of a player as though it meant something to them. Nothing
/// on that wall is called a mark. There are digits, and one of them in each
/// line is lit, and the lit one is the only thing the reader has to notice.
///
/// **And the voice is a concierge's**, which is Jake's note and is also the
/// joke this hotel runs on. A professional takes the impossible in stride:
/// the desk has shuffled endlessly many guests up a room without once
/// raising an eyebrow, and it should sound like somebody for whom that is
/// Tuesday. It gives the words as well. Somebody keeping a ledger calls a
/// digit a figure, which is the word that should have been reached for
/// instead of this file's own `mark`.
///
/// The line points at what is on the wall: the room you begin at, and the
/// figure that is already lit.
///
/// **And "take" is an instruction for something nobody does.** Jake read it
/// and asked what it was supposed to mean, because there is nothing in this
/// room to press: walking is the whole of it, by spec 0031, which says the
/// diagonal is a construction rather than a riddle. A line that sounds like
/// it is asking for an action in a room with no actions reads as a thing
/// that is broken.
pub const BOOK_SAYS: &str =
    "The house register. Room 1 is at the lit end, and from there each line gives up its figure as you pass.";

/// What the desk says to somebody holding one of these, and after.
///
/// **It says yes.** Spec 0031 had it refuse and the refusal was false: one
/// guest can always be housed. What the desk cannot do is get to the end of
/// the book, and the only way to show that is to say yes and let them look
/// again.
/// A concierge's, the same as `BOOK_SAYS`, and for the same reason: nothing
/// about this is remarkable to the person behind the desk. They have a room.
/// They always have a room. The third line is the only one that lets on that
/// anybody has noticed, and it still does not explain anything.
pub const DESK_TAKES: &str = "is in no room of ours. Room 1, then, and everybody moves up.";
pub const DESK_WROTE_IN: &str = "Room 1 is theirs. The register has them, and nobody has left.";
pub const DESK_AGAIN: &str = "Twice now. There will be a room the next time as well.";

/// The band a heading hangs in: between the top row and the ceiling.
///
/// **The same fault as the hotel's sign, one room over.** That one cleared
/// the corridor's head by six centimetres, which is a gap in the arithmetic
/// and none on screen. This one did not clear the ceiling at all. Both are
/// text hung against a surface with no band worked out first, so this is the
/// band, and the heading is sized and placed inside it rather than either one
/// being chosen on its own.
pub fn heading_band(reaches: f32) -> (f32, f32) {
    // the top of the drawn cell in row nought, which is 0.82 of a cell
    let row = cell_at(reaches, 0, 0).y + CELL * 0.41;

    (row + CELL * 0.25, TALL - CELL * 0.25)
}

/// Where each heading starts: over the column it names.
///
/// **Each at its own column's left edge**, so a heading is as long as its
/// words and sits over what it names. They were placed by eye and ran past
/// the end of the wall, which clipped them against it.
pub fn heading_at(reaches: f32, which: usize) -> Vec3 {
    // **The bottom of it, not the middle.** Centred in the band it floated
    // back up to the ceiling the moment the band grew. A heading belongs
    // with the thing it names, so it sits just over the top row and the rest
    // of the band is bare wall.
    let (low, _) = heading_band(reaches);
    let (far, _) = across(reaches);
    let left = match which {
        // **From the wall rather than from the labels.** Capped to the width
        // of the room numbers it names, "ROOM" came out 0.095 tall against
        // "WHO IS IN IT" at 0.19, which is the two sizes of letter Jake
        // named on the hotel's sign the same morning. The wall west of the
        // numbers is bare, so the heading starts there and comes out within
        // a quarter of the other one.
        0 => along(reaches).0 + CELL * 0.5,
        _ => page_left(reaches),
    };

    vec3(left, low, far + THICK * 0.6)
}

/// How wide a heading comes out, which is its words at the row height.
///
/// Capped at what is left of the wall, so a longer one shrinks rather than
/// running off the end and being clipped by it.
pub fn heading_wide(reaches: f32, which: usize, aspect: f32) -> f32 {
    let (_, east) = along(reaches);
    let left = heading_at(reaches, which).x;
    // **Each capped to its own column**, not to the rest of the wall. Capped
    // only at the wall, "ROOM" ran on into where the other one starts and the
    // two read as "ROOMWHO IS IN IT".
    let room = match which {
        0 => page_left(reaches) - CELL * 0.4 - left,
        _ => east - CELL - left,
    };

    // **The band sets the height and the height sets the width.** It was
    // `CELL * aspect`, which is a line a whole cell tall whatever is above
    // it, and the ceiling cut the top off both of them.
    let (low, high) = heading_band(reaches);
    // **Half again a row.** A heading the size of the entries under it is
    // not a heading, and there is wall to spare now the page has come down.
    let tall = (high - low).min(CELL * 1.5);

    (tall * aspect).min(room.max(CELL * 0.6))
}

/// Which room's entry row `row` is.
///
/// **The rows are rooms.** Without this the wall is an anonymous grid of
/// digits that could be a timetable or a phone book, and nothing on it says
/// it belongs to the hotel you just walked out of. Jake walked the room and
/// asked where the diagonal fits, which is what that looks like from inside.
///
/// With it, the lit diagonal is one digit taken from each room in turn, what
/// you carry differs from room n's entry at place n, and the desk refusing it
/// stops being a rule the building imposes and becomes the only thing it
/// could say.
///
/// Counted from one, because rooms are and places in a string are not.
pub fn room_of(row: u64) -> u64 {
    row + 1
}

/// Where a row's room number is written, which is at the start of it.
///
/// To the left of the first character, where a room number goes in a
/// register and where anybody reading the line looks first.
pub fn label_at(reaches: f32, down: u64) -> Vec3 {
    cell_at(reaches, down, 0) - vec3(CELL * 1.6, 0.0, 0.0)
}

// Which rows are on the wall is not a window any more: it is all of them,
// nought to `ROWS`, in the same place every time. A window was what made the
// wall move.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::room::Room;

    fn reaches() -> f32 {
        Room::of(crate::games()).reaches
    }

    /// Spec 0031: the book has no last row and no last column.
    #[test]
    fn there_is_always_another_row() {
        // a row and a column far past anything drawn still answer
        for (row, column) in [
            (0u64, 0u64),
            (ROWS, 24),
            (10_000, 10_000),
            (u32::MAX as u64, 7),
        ] {
            let got = mark(row, column);

            assert!(
                (got as usize) < MARKS.len(),
                "row {} column {} is not a mark",
                row,
                column
            );
        }

        // and the wall does not read as one character repeated
        let mut seen = [0u32; MARKS.len()];

        for row in 0..200u64 {
            for column in 0..200u64 {
                seen[mark(row, column) as usize] += 1;
            }
        }

        for (n, count) in seen.iter().enumerate() {
            assert!(*count > 0, "the mark {} never appears", MARKS[n]);
        }
    }

    /// Spec 0031: the diagonal takes the nth character of the nth row.
    #[test]
    fn the_diagonal_climbs_as_it_recedes() {
        let reaches = reaches();

        let book = Book::new();

        for row in [0u64, 1, 5, ROWS, 97] {
            assert_eq!(on_the_diagonal(&book, row), mark(row, row));
        }

        // and on the wall it runs corner to corner: the cell taken from row
        // n is n along as well as n down, so a later one is both further
        // along the wall and lower on it
        let one = cell_at(reaches, 3, 3);
        let later = cell_at(reaches, 4, 4);

        // along `+x`, which is the reader's left to right: the rows are laid
        // out the way they are read and the diagonal runs down and to the
        // right through them
        assert!(
            later.x > one.x,
            "a later cell is not further along the wall"
        );
        assert!(later.y < one.y, "a later cell is not further down it");

        // and walking along the wall takes you along that diagonal: the cell
        // you are beside is the one at your own position, because a cell is
        // a row and a cell's width is a step
        let stood = vec3(page_left(reaches) + CELL * 5.5, 0.0, 0.0);

        assert_eq!(walked(reaches, stood), 5, "five cells in is not row five");
        assert!(
            (cell_at(reaches, 5, 5).x - stood.x).abs() < CELL,
            "row five's cell on the diagonal is not where you stand to take it"
        );
    }

    /// Spec 0031: what you hold is on no row. This is the proof.
    ///
    /// It differs from row n at place n, for every n walked, because that is
    /// the place the diagonal was changed at.
    #[test]
    fn what_you_hold_is_on_no_row() {
        let book = Book::new();
        let mut carried = Carried::new();

        carried.walked_to(&book, 0);
        carried.walked_to(&book, 60);

        assert_eq!(carried.len(), 60);

        for row in 0..60u64 {
            assert!(
                carried.differs_from(&book, row),
                "what is carried matches row {} at place {}",
                row,
                row
            );
            assert_ne!(
                carried.at(row as usize),
                Some(book.at(row, row)),
                "place {} was not changed",
                row
            );
        }
    }

    /// And it only grows while you move.
    ///
    /// Spec 0031: the diagonal has to be yours rather than watched. A thing
    /// that writes itself while you stand there is a cutscene, and this
    /// building has never had one.
    #[test]
    fn standing_still_writes_nothing() {
        let reaches = reaches();
        let book = Book::new();
        let stood = vec3(page_left(reaches) + CELL * 4.0, 0.0, 0.0);
        let mut carried = Carried::new();

        // from the head of the page, because spec 0032 takes nothing until
        // you have stood at it
        carried.walked_to(&book, walked(reaches, vec3(page_left(reaches), 0.0, 0.0)));
        carried.walked_to(&book, walked(reaches, stood));

        let was = carried.len();

        // ten more readings from the same place
        for _ in 0..10 {
            carried.walked_to(&book, walked(reaches, stood));
        }

        assert_eq!(carried.len(), was, "it wrote itself while nobody moved");
        assert!(was > 0, "standing two metres in had written nothing at all");
    }

    /// And walking back does not unwrite what was taken.
    #[test]
    fn walking_back_does_not_unwrite_it() {
        let reaches = reaches();
        let book = Book::new();
        let mut carried = Carried::new();

        carried.walked_to(&book, walked(reaches, vec3(page_left(reaches), 0.0, 0.0)));
        carried.walked_to(
            &book,
            walked(reaches, vec3(page_left(reaches) + CELL * 9.0, 0.0, 0.0)),
        );

        let far = carried.len();

        carried.walked_to(&book, walked(reaches, vec3(page_left(reaches), 0.0, 0.0)));

        assert_eq!(carried.len(), far, "walking back shortened it");
    }

    /// Spec 0031: the headings hang clear of the ceiling and of the top row.
    ///
    /// **Jake looked up and said the ceiling clips the text.** "WHO IS IN IT"
    /// was a whole cell tall, centred 0.019 under a ceiling at 4.2, so it ran
    /// 76mm into it. The sign over the hotel's corridor had the same fault
    /// the same morning, one room over, and for the same reason: text hung
    /// against a surface with no band worked out first.
    #[test]
    fn the_headings_hang_in_the_band() {
        let reaches = reaches();
        let (low, high) = heading_band(reaches);

        assert!(high <= TALL, "the band reaches into the ceiling");
        assert!(low < high, "there is no band to hang anything in");

        for (which, says) in HEADINGS.iter().enumerate() {
            let shape = blitzkit::text::room_for(says, crate::aleph::PLAQUE_TEXELS);
            let aspect = shape.x / shape.y.max(1e-4);
            // the same two the drawing works out
            let wide = heading_wide(reaches, which, aspect);
            let tall = wide / aspect;
            let up = heading_at(reaches, which).y + tall * 0.5;

            assert!(
                up + tall * 0.5 <= TALL - CELL * 0.1,
                "{:?} reaches {:.3} into a room {:.3} tall",
                says,
                up + tall * 0.5,
                TALL
            );
            assert!(
                up - tall * 0.5 >= cell_at(reaches, 0, 0).y + CELL * 0.41,
                "{:?} hangs down into the top row",
                says
            );
        }
    }

    /// Spec 0032: no entry ever ends.
    ///
    /// **The fault this spec exists for.** The rows were drawn to a flat
    /// twenty four columns, which put the last digit of every entry five
    /// metres short of the room's east wall with bare plaster after it. An
    /// entry with a last digit is a number, there are only countably many
    /// numbers, and a list of them is a list the hotel has a room for every
    /// member of. Jake walked up to the desk holding one and said exactly
    /// that.
    #[test]
    fn no_entry_ever_ends() {
        let reaches = reaches();
        let (_, east) = along(reaches);
        let columns = columns(reaches);

        // the drawn row reaches the end wall rather than stopping on open
        // plaster, and the last cell straddles it, so nothing on the wall
        // shows where an entry finishes
        let last = cell_at(reaches, 0, columns - 1).x;

        assert!(
            last >= east - CELL,
            "the row stops at {:.2}, short of the wall at {:.2}",
            last,
            east
        );
        assert!(
            last <= east + CELL,
            "the row runs {:.2} past the wall at {:.2}",
            last,
            east
        );

        // and the book itself answers past it, as far as anybody can ask
        let book = Book::new();

        for column in [columns, columns * 3, 10_000, u32::MAX as u64] {
            assert!(
                (book.at(7, column) as usize) < MARKS.len(),
                "column {} of row 7 is not a mark",
                column
            );
        }
    }

    /// Spec 0031 and the building's own rule: every lamp sits in a fitting.
    ///
    /// Derived rather than placed twice. The cellar lit a wall with nothing
    /// on it and the baths ran the lamps along a ceiling while the sconces
    /// stood on a wall, so every sconce was dark and the ceiling glowed at
    /// nothing.
    #[test]
    fn every_lamp_is_in_a_fitting() {
        let reaches = reaches();
        let sconces = sconces(reaches);

        assert!(!sconces.is_empty(), "the room has no fittings at all");

        for lamp in lamps(reaches) {
            assert!(
                sconces.iter().any(|at| (*at - lamp).length() < SCONCE.y),
                "a lamp at {:?} is in no fitting",
                lamp
            );
        }

        // and it is at the head of the page rather than in the middle of the
        // room, because it is a signpost before it is a lamp
        for at in sconces.iter() {
            assert!(
                at.x < page_left(reaches),
                "the fitting is past the head of the page"
            );
            assert!(
                at.y < TALL && at.y > TALL * 0.5,
                "the fitting is not up where the page is"
            );
        }
    }

    /// Spec 0031: the diagonal is lit behind the walk and marked ahead of it.
    ///
    /// **Jake walked the room and said he could see no actions in it.** The
    /// whole diagonal was lit from the moment you came through the door, so
    /// walking changed nothing on the wall: the one thing this room asks of
    /// anybody had no sign that it was working.
    #[test]
    fn the_diagonal_fills_in_as_you_walk() {
        let book = Book::new();
        let mut carried = Carried::new();

        assert_eq!(carried.reach(), 0, "something is taken before a step");

        carried.walked_to(&book, 0);
        carried.walked_to(&book, 5);

        assert_eq!(carried.reach(), 5, "five rows walked and nothing taken");

        carried.walked_to(&book, 9);

        assert_eq!(carried.reach(), 9, "it stopped filling in");

        // and the two colours the wall draws those two states in are not the
        // same, or none of this shows
        assert_ne!(
            crate::aim::REGISTER_DIAGONAL,
            crate::aim::REGISTER_AHEAD,
            "taken and not taken are drawn the same"
        );
    }

    /// Spec 0032: the part of an entry a reader uses is lit, and the run-on
    /// falls away without ever going out.
    ///
    /// **Running them to the wall is what made the wall unreadable.** Forty
    /// four columns at one brightness is a texture, and the diagonal was
    /// lost in it along with every line. Jake looked at it and said it is
    /// unintelligible. Dimming the run-on is what lets the entries go on
    /// for ever and still be read: if it went out altogether the page would
    /// end after all, which is the fault this whole spec exists for.
    #[test]
    fn the_run_on_falls_away_without_going_out() {
        let reaches = reaches();
        let columns = columns(reaches);

        // everything the diagonal can reach is at full strength
        for column in 0..ROWS {
            assert_eq!(lit_at(column), 1.0, "column {} is dimmed", column);
        }

        // and it falls away after
        assert!(
            lit_at(columns - 1) < 0.4,
            "the far end of a row is {:.2}, which is no fainter than the near end",
            lit_at(columns - 1)
        );

        // never going out, and never getting brighter again
        let mut was = 1.0;

        for column in 0..columns * 2 {
            let now = lit_at(column);

            assert!(now > 0.0, "column {} is not drawn at all", column);
            assert!(
                now <= was + 1e-6,
                "column {} is brighter than the one before",
                column
            );

            was = now;
        }
    }

    /// Spec 0032: the left column ends and the right one does not.
    ///
    /// That difference is the only thing on the wall carrying the point, now
    /// that the alphabet has stopped pretending to. A room's number is
    /// finite because it is a counting number. An entry is not.
    #[test]
    fn one_column_ends_and_the_other_does_not() {
        let reaches = reaches();

        // the room number is as long as it is and no longer: every row's
        // label sits west of where the entry starts, in a cell of its own
        for down in 0..ROWS {
            let label = label_at(reaches, down);
            let first = cell_at(reaches, down, 0);

            assert!(
                label.x < first.x,
                "row {}'s number is not before its entry",
                down
            );
            assert!(
                room_of(down).to_string().len() <= 6,
                "a room number that long is not a number anybody finishes"
            );
        }

        // and the entry has no last column at all
        assert!(
            columns(reaches) > ROWS * 2,
            "the entry is drawn shorter than the diagonal is long"
        );
    }

    /// Spec 0032: a book is read from its first row.
    ///
    /// **The walk was over before it started.** The page begins at the room's
    /// west end and the way in is at its east, so `walked` answered fourteen
    /// at the doorway: spec 0031 called this a walk and it was handed to you
    /// in the first frame you were in the room, without a step.
    #[test]
    fn the_walk_starts_at_the_head_of_the_page() {
        let reaches = reaches();
        let book = Book::new();
        let door = crate::aleph::blank_door(reaches);
        let mut carried = Carried::new();

        // standing in the doorway, which is the far end of the book
        assert_eq!(
            walked(reaches, vec3(door.x, 0.0, door.z)),
            ROWS,
            "the doorway is not at the end of the page"
        );

        carried.walked_to(&book, walked(reaches, vec3(door.x, 0.0, door.z)));

        assert!(
            carried.is_empty(),
            "walking in through the door wrote the whole diagonal"
        );

        // and once you have been to the head of it, it writes
        carried.walked_to(&book, walked(reaches, vec3(page_left(reaches), 0.0, 0.0)));
        carried.walked_to(&book, walked(reaches, vec3(door.x, 0.0, door.z)));

        assert_eq!(carried.len() as u64, ROWS, "reading it wrote nothing");
    }

    /// Spec 0032: housing them writes them in as room 1, and everybody else
    /// moves down a row.
    #[test]
    fn housing_them_writes_them_in() {
        let book = Book::new();
        let was: Vec<u8> = (0..6).map(|column| book.at(0, column)).collect();

        let mut carried = Carried::new();

        carried.walked_to(&book, 0);
        carried.walked_to(&book, ROWS);

        let who = carried.taken();
        let mut book = book;

        book.write_in(who.clone());

        assert_eq!(book.written(), 1);

        // they are room 1, which is row nought
        for (column, mark) in who.iter().enumerate() {
            assert_eq!(
                book.at(0, column as u64),
                *mark,
                "row nought is not the one just written in, at {}",
                column
            );
        }

        // and whoever was row nought is row one now
        for (column, mark) in was.iter().enumerate() {
            assert_eq!(
                book.at(1, column as u64),
                *mark,
                "the old row nought did not move down, at {}",
                column
            );
        }

        // and their entry runs on past where they walked, like every other
        assert!(
            (book.at(0, ROWS * 4) as usize) < MARKS.len(),
            "the one written in has a last digit"
        );
    }

    /// Spec 0032: and the next diagonal differs from them too, which is why
    /// housing them did not help.
    #[test]
    fn the_next_diagonal_differs_from_the_last_guest() {
        let mut book = Book::new();
        let mut carried = Carried::new();

        carried.walked_to(&book, 0);
        carried.walked_to(&book, ROWS);

        let first = carried.taken();

        book.write_in(first.clone());

        // read it again, out of the book that now has them in it
        carried.walked_to(&book, 0);
        carried.walked_to(&book, ROWS);

        assert!(
            carried.differs_from(&book, 0),
            "the new one matches the guest the desk just wrote in"
        );

        // every row, which is the proof again and now includes them
        for row in 0..ROWS {
            assert!(
                carried.differs_from(&book, row),
                "the new one matches row {}",
                row
            );
        }
    }

    /// Spec 0032: walking it twice gives two different guests, so the loop is
    /// a loop and not the same entry over again.
    #[test]
    fn the_second_walk_is_a_different_guest() {
        let mut book = Book::new();
        let mut carried = Carried::new();

        carried.walked_to(&book, 0);
        carried.walked_to(&book, ROWS);

        let first = carried.taken();

        book.write_in(first.clone());

        carried.walked_to(&book, 0);
        carried.walked_to(&book, ROWS);

        let second = carried.taken();

        assert_ne!(first, second, "the second walk gave the same guest again");
        assert_eq!(first.len(), second.len());
    }

    /// Spec 0031, still: the page does not move with the reader.
    ///
    /// Struck once already and worth a test rather than a comment, because
    /// the thing that broke it was a function reading where you stand.
    #[test]
    fn the_page_is_fixed_to_the_wall() {
        let reaches = reaches();

        for down in [0u64, 3, ROWS - 1] {
            for along_by in [0u64, 5, 40] {
                let once = cell_at(reaches, down, along_by);
                let twice = cell_at(reaches, down, along_by);

                assert_eq!(once, twice);
            }
        }

        // and the head of the page is set by the room rather than by anybody
        // in it: west of it there is room for the numbers and the heading
        // over them, and east of it the entries run to the far wall
        let (west, east) = along(reaches);
        let head = page_left(reaches);

        assert!(head > west + CELL * 2.0, "no room for the room numbers");
        assert!(head < east - CELL * 20.0, "no room for the entries");
    }

    /// Spec 0031: the blank door is solid, standing open.
    ///
    /// It is not in the hotel's list of rooms, which is where every other
    /// leaf gets its collider, so it had none and you walked through it.
    #[test]
    fn the_blank_door_is_solid() {
        let room = Room::of(crate::games());
        let want = crate::aleph::leaf_box(room.reaches, crate::aleph::BLANK, 1.0);

        assert!(
            room.solid().contains(&want),
            "the one door with no number is a door you walk through"
        );
    }

    /// Spec 0003's sight does not reach through these two rooms' walls.
    ///
    /// **They were built after it and were never in its list.** Standing in
    /// the register you could point at the wall and the building named a
    /// corridor door behind it: "Room 8 has somebody in it". Jake found that
    /// by looking at a wall.
    #[test]
    fn the_sight_does_not_reach_through_these_walls() {
        let room = Room::of(crate::games());
        let reaches = room.reaches;
        let opaque = room.opaque();

        for (name, boxes) in [
            ("the register", solid(reaches)),
            ("the hotel", crate::aleph::solid(reaches)),
        ] {
            for wall in boxes.iter().filter(|b| b.max.y > 1.0 && b.min.y < 1.0) {
                assert!(
                    opaque.iter().any(|o| o == wall),
                    "{} has a wall the sight sees through: {:?}",
                    name,
                    wall
                );
            }
        }
    }

    /// Spec 0012's warning: the strip over a borrowed wall is closed.
    ///
    /// This room is nearly twice the height of the corridor it is off, and
    /// its near side is that corridor's wall, which stops at the corridor's
    /// ceiling. Without something over it the gap is open at the height
    /// between the two, which is a thing you find by sweeping the boundary
    /// rather than by looking at the corners. This building has done it twice
    /// and missed it both times.
    #[test]
    fn the_strip_over_the_borrowed_wall_is_closed() {
        let reaches = reaches();
        let (west, east) = along(reaches);
        let (_, near) = across(reaches);
        let boxes = solid(reaches);

        // sweep the boundary at a few heights between the two ceilings
        // between the corridor's ceiling and this room's, which is the gap
        // only this room can close
        for up in [
            crate::aleph::TALL + THICK + 0.05,
            (crate::aleph::TALL + THICK + TALL) * 0.5,
            TALL - 0.05,
        ] {
            for along_by in 0..12 {
                let at = vec3(
                    west + (east - west) * (along_by as f32 + 0.5) / 12.0,
                    up,
                    near + THICK * 0.5,
                );
                let closed = boxes.iter().any(|b| {
                    b.min.x <= at.x
                        && at.x <= b.max.x
                        && b.min.y <= at.y
                        && at.y <= b.max.y
                        && b.min.z <= at.z
                        && at.z <= b.max.z
                });

                assert!(
                    closed,
                    "the strip over the corridor's wall is open at {:.2} up, {:.2} along",
                    up, at.x
                );
            }
        }
    }

    /// Spec 0014: this room and the corridor it is off do not both build
    /// the wall between them.
    ///
    /// **They did.** The register's near wall and the corridor's even side
    /// were the same box, in the same plane, drawn twice, which speckles as
    /// you move and is the fault this building has fought seven times. Jake
    /// walked the corridor and saw it. The corridor's wall is cut for the
    /// doorway now and this room builds none of that side.
    #[test]
    fn the_wall_between_is_built_once() {
        let reaches = reaches();
        let (far, near) = across(reaches);
        let _ = far;

        for box_ in solid(reaches) {
            // anything of this room's reaching into the plane of that wall,
            // or stopping flush with its face. A box that crosses a plane
            // and carries on is buried and hidden; one that ends in it
            // fights, and so does a second wall laid along it.
            // only what stands at the wall's own height. The ceiling passes
            // over that wall and is buried inside the corridor's own, and a
            // face inside another solid is hidden rather than fighting.
            // at the borrowed wall's own height, which is the corridor's and
            // not this room's. The strip above it is this room's to build,
            // because that wall stops short of this ceiling.
            let at_its_height = box_.min.y < crate::aleph::TALL - 1e-3 && box_.max.y > 1e-3;
            let reaches_it = at_its_height && box_.max.z > near - THICK * 0.5 + 1e-3;

            assert!(
                !reaches_it,
                "the register builds {:?}, which reaches the corridor's wall",
                box_
            );
        }

        // and the corridor's own wall is cut for the door rather than whole
        let blank = crate::aleph::blank_door(reaches);
        let across_it: Vec<_> = crate::aleph::solid(reaches)
            .into_iter()
            .filter(|b| {
                b.min.z < blank.z + 1e-3
                    && b.max.z > blank.z - THICK - 1e-3
                    && b.min.x < blank.x
                    && b.max.x > blank.x
                    // in the doorway's own height. A ceiling over it is a
                    // head, which every doorway has.
                    && b.min.y < crate::aleph::DOOR_TALL - 1e-3
                    && b.max.y > 1e-3
            })
            .collect();

        assert!(
            across_it.is_empty(),
            "the corridor's wall runs straight across the blank doorway: {:?}",
            across_it
        );
    }

    /// Spec 0031: the room is behind the blank door and runs off the
    /// corridor rather than through anything else.
    #[test]
    fn nothing_of_the_register_is_in_another_room() {
        let reaches = reaches();
        let mine = solid(reaches);

        let theirs: Vec<(&str, Vec<Aabb>)> = vec![
            ("the garden", crate::garden::solid(reaches)),
            ("the baths", crate::spa::solid(reaches)),
            ("the cellar", crate::cellar::solid(reaches)),
            ("the back room", crate::back::solid(reaches)),
            ("behind the wall", crate::behind::solid(reaches)),
        ];

        for (name, boxes) in theirs {
            for one in &mine {
                for other in &boxes {
                    let over = one.min.x < other.max.x - 1e-4
                        && other.min.x < one.max.x - 1e-4
                        && one.min.y < other.max.y - 1e-4
                        && other.min.y < one.max.y - 1e-4
                        && one.min.z < other.max.z - 1e-4
                        && other.min.z < one.max.z - 1e-4;

                    assert!(!over, "the register runs into {} at {:?}", name, other);
                }
            }
        }
    }
}
