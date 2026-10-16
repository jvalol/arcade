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
/// how far apart the furniture goes and letting the ceiling be a ceiling. This
/// is as small as it can be and still be walked across, and that makes it
/// taller than a room of this footprint has any business being. The height is
/// the price of the trick and reads as part of it.
pub const CELL: f32 = 4.6;

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
/// through another one. This is far enough in to turn round in.
pub const INSET: f32 = 1.25;

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

/// The same, plus whatever is standing in this particular cell.
///
/// Which is the plinth, in the one cell that has it. Eight rooms alike in
/// everything but their marks and one of them with the way out in the middle
/// of the floor, so the floor of that one is not quite clear.
pub fn solid_in(cell: Cell) -> Vec<Aabb> {
    let mut out = solid();

    if cell == home() {
        let (at, size) = plinth();
        out.push(Aabb::from_center_size(at, size));
    }

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
}
