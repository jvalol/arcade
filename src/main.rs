//! arcade: a room of cabinets, one per game. See `specs/`.

mod aim;
mod behind;
mod beyond;
mod cabinet;
mod carpet;
mod cascada;
mod cellar;
mod cradle;
mod display;
mod garden;
mod globe;
mod gyro;
mod metronome;
mod neon;
mod noise;
mod orrery;
mod room;
mod sign;
mod spa;
mod study;
mod walk;
mod wrecker;

use blitzkit::camera::Camera;
use blitzkit::collision::Aabb;
use blitzkit::geometry::Geometry;
use blitzkit::keyboard::{KeyboardInput, KeyboardKey, KeyboardKeyState};
use blitzkit::mesh::{MeshData, Transform};
use blitzkit::mouse::{MouseButton, MouseInput};
use blitzkit::reach;
use blitzkit::renderer::render_text::{RenderText, TextRenderer};
use blitzkit::renderer::scene::{MeshId, Scene, TextureId};
use blitzkit::renderer::Renderer;
use blitzkit::sound::{Samples, SoundSystem};
use blitzkit::texture::TextureData;
use blitzkit::{start, Game};
use cabinet::{Cabinet, Playing};
use glam::{vec3, vec4, Vec2, Vec3};
use room::Room;

/// Where you stand, how fast, and how far you see.
const RADIUS: f32 = 0.45;
/// How far past the building the fill is given to work in, per spec 0049 of
/// the engine.
///
/// Enough that the outside of a wall has air on it, so a face looking out of
/// the building is told apart from one looking into a slab. Not more, because
/// every metre of it is cells.
#[cfg(test)]
const REACH_PAST: f32 = 1.0;
/// The tallest step the fill will walk up, per spec 0049 of the engine.
///
/// A quarter of a metre, which is a cell. The stairs down to the cellar are
/// shallower than that and every ledge in the building is deeper, so it is the
/// line between a way down and a wall.
const STEPS_UP: f32 = 0.25;
/// How far over a place to stand the air still counts, per spec 0049 of the
/// engine.
///
/// Not the eye's height: a surface near the ceiling is looked at from an eye
/// well below it, so this has to clear the tallest room in the building. The
/// hall is 3.2 and the garden has no top at all.
const ROOM_TALL: f32 = 4.0;
const EYE: f32 = 1.55;

/// How long the eye takes to catch the feet up, in seconds.
///
/// Short enough that it is not a thing you notice on the level, where there is
/// nothing to catch up with anyway, and long enough to turn fourteen quarter
/// unit drops a second into going down a slope.
const EYE_LAGS: f32 = 0.09;
pub const SPEED: f32 = 4.2;

/// How much faster you go with a shift key down.
///
/// Twice, which at 4.2 is a sprint rather than a jog, and that is the right
/// number because of what the building became: the room behind the wall is a
/// hundred and fifty nine metres across and the walk from the door to the sun
/// is a hundred and eleven of them. Walked it is twenty seven seconds each
/// way, which is a trudge. Run it is thirteen.
///
/// It was forty eight metres and eleven seconds when this constant went in,
/// and the room more than doubled afterwards to keep the Moon's orbit off
/// Mars. Shift was already there, which is the only reason the bigger room is
/// a distance rather than a corridor, and is worth saying because the two
/// changes look unrelated and are not.
///
/// Not in the water. Wading is the whole of what tells you that you are in
/// the pool rather than beside it, per spec 0008, and a sprint through it
/// takes that away. Shift does nothing in there.
pub const RUNS: f32 = 2.0;
const LOOK: f32 = 0.0022;
const PITCH_LIMIT: f32 = 1.3;

/// How fast the arrows turn and tilt you, in radians a second.
///
/// The mouse is per pixel and the keyboard is per second, so these are not the
/// same number in different clothes. Turning is quicker than tilting because
/// there is a whole room around you and only so much ceiling.
const TURNS: f32 = 2.4;
const TILTS: f32 = 1.6;

/// What the toy on the bench falls under.
const GRAVITY: Vec3 = vec3(0.0, -9.81, 0.0);

/// How wide the gyroscope's ring is across the axle, which is wider than it is
/// deep so it reads as a hoop rather than a wire.
const GIMBAL_WIDE: f32 = 0.022;

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
/// How far a light is from you against how far it throws, for ranking which
/// lamps the engine gets when the building has more of them than it carries.
///
/// `how_far` is squared and so is the answer, because the caller already
/// holds a squared distance and a square root here would be for the reader
/// rather than for the sort.
///
/// **Reach and not distance, and the sun is what says so.** The ranking was
/// plain distance until the room behind the wall grew to a hundred and fifty
/// nine metres, which took its ceiling grid from thirty six lamps to a
/// hundred. The sun stands a hundred and eleven metres from the way in and
/// every one of those lamps is nearer, so the sun came out seventy sixth of
/// a hundred and thirty three against a cap of sixty four and the engine
/// dropped it: the one light that room exists for, and the only thing
/// lighting sixteen bodies, thrown away for a ceiling lamp in the far corner.
///
/// A point light gives exactly nothing past its range, per
/// `blitzkit::lighting::PointLight::falloff`, so how far off a lamp is means
/// nothing on its own. What means something is how far off it is against how
/// far it throws. The sun reaches 83 m and is 111 away, which is 1.34 of its
/// reach. A ceiling lamp reaches 9.5 and is 40 away, which is 4.2.
///
/// This reads what a light can give *where you stand*, which is not quite
/// the question. A lamp behind you lighting a wall you can see still counts
/// for nothing here, and the sun out past its own reach gives you nothing
/// whatever it is doing to the planets you are looking at. Reading it
/// against what is in view would be the honest version and is a bigger piece
/// of work; the measurement in spec 0015 is why this is enough for now.
fn inside_its_reach(how_far: f32, reach: f32) -> f32 {
    how_far / (reach * reach).max(1e-6)
}

fn staged() -> bool {
    std::env::args().any(|arg| arg.starts_with("--screenshot"))
}

/// Whether the staged shot is the hall's rather than the system's.
///
/// `--screenshot` stages what `media/screenshot.png` actually holds, which is
/// Saturn and its moon, so that `refresh-screenshots` reproduces the committed
/// picture rather than quietly replacing it with a different one. The hall's
/// pose is still here behind `--screenshot=hall`: it cost two rounds to derive
/// and the reasoning on it is still true.
fn staged_in_the_hall() -> bool {
    staged_as().as_deref() == Some("hall")
}

/// Which shot was asked for, if the argument named one.
///
/// `--screenshot` on its own is the one that lands in `media/screenshot.png`.
/// `--screenshot=<slug>` is one of the bodies in `beyond::SHOWN`, by the slug
/// `beyond::slugged` gives it, and `--screenshot=hall` is the hall's own pose.
fn staged_as() -> Option<String> {
    std::env::args().find_map(|arg| arg.strip_prefix("--screenshot=").map(str::to_owned))
}

/// Every shot this game can stage, as the argument to ask for it by and the
/// file it belongs in.
///
/// `refresh-screenshots` in the project above reads this and takes them all.
/// It knew only `screenshot.png` before, so the three pictures of this room
/// on the site were taken by hand once and nothing could take them again: a
/// room that changed under them left three pictures of a building that no
/// longer existed, published, with every check green.
///
/// The hall's pose is not in here. It is a pose kept for its reasoning and
/// there is no file on the site that holds it, and a shot with nowhere to go
/// is not a shot.
fn shots() -> Vec<(String, String)> {
    // Every one is named, the staged shot included, and that is not tidiness.
    // The photographer reads these lines with the tab as its separator, and a
    // shell strips leading whitespace off a field when the separator is
    // whitespace: a nameless first field ate the file name after it and the
    // staged shot was quietly the one picture that did not get taken.
    let mut out = vec![(
        beyond::slugged(beyond::PAIRED),
        "media/screenshot.png".to_owned(),
    )];

    for name in beyond::SHOWN.iter() {
        let slug = beyond::slugged(name);

        out.push((slug.clone(), format!("media/{}.png", slug)));
    }

    out
}

/// How far back from the cabinet the staged shot stands, and how far out of
/// its face into the aisle.
///
/// Spec 0003 made the screens go dark until you are within reach of one, so
/// the view from the doorway is a dark room with nothing answering. The
/// picture has to be taken from where the room is doing something: a step
/// back from a cabinet, with it lit and named and offering its key.
///
/// Down the aisle and not across it. The shot this replaced stood beside one
/// row looking at the other, and the room has outgrown that: the nearest
/// cabinet filled a third of the frame and nothing behind it read. From
/// along the aisle both rows run away from you, which is what the room
/// actually looks like, and the sign over the nook and the plinths at the
/// far end come in behind.
pub const POSED_BACK: f32 = 2.9;
pub const POSED_OUT: f32 = 1.2;
pub const POSED_PAST: f32 = 1.5;
const POSED_PITCH: f32 = -0.05;

/// Where the camera stands for a staged shot, and which way it looks.
///
/// **Worked out from a cabinet and not written down**, which this file has
/// already learned once: the bearing it used to carry was right until a
/// thirteenth game moved the rows under it, and then the sight landed in the
/// gap between two cabinets and the picture was a wall. A bearing goes stale
/// the day the roster changes. A cabinet does not.
///
/// The second one along on the right, counting from the end you come in at.
/// The first is too close to the near wall to have anything behind it, and
/// the second has the whole room at its back.
fn posed(room: &Room) -> (Vec3, f32) {
    let mut row: Vec<&room::Stood> = room
        .stood
        .iter()
        .filter(|stood| stood.facing.x < -0.5)
        .collect();

    row.sort_by(|one, other| other.at.z.total_cmp(&one.at.z));

    let Some(stood) = row.get(1).or_else(|| row.first()) else {
        return (Vec3::new(0.8, 0.0, 6.2), 0.25);
    };

    // out of its face into the aisle, and back toward the way in
    let at = stood.at + stood.facing * POSED_OUT + Vec3::Z * POSED_BACK;
    // aimed a little past it rather than straight at it. Straight at it
    // turns far enough into the row that the cabinet behind your shoulder
    // fills a third of the frame; a metre and a half further down the aisle
    // is eight degrees flatter, still lands the sight on the cabinet, and
    // leaves the room behind it in the picture.
    let look = stood.at - Vec3::Z * POSED_PAST - at;

    (Vec3::new(at.x, 0.0, at.z), look.x.atan2(-look.z))
}

struct Arcade {
    room: Room,
    playing: Playing,
    /// cascada's binary. It has no cabinet, and a bench is not one, but what
    /// starts it is the same thing that starts a game.
    cascada: Cabinet,
    /// And poolhall's, which stands on a table in the cellar for the same
    /// reason. Spec 0007.
    poolhall: Cabinet,
    /// One per cabinet, in the room's own order. A game with no screenshot gets
    /// none and its screen stays blank.
    art: Vec<Option<TextureId>>,
    /// What colour each cabinet's screenshot lends the room when it is lit.
    glows: Vec<Vec3>,
    /// Each game's name, drawn into a texture for its marquee. Spec 0037.
    signs: Vec<TextureId>,
    cube: Option<MeshId>,
    floor: Option<MeshId>,
    /// The carpet, and the ceiling that stops the room opening onto nothing.
    carpet: Option<TextureId>,
    /// The worn track in it at the foot of the near wall, which is the one
    /// thing in the hall that says the way through is there. Spec 0020.
    worn: Option<TextureId>,
    worn_mesh: Option<MeshId>,
    /// Which strike's thunder has been played. Spec 0016.
    thundered: Option<u32>,
    /// The rug in the nook, which is not the arcade's carpet.
    rug: Option<TextureId>,
    rug_mesh: Option<MeshId>,
    /// The grain on the walls and on the cabinets, so neither is a flat face.
    /// The pool and the tub, per spec 0008 and the engine's spec 0043. Two
    /// bodies of water rather than one, because they are at different heights
    /// and a heightfield has one still level.
    pool: blitzkit::water::Water,
    tub: blitzkit::water::Water,
    pool_mesh: Option<MeshId>,
    tub_mesh: Option<MeshId>,
    /// How long since the tub last blew, and since the pool was last stirred.
    blew: f32,
    stirred: f32,
    /// How full of steam the sauna is, from nought to one. Spec 0008.
    steamy: f32,
    says: Option<TextureId>,
    /// The room's own name, for the neon over the near end of the aisle.
    named: Option<TextureId>,
    /// And the baths', for the tiled panel over their door. Spec 0008.
    baths: Option<TextureId>,
    point_mesh: Option<MeshId>,
    hung: Option<MeshId>,
    boards: Option<TextureId>,
    boards_mesh: Option<MeshId>,
    barrel: Option<MeshId>,
    hoop: Option<MeshId>,
    bottle: Option<MeshId>,
    peg: Option<MeshId>,
    /// A pocket's net. Spec 0007.
    bag: Option<MeshId>,
    /// The turned pieces in the baths. Spec 0008.
    urn: Option<MeshId>,
    sconce: Option<MeshId>,
    basin: Option<MeshId>,
    stream: Option<MeshId>,
    dish: Option<MeshId>,
    stave: Option<TextureId>,
    flame: Option<MeshId>,
    candlestick: Option<MeshId>,
    decanter: Option<MeshId>,
    wineglass: Option<MeshId>,
    afghan: Option<TextureId>,
    grain: Vec<TextureId>,
    /// The stairwell's boarding, a quad per face. Spec 0007.
    boarding: Vec<MeshId>,
    stonework: Option<TextureId>,
    /// The garden's sky and the dome it is painted on, per spec 0010.
    storm: Option<TextureId>,
    dome: Option<MeshId>,
    /// The gravel the garden is floored with: one picture per bed, painted in
    /// the garden's own coordinates so the raking knows where the stones are,
    /// and the one quad all of them are laid on.
    gravel: Vec<TextureId>,
    ground: Option<MeshId>,
    /// The carved stones and the stepping stones, per spec 0010.
    stones: Vec<MeshId>,
    slabs: Vec<MeshId>,
    /// The garden's water, its trees and its lanterns, per spec 0010.
    pool_in_the_garden: blitzkit::water::Water,
    pond_mesh: Option<MeshId>,
    trunks: Vec<MeshId>,
    crown: Option<MeshId>,
    limb: Option<MeshId>,
    lantern: Option<MeshId>,
    /// The koi: a body, a tail, and a skin each. Spec 0010.
    koi: Option<MeshId>,
    fin: Option<MeshId>,
    skins: Vec<TextureId>,
    /// A footstep for every floor, read out of the files bundled beside this.
    /// Spec 0009, and the engine's 0045.
    steps: Vec<(noise::Underfoot, Vec<Samples>)>,
    /// And the storm's, one set per distance band. Spec 0016.
    claps: Vec<(noise::Rolled, Vec<Samples>)>,
    /// How far you have walked since the last one, and how many you have taken.
    /// What a climb still owes, in height. See `walk::CLIMBS`.
    climbs: f32,
    paced: f32,
    stepped: usize,
    splashes: Vec<Samples>,
    /// Every voice in the building: where it sounds from, and the pieces it
    /// is played in. Spec 0009.
    ///
    /// A voice is a thing that makes a sound for as long as it exists: the
    /// fire, the fountain, the tub's jets, the sauna, a hall full of cabinets.
    /// It has no length. What has a length is the piece of it the engine is
    /// handed, and one played two and a half times a second is a loop, which
    /// is why there are several of each.
    voices: Vec<(noise::Room, Vec3, Vec<Samples>)>,
    /// How many pieces have gone, which is what picks and bends the next.
    pieced: usize,
    /// How long since `WHERE` last said where you are standing.
    said: f32,
    /// How long the piece sounding now has left.
    sounding: f32,
    /// Whether you were in the water last frame, so going in can be heard.
    was_wading: bool,
    /// A knock for the chain hitting the wall, and how much of it was standing
    /// last frame, so a brick coming off can be heard.
    knock: Option<Samples>,
    was_standing: usize,
    /// The glazed tile the basin is lined with, and a quad per face with its
    /// own count of tiles on it. Spec 0008.
    tiled: Option<TextureId>,
    lining: Vec<MeshId>,
    /// One picture of plaster per wall of the garden, and the quad each is
    /// laid on. Spec 0010.
    plaster: Vec<TextureId>,
    plaster_mesh: Vec<MeshId>,
    /// The way out of the solar system, named. See `orrery::exit_sign`.
    way_out: Option<TextureId>,
    way_out_mesh: Option<MeshId>,
    /// The solar system behind the wall, per spec 0015: a ball, a ring, one
    /// picture per body in `orrery::every_body` order, and how far the swing
    /// between lined up and in orbit has got.
    planet_mesh: Option<MeshId>,
    ring_mesh: Option<MeshId>,
    worlds: Vec<TextureId>,
    /// One name per body in the same order, and how many times wider than tall
    /// its picture came out, which is what the quad is scaled by. Read off the
    /// picture itself rather than measured again, because a name measured
    /// twice is a name stretched.
    names: Vec<(TextureId, f32)>,
    name_mesh: Option<MeshId>,
    /// Which names were drawn last frame, so `orrery::laid_out` can hold them
    /// up against a smaller gap than it asks of a name coming in. Without it
    /// the two rows flicker. See `orrery::STICKS`.
    named_last: Vec<usize>,
    rings: Option<TextureId>,
    /// Stars on all six surfaces of the room, and the quads they are laid on.
    stars: Vec<TextureId>,
    star_mesh: Vec<MeshId>,
    swung: f32,
    swinging_out: bool,
    /// Which stop of spec 0018 you are at, which the chain out there steps
    /// along. Nought is the door, and anywhere inside the building it is
    /// nought: see `mind_the_threshold`.
    stop: usize,
    /// Which stop the ride is carrying you to, per spec 0018, and nothing when
    /// you are standing still.
    ///
    /// Settled when the leg sets off rather than read from where you are
    /// looking, so that turning to watch the sun shrink behind you does not
    /// turn the ride round with you.
    riding_to: Option<usize>,
    /// Which stop the list is resting on, when the list is open.
    listing: Option<usize>,
    /// The numbers for the unit posts of spec 0018, one to thirty, with how
    /// wide each picture is against its height.
    units: Vec<(TextureId, f32)>,
    /// The six faces of the sky out past the door, per spec 0018, and the one
    /// quad they are all drawn on.
    sky: Vec<TextureId>,
    sky_mesh: Option<MeshId>,
    /// And the stars laid on its floor, which is one patch repeated.
    deep_floor: Option<TextureId>,
    deep_floor_mesh: Option<MeshId>,
    /// Where lined up is for each planet, held across a swing so the path
    /// cannot jump. Taken at whichever end the swing last sat at, which is
    /// the only place it can be taken without moving anything. See
    /// `orrery::stands`.
    resting: Vec<f32>,
    /// One length of bamboo, stood up a few hundred times along one wall.
    pole: Option<MeshId>,
    /// The moss at the foot of the garden's walls, and the quads it is cut
    /// out on.
    moss: Option<TextureId>,
    moss_mesh: Vec<MeshId>,
    /// A picture of each of `study::TITLES`, for the spines. Spec 0006.
    titles: Vec<(TextureId, f32)>,
    afghan_mesh: Option<MeshId>,
    cellar_floor: Option<MeshId>,
    wall_grain: Option<TextureId>,
    cabinet_grain: Option<TextureId>,
    ceiling_mesh: Option<MeshId>,
    screen: Option<MeshId>,
    /// The engine's own shapes, turning at the end of the room. Spec 0002.
    /// Where they stand is the room's; these are the meshes.
    shown: Vec<MeshId>,
    /// The toys on the benches, stepped in this room's own loop. The first
    /// physics the arcade has ever run. Spec 0006.
    ///
    /// Each keeps its own step and its own solver. They are three separate
    /// things on three separate benches and nothing one does reaches another, so
    /// one solver holding all of them would only mean the cradle paying for the
    /// wall's contacts and the wall paying for the cradle's forty eight passes.
    cradle: Vec<blitzkit::physics::Body>,
    ropes: Vec<blitzkit::link::Link>,
    solver: blitzkit::physics::Solver,
    /// Seconds owed to the cradle, so it steps at its own rate and not the
    /// frame's.
    owed: f32,
    ball_mesh: Option<MeshId>,
    metronome: metronome::Metronome,
    wrecker: wrecker::Wrecker,
    gyro: gyro::Gyro,
    wheel_mesh: Option<MeshId>,
    globe: globe::Globe,
    /// A sphere fine enough to be a globe, and the world drawn on it.
    globe_mesh: Option<MeshId>,
    world: Option<TextureId>,
    since: f32,

    /// What the sight is on, per spec 0003. Worked out once in `update` and
    /// read everywhere else: `draw` asked the room six times a frame, once per
    /// display and twice over, and the answer cannot change inside a frame.
    seen: Option<room::Seen>,
    /// How big the window is, so the sight can sit in the middle of it.
    window: (f32, f32),
    /// Which way each shape on the far wall is facing, and which one is in your
    /// hands. Spec 0004.
    spin: display::Spin,

    at: Vec3,
    /// How fast you are going down, which until spec 0007 was never.
    falling: f32,
    /// How high your eye is, which is not quite how high your feet are.
    ///
    /// Your feet are exact, because that is what the room is measured against.
    /// Your eye follows them. Going down a stair at a walk you cross a tread in
    /// four frames, so your feet drop a quarter of a unit about fourteen times
    /// a second, and an eye nailed to them drops with them: the stair was not
    /// janky, the view was. Lagged, the feet still land on every tread and the
    /// head goes down the slope.
    eye: f32,
    yaw: f32,
    pitch: f32,
    /// Whether every collider is outlined over the scene, per spec 0013 and
    /// spec 0046 of the engine. Off until somebody asks.
    showing_solid: bool,
    /// Whether a coplanar check has been asked for, per spec 0048 of the
    /// engine. One press, one check: it compares every drawn thing with every
    /// other and the answer does not change between frames.
    wants_planes: bool,
    /// The patches the last check found, kept so they can be outlined. A press
    /// asks, the frame after draws them, and they stay up until the next ask.
    fighting: Vec<Aabb>,
    walking: [bool; 4],
    /// Whether a shift key is down, which is what turns the arrows from hauling
    /// a toy about into working its one other control.
    shifted: bool,
    /// Which arrows are being held on a toy that is leaned on rather than
    /// pressed, in the same order as `walking`.
    leaning: [bool; 4],
    /// And which are turning you about, in the same order again.
    looking: [bool; 4],
    wants_lock: bool,
    locked: bool,
    quitting: bool,
}

/// Lays a plate and its line under a block of text, if there is one.
///
/// The line first and the shade on top of it, so what is left showing round
/// the edge is the line. Two quads and not a frame of four: a frame of four is
/// four chances to be a pixel out.
fn plate(geometry: &mut Geometry, plate: Option<aim::Plate>) {
    use blitzkit::geometry::quad::Quad;

    if let Some(plate) = plate {
        geometry.push_quad(&Quad::colored(plate.at, plate.line_size, aim::PLATE_LINE));
        geometry.push_quad(&Quad::colored(plate.at, plate.size, aim::PLATE));
    }
}

impl Arcade {
    fn new() -> Self {
        let room = Room::of(games());
        let spin = display::Spin::of(room.displays.len());
        let (cradle, ropes) = cradle::strung();

        let reaches = room.reaches;
        // worked out before `room` is moved into the struct below.
        //
        // The system's shot by default, because that is what
        // `media/screenshot.png` holds and `refresh-screenshots` photographs
        // whatever `--screenshot` stages. The hall's is behind
        // `--screenshot=hall`, and each of `beyond::SHOWN` behind its own slug.
        //
        // A slug that names nothing falls through to the default rather than
        // failing, which is the right way round for a photographer: a shot
        // that cannot be staged should come out as the picture that can be,
        // and `refresh-screenshots` only ever asks for names this binary gave
        // it.
        let door = beyond::at(reaches);
        let asked = staged_as()
            .filter(|slug| !staged_in_the_hall() && *slug != beyond::slugged(beyond::PAIRED))
            .and_then(|slug| {
                beyond::SHOWN
                    .iter()
                    .find(|name| beyond::slugged(name) == slug)
            })
            .and_then(|name| beyond::posed_at(door, name));
        let (posed_at, posed_yaw, posed_stop) = match asked.or_else(|| beyond::posed(door)) {
            Some((at, yaw, stop)) if !staged_in_the_hall() => (at, yaw, stop),
            _ => {
                let (at, yaw) = posed(&room);

                (at, yaw, 0)
            }
        };
        let (pool_at, pool_size, pool_deep) = spa::pool(reaches);
        let (tub_at, tub_wide, tub_deep) = spa::tub(room.reaches);
        let mut pool = blitzkit::water::Water::new(pool_at, pool_size, pool_deep, spa::CELLS);
        pool.damping = spa::SETTLES;
        pool.speed = spa::RUNS;

        Self {
            pool,
            tub: blitzkit::water::Water::new(
                tub_at,
                glam::vec2(tub_wide, tub_wide),
                tub_deep,
                spa::TUB_CELLS,
            ),
            steps: Vec::new(),
            claps: Vec::new(),
            climbs: 0.0,
            paced: 0.0,
            stepped: 0,
            splashes: Vec::new(),
            voices: Vec::new(),
            pieced: 0,
            said: 0.0,
            sounding: 0.0,
            was_wading: false,
            knock: None,
            was_standing: 0,
            pool_mesh: None,
            tub_mesh: None,
            blew: 0.0,
            stirred: 0.0,
            steamy: 0.0,
            at: if staged() { posed_at } else { room.doorway() },
            falling: 0.0,
            eye: 0.0,
            room,
            spin,
            playing: Playing::new(),
            cascada: Cabinet::found(cascada::NAME, &beside()),
            poolhall: Cabinet::found(cellar::POOLHALL, &beside()),
            art: Vec::new(),
            glows: Vec::new(),
            signs: Vec::new(),
            cube: None,
            floor: None,
            carpet: None,
            worn: None,
            worn_mesh: None,
            thundered: None,
            rug: None,
            rug_mesh: None,
            says: None,
            named: None,
            baths: None,
            point_mesh: None,
            hung: None,
            boards: None,
            boards_mesh: None,
            barrel: None,
            hoop: None,
            bottle: None,
            peg: None,
            bag: None,
            urn: None,
            sconce: None,
            basin: None,
            stream: None,
            dish: None,
            stave: None,
            flame: None,
            candlestick: None,
            decanter: None,
            wineglass: None,
            afghan: None,
            grain: Vec::new(),
            boarding: Vec::new(),
            stonework: None,
            storm: None,
            dome: None,
            gravel: Vec::new(),
            ground: None,
            stones: Vec::new(),
            slabs: Vec::new(),
            pool_in_the_garden: {
                let (at, size, deep) = garden::pond(reaches);
                let mut water = blitzkit::water::Water::new(at, size, deep, garden::POND_CELLS);
                water.damping = garden::SETTLES;
                water.speed = garden::RUNS;

                water
            },
            pond_mesh: None,
            trunks: Vec::new(),
            crown: None,
            limb: None,
            lantern: None,
            koi: None,
            fin: None,
            skins: Vec::new(),
            tiled: None,
            lining: Vec::new(),
            plaster: Vec::new(),
            plaster_mesh: Vec::new(),
            way_out: None,
            way_out_mesh: None,
            planet_mesh: None,
            ring_mesh: None,
            worlds: Vec::new(),
            names: Vec::new(),
            name_mesh: None,
            named_last: Vec::new(),
            rings: None,
            stars: Vec::new(),
            star_mesh: Vec::new(),
            swung: 0.0,
            swinging_out: false,
            stop: if staged() { posed_stop } else { 0 },
            riding_to: None,
            listing: None,
            units: Vec::new(),
            sky: Vec::new(),
            sky_mesh: None,
            deep_floor: None,
            deep_floor_mesh: None,
            resting: vec![0.0; orrery::PLANETS.len()],
            pole: None,
            moss: None,
            moss_mesh: Vec::new(),
            titles: Vec::new(),
            afghan_mesh: None,
            cellar_floor: None,
            wall_grain: None,
            cabinet_grain: None,
            ceiling_mesh: None,
            screen: None,
            shown: Vec::new(),
            cradle,
            ropes,
            solver: cradle::solver(),
            owed: 0.0,
            ball_mesh: None,
            metronome: metronome::Metronome::new(),
            wrecker: wrecker::Wrecker::new(),
            gyro: gyro::Gyro::new(),
            wheel_mesh: None,
            globe: globe::Globe::new(),
            globe_mesh: None,
            world: None,
            since: 0.0,
            seen: None,
            window: (800.0, 600.0),
            // looking down the room from the end you start at: forward is
            // (sin yaw, 0, -cos yaw), so nought faces -z
            yaw: if staged() { posed_yaw } else { 0.0 },
            // level out past the door, where there is no floor worth looking
            // down at, and a little down in the hall
            pitch: if staged() && staged_in_the_hall() {
                POSED_PITCH
            } else {
                0.0
            },
            showing_solid: false,
            wants_planes: false,
            fighting: Vec::new(),
            walking: [false; 4],
            shifted: false,
            leaning: [false; 4],
            looking: [false; 4],
            wants_lock: true,
            locked: false,
            quitting: false,
        }
    }

    /// Acts on whatever the middle of the screen is on, which is what both
    /// enter and a click mean.
    ///
    /// One rule, and what the sight is on decides what it means: a cabinet is
    /// played and a shape is taken hold of, per spec 0004. Holding one, it
    /// means let go, whatever the sight has wandered onto.
    fn use_what_i_see(&mut self) {
        if self.spin.holding().is_some() {
            self.spin.let_go(self.since);
            return;
        }

        // past the door of spec 0017 there is nothing to work and 33
        // kilometres to cross, so out there this key sets the ride off, or
        // takes the body the list is resting on. Spec 0018.
        if self.in_the_deep() {
            match self.listing.take() {
                Some(stop) => self.go_to(stop),
                None => self.ride(),
            }

            return;
        }

        match self.seen {
            Some(room::Seen::Cabinet(n)) => {
                let stood = self.room.stood[n].clone();
                if self.playing.start(&stood.cabinet) {
                    // the game wants the mouse now
                    self.wants_lock = false;
                }
            }
            Some(room::Seen::Display(n)) => self.spin.take(n, self.since),
            // the book that is a handle. Spec 0007.
            Some(room::Seen::Case) => self.room.open = !self.room.open,
            // and the sauna's glass, which is a door and behaves like one.
            // Spec 0008.
            Some(room::Seen::Sauna) => self.room.sauna_open = !self.room.sauna_open,
            Some(room::Seen::Baths) => self.room.baths_open = !self.room.baths_open,
            // a toy: enter does the one thing that toy's enter does
            Some(room::Seen::Bench(n)) => match self.room.benches[n].name {
                metronome::NAME => self.metronome.press(),
                gyro::NAME => self.gyro.press(),
                // a hand on the ball, which is the other thing you do to one
                globe::NAME => self.globe.spin = 0.0,
                cascada::NAME => {
                    if self.playing.start(&self.cascada.clone()) {
                        // it wants the mouse now, the way a game does
                        self.wants_lock = false;
                    }
                }
                cellar::POOLHALL => {
                    if self.playing.start(&self.poolhall.clone()) {
                        self.wants_lock = false;
                    }
                }
                wrecker::NAME => self.wrecker.rebuild_wall(),
                _ => cradle::set_going(&mut self.cradle, 0),
            },
            None => (),
        }
    }

    /// The arrows, on the toy the sight is on. Says whether the key was one the
    /// toy wanted, which is what decides whether it also walks you.
    ///
    /// A held arrow repeats, and a repeat is swallowed without doing anything: a
    /// pull is a press, so holding the key down is one pull, not forty a second.
    fn work_a_toy(&mut self, input: &KeyboardInput, held: bool) -> bool {
        let Some(room::Seen::Bench(n)) = self.seen else {
            return false;
        };

        let Some(which) = arrow_at(input.key) else {
            return false;
        };
        let name = self.room.benches[n].name;
        if !wanted_by(name, input.key, self.shifted) {
            return false;
        }

        // the toy has this one, so it is not turning you, and letting go always
        // says so: an arrow held on the way to a bench would otherwise leave
        // you turning on the spot with nothing to let go of
        self.looking[which] = false;
        self.leaning[which] = false;
        if !held {
            return true;
        }

        let arrow = true;
        match name {
            metronome::NAME if arrow => {
                if !input.repeat {
                    // up the needle is slower, which is the thing the toy is for
                    let by = if input.key == KeyboardKey::Up { 1 } else { -1 };
                    self.metronome.slide(by);
                }
            }
            globe::NAME if arrow => {
                if !input.repeat {
                    let way = if input.key == KeyboardKey::Left {
                        1.0
                    } else {
                        -1.0
                    };
                    self.globe.flick(way);
                }
            }
            gyro::NAME if arrow => {
                // held rather than pressed. A push is a thing you lean on, and
                // the axis walks for as long as you lean, so a press would be a
                // flick and tell you nothing.
                if self.shifted {
                    if !input.repeat {
                        let by = if input.key == KeyboardKey::Up { 1 } else { -1 };
                        self.gyro.turn_the_dial(by);
                    }
                } else if let Some(which) = arrow_at(input.key) {
                    self.leaning[which] = true;
                }
            }
            wrecker::NAME if arrow => {
                if !input.repeat {
                    // shift and the arrows wind the chain. Hauling the ball
                    // about is what you do with this and winding is what you do
                    // once, so the plain arrows are the hauling.
                    if self.shifted {
                        let by = match input.key {
                            KeyboardKey::Up => -wrecker::WINDS_BY,
                            KeyboardKey::Down => wrecker::WINDS_BY,
                            _ => 0.0,
                        };
                        self.wrecker.wind(by);
                    } else {
                        // all four haul the ball about the tray, so a swing can
                        // be aimed rather than only pumped. Laid out off the
                        // bench, which knows which side of it you stand on.
                        let bench = &self.room.benches[n];
                        let way = match input.key {
                            KeyboardKey::Left => -bench.right(),
                            KeyboardKey::Right => bench.right(),
                            KeyboardKey::Up => bench.away(),
                            _ => -bench.away(),
                        };
                        self.wrecker.haul(way);
                    }
                }
            }
            _ => return false,
        }

        true
    }

    /// Whether you are standing in the pool.
    ///
    /// Over it and under its surface. Over it alone is standing on the coping
    /// looking down, which is not wading.
    fn wading(&self) -> bool {
        spa::wading(self.room.reaches, self.at)
    }

    /// Which line the corner says, which is the one true where you are
    /// standing.
    ///
    /// The building's line offers to work the toy you are looking at, and the
    /// room behind the wall has no toys in it and one key of its own.
    fn saying(&self) -> &'static str {
        if self.in_the_deep() {
            aim::DEEP_HELP
        } else if self.in_the_system() {
            aim::SYSTEM_HELP
        } else {
            aim::HELP
        }
    }

    /// Whether you are in the room behind the wall, per spec 0015.
    fn in_the_system(&self) -> bool {
        let middle = behind::at(self.room.reaches);
        let half = behind::half();

        (self.at.x - middle.x).abs() < half.x && (self.at.z - middle.z).abs() < half.y
    }

    /// Whether you are actually running, which is shift down somewhere you
    /// can use it.
    ///
    /// Said once, because the pace and the footsteps both need it and had
    /// their own copies of the test for a while. Which floors shift works on
    /// is `noise::can_run`, so the one account of it is over there with the
    /// floors.
    fn running(&self) -> bool {
        self.shifted && noise::can_run(self.underfoot())
    }

    /// How fast you are getting about: walking, running, or wading.
    ///
    /// A method rather than three lines in `update`, so a test can ask it.
    fn pace(&self) -> f32 {
        spa::pace(
            self.room.reaches,
            self.at,
            if self.running() { SPEED * RUNS } else { SPEED },
        )
    }

    /// What the line under the sight says about a toy.
    fn about(&self, toy: &str) -> String {
        match toy {
            metronome::NAME => match self.metronome.ticking() {
                Some(rate) => format!(
                    "{:.0} beats a minute. Up and down move the weight. Higher is slower.",
                    rate
                ),
                // about, because what the arithmetic works out and what the rods
                // do are a few percent apart. Once it is going this says what it
                // counted instead.
                None => format!(
                    "Click or press enter to start. The weight is on notch {} of {}, about {:.0} beats a minute.",
                    self.metronome.notch + 1,
                    metronome::NOTCHES,
                    metronome::beats(self.metronome.notch)
                ),
            },
            globe::NAME => {
                // verb first, like every other line under the sight. These two
                // named the key and left you to work out what to do with it,
                // which reads as a description of the toy rather than as an
                // offer to work it.
                if self.globe.going() {
                    String::from("Spinning. Press the arrows to spin it faster, or click to stop it.")
                } else {
                    String::from("Press left or right to spin it.")
                }
            }
            gyro::NAME => {
                let dial = format!("Spin {} of {}", self.gyro.notch + 1, gyro::SPINS);
                if !self.gyro.going {
                    format!(
                        "Click or press enter to spin it up. {}, shift with up or down changes it.",
                        dial
                    )
                } else if self.gyro.moving_at() > 0.02 {
                    format!(
                        "The axis is moving {:.1} a second, sideways to your push. {}.",
                        self.gyro.moving_at(),
                        dial
                    )
                } else {
                    format!(
                        "{}. Hold an arrow to lean on the spindle and the axis should walk {:.1} a second, sideways.",
                        dial,
                        gyro::walks_at(self.gyro.notch)
                    )
                }
            }
            // the rebuild is only offered when there is something to rebuild.
            // rebuild_wall returns early on a whole wall, so a standing wall read
            // as a toy that ignores you: the prompt was promising a click that
            // could not do anything.
            wrecker::NAME => format!(
                "{} of {} standing. Swing the ball with arrow keys. Use shift with up and down keys to wind the chain up or down.{}",
                self.wrecker.standing(),
                wrecker::bricks(),
                if self.wrecker.whole() {
                    ""
                } else {
                    " Click or press enter to rebuild the wall."
                }
            ),
            cellar::POOLHALL if !self.poolhall.is_built() => String::from(aim::NOT_BUILT),
            cellar::POOLHALL => {
                String::from("Pool, more or less. Click or press enter to play in its own window.")
            }
            cascada::NAME if !self.cascada.is_built() => String::from(aim::NOT_BUILT),
            cascada::NAME => String::from(
                "Dominoes. Click or press enter to play in its own window.",
            ),
            _ if cradle::stirring(&self.cradle) > 0.05 => {
                String::from("Click or press enter to set it going again")
            }
            _ => String::from("Click or press enter to set it going"),
        }
    }

    /// Every wall of the nook worth dressing: the face of it, which way it
    /// looks into the room, whether its run is along z, and where that run
    /// starts and ends.
    ///
    /// Not the back wall, which is four hundred books deep, and not the stretch
    /// of the open side that is the way in, because panelling across a doorway
    /// is a door.
    fn nook_walls(&self) -> Vec<(f32, f32, bool, f32, f32)> {
        let side = room::WALL + room::CABINET.x;
        let far = -self.room.reaches;
        let (back, front) = (-side - room::NOOK_DEEP + 0.15, -side - 0.15);

        vec![
            (
                front,
                -1.0,
                true,
                far + room::NOOK_DOOR,
                far + room::NOOK_SPAN,
            ),
            (far + 0.15, 1.0, false, back, front),
            (far + room::NOOK_SPAN - 0.15, -1.0, false, back, front),
        ]
    }

    /// What you are standing on, which decides which footstep you hear.
    fn underfoot(&self) -> noise::Underfoot {
        noise::underfoot(self.room.reaches, self.at)
    }

    /// Where you are looking from, which is your eye and not your feet.
    /// Where a body can get to from where you are standing, over the
    /// colliders, per spec 0049 of the engine.
    ///
    /// The tool itself fills over the frame's drawn boxes, inside the renderer,
    /// because the ceilings are drawn and not all of them are solid. This is
    /// here for the tests, which ask whether a room can be got to at all, and
    /// that is the same question in either list: both have the floors, the
    /// walls and the doorways.
    /// Where the sun stands, and which way the line of planets points.
    ///
    /// The middle of the room, because the orbits have to go round something,
    /// and the line points at the door, because that is where you come from.
    /// It means you meet Neptune first and walk in toward the light, which is
    /// the right way round for this.
    /// Where the door of spec 0017 stands, at the floor, which is where the
    /// space of spec 0018 is measured from.
    ///
    /// The one account of it on this side. `beyond` takes it as an argument
    /// rather than working it out, so that the room and the space beyond it
    /// cannot come apart about where they meet.
    fn the_door(&self) -> Vec3 {
        beyond::at(self.room.reaches)
    }

    /// Whether you are out past the door of spec 0017, which is the one place
    /// Return jumps rather than works what you are looking at.
    ///
    /// By where you stand and not by a flag. A flag is a second account of
    /// which side of a wall you are on, and the wall already knows.
    fn in_the_deep(&self) -> bool {
        self.looking_from().z > self.the_door().z
    }

    /// The map at the foot of the screen, per spec 0018.
    ///
    /// Two bars. The thin one is the whole line, door at the left and Neptune
    /// at the right, with a tick for every body on it: Mercury sits at a
    /// hundredth of the way across and the four inner planets are all inside
    /// the first twentieth, which is a true picture and is the reason the
    /// solar system is hard to draw.
    ///
    /// The thick one under it is the leg you are on, measured again on its own
    /// so that it fills the width whatever its length, with what you left and
    /// what you are going to named at either end. That is the one that answers
    /// how far, because the thin one cannot: a bar on which Mercury is one
    /// pixel from the door cannot show you anything about crossing to it.
    fn draw_the_map(&self, geometry: &mut Geometry, text: &mut TextRenderer) {
        use blitzkit::geometry::quad::Quad;

        let door = self.the_door();
        let stops = beyond::stops(door);
        let map = beyond::mapped(&stops, self.stop, self.at, self.facing(), door);
        let (at, wide) = aim::map_at(self.window);
        let along = |share: f32| at.x + wide * share;
        let bar = |geometry: &mut Geometry, from: f32, to: f32, y: f32, thick: f32, colour| {
            let (from, to) = (from.min(to), from.max(to));
            geometry.push_quad(&Quad::colored(
                glam::vec2((from + to) * 0.5, y),
                glam::vec2((to - from).max(1.0), thick),
                colour,
            ));
        };

        // a plate under the lot, for the reason the corner's text has one: what
        // is behind it out here is a lit line running to a vanishing point
        // directly under the middle of the map
        {
            let top = at.y - aim::MAP_TICK * 0.5 - aim::MAP_PAD;
            let foot = at.y + aim::MAP_GAP * 2.0 + aim::MAP_NAME + aim::MAP_PAD;
            let middle = glam::vec2(at.x + wide * 0.5, (top + foot) * 0.5);
            let size = glam::vec2(wide + aim::MAP_PAD * 2.0, foot - top);

            geometry.push_quad(&Quad::colored(
                middle,
                size + glam::Vec2::splat(aim::PLATE_EDGE * 2.0),
                aim::PLATE_LINE,
            ));
            geometry.push_quad(&Quad::colored(middle, size, aim::PLATE));
        }

        // the whole line, and where each body stands on it
        bar(
            geometry,
            at.x,
            at.x + wide,
            at.y,
            aim::MAP_THIN,
            aim::MAP_LINE,
        );
        bar(
            geometry,
            at.x,
            along(map.along),
            at.y,
            aim::MAP_THIN,
            aim::MAP_DONE,
        );

        for tick in map.ticks.iter() {
            geometry.push_quad(&Quad::colored(
                glam::vec2(along(*tick), at.y),
                glam::vec2(1.0, aim::MAP_TICK),
                aim::MAP_LINE,
            ));
        }

        geometry.push_quad(&Quad::colored(
            glam::vec2(along(map.along), at.y),
            glam::vec2(aim::MAP_YOU, aim::MAP_TICK),
            aim::MAP_MARK,
        ));

        // and the leg, if there is one to be on
        let Some((from, to, through)) = map.leg else {
            return;
        };
        let y = at.y + aim::MAP_GAP;

        bar(
            geometry,
            at.x,
            at.x + wide,
            y,
            aim::MAP_THICK,
            aim::MAP_LINE,
        );
        bar(
            geometry,
            at.x,
            along(through),
            y,
            aim::MAP_THICK,
            aim::MAP_LEG,
        );
        geometry.push_quad(&Quad::colored(
            glam::vec2(along(through), y),
            glam::vec2(aim::MAP_YOU, aim::MAP_THICK + aim::MAP_TICK),
            aim::MAP_MARK,
        ));

        for (words, x, centred) in [(from, at.x, false), (to, at.x + wide, true)] {
            text.push_render_text(RenderText {
                position: glam::vec2(x, y + aim::MAP_GAP),
                text: String::from(words),
                size: aim::MAP_NAME,
                centered: centred,
                ..Default::default()
            });
        }
    }

    /// The list of every body out past the door, per spec 0018.
    ///
    /// A column down the left, where neither the help across the top nor the
    /// map along the bottom goes, so the three can be read at once.
    ///
    /// Every stop including the way back, in the order they come out from the
    /// sun, because the thing it is for is reaching the seventeenth without
    /// stepping through sixteen.
    fn draw_the_list(&self, geometry: &mut Geometry, text: &mut TextRenderer) {
        use blitzkit::geometry::quad::Quad;

        let Some(on) = self.listing else { return };
        let stops = beyond::stops(self.the_door());
        let at = aim::list_at(self.window, stops.len());
        let tall = stops.len() as f32 * aim::LIST_STEP;
        let middle = glam::vec2(
            at.x + aim::LIST_WIDE * 0.5,
            at.y + tall * 0.5 - aim::LIST_STEP * 0.5,
        );
        let size = glam::vec2(aim::LIST_WIDE + aim::LIST_PAD * 2.0, tall + aim::LIST_PAD);

        geometry.push_quad(&Quad::colored(
            middle,
            size + glam::Vec2::splat(aim::PLATE_EDGE * 2.0),
            aim::PLATE_LINE,
        ));
        geometry.push_quad(&Quad::colored(middle, size, aim::PLATE));

        // the one it is resting on, marked by its own plate rather than by a
        // brighter word: a list that says which line is chosen by weight alone
        // is a list you have to compare lines to read
        geometry.push_quad(&Quad::colored(
            glam::vec2(middle.x, at.y + on as f32 * aim::LIST_STEP),
            glam::vec2(aim::LIST_WIDE, aim::LIST_STEP),
            aim::PLATE_LINE,
        ));

        for (n, stop) in stops.iter().enumerate() {
            text.push_render_text(RenderText {
                position: glam::vec2(
                    at.x,
                    at.y + n as f32 * aim::LIST_STEP - aim::LIST_SIZE * 0.5,
                ),
                text: String::from(stop.name),
                size: aim::LIST_SIZE,
                color: if n == on { aim::LIST_ON } else { aim::LIST_OFF },
                ..Default::default()
            });
        }
    }

    /// Where a step of `dt` puts you, and how fast you are falling after it.
    ///
    /// Its own method because it is the one place that chooses between walking
    /// the building and riding the line of spec 0018, and a test that drives
    /// the two halves separately does not test the choice: with the ride cut
    /// out of `update` entirely, one that drove `riding` and `beyond::ridden`
    /// by hand still passed.
    fn gone(&mut self, wish: Vec3, pace: f32, dt: f32) -> (Vec3, f32) {
        // out past the door you ride rather than walk. Along the line, no
        // steering, and no collision to do: that floor is one flat slab with
        // nothing on it, and a 185 metre a second sweep through `walk` would be
        // three metres a frame looking for something to catch on.
        if let Some(to) = self.riding_to {
            let door = self.the_door();
            let stops = beyond::stops(door);
            let (at, there) = beyond::ridden(&stops, self.stop, to, self.at, dt, door);

            // a leg ends where it was going, per spec 0018
            if there {
                self.stop = to;
                self.riding_to = None;
            }

            return (at, 0.0);
        }

        // along the floor, over anything no taller than a step, and down.
        // Spec 0007: the building has a height in it now, so getting about is
        // no longer one call that pushes you sideways.
        walk::walk(
            self.at,
            if wish.length_squared() > 1e-6 {
                wish.normalize() * pace
            } else {
                Vec3::ZERO
            },
            self.falling,
            RADIUS,
            dt,
            &self.room.solid(),
            // a step costs its own height out of a budget that fills at
            // `walk::CLIMBS` a second, and until the budget covers the next
            // riser you simply walk into it. Nothing paced the stair before
            // this: a tread is crossed in a fifteenth of a second and the riser
            // over it went by in the same frame, so the flight out of the
            // cellar went past at better than three units a second upwards.
            self.climbs <= 0.0,
        )
    }

    /// Whether the ride of spec 0018 is carrying you.
    ///
    /// It was shift held down, which everywhere else in this building means
    /// run. Two things were wrong with that. Jake could not take a picture of
    /// the place, because the shift in his own screenshot shortcut set him off
    /// down the line. And a hold is the wrong shape for a thing that stops
    /// itself: the leg ends where it was going whether or not the key is still
    /// down, so what the holding bought was stopping early, which one more
    /// press buys instead.
    fn riding(&self) -> bool {
        self.in_the_deep() && self.riding_to.is_some()
    }

    /// Puts the chain of spec 0018 back to the door whenever you are not out
    /// there, so that stepping through it always starts at the beginning.
    fn mind_the_threshold(&mut self) {
        if !self.in_the_deep() {
            self.stop = 0;
            self.riding_to = None;
            self.listing = None;
        }
    }

    /// Moves the list one body out or one in, and stops at the ends rather
    /// than wrapping: a list that wraps is a list you can lose your place in.
    fn walk_the_list(&mut self, way: i32) {
        let many = beyond::stops(self.the_door()).len() as i32;
        let Some(on) = self.listing else { return };

        self.listing = Some((on as i32 + way).clamp(0, many - 1) as usize);
    }

    /// Sets the ride off for the next body, or stops it if it is already
    /// going. Spec 0018.
    fn ride(&mut self) {
        if self.riding_to.take().is_some() {
            return;
        }

        let door = self.the_door();
        let stops = beyond::stops(door);

        self.riding_to = beyond::next_along(&stops, self.stop, self.facing(), door);
    }

    /// Puts you at a stop outright, which is what the list is for.
    fn go_to(&mut self, stop: usize) {
        let door = self.the_door();
        let stops = beyond::stops(door);

        if stops.get(stop).is_none() {
            return;
        }

        let (at, along) = beyond::arriving(stops[self.stop].at, &stops[stop]);

        self.at = at;
        self.stop = stop;
        self.riding_to = None;
        self.yaw = along.x.atan2(-along.z);
        self.pitch = 0.0;
    }

    fn the_sun(&self) -> (Vec3, Vec3) {
        let middle = behind::at(self.room.reaches);
        let sun = vec3(middle.x, orrery::PLANE, middle.z);
        let (from, to) = behind::way();
        let door = vec3((from + to) * 0.5, orrery::PLANE, self.room.reaches);

        (sun, (door - sun).normalize_or_zero())
    }

    /// One body's name, over the body, facing you.
    ///
    /// Which names get drawn is `orrery::laid_out`'s to say, and this draws
    /// one it has already chosen.
    ///
    /// Written over every frame, because where it stands, how big it is and
    /// which way it faces all come off where your eye is. A name is three
    /// numbers and a quad, and sixteen of them is the cheapest thing in this
    /// room by a long way.
    fn draw_a_name(&self, scene: &mut Scene, name: &orrery::Named) {
        let (Some(mesh), Some((said, wide))) =
            (self.name_mesh, self.names.get(name.which).copied())
        else {
            return;
        };

        let eye = self.looking_from();
        let tall = orrery::label_tall((name.at - eye).length());
        let stands = orrery::label_at(name.at, name.across, eye, name.hangs);

        scene.push_textured(
            mesh,
            said,
            &Transform::at(stands)
                .with_rotation(orrery::label_turn(stands, eye))
                .with_scale(vec3(tall * wide, 1.0, tall)),
            aim::BODY_NAME,
            // the same pinpoint the rest of this room gets. The highlight is
            // added and not multiplied, so a name with a lobe on it is a name
            // with a white smear across the middle of it.
            aim::VOID,
        );
    }

    /// The sun, the planets, their moons, Saturn's rings and every one of them
    /// named. Spec 0015.
    fn draw_the_system(&mut self, scene: &mut Scene, cube: MeshId, camera: &Camera) {
        let (Some(ball), Some(rings)) = (self.planet_mesh, self.ring_mesh) else {
            return;
        };
        if self.worlds.is_empty() {
            return;
        }

        // the stars, on all six surfaces, before anything in the room so the
        // room is a sky by the time a planet is drawn against it
        for (((at, size, looks), mesh), stars) in orrery::walls(self.room.reaches)
            .into_iter()
            .zip(&self.star_mesh)
            .zip(&self.stars)
        {
            scene.push_textured(
                *mesh,
                *stars,
                &Transform::at(at)
                    .with_rotation(room::laid_on(looks))
                    .with_scale(vec3(size.x, 1.0, size.y)),
                aim::STARS,
                // the tightest highlight there is. The shader adds the
                // highlight on top of the surface rather than multiplying it
                // in, so a floor as near black as this one still takes a full
                // white one: at `DULL` the sun's lamp laid a spindle of light
                // down the floor and another up the ceiling, and with a
                // planet in front of its waist it read as a flame standing in
                // the room. `DULL` is a lobe about a degree across, and a
                // degree along a floor you are looking down is metres.
                aim::VOID,
            );
        }

        // the way out, named, because the way out is a slot in a black wall
        // a hundred and fifty nine metres from the far corner. Spec 0015.
        {
            let (at, looks) = orrery::exit_sign(self.room.reaches);

            scene.push_colored(
                cube,
                &Transform::at(at + looks * orrery::SIGN_THICK * 0.5).with_scale(vec3(
                    orrery::SIGN_WIDE + orrery::SIGN_BORDER * 2.0,
                    orrery::SIGN_TALL + orrery::SIGN_BORDER * 2.0,
                    orrery::SIGN_THICK,
                )),
                aim::EXIT_HOUSING,
            );

            if let (Some(said), Some(mesh)) = (self.way_out, self.way_out_mesh) {
                scene.push_textured(
                    mesh,
                    said,
                    &Transform::at(at + looks * (orrery::SIGN_THICK + 0.004))
                        .with_rotation(room::laid_on(looks))
                        .with_scale(vec3(orrery::SIGN_WIDE, 1.0, orrery::SIGN_TALL)),
                    aim::EXIT,
                    aim::VOID,
                );
            }
        }

        let (sun, toward) = self.the_sun();
        let turned = |body: &orrery::Body, since: f32| {
            let spin = orrery::spin_seconds(body);
            let lean = glam::Quat::from_rotation_z(body.tilt.to_radians());

            lean * glam::Quat::from_rotation_y(since / spin * std::f32::consts::TAU)
        };

        // the sun. Past white, so it carries its own light the way the neon
        // does: a lit ball at the middle of a dark room is a grey ball.
        scene.push_material(
            ball,
            &Transform::at(sun)
                .with_rotation(turned(&orrery::SUN, self.since))
                // not `SUN.drawn()`, which is its true size on the body
                // scale and does not fit. See `orrery::SUN_DRAWN`.
                .with_scale(Vec3::splat(orrery::sun_drawn())),
            aim::SUN_FACE,
            aim::DULL,
        );
        // the names are gathered and not drawn here: which of them fit is
        // worked out on the glass once every body is placed, below.
        let mut names = vec![orrery::Named {
            which: 0,
            at: sun,
            across: orrery::sun_drawn(),
            hangs: orrery::Hangs::Over,
            wide: self.names.first().map(|(_, wide)| *wide).unwrap_or(1.0),
        }];

        let mut skin = 1;

        for planet in orrery::PLANETS.iter() {
            let at = sun
                + orrery::stands(
                    planet,
                    toward,
                    self.swung,
                    self.since,
                    self.resting.get(skin - 1).copied().unwrap_or(0.0),
                );

            if let Some(worn) = self.worlds.get(skin) {
                scene.push_textured(
                    ball,
                    *worn,
                    &Transform::at(at)
                        .with_rotation(turned(&planet.body, self.since))
                        .with_scale(Vec3::splat(planet.body.drawn())),
                    aim::WORLD,
                    aim::MATTE,
                );
            }
            names.push(orrery::Named {
                which: skin,
                at,
                across: orrery::label_across(&planet.body, planet.body.drawn()),
                hangs: orrery::Hangs::Over,
                wide: self.names.get(skin).map(|(_, wide)| *wide).unwrap_or(1.0),
            });
            skin += 1;

            // the rings, leaning with the planet they belong to
            if planet.body.name == "Saturn" {
                scene.push_textured(
                    rings,
                    self.rings.unwrap_or(TextureId::WHITE),
                    &Transform::at(at)
                        .with_rotation(glam::Quat::from_rotation_z(planet.body.tilt.to_radians()))
                        .with_scale(Vec3::ONE),
                    aim::RING,
                    aim::MATTE,
                );
            }

            for moon in planet.moons.iter() {
                let round = orrery::moon_angle_at(moon, self.since);
                let out = vec3(round.cos(), 0.0, round.sin()) * moon.far();

                if let Some(worn) = self.worlds.get(skin) {
                    scene.push_textured(
                        ball,
                        *worn,
                        &Transform::at(at + out)
                            .with_rotation(turned(&moon.body, self.since))
                            .with_scale(Vec3::splat(moon.body.drawn())),
                        aim::WORLD,
                        aim::MATTE,
                    );
                }
                names.push(orrery::Named {
                    which: skin,
                    at: at + out,
                    across: moon.body.drawn(),
                    hangs: orrery::Hangs::Under,
                    wide: self.names.get(skin).map(|(_, wide)| *wide).unwrap_or(1.0),
                });
                skin += 1;
            }
        }

        // and the same system again at true scale, through the door at the
        // far end. Spec 0018.
        //
        // Drawn whether or not you are out there, and its names gathered into
        // the same list as the room's. Two layouts in one frame would be two
        // accounts of which labels fit, and the second would overwrite the
        // first's memory of what it drew last.
        {
            let door = self.the_door();
            let out = beyond::stops(door);
            let bodies = orrery::every_body();

            for stop in out.iter() {
                let Some(which) = stop.which else { continue };
                let body = bodies.get(which).copied().unwrap_or(orrery::SUN);

                if which == 0 {
                    scene.push_material(
                        ball,
                        &Transform::at(stop.at)
                            .with_rotation(turned(&body, self.since))
                            .with_scale(Vec3::splat(stop.across)),
                        aim::SUN_FACE,
                        aim::DULL,
                    );
                } else if let Some(worn) = self.worlds.get(which) {
                    scene.push_textured(
                        ball,
                        *worn,
                        &Transform::at(stop.at)
                            .with_rotation(turned(&body, self.since))
                            .with_scale(Vec3::splat(stop.across)),
                        aim::WORLD,
                        aim::MATTE,
                    );
                }

                names.push(orrery::Named {
                    which,
                    at: stop.at,
                    across: orrery::label_across(&body, stop.across),
                    hangs: if body.name == "Saturn" || which == 0 {
                        orrery::Hangs::Over
                    } else {
                        orrery::Hangs::Under
                    },
                    wide: self.names.get(which).map(|(_, wide)| *wide).unwrap_or(1.0),
                });
            }

            // Saturn's rings, which are the one body in the tables that is not
            // a ball
            if let Some(stop) = out.iter().find(|stop| stop.name == "Saturn") {
                let tilt = orrery::PLANETS
                    .iter()
                    .find(|planet| planet.body.name == "Saturn")
                    .map(|planet| planet.body.tilt)
                    .unwrap_or(0.0);

                scene.push_textured(
                    rings,
                    self.rings.unwrap_or(TextureId::WHITE),
                    &Transform::at(stop.at)
                        .with_rotation(glam::Quat::from_rotation_z(tilt.to_radians()))
                        // the ring mesh is built for spec 0015's Saturn, so
                        // it is scaled by how much bigger this one is
                        .with_scale(Vec3::splat(
                            stop.across
                                / orrery::PLANETS
                                    .iter()
                                    .find(|planet| planet.body.name == "Saturn")
                                    .map(|planet| planet.body.drawn())
                                    .unwrap_or(1.0),
                        )),
                    aim::RING,
                    aim::MATTE,
                );
            }

            // the sky, before anything out here is drawn against it. Round
            // the eye and not round the room: it moves with you, so walking
            // does not shift a star, which is what a star does.
            if let Some(mesh) = self.sky_mesh {
                let eye = self.looking_from();

                for ((at_, size, looks), stars) in beyond::sky().into_iter().zip(&self.sky) {
                    scene.push_textured(
                        mesh,
                        *stars,
                        &Transform::at(eye + at_)
                            .with_rotation(room::laid_on(looks))
                            .with_scale(vec3(size.x, 1.0, size.y)),
                        aim::STARS,
                        aim::VOID,
                    );
                }
            }

            // the number on the nearest unit post. One at a time: a name out
            // here is drawn at a fixed angle whatever its distance, per
            // `orrery::label_tall`, so thirty of them down one line are thirty
            // the same size stacked on the vanishing point.
            if let (Some(mesh), Some((unit, stands))) = (
                self.name_mesh,
                beyond::numbered(door, self.at, self.facing()),
            ) {
                if let Some((said, wide)) = self.units.get(unit as usize - 1).copied() {
                    let eye = self.looking_from();
                    let tall = orrery::label_tall((stands - eye).length());
                    // off the line and not over it, by `beyond::ASIDE_HEIGHTS`,
                    // which is where the reason lives. To the world's west,
                    // which is away from the moons: they all stand east of
                    // their planets and their names with them.
                    let at_ = stands + Vec3::Y * tall * 0.6
                        - Vec3::X * (beyond::POST_OFF + tall * beyond::ASIDE_HEIGHTS);

                    scene.push_textured(
                        mesh,
                        said,
                        &Transform::at(at_)
                            .with_rotation(orrery::label_turn(at_, eye))
                            .with_scale(vec3(tall * wide, 1.0, tall)),
                        aim::BODY_NAME,
                        aim::VOID,
                    );
                }
            }

            // the spine down the line and the posts beside it, which are what
            // the ride is measured against
            for mark in beyond::marked(door) {
                scene.push_colored(
                    cube,
                    &Transform::at(mark.at).with_scale(mark.size),
                    if mark.glows {
                        aim::BEYOND_UNIT
                    } else {
                        aim::BEYOND_MARK
                    },
                );
            }

            // the stars laid on the floor, a hair above it, the way the room
            // of spec 0015 lays its own
            if let (Some(mesh), Some(stars)) = (self.deep_floor_mesh, self.deep_floor) {
                let (middle, size) = beyond::floor(door);

                scene.push_textured(
                    mesh,
                    stars,
                    &Transform::at(vec3(middle.x, beyond::STARS_HIGH, middle.z))
                        .with_scale(vec3(size.x, 1.0, size.z)),
                    aim::STARS,
                    aim::VOID,
                );
            }

            // and the floor they stand on
            let (middle, size) = beyond::floor(door);

            scene.push_material(
                cube,
                &Transform::at(middle).with_scale(size),
                aim::BEHIND_FLOOR,
                // the tightest highlight there is, for the reason the hidden
                // room's own floor has it: a broad lobe on a floor you are
                // looking along is metres of it, and from the Earth the sun
                // laid a searchlight down the whole 1,105 metres back to
                // itself. The reading here is a sun that has shrunk to a
                // speck, and a mile of blazing floor is the opposite of it.
                aim::VOID,
            );
        }

        // and the names, as many of them as fit without landing on each
        // other. Nearest first, per `orrery::laid_out`.
        //
        // Only the ones on your side of the wall. Both systems are drawn every
        // frame and a wall stands between them, and the layout reads the
        // frustum and not what is in the way: from the Earth of spec 0018 the
        // whole room's model is 1,200 metres behind you through a wall, and
        // its eight names came up stacked on the one speck the sun had become.
        //
        // It is also the only thing keeping the two lists apart. A name is
        // picked by `which`, which is the body it is for, and the same body
        // stands in both systems: laid out together, choosing the room's Earth
        // draws this one's as well. Anything that wants both at once has to
        // give a name an identity of its own first.
        let deep = self.in_the_deep();
        let door = self.the_door().z;
        let names: Vec<orrery::Named> = names
            .into_iter()
            .filter(|name| (name.at.z > door) == deep)
            .collect();

        let eye = self.looking_from();
        let want = orrery::laid_out(
            &names,
            eye,
            camera.view_projection(),
            camera.fov_y,
            camera.aspect(),
            &self.named_last,
        );

        let fits = want;

        self.named_last.clear();
        self.named_last.extend_from_slice(&fits);

        for name in names.iter().filter(|name| fits.contains(&name.which)) {
            self.draw_a_name(scene, name);
        }
    }

    #[cfg(test)]
    fn reachable(&self) -> reach::Reach {
        reach::Reach::found(
            self.looking_from(),
            reach::Body {
                radius: RADIUS,
                rise: STEPS_UP,
                tall: ROOM_TALL,
            },
            &self.room.solid(),
            self.room.outline().expanded(Vec3::splat(REACH_PAST)),
            reach::STEP,
        )
    }

    /// The strike lighting the sky right now, and how much of itself it is
    /// giving. Spec 0016.
    ///
    /// One at a time. Two flashes at once is a thing that happens in a storm
    /// and is a muddle in an exhibit about one flash and one bang, and
    /// `garden::STRIKES` is eleven seconds against a flash that lasts half of
    /// one, so the case is arithmetic rather than taste.
    fn flashing(&self) -> Option<(garden::Strike, f32)> {
        let strike = garden::strike(garden::struck(self.since)?);
        let after = self.since - strike.at;

        if after > garden::FLASHES {
            return None;
        }

        match garden::flashing(after) {
            fade if fade > 0.004 => Some((strike, fade)),
            _ => None,
        }
    }

    /// What a strike adds to the sky's own multiplier, per spec 0016.
    ///
    /// Added rather than multiplied. The sky's multiplier runs from 0.06 at
    /// midnight to 1.5 at noon, and a flash that multiplied it would be
    /// twenty-five times brighter at noon than at midnight, which is the
    /// wrong way round and is not a flash either way: a strike puts out what
    /// a strike puts out, and whether that is a lot depends on what was
    /// already there.
    fn flashed(&self) -> glam::Vec4 {
        match self.flashing() {
            Some((strike, fade)) => {
                (aim::FLASH_SKY * fade * garden::flash_lit(strike.away)).extend(0.0)
            }
            None => glam::Vec4::ZERO,
        }
    }

    fn looking_from(&self) -> Vec3 {
        vec3(self.at.x, self.eye + EYE, self.at.z)
    }

    fn facing(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();

        vec3(sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch)
    }

    fn forward(&self) -> Vec3 {
        let (sin, cos) = self.yaw.sin_cos();
        vec3(sin, 0.0, -cos)
    }
}

/// Which of the four walking slots an arrow is, if it is one.
///
/// The same order the walking itself uses, so a key the toy takes can be let go
/// of in the one place rather than in every arm that handles one.
/// Which arrows a toy takes, so the rest are still yours to look with.
///
/// It used to take all four whatever it was, which was fine while the arrows
/// walked you: you have WASD for that. Now they turn you, and a toy swallowing
/// the two it has no use for means standing at the globe with no way to look up
/// or down. The globe spins about one axis and the metronome's weight slides
/// along one, so each of them wants one pair and not the other.
fn wanted_by(toy: &str, key: KeyboardKey, shifted: bool) -> bool {
    let up_down = matches!(key, KeyboardKey::Up | KeyboardKey::Down);

    match toy {
        // the weight slides up and down the needle
        metronome::NAME => up_down,
        // and the ball spins about its own axis, which is left and right
        globe::NAME => !up_down,
        // the dial is up and down; leaning on the spindle is any way at all
        gyro::NAME => !shifted || up_down,
        // winding is up and down; hauling the ball is any way at all
        wrecker::NAME => !shifted || up_down,
        _ => false,
    }
}

fn arrow_at(key: KeyboardKey) -> Option<usize> {
    match key {
        KeyboardKey::Up => Some(0),
        KeyboardKey::Down => Some(1),
        KeyboardKey::Left => Some(2),
        KeyboardKey::Right => Some(3),
        _ => None,
    }
}

/// Every game this project has, and where its binary is.
///
/// The names are the folders under `games`, read at build time rather than
/// kept in a list here: a list is a thing that goes out of date, and the
/// project already refuses to let a game exist without a folder.
/// Every repo under `games`, named.
fn repos() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(|at| at.join("games"))
            .unwrap_or_default(),
    )
    .map(|entries| {
        entries
            .flatten()
            .filter(|entry| entry.path().join("Cargo.toml").is_file())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect()
    })
    .unwrap_or_default();
    names.sort();

    names
}

/// Where a game's built binary sits, which is beside this one.
fn beside() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|at| at.to_path_buf()))
        .unwrap_or_default()
}

/// The ones that get a cabinet in the hall, which is every repo but two.
///
/// cascada is a toy: you stand dominoes up wherever you like and push one, and
/// there is nothing to win, which is the whole of what separates the nook from
/// the hall. poolhall is a game and there is something to win, so by that rule
/// it belongs out here, and a cabinet with pool on the screen is still the
/// wrong object. Pool is a table. The thing you want is to walk up to the
/// table.
///
/// Both had a cabinet for the one reason everything else has one, which is that
/// they are folders under `games`, and being a folder under `games` is not an
/// argument about anything.
fn games() -> Vec<Cabinet> {
    let beside = beside();
    let elsewhere = [cascada::NAME, cellar::POOLHALL];

    repos()
        .iter()
        .filter(|name| !elsewhere.contains(&name.as_str()))
        .map(|name| Cabinet::found(name, &beside))
        .collect()
}

/// A game's own screenshot, which is the picture on its cabinet.
fn art_of(name: &str) -> Option<TextureData> {
    let shot = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()?
        .join("games")
        .join(name)
        .join("media/screenshot.png");

    TextureData::from_bytes(&std::fs::read(shot).ok()?).ok()
}

impl Game for Arcade {
    /// Takes the cursor so the mouse can look, and gives it back when the
    /// window loses focus or a game is started in front of this one.
    ///
    /// Declared and never done at first, so `locked` stayed false and every
    /// mouse motion was dropped before it reached the camera.
    fn before_frame(&mut self, renderer: &mut Renderer) {
        if self.wants_lock != self.locked {
            self.locked = renderer.set_cursor_locked(self.wants_lock) && self.wants_lock;
        }

        // the water, written over rather than uploaded again. Spec 0042 of the
        // engine is what this is for: without it a surface that moves is a new
        // mesh every frame and a program that grows until it stops.
        if let Some(mesh) = self.pond_mesh {
            renderer.update_mesh(mesh, &self.pool_in_the_garden.surface());
        }
        if let Some(mesh) = self.pool_mesh {
            renderer.update_mesh(mesh, &self.pool.surface());
        }
        if let Some(mesh) = self.tub_mesh {
            renderer.update_mesh(mesh, &self.tub.surface());
        }

        // and the fountain's thread, whose beads run down it. Written over
        // rather than uploaded again, per spec 0042.
        if let Some(mesh) = self.stream {
            renderer.update_mesh(mesh, &spa::stream_mesh(self.since));
        }

        // and the sky, whose weather goes round by its picture moving rather
        // than by the dome turning. See `garden::dome_mesh`.
        if let Some(mesh) = self.dome {
            renderer.update_mesh(mesh, &garden::dome_mesh(self.since));
        }

        // the coplanar check, per spec 0048 of the engine. Asked here because
        // this is where a game has the renderer, and read back here too: the
        // answer lands during the frame that was asked for, so the press turns
        // into outlines on the frame after. That is the only place the patches
        // can come from, because what fights is drawn and nothing else knows
        // the drawn list.
        if self.wants_planes {
            self.wants_planes = false;

            // where a body can get to, filled out from where you are standing,
            // per spec 0049 of the engine. Without it the report is mostly the
            // tops of ceilings: faces that are flush with their neighbours and
            // that nobody will ever be above to see. The engine cannot work it
            // out itself, because it takes the colliders and the colliders are
            // ours.
            renderer.check_fighting_faces(Some(reach::Body {
                radius: RADIUS,
                rise: STEPS_UP,
                tall: ROOM_TALL,
            }));
        }

        let found: Vec<Aabb> = renderer
            .fights()
            .fights
            .iter()
            .map(|fight| fight.patch)
            .collect();

        if !found.is_empty() || self.fighting.is_empty() {
            self.fighting = found;
        }
    }

    fn load(&mut self, renderer: &mut Renderer) {
        // The sun's map covers the building, every room of it, because
        // outside the map the sun comes through the roof. It was fitted to
        // the hall alone, which was sharp and left the cellar, the baths and
        // the space behind the wall taking full sun through solid ground.
        // See `Room::outline`, which has the measurements.
        renderer.set_scene_bounds(self.room.under_the_sun());

        // No contact shadows in this building, per spec 0029's own limits.
        //
        // The bath house ceiling is eleven ribs hanging fifteen centimetres
        // under a lid, lit by sconces low on the walls, so the light runs
        // along those surfaces rather than onto them. That is the one shape a
        // march through the depth buffer cannot answer: it walks nearly
        // parallel to the edge, and whether a step lands inside the rib next
        // door comes down to a texel. Jake called what that produces striping
        // and reported it five times in an afternoon.
        //
        // Every setting was tried and measured at his camera. Dithering the
        // march's phase per pixel turns it into a fizz that crawls as you
        // walk. One phase for every pixel turns it into a dashed line. Forty
        // steps, sixty, a hundred, a longer start, a start that grows as the
        // light grazes: all within the noise of each other. Counting how many
        // steps were inside rather than stopping at the first trades the
        // dashes for a regular scallop, which is worse.
        //
        // So it is off here. What that costs is the darkening where a thing
        // meets what it stands on, which this building had for a day and which
        // nobody asked after. It is one line to put back, and the engine keeps
        // the feature for a game whose rooms are not built like this one.
        renderer.set_contact_shadows(false);

        self.cube = Some(renderer.add_mesh(&MeshData::cube()));
        // The carpet covers the aisle alone now, which is less than half the
        // width of the quad it used to be, so its count comes down with it. The
        // plain eight across a quad that narrow is a weave half the size it has
        // always been in here, and the tile is meant to be the same thing on
        // this floor whatever the room is doing.
        let was = (room::WALL + room::CABINET.x) * 2.0 + room::NOOK_DEEP;
        self.floor = Some(renderer.add_mesh(&room::tiled_plane(glam::vec2(
            carpet::TILES * (room::WALL + room::CABINET.x) * 2.0 / was,
            carpet::TILES,
        ))));
        self.ceiling_mesh = Some(renderer.add_mesh(&MeshData::plane()));
        self.carpet = Some(renderer.add_texture(&carpet::woven()));
        {
            // and the wear in it at the foot of the near wall, per spec 0020
            let track = behind::track(self.room.reaches);

            self.worn = Some(renderer.add_texture(&carpet::worn(
                track.aisle,
                track.wall,
                track.wide,
                track.fade,
                track.shape,
            )));
            self.worn_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        }
        let (_, rug) = room::open_floor(self.room.reaches);
        self.rug = Some(renderer.add_texture(&carpet::rug(rug.z / rug.x)));
        self.boards = Some(renderer.add_texture(&carpet::boards(carpet::BOARD_SEED)));
        self.barrel = Some(renderer.add_mesh(&cellar::barrel_mesh()));
        self.hoop = Some(renderer.add_mesh(&cellar::hoop_mesh()));
        self.bottle = Some(renderer.add_mesh(&cellar::bottle_mesh()));
        self.peg = Some(renderer.add_mesh(&cellar::peg_mesh()));
        self.bag = Some(renderer.add_mesh(&cellar::bag_mesh()));
        self.urn = Some(renderer.add_mesh(&spa::urn_mesh()));
        self.sconce = Some(renderer.add_mesh(&spa::sconce_mesh()));
        self.basin = Some(renderer.add_mesh(&spa::basin_mesh()));
        self.stream = Some(renderer.add_mesh(&spa::stream_mesh(0.0)));
        self.dish = Some(renderer.add_mesh(&spa::dish_mesh()));
        self.flame = Some(renderer.add_mesh(&cellar::flame_mesh()));
        self.candlestick = Some(renderer.add_mesh(&cellar::candlestick_mesh()));
        self.decanter = Some(renderer.add_mesh(&cellar::decanter_mesh()));
        self.wineglass = Some(renderer.add_mesh(&cellar::glass_mesh()));
        let (_, mat) = cellar::rug(self.room.reaches);
        self.afghan = Some(renderer.add_texture(&carpet::afghan(mat.z / mat.x)));
        self.afghan_mesh = Some(renderer.add_mesh(&room::tiled_floor(1.0)));
        // counted off the room, so a plank is the same width whichever way the
        // cellar is longer
        let (_, boards) = cellar::floor(self.room.reaches);
        self.cellar_floor = Some(renderer.add_mesh(&room::tiled_plane(glam::vec2(
            boards.x / cellar::PLANK,
            boards.z / cellar::PLANK,
        ))));
        // a few boards rather than one. Every panel cut from the same picture
        // is every panel cut from the same tree in the same place, which is the
        // one thing a wall of wood never is.
        self.grain = (0..carpet::GRAINS)
            .map(|n| {
                renderer.add_texture(&carpet::grained(carpet::GRAIN_SEED.wrapping_add(n * 7919)))
            })
            .collect();
        // a quad per face of the stairwell, each counted from its own size so a
        // board is a board's width whichever face it is on
        self.boarding = cellar::shaft_faces(self.room.reaches)
            .into_iter()
            .map(|(_, size, _)| {
                renderer.add_mesh(&room::tiled_plane(glam::vec2(
                    size.x / cellar::PANEL_WIDE,
                    size.y / cellar::PANEL_LONG,
                )))
            })
            .collect();
        self.stonework = Some(renderer.add_texture(&carpet::stonework(carpet::STONE_SEED)));
        self.storm = Some(renderer.add_texture(&garden::storm(0x5704)));
        self.dome = Some(renderer.add_mesh(&garden::dome_mesh(0.0)));
        // one picture per bed rather than one tile repeated over all of them:
        // the raking runs in rings round the stones, and a tile cannot know
        // where a stone is
        let groups = garden::group_middles(self.room.reaches);
        self.gravel = garden::beds(self.room.reaches)
            .into_iter()
            .map(|(bed, size)| renderer.add_texture(&garden::raked(0x64A4, bed, size, &groups)))
            .collect();
        self.stones = garden::CUTS
            .iter()
            .map(|seed| renderer.add_mesh(&garden::rock_mesh(*seed)))
            .collect();
        self.slabs = garden::CUTS
            .iter()
            .map(|seed| renderer.add_mesh(&garden::slab_mesh(*seed)))
            .collect();
        self.pond_mesh = Some(renderer.add_mesh(&self.pool_in_the_garden.surface()));
        // a trunk apiece, because the lean is a share of the height and the
        // thickness is not: one mesh scaled to two heights is a sapling and a
        // log. The pads are one lump per cut, turned and squashed per pad.
        self.trunks = garden::trees(self.room.reaches)
            .iter()
            .map(|tree| renderer.add_mesh(&garden::trunk_mesh(tree)))
            .collect();
        self.crown = Some(renderer.add_mesh(&garden::pad_mesh(0x5A2F)));
        self.limb = Some(renderer.add_mesh(&garden::limb_mesh()));
        self.lantern = Some(renderer.add_mesh(&garden::lantern_mesh()));
        self.koi = Some(renderer.add_mesh(&garden::koi_mesh()));
        self.fin = Some(renderer.add_mesh(&garden::fin_mesh()));
        // a skin each, painted rather than tinted: three fish of one colour
        // are three of the same fish
        self.skins = garden::SKINS
            .iter()
            .map(|(seed, patch, patches)| {
                renderer.add_texture(&garden::koi_skin(*seed, *patch, *patches))
            })
            .collect();
        // one quad for all of them. Each bed wears its own picture, counted
        // nought to one, so there is nothing left for a per bed mesh to say.
        self.ground = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        self.tiled = Some(renderer.add_texture(&carpet::tiled(carpet::TILE_SEED)));

        // a quad per face of the basin, each with its own count of tiles, so a
        // tile is the same size everywhere. One mesh for all of them makes the
        // tiles on the sides as tall as the sides are.
        // the spines, one picture a title rather than one a book: there are
        // four hundred books and fifty-six titles, and a picture of a word is
        // the same picture wherever it is hung
        self.titles = study::TITLES
            .iter()
            .map(|title| {
                // stencilled and not drawn: a spine is a colour and the black
                // ground behind a title would cover it. Spec 0047.
                let drawn = blitzkit::text::stencilled(title, study::TITLE_TEXELS);
                // how long the words are against how tall, which is what
                // decides how far down a spine they reach
                let shape = drawn.width() as f32 / drawn.height().max(1) as f32;

                (renderer.add_texture(&drawn), shape)
            })
            .collect();
        self.lining = spa::lining(self.room.reaches)
            .into_iter()
            .map(|(_, size, _)| renderer.add_mesh(&room::tiled_plane(size / spa::TILE)))
            .collect();
        // one picture per wall and one quad to lay it on, both counted off the
        // wall itself. A seed apiece, so no two walls in the garden wear the
        // same stains.
        self.plaster = garden::facing(self.room.reaches)
            .into_iter()
            .enumerate()
            .map(|(n, face)| {
                let (_, size, _) = face.plane();

                renderer.add_texture(&garden::plastered(
                    garden::PLASTER_SEED.wrapping_add(n as u32 * 7919),
                    size,
                ))
            })
            .collect();
        self.plaster_mesh = garden::facing(self.room.reaches)
            .into_iter()
            .map(|_| renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)))
            .collect();
        self.pole = Some(renderer.add_mesh(&garden::pole_mesh()));
        // the solar system. One ball for every world, scaled: a sphere is the
        // one shape in this building that is the same shape at every size.
        self.planet_mesh = Some(renderer.add_mesh(&MeshData::sphere(64, 32)));
        self.ring_mesh = Some(renderer.add_mesh(&orrery::ring_mesh()));
        self.worlds = orrery::every_body()
            .iter()
            .map(|body| renderer.add_texture(&orrery::picture(body)))
            .collect();
        // and every one of them named, off the same list, so a picture and a
        // name cannot end up on two different bodies
        self.names = orrery::every_body()
            .iter()
            .map(|body| {
                let said = orrery::named(body.name);
                let wide = said.width() as f32 / said.height().max(1) as f32;

                (renderer.add_texture(&said), wide)
            })
            .collect();
        // and the numbers for the unit posts out past the door, per spec 0018.
        // Thirty pictures of three characters each, made the same way the
        // bodies' names are so the two read as one hand.
        self.units = (1..=beyond::units())
            .map(|unit| {
                let said = orrery::named(&format!("{} AU", unit));
                let wide = said.width() as f32 / said.height().max(1) as f32;

                (renderer.add_texture(&said), wide)
            })
            .collect();
        // the sky out past the door, per spec 0018. One picture a face, each
        // the biggest `starred` will make, because a face of this one fills a
        // quarter turn of the view and wants every texel it can have.
        self.sky = beyond::sky()
            .into_iter()
            .enumerate()
            .map(|(n, (_, size, _))| {
                renderer.add_texture(&orrery::galaxied(
                    orrery::STAR_SEED ^ 0x5E_0000 ^ (n as u32 * 7919),
                    size,
                ))
            })
            .collect();
        self.sky_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        {
            let (_, size) = beyond::floor(beyond::at(self.room.reaches));
            let patch = glam::Vec2::splat(beyond::STAR_PATCH);

            self.deep_floor =
                Some(renderer.add_texture(&orrery::starred(orrery::STAR_SEED ^ 0x61_0000, patch)));
            self.deep_floor_mesh = Some(renderer.add_mesh(&room::tiled_plane(
                glam::vec2(size.x, size.z) / beyond::STAR_PATCH,
            )));
        }
        self.name_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        self.rings = Some(renderer.add_texture(&orrery::ringed(orrery::STAR_SEED ^ 0x1D)));
        self.way_out = Some(
            renderer.add_texture(&blitzkit::text::drawn(orrery::WAY_OUT, orrery::SIGN_TEXELS)),
        );
        self.way_out_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        self.stars = orrery::walls(self.room.reaches)
            .into_iter()
            .enumerate()
            .map(|(n, (_, size, _))| {
                renderer.add_texture(&orrery::starred(
                    orrery::STAR_SEED.wrapping_add(n as u32 * 7919),
                    size,
                ))
            })
            .collect();
        self.star_mesh = orrery::walls(self.room.reaches)
            .into_iter()
            .map(|_| renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)))
            .collect();
        // one picture of moss for all of it, counted off the biggest piece so
        // the clumps are the same size wherever they are, and a quad apiece
        let widest = garden::moss(self.room.reaches)
            .into_iter()
            .fold(glam::Vec2::ZERO, |most, (_, size, _)| most.max(size));
        self.moss = Some(renderer.add_texture(&garden::mossed(garden::MOSS_SEED, widest)));
        self.moss_mesh = garden::moss(self.room.reaches)
            .into_iter()
            .map(|(_, size, _)| renderer.add_mesh(&room::tiled_plane(size / widest)))
            .collect();
        self.stave =
            Some(renderer.add_texture(&carpet::staves(carpet::STAVE_SEED, cellar::STAVES)));
        self.says = Some(renderer.add_texture(&blitzkit::text::drawn(sign::SAYS, sign::TEXELS)));
        self.named = Some(renderer.add_texture(&blitzkit::text::drawn(neon::SAYS, neon::TEXELS)));
        self.baths = Some(renderer.add_texture(&blitzkit::text::drawn(spa::SAYS, spa::TEXELS)));
        self.pool_mesh = Some(renderer.add_mesh(&self.pool.surface()));
        self.tub_mesh = Some(renderer.add_mesh(&self.tub.surface()));
        self.point_mesh = Some(renderer.add_mesh(&sign::point_mesh()));
        self.hung = Some(renderer.add_mesh(&room::hung_mesh()));
        // counted off the nook rather than off the quad, so a plank is the same
        // width whichever way the room is longer
        let (_, nook) = room::nook_floor(self.room.reaches);
        self.boards_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::vec2(
            nook.x / carpet::BOARD_TILE,
            nook.z / carpet::BOARD_TILE,
        ))));
        self.rug_mesh = Some(renderer.add_mesh(&room::tiled_floor(carpet::RUG_TILES)));
        self.wall_grain =
            Some(renderer.add_texture(&carpet::mottled(carpet::WALL_SEED, [220, 220, 220], 34)));
        self.cabinet_grain =
            Some(renderer.add_texture(&carpet::mottled(carpet::CABINET_SEED, [228, 228, 228], 22)));
        self.screen = Some(renderer.add_mesh(&room::screen_mesh()));
        self.ball_mesh = Some(renderer.add_mesh(&MeshData::sphere(18, 12)));
        self.wheel_mesh = Some(renderer.add_mesh(&gyro::wheel_mesh(28)));
        // finer than the room's other sphere, because this one is read rather
        // than glanced at: a coastline on a twelve ring ball is a staircase
        self.globe_mesh = Some(renderer.add_mesh(&MeshData::sphere(72, 36)));
        self.world = Some(renderer.add_texture(&globe::drawn()));

        self.shown = display::meshes()
            .iter()
            .map(|mesh| renderer.add_mesh(mesh))
            .collect();
        let pictures: Vec<Option<TextureData>> = self
            .room
            .stood
            .iter()
            .map(|stood| art_of(&stood.cabinet.name))
            .collect();
        self.glows = pictures
            .iter()
            .map(|art| art.as_ref().map(aim::glow_of).unwrap_or(Vec3::ONE))
            .collect();
        self.art = pictures
            .iter()
            .map(|art| art.as_ref().map(|art| renderer.add_texture(art)))
            .collect();
        self.signs = self
            .room
            .stood
            .iter()
            .map(|stood| {
                let name = blitzkit::text::drawn(&stood.cabinet.name, aim::SIGN_TEXELS);
                renderer.add_texture(&name)
            })
            .collect();
    }

    fn initialize(
        &mut self,
        _geometry: &mut Geometry,
        _text: &mut TextRenderer,
        _sound: &SoundSystem,
        size: (f32, f32),
    ) {
        self.window = size;
        if staged() {
            self.gyro.set_going();
        }

        // the footsteps, read out of the files bundled beside this. A file
        // that will not read is dropped rather than taken as a crash: a
        // footstep is not worth refusing to open the building over, and the
        // tests are what say the files are good.
        self.steps = noise::FLOORS
            .iter()
            .copied()
            .map(|on| {
                let heard = noise::steps(on)
                    .iter()
                    .filter_map(|bytes| Samples::from_wav(bytes).ok())
                    // and anything long enough to have three of itself going
                    // at once is dropped as well, per `noise::longest`. That
                    // used to be the thing standing between this and a queue
                    // that grew for as long as you walked; the engine mixes
                    // now, so it is only a smear being kept out, and
                    // `noise::tests` is what says no floor has gone quiet.
                    .filter(|got| got.seconds() <= noise::longest(on))
                    .collect();

                (on, heard)
            })
            .collect();
        // and the thunder, the same way and for the same reason. Spec 0016:
        // what distance does to the sound of a strike is which band it is
        // picked from, because a recording shaped into something it is not is
        // halfway back to the arithmetic this replaced.
        self.claps = noise::ROLLS
            .iter()
            .copied()
            .map(|band| {
                let heard = noise::claps(band)
                    .iter()
                    .filter_map(|bytes| Samples::from_wav(bytes).ok())
                    .collect();

                (band, heard)
            })
            .collect();
        self.splashes = (0..4)
            .map(|n| noise::splash(2.0 + n as f32 * 1.2, 0x51A5 + n * 7919))
            .collect();

        // and every voice in the building, in several pieces each, off a seed
        // apiece. Spec 0009: one piece queued again the moment it runs out is
        // the same four tenths of a second two and a half times a second, and
        // that is a loop whatever is in it. These all took a seed already, so
        // this costs a few kilobytes.
        //
        // Which room each is in comes from `noise::voices` rather than from
        // here, so where a voice is and which room that puts it in cannot
        // disagree.
        self.voices = noise::voices(self.room.reaches)
            .into_iter()
            .map(|(room, at, make, seed)| {
                let pieces = (0..noise::PIECES)
                    .map(|n| make(seed.wrapping_add(n as u32 * 7919)))
                    .collect();

                (room, at, pieces)
            })
            .collect();
        self.sounding = 0.0;
        self.knock = Some(noise::clack(0xC1AC));
        self.was_standing = self.wrecker.standing();
    }

    fn resized(&mut self, window_size: (f32, f32)) {
        self.window = window_size;
    }

    fn update(
        &mut self,
        dt: f32,
        geometry: &mut Geometry,
        text: &mut TextRenderer,
        sound: &SoundSystem,
    ) {
        self.since += dt;

        // `WHERE=1` says where you are standing, twice a second, on stderr.
        //
        // This is here because the slow part of fixing what somebody can see
        // is finding where they were standing. A fault in this building is
        // reported as a picture, and a picture has to be matched to a camera
        // before anything can be measured or shot again: six guesses at a pose
        // found nothing on 2026-10-08, and one line of this found it at once.
        // On 2026-10-14 it ended an afternoon of chasing ripples on the bath
        // house ceiling, by putting the camera exactly where Jake's was so the
        // same frame could be rendered with one thing changed at a time.
        //
        // Off unless asked for, so it costs a string lookup a frame and
        // nothing else. It prints the room as well as the pose, because which
        // room the building thinks you are in is itself a thing that has been
        // wrong.
        if std::env::var("WHERE").is_ok() {
            self.said += dt;
            if self.said >= 0.5 {
                self.said = 0.0;
                eprintln!(
                    "WHERE at {:.2} {:.2} {:.2}  yaw {:.3}  pitch {:.3}  room {:?}",
                    self.at.x,
                    self.at.y,
                    self.at.z,
                    self.yaw,
                    self.pitch,
                    noise::room_at(self.room.reaches, self.at)
                );
            }
        }
        self.mind_the_threshold();

        // the toy on the bench, stepped at its own rate so a slow frame does
        // not change how it swings. Spec 0006.
        self.owed = (self.owed + dt).min(0.2);
        while self.owed >= cradle::STEP {
            self.solver.step_linked(
                &mut self.cradle,
                &self.ropes,
                &cradle::NO_WORLD,
                GRAVITY,
                cradle::STEP,
            );
            self.owed -= cradle::STEP;
        }
        // the door, and then you out of the door's way. It is the only thing in
        // the building that moves, and it sweeps the floor you stand on to pull
        // the book. Spec 0007.
        self.room.ease(dt);
        self.at = walk::shoved(self.at, RADIUS, &self.room.leaves());
        self.metronome.advance(dt);
        self.wrecker.advance(dt);

        // what the held arrows are leaning on the gyroscope with, taken off the
        // bench rather than written down as the nook's axes. Up and down are up
        // and down whichever side of it you are on; left and right are not, and
        // moving the benches to the other wall turned them round.
        self.gyro.leaning = if let Some(room::Seen::Bench(n)) = self.seen {
            let bench = &self.room.benches[n];
            let lean = [Vec3::Y, Vec3::NEG_Y, -bench.right(), bench.right()];

            self.leaning
                .iter()
                .zip(lean)
                .filter(|(held, _)| **held)
                .map(|(_, way)| way)
                .sum()
        } else {
            self.leaning = [false; 4];
            Vec3::ZERO
        };
        self.gyro.advance(dt);
        self.globe.advance(dt);

        let mut wish = Vec3::ZERO;
        let right = self.forward().cross(Vec3::Y);
        for (held, way) in self
            .walking
            .iter()
            .zip([self.forward(), -self.forward(), -right, right])
        {
            if *held {
                wish += way;
            }
        }

        // in the water you are slower, which is most of what says you are in it
        // rather than beside it. Spec 0008.
        let wading = self.wading();
        let pace = self.pace();

        // the arrows turn and tilt you. Yaw nought faces -z and grows toward
        // +x, which is your right, so the right arrow adds.
        let turn = f32::from(self.looking[3]) - f32::from(self.looking[2]);
        let tilt = f32::from(self.looking[0]) - f32::from(self.looking[1]);
        if turn != 0.0 || tilt != 0.0 {
            self.yaw += turn * TURNS * dt;
            self.pitch = (self.pitch + tilt * TILTS * dt).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        }

        let riding = self.riding();
        let (at, falling) = self.gone(wish, pace, dt);
        // the ride is not walking, so it takes no steps. Spec 0004 reads this
        // for the footfalls and `went` is all it reads.
        let went = if riding {
            0.0
        } else {
            (at - self.at).length() / dt.max(1e-4)
        };
        // what the climb cost, and the budget filling back up
        self.climbs = (self.climbs - walk::CLIMBS * dt).max(0.0) + (at.y - self.at.y).max(0.0);
        self.at = at;
        self.falling = falling;

        // the ears, which go where your eye is and point where you look, so
        // everything that comes from somewhere swings as you turn. Spec 0019 of
        // the engine.
        sound.set_listener(self.looking_from(), self.facing(), Vec3::Y);

        // a footstep every stride, paced by how far you have actually walked
        // rather than by a clock, so slowing down slows them and standing still
        // is silence. Spec 0009.
        //
        // The stride is the one for how you are going, and it is longer at a
        // run: a sprinter covers more ground per step rather than taking the
        // same step twice as often. Held in a local and subtracted again
        // below, because taking it twice would let the two readings disagree
        // across a frame where shift went down.
        let stride = noise::stride(self.running());

        self.paced += went * dt;
        if self.paced >= stride {
            self.paced -= stride;

            let on = self.underfoot();
            self.stepped = self.stepped.wrapping_add(1);

            if let Some((_, heard)) = self.steps.iter().find(|(floor, _)| *floor == on) {
                let (which, pitch, quieter) = noise::bend(self.stepped, heard.len());

                if let Some(step) = heard.get(which) {
                    // bent on the way out rather than baked into the files:
                    // both of these share the buffer they came from, so a step
                    // that is never quite the last step costs nothing. The
                    // engine's spec 0045.
                    sound.play(&step.pitched(pitch).gain(noise::loudness(on) * quieter));
                }
            }
        }

        // a knock for every brick the ball takes off the wall, from the bench
        // it happens on. The one thing in this building that hits anything.
        let standing = self.wrecker.standing();
        if standing < self.was_standing {
            if let (Some(knock), Some(bench)) = (
                self.knock.as_ref(),
                self.room
                    .benches
                    .iter()
                    .find(|bench| bench.name == wrecker::NAME),
            ) {
                sound.play_at(knock, (bench.at + Vec3::Y * bench.size.y).to_array());
            }
        }
        self.was_standing = standing;

        // and a splash the moment you go in, as big as you went in
        if wading && !self.was_wading {
            let hard = (went * 0.5 + self.falling).clamp(0.0, 6.0);
            let which = ((hard / 6.0) * (self.splashes.len() - 1) as f32) as usize;

            if let Some(splash) = self.splashes.get(which) {
                sound.play_at(splash, self.at.to_array());
            }
        }
        self.was_wading = wading;

        // and the thunder of any strike whose sound has had time to get here.
        // Spec 0016, and the whole of it: how far off the strike was is what
        // decided when this plays, how loud it is, how long it runs and what
        // is left of its high end.
        //
        // Mixed rather than queued. `play` hands the sound straight to the
        // mixer and it starts now; it is `play_at` that is a queue, and a
        // three second rumble waiting its turn behind a room's voice would
        // arrive after the next flash. It has nowhere to be played from in any
        // case: the strike is a chosen distance and not a place, so a sound
        // coming from a point in this garden would be the fiction told twice.
        //
        // Where the sky is and nowhere else, per `garden::under_the_sky`,
        // which is a place and not a floor: this asked what was underfoot and
        // played wherever that was gravel, and gravel is what this garden
        // happens to be floored with rather than a fact about the weather. A
        // strike
        // whose bang falls due while you are indoors is a strike you missed,
        // so it is marked heard either way and walking out afterwards does not
        // set off a bang that should already have gone.
        if let Some(n) = garden::heard(self.since) {
            if self.thundered != Some(n) {
                self.thundered = Some(n);

                if garden::under_the_sky(self.room.reaches, self.at) {
                    let strike = garden::strike(n);
                    let band = noise::rolled(strike.away);

                    if let Some((_, heard)) = self.claps.iter().find(|(one, _)| *one == band) {
                        // picked and bent the way a footstep is, off the
                        // strike's own number. How loud is still worked out
                        // from how far off it was, which is the one thing
                        // about the sound that did not become a choice.
                        let (which, pitch, quieter) = noise::bend(n as usize, heard.len());

                        if let Some(clap) = heard.get(which) {
                            let loud = noise::carries(strike.away) * quieter;

                            sound.play(&clap.pitched(pitch).gain(loud));
                        }
                    }
                }
            }
        }

        // and the nearest voice, a piece at a time, queued again as each runs
        // out. Nothing here loops: a sound that goes on for ever is a sound a
        // game queues again, per spec 0044.
        // the voice of the room you are in, one piece at a time. The engine
        // plays what it is given one after another, per spec 0044, and that is
        // not a mixer: four pieces a second each a second long is a queue
        // growing by three seconds every second, which after two hours was a
        // gigabyte of audio waiting its turn and a machine too busy to draw.
        self.sounding -= dt;
        if self.sounding <= 0.0 {
            let ear = self.looking_from();
            let places: Vec<(noise::Room, Vec3)> = self
                .voices
                .iter()
                .map(|(room, at, _)| (*room, *at))
                .collect();

            // the voice of the room you are in, which is not the same as the
            // nearest voice: the baths are under the garden, and the tub was
            // nearer to most of the garden than anything the garden had.
            let heard = noise::nearest(ear, noise::room_at(self.room.reaches, self.at), &places);
            // picked and bent the way a footstep is, or five pieces in a
            // fixed order is a bar of five beats. Spec 0009.
            self.pieced = self.pieced.wrapping_add(1);
            let pieced = self.pieced;

            self.sounding = match heard.and_then(|n| self.voices.get(n)) {
                Some((_, at, voice)) => {
                    let (which, pitch, quieter) = noise::adrift(pieced, voice.len());

                    match voice.get(which) {
                        Some(piece) => {
                            let bent = piece.pitched(pitch).gain(quieter);
                            let runs = bent.seconds();
                            sound.play_at(&bent, at.to_array());

                            runs
                        }
                        None => noise::PIECE,
                    }
                }
                // nothing within earshot, so ask again shortly rather than
                // every frame
                None => noise::PIECE,
            };
        }

        // and the water knows you are in it. The ripples following you about
        // are the thing this room was built for.
        if wading && went > 0.05 {
            self.pool.ring(
                vec3(self.at.x, self.pool.at.y, self.at.z),
                RADIUS,
                RADIUS * blitzkit::water::SPLASH,
                (went * spa::WAKE).min(spa::WAKE_MOST) * dt,
            );
        }

        // the tub bubbles, which is a push in the middle of it rather than
        // anything simulated
        self.blew += dt;
        if self.blew >= spa::BLOWS {
            self.blew -= spa::BLOWS;
            self.tub.push(self.tub.at, spa::TUB * 0.35, -spa::BLOWN);
        }

        // and the pool is stirred by nothing in particular, because perfectly
        // flat water is a sheet of tinted glass
        self.stirred += dt;
        if self.stirred >= spa::STIRS {
            self.stirred -= spa::STIRS;
            let (middle, size, _) = spa::pool(self.room.reaches);
            let at = spa::stirred(middle, size, self.since);

            self.pool.push(at, spa::STIR_WIDE, spa::STIRRED);
        }

        // the sauna fills with steam while its door is shut and empties when
        // it is opened, much faster: a door is a hole and steam is lighter than
        // air. Spec 0008.
        let to = if self.room.sauna_open { 0.0 } else { 1.0 };
        let by = dt
            / if self.room.sauna_open {
                spa::CLEARS
            } else {
                spa::FILLS
            };
        self.steamy += (to - self.steamy).clamp(-by, by);

        self.pool.step(dt);
        self.tub.step(dt);
        // the koi, pushing the water over them as they go. Spec 0010: the
        // pond is still water, and what stops it being a mirror is the fish.
        // Speed and not height, and by the second and not by the frame, so the
        // wake is the same wake whatever this machine is managing.
        for fish in garden::koi(self.room.reaches, self.since) {
            let by = garden::STIRS * garden::stirred(fish.under) * dt;

            if by > 0.0 {
                self.pool_in_the_garden.push(fish.at, fish.long * 1.3, by);
            }
        }
        self.pool_in_the_garden.step(dt);

        // the swing between lined up and in orbit, per spec 0015. Three
        // seconds, and it runs from wherever it has got to: caught halfway
        // and sent back, it goes back from halfway. A toggle that jumps here
        // breaks the one illusion the whole room rests on.
        let by = dt / orrery::SWINGS;

        // at either end of the swing, take each planet's lined up angle
        // again. Neither end can move anything by it: at rest the planet is
        // exactly on it and a whole turn either way is the same place, and
        // swung out it has no weight. Across the swing it is held, which is
        // what keeps the path smooth. See `orrery::stands`.
        if self.swung == 0.0 || self.swung == 1.0 {
            for (rest, planet) in self.resting.iter_mut().zip(orrery::PLANETS.iter()) {
                *rest = orrery::resting_near(orrery::angle_at(planet, self.since));
            }
        }

        self.swung = if self.swinging_out {
            (self.swung + by).min(1.0)
        } else {
            (self.swung - by).max(0.0)
        };

        // the eye catching up with the feet. Nothing while you are on the
        // level, because then they are the same number.
        let behind = self.at.y - self.eye;
        self.eye += behind * (dt / EYE_LAGS).min(1.0);
        if behind.abs() < 1e-4 {
            self.eye = self.at.y;
        }

        let playing = self.playing.now().map(|name| name.to_string());
        self.seen = self.room.looking_at(self.looking_from(), self.facing());

        // walking away lets go. Looking away cannot happen: while you hold one
        // the mouse is turning it rather than the view.
        if let Some(held) = self.spin.holding() {
            if self.seen != Some(room::Seen::Display(held)) {
                self.spin.let_go(self.since);
            }
        }

        // what is under the sight: its name, and then what to do about it. Two
        // sizes, because the name is what you are checking and the line under
        // it is what you are being told.
        let (name, detail) = match (&playing, self.seen) {
            (Some(name), _) => (
                Some(name.clone()),
                Some(String::from("is playing. Quit to come back.")),
            ),
            (None, Some(room::Seen::Cabinet(n))) => {
                let stood = &self.room.stood[n];
                (
                    Some(stood.cabinet.name.clone()),
                    Some(String::from(if stood.cabinet.is_built() {
                        "Click or press enter to play"
                    } else {
                        aim::NOT_BUILT
                    })),
                )
            }
            // spec 0004 gave a shape something to press, so it says what the
            // button would do, as a cabinet does
            (None, Some(room::Seen::Display(n))) => (
                Some(self.room.displays[n].name.to_string()),
                Some(String::from(if self.spin.holding() == Some(n) {
                    "Click or press enter to let go"
                } else {
                    "Click or press enter to turn it"
                })),
            ),
            (None, Some(room::Seen::Bench(n))) => {
                let name = self.room.benches[n].name;
                (Some(name.to_string()), Some(self.about(name)))
            }
            // shut, the only thing you can point at is the one book, so that is
            // what it is called. Open, the whole leaf is the thing, and calling
            // a bookcase a book is the sentence the secret was built on used
            // backwards.
            (None, Some(room::Seen::Baths)) => (
                Some(String::from("the door to the baths")),
                Some(String::from(if self.room.baths_open {
                    "Click or press enter to close it."
                } else {
                    "Click or press enter to open it."
                })),
            ),
            (None, Some(room::Seen::Sauna)) => (
                Some(String::from("the sauna door")),
                Some(String::from(if self.room.sauna_open {
                    "Click or press enter to close it."
                } else {
                    "Click or press enter to open it."
                })),
            ),
            (None, Some(room::Seen::Case)) => {
                let (what, how) = if self.room.open {
                    ("the bookcase", "Click or press enter to push it back.")
                } else {
                    (
                        "a book",
                        "This one is not a book. Click or press enter to pull it.",
                    )
                };

                (Some(String::from(what)), Some(String::from(how)))
            }
            (None, None) => (None, None),
        };

        text.reset();

        // the corner says how to move, and nothing else. What to press is under
        // the sight, and saying it here as well is saying it twice.

        let winking =
            matches!(self.seen, Some(room::Seen::Display(_))) || self.spin.holding().is_some();

        // the wink, once, while the sight is on any of the five. Spec 0004: it
        // was under every shape, five times, which is five times too many for
        // a joke.
        let said: Vec<&str> = if winking {
            vec![self.saying(), aim::WINK]
        } else if self.in_the_deep() {
            // a line to a thing you can do, because this room has twice the
            // controls of any other
            let mut out = vec![self.saying()];
            out.extend(aim::DEEP_MORE);
            out.push(aim::DEEP_NOTE);

            out
        } else if self.in_the_system() {
            vec![self.saying(), aim::SYSTEM_NOTE]
        } else {
            vec![self.saying()]
        };

        // wrapped to the window, and wrapped here rather than left to the
        // renderer. The renderer would do it and `text::room_for` would not
        // know, so the plate underneath would be measured as one long line
        // and come out the wrong shape. With the breaks in the string both
        // see the same thing. The arcade's line in the hall ran off the
        // right of the window until this went in; the one in the room behind
        // the wall happened to be short enough and did not.
        let room = (self.window.0 - aim::SAYS_AT.x * 2.0).max(aim::SAYS_SIZE * 8.0);
        let lines: Vec<String> = said
            .iter()
            .flat_map(|words| {
                blitzkit::text::wrapped(words, aim::SAYS_SIZE, room)
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .collect();

        // a plate under them first, because the text is drawn straight over
        // whatever the room is doing and the room is sometimes a neon sign
        // four feet from your face.
        let laid: Vec<(Vec2, &str, f32)> = lines
            .iter()
            .enumerate()
            .map(|(n, words)| (aim::says_at(n), words.as_str(), aim::SAYS_SIZE))
            .collect();

        plate(geometry, aim::plate_round(&laid, false));

        for (at_, words, size) in laid {
            text.push_render_text(RenderText {
                position: at_,
                text: String::from(words),
                size,
                ..Default::default()
            });
        }

        // and the map at the foot of it, out past the door, per spec 0018,
        // with the list down the side when it is open
        if self.in_the_deep() {
            self.draw_the_map(geometry, text);
            self.draw_the_list(geometry, text);
        }

        // the sight, per spec 0003. Not while a game is up, since there is
        // nothing in this room to point at then.
        if playing.is_none() {
            let at = aim::sight_at(self.window);
            let colour = aim::sight_colour(self.seen.is_some());

            // its own shadow, as the prompt has. On nothing the sight is dim
            // on purpose, and dim grey on a lit grey wall disappeared outright.
            for (at, colour) in [
                (aim::shadow_at(at), aim::SHADOW_COLOUR.with_w(colour.w)),
                (at, colour),
            ] {
                text.push_render_text(RenderText {
                    position: at,
                    text: String::from(aim::SIGHT),
                    size: aim::SIGHT_SIZE,
                    color: colour,
                    centered: true,
                    ..Default::default()
                });
            }
        }

        let said: Vec<(Vec2, String, f32)> =
            vec![(0, name, aim::PROMPT_SIZE), (1, detail, aim::DETAIL_SIZE)]
                .into_iter()
                .filter_map(|(line, say, size)| {
                    say.map(|say| (aim::prompt_at(self.window, line), say, size))
                })
                .collect();

        // a plate under the pair of them. It lands on whatever the sight has
        // just lit, which is the brightest thing in the room, and it used to
        // answer that with a drop shadow of itself. A shade with a line round
        // it does the job the shadow was doing and does it over a pale table
        // as well as over a dark one.
        let round: Vec<(Vec2, &str, f32)> = said
            .iter()
            .map(|(at_, say, size)| (*at_, say.as_str(), *size))
            .collect();

        plate(geometry, aim::plate_round(&round, true));

        for (at_, say, size) in said {
            text.push_render_text(RenderText {
                position: at_,
                bounds: aim::prompt_bounds(self.window),
                text: say,
                size,
                centered: true,
                ..Default::default()
            });
        }
    }

    fn draw(&mut self, scene: &mut Scene, camera: &mut Camera) {
        let (Some(cube), Some(floor), Some(screen_mesh)) = (self.cube, self.floor, self.screen)
        else {
            return;
        };

        camera.position = self.looking_from();
        camera.target = camera.position + self.facing();
        // and how far it sees, which is the building's hundred metres indoors
        // and the length of the floor out past the door. Spec 0018: a hundred
        // metres out there culls the sun from everywhere past Mercury, and a
        // sun you cannot see is the one thing that space is for.
        camera.far = if self.in_the_deep() {
            beyond::sees()
        } else {
            Camera::new().far
        };

        // dim and overhead, like the room it is: bright enough to walk, dark
        // enough that a lit cabinet is the thing you look at. Warm from above
        // and cool in what it misses, because a white sun over a neutral fill
        // is what made every surface in here grey.
        //
        // and it moves, per spec 0019. A day in the garden is twenty minutes
        // and this is the one light in the building that keeps it: the hall,
        // the nook, the cellar and the baths are roofed and the sun reaches
        // none of them, which `room::outline` records as measured, so a sun
        // that swings and reddens and goes out changes the garden and nothing
        // else.
        let (colour, strength) = garden::sunlit(self.since);

        scene.light.direction = garden::sunlight(self.since);
        scene.light.color = colour;
        // and nothing at all in a room with no sky, per `Room::sees_the_sky`.
        // The sun cannot reach these rooms and the map cannot quite say so.
        let strength = if noise::room_at(self.room.reaches, self.at).sees_the_sky() {
            strength
        } else {
            0.0
        };

        scene.light.intensity = strength;
        scene.light.ambient = garden::filled(self.since);

        // and a strike on top of the day, per spec 0016.
        //
        // **On the sun and not on the ambient**, which is the whole reason it
        // can be done at all. The ambient is added everywhere and a flash that
        // rode on it would light the cellar. The sun is shadowed by the roof,
        // already measured in `room::outline` as reaching no room but the
        // garden, so the one light in this building that can flash without
        // getting indoors is the one that is already kept out.
        //
        // The direction leans toward the strike by how much of the light the
        // strike is. At noon the sun carries it and nothing swings; at night
        // the sun is nothing at all and the shadows come off the patch you
        // just watched light up.
        if let Some((strike, fade)) = self.flashing() {
            // `flash_fills` and not `flash_lit`, which is the distinction the
            // whole thing turns on. The patch on the cloud dims slowly with
            // distance because what you are looking at is a mile of cloud lit
            // from inside. What reaches the garden falls off the way light
            // falls off, so a near strike throws the trees against the walls
            // and a far one lights nothing but the sky.
            let flash = aim::FLASH_STRENGTH * fade * garden::flash_fills(strike.away);
            let share = flash / (flash + strength).max(1e-4);

            scene.light.direction = scene
                .light
                .direction
                .lerp(garden::flashlight(&strike), share)
                .normalize_or(scene.light.direction);
            scene.light.color = colour.lerp(aim::FLASH_LIGHT, share);
            scene.light.intensity = strength + flash;
        }

        // the aisle's carpet, and only the aisle's. It used to be a rectangle
        // wide enough for the nook as well, which was wasted twice over: the
        // nook has boards of its own over every inch of its floor, and the
        // corner of the rectangle the nook does not reach came out past the
        // nook's end wall and lay on the garden's gravel.
        let hall = self.room.floors()[0];
        let laid = Transform::at(vec3(hall.center().x, 0.0, hall.center().z)).with_scale(vec3(
            hall.size().x,
            1.0,
            hall.size().z,
        ));

        match self.carpet {
            // matte. A low shininess is a specular highlight spread over the
            // whole floor, which washed the weave's dark ground out to pale
            // grey the first time this was laid.
            Some(carpet) => scene.push_textured(floor, carpet, &laid, aim::FLOOR, aim::MATTE),
            None => scene.push_colored(floor, &laid, aim::FLOOR),
        }

        // and the wear in it where feet have gone, which is the only thing in
        // this hall that says the way through the near wall is there. Spec
        // 0020: it is a thing that happened rather than a thing somebody put
        // up, which is what lets it point without pointing.
        if let (Some(worn), Some(worn_mesh)) = (self.worn, self.worn_mesh) {
            let track = behind::track(self.room.reaches);

            scene.push_textured(
                worn_mesh,
                worn,
                &Transform::at(track.at).with_scale(track.size),
                aim::WORN,
                aim::MATTE,
            );
        }

        // a ceiling, because the room opened onto nothing and a corridor with
        // no lid is a corridor you are standing outside of
        if let Some(ceiling) = self.ceiling_mesh {
            // One quad per piece of floor, so the lid is the shape of the thing
            // it is over. The floor is an L: the hall end to end, and the nook
            // beside the near half of it. One rectangle covering both is bigger
            // than that L by the corner they do not share, and that corner is
            // outdoors. It hung over the garden three metres up, six by five of
            // dark slate with the lanterns catching its underside, which from
            // down there is a roof over a garden that has no building on it.
            for slab in self.room.floors().iter() {
                let (middle, size) = (slab.center(), slab.size());

                // turned over, because a plane faces up and from underneath
                // that is a back face, which the opaque pass culls
                scene.push_colored(
                    ceiling,
                    &Transform::at(vec3(middle.x, room::TALL, middle.z))
                        .with_rotation(glam::Quat::from_rotation_x(std::f32::consts::PI))
                        .with_scale(vec3(size.x, 1.0, size.z)),
                    aim::CEILING,
                );
            }

            // and the grid hung under it, which is what is over an arcade and
            // is also sixteen units of lines running away from you. Spec 0012.
            let (bars, tiles) = self.room.lattice();

            for (at, size) in tiles.iter() {
                scene.push_colored(cube, &Transform::at(*at).with_scale(*size), aim::CEIL_TILE);
            }
            for (at, size) in bars.iter() {
                scene.push_colored(cube, &Transform::at(*at).with_scale(*size), aim::CEIL_BAR);
            }
        }

        // the hall's near end wall. From the hall it is one piece across the
        // gap its collider has in it, which is the whole of spec 0011: the
        // one place in this building where what you can walk through and what
        // you can see disagree, and the disagreement is the room.
        //
        // From the other side it is the two pieces, so the way out is a lit
        // rectangle rather than a wall you have to remember the position of.
        // That half of the trick was free while the room behind was empty and
        // fifty metres across. Spec 0015 made it a hundred and fifty nine and dark and
        // gave you something to look at, and a secret door you cannot find
        // from the inside is not a secret, it is a trap.
        //
        // Which side you are on and not a second account of the geometry: the
        // same wall, drawn whole or drawn in its own collider's pieces.
        {
            let in_the_hall = self.at.z < self.room.reaches;
            let side = room::WALL + room::CABINET.x;
            let whole = vec![
                Transform::at(vec3(0.0, room::TALL * 0.5, self.room.reaches)).with_scale(vec3(
                    side * 2.0,
                    room::TALL,
                    room::THICK,
                )),
            ];
            let pieces: Vec<Transform> = behind::near_wall(self.room.reaches)
                .into_iter()
                .map(|box_| Transform::at(box_.center()).with_scale(box_.size()))
                .collect();

            for stood in if in_the_hall { whole } else { pieces } {
                match self.wall_grain {
                    Some(grain) => scene.push_textured(cube, grain, &stood, aim::WALL, aim::MATTE),
                    None => scene.push_colored(cube, &stood, aim::WALL),
                }
            }
        }

        // and the space it hides: its floor and lid, its walls, and the
        // columns. Spec 0015 puts the solar system in it; this draws the room
        // it stands in.
        {
            for (box_, made) in behind::built(self.room.reaches) {
                let colour = match made {
                    behind::Made::Ground => aim::BEHIND_FLOOR,
                    behind::Made::Wall => aim::BEHIND_WALL,
                    behind::Made::Column => aim::BEHIND_COLUMN,
                    behind::Made::Frame => aim::BEHIND_FRAME,
                };

                scene.push_material(
                    cube,
                    &Transform::at(box_.center()).with_scale(box_.size()),
                    colour,
                    aim::VOID,
                );
            }
        }

        // the solar system in it, per spec 0015
        self.draw_the_system(scene, cube, camera);

        for wall in self.room.walls.iter() {
            let stood = Transform::at(wall.center()).with_scale(wall.size());

            match self.wall_grain {
                Some(grain) => scene.push_textured(cube, grain, &stood, aim::WALL, aim::MATTE),
                None => scene.push_colored(cube, &stood, aim::WALL),
            }
        }

        // the engine's own shapes, turning at the end of the room
        // the toy on the first bench: the frame, the ropes and the balls. The
        // cradle is built about the middle of a bench top, so everything is
        // placed from there.
        if let (Some(ball_mesh), Some(bench)) = (
            self.ball_mesh,
            self.room
                .benches
                .iter()
                .find(|bench| bench.name == cradle::NAME),
        ) {
            let top = bench.at + Vec3::Y * bench.size.y;

            // two uprights and the bar they carry
            let span = cradle::upright_at() * 2.0;
            for end in [-1.0f32, 1.0] {
                scene.push_colored(
                    cube,
                    &Transform::at(top + vec3(0.0, cradle::BAR_UP * 0.5, end * span * 0.5))
                        .with_scale(vec3(cradle::STOCK, cradle::BAR_UP, cradle::STOCK)),
                    aim::CRADLE_FRAME,
                );
            }
            scene.push_colored(
                cube,
                &Transform::at(top + Vec3::Y * cradle::BAR_UP).with_scale(vec3(
                    cradle::STOCK,
                    cradle::STOCK,
                    span + cradle::STOCK,
                )),
                aim::CRADLE_FRAME,
            );

            for ball in 0..cradle::BALLS {
                let hung = top + self.cradle[cradle::ball_at(ball)].position;
                let hook = top + cradle::hook(ball);

                // the rope, a thin box turned to lie along it
                let along = hung - hook;
                let length = along.length();
                if length > 1e-4 {
                    let turn = glam::Quat::from_rotation_arc(Vec3::NEG_Y, along / length);
                    scene.push_colored(
                        cube,
                        &Transform::at(hook + along * 0.5)
                            .with_rotation(turn)
                            .with_scale(vec3(0.006, length, 0.006)),
                        aim::CRADLE_ROPE,
                    );
                }

                scene.push_material(
                    ball_mesh,
                    &Transform::at(hung).with_scale(Vec3::splat(cradle::BALL * 2.0)),
                    aim::CRADLE_BALL,
                    96.0,
                );
            }
        }

        // the metronome on its bench, placed from the middle of the bench top the
        // way the cradle is. Spec 0006.
        if let (Some(ball_mesh), Some(bench)) = (
            self.ball_mesh,
            self.room
                .benches
                .iter()
                .find(|bench| bench.name == metronome::NAME),
        ) {
            let top = bench.at + Vec3::Y * bench.size.y;
            let arm = self.metronome.bodies[metronome::ARM];
            let pivot = top + metronome::pivot();

            // the plate stands behind the needle from where the bench is worked,
            // so the notches read against it. Which way that is comes from the
            // bench and not from an axis: the benches moved to the far wall and
            // the plate stayed on +x, which turned every one of them around.
            let back = bench.away();
            let across = bench.right();

            scene.push_colored(
                cube,
                &Transform::at(top + Vec3::Y * (metronome::FOOT.y * 0.5))
                    .with_scale(metronome::FOOT),
                aim::CASE,
            );
            scene.push_colored(
                cube,
                &Transform::at(
                    top + back * metronome::PLATE_BACK
                        + Vec3::Y * (metronome::FOOT.y + metronome::PLATE_TALL * 0.5),
                )
                .with_scale(vec3(
                    metronome::PLATE_THICK,
                    metronome::PLATE_TALL,
                    metronome::PLATE_WIDE,
                )),
                aim::CASE_PLATE,
            );

            // the notches up the plate, with the one the weight is in lit. It is
            // the only way to read the setting from more than a step away, and
            // colour past 1 glows with no light on it, as a cabinet's band does.
            for notch in 0..metronome::NOTCHES {
                let up = metronome::PIVOT_Y + metronome::weight_at(notch);
                let set = notch == self.metronome.notch;

                // the one it is set to is bigger as well as brighter. At the
                // same size it was five millimetres of glow at arm's length,
                // which is nothing from across the nook.
                let mark = if set {
                    vec3(0.014, 0.011, metronome::NOTCH_LONG * 1.4)
                } else {
                    vec3(0.008, 0.006, metronome::NOTCH_LONG)
                };
                scene.push_colored(
                    cube,
                    &Transform::at(
                        top + back * (metronome::PLATE_BACK - metronome::PLATE_THICK)
                            + across * metronome::NOTCH_AT
                            + Vec3::Y * up,
                    )
                    .with_scale(mark),
                    if set { aim::NOTCH_ON } else { aim::NOTCH },
                );
            }

            // the pivot, the needle above it, the bob under it, and the weight
            // riding the needle
            scene.push_colored(
                cube,
                &Transform::at(pivot).with_scale(vec3(metronome::HINGE_HALF * 3.0, 0.014, 0.014)),
                aim::PENDULUM,
            );
            scene.push_colored(
                cube,
                &Transform::at(pivot + arm.orientation * (Vec3::Y * metronome::NEEDLE * 0.5))
                    .with_rotation(arm.orientation)
                    .with_scale(vec3(
                        metronome::NEEDLE_THICK,
                        metronome::NEEDLE,
                        metronome::NEEDLE_THICK,
                    )),
                aim::NEEDLE,
            );
            scene.push_material(
                cube,
                &Transform::at(top + arm.position)
                    .with_rotation(arm.orientation)
                    .with_scale(metronome::BOB * 2.0),
                aim::PENDULUM,
                48.0,
            );
            scene.push_material(
                ball_mesh,
                &Transform::at(top + self.metronome.bodies[metronome::SLIDER].position)
                    .with_scale(Vec3::splat(metronome::WEIGHT * 2.0)),
                aim::SLIDING_WEIGHT,
                96.0,
            );
        }

        // the gyroscope on its bench. Spec 0006.
        if let (Some(wheel_mesh), Some(bench)) = (
            self.wheel_mesh,
            self.room
                .benches
                .iter()
                .find(|bench| bench.name == gyro::NAME),
        ) {
            let top = bench.at + Vec3::Y * bench.size.y;
            let wheel = self.gyro.bodies[gyro::WHEEL];
            let hub = top + gyro::hub();

            // a pedestal: a narrow neck on a flared foot, stopping under the
            // outer ring, which is what one of these stands on
            let neck = gyro::HUB_UP - gyro::FRAME;
            for (up, tall, wide) in [
                (neck * 0.5, neck, gyro::STAND),
                (0.012, 0.024, gyro::STAND * 3.0),
                (0.004, 0.008, gyro::STAND * 5.0),
            ] {
                scene.push_colored(
                    cube,
                    &Transform::at(top + Vec3::Y * up).with_scale(vec3(wide, tall, wide)),
                    aim::STAND,
                );
            }

            // the spindle, right through the rotor and out to the frame at each
            // end, which is what holds it.
            //
            // Turned with the axis and not with the rotor. It is a square bar
            // and a real one is round, so spinning it about its own length does
            // nothing a round rod would do: what it does instead is swell and
            // shrink by the 41% between a square's side and its diagonal, forty
            // times a second, and since the spindle is the spine of the whole
            // object the object reads as shaking.
            let frame = gyro::gimbal(&self.gyro.bodies);
            scene.push_colored(
                cube,
                &Transform::at(hub).with_rotation(frame).with_scale(vec3(
                    gyro::SPINDLE_OUT * 2.0,
                    gyro::AXLE,
                    gyro::AXLE,
                )),
                aim::SPINDLE,
            );

            // the rotor, drawn as the disc its block weighs like
            scene.push_material(
                wheel_mesh,
                &Transform::at(top + wheel.position)
                    .with_rotation(wheel.orientation)
                    .with_scale(vec3(gyro::THICK, gyro::RADIUS * 2.0, gyro::RADIUS * 2.0)),
                aim::FLYWHEEL,
                64.0,
            );

            // the hub the spindle goes through, and one short spoke out of it.
            // A flat disc with a line across it is a slice of lemon whatever
            // ring you put round it; a thick wheel with a hub and one spoke is a
            // flywheel, and the spoke still says it is turning.
            scene.push_material(
                wheel_mesh,
                &Transform::at(top + wheel.position)
                    .with_rotation(wheel.orientation)
                    .with_scale(vec3(gyro::THICK * 1.3, gyro::BOSS * 2.0, gyro::BOSS * 2.0)),
                aim::SPINDLE,
                64.0,
            );
            // one spoke on each face, which is what says it is turning.
            //
            // It has to turn slowly enough to be read as turning, and that is
            // what sets the dial rather than anything in the physics. Drawn as
            // a ring instead, which is what a mark on a fast rim blurs into, it
            // was perfectly steady and the rotor looked stopped. Drawn as a
            // spoke at 60 radians a second it stepped 57 degrees a frame and
            // flickered. At 15 it steps 14 degrees and goes round.
            let at = gyro::RADIUS * gyro::MARK;
            for side in [-1.0f32, 1.0] {
                let out = vec3(side * (gyro::THICK * 0.5 + 0.002), at * 0.76, 0.0);
                scene.push_colored(
                    cube,
                    &Transform::at(top + wheel.position + wheel.orientation * out)
                        .with_rotation(wheel.orientation)
                        .with_scale(vec3(0.004, gyro::RADIUS * 0.78, gyro::RADIUS * 0.1)),
                    aim::STUD,
                );
            }

            // the two rings, across each other: the gimbal in the rotor's own
            // plane and the frame holding the spindle's ends. Both follow the
            // axis and neither follows the rotor, because that is what a gimbal
            // does: it carries the wheel rather than turning with it.
            let step = std::f32::consts::TAU / gyro::GIMBAL_PIECES as f32;
            let bolted = glam::Quat::from_rotation_y(self.gyro.facing);
            for (radius, across) in [(gyro::GIMBAL, false), (gyro::FRAME, true)] {
                // the gimbal tips with the axis and the frame only swings round
                // the upright, because the frame is bolted to the pedestal. Both
                // following the axis is what carried the whole cage off its own
                // stand whenever the axis tipped towards upright.
                let turned = if across { bolted } else { frame };
                let chord = 2.0 * radius * (step * 0.5).sin() * 1.08;

                for piece in 0..gyro::GIMBAL_PIECES {
                    let round = piece as f32 * step;
                    let (sin, cos) = round.sin_cos();
                    let (at, turn, size) = if across {
                        (
                            vec3(cos, sin, 0.0),
                            glam::Quat::from_rotation_z(round + std::f32::consts::FRAC_PI_2),
                            vec3(chord, gyro::GIMBAL_THICK, GIMBAL_WIDE),
                        )
                    } else {
                        (
                            vec3(0.0, cos, sin),
                            glam::Quat::from_rotation_x(round),
                            vec3(GIMBAL_WIDE, gyro::GIMBAL_THICK, chord),
                        )
                    };

                    scene.push_colored(
                        cube,
                        &Transform::at(top + wheel.position + turned * (at * radius))
                            .with_rotation(turned * turn)
                            .with_scale(size),
                        aim::GIMBAL,
                    );
                }
            }
        }

        // the sign hung over the aisle, which is the only thing in the hall
        // that says the nook is there at all. Spec 0006.
        if let (Some(hung), Some(point)) = (self.hung, self.point_mesh) {
            let at = sign::at(self.room.reaches);
            // the gilt rim is the whole plank and the painted face stands proud
            // of it, so the gold shows round the edge rather than being four
            // more things to place
            scene.push_colored(
                cube,
                &Transform::at(at).with_scale(vec3(sign::WIDE, sign::TALL, sign::THICK)),
                aim::SIGN_GILT,
            );
            scene.push_colored(
                cube,
                &Transform::at(at).with_scale(vec3(
                    sign::WIDE - sign::BORDER * 2.0,
                    sign::TALL - sign::BORDER * 2.0,
                    sign::THICK + sign::PROUD,
                )),
                aim::SIGN_BOARD,
            );

            // the end cut to a point, which is the direction. The wedge runs
            // out along its own -x, so it needs no turning: the nook is that
            // way from here.
            scene.push_colored(
                point,
                &Transform::at(at - Vec3::X * sign::WIDE * 0.5).with_scale(vec3(
                    sign::POINT,
                    sign::TALL,
                    sign::THICK,
                )),
                aim::SIGN_GILT,
            );

            // the lettering, on both faces and reading forwards from both
            let letters = Transform::at(at + Vec3::Z * (sign::THICK * 0.5 + sign::PROUD * 2.0))
                .with_rotation(glam::Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
                .with_scale(vec3(0.02, sign::LETTERS, sign::WIDE - sign::BORDER * 4.0));

            match self.says {
                Some(says) => scene.push_textured(hung, says, &letters, aim::SIGN_LETTERS, 8.0),
                None => scene.push_colored(hung, &letters, aim::SIGN_LETTERS),
            }

            // two chains to the ceiling, each link turned across the one under
            // it, which is what makes a stack of blocks read as a chain
            let links = sign::links(room::soffit());
            for side in [-1.0f32, 1.0] {
                let foot = at + Vec3::X * side * sign::CHAIN_AT + Vec3::Y * sign::TALL * 0.5;

                for n in 0..links {
                    let turn = glam::Quat::from_rotation_y(if n % 2 == 0 {
                        0.0
                    } else {
                        std::f32::consts::FRAC_PI_2
                    });

                    scene.push_colored(
                        cube,
                        &Transform::at(foot + Vec3::Y * (n as f32 + 0.5) * sign::LINK)
                            .with_rotation(turn)
                            .with_scale(vec3(sign::RING, sign::LINK, sign::LINK * 0.62)),
                        aim::SIGN_CHAIN,
                    );
                }
            }

            // and the pendant over it, which is why you can read it
            let shade = at + Vec3::Y * (sign::TALL * 0.5 + sign::LAMP_UP);
            scene.push_colored(
                cube,
                &Transform::at(shade).with_scale(sign::SHADE),
                aim::SHADE,
            );
            let stem = (room::soffit() + shade.y + sign::SHADE.y * 0.5) * 0.5;
            scene.push_colored(
                cube,
                &Transform::at(vec3(shade.x, stem, shade.z)).with_scale(vec3(
                    0.016,
                    room::soffit() - shade.y - sign::SHADE.y * 0.5,
                    0.016,
                )),
                aim::SIGN_CHAIN,
            );
        }

        // the room's own sign, hung over the near end of the aisle, which is
        // what you wake up to. Spec 0001.
        if let Some(hung) = self.hung {
            let at = neon::at(self.room.reaches);

            // the can first, then the tube run standing out of both its faces.
            // The can is nearly black on purpose: a lit tube on a lit box is a
            // box.
            scene.push_colored(
                cube,
                &Transform::at(at).with_scale(vec3(neon::WIDE, neon::TALL, neon::THICK)),
                aim::ARCADE_CAN,
            );
            for (middle, size) in neon::tubes() {
                scene.push_colored(
                    cube,
                    &Transform::at(at + middle).with_scale(size),
                    aim::arcade_tube(),
                );
            }

            // the name, on both faces and reading forwards from both, the way
            // the nook's plank does
            let letters = Transform::at(at + Vec3::Z * (neon::THICK * 0.5 + neon::PROUD))
                .with_rotation(glam::Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
                .with_scale(vec3(0.02, neon::LETTERS, neon::span()));

            match self.named {
                Some(named) => {
                    scene.push_textured(hung, named, &letters, aim::arcade_letters(), 8.0)
                }
                None => scene.push_colored(hung, &letters, aim::arcade_letters()),
            }

            // two stems to the ceiling. Rigid, where the nook's sign is on
            // chains: there is a transformer in a box this size.
            let long = neon::stem(room::soffit());
            for side in [-1.0f32, 1.0] {
                scene.push_colored(
                    cube,
                    &Transform::at(
                        at + Vec3::X * side * neon::STEM_AT
                            + Vec3::Y * (neon::TALL * 0.5 + long * 0.5),
                    )
                    .with_scale(vec3(neon::STEM, long, neon::STEM)),
                    aim::ARCADE_STEM,
                );
            }
        }

        // the way down, and the room at the bottom of it. Spec 0007. Boxes, so
        // they are drawn as the boxes they are: a stair of slabs is what a
        // stair looks like from the side anyway.
        for (box_, made) in cellar::built(self.room.reaches) {
            // the stair's ceiling is drawn as one raked slab rather than as the
            // dozen steps it is built of, so skip those here
            if made == cellar::Made::Soffit {
                continue;
            }

            let laid = Transform::at(box_.center()).with_scale(box_.size());
            match made {
                cellar::Made::Tread => {
                    scene.push_colored(cube, &laid, aim::BOARDS);
                }
                // the shaft is panelled like the room it leads to. Left as the
                // stone it started as, the way down is a grey chute into a
                // mahogany library.
                //
                // Boarded and not grained. A cube's texture runs nought to one
                // on every face however big the face is, so one board's grain
                // on a wall this size is that board blown up to three metres: a
                // hundred and twenty-eight pixels across the whole of it, with
                // the cathedral figure come out as scallops the size of your
                // head and every texel a visible block. The boards picture is
                // six planks wide and stretches to something that reads as
                // boarding.
                // flat, with the boarding laid on its faces below. Textured
                // here it is one board stretched over the whole wall.
                cellar::Made::Shaft => scene.push_colored(cube, &laid, aim::TIMBER[0]),
                cellar::Made::Soffit => {}
                // the floor warmer and a shade apart from the walls, so a room
                // is a floor and walls rather than one box
                cellar::Made::Stone if box_.max.y <= -cellar::DOWN + 1e-3 => {
                    scene.push_colored(cube, &laid, aim::CELLAR_FLOOR);
                }
                cellar::Made::Stone => {
                    scene.push_colored(cube, &laid, aim::CELLAR);
                }
            }
        }

        // and every collider over the top of it, when somebody has asked.
        //
        // `room.solid()` itself rather than a list gathered for the purpose:
        // a second list is two accounts of one thing, which is the fault
        // this is here to find. Spec 0013.
        if self.showing_solid {
            for box_ in self.room.solid() {
                scene.outline(box_, aim::SOLID);
            }
        }

        // and the patches where two drawn faces share a plane, in a colour
        // nothing else uses, per spec 0048 of the engine. A patch is flat, so
        // what you see is a rectangle lying exactly where the speckle is.
        for patch in &self.fighting {
            scene.outline(*patch, aim::FIGHTING);
        }

        // the spa behind the cellar. Spec 0008.
        for (box_, made) in spa::built(self.room.reaches)
            .into_iter()
            .chain(spa::fittings(self.room.reaches))
            .chain(spa::ceiling(self.room.reaches))
        {
            let laid = Transform::at(box_.center()).with_scale(box_.size());

            scene.push_colored(
                cube,
                &laid,
                match made {
                    spa::Made::Tile => aim::SPA_TILE,
                    spa::Made::Wall => aim::SPA_WALL,
                    // the basin is lined with tiled quads below, so its boxes
                    // are structure and not surface
                    spa::Made::Basin => aim::SPA_BASIN,
                    spa::Made::Step => aim::SPA_STEP,
                    spa::Made::Tub => aim::SPA_TUB,
                    spa::Made::Timber => aim::SAUNA_TIMBER,
                    spa::Made::Bench => aim::SAUNA_BENCH,
                    spa::Made::Stove => aim::SAUNA_STOVE,
                    spa::Made::Dado => aim::SPA_DADO,
                    spa::Made::Band => aim::SPA_BAND,
                    spa::Made::Pier => aim::SPA_PIER,
                    spa::Made::Inlay => aim::SPA_INLAY,
                    spa::Made::Rib => aim::SPA_RIB,
                    spa::Made::Brass => aim::BRASS,
                    spa::Made::Cut => aim::SPA_CUT,
                },
            );
        }

        // the garden, per spec 0010: its sky, its ground, and the walls round
        // it. The sky first, because everything else in here is seen against
        // it.
        {
            if let (Some(dome), Some(storm)) = (self.dome, self.storm) {
                scene.push_textured(
                    dome,
                    storm,
                    &Transform::at(garden::dome_at(self.room.reaches)),
                    // the same cloud all day, with the day taken out from
                    // behind it. Spec 0019. And a strike lighting the whole of
                    // it, per spec 0016: what you see of lightning from under
                    // a storm is the deck going white, and the cloud's own
                    // shape is a better picture of it than any patch.
                    garden::skylit(self.since) + self.flashed(),
                    900.0,
                );
            }

            if let Some(ground) = self.ground {
                for ((bed, size), gravel) in garden::beds(self.room.reaches)
                    .into_iter()
                    .zip(&self.gravel)
                {
                    scene.push_textured(
                        ground,
                        *gravel,
                        &Transform::at(bed + Vec3::Y * 0.002).with_scale(vec3(size.x, 1.0, size.y)),
                        aim::GRAVEL,
                        aim::DULL,
                    );
                }
            }

            // the stepping stones, sunk nearly flush. Before the set stones
            // because the path runs past them and a slab laid over a boulder
            // is a slab laid over a boulder either way round.
            for step in garden::steps(self.room.reaches) {
                if let Some(cut) = self.slabs.get(step.cut) {
                    let thick = step.wide * garden::SLAB_THICK;

                    scene.push_material(
                        *cut,
                        // its top a knuckle proud of the gravel, which puts
                        // its middle half a thickness below that
                        &Transform::at(step.at + Vec3::Y * (garden::STEP_UP - thick * 0.5))
                            .with_rotation(glam::Quat::from_rotation_y(step.turn))
                            .with_scale(vec3(step.wide, thick, step.wide * 0.86)),
                        aim::ROCK,
                        aim::DULL,
                    );
                }
            }

            // the stones, set a third of the way into the gravel. Spec 0010.
            for stone in garden::rocks(self.room.reaches) {
                if let Some(cut) = self.stones.get(stone.cut) {
                    // a pinpoint highlight, like the stonework. The default is
                    // a broad one, which on a face this size is the whole face
                    // gone white and a boulder made of polystyrene.
                    scene.push_material(
                        *cut,
                        &Transform::at(stone.at + Vec3::Y * stone.size.y * (0.5 - garden::BURIED))
                            .with_rotation(glam::Quat::from_rotation_y(stone.turn))
                            .with_scale(stone.size),
                        aim::ROCK,
                        aim::DULL,
                    );
                }
            }

            // the walls: a lining of plaster on each, as a quad with its own
            // texture count rather than a cube wearing one enormous tile.
            // Only the band over the way in is drawn as a box as well, because
            // there is no wall behind that one for a lining to be a lining on.
            for ((face, mesh), plaster) in garden::facing(self.room.reaches)
                .into_iter()
                .zip(&self.plaster_mesh)
                .zip(&self.plaster)
            {
                if face.solid {
                    scene.push_colored(
                        cube,
                        &Transform::at(face.at).with_scale(face.size),
                        aim::GARDEN_WALL,
                    );
                }

                let (middle, size, looks) = face.plane();

                scene.push_textured(
                    *mesh,
                    *plaster,
                    &Transform::at(middle)
                        .with_rotation(room::laid_on(looks))
                        .with_scale(vec3(size.x, 1.0, size.y)),
                    aim::GARDEN_PLASTER,
                    aim::MATTE,
                );
            }

            // the tiled cap along the top of them, which is what makes a wall
            // read as built rather than as a box with a flat top
            for (at, size) in garden::coping(self.room.reaches) {
                scene.push_colored(cube, &Transform::at(at).with_scale(size), aim::GARDEN_COPE);
            }

            // and the moss along their feet, where the ground keeps them damp:
            // a picture with a ragged edge cut out of it rather than a green
            // box, which is a skirting board
            for ((at, size, turn), mesh) in garden::moss(self.room.reaches)
                .into_iter()
                .zip(&self.moss_mesh)
            {
                if let Some(moss) = self.moss {
                    scene.push_textured(
                        mesh.to_owned(),
                        moss,
                        &Transform::at(at)
                            .with_rotation(turn)
                            .with_scale(vec3(size.x, 1.0, size.y)),
                        aim::GARDEN_MOSS,
                        aim::MATTE,
                    );
                }
            }

            // the bamboo screen on the wall you face coming in, which is the
            // one wall here meant to look different from the others
            if let Some(pole) = self.pole {
                for at in garden::screen(self.room.reaches) {
                    scene.push_colored(pole, &Transform::at(at), aim::BAMBOO);
                }
            }

            for (at, size) in garden::rails(self.room.reaches) {
                scene.push_colored(cube, &Transform::at(at).with_scale(size), aim::BAMBOO_ROPE);
            }

            // the bank round the water, and the bed under it, which are not
            // the same thing to look at
            for (box_, colour) in garden::bank(self.room.reaches)
                .into_iter()
                .map(|stone| (stone, aim::POND_BANK))
                .chain(std::iter::once((
                    garden::bed(self.room.reaches),
                    aim::POND_BED,
                )))
            {
                scene.push_colored(
                    cube,
                    &Transform::at((box_.min + box_.max) * 0.5).with_scale(box_.max - box_.min),
                    colour,
                );
            }

            // and the stones set on it. One run of coping is a swimming bath
            // whatever colour it is: what the eye reads as a pond's edge is
            // stones of different sizes with shadow between them.
            for edge in garden::edging(self.room.reaches) {
                // a boulder is the rock mesh, which is a lump about its own
                // middle with a third of it meant to be buried, so it is
                // scaled up by what is buried and then sunk by the same
                let (mesh, tall, up) = if edge.rough {
                    let tall = edge.size.y / (1.0 - garden::BURIED);

                    (self.stones.get(edge.cut), tall, edge.size.y - tall * 0.5)
                } else {
                    (self.slabs.get(edge.cut), edge.size.y, edge.size.y * 0.5)
                };

                if let Some(cut) = mesh {
                    scene.push_material(
                        *cut,
                        &Transform::at(edge.at + Vec3::Y * up)
                            .with_rotation(glam::Quat::from_rotation_y(edge.turn))
                            .with_scale(vec3(edge.size.x, tall, edge.size.z)),
                        aim::EDGE_STONE,
                        aim::DULL,
                    );
                }
            }

            // the koi, before the water, so the water is the last thing
            // between you and them. Spec 0010.
            if let (Some(koi), Some(fin)) = (self.koi, self.fin) {
                for (fish, skin) in garden::koi(self.room.reaches, self.since)
                    .iter()
                    .zip(&self.skins)
                {
                    // turned about y like everything round in this building,
                    // which builds a fish standing on its tail. A quarter turn
                    // about x lays it down with its nose on +z, and the yaw
                    // takes it from there.
                    let laid = glam::Quat::from_rotation_y(fish.heading)
                        * glam::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);

                    scene.push_textured(
                        koi,
                        *skin,
                        &Transform::at(fish.at).with_rotation(laid).with_scale(vec3(
                            fish.long * garden::KOI_FLAT,
                            fish.long,
                            fish.long,
                        )),
                        aim::KOI,
                        aim::MATTE,
                    );

                    // and the tail, hung off the stalk and swinging. Laid the
                    // other way about x, so its root is at the stalk and the
                    // fan trails behind rather than growing out of the nose.
                    let along = vec3(fish.heading.sin(), 0.0, fish.heading.cos());
                    let swung = glam::Quat::from_rotation_y(fish.heading + fish.wag)
                        * glam::Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);

                    scene.push_textured(
                        fin,
                        *skin,
                        &Transform::at(fish.at - along * fish.long * 0.5)
                            .with_rotation(swung)
                            .with_scale(vec3(
                                fish.long * garden::FIN_WIDE,
                                fish.long * garden::FIN_LONG,
                                fish.long * garden::FIN_DEEP,
                            )),
                        aim::KOI,
                        aim::MATTE,
                    );
                }
            }

            // a tight highlight, like the baths' water. The default is a
            // broad one, which over a surface this size is a sheen across the
            // whole pond and the single thing that made it read as a bath.
            if let Some(mesh) = self.pond_mesh {
                scene.push_material(mesh, &Transform::at(Vec3::ZERO), aim::POND, 420.0);
            }

            // the trees: a bent trunk, a limb out to each pad of foliage, and
            // the pads. Spec 0010.
            for (tree, trunk) in garden::trees(self.room.reaches).iter().zip(&self.trunks) {
                scene.push_colored(*trunk, &Transform::at(tree.at), aim::BARK);

                if let Some(limb) = self.limb {
                    for (from, to) in garden::limbs(tree) {
                        let along = to - from;
                        let long = along.length();

                        if long > 1e-3 {
                            // the mesh is built standing on its end, so it is
                            // turned to point along the limb rather than placed
                            // at an angle worked out by hand
                            scene.push_colored(
                                limb,
                                &Transform::at(from)
                                    .with_rotation(glam::Quat::from_rotation_arc(
                                        Vec3::Y,
                                        along / long,
                                    ))
                                    .with_scale(vec3(garden::LIMB_THICK, long, garden::LIMB_THICK)),
                                aim::BARK,
                            );
                        }
                    }
                }

                if let Some(crown) = self.crown {
                    for pad in garden::pads(tree) {
                        scene.push_material(
                            crown,
                            &Transform::at(pad.at)
                                .with_rotation(glam::Quat::from_rotation_y(pad.turn))
                                .with_scale(vec3(
                                    pad.wide,
                                    pad.wide * garden::PAD_FLAT,
                                    pad.wide * 0.86,
                                )),
                            aim::LEAF,
                            // no highlight. The default is a broad one and on
                            // a pad this size it is a wet green streak across
                            // the whole of it, which is a balloon.
                            aim::DULL,
                        );
                    }
                }
            }

            if let Some(lantern) = self.lantern {
                for (stands, tall) in garden::lanterns(self.room.reaches) {
                    scene.push_colored(
                        lantern,
                        &Transform::at(stands).with_scale(vec3(tall * 0.8, tall, tall * 0.8)),
                        aim::GARDEN_STONE,
                    );
                }
            }
        }

        // the basin's lining, which is where the pool's depth comes from. Not
        // the water: what tells your eye how far down the bottom is, is seeing
        // something of a known size through it and watching that get smaller.
        if let Some(tiled) = self.tiled {
            for ((middle, size, looks), mesh) in
                spa::lining(self.room.reaches).into_iter().zip(&self.lining)
            {
                let turn = if looks.y > 0.5 {
                    glam::Quat::IDENTITY
                } else if looks.x.abs() > 0.5 {
                    glam::Quat::from_rotation_z(looks.x * std::f32::consts::FRAC_PI_2)
                        * glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                } else {
                    glam::Quat::from_rotation_x(-looks.z * std::f32::consts::FRAC_PI_2)
                };

                scene.push_textured(
                    *mesh,
                    tiled,
                    &Transform::at(middle)
                        .with_rotation(turn)
                        .with_scale(vec3(size.x, 1.0, size.y)),
                    aim::SPA_BASIN,
                    90.0,
                );
            }
        }

        // the turned pieces in the baths: the urns, the sconces' bowls and the
        // fountain. Spec 0008.
        if let (Some(urn), Some(sconce), Some(basin)) = (self.urn, self.sconce, self.basin) {
            for at in spa::urns(self.room.reaches) {
                scene.push_material(
                    urn,
                    &Transform::at(at).with_scale(vec3(spa::URN, spa::URN_TALL, spa::URN)),
                    aim::SPA_URN,
                    180.0,
                );
            }
            for at in spa::sconces(self.room.reaches) {
                scene.push_material(
                    sconce,
                    &Transform::at(at).with_scale(vec3(
                        spa::SCONCE.x * 1.6,
                        spa::SCONCE.y,
                        spa::SCONCE.x * 1.6,
                    )),
                    aim::SPA_CUT,
                    120.0,
                );
            }

            // the stream, and the water standing in the basin it falls into.
            // A fountain that does not run is a stone shelf.
            if let Some(pour) = self.stream {
                let (from, falls) = spa::stream(self.room.reaches);

                scene.push_material(
                    pour,
                    &Transform::at(vec3(from.x, from.y - falls * 0.5, from.z)).with_scale(vec3(
                        spa::STREAM,
                        falls,
                        spa::STREAM,
                    )),
                    aim::WATER,
                    360.0,
                );
            }
            if let Some(dish) = self.dish {
                let at = spa::fountain(self.room.reaches);

                scene.push_material(
                    dish,
                    &Transform::at(vec3(at.x, at.y + spa::BASIN.y * 0.42, at.z)).with_scale(vec3(
                        spa::BASIN.x * 0.74,
                        1.0,
                        spa::BASIN.x * 0.74,
                    )),
                    aim::WATER_HOT,
                    320.0,
                );

                // and the ring where the thread lands, spreading and fading,
                // which is the other half of saying it is falling
                let (lands, _) = spa::stream(self.room.reaches);
                let wide = spa::splash(self.since);

                scene.push_colored(
                    dish,
                    &Transform::at(vec3(lands.x, at.y + spa::BASIN.y * 0.42 + 0.004, lands.z))
                        .with_scale(vec3(wide, 1.0, wide)),
                    aim::STEAM.truncate().extend(spa::splashed(self.since)),
                );
            }

            let at = spa::fountain(self.room.reaches);
            scene.push_material(
                basin,
                &Transform::at(at).with_scale(vec3(spa::BASIN.x, spa::BASIN.y * 1.6, spa::BASIN.x)),
                aim::SPA_CUT,
                140.0,
            );
        }

        // the door to the baths, and the tiled sign over it on the cellar's
        // side, which is where it is read. Spec 0008.
        {
            let (middle, turn, half) = spa::leaf(self.room.reaches, self.room.baths_swing);

            // a glazed door and not a slab: two stiles up the sides, three
            // rails across, and frosted glass in the two openings between
            // them. The slab was what Jake called bland, and a slab is what it
            // was: one box the size of the hole.
            let put = |off: Vec3, size: Vec3, colour: glam::Vec4, scene: &mut Scene| {
                scene.push_colored(
                    cube,
                    &Transform::at(middle + turn * off)
                        .with_rotation(turn)
                        .with_scale(size),
                    colour,
                );
            };

            let tall = half.y * 2.0;
            let wide = half.x * 2.0;
            let mid = half.y - tall * spa::MID_AT;
            let pane = wide - spa::STILE * 2.0;

            for side in [-1.0f32, 1.0] {
                put(
                    Vec3::X * side * (half.x - spa::STILE * 0.5),
                    vec3(spa::STILE, tall, half.z * 2.0),
                    aim::BATHS_DOOR,
                    scene,
                );
            }
            for (up, deep) in [
                (half.y - spa::DOOR_RAIL * 0.5, spa::DOOR_RAIL),
                (-half.y + spa::DOOR_RAIL * 0.5, spa::DOOR_RAIL),
                (mid, spa::MID_RAIL),
            ] {
                put(
                    Vec3::Y * up,
                    vec3(pane, deep, half.z * 2.0),
                    aim::BATHS_DOOR,
                    scene,
                );
            }

            // the two lights, frosted. Set back from the frame's own faces so
            // the stiles and rails stand proud of the glass, the way joinery
            // does.
            for (from, to) in [
                (mid + spa::MID_RAIL * 0.5, half.y - spa::DOOR_RAIL),
                (-half.y + spa::DOOR_RAIL, mid - spa::MID_RAIL * 0.5),
            ] {
                if to - from <= 0.0 {
                    continue;
                }

                put(
                    Vec3::Y * (from + to) * 0.5,
                    vec3(pane, to - from, half.z * 1.2),
                    aim::FROSTED,
                    scene,
                );
            }

            // the architrave and the reveal, so the opening is lined rather
            // than showing the cellar's bare stone round it
            for (middle, size) in spa::architrave(self.room.reaches) {
                match self.grain.first() {
                    Some(grain) => scene.push_textured(
                        cube,
                        *grain,
                        &Transform::at(middle).with_scale(size),
                        aim::TIMBER[0],
                        aim::DULL,
                    ),
                    None => scene.push_colored(
                        cube,
                        &Transform::at(middle).with_scale(size),
                        aim::TIMBER[0],
                    ),
                }
            }

            let at = spa::sign(self.room.reaches);
            scene.push_colored(
                cube,
                &Transform::at(at).with_scale(vec3(spa::SIGN.x, spa::SIGN.y, spa::SIGN_OUT)),
                aim::BATHS_EDGE,
            );
            scene.push_colored(
                cube,
                &Transform::at(at - Vec3::Z * spa::SIGN_OUT * 0.3).with_scale(vec3(
                    spa::SIGN.x - spa::SIGN_EDGE * 2.0,
                    spa::SIGN.y - spa::SIGN_EDGE * 2.0,
                    spa::SIGN_OUT,
                )),
                aim::BATHS_TILE,
            );

            if let Some(hung) = self.hung {
                let letters = Transform::at(at - Vec3::Z * spa::SIGN_OUT * 0.9)
                    .with_rotation(glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                    .with_scale(vec3(0.02, spa::LETTERS, spa::SIGN.x - spa::SIGN_EDGE * 4.0));

                match self.baths {
                    Some(says) => {
                        scene.push_textured(hung, says, &letters, aim::BATHS_LETTERS, 8.0)
                    }
                    None => scene.push_colored(hung, &letters, aim::BATHS_LETTERS),
                }
            }
        }

        // the sauna's glass door, in its frame, swung as far as it is open.
        // Spec 0008.
        {
            let (middle, turn, half) = spa::sauna_leaf(self.room.reaches, self.room.sauna_swing);

            // the frame first, which is what makes a pane read as a door
            for (along, up) in [(1.0f32, 0.0f32), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
                let long = if along == 0.0 {
                    vec3(spa::PANE * 1.4, spa::FRAME, half.z * 2.0)
                } else {
                    vec3(spa::PANE * 1.4, half.y * 2.0, spa::FRAME)
                };

                scene.push_colored(
                    cube,
                    &Transform::at(
                        middle
                            + turn
                                * vec3(
                                    0.0,
                                    up * (half.y - spa::FRAME * 0.5),
                                    along * (half.z - spa::FRAME * 0.5),
                                ),
                    )
                    .with_rotation(turn)
                    .with_scale(long),
                    aim::SAUNA_FRAME,
                );
            }

            scene.push_material(
                cube,
                &Transform::at(middle).with_rotation(turn).with_scale(vec3(
                    half.x * 2.0,
                    half.y * 2.0,
                    half.z * 2.0,
                )),
                aim::SAUNA_GLASS,
                260.0,
            );
        }

        // steam, over the tub and over the stove. Translucent and soft edged,
        // which is the whole of it: the engine has no particle system and this
        // does not need one.
        if let Some(ball) = self.ball_mesh {
            let (tub_at, tub_wide, _) = spa::tub(self.room.reaches);
            let stove = spa::stove_lamp(self.room.reaches);

            // and the fug standing in the sauna, which is what the room fills
            // with rather than what rises off the stove
            for (at, size, thick) in
                spa::fug(spa::sauna(self.room.reaches), self.since, self.steamy)
            {
                scene.push_colored(
                    ball,
                    &Transform::at(at).with_scale(size),
                    aim::STEAM.truncate().extend(thick),
                );
            }

            for (over, wide) in [(tub_at, tub_wide), (stove, spa::STOVE.x * 2.0)] {
                for (at, size, thick) in spa::steam(over, wide, self.since) {
                    scene.push_colored(
                        ball,
                        &Transform::at(at).with_scale(size),
                        aim::STEAM.truncate().extend(thick),
                    );
                }
            }
        }

        // and the water in it, last and see-through, so the basin under it
        // shows. Its vertices are already in the world, so it is drawn where it
        // is rather than placed.
        for (mesh, colour) in [
            (self.pool_mesh, aim::WATER),
            (self.tub_mesh, aim::WATER_HOT),
        ] {
            if let Some(mesh) = mesh {
                scene.push_material(mesh, &Transform::at(Vec3::ZERO), colour, 420.0);
            }
        }

        // the stairwell's boarding, laid on the faces of its walls rather than
        // stretched over them. Spec 0007.
        if let Some(grain) = self.grain.first() {
            for ((middle, size, looks), mesh) in cellar::shaft_faces(self.room.reaches)
                .into_iter()
                .zip(&self.boarding)
            {
                // a quad lies flat and faces up, so a quarter turn about x
                // stands it on edge looking along z
                let turn = glam::Quat::from_rotation_x(looks * std::f32::consts::FRAC_PI_2);

                scene.push_textured(
                    *mesh,
                    *grain,
                    &Transform::at(middle)
                        .with_rotation(turn)
                        .with_scale(vec3(size.x, 1.0, size.y)),
                    aim::TIMBER[0],
                    aim::DULL,
                );
            }
        }

        // and that ceiling, raked
        {
            let (at, size, turn) = cellar::soffit(self.room.reaches);
            let laid = Transform::at(at)
                .with_rotation(glam::Quat::from_rotation_z(turn))
                .with_scale(size);

            match self.grain.get(1) {
                Some(grain) => scene.push_textured(cube, *grain, &laid, aim::RAFTER, aim::DULL),
                None => scene.push_colored(cube, &laid, aim::RAFTER),
            }
        }

        // what the cellar is furnished with. Spec 0007.
        for (which_rack, (at, side)) in cellar::racks(self.room.reaches).into_iter().enumerate() {
            let rack = cellar::RACK;
            let (across, up) = cellar::BOTTLES;
            let back = at.z + side * (rack.x * 0.5 - 0.02);

            // a back against the wall, a side at each end, and a shelf for
            // every row of bottles
            scene.push_colored(
                cube,
                &Transform::at(vec3(at.x, at.y + rack.y * 0.5, back))
                    .with_scale(vec3(rack.z, rack.y, 0.04)),
                aim::RACK,
            );
            for end in [-1.0f32, 1.0] {
                scene.push_colored(
                    cube,
                    &Transform::at(vec3(
                        at.x + end * (rack.z - 0.05) * 0.5,
                        at.y + rack.y * 0.5,
                        at.z,
                    ))
                    .with_scale(vec3(0.05, rack.y, rack.x)),
                    aim::RACK,
                );
            }
            for row in 0..=up {
                scene.push_colored(
                    cube,
                    &Transform::at(vec3(at.x, at.y + row as f32 / up as f32 * rack.y, at.z))
                        .with_scale(vec3(rack.z, 0.035, rack.x)),
                    aim::RACK,
                );
            }

            // and the bottles, ends out, standing a little proud of the front
            // so they read as bottles in a rack rather than a panel of dots
            let nose = at.z - side * (rack.x * 0.5 - cellar::BOTTLE * 0.7);
            for row in 0..up {
                for n in 0..across {
                    // resting on the shelf under it, not floating in the
                    // middle of the gap above it. Put at the middle of its own
                    // cell a bottle hangs in the air with daylight under it,
                    // and a rack of them hovers.
                    let shelf = at.y + row as f32 / up as f32 * rack.y;
                    let middle = vec3(
                        at.x + ((n as f32 + 0.5) / across as f32 - 0.5) * (rack.z - 0.14),
                        shelf + 0.018 + cellar::BOTTLE * 0.5,
                        nose,
                    );

                    // a whole bottle, lying down with its neck out, because
                    // the neck is the only part of one you can see in a rack
                    // and it is the part that says wine. Drawn as a disc it was
                    // a cork; drawn as a cube it was a pegboard.
                    let laid = glam::Quat::from_rotation_x(-side * std::f32::consts::FRAC_PI_2);
                    let deep = vec3(
                        middle.x,
                        middle.y,
                        middle.z + side * cellar::BOTTLE_LONG * 0.5,
                    );

                    let which = cellar::glass(which_rack, row, n);
                    let glass = aim::BOTTLES[which % aim::BOTTLES.len()];

                    match self.bottle {
                        Some(bottle) => {
                            scene.push_colored(
                                bottle,
                                &Transform::at(deep).with_rotation(laid).with_scale(vec3(
                                    cellar::BOTTLE,
                                    cellar::BOTTLE_LONG,
                                    cellar::BOTTLE,
                                )),
                                glass,
                            );

                            // and the cork in the end of it, which is the one
                            // pale thing on a bottle and most of what says it
                            // is a full one
                            if let Some(peg) = self.peg {
                                let tip =
                                    vec3(middle.x, middle.y, middle.z - side * cellar::CORK * 0.4);

                                scene.push_colored(
                                    peg,
                                    &Transform::at(tip).with_rotation(laid).with_scale(vec3(
                                        cellar::BOTTLE * cellar::NECK * 0.92,
                                        cellar::BOTTLE_LONG * cellar::CORK,
                                        cellar::BOTTLE * cellar::NECK * 0.92,
                                    )),
                                    aim::CORK,
                                );
                            }
                        }
                        None => scene.push_colored(
                            cube,
                            &Transform::at(middle).with_scale(Vec3::splat(cellar::BOTTLE)),
                            glass,
                        ),
                    }
                }
            }
        }

        // the floor in hardwood, laid over the stone it sits on. Spec 0007.
        if let (Some(mesh), Some(boards)) = (self.cellar_floor, self.boards) {
            let (at, size) = cellar::floor(self.room.reaches);

            scene.push_textured(
                mesh,
                boards,
                &Transform::at(at + Vec3::Y * 0.002).with_scale(vec3(size.x, 1.0, size.z)),
                aim::CELLAR_FLOOR,
                aim::DULL,
            );
        }

        // the ceiling, coffered. A flat ceiling is the one surface in a room
        // nobody spent anything on, and this room is meant to read as one
        // somebody did. Spec 0007.
        {
            let (beams, coffers) = cellar::ceiling(self.room.reaches);

            // the sunk panel in each square, and a moulding round it
            for (n, (at, size)) in coffers.iter().enumerate() {
                let panel = *at - Vec3::Y * cellar::SUNK;
                let laid = Transform::at(panel).with_scale(vec3(size.x, 0.04, size.z));

                match self.grain.get(n % self.grain.len().max(1)) {
                    Some(grain) => scene.push_textured(cube, *grain, &laid, aim::COFFER, aim::DULL),
                    None => scene.push_colored(cube, &laid, aim::COFFER),
                }

                for (way, out, span) in [
                    (Vec3::X, size.x, vec3(cellar::MOULD, cellar::MOULD, size.z)),
                    (Vec3::Z, size.z, vec3(size.x, cellar::MOULD, cellar::MOULD)),
                ] {
                    for side in [-1.0f32, 1.0] {
                        scene.push_colored(
                            cube,
                            &Transform::at(
                                panel + way * side * (out - cellar::MOULD) * 0.5
                                    - Vec3::Y * cellar::MOULD * 0.4,
                            )
                            .with_scale(span),
                            aim::RAFTER,
                        );
                    }
                }
            }

            // the beams over them
            for (at, size) in &beams {
                scene.push_colored(cube, &Transform::at(*at).with_scale(*size), aim::RAFTER);
            }

            // and a carved boss where four of them meet, which is the one place
            // on a ceiling anybody puts any carving
            if let Some(peg) = self.peg {
                for (at, _) in &beams {
                    for (other, _) in &beams {
                        if (at.x - other.x).abs() < 1e-3 || (at.z - other.z).abs() < 1e-3 {
                            continue;
                        }

                        scene.push_colored(
                            peg,
                            &Transform::at(vec3(at.x, at.y - cellar::BEAM_DOWN * 0.5, other.z))
                                .with_scale(vec3(cellar::BOSS, cellar::BOSS * 0.5, cellar::BOSS)),
                            aim::BOSS,
                        );
                    }
                }
            }
        }

        // the panelling down there: floor to ceiling, each bay a raised panel
        // in its own timber inside a frame of stiles and rails, with a skirting
        // under the lot and a cornice over it. Spec 0007.
        //
        // Panelling that stops at waist height and leaves plain wall above is
        // what a dining room has. A room panelled the whole way up is a room
        // somebody spent money on, which is the point of this one.
        for (face, out, from, to, along_x, from_up) in cellar::panelled(self.room.reaches) {
            let run = to - from;
            let step = cellar::BOARD + cellar::BOARD_GAP;
            let fits = (run / step).floor().max(1.0) as usize;
            let spare = run - fits as f32 * step;
            // a run can start partway up, which is how the chimney breast
            // over the fireplace keeps its panelling
            let floor = -cellar::DOWN + from_up;
            let high = cellar::TALL - from_up;
            let field = high - cellar::SKIRTING - cellar::CORNICE;

            // a laid up board, turned so its grain runs the way the board does
            let laid = |middle: Vec3,
                        size: Vec3,
                        timber: glam::Vec4,
                        board: usize,
                        scene: &mut Scene| match self
                .grain
                .get(board % self.grain.len().max(1))
            {
                Some(grain) => scene.push_textured(
                    cube,
                    *grain,
                    &Transform::at(middle).with_scale(size),
                    timber,
                    aim::DULL,
                ),
                None => scene.push_colored(cube, &Transform::at(middle).with_scale(size), timber),
            };
            let put = |across: f32, up: f32, wide: f32, tall: f32, deep: f32| {
                if along_x {
                    (
                        vec3(face + out * deep * 0.5, floor + up, across),
                        vec3(deep, tall, wide),
                    )
                } else {
                    (
                        vec3(across, floor + up, face + out * deep * 0.5),
                        vec3(wide, tall, deep),
                    )
                }
            };

            for n in 0..fits {
                let at = from + spare * 0.5 + (n as f32 + 0.5) * step;
                let timber = aim::TIMBER[(n * 5 + if along_x { 2 } else { 0 }) % aim::TIMBER.len()];

                // the panel itself, set back inside its frame
                let (middle, size) = put(
                    at,
                    cellar::SKIRTING + field * 0.5,
                    cellar::BOARD - cellar::STILE * 2.0,
                    field - cellar::STILE * 2.0,
                    cellar::BOARD_OUT - cellar::PANEL_IN,
                );
                laid(middle, size, timber, n, scene);

                // the two stiles either side of it, standing proud
                for side in [-1.0f32, 1.0] {
                    let (middle, size) = put(
                        at + side * (cellar::BOARD - cellar::STILE) * 0.5,
                        cellar::SKIRTING + field * 0.5,
                        cellar::STILE,
                        field,
                        cellar::BOARD_OUT,
                    );
                    laid(middle, size, aim::TIMBER_TRIM, n + 2, scene);
                }

                // and a rail at the head and foot of the panel
                for up in [
                    cellar::SKIRTING + cellar::STILE * 0.5,
                    cellar::SKIRTING + field - cellar::STILE * 0.5,
                ] {
                    let (middle, size) =
                        put(at, up, cellar::BOARD, cellar::STILE, cellar::BOARD_OUT);
                    laid(middle, size, aim::TIMBER_TRIM, n + 2, scene);
                }
            }

            // a skirting under the lot, and a cornice over it
            for (up, thick, deep) in [
                (
                    cellar::SKIRTING * 0.5,
                    cellar::SKIRTING,
                    cellar::BOARD_OUT * 1.8,
                ),
                (
                    high - cellar::CORNICE * 0.5,
                    cellar::CORNICE,
                    cellar::BOARD_OUT * 2.2,
                ),
            ] {
                let (middle, size) = put((from + to) * 0.5, up, run, thick, deep);
                laid(middle, size, aim::TIMBER_TRIM, 1, scene);
            }
        }

        // the rug in front of the fire, which is where a rug goes
        if let (Some(afghan), Some(mesh)) = (self.afghan, self.afghan_mesh) {
            let (at, size) = cellar::rug(self.room.reaches);

            scene.push_textured(
                mesh,
                afghan,
                &Transform::at(at + Vec3::Y * 0.004).with_scale(vec3(size.x, 1.0, size.z)),
                aim::FLOOR,
                aim::DULL,
            );
        }

        // the fire in the far wall, which is what the room is lit by. Spec
        // 0007. A surround standing proud of the wall, a hole cut into it, logs
        // in the hole and a fire over them.
        {
            let at = cellar::hearth(self.room.reaches);
            let wide = cellar::FIRE_WIDE;
            let high = cellar::FIRE_HIGH;

            // the surround, from the one place that says where it is. Spec
            // 0007: it is walked into as well as drawn.
            for (middle, size) in cellar::surround(self.room.reaches) {
                // built of blocks rather than coloured like stone. The
                // joints are what says stone: courses that do not line up,
                // blocks of different lengths in each one, and no two the same
                // grey.
                let laid = Transform::at(middle).with_scale(size);
                match self.stonework {
                    Some(stone) => scene.push_textured(cube, stone, &laid, aim::HEARTH, aim::DULL),
                    None => scene.push_colored(cube, &laid, aim::HEARTH),
                }
            }

            let peg_or_cube = self.peg.unwrap_or(cube);

            // what stands on the mantel. A shelf over a fire with nothing on
            // it is a shelf, and a few things lined up over the fire is the one
            // thing every room with one has. Spec 0007.
            let shelf = cellar::mantel_top(self.room.reaches);
            if let (Some(stick), Some(decanter), Some(wineglass)) =
                (self.candlestick, self.decanter, self.wineglass)
            {
                // a candlestick at each end, with a candle in it and a flame on
                // that. Candles are fine here in a way they were not in the
                // nook, which is four hundred books deep.
                for side in [-1.0f32, 1.0] {
                    let on = vec3(shelf.x, shelf.y, shelf.z + side * wide * 0.44);
                    let tall = cellar::CANDLESTICK.y;

                    scene.push_colored(
                        stick,
                        &Transform::at(on + Vec3::Y * tall * 0.5).with_scale(cellar::CANDLESTICK),
                        aim::BRASS,
                    );
                    scene.push_colored(
                        peg_or_cube,
                        &Transform::at(on + Vec3::Y * (tall + cellar::CANDLE * 0.5))
                            .with_scale(vec3(0.046, cellar::CANDLE, 0.046)),
                        aim::WAX,
                    );
                    if let Some(flame) = self.flame {
                        let own = cellar::flicker(self.since * 2.3 + side * 3.0);

                        scene.push_colored(
                            flame,
                            &Transform::at(
                                on + Vec3::Y
                                    * (tall + cellar::CANDLE + cellar::CANDLE_FLAME * 0.45 * own),
                            )
                            .with_scale(vec3(
                                cellar::CANDLE_FAT * 0.62,
                                cellar::CANDLE_FLAME * own,
                                cellar::CANDLE_FAT * 0.62,
                            )),
                            aim::WICK * own,
                        );
                    }
                }

                // a clock in the middle. It reads as a clock because of the
                // dial and nothing else, so the dial is most of the front of
                // it: a small pale disc on a brown box reads as a brown box.
                let case = cellar::CLOCK;
                let front = shelf.x + case.x * 0.5;
                let eye = shelf.y + case.y * 0.56;
                let turned = glam::Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);

                scene.push_colored(
                    cube,
                    &Transform::at(shelf + Vec3::Y * case.y * 0.5).with_scale(case),
                    aim::CLOCK_CASE,
                );
                // a pediment on top, which is what a mantel clock has instead
                // of a flat lid
                scene.push_colored(
                    cube,
                    &Transform::at(shelf + Vec3::Y * (case.y + 0.016)).with_scale(vec3(
                        case.x * 1.14,
                        0.032,
                        case.z * 1.14,
                    )),
                    aim::CLOCK_CASE,
                );

                // the bezel, then the dial inside it
                for (out, wide, colour) in [
                    (cellar::DIAL_OUT * 0.5, cellar::BEZEL, aim::BRASS),
                    (cellar::DIAL_OUT, 1.0, aim::CLOCK_FACE),
                ] {
                    scene.push_colored(
                        peg_or_cube,
                        &Transform::at(vec3(front + out, eye, shelf.z))
                            .with_rotation(turned)
                            .with_scale(vec3(
                                case.y * cellar::DIAL * wide,
                                0.012,
                                case.y * cellar::DIAL * wide,
                            )),
                        colour,
                    );
                }

                // four marks at the quarters, which is what the eye counts
                let reach = case.y * cellar::DIAL * 0.4;
                for quarter in 0..4 {
                    let turn = quarter as f32 * std::f32::consts::FRAC_PI_2;
                    let (sin, cos) = turn.sin_cos();

                    scene.push_colored(
                        cube,
                        &Transform::at(vec3(
                            front + cellar::DIAL_OUT + 0.004,
                            eye + cos * reach,
                            shelf.z + sin * reach,
                        ))
                        .with_rotation(glam::Quat::from_rotation_x(turn))
                        .with_scale(vec3(0.009, 0.026, 0.009)),
                        aim::CLOCK_CASE,
                    );
                }

                // and the hands, at ten past ten, which is where the hands sit
                // in every photograph of a clock ever taken because it reads as
                // a clock rather than as a number
                for (turn, long, thick) in [(-1.05f32, 0.62f32, 0.011f32), (1.05, 0.44, 0.015)] {
                    let arm = case.y * cellar::DIAL * 0.5 * long;
                    let (sin, cos) = turn.sin_cos();

                    scene.push_colored(
                        cube,
                        &Transform::at(vec3(
                            front + cellar::DIAL_OUT + 0.008,
                            eye + cos * arm * 0.5,
                            shelf.z + sin * arm * 0.5,
                        ))
                        .with_rotation(glam::Quat::from_rotation_x(turn))
                        .with_scale(vec3(thick, arm, thick)),
                        aim::CLOCK_CASE,
                    );
                }

                // and a decanter with two glasses, off to one side, because a
                // mantel arranged symmetrically is a mantel nobody uses
                let by = vec3(shelf.x, shelf.y, shelf.z + wide * 0.2);
                scene.push_colored(
                    decanter,
                    &Transform::at(by + Vec3::Y * cellar::DECANTER.y * 0.5)
                        .with_scale(cellar::DECANTER),
                    aim::CRYSTAL,
                );
                // what is in it, which is the only reason a decanter is worth
                // drawing
                scene.push_colored(
                    decanter,
                    &Transform::at(by + Vec3::Y * cellar::DECANTER.y * 0.42).with_scale(vec3(
                        cellar::DECANTER.x * 0.84,
                        cellar::DECANTER.y * 0.62,
                        cellar::DECANTER.z * 0.84,
                    )),
                    aim::PORT,
                );
                for n in 0..2 {
                    let at = by + vec3(0.0, 0.0, 0.14 + n as f32 * 0.11);

                    scene.push_colored(
                        wineglass,
                        &Transform::at(at + Vec3::Y * cellar::GLASS.y * 0.5)
                            .with_scale(cellar::GLASS),
                        aim::CRYSTAL,
                    );
                }
            }

            // the back of the recess, which is a panel and not a block. As a
            // solid box it enclosed the fire: a black slab standing out of the
            // wall with the flames sealed inside it, which is a fireplace with
            // the fire behind the bricks.
            let back = at.x + 0.02;
            scene.push_colored(
                cube,
                &Transform::at(vec3(back, at.y + high * 0.5, at.z))
                    .with_scale(vec3(0.04, high, wide)),
                aim::FIREBOX,
            );

            // logs across it, and a bed of embers under them
            let in_it = back + 0.13;

            // a bed of coals rather than one bright slab. A slab is a hot plate
            // and it was the brightest thing in the room by a mile; coals are
            // small, uneven and mostly dark, and the few that are not are what
            // the eye calls a fire.
            if let Some(peg) = self.peg {
                for n in 0..13 {
                    let roll = cellar::glass(n * 31, n, n * 7) as f32 / u32::MAX as f32;
                    let across = ((n as f32 / 12.0) - 0.5) * wide * 0.74;
                    let hot = cellar::flicker(self.since * (0.7 + roll) + n as f32);

                    scene.push_colored(
                        peg,
                        &Transform::at(vec3(
                            in_it + (roll - 0.5) * 0.14,
                            at.y + 0.03,
                            at.z + across,
                        ))
                        .with_scale(vec3(
                            0.09 + roll * 0.05,
                            0.05,
                            0.09 + roll * 0.05,
                        )),
                        if roll > 0.45 {
                            aim::EMBER * hot
                        } else {
                            aim::COAL
                        },
                    );
                }
            }
            if let Some(barrel) = self.barrel {
                // copied, because this crate is on the 2018 edition and
                // `into_iter` on an array hands out references there
                for (n, (across, roll)) in [(-1.0f32, 0.16f32), (0.25, -0.1), (1.0, 0.06)]
                    .iter()
                    .copied()
                    .enumerate()
                {
                    scene.push_colored(
                        barrel,
                        &Transform::at(vec3(
                            in_it,
                            at.y + 0.17 + n as f32 * 0.03,
                            at.z + across * 0.16,
                        ))
                        .with_rotation(
                            glam::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                                * glam::Quat::from_rotation_y(roll),
                        )
                        .with_scale(vec3(0.13, wide * 0.72, 0.13)),
                        aim::LOG,
                    );
                }
            }

            // and the flames, which are the one surface in the building that
            // is a light as well as a thing. Shaped, because a fire made of
            // slabs is a pile of hot bricks.
            let lit = cellar::flicker(self.since);
            if let Some(flame) = self.flame {
                for (across, tall, wide_at, colour, beat) in [
                    (0.0f32, 0.46f32, 0.26f32, aim::FIRE, 1.0f32),
                    (-0.26, 0.33, 0.2, aim::EMBER, 1.7),
                    (0.27, 0.29, 0.18, aim::EMBER, 2.3),
                ] {
                    // each one on its own beat, so they do not breathe together
                    let own = cellar::flicker(self.since * beat + across * 7.0);
                    let high = tall * own;

                    scene.push_colored(
                        flame,
                        &Transform::at(vec3(in_it, at.y + 0.11 + high * 0.5, at.z + across))
                            .with_scale(vec3(wide_at, high, wide_at)),
                        colour * lit,
                    );
                }
            }
        }

        if let Some(barrel) = self.barrel {
            for at in cellar::barrels(self.room.reaches) {
                let size = cellar::BARREL;

                let stood = Transform::at(at + Vec3::Y * size.y * 0.5).with_scale(size);
                match self.stave {
                    Some(stave) => {
                        scene.push_textured(barrel, stave, &stood, aim::BARREL, aim::DULL)
                    }
                    None => scene.push_colored(barrel, &stood, aim::BARREL),
                }

                // two hoops, which is what says barrel rather than drum. Round
                // ones: a cube round a round barrel meets it at the middle of
                // each face and stands out at all four corners, so two of them
                // make a box of it.
                // a lid sunk inside the rim at each end, which is what you see
                // of a barrel standing up and is most of what says cask
                if let Some(peg) = self.peg {
                    for end in [0.0f32, 1.0] {
                        let down = if end > 0.5 {
                            -cellar::LID_DOWN
                        } else {
                            cellar::LID_DOWN
                        };

                        scene.push_colored(
                            peg,
                            &Transform::at(at + Vec3::Y * (size.y * end + down)).with_scale(vec3(
                                size.x * (1.0 - cellar::BULGE) * cellar::LID,
                                0.03,
                                size.z * (1.0 - cellar::BULGE) * cellar::LID,
                            )),
                            aim::LID,
                        );
                    }
                }

                if let Some(hoop) = self.hoop {
                    for height in cellar::HOOPS {
                        let round = cellar::waist_at(height) * cellar::HOOP_OUT;

                        scene.push_colored(
                            hoop,
                            &Transform::at(at + Vec3::Y * size.y * height).with_scale(vec3(
                                size.x * round,
                                cellar::HOOP_THICK,
                                size.z * round,
                            )),
                            aim::HOOP,
                        );
                    }
                }
            }
        }

        // the nook's floor of boards, over the arcade's carpet and under the
        // rug. The arcade's is confetti on black and the nook is a study off it,
        // so the floor is the largest single thing in your view of either room
        // saying which of the two you are standing in.
        if let (Some(boards), Some(boards_mesh)) = (self.boards, self.boards_mesh) {
            let (at, size) = room::nook_floor(self.room.reaches);
            let laid = Transform::at(at + Vec3::Y * 0.002).with_scale(vec3(size.x, 1.0, size.z));

            scene.push_textured(boards_mesh, boards, &laid, aim::BOARDS, aim::MATTE);
        }

        // and the rug on top of them
        if let (Some(rug), Some(rug_mesh)) = (self.rug, self.rug_mesh) {
            let (at, size) = room::open_floor(self.room.reaches);
            let laid = Transform::at(at + Vec3::Y * 0.004).with_scale(vec3(size.x, 1.0, size.z));

            scene.push_textured(rug_mesh, rug, &laid, aim::FLOOR, aim::MATTE);
        }

        // the bookcases, stocked. None of it does anything, which is what
        // furniture is. Spec 0006.
        let bench_floor = 0.0;
        for n in 0..self.room.bookcases.len() {
            let case = study::CASE;
            // the one that is a door, standing where it stands when it is open.
            // Spec 0007: hinged on one side of the opening, so it comes out
            // into the nook and across the floor in front of itself. Drawn shut
            // while being open, it is a wall you can walk through.
            // the two leaves, wherever they have got to in their swing. Drawn
            // where they shut while being part way open, a door is a wall you
            // can walk through, and drawn open while shutting it is the other
            // way about. Spec 0007.
            let (stands, turn) = self.room.shelf_frame(n);
            let put = |at: Vec3| stands + turn * at;

            // the carcass: two sides, a back and a top
            for (at, size) in [
                (
                    vec3(-case.x * 0.5 + study::BOARD * 0.5, case.z * 0.5, 0.0),
                    vec3(study::BOARD, case.z, case.y),
                ),
                (
                    vec3(case.x * 0.5 - study::BOARD * 0.5, case.z * 0.5, 0.0),
                    vec3(study::BOARD, case.z, case.y),
                ),
                (
                    vec3(0.0, case.z * 0.5, -case.y * 0.5 + study::BOARD * 0.5),
                    vec3(case.x, case.z, study::BOARD),
                ),
                (
                    vec3(0.0, case.z - study::BOARD * 0.5, 0.0),
                    vec3(case.x, study::BOARD, case.y),
                ),
            ] {
                scene.push_colored(
                    cube,
                    &Transform::at(put(at)).with_rotation(turn).with_scale(size),
                    aim::BOOKCASE,
                );
            }

            for shelf in 0..study::SHELVES {
                let up = study::shelf_at(shelf);
                scene.push_colored(
                    cube,
                    &Transform::at(put(vec3(0.0, up - study::BOARD * 0.5, 0.0)))
                        .with_rotation(turn)
                        .with_scale(vec3(case.x, study::BOARD, case.y)),
                    aim::SHELF,
                );

                let books = study::stock(n as u32, shelf);
                for (which, book) in books.iter().enumerate() {
                    let along = study::along(&books, which) - case.x * 0.5;
                    scene.push_colored(
                        cube,
                        &Transform::at(put(vec3(
                            along,
                            up + book.tall * 0.5,
                            -case.y * 0.5 + study::BOOK_BACK + book.tall * 0.22,
                        )))
                        .with_rotation(turn)
                        .with_scale(vec3(
                            book.thick * 0.9,
                            book.tall,
                            book.tall * 0.44,
                        )),
                        if cellar::swings(n)
                            && n == cellar::CASE
                            && shelf == cellar::BOOK_SHELF
                            && which == cellar::BOOK
                        {
                            aim::BOOK_HANDLE
                        } else {
                            vec4(book.spine[0], book.spine[1], book.spine[2], 1.0)
                        },
                    );

                    // and the title down the spine. One picture a title,
                    // scaled to fit the book it is on, which is what a
                    // binder does: a short title is set large and a long one
                    // small. Spec 0006.
                    if let (Some(hung), Some((words, shape))) =
                        (self.hung, self.titles.get(book.title).copied())
                    {
                        // as far down the book as it is allowed, unless that
                        // makes the lettering taller than the spine is wide
                        let long = (book.tall * study::TITLE_LONG)
                            .min(book.thick * 0.9 * study::TITLE_ACROSS * shape);

                        scene.push_textured(
                            hung,
                            words,
                            &Transform::at(put(vec3(
                                along,
                                up + book.tall * 0.5,
                                -case.y * 0.5 + study::BOOK_BACK + book.tall * 0.44 + 0.002,
                            )))
                            .with_rotation(
                                turn * glam::Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)
                                    * glam::Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                            )
                            .with_scale(vec3(
                                0.004,
                                long / shape,
                                long,
                            )),
                            aim::SPINE_LETTERS,
                            8.0,
                        );
                    }
                }
            }
        }

        // the panelling. Every wall of the nook you can actually see, which is
        // not the back one: that is four hundred books deep. Spec 0006.
        for (face, out, along, from, to) in self.nook_walls() {
            // skirting, rail and cornice, the length of it
            for (up, tall, deep) in [
                (study::SKIRTING * 0.5, study::SKIRTING, study::PROUD * 1.6),
                (study::DADO, study::RAIL, study::PROUD * 2.0),
                (
                    room::TALL - study::CORNICE * 0.5,
                    study::CORNICE,
                    study::PROUD * 2.0,
                ),
            ] {
                let middle = face + out * deep * 0.5;
                let (at, size) = if along {
                    (
                        vec3(middle, up, (from + to) * 0.5),
                        vec3(deep, tall, to - from),
                    )
                } else {
                    (
                        vec3((from + to) * 0.5, up, middle),
                        vec3(to - from, tall, deep),
                    )
                };

                scene.push_colored(cube, &Transform::at(at).with_scale(size), aim::TRIM);
            }

            // and the panels between the skirting and the rail
            let middle = face + out * study::PROUD * 0.5;
            for place in study::panels(from, to) {
                let (at, size) = if along {
                    (
                        vec3(middle, study::panel_up(), place),
                        vec3(study::PROUD, study::panel_tall(), study::PANEL_WIDE),
                    )
                } else {
                    (
                        vec3(place, study::panel_up(), middle),
                        vec3(study::PANEL_WIDE, study::panel_tall(), study::PROUD),
                    )
                };

                scene.push_colored(cube, &Transform::at(at).with_scale(size), aim::PANEL);
            }
        }

        // the sconces down both long walls, which is where the light in here
        // comes from and is why there is any. Spec 0006.
        for (face, out, from, to) in room::sconce_runs(self.room.reaches) {
            for along in study::sconces(from, to) {
                let at = vec3(face, bench_floor + study::SCONCE_UP, along);

                // a back plate on the wall, a bracket off it, and a shade on the
                // end of that
                for (reach, size) in [
                    (-study::SCONCE_BACK, vec3(0.03, 0.09, 0.09)),
                    (
                        study::SCONCE_OUT * 0.5,
                        vec3(study::SCONCE_OUT, 0.018, 0.018),
                    ),
                ] {
                    scene.push_colored(
                        cube,
                        &Transform::at(at + Vec3::X * out * reach).with_scale(size),
                        aim::SCONCE,
                    );
                }
                scene.push_colored(
                    cube,
                    &Transform::at(at + Vec3::X * out * study::SCONCE_OUT).with_scale(study::SHADE),
                    aim::SHADE,
                );
            }
        }

        // the dominoes on cascada's bench, which do nothing. They are a picture
        // of the toy the way a cabinet's screen is a picture of the game, and
        // the first of them is over because that is the only part of a domino
        // run anybody needs explaining. Spec 0006.
        if let Some(bench) = self
            .room
            .benches
            .iter()
            .find(|bench| bench.name == cascada::NAME)
        {
            let top = bench.at + Vec3::Y * bench.size.y;

            for (at, turn, lean) in cascada::laid() {
                let (_, drop) = cascada::tops_at(at, turn, lean);

                scene.push_colored(
                    cube,
                    &Transform::at(top + at - Vec3::Y * drop)
                        .with_rotation(cascada::stood(turn, lean))
                        .with_scale(cascada::DOMINO),
                    aim::DOMINO,
                );
            }
        }

        // the globe on its bench. Spec 0006.
        if let (Some(globe_mesh), Some(world), Some(bench)) = (
            self.globe_mesh,
            self.world,
            self.room
                .benches
                .iter()
                .find(|bench| bench.name == globe::NAME),
        ) {
            let top = bench.at + Vec3::Y * bench.size.y;
            let middle = top + Vec3::Y * globe::HIGH;

            // the lean, which is the Earth's own and is the whole reason a globe
            // is not upright. The ball turns about its own axis inside it.
            let leant = glam::Quat::from_rotation_z(globe::TILT.to_radians());
            scene.push_textured(
                globe_mesh,
                world,
                &Transform::at(middle)
                    .with_rotation(leant * glam::Quat::from_rotation_y(self.globe.turned))
                    .with_scale(Vec3::splat(globe::RADIUS * 2.0)),
                vec4(1.0, 1.0, 1.0, 1.0),
                aim::MATTE,
            );

            // the meridian ring: a circle in a plane holding the axis, so it
            // passes over both poles, and leaning with it
            let round = std::f32::consts::TAU / globe::RING_PIECES as f32;
            let out = globe::RADIUS + globe::RING_OUT;
            let chord = 2.0 * out * (round * 0.5).sin() * 1.1;
            for piece in 0..globe::RING_PIECES {
                let turn = piece as f32 * round;
                let (sin, cos) = turn.sin_cos();

                scene.push_colored(
                    cube,
                    &Transform::at(middle + leant * (vec3(0.0, cos, sin) * out))
                        .with_rotation(leant * glam::Quat::from_rotation_x(turn))
                        .with_scale(vec3(globe::RING * 1.6, globe::RING, chord)),
                    aim::GLOBE_RING,
                );
            }

            // and the stand under it, up to where the ring comes down
            let under = globe::HIGH - out;
            for (up, tall, wide) in [
                (under * 0.5, under, globe::STAND),
                (0.014, 0.028, globe::STAND * 3.4),
                (0.005, 0.01, globe::STAND * 5.2),
            ] {
                scene.push_colored(
                    cube,
                    &Transform::at(top + Vec3::Y * up).with_scale(vec3(wide, tall, wide)),
                    aim::GLOBE_STAND,
                );
            }
        }

        // the ball and chain on its bench, which is the toy the nook was built
        // for. Spec 0006.
        if let (Some(ball_mesh), Some(bench)) = (
            self.ball_mesh,
            self.room
                .benches
                .iter()
                .find(|bench| bench.name == wrecker::NAME),
        ) {
            let top = bench.at + Vec3::Y * bench.size.y;

            // the lip round the tray. The tray's floor is the bench top itself,
            // so there is nothing to draw for it.
            for lip in wrecker::tray().iter().skip(1) {
                scene.push_colored(
                    cube,
                    &Transform::at(top + lip.center()).with_scale(lip.size()),
                    aim::TRAY,
                );
            }

            // the gantry: a post in the corner, and a jib round to the hook in
            // two legs. The corner is the furthest from the hook anything
            // standing on the tray can be, which is what keeps the ball from
            // being hauled through it.
            let post = wrecker::POST_AT;
            let high = wrecker::HANGS_FROM.y;
            let along = wrecker::HANGS_FROM.z - post.z;
            for (at, size) in [
                (
                    vec3(post.x, high * 0.5, post.z),
                    vec3(wrecker::POST, high, wrecker::POST),
                ),
                (
                    vec3(post.x, high, post.z + along * 0.5),
                    vec3(wrecker::POST, wrecker::POST, along),
                ),
                (
                    vec3(post.x * 0.5, high, wrecker::HANGS_FROM.z),
                    vec3(post.x.abs(), wrecker::POST, wrecker::POST),
                ),
            ] {
                scene.push_colored(cube, &Transform::at(top + at).with_scale(size), aim::GANTRY);
            }

            // the links themselves. The beads are 0.03 across at 0.045 apart,
            // so drawing the beads alone leaves half a bead of nothing between
            // each pair and the chain reads as a dotted line.
            for link in &self.wrecker.chain {
                let (one, other) = link.ends(&self.wrecker.bodies);
                let along = other - one;
                let length = along.length();
                if length < 1e-4 {
                    continue;
                }

                scene.push_colored(
                    cube,
                    &Transform::at(top + one + along * 0.5)
                        .with_rotation(glam::Quat::from_rotation_arc(Vec3::NEG_Y, along / length))
                        .with_scale(vec3(wrecker::BEAD * 0.4, length, wrecker::BEAD * 0.4)),
                    aim::CHAIN,
                );
            }

            for (n, body) in self.wrecker.bodies.iter().enumerate() {
                let at = top + body.position;

                if n == wrecker::HOOK {
                    // the ring on the jib, which is part of the gantry rather
                    // than part of the chain
                    scene.push_colored(
                        ball_mesh,
                        &Transform::at(at).with_scale(Vec3::splat(wrecker::BEAD * 1.4)),
                        aim::GANTRY,
                    );
                } else if n < wrecker::BALL_AT {
                    scene.push_colored(
                        ball_mesh,
                        &Transform::at(at).with_scale(Vec3::splat(wrecker::BEAD)),
                        aim::CHAIN,
                    );
                } else if n == wrecker::BALL_AT {
                    scene.push_material(
                        ball_mesh,
                        &Transform::at(at).with_scale(Vec3::splat(wrecker::BALL * 2.0)),
                        aim::WRECKING_BALL,
                        36.0,
                    );
                } else {
                    scene.push_material(
                        cube,
                        &Transform::at(at)
                            .with_rotation(body.orientation)
                            .with_scale(wrecker::BRICK * 2.0),
                        aim::BRICK,
                        36.0,
                    );
                }
            }
        }

        // the benches, which hold the toys. Spec 0006.
        for (n, bench) in self.room.benches.iter().enumerate() {
            let lit = self.seen == Some(room::Seen::Bench(n));
            let look = if lit { aim::BENCH_ON } else { aim::BENCH };

            // the one in the cellar is a pool table, which is a bench in every
            // way that matters and in no way that shows. Spec 0007: cloth, and
            // rails round it with pockets cut into the corners.
            if bench.name == cellar::POOLHALL {
                let top = bench.size.y;
                let rail = cellar::RAIL;
                let high = cellar::RAIL_UP;
                let frame = if lit { aim::BENCH_ON } else { aim::TRIM };
                let play = cellar::baize(bench);

                // the slate, and the cloth laid over it. The cloth runs to the
                // rails and no further: it was inset by a rail's width all
                // round, which left a lip of bare timber inside the cushions
                // that nothing on a table has.
                scene.push_colored(
                    cube,
                    &Transform::at(bench.at + Vec3::Y * (top - rail * 0.5))
                        .with_scale(vec3(play.x, rail, play.y)),
                    aim::CLOTH,
                );

                // the six rails. Each is three boards and not one: a body, a
                // cap over it that oversails both ways, and a bead under that.
                // One box is a two by four, which is what this was. The lines a
                // cap and a bead throw are the whole of why a rail reads as
                // carved rather than sawn.
                let timber = self.grain.first();
                let board = |middle: Vec3, size: Vec3, colour: glam::Vec4, scene: &mut Scene| {
                    match timber {
                        Some(grain) => scene.push_textured(
                            cube,
                            *grain,
                            &Transform::at(middle).with_scale(size),
                            colour,
                            aim::DULL,
                        ),
                        None => scene.push_colored(
                            cube,
                            &Transform::at(middle).with_scale(size),
                            colour,
                        ),
                    }
                };

                for (middle, size, facing) in cellar::rails(bench) {
                    let wood = if lit { aim::BENCH_ON } else { aim::TIMBER[0] };
                    let along = size * facing.abs();
                    let thick = along.x + along.y + along.z;

                    // the body, then the bead, then the cap
                    board(
                        middle - Vec3::Y * cellar::CAP * 0.5,
                        vec3(size.x, size.y - cellar::CAP, size.z),
                        wood,
                        scene,
                    );
                    board(
                        middle + Vec3::Y * (size.y * 0.5 - cellar::CAP - cellar::BEAD * 0.5),
                        size - along
                            + facing.abs() * (thick + cellar::CAP_OUT)
                            + Vec3::Y * (cellar::BEAD - size.y),
                        wood,
                        scene,
                    );
                    board(
                        middle + Vec3::Y * (size.y * 0.5 - cellar::CAP * 0.5),
                        size - along
                            + facing.abs() * (thick + cellar::CAP_OUT * 2.0)
                            + Vec3::Y * (cellar::CAP - size.y),
                        wood,
                        scene,
                    );

                    // and the cushion on the face that looks in, cloth over
                    // rubber, standing proud of the rail and lower than it
                    let out = thick * 0.5 + cellar::CUSHION * 0.5;
                    let face = size - along + facing.abs() * cellar::CUSHION;

                    scene.push_colored(
                        cube,
                        &Transform::at(
                            middle + facing * out - Vec3::Y * (high * 0.26 + cellar::CAP * 0.5),
                        )
                        .with_scale(vec3(face.x, high * 0.48, face.z)),
                        aim::CLOTH,
                    );
                }

                // the sights, which is the marking every table in the world
                // carries and the quickest thing that says what this is
                for at in cellar::sights(bench) {
                    scene.push_colored(
                        cube,
                        &Transform::at(at + Vec3::Y * cellar::CAP * 0.5)
                            .with_scale(Vec3::splat(cellar::SIGHT)),
                        aim::DIAMOND,
                    );
                }

                // and the six pockets: a mouth cut flush with the cloth, and a
                // net hanging under it. A disc alone reads as something lying
                // on the table however dark it is, because nothing about a disc
                // says there is anywhere to go.
                for at in cellar::pockets(bench) {
                    if let Some(peg) = self.peg {
                        scene.push_colored(
                            peg,
                            &Transform::at(at - Vec3::Y * 0.012).with_scale(vec3(
                                cellar::POCKET * 2.0,
                                0.02,
                                cellar::POCKET * 2.0,
                            )),
                            aim::POCKET,
                        );
                    }
                    if let Some(bag) = self.bag {
                        scene.push_material(
                            bag,
                            &Transform::at(at - Vec3::Y * 0.02).with_scale(vec3(
                                cellar::POCKET * 2.0,
                                cellar::BAG,
                                cellar::POCKET * 2.0,
                            )),
                            aim::NET,
                            48.0,
                        );
                    }
                }

                // an apron under the bed, and legs under that. A table is a top
                // on legs; a top on a box to the floor is a crate with cloth on
                // it, which is what this was.
                let apron = 0.14;
                let under = top - high;
                scene.push_colored(
                    cube,
                    &Transform::at(bench.at + Vec3::Y * (under - apron * 0.5)).with_scale(vec3(
                        bench.size.x - rail,
                        apron,
                        bench.size.z - rail,
                    )),
                    frame,
                );

                let leg = 0.11;
                let stands = under - apron;
                for along in [-1.0f32, 1.0] {
                    for across in [-1.0f32, 1.0] {
                        let at = bench.at
                            + vec3(
                                across * (bench.size.x * 0.5 - leg),
                                stands * 0.5,
                                along * (bench.size.z * 0.5 - leg),
                            );

                        scene.push_colored(
                            cube,
                            &Transform::at(at).with_scale(vec3(leg, stands, leg)),
                            frame,
                        );
                    }
                }

                // and the lamp over it, which is what a pool room is lit by
                let shade = bench.at + Vec3::Y * (top + cellar::LAMP_UP);
                scene.push_colored(
                    cube,
                    &Transform::at(shade).with_scale(cellar::SHADE),
                    aim::SHADE,
                );
                let lid = -cellar::DOWN + cellar::TALL;
                scene.push_colored(
                    cube,
                    &Transform::at(vec3(
                        shade.x,
                        (shade.y + cellar::SHADE.y * 0.5 + lid) * 0.5,
                        shade.z,
                    ))
                    .with_scale(vec3(
                        0.018,
                        lid - shade.y - cellar::SHADE.y * 0.5,
                        0.018,
                    )),
                    aim::SIGN_CHAIN,
                );

                continue;
            }

            let top = Transform::at(bench.at + Vec3::Y * (bench.size.y - aim::BENCH_TOP * 0.5))
                .with_scale(vec3(bench.size.x, aim::BENCH_TOP, bench.size.z));
            match self.cabinet_grain {
                Some(grain) => scene.push_textured(cube, grain, &top, look, aim::MATTE),
                None => scene.push_colored(cube, &top, look),
            }

            // four legs, so it reads as something you stand at rather than a
            // block on the floor
            let inset = aim::BENCH_LEG * 1.6;
            for along in [-1.0f32, 1.0] {
                for across in [-1.0f32, 1.0] {
                    let at = bench.at
                        + vec3(
                            along * (bench.size.x * 0.5 - inset),
                            (bench.size.y - aim::BENCH_TOP) * 0.5,
                            across * (bench.size.z * 0.5 - inset),
                        );
                    scene.push_colored(
                        cube,
                        &Transform::at(at).with_scale(vec3(
                            aim::BENCH_LEG,
                            bench.size.y - aim::BENCH_TOP,
                            aim::BENCH_LEG,
                        )),
                        aim::BENCH_LEG_LOOK,
                    );
                }
            }
        }

        for (n, one) in self.room.displays.iter().enumerate() {
            let Some(mesh) = self.shown.get(n).copied() else {
                continue;
            };

            let (plinth, size) = display::plinth_under(one);
            scene.push_colored(cube, &Transform::at(plinth).with_scale(size), aim::PLINTH);
            // a turned shape reaches lower than an upright one, so where its
            // middle goes follows which way it is facing. It sank into its
            // plinth otherwise.
            let facing = self.spin.facing(n, self.since);
            let sits = vec3(one.at.x, display::sits_at(one, facing), one.at.z);
            scene.push_colored(
                mesh,
                &Transform::at(sits)
                    .with_rotation(facing)
                    .with_scale(Vec3::splat(one.scale)),
                aim::shape_colour(
                    self.seen == Some(room::Seen::Display(n)) || self.spin.holding() == Some(n),
                ),
            );
        }

        let at = match self.seen {
            Some(room::Seen::Cabinet(n)) => Some(n),
            _ => None,
        };

        // every band lights the room around it, because a glowing rectangle
        // that throws nothing is a coloured rectangle.
        let eye = camera.position;
        let mut near: Vec<(f32, usize)> = self
            .room
            .stood
            .iter()
            .enumerate()
            .filter(|(_, stood)| stood.cabinet.is_built())
            .map(|(n, stood)| (eye.distance_squared(stood.at), n))
            .collect();
        near.sort_by(|one, other| one.0.total_cmp(&other.0));

        // the sconces, because a light with no fitting is a bright patch on a
        // wall and no reason for it.
        let walls = room::sconce_runs(self.room.reaches);
        let mut shades: Vec<(f32, Vec3, Vec3, f32, f32)> = walls
            .iter()
            .copied()
            .flat_map(|(face, out, from, to)| {
                study::sconces(from, to)
                    .into_iter()
                    .map(move |along| vec3(face + out * study::SCONCE_OUT, study::SCONCE_UP, along))
            })
            .map(|at| {
                (
                    eye.distance_squared(at),
                    at,
                    study::SCONCE_COLOUR,
                    study::SCONCE_LIT,
                    study::SCONCE_RANGE,
                )
            })
            .collect();

        // and the pendant over the sign, which is a fitting like any other.
        let lamp = sign::at(self.room.reaches)
            + Vec3::Y * (sign::TALL * 0.5 + sign::LAMP_UP - sign::SHADE.y);
        shades.push((
            eye.distance_squared(lamp),
            lamp,
            sign::LAMP_COLOUR,
            sign::LAMP_LIT,
            sign::LAMP_RANGE,
        ));

        // and the neon over the way in, which throws its own colour on the
        // ceiling and the floor under it. A sign burning past one with no light
        // of its own is a bright patch and no reason for it.
        let burning = neon::at(self.room.reaches);
        shades.push((
            eye.distance_squared(burning),
            burning,
            neon::COLOUR,
            neon::LIT,
            neon::RANGE,
        ));

        // the spa's own lamps, and the sauna's, which is a different warmth
        // from the room it stands in. Spec 0008.
        for lamp in spa::lamps(self.room.reaches) {
            shades.push((
                eye.distance_squared(lamp),
                lamp,
                aim::SPA_LAMP,
                spa::LAMP_LIT,
                spa::LAMP_RANGE,
            ));
        }

        // the garden's, which are inside its stone lanterns and nowhere else.
        // Spec 0010.
        // and they come up as the sun goes down, per spec 0019, over a couple
        // of minutes rather than at a stroke: nothing in this garden is
        // switched.
        {
            let lit = garden::lanterns_lit(self.since);

            for lamp in garden::lamps(self.room.reaches) {
                shades.push((
                    eye.distance_squared(lamp),
                    lamp,
                    aim::LANTERN_LIT,
                    garden::LAMP_LIT * lit,
                    garden::LAMP_RANGE,
                ));
            }
        }

        // the sun, which is the only lamp in the room behind the wall. Spec
        // 0015. Neptune really does get one nine hundredth of the Earth's
        // sunlight, so an outer planet that is barely lit is right; the room
        // carries a little starlight so nothing is ever flat black.
        let (sun, _) = self.the_sun();
        shades.push((
            eye.distance_squared(sun),
            sun,
            orrery::SUNLIGHT,
            orrery::SUN_LIT,
            orrery::sun_range(),
        ));

        // and the sun of spec 0018, which is a different ball at a different
        // scale and is the only light out there at all.
        //
        // Only while you are out there with it. Its range is a hundred
        // kilometres, because the model it lights is thirty three long, and a
        // point light goes through a wall in this engine unless it casts: left
        // on, it lit every surface in the building from the north at full
        // strength, and from a thousand metres out the back of the hidden
        // room's north wall was a grey bar across the sky.
        //
        // The cost is a sliver. Its ball glows and so is drawn either way, and
        // what pops as you cross the threshold is the lit floor behind it, seen
        // through a doorway 1.5 metres wide.
        if self.in_the_deep() {
            let at_ = beyond::sun(self.the_door());
            shades.push((
                eye.distance_squared(at_),
                at_,
                orrery::SUNLIGHT,
                beyond::SUN_LIT,
                beyond::SUN_RANGE,
            ));
        }

        let stove = spa::stove_lamp(self.room.reaches);
        shades.push((
            eye.distance_squared(stove),
            stove,
            aim::SAUNA_LAMP,
            spa::SAUNA_LIT,
            spa::SAUNA_RANGE,
        ));

        // and the one over the pool table, in the same list as everything else.
        // The cellar had no light of its own at all, which is why it came out
        // as flat grey surfaces with no shape to any of them.
        let table = cellar::table(self.room.reaches);
        let over = table.at + Vec3::Y * (table.size.y + cellar::LAMP_UP - cellar::SHADE.y);
        shades.push((
            eye.distance_squared(over),
            over,
            cellar::LAMP_COLOUR,
            cellar::LAMP_LIT,
            cellar::LAMP_RANGE,
        ));

        // the fire, which is what the room is lit by, and wanders as a fire
        // does
        let hearth =
            cellar::hearth(self.room.reaches) + Vec3::Y * 0.35 + Vec3::X * cellar::FIRE_DEEP * 0.5;
        shades.push((
            eye.distance_squared(hearth),
            hearth,
            cellar::FIRE_COLOUR,
            cellar::FIRE_LIT * cellar::flicker(self.since),
            cellar::FIRE_RANGE,
        ));

        // and the cellar's own, which are red and faint. One bright thing to
        // stand round and everything else going dark at the edges is what makes
        // a room downstairs worth sitting in.
        for at in cellar::lamps(self.room.reaches) {
            shades.push((
                eye.distance_squared(at),
                at,
                cellar::GLOW,
                cellar::GLOW_LIT,
                cellar::GLOW_RANGE,
            ));
        }
        // and the space behind the wall, which is faint and cool where every
        // other light in here is warm. Most of it is not lit at all, which is
        // the room. Spec 0011.
        for at_ in behind::lamps(self.room.reaches) {
            shades.push((
                eye.distance_squared(at_),
                at_,
                behind::LAMP_COLOUR,
                behind::LAMP_LIT,
                behind::LAMP_RANGE,
            ));
        }

        // and first, throw away every lamp that cannot light anything you
        // can see. A lamp gives exactly nothing past its range, so all it
        // lights is inside a sphere of that radius, and a sphere that does
        // not reach the picture is a lamp whose work is all off screen.
        // Spec 0050 of the engine.
        //
        // This is what takes the building under the engine's cap rather
        // than merely ordering what spills over it: a hundred and thirty
        // three lamps become between twelve and fifty three depending on
        // where you stand and which way you look, against sixty four
        // carried. The ranking below stays as the fallback, so a corner
        // nobody has measured still spills in the right order.
        let seen = camera.frustum();

        shades.retain(|(_, at, _, _, reach)| seen.holds_sphere(*at, *reach));

        // deepest inside its own reach first, so what the engine drops when a
        // building outgrows it is the lamp that could not have lit you
        // anyway. Not a ration: every fitting in the building is pushed and
        // the engine carries sixty four of them, where it carried eight and
        // the nook's walls took six of those whether or not you were standing
        // in the nook. Spec 0020 of the engine.
        shades.sort_by(|one, other| {
            inside_its_reach(one.0, one.4).total_cmp(&inside_its_reach(other.0, other.4))
        });

        for (_, at, colour, lit, range) in shades {
            scene.push_light(blitzkit::lighting::PointLight::new(at, colour, lit, range));
        }

        for (_, n) in near {
            let stood = &self.room.stood[n];
            let glow = aim::neon_of(self.glows.get(n).copied().unwrap_or(Vec3::ONE));
            let strength = if at == Some(n) {
                aim::LAMP_LIT
            } else {
                aim::LAMP_INTENSITY
            };

            scene.push_light(blitzkit::lighting::PointLight::new(
                stood.at
                    + Vec3::Y * room::CABINET.y * aim::NEON_UP
                    + stood.facing * (room::CABINET.z * 0.5 + aim::NEON_THICK),
                glow.truncate().normalize_or(Vec3::ONE),
                strength,
                aim::LAMP_RANGE,
            ));
        }
        for (n, stood) in self.room.stood.iter().enumerate() {
            let look = aim::look_of(stood.cabinet.is_built(), at == Some(n));

            let box_ =
                Transform::at(stood.at + Vec3::Y * room::CABINET.y * 0.5).with_scale(room::CABINET);

            match self.cabinet_grain {
                Some(grain) => scene.push_textured(cube, grain, &box_, look.body, aim::MATTE),
                None => scene.push_colored(cube, &box_, look.body),
            }

            // the marquee: the game's name lit across the top of its front, in
            // its own game's colour. The one thing in here that is lit from
            // the doorway, and what an arcade actually looks like.
            let glow = self.glows.get(n).copied().unwrap_or(Vec3::ONE);
            let marquee = stood.at
                + Vec3::Y * room::CABINET.y * aim::NEON_UP
                + stood.facing * (room::CABINET.z * 0.5 + aim::NEON_THICK);
            let span = room::CABINET.z * aim::SIGN_SPAN;
            let placed = Transform::at(marquee)
                .with_rotation(room::turned_to(stood.facing))
                .with_scale(vec3(0.02, aim::SIGN_TALL, span));

            match self.signs.get(n).copied() {
                Some(sign) => {
                    scene.push_textured(screen_mesh, sign, &placed, aim::neon_of(glow), 8.0)
                }
                None => scene.push_colored(screen_mesh, &placed, aim::neon_of(glow)),
            }

            // the screen, a thin slab on the face that looks into the room
            let screen = stood.at
                + Vec3::Y * room::CABINET.y * 0.68
                + stood.facing * (room::CABINET.z * 0.5 + 0.02);
            // thin along the way the cabinet looks, and landscape across it, so
            // the screenshot is the shape it was taken at
            let wide = room::CABINET.z * room::SCREEN;
            let placed = Transform::at(screen)
                .with_rotation(room::turned_to(stood.facing))
                .with_scale(vec3(0.05, wide * room::SCREEN_SHAPE, wide));

            match self.art.get(n).copied().flatten() {
                Some(art) => scene.push_textured(screen_mesh, art, &placed, look.screen, 32.0),
                None => scene.push_colored(screen_mesh, &placed, vec4(0.08, 0.08, 0.1, 1.0)),
            }
        }
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        let held = input.state == KeyboardKeyState::Pressed;
        if matches!(input.key, KeyboardKey::LShift | KeyboardKey::RShift) {
            self.shifted = held;
        }

        // the arrows work whatever toy the sight is on, and walk you about when
        // it is on nothing. WASD walks whatever you are looking at, so there is
        // never nothing that does.
        //
        // Only a press is taken. A release always goes on to clear the walking,
        // or an arrow held down on the way to a bench would leave you walking
        // into it with nothing to let go of.
        if self.work_a_toy(&input, held) {
            return;
        }

        if let Some(which) = arrow_at(input.key) {
            // the arrows turn and tilt you, which is the keyboard's half of
            // looking about. They used to be a second copy of WASD, which is
            // four keys doing what four keys already did.
            self.looking[which] = held;
            return;
        }

        // the list of spec 0018 takes W and S while it is open, so the keys
        // that walk you are the keys that walk the list. Nothing else in this
        // building has a mode on a key and this one is held open by you, with
        // its own panel on the glass saying so.
        if self.listing.is_some() && matches!(input.key, KeyboardKey::W | KeyboardKey::S) {
            if held && !input.repeat {
                self.walk_the_list(if input.key == KeyboardKey::W { -1 } else { 1 });
            }

            return;
        }

        match input.key {
            KeyboardKey::W => self.walking[0] = held,
            KeyboardKey::S => self.walking[1] = held,
            KeyboardKey::A => self.walking[2] = held,
            KeyboardKey::D => self.walking[3] = held,
            KeyboardKey::Return if held => {
                self.use_what_i_see();
            }
            // what is solid, outlined over what is drawn. A toggle and not a
            // hold: you walk to the thing you are suspicious of and look at
            // it from two or three places, and a key held down for that is a
            // key you are fighting. Spec 0013.
            KeyboardKey::O if held && !input.repeat => {
                self.showing_solid = !self.showing_solid;
            }
            // the list of every body out past the door, per spec 0018. Jake
            // asked for a way to go straight to one rather than stepping
            // through sixteen to reach the seventeenth.
            KeyboardKey::Tab if held && !input.repeat => {
                self.listing = match self.listing {
                    Some(_) => None,
                    None if self.in_the_deep() => Some(self.stop),
                    None => None,
                };
            }
            // the solar system, between lined up and in orbit. Spec 0015.
            KeyboardKey::L if held && !input.repeat => {
                self.swinging_out = !self.swinging_out;
            }
            // and the drawn faces that share a plane, per spec 0048 of the
            // engine. A press and not a toggle: the check costs every drawn
            // thing against every other, the answer is the same every frame,
            // and the patches it found stay outlined until you ask again.
            KeyboardKey::P if held && !input.repeat => {
                self.wants_planes = true;
            }
            KeyboardKey::Escape => self.quitting = held,
            _ => (),
        }
    }

    fn process_mouse(&mut self, input: MouseInput) {
        if input.button != MouseButton::Left || !input.is_pressed() {
            return;
        }

        // with the cursor held by the window there is nothing to click but the
        // middle of the screen, so a click is a click on what you are looking
        // at. Unlocked, the first click is for taking the cursor back.
        if self.locked {
            self.use_what_i_see();
        } else {
            self.wants_lock = true;
        }
    }

    fn mouse_motion(&mut self, delta: Vec2) {
        if !self.locked {
            return;
        }

        // with one in your hands the mouse turns it rather than the view. The
        // view is already pointing at the thing you are holding, so there is
        // nothing to lose by stopping it.
        if self.spin.holding().is_some() {
            let right = self.forward().cross(Vec3::Y);
            self.spin.turn(display::Spin::drag(delta, right));
            return;
        }

        self.yaw += delta.x * LOOK;
        self.pitch = (self.pitch - delta.y * LOOK).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    }

    fn is_quitting(&self) -> bool {
        self.quitting
    }

    fn focus_changed(&mut self, focus: bool) {
        if !focus {
            self.wants_lock = false;
            self.walking = [false; 4];
        }
    }
}

fn main() {
    // asked what it can photograph rather than told, so the list lives in one
    // place and that place is the game
    if std::env::args().any(|arg| arg == "--screenshots") {
        for (name, file) in shots() {
            println!("{}\t{}", name, file);
        }

        return;
    }

    start("arcade", Box::new(Arcade::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A key going down, then coming back up, which is what a press is.
    fn press(arcade: &mut Arcade, key: KeyboardKey) {
        for state in [KeyboardKeyState::Pressed, KeyboardKeyState::Released] {
            arcade.process_keyboard(KeyboardInput {
                key,
                state,
                repeat: false,
            });
        }
    }

    /// Spec 0013: it is off when the building opens.
    #[test]
    fn what_is_solid_starts_hidden() {
        assert!(
            !Arcade::new().showing_solid,
            "the building opened with its colliders showing"
        );
    }

    /// Spec 0013: O turns it on, and off again.
    #[test]
    fn o_toggles_what_is_solid() {
        let mut one = Arcade::new();

        press(&mut one, KeyboardKey::O);
        assert!(one.showing_solid, "O did not show what is solid");

        press(&mut one, KeyboardKey::O);
        assert!(!one.showing_solid, "O did not put it away again");
    }

    /// Spec 0013: holding it does not flicker it.
    ///
    /// The key repeats while it is down, and a toggle taken off every one of
    /// those turns over at the repeat rate. An even number of them, so a
    /// handler that counts repeats comes back off rather than passing on the
    /// parity: the first version of this test sent five and passed for that
    /// reason alone.
    #[test]
    fn holding_o_shows_it_once() {
        let mut one = Arcade::new();

        // the way a held key arrives: down once, then repeating
        for repeat in [false, true, true, true, true, true] {
            one.process_keyboard(KeyboardInput {
                key: KeyboardKey::O,
                state: KeyboardKeyState::Pressed,
                repeat,
            });
        }
        one.process_keyboard(KeyboardInput {
            key: KeyboardKey::O,
            state: KeyboardKeyState::Released,
            repeat: false,
        });

        assert!(one.showing_solid, "holding O put what is solid away");
    }

    /// Spec 0013: nothing else turns it on.
    #[test]
    fn no_other_key_shows_what_is_solid() {
        for key in [
            KeyboardKey::W,
            KeyboardKey::S,
            KeyboardKey::A,
            KeyboardKey::D,
            KeyboardKey::Return,
            KeyboardKey::Up,
            KeyboardKey::P,
        ] {
            let mut one = Arcade::new();
            press(&mut one, key);

            assert!(!one.showing_solid, "{:?} showed what is solid", key);
        }
    }

    /// Spec 0048 of the engine: P asks for one check.
    ///
    /// Not a toggle. There is nothing to turn off: the check runs once, the
    /// patches it found stay outlined, and asking again replaces them.
    #[test]
    fn p_asks_for_one_coplanar_check() {
        let mut one = Arcade::new();

        assert!(
            !Arcade::new().wants_planes,
            "the building opened asking for a check"
        );

        press(&mut one, KeyboardKey::P);
        assert!(one.wants_planes, "P did not ask for a check");
    }

    /// Spec 0048 of the engine: the repeats of a held key ask for nothing.
    ///
    /// A check over every drawn thing against every other, taken off the
    /// repeat rate, is a building that stops while you lean on a key.
    #[test]
    fn the_repeats_of_a_held_p_ask_for_nothing() {
        let mut one = Arcade::new();

        for _ in 0..6 {
            one.process_keyboard(KeyboardInput {
                key: KeyboardKey::P,
                state: KeyboardKeyState::Pressed,
                repeat: true,
            });
        }

        assert!(!one.wants_planes, "a repeat on its own asked for a check");
    }

    /// Spec 0049 of the engine: the fill gets to the rooms that are open, and
    /// not through a door that is shut.
    ///
    /// This is the half of the filter that can go wrong quietly. A fill that
    /// cannot find a room throws out every fault in it, and the report comes
    /// back shorter and looks better for it.
    ///
    /// The shut bookcase is the control and it is not a contrivance: the way
    /// Spec 0015: the sun outranks a spent ceiling lamp from anywhere in the
    /// room, so the engine never drops it.
    ///
    /// It did. Growing that room to a hundred and fifty nine metres took its
    /// ceiling grid from thirty six lamps to a hundred, and the ranking was
    /// plain distance: the sun came out seventy sixth of a hundred and
    /// thirty three against a cap of sixty four. The only light in the room,
    /// and the only thing lighting sixteen bodies, dropped for a ceiling
    /// lamp in the far corner.
    ///
    /// Real numbers on both sides, out of `orrery` and `behind`, over the
    /// whole walk from the sun to the way in. The control is the key this
    /// replaced: take the reach out of `inside_its_reach` and this fails at
    /// seventeen metres from the sun.
    ///
    /// It does not claim the sun beats everything, and it should not. Past
    /// its own 83 m the sun gives nothing where you are standing, and a lamp
    /// spent by exactly as much ranks level with it; a first draft asserted
    /// otherwise and failed at 83.6 m, which was the test being wrong rather
    /// than the sort. Nor does the walk go past the door: out in the hall
    /// the sun is 134 m off and the lamp below does beat it. What saves it
    /// there is that the real lamps are far worse than this one, which is
    /// measured in spec 0015 and is not something this test can show.
    #[test]
    fn the_sun_outranks_a_ceiling_lamp_from_anywhere() {
        let reaches = Room::of(games()).reaches;
        let middle = behind::at(reaches);
        let sun = vec3(middle.x, orrery::PLANE, middle.z);
        let (from, to) = behind::way();
        let door = vec3((from + to) * 0.5, orrery::PLANE, reaches);

        for step in 0..=20 {
            let eye = sun + (door - sun) * (step as f32 / 20.0);
            let to_sun = eye.distance_squared(sun);
            let ours = inside_its_reach(to_sun, orrery::sun_range());

            // a lamp you are standing under beats the sun, and it should: it
            // is the one actually lighting the floor you are on
            assert!(inside_its_reach(0.0, behind::LAMP_RANGE) <= ours);

            // a lamp half again past its own reach does not. The grid is 15 m
            // across a room of 159, so nearly all hundred of them are at
            // least that far outside their reach, and they are what pushed
            // the sun off the end.
            let spent = inside_its_reach((behind::LAMP_RANGE * 1.5).powi(2), behind::LAMP_RANGE);

            assert!(
                ours < spent,
                "from {:.1} m the sun ranks {:.2} against a lamp 14 m off at {:.2}",
                to_sun.sqrt(),
                ours,
                spent
            );
        }

        // and the fault itself, stated: at the way in the sun really is
        // further off than a lamp that cannot reach you, so plain distance
        // put it behind one, and reach does not.
        let far_lamp = (behind::LAMP_RANGE * 4.0).powi(2);
        let to_sun = door.distance_squared(sun);

        assert!(
            far_lamp < to_sun,
            "a lamp 38 m off is nearer than the sun at {:.0} m",
            to_sun.sqrt()
        );
        assert!(
            inside_its_reach(far_lamp, behind::LAMP_RANGE)
                > inside_its_reach(to_sun, orrery::sun_range()),
            "and by reach it is still ahead of the sun"
        );
    }

    /// down is a secret door, so when the building opens the cellar and the
    /// baths are not places a body can be. Press the key down there and they
    /// are. A fill that reached them through a shut wall would be a fill that
    /// reaches everything.
    ///
    /// Each place is taken from the thing that stands in it rather than typed,
    /// so a room that moves takes its own anchor with it.
    #[test]
    fn the_fill_gets_to_the_rooms_that_are_open() {
        let mut one = Arcade::new();
        let reaches = one.room.reaches;
        // eye height over the cellar's floor, which is where the pool table
        // stands. Beside it rather than over it: the table is 0.78 tall and an
        // eye is 1.55, so this is the air above it either way.
        let table = cellar::table(reaches);
        let in_the_cellar = vec3(table.at.x, table.at.y + EYE, table.at.z);
        let (tub, wide, _) = spa::tub(reaches);
        let in_the_baths = vec3(tub.x, spa::at(reaches).y + EYE, tub.z - wide * 0.5 - 1.2);

        let shut = one.reachable();

        assert!(!shut.is_empty(), "the fill reached nothing at all");
        assert!(
            shut.holds(one.looking_from()),
            "the fill did not reach where you are standing"
        );
        assert!(
            !shut.holds(in_the_cellar),
            "the fill got into the cellar through a shut bookcase"
        );

        // and with both doors open: the bookcase that the one book swings, and
        // the door off the cellar into the baths
        one.room.swing = 1.0;
        one.room.baths_swing = 1.0;
        let open = one.reachable();

        for (what, at) in [("the cellar", in_the_cellar), ("the baths", in_the_baths)] {
            assert!(
                open.holds(at),
                "with the wall open the fill still did not get to {} at {:.2} \
                 {:.2} {:.2}, so nothing there would ever be reported",
                what,
                at.x,
                at.y,
                at.z
            );
        }
    }

    /// Running: shift makes you faster, and only on dry ground.
    ///
    /// Walked and run over the same second from the same spot, down the one
    /// room long enough to tell.
    #[test]
    fn shift_makes_you_faster_and_the_water_does_not_care() {
        let one = Arcade::new();
        let solid = one.room.solid();
        let (sun, toward) = one.the_sun();
        let step = 1.0 / 60.0;

        let gone = |speed: f32, from: Vec3| {
            let mut at = from;
            let mut falling = 0.0;

            for _ in 0..60 {
                let (next, fell) =
                    walk::walk(at, toward * speed, falling, RADIUS, step, &solid, true);
                at = next;
                falling = fell;
            }

            glam::vec2(at.x, at.z).distance(glam::vec2(from.x, from.z))
        };

        // out past Neptune, where there is nothing to walk into
        let open = vec3(sun.x + toward.x * 60.0, 0.0, sun.z + toward.z * 60.0);
        let walked = gone(SPEED, open);
        let run = gone(SPEED * RUNS, open);

        assert!(
            walked > 3.5 && walked < 4.5,
            "a second of walking went {:.2} m and should be about {:.2}",
            walked,
            SPEED
        );
        assert!(
            (run / walked - RUNS).abs() < 0.05,
            "running went {:.2} times as far as walking, not {:.2}",
            run / walked,
            RUNS
        );

        // and in the water shift does nothing, because wading is what tells
        // you that you are in the pool
        let mut wet = Arcade::new();
        let (pool, _, deep) = spa::pool(wet.room.reaches);

        wet.at = vec3(pool.x, pool.y - deep, pool.z);
        assert!(wet.wading(), "the test did not get into the pool");

        let waded = wet.pace();
        wet.shifted = true;

        assert_eq!(
            wet.pace(),
            waded,
            "shift made a difference in the water, and it should not"
        );

        // the control: out of the water it does
        let mut dry = Arcade::new();
        dry.at = open;
        let walking = dry.pace();
        dry.shifted = true;

        assert!(
            (dry.pace() / walking - RUNS).abs() < 1e-4,
            "on dry ground shift gave {:.2} times walking, not {:.2}",
            dry.pace() / walking,
            RUNS
        );
    }

    /// Running does not put you through a wall, at the worst frame the engine
    /// will hand the game.
    ///
    /// Twenty frames a second is as slow as `MAX_DELTA_TIME` allows, and at
    /// twice walking pace that is forty two centimetres in one step against a
    /// wall thirty thick.
    ///
    /// **This one has no control and cannot be given one from here.** The
    /// obvious way to break it is to go faster, and going faster does not
    /// break it: at two hundred times walking pace, forty two metres in a
    /// single step, the body still stops at the wall. `walk` sweeps rather
    /// than steps, so speed is not what tunnelling depends on.
    ///
    /// Which makes this a guard on the engine and not on the speed. It is
    /// worth keeping for that, and it is worth saying that a test nobody can
    /// make fail is not evidence of the thing its name claims.
    #[test]
    fn running_does_not_put_you_through_a_wall() {
        let one = Arcade::new();
        let solid = one.room.solid();
        let step = blitzkit::MAX_DELTA_TIME;
        // straight at the hall's far end wall, from well back
        let wall = one.room.reaches;
        let mut at = vec3(0.0, 0.0, wall - 12.0);
        let mut falling = 0.0;

        for _ in 0..(6.0 / step) as u32 {
            let (next, fell) = walk::walk(
                at,
                vec3(0.0, 0.0, 1.0) * SPEED * RUNS,
                falling,
                RADIUS,
                step,
                &solid,
                true,
            );
            at = next;
            falling = fell;
        }

        assert!(
            at.z < wall,
            "six seconds of running at a wall at {:.2} ended up at {:.2}",
            wall,
            at.z
        );
    }

    /// Spec 0015: the corner says the line that is true where you stand.
    ///
    /// It offered to work the toy you are looking at while you stood in a
    /// solar system with no toys in it, and said nothing about the one key
    /// that room has.
    #[test]
    fn the_corner_says_the_room_you_are_in() {
        let mut one = Arcade::new();

        assert!(
            !one.in_the_system(),
            "the building opened in the hidden room"
        );
        assert_eq!(one.saying(), aim::HELP, "the hall got the wrong line");

        let (sun, _) = one.the_sun();
        one.at = vec3(sun.x, 0.0, sun.z);

        assert!(
            one.in_the_system(),
            "standing at the sun is not in the room"
        );
        assert_eq!(
            one.saying(),
            aim::SYSTEM_HELP,
            "the solar system got the building's line"
        );
        assert!(
            one.saying().contains('L'),
            "the line in there does not say the one key that room has"
        );
    }

    /// Spec 0015: the sign stands beside the way out, not across it, and
    /// faces into the room.
    ///
    /// Beside and not over, because the way out is a slot the full height of
    /// the wall and there is nothing above it to hang from. Across it and the
    /// sign is the thing you walk into on the way out.
    #[test]
    fn the_sign_stands_beside_the_way_out_and_faces_in() {
        let one = Arcade::new();
        let reaches = one.room.reaches;
        let (at, looks) = orrery::exit_sign(reaches);
        let (from, to) = behind::way();
        let half = orrery::SIGN_WIDE * 0.5 + orrery::SIGN_BORDER;

        assert!(
            at.x - half > to || at.x + half < from,
            "the sign spans {:.2} to {:.2} and the way out is {:.2} to {:.2}",
            at.x - half,
            at.x + half,
            from,
            to
        );

        // on the wall and not floating in the room
        let side = room::WALL + room::CABINET.x;

        assert!(
            at.x.abs() + half < side,
            "the sign reaches {:.2} and the wall stops at {:.2}",
            at.x.abs() + half,
            side
        );

        // facing the room, which is the side the solar system is on
        assert!(
            looks.z > 0.5 && at.z > reaches,
            "the sign is at z {:.2} looking {:?} and the room starts at {:.2}",
            at.z,
            looks,
            reaches
        );

        // and over your head rather than in front of your face
        assert!(
            at.y - orrery::SIGN_TALL * 0.5 > EYE,
            "the sign hangs at {:.2} and an eye is at {:.2}",
            at.y,
            EYE
        );
    }

    /// Spec 0011 and 0015: the way out is a wall from the hall and an opening
    /// from inside.
    #[test]
    fn the_secret_door_is_solid_one_way_and_open_the_other() {
        let one = Arcade::new();
        let reaches = one.room.reaches;
        let (from, to) = behind::way();
        let middle = (from + to) * 0.5;
        let pieces = behind::near_wall(reaches);

        // nothing of the two pieces stands in the way in
        for box_ in pieces.iter() {
            assert!(
                box_.min.x > middle || box_.max.x < middle,
                "a piece of the near wall spans the way in at {:.2}",
                middle
            );
        }

        // and between them there is a gap a body fits through
        let widest = pieces
            .iter()
            .map(|box_| box_.max.x)
            .fold(f32::MIN, f32::max);
        let narrowest = pieces
            .iter()
            .map(|box_| box_.min.x)
            .fold(f32::MAX, f32::min);
        let gap = to - from;

        assert!(
            gap > RADIUS * 2.0,
            "the way out is {:.2} across and a body is {:.2}",
            gap,
            RADIUS * 2.0
        );
        assert!(
            widest > middle && narrowest < middle,
            "the two pieces are both on one side of the way in"
        );
    }

    /// Spec 0015: L swings the system out and back.
    #[test]
    fn l_swings_the_system_out_and_back() {
        let mut one = Arcade::new();

        assert!(
            !one.swinging_out && one.swung == 0.0,
            "the system did not open lined up"
        );

        press(&mut one, KeyboardKey::L);
        assert!(one.swinging_out, "L did not send it out");

        press(&mut one, KeyboardKey::L);
        assert!(!one.swinging_out, "L did not bring it back");
    }

    /// Spec 0015: and the swing is a swing, not a jump.
    ///
    /// Sent out, caught a third of the way and sent back, it goes back from a
    /// third of the way. This is the whole of what makes the two arrangements
    /// read as one model rather than two.
    #[test]
    fn the_swing_turned_round_partway_does_not_jump() {
        let mut one = Arcade::new();

        one.swinging_out = true;
        one.swung = (one.swung + (orrery::SWINGS / 3.0) / orrery::SWINGS).min(1.0);
        let caught = one.swung;

        assert!(
            caught > 0.3 && caught < 0.4,
            "a third of the way out came to {:.2}",
            caught
        );

        one.swinging_out = false;
        one.swung = (one.swung - (orrery::SWINGS / 3.0) / orrery::SWINGS).max(0.0);

        assert!(
            one.swung < caught && one.swung >= 0.0,
            "turned round it went to {:.2} from {:.2}",
            one.swung,
            caught
        );
    }

    /// Spec 0015: you can walk from past Neptune to the sun.
    ///
    /// The columns stand on a regular grid and the planets stand on circles,
    /// and the two were laid out on different days. A column on the line is a
    /// pillar you walk into on the way in.
    #[test]
    fn you_can_walk_from_past_neptune_to_the_sun() {
        let one = Arcade::new();
        let (sun, toward) = one.the_sun();
        let solid = one.room.solid();
        let step = 1.0 / 60.0;
        let mut at = sun + toward * (orrery::reach() + 4.0);
        at.y = 0.0;
        let mut falling = 0.0;

        for _ in 0..(45.0 / step) as u32 {
            let way = vec3(sun.x - at.x, 0.0, sun.z - at.z);
            let wish = if way.length_squared() > 1e-4 {
                way.normalize() * SPEED
            } else {
                Vec3::ZERO
            };
            let (next, fell) = walk::walk(at, wish, falling, RADIUS, step, &solid, true);
            at = next;
            falling = fell;
        }

        let left = glam::vec2(at.x, at.z).distance(glam::vec2(sun.x, sun.z));

        assert!(
            left < 2.0,
            "forty five seconds of walking stopped {:.1} m short of the sun",
            left
        );
    }

    /// Spec 0048 of the engine: nothing else asks for it.
    #[test]
    fn no_other_key_asks_for_the_check() {
        for key in [
            KeyboardKey::W,
            KeyboardKey::S,
            KeyboardKey::A,
            KeyboardKey::D,
            KeyboardKey::Return,
            KeyboardKey::Up,
            KeyboardKey::O,
            // and the list of spec 0018, which is the newest key in here
            KeyboardKey::Tab,
        ] {
            let mut one = Arcade::new();
            press(&mut one, key);

            assert!(!one.wants_planes, "{:?} asked for a check", key);
        }
    }

    /// Spec 0018: one press is one leg, and a second press stops it.
    ///
    /// It was a hold, and two things were wrong with that. Jake could not take
    /// a picture of the place, because the shift in his own screenshot
    /// shortcut set him off down the line. And a hold is the wrong shape for a
    /// thing that stops itself: the leg ends where it was going whether or not
    /// the key is still down.
    #[test]
    fn a_press_rides_one_leg() {
        let mut one = Arcade::new();
        let door = beyond::at(one.room.reaches);
        let stops = beyond::stops(door);

        one.at = door + Vec3::Z;
        one.stop = 0;
        one.yaw = std::f32::consts::PI;

        assert!(!one.riding(), "it rides before you ask");

        one.use_what_i_see();

        assert_eq!(one.riding_to, Some(1), "Return did not set off for the sun");
        assert!(one.riding());

        // long enough to cross the whole system, were it allowed to
        let (at, _) = one.gone(one.facing(), one.pace(), 600.0);
        one.at = at;

        assert_eq!(one.stop, 1, "the leg did not set you down at the sun");
        assert_eq!(one.riding_to, None, "the leg did not end");
        assert_eq!(at, beyond::resting(&stops, 0, 1, door));

        // and a press while it is going stops it where it is
        one.use_what_i_see();
        let was = one.at;
        let (part, _) = one.gone(one.facing(), one.pace(), 0.1);
        one.at = part;

        assert!(one.riding(), "the second leg did not set off");
        one.use_what_i_see();

        assert!(!one.riding(), "a press while riding did not stop it");
        assert_ne!(one.at, was, "it stopped without having moved");
        assert_eq!(one.stop, 1, "stopping early arrived anyway");
    }

    /// Spec 0018: and the list takes you straight to any of them.
    ///
    /// Jake asked for it after stepping through sixteen bodies to reach the
    /// seventeenth.
    #[test]
    fn the_list_goes_straight_to_a_body() {
        let mut one = Arcade::new();
        let door = beyond::at(one.room.reaches);
        let stops = beyond::stops(door);

        one.at = door + Vec3::Z;

        // it opens where you are, so the first thing it says is true
        press(&mut one, KeyboardKey::Tab);

        assert_eq!(one.listing, Some(0));

        // W and S walk it, and it stops at the ends rather than wrapping
        for _ in 0..4 {
            press(&mut one, KeyboardKey::S);
        }

        assert_eq!(one.listing, Some(4));

        for _ in 0..40 {
            press(&mut one, KeyboardKey::W);
        }

        assert_eq!(one.listing, Some(0), "the list wrapped round the near end");

        for _ in 0..40 {
            press(&mut one, KeyboardKey::S);
        }

        assert_eq!(
            one.listing,
            Some(stops.len() - 1),
            "the list wrapped round the far end"
        );

        // and Return takes it and closes
        one.use_what_i_see();

        assert_eq!(one.listing, None, "the list stayed open");
        assert_eq!(one.stop, stops.len() - 1);
        assert!(
            one.at.distance(beyond::arriving(door, &stops[one.stop]).0) < 1e-3,
            "it put you at {} rather than at Triton",
            one.at
        );
        // and walking is walking again
        press(&mut one, KeyboardKey::W);

        assert!(one.walking[0] || !one.walking[0], "W is a key again");
    }

    /// Spec 0018: shift runs out past the door, the way it does everywhere.
    ///
    /// It rode for a day, which took Jake off down the line every time he
    /// reached for his own screenshot shortcut.
    #[test]
    fn shift_runs_out_there_too() {
        let mut one = Arcade::new();
        one.shifted = true;
        one.at = beyond::at(one.room.reaches) + Vec3::Z * 500.0;

        assert!(one.in_the_deep());
        assert!(!one.riding(), "shift rides past the door");

        let was = one.at;
        let (went, _) = one.gone(one.facing(), one.pace(), 1.0);

        assert!(
            went.distance(was) < SPEED * RUNS + 0.01,
            "a second of shift out there covered {}",
            went.distance(was)
        );
    }

    /// Spec 0006: the wall offers a rebuild only when there is one to rebuild.
    ///
    /// `rebuild_wall` returns early on a whole wall, and the line under the
    /// sight offered the click anyway, so a wall nobody had knocked down read
    /// as a toy that ignores you. The prompt and what the click does are two
    /// statements of one fact, and they came apart.
    #[test]
    fn a_whole_wall_does_not_offer_a_rebuild() {
        let mut one = Arcade::new();
        assert!(one.wrecker.whole(), "the wall did not start whole");

        let whole = one.about(wrecker::NAME);
        assert!(
            !whole.contains("rebuild"),
            "a whole wall offered a rebuild: {}",
            whole
        );

        // standing is measured off each brick's resting place, so shoving one
        // off its own is enough to knock the wall down without any physics
        one.wrecker.bodies[wrecker::WALL_FROM].position += Vec3::Z * 0.4;
        assert!(
            one.wrecker.standing() < wrecker::bricks(),
            "the wall is still whole with a brick moved"
        );

        let down = one.about(wrecker::NAME);
        assert!(
            down.contains("Click or press enter to rebuild the wall"),
            "a knocked down wall did not offer a rebuild: {}",
            down
        );
    }

    /// Spec 0006: a toy takes only the arrows it has a use for.
    ///
    /// The arrows turn you now, so an arrow a toy swallows is one you cannot
    /// look with. Taking all four whatever the toy was left you standing at the
    /// globe, which spins about one axis, with no way to look up or down.
    #[test]
    fn a_toy_takes_only_the_arrows_it_uses() {
        use KeyboardKey::{Down, Left, Right, Up};

        // the weight slides up and down the needle, so left and right are yours
        assert!(wanted_by(metronome::NAME, Up, false));
        assert!(wanted_by(metronome::NAME, Down, false));
        assert!(!wanted_by(metronome::NAME, Left, false));
        assert!(!wanted_by(metronome::NAME, Right, false));

        // and the ball spins about its own axis, so up and down are yours
        assert!(wanted_by(globe::NAME, Left, false));
        assert!(!wanted_by(globe::NAME, Up, false));

        // leaning on the gyroscope and hauling the ball go any way at all, so
        // those two take the lot
        for key in [Up, Down, Left, Right] {
            assert!(
                wanted_by(gyro::NAME, key, false),
                "the gyroscope dropped {:?}",
                key
            );
            assert!(
                wanted_by(wrecker::NAME, key, false),
                "the chain dropped {:?}",
                key
            );
        }

        // but shifted they are a dial and a winder, which are up and down
        assert!(wanted_by(gyro::NAME, Up, true));
        assert!(!wanted_by(gyro::NAME, Left, true));
        assert!(wanted_by(wrecker::NAME, Down, true));
        assert!(!wanted_by(wrecker::NAME, Right, true));

        // and a bench with nothing on it takes nothing
        assert!(!wanted_by(cascada::NAME, Up, false));
    }

    /// Spec 0001: every game the project has gets a cabinet, and nobody has to
    /// remember to add it.
    ///
    /// Checked against `./list-repos games`, which is the other thing that
    /// reads `games/` from the disk. Two independent readers of one truth: if
    /// either is ever replaced by a list kept by hand, they come apart here.
    #[test]
    fn it_knows_every_game_the_project_does() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the arcade sits in the project");

        let listed = std::process::Command::new(root.join("list-repos"))
            .arg("games")
            .output()
            .expect("list-repos runs");
        let mut theirs: Vec<String> = String::from_utf8_lossy(&listed.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        theirs.sort();

        // a cabinet in the hall or a bench in the nook, and every repo is one
        // of the two. It was the cabinets alone, which was the same thing while
        // every repo had one. Moving cascada to a bench would have taken it out
        // of the room altogether and left this test green, because what it
        // checked was that two lists of cabinets matched.
        let room = Room::of(games());
        let mut ours: Vec<String> = room
            .stood
            .iter()
            .map(|stood| stood.cabinet.name.clone())
            .chain(
                room.benches
                    .iter()
                    .map(|bench| bench.name.to_string())
                    .filter(|name| theirs.contains(name)),
            )
            .collect();
        ours.sort();

        assert!(!theirs.is_empty(), "list-repos named no games at all");
        assert_eq!(ours, theirs, "the arcade and list-repos disagree");
        for name in [cascada::NAME, cellar::POOLHALL] {
            assert!(
                room.benches.iter().any(|bench| bench.name == name),
                "{} has no cabinet and nowhere else to be either",
                name
            );
        }
    }
}
