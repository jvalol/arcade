//! The field off the garden: ground that does not run out, and a horizon you
//! cannot walk to. Spec 0040.
//!
//! **Nothing here is a place the way the rest of this building is.** Every
//! other room is a list of boxes at fixed coordinates, written once and asked
//! about for ever. A field is too big to write down, so what is written down
//! is the rule: a cell of ground is eight metres square, its contents come out
//! of its own coordinates, and asking the same cell twice gives the same
//! answer for ever without anything having been kept.

use blitzkit::collision::Aabb;
use glam::{vec3, Vec3};

/// How big a cell of the field is.
///
/// Eight metres, which is a tree's crown and a half. Small enough that the
/// cells near you are a handful and large enough that there are not thousands
/// of them out at the tree line.
pub const CELL: f32 = 8.0;

/// How far out each kind of thing is stood, in metres.
///
/// **Trees far and ground cover near.** A tree at seventy metres is four
/// pixels of trunk and is still what tells you the ground goes on; a tuft of
/// grass at seventy metres is a quarter of a pixel and costs the same to draw
/// as one at your feet. What the far distance is made of is ground and sky.
pub const TREES_REACH: f32 = GROUND_REACH;
pub const COVER_REACH: f32 = 22.0;

/// How far the ground slab reaches from the middle of itself.
///
/// **A hundred and sixty metres, which is a horizon and not an edge.** The
/// true horizon of a flat plane is at eye level exactly, because the plane
/// runs to infinity; the edge of a slab a hundred and sixty metres off is
/// 0.55 degrees under it, which at this window's sixty degrees over a
/// thousand pixels is nine pixels. The difference between this and a plane
/// with no edge at all is nine pixels of sky, once, at the skyline.
///
/// And it follows you, so the nine pixels never become ten.
///
/// What stops it being larger is the sky, which has to clear it: see
/// [`SKY_BY`].
pub const GROUND_REACH: f32 = 160.0;

/// How big one tile of the ground's own texture is.
///
/// Named because the slab is snapped to it: see [`ground`].
pub const TILE: f32 = 4.0;

/// The gap in the garden's west wall, as the two `z` it runs between.
///
/// Said here rather than in `garden`, because the garden is the room that
/// loses a piece of wall and the field is the reason. The garden's own way in
/// is `garden::way`, which is the hall's wall, and the two would read as one
/// number written twice.
///
/// Wide, because what is through it is the whole point of it and a slot
/// makes a field look like a painting hung in a doorway.
pub const WAY_WIDE: f32 = 3.2;

pub fn way(reaches: f32) -> (f32, f32) {
    let middle = crate::garden::at(reaches).z;

    (middle - WAY_WIDE * 0.5, middle + WAY_WIDE * 0.5)
}

/// Where the garden stops and the field starts.
pub fn starts(reaches: f32) -> f32 {
    crate::garden::at(reaches).x - crate::garden::half().x
}

/// Whether the field's ground is what is holding you up.
///
/// **Asked of what is under you rather than of where you are.** Every
/// region this has been written as has had an edge somewhere, and each time
/// the edge was somewhere nobody had thought to look: first east of the
/// garden's west face, then round the garden's corner, then the strip north
/// of it, which had grass drawn on it and nothing under it for eleven
/// hundred square metres.
///
/// A region cannot help being a region. This is not one: if nothing of the
/// building is under your feet, the field is, and there is nowhere left for
/// an edge to be. The building's own floors win where there are any,
/// because they are two centimetres higher. See [`DROP`].
///
/// **Asked at any height and not only above the ground.** It tested that
/// you were above -0.1 first, so the moment anything put a body a hand's
/// breadth under the grass the grass stopped being there and the body
/// fell for ever. A wandering body found that in half a minute. Nothing
/// needs the height test: in the cellar and the baths it is their own
/// floors that are under you and the field is not asked.
pub fn afoot(solid: &[Aabb], at: Vec3) -> bool {
    // the rooms under this one have their own floors and their own
    // ceilings and want nothing from the field
    if at.y < -2.0 {
        return false;
    }

    // and a body inside a box of the building is in the building,
    // whatever is under it. The picture of the space behind the wall
    // stands at the bottom of that room's own boxes, a floor's thickness
    // under its floor, where nothing of the building is *under* the feet
    // because the feet are in the floor.
    if solid.iter().any(|box_| {
        box_.min.x <= at.x
            && box_.max.x >= at.x
            && box_.min.z <= at.z
            && box_.max.z >= at.z
            && box_.min.y <= at.y
            && box_.max.y >= at.y
    }) {
        return false;
    }

    // **A floor only counts if it is at least as high as the field's
    // would be.** The baths' roof is thirty centimetres down and runs out
    // past the garden, and counted as a floor it took the field's ground
    // away over the whole of it: a body walking east from the grass had
    // the wall vanish from under its nose and carried on into the nook.
    !solid.iter().any(|box_| {
        box_.min.x <= at.x
            && box_.max.x >= at.x
            && box_.min.z <= at.z
            && box_.max.z >= at.z
            && box_.max.y >= -DROP - 1e-3
            && box_.max.y <= at.y + 0.2
    })
}

/// Whether a spot is out in the field rather than in the building.
///
/// Measured against the garden's west face and nothing else. That is right
/// for the way out and for everywhere you can get to from it along this
/// wall, and it makes the field a half plane rather than a plane: see
/// `tests::the_field_has_one_edge_and_you_cannot_reach_it`, which says
/// where that edge is and what is still open about it.
pub fn open(reaches: f32, at: Vec3) -> bool {
    at.y > -0.1 && at.x < starts(reaches)
}

/// The ground you are standing on, which is a slab that follows you.
///
/// **Snapped to the ground's own texture, which is the whole trick.** A slab
/// that followed exactly would carry its texture with it, so walking would
/// move the trees past you over ground that never moved, and the ground is
/// half of what tells you that you are moving at all. Snapped to a whole
/// number of tiles it stands still in the world and jumps a tile at a time,
/// and a jump of a tile is a jump of nothing, because what it jumps to looks
/// the same as what it left.
pub fn ground(at: Vec3) -> Aabb {
    let middle = vec3(
        (at.x / TILE).round() * TILE,
        -crate::room::THICK * 0.5 - DROP,
        (at.z / TILE).round() * TILE,
    );

    Aabb::from_center_size(
        middle,
        vec3(GROUND_REACH * 2.0, crate::room::THICK, GROUND_REACH * 2.0),
    )
}

/// How far the field's ground lies under the garden's.
///
/// **Because the field is drawn from inside the garden too.** You can see
/// the whole of it through the gap from anywhere in there, so it has to be
/// drawn from anywhere in there, and a slab three hundred metres across at
/// the garden's own floor height is two surfaces in one plane: the fault
/// this building has fought five times. Two centimetres under it, the
/// garden's floor covers it wherever the two overlap and nothing shares a
/// plane with anything.
///
/// What it costs is a two centimetre step down on the way out, which `walk`
/// takes as a step and nobody can see.
pub const DROP: f32 = 0.02;

/// Whether the field can be seen from here, which is a different question
/// from whether you are standing in it.
///
/// **The same question [`afoot`] answers, plus the garden.** It was
/// `garden::under_the_sky` alone, which is a quarter plane: everything
/// west of the hall's left wall and north of the nook's end. South of
/// that line the field was solid and not drawn, so you stood on grass
/// nobody had painted and looked straight into the nook with black where
/// the sky should be. Jake walked out of the gap, turned south and was
/// there in four seconds.
///
/// Regions again. So it is not a region: if the field is what is under
/// you, the field is what you can see, and the garden says so for itself
/// because its own floor is what is under you in there.
pub fn in_view(reaches: f32, solid: &[Aabb], at: Vec3) -> bool {
    afoot(solid, at) || crate::garden::under_the_sky(reaches, at)
}

/// How much bigger the field's sky is than the garden's.
///
/// **The rim is the garden's own sixteen metres and not its nineteen.**
/// `garden::DOME` is how big the sky's picture is; what the vault actually
/// springs from is `garden::vault_rim`, which traces the sixteen metre
/// garden. Scaled for the picture's number the sky stopped eight metres
/// inside the ground, with the clear colour under the horizon in a ring all
/// the way round.
///
/// Twenty two puts the rim at 176 along the axes and 249 at the corners,
/// against a ground that reaches 160 and 226. The corners are what have to
/// clear: a dome sized on the sides is one the ground runs out from under at
/// forty five degrees.
pub const SKY_BY: f32 = 22.0;

/// How far the camera sees out here.
///
/// Past the sky's own corner, which is the furthest thing there is: short of
/// it the dome is clipped and what is behind the clip is the clear colour,
/// which out here is black. The first build of this ran on the building's
/// hundred metres and was a field in a cave.
pub const SEES: f32 = 420.0;

/// What the sun's shadow map covers out here.
///
/// **As much of the field as has anything in it to cast.** The map is one
/// texture over whatever it is given, so the wider the box the coarser the
/// shadows: the building's own is 40 metres across and that is already a
/// couple of centimetres a texel. Round you rather than round the room,
/// because out here there is no room.
pub const SHADOWS_REACH: f32 = GROUND_REACH;

pub fn under_the_sun(at: Vec3) -> Aabb {
    Aabb::from_center_size(
        vec3(at.x, TREE_TALL * 0.75, at.z),
        vec3(SHADOWS_REACH * 2.0, TREE_TALL * 2.5, SHADOWS_REACH * 2.0),
    )
}

/// Which cell a spot is in.
pub fn cell_of(at: Vec3) -> (i32, i32) {
    ((at.x / CELL).floor() as i32, (at.z / CELL).floor() as i32)
}

/// Every cell within `reach` of a spot.
///
/// **A ring and not a square**, because a square of cells out to seventy
/// metres is a corner at a hundred, and the things in that corner are behind
/// you as often as not and cost the same either way.
pub fn cells_round(at: Vec3, reach: f32) -> Vec<(i32, i32)> {
    let (x, z) = cell_of(at);
    let out = (reach / CELL).ceil() as i32;
    let mut cells = Vec::new();

    for down in -out..=out {
        for along in -out..=out {
            let cell = (x + along, z + down);
            let middle = middle_of(cell);

            if (middle.x - at.x).hypot(middle.z - at.z) <= reach + CELL {
                cells.push(cell);
            }
        }
    }

    cells
}

/// The middle of a cell's ground.
pub fn middle_of(cell: (i32, i32)) -> Vec3 {
    vec3(
        (cell.0 as f32 + 0.5) * CELL,
        0.0,
        (cell.1 as f32 + 0.5) * CELL,
    )
}

/// What one thing standing in the field is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum What {
    /// Which of the field's trees it is: see `TREES`.
    Tree(usize),
    /// Which of the field's flowers it is, which is a colour.
    Flower(usize),
    Tuft,
}

/// One thing standing in the field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stood {
    pub at: Vec3,
    /// How far round its own upright it is turned.
    pub turn: f32,
    /// How big it is against the thing it is one of.
    pub size: f32,
    pub what: What,
}

/// How many kinds of tree and of flower the field has.
///
/// **Three trees, stood thousands of times.** `garden::Tree` already says
/// what a tree is in this building, with a trunk mesh built from its own
/// lean, limbs off that line and pads on the limbs. A field cannot have a
/// mesh a tree, so it has three and turns each one as it stands it. Three,
/// because two reads as two and four is a mesh nobody looks at.
pub const TREES: usize = 3;
pub const FLOWERS: usize = 4;

/// How many of each a cell holds.
///
/// **Trees are thin on the ground on purpose.** Two a cell is a tree every
/// five and a half metres, which is a wood: you cannot see out of a wood, and
/// what this room is about is seeing out. Two cells in three have nothing on
/// them at all.
pub const TREES_A_CELL: u32 = 2;
pub const FLOWERS_A_CELL: u32 = 7;
pub const TUFTS_A_CELL: u32 = 30;

/// How big a tree, a flower and a tuft are.
pub const TREE_TALL: f32 = 4.4;
pub const FLOWER_TALL: f32 = 0.36;
pub const FLOWER_WIDE: f32 = 0.22;
pub const TUFT_TALL: f32 = 0.34;
pub const TUFT_WIDE: f32 = 0.3;

/// How many different tufts of grass there are.
pub const TUFTS: u32 = 5;

/// The trees the field stands, each at the origin and leaning the same way,
/// so that standing one is turning it rather than building it again.
///
/// `garden::Tree` says what a tree is in this building and the field does not
/// get a second answer. What it gets is three of them.
pub fn kinds() -> Vec<crate::garden::Tree> {
    [
        (TREE_TALL, 3.7f32, 0x5A17u32),
        (TREE_TALL * 0.78, 2.6, 0x2C9F),
        (TREE_TALL * 1.15, 4.3, 0xB3E4),
    ]
    .iter()
    .map(|(tall, wide, seed)| crate::garden::Tree {
        at: Vec3::ZERO,
        tall: *tall,
        wide: *wide,
        leans: glam::vec2(1.0, 0.0),
        seed: *seed,
    })
    .collect()
}

/// Where a part of a tree standing in the field ends up.
///
/// The parts of `garden::Tree` are worked out about the tree's own foot, so
/// standing one is turning its parts about that foot and scaling them. One
/// place that says so, because the trunk, the limbs and the pads each have to
/// be moved the same way and three accounts of that is three trees coming
/// apart.
pub fn stands(stood: &Stood, part: Vec3) -> Vec3 {
    let turned = glam::Quat::from_rotation_y(stood.turn) * (part * stood.size);

    stood.at + turned
}

/// The trees of a cell.
pub fn trees(reaches: f32, cell: (i32, i32)) -> Vec<Stood> {
    let mut rolls = Rolls::of(cell, 0x7EE5);
    let mut out = Vec::new();

    for _ in 0..TREES_A_CELL {
        // most of them are no tree at all, which is what makes a field with
        // trees in it rather than a wood
        if rolls.next() > 0.38 {
            continue;
        }

        let at = rolls.somewhere(cell);

        if !open(reaches, at) {
            continue;
        }

        out.push(Stood {
            at,
            turn: rolls.next() * std::f32::consts::TAU,
            size: 0.75 + rolls.next() * 0.55,
            what: What::Tree((rolls.next() * TREES as f32) as usize % TREES),
        });
    }

    out
}

/// The flowers and the tufts of grass of a cell, which are the near ground.
pub fn cover(reaches: f32, cell: (i32, i32)) -> Vec<Stood> {
    let mut rolls = Rolls::of(cell, 0xC0F5);
    let mut out = Vec::new();

    for n in 0..(FLOWERS_A_CELL + TUFTS_A_CELL) {
        let at = rolls.somewhere(cell);
        let turn = rolls.next() * std::f32::consts::TAU;
        let size = 0.7 + rolls.next() * 0.7;

        if !open(reaches, at) {
            continue;
        }

        out.push(Stood {
            at,
            turn,
            size,
            what: if n < FLOWERS_A_CELL {
                What::Flower((rolls.next() * FLOWERS as f32) as usize % FLOWERS)
            } else {
                What::Tuft
            },
        });
    }

    out
}

/// The wall the building presents to the field, as the boxes of it.
///
/// **The field is a half plane and this is its edge.** The ground is only
/// under you west of `starts`, so until this went in you could walk out of
/// the gap, turn north, pass the garden's corner in four paces and walk
/// east into no ground at all: Jake did, and came back standing on the
/// hall's ceiling. Calling that unreachable was wrong. It took seconds.
///
/// So the wall runs the whole length of the ground rather than the length
/// of the garden, with one gap in it, and the stretch the garden already
/// walls is left to the garden: two walls in one plane is the fault this
/// building has fought six times.
///
/// It follows you, like the ground, and ends where the ground does.
pub fn wall(reaches: f32, at: Vec3) -> Vec<Aabb> {
    let lies = ground(at);
    let face = starts(reaches);
    let (from, to) = way(reaches);
    let (south_end, north_end) = crate::garden::west_ends(reaches);
    let tall = crate::garden::HIGH;
    // west of the line and not east of it, which is the side the garden's
    // own west wall is on. Built east, it stood inside the garden, and a
    // body already on the garden's floor walked straight through it.
    let piece = |near: f32, far: f32| {
        (far > near + 1e-3).then(|| {
            Aabb::new(
                vec3(face - crate::room::THICK, -DROP, near),
                vec3(face, tall, far),
            )
        })
    };

    // only the stretches past the garden's own ends, which is where the
    // walk off the world was. The garden walls its own span and has the
    // way out cut through it already, and a second wall in that plane is
    // the fault this building has fought six times.
    let _ = (from, to);

    [piece(lies.min.z, south_end), piece(north_end, lies.max.z)]
        .iter()
        .copied()
        .flatten()
        .collect()
}

/// The plaster on the field's side of the wall, as the two runs of it.
///
/// **One run a side, through the garden's own stretch, with the way out
/// the only break.** The garden used to plaster its own outer face from
/// its own list, one picture for the whole face the way its other walls
/// get one each, and `field::wall` tiled a different picture over the rest:
/// two pictures at two phases meeting at the corner, which reads as a
/// seam. It is the same wall, so it is one run.
///
/// **And the run's length is a whole number of tiles.** The quad carries
/// its tile count in its own corners, so a run whose length wanders by a
/// metre is a run whose tiles breathe as you walk. The near end is the way
/// out, which does not move, and the far end is snapped out to the next
/// tile past where the ground stops.
pub fn wall_faces(reaches: f32, at: Vec3) -> Vec<Aabb> {
    let lies = ground(at);
    let face = starts(reaches) - crate::room::THICK;
    let (from, to) = way(reaches);
    let tall = crate::garden::HIGH;
    let run = |near: f32, far: f32| {
        let tiles = ((far - near).abs() / RENDER).ceil().max(1.0) * RENDER;
        let far = if far < near {
            near - tiles
        } else {
            near + tiles
        };

        Aabb::new(
            vec3(face, -DROP, near.min(far)),
            vec3(face, tall, near.max(far)),
        )
    };

    vec![run(from, lies.min.z), run(to, lies.max.z)]
}

/// What you walk into out here, which is the trunks and nothing else.
///
/// A tuft of grass you walk through and a flower you walk over. A field where
/// the grass stops you is a field made of wire.
pub fn solid(reaches: f32, at: Vec3) -> Vec<Aabb> {
    let mut out = vec![ground(at)];

    out.extend(wall(reaches, at));

    for cell in cells_round(at, CELL * 2.0) {
        for stood in trees(reaches, cell) {
            let What::Tree(kind) = stood.what else {
                continue;
            };
            let tall = kinds()[kind].tall * stood.size;
            // only as high as a body, like the garden's own trunks: what a
            // trunk has to do is stop you, and a box the height of the whole
            // tree stops you under a branch you could walk beneath
            let stops = tall.min(1.8);
            let wide = 0.26 * stood.size;

            out.push(Aabb::from_center_size(
                stood.at + Vec3::Y * stops * 0.5,
                vec3(wide, stops, wide),
            ));
        }
    }

    out
}

/// A cell's own run of numbers.
///
/// **Out of the cell's coordinates and nothing else**, so a cell that has
/// been walked away from and come back to is the same cell, with no record
/// kept of it anywhere. That is the whole of what makes a field endless
/// rather than large.
struct Rolls(u64);

impl Rolls {
    fn of(cell: (i32, i32), salt: u64) -> Self {
        let mixed = (cell.0 as i64 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (cell.1 as i64 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ salt.wrapping_mul(0x1656_67B1_9E37_79F9);

        Rolls(Self::stirred(mixed))
    }

    fn stirred(mut n: u64) -> u64 {
        n ^= n >> 30;
        n = n.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        n ^= n >> 27;
        n = n.wrapping_mul(0x94D0_49BB_1331_11EB);
        n ^ (n >> 31)
    }

    /// The next number, nought to one.
    fn next(&mut self) -> f32 {
        self.0 = Self::stirred(self.0.wrapping_add(0x9E37_79B9_7F4A_7C15));

        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Somewhere inside a cell.
    fn somewhere(&mut self, cell: (i32, i32)) -> Vec3 {
        let (along, down) = (self.next(), self.next());

        vec3(
            (cell.0 as f32 + along) * CELL,
            -DROP,
            (cell.1 as f32 + down) * CELL,
        )
    }
}

/// How many texels across a tile of the ground's grass is.
pub const TURF: u32 = 128;

/// The ground, as a picture of grass one tile across.
///
/// **Blades and not noise.** A tile of value noise in two greens is a
/// carpet, which is what the first ground was and what a field is least
/// like. What makes grass read as grass from above is that it lies in
/// directions: short strokes, each one a few texels long, lying every way at
/// once, with the ground showing between them.
///
/// It tiles, so a stroke that runs off one edge comes back on the other.
pub fn turf(seed: u32) -> blitzkit::texture::TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    let roll = |n: u32| (n % 10_000) as f32 / 10_000.0;

    let under = [96u8, 110, 66];
    let mut pixels = Vec::with_capacity((TURF * TURF * 4) as usize);

    for _ in 0..TURF * TURF {
        pixels.extend([under[0], under[1], under[2], 255]);
    }

    // every stroke is laid twice as long as it looks, because a stroke drawn
    // texel by texel along a shallow angle leaves gaps in itself
    for _ in 0..TURF * 7 {
        let from = (roll(next()) * TURF as f32, roll(next()) * TURF as f32);
        let round = roll(next()) * std::f32::consts::TAU;
        let long = 3.0 + roll(next()) * 4.0;
        let lit = 0.55 + roll(next()) * 0.75;

        for step in 0..(long * 2.0) as u32 {
            let on = step as f32 * 0.5;
            let x = (from.0 + round.cos() * on).rem_euclid(TURF as f32) as u32;
            let z = (from.1 + round.sin() * on).rem_euclid(TURF as f32) as u32;
            let at = ((z.min(TURF - 1) * TURF + x.min(TURF - 1)) * 4) as usize;
            // darker at the foot of the stroke and lighter at its tip, which
            // is what a blade leaning away from you does to the light
            let along = 0.6 + 0.4 * (on / long);

            for (n, under) in [112.0f32, 140.0, 74.0].iter().enumerate() {
                pixels[at + n] = (under * lit * along).min(255.0) as u8;
            }
        }
    }

    blitzkit::texture::TextureData::from_pixels(TURF, TURF, pixels)
}

/// How big one tile of the field wall's plaster is.
///
/// The garden gets one picture per wall, which is right for sixteen metres
/// and absurd for three hundred: a tile and a half of this is what the
/// garden's whole west wall would be.
pub const RENDER: f32 = 6.0;

/// How many texels across a tuft's blades are drawn.
pub const BLADES: u32 = 64;

/// A tuft of grass, as a picture of blades with nothing between them.
///
/// **Nothing in this building has been this before.** A tuft was a box, at
/// three proportions: a post, a nail and a cube, and Jake would have said so
/// about all three. Grass is not a solid of any shape. What it is, from
/// standing height, is a silhouette, so it is drawn as one and the engine's
/// own cutout throws the rest away.
pub fn blades(seed: u32) -> blitzkit::texture::TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    let roll = |n: u32| (n % 10_000) as f32 / 10_000.0;

    let mut pixels = vec![0u8; (BLADES * BLADES * 4) as usize];

    for _ in 0..9 {
        let foot = 0.2 + roll(next()) * 0.6;
        let lean = (roll(next()) - 0.5) * 0.55;
        let tall = 0.55 + roll(next()) * 0.45;
        let thick = 0.028 + roll(next()) * 0.022;
        let lit = 0.72 + roll(next()) * 0.5;

        for step in 0..BLADES * 2 {
            let on = step as f32 / (BLADES * 2) as f32;

            if on > tall {
                break;
            }

            // the blade bends away as it rises and narrows to nothing at the
            // tip, which is the whole difference between a blade and a stick
            let along = on / tall;
            let x = foot + lean * along * along;
            let wide = thick * (1.0 - along * 0.85);

            for across in -2i32..=2 {
                let off = across as f32 * 0.5 / BLADES as f32;

                if off.abs() > wide {
                    continue;
                }

                let at_x = ((x + off) * BLADES as f32) as i32;
                let at_y = ((1.0 - on) * BLADES as f32) as i32;

                if at_x < 0 || at_x >= BLADES as i32 || at_y < 0 || at_y >= BLADES as i32 {
                    continue;
                }

                let at = ((at_y as u32 * BLADES + at_x as u32) * 4) as usize;
                // darker at the foot, where a tuft shades itself
                let deep = 0.55 + 0.45 * along;

                for (n, under) in [96.0f32, 148.0, 72.0].iter().enumerate() {
                    pixels[at + n] = (under * lit * deep).min(255.0) as u8;
                }
                pixels[at + 3] = 255;
            }
        }
    }

    blitzkit::texture::TextureData::from_pixels(BLADES, BLADES, pixels)
}

/// How many ways a tuft's blades face.
///
/// Three planes through the upright at sixty degrees to each other, each
/// drawn both ways round, so a tuft looks like a tuft from anywhere and from
/// nowhere looks like a sheet of paper.
pub const FACES: u32 = 3;

/// The mesh a tuft is drawn on: those planes, standing on the ground, one
/// unit each way.
///
/// **Every normal points straight up.** A blade's own normal is sideways,
/// which under a sun overhead is a tuft that goes black while the ground
/// round it is lit. Grass takes its light from the ground it is part of, and
/// this is the usual way of saying so.
pub fn tuft_mesh() -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for face in 0..FACES {
        let round = face as f32 / FACES as f32 * std::f32::consts::PI;
        let (out_x, out_z) = (round.cos() * 0.5, round.sin() * 0.5);
        let at = vertices.len() as u32;

        for (x, y, z, u, v) in [
            (-out_x, 0.0, -out_z, 0.0, 1.0),
            (out_x, 0.0, out_z, 1.0, 1.0),
            (out_x, 1.0, out_z, 1.0, 0.0),
            (-out_x, 1.0, -out_z, 0.0, 0.0),
        ] {
            vertices.push(Vertex::new([x, y, z], [0.0, 1.0, 0.0], [u, v]));
        }

        // and both ways round, because a face wound away from you is drawn
        // nowhere and half a tuft is half the time
        indices.extend([at, at + 1, at + 2, at, at + 2, at + 3]);
        indices.extend([at, at + 2, at + 1, at, at + 3, at + 2]);
    }

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// A flower, as a picture of stalks with a head on each.
///
/// The same silhouette as a tuft and for the same reason: a flower drawn as
/// a solid is a box, and a box the size of a flower is a die lying in the
/// grass. Which is what the first build of these was.
pub fn flower(seed: u32, head: [u8; 3]) -> blitzkit::texture::TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    let roll = |n: u32| (n % 10_000) as f32 / 10_000.0;

    let mut pixels = vec![0u8; (BLADES * BLADES * 4) as usize];
    let mut ink = |x: f32, y: f32, colour: [u8; 3]| {
        let (at_x, at_y) = (
            (x * BLADES as f32) as i32,
            ((1.0 - y) * BLADES as f32) as i32,
        );

        if at_x < 0 || at_x >= BLADES as i32 || at_y < 0 || at_y >= BLADES as i32 {
            return;
        }

        let at = ((at_y as u32 * BLADES + at_x as u32) * 4) as usize;

        pixels[at] = colour[0];
        pixels[at + 1] = colour[1];
        pixels[at + 2] = colour[2];
        pixels[at + 3] = 255;
    };

    for _ in 0..3 {
        let foot = 0.3 + roll(next()) * 0.4;
        let lean = (roll(next()) - 0.5) * 0.4;
        let tall = 0.6 + roll(next()) * 0.35;
        let wide = 0.045 + roll(next()) * 0.03;

        for step in 0..BLADES * 2 {
            let on = step as f32 / (BLADES * 2) as f32;

            if on > tall {
                break;
            }

            let x = foot + lean * (on / tall) * (on / tall);

            for across in -1i32..=1 {
                ink(x + across as f32 / BLADES as f32, on, [58, 96, 48]);
            }
        }

        // and the head on top of it, which is the whole of what makes it a
        // flower rather than a reed
        let top = foot + lean;

        for down in -2i32..=2 {
            for across in -2i32..=2 {
                if (down * down + across * across) as f32 > 5.6 {
                    continue;
                }

                ink(
                    top + across as f32 * wide * 0.5,
                    tall + down as f32 * wide * 0.5,
                    head,
                );
            }
        }
    }

    blitzkit::texture::TextureData::from_pixels(BLADES, BLADES, pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A building the size of the one that ships, for the tests to measure
    /// the field off.
    fn reaches() -> f32 {
        crate::room::Room::of(crate::games()).reaches
    }

    /// Spec 0040: you can walk out of the garden into it.
    ///
    /// Walked rather than measured, because what the garden's west wall has
    /// in it is a hole and a hole is a thing you can be stood beside.
    #[test]
    fn you_can_walk_out_of_the_garden() {
        let reaches = reaches();
        let middle = crate::garden::at(reaches);
        let from = vec3(starts(reaches) + 2.5, 0.0, middle.z);
        let to = vec3(starts(reaches) - 20.0, 0.0, middle.z);
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(14.0 / step) as u32 {
            let solid = standing_on(reaches, at);
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };
            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

            at = next;
            falling = fell;
        }

        assert!(
            open(reaches, at),
            "fourteen seconds at the gap and you are still at x {:.2}, with the field past {:.2}",
            at.x,
            starts(reaches)
        );
        assert!(
            at.y > -0.2,
            "you walked out of the garden and fell to {:.2}",
            at.y
        );
    }

    /// Spec 0040: the ground never gets nearer its own edge.
    ///
    /// **Which is the whole room.** A field you can reach the end of is a
    /// lawn. Walked a kilometre in each of four directions, the edge is the
    /// same distance off as it was from the gate, give or take the tile the
    /// slab is snapped to.
    #[test]
    fn the_ground_never_gets_nearer_its_edge() {
        let reaches = reaches();
        let gate = vec3(starts(reaches) - 1.0, 0.0, crate::garden::at(reaches).z);

        for away in [0.0f32, 40.0, 1_000.0, 8_000.0] {
            for (x, z) in [(-1.0f32, 0.0f32), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)] {
                let at = gate + vec3(x, 0.0, z) * away;
                let lies = ground(at);
                let reach = (lies.max.x - at.x)
                    .min(at.x - lies.min.x)
                    .min(lies.max.z - at.z)
                    .min(at.z - lies.min.z);

                assert!(
                    (reach - GROUND_REACH).abs() <= TILE,
                    "a kilometre out the ground's edge is {:.1} off, against {:.1} at the gate",
                    reach,
                    GROUND_REACH
                );
            }
        }
    }

    /// Spec 0040: the sky is further off than the ground, everywhere.
    ///
    /// **The rim traces a square and so does the slab**, so it is the
    /// corners that have to clear and not the sides: a dome scaled to clear
    /// the ground along the axes is a dome the ground runs out from under at
    /// forty five degrees, and what is past the ground there is the clear
    /// colour, which is black.
    #[test]
    fn the_sky_is_further_off_than_the_ground() {
        for round in 0..64 {
            let round = round as f32 / 64.0 * std::f32::consts::TAU;
            let sky = crate::garden::vault_rim(round) * SKY_BY;
            // how far the square slab reaches on that bearing
            let (c, s) = (round.cos().abs(), round.sin().abs());
            let ground = GROUND_REACH / c.max(s);

            assert!(
                sky > ground,
                "at {:.2} radians the sky stops at {:.1} and the ground runs to {:.1}",
                round,
                sky,
                ground
            );
        }
    }

    /// Spec 0040: you can see as far as the sky.
    ///
    /// The first build ran on the building's hundred metres, which cut the
    /// ground off at a hundred and left black above it.
    #[test]
    fn you_can_see_as_far_as_the_sky() {
        let corner = (0..64)
            .map(|n| crate::garden::vault_rim(n as f32 / 64.0 * std::f32::consts::TAU) * SKY_BY)
            .fold(0.0f32, f32::max);
        let top = crate::garden::DOME_UP * SKY_BY;

        assert!(
            SEES > corner.hypot(top),
            "the sky's furthest corner is {:.0} off and the camera sees {:.0}",
            corner.hypot(top),
            SEES
        );
    }

    /// Spec 0040: a cell holds the same things every time, and no two cells
    /// hold the same things.
    #[test]
    fn a_cell_is_itself_and_nothing_else() {
        let reaches = reaches();
        let away = (-40i32, 17i32);

        assert_eq!(
            trees(reaches, away),
            trees(reaches, away),
            "a cell asked twice gave two answers"
        );
        assert_eq!(cover(reaches, away), cover(reaches, away));

        let mut seen = std::collections::HashSet::new();

        for down in -6..6 {
            for along in -6..6 {
                let cell = (away.0 + along, away.1 + down);
                let how = cover(reaches, cell)
                    .iter()
                    .map(|one| format!("{:?}", one))
                    .collect::<Vec<_>>()
                    .join("|");

                assert!(
                    seen.insert(how),
                    "two cells near {:?} are the same cell",
                    cell
                );
            }
        }
    }

    /// Spec 0040: nothing of the field stands indoors.
    ///
    /// The cells near the gate lap over the garden and the building, and a
    /// tree in the hall is what a rule that does not check looks like.
    #[test]
    fn nothing_of_the_field_is_indoors() {
        let reaches = reaches();
        let gate = vec3(starts(reaches), 0.0, crate::garden::at(reaches).z);

        for cell in cells_round(gate, TREES_REACH) {
            for stood in trees(reaches, cell).into_iter().chain(cover(reaches, cell)) {
                assert!(
                    open(reaches, stood.at),
                    "a {:?} stands at x {:.2}, and the field starts at {:.2}",
                    stood.what,
                    stood.at.x,
                    starts(reaches)
                );
            }
        }
    }

    /// Everything a body can walk into from where it is, which is what
    /// `Arcade::standing_on` hands the walk.
    fn standing_on(reaches: f32, at: Vec3) -> Vec<Aabb> {
        let mut solid = crate::room::Room::of(crate::games()).solid();

        if afoot(&solid, at) {
            solid.extend(self::solid(reaches, at));
        }

        solid
    }

    /// Spec 0040: there is ground under every spot a body could be.
    ///
    /// **Swept rather than walked**, because a walk only finds the holes it
    /// happens into and this room has had three. Over sixty metres each way
    /// round the building, every half metre: either something of the
    /// building is under you or the field is, and there is nowhere that is
    /// neither.
    ///
    /// The strip north of the garden was eleven hundred square metres of
    /// grass with nothing under it, drawn because `garden::under_the_sky` is
    /// a quarter plane and solid because `open` was a half plane, and the
    /// two are not the same shape. Nothing is asked about shapes now: see
    /// [`afoot`].
    #[test]
    fn there_is_ground_under_everywhere() {
        let room = crate::room::Room::of(crate::games());
        let reaches = room.reaches;
        let fixed = room.solid();
        let mut holes = 0;
        let mut first = None;
        let mut z = -60.0f32;

        while z < 60.0 {
            let mut x = -60.0f32;

            while x < 30.0 {
                let at = vec3(x, 0.5, z);
                // not inside a wall, where nobody can be and nothing has
                // to hold anybody up
                if fixed.iter().any(|box_| {
                    box_.min.x <= x
                        && box_.max.x >= x
                        && box_.min.z <= z
                        && box_.max.z >= z
                        && box_.min.y <= 0.5
                        && box_.max.y >= 0.5
                }) {
                    x += 0.5;
                    continue;
                }

                let mut solid = fixed.clone();

                if afoot(&solid, at) {
                    solid.extend(self::solid(reaches, at));
                }

                let under = solid.iter().any(|box_| {
                    box_.min.x <= x
                        && box_.max.x >= x
                        && box_.min.z <= z
                        && box_.max.z >= z
                        && box_.max.y <= 0.7
                        && box_.max.y > -3.0
                });

                if !under {
                    holes += 1;
                    first.get_or_insert((x, z));
                }

                // and wherever the field is what holds you up, the field
                // is what you see. The two were different shapes twice:
                // solid as a half plane and drawn as a quarter one, which
                // is grass nobody painted south of the garden and grass
                // painted over nothing north of it. They are one question
                // now and this is here so they stay one.
                assert!(
                    !afoot(&fixed, at) || in_view(reaches, &fixed, at),
                    "the field holds you up at {:.1}, {:.1} and is not drawn there",
                    x,
                    z
                );

                x += 0.5;
            }

            z += 0.5;
        }

        assert_eq!(
            holes, 0,
            "{} spots with nothing under them, the first at {:?}",
            holes, first
        );
    }

    /// Spec 0040: a body let loose outdoors stays in the world.
    ///
    /// **Jake's, after finding the third hole in it by walking.** Three
    /// times now the field has had an edge somewhere nobody thought to
    /// look, and three times what found it was a person wandering rather
    /// than a test measuring the place the fault was last time. So this
    /// wanders: a seeded heading that changes every so often, from eight
    /// starts, for half a minute of walking each. Half a minute, because
    /// the gate runs it: a minute each found nothing a half minute did
    /// not and doubled what this file costs to check.
    ///
    /// What it holds is the one thing that must never happen, which is
    /// being somewhere with no ground under you. Everything else out here
    /// is a matter of taste.
    #[test]
    fn a_body_let_loose_outdoors_stays_in_the_world() {
        /// How far under the lowest floor anybody can get before they are
        /// falling rather than walking. The baths are the deepest room and
        /// their floor is at -3.
        const UNDER: f32 = -4.0;

        let reaches = reaches();
        let face = starts(reaches);
        let middle = crate::garden::at(reaches);
        let half = crate::garden::half();
        let step = 1.0 / 60.0;
        let mut rolls = Rolls::of((0x5EED, 0x11FE), 0xA11);

        for (what, from) in [
            ("the way out", vec3(face - 1.0, 0.0, middle.z)),
            ("the garden", vec3(middle.x, 0.0, middle.z)),
            (
                "the garden's north end",
                vec3(middle.x, 0.0, middle.z + half.y - 1.0),
            ),
            (
                "the garden's south end",
                vec3(middle.x, 0.0, middle.z - half.y + 1.0),
            ),
            ("just outside", vec3(face - 2.0, 0.0, middle.z + half.y)),
            ("the field", vec3(face - 25.0, 0.0, middle.z + 30.0)),
            ("the field, south", vec3(face - 25.0, 0.0, middle.z - 30.0)),
            ("far out", vec3(face - 90.0, 0.0, middle.z)),
        ] {
            let mut at = from;
            let mut falling = 0.0;
            let mut round = rolls.next() * std::f32::consts::TAU;

            for frame in 0..(30.0 / step) as u32 {
                // a new heading every half second or so, which is a body
                // wandering rather than a body walking a line
                if frame % 30 == 0 {
                    round += (rolls.next() - 0.5) * 2.4;
                }

                let wish = vec3(round.cos(), 0.0, round.sin()) * crate::SPEED;
                let solid = standing_on(reaches, at);
                let (next, fell) =
                    crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

                at = next;
                falling = fell;

                assert!(
                    at.y > UNDER,
                    "let loose at {} and {:.1} seconds later the body is at {:?}, \
                     which is under the building",
                    what,
                    frame as f32 * step,
                    at
                );
            }
        }
    }

    /// Spec 0040: you cannot walk out of the field.
    ///
    /// **Walked, because the first version of this was measured and the
    /// measurement was of the wrong thing.** It checked that the garden's
    /// west face is solid for the whole of the garden's own span, which it
    /// is, and called the rest unreachable because the wall runs away north
    /// and south. The garden is sixteen metres long. Jake walked out of the
    /// gap, round its corner and east into no ground at all inside a few
    /// seconds, and came back standing on the hall's ceiling.
    ///
    /// So: a body your size, from four places in the field, walked hard at
    /// the building for twenty seconds, ends west of the wall and on the
    /// ground.
    #[test]
    fn you_cannot_walk_out_of_the_field() {
        let reaches = reaches();
        let face = starts(reaches);
        let middle = crate::garden::at(reaches).z;
        let half = crate::garden::half().y;
        let step = 1.0 / 60.0;

        for from in [
            vec3(face - 6.0, 0.0, middle + half + 6.0),
            vec3(face - 6.0, 0.0, middle - half - 6.0),
            vec3(face - 30.0, 0.0, middle + half + 40.0),
            vec3(face - 2.0, 0.0, middle + half + 1.0),
        ] {
            let mut at = from;
            let mut falling = 0.0;

            for _ in 0..(20.0 / step) as u32 {
                let solid = standing_on(reaches, at);
                let wish = vec3(1.0, 0.0, 0.0) * crate::SPEED;
                let (next, fell) =
                    crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);

                at = next;
                falling = fell;
            }

            assert!(
                at.x < face,
                "walked east from {:?} and ended at x {:.2}, past the wall at {:.2}",
                from,
                at.x,
                face
            );
            assert!(
                at.y > -0.5,
                "walked east from {:?} and fell to {:.2}",
                from,
                at.y
            );
        }
    }

    /// Spec 0040: the wall along that edge has one gap in it and no others.
    ///
    /// **Drawn and not solid, which is the half that was missing.** The
    /// garden's outer plaster ran to the garden's own edge and
    /// `field::wall` began a wall's thickness further out, so there was a
    /// slot one wall thick at each end of the garden with nothing drawn in
    /// it: from the field you looked through the corner at the grass
    /// beyond, and nothing stopped you because the colliders had no such
    /// gap.
    ///
    /// It is one run a side now, through the garden's own stretch, because
    /// it is one wall. This walks the line every ten centimetres to say
    /// the only hole in it is the way out.
    #[test]
    fn the_wall_has_one_gap_in_it() {
        let reaches = reaches();
        let face = starts(reaches);
        let (from, to) = way(reaches);
        let at = vec3(face - 2.0, 0.0, crate::garden::at(reaches).z);
        // the drawn account and not the solid one. The solid wall leaves
        // the garden's own stretch to the garden, which walls it already;
        // what has to have no holes in it is what you see.
        let pieces = wall_faces(reaches, at);
        let mut z = -60.0f32;

        while z < 60.0 {
            if z > from + 0.05 && z < to - 0.05 {
                z += 0.1;
                continue;
            }

            let walled = pieces.iter().any(|box_| box_.min.z <= z && box_.max.z >= z)
                || crate::garden::facing(reaches).iter().any(|face| {
                    // `plane` gives the run across the face and not the
                    // box's z, which for a wall looking west is its z
                    let (surface, across, looks) = face.plane();

                    looks.x < -0.5
                        && surface.z - across.x * 0.5 <= z
                        && surface.z + across.x * 0.5 >= z
                });

            assert!(
                walled,
                "nothing is drawn along the field's edge at z {:.2}, and the way out is \
                 {:.2} to {:.2}",
                z, from, to
            );

            z += 0.1;
        }
    }

    /// Spec 0040: the field has one edge, and you cannot walk to it.
    ///
    /// **It is a half plane and not a plane.** The ground is only under you
    /// west of the garden's west face, so a body that got round the garden's
    /// corner and walked back east would run out of ground and fall. What
    /// stops it is the garden's own walls: the gap is the only way through,
    /// and from the gap the wall runs away north and south further than you
    /// can get round before the ground ends.
    ///
    /// Held here because it is the one thing about this room that is true by
    /// accident rather than by construction, and the next person to widen
    /// the garden or move its gap should be told so by a test rather than by
    /// walking off the world. What it is waiting on is in spec 0040: the
    /// building has three rooms that run a hundred metres or more, and a
    /// field that went round all of them would start past the horizon.
    #[test]
    fn the_field_has_one_edge_and_you_cannot_reach_it() {
        let reaches = reaches();
        let middle = crate::garden::at(reaches);
        let half = crate::garden::half();
        let (from, to) = way(reaches);

        // the edge runs along the garden's west face
        assert!(
            !open(reaches, vec3(starts(reaches) + 0.01, 0.5, middle.z)),
            "the field reaches east of the garden's west face"
        );

        // and the only hole in the wall along it is the way out
        for step in 0..=200 {
            let z = middle.z - half.y + (step as f32 / 200.0) * half.y * 2.0;

            if z > from - 0.1 && z < to + 0.1 {
                continue;
            }

            let wall = crate::garden::solid(reaches).into_iter().any(|box_| {
                box_.min.x < starts(reaches) + 0.05
                    && box_.max.x > starts(reaches) - 0.05
                    && box_.min.z <= z
                    && box_.max.z >= z
                    && box_.max.y > 1.8
            });

            assert!(
                wall,
                "the garden's west face is open at z {:.2}, and the way out is {:.2} to {:.2}",
                z, from, to
            );
        }
    }

    /// Spec 0040: an hour of walking holds no more than a minute of it.
    ///
    /// **A chunked field that does not drop what it has built is the way
    /// this machine has gone down before.** Nothing here is kept at all, so
    /// what this holds is the other half of that: how much is asked for at
    /// once does not grow with how far you have walked.
    #[test]
    fn walking_does_not_fill_the_machine() {
        let reaches = reaches();
        let gate = vec3(starts(reaches) - 1.0, 0.0, crate::garden::at(reaches).z);
        let at_the_gate = cells_round(gate, TREES_REACH).len();

        for away in [100.0f32, 5_000.0, 40_000.0] {
            let at = gate - Vec3::X * away;
            let cells = cells_round(at, TREES_REACH).len();
            let things: usize = cells_round(at, COVER_REACH)
                .into_iter()
                .map(|cell| cover(reaches, cell).len())
                .sum();

            // within a cell's worth either way: `cells_round` measures from
            // cell middles against a circle, so standing a third of a cell
            // further along takes one in and leaves one out
            assert!(
                cells.abs_diff(at_the_gate) * 50 < at_the_gate,
                "{} metres out the field asks for {} cells against {} at the gate",
                away,
                cells,
                at_the_gate
            );
            assert!(
                things < 20_000,
                "{} metres out the near ground is {} things",
                away,
                things
            );
        }
    }

    /// Spec 0040: the grass is mostly not there.
    ///
    /// Blades are drawn as a silhouette on crossed planes and the engine
    /// throws the rest away by its alpha, which it only does for a texture
    /// that says it has holes in it. A blade picture with no holes is three
    /// opaque rectangles standing in the grass, which is what a first build
    /// of this would have been and what nothing but this would have caught.
    #[test]
    fn the_grass_is_mostly_not_there() {
        for (what, picture) in [
            ("a tuft", blades(0x2B19)),
            ("a flower", flower(0x91C7, [228, 206, 92])),
        ] {
            assert!(picture.has_holes(), "{} is drawn solid", what);

            let lit = picture.levels[0]
                .pixels
                .chunks(4)
                .filter(|texel| texel[3] > 128)
                .count();

            assert!(
                lit * 4 < (BLADES * BLADES) as usize,
                "{} covers {} of {} texels, which is a leaf and not blades",
                what,
                lit,
                BLADES * BLADES
            );
        }
    }
}
