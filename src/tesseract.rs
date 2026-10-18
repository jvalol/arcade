//! Eight rooms that are one room. Spec 0021.
//!
//! **Built, parked, and brought back with a map.** The room went in on
//! 2026-10-16 and came out the same day, because it worked and nobody could
//! tell where they were. Three goes at fixing that failed in order: a colour
//! per axis with a bar for the end, which was a code with no key; the cells'
//! names written on the walls, which fixed the code and not the problem; and
//! the crossing drawn as a ninety degree turn, which was the one thing the
//! room had never shown. Jake, after walking the last of them: "walk through
//! purple, no idea where I am now."
//!
//! The reason is that a cell is a closed box with no landmark in it, and
//! everything you can see inside one is stated relative to the frame you
//! happen to be standing in. Walk through a wall and every colour has changed
//! meaning and nothing you can see was there a moment ago. A turn can show
//! you that something rotated. It cannot tell you where you are.
//!
//! **So the room has a map, and that changes what it is.** It stops being
//! hold the turning in your head, which this spec asked for and which ninety
//! six states cannot be, and becomes read where you are and work out the
//! route. The turning is the obstacle rather than the whole game.
//!
//! Nothing here draws anything. Which cell meets which, what happens to
//! somebody who walks out of one, where the walls and their marks are, and
//! what the map has to say are all arithmetic, and arithmetic can be checked
//! without a window.

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

    /// Vocabulary of the shape rather than of the room: this and the few
    /// below are what the spec's acceptance criteria are written in and what
    /// the tests exercise, and the drawing has no call for them.
    #[allow(dead_code)]
    pub fn signed(self, by: bool) -> Way {
        if by {
            self
        } else {
            self.back()
        }
    }

    /// Where this points, as four numbers.
    #[allow(dead_code)]
    pub fn along(self) -> [f32; 4] {
        let mut out = [0.0; 4];
        out[self.axis.n()] = if self.up { 1.0 } else { -1.0 };

        out
    }
}

/// A cell is a way, and this name is here so the code can say which it means.
pub type Cell = Way;

/// Every cell, in a fixed order so two runs agree.
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn meets(one: Cell, other: Cell) -> bool {
    one.axis != other.axis
}

/// The one cell you cannot reach in a single step.
#[allow(dead_code)]
pub fn opposite(cell: Cell) -> Cell {
    cell.back()
}

/// The six that you can.
#[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

// ---------------------------------------------------------------------------
// The room itself: one cube, drawn eight ways.
// ---------------------------------------------------------------------------

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How big a cell is, every way, because a cell is a cube.
///
/// One number and not three, and that is the whole constraint this room works
/// under. The wall you walk at becomes the floor you stand on, so a room four
/// wide and three tall would land you in a room four wide and four tall, and
/// the second crossing would land you somewhere else again. Only a cube comes
/// back to itself.
///
/// So it cannot be sized the way the other rooms were, which is by deciding
/// how far apart the furniture goes and letting the ceiling be a ceiling. The
/// height is the price of the trick and reads as part of it.
///
/// **It was 4.6 and that was cramped.** Jake's reading after walking it with
/// the map in: opening it up might help clarify how it works. He is right
/// about why. The room has exactly one thing to understand, which is that
/// four of its faces are the way out, and a box you can cross in two strides
/// gives you no time to notice which one you walked at. At 6.5 there is a
/// room between you and every wall.
pub const CELL: f32 = 6.5;

/// Half of that, which is what every wall is at.
pub const HALF: f32 = CELL * 0.5;

/// How thick the floor and the ceiling are drawn.
pub const SLAB: f32 = 0.3;

/// How far inside the far wall you land after a crossing.
///
/// Not nought. You arrive at the face you came through and that face is now
/// the floor, which puts you against the wall behind you, and a body standing
/// exactly on a wall plane is a body the next frame reads as having crossed
/// it.
///
/// **And not 0.6 either**, which is what it was and which made the room
/// unusable in a way no test could see. A body that lands 0.6 from a wall is
/// a body that crosses again on the first step backwards, so every arrival
/// was spent looking at a panel an arm's length away and any drift put you
/// through another one. This is far enough in to turn round in, and it grew
/// with the room rather than staying put in a bigger one.
pub const INSET: f32 = 1.7;

/// The six faces of a cell, as a person standing in it finds them.
///
/// Four of them are the walls, which are openings rather than walls, and two
/// of them are the floor and the ceiling. The order is the order they are
/// listed in, and nothing depends on it except the tests.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    Ahead,
    Back,
    Right,
    Left,
    Down,
    Up,
}

impl Face {
    pub const ALL: [Face; 6] = [
        Face::Ahead,
        Face::Back,
        Face::Right,
        Face::Left,
        Face::Down,
        Face::Up,
    ];

    /// The step that goes through this face, for the four that one does.
    ///
    /// Nothing for the floor and the ceiling, which is the whole of why
    /// [`Step`] has four and this has six.
    pub fn step(self) -> Option<Step> {
        match self {
            Face::Ahead => Some(Step::Ahead),
            Face::Back => Some(Step::Back),
            Face::Right => Some(Step::Right),
            Face::Left => Some(Step::Left),
            Face::Down | Face::Up => None,
        }
    }

    /// Which way it faces, in the room's own coordinates.
    pub fn way(self) -> Vec3 {
        match self {
            Face::Ahead => Vec3::Z,
            Face::Back => Vec3::NEG_Z,
            Face::Right => Vec3::X,
            Face::Left => Vec3::NEG_X,
            Face::Down => Vec3::NEG_Y,
            Face::Up => Vec3::Y,
        }
    }
}

impl Frame {
    /// Which cell is on the other side of a face.
    ///
    /// **The floor is the room you came from**, which is the one thing about
    /// this room that reads backwards and is worth saying twice. Spec 0021
    /// said the room you left is overhead; the arithmetic says it is under
    /// your feet, and the arithmetic is right. The square you walked at is a
    /// face of both cells. Once it is your floor, the cell on its far side is
    /// the one you were just standing in, and that is why you cannot walk
    /// back the way you came: the way you came is a floor, and nobody walks
    /// into a floor.
    pub fn beyond(self, face: Face) -> Cell {
        match face {
            Face::Up => self.up,
            Face::Down => self.up.back(),
            other => self.toward(other.step().expect("the other four are steps")),
        }
    }
}

/// Where the middle of the room is, with the floor at y nought.
pub fn middle() -> Vec3 {
    vec3(0.0, HALF, 0.0)
}

/// What holds you up and what stops you going through the ceiling.
///
/// Two boxes and no walls, because the walls are the way out. A cell's four
/// upright faces are openings: walking at one is how you leave, so a solid
/// there would be a room with nothing to do in it.
///
/// Both are laid outside the room rather than inside it, so the surface you
/// walk on is at y nought exactly and the span between them is the cube.
pub fn solid() -> Vec<Aabb> {
    let wide = HALF + SLAB;

    vec![
        Aabb::new(vec3(-wide, -SLAB, -wide), vec3(wide, 0.0, wide)),
        Aabb::new(vec3(-wide, CELL, -wide), vec3(wide, CELL + SLAB, wide)),
    ]
}

/// The same, plus what is standing in the middle of the floor.
///
/// Which is the plinth, and there is one in every cell. It held the way out
/// and nothing else when only one room had it, and eight rooms alike in
/// everything but a word and a colour is the thing three rewrites of the map
/// foundered on. Now each of them has its own object on its own plinth, and
/// the way out is one of the eight rather than the only furniture.
pub fn solid_in_a_cell() -> Vec<Aabb> {
    let mut out = solid();
    let (at, size) = plinth();

    out.push(Aabb::from_center_size(at, size));
    out
}

/// Which wall somebody has walked past, if they have walked past one.
///
/// Read off the position rather than off the movement, because a body that
/// has been pushed, slid or stepped can leave by a wall it was not walking
/// at. The furthest one, so a corner picks a side rather than crossing twice.
pub fn crossing(at: Vec3) -> Option<Step> {
    let past = [
        (at.z - HALF, Step::Ahead),
        (-HALF - at.z, Step::Back),
        (at.x - HALF, Step::Right),
        (-HALF - at.x, Step::Left),
    ];

    past.iter()
        .filter(|(over, _)| *over > 0.0)
        .max_by(|one, other| one.0.total_cmp(&other.0))
        .map(|(_, step)| *step)
}

/// Where a crossing puts you down in the next cell.
///
/// On the floor, just inside the wall opposite the one you walked at, with
/// the way you did not spend carried across unchanged. You keep walking the
/// way you were walking and the room turns under you, which is the only part
/// of a four dimensional rotation a person can be shown.
///
/// The height goes because there is no height to keep. You crossed at the
/// foot of a wall and that wall is the floor now; whatever you were standing
/// on is behind the floor, not under it.
pub fn arrive(at: Vec3, step: Step) -> Vec3 {
    let edge = HALF - INSET;

    match step {
        Step::Ahead => vec3(at.x, 0.0, -edge),
        Step::Back => vec3(at.x, 0.0, edge),
        Step::Right => vec3(-edge, 0.0, at.z),
        Step::Left => vec3(edge, 0.0, at.z),
    }
}

/// How wide a dark border is left round each panel.
///
/// The panels are what you read the room off, and six coloured faces meeting
/// edge to edge is a box of colour rather than six marks. The border is what
/// makes each one a thing hung on a wall.
pub const BORDER: f32 = 0.22;

/// How far a panel stands off the face it is on.
///
/// Small, and it has to be more than nothing: a panel laid exactly in the
/// plane of the face is two coplanar surfaces both on show, which this
/// building has got wrong seven times and which speckles as you move.
pub const PROUD: f32 = 0.03;

/// The stripe that says which end of its axis a cell is, and how far it sits
/// from the panel's edge.
pub const STRIPE: f32 = 0.42;
pub const STRIPE_IN: f32 = 0.28;

/// The six dark slabs that close the room.
///
/// Drawn and not solid, which is the whole of this room: a person walks at a
/// wall and through it. Only the floor and the ceiling in [`solid`] actually
/// stop anybody, and these are what they see.
///
/// They are what the border round each panel shows, so the room is a dark box
/// with six lit marks hung in it rather than a box made of colour.
pub fn shell() -> Vec<(Vec3, Vec3)> {
    let span = CELL + SLAB * 2.0;

    Face::ALL
        .iter()
        .map(|face| {
            let at = middle() + face.way() * (HALF + SLAB * 0.5);
            let size = match face {
                Face::Ahead | Face::Back => vec3(span, span, SLAB),
                Face::Right | Face::Left => vec3(SLAB, span, span),
                Face::Down | Face::Up => vec3(span, SLAB, span),
            };

            (at, size)
        })
        .collect()
}

/// The field on a face, in the colour of the cell beyond it.
///
/// Where it is and how big, which is all the room needs to say; what colour
/// is `aim`'s business.
///
/// **Half buried in the shell behind it.** A panel laid exactly on the face
/// is two surfaces in one plane both on show, which this building has got
/// wrong seven times and which speckles as you move. Sinking its back into
/// the slab leaves one face of it visible and the rest inside a solid, where
/// a face is hidden rather than fighting.
pub fn panel(face: Face) -> (Vec3, Vec3) {
    let span = CELL - BORDER * 2.0;
    let at = middle() + face.way() * HALF;

    let size = match face {
        Face::Ahead | Face::Back => vec3(span, span, PROUD * 2.0),
        Face::Right | Face::Left => vec3(PROUD * 2.0, span, span),
        Face::Down | Face::Up => vec3(span, PROUD * 2.0, span),
    };

    (at, size)
}

/// The sixteen corners of a four dimensional cube, flattened to three.
///
/// The projection is the ordinary one and is the whole of why a drawn
/// tesseract looks like a cube inside a cube: a corner is divided by how far
/// away it is along `w`, so the eight at `w = -1` come out small and the
/// eight at `w = +1` come out large, and the thirty two edges join them.
///
/// `EYE` is how far off the fourth dimension is looked at from. Closer makes
/// the inner cube smaller and the shape more dramatic; this is far enough
/// that the inner cube is a cube rather than a dot.
pub const EYE: f32 = 2.6;

pub fn corners() -> Vec<Vec3> {
    let mut out = Vec::with_capacity(16);

    for n in 0..16 {
        let at: Vec<f32> = (0..4)
            .map(|axis| if n >> axis & 1 == 1 { 1.0 } else { -1.0 })
            .collect();
        let near = 1.0 / (EYE - at[3]);

        out.push(vec3(at[0] * near, at[1] * near, at[2] * near));
    }

    out
}

/// Which corners are joined: the pairs that differ in exactly one of the four.
pub fn edges() -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(32);

    for n in 0..16usize {
        for axis in 0..4 {
            let other = n ^ (1 << axis);
            if other > n {
                out.push((n, other));
            }
        }
    }

    out
}

/// The sixteen corners again, but laid out in the frame of whoever is
/// holding it.
///
/// **A map that does not line up with the room is a picture of a tesseract,
/// not a map.** Jake, after walking it: it is hard to tell from the
/// tesseract map what the walls mean. He is right, and the free spin was
/// half of why. A shape turning on its own cannot correspond to anything
/// around you, however pretty it is.
///
/// So the four ways of the projection are the four ways of the person
/// standing in it. Right is right, up is up, ahead is the way they are
/// facing, and the fourth, which decides which cell comes out as the outer
/// cube, is the cell they are standing in. The part of the shape on your
/// left is the cell through the wall on your left.
///
/// [`corners`] is the same thing from the world's point of view and is what
/// the object on the plinth is built from, where there is nobody standing
/// inside it to have a point of view.
pub fn corners_in(frame: Frame) -> Vec<Vec3> {
    let along = [frame.right, frame.up, frame.ahead, frame.ana];
    let mut out = Vec::with_capacity(16);

    for n in 0..16usize {
        // where this corner sits along each of the four ways the holder has
        let mut mine = [0.0f32; 4];
        for (slot, way) in along.iter().enumerate() {
            let at = if n >> way.axis.n() & 1 == 1 {
                1.0
            } else {
                -1.0
            };
            mine[slot] = if way.up { at } else { -at };
        }

        let near = 1.0 / (EYE - mine[3]);
        out.push(vec3(mine[0] * near, mine[1] * near, mine[2] * near));
    }

    out
}

/// Which of the sixteen corners belong to one cell.
///
/// A cell is a facet: the eight corners whose coordinate on that axis is the
/// one the cell is named for. [`corners`] numbers them so that bit `axis` of
/// the index is that coordinate, which makes this a mask rather than a
/// search.
pub fn corners_of(cell: Cell) -> Vec<usize> {
    (0..16)
        .filter(|n| (n >> cell.axis.n() & 1 == 1) == cell.up)
        .collect()
}

/// And which of the thirty two bars are its own: the ones with both ends in
/// it, which is twelve, because a cell is a cube.
pub fn edges_of(cell: Cell) -> Vec<(usize, usize)> {
    let mine = corners_of(cell);

    edges()
        .into_iter()
        .filter(|(one, other)| mine.contains(one) && mine.contains(other))
        .collect()
}

/// The four walls of the room you are standing in, laid out flat beside the
/// shape.
///
/// **Two maps, because they answer two different questions.** The shape says
/// what this place is: eight cells, the one you are in as the outer cube and
/// the one you cannot reach inside it. It is the wrong instrument for the
/// question a person actually has while walking, which is which wall leads
/// where, because the six cells between the cubes are wedges and a wedge
/// does not read as the wall on your left however well it is labelled. Jake
/// said so three times before I stopped tuning it and listened.
///
/// This answers that one and nothing else: four squares, each in the colour
/// of the room through that wall and carrying the thing that stands in it.
/// How they are laid out is in [`cross_at`], and why they are not a cube any
/// more is there too.
///
/// It carries no writing. The walls of the room each have their name on
/// them already, and a name on a square this size is a word nobody can read
/// over a picture nobody can then see.
/// Half a square, which is smaller than the cube's face was. A cube at an
/// angle shows you three faces that each cover less than they measure, and
/// four squares laid flat and facing you cover every pixel they are given:
/// at the cube's own size the cross took a quarter of the window.
pub const ROOM_ACROSS: f32 = 0.0142;
pub const ROOM_SKIN: f32 = 0.0022;
pub const ROOM_UP: f32 = 0.104;

/// Where one of the four squares sits, in the holder's own frame, and how
/// big it is.
///
/// **A flat cross and not a cube.** This was a small cube of the room you
/// are standing in, one face per wall, turned to the holder's frame so that
/// the face pointing the way you look is the wall you are looking at. It is
/// the third thing in this spec that was right on paper and would not read:
/// a cube drawn at an angle shows you three faces at once and asks you to
/// work out which of them is the wall in front of you, while the other three
/// are hidden behind it. Jake, in kata with seven rooms walked: the map
/// still doesn't seem clear how it corresponds to the room I'm in and which
/// ways I can go.
///
/// So the four walls are laid out flat, facing you, the way a compass is
/// drawn. Up is the wall in front of you, down is the one behind, left is
/// left. Nothing is hidden, nothing is foreshortened, and there is no
/// matching to do. The two that are not on it are the floor and the ceiling,
/// which are the two you cannot walk at.
pub fn cross_at(face: Face) -> Option<(Vec3, Vec3)> {
    face.step()?;

    // in the holder's own frame, where x is right, y is up and z is ahead
    let along = match face {
        Face::Ahead => Vec3::Y,
        Face::Back => Vec3::NEG_Y,
        Face::Right => Vec3::X,
        Face::Left => Vec3::NEG_X,
        Face::Down | Face::Up => return None,
    };

    Some((
        along * CROSS_STEP,
        vec3(ROOM_ACROSS * 2.0, ROOM_ACROSS * 2.0, ROOM_SKIN),
    ))
}

/// How far from the middle of the cross each square sits.
///
/// Far enough apart to read as four things rather than one shape, near
/// enough that the whole of it is one glance. A little over a square and a
/// quarter, which leaves a gap of about half a square between neighbours.
pub const CROSS_STEP: f32 = ROOM_ACROSS * 2.6;

/// Where the thing for the room through one of those four walls sits, in the
/// same frame, and how big across it is drawn.
///
/// Stood just off the face rather than sunk into it, so what you see is the
/// whole object and not the half of it that cleared a glowing square. Off
/// toward the holder now rather than out along a wall's own way, because the
/// squares all face the same direction.
///
/// **Toward the holder is plus z here.** The basis this is laid out in has
/// its third axis pointing back at the eye, which is written down where the
/// cross is drawn and was got wrong anyway: minus z put every object behind
/// its own square, where the square hid it completely.
pub fn face_thing(face: Face) -> Option<(Vec3, f32)> {
    let (at, _) = cross_at(face)?;
    let across = ROOM_ACROSS * 2.0 * FACE_ACROSS;

    Some((at + Vec3::Z * (ROOM_SKIN + across * 0.5), across))
}

/// Where the map is held, as an offset from the eye in the camera's own
/// right, up and forward, and how big it is drawn.
///
/// **A thing in the hand rather than a grid in the corner.** The grid was
/// right about the one rule it showed, which is that the cell you cannot
/// reach is the other end of your own axis, and wrong about everything else:
/// it looks nothing like the room it is a map of, and Jake's reading was
/// that it leaves a lot of questions for a new user.
///
/// This is the same object that stands on the sixth plinth, which is the
/// thing you turned to get in here, so the map and the way in and the room
/// are all one shape. The cell you are standing in is the part of it that is
/// lit.
///
/// It cannot be drawn in the corner of the screen the way the grid was. The
/// two dimensional layer has axis aligned rectangles and nothing else, so a
/// wireframe at any angle is not available to it at all.
///
/// **Held nearer than anything in the room can be, which is what stops it
/// being clipped.** It is drawn in the world rather than on the glass, so a
/// wall or the plinth between it and the eye hides it, and at 1.05 ahead
/// both do: Jake walked up to the plinth and the map went behind it and off
/// the edge of the window. A body is `RADIUS` across and cannot get nearer
/// than 0.45 to anything solid, so a map at 0.38 has nothing that can come
/// between it and the eye, and the camera's near plane is 0.1 and well
/// clear of it.
///
/// What decides how big it is drawn is the angle it covers, not how long
/// anybody's arm is. Everything here scales with `HELD_OUT`, so moving it
/// nearer and shrinking it in the same breath leaves it the same size on
/// the window.
pub const HELD_OUT: f32 = 0.38;
pub const HELD_RIGHT: f32 = 0.140;
pub const HELD_DOWN: f32 = 0.082;
pub const HELD_ACROSS: f32 = 0.0543;
pub const HELD_BAR: f32 = 0.00326;

/// How far the held one is tilted, so it reads as a solid rather than a
/// diagram and you are not looking straight down its own ahead.
///
/// **It used to turn on its own, and that was the fault.** A spinning map
/// cannot line up with the room, so nothing told you which part of it was
/// the wall in front of you. It is still now and built in your own frame
/// instead, and the tilt is the smallest that still reads as three
/// dimensional.
pub const HELD_TILT_Y: f32 = 0.62;
pub const HELD_TILT_X: f32 = -0.34;

/// How thick the bars of the shape on the plinth are drawn.
pub const BAR: f32 = 0.035;
pub const BAR_SIDES: u32 = 6;

/// The shape that stands on the sixth plinth: a four dimensional cube, as a
/// frame of thirty two bars.
///
/// This is the object spec 0021 says is the most made thing in graphics, and
/// it is on a plinth rather than in a room for exactly that reason. The five
/// beside it are things you turn and watch; this is the one that turns you,
/// and a person has to recognise what they are reaching for.
///
/// Built out of the engine's own tube, one run of it per edge, so the bars
/// are round and lit like everything else in the nook rather than being
/// lines, which this renderer does not draw.
pub fn shape() -> blitzkit::mesh::MeshData {
    let corners = corners();
    let mut out = blitzkit::mesh::MeshData::new(Vec::new(), Vec::new());

    for (one, other) in edges() {
        out.extend(&blitzkit_shapes::hilbert::tube(
            &[corners[one], corners[other]],
            BAR_SIDES,
            BAR,
        ));
    }

    out
}

/// Where `--screenshot=` stands to photograph the room, and which way it
/// looks.
///
/// Not where a person arrives. Arriving puts you an arm's length from the
/// plinth facing the thing you just turned, which is right for walking in and
/// is a bad photograph: the first one taken from there was a wall a metre
/// away filling the frame.
///
/// So this stands back in a corner and looks across at the far one, which is
/// the longest line in a cube and the only view that gets a whole panel, its
/// mark, the floor and some ceiling into one frame.
pub fn posed() -> (Vec3, f32, f32) {
    (
        vec3(HALF * 0.52, 0.0, -(HALF - 0.45)),
        // yaw nought faces -z and grows toward +x, so this looks across the
        // room and a little to the left
        std::f32::consts::PI + 0.41,
        0.17,
    )
}

/// What the eight cells are called.
///
/// **Words and not a colour code.** The first build said which cell was
/// through a wall with a hue for the axis and a bar high or low for the end,
/// which is a code, and nothing in the room taught it. Jake read the room and
/// asked what the white bars meant, which is the whole answer to whether a
/// mark needs a legend.
///
/// So the walls say it. The colours stay, because a colour is read across a
/// room faster than a word and costs nothing, but what a wall means is now
/// written on it.
///
/// Four pairs, one to an axis, so the pairing is knowledge a person already
/// has: north is across from south and nothing else. That carries the one
/// rule of the shape without anybody being told it, because the room you
/// cannot reach is the other end of your own name.
///
/// `ana` and `kata` are Hinton's, for the direction nobody has. They are the
/// odd pair on purpose: the fourth axis should not sound like the other
/// three.
///
/// These are Jake's to approve, like every other string the building says.
/// They are unstruck, and `aim::TESSERACT_HELP` carries the marker that holds
/// the gate for all of them.
pub const NAMES: [(&str, &str); 4] = [
    ("east", "west"),
    ("zenith", "nadir"),
    ("north", "south"),
    ("ana", "kata"),
];

/// What one cell is called.
pub fn named(cell: Cell) -> &'static str {
    let (up, down) = NAMES[cell.axis.n()];

    if cell.up {
        up
    } else {
        down
    }
}

/// The line the corner says about where you are standing.
///
/// Which room, and where the way out is, which is what Jake asked for after
/// walking it: the room gave you nothing to hold and so every cell was the
/// same cell. Naming the cell with the plinth in it is enough. A count of
/// steps to it would be a hint system, and working out the route is the game.
pub fn standing_in(cell: Cell) -> &'static str {
    match (cell.axis, cell.up) {
        (Axis::X, true) => "You are in east. The way out is in ana.",
        (Axis::X, false) => "You are in west. The way out is in ana.",
        (Axis::Y, true) => "You are in zenith. The way out is in ana.",
        (Axis::Y, false) => "You are in nadir. The way out is in ana.",
        (Axis::Z, true) => "You are in north. The way out is in ana.",
        (Axis::Z, false) => "You are in south. The way out is in ana.",
        (Axis::W, true) => "You are in ana. The way out is here.",
        (Axis::W, false) => "You are in kata. The way out is in ana.",
    }
}

/// What stands in each room, and what its walls show of the rooms beyond.
///
/// **The thing that makes the room navigable, and it is not a map.** Three
/// maps failed because a map can only point at destinations that already
/// mean something, and these did not: eight rooms alike but for a word on a
/// wall and a colour stated relative to where you happen to be standing.
/// Jake, an experienced topologist, after the third: if I walk through one
/// where will it take me, and I want it to make sense to a seven year old.
///
/// So each room has a thing in it, and each wall shows a small one of the
/// thing in the room beyond. Walk through the wall with the teapot on it and
/// you are in the room with the teapot. Nothing to read, nothing to
/// remember, nothing relative.
///
/// The names stay. They are in the corner and in the gloss, because they are
/// how the room is talked about. They are not what anybody navigates by.
///
/// ana keeps the four dimensional cube, which is both its thing and the way
/// out, so finding the four-cube room is finding the way out.
///
/// kata, which is what there is to find, had the sphere: the plainest of
/// them, and the one that looks the same from every side, which was meant as
/// the joke the far room had earned. Jake, having walked all seven rooms to
/// reach it: one of the rooms is just a sphere. The room at the end of three
/// crossings is the one that should be worth arriving in, and a joke that
/// lands as an unfinished object is not worth the walk. It has a trefoil
/// knot now, which is the prettiest thing in here and the only one that
/// cannot be untied.
///
/// south had a plain cube and has a torus instead. The cube was meant as the
/// deliberate ordinary one, a three dimensional cell standing against the
/// four dimensional one in ana, which is a joke that needs explaining. What
/// Jake saw was a grey block on a plinth in a room full of carved shapes,
/// and what he asked was what is this block. In a building where a box is
/// what a thing looks like before it is built, a box is a placeholder
/// whatever it was meant to be.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Thing {
    Teapot,
    Klein,
    Sierpinski,
    Menger,
    Hilbert,
    Torus,
    FourCube,
    Knot,
}

impl Thing {
    pub const ALL: [Thing; 8] = [
        Thing::Teapot,
        Thing::Klein,
        Thing::Sierpinski,
        Thing::Menger,
        Thing::Hilbert,
        Thing::Torus,
        Thing::FourCube,
        Thing::Knot,
    ];

    pub fn n(self) -> usize {
        Thing::ALL
            .iter()
            .position(|it| *it == self)
            .expect("every one is in the list")
    }
}

/// Which thing is in which room, in [`cells`] order.
pub fn thing(cell: Cell) -> Thing {
    Thing::ALL[numbered(cell)]
}

/// Which thing goes on one of a room's walls, and on that wall's face of the
/// map in your hand.
///
/// The thing in the room through it, which is the whole promise: walk at the
/// teapot and you are in the room with the teapot in it.
///
/// One function rather than the same two calls written out in the room and
/// in the map, because a wall and the face of the map that stands for it
/// showing different objects is worse than either showing nothing. Two
/// lists of one thing drift, and this room has already paid for that lesson
/// in the lamps and the plinths.
pub fn sign_on(frame: Frame, face: Face) -> Thing {
    thing(frame.beyond(face))
}

/// What is standing on the plinth in here, where that is worth saying.
///
/// **Only what cannot be seen.** The first draft captioned all eight, and
/// Jake read it back: on the plinth here is repetitive and not that
/// informative, the player can see what is there. He is right. A teapot, a
/// cube and a sphere are what they look like and a line under them is noise,
/// so those three say nothing at all.
///
/// What is left is a name a person cannot work out by looking, which is the
/// four carved shapes and nothing else.
///
/// Not the four dimensional cube either, though that one was drafted. It
/// stands in ana and nowhere else, and [`standing_in`] already says the way
/// out is here in exactly that room, so a line about it is the same sentence
/// twice.
///
/// Separate from [`standing_in`] because that line is Jake's own and struck,
/// and this one is a draft of mine.
pub fn on_the_plinth(cell: Cell) -> Option<&'static str> {
    match thing(cell) {
        Thing::Klein => Some("A Klein bottle: one surface, and no inside."),
        Thing::Sierpinski => Some("A Sierpinski tetrahedron."),
        Thing::Menger => Some("A Menger sponge."),
        Thing::Hilbert => Some("A Hilbert curve, which is one line."),
        Thing::Knot => Some("A trefoil knot: one loop, and it will not come undone."),
        Thing::Teapot | Thing::Torus | Thing::FourCube => None,
    }
}

/// How big a room's own thing is drawn, how big the one on a wall is, and
/// how big the one on a face of the map is.
///
/// A wall's is small on purpose. It is a sign saying what is through there,
/// not a second copy of the room.
pub const THING_ACROSS: f32 = 0.62;
pub const SIGN_ACROSS: f32 = 0.95;
pub const FACE_ACROSS: f32 = 0.62;

/// Where a room's own thing stands: on the plinth in the middle.
pub fn thing_at() -> Vec3 {
    shape_at()
}

/// How far a sign stands off its own wall, as a fraction of its width.
///
/// **Half of it is not enough.** Every one of these is scaled so its widest
/// way is [`SIGN_ACROSS`], which puts it inside a cube of that side, and
/// then turned by [`SIGN_TURN`]. A cube turned about its upright reaches
/// furthest along a horizontal axis at forty five degrees, where the
/// half-extent is not half the side but half its diagonal, which is 0.707 of
/// it. At a half, the corner of that box is inside the wall: Jake, looking
/// at the Klein bottle on a wall in south, said it was embedded in it.
///
/// Three quarters, so the furthest corner of the furthest turn still stands
/// a little clear and the sign reads as a thing in the room rather than a
/// shape painted on the wall.
pub const SIGN_OUT: f32 = 0.75;

/// Which way every small one is turned.
///
/// Still, and all of them the same way, so the one on a wall and the one on
/// the map are the same picture rather than two views of one object. Off
/// square by about a quarter turn, which is the angle that tells a teapot
/// from a sphere at the size these are drawn.
pub const SIGN_TURN: f32 = 0.78;

/// And where the sign for the room beyond a wall sits, for the four walls
/// that have one.
///
/// In the middle of its panel, which is where somebody looking at a wall is
/// already looking. It was high on the panel above the name first, and from
/// anywhere nearer than the far corner it was over your head and out of the
/// picture: the name plate is at the top or the bottom of a panel already,
/// so the middle was free and is the one place on a wall you cannot miss.
///
/// **The floor and the ceiling have none.** They had one briefly, set into
/// their own surface, because a metre of Menger sponge standing on the floor
/// of a room you walk about in is furniture you would walk through. Sunk, it
/// read as exactly what it looked like: Jake, with a four dimensional cube
/// coming out of the floor in the corner of his picture, said things can get
/// clipped, and he is right.
///
/// Taking them out is better than hiding them better. A sign on a wall says
/// where walking at that wall puts you, and these two are the faces nobody
/// can walk at: whatever is through them, you cannot get there this way. The
/// floor's own plate still names the room you came from, which is the one
/// thing about the floor worth knowing, and the map in your hand still
/// carries all six.
pub fn sign_at(face: Face) -> Option<Vec3> {
    face.step()?;

    let (at, _) = panel(face);

    Some(at - face.way() * (SIGN_ACROSS * SIGN_OUT))
}

/// What an axis's pair of names means, for the two pairs that need saying.
///
/// **East and west are obviously opposites and zenith and nadir are not**,
/// which is Jake's reading and is the whole of why this exists. Three of the
/// four pairs are directions a person already has words for; the fourth is a
/// direction nobody has at all, and Hinton's names for it carry no meaning
/// to anyone who has not read Hinton.
///
/// So the two that need it say what they are, and the two that do not are
/// left alone. A room that explains north and south is a room that thinks you
/// are stupid.
///
/// Said every time rather than once, because this room is entered rarely and
/// a thing explained on the first visit only is a thing forgotten by the
/// second. It is one short line under two that are already there.
pub fn gloss(cell: Cell) -> Option<&'static str> {
    match cell.axis {
        Axis::Y => Some("zenith is up and nadir is down, the way a ceiling and a floor are."),
        Axis::W => {
            Some("ana and kata are the two ends of the fourth way, which has no other name.")
        }
        Axis::X | Axis::Z => None,
    }
}

/// How tall the written name is on a panel, as a fraction of the panel, and
/// how many texels it is drawn with.
///
/// Only the height. How wide follows from the word, because a quad sized
/// without asking comes out stretched: `drawn` makes a texture the shape of
/// the letters in it, and "ana" and "zenith" are not the same shape. The
/// first build forced every one into the same box and the short names came
/// out fat.
pub const WORD_TALL: f32 = 0.072;
pub const WORD_TEXELS: f32 = 192.0;

/// Where a face's written name hangs, which way it faces, and how big.
///
/// The quad this is drawn on faces its own `+x` with its width along `z`, per
/// `room::screen_mesh`, so the rotation is what turns that to face into the
/// room. `room::turned_to` only knows about the two ways a cabinet can face
/// and is no use for a ceiling.
///
/// It sits where the bar sat, high on the panel for a plus cell and low for a
/// minus one. The word says which end now and the place no longer has to,
/// which makes the place a second telling rather than the only one.
pub fn plate(face: Face, cell: Cell) -> (Vec3, glam::Quat, Vec3) {
    use std::f32::consts::{FRAC_PI_2, PI};

    let span = CELL - BORDER * 2.0;
    let tall = span * WORD_TALL;
    let shape = blitzkit::text::room_for(named(cell), WORD_TEXELS);
    let wide = tall * (shape.x / shape.y.max(1e-4));
    let toward = if cell.up { 1.0 } else { -1.0 };
    let off = (span * 0.5 - STRIPE_IN - STRIPE * 0.5) * toward;

    let (at, _) = panel(face);
    // clear of the panel, into the room
    let out = -face.way() * PROUD * 2.0;

    let along = match face {
        Face::Down | Face::Up => Vec3::Z * off,
        _ => Vec3::Y * off,
    };

    let turn = match face {
        Face::Ahead => glam::Quat::from_rotation_y(FRAC_PI_2),
        Face::Back => glam::Quat::from_rotation_y(-FRAC_PI_2),
        Face::Right => glam::Quat::from_rotation_y(PI),
        Face::Left => glam::Quat::IDENTITY,
        // the floor and the ceiling need a second turn. The first points the
        // quad's face at the room; without the second the word runs away
        // along `ahead` instead of across the room, which is a name written
        // sideways and read upside down.
        Face::Down => {
            glam::Quat::from_rotation_y(-FRAC_PI_2) * glam::Quat::from_rotation_z(-FRAC_PI_2)
        }
        Face::Up => glam::Quat::from_rotation_y(FRAC_PI_2) * glam::Quat::from_rotation_z(FRAC_PI_2),
    };

    (at + along + out, turn, vec3(0.02, tall, wide))
}

// ---------------------------------------------------------------------------
// The map, which is what makes the room playable.
// ---------------------------------------------------------------------------

/// Which of the eight a cell is, in [`cells`] order.
///
/// The map, the drawn names and what has been found all index by this, so
/// there is one answer to "which one is this" rather than three.
pub fn numbered(cell: Cell) -> usize {
    cell.axis.n() * 2 + usize::from(!cell.up)
}

/// The cell the room asks you to reach.
///
/// The one opposite where you came in, which is spec 0021's own answer to
/// what there is to do. On the map it is the box directly under the one with
/// the plinth in it, which is the single cell you cannot walk to from there,
/// and that is the whole puzzle stated in one glance.
pub fn goal() -> Cell {
    opposite(home())
}

/// Which cells somebody has stood in.
///
/// The map fills in as it is walked rather than arriving complete. A map that
/// shows everything at once answers the question before it is asked; one that
/// fills in turns being lost into progress, which is the thing this room had
/// none of.
#[derive(Clone, Copy, Debug, Default)]
pub struct Found {
    been: [bool; 8],
}

impl Found {
    /// Starting out, having stood in the cell you came in by.
    pub fn new() -> Found {
        let mut out = Found::default();
        out.went(home());

        out
    }

    pub fn went(&mut self, cell: Cell) {
        self.been[numbered(cell)] = true;
    }

    pub fn been(&self, cell: Cell) -> bool {
        self.been[numbered(cell)]
    }

    pub fn count(&self) -> usize {
        self.been.iter().filter(|it| **it).count()
    }

    /// Whether the room has been solved: the far cell stood in.
    pub fn done(&self) -> bool {
        self.been(goal())
    }
}

// ---------------------------------------------------------------------------
// The crossing, which is the only part of this a person can be shown.
// ---------------------------------------------------------------------------

/// How long a crossing takes.
///
/// The first build had none: `step` was instantaneous and `arrive` put you
/// down, so walking at a wall was a scene change and everything that went on
/// top of it afterwards was signage explaining a transition that was never
/// drawn.
///
/// It did not save the room on its own, which is what the map is for, but a
/// crossing you can see is still the difference between a room turning and a
/// picture being swapped.
pub const TURN: f32 = 0.85;

/// When, through the turn, the room changes under you.
///
/// Halfway, which is the moment you are in the plane of the square the two
/// cells share. Both cells have that square as a face, so it is the one
/// instant when what is on screen belongs to either of them.
pub const SWAPS_AT: f32 = 0.5;

/// A crossing in progress.
#[derive(Clone, Copy, Debug)]
pub struct Turn {
    pub step: Step,
    pub from: Vec3,
    pub to: Vec3,
    /// Nought to one.
    pub through: f32,
    pub swapped: bool,
}

/// Eased, so the turn starts and ends still.
///
/// A linear one begins and ends with a jerk, which on a ninety degree roll
/// reads as the picture being yanked rather than the room going round.
pub fn eased(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

/// Which way the world goes round, and how far, for a given step.
///
/// The wall you walk at becomes the floor, so the direction you walked turns
/// into the direction down. Walking ahead is a pitch forward about your
/// right; stepping sideways is a roll about the way you are facing.
pub fn turning(step: Step) -> (Vec3, f32) {
    use std::f32::consts::FRAC_PI_2;

    match step {
        Step::Ahead => (Vec3::X, FRAC_PI_2),
        Step::Back => (Vec3::X, -FRAC_PI_2),
        Step::Right => (Vec3::Z, -FRAC_PI_2),
        Step::Left => (Vec3::Z, FRAC_PI_2),
    }
}

/// How far round the world is at a given moment of the turn.
///
/// **It jumps by a right angle at the halfway mark, and that is the trick.**
/// You tip through forty five degrees into the square the two cells share;
/// the room is drawn as the other cell from there on; and because that cell
/// is at right angles to the one you left, the tip you have built up now
/// reads as forty five degrees the other way and unwinds to nothing. Both
/// ends of a crossing are upright and all the turning happens inside it.
pub fn rolled(step: Step, through: f32) -> (Vec3, f32) {
    let (axis, whole) = turning(step);
    let t = eased(through);
    let part = if through < SWAPS_AT { t } else { t - 1.0 };

    (axis, whole * part)
}

/// Where you are partway through a crossing.
pub fn crossing_at(turn: &Turn) -> Vec3 {
    turn.from.lerp(turn.to, eased(turn.through))
}

/// What `--screenshot=` calls this room.
pub const SLUG: &str = "tesseract";

/// Which cell the way in and the way out stands in.
///
/// The one you arrive in, which is [`Frame::new`]'s, so the plinth is under
/// your hand when you get there and three steps away by the time you have
/// been anywhere. Spec 0021: eight rooms alike in everything but their marks,
/// and exactly one of them with the way out in the middle of the floor.
pub fn home() -> Cell {
    Frame::new().cell()
}

/// How tall the plinth in that cell is, and how wide.
pub const PLINTH_TALL: f32 = 1.05;
pub const PLINTH_WIDE: f32 = 0.52;

/// How far from it you have to be to put your hand on it.
pub const PLINTH_REACH: f32 = 1.5;

/// Where it stands, which is the middle of the floor.
pub fn plinth() -> (Vec3, Vec3) {
    (
        vec3(0.0, PLINTH_TALL * 0.5, 0.0),
        vec3(PLINTH_WIDE, PLINTH_TALL, PLINTH_WIDE),
    )
}

/// How big the shape standing on it is drawn, across its own widest way.
pub const SHAPE_ACROSS: f32 = 0.62;

/// Where that shape's middle sits.
pub fn shape_at() -> Vec3 {
    vec3(0.0, PLINTH_TALL + SHAPE_ACROSS * 0.62, 0.0)
}

/// Whether somebody standing here can reach the plinth.
///
/// The cell as well as the distance. Every room in here is the same room, and
/// a way out you could use from any of them would be a maze with eight exits.
pub fn at_the_plinth(cell: Cell, at: Vec3) -> bool {
    let (stands, _) = plinth();

    cell == home() && (at - vec3(stands.x, at.y, stands.z)).length() < PLINTH_REACH
}

/// Where somebody arriving in the tesseract is put down.
///
/// Off to one side of the plinth rather than on it, far enough back to see
/// the thing they just turned, and within reach of it so the way out is
/// under their hand the moment they arrive.
///
/// Nearer the middle than the wall. At `PLINTH_REACH + 0.2` the back wall was
/// 0.6 behind, and the first thing anybody does in a new room is step back.
pub fn arriving() -> Vec3 {
    vec3(0.0, 0.0, -(PLINTH_REACH - 0.3))
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

    /// One frame standing in each of the eight cells.
    ///
    /// Walked rather than constructed, because a frame built by hand is a
    /// frame nothing checked. Four steps and not one: stepping Ahead over and
    /// over visits three cells and comes home, which is what
    /// `three_steps_bring_you_back_exactly` says and what an earlier version
    /// of this spun forever on.
    fn one_frame_per_cell() -> Vec<Frame> {
        let start = Frame::new();
        let key = |f: Frame| (f.right, f.up, f.ahead, f.ana);
        let mut seen = std::collections::HashSet::new();
        let mut found = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::new();

        seen.insert(key(start));
        found.insert(start.cell(), start);
        queue.push_back(start);

        while let Some(frame) = queue.pop_front() {
            for step in Step::ALL.iter().copied() {
                let next = frame.step(step);
                if seen.insert(key(next)) {
                    found.entry(next.cell()).or_insert(next);
                    queue.push_back(next);
                }
            }
        }

        assert_eq!(found.len(), 8, "every cell should be stood in");

        let mut out: Vec<Frame> = found.into_values().collect();
        out.sort_by_key(|f| (f.cell().axis, f.cell().up));
        out
    }

    /// Spec 0021: the six faces of a cell name its six neighbours, and the
    /// two cells they do not name are the one you are in and its opposite.
    #[test]
    fn the_six_faces_name_the_six_neighbours() {
        for frame in one_frame_per_cell() {
            let mut named: Vec<Cell> = Face::ALL.iter().map(|it| frame.beyond(*it)).collect();
            named.sort_by_key(|cell| (cell.axis, cell.up));
            named.dedup();

            assert_eq!(named.len(), 6, "six faces, six different cells");

            let mut want = neighbours(frame.cell());
            want.sort_by_key(|cell| (cell.axis, cell.up));
            assert_eq!(named, want, "and they are the neighbours");

            assert!(
                !named.contains(&frame.cell()) && !named.contains(&opposite(frame.cell())),
                "the cell you are in and the one opposite are through no face"
            );
        }
    }

    /// Spec 0021: the floor is the room you came from, which is why you
    /// cannot walk back the way you came.
    ///
    /// The spec's prose says the room you left is overhead and is wrong about
    /// it. This is the test that says so: whatever step is taken, the cell
    /// you were standing in is through the floor of the one you arrive in,
    /// and the cell overhead is its opposite.
    #[test]
    fn the_room_you_came_from_is_under_the_floor() {
        for step in Step::ALL {
            let was = Frame::new();
            let now = was.step(step);

            assert_eq!(
                now.beyond(Face::Down),
                was.cell(),
                "{:?} should leave the old cell under the floor",
                step
            );
            assert_eq!(
                now.beyond(Face::Up),
                opposite(was.cell()),
                "and its opposite overhead"
            );
        }
    }

    /// Spec 0021: a crossing puts you on the floor, just inside the wall
    /// opposite the one you walked at, with the other way carried across.
    #[test]
    fn a_crossing_lands_you_inside_the_far_wall() {
        let cases = [
            (vec3(1.3, 0.0, HALF + 0.05), Step::Ahead),
            (vec3(-0.8, 0.0, -HALF - 0.05), Step::Back),
            (vec3(HALF + 0.05, 0.0, 0.4), Step::Right),
            (vec3(-HALF - 0.05, 0.0, -1.1), Step::Left),
        ];

        for (at, want) in cases {
            assert_eq!(crossing(at), Some(want), "walked past the {:?} wall", want);

            let now = arrive(at, want);

            assert_eq!(now.y, 0.0, "on the floor");
            assert!(
                crossing(now).is_none(),
                "and inside the next room rather than through it again"
            );

            // the way the step did not spend is carried over
            match want {
                Step::Ahead | Step::Back => assert_eq!(now.x, at.x),
                Step::Right | Step::Left => assert_eq!(now.z, at.z),
            }

            // and it is the far wall you come out of, not the near one
            let out = match want {
                Step::Ahead => now.z,
                Step::Back => -now.z,
                Step::Right => now.x,
                Step::Left => -now.x,
            };
            assert!(out < 0.0, "you come in at the back and walk forward");
        }
    }

    /// Nobody standing in the room is crossing anything.
    #[test]
    fn standing_in_the_room_crosses_nothing() {
        let inside = [
            middle(),
            vec3(0.0, 0.0, 0.0),
            vec3(HALF - 0.01, 1.0, HALF - 0.01),
            vec3(-HALF + 0.01, 0.0, -HALF + 0.01),
        ];

        for at in inside {
            assert_eq!(crossing(at), None, "{:?} is inside", at);
        }
    }

    /// Spec 0021: three steps bring you home, and they bring your feet home
    /// as well as your frame.
    ///
    /// The frame's own test says the four directions come back. This says the
    /// place does: walking a triangle and arriving three times puts you where
    /// a fourth crossing would put you, which is the same spot each time
    /// round, so the room is a loop rather than a drift.
    #[test]
    fn three_steps_bring_your_feet_back_too() {
        let mut frame = Frame::new();
        let mut at = vec3(1.1, 0.0, 0.0);
        let mut first = None;

        for n in 0..3 {
            // walk at the wall ahead
            let walked = vec3(at.x, at.y, HALF + 0.02);
            let step = crossing(walked).expect("walked past it");

            frame = frame.step(step);
            at = arrive(walked, step);

            if n == 0 {
                first = Some(at);
            }
        }

        assert_eq!(frame, Frame::new(), "the frame is home");
        assert_eq!(
            at,
            first.expect("set on the first step"),
            "and so are the feet"
        );
    }

    /// The floor holds you up and the ceiling stops you, and nothing else is
    /// solid, because the walls are the way out.
    #[test]
    fn only_the_floor_and_the_ceiling_are_solid() {
        let solid = solid();
        assert_eq!(solid.len(), 2, "a floor and a ceiling");

        let floor = solid[0];
        let ceiling = solid[1];

        assert_eq!(floor.max.y, 0.0, "you walk on y nought");
        assert_eq!(ceiling.min.y, CELL, "and the room is a cube tall");

        // and both reach past every wall, so there is no gap at a corner to
        // fall through on the way out
        for one in [floor, ceiling] {
            assert!(one.min.x <= -HALF && one.max.x >= HALF);
            assert!(one.min.z <= -HALF && one.max.z >= HALF);
        }
    }

    /// Spec 0021: eight rooms, eight different things, one each.
    ///
    /// The whole of what makes this room navigable rests on the eight being
    /// eight. Two rooms with a teapot in them and every sign on every wall is
    /// a coin toss.
    #[test]
    fn every_room_has_its_own_thing() {
        let mut seen = Vec::new();

        for cell in cells() {
            let thing = thing(cell);

            assert!(
                !seen.contains(&thing),
                "{:?} is in {} and in another room too",
                thing,
                named(cell)
            );
            seen.push(thing);
        }

        assert_eq!(seen.len(), 8, "one for each cell");
        assert_eq!(Thing::ALL.len(), 8, "and no spares");

        for (n, thing) in Thing::ALL.iter().enumerate() {
            assert_eq!(thing.n(), n, "{:?} knows which one it is", thing);
        }
    }

    /// Spec 0021: the thing on a wall is the thing in the room you land in.
    ///
    /// **This is the promise the room is built on**, in Jake's own words: if
    /// I walk through one, where will it take me. Walking at the wall with
    /// the teapot on it puts you in the room with the teapot in it, from any
    /// cell and standing any way up in it.
    ///
    /// On [`sign_on`], which is what the drawing calls, and against the cell
    /// that [`Frame::step`] actually arrives in. Written first as `thing` of
    /// the face against `thing` of the step it passed with the assignment
    /// permuted any way at all, because both sides went through the same
    /// table: it was restating [`the_six_faces_name_the_six_neighbours`] and
    /// nothing else. The control that holds now is `sign_on` returning
    /// `thing(frame.cell())`, which fails on every face of every cell.
    #[test]
    fn a_wall_shows_the_thing_in_the_room_you_land_in() {
        for frame in one_frame_per_cell() {
            for face in Face::ALL {
                let shown = sign_on(frame, face);

                // never the room you are already standing in, whichever face
                // it is, which is the fault a sign read off the wrong frame
                // falls into
                assert_ne!(
                    shown,
                    thing(frame.cell()),
                    "in {} the {:?} wall points at {}",
                    named(frame.cell()),
                    face,
                    named(frame.cell())
                );

                let Some(step) = face.step() else {
                    continue;
                };

                let landed = thing(frame.step(step).cell());

                assert_eq!(
                    shown,
                    landed,
                    "in {} the {:?} wall shows {:?} and lands you with {:?}",
                    named(frame.cell()),
                    face,
                    shown,
                    landed
                );
            }
        }
    }

    /// And the sign stands in the room, clear of the panel it is on, clear of
    /// the plinth, and out of the way of anybody walking.
    ///
    /// A sign sunk into its own wall is half a shape, and one in the middle
    /// of the floor is furniture. Both were drawn before this was written.
    #[test]
    fn a_wall_sign_is_in_the_room_and_out_of_the_way() {
        let (plinth_at, plinth_size) = plinth();
        let plinth = Aabb::from_center_size(plinth_at, plinth_size);

        for face in Face::ALL {
            // the floor and the ceiling have none, because they are the two
            // faces nobody can walk at
            let Some(at) = sign_at(face) else {
                assert!(
                    face.step().is_none(),
                    "the {:?} wall can be walked at and has no sign",
                    face
                );
                continue;
            };
            let sign = Aabb::from_center_size(at, Vec3::splat(SIGN_ACROSS));

            // inside the cell, which is the cube between the floor and the
            // ceiling and out to each wall
            assert!(
                at.x.abs() <= HALF && at.z.abs() <= HALF && at.y >= 0.0 && at.y <= CELL,
                "the {:?} sign is at {:?}, which is outside the room",
                face,
                at
            );

            // and proud of its own panel rather than in it, by enough that
            // the furthest corner of its own bounding cube clears the wall
            // at the angle it is turned to. Half the width is not that
            // number: a cube turned about its upright reaches 0.707 of its
            // side along a horizontal axis, not 0.5.
            let depth = HALF - at.dot(face.way());
            let corner = SIGN_ACROSS * std::f32::consts::FRAC_1_SQRT_2;

            assert!(
                depth > corner,
                "the {:?} sign at {:?} stands {} off its wall and its own \
                 corner reaches {}",
                face,
                at,
                depth,
                corner
            );
            assert!(
                depth < SIGN_ACROSS * 2.0,
                "the {:?} sign at {:?} is out in the room rather than on its wall",
                face,
                at
            );

            // and nothing of it is in the plinth, or in the thing standing
            // on the plinth
            assert!(
                !plinth.intersects(&sign),
                "the {:?} sign at {:?} is inside the plinth",
                face,
                at
            );
            assert!(
                (at - thing_at()).length() > (THING_ACROSS + SIGN_ACROSS) * 0.5,
                "the {:?} sign at {:?} is inside the thing on the plinth",
                face,
                at
            );
        }
    }

    /// Each square of the cross carries its thing on the side the holder is
    /// on, square by square, and the whole of it stays inside the hand it is
    /// held in rather than reaching the eye.
    #[test]
    fn a_square_of_the_cross_carries_its_thing_toward_you() {
        for face in Face::ALL {
            let Some((at, across)) = face_thing(face) else {
                assert!(
                    cross_at(face).is_none(),
                    "{:?} has a square and no thing on it",
                    face
                );
                continue;
            };
            let (middle, _) = cross_at(face).expect("a square to carry it");

            // on the square it belongs to, and nowhere near another
            assert!(
                (at.truncate() - middle.truncate()).length() < 1e-5,
                "the {:?} thing sits at {:?} and its square is at {:?}",
                face,
                at,
                middle
            );

            // toward the holder rather than behind the square, which in
            // this basis is plus z, because its third axis points back at
            // the eye. Written the other way round first, which hid every
            // object behind the square it belonged to.
            assert!(
                at.z > middle.z,
                "the {:?} thing is behind its own square",
                face
            );

            // and touching it rather than floating off it
            assert!(
                at.z - middle.z < across,
                "the {:?} thing is adrift of its square",
                face
            );

            // and the far side of it is still nearer than the map is held,
            // so nothing of it is behind your eye
            assert!(
                ROOM_UP + CROSS_STEP + across < HELD_OUT,
                "the cross and the things on it reach past where it is held"
            );
        }
    }

    /// A four dimensional cube has sixteen corners and thirty two edges, and
    /// every corner is on four of them, one for each direction.
    #[test]
    fn the_shape_on_the_plinth_is_a_four_cube() {
        let corners = corners();
        let edges = edges();

        assert_eq!(corners.len(), 16);
        assert_eq!(edges.len(), 32);

        for n in 0..16 {
            let on = edges.iter().filter(|(a, b)| *a == n || *b == n).count();
            assert_eq!(on, 4, "corner {} is on {} edges", n, on);
        }

        // and the projection makes two cubes, one inside the other, which is
        // what a drawn tesseract is. The eight far along w come out smaller.
        let near: Vec<f32> = (0..8).map(|n| corners[n].length()).collect();
        let far: Vec<f32> = (8..16).map(|n| corners[n].length()).collect();
        let biggest = near.iter().copied().fold(0.0f32, f32::max);
        let smallest = far.iter().copied().fold(f32::MAX, f32::min);

        assert!(
            biggest < smallest,
            "every corner at w minus one should be inside every corner at w plus one"
        );
    }

    /// Every panel is inside the room, clear of the faces it is not on, and
    /// standing off the one it is.
    #[test]
    fn the_panels_hang_inside_the_room() {
        for face in Face::ALL {
            let (at, size) = panel(face);
            let half = size * 0.5;

            // across the face, not through it: the panel is meant to reach
            // past its own face and into the shell, and only the two ways it
            // lies in are bounded by the room
            let across = Vec3::ONE - face.way().abs();
            let low = (at - half - middle()) * across;
            let high = (at + half - middle()) * across;

            assert!(
                low.min_element() >= -HALF - 1e-4 && high.max_element() <= HALF + 1e-4,
                "{:?} reaches past the room",
                face
            );

            // it stands into the room, and its back is inside the shell
            // rather than flush with it, because two surfaces in one plane
            // fight and a face inside a solid is hidden
            let stands = (at - middle()).dot(face.way()) - half.dot(face.way().abs());
            let backs = (at - middle()).dot(face.way()) + half.dot(face.way().abs());

            assert!(
                stands < HALF - 1e-4,
                "{:?} does not stand into the room",
                face
            );
            assert!(backs > HALF + 1e-4, "{:?} is flush with the shell", face);
        }
    }

    /// Spec 0021: a cell's written name hangs on the panel, in front of it
    /// rather than behind, and the two ends of an axis put it on opposite
    /// sides.
    ///
    /// The second half is what the first build got wrong and could not see.
    /// The mark stands off its panel along the face's own way, and that way
    /// points out of the room, so adding it buried every one of them in the
    /// shell where they were drawn each frame and never once seen.
    #[test]
    fn the_name_hangs_where_it_can_be_read() {
        for face in Face::ALL {
            let (field, _) = panel(face);

            let (up, _, _) = plate(face, Way::new(Axis::X, true));
            let (down, _, _) = plate(face, Way::new(Axis::X, false));

            for at in [up, down] {
                let stands = (at - field).dot(face.way());
                assert!(
                    stands < -1e-4,
                    "the {:?} name is behind its panel rather than in front",
                    face
                );
            }

            assert_ne!(up, down, "{:?} marks both ends the same", face);
            assert!(
                (up - field).dot(down - field) < 0.0,
                "{:?} puts both ends on one side",
                face
            );
        }
    }

    /// And a name is drawn the shape of its own letters rather than squashed
    /// into one box, so the short ones do not come out fat.
    #[test]
    fn a_short_name_is_not_stretched_to_a_long_one() {
        let (_, _, short) = plate(Face::Ahead, Way::new(Axis::W, true));
        let (_, _, long) = plate(Face::Ahead, Way::new(Axis::Y, true));

        assert_eq!(named(Way::new(Axis::W, true)), "ana");
        assert_eq!(named(Way::new(Axis::Y, true)), "zenith");
        assert_eq!(short.y, long.y, "every name is drawn the same height");
        assert!(
            short.z < long.z,
            "ana came out {:.2} wide against zenith's {:.2}",
            short.z,
            long.z
        );
    }

    /// A name is square to its face, and on a wall it stands upright.
    ///
    /// What this catches is a word drawn sideways, which is what the floor
    /// and the ceiling were: pointing the quad at the room takes one turn and
    /// leaves the letters running along whichever way the first turn happened
    /// to leave them, so "nadir" ran away across the floor instead of along
    /// it.
    ///
    /// What it cannot catch is a word drawn backwards. Whether a texture
    /// comes out mirrored is decided by the winding and the u direction
    /// together, and `room::screen_mesh` says in its own comment that the
    /// only reliable way to settle that is to draw a word and read it. Four
    /// candidate turns drawn side by side settled it in one picture, after
    /// two builds spent deriving it wrongly.
    #[test]
    fn a_name_faces_into_the_room_and_stands_up() {
        for face in Face::ALL {
            let (_, turn, _) = plate(face, Way::new(Axis::X, true));

            // `room::screen_mesh` faces its own +x, with the letters running
            // along z and standing along y
            let looks = turn * Vec3::X;
            let stands = turn * Vec3::Y;
            let runs = turn * Vec3::Z;

            // square to its face, either way about. `room::hung_mesh` is two
            // sided and the back carries mirrored u on purpose, so a word
            // reads forwards from both and which side it presents is not
            // what matters. The floor's shows its back and reads correctly.
            assert!(
                looks.dot(face.way()).abs() > 1.0 - 1e-4,
                "the {:?} name faces {:?}, which is not square to its face",
                face,
                looks
            );

            match face {
                Face::Down | Face::Up => {
                    assert!(
                        stands.y.abs() < 1e-4 && runs.y.abs() < 1e-4,
                        "a name laid flat should have nothing of it upright, got {:?} and {:?}",
                        stands,
                        runs
                    );
                    // and it runs across the room rather than along it.
                    // Flat and horizontal is not enough: the first build's
                    // floor name was both, and ran away down `ahead` so you
                    // read it side on. This is the assertion that was
                    // missing, found by breaking the fix and watching the
                    // test pass anyway.
                    assert!(
                        runs.x.abs() > 1.0 - 1e-4,
                        "the {:?} name runs along {:?} rather than across the room",
                        face,
                        runs
                    );
                }
                _ => {
                    assert!(
                        (stands - Vec3::Y).length() < 1e-4,
                        "the {:?} name leans: it stands along {:?}",
                        face,
                        stands
                    );
                    assert!(
                        runs.y.abs() < 1e-4,
                        "the {:?} name runs uphill along {:?}",
                        face,
                        runs
                    );
                }
            }
        }
    }

    /// Spec 0021: what there is to do is reach the cell opposite the one you
    /// came in by, which is the one the map puts under the way out.
    #[test]
    fn the_goal_is_the_one_box_the_way_out_cannot_reach() {
        assert_eq!(goal(), opposite(home()));
        assert!(!meets(goal(), home()), "the goal would be one step away");

        // and on the shape you carry they are the inner cube and the outer
        // one, which is why they share no bar and cannot touch
        let out = corners_of(home());
        assert!(corners_of(goal()).iter().all(|n| !out.contains(n)));
    }

    /// The map fills in as it is walked, and starts knowing only where you
    /// came in.
    #[test]
    fn the_map_fills_in_as_it_is_walked() {
        let mut found = Found::new();

        assert_eq!(found.count(), 1, "it should start knowing one cell");
        assert!(found.been(home()));
        assert!(!found.done());

        // walk the shortest route to the far cell, found rather than
        // guessed. An earlier version of this walked a route picked by hand
        // and never arrived, which is the room's whole problem in miniature.
        let key = |f: Frame| (f.right, f.up, f.ahead, f.ana);
        let mut seen = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        seen.insert(key(Frame::new()));
        queue.push_back((Frame::new(), Vec::new()));

        let route = loop {
            let (frame, so_far) = queue.pop_front().expect("the room is connected");

            if frame.cell() == goal() {
                break so_far;
            }

            for step in Step::ALL {
                let next = frame.step(step);
                if seen.insert(key(next)) {
                    let mut path = so_far.clone();
                    path.push(step);
                    queue.push_back((next, path));
                }
            }
        };

        assert_eq!(route.len(), 3, "the far cell should be three steps off");

        let mut frame = Frame::new();
        for step in route {
            frame = frame.step(step);
            found.went(frame.cell());
        }

        assert_eq!(frame.cell(), goal());
        assert!(found.done(), "standing in the far cell should finish it");
        assert_eq!(found.count(), 4, "one to start and three walked");
    }

    /// Spec 0021: a crossing turns the world through a right angle, and both
    /// ends of it are upright.
    ///
    /// The one part of a four dimensional rotation a person can be shown. The
    /// first build had none of it: `step` was instantaneous, so walking at a
    /// wall was a scene change and everything afterwards was signage
    /// explaining a transition that was never drawn.
    #[test]
    fn a_crossing_turns_the_world_and_ends_upright() {
        for step in Step::ALL {
            let (_, start) = rolled(step, 0.0);
            let (_, end) = rolled(step, 1.0);

            assert!(start.abs() < 1e-5, "{:?} does not start upright", step);
            assert!(end.abs() < 1e-5, "{:?} does not end upright", step);

            // and in between it is turning. Halfway is where the room changes
            // under you, and the jump across that instant is the right angle
            // the crossing is made of.
            let (_, before) = rolled(step, SWAPS_AT - 1e-4);
            let (_, after) = rolled(step, SWAPS_AT);
            let jump = (after - before).abs();

            assert!(
                (jump - std::f32::consts::FRAC_PI_2).abs() < 1e-3,
                "{:?} jumps {:.3} at the swap rather than a right angle",
                step,
                jump
            );
            // opposite signs rather than a particular one: Back and Right
            // turn the other way about, which is what their own walls being
            // the floor means.
            assert!(
                before * after < 0.0,
                "{:?} should lean one way into the swap and the other out of it, got {:.3} and {:.3}",
                step,
                before,
                after
            );
        }
    }

    /// The direction you walked becomes the direction down, which is what
    /// makes the wall you walked at the floor you land on.
    #[test]
    fn the_way_you_walked_turns_into_the_way_down() {
        for step in Step::ALL {
            let (axis, whole) = turning(step);
            let round = glam::Quat::from_axis_angle(axis, whole);

            let walked = match step {
                Step::Ahead => Vec3::Z,
                Step::Back => Vec3::NEG_Z,
                Step::Right => Vec3::X,
                Step::Left => Vec3::NEG_X,
            };

            assert!(
                (round * walked - Vec3::NEG_Y).length() < 1e-4,
                "{:?} turns {:?} into {:?} rather than down",
                step,
                walked,
                round * walked
            );
        }
    }

    /// The turn starts and ends still, because a linear one reads as the
    /// picture being yanked rather than the room going round.
    #[test]
    fn the_turn_starts_and_ends_still() {
        assert!(eased(0.0).abs() < 1e-6);
        assert!((eased(1.0) - 1.0).abs() < 1e-6);
        assert!((eased(0.5) - 0.5).abs() < 1e-6);

        let creep = eased(0.02) / 0.02;
        let middle = (eased(0.52) - eased(0.48)) / 0.04;

        assert!(
            creep < middle * 0.5,
            "it should be slower at the start than in the middle: {:.3} against {:.3}",
            creep,
            middle
        );
    }

    /// And it carries you from where you crossed to where you land, moving
    /// nowhere at either end of it.
    #[test]
    fn a_crossing_carries_you_from_the_wall_to_the_far_side() {
        let from = vec3(1.1, 0.0, HALF + 0.02);
        let step = crossing(from).expect("walked past the wall");
        let to = arrive(from, step);

        let turn = Turn {
            step,
            from,
            to,
            through: 0.0,
            swapped: false,
        };

        let at_start = crossing_at(&Turn {
            through: 0.0,
            ..turn
        });
        let at_end = crossing_at(&Turn {
            through: 1.0,
            ..turn
        });

        assert_eq!(at_start, from);
        assert_eq!(at_end, to);

        // and the middle of it is somewhere between, not at either end
        let halfway = crossing_at(&Turn {
            through: 0.5,
            ..turn
        });
        assert!((halfway - from).length() > 1e-3);
        assert!((halfway - to).length() > 1e-3);
    }

    /// A cell is a cube: eight of the sixteen corners and twelve of the
    /// thirty two bars, and the two cells on one axis share none of either.
    #[test]
    fn a_cell_is_eight_corners_and_twelve_bars_of_the_shape() {
        for cell in cells() {
            let corners = corners_of(cell);
            let edges = edges_of(cell);

            assert_eq!(
                corners.len(),
                8,
                "{:?} is not a cube's worth of corners",
                cell
            );
            assert_eq!(edges.len(), 12, "{:?} is not a cube's worth of bars", cell);

            // the other end of its own axis shares nothing with it, which is
            // the one pair that does not meet
            let far = corners_of(opposite(cell));
            assert!(
                corners.iter().all(|n| !far.contains(n)),
                "{:?} shares a corner with its own opposite",
                cell
            );

            // and every cell it does meet shares a square: four corners
            for other in neighbours(cell) {
                let shared = corners_of(other)
                    .into_iter()
                    .filter(|n| corners.contains(n))
                    .count();

                assert_eq!(
                    shared, 4,
                    "{:?} and {:?} share {} corners rather than a square",
                    cell, other, shared
                );
            }
        }

        // and every bar belongs to exactly three cells.
        //
        // **Three and not two**, which this said first because a cube's edge
        // is on two of its faces and that is one dimension short. A bar runs
        // along one axis, so the other three of its coordinates are fixed,
        // and each of those three names a cell it is in.
        for edge in edges() {
            let on = cells()
                .iter()
                .filter(|cell| edges_of(**cell).contains(&edge))
                .count();

            assert_eq!(on, 3, "{:?} is on {} cells", edge, on);
        }
    }

    /// Spec 0021: the map lines up with the room it is a map of.
    ///
    /// The part of the shape on your left is the cell through the wall on
    /// your left, the part ahead is the cell ahead, and the cell you are
    /// standing in is the outer cube with the one you cannot reach inside it.
    ///
    /// This is what makes it a map rather than a picture of a tesseract, and
    /// it is what the first version did not have: it was built in the world's
    /// axes and spun on its own, so nothing on it meant anything about the
    /// room.
    #[test]
    fn the_held_map_lines_up_with_the_room() {
        // every way of standing, not just the one you arrive in
        let key = |f: Frame| (f.right, f.up, f.ahead, f.ana);
        let mut seen = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        seen.insert(key(Frame::new()));
        queue.push_back(Frame::new());

        while let Some(frame) = queue.pop_front() {
            let corners = corners_in(frame);
            let middle = |cell: Cell| {
                let mine = corners_of(cell);
                mine.iter().map(|n| corners[*n]).sum::<Vec3>() / mine.len() as f32
            };

            // the four walls, each where its own wall is
            for (step, way) in [
                (Step::Ahead, Vec3::Z),
                (Step::Back, Vec3::NEG_Z),
                (Step::Right, Vec3::X),
                (Step::Left, Vec3::NEG_X),
            ] {
                let at = middle(frame.through(step));

                assert!(
                    at.dot(way) > 1e-3,
                    "{:?} sits at {:?}, which is not {:?} of the shape",
                    step,
                    at,
                    way
                );
            }

            // and the floor and the ceiling likewise
            assert!(middle(frame.beyond(Face::Up)).y > 1e-3);
            assert!(middle(frame.beyond(Face::Down)).y < -1e-3);

            // the cell you are standing in is the outer cube, and the one you
            // cannot reach is inside it
            let out = corners_of(frame.cell())
                .iter()
                .map(|n| corners[*n].length())
                .fold(f32::MAX, f32::min);
            let inner = corners_of(opposite(frame.cell()))
                .iter()
                .map(|n| corners[*n].length())
                .fold(0.0f32, f32::max);

            assert!(
                inner < out,
                "the cell you are in should be the outer cube: {:.3} against {:.3}",
                out,
                inner
            );

            for step in Step::ALL {
                let next = frame.step(step);
                if seen.insert(key(next)) {
                    queue.push_back(next);
                }
            }
        }

        assert_eq!(seen.len(), 96, "every way of standing should be checked");
    }

    /// Spec 0021: the cross is the four walls you can walk at, laid out the
    /// way a compass is drawn.
    ///
    /// Up is the wall in front of you, down is the one behind, left is left.
    /// Nothing is hidden behind anything, which is the whole of why this is
    /// not a cube any more.
    #[test]
    fn the_cross_lays_out_the_four_walls_flat() {
        let want = [
            (Face::Ahead, Vec3::Y),
            (Face::Back, Vec3::NEG_Y),
            (Face::Right, Vec3::X),
            (Face::Left, Vec3::NEG_X),
        ];

        for (face, along) in want {
            let (at, size) = cross_at(face).expect("a wall you can walk at");

            assert!(
                (at - along * CROSS_STEP).length() < 1e-5,
                "the {:?} square sits at {:?} rather than {:?} of the middle",
                face,
                at,
                along
            );

            // square, and flat: all four face the holder rather than each
            // standing at its own angle
            assert!(
                (size.x - size.y).abs() < 1e-5 && size.z < size.x * 0.5,
                "the {:?} square is {:?}, which is not a flat square",
                face,
                size
            );

            // and clear of its neighbours, or the cross is one shape
            for (other, _) in want {
                if other == face {
                    continue;
                }

                let (beside, _) = cross_at(other).expect("another wall");

                assert!(
                    (at - beside).length() > size.x,
                    "the {:?} and {:?} squares overlap",
                    face,
                    other
                );
            }
        }

        // the floor and the ceiling are not on it, because they are the two
        // nobody can walk at
        for face in [Face::Down, Face::Up] {
            assert!(cross_at(face).is_none(), "{:?} is on the cross", face);
        }

        // and what is through the four is four different cells, none of them
        // the one you are standing in
        let frame = Frame::new();
        let mut through: Vec<Cell> = want.iter().map(|(f, _)| frame.beyond(*f)).collect();

        through.sort_by_key(|cell| (cell.axis, cell.up));
        through.dedup();

        assert_eq!(through.len(), 4, "the four walls name four rooms");
        assert!(!through.contains(&frame.cell()));
        assert!(!through.contains(&opposite(frame.cell())));
    }
}
