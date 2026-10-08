//! The garden off the entrance: gravel, trees, stone lanterns and a koi pond.
//! See `specs/0010-a-garden-off-the-entrance.md`.
//!
//! Everything here is laid out from the hall's own numbers rather than from
//! constants of its own, because the one thing that has gone wrong in every
//! room of this building is two pieces of code saying where something is.

use blitzkit::collision::Aabb;
use glam::{vec2, vec3, Vec2, Vec3};

use crate::room::{CABINET, NOOK_DEEP, NOOK_SPAN, THICK, WALL};

/// How far the garden cuts back behind the left wall, and how far it runs.
///
/// Sixteen by sixteen, against the nook's six by fourteen. Every room in this
/// building has been too small on its first build and has been sent back for
/// it: the nook twice, the baths three times. This one was eleven by eleven
/// and Jake made it bigger before anything was built in it, which costs
/// nothing now and a rebuild later.
///
/// It has to hold a pond with walking room on four sides, trees with room to
/// stand back from, and a sky worth looking up at.
pub const DEEP: f32 = 16.0;
pub const SPAN: f32 = 16.0;

/// How high its walls stand. Taller than the hall's, because a garden under a
/// low lid is a cupboard with a tree in it.
pub const HIGH: f32 = 4.2;

/// The way in: how wide the opening is, and how far short of the near wall it
/// stops.
///
/// Named here and nowhere else. The hall's wall is cut for it and the garden
/// lines it, and those are the two places that would otherwise drift.
///
/// Measured back from the near end rather than written down as a place. It was
/// a place, `z` 8.35, which is right for the fourteen cabinets this project has
/// and wrong for any other number: the tests build a room of twelve, where the
/// hall is a cabinet and a half shorter, and the opening fell outside the
/// building. One of the two wall pieces either side of it came out with a
/// negative length.
///
/// Measured back, it lands in the bare stretch past the last cabinet whatever
/// the count, which is where this spec always said the garden opens.
///
/// The ground inside is level with the hall's floor and not a step below it.
/// A step down is what ground wants and three things said no: the hall's floor
/// slab, the nook's end wall, which stands on the nook's floor and left a slot
/// under itself, and the doorway, which had no ground under its threshold at
/// all. A sunk garden is a spec about where the building's floors stop, and
/// this one is about a garden.
/// Open to the top, with no beam over it. A garden you step into under a
/// lintel is a garden through a door, and a doorway is a thing a building
/// does: what this wants is the wall simply stopping and the sky carrying on
/// over the gap.
pub const WAY_WIDE: f32 = 1.7;
pub const WAY_SHORT: f32 = 0.25;

/// Which side of the hall the garden is on, as a sign on x.
pub const SIDE: f32 = -1.0;

/// The hall's left wall, as the garden measures from it.
pub fn wall_at() -> f32 {
    SIDE * (WALL + CABINET.x)
}

/// Where the garden's near edge is, which is the nook's end wall.
pub fn near(reaches: f32) -> f32 {
    -reaches + NOOK_SPAN + THICK * 0.5
}

/// The middle of the garden's ground.
pub fn at(reaches: f32) -> Vec3 {
    vec3(
        wall_at() + SIDE * (THICK * 0.5 + DEEP * 0.5),
        0.0,
        near(reaches) + SPAN * 0.5,
    )
}

/// How far the ground reaches from that middle, along x and z.
pub fn half() -> Vec2 {
    vec2(DEEP * 0.5, SPAN * 0.5)
}

/// The opening in the hall's left wall, as the two pieces of wall left either
/// side of it and the lintel over it.
///
/// The hall's wall used to be one box from the nook's door to the near end.
/// It is three now, and this is the only place that says where the gap is.
pub fn way(reaches: f32) -> (f32, f32) {
    let to = reaches - WAY_SHORT;

    (to - WAY_WIDE, to)
}

/// Everything solid in the garden: its three new walls, and the ground.
///
/// Three and not four. The hall's left wall is the garden's east side and the
/// nook's end wall is most of its south, and both already exist: a second box
/// in the same place is two surfaces in one plane, which this building has
/// fought four times.
pub fn solid(reaches: f32) -> Vec<Aabb> {
    let middle = at(reaches);
    let half = half();
    let (west, north) = (middle.x - half.x, middle.z + half.y);
    let south = middle.z - half.y;

    let mut out = vec![
        // the west wall, the whole span
        Aabb::from_center_size(
            vec3(west - THICK * 0.5, HIGH * 0.5, middle.z),
            vec3(THICK, HIGH, SPAN + THICK * 2.0),
        ),
        // the north wall, across the top
        Aabb::from_center_size(
            vec3(middle.x, HIGH * 0.5, north + THICK * 0.5),
            vec3(DEEP, HIGH, THICK),
        ),
        // the south wall, only the part the nook's end wall does not already
        // close: the nook is 6.4 deep and the garden is 11
        Aabb::from_center_size(
            vec3(west + (DEEP - NOOK_DEEP) * 0.5, HIGH * 0.5, south),
            vec3(DEEP - NOOK_DEEP, HIGH, THICK),
        ),
        // a parapet over the hall's wall, which is shorter than the garden's,
        // in two pieces with the way in between them.
        //
        // The hall is 3.2 to the garden's 4.2, so along the stretch they share
        // there was a metre of nothing above the wall, and over it you were
        // looking at the roof of the hall from outside: a dark slab hanging in
        // the garden's sky with the lanterns catching its underside.
        //
        // In one piece it crossed the opening, which put a beam back over the
        // way in the moment the lintel came out of it. From the garden that
        // beam's underside was a ledge hanging over the entrance, which is the
        // same dark slab wearing a different hat.
        Aabb::from_center_size(
            vec3(
                wall_at(),
                (crate::room::TALL + HIGH) * 0.5,
                (near(reaches) + way(reaches).0) * 0.5,
            ),
            vec3(
                THICK,
                HIGH - crate::room::TALL,
                way(reaches).0 - near(reaches),
            ),
        ),
        Aabb::from_center_size(
            vec3(
                wall_at(),
                (crate::room::TALL + HIGH) * 0.5,
                (way(reaches).1 + reaches) * 0.5,
            ),
            vec3(THICK, HIGH - crate::room::TALL, reaches - way(reaches).1),
        ),
        // the east wall past the hall, where the hall's own wall has run out
        Aabb::from_center_size(
            vec3(wall_at(), HIGH * 0.5, (reaches + north + THICK) * 0.5),
            vec3(THICK, HIGH, north + THICK - reaches),
        ),
        // and the ground, which is a step below the hall's floor.
        //
        // It runs under the wall to meet that floor's edge rather than stopping
        // at the wall's outer face. Stopped there it left the width of the wall
        // with no ground under it at all, which is a hole in the doorway: you
        // could walk up to the opening from either side and not through it.
        Aabb::from_center_size(
            vec3((west + wall_at()) * 0.5, -WALL * 0.5, middle.z),
            vec3(wall_at() - west, WALL, SPAN),
        ),
    ];

    // the pond's stone, which you stand on rather than in
    out.extend(basin(reaches));
    // and the trees and lanterns, which you walk round
    out.extend(trees(reaches).into_iter().map(|(where_, tall, _)| {
        Aabb::from_center_size(where_ + Vec3::Y * tall * 0.5, vec3(0.34, tall, 0.34))
    }));
    out.extend(lanterns(reaches).into_iter().map(|(where_, tall)| {
        Aabb::from_center_size(where_ + Vec3::Y * tall * 0.5, vec3(0.5, tall, 0.5))
    }));
    // and the set stones, one box each: a group is three things to walk round
    // and not one, and the gaps between them are where your foot goes
    out.extend(rocks(reaches).into_iter().map(|stone| {
        let up = stands(stone.size);

        Aabb::from_center_size(
            stone.at + Vec3::Y * up * 0.5,
            vec3(stone.size.x, up, stone.size.z),
        )
    }));

    out
}

/// How big the sky's picture is, and how high the dome stands over the garden.
///
/// The dome springs from below the top of the walls and is half as wide again
/// as the garden, so the walls hide its rim and every way you can look up from
/// inside lands on cloud. Sprung level with the wall tops and only as wide as
/// the room, there was a wedge of black over one corner where the sky ran out
/// and the roof of the hall showed behind it.
pub const SKY: u32 = 320;
pub const DOME: f32 = 19.0;
pub const DOME_UP: f32 = 12.0;

/// How fast the weather goes over, in radians a second.
///
/// The dome turns rather than the clouds moving across it. A texture that
/// scrolls wants its coordinates changed every frame and this wants one
/// rotation, which the engine already does to everything else.
///
/// Slow enough that nothing is seen to move while you watch it and the sky is
/// different when you look up again.
pub const WEATHER: f32 = 0.013;

/// A storm, as a picture to wrap round the inside of the dome.
///
/// Value noise over a few octaves, wrapped the way round the dome so the seam
/// does not show, and taken through a hard curve: a soft gradient of grey is
/// haze, and what this wants is cloud with edges and breaks between.
pub fn storm(seed: u32) -> blitzkit::texture::TextureData {
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };

    // the grid each octave is smoothed from, wrapping in u so the two edges of
    // the picture meet round the back of the dome
    let octaves: Vec<(usize, Vec<f32>)> = [6usize, 12, 24, 48]
        .iter()
        .map(|side| {
            let side = *side;
            let cells = (0..side * side)
                .map(|_| (next() % 1024) as f32 / 1024.0)
                .collect::<Vec<_>>();

            (side, cells)
        })
        .collect();

    let at = |cells: &[f32], side: usize, u: f32, v: f32| {
        let (x, y) = (u * side as f32, v * side as f32);
        let (ix, iy) = (x.floor() as usize, y.floor() as usize);
        let (fx, fy) = (x - x.floor(), y - y.floor());
        // smoothstep, so the grid does not show as diamonds
        let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
        let get = |gx: usize, gy: usize| cells[(gy % side) * side + (gx % side)];
        let (a, b) = (get(ix, iy), get(ix + 1, iy));
        let (c, d) = (get(ix, iy + 1), get(ix + 1, iy + 1));

        (a + (b - a) * sx) * (1.0 - sy) + (c + (d - c) * sx) * sy
    };

    let mut pixels = Vec::with_capacity((SKY * SKY * 4) as usize);
    for y in 0..SKY {
        for x in 0..SKY {
            let (u, v) = (x as f32 / SKY as f32, y as f32 / SKY as f32);
            let mut cloud = 0.0;
            let mut weight = 0.0;
            for (n, (side, cells)) in octaves.iter().enumerate() {
                let share = 1.0 / (1 << n) as f32;
                cloud += at(cells, *side, u, v) * share;
                weight += share;
            }
            cloud /= weight;

            // v nought is the dome's rim and v one is its apex, so this is the
            // way up it reads on the mesh and not the way up the picture
            // looks. Taken the other way the heaviest cloud was round the
            // horizon and the darkest slate was directly overhead, which is a
            // storm seen from above.
            let overhead = v;
            let heavy = (cloud * 1.6 - 0.25 + overhead * 0.18).clamp(0.0, 1.0);
            // hard, so there are cloud edges rather than a wash
            let shaped = heavy * heavy * (3.0 - 2.0 * heavy);

            // slate behind, and the breaks between the cloud are the brightest
            // thing in the room
            // A storm is dark for a sky and bright for a room. These read as
            // nearly black twice over, because a slate chosen against daylight
            // is the only light this garden has. What it has to be is dark for
            // a sky and bright for a room, and those pull opposite ways.
            let dark = [96.0, 104.0, 128.0];
            let pale = [228.0, 232.0, 240.0];
            let paint = |n: usize| (dark[n] + (pale[n] - dark[n]) * shaped) as u8;

            pixels.extend_from_slice(&[paint(0), paint(1), paint(2), 255]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(SKY, SKY, pixels)
}

/// The dome the sky is painted on: a half sphere seen from inside.
///
/// Wound the opposite way round to everything else this building turns, which
/// is what puts its faces towards the middle. `turned` winds for a barrel,
/// seen from outside, and a sky is the one surface here that is only ever seen
/// from within.
///
/// Not two sided. That was the first answer and it is the fault this building
/// has fought four times: two surfaces in one plane, both on show. Every
/// triangle had a twin a hair away facing the other way, the depth test picked
/// between them by rounding, and the sky came out in patches of cloud and
/// patches of unlit dark.
pub fn dome_mesh() -> blitzkit::mesh::MeshData {
    let mut mesh = blitzkit::mesh::MeshData::surface(48, 24, |u, v| {
        let round = u * std::f32::consts::TAU;
        // v nought is the rim and v one is the top. Taken the other way round
        // the apex came out at the bottom and the whole sky was a bowl hanging
        // over the garden with black between it and the walls.
        let up = (v * std::f32::consts::FRAC_PI_2).sin();
        let out = (v * std::f32::consts::FRAC_PI_2).cos();

        vec3(
            round.cos() * DOME * out,
            DOME_UP * up,
            round.sin() * DOME * out,
        )
    });
    mesh.compute_normals();

    mesh
}

/// Where the dome stands: over the middle of the garden, springing from the
/// top of its walls.
pub fn dome_at(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x, HIGH * 0.55, middle.z)
}

/// The gravel, as the four strips of it round the pond.
///
/// Four and not one. One quad over the whole garden is laid over the water as
/// well, and the pond came out as an empty stone frame with gravel in the
/// bottom of it: the water is a hair below the ground and the ground was on
/// top of it.
pub fn beds(reaches: f32) -> Vec<(Vec3, Vec2)> {
    let middle = at(reaches);
    let half = half();
    let (water, size, _) = pond(reaches);
    let stone = size * 0.5 + Vec2::splat(KERB);
    let (west, east) = (middle.x - half.x, middle.x + half.x);
    let (south, north) = (middle.z - half.y, middle.z + half.y);
    let (wet_w, wet_e) = (water.x - stone.x, water.x + stone.x);
    let (wet_s, wet_n) = (water.z - stone.y, water.z + stone.y);

    vec![
        // west of the water, the full depth of the garden
        (
            vec3((west + wet_w) * 0.5, 0.0, middle.z),
            vec2(wet_w - west, half.y * 2.0),
        ),
        // east of it, likewise
        (
            vec3((east + wet_e) * 0.5, 0.0, middle.z),
            vec2(east - wet_e, half.y * 2.0),
        ),
        // and the two ends, which only run between those
        (
            vec3((wet_w + wet_e) * 0.5, 0.0, (south + wet_s) * 0.5),
            vec2(wet_e - wet_w, wet_s - south),
        ),
        (
            vec3((wet_w + wet_e) * 0.5, 0.0, (north + wet_n) * 0.5),
            vec2(wet_e - wet_w, north - wet_n),
        ),
    ]
}

/// Flat shading, by giving every triangle its own three corners.
///
/// `compute_normals` averages a face's normal into the corners it shares, which
/// is right for a barrel and wrong for a stone: a boulder of twelve facets
/// shaded smooth is a potato. A rock is the one thing in this garden whose
/// faces you are meant to see.
pub fn faceted(mesh: blitzkit::mesh::MeshData) -> blitzkit::mesh::MeshData {
    use blitzkit::mesh::Vertex;

    let mut vertices = Vec::with_capacity(mesh.indices.len());

    for triangle in mesh.indices.chunks_exact(3) {
        let corners = [
            mesh.vertices[triangle[0] as usize],
            mesh.vertices[triangle[1] as usize],
            mesh.vertices[triangle[2] as usize],
        ];
        let (a, b, c) = (
            Vec3::from(corners[0].position),
            Vec3::from(corners[1].position),
            Vec3::from(corners[2].position),
        );
        let normal = (b - a).cross(c - a).normalize_or_zero().to_array();

        for corner in corners.iter() {
            vertices.push(Vertex::new(corner.position, normal, corner.uv));
        }
    }

    let indices = (0..vertices.len() as u32).collect();

    blitzkit::mesh::MeshData::new(vertices, indices)
}

/// How many facets a stone is cut into, round and from pole to pole.
///
/// Few. A stone of a hundred faces is a ball with a texture problem; what makes
/// it read as rock is being able to count the planes.
pub const CUT_ROUND: u32 = 11;
pub const CUT_STEPS: u32 = 7;

/// How far a stone wanders off round, as a share of its radius.
pub const WANDER: f32 = 0.30;

/// How far a stone is out of round at a point on it.
///
/// Three waves rather than noise, because a stone wants a shape and not a
/// texture: a boulder is a few big planes meeting at edges, and a sum of
/// octaves gives it a rind of warts instead.
///
/// Every wave is a whole number of turns round and a whole number of halves
/// from pole to pole, so the two seams close: a wander that does not come back
/// to itself leaves a crack up the side of the stone and a tear at the top.
pub fn wander(seed: u32, u: f32, v: f32) -> f32 {
    use std::f32::consts::{PI, TAU};

    let mut state = seed | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };

    let mut out = 0.0;
    let mut weight = 0.0;

    for n in 0..3 {
        let round = 1.0 + (next() % 3) as f32;
        let up = 1.0 + (next() % 2) as f32;
        let phase = (next() % 360) as f32 / 360.0 * TAU;
        let share = 1.0 / (1.0 + n as f32);

        out += (u * TAU * round + phase).sin() * (v * PI * up).sin() * share;
        weight += share;
    }

    out / weight * WANDER
}

/// How far a stone leans off its own axis, as a share of its height.
pub const LEANS: f32 = 0.24;

/// Which way a stone leans a given way up itself, and how far.
///
/// A function of the way up alone, which is the only kind of wandering that can
/// move a pole. Roundness that varies with u has to come back to nothing at the
/// top and the bottom or the stone is torn open there, so on its own it leaves
/// a lump that is still a ball in outline: pinched at two points, the same
/// width every way across. What makes the outline lopsided is leaning the whole
/// of it over, and a lean takes the poles with it.
pub fn tip(seed: u32, v: f32) -> Vec2 {
    use std::f32::consts::{PI, TAU};

    let mut state = seed.wrapping_mul(2_654_435_761) | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        state
    };

    let angle = (next() % 360) as f32 / 360.0 * TAU;
    let bend = 0.4 + (next() % 100) as f32 / 125.0;
    let way = vec2(angle.cos(), angle.sin());

    // a lean, and a belly on one side of it
    way * ((v - 0.5) * LEANS + (v * PI).sin() * LEANS * bend * 0.5)
}

/// One stone, carved: a lump the size of a unit, a third of it meant to be
/// under the gravel.
pub fn rock_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::{PI, TAU};

    faceted(blitzkit::mesh::MeshData::surface(
        CUT_ROUND,
        CUT_STEPS,
        |u, v| {
            // wound the way `turned` winds, which is for looking at from
            // outside
            let round = -u * TAU;
            let up = v * PI;
            let out = (0.5 + wander(seed, u, v)) * up.sin();
            let lean = tip(seed, v);

            vec3(
                round.cos() * out + lean.x,
                -up.cos() * 0.5,
                round.sin() * out + lean.y,
            )
        },
    ))
}

/// How much of a stone is under the gravel.
///
/// A third. Set on the surface they are pebbles on a tray, and the one thing
/// every account of this says is that a stone has to look like it came up out
/// of the ground rather than having been put down on it.
pub const BURIED: f32 = 0.34;

/// How many cuts there are, and so how many stones are the same stone.
pub const CUTS: [u32; 3] = [0x4E21, 0x91C7, 0x2BD5];

/// A group of stones: where its middle is from the middle of the garden, which
/// way round it is set, how big it is and how many stones it has.
#[derive(Clone, Copy)]
pub struct Group {
    pub about: Vec2,
    pub turn: f32,
    pub size: f32,
    pub stones: usize,
}

/// Where the groups are.
///
/// In the open gravel, which in this garden is the south strip and the east
/// one: the pond takes the middle and the walk round it takes a body's width
/// outside that, and what is left either side of the water is a corridor and
/// not a place to put anything.
///
/// Three groups of three, three and two. Odd numbers, and never the same
/// number twice, which is the one rule every account of these gardens agrees
/// on.
pub const GROUPS: [Group; 3] = [
    Group {
        about: vec2(2.2, -5.4),
        turn: 0.4,
        size: 1.0,
        stones: 3,
    },
    Group {
        about: vec2(-4.2, -5.6),
        turn: 2.3,
        size: 0.82,
        stones: 3,
    },
    Group {
        about: vec2(5.8, -0.5),
        turn: -1.1,
        size: 0.7,
        stones: 2,
    },
];

/// The stones of a group: where each sits from its middle, how big it is and
/// which cut it is.
///
/// One tall, one low and broad, one small, and the small one nearer the tall
/// one than the broad one is. Three of a size evenly spaced is a bus queue, and
/// what this is meant to read as is two things, one of which is two things.
const SET: [(Vec2, Vec3, usize); 3] = [
    (vec2(0.0, 0.0), vec3(0.60, 1.18, 0.52), 0),
    (vec2(0.84, -0.40), vec3(0.98, 0.44, 0.82), 1),
    (vec2(-0.30, 0.46), vec3(0.42, 0.62, 0.38), 2),
];

/// One stone as it stands.
pub struct Stone {
    /// Where it meets the gravel.
    pub at: Vec3,
    /// How wide, how tall and how deep, before any of it is buried.
    pub size: Vec3,
    pub turn: f32,
    /// Which of the cuts it is.
    pub cut: usize,
}

/// Every stone in the garden.
pub fn rocks(reaches: f32) -> Vec<Stone> {
    let middle = at(reaches);

    GROUPS
        .iter()
        .flat_map(|group| {
            let (sin, cos) = group.turn.sin_cos();

            SET.iter()
                .take(group.stones)
                .enumerate()
                .map(move |(n, (off, size, cut))| {
                    // turned about the group's middle, so no two groups are
                    // the same group seen from the same side
                    let off =
                        vec2(off.x * cos - off.y * sin, off.x * sin + off.y * cos) * group.size;

                    Stone {
                        at: vec3(
                            middle.x + group.about.x + off.x,
                            0.0,
                            middle.z + group.about.y + off.y,
                        ),
                        size: *size * group.size,
                        // and each stone turned again, so the same cut used
                        // twice is not the same stone twice
                        turn: group.turn + n as f32 * 1.9,
                        cut: *cut,
                    }
                })
        })
        .collect()
}

/// How far a stone stands out of the ground.
pub fn stands(size: Vec3) -> f32 {
    size.y * (1.0 - BURIED)
}

/// How far across a group is: the furthest any of its stones reaches from its
/// middle, doubled.
///
/// Groups clear the walls and the walk as groups and not as stones. The stones
/// of one are meant to be within touching distance of each other, which is the
/// whole of what makes them a group rather than three stones; a rule that
/// keeps every stone a body's width from every other stone forbids a group.
pub fn group_wide(group: &Group) -> f32 {
    SET.iter()
        .take(group.stones)
        .map(|(off, size, _)| (off.length() + size.x.max(size.z) * 0.5) * group.size)
        .fold(0.0f32, f32::max)
        * 2.0
}

/// Every group: where its middle stands and how far across it is.
pub fn group_at(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);

    GROUPS
        .iter()
        .map(|group| {
            (
                vec3(middle.x + group.about.x, 0.0, middle.z + group.about.y),
                group_wide(group),
            )
        })
        .collect()
}

/// The middle of each group flat, which is what the raking runs round.
pub fn group_middles(reaches: f32) -> Vec<Vec2> {
    group_at(reaches)
        .into_iter()
        .map(|(where_, _)| vec2(where_.x, where_.z))
        .collect()
}

/// How far apart the rake's lines are.
///
/// A hand's width. It was a metre and a half, which is the width of a furrow a
/// tractor leaves: standing next to it you were inside one line, so the gravel
/// read as a wash with a gradient across it and the raking was not visible at
/// any distance at all. What a rake leaves is a corrugation you can see the
/// whole of from where you stand.
pub const RAKE: f32 = 0.22;

/// How many pixels a unit of gravel is painted at, and how far the rings round
/// a group of stones run before the straight raking takes over.
///
/// A bed gets one picture of its own rather than one tile repeated, because the
/// rings have to know where the stones are and a tile cannot.
///
/// Enough pixels for about nine to a line, so a furrow is a furrow and not a
/// stair. The rings are ten lines out, which is a whole number of them on
/// purpose: the handover lands on a line and reads as the outermost ring
/// rather than as a cut edge.
pub const PER_UNIT: f32 = 46.0;
pub const RINGS: f32 = RAKE * 10.0;

/// How far along the furrows a point is, in whole lines.
///
/// Rings where there are stones and straight lines where there are none, with
/// the handover falling on a furrow so the outermost ring reads as a line and
/// not as a cut edge.
pub fn furrow_at(here: Vec2, about: &[Vec2]) -> f32 {
    let near = about
        .iter()
        .map(|group| here.distance(*group))
        .fold(f32::MAX, f32::min);

    let from = if near < RINGS { near } else { here.y };

    from / RAKE
}

/// Raked gravel: one bed of it, painted in the garden's own coordinates.
///
/// Lines rather than a scatter. A gravel texture of pure speckle is sand, and
/// what says somebody keeps this garden is the raking.
///
/// Straight in the open and round where there are stones. Straight everywhere
/// it was wallpaper: lines with nothing to break round are corduroy, and the
/// rings are the whole point of the thing, which is that the stones are
/// islands and the gravel is water.
pub fn raked(
    seed: u32,
    middle: Vec3,
    size: Vec2,
    about: &[Vec2],
) -> blitzkit::texture::TextureData {
    let across = ((size.x * PER_UNIT).round() as u32).max(4);
    let along = ((size.y * PER_UNIT).round() as u32).max(4);
    let mut rng = seed | 1;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };

    let mut pixels = Vec::with_capacity((across * along * 4) as usize);
    for j in 0..along {
        let z = middle.z + (j as f32 / along as f32 - 0.5) * size.y;

        for i in 0..across {
            let x = middle.x + (i as f32 / across as f32 - 0.5) * size.x;
            let here = vec2(x, z);
            let phase = furrow_at(here, about);
            let furrow = (phase * std::f32::consts::TAU).sin() * 0.5 + 0.5;
            // the grit itself, which is what stops the furrows being corduroy
            let grit = (next() % 24) as f32 / 24.0;
            let tone = 142.0 + furrow * 52.0 - grit * 22.0;

            pixels.extend_from_slice(&[
                (tone * 1.02) as u8,
                (tone * 1.0) as u8,
                (tone * 0.93) as u8,
                255,
            ]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(across, along, pixels)
}

/// The faces of the garden's walls, as a box each.
///
/// Laid on the inside of whatever is already there rather than built again:
/// the hall's left wall and the nook's end wall are the garden's east and
/// south, and a second box in the same place is two surfaces in one plane.
/// This building has fought that four times.
///
/// Which also finishes them. The baths borrowed the cellar's wall and had a
/// mahogany wall down one side of a tiled room until somebody looked, and from
/// in here the nook's end wall is the back of a bookcase.
pub fn facing(reaches: f32) -> Vec<(Vec3, Vec3)> {
    let middle = at(reaches);
    let half = half();
    let proud = 0.01;
    let up = HIGH * 0.5;

    vec![
        // east, in two pieces with the way in between them. In one it ran
        // across the opening, so from the hall you looked through the gap at a
        // panel a hair behind it and the passage read as an alcove.
        (
            vec3(
                wall_at() - THICK * 0.5 - proud,
                up,
                (middle.z - half.y + way(reaches).0) * 0.5,
            ),
            vec3(proud * 2.0, HIGH, way(reaches).0 - (middle.z - half.y)),
        ),
        (
            vec3(
                wall_at() - THICK * 0.5 - proud,
                up,
                (way(reaches).1 + middle.z + half.y) * 0.5,
            ),
            vec3(proud * 2.0, HIGH, middle.z + half.y - way(reaches).1),
        ),
        // south, the nook's end wall and the garden's own beside it
        (
            vec3(middle.x, up, middle.z - half.y + proud),
            vec3(DEEP, HIGH, proud * 2.0),
        ),
        // west
        (
            vec3(middle.x - half.x + proud, up, middle.z),
            vec3(proud * 2.0, HIGH, SPAN),
        ),
        // north
        (
            vec3(middle.x, up, middle.z + half.y - proud),
            vec3(DEEP, HIGH, proud * 2.0),
        ),
    ]
}

/// The pond: how far across, how deep, and where it sits in the garden.
///
/// Off centre and off square. A pond in the middle of a square room is a
/// swimming bath, and the whole of what this kind of garden looks like is
/// things placed so that no two of them line up.
pub const POND: Vec2 = vec2(7.0, 5.4);
pub const POND_DEEP: f32 = 0.85;
pub const POND_AT: Vec2 = vec2(-1.1, 1.4);

/// How far the water sits below the gravel, and how thick the stone round it
/// is.
pub const BRIM: f32 = 0.12;
pub const KERB: f32 = 0.45;

/// Where the pond's water sits, how far across it is and how deep.
pub fn pond(reaches: f32) -> (Vec3, Vec2, f32) {
    let middle = at(reaches);

    (
        vec3(middle.x + POND_AT.x, -BRIM, middle.z + POND_AT.y),
        POND,
        POND_DEEP,
    )
}

/// The stone round the pond and the basin under it, as boxes.
///
/// The kerb stands a little proud of the gravel and the basin hangs below it,
/// so the water is held in a stone trough rather than lying on the floor.
pub fn basin(reaches: f32) -> Vec<Aabb> {
    let mut out = kerbs(reaches);
    out.push(bed(reaches));

    out
}

/// The four stones round the lip, which are the ones that read as stone.
///
/// Apart from the bed, because they are not the same thing to look at. Cut
/// together they were one colour, and a pale floor under see-through water is
/// a tiled bath with a coping round it. A pond's bottom is silt.
pub fn kerbs(reaches: f32) -> Vec<Aabb> {
    let (water, size, _) = pond(reaches);
    let half = size * 0.5;
    let out = KERB;

    vec![
        // the four kerb stones, each overlapping the next at the corners,
        // which is hidden rather than fighting: a corner inside a corner
        Aabb::from_center_size(
            vec3(water.x, BRIM * 0.5, water.z - half.y - out * 0.5),
            vec3(size.x + out * 2.0, BRIM * 2.0, out),
        ),
        Aabb::from_center_size(
            vec3(water.x, BRIM * 0.5, water.z + half.y + out * 0.5),
            vec3(size.x + out * 2.0, BRIM * 2.0, out),
        ),
        Aabb::from_center_size(
            vec3(water.x - half.x - out * 0.5, BRIM * 0.5, water.z),
            vec3(out, BRIM * 2.0, size.y),
        ),
        Aabb::from_center_size(
            vec3(water.x + half.x + out * 0.5, BRIM * 0.5, water.z),
            vec3(out, BRIM * 2.0, size.y),
        ),
    ]
}

/// The bed of it, which is what stops you at the bottom and what you see
/// through the water.
pub fn bed(reaches: f32) -> Aabb {
    let (water, size, deep) = pond(reaches);

    Aabb::from_center_size(
        vec3(water.x, -deep - BRIM - 0.15, water.z),
        vec3(size.x, 0.3, size.y),
    )
}

/// Where the trees stand, how tall each one is and how wide its crown.
///
/// Three, in no line and no two the same size. Four would fill it and two
/// would look placed.
///
/// None of them near the door. One stood two paces inside it and square in
/// front of it, which is a tree you walk into on the way in and, from a step
/// further, a tree whose crown is the whole sky.
pub fn trees(reaches: f32) -> Vec<(Vec3, f32, f32)> {
    let middle = at(reaches);
    let put = |x: f32, z: f32| vec3(middle.x + x, 0.0, middle.z + z);

    vec![
        (put(-6.8, -6.2), 4.1, 3.6),
        (put(5.4, 4.8), 3.2, 2.8),
        (put(-6.6, 6.9), 3.6, 3.1),
    ]
}

/// A tree: a trunk that tapers, and a crown of two or three flat layers over
/// it.
///
/// Layers and not a cone. A cone is a fir, and a fir is not what anybody draws
/// in a garden like this: a pine clipped in this style is a stack of flat
/// plates of foliage with sky between them.
pub fn trunk_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(10, 6, |v| {
        // thick at the foot, thin at the fork, with a swell where it leaves
        // the ground
        let waist = 0.30 - v * 0.17 + (1.0 - v).powi(4) * 0.12;

        (v, waist)
    })
}

/// One plate of foliage: wide, flat, and rounded off at the rim.
pub fn crown_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(16, 8, |v| {
        let along = v * std::f32::consts::PI;

        (0.5 - along.cos() * 0.5, along.sin())
    })
}

/// The plates of one tree: how far up each sits, how wide it is and how thick.
pub fn crowns(tall: f32, wide: f32) -> Vec<(f32, f32, f32)> {
    vec![
        (tall * 0.55, wide, 0.42),
        (tall * 0.78, wide * 0.78, 0.36),
        (tall * 0.96, wide * 0.46, 0.3),
    ]
}

/// Where the stone lanterns stand and how tall each is.
///
/// On the pond's edge and among the trees, which is where they are put in a
/// real one: a lantern is there to light a path or a water's edge.
pub fn lanterns(reaches: f32) -> Vec<(Vec3, f32)> {
    let middle = at(reaches);
    let (water, size, _) = pond(reaches);
    let half = size * 0.5;

    let _ = (water, half);

    vec![
        (vec3(middle.x - 1.1, 0.0, middle.z + 6.8), 1.5),
        (vec3(middle.x + 5.6, 0.0, middle.z + 2.0), 1.2),
        (vec3(middle.x - 1.1, 0.0, middle.z - 4.6), 1.75),
    ]
}

/// A stone lantern, turned in one piece: foot, shaft, a wide roof and a cap.
pub fn lantern_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(12, 26, |v| {
        let waist = if v < 0.1 {
            // the foot, spreading
            0.52 - v * 2.0
        } else if v < 0.52 {
            // the shaft
            0.22
        } else if v < 0.62 {
            // the shelf the light sits on
            0.30 + (v - 0.52) * 2.0
        } else if v < 0.78 {
            // the light box
            0.46
        } else if v < 0.86 {
            // the roof, oversailing it
            0.72 - (v - 0.78) * 5.0
        } else {
            // and the cap
            0.26 * (1.0 - (v - 0.86) / 0.14)
        };

        (v, waist.max(0.02))
    })
}

/// How far up a lantern its light sits, as a share of its height.
///
/// The lamps are worked out from the lanterns rather than written down a
/// second time, per what the cellar and the baths both learned: there is no
/// lamp in this garden that is not inside a lantern.
pub const LANTERN_LIT: f32 = 0.7;

/// Where the garden's lamps are, which is inside its lanterns.
pub fn lamps(reaches: f32) -> Vec<Vec3> {
    lanterns(reaches)
        .into_iter()
        .map(|(at, tall)| at + Vec3::Y * tall * LANTERN_LIT)
        .collect()
}

/// How finely the pond's water is cut up, how fast it settles and how fast a
/// wave crosses it.
///
/// Stiller than the baths. That pool is stirred by wading and by a fountain
/// and wants to answer at once; this one is a mirror with fish under it, and
/// what it has to do is hold a ring for long enough to be seen crossing.
pub const POND_CELLS: u32 = 64;
pub const SETTLES: f32 = 0.22;
pub const RUNS: f32 = 2.1;

/// How brightly a lantern burns and how far it carries.
///
/// Weak and short. A stone lantern holds a candle behind paper and lights the
/// few feet round itself; the garden's own light comes from the sky, and a
/// lantern that lit the whole room would be competing with a storm.
pub const LAMP_LIT: f32 = 1.5;
pub const LAMP_RANGE: f32 = 4.5;

/// How fat a koi is at its widest, as a fraction of its length, and how far
/// along it that is.
///
/// Forward of the middle, because a fish's shoulders are behind its head and
/// everything behind them is taper. A spindle fattest in the middle is a
/// lozenge and reads as a bar of soap.
pub const KOI_FAT: f32 = 0.26;
pub const KOI_SHOULDER: f32 = 0.62;

/// How wide a koi is against how deep it is. A fish is a flat thing carried on
/// edge, and a round one is a sausage.
pub const KOI_FLAT: f32 = 0.66;

/// How far behind the body the tail trails, how far across it spreads and how
/// far it swings, and how often it swings a second.
pub const FIN_LONG: f32 = 0.30;
pub const FIN_WIDE: f32 = 0.34;
pub const FIN_DEEP: f32 = 0.30;
pub const WAG: f32 = 0.34;
pub const BEATS: f32 = 0.75;

/// How far a koi rises and falls as it goes round, and how far under the
/// surface the shallowest of them may come.
///
/// They hang at a depth each and drift about it. Held exactly, three fish at
/// three fixed heights are three beads on three wires.
pub const BOB: f32 = 0.03;

/// How fat a koi is a given way along itself, nought at the tail and one at
/// the nose.
pub fn koi_waist(v: f32) -> f32 {
    if v >= KOI_SHOULDER {
        // the head, which is a quarter ellipse from the shoulder to a snout
        // that is blunt rather than pointed. A fish is not an arrow.
        let on = (v - KOI_SHOULDER) / (1.0 - KOI_SHOULDER);

        KOI_FAT * (1.0 - on * on * 0.86).max(0.0).sqrt()
    } else {
        // and the body behind them, down to the stalk the tail hangs off
        let on = v / KOI_SHOULDER;

        KOI_FAT * (0.07 + 0.93 * on.powf(0.75))
    }
}

/// The koi's body, turned like everything else round in this building.
pub fn koi_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(12, 18, |v| (v - 0.5, koi_waist(v)))
}

/// The tail: a fan off the stalk, root at nought and trailing edge at one.
///
/// What you see of a fish in dark water, before you see the fish, is the tail
/// going over. A body alone drifts like a leaf.
pub fn fin_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(8, 6, |v| (v, 0.10 + v * v * 0.90))
}

/// One koi's round: a circle, and nothing else.
///
/// Not simulated, per spec 0010. A fish that knows where another fish is, is a
/// spec of its own. What keeps three fixed circles from reading as three clock
/// hands is that no two share a middle, a speed, a direction or a depth, and
/// the depths are what keep them from swimming through each other.
#[derive(Clone, Copy)]
pub struct Round {
    /// The middle of the circle, from the middle of the water.
    pub about: Vec2,
    /// How far out from that middle it swims.
    pub out: f32,
    /// Turns a second, signed: a negative one goes round the other way.
    pub turns: f32,
    /// How far under the surface it hangs, and how often it rises and falls.
    pub under: f32,
    pub rises: f32,
    /// Nose to the root of the tail.
    pub long: f32,
    /// Where in its own circle it is when the room opens, in turns.
    pub from: f32,
}

/// The three of them.
///
/// Three, like the trees, and for the same reason: two is a pair and four is a
/// shoal. The outer one is the big one and the deep one is the small quick
/// one, which is the order a pond puts them in by itself.
pub const ROUNDS: [Round; 3] = [
    Round {
        about: vec2(-0.9, -0.3),
        out: 1.6,
        turns: 0.042,
        under: 0.17,
        rises: 0.11,
        long: 0.62,
        from: 0.0,
    },
    Round {
        about: vec2(1.2, 0.6),
        out: 1.5,
        turns: -0.055,
        under: 0.45,
        rises: 0.17,
        long: 0.54,
        from: 0.3,
    },
    Round {
        about: vec2(-0.4, 1.1),
        out: 0.8,
        turns: 0.071,
        under: 0.70,
        rises: 0.23,
        long: 0.42,
        from: 0.65,
    },
];

/// A koi as it is this instant.
pub struct Swims {
    pub at: Vec3,
    /// Which way its nose points, as a turn about y from +z.
    pub heading: f32,
    pub long: f32,
    /// How far the tail is over, in radians.
    pub wag: f32,
    /// How far under the surface it is hanging, which is what says how much
    /// of a wake it drags.
    pub under: f32,
}

/// Where the koi are now.
pub fn koi(reaches: f32, since: f32) -> Vec<Swims> {
    use std::f32::consts::TAU;

    let (water, _, _) = pond(reaches);

    ROUNDS
        .iter()
        .map(|round| {
            let round_to = (round.from + since * round.turns) * TAU;
            // the tangent is a quarter turn on from where it stands, and the
            // sign of `turns` is which quarter
            let way = round.turns.signum();
            let (dx, dz) = (-round_to.sin() * way, round_to.cos() * way);

            Swims {
                at: vec3(
                    water.x + round.about.x + round_to.cos() * round.out,
                    water.y - round.under + ((since * round.rises + round.from) * TAU).sin() * BOB,
                    water.z + round.about.y + round_to.sin() * round.out,
                ),
                heading: dx.atan2(dz),
                long: round.long,
                wag: ((since * BEATS + round.from) * TAU).sin() * WAG,
                under: round.under,
            }
        })
        .collect()
}

/// How hard a koi pushes the water over it, and how deep it has to be before
/// that push counts for nothing.
///
/// Per second, not per frame, so the wake is the same wake whatever the room
/// is managing. A koi near the surface drags a dimple after it and the deep
/// one drags nothing, which is the whole of why this pond is not a mirror.
///
/// Gentle. Three fish stirring hard is a chop, every facet of it catches the
/// sun, and a pond under a storm came out glittering like a sea. Most of the
/// time this is meant to be a mirror with something moving under it.
pub const STIRS: f32 = 0.28;
pub const SHOWS: f32 = 0.9;

/// How hard a koi at a given depth pushes, as a share of `STIRS`.
pub fn stirred(under: f32) -> f32 {
    (1.0 - under / SHOWS).clamp(0.0, 1.0).powi(2)
}

/// How many pixels a koi's skin is painted at.
pub const SKIN: u32 = 64;

/// A koi's skin: a pale ground with patches laid over it.
///
/// Painted rather than coloured, because one colour per fish is three fish the
/// colour of plastic. What a koi is, is the patching: a white one with red
/// over its shoulders is a different animal from a red one.
///
/// The patches run in `u`, which goes round the body, so one that falls off the
/// side comes back on the other. Along `v` it just stops, which is right: a
/// patch does not wrap round a nose.
pub fn koi_skin(seed: u32, patch: [f32; 3], patches: usize) -> blitzkit::texture::TextureData {
    let mut state = seed | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        (state >> 8) as f32 / ((1u32 << 24) as f32)
    };

    // each patch as a middle in uv, a size and a softness
    let blots: Vec<(f32, f32, f32, f32)> = (0..patches)
        .map(|_| {
            (
                next(),
                0.12 + next() * 0.76,
                0.10 + next() * 0.16,
                0.3 + next() * 0.5,
            )
        })
        .collect();

    let mut pixels = Vec::with_capacity((SKIN * SKIN * 4) as usize);
    for y in 0..SKIN {
        for x in 0..SKIN {
            let (u, v) = (x as f32 / SKIN as f32, y as f32 / SKIN as f32);
            let mut on: f32 = 0.0;

            for (bu, bv, size, soft) in blots.iter().copied() {
                // round the body the short way, so a patch at u nought is one
                // patch and not two half ones
                let du = (u - bu).abs().min(1.0 - (u - bu).abs());
                let dv = v - bv;
                let far = ((du * du) + (dv * dv)).sqrt() / size;
                let edge = (1.0 - far).clamp(0.0, 1.0);

                on = on.max(edge.powf(soft));
            }

            // hard edges. A koi's patches have a line round them; a soft wash
            // is a fish that has been left in the sun.
            let shaped = (on * 1.8 - 0.4).clamp(0.0, 1.0);
            let ground = [242.0, 240.0, 234.0];
            let paint =
                |n: usize| (ground[n] + (patch[n] * 255.0 - ground[n]) * shaped).round() as u8;

            pixels.extend_from_slice(&[paint(0), paint(1), paint(2), 255]);
        }
    }

    blitzkit::texture::TextureData::from_pixels(SKIN, SKIN, pixels)
}

/// What each of the three wears: a seed, the colour of its patches and how
/// many it has.
///
/// A red one over white, an orange one carrying more of it, and a dark one
/// with a few patches of near black. Three fish of one colour is three of the
/// same fish.
pub const SKINS: [(u32, [f32; 3], usize); 3] = [
    (0x51C3, [0.84, 0.26, 0.12], 4),
    (0x2E07, [0.90, 0.48, 0.14], 6),
    (0x7A19, [0.14, 0.13, 0.16], 3),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0010: every koi stays in the water, all the way round.
    ///
    /// Nose and the tip of the tail, not the middle. A fish placed by its
    /// middle inside a pond still has its nose through the kerb for a quarter
    /// of every lap, and the nose is the end you watch.
    #[test]
    fn every_koi_stays_in_the_water() {
        let room = room();
        let (water, size, deep) = pond(room.reaches);
        let half = size * 0.5;
        // the surface, and the top of the slab the basin's floor is
        let (top, bottom) = (water.y, water.y - deep);

        let mut step = 0.0f32;
        while step < 600.0 {
            for (n, fish) in koi(room.reaches, step).iter().enumerate() {
                let along = vec3(fish.heading.sin(), 0.0, fish.heading.cos());
                // nose at half a length; the tail's fan trails past that
                let reach = fish.long * (0.5 + FIN_LONG);
                let thick = fish.long * KOI_FAT * 0.5;

                for end in [fish.at + along * fish.long * 0.5, fish.at - along * reach] {
                    assert!(
                        (end.x - water.x).abs() < half.x && (end.z - water.z).abs() < half.y,
                        "koi {} is through the kerb at {:.1}s: {:?}",
                        n,
                        step,
                        end,
                    );
                }

                assert!(
                    fish.at.y + thick < top,
                    "koi {} breaks the surface at {:.1}s: {} against {}",
                    n,
                    step,
                    fish.at.y + thick,
                    top,
                );
                assert!(
                    fish.at.y - thick > bottom,
                    "koi {} is through the floor of the pond at {:.1}s: {} against {}",
                    n,
                    step,
                    fish.at.y - thick,
                    bottom,
                );
            }

            step += 0.05;
        }
    }

    /// Spec 0010: no two of them ever swim through each other.
    ///
    /// Nothing watches for that at run time and nothing is going to: a fish
    /// that avoids another fish is a spec of its own. The circles are laid out
    /// so it cannot happen, and this is what says they still are.
    #[test]
    fn no_two_koi_swim_through_each_other() {
        let room = room();
        let mut closest = f32::MAX;

        let mut step = 0.0f32;
        while step < 600.0 {
            let fish = koi(room.reaches, step);

            for a in 0..fish.len() {
                for b in (a + 1)..fish.len() {
                    let apart = fish[a].at.distance(fish[b].at);
                    let want = (fish[a].long + fish[b].long) * KOI_FAT * 0.5;

                    closest = closest.min(apart - want);
                    assert!(
                        apart > want,
                        "koi {} and {} are in the same water at {:.1}s: {:.3} apart, {:.3} wanted",
                        a,
                        b,
                        step,
                        apart,
                        want,
                    );
                }
            }

            step += 0.05;
        }

        // and the depths are what do it, so the clearance should be most of a
        // body rather than a hair that happens to have worked out
        assert!(
            closest > 0.05,
            "the koi clear each other by only {:.3}",
            closest,
        );
    }

    /// A koi points the way it is going.
    ///
    /// Taken from the circle rather than from where it was last frame, so this
    /// is the test that the quarter turn is the right quarter. Pointed the
    /// other way they swim the whole pond backwards, which looks like nothing
    /// in particular until you notice the tails are leading.
    #[test]
    fn a_koi_points_the_way_it_swims() {
        let room = room();

        for (n, (now, soon)) in koi(room.reaches, 0.0)
            .iter()
            .zip(koi(room.reaches, 0.25).iter())
            .enumerate()
        {
            let moved = (soon.at - now.at) * vec3(1.0, 0.0, 1.0);
            let along = vec3(now.heading.sin(), 0.0, now.heading.cos());

            assert!(moved.length() > 1e-4, "koi {} is not going anywhere", n);
            assert!(
                moved.normalize().dot(along) > 0.99,
                "koi {} swims {:?} and points {:?}",
                n,
                moved.normalize(),
                along,
            );
        }
    }

    /// The body is a fish shape: fattest forward of the middle, and it comes
    /// to something at both ends rather than being cut off.
    #[test]
    fn the_koi_is_fattest_at_its_shoulders() {
        let fattest = (0..=100)
            .map(|n| n as f32 / 100.0)
            .fold((0.0f32, 0.0f32), |best, v| {
                if koi_waist(v) > best.1 {
                    (v, koi_waist(v))
                } else {
                    best
                }
            });

        assert!(
            (fattest.0 - KOI_SHOULDER).abs() < 0.03,
            "the widest part is at {:.2}, not at the shoulders",
            fattest.0,
        );
        assert!(koi_waist(0.0) < KOI_FAT * 0.2, "the tail stalk is not thin");
        assert!(
            koi_waist(1.0) > 0.0 && koi_waist(1.0) < KOI_FAT * 0.6,
            "the snout is either a point or a cliff: {:.3}",
            koi_waist(1.0),
        );
    }

    /// Spec 0010: the shallow koi is what moves the water and the deep one is
    /// not.
    ///
    /// A pond stirred the same by all three is a pond with a mechanism in it.
    /// The thing you are meant to read off the surface is that something is
    /// down there and how far down.
    #[test]
    fn only_the_koi_near_the_surface_stir_it() {
        let stirs: Vec<f32> = ROUNDS.iter().map(|round| stirred(round.under)).collect();

        assert!(
            stirs[0] > 0.5,
            "the shallow one barely stirs: {:.2}",
            stirs[0]
        );
        assert!(
            stirs[2] < 0.1,
            "the deep one stirs the surface anyway: {:.2}",
            stirs[2]
        );
        assert!(
            stirs[0] > stirs[1] && stirs[1] > stirs[2],
            "deeper should mean less: {:?}",
            stirs
        );
        assert_eq!(stirred(SHOWS * 2.0), 0.0, "below the floor it still pushes");
    }

    /// Spec 0010: a stone is a stone and not a ball.
    ///
    /// The fault this building has hit with every turned thing is a mesh with
    /// no surface on it: too few steps and it draws two points and no skin,
    /// which is how the candles, the clock dial and the fire's coals all came
    /// out invisible at once.
    #[test]
    fn a_stone_is_cut_and_out_of_round() {
        for seed in CUTS.iter().copied() {
            let mesh = rock_mesh(seed);

            assert!(
                mesh.triangle_count() > 50,
                "a stone of {} triangles has no shape to see",
                mesh.triangle_count()
            );

            // measured round its waist rather than off its bounding box. A
            // box says how wide the widest part is and nothing about whether
            // the thing inside it is round, and a lump wandering in and out by
            // the same amount all the way round fills the same box a ball does.
            let waist: Vec<Vec3> = mesh
                .vertices
                .iter()
                .map(|vertex| Vec3::from(vertex.position))
                .filter(|at| at.y.abs() < 0.12)
                .collect();

            assert!(
                waist.len() > 12,
                "stone {:x} has no middle to measure: {} vertices",
                seed,
                waist.len()
            );

            let middle = waist.iter().copied().sum::<Vec3>() / waist.len() as f32;
            let (mut near, mut far) = (f32::MAX, 0.0f32);
            for at in waist.iter() {
                let out = vec2(at.x - middle.x, at.z - middle.z).length();

                near = near.min(out);
                far = far.max(out);
            }

            assert!(
                far > near * 1.3,
                "stone {:x} is as round as a ball: {:.3} to {:.3} across its waist",
                seed,
                near,
                far
            );
        }
    }

    /// The two seams close: round the back, and at both poles.
    ///
    /// A wander that does not come back to itself leaves a crack up the side
    /// of the stone and a tear at the top, and neither shows in a test that
    /// only counts triangles.
    #[test]
    fn a_stone_has_no_crack_up_the_back_of_it() {
        for seed in CUTS.iter().copied() {
            for step in 0..=8 {
                let v = step as f32 / 8.0;

                assert!(
                    (wander(seed, 0.0, v) - wander(seed, 1.0, v)).abs() < 1e-5,
                    "stone {:x} does not meet itself at v {}",
                    seed,
                    v
                );
            }

            for step in 0..=8 {
                let u = step as f32 / 8.0;

                for v in [0.0, 1.0] {
                    assert!(
                        wander(seed, u, v).abs() < 1e-5,
                        "stone {:x} is torn at the pole at u {}",
                        seed,
                        u
                    );
                }
            }
        }
    }

    /// Spec 0010: no group is three of a size evenly spaced.
    ///
    /// Which is the only thing anybody writing about these gardens agrees on,
    /// and the one a list of coordinates gets wrong by default.
    #[test]
    fn a_group_is_not_a_bus_queue() {
        for group in GROUPS.iter() {
            let set: Vec<_> = SET.iter().take(group.stones).collect();

            for (n, (_, size, _)) in set.iter().enumerate() {
                for (_, other, _) in set[n + 1..].iter() {
                    let (tall, short) = (size.y.max(other.y), size.y.min(other.y));

                    assert!(
                        tall > short * 1.3,
                        "two stones of a group are the same height: {} and {}",
                        size.y,
                        other.y
                    );
                }
            }

            if set.len() < 3 {
                continue;
            }

            // and not in a line, which three stones put round a middle fall
            // into the moment two of them are opposite each other
            let (a, b, c) = (set[0].0, set[1].0, set[2].0);
            let bend = (b - a).perp_dot(c - a).abs();

            assert!(
                bend > 0.2,
                "the three stones of a group are in a row: {:.3}",
                bend
            );
        }
    }

    /// Spec 0010: the raking goes round the stones and straight everywhere
    /// else.
    ///
    /// Straight everywhere, it is corduroy with rocks standing on it. The
    /// rings are what say the stones are islands.
    #[test]
    fn the_raking_runs_round_the_stones() {
        let reaches = room().reaches;
        let about = group_middles(reaches);
        let first = about[0];

        // two points the same way out from a group are on the same furrow,
        // whichever side of it they are
        let near = RINGS * 0.5;
        for turn in 0..8 {
            let angle = turn as f32 / 8.0 * std::f32::consts::TAU;
            let on = first + vec2(angle.cos(), angle.sin()) * near;

            assert!(
                (furrow_at(on, &about) - near / RAKE).abs() < 1e-4,
                "the raking does not ring the stones at {:?}",
                on
            );
        }

        // and well away from every group it runs in lines of a steady z
        let far = vec2(first.x, first.y) + vec2(0.0, RINGS * 3.0);
        assert!(
            (furrow_at(far, &about) - furrow_at(far + vec2(2.0, 0.0), &about)).abs() < 1e-4,
            "the raking is not straight away from the stones",
        );
    }

    /// Spec 0010: no part of the hall is laid over the garden.
    ///
    /// The hall's floor is two slabs, and the carpet and the ceiling are now
    /// drawn from those two slabs rather than from one rectangle covering both.
    /// The rectangle was bigger than the floor by the corner the hall and the
    /// nook do not share, and that corner is out of doors: it put a patch of
    /// carpet on the gravel and a slab of ceiling in the sky over the garden,
    /// which is what you saw walking in and looking back at the door.
    ///
    /// Testing the floor rather than the drawing, because the drawing reads
    /// the floor. Two accounts of one number is the fault this building keeps
    /// making, and this is the test that there is only one account.
    #[test]
    fn nothing_of_the_hall_is_laid_over_the_garden() {
        let room = room();
        let middle = at(room.reaches);
        let half = half();

        for (n, slab) in room.floors().iter().enumerate() {
            let (at_, size) = (slab.center(), slab.size());
            let overlaps = |a: f32, ah: f32, b: f32, bh: f32| (a - b).abs() < ah + bh - 1e-4;

            assert!(
                !(overlaps(at_.x, size.x * 0.5, middle.x, half.x)
                    && overlaps(at_.z, size.z * 0.5, middle.z, half.y)),
                "floor slab {} covers ground the garden is on: x {} .. {}, z {} .. {}",
                n,
                at_.x - size.x * 0.5,
                at_.x + size.x * 0.5,
                at_.z - size.z * 0.5,
                at_.z + size.z * 0.5,
            );
        }

        // and the negative control, because a test that cannot fail has no
        // teeth. One rectangle round both slabs is what the carpet and the
        // ceiling were, and it is over the garden by six units in x and five
        // in z.
        let slabs = room.floors();
        let (low, high) = (
            slabs[0].min.min(slabs[1].min),
            slabs[0].max.max(slabs[1].max),
        );
        let (one, size) = ((low + high) * 0.5, high - low);
        let overlaps = |a: f32, ah: f32, b: f32, bh: f32| (a - b).abs() < ah + bh - 1e-4;

        assert!(
            overlaps(one.x, size.x * 0.5, middle.x, half.x)
                && overlaps(one.z, size.z * 0.5, middle.z, half.y),
            "the rectangle round both slabs should be the thing that overhangs",
        );
    }

    /// The room these tests walk, and how far it reaches.
    ///
    /// Taken from the room rather than written down. It was written down, as
    /// 9.4, which is the real project's fourteen cabinets; the room built here
    /// has twelve and reaches 8.3, so every target these tests walked towards
    /// was a step and a half outside the garden. Two accounts of one number,
    /// which is the fault this building keeps making.
    fn room() -> crate::room::Room {
        crate::room::Room::of(
            (0..12)
                .map(|n| {
                    crate::cabinet::Cabinet::found(
                        &format!("game{}", n),
                        std::path::Path::new("/nowhere"),
                    )
                })
                .collect(),
        )
    }

    /// Walks a body your size from one place to another and says where it got
    /// to.
    ///
    /// Every box the walk could meet, not just this room's: the hall's left
    /// wall is the garden's east side and the nook's end wall is its south, so
    /// a walk that only knew the garden's own boxes would stroll out through
    /// both of them.
    fn walked(from: Vec3, to: Vec3, seconds: f32) -> Vec3 {
        let solid = room().solid();
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;

        for _ in 0..(seconds / step) as u32 {
            let way = vec3(to.x - at.x, 0.0, to.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * crate::SPEED
            } else {
                Vec3::ZERO
            };

            let (next, fell) = crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid);
            at = next;
            falling = fell;
        }

        at
    }

    /// Spec 0010: you can get into the garden from where you wake up.
    #[test]
    fn you_can_walk_in_from_where_you_wake() {
        let middle = at(room().reaches);
        // dropped in from above rather than placed, because a point on the
        // floor can be inside something
        let wake = vec3(0.0, 1.2, room().reaches - crate::room::INSIDE);

        let got = walked(wake, middle, 6.0);

        assert!(
            got.x < wall_at() - THICK,
            "it never got through the wall: {:?}",
            got
        );
        assert!(
            (got.z - middle.z).abs() < 2.0,
            "it got through and then lost its way: {:?}",
            got
        );
    }

    /// Spec 0010: nothing stands in the pond, against a wall, or on the walk
    /// round the water.
    ///
    /// Arithmetic, so it belongs here rather than in the window. Two trees
    /// were put in the walkway's own corners and the walk round fetched up
    /// inside them, which is the kind of thing a screenshot shows you only if
    /// you happen to stand in the right place.
    #[test]
    fn nothing_stands_where_you_have_to_walk() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let (water, size, _) = pond(reaches);
        let wet = size * 0.5 + Vec2::splat(KERB);
        let ring = size * 0.5 + Vec2::splat(KERB + crate::RADIUS + 0.55);
        let body = crate::RADIUS * 2.0;

        let standing: Vec<(&str, Vec3, f32)> = trees(reaches)
            .into_iter()
            .map(|(where_, _, _)| ("a tree", where_, 0.34))
            .chain(
                lanterns(reaches)
                    .into_iter()
                    .map(|(where_, _)| ("a lantern", where_, 0.5)),
            )
            .chain(
                group_at(reaches)
                    .into_iter()
                    .map(|(where_, wide)| ("a set of stones", where_, wide)),
            )
            .collect();

        for (what, where_, across) in &standing {
            let off = vec2(where_.x - water.x, where_.z - water.z).abs();

            assert!(
                off.x > wet.x + across * 0.5 || off.y > wet.y + across * 0.5,
                "{} stands in the pond at {:?}",
                what,
                where_
            );

            for (side, gap) in [
                ("the west wall", where_.x - (middle.x - half.x)),
                ("the east wall", (middle.x + half.x) - where_.x),
                ("the south wall", where_.z - (middle.z - half.y)),
                ("the north wall", (middle.z + half.y) - where_.z),
            ] {
                assert!(
                    gap > across * 0.5 + body,
                    "{} is {:.2} from {}, which nobody can get past",
                    what,
                    gap,
                    side
                );
            }

            // and nothing standing in the way in. A tree two paces inside the
            // door is a tree you meet before you have seen the garden, and
            // from under it the sky is leaves.
            let (from_z, to_z) = way(reaches);
            let in_the_way = where_.x > wall_at() - 2.6
                && where_.z > from_z - across * 0.5 - crate::RADIUS
                && where_.z < to_z + across * 0.5 + crate::RADIUS;

            assert!(!in_the_way, "{} stands in the way in at {:?}", what, where_);

            let clear = across * 0.5 + crate::RADIUS;
            let on_a_side = (off.x - ring.x).abs() < clear && off.y < ring.y + clear;
            let on_an_end = (off.y - ring.y).abs() < clear && off.x < ring.x + clear;

            assert!(
                !(on_a_side || on_an_end),
                "{} stands on the walk round the pond at {:?}",
                what,
                where_
            );
        }

        for (n, (what, where_, across)) in standing.iter().enumerate() {
            for (other, too, wide) in &standing[n + 1..] {
                let apart = vec2(where_.x - too.x, where_.z - too.z).length();

                assert!(
                    apart > (across + wide) * 0.5 + 0.3,
                    "{} and {} are {:.2} apart",
                    what,
                    other,
                    apart
                );
            }
        }
    }

    /// Spec 0010: the walkway round the pond goes all the way round.
    ///
    /// Round it and not across it. The first go at this walked corner to
    /// corner and passed, then stopped passing the moment there was a pond in
    /// the middle, which is the test being naive rather than the room being
    /// wrong: nobody walks through a pond. What has to be true is that the
    /// path round the water joins up, and that is what this walks.
    #[test]
    fn the_walkway_goes_right_round_the_pond() {
        let reaches = room().reaches;
        let (water, size, _) = pond(reaches);
        let out = size * 0.5 + Vec2::splat(KERB + crate::RADIUS + 0.55);

        // the four sides of the walk, taken in order
        let round = [
            vec3(water.x - out.x, 1.2, water.z - out.y),
            vec3(water.x + out.x, 1.2, water.z - out.y),
            vec3(water.x + out.x, 1.2, water.z + out.y),
            vec3(water.x - out.x, 1.2, water.z + out.y),
        ];

        let mut at = round[0];
        for (n, to) in round.iter().enumerate().skip(1).chain([(0, &round[0])]) {
            at = walked(at, *to, 7.0);
            assert!(
                (at.x - to.x).abs() < 1.1 && (at.z - to.z).abs() < 1.1,
                "the walk round stopped at corner {}: {:?} rather than {:?}",
                n,
                at,
                to
            );
        }
    }

    /// Spec 0010: and out again from the far side of the pond, by way of the
    /// door, which is what a person does.
    #[test]
    fn you_can_walk_back_out_from_the_far_side() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let (from_z, to_z) = way(reaches);
        let door = vec3(wall_at() - 1.0, 1.2, (from_z + to_z) * 0.5);
        let wake = vec3(0.0, 0.0, reaches - crate::room::INSIDE);

        // the far corner, behind the pond from the door
        let far = vec3(
            middle.x - half.x + crate::RADIUS + 1.2,
            1.2,
            middle.z + half.y - crate::RADIUS - 1.2,
        );
        // round the north side of the water, then down to the door
        let north = vec3(middle.x + half.x - crate::RADIUS - 1.2, 1.2, far.z);

        let at_north = walked(far, north, 8.0);
        let at_door = walked(at_north, door, 9.0);
        assert!(
            (at_door.z - door.z).abs() < 1.1 && (at_door.x - door.x).abs() < 1.4,
            "it never reached the door: {:?}",
            at_door
        );

        let got = walked(at_door, wake, 5.0);
        assert!(
            got.x > wall_at(),
            "it reached the door and not through it: {:?}",
            got
        );
    }
}
