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

/// The band over the hall's wall: the metre between the hall's ceiling and
/// the top of the garden's own walls, as a bottom and a top.
///
/// Named because it was written twice and the two came apart. The parapet
/// was in two pieces either side of the way in, one of them sprung from
/// `(TALL + HIGH) * 0.5` and the other from `HIGH * 0.55`, which is 2.31
/// against 3.70. The low one sat inside the wall and left the strip over it
/// open, and from the garden you were looking at a slot of sky over the way
/// in.
pub fn parapet() -> (f32, f32) {
    (crate::room::TALL, HIGH)
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
        //
        // In one piece and not two. Two is what the way in asked for, since a
        // parapet across the opening was a beam over the entrance while the
        // opening still had a lintel in it. The lintel is gone and the
        // opening stops at the hall's own ceiling, so the band over it is the
        // head of the doorway and not a beam. Left out, it was a notch of sky
        // over the way in, which is what Jake saw from the garden.
        Aabb::from_center_size(
            vec3(
                wall_at(),
                (parapet().0 + parapet().1) * 0.5,
                (near(reaches) + reaches) * 0.5,
            ),
            vec3(THICK, parapet().1 - parapet().0, reaches - near(reaches)),
        ),
        // and a parapet over the rest of it, where the nook's end wall closes
        // the garden. That wall stops at the nook's ceiling and the garden is a
        // metre taller, so there was a strip of nothing along the whole of it:
        // from inside the nook, looking up near that end, a wedge of the
        // garden's sky came through the ceiling.
        //
        // The same fault as the parapet over the hall's wall, on the other
        // borrowed wall. Both were borrowed and only one was topped.
        Aabb::from_center_size(
            vec3(
                west + DEEP - NOOK_DEEP * 0.5,
                (crate::room::TALL + HIGH) * 0.5,
                south,
            ),
            vec3(NOOK_DEEP, HIGH - crate::room::TALL, THICK),
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
            vec3((west + wall_at()) * 0.5, -THICK * 0.5, middle.z),
            vec3(wall_at() - west, THICK, SPAN),
        ),
    ];

    // the pond's stone, which you stand on rather than in
    out.extend(basin(reaches));
    // and the trees and lanterns, which you walk round
    out.extend(trees(reaches).iter().map(trunk_box));
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
pub const SKY: u32 = 512;
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
    let octaves: Vec<(usize, Vec<f32>)> = [12usize, 24, 48, 96]
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

            // The picture goes on the dome as a disc seen from below, so the
            // apex is the middle of it and the rim is the circle round the
            // edge. This is how far up the dome a pixel lands, which is not
            // the way up the picture looks.
            //
            // Taken the other way the heaviest cloud was round the horizon
            // and the darkest slate was directly overhead, which is a storm
            // seen from above.
            let from_middle =
                (((u - 0.5) * (u - 0.5) + (v - 0.5) * (v - 0.5)).sqrt() / 0.5).min(1.0);
            let overhead = 1.0 - from_middle;
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

    // The normals off the shape itself rather than averaged off the faces.
    //
    // The forty-nine vertices at the apex are all in one place, so the
    // triangles between them have no area and every normal there came out of
    // a cross product made of rounding. Forty-eight of them pointed down and
    // the one at the seam pointed up, which drew a dark spoke out of the top
    // of the sky.
    //
    // A half ellipsoid of DOME by DOME_UP by DOME has its outward normal at
    // a point along that point over the square of each axis, and this is the
    // one surface here only ever seen from within, so it is the other way
    // about. At the apex that is straight down, with no special case for it
    // and nothing left to round.
    for vertex in mesh.vertices.iter_mut() {
        let [x, y, z] = vertex.position;
        let out = vec3(
            x / (DOME * DOME),
            y / (DOME_UP * DOME_UP),
            z / (DOME * DOME),
        );

        vertex.normal = (-out).normalize_or_zero().to_array();

        // and the picture laid on as a disc seen from below rather than
        // wrapped round like a map of the world.
        //
        // Wrapped, every one of the forty-eight columns ran into the apex and
        // the whole top edge of the picture was wedged into one point: it
        // drew a dark star over the middle of the sky, which is the part of
        // it you lie in the garden and look at. The middle of a disc is a
        // point to begin with, so there is nothing there to pinch, and the
        // seam goes with it: the two sides of the picture that used to meet
        // round the back now fall on the same line of texels.
        //
        // The angle down from the apex gives the radius, so the scale is even
        // from the middle out. Only the circle inside the picture is used and
        // the four corners are not, which is a fifth of it spent on having no
        // pole.
        let round = z.atan2(x);
        let from_top = (y / DOME_UP).clamp(-1.0, 1.0).acos() / std::f32::consts::FRAC_PI_2;

        vertex.uv = [
            0.5 + from_top * 0.5 * round.cos(),
            0.5 + from_top * 0.5 * round.sin(),
        ];
    }

    mesh
}

/// How high the sky springs.
///
/// Between the building's ceilings and the top of the garden's walls, and both
/// ends of that are load bearing.
///
/// Above the ceilings, because the dome is nineteen across and the garden is
/// sixteen: it reaches well past the garden and over the hall and the nook,
/// and anything of it below their ceilings is inside those rooms. Sprung at a
/// little over half the garden's height, which it was, the rim sat at 2.31 and
/// every ceiling in this building is at 3.2, so the rim ran through the nook
/// and the hall at head height. From in there it is a pale blade hanging
/// through the ceiling, and it took a recording, a camera put exactly where
/// Jake was standing, and a build with the dome left out to find.
///
/// Below the garden's own walls, which is the older reason: level with them or
/// over them and you see the rim from inside the garden, which is the edge of
/// the sky.
pub fn springs() -> f32 {
    (crate::room::TALL + HIGH) * 0.5
}

/// Where the dome stands: over the middle of the garden, springing between the
/// building's ceilings and the top of the garden's walls.
pub fn dome_at(reaches: f32) -> Vec3 {
    let middle = at(reaches);

    vec3(middle.x, springs(), middle.z)
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
    wandered(seed, u, v, WANDER)
}

/// The same, by a given amount: a stone wanders further than a pad of foliage.
pub fn wandered(seed: u32, u: f32, v: f32, by: f32) -> f32 {
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

    out / weight * by
}

/// How far a stone leans off its own axis, as a share of its height.
pub const ROCK_LEANS: f32 = 0.24;

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
    let belly = 0.4 + (next() % 100) as f32 / 125.0;
    let way = vec2(angle.cos(), angle.sin());

    // a lean, and a belly on one side of it
    way * ((v - 0.5) * ROCK_LEANS + (v * PI).sin() * ROCK_LEANS * belly * 0.5)
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

/// How far the path bows off the straight line between its ends.
///
/// Bowed, and bowed away from the water. A straight line between two points is
/// a kerb, and the whole business of a path of these is that it makes you look
/// down and take one step at a time.
pub const PATH_BOW: f32 = 0.9;

/// How far apart the stones are, middle to middle, and how far each is thrown
/// off the line of the path.
///
/// A stride. Closer and you shuffle, further and you stretch, and the one thing
/// everybody says about these is that the spacing is the pace they want you to
/// go at.
pub const STRIDE: f32 = 0.62;
pub const THROWN: f32 = 0.07;

/// How far a stepping stone stands out of the gravel.
///
/// A knuckle. Flush, it is a pattern on the floor; any higher and it is
/// something to trip on, which is the opposite of what it is for. Low enough
/// that it is not in `solid` either: a thing you step on is not a thing you
/// walk round.
pub const STEP_UP: f32 = 0.055;

/// How wide the stepping stones run.
pub const STEPS_WIDE: (f32, f32) = (0.40, 0.58);

/// Where the path starts: a stride inside the way in, on the middle of it.
///
/// Taken from the opening rather than written down. It was written down, as a
/// place in the garden, and the opening is a fixed distance from the near end
/// of the hall while the garden's middle is not: add two cabinets and the hall
/// gets longer, the garden's middle moves half of that, and the path's first
/// stone ends up a stride and a half outside the door. This is the same fault
/// the tests in here were already built to catch once.
pub fn path_from(reaches: f32) -> Vec2 {
    let (from_z, to_z) = way(reaches);

    vec2(wall_at() - STRIDE * 1.5, (from_z + to_z) * 0.5)
}

/// And where it ends: off the near corner of the pond's stone, by half a step,
/// so the last one is at the water and not on the kerb.
pub fn path_to(reaches: f32) -> Vec2 {
    let (water, size, _) = pond(reaches);
    let stone = size * 0.5 + Vec2::splat(KERB);

    vec2(
        water.x + stone.x + STRIDE * 0.5,
        water.z - stone.y - STRIDE * 0.5,
    )
}

/// A point along the path, from nought at the door to one at the water.
pub fn path_at(reaches: f32, on: f32) -> Vec2 {
    let (from, to) = (path_from(reaches), path_to(reaches));
    let line = to - from;
    // bowed away from the water, which is the side the pond is not on
    let off = vec2(-line.y, line.x).normalize_or_zero() * PATH_BOW * 2.0;
    let bend = (from + to) * 0.5 + off;
    let back = 1.0 - on;

    from * (back * back) + bend * (2.0 * back * on) + to * (on * on)
}

/// One stepping stone: where it sits, how far across, which way round and
/// which cut.
pub struct Step {
    pub at: Vec3,
    pub wide: f32,
    pub turn: f32,
    pub cut: usize,
}

/// The stepping stones, laid a stride apart along the path.
///
/// Walked rather than divided. The path is bowed, so dividing nought to one
/// into equal pieces puts the stones closer together round the bend and further
/// apart on the straights, which is the one thing the spacing is not allowed to
/// do.
pub fn steps(reaches: f32) -> Vec<Step> {
    const SAMPLES: usize = 400;

    let mut walked = Vec::with_capacity(SAMPLES + 1);
    let mut along = 0.0;
    let mut last = path_at(reaches, 0.0);

    walked.push((0.0f32, last));
    for n in 1..=SAMPLES {
        let here = path_at(reaches, n as f32 / SAMPLES as f32);
        along += here.distance(last);
        walked.push((along, here));
        last = here;
    }

    let mut out = Vec::new();
    let mut want = 0.0;
    let mut n = 0usize;

    while want <= along {
        let found = walked
            .windows(2)
            .find(|pair| pair[1].0 >= want)
            .map(|pair| {
                let (from, to) = (pair[0], pair[1]);
                let part = if to.0 > from.0 {
                    (want - from.0) / (to.0 - from.0)
                } else {
                    0.0
                };

                (from.1 + (to.1 - from.1) * part, (to.1 - from.1))
            });

        if let Some((where_, way)) = found {
            // thrown off the line, left and right by turns, so the path is not
            // a dotted line somebody drew
            let side = vec2(-way.y, way.x).normalize_or_zero();
            let jog = if n.is_multiple_of(2) { THROWN } else { -THROWN };
            let on = where_ + side * jog;
            // and each one its own size and its own way round. A path of one
            // stone repeated is a row of tiles.
            let wiggle = ((n as f32 * 2.399).sin() * 0.5 + 0.5).clamp(0.0, 1.0);

            out.push(Step {
                at: vec3(on.x, 0.0, on.y),
                wide: STEPS_WIDE.0 + (STEPS_WIDE.1 - STEPS_WIDE.0) * wiggle,
                turn: n as f32 * 1.37,
                cut: n % CUTS.len(),
            });
        }

        want += STRIDE;
        n += 1;
    }

    out
}

/// How thick a stepping stone is against how wide.
///
/// How deep it sits is not written down beside this. The top stands `STEP_UP`
/// out of the gravel and the rest of it is under, which is one number and not
/// two: a thickness and a sinking, named apart, are two accounts of where the
/// top of the stone is and they come apart the first time either moves.
pub const SLAB_THICK: f32 = 0.34;

/// How far along a slab a given point is and how far out, as a share of the
/// whole: a flat top, a rim that is widest below it, and a flat bottom.
///
/// Not a squashed boulder. A turned lump flattened has a domed top, and the top
/// is the one face of a stepping stone anybody ever sees: it has to be flat
/// enough to stand on and to read as cut.
pub fn slab_at(v: f32) -> (f32, f32) {
    if v < 0.12 {
        (-0.5, v / 0.12 * 0.92)
    } else if v < 0.5 {
        let on = (v - 0.12) / 0.38;

        (-0.5 + on * 0.46, 0.92 + on * 0.08)
    } else if v < 0.88 {
        let on = (v - 0.5) / 0.38;

        (-0.04 + on * 0.54, 1.0 - on * 0.12)
    } else {
        let on = (v - 0.88) / 0.12;

        (0.5, 0.88 * (1.0 - on))
    }
}

/// One stepping stone, cut.
pub fn slab_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    faceted(blitzkit::mesh::MeshData::surface(CUT_ROUND, 12, |u, v| {
        let round = -u * TAU;
        let (up, wide) = slab_at(v);
        // the outline wanders, and like the stones' it comes back to
        // nothing at both ends, which here are the middles of the two
        // flat faces rather than the top and the bottom of a lump
        let out = wide * 0.5 * (1.0 + wander(seed, u, v) * 1.3);

        vec3(round.cos() * out, up, round.sin() * out)
    }))
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
        // and the band over the way in itself, which the two pieces either
        // side of it leave open. The opening stops at the hall's ceiling and
        // the garden's wall is a metre taller, so without this you look at a
        // notch of sky over the doorway.
        //
        // The whole thickness of the wall and not a skin on the face of it,
        // which every other piece here is. Above the hall's head there is no
        // hall wall to be a skin on: a skin alone left a hand's breadth of
        // open channel behind it, and from the garden, at the angle you look
        // up at a doorway from, you could see sky through the slot. It was
        // four pixels and it was the same fault as the metre.
        (
            vec3(
                wall_at(),
                (parapet().0 + parapet().1) * 0.5,
                (way(reaches).0 + way(reaches).1) * 0.5,
            ),
            vec3(
                THICK + proud * 2.0,
                parapet().1 - parapet().0,
                way(reaches).1 - way(reaches).0,
            ),
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

/// How high the bank round the water stands out of the gravel, how far below
/// its top the water sits, and how wide the band of stone is.
///
/// The bank is higher than you can step, and that number is not a choice. The
/// garden's ground is one slab under the whole of it, the pond included, so
/// there is no hole to fall into: the water is drawn over ground you can stand
/// on, and anything round it low enough to climb is a lip you walk over to
/// stand on the pond. `walk::STEP` is 0.42, and this has to clear it.
///
/// A raised pond rather than a sunken one with a wall round it, because the
/// second is a trough. The water comes up to a hand's width under the lip,
/// which is where a koi pond's water is: near enough the top to be the thing
/// you see when you look at it, and near enough to lean on.
pub const BANK_UP: f32 = 0.46;
pub const BRIM: f32 = 0.10;
pub const KERB: f32 = 0.45;

/// Where the pond's water sits, how far across it is and how deep.
pub fn pond(reaches: f32) -> (Vec3, Vec2, f32) {
    let middle = at(reaches);

    (
        vec3(middle.x + POND_AT.x, BANK_UP - BRIM, middle.z + POND_AT.y),
        POND,
        POND_DEEP,
    )
}

/// The stone round the pond and the basin under it, as boxes.
///
/// The kerb stands a little proud of the gravel and the basin hangs below it,
/// so the water is held in a stone trough rather than lying on the floor.
pub fn basin(reaches: f32) -> Vec<Aabb> {
    let mut out = bank(reaches);
    out.push(bed(reaches));
    // and the stones set on the bank, which are things to walk round in their
    // own right rather than a pattern on it
    out.extend(edging(reaches).into_iter().map(|edge| {
        let (long, across) = if edge.turn.cos().abs() > 0.5 {
            (edge.size.x, edge.size.z)
        } else {
            (edge.size.z, edge.size.x)
        };

        Aabb::from_center_size(
            edge.at + Vec3::Y * edge.size.y * 0.5,
            vec3(long, edge.size.y, across),
        )
    }));

    out
}

/// The bank: four runs round the lip, meeting at the corners, each one the
/// basin's wall on that side and its rim in the same box.
///
/// Each overlaps the next at a corner, which is hidden rather than fighting: a
/// corner inside a corner.
///
/// The bank is what holds the water and what stops you walking into it. The
/// stones are set on top of it: they vary, and a thing that varies cannot also
/// be the thing that has to be continuous and higher than a step.
pub fn bank(reaches: f32) -> Vec<Aabb> {
    let (water, size, deep) = pond(reaches);
    let half = size * 0.5;
    let out = KERB;
    let foot = water.y - deep;
    let up = (foot + BANK_UP) * 0.5;
    let thick = BANK_UP - foot;

    vec![
        Aabb::from_center_size(
            vec3(water.x, up, water.z - half.y - out * 0.5),
            vec3(size.x + out * 2.0, thick, out),
        ),
        Aabb::from_center_size(
            vec3(water.x, up, water.z + half.y + out * 0.5),
            vec3(size.x + out * 2.0, thick, out),
        ),
        Aabb::from_center_size(
            vec3(water.x - half.x - out * 0.5, up, water.z),
            vec3(out, thick, size.y),
        ),
        Aabb::from_center_size(
            vec3(water.x + half.x + out * 0.5, up, water.z),
            vec3(out, thick, size.y),
        ),
    ]
}

/// How long the stones round the lip run, how high they stand, and how far
/// across the band each is.
///
/// Across more than the band is wide, and set towards the water, so a stone
/// leans out over it. A run of them cut flush with the water's edge is a kerb
/// however irregular it is: what breaks the line is the shadow under an
/// overhang.
pub const EDGE_LONG: (f32, f32) = (0.52, 1.12);
pub const EDGE_UP: (f32, f32) = (0.12, 0.26);
pub const EDGE_ACROSS: (f32, f32) = (0.46, 0.62);
pub const EDGE_OVER: f32 = 0.42;

/// How far a stone is bedded into the top of the bank.
///
/// A little. Sat on it they are a course of blocks, and what wants to be true
/// of a set stone is that it was put where it is and the bank was made to take
/// it.
pub const EDGE_BED: f32 = 0.055;

/// How far apart the stones are set, and how wide the occasional proper gap is.
///
/// Neither is a gap you can get through: a body is nine tenths across, and a
/// hole in the stones that lets you walk into the pond is a hole whichever way
/// it looks.
pub const EDGE_GAP: f32 = 0.055;
pub const EDGE_BREAK: f32 = 0.34;

/// How much taller a rough stone stands than a flat one in the same run.
///
/// A lip of flats alone is crazy paving stood on edge. What an edge like this
/// is, is mostly flats with a boulder every few feet holding them, and the
/// boulder is the taller thing or there is no point to it.
pub const EDGE_ROUGH: f32 = 1.45;

/// One stone set round the lip.
pub struct Edge {
    pub at: Vec3,
    /// Along the side it is on, up, and across the band.
    pub size: Vec3,
    pub turn: f32,
    pub cut: usize,
    /// Whether it is a boulder rather than a flat.
    pub rough: bool,
}

/// The stones round the lip of the pond.
///
/// Laid along each side in turn and a bigger one at each corner, which is how
/// an edge like this is actually built: the corners are set first because they
/// are the two directions at once, and the runs are filled in between them.
pub fn edging(reaches: f32) -> Vec<Edge> {
    let (water, size, _) = pond(reaches);
    let half = size * 0.5;
    let out = KERB;
    // the stones sit towards the water across the band, so their inner halves
    // are inside the bank and their outer edges stand proud of it
    let set = out * EDGE_OVER;

    let mut laid = Vec::new();
    let mut n = 0usize;
    // a wiggle that is the same every run and different every stone
    let mut wiggle = move |step: usize| {
        let on = (n as f32 * 2.3999632 + step as f32 * 0.7).sin() * 0.5 + 0.5;
        n += 1;

        on.clamp(0.0, 1.0)
    };

    for (side, (middle, along, across)) in [
        (
            vec3(water.x, BANK_UP - EDGE_BED, water.z - half.y - set),
            vec2(1.0, 0.0),
            vec2(0.0, -1.0),
        ),
        (
            vec3(water.x, BANK_UP - EDGE_BED, water.z + half.y + set),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
        ),
        (
            vec3(water.x - half.x - set, BANK_UP - EDGE_BED, water.z),
            vec2(0.0, 1.0),
            vec2(-1.0, 0.0),
        ),
        (
            vec3(water.x + half.x + set, BANK_UP - EDGE_BED, water.z),
            vec2(0.0, 1.0),
            vec2(1.0, 0.0),
        ),
    ]
    .iter()
    .enumerate()
    {
        let run = if along.x > 0.5 { size.x } else { size.y };
        let turn = if along.x > 0.5 {
            0.0
        } else {
            std::f32::consts::FRAC_PI_2
        };
        let mut on = 0.0;

        while on < run {
            let long = (EDGE_LONG.0 + (EDGE_LONG.1 - EDGE_LONG.0) * wiggle(0)).min(run - on);

            // a stub is not a stone. The leftover at the end of a run goes to
            // the corner rather than being laid as a chip.
            if long < EDGE_LONG.0 * 0.6 {
                break;
            }

            let at = middle
                + Vec3::new(along.x, 0.0, along.y) * (on + long * 0.5 - run * 0.5)
                + Vec3::new(across.x, 0.0, across.y) * (wiggle(1) - 0.5) * 0.06;

            // one in three a boulder, and the long stones stay flat: a
            // boulder as long as a bench is a wall
            let rough = laid.len() % 3 == 1 && long < EDGE_LONG.1 * 0.8;
            let up = (EDGE_UP.0 + (EDGE_UP.1 - EDGE_UP.0) * wiggle(2))
                * if rough { EDGE_ROUGH } else { 1.0 };

            laid.push(Edge {
                at,
                size: vec3(
                    long,
                    up,
                    EDGE_ACROSS.0 + (EDGE_ACROSS.1 - EDGE_ACROSS.0) * wiggle(3),
                ),
                // a few degrees each, so the run is set and not sawn
                turn: turn + (wiggle(4) - 0.5) * 0.17,
                cut: (side + laid.len()) % CUTS.len(),
                rough,
            });

            on += long
                + if laid.len() % 4 == 2 {
                    EDGE_BREAK
                } else {
                    EDGE_GAP
                };
        }
    }

    // and the four corners, which are two directions at once and so the biggest
    // stones in the run
    for (x, z) in [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
        let at = vec3(
            water.x + x * (half.x + set),
            BANK_UP - EDGE_BED,
            water.z + z * (half.y + set),
        );

        laid.push(Edge {
            at,
            size: vec3(
                EDGE_LONG.1 * 0.78,
                EDGE_UP.1 * EDGE_ROUGH,
                EDGE_ACROSS.1 * 1.05,
            ),
            turn: (x * z) * 0.6,
            cut: ((x + 1.0) as usize + (z + 1.0) as usize * 2) % CUTS.len(),
            // a boulder at each corner, which is what holds a run of flats
            rough: true,
        });
    }

    laid
}

/// The bed of it, which is what stops you at the bottom and what you see
/// through the water.
pub fn bed(reaches: f32) -> Aabb {
    let (water, size, deep) = pond(reaches);

    Aabb::from_center_size(
        vec3(water.x, water.y - deep - 0.15, water.z),
        vec3(size.x, 0.3, size.y),
    )
}

/// How far a tree leans off upright, as a share of its own height, and how
/// sharply the lean comes on up the trunk.
///
/// Leaning, because a tree in a garden like this is pruned for years to lean,
/// and a straight trunk with discs on it is a lollipop. Which way it leans is
/// not written down: every one of them leans towards the water, which is what
/// they are pruned to do and what puts something over the pond to look through.
///
/// The bend comes on with the square and a bit of the height, so the foot is
/// upright and the lean is in the top half. A straight slope is a mast guyed
/// over; a tree bends.
pub const LEANS: f32 = 0.26;
pub const BENDS: f32 = 1.7;

/// One tree.
pub struct Tree {
    pub at: Vec3,
    pub tall: f32,
    pub wide: f32,
    /// Which way it leans, as a direction on the floor.
    pub leans: Vec2,
    pub seed: u32,
}

/// Where the trees stand, how tall each is and how wide its crown.
///
/// Three, in no line and no two the same size. Four would fill it and two
/// would look placed.
///
/// None of them near the door. One stood two paces inside it and square in
/// front of it, which is a tree you walk into on the way in and, from a step
/// further, a tree whose crown is the whole sky.
pub fn trees(reaches: f32) -> Vec<Tree> {
    let middle = at(reaches);
    let (water, _, _) = pond(reaches);

    [
        (vec2(-6.8, -6.2), 4.1f32, 3.6f32, 0x3D71u32),
        (vec2(5.4, 4.8), 3.2, 2.8, 0x8C44),
        (vec2(-6.6, 6.9), 3.6, 3.1, 0x16BE),
    ]
    .iter()
    .map(|(off, tall, wide, seed)| {
        let stands = vec3(middle.x + off.x, 0.0, middle.z + off.y);

        Tree {
            at: stands,
            tall: *tall,
            wide: *wide,
            leans: vec2(water.x - stands.x, water.z - stands.z).normalize_or_zero(),
            seed: *seed,
        }
    })
    .collect()
}

/// How far along its lean a trunk is a given way up itself, from nought at the
/// foot to one at the top.
pub fn bend(on: f32) -> f32 {
    on.clamp(0.0, 1.0).powf(BENDS)
}

/// Where a tree's trunk is a given way up itself.
///
/// The one account of the trunk's line. The mesh is built from it, the pads
/// hang off it, the limbs start on it and the collider follows it, and the
/// first version of this had the lean written into the mesh and the pads
/// placed on the straight line the trunk used to be on, so every pad floated
/// off the side of the tree it belonged to.
pub fn trunk_at(tree: &Tree, on: f32) -> Vec3 {
    let out = tree.leans * tree.tall * LEANS * bend(on);

    tree.at + vec3(out.x, tree.tall * on.clamp(0.0, 1.0), out.y)
}

/// How thick a trunk is a given way up itself.
///
/// Thinner than it was by a fifth. The old number was never measured against
/// anything: the collider round a tree was a box 0.34 across and the trunk it
/// stood for was 0.42 at the foot, so the tree you walked round was narrower
/// than the tree you could see. Taking the collider off the trunk's own
/// thickness made the difference show up as three trees too close to the
/// walls, and the trunk was the thing that was wrong.
pub fn trunk_wide(on: f32) -> f32 {
    0.26 - on * 0.14 + (1.0 - on).powi(4) * 0.08
}

/// The trunk, bent along its own lean.
///
/// Built per tree rather than once and scaled, because the lean is a share of
/// the height and the thickness is not: one mesh scaled to two heights is a
/// sapling and a log.
pub fn trunk_mesh(tree: &Tree) -> blitzkit::mesh::MeshData {
    use std::f32::consts::TAU;

    let mut mesh = blitzkit::mesh::MeshData::surface(10, 14, |u, v| {
        let round = -u * TAU;
        let wide = trunk_wide(v) * 0.5;
        let on = trunk_at(tree, v) - tree.at;

        vec3(round.cos() * wide + on.x, on.y, round.sin() * wide + on.z)
    });
    mesh.compute_normals();

    mesh
}

/// The box a trunk fills, which is what you walk round.
///
/// Up to head height and no further. A leaning trunk is somewhere else at the
/// top than at the foot, and a box at the foot lets you walk through the part
/// of it that is actually in front of your face; a box round the whole lean is
/// a tree you cannot get near on the side it leans away from.
pub fn trunk_box(tree: &Tree) -> Aabb {
    let head = (1.8f32 / tree.tall).min(1.0);
    let top = trunk_at(tree, head);
    let foot = trunk_wide(0.0);

    Aabb::from_center_size(
        vec3(
            (tree.at.x + top.x) * 0.5,
            tree.tall * 0.5,
            (tree.at.z + top.z) * 0.5,
        ),
        vec3(
            foot + (top.x - tree.at.x).abs(),
            tree.tall,
            foot + (top.z - tree.at.z).abs(),
        ),
    )
}

/// Where a tree's pads of foliage sit: how far up the trunk, how far out from
/// it, how far round, and how wide.
///
/// Pads and not plates. Three flat discs threaded on the trunk is a fir and a
/// tree in a garden like this is pruned the other way about: the foliage is
/// cleared off the limbs except at their ends, so what is left is a handful of
/// clouds at different heights with sky between them. The gaps are the point.
///
/// Five, going round as they go up, each a different size, and the last one on
/// the trunk's own line because the top of a pruned pine is its apex.
const PADS: [(f32, f32, f32, f32); 5] = [
    (0.45, 0.44, 0.00, 0.62),
    (0.58, 0.38, 0.42, 0.50),
    (0.70, 0.33, 0.80, 0.47),
    (0.83, 0.28, 0.24, 0.37),
    (0.98, 0.05, 0.62, 0.29),
];

/// One pad: where it sits, how far across and how far round.
pub struct Pad {
    pub at: Vec3,
    pub wide: f32,
    pub turn: f32,
}

/// The pads of one tree.
pub fn pads(tree: &Tree) -> Vec<Pad> {
    let about = (tree.seed % 360) as f32 / 360.0;

    PADS.iter()
        .enumerate()
        .map(|(n, (up, out, round, wide))| {
            // turned round by the tree's own amount, so three trees built from
            // one list are not three of the same tree
            let round = (round + about) * std::f32::consts::TAU;
            let along = vec2(round.cos(), round.sin()) * out * tree.wide;
            let on = trunk_at(tree, *up);

            Pad {
                at: vec3(on.x + along.x, on.y, on.z + along.y),
                wide: wide * tree.wide,
                turn: round + n as f32 * 0.9,
            }
        })
        .collect()
}

/// The limb under each pad: where it leaves the trunk and where it ends.
///
/// A pad with no limb is a cloud. The limb leaves the trunk a little below the
/// pad, because a branch goes out and up rather than straight out.
pub fn limbs(tree: &Tree) -> Vec<(Vec3, Vec3)> {
    pads(tree)
        .into_iter()
        .map(|pad| {
            let on = ((pad.at.y / tree.tall) - LIMB_UNDER).max(0.0);

            (trunk_at(tree, on), pad.at)
        })
        .collect()
}

/// How far below its pad a limb leaves the trunk, as a share of the height,
/// and how thick a limb is against the pad it carries.
pub const LIMB_UNDER: f32 = 0.12;
pub const LIMB_THICK: f32 = 0.055;

/// How flat a pad is against how wide, and how far its edge wanders.
///
/// Flat, because a pruned pad is a plate of needles held out level, and round
/// it is a bush. Wandering, and wandering a good deal: a smooth one is a
/// pebble, and worse than that, a squashed sphere carries the one highlight it
/// is given all the way round its rim as a wet green streak. What breaks that
/// up is the same thing that says foliage, which is a surface that is not
/// going anywhere smoothly.
pub const PAD_FLAT: f32 = 0.33;
pub const PAD_WANDER: f32 = 0.31;

/// One pad of foliage, lumpy and smooth.
///
/// Smooth and not faceted, which is the opposite of the stones. A faceted pad
/// is cut glass; what reads as a mass of needles is a surface with no edges in
/// it at all.
pub fn pad_mesh(seed: u32) -> blitzkit::mesh::MeshData {
    use std::f32::consts::{PI, TAU};

    let mut mesh = blitzkit::mesh::MeshData::surface(16, 9, |u, v| {
        let round = -u * TAU;
        let up = v * PI;
        let out = (0.5 + wandered(seed, u, v, PAD_WANDER)) * up.sin();

        vec3(round.cos() * out, -up.cos() * 0.5, round.sin() * out)
    });
    mesh.compute_normals();

    mesh
}

/// A limb: a taper from the trunk to the pad, built along y.
pub fn limb_mesh() -> blitzkit::mesh::MeshData {
    crate::cellar::turned(7, 4, |v| (v, 1.0 - v * 0.45))
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

    /// Spec 0010: the path goes from the way in to the water.
    ///
    /// Both ends, because a path that starts in the open is a row of stones and
    /// one that stops short of the water goes nowhere.
    #[test]
    fn the_path_runs_from_the_door_to_the_water() {
        // at several sizes of room, because both ends of this moved with the
        // cabinet count the first time and only one of them was derived
        for count in [6usize, 9, 12, 14, 17] {
            let reaches = crate::room::Room::of(
                (0..count)
                    .map(|n| {
                        crate::cabinet::Cabinet::found(
                            &format!("game{}", n),
                            std::path::Path::new("/nowhere"),
                        )
                    })
                    .collect(),
            )
            .reaches;

            the_path_at(reaches);
        }
    }

    fn the_path_at(reaches: f32) {
        let laid = steps(reaches);
        let (water, size, _) = pond(reaches);
        let (from_z, to_z) = way(reaches);

        assert!(laid.len() > 4, "a path of {} stones", laid.len());

        let first = laid.first().unwrap().at;
        let last = laid.last().unwrap().at;

        // the first one is inside the opening, across its width and a stride
        // or two in from the wall
        assert!(
            first.z > from_z - 0.6 && first.z < to_z + 0.6,
            "the path starts off to one side of the way in: {:?}",
            first
        );
        assert!(
            (first.x - wall_at()).abs() < 2.4,
            "the path starts {:.2} from the wall, which is out in the open",
            (first.x - wall_at()).abs()
        );

        // and the last is at the kerb, not short of it and not in the water
        let off = vec2(last.x - water.x, last.z - water.z).abs();
        let stone = size * 0.5 + Vec2::splat(KERB);

        assert!(
            off.x > stone.x || off.y > stone.y,
            "the path's last stone is in the pond at {:?}",
            last
        );
        assert!(
            (off.x - stone.x).min(off.y - stone.y) < STRIDE * 2.0,
            "the path stops {:.2} short of the water",
            (off.x - stone.x).min(off.y - stone.y)
        );
    }

    /// Spec 0010: every step is a stride, and the path is not a straight line.
    ///
    /// Walked rather than divided, so the gaps have to come out even round the
    /// bend as well as on the straights. Dividing the curve into equal pieces
    /// of its own parameter is the easy way and it bunches the stones up where
    /// it turns.
    #[test]
    fn every_step_on_the_path_is_a_stride() {
        let laid = steps(room().reaches);

        for pair in laid.windows(2) {
            let apart = vec2(pair[1].at.x - pair[0].at.x, pair[1].at.z - pair[0].at.z).length();

            assert!(
                (apart - STRIDE).abs() < THROWN * 2.2 + 0.03,
                "two stones are {:.3} apart and a stride is {:.3}",
                apart,
                STRIDE
            );
        }

        // and the bow is real: the middle of the path is off the line between
        // its ends
        let reaches = room().reaches;
        let middle = path_at(reaches, 0.5);
        let line = (path_from(reaches) + path_to(reaches)) * 0.5;

        assert!(
            middle.distance(line) > PATH_BOW * 0.8,
            "the path is {:.2} off straight, which is straight",
            middle.distance(line)
        );
    }

    /// Spec 0010: nothing of the path lands on anything else.
    ///
    /// The stones are not in `solid`, since a thing you step on is not a thing
    /// you walk round, which means nothing at run time will stop one being laid
    /// through a boulder or over the kerb.
    #[test]
    fn no_stepping_stone_lands_on_anything() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();

        for step in steps(reaches) {
            let here = vec2(step.at.x, step.at.z);

            for stone in rocks(reaches) {
                let apart = here.distance(vec2(stone.at.x, stone.at.z));
                let want = (step.wide + stone.size.x.max(stone.size.z)) * 0.5;

                assert!(
                    apart > want,
                    "a stepping stone is laid against a set stone: {:.2} apart, {:.2} wanted",
                    apart,
                    want
                );
            }

            for (where_, tall) in lanterns(reaches) {
                let apart = here.distance(vec2(where_.x, where_.z));

                assert!(
                    apart > step.wide * 0.5 + 0.25 + tall * 0.0,
                    "a stepping stone is under a lantern at {:?}",
                    where_
                );
            }

            for tree in trees(reaches) {
                let apart = here.distance(vec2(tree.at.x, tree.at.z));

                assert!(
                    apart > step.wide * 0.5 + 0.17,
                    "a stepping stone is under a tree at {:?}",
                    tree.at
                );
            }

            // and on the gravel, not through a wall
            assert!(
                (here.x - middle.x).abs() + step.wide * 0.5 < half.x
                    && (here.y - middle.z).abs() + step.wide * 0.5 < half.y,
                "a stepping stone is through a wall at {:?}",
                here
            );
        }
    }

    #[test]
    fn probe3() {
        let reaches = room().reaches;
        let (water, size, _) = pond(reaches);
        let half = size * 0.5;
        let angle = 3.93f32;
        let out = vec2(angle.cos(), angle.sin());
        let want = half + Vec2::splat(KERB + crate::RADIUS + 1.2);
        let gone = (want.x / out.x.abs()).min(want.y / out.y.abs());
        let from = vec3(water.x + out.x * gone, 0.0, water.z + out.y * gone);
        let to = vec3(water.x, 0.0, water.z);
        let solid = room().solid();
        let step = 1.0 / 60.0;
        let mut at = from;
        let mut falling = 0.0;
        println!("from {:.3} {:.3} {:.3}", from.x, from.y, from.z);
        let mut was = 0.0f32;
        for n in 0..180 {
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
            if (at.y - was).abs() > 1e-3 {
                println!(
                    "{n:3} {:.3} {:.3} {:.3}  rose {:.3}",
                    at.x,
                    at.y,
                    at.z,
                    at.y - was
                );
                was = at.y;
            }
        }
        println!("end {:.3} {:.3} {:.3}", at.x, at.y, at.z);
        println!(
            "bank outer corner at {:.3},{:.3}",
            water.x - half.x - KERB,
            water.z - half.y - KERB
        );
    }

    /// Spec 0010: you cannot walk into the pond.
    ///
    /// Walked and not measured. The lip used to be one run of coping, and the
    /// moment it became stones with gaps between them the question of whether
    /// you can get through a gap stopped being answerable by looking at the
    /// numbers: a body is round, a gap is a slot between two boxes set at
    /// angles to each other, and sliding is what `move_and_slide` is for.
    #[test]
    fn you_cannot_walk_into_the_pond() {
        let reaches = room().reaches;
        let (water, size, _) = pond(reaches);
        let half = size * 0.5;

        for turn in 0..16 {
            let angle = turn as f32 / 16.0 * std::f32::consts::TAU;
            let out = vec2(angle.cos(), angle.sin());
            // from outside the walk round the pond, aimed at the middle of the
            // water, with long enough to get there twice over
            // out to where the ray leaves the stone, not a fixed amount on
            // each axis. A fixed amount is a fixed amount along x and along z
            // and so a smaller one on the diagonal: at a corner the body
            // started a finger's width outside the bank with its own radius
            // already inside it, and what the walk then measured was how this
            // engine pushes a body out of a box it is spawned in.
            let want = half + Vec2::splat(KERB + crate::RADIUS + 1.2);
            let reach = [
                if out.x.abs() > 1e-3 {
                    want.x / out.x.abs()
                } else {
                    f32::MAX
                },
                if out.y.abs() > 1e-3 {
                    want.y / out.y.abs()
                } else {
                    f32::MAX
                },
            ];
            let gone = reach[0].min(reach[1]);

            // started on the gravel, and not dropped in from above the way
            // the other walks in here are. The drop is there because a point
            // on the floor can be inside the furniture, and it is wrong for
            // this one: `walk` climbs anything within `STEP` of where your
            // feet are, so a body hovering at a third of a unit can step onto
            // a lip it could not reach from the ground. Dropped from above it
            // sailed over the bank and landed on the stones every time, which
            // measures falling and not walking, and nothing in this building
            // puts you in the air.
            let from = vec3(water.x + out.x * gone, 0.0, water.z + out.y * gone);
            let got = walked(from, vec3(water.x, 0.0, water.z), 4.0);
            let off = vec2(got.x - water.x, got.z - water.z).abs();

            assert!(
                off.x > half.x - crate::RADIUS || off.y > half.y - crate::RADIUS,
                "walked into the water from {:.2} radians: ended at {:?}",
                angle,
                got
            );
        }
    }

    /// Spec 0010: the lip is set stones and not a run of coping.
    ///
    /// Which is a thing about sizes rather than about shape. Cut irregular and
    /// all of a height, a kerb is a kerb with a wavy top; what reads as set
    /// stones is that no two are the same and some of them lean out over the
    /// water.
    #[test]
    fn the_lip_is_set_stones_and_not_a_kerb() {
        let reaches = room().reaches;
        let (water, size, _) = pond(reaches);
        let half = size * 0.5;
        let laid = edging(reaches);

        assert!(laid.len() > 18, "a lip of {} stones", laid.len());

        let (mut lowest, mut tallest) = (f32::MAX, 0.0f32);
        let (mut shortest, mut longest) = (f32::MAX, 0.0f32);
        let mut over = 0;

        for edge in laid.iter() {
            lowest = lowest.min(edge.size.y);
            tallest = tallest.max(edge.size.y);
            shortest = shortest.min(edge.size.x);
            longest = longest.max(edge.size.x);

            // how far in it reaches, across the band it is set in
            let off = vec2(edge.at.x - water.x, edge.at.z - water.z).abs();
            let inner = if off.x > off.y {
                off.x - edge.size.z * 0.5 - half.x
            } else {
                off.y - edge.size.z * 0.5 - half.y
            };

            if inner < 0.0 {
                over += 1;
            }
        }

        assert!(
            tallest > lowest * 1.5,
            "every stone is the same height: {:.2} to {:.2}",
            lowest,
            tallest
        );
        assert!(
            longest > shortest * 1.4,
            "every stone is the same length: {:.2} to {:.2}",
            shortest,
            longest
        );
        assert!(
            over > laid.len() / 3,
            "only {} of {} stones lean out over the water",
            over,
            laid.len()
        );
    }

    /// Spec 0010: the garden stands on the roof of the baths, and neither is
    /// inside the other.
    ///
    /// The garden was laid over the baths without either knowing, and it went
    /// wrong in both directions at once. The baths' roof stood a third of a
    /// unit above the garden's gravel, which is a ledge you cannot see, can
    /// walk up, and can then step off into the koi pond. The garden's ground
    /// was two units thick and hung that far down into the far corner of the
    /// baths, where it is a block of nothing at chest height.
    ///
    /// Strictly below and not level with. Two floors whose tops are in one
    /// plane put the lower one's end face in the middle of the upper one's
    /// floor, a body resting on a surface counts as touching it, and the sweep
    /// against that face lets you most of the way through and then jams.
    #[test]
    fn the_garden_sits_on_the_roof_of_the_baths() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let ground = solid(reaches)
            .into_iter()
            .find(|box_| box_.max.y > -1e-4 && box_.max.y < 1e-4 && box_.max.x - box_.min.x > 8.0)
            .expect("the garden has a ground");

        let mut highest = f32::MIN;
        for below in crate::spa::solid(reaches)
            .into_iter()
            .chain(crate::cellar::solid(reaches))
        {
            let apart = (below.min.x > middle.x + half.x)
                || (below.max.x < middle.x - half.x)
                || (below.min.z > middle.z + half.y)
                || (below.max.z < middle.z - half.y);

            if apart {
                continue;
            }

            assert!(
                below.max.y < ground.max.y - 1e-3,
                "something under the garden reaches {:.3}, and the gravel is at {:.3}: \
                 x {:.2}..{:.2} z {:.2}..{:.2}",
                below.max.y,
                ground.max.y,
                below.min.x,
                below.max.x,
                below.min.z,
                below.max.z,
            );

            highest = highest.max(below.max.y);
        }

        assert!(
            highest > f32::MIN,
            "nothing of the building is under the garden at all, which cannot be right",
        );
        assert!(
            ground.min.y >= highest - 1e-3,
            "the garden's ground hangs {:.2} into the room below it",
            highest - ground.min.y,
        );
    }

    /// Spec 0010: every tree leans towards the water, and leans rather than
    /// slopes.
    ///
    /// Which way is not written down anywhere. Written down it is three more
    /// numbers to get wrong, and the one thing these trees are pruned to do is
    /// reach out over the pond.
    #[test]
    fn every_tree_leans_over_the_water() {
        let reaches = room().reaches;
        let (water, _, _) = pond(reaches);

        for tree in trees(reaches) {
            let top = trunk_at(&tree, 1.0);
            let was = vec2(water.x - tree.at.x, water.z - tree.at.z).length();
            let now = vec2(water.x - top.x, water.z - top.z).length();

            assert!(
                now < was - tree.tall * LEANS * 0.5,
                "a tree's top is {:.2} from the water and its foot is {:.2}",
                now,
                was
            );

            // and it bends: the lean is in the top half, so half way up it is
            // nowhere near half way over
            let middle = trunk_at(&tree, 0.5);
            let over = vec2(middle.x - tree.at.x, middle.z - tree.at.z).length();
            let whole = vec2(top.x - tree.at.x, top.z - tree.at.z).length();

            assert!(
                over < whole * 0.4,
                "half way up it is {:.0}% of the way over, which is a mast and not a tree",
                over / whole * 100.0
            );
        }
    }

    /// Spec 0010: the pads go round the trunk and are not three of a size.
    ///
    /// A tree whose foliage is all on one side is a hedge that lost an
    /// argument, and one whose pads are a stack of the same disc is the fir
    /// this was.
    #[test]
    fn a_tree_is_pads_and_not_a_stack_of_plates() {
        let reaches = room().reaches;

        for tree in trees(reaches) {
            let laid = pads(&tree);

            assert!(laid.len() > 3, "a tree of {} pads", laid.len());

            let (mut small, mut big) = (f32::MAX, 0.0f32);
            let (mut low, mut high) = (f32::MAX, 0.0f32);
            let mut round = Vec2::ZERO;

            for pad in laid.iter() {
                small = small.min(pad.wide);
                big = big.max(pad.wide);
                low = low.min(pad.at.y);
                high = high.max(pad.at.y);
                round += vec2(pad.at.x - tree.at.x, pad.at.z - tree.at.z);
            }

            assert!(
                big > small * 1.6,
                "every pad is the same size: {:.2} to {:.2}",
                small,
                big
            );
            assert!(
                high - low > tree.tall * 0.4,
                "the pads are all at one height, over {:.2} of a tree {:.2} tall",
                high - low,
                tree.tall
            );
            // they go round, so what is left of the trunk's own lean when you
            // add them all up is the lean and not a side
            let leaning = round.dot(tree.leans);
            assert!(
                (round - tree.leans * leaning).length() < tree.wide * 0.6,
                "the pads are all on one side of the trunk: {:?}",
                round
            );
        }
    }

    /// Spec 0010: every pad hangs off a limb that starts on the trunk.
    ///
    /// A pad with no limb under it is a cloud, and a limb that starts in thin
    /// air is worse than none. Both ends are taken from `trunk_at`, so this is
    /// the test that they are taken from the same `trunk_at`: the first build
    /// of this had the lean inside the trunk's mesh and the pads placed on the
    /// straight line the trunk used to be on.
    #[test]
    fn every_pad_hangs_off_the_trunk() {
        let reaches = room().reaches;

        for tree in trees(reaches) {
            let laid = pads(&tree);
            let out = limbs(&tree);

            assert_eq!(laid.len(), out.len(), "a limb short");

            for (pad, (from, to)) in laid.iter().zip(out.iter()) {
                assert!(
                    to.distance(pad.at) < 1e-4,
                    "a limb ends {:.3} from its pad",
                    to.distance(pad.at)
                );

                // the foot of the limb is on the trunk's line at that height,
                // within the thickness of the trunk there
                let on = (from.y / tree.tall).clamp(0.0, 1.0);
                let line = trunk_at(&tree, on);

                assert!(
                    vec2(from.x - line.x, from.z - line.z).length() < trunk_wide(on),
                    "a limb starts off the trunk at {:?}",
                    from
                );
                assert!(
                    from.y < pad.at.y - 1e-3,
                    "a limb goes down to its pad rather than up to it",
                );
            }
        }
    }

    /// Spec 0010: the head of the way in is drawn through the wall, not skinned.
    ///
    /// Every other face of this garden is a skin a hair proud of a wall that
    /// is already there. Over the way in there is no wall above the hall's
    /// head to be a skin on, so a skin left the wall's own thickness open
    /// behind it: a hand's breadth of channel, and from the garden, at the
    /// angle you look up at a doorway from, a line of sky through the slot.
    ///
    /// It was four pixels. The metre of open wall under it was found the same
    /// way, by somebody standing in the garden and looking at the door.
    #[test]
    fn the_head_of_the_way_in_is_drawn_through_the_wall() {
        let reaches = room().reaches;
        let (from, to) = way(reaches);
        let (low, high) = parapet();
        let faces = facing(reaches);

        for step in 0..=20 {
            let at_ = vec3(
                wall_at() - THICK * 0.5 + THICK * step as f32 / 20.0,
                (low + high) * 0.5,
                (from + to) * 0.5,
            );

            let drawn = faces.iter().any(|(mid, size)| {
                (mid.x - at_.x).abs() <= size.x * 0.5 + 1e-4
                    && (mid.y - at_.y).abs() <= size.y * 0.5 + 1e-4
                    && (mid.z - at_.z).abs() <= size.z * 0.5 + 1e-4
            });

            assert!(
                drawn,
                "nothing is drawn over the way in at {:?}, so the channel \
                 behind its face is open to the sky",
                at_
            );
        }
    }

    /// Spec 0010: the garden is closed all the way up, on every side.
    ///
    /// Two of its walls are borrowed from rooms a metre shorter than it, and
    /// only one of them was topped. Over the nook's end wall there was a strip
    /// of nothing the whole width of the nook: from in there, looking up at
    /// that end, a wedge of the garden's sky came through the ceiling, and from
    /// the garden you were looking down into the nook.
    ///
    /// Swept along each side rather than checked at a corner, because a gap in
    /// the middle of a wall is what this was.
    #[test]
    fn the_garden_is_closed_all_the_way_up() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let half = half();
        let solid = solid(reaches);
        let (from_z, to_z) = way(reaches);

        // every side, at the height between the rooms' ceilings and the
        // garden's own, which is the band that was open
        for up in [crate::room::TALL + 0.05, (crate::room::TALL + HIGH) * 0.5] {
            for step in 0..=120 {
                let on = step as f32 / 120.0;

                for (what, at_) in [
                    (
                        "the south wall",
                        vec3(middle.x - half.x + on * half.x * 2.0, up, middle.z - half.y),
                    ),
                    (
                        "the north wall",
                        vec3(middle.x - half.x + on * half.x * 2.0, up, middle.z + half.y),
                    ),
                    (
                        "the west wall",
                        vec3(middle.x - half.x, up, middle.z - half.y + on * half.y * 2.0),
                    ),
                    (
                        "the east wall",
                        vec3(middle.x + half.x, up, middle.z - half.y + on * half.y * 2.0),
                    ),
                ] {
                    // the way in is a hole on purpose, and only below the
                    // hall's own ceiling. Above that it is wall like the rest
                    // of the stretch, and skipping it at every height is why
                    // this test watched the parapet drop a metre and said
                    // nothing.
                    let through = what == "the east wall"
                        && at_.z > from_z - 0.2
                        && at_.z < to_z + 0.2
                        && up < crate::room::TALL;

                    if through {
                        continue;
                    }

                    // and within a hand of a box rather than within a fifth
                    // of a metre. At 0.2, with the sweep stepping 0.13, every
                    // sample in the hole over the way in was either inside
                    // that reach of the nook's parapet or inside the margin
                    // above, and a metre of open wall passed.
                    let shut = solid
                        .iter()
                        .any(|box_| box_.contains_point(at_) || near(box_, at_) < 0.06);

                    assert!(shut, "{} is open at {:?}", what, at_);
                }
            }
        }
    }

    /// How far a point is from a box, nought if it is inside.
    fn near(box_: &Aabb, at_: Vec3) -> f32 {
        let on = vec3(
            at_.x.clamp(box_.min.x, box_.max.x),
            at_.y.clamp(box_.min.y, box_.max.y),
            at_.z.clamp(box_.min.z, box_.max.z),
        );

        on.distance(at_)
    }

    /// The sky's picture is not wedged into a point anywhere.
    ///
    /// Forty-nine of the dome's vertices stand at the apex in one place. Laid
    /// on like a map of the world, each of them carried a different place in
    /// the picture, so the whole top edge of it was pulled into that one
    /// point and the sky had a dark star over the middle of the garden.
    ///
    /// Asked of any two vertices that share a place, so it holds the seam as
    /// well: the two edges of the picture used to meet round the back of the
    /// dome, and now they fall on the same texels.
    #[test]
    fn the_sky_has_no_point_where_its_picture_is_wedged() {
        let mesh = dome_mesh();

        for (n, one) in mesh.vertices.iter().enumerate() {
            for other in mesh.vertices.iter().skip(n + 1) {
                let together =
                    (Vec3::from(one.position) - Vec3::from(other.position)).length() < 1e-3;

                if !together {
                    continue;
                }

                let apart = (Vec2::from(one.uv) - Vec2::from(other.uv)).length();
                assert!(
                    apart < 1e-3,
                    "the sky at {:?} is {:?} of its picture and also {:?}, \
                     {:.3} away, so the picture is wedged there",
                    one.position,
                    one.uv,
                    other.uv,
                    apart
                );
            }
        }
    }

    /// The sky is lit the same all the way to its apex.
    ///
    /// Forty-nine of its vertices sit at the apex in one place, so the
    /// triangles between them have no area. Averaged off those faces, every
    /// normal up there came out of rounding, and the one at the seam came out
    /// inverted: a dark spoke running out of the top of the sky, which is
    /// what you look at lying in the garden.
    ///
    /// Checked against the position rather than against the formula the
    /// normals are built from, so this is asking the surface a question and
    /// not reading its answer back.
    #[test]
    fn the_sky_faces_inward_everywhere_including_its_apex() {
        let mesh = dome_mesh();

        for vertex in mesh.vertices.iter() {
            let at = Vec3::from(vertex.position);
            let normal = Vec3::from(vertex.normal);

            // the dome is convex about its own middle, so anything facing in
            // leans against the way out
            assert!(
                normal.dot(at) < 0.0,
                "the sky at {:?} faces {:?}, which is outwards",
                at,
                normal
            );
        }

        // and the apex is one point, so the sky must not change across it
        let apex: Vec<Vec3> = mesh
            .vertices
            .iter()
            .filter(|vertex| vertex.position[1] > DOME_UP - 1e-3)
            .map(|vertex| Vec3::from(vertex.normal))
            .collect();

        assert!(apex.len() > 2, "no apex to speak of");
        for normal in apex.iter() {
            assert!(
                normal.dot(apex[0]) > 0.999,
                "two of the sky's apex face {:?} and {:?}, so it is lit in \
                 wedges",
                normal,
                apex[0]
            );
        }
    }

    /// Spec 0010: no part of the sky is inside the building.
    ///
    /// The dome is nineteen across and the garden is sixteen, so it reaches
    /// well past the garden and over the hall and the nook. Anything of it
    /// below their ceilings is inside those rooms, and sprung at a little over
    /// half the garden's height its rim sat at 2.31 against ceilings at 3.2:
    /// from the nook it was a pale blade hanging down through the ceiling.
    ///
    /// Measured at the rim, which is the lowest the dome ever gets, and
    /// against the ceiling of every room the dome reaches over.
    #[test]
    fn no_part_of_the_sky_is_inside_the_building() {
        let reaches = room().reaches;
        let middle = at(reaches);
        let stands = dome_at(reaches);

        // the rim is the lowest of it, and the whole of the dome is at that
        // height or above
        let mut lowest = f32::MAX;
        for vertex in dome_mesh().vertices.iter() {
            lowest = lowest.min(vertex.position[1]);
        }
        let rim = stands.y + lowest;

        assert!(
            rim > crate::room::TALL,
            "the sky comes down to {:.2} and the rooms it crosses have ceilings at {:.2}",
            rim,
            crate::room::TALL
        );

        // and it reaches over them, which is why that matters
        let over = |x: f32, z: f32| vec2(x - middle.x, z - middle.z).length() < DOME;
        assert!(over(0.0, 0.0), "the dome should reach over the hall");
        assert!(
            over(-crate::room::WALL - CABINET.x - NOOK_DEEP * 0.5, 0.0),
            "the dome should reach over the nook"
        );

        // still tucked under the garden's own walls, which is the older reason
        // it is sprung low at all: level with them and you see the edge of the
        // sky from inside the garden
        assert!(
            rim < HIGH,
            "the rim is at {:.2} and the garden's walls stop at {:.2}",
            rim,
            HIGH
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

            let (next, fell) =
                crate::walk::walk(at, wish, falling, crate::RADIUS, step, &solid, true);
            at = next;
            falling = fell;
        }

        at
    }

    /// Spec 0010: you can get into the garden from where you wake up.
    #[test]
    fn you_can_walk_in_from_where_you_wake() {
        let reaches = room().reaches;
        // dropped in from above rather than placed, because a point on the
        // floor can be inside something
        let wake = vec3(0.0, 1.2, reaches - crate::room::INSIDE);
        // a point on the path, and not the middle of the garden. The middle
        // of the garden is inside the pond: this walked into the water, and it
        // only passed because the lip was low enough to climb.
        //
        // Half way along the path and no further. Getting in is what this is
        // about; getting round the water is `the_walkway_goes_right_round_the_pond`,
        // which walks it in legs the way a person does. Aimed across the whole
        // garden instead, what it measures is whether steering straight at a
        // target can round an obstacle, and it cannot: it caught on the corner
        // stone of the lip and sat there for eight seconds.
        let on = path_at(reaches, 0.45);
        let want = vec3(on.x, 0.0, on.y);

        let got = walked(wake, want, 8.0);

        assert!(
            got.x < wall_at() - THICK,
            "it never got through the wall: {:?}",
            got
        );
        assert!(
            vec2(got.x - want.x, got.z - want.z).length() < crate::RADIUS * 2.0,
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
            .map(|tree| {
                // the trunk's own box, which is wider than the trunk because
                // it leans: taken as the trunk's thickness alone, a tree you
                // can walk through at head height passes this
                let box_ = trunk_box(&tree);

                (
                    "a tree",
                    vec3(
                        (box_.min.x + box_.max.x) * 0.5,
                        0.0,
                        (box_.min.z + box_.max.z) * 0.5,
                    ),
                    (box_.max.x - box_.min.x).max(box_.max.z - box_.min.z),
                )
            })
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
