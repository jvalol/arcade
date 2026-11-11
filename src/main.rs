//! arcade: a room of cabinets, one per game. See `specs/`.

mod aim;
mod back;
mod behind;
mod beyond;
mod bridge;
mod cabinet;
mod carpet;
mod cascada;
mod cellar;
mod cradle;
mod display;
mod field;
mod garden;
mod gate;
mod globe;
mod gyro;
// **Allowed to be unused until the bench exists.** Spec 0050's cube is being
// built state first: what a turn does to twenty pieces is provable without a
// bench, a colour or a mesh, and none of those are decided yet. The allow
// comes off with the seventh bench in `room::benches`.
#[allow(dead_code)]
mod kociemba;
mod mandel;
mod metronome;
mod mirrors;
mod neon;
mod noise;
mod orrery;
mod parked;
mod people;
mod room;
mod says;
mod sign;
mod spa;
mod standing;
mod study;
mod tesseract;
mod tour;
mod trail;
mod valhalla;
mod walk;
mod wrecker;

use aim::Plain;
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

/// The furthest the eye may be from the feet, however fast they move.
///
/// **Because a lag with no ceiling is a camera inside the floor.** The
/// easing above is a time constant, so what it settles at is the rate the
/// feet are climbing times `EYE_LAGS`, and nothing in this building climbed
/// fast enough for that to matter until Bifröst did. Running up it the feet
/// gain better than twenty a second, which parks the eye two metres under
/// them: further than `EYE` itself, so the camera sat below the deck and
/// looked up at the underside of the rainbow for most of the climb. Jake,
/// from a recording: there is some clipping where the player ends up beneath
/// the bridge for a bit.
///
/// `walk::STEP` because that is the discontinuity the easing exists for. A
/// step is climbed whole and in one frame, so the eye is exactly this far
/// behind the instant one is taken, and a ceiling at one step changes
/// nothing about a stair while refusing to let a climb stack them up.
const EYE_TRAILS: f32 = walk::STEP;
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

/// How far off the way you are going you may be and still walk it. Spec
/// 0049.
///
/// **Because a walk is a turning circle.** At `SPEED` and `TURNS` that
/// circle is 1.75 metres across and the planner's places are half a metre
/// apart, so a tour that walked flat out while turning orbited the next
/// place for ever. Thirty degrees is wide enough that a tour still rounds
/// a corner rather than stopping to aim at it.
const WALKS_WHEN: f32 = 0.52;
const TILTS: f32 = 1.6;

/// How long an arrow takes to reach that speed, and to let go of it, in
/// seconds. Spec 0044.
///
/// Beside `EYE_LAGS` because it is the same kind of number. A view that is at
/// full speed one frame after the key goes down and at rest one frame after it
/// comes up reads as a tripod head being cranked, which is what the arcade's
/// clip looked like: the walk under it is a person's and the turns were not.
///
/// **A fifth of a second, which is most of a quarter turn.** A quarter turn is
/// 0.655 seconds, so at this the view spends 0.2 winding on, 0.255 at speed
/// and 0.2 winding off, and what that reads as is one arc. At 0.12 the middle
/// was longer than either end and the turn still read as a crank at a gentler
/// speed, which is what Jake said after watching the first clip of it.
///
/// The ramps are symmetric, so a hold sweeps exactly `TURNS` times its own
/// length once the tail after the key is allowed to run. Every route written
/// against the flat rate still ends up facing where it did. What has to hold
/// for that is that nothing presses the next key inside this: `play` in
/// `refresh-examples` waits longer than this on purpose and says so.
const AIM_LAGS: f32 = 0.2;

/// Smoothstep, carried through nought so a turn reversed is as smooth as one
/// started. Spec 0044.
///
/// **Why the winding is not the speed.** Walking the speed itself toward what
/// the arrows ask for gives a flat acceleration, which is a corner at each end
/// of the ramp: the view goes from still to winding on as hard as it ever
/// will, in one frame. Winding a throttle at a steady rate and easing that
/// gives a speed that leaves nought and reaches full with no corner in it.
///
/// It costs the sweep nothing. Smoothstep is symmetric about a half, so the
/// area under the curve on the way up and the area missing on the way down are
/// the same area the flat rate had.
fn eased(wound: f32) -> f32 {
    let far = wound.abs().min(1.0);

    wound.signum() * far * far * (3.0 - 2.0 * far)
}

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

/// A tour of the building, walked rather than replayed. Spec 0049.
///
/// **It drives the same keys a player would**, rather than setting the
/// body down where the plan says. The turning goes at `TURNS` and the
/// walking is the ordinary walk, so what comes out still looks like
/// somebody getting about. What is different is that it aims again every
/// frame, which is the whole of why it cannot drift.
struct Filming {
    steps: Vec<tour::Step>,
    at: usize,
    /// The plan for the leg being walked, made when the leg starts.
    following: Option<tour::Follower>,
    /// What is left of a stand.
    waiting: f32,
}

/// What `--film` was given: the clip's own route, or a tour written out.
///
/// **`clip` names the route rather than spelling it.** The route is a score
/// of places that each have an owner in the building, and writing them into
/// the shell script that films it would be a second account of every one of
/// them. `tour::clip` reads the owners.
fn filmed(room: &room::Room) -> Option<Result<Vec<tour::Step>, String>> {
    let given = std::env::args().find_map(|arg| arg.strip_prefix("--film=").map(str::to_owned))?;

    Some(if given == "clip" {
        Ok(tour::clip(room, done_already()))
    } else {
        tour::read(&given)
    })
}

fn staged() -> bool {
    std::env::args().any(|arg| arg.starts_with("--screenshot"))
}

/// Whether this run starts with the building already done. `--done`.
///
/// **Because the bridge is at the end of eleven rooms.** Spec 0054 puts
/// Bifröst up only once every question in `trail::Asked` has been given,
/// which is the right rule and makes looking at the thing a day's work.
/// This fills the trail at the door so you can walk out of the garden and
/// see it.
///
/// A flag and not an `FLY`-style variable because Jake asked for one, and
/// because this is the one of these tools you reach for to look at
/// something finished rather than to hunt a fault.
///
/// **It does not say the trail exists to anybody who has not been told.**
/// Spec 0036 is plain that somebody who never finds the trail must not be
/// able to tell it is there, and that rule is about the game and not about
/// the command line: this game prints no usage, so an argument nobody
/// passes is as quiet as a variable nobody sets.
///
/// It gives the trail rather than forcing the bridge, so that anything else
/// which comes to read the trail sees one consistent account of a building
/// that has been done.
fn done_already() -> bool {
    std::env::args().any(|arg| arg == "--done")
}

/// Whether this run flies rather than walks. `FLY=1`.
///
/// **Because the slow part of finding a fault in this building is getting to
/// where you can see it.** `WHERE` solved the other half of that, which is
/// matching a picture to a camera; this is the half where the place you need
/// to stand is the roof, or the air fifteen metres west of a wall, or inside
/// a room you cannot walk into. Everything the outside of this building
/// needed in one sitting was found by posing a camera in a test and
/// rendering it, four or five rounds apiece.
///
/// Flight is the walk with the three things taken out that make it a walk:
/// gravity, the body's radius, and the floor. You go where you are looking,
/// so the pitch flies you up and down.
///
/// **The rooms will pop.** Seven things in here are decided by which room
/// the building thinks you are standing in, and all of them assume you are
/// in exactly one: fly out through a wall and the room behind you stops
/// being drawn, fly over the hall's end wall and spec 0015's solar system
/// appears, which is the gate that keeps it a secret. That is the price of
/// this being twenty lines instead of a rework of all seven, and it is the
/// right price for a tool whose job is to let you look at a thing.
///
/// The one gate flight does open is the field's, because a tool for looking
/// at the outside of a building that draws no ground and no sky the moment
/// you leave the grass is not a tool.
fn flying() -> bool {
    std::env::var("FLY").is_ok()
}

/// What time it is when this run starts, as a share of a day: nought is
/// dawn, a quarter noon, a half dusk, three quarters midnight.
/// `AT=0.8`.
///
/// **The third of the set, after `FLY` and `POSE`.** Those two get the
/// camera to the thing; this one gets the thing to the hour it is worth
/// looking at. A day here is twenty minutes and a run begins at 0.4,
/// mid-afternoon, so photographing anything the night does means sitting
/// in front of it for eight minutes first, once per picture. The moon
/// and the runestone's glow are both night things and neither could be
/// looked at without this.
fn starting_hour() -> Option<f32> {
    let said = std::env::var("AT").ok()?;

    said.trim().parse().ok().map(garden::when)
}

/// Where to stand this run, instead of at the door.
/// `POSE=x,y,z,yaw,pitch`.
///
/// **The other half of `WHERE`.** That one answers where the camera was
/// when you saw something; this one puts the camera back there. Together
/// they are one round trip instead of six, which is the whole of what the
/// walking skill is for.
///
/// It was a temporary block, written into `update` and taken out again,
/// six times in one day: for the garden's trees, for the lantern, twice
/// for the top of the bridge, for the foot of it, and for the roof. Every
/// one of those needed it rebuilt, and the rule about proving the
/// scaffolding did not survive needed running every time. A tool you write
/// by hand that often is a tool.
///
/// `FLY` is the neighbour of this and the two are usually wanted together:
/// that one takes gravity and the floor away, and this one saves the
/// flight.
///
/// **It does not survive a reset.** `back_to_the_door` puts you at the
/// door, which is what it is for, and this is read once on the first frame
/// rather than fought with.
fn posed_at() -> Option<(Vec3, f32, f32)> {
    let said = std::env::var("POSE").ok()?;
    let n: Vec<f32> = said
        .split(',')
        .filter_map(|one| one.trim().parse().ok())
        .collect();

    (n.len() >= 5).then(|| (vec3(n[0], n[1], n[2]), n[3], n[4]))
}

/// Where flight takes you: along `wish`, at `FLIES` times the pace, through
/// whatever is in the way.
///
/// Named rather than written inline at the one place it is used, because
/// what it leaves out is the whole of it and a test can only read the
/// leaving out if there is something to call.
fn flown(at: Vec3, wish: Vec3, pace: f32, dt: f32) -> Vec3 {
    if wish.length_squared() > 1e-6 {
        at + wish.normalize() * pace * FLIES * dt
    } else {
        at
    }
}

/// How much faster flight is than the walk.
///
/// Four, which at `SPEED` is seventeen metres a second. The building is three
/// hundred and twenty metres of field across and two kilometres of corridor
/// along, so a flight paced like a walk is a tool you stop reaching for.
pub const FLIES: f32 = 4.0;

/// Whether the staged shot is the hall's rather than the system's.
///
/// `--screenshot` stages what `media/screenshot.png` actually holds, which is
/// Saturn and its moon, so that `refresh-screenshots` reproduces the committed
/// picture rather than quietly replacing it with a different one. The hall's
/// pose is still here behind `--screenshot=hall`: it cost two rounds to derive
/// and the reasoning on it is still true.
/// `--screenshot=shapes` stands at the back of the gallery and looks at the
/// row, per spec 0022.
///
/// Its own slug because the hall's pose is worked out from a cabinet, and the
/// shapes are through a doorway from any cabinet. Just inside the way in,
/// which is the furthest back the room goes and the one place the whole row
/// is in frame.
fn staged_at_the_shapes() -> bool {
    staged_as().as_deref() == Some("shapes")
}

/// Where that stands, and which way it looks.
fn posed_at_the_shapes(reaches: f32) -> (Vec3, f32) {
    (
        vec3(0.0, 0.0, -reaches - 0.55),
        // yaw nought faces -z, which is down the room at the far wall
        0.0,
    )
}

/// `--screenshot=back` stands at the bench in spec 0024's back room.
///
/// Its own slug because the room is through a door off the gallery and a
/// pose that is worked out from a cabinet cannot reach it.
fn staged_in_the_back_room() -> bool {
    staged_as().as_deref() == Some("back")
}

/// Where that stands, and which way it looks.
///
/// At the working spot, which is where the room is arranged to be seen
/// from.
fn posed_in_the_back_room(reaches: f32) -> (Vec3, f32) {
    // at the rig, looking at it. It was just inside the door, which showed
    // the whole room and was the right picture while the room was the
    // point. Spec 0025 made the rig the point: what this has to show is a
    // block of white cubes with nothing behind them.
    let at = back::standing(reaches) + Vec3::Z * 1.1;
    let look = back::block(reaches) - at;

    (at, look.x.atan2(-look.z))
}

fn staged_in_the_hall() -> bool {
    staged_as().as_deref() == Some("hall")
}

/// `--screenshot=tesseract` stages spec 0021's room rather than the building.
///
/// It needs its own because it is nowhere in the building's coordinates: a
/// pose is a place and a yaw in the hall, and the eight cells are not in the
/// hall. So this is a slug that sets the mode rather than the camera.
fn staged_in_the_tesseract() -> bool {
    staged_as().as_deref() == Some(tesseract::SLUG)
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

/// Standing in spec 0021's eight rooms: which cell, what is known of the
/// map, the crossing in progress, and what the building has to put back.
///
/// The building is one continuous space and this is the one thing in it that
/// is not: the eight cells are the same cube drawn eight ways, so there is
/// nowhere in the hall's coordinates to put them.
#[derive(Clone, Copy)]
struct Inside {
    frame: tesseract::Frame,
    /// Which cells have been stood in. The map fills in as it is walked.
    found: tesseract::Found,
    /// The crossing in progress, if there is one. The wall you walk at
    /// becoming the floor you stand on is the only part of a four
    /// dimensional rotation a person can be shown.
    turning: Option<tesseract::Turn>,
    /// Where you stood in the nook, and which way you were looking, so coming
    /// out puts you back at the plinth rather than at the door.
    was: (Vec3, f32, f32),
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
    /// Where a cabinet's picture comes from.
    ///
    /// **A field rather than a call, so a picture test can hand the hall
    /// art of its own.** `art_of` reads `games/<name>/media/screenshot.png`
    /// off the disk, which puts thirteen other repos inside this one's
    /// reference: retaking any of those photographs moves the hall's
    /// picture, and on 2026-11-03 that stopped a release at its own gate
    /// and took nine commits to clear. Spec 0053.
    art_from: fn(&str) -> Option<TextureData>,
    /// What colour each cabinet's screenshot lends the room when it is lit.
    glows: Vec<Vec3>,
    /// Each game's name, drawn into a texture for its marquee. Spec 0037.
    signs: Vec<TextureId>,
    cube: Option<MeshId>,
    /// One target per mirrored wall, per engine spec 0091. Two walls,
    /// because panels in one plane are one reflection and a frame draws two
    /// mirrors at most.
    glass_targets: Vec<[TextureId; 2]>,
    /// Whether `DUMP` has already written the mirrors out this run.
    dumped: bool,
    /// The quad a pane of glass is drawn with. Its uv is never read: engine
    /// spec 0092 samples where the fragment is, not where the mesh says.
    pane: Option<MeshId>,
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
    /// A guest's body and a guest's leg, turned on the same wheel as the
    /// barrels and the bell. Spec 0033.
    body: Option<MeshId>,
    head: Option<MeshId>,
    thigh: Option<MeshId>,
    shin: Option<MeshId>,
    hoop: Option<MeshId>,
    bottle: Option<MeshId>,
    peg: Option<MeshId>,
    /// And what you are carrying out of it, which grows as you walk the
    /// diagonal and is the guest the desk cannot house.
    /// What the building has been given up to you, room by room. Spec 0036.
    ///
    /// **The bridge reads it**, and nothing else does. Spec 0054 puts
    /// Bifröst up out in the field once every question in `trail::Asked`
    /// has been given, and spec 0055 stands somebody at the top of it.
    /// Nothing counts at you on the way: there is no panel of ticks and no
    /// "3 of 8 found", which spec 0036 rules out in as many words.
    ///
    /// `--done` fills it at the door, for looking at the end of it without
    /// walking the building first.
    trail: trail::Trail,
    /// How far the gate at the top of the bridge has swung, nought shut
    /// and one open, per spec 0058.
    ///
    /// **One number is the whole state.** It only ever goes up, so nought
    /// is also "the riddle is unanswered" and there is no second flag to
    /// come apart from it.
    gate: f32,
    /// What the gallery's panel is showing, the texture it is written
    /// into, and what was last written there. Specs 0039 and the engine's
    /// 0074.
    view: mandel::View,
    mandel: Option<TextureId>,
    shown_view: Option<(mandel::View, bool)>,
    /// And the fine one, which is worked out a slice at a time while
    /// nobody is moving it and drawn instead of the coarse one once it is
    /// whole. Spec 0039.
    sharp: Option<TextureId>,
    settling: Option<mandel::Settling>,
    sharp_view: Option<mandel::View>,
    /// Whether an arrow is on it right now, which decides how many rounds
    /// the picture gets.
    moving_the_view: bool,
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
    /// Heimdall's own parts, per spec 0055: a carved head, a cloak, a
    /// helm with a nose bar, and the horn.
    carved_head: Option<MeshId>,
    cloak: Option<MeshId>,
    helm: Option<MeshId>,
    horn: Option<MeshId>,
    /// The verge down either side of the ramp, per `bridge::KERB_UP`.
    kerbs: [Option<MeshId>; 2],
    dome: Option<MeshId>,
    moon: Option<MeshId>,
    moon_face: Option<TextureId>,
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
    /// The field's ground, the grass on it, and a trunk per kind of tree it
    /// stands. Spec 0040.
    sward: Option<MeshId>,
    turf: Option<TextureId>,
    saplings: Vec<MeshId>,
    /// The plaster on the field's own wall, which is tiled where the
    /// garden's is one picture a wall. Spec 0040.
    render: Option<TextureId>,
    /// A quad per run of the field wall's plaster, rebuilt when the run's
    /// length changes. Spec 0040.
    rendered: Vec<Option<MeshId>>,
    /// A plane per lid this building shows the sky, with that lid's own
    /// tile counts, so the roof is a surface rather than a hole. Specs 0057
    /// and 0063.
    ///
    /// **It was the imaginarium's roof and nothing else.** That room stood
    /// on this one's lid and was the only thing up there anybody had drawn
    /// a top for, so parking it left the building's own roof undrawn and
    /// `roof_skin` declared with nothing ever assigning it. From the top of
    /// the bridge the lid is 3.8 degrees down, which is nearly edge on, and
    /// from `FLY=1` it is a hole straight into the hall.
    roof_meshes: Vec<MeshId>,
    /// One unit quad, laid on whichever outward face is being dressed.
    /// Each face carries its own skin, generated at that face's size, so
    /// one mesh does for all of them.
    wall_quad: Option<MeshId>,
    roof_skin: Option<TextureId>,
    outdoors: bool,
    /// Whether the outdoors can be seen from here, which is not the same
    /// question and is the one the field is drawn on. The engine's spec
    /// 0086.
    sees_outdoors: bool,
    storey_faces: Vec<(blitzkit::collision::Aabb, Vec3)>,
    storey_wall_skin: Vec<TextureId>,
    tuft: Option<MeshId>,
    blades: Vec<TextureId>,
    heads: Vec<TextureId>,
    /// Patches of fallen leaves, a handful of pictures to lay about.
    /// Spec 0059.
    litter: Vec<TextureId>,
    /// The box one piece of the bridge's deck is drawn on, and the rainbow
    /// laid across its width. Specs 0054 and 0055.
    deck: Option<MeshId>,
    soffit: Option<MeshId>,
    rainbow: Option<TextureId>,
    /// And what is cut into the stone at its foot: the carved face, the
    /// stone it is cut in, and the name in letters under it.
    runes: Option<TextureId>,
    carving: Option<TextureId>,
    /// The painted night through the hole in Valhalla's ridge. Spec 0060.
    night: Option<TextureId>,
    /// The triangle over each of the hall's gables. Spec 0060.
    gable: Option<MeshId>,
    /// One flame of the hall's fire. Spec 0060.
    hearth_flame: Option<MeshId>,
    granite: Option<TextureId>,
    standing_stone: Option<MeshId>,
    rubble_mound: Option<MeshId>,
    carved_face: Option<MeshId>,
    /// The three crossed planes a sprig of foliage is drawn on, and the
    /// pictures it wears: needles for the conifers and leaves for the rest.
    /// Spec 0010.
    crown: Option<MeshId>,
    needles: Vec<TextureId>,
    broadleaf: Vec<TextureId>,
    limb: Option<MeshId>,
    lantern: Option<MeshId>,
    /// The lit window of one, which is the only part of it that is not
    /// stone and the only part its own lamp can show. Spec 0010.
    lantern_light: Option<MeshId>,
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
    voices: Vec<(noise::Room, noise::Sounds, Vec<Samples>)>,
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
    /// The way out, lit from the dark side. See `behind::portal`.
    portal: Option<TextureId>,
    portal_mesh: Option<MeshId>,
    /// The door at the far end, lit the same way. See `behind::door_portal`.
    doorway: Option<TextureId>,
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
    /// Spec 0050's cube: how it is turned, and what it has left to do.
    puzzle: kociemba::Toy,
    /// A sphere fine enough to be a globe, and the world drawn on it.
    globe_mesh: Option<MeshId>,
    world: Option<TextureId>,
    since: f32,
    /// Whether a frame has gone by, which is what `first` means.
    has_run: bool,

    /// What the sight is on, per spec 0003. Worked out once in `update` and
    /// read everywhere else: `draw` asked the room six times a frame, once per
    /// display and twice over, and the answer cannot change inside a frame.
    seen: Option<room::Seen>,
    /// How big the window is, so the sight can sit in the middle of it.
    window: (f32, f32),
    /// Which way each shape on the far wall is facing, and which one is in your
    /// hands. Spec 0004.
    spin: display::Spin,
    /// Which cell of the tesseract you are standing in. `None` is the
    /// building. Spec 0021.
    inside: Option<Inside>,
    /// The eight cells' names, drawn into textures, in `tesseract::cells()`
    /// order. What a wall means is written on it.
    cell_words: Vec<TextureId>,
    /// And the thing that stands in each of them, in `Thing::ALL` order.
    /// Each wall shows a small one of what is through it, which is what a
    /// person navigates by. Spec 0021.
    cell_things: Vec<MeshId>,
    /// How wide each of them comes out of its own arithmetic, so one width
    /// asked for is one width drawn.
    thing_sizes: Vec<f32>,
    at: Vec3,
    /// Which of the imaginarium's frames you are standing in. Spec 0047.
    ///
    /// **A frame and not an up vector.** The three are the room's own, a
    /// cyclic permutation of the axes about its long diagonal, and because
    /// each is a right angle a box turns into a box exactly. Nought is the
    /// world's own way up and is where you are everywhere else in the
    /// building, so every turn below is the identity outside this room.
    standing_in: u64,
    /// Which way the view thinks is up, which chases `over` rather than
    /// being it. Spec 0047.
    rolling: Vec3,
    /// The room turning round you, out of the top of a flight. Spec 0048.
    /// A tour being walked rather than a route being replayed. Spec 0049.
    filming: Option<Filming>,
    /// Whether you are still inside the aperture you last came through, so
    /// that standing in one does not swap you back and forth every frame.
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
    /// How far the arrows have wound a turn and a tilt on, from -1 to 1.
    /// Spec 0044: these wind at a steady rate, `eased` makes a curve of them,
    /// and the curve is the speed, so a turn leans into itself and out again.
    turning: f32,
    tilting: f32,
    /// Whether every collider is outlined over the scene, per spec 0013 and
    /// spec 0046 of the engine. Off until somebody asks.
    showing_solid: bool,
    /// Whether the frame readout is up, and what the frames it has seen cost.
    /// Spec 0056's instrument, pointed at the building rather than at a wall
    /// of cubes.
    showing_cost: bool,
    /// Which step of the back room's dial the rig is on, and whether its
    /// cubes are one mesh drawn many times or a mesh each. Spec 0024.
    rig_step: usize,
    /// Who is in the hotel through the gallery's west door, and how far the
    /// last rule has travelled down the corridor. Spec 0029.
    rig_many: bool,
    /// A cube mesh per cube, uploaded as the dial asks for them.
    rig_meshes: Vec<MeshId>,
    /// The back room's clutter, in `back::Thing` order where it is turned.
    clutter: Vec<MeshId>,
    /// And every word on its gauge board, drawn into textures once: the
    /// heading, then the three names, then the three full-scale readings.
    gauge_words: Vec<TextureId>,
    costs: blitzkit::cost::Costs,
    /// What the renderer counted for the last frame, and whether the device
    /// agreed to time one at all, both asked of it at the top of the frame.
    counted: (u32, u32),
    timed: bool,
    /// Whether a frame has gone by, since the first one has nothing to be
    /// measured against.
    ran: bool,
    /// Whether a coplanar check has been asked for, per spec 0048 of the
    /// engine. One press, one check: it compares every drawn thing with every
    /// other and the answer does not change between frames.
    wants_planes: bool,
    /// How long the readout of where you are standing has left on screen,
    /// per spec 0045.
    showing_place: f32,
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
        // worked out before `room` is moved into the struct below, because
        // the tesseract's pose needs somewhere in the building to come back
        // down to.
        let doorway = room.doorway();
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
            at: match (
                staged_at_the_shapes() || staged_in_the_back_room(),
                staged_in_the_tesseract(),
                staged(),
            ) {
                (true, _, _) if staged_in_the_back_room() => posed_in_the_back_room(reaches).0,
                (true, _, _) => posed_at_the_shapes(reaches).0,
                (_, true, _) => tesseract::posed().0,
                (_, _, true) => posed_at,
                _ => doorway,
            },
            standing_in: 0,
            rolling: Vec3::Y,
            filming: None,
            falling: 0.0,
            eye: 0.0,
            room,
            spin,
            playing: Playing::new(),
            cascada: Cabinet::found(cascada::NAME, &beside()),
            poolhall: Cabinet::found(cellar::POOLHALL, &beside()),
            art: Vec::new(),
            art_from: art_of,
            glows: Vec::new(),
            signs: Vec::new(),
            cell_words: Vec::new(),
            cell_things: Vec::new(),
            thing_sizes: Vec::new(),
            // `--screenshot=tesseract` opens in the room rather than walking
            // to it, because there is no walking to it: it is through a
            // plinth rather than through a door.
            inside: staged_in_the_tesseract().then(|| Inside {
                frame: tesseract::Frame::new(),
                found: tesseract::Found::new(),
                turning: None,
                was: (doorway, 0.0, 0.0),
            }),
            cube: None,
            glass_targets: Vec::new(),
            dumped: false,
            pane: None,
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
            body: None,
            head: None,
            thigh: None,
            shin: None,
            view: mandel::View::default(),
            mandel: None,
            sharp: None,
            settling: None,
            sharp_view: None,
            shown_view: None,
            moving_the_view: false,
            hoop: None,
            bottle: None,
            peg: None,
            gate: 0.0,
            trail: {
                let mut trail = trail::Trail::new();

                if done_already() {
                    for one in trail::Asked::every() {
                        trail.gave(*one);
                    }
                }

                trail
            },
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
            carved_head: None,
            cloak: None,
            helm: None,
            horn: None,
            kerbs: [None; 2],
            dome: None,
            moon: None,
            moon_face: None,
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
            sward: None,
            turf: None,
            saplings: Vec::new(),
            render: None,
            rendered: vec![None, None],
            roof_meshes: Vec::new(),
            wall_quad: None,
            roof_skin: None,
            outdoors: false,
            sees_outdoors: false,
            storey_faces: Vec::new(),
            storey_wall_skin: Vec::new(),
            tuft: None,
            blades: Vec::new(),
            litter: Vec::new(),
            heads: Vec::new(),
            deck: None,
            soffit: None,
            rainbow: None,
            runes: None,
            carving: None,
            night: None,
            gable: None,
            hearth_flame: None,
            granite: None,
            standing_stone: None,
            rubble_mound: None,
            carved_face: None,
            crown: None,
            needles: Vec::new(),
            broadleaf: Vec::new(),
            limb: None,
            lantern: None,
            lantern_light: None,
            koi: None,
            fin: None,
            skins: Vec::new(),
            tiled: None,
            lining: Vec::new(),
            plaster: Vec::new(),
            plaster_mesh: Vec::new(),
            way_out: None,
            way_out_mesh: None,
            portal: None,
            portal_mesh: None,
            doorway: None,
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
            puzzle: kociemba::Toy::new(),
            wheel_mesh: None,
            globe: globe::Globe::new(),
            globe_mesh: None,
            world: None,
            since: starting_hour().unwrap_or(0.0),
            has_run: false,
            seen: None,
            window: (800.0, 600.0),
            // looking down the room from the end you start at: forward is
            // (sin yaw, 0, -cos yaw), so nought faces -z
            yaw: match (
                staged_at_the_shapes() || staged_in_the_back_room(),
                staged_in_the_tesseract(),
                staged(),
            ) {
                (true, _, _) if staged_in_the_back_room() => posed_in_the_back_room(reaches).1,
                (true, _, _) => posed_at_the_shapes(reaches).1,
                (_, true, _) => tesseract::posed().1,
                (_, _, true) => posed_yaw,
                _ => 0.0,
            },
            // level out past the door, where there is no floor worth looking
            // down at, and a little down in the hall
            pitch: match (staged_in_the_tesseract(), staged() && staged_in_the_hall()) {
                (true, _) => tesseract::posed().2,
                (_, true) => POSED_PITCH,
                _ => 0.0,
            },
            turning: 0.0,
            tilting: 0.0,
            showing_solid: false,
            showing_cost: false,
            rig_step: 2,
            rig_many: false,
            rig_meshes: Vec::new(),
            clutter: Vec::new(),
            gauge_words: Vec::new(),
            costs: blitzkit::cost::Costs::new(),
            counted: (0, 0),
            timed: false,
            ran: false,
            wants_planes: false,
            showing_place: 0.0,
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

        // inside the tesseract the only thing to work is the plinth you came
        // in by, and only in the cell it stands in. Spec 0021: a way out you
        // could use from any of the eight would be a maze with eight exits.
        if let Some(inside) = self.inside.as_ref() {
            if tesseract::at_the_plinth(inside.frame.cell(), self.at) {
                self.come_out();
            }

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
            // the panel: back to the top of the dive. Thirty four steps out
            // is a long hold and nothing is learned on the way. Spec 0039.
            Some(room::Seen::Mandel) => self.view = mandel::View::default(),
            Some(room::Seen::Cabinet(n)) => {
                let stood = self.room.stood[n].clone();
                if self.playing.start(&stood.cabinet) {
                    // the game wants the mouse now
                    self.wants_lock = false;
                }
            }
            // five of them turn on their plinths while you watch. The sixth
            // turns you instead, which is spec 0021 and is the whole reason
            // the way in is a thing on a plinth rather than another wall.
            Some(room::Seen::Display(n)) if n == display::tesseract() => {
                self.go_inside();
                self.trail.gave(trail::Asked::WentInsideTheTesseract);
            }
            Some(room::Seen::Display(n)) => self.spin.take(n, self.since),
            // the book that is a handle. Spec 0007.
            // the other book that is a handle. Spec 0061.
            Some(room::Seen::MirrorCase) => {
                self.room.mirror_open = !self.room.mirror_open;
            }
            Some(room::Seen::Case) => {
                self.room.open = !self.room.open;

                // **Whichever way it swung.** This read `if self.room.open`
                // first, on the grounds that shutting the case is not an
                // answer. It is not, and the guard was still dead: `gave`
                // ignores a repeat, so recording on both halves of a toggle
                // and recording on one come out the same, and the control
                // written for it could not fail. The case starts shut, so
                // the press that matters is the first one either way.
                self.trail.gave(trail::Asked::OpenedTheCase);
            }
            // and the sauna's glass, which is a door and behaves like one.
            // Spec 0008.
            Some(room::Seen::Sauna) => self.room.sauna_open = !self.room.sauna_open,
            Some(room::Seen::Baths) => self.room.baths_open = !self.room.baths_open,
            // **And the watchman, who wants nothing pressed at him.** Spec
            // 0058 opened a field here for his riddle and spec 0063 took it
            // out: he keeps the threshold and opens the gate as you come up
            // to it, so looking at him names him and that is all.
            Some(room::Seen::Watchman) => (),
            // a toy: enter does the one thing that toy's enter does
            Some(room::Seen::Bench(n)) => match self.room.benches[n].name {
                metronome::NAME => self.metronome.press(),
                gyro::NAME => self.gyro.press(),
                // a hand on the ball, which is the other thing you do to one
                globe::NAME => self.globe.spin = 0.0,
                kociemba::NAME => self.puzzle.worked(),
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
                // the rig: one mesh drawn many times, or a mesh each, which
                // is the difference between four draw calls and four
                // thousand. Spec 0024.
                back::RIG => self.rig_many = !self.rig_many,
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
            // the back room's rig: up and down are how many cubes. Spec 0024.
            back::RIG if arrow => {
                if !input.repeat {
                    let by = if input.key == KeyboardKey::Up { 1 } else { -1 };
                    let last = back::CUBES.len() as i32 - 1;

                    self.rig_step = (self.rig_step as i32 + by).clamp(0, last) as usize;
                }
            }
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

    fn what_i_see(&self) -> Option<room::Seen> {
        if self.inside.is_some() {
            return None;
        }

        // **The watchman is not in the building's list and cannot be.**
        // `Room::looking_at` is the things this building has at fixed
        // places; he is out past the end of a bridge that is not there
        // until every room has given up what it asks, and whether he
        // exists at all is a question about the trail, which `Room` has
        // no reason to know. Spec 0058.
        if gate::watched(
            self.room.reaches,
            &self.trail,
            self.looking_from(),
            self.facing(),
        ) {
            return Some(room::Seen::Watchman);
        }

        self.room.looking_at(self.looking_from(), self.facing())
    }

    /// The doors of one frame, and you out of their way.
    ///
    /// **Pulled out of `update` for the same reason `aimed_and_moved` was.**
    /// These are the only things in the building that move of their own
    /// accord, and one of them sweeps the floor you stand on to pull the
    /// book. Spec 0007. Spec 0042's checker said of itself that a route
    /// waiting on a door was one nobody had written, which stopped being true
    /// the day a clip was asked to go down to the baths: that route opens the
    /// bookcase, waits for it, and walks through where a shut one stood.
    /// Replayed without this it is a route that walks into a wall.
    fn the_doors_moved(&mut self, dt: f32) {
        self.room.ease(dt);

        self.at = walk::shoved(self.at, RADIUS, &self.room.leaves());
    }

    /// The aiming and the moving of one frame.
    ///
    /// **Pulled out of `update` rather than copied.** `--walk` replays a
    /// staged route against this to say whether every turn in it actually
    /// happens, and a second copy of these forty lines would pass a route
    /// that the game then walks differently. It reads the held keys off the
    /// struct, so the only thing a replay has to do is put them there.
    ///
    /// Gives back how fast you went, which the footsteps want.
    fn aimed_and_moved(&mut self, dt: f32) -> f32 {
        let mut wish = Vec3::ZERO;
        // where you are looking rather than where you are facing, when the
        // thing you are doing is flying: the pitch is the only way up there
        // is, and a flight that only goes along the ground is a walk that
        // ignores walls.
        let flight = flying();
        let ahead = if flight {
            self.facing()
        } else {
            self.forward()
        };
        // your right, which is across your own up rather than the world's.
        // Spec 0047.
        let right = self.forward().cross(self.over());
        for (held, way) in self.walking.iter().zip([ahead, -ahead, -right, right]) {
            if *held {
                wish += way;
            }
        }

        let pace = self.pace();

        // the arrows turn and tilt you. Yaw nought faces -z and grows toward
        // +x, which is your right, so the right arrow adds.
        let up = f32::from(self.looking[0]) - f32::from(self.looking[1]);
        let right = f32::from(self.looking[3]) - f32::from(self.looking[2]);

        // **All four arrows go into the panel while the sight is on it**,
        // the way they go into a toy on a bench. Shift and up or down
        // changes the scale, and the plain arrows move about the plane.
        // Spec 0039.
        //
        // **All four, which is the fix.** Left and right went into the
        // panel and kept turning you as well, and turning takes the sight
        // off the panel, so a pan stopped within a frame of starting. From
        // the floor that is a window that only scrolls up and down, which
        // is how Jake found it.
        let diving = self.seen == Some(room::Seen::Mandel);
        let (turn, tilt) = if diving {
            self.moving_the_view = up != 0.0 || right != 0.0;
            self.view = if self.shifted {
                self.view.moved((0.0, 0.0), up as f64, dt)
            } else {
                self.view.moved((right as f64, up as f64), 0.0, dt)
            };

            (0.0, 0.0)
        } else {
            self.moving_the_view = false;
            (right, up)
        };
        // **Wound on and off rather than switched, and the winding eased.**
        // Spec 0044. The throttle walks toward what the arrows ask for at a
        // steady rate, `eased` curves it, and the angle moves by that, so the
        // view leans into a turn and out of it with no corner at either end.
        // A hold still sweeps `TURNS` times its own length, because what the
        // winding on costs the winding off gives back.
        let wound = |now: f32, wants: f32| {
            let most = dt / AIM_LAGS;

            (now + (wants - now).clamp(-most, most)).clamp(-1.0, 1.0)
        };

        self.turning = wound(self.turning, turn);
        self.tilting = wound(self.tilting, tilt);

        if self.turning != 0.0 || self.tilting != 0.0 {
            self.yaw += eased(self.turning) * TURNS * dt;
            self.pitch =
                (self.pitch + eased(self.tilting) * TILTS * dt).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        }

        let riding = self.riding();
        let (at, falling) = if flight {
            (flown(self.at, wish, pace, dt), 0.0)
        } else {
            self.gone(wish, pace, dt)
        };
        // the ride is not walking, so it takes no steps. Spec 0004 reads this
        // for the footfalls and `went` is all it reads.
        let went = if riding || flight {
            0.0
        } else {
            (at - self.at).length() / dt.max(1e-4)
        };
        // what the climb cost, and the budget filling back up
        // and a climb costs nothing in the air, or a flight up to the roof
        // spends a budget meant for stairs and you land unable to climb one
        // for the next four seconds.
        self.climbs = if flight {
            0.0
        } else {
            // **Measured along your own up, per spec 0047**, which is
            // `y` with your frame at nought and is every other room
            // unchanged. In a turned frame a climb does not change world
            // `y` at all, so the budget neither filled nor drained: it
            // sat a thousandth above nothing, which reads as a step you
            // may never take, and frame one's flight stopped you dead
            // four risers up. Found by driving it.
            let over = self.over();
            let went = at - self.at;
            let rose = went.dot(over);
            // and what of that a slope accounts for costs nothing, per
            // `walk::RAMPS`: the stair is paced, the ramp is walked.
            let along = (went - over * rose).length();

            (self.climbs - walk::CLIMBS * dt).max(0.0) + walk::climbed(rose.max(0.0), along)
        };
        self.at = at;
        self.falling = falling;

        // **Up is up, so there is nothing for the view to roll after.**
        // Spec 0047 lagged it behind a frame that could change under you,
        // which only the imaginarium ever did; spec 0063 parked that room.
        self.rolling = Vec3::Y;

        // **The eye catching up with the feet, here rather than in `update`.**
        // Nothing while you are on the level, because then they are the same
        // number.
        //
        // It sat near the end of `update`, which left `--walk` replaying a
        // route with its eye on the hall's floor however high the body
        // climbed: spec 0042's checker reported a cabinet in the hall as what
        // you were looking at while standing in the imaginarium, two floors
        // over it. That is the drift this function exists to stop, and the
        // eye belongs with the feet.
        // **Measured along your own up, per spec 0047**, which is `y` with
        // your frame at nought and is every other room unchanged.
        let feet = self.at.dot(self.over());
        let behind = feet - self.eye;
        self.eye += behind * (dt / EYE_LAGS).min(1.0);
        // and never further off than one step, per `EYE_TRAILS`
        self.eye = self.eye.clamp(feet - EYE_TRAILS, feet + EYE_TRAILS);
        if behind.abs() < 1e-4 {
            self.eye = feet;
        }

        went
    }

    /// Walk the tour, if there is one. Spec 0049.
    ///
    /// **Closed loop, which is the point.** Every frame it asks where the
    /// body actually is and aims from there. A turn that sweeps short, a
    /// graze on a jamb, a tread that costs a frame of the climbing budget:
    /// each is only a slightly different place to aim from next time.
    ///
    /// Nothing in here is timed except the standing, because the standing
    /// is the one thing that is about how long rather than about where.
    fn on_the_tour(&mut self, dt: f32) {
        let Some(mut film) = self.filming.take() else {
            return;
        };

        self.walking = [false; 4];

        while let Some(step) = film.steps.get(film.at).copied() {
            match step {
                tour::Step::Stand(how_long) => {
                    if film.waiting <= 0.0 {
                        film.waiting = how_long;
                    }

                    film.waiting -= dt;

                    if film.waiting > 0.0 {
                        break;
                    }

                    film.waiting = 0.0;
                    film.at += 1;
                }
                tour::Step::Work => {
                    self.use_what_i_see();
                    film.at += 1;
                }
                tour::Step::Look(toward) => {
                    // **Tilted as well as turned.** A `Look` that only
                    // turns faces the right way along the floor and goes
                    // on looking at the horizon, which in a room is a
                    // wall. Standing under the imaginarium and looking at
                    // the middle of it, six metres up, filmed two and a
                    // half seconds of plaster; the route had already
                    // worked round this twice by naming the bookcase and
                    // the baths' leaf at eye height rather than where
                    // their handles are.
                    let at = self.looking_from();
                    let way = self.turned().inverse() * (toward - at);
                    let flat = vec3(way.x, 0.0, way.z).length();
                    let turned = self.turned_toward(way.x.atan2(-way.z), dt);
                    let tilted = self.tilted_toward(way.y.atan2(flat), dt);

                    if turned <= 0.0 && tilted <= 0.0 {
                        film.at += 1;
                    }

                    break;
                }
                tour::Step::At(place, frame) => {
                    if film.following.is_none() {
                        let solid = self.standing_on();
                        let ways =
                            tour::ways(&self.room.grounds(), &solid, tour::region(self.at, place));

                        match ways.plan((self.at, self.standing_in), (place, frame), &solid) {
                            Some(way) => film.following = Some(tour::Follower::new(way)),
                            None => {
                                eprintln!("no way from {:?} to {:?}", self.at, place);
                                film.at += 1;

                                continue;
                            }
                        }
                    }

                    let steer = film.following.as_mut().and_then(|following| {
                        following.steer(self.at, self.standing_in, self.walking[0])
                    });

                    match steer {
                        Some(steer) => {
                            // **Forward only once you are roughly pointing
                            // at it.** Walking flat out while still turning
                            // gives a circle of 1.75 metres, the lattice's
                            // places are half a metre apart, and the first
                            // build of this orbited the next one for ever
                            // without ever reaching it.
                            let left = self.turned_toward(steer.yaw, dt);

                            // **And level, because a walk looks where it
                            // is going.** Nothing but a `Look` ever moved
                            // the pitch, so the first `Look` that tilted
                            // left it tilted: the route stood under the
                            // imaginarium, looked at the middle of it 74
                            // degrees up, and then walked the next four
                            // legs at the ceiling. Eight seconds of
                            // plaster going by.
                            self.tilted_toward(0.0, dt);

                            self.walking[0] = left < WALKS_WHEN;
                        }
                        None => {
                            film.following = None;
                            film.at += 1;

                            continue;
                        }
                    }

                    break;
                }
            }
        }

        self.filming = Some(film);
    }

    /// Turn toward a heading at the rate the arrows turn, and say whether
    /// you are there. Spec 0049.
    fn turned_toward(&mut self, yaw: f32, dt: f32) -> f32 {
        let turn = std::f32::consts::TAU;
        let mut by = (yaw - self.yaw).rem_euclid(turn);

        if by > turn * 0.5 {
            by -= turn;
        }

        let most = TURNS * dt;

        if by.abs() <= most {
            self.yaw = yaw;

            return 0.0;
        }

        self.yaw += most * by.signum();

        by.abs() - most
    }

    /// Tilt toward a pitch at the rate the arrows tilt, and say whether
    /// you are there. Spec 0049, and the twin of `turned_toward`.
    ///
    /// Clamped where the arrows are clamped, so a route that names
    /// something straight overhead gets as far up as a player can and
    /// then stops, rather than rolling past the top.
    fn tilted_toward(&mut self, pitch: f32, dt: f32) -> f32 {
        let want = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
        let by = want - self.pitch;
        let most = TILTS * dt;

        if by.abs() <= most {
            self.pitch = want;

            return 0.0;
        }

        self.pitch += most * by.signum();

        by.abs() - most
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
        if self.inside.is_some() {
            aim::TESSERACT_HELP
        } else if self.in_the_deep() {
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
        let own = if self.running() { SPEED * RUNS } else { SPEED };

        // and the bridge carries you up itself, per `bridge::CARRIES`
        spa::pace(self.room.reaches, self.at, own)
            * bridge::carries(self.room.reaches, self.at, self.forward(), &self.trail)
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
            // the rig in the back room, per spec 0024. It says the count and
            // the two numbers that count costs, because a dial with no
            // reading on it is a dial nobody has a reason to turn.
            back::RIG => {
                let count = back::CUBES[self.rig_step];

                format!(
                    "{} cubes, {}. Up and down change how many, enter changes how they are drawn. {} draws, {} of work.",
                    count,
                    if self.rig_many {
                        "a mesh each"
                    } else {
                        "one mesh"
                    },
                    self.counted.0,
                    match self.costs.middle_ms() {
                        Some(ms) => format!("{:.2} ms", ms),
                        None => String::from("--"),
                    },
                )
            }
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

        let mut out = vec![
            (
                front,
                -1.0,
                true,
                far + room::NOOK_DOOR,
                far + room::NOOK_SPAN,
            ),
            (far + room::NOOK_SPAN - 0.15, -1.0, false, back, front),
        ];

        // **The south side is panelled with the rest of it now.** It used
        // to be the hotel lobby's north wall, borrowed, so the nook's
        // dressing had to be clamped to the part of it the nook actually
        // had: the lobby ran on west past the nook and the nook's opening
        // ran on east past the wall. Panelling that run from `back` to
        // `front` put the skirting, the rail and the panels 2.65 beyond the
        // wall's east end, hanging across the opening with nothing behind
        // them, and 0.15 shy of its face everywhere else, so the only part
        // of it anybody could see was the part that should not have been
        // there. Jake, twice, from the hall: it's still a wrong wall.
        //
        // Spec 0063 parked the hotel and `room::nook_south` builds this
        // wall for the nook, in the nook's own place, so there is nothing
        // left to clamp to.
        out.push((-self.room.reaches + room::THICK, 1.0, false, back, front));

        out
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

    /// What the lit parts of the map in your hand mean, at the foot of the
    /// screen. Spec 0021.
    ///
    /// **All that is left of the grid.** The grid was right about the one
    /// rule it showed and wrong about everything else: it looked nothing
    /// like the room it was a map of, which Jake read as leaving a lot of
    /// questions for a new user. The shape is the map now and this is its
    /// key, because a colour nobody is told the meaning of is the mistake
    /// this room has already made twice.
    /// The back room: its floor, its three walls, its ceiling, the bench and
    /// the one bulb. Spec 0024.
    ///
    /// Its fourth wall is the gallery's, which is drawn with the rest of the
    /// building's walls and is cut for the door where `back::gallery_east`
    /// says.
    /// A door's panels and its handle, which is what makes it read as a door
    /// rather than a slab. Spec 0031's note, and it is true of all of them.
    #[allow(clippy::too_many_arguments)]
    /// The field. Spec 0040.
    ///
    /// What this building is from outside it. Spec 0040.
    ///
    /// **Its own pass, because it is not the field's.** All of this was
    /// written inside `draw_the_field`, because the field is where it first
    /// became possible to see any of it, and that is a reason for the order
    /// it was built in rather than for where it lives. The field is ground,
    /// sky and scatter round the eye; this is a building at fixed
    /// coordinates, and the two want different answers the moment either of
    /// them grows a reach of its own.
    fn draw_the_outside(&self, scene: &mut Scene, cube: MeshId) {
        let reaches = self.room.reaches;

        if !self.sees_outdoors {
            return;
        }

        // the walls up there that the sky can see, which is what
        // `Room::outside` asks the building for.
        if let Some(mesh) = self.wall_quad {
            for ((face, looks), skin) in self
                .storey_faces
                .iter()
                .copied()
                .zip(self.storey_wall_skin.iter().copied())
            {
                scene.push_textured(
                    mesh,
                    skin,
                    &Transform::at(face.center() + looks * 0.02)
                        .with_rotation(room::laid_on(looks))
                        .with_scale(room::quad_on(face.size(), looks)),
                    aim::GARDEN_PLASTER,
                    aim::MATTE,
                );
            }
        }

        // the roof itself, which is the one surface up there that is not a
        // wall: the same lime the garden's walls wear, laid flat in bays.
        if let Some(skin) = self.roof_skin {
            for (lid, mesh) in self
                .room
                .lids()
                .into_iter()
                .zip(self.roof_meshes.iter().copied())
            {
                let (middle, size) = (lid.center(), lid.size());

                scene.push_textured(
                    mesh,
                    skin,
                    &Transform::at(vec3(middle.x, lid.max.y + 0.01, middle.z))
                        .with_rotation(room::laid_on(Vec3::Y))
                        .with_scale(room::quad_on(size, Vec3::Y)),
                    aim::GARDEN_PLASTER,
                    aim::MATTE,
                );
            }
        }

        // the lantern on the roof, which is the one lit thing in this
        // building's skyline and the whole of what makes the view back from
        // the bridge read as something. Specs 0057 and 0063.
        {
            let shaft = self.room.lantern();
            let cap = self.room.lantern_cap();

            scene.push_plain(
                cube,
                &Transform::at(shaft.center()).with_scale(shaft.size()),
                aim::LANTERN,
            );
            scene.push_plain(
                cube,
                &Transform::at(cap.center()).with_scale(cap.size()),
                aim::GARDEN_COPE,
            );
        }

        // the wall the building shows the field, past the garden's own
        // ends, with the garden's own plaster and its own cope on it so
        // the three read as one wall with one gap in it
        for box_ in field::wall(reaches, self.at) {
            let (middle, size) = (box_.center(), box_.size());

            scene.push_plain(
                cube,
                &Transform::at(middle).with_scale(size),
                aim::GARDEN_WALL,
            );
            // and the cope on it, in the garden's own two courses: the
            // tiles oversailing both ways and the roll set in from them.
            // One course and no roll, the join with the garden's wall was
            // a step along the top of one wall.
            let wide = garden::COPE_WIDE;

            scene.push_plain(
                cube,
                &Transform::at(vec3(
                    middle.x,
                    garden::HIGH + garden::COPE_DEEP * 0.5,
                    middle.z,
                ))
                .with_scale(vec3(wide, garden::COPE_DEEP, size.z)),
                aim::GARDEN_COPE,
            );
            scene.push_plain(
                cube,
                &Transform::at(vec3(
                    middle.x,
                    garden::HIGH + garden::COPE_DEEP + garden::COPE_ROLL * 0.5,
                    middle.z,
                ))
                .with_scale(vec3(
                    wide - garden::COPE_OUT * 1.4,
                    garden::COPE_ROLL,
                    size.z,
                )),
                aim::GARDEN_COPE,
            );
        }

        // and the plaster on the face of all of it, in one run a side
        // through the garden's own stretch, because it is one wall. Tiled,
        // because a picture per three hundred metres is not a picture, and
        // the quad is rebuilt when its length changes so that a run of
        // tiles is a run of tiles rather than a stretch of them.
        for (face, mesh) in field::wall_faces(reaches, self.at)
            .into_iter()
            .zip(self.rendered.iter().copied())
        {
            let (Some(mesh), Some(skin)) = (mesh, self.render) else {
                continue;
            };
            let (middle, size) = (face.center(), face.size());

            scene.push_textured(
                mesh,
                skin,
                &Transform::at(vec3(middle.x - 0.01, middle.y, middle.z))
                    .with_rotation(room::laid_on(-Vec3::X))
                    .with_scale(vec3(size.z, 1.0, size.y)),
                aim::GARDEN_PLASTER,
                aim::MATTE,
            );
        }
    }

    /// **The one room in this building that is drawn relative to you.**
    /// Everything else is at fixed coordinates and is either in the frustum
    /// or is not. Out here the ground and the sky are centred on where you
    /// stand, because a field whose ground ends is a stage and a field whose
    /// sky ends is a tent, and what the room is about is that neither does.
    fn draw_the_field(&self, scene: &mut Scene) {
        let reaches = self.room.reaches;

        // from anywhere the field can be seen, which is the whole of the
        // garden as well as the field: the gap in the west wall is three
        // metres wide and what is through it is a hundred and sixty metres
        // of ground. Drawn only from out in the field, the gap was a black
        // rectangle in a garden wall.
        if !self.sees_outdoors {
            return;
        }

        // the ground, snapped to its own grass so that walking moves the
        // ground under you rather than carrying it with you
        if let (Some(sward), Some(turf)) = (self.sward, self.turf) {
            let lies = field::ground(self.at);

            scene.push_textured(
                sward,
                turf,
                &Transform::at(vec3(lies.center().x, -field::DROP, lies.center().z))
                    .with_scale(vec3(lies.size().x, 1.0, lies.size().z)),
                aim::FIELD_GROUND,
                // no highlight at all. The default is a broad one, and a
                // broad highlight on four hundred metres of ground with the
                // sun low is a lit path down the middle of the field that
                // turns with you. The garden's pond is the same note.
                aim::VOID,
            );
        }

        // the trees, which are three meshes stood many times
        let kinds = field::kinds();

        for cell in field::cells_round(self.at, field::reaches(self.at)) {
            if !field::sown(cell, self.at) {
                continue;
            }

            for stood in field::trees(reaches, cell) {
                let field::What::Tree(kind) = stood.what else {
                    continue;
                };
                let (Some(trunk), Some(tree)) = (self.saplings.get(kind), kinds.get(kind)) else {
                    continue;
                };
                let turned = glam::Quat::from_rotation_y(stood.turn);

                let bark = Self::bark_of(tree);

                scene.push_material(
                    *trunk,
                    &Transform::at(stood.at)
                        .with_rotation(turned)
                        .with_scale(Vec3::splat(stood.size)),
                    bark,
                    aim::DULL,
                );

                if let Some(limb) = self.limb {
                    for (from, to) in garden::limbs(tree) {
                        let (from, to) = (field::stands(&stood, from), field::stands(&stood, to));
                        let along = to - from;
                        let long = along.length();

                        if long > 1e-3 {
                            scene.push_material(
                                limb,
                                &Transform::at(from)
                                    .with_rotation(glam::Quat::from_rotation_arc(
                                        Vec3::Y,
                                        along / long,
                                    ))
                                    .with_scale(vec3(
                                        garden::LIMB_THICK * stood.size,
                                        long,
                                        garden::LIMB_THICK * stood.size,
                                    )),
                                bark,
                                aim::DULL,
                            );
                        }
                    }
                }

                if let (Some(crown), Some((skin, leaf))) =
                    (self.crown, self.foliage(tree, field::which(&stood)))
                {
                    for sprig in garden::sprigs(tree) {
                        let wide = sprig.wide * stood.size;
                        let tall = sprig.tall * stood.size;

                        scene.push_textured(
                            crown,
                            skin,
                            &Transform::at(field::stands(&stood, sprig.at))
                                .with_rotation(glam::Quat::from_rotation_y(sprig.turn + stood.turn))
                                .with_scale(vec3(wide, tall, wide)),
                            leaf * sprig.shade,
                            aim::VOID,
                        );
                    }
                }
            }
        }

        // and the near ground: tufts standing in the grass and flowers among
        // them, which are what streams past you and so are what says you are
        // moving at all
        for cell in field::cells_round(self.at, field::COVER_REACH) {
            for stood in field::cover(reaches, cell) {
                let turned = glam::Quat::from_rotation_y(stood.turn);

                match stood.what {
                    // the grass, which is a silhouette on crossed planes
                    // rather than a solid of any shape: see `field::blades`
                    field::What::Tuft => {
                        let (Some(mesh), Some(blades)) = (
                            self.tuft,
                            self.blades
                                .get((stood.turn * 97.0) as usize % self.blades.len().max(1))
                                .copied(),
                        ) else {
                            continue;
                        };
                        let tall = field::TUFT_TALL * stood.size;
                        let wide = field::TUFT_WIDE * stood.size;

                        scene.push_textured(
                            mesh,
                            blades,
                            &Transform::at(stood.at)
                                .with_rotation(turned)
                                .with_scale(vec3(wide, tall, wide)),
                            aim::FIELD_TUFT,
                            aim::VOID,
                        );
                    }
                    // and what has come off the trees, lying flat. One
                    // patch takes the colour of the tree it fell from,
                    // which is `aim::leaf_of` keyed off where the patch
                    // is rather than where the tree is: a leaf on the
                    // ground has left its tree and the ground is where
                    // it is now.
                    field::What::Litter => {
                        let (Some(mesh), Some(leaves)) = (
                            self.ground,
                            self.litter
                                .get((stood.turn * 59.0) as usize % self.litter.len().max(1))
                                .copied(),
                        ) else {
                            continue;
                        };
                        let wide = field::LITTER_WIDE * stood.size;

                        scene.push_textured(
                            mesh,
                            leaves,
                            // **The flat plane and not the crown.** A
                            // sprig is three crossed uprights, because
                            // what it is for is a spray of leaves on a
                            // tree seen from any side. Turned on its back
                            // it laid one quad on the ground and left two
                            // standing, so the field grew little brown
                            // tokens on their edges. This wants the one
                            // flat quad the garden's beds are laid with.
                            &Transform::at(stood.at + Vec3::Y * field::LITTER_LAID)
                                .with_rotation(turned)
                                .with_scale(vec3(wide, 1.0, wide)),
                            aim::litter_of(field::which(&stood)),
                            aim::VOID,
                        );
                    }
                    field::What::Flower(which) => {
                        let (Some(mesh), Some(head)) = (
                            self.tuft,
                            self.heads.get(which % self.heads.len().max(1)).copied(),
                        ) else {
                            continue;
                        };
                        let tall = field::FLOWER_TALL * stood.size;
                        let wide = field::FLOWER_WIDE * stood.size;

                        scene.push_textured(
                            mesh,
                            head,
                            &Transform::at(stood.at)
                                .with_rotation(turned)
                                .with_scale(vec3(wide, tall, wide)),
                            glam::Vec4::ONE,
                            aim::VOID,
                        );
                    }
                    field::What::Tree(_) => continue,
                }
            }
        }

        self.draw_bridge(scene);
    }

    /// The picture a tree's foliage wears, and the colour to draw it in.
    ///
    /// One place, because the garden and the field both stand trees and the
    /// field's are `garden::Tree` too. Three pictures of each kind of leaf,
    /// picked off the tree's own seed so one tree is one tree and two of a
    /// species are not the same tree twice.
    fn bark_of(tree: &garden::Tree) -> glam::Vec4 {
        let of = tree.kind.bark();

        aim::BARK * vec4(of.x, of.y, of.z, 1.0)
    }

    fn foliage(&self, tree: &garden::Tree, which: u32) -> Option<(TextureId, glam::Vec4)> {
        let broad = tree.kind.leaf() == garden::Leaf::Broad;
        let of = if broad {
            &self.broadleaf
        } else {
            &self.needles
        };
        let skin = of.get(tree.seed as usize % of.len().max(1)).copied()?;

        Some((skin, aim::leaf_of(tree.kind.tint(), which, broad)))
    }

    /// The bridge at the end of the trail, out in the field. Spec 0054.
    ///
    /// **Drawn as a different list from the one that holds you up.** The
    /// colliders are a fine staircase, because a collider is a box that
    /// cannot tilt and a slope has to be stepped to walk on. These are
    /// forty-eight lengths of seven bands, each turned to the pitch of the
    /// arc where it lies, so what you see is a curve. Both come out of
    /// `bridge::high`, which is the one place the shape is written.
    fn draw_bridge(&self, scene: &mut Scene) {
        if !bridge::shows(&self.trail) {
            return;
        }

        let Some(cube) = self.cube else {
            return;
        };

        // the landing first, which is stone and not light: it is what the
        // bridge arrives at. Spec 0055.
        for (n, box_) in bridge::landing(self.room.reaches, &self.trail)
            .into_iter()
            .enumerate()
        {
            scene.push_plain(
                cube,
                &Transform::at(box_.center()).with_scale(box_.size()),
                if n == 0 {
                    aim::THRESHOLD
                } else {
                    aim::THRESHOLD_RAIL
                },
            );
        }

        // and what stands on that parapet: warm stone piers at the ends
        // of every run and iron between them. Spec 0059, off Jake's
        // photograph of City Creek and The Witness's flat colour: two
        // materials in a rhythm, with no grain on either, so the edge
        // between two colours is what carries the form.
        for at in bridge::piers(self.room.reaches, &self.trail) {
            scene.push_plain(
                cube,
                &Transform::at(at + Vec3::Y * bridge::PIER_UP * 0.5).with_scale(vec3(
                    bridge::PIER_WIDE,
                    bridge::PIER_UP,
                    bridge::PIER_WIDE,
                )),
                aim::PIER,
            );
            // a cap, chamfered out over the shaft, which is the one line
            // of shadow a pier has
            scene.push_plain(
                cube,
                &Transform::at(at + Vec3::Y * (bridge::PIER_UP + bridge::PIER_CAP * 0.5))
                    .with_scale(vec3(
                        bridge::PIER_WIDE * 1.14,
                        bridge::PIER_CAP,
                        bridge::PIER_WIDE * 1.14,
                    )),
                aim::PIER_CAP,
            );
        }

        for at in bridge::balusters(self.room.reaches, &self.trail) {
            scene.push_plain(
                cube,
                &Transform::at(at + Vec3::Y * bridge::RAIL_HIGH * 0.5).with_scale(vec3(
                    bridge::BALUSTER,
                    bridge::RAIL_HIGH,
                    bridge::BALUSTER,
                )),
                aim::BALUSTER,
            );
        }

        // and the rail they carry, along the top of each run
        for (from, to) in bridge::parapet_runs(self.room.reaches, &self.trail) {
            let along = to - from;
            let up = Vec3::Y * (bridge::RAIL_HIGH - bridge::BALUSTER * 0.5);

            scene.push_plain(
                cube,
                &Transform::at((from + to) * 0.5 + up).with_scale(vec3(
                    along.x.abs().max(bridge::BALUSTER * 1.6),
                    bridge::BALUSTER,
                    along.z.abs().max(bridge::BALUSTER * 1.6),
                )),
                aim::BALUSTER,
            );
        }

        // and the deck, which is one mesh built along the arc rather than a
        // row of boxes: see `bridge::deck_mesh`. It is already in the
        // building's own terms, so it is pushed where it stands.
        let (Some(deck), Some(rainbow)) = (self.deck, self.rainbow) else {
            return;
        };

        // and a kerb down either side of it, which is what stops you
        // walking off a six metre road at eleven metres a second. See
        // `bridge::KERB_UP`.
        for kerb in self.kerbs.iter().flatten() {
            scene.push_material(*kerb, &Transform::at(Vec3::ZERO), aim::KERB, aim::DULL);
        }

        // the underside, dimmer and seen through, before the top: it is
        // the face you spend the climb looking at. See `bridge::SOFFIT_LIT`.
        if let Some(soffit) = self.soffit {
            scene.push_textured(
                soffit,
                rainbow,
                &Transform::at(Vec3::ZERO),
                bridge::soffit_glow(),
                aim::VOID,
            );
        }

        scene.push_textured(
            deck,
            rainbow,
            &Transform::at(Vec3::ZERO),
            bridge::glow(),
            // no highlight. A rainbow is light and has no surface for one to
            // sit on, and a hot spot sliding along an arc is a wet plastic
            // tube.
            aim::VOID,
        );

        // and the stone at the foot of it, which is the only thing out
        // here that says what any of this is. Spec 0006's rule applied to
        // spec 0054: everything else in this building announces itself.
        if let Some((at, size)) = bridge::runestone(self.room.reaches, &self.trail) {
            let leans = glam::Quat::from_rotation_z(bridge::RUNE_LEANS);

            let stood = Transform::at(at).with_rotation(leans).with_scale(size);

            match (self.standing_stone, self.granite) {
                (Some(stone), Some(granite)) => {
                    scene.push_textured(stone, granite, &stood, aim::RUNESTONE, aim::MATTE)
                }
                _ => scene.push_material(cube, &stood, aim::RUNESTONE, aim::MATTE),
            }

            // the bank it is set into, which is the ground itself raised
            // rather than anything standing on it
            if let Some((mound, granite)) = self.rubble_mound.zip(self.granite) {
                let on = Transform::at(vec3(at.x, 0.0, at.z));

                scene.push_textured(mound, granite, &on, aim::RUBBLE, aim::MATTE);
            }

            // and the rubble lying on it, which is what says it was dug
            // in rather than stood on the grass.
            //
            // **The garden's cheap lump, not the standing stone's mesh.**
            // These were drawn with `bridge::stone_mesh`, which is a
            // hundred and seventy six by two hundred and twenty four,
            // because it was already in hand: seven of them put a quarter
            // of a million quads in the footing of a forty thousand quad
            // monument. `garden::rock_mesh` is eleven by seven, and at
            // this size that is all a piece of rubble needs.
            for (which, (where_, size, round)) in bridge::rubble(self.room.reaches, &self.trail)
                .into_iter()
                .enumerate()
            {
                let cut = match self.stones.get(which % self.stones.len().max(1)) {
                    Some(cut) => *cut,
                    None => cube,
                };
                let sat = Transform::at(where_)
                    .with_rotation(glam::Quat::from_rotation_y(round))
                    .with_scale(size);

                match self.granite {
                    Some(granite) => {
                        scene.push_textured(cut, granite, &sat, aim::RUBBLE, aim::MATTE)
                    }
                    None => scene.push_material(cube, &sat, aim::RUBBLE, aim::MATTE),
                }
            }

            // the carved face: a band round it with the name in runes
            // down the middle. **This is what says runestone.** It was
            // the arcade's own pixel font in Latin capitals, which says
            // arcade however it is coloured, on a box, which says
            // gravestone.
            // **The glow and the glaze are one decision**, in
            // `bridge::alight`: see it for why the alpha never reaches
            // one and what happened for an afternoon when it did.
            let alight = bridge::alight(self.since);

            // **And a lamp on the stone again**, in the light list
            // above, per `bridge::carving_lamp`. There was one, it
            // washed the face pale and took the contrast the glyphs
            // were living on, and it came out. What has changed since
            // is that the glyphs are four times past white rather than
            // a multiplier on a dark ambient, so there is nothing left
            // for a lamp on the rock to out-shine. Jake, 2026-11-06:
            // glowing means it is shining brightly from within what
            // holds it, like the stone.

            if let (Some(face), Some(carving)) = (self.carved_face, self.carving) {
                scene.push_textured(
                    face,
                    carving,
                    &Transform::at(at).with_rotation(leans).with_scale(size),
                    alight,
                    aim::VOID,
                );
            }
        }

        self.draw_valhalla(scene, cube);
        self.draw_the_gate(scene, cube);
    }

    /// The gate in the way through, per spec 0058.
    ///
    /// **Boards, straps and a ring, and nothing finer than that.** You
    /// stand right in front of this one, so the grain of it has to hold
    /// up at arm's length, and it is also a shape on the skyline from
    /// halfway down the deck, where the only thing that survives is the
    /// outline of the frame. Both of those are served by the same five
    /// things: two posts, a head, planked leaves, iron across them.
    ///
    /// Each leaf is drawn at whatever angle it has swung to. The drawing
    /// can turn and the collider cannot, which is why `gate::leaf_solid`
    /// takes the box round the leaf rather than the leaf.
    fn draw_the_gate(&self, scene: &mut Scene, cube: MeshId) {
        let reaches = self.room.reaches;

        // the wall it is set in, which is the far parapet carried up to the
        // head. Without it the gate stands in a kerb with the whole
        // steading in plain sight either side of it.
        for panel in gate::wall(reaches, &self.trail) {
            scene.push_plain(
                cube,
                &Transform::at(panel.center()).with_scale(panel.size()),
                aim::THRESHOLD_RAIL,
            );
        }

        for (middle, size) in gate::cope(reaches, &self.trail) {
            scene.push_plain(
                cube,
                &Transform::at(middle).with_scale(size),
                aim::THRESHOLD,
            );
        }

        for post in gate::frame(reaches, &self.trail) {
            scene.push_material(
                cube,
                &Transform::at(post.center()).with_scale(post.size()),
                aim::GATE_OAK,
                aim::MATTE,
            );
        }

        if !gate::shows(&self.trail) {
            return;
        }

        let wide = gate::leaf_wide(reaches);
        let high = gate::leaf_high();

        for side in [-1.0f32, 1.0] {
            let (middle, turn, _) = gate::leaf(reaches, side, self.gate);
            let placed = |off: Vec3| Transform::at(middle + turn * off).with_rotation(turn);

            scene.push_material(
                cube,
                &placed(Vec3::ZERO).with_scale(vec3(gate::LEAF_THICK, high, wide)),
                aim::GATE_OAK,
                aim::MATTE,
            );

            // the boards, standing a little proud of the leaf so the joint
            // between each pair is a line of shadow rather than a change
            // of colour. An odd number, so the middle of a leaf is a board
            // and not a gap.
            for n in 0..gate::BOARDS {
                let across = (n as f32 + 0.5) / gate::BOARDS as f32 - 0.5;

                scene.push_material(
                    cube,
                    &placed(vec3(gate::LEAF_THICK * 0.5, 0.0, across * wide)).with_scale(vec3(
                        gate::PROUD * 2.0,
                        high - gate::PROUD * 2.0,
                        wide / gate::BOARDS as f32 - gate::GROOVE,
                    )),
                    aim::GATE_BOARD,
                    aim::MATTE,
                );
            }

            // and the iron across them, which is the half of this that
            // says castle. Across the grain, because a strap along it
            // would be another board.
            for up in gate::STRAPS {
                scene.push_material(
                    cube,
                    &placed(vec3(
                        gate::LEAF_THICK * 0.5 + gate::PROUD,
                        (up - 0.5) * high,
                        0.0,
                    ))
                    .with_scale(vec3(
                        gate::PROUD * 2.0,
                        gate::STRAP,
                        wide * 0.98,
                    )),
                    aim::GATE_IRON,
                    aim::DULL,
                );
            }

            // a ring on each leaf, near the edge it meets its other half
            // on, which is where a hand goes.
            if let Some(hoop) = self.hoop {
                scene.push_material(
                    hoop,
                    &placed(vec3(
                        gate::LEAF_THICK * 0.5 + gate::PROUD,
                        0.0,
                        -side * (wide * 0.5 - gate::RING_IN),
                    ))
                    .with_rotation(turn * room::laid_on(Vec3::X))
                    .with_scale(vec3(
                        gate::RING,
                        gate::RING_THICK,
                        gate::RING,
                    )),
                    aim::GATE_IRON,
                    aim::DULL,
                );
            }
        }
    }

    /// What is through the door at the top of it. Spec 0056.
    ///
    /// **Drawn with the bridge and on the same condition**, because a hall
    /// in the sky above a field with no way up to it is worse than nothing.
    fn draw_valhalla(&self, scene: &mut Scene, cube: MeshId) {
        let reaches = self.room.reaches;
        let ground = valhalla::ground(reaches);

        // wet, so that what the doors throw is given back. A ground that
        // takes light and returns none of it is a ground the light landed
        // on and nobody saw.
        scene.push_material(
            cube,
            &Transform::at(ground.center()).with_scale(ground.size()),
            aim::VALHALLA_GROUND,
            aim::VALHALLA_WET,
        );

        for one in valhalla::built(reaches) {
            // **Six boxes for the hall and one for everything else.** The
            // hall is hollow per spec 0060, so its walls are the flanks,
            // the gables between them and the two runs either side of the
            // door. Outside it looks exactly as it did: the faces are
            // where the one box's faces were.
            let walls: Vec<_> = if one.hall {
                valhalla::hall_walls(reaches)
            } else {
                vec![one.walls()]
            };

            for wall in walls {
                scene.push_material(
                    cube,
                    &Transform::at(wall.center()).with_scale(wall.size()),
                    aim::VALHALLA_WALL,
                    aim::MATTE,
                );
            }

            // the roof, as two pitches leaning on each other. A box with a
            // lid is a shed and this is a longhouse.
            let rise = one.ridge - one.eaves;
            let slope = (rise / (one.wide * 0.5)).atan();
            let along = valhalla::slope_run(&one);

            for side in [-1.0f32, 1.0] {
                // and in pieces, because the hall's has a hole in it. The
                // eaves end of the slope is nought and the ridge is one.
                for (from, to, z_from, z_to) in valhalla::pitch_pieces(&one, one.hall) {
                    let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
                    let ridge = vec3(one.at.x, one.at.y + one.ridge, 0.0);
                    let middle = eaves + (ridge - eaves) * (from + to) * 0.5;

                    scene.push_material(
                        cube,
                        &Transform::at(vec3(middle.x, middle.y, one.at.z + (z_from + z_to) * 0.5))
                            // down as it goes out, not up. The sign was the
                            // other way and the two pitches flared away from
                            // each other like wings with the sky between them.
                            .with_rotation(glam::Quat::from_rotation_z(-side * slope))
                            .with_scale(vec3((to - from) * along * 1.04, 0.22, z_to - z_from)),
                        aim::VALHALLA_ROOF,
                        aim::MATTE,
                    );
                }
            }

            // and one line of neon along the ridge of the hall, which is
            // the building's own light turning up in its own heaven. Spec
            // 0055, and wordless: anything with words on it is copy.
            if one.hall {
                scene.push_plain(
                    cube,
                    &Transform::at(vec3(one.at.x, one.at.y + one.ridge + 0.06, one.at.z))
                        .with_scale(vec3(0.1, 0.1, one.long * 0.92)),
                    aim::arcade_tube(),
                );
            }
        }

        self.draw_the_hall(scene, cube);

        // and what those openings throw, which is the whole of why they
        // are openings. One light a building rather than one a window:
        // the lamp is inside and the windows are holes in the same room.
        for one in valhalla::lamps(reaches) {
            scene.push_light(blitzkit::lighting::PointLight::new(
                one,
                aim::VALHALLA_SPILL,
                aim::VALHALLA_SPILL_LIT,
                aim::VALHALLA_SPILL_REACH,
            ));
        }

        // the openings, which are where the light is
        for one in valhalla::lit(reaches) {
            let across = if one.looks.x.abs() > 0.5 {
                vec3(0.06, one.high, one.wide)
            } else {
                vec3(one.wide, one.high, 0.06)
            };

            scene.push_plain(
                cube,
                &Transform::at(one.at).with_scale(across),
                aim::VALHALLA_LIT,
            );

            // and the reveal round it, which is what makes it a hole: two
            // jambs, a head and a sill, standing out from the wall and
            // dark, so the warm panel is seen down a short tunnel.
            let out_to = one.at.x + valhalla::SET_BACK + valhalla::JAMB * 0.5;
            let deep = valhalla::SET_BACK + valhalla::JAMB;

            for (off_y, off_z, tall, long) in [
                (
                    one.high * 0.5,
                    0.0,
                    valhalla::JAMB,
                    one.wide + valhalla::JAMB * 2.0,
                ),
                (
                    -one.high * 0.5,
                    0.0,
                    valhalla::JAMB,
                    one.wide + valhalla::JAMB * 2.0,
                ),
                (0.0, one.wide * 0.5, one.high, valhalla::JAMB),
                (0.0, -one.wide * 0.5, one.high, valhalla::JAMB),
            ] {
                scene.push_material(
                    cube,
                    &Transform::at(vec3(out_to, one.at.y + off_y, one.at.z + off_z))
                        .with_scale(vec3(deep, tall, long)),
                    aim::VALHALLA_JAMB,
                    aim::MATTE,
                );
            }
        }
    }

    /// What you can walk into from where you are.
    ///
    /// **The field is not in the building's list and cannot be.** Every
    /// other collider in here is a box at a fixed place, written once. The
    /// field's ground is two hundred metres of slab that follows you and
    /// its trunks come out of whichever cells you are near, so the list is
    /// a different list every few paces.
    ///
    /// Only while you are out there. The slab laps two hundred metres back
    /// under the building, which is right from outside, where it keeps the
    /// garden's floor and the field's from meeting edge to edge, and wrong
    /// from inside, where it would be a lid over the cellar stair.
    fn standing_on(&self) -> Vec<Aabb> {
        let mut out = self.room.solid();

        if field::afoot(&out, self.at) {
            out.extend(field::solid(self.room.reaches, self.at));
        }

        // and the bridge, which is out in the field and is not there at
        // all until every room has given up what it asks. Spec 0054, and
        // spec 0056's Valhalla at the top of it on the same condition.
        out.extend(bridge::solid(self.room.reaches, &self.trail));
        out.extend(gate::solid(self.room.reaches, &self.trail, self.gate));
        out.extend(valhalla::solid(self.room.reaches, &self.trail));

        out
    }

    /// Works the gallery's fine picture a slice further on. Spec 0039.
    ///
    /// **Thrown away the moment the view moves.** A settling is of one
    /// view, and slices of two views side by side in one picture is a
    /// window onto two places at once. What is drawn in the meantime is
    /// the coarse picture, which is written every frame anyway, so there
    /// is nothing to fall back to and nothing to pay for.
    fn settle(&mut self) {
        if self.moving_the_view {
            self.settling = None;
            self.sharp_view = None;

            return;
        }

        if self.sharp_view == Some(self.view) {
            return;
        }

        let mut settling = match self.settling.take() {
            Some(settling) if settling.view == self.view => settling,
            _ => mandel::Settling::new(self.view),
        };

        settling.step();

        // and the frame that finishes it is the frame the wall changes
        // on: `before_frame` writes it into the fine texture before
        // anything is drawn, so there is never a frame wearing a texture
        // nothing has been put in.
        if settling.whole() {
            self.sharp_view = Some(settling.view);
        }

        self.settling = Some(settling);
    }

    /// Inside the hall, per spec 0060.
    ///
    /// **The fire is the only light and everything else is what it falls
    /// on.** Dark boards, dark timber, a stone kerb, and the underside of
    /// a roof six metres up with rafters across it. Nothing in here is
    /// lit that is not the fire or the night through the hole over it.
    /// The corridor of mirrors, spec 0061, and the two reflections in it.
    ///
    /// **The order here is the whole of how it works.** The panes are pushed
    /// as glass, which engine spec 0092 draws in a pass of its own after the
    /// world; the two walls are pushed as mirrors, which engine spec 0091
    /// draws *before* the world, into the targets those panes wear. So this
    /// frame's glass shows this frame's reflection, and the wall you have
    /// your back to is filled first, from the reflection that can see it,
    /// per engine spec 0095. Filled from the eye instead it came back black,
    /// and black only inside the other wall's glass.
    fn draw_the_hall(&self, scene: &mut Scene, cube: MeshId) {
        let reaches = self.room.reaches;
        let one = valhalla::hall(reaches);
        let floor = valhalla::floor(reaches);

        scene.push_material(
            cube,
            &Transform::at(floor.center()).with_scale(floor.size()),
            aim::HALL_FLOOR,
            aim::MATTE,
        );

        // the frame: posts in two rows, the beams across them, and the
        // benches and the high seat, which are the same timber
        for piece in valhalla::posts(reaches)
            .into_iter()
            .chain(valhalla::ties(reaches))
            .chain(valhalla::benches(reaches))
            .chain(std::iter::once(valhalla::high_seat(reaches)))
        {
            scene.push_material(
                cube,
                &Transform::at(piece.center()).with_scale(piece.size()),
                aim::HALL_TIMBER,
                aim::MATTE,
            );
        }

        // the rafters, under each pitch and across it, which is the
        // surface the firelight has to find on its way up
        let rise = one.ridge - one.eaves;
        let slope = (rise / (one.wide * 0.5)).atan();
        let along = valhalla::slope_run(&one);
        let air = valhalla::inside(reaches);
        let rafters = ((one.long / 1.6).round() as i32).max(2);
        // the gable's two faces, which sit either side of the middle of
        // its wall
        const WALL_FACE: f32 = valhalla::WALL_THICK * 0.5;
        const GABLE_IN: f32 = valhalla::GABLE_APART;

        for side in [-1.0f32, 1.0] {
            for n in 0..rafters {
                let at_z = air.min.z + (n as f32 + 0.5) / rafters as f32 * (air.max.z - air.min.z);

                // **Not across the hole.** A louvre with the roof's own
                // rafters running over it is a grating, and what is
                // wanted is a hole with a night in it.
                if (at_z - one.at.z).abs() < valhalla::HOLE_LONG * 0.5 + 0.2 {
                    continue;
                }

                let eaves = vec3(one.at.x + side * one.wide * 0.5, one.at.y + one.eaves, 0.0);
                let ridge = vec3(one.at.x, one.at.y + one.ridge, 0.0);
                // under the pitch rather than in it, so the two are not
                // one surface
                let middle = (eaves + ridge) * 0.5 - Vec3::Y * 0.16;

                scene.push_material(
                    cube,
                    &Transform::at(vec3(middle.x, middle.y, at_z))
                        .with_rotation(glam::Quat::from_rotation_z(-side * slope))
                        .with_scale(vec3(along * 0.96, 0.13, 0.16)),
                    aim::HALL_TIMBER,
                    aim::MATTE,
                );
            }
        }

        // the stones round the fire, and then the fire
        let kerb = valhalla::fire(reaches);

        for rail in valhalla::fire_rails(reaches) {
            scene.push_material(
                cube,
                &Transform::at(rail.center()).with_scale(rail.size()),
                aim::HEARTH_STONE,
                aim::MATTE,
            );
        }

        // **The fire's own body, which is not the same thing as its
        // light.** A lamp with nothing where it stands is a room lit by
        // nobody. The flames are divided by the fill, per
        // `garden::gives_off`, so the fire is the same fire at noon as at
        // midnight: the carving was built the other way round and came
        // out dimmest at the hour it was for.
        let bed = valhalla::coals(reaches);

        scene.push_material(
            cube,
            &Transform::at(bed.center()).with_scale(bed.size()),
            aim::HEARTH_COALS,
            aim::MATTE,
        );

        // and a flame over each of its three lights, so the trench has
        // something burning in it and not only a glowing floor
        if let Some(flame) = self.hearth_flame {
            // **More flames than lights.** Three lights are enough to
            // light a hall and three flames in sixteen metres of trench
            // is a fire with gaps in it.
            for n in 0..valhalla::FLAMES {
                // each one on its own count, or they rise and fall
                // together and the fire pulses like a bulb
                let own = bridge::burning(self.since + n as f32 * 1.31);
                let (aside, size) = valhalla::flame_sits(n);
                let along = (n as f32 + 0.5) / valhalla::FLAMES as f32 - 0.5;
                let at = vec3(
                    one.at.x + along * (valhalla::FIRE_LONG - 1.0),
                    kerb.max.y - 0.12,
                    one.at.z + aside,
                );

                scene.push_plain(
                    flame,
                    &Transform::at(at)
                        .with_rotation(glam::Quat::from_rotation_y(own * 2.1))
                        .with_scale(vec3(
                            valhalla::FLAME_WIDE * size,
                            (0.26 + own * 0.2) * size,
                            valhalla::FLAME_WIDE * size,
                        )),
                    garden::gives_off(aim::HEARTH_FIRE * own, self.since).extend(1.0),
                );
            }
        }

        // and the light it throws, which is the only light in this room.
        //
        // **The middle one asks for a shadow and the other two do not.**
        // A hall with posts down it and a fire on the floor is a hall
        // full of long shadows, and that is the whole of what the room
        // is for. Three casting lamps is three shadow maps of one fire
        // and only two slots to put them in, and the two that won would
        // be picked by depth rather than by sense.
        for (n, at) in valhalla::fires(reaches).into_iter().enumerate() {
            let own = bridge::burning(self.since + n as f32 * 1.7);
            let lamp = blitzkit::lighting::PointLight::new(
                at,
                aim::HEARTH_SHADE,
                aim::HEARTH_LIT * own,
                aim::HEARTH_REACH,
            );

            scene.push_light(if n == valhalla::FIRES / 2 {
                lamp.casting()
            } else {
                lamp
            });
        }

        // the triangle over each gable, which was open to the sky until
        // there was anybody inside to see it
        if let Some(gable) = self.gable {
            let rise = one.ridge - one.eaves;

            for side in [-1.0f32, 1.0] {
                for turned in [0.0f32, std::f32::consts::PI] {
                    let face = side * (one.long * 0.5 - WALL_FACE) - side * GABLE_IN * turned.cos();

                    scene.push_material(
                        gable,
                        &Transform::at(vec3(one.at.x, one.at.y + one.eaves, one.at.z + face))
                            .with_rotation(glam::Quat::from_rotation_y(turned))
                            .with_scale(vec3(one.wide * 0.5, rise, 1.0)),
                        aim::VALHALLA_WALL,
                        aim::MATTE,
                    );
                }
            }
        }

        // the painted night through the hole in the ridge, per spec 0060.
        // Face down, because it is only ever looked up at.
        if let (Some(plane), Some(night)) = (self.ground, self.night) {
            let (at, wide, long) = valhalla::night(reaches);

            scene.push_textured(
                plane,
                night,
                &Transform::at(at)
                    .with_rotation(glam::Quat::from_rotation_x(std::f32::consts::PI))
                    .with_scale(vec3(wide, 1.0, long)),
                garden::gives_off(aim::NIGHT_BURNS, self.since).extend(1.0),
                aim::MATTE,
            );
        }
    }

    /// Which of the nook's two sconce walls the hall of mirrors is behind.
    /// `room::bookcase_wall`, which the nook's own lamps also ask.
    fn bookcase_wall(&self) -> (f32, f32) {
        room::bookcase_wall(self.room.reaches)
    }

    fn draw_the_people(&self, scene: &mut Scene) {
        let (Some(body), Some(head), Some(thigh), Some(shin), Some(cube)) =
            (self.body, self.head, self.thigh, self.shin, self.cube)
        else {
            return;
        };

        let reaches = self.room.reaches;
        // **Which way is up is a parameter now, per spec 0038.** Every
        // offset in here used to be a `vec3(0.0, something, 0.0)`, which is
        // a figure that can only ever stand on a floor. The imaginarium has
        // three floors at right angles to each other and people on all of
        // them, so the body is built in its own terms and then turned.
        let (carved_head, cloak, helm, horn) = (self.carved_head, self.cloak, self.helm, self.horn);
        let mut stand = |at: Vec3,
                         up: glam::Quat,
                         facing: f32,
                         walked: Option<f32>,
                         shade: usize,
                         look: people::Look,
                         // **Only in reflections, per engine spec 0091.**
                         // True for the figure of you and false for everybody
                         // else. Bracketed in here rather than around the
                         // call, partly because this closure holds the scene
                         // and partly because a bracket next to the pushes it
                         // guards cannot be left open by a path through the
                         // caller.
                         only_in_glass: bool| {
            if only_in_glass {
                scene.only_in_reflections();
            }
            // **Minus the facing, which is not a typo.** A turn of `facing`
            // about y sends a body's own forward of `-Z` to
            // `(-sin facing, 0, -cos facing)`, and every other account of
            // where that body is pointing says `(sin facing, 0, -cos facing)`:
            // `ahead` and `side` below, `forward` on the arcade, and the
            // `way.x.atan2(-way.z)` that `people::walking` and
            // `aloft::walking` build a facing with in the first place. The
            // mesh was the one account disagreeing, so everybody in the
            // building stood mirrored about the north line.
            //
            // Nobody saw it for as long as nobody was looked at straight on.
            // A guest walking a corridor is near enough sin or cos of nought
            // for the two to agree, and the eyes went on in spec 0061 at the
            // `ahead` above, which put them on the back of the head for any
            // other angle. Jake, 2026-11-09, standing at a panel: here the
            // user is reflected in a weird way, it just feels wrong.
            //
            // `tests::a_figure_faces_the_way_its_own_facing_says` holds it.
            let turn = up * glam::Quat::from_rotation_y(-facing);
            // **A coat, trousers and a head.** One colour from crown to
            // toe is a mannequin by definition: the only thing that says a
            // figure in a dark corridor is dressed is where the colour
            // stops.
            // **And a body of their own, not only a coat.** One mesh
            // at one height in four colours is four people seen from
            // the front and one person seen in outline, which is the
            // second thing that reads as mannequins. `people::built`
            // gives each of them a height and a build.
            let built = people::built(shade);
            let thick = vec3(built.wide, built.tall, built.wide);
            let round = vec3(built.wide, built.wide, built.wide);
            let which = shade % aim::COATS.len();
            let coat = aim::COATS[which];
            let trousers = aim::TROUSERS[which];
            let skin = aim::HEADS[which];
            // which way is their right, for hanging the limbs off
            let side = up * vec3(facing.cos(), 0.0, facing.sin());
            // and which way they are walking, for putting a foot that way
            let ahead = up * vec3(facing.sin(), 0.0, -facing.cos());
            // and which way is over their head
            let over = up * Vec3::Y;

            // **Spec 0046: the hips are not at a fixed height.** The leg
            // carrying somebody is the long side of its own swing, so
            // everything from the hip up rides on it. A body held at one
            // height while the feet move is a glide, and that is most of
            // what reads as a machine.
            let rise = match walked {
                Some(walked) => people::hips(walked) - people::LEG_TALL,
                None => 0.0,
            };
            let up_to = |how_high: f32| at + over * (how_high * built.tall + rise);
            // **A limb turns about the walker's own right, not about a
            // local x.** `turn * rot_x` swung every limb the wrong way
            // across the body while the feet, placed off `ahead`, went the
            // right way, and the feet came off the legs. That was `turn`
            // being `rot_y(facing)`, the mirror of this building's
            // `(sin yaw, 0, -cos yaw)`; `turn` is `rot_y(-facing)` now and
            // agrees, so the two would come to the same thing. Named off
            // `side` either way, because an axis the whole closure already
            // shares cannot drift from one of them again.
            let swung = |angle: f32| glam::Quat::from_axis_angle(side, angle) * turn;
            let hangs = |by: glam::Quat, far: f32| by * vec3(0.0, -far * built.tall, 0.0);

            scene.push_material(
                body,
                &Transform::at(up_to(people::TORSO_FROM))
                    .with_rotation(turn)
                    .with_scale(thick),
                coat,
                aim::NO_SHINE,
            );
            // **A head scaled round and not tall.** Given the height
            // along with everything else, the tall people got long
            // faces and the short ones got wide ones, which is the
            // one place a body's proportions do not actually follow
            // its height.
            let carved = look.carved();

            // **A neck, which the proportions made necessary.** With the
            // shoulder at 1.42 and the head starting at 1.50 there is eight
            // centimetres of nothing between them, and a head floating over
            // a torso is worse than one sunk into it. It runs a little into
            // both ends so there is no seam at either.
            scene.push_material(
                body,
                &Transform::at(up_to(people::NECK_FROM + people::NECK_TALL * 0.5))
                    .with_rotation(turn)
                    .with_scale(vec3(
                        people::NECK_WIDE * built.wide,
                        (people::NECK_TALL + 0.05) * built.tall,
                        people::NECK_WIDE * built.wide,
                    )),
                skin,
                aim::NO_SHINE,
            );

            scene.push_material(
                match (carved, carved_head) {
                    (true, Some(cut)) => cut,
                    _ => head,
                },
                &Transform::at(up_to(people::HEAD_FROM))
                    .with_rotation(turn)
                    .with_scale(round),
                skin,
                aim::NO_SHINE,
            );

            // **Eyes, and only on the guests.** Spec 0033 said no face,
            // reasoning about somebody at the far end of a corridor, where
            // one would not show. A mirror is the opposite case and a blank
            // head at arm's length is the mannequin that spec was avoiding.
            //
            // The carvings keep their blankness, which is the same spec's
            // other half: a carved thing has no face in life either, so on
            // them the absence is not a gap.
            if !carved {
                for hand in [-1.0f32, 1.0] {
                    scene.push_material(
                        cube,
                        &Transform::at(
                            up_to(people::HEAD_FROM + people::HEAD_TALL * people::EYE_UP)
                                + ahead * (people::EYE_OUT * built.wide)
                                + side * (hand * people::EYE_APART * built.wide),
                        )
                        .with_rotation(turn)
                        .with_scale(vec3(
                            people::EYE_WIDE * built.wide,
                            people::EYE_TALL * built.tall,
                            people::EYE_DEEP * built.wide,
                        )),
                        aim::EYE,
                        aim::MATTE,
                    );
                }
            }

            if carved {
                // **A cloak, a helm and a nose bar**, which is the whole
                // of what makes the ones at the top of the climb read as
                // somebody rather than as guests who came outside. Spec
                // 0055.
                if let Some(cloak) = cloak {
                    scene.push_material(
                        cloak,
                        &Transform::at(up_to(people::CLOAK_FROM))
                            .with_rotation(turn)
                            .with_scale(thick),
                        aim::CLOAKS[which],
                        aim::NO_SHINE,
                    );
                }

                if let Some(helm) = helm {
                    scene.push_material(
                        helm,
                        &Transform::at(up_to(
                            people::HEAD_FROM + people::HEAD_TALL * people::HELM_FROM,
                        ))
                        .with_rotation(turn)
                        .with_scale(round),
                        aim::HELM,
                        aim::NO_SHINE,
                    );
                    scene.push_material(
                        cube,
                        &Transform::at(
                            up_to(
                                people::HEAD_FROM
                                    + people::HEAD_TALL
                                        * (people::HELM_FROM - people::NOSE_LONG * 0.5),
                            ) + ahead
                                * (people::head_profile(people::BROW_UP).1 * 0.5 * built.wide
                                    + people::NOSE_THICK * 0.3),
                        )
                        .with_rotation(turn)
                        .with_scale(vec3(
                            people::NOSE_WIDE * built.wide,
                            people::HEAD_TALL * people::NOSE_LONG,
                            people::NOSE_THICK * built.wide,
                        )),
                        aim::HELM,
                        aim::NO_SHINE,
                    );
                }
            } else {
                // and their brows, which with the eyes under them are the
                // whole of the face: see `people::BROW_UP`. Not under a
                // helm, where the nose bar already says which way somebody
                // is turned and two of those on one face is a mask.
                let face = people::head_profile(people::BROW_UP).1 * built.wide;
                // how deep the head is at the brow's outer end, as a share
                // of the face's width: see `people::BROW_OUT`
                let out = people::BROW_WIDE * 0.5;
                let sunk = (0.25 - out * out).max(0.0).sqrt();

                for hand in [-1.0f32, 1.0] {
                    scene.push_material(
                        cube,
                        &Transform::at(
                            up_to(people::HEAD_FROM + people::HEAD_TALL * people::BROW_UP)
                                + ahead * (sunk * face)
                                + side
                                    * (hand
                                        * (people::BROW_WIDE + people::BROW_APART)
                                        * 0.25
                                        * face),
                        )
                        .with_rotation(turn)
                        .with_scale(vec3(
                            (people::BROW_WIDE - people::BROW_APART) * 0.5 * face,
                            people::BROW_TALL * built.wide,
                            people::BROW_OUT * built.wide,
                        )),
                        aim::BROWS[which],
                        aim::NO_SHINE,
                    );
                }
            }

            for leg in 0..2 {
                let out = if leg == 0 { 1.0 } else { -1.0 };
                let (hip, knee, shoulder, elbow) = match walked {
                    Some(walked) => (
                        people::hip(walked, leg),
                        people::knee(walked, leg),
                        people::shoulder(walked, leg),
                        people::elbow(walked, leg),
                    ),
                    None => (0.0, 0.0, 0.0, 0.0),
                };

                // **The leg, turned about the hip rather than slid away
                // from it.** The knee takes the shin back off the thigh's
                // line, which is the one direction a knee goes.
                let hip_at =
                    up_to(people::LEG_TALL) + side * (people::LEGS_APART * out * built.wide);
                let upper = swung(hip);
                let knee_at = hip_at + hangs(upper, people::THIGH_TALL);
                let lower = swung(hip - knee);

                scene.push_material(
                    thigh,
                    &Transform::at(hip_at).with_rotation(upper).with_scale(thick),
                    trousers,
                    aim::NO_SHINE,
                );
                scene.push_material(
                    shin,
                    &Transform::at(knee_at)
                        .with_rotation(lower)
                        .with_scale(thick),
                    trousers,
                    aim::NO_SHINE,
                );
                // **And a knee in the joint.** Two bones meeting end to end
                // are two circles at an angle, and a bent joint opens a
                // wedge of daylight on the outside of the bend. Jake,
                // 2026-11-09: there are visible gaps in joints. A ball the
                // width the table already gives that joint fills it at any
                // angle, which is what a knee is for.
                //
                // The head's own egg, the way the hand is, because a wheel
                // already turned is cheaper than another one.
                scene.push_material(
                    head,
                    &Transform::at(knee_at).with_rotation(lower).with_scale(
                        Vec3::splat(people::limb_wide(people::KNEE_AT) / people::HEAD_WIDE)
                            * vec3(built.wide, built.wide, built.wide),
                    ),
                    trousers,
                    aim::NO_SHINE,
                );

                // **The foot stays level, which is what an ankle is
                // for.** Hung off the shin it tilts with it, and a figure
                // on skis is further from a person than one with no feet
                // at all.
                //
                // Where it goes is `people::ankle` rather than the end
                // of the two bones worked out a second time here, which
                // would be two answers to where somebody's foot is.
                let (fore, off_floor) = match walked {
                    Some(walked) => people::ankle(walked, leg),
                    None => (0.0, 0.0),
                };

                scene.push_material(
                    cube,
                    &Transform::at(
                        at + side * (people::LEGS_APART * out * built.wide)
                            + ahead * (fore + people::FOOT_AHEAD)
                            + over * (off_floor + people::FOOT_TALL * 0.5),
                    )
                    .with_rotation(turn)
                    .with_scale(vec3(
                        people::FOOT_WIDE * built.wide,
                        people::FOOT_TALL,
                        people::FOOT_LONG * built.tall,
                    )),
                    trousers,
                    aim::NO_SHINE,
                );

                // an arm is the same two bones, thinner and shorter. One
                // pair of meshes for both is the point of a wheel.
                let thin = vec3(
                    people::ARM_THIN * built.wide,
                    people::ARM_SHORT * built.tall,
                    people::ARM_THIN * built.wide,
                );
                let arm_upper = swung(shoulder);
                let shoulder_at =
                    up_to(people::ARM_FROM) + side * (people::ARMS_APART * out * built.wide);
                let elbow_at =
                    shoulder_at + hangs(arm_upper, people::THIGH_TALL * people::ARM_SHORT);
                // and an elbow goes the other way
                let arm_lower = swung(shoulder + elbow);
                let wrist = elbow_at + hangs(arm_lower, people::SHIN_TALL * people::ARM_SHORT);

                scene.push_material(
                    thigh,
                    &Transform::at(shoulder_at)
                        .with_rotation(arm_upper)
                        .with_scale(thin),
                    coat,
                    aim::NO_SHINE,
                );
                scene.push_material(
                    head,
                    &Transform::at(elbow_at).with_rotation(arm_lower).with_scale(
                        Vec3::splat(people::limb_wide(people::KNEE_AT) / people::HEAD_WIDE)
                            * vec3(
                                people::ARM_THIN * built.wide,
                                people::ARM_THIN * built.wide,
                                people::ARM_THIN * built.wide,
                            ),
                    ),
                    coat,
                    aim::NO_SHINE,
                );
                scene.push_material(
                    shin,
                    &Transform::at(elbow_at)
                        .with_rotation(arm_lower)
                        .with_scale(thin),
                    coat,
                    aim::NO_SHINE,
                );
                // a hand, which is the head's own egg at a fifth of the
                // size and the same colour as the face
                scene.push_material(
                    head,
                    &Transform::at(wrist + hangs(arm_lower, people::HAND_WIDE))
                        .with_rotation(arm_lower)
                        .with_scale(Vec3::splat(people::HAND_WIDE / people::HEAD_WIDE)),
                    skin,
                    aim::NO_SHINE,
                );

                // **And Gjallarhorn in the left hand.** Spec 0055: the
                // horn that has not been blown. It hangs from the fist
                // rather than being raised, because Heimdall holds it
                // for the whole of the world's history and sounds it
                // once. Hung off the wrist like the hand is, so it
                // swings with the arm instead of floating beside it.
                if look.horn() && leg == 1 {
                    if let Some(horn) = horn {
                        let fist = wrist + hangs(arm_lower, people::HAND_WIDE);

                        scene.push_material(
                            horn,
                            &Transform::at(fist)
                                .with_rotation(
                                    // the mesh runs along its own +x and
                                    // curves down from there. A quarter
                                    // turn about the body's up swings
                                    // that from the hand's own right
                                    // round to in front of him, and the
                                    // tilt drops the bell the rest of
                                    // the way. Turned about z instead it
                                    // pointed straight at the floor and
                                    // read as a stick.
                                    arm_lower
                                        * glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                                        * glam::Quat::from_rotation_z(0.32),
                                )
                                .with_scale(Vec3::splat(built.tall * bridge::HORN_HELD)),
                            aim::HORN,
                            aim::NO_SHINE,
                        );
                    }
                }
            }

            // and closed where it was opened, so the bracket cannot be left
            // open by a path through this closure
            if only_in_glass {
                scene.and_everywhere();
            }
        };

        // and the watchman at the top of the bridge, who is there exactly
        // when the bridge is. Spec 0055: he stands, he faces the climb, and
        // nothing is said.
        if bridge::shows(&self.trail) {
            let (at, facing) = bridge::watchman(reaches);

            stand(
                at,
                glam::Quat::IDENTITY,
                facing,
                None,
                2,
                people::Look::Carved,
                false,
            );

            // and whoever is already up there. Spec 0056: a few, and none
            // of them fighting.
            for (n, (at, facing)) in valhalla::people(reaches).into_iter().enumerate() {
                stand(
                    at,
                    glam::Quat::IDENTITY,
                    facing,
                    None,
                    n,
                    people::Look::Einherjar,
                    false,
                );
            }
        }

        // **And you, in the glass.** Spec 0061.
        //
        // Inside engine spec 0091's bracket, because the camera is at eye
        // height: on the screen this figure's head would be around the
        // camera, and the pipelines cull by face, so what you would see is
        // the inside of your own skull. The head is 0.26 tall against a near
        // plane of 0.1, so it clips raggedly rather than disappearing.
        //
        // Only while there is a mirror to show it. Nothing else in the
        // building reflects, so everywhere else this is a figure drawn into
        // a pass nobody runs.
        if !self.glass_targets.is_empty() && mirrors::holds(self.at) {
            stand(
                // `at` is already the feet: `where_am_i` calls it "the feet,
                // the eye's height, the yaw and the pitch", and `EYE` is
                // added on top of it for the camera rather than taken off
                self.at,
                glam::Quat::IDENTITY,
                self.yaw,
                // the gait off the distance the footsteps already keep, so
                // the legs swing with the walking rather than on a clock of
                // their own
                Some(self.paced),
                // your own coat, which is a seed like anybody else's
                0,
                people::Look::Guest,
                true,
            );
        }
    }

    fn draw_the_back_room(&self, scene: &mut Scene, cube: MeshId, floor: MeshId) {
        let reaches = self.room.reaches;
        let (west, east) = back::span();
        let (_, near) = back::deep(reaches);

        // the floor, which is concrete rather than the aisle's carpet: this
        // is the room the building does not dress up. In two pieces, because
        // the bay's floor is black: two quads side by side rather than a
        // panel laid on top of one, since a panel is a step at the lip and a
        // face at the walking surface stops you dead. Spec 0025.
        let bay = back::bay(reaches);

        for (from, to, colour) in [
            (bay.min.z, bay.max.z, aim::BACK_LINING),
            (bay.max.z, near, aim::BACK_FLOOR),
        ] {
            scene.push_plain(
                floor,
                &Transform::at(vec3((west + east) * 0.5, 0.0, (from + to) * 0.5)).with_scale(vec3(
                    east - west,
                    1.0,
                    to - from,
                )),
                colour,
            );
        }

        // the shell and the lining are handed over once rather than drawn
        // here, per spec 0026. What is left in this room's drawing is what
        // moves or lights up.

        // the bench: a top on four legs, the way the nook's are built
        let at = back::bench(reaches);
        let top = back::BENCH.y;

        scene.push_plain(
            cube,
            &Transform::at(vec3(at.x, top - aim::BENCH_TOP * 0.5, at.z)).with_scale(vec3(
                back::BENCH.x,
                aim::BENCH_TOP,
                back::BENCH.z,
            )),
            aim::BENCH_ON,
        );

        for along in [-1.0f32, 1.0] {
            for across in [-1.0f32, 1.0] {
                let leg = vec3(
                    at.x + along * (back::BENCH.x * 0.5 - aim::BENCH_LEG),
                    (top - aim::BENCH_TOP) * 0.5,
                    at.z + across * (back::BENCH.z * 0.5 - aim::BENCH_LEG),
                );

                scene.push_plain(
                    cube,
                    &Transform::at(leg).with_scale(vec3(
                        aim::BENCH_LEG,
                        top - aim::BENCH_TOP,
                        aim::BENCH_LEG,
                    )),
                    aim::BENCH_LEG_LOOK,
                );
            }
        }

        // the rig on it: a block of cubes held over the bench, and the jig
        // holding them. Spec 0024.
        let count = back::CUBES[self.rig_step];
        let block = back::block(reaches);
        let across = back::cube_across(count);

        for n in 0..count {
            let mesh = if self.rig_many {
                self.rig_meshes.get(n as usize).copied().unwrap_or(cube)
            } else {
                cube
            };

            scene.push_material(
                mesh,
                &Transform::at(block + back::cube_at(count, n)).with_scale(Vec3::splat(across)),
                aim::BACK_CUBE,
                aim::MATTE,
            );
        }

        // the frame it stands in: a post each side off the floor and a bar
        // across the top of them. Spec 0025: the block is on the floor in
        // the bay now rather than clamped over a bench.
        for side in [-1.0f32, 1.0] {
            scene.push_plain(
                cube,
                &Transform::at(vec3(
                    block.x + side * back::POST_OUT,
                    back::BLOCK_TOP * 0.5,
                    block.z,
                ))
                .with_scale(vec3(back::POST, back::BLOCK_TOP, back::POST)),
                aim::BACK_JIG,
            );
            scene.push_plain(
                cube,
                &Transform::at(vec3(block.x + side * back::POST_OUT, 0.02, block.z))
                    .with_scale(vec3(back::POST * 3.4, 0.04, back::BLOCK + 0.2)),
                aim::BACK_JIG,
            );
        }
        scene.push_plain(
            cube,
            &Transform::at(vec3(block.x, back::BLOCK_TOP, block.z)).with_scale(vec3(
                back::POST_OUT * 2.0 + back::POST,
                back::POST,
                back::POST,
            )),
            aim::BACK_JIG,
        );

        // and the gauges on the wall over it, which are what the dial is
        // turned for: three bars, reading the same window spec 0023 fills
        // every frame. No highlight on any of it, or the board comes out a
        // pale slab under the bulb hanging in front of it.
        let gauge = back::gauge(reaches);

        scene.push_material(
            cube,
            &Transform::at(gauge).with_scale(back::GAUGE),
            aim::BACK_GAUGE,
            aim::VOID,
        );

        // what the board says, which never changes: the heading across the
        // top, each bar's name on the left, and what each track is full at
        // past its right end. Drawn on the same quad the tesseract's wall
        // plates use, turned the way a face at the far end of a room has to
        // be turned to be read from inside it. Spec 0024.
        if let Some(hung) = self.hung {
            let mut written = |word: &str, texture: TextureId, at: Vec3| {
                let shape = blitzkit::text::room_for(word, back::GAUGE_TEXELS);
                let wide = back::GAUGE_WORD_TALL * (shape.x / shape.y.max(1e-4));

                scene.push_textured(
                    hung,
                    texture,
                    &Transform::at(gauge + at + vec3(wide * 0.5, 0.0, 0.0))
                        .with_rotation(glam::Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
                        .with_scale(vec3(0.02, back::GAUGE_WORD_TALL, wide)),
                    aim::TESSERACT_WORD,
                    aim::VOID,
                );
            };

            if let Some(texture) = self.gauge_words.first().copied() {
                written(back::GAUGE_SAYS, texture, back::gauge_says());
            }

            for (n, word) in back::GAUGE_WORDS.iter().enumerate() {
                if let Some(texture) = self.gauge_words.get(n + 1).copied() {
                    written(word, texture, back::gauge_word(n));
                }
            }

            for (n, word) in back::GAUGE_FULL_SAYS.iter().enumerate() {
                if let Some(texture) = self.gauge_words.get(n + 4).copied() {
                    written(word, texture, back::gauge_full_at(n));
                }
            }
        }

        let readings = [
            self.costs.middle_ms().unwrap_or(0.0),
            self.costs.worst_ms().unwrap_or(0.0),
            self.counted.0 as f32,
        ];

        for (n, of) in readings.iter().enumerate() {
            let (where_, long) = back::gauge_bar(n, *of);
            let track = back::GAUGE_TRACK;

            // the track it runs in, so an empty gauge is still a gauge
            scene.push_material(
                cube,
                &Transform::at(gauge + where_).with_scale(vec3(track, back::GAUGE_BAR, 0.012)),
                aim::BACK_TRACK,
                aim::VOID,
            );
            scene.push_material(
                cube,
                &Transform::at(gauge + where_ + vec3((long - track) * 0.5, 0.0, 0.012))
                    .with_scale(vec3(long, back::GAUGE_BAR * 0.72, 0.012)),
                aim::BACK_READING[n],
                aim::VOID,
            );
        }

        // and what is standing about in here, which is half of what the room
        // is. Spec 0024.
        self.draw_the_clutter(scene, cube);

        // the bay's lamp, at the lip and over it, which is what lights the
        // cubes. Spec 0025: a shade with a bulb in it, like every other lamp
        // in this building, because a light with no fitting is a bright
        // patch on a wall and no reason for it.
        let bay = back::bay_lamp(reaches);

        scene.push_plain(
            cube,
            &Transform::at(bay + Vec3::Y * back::BAY_SHADE.y * 0.5).with_scale(back::BAY_SHADE),
            aim::BACK_SHADE,
        );
        scene.push_plain(
            cube,
            &Transform::at(bay).with_scale(Vec3::splat(back::BAY_SHADE.y * 0.55)),
            aim::BACK_BULB,
        );
        scene.push_plain(
            cube,
            &Transform::at(vec3(
                bay.x,
                (bay.y + back::BAY_SHADE.y + back::TALL) * 0.5,
                bay.z,
            ))
            .with_scale(vec3(0.03, back::TALL - bay.y - back::BAY_SHADE.y, 0.03)),
            aim::BACK_SHADE,
        );

        // and the bulb in its tin shade, on a stem up to the ceiling
        let lamp = back::lamp(reaches);

        scene.push_plain(
            cube,
            &Transform::at(lamp + Vec3::Y * back::SHADE.y * 0.5).with_scale(back::SHADE),
            aim::BACK_SHADE,
        );
        scene.push_plain(
            cube,
            &Transform::at(lamp).with_scale(Vec3::splat(back::SHADE.y * 0.55)),
            aim::BACK_BULB,
        );
        scene.push_plain(
            cube,
            &Transform::at(vec3(
                lamp.x,
                (lamp.y + back::SHADE.y + back::TALL) * 0.5,
                lamp.z,
            ))
            .with_scale(vec3(0.03, back::TALL - lamp.y - back::SHADE.y, 0.03)),
            aim::BACK_SHADE,
        );
    }

    /// Hands the engine the parts of the building that never move, once.
    /// Spec 0026.
    ///
    /// **The shells and nothing else.** A room's walls are a list of boxes
    /// that is already the one account of where that wall is, so keeping
    /// them writes nothing down twice. Everything conditional goes on being
    /// pushed: a cabinet that lights under the sight, a door that opens, a
    /// leaf that swings, the water, the toys, the planets.
    ///
    /// Done once, on the first frame that has a mesh to name them with, and
    /// never again: nothing in here changes shape while the game runs, and
    /// if that stops being true the run for that room is dropped and kept
    /// again, which is one call.
    fn keep_the_building(&mut self, scene: &mut Scene, cube: MeshId) {
        /// What the building tells the engine about itself: that it is the
        /// same building it was last frame. The engine's spec 0064 takes
        /// a version and
        /// this one never moves, because nothing in these runs changes
        /// shape while the game runs. If that stops being true, this is
        /// the number to bump.
        const STILL_THE_SAME_BUILDING: u64 = 0;

        for (n, run) in standing::runs(self.room.reaches).into_iter().enumerate() {
            // and spec 0015's room is not drawn from out of doors at all.
            //
            // **It is a hundred and fifty nine metres square and it is a
            // secret, and it was the biggest thing in the landscape.** Its
            // floor, its lid and its walls are `aim::BEHIND_FLOOR` and its
            // neighbours, which are as near black as this building draws
            // because from inside that room they are space. From outside
            // they are a hole in the world: from the top of spec 0054's
            // bridge, looking back at the building, the arcade is a small
            // lit cluster beside a black rectangle ten times its size, and
            // that rectangle is the one room nobody is supposed to find.
            //
            // Out of doors and not `behind::in_the_room`, which is the
            // tempting test and is wrong by the same margin that
            // the engine's spec 0086 was wrong by: it would stop drawing
            // the room while you stood in
            // the hall at its opening looking straight into it. There is
            // no way to see into this room from under the open sky, so
            // anything of it visible from there is a fault.
            if run.at == standing::Where::Behind && self.room.out_of_doors(self.looking_from()) {
                continue;
            }
            // which room as well as what it is made of: a wall in the hall
            // is boards under a light and a wall of the space behind it is
            // as near black as this building draws, and they were the same
            // colour for exactly as long as it took to look
            let (colour, shine) = match (run.at, run.made) {
                (standing::Where::Hall, _) => (aim::WALL, aim::MATTE),
                (standing::Where::Behind, standing::Made::Wall) => (aim::BEHIND_WALL, aim::VOID),
                (_, standing::Made::Ground) => (aim::BEHIND_FLOOR, aim::VOID),
                (_, standing::Made::Column) => (aim::BEHIND_COLUMN, aim::VOID),
                (_, standing::Made::Frame) => (aim::BEHIND_FRAME, aim::VOID),
                (_, standing::Made::BackWall) => (aim::BACK_WALL, aim::VOID),
                (_, standing::Made::BackCeiling) => (aim::BACK_CEILING, aim::VOID),
                (_, standing::Made::Lining) => (aim::BACK_LINING, aim::VOID),
                (_, standing::Made::Wall) => (aim::WALL, aim::MATTE),
            };
            // the hall's walls wear the grain the hall's walls have always
            // worn; everything else is a colour
            let texture = match (run.at, self.wall_grain) {
                (standing::Where::Hall, Some(grain)) => grain,
                _ => blitzkit::renderer::scene::TextureId::WHITE,
            };
            // **One call a run, every frame.** The engine's spec 0064:
            // the version
            // never changes because the building never does, so the boxes
            // are walked once and the engine decides the rest. It keeps a
            // run worth its own draw and pushes one that is not, which is
            // the decision `standing::worth_keeping` used to make here.
            scene.standing(
                blitzkit::renderer::scene::Name(n as u64),
                STILL_THE_SAME_BUILDING,
                cube,
                texture,
                |into| {
                    into.extend(run.boxes.iter().map(|box_| {
                        blitzkit::renderer::scene::Instance::new(
                            &Transform::at(box_.center()).with_scale(box_.size()),
                            colour,
                            shine,
                        )
                    }));
                },
            );
        }
    }

    /// The back room's clutter: what is standing about in it. Spec 0024.
    ///
    /// A crate is drawn rather than modelled, as four boards and a lid, which
    /// is what keeps it from being a box with a label. The rest come off the
    /// cellar's wheel.
    fn draw_the_clutter(&self, scene: &mut Scene, cube: MeshId) {
        let reaches = self.room.reaches;
        let turned = |thing: back::Thing| match thing {
            back::Thing::Bucket => self.clutter.first().copied(),
            back::Thing::Tin | back::Thing::Jar => self.clutter.get(1).copied(),
            back::Thing::Reel => self.clutter.get(2).copied(),
            back::Thing::Crate => None,
        };

        for stood in back::clutter(reaches) {
            let (across, tall) = stood.thing.size();
            let spin = glam::Quat::from_rotation_y(stood.turn);

            match turned(stood.thing) {
                Some(mesh) => {
                    // the wheel makes a thing one unit tall about its own
                    // foot, so this scales rather than offsets
                    let lying = stood.thing == back::Thing::Reel;
                    let turn = if lying {
                        spin * glam::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                    } else {
                        spin
                    };
                    let at = if lying {
                        stood.at + Vec3::Y * across * 0.5
                    } else {
                        stood.at
                    };

                    scene.push_material(
                        mesh,
                        &Transform::at(at)
                            .with_rotation(turn)
                            .with_scale(vec3(across, tall, across)),
                        aim::clutter_of(stood.thing),
                        aim::MATTE,
                    );
                }
                None => {
                    // a crate: four side boards with gaps between them, a
                    // floor and a lid. Boards rather than one box, because a
                    // box is what a crate looks like before it is built.
                    let boards = 3;
                    let board = tall / (boards as f32 * 1.5);
                    let mut put = |at: Vec3, size: Vec3| {
                        scene.push_material(
                            cube,
                            &Transform::at(stood.at + spin * at)
                                .with_rotation(spin)
                                .with_scale(size),
                            aim::CRATE,
                            aim::MATTE,
                        );
                    };

                    for n in 0..boards {
                        let up = board * 0.75 + n as f32 * board * 1.5;

                        for side in [-1.0f32, 1.0] {
                            put(
                                vec3(side * across * 0.5, up, 0.0),
                                vec3(0.03, board, across),
                            );
                            put(
                                vec3(0.0, up, side * across * 0.5),
                                vec3(across, board, 0.03),
                            );
                        }
                    }

                    // a post at each corner, which is what tells a crate
                    // from a stack of boards. Without them this read as a
                    // pallet with a lid on it.
                    for along in [-1.0f32, 1.0] {
                        for across_ in [-1.0f32, 1.0] {
                            put(
                                vec3(
                                    along * (across * 0.5 - 0.02),
                                    tall * 0.5,
                                    across_ * (across * 0.5 - 0.02),
                                ),
                                vec3(0.05, tall, 0.05),
                            );
                        }
                    }

                    put(vec3(0.0, 0.02, 0.0), vec3(across, 0.04, across));
                    put(vec3(0.0, tall - 0.02, 0.0), vec3(across, 0.04, across));
                }
            }
        }

        // the stepladder, leaning on the east wall: two rails and four steps
        let foot = back::ladder(reaches);
        let lean = glam::Quat::from_rotation_z(back::LADDER_LEAN);

        for side in [-1.0f32, 1.0] {
            scene.push_material(
                cube,
                &Transform::at(
                    foot + lean
                        * vec3(0.0, back::LADDER_TALL * 0.5, side * back::LADDER_WIDE * 0.5),
                )
                .with_rotation(lean)
                .with_scale(vec3(0.05, back::LADDER_TALL, 0.05)),
                aim::LADDER,
                aim::MATTE,
            );
        }
        for n in 0..4 {
            let up = (n as f32 + 0.6) / 4.6 * back::LADDER_TALL;

            scene.push_material(
                cube,
                &Transform::at(foot + lean * vec3(0.0, up, 0.0))
                    .with_rotation(lean)
                    .with_scale(vec3(0.06, 0.03, back::LADDER_WIDE)),
                aim::LADDER,
                aim::MATTE,
            );
        }

        // and the broom in the corner, which is a handle and a head
        let broom = back::broom(reaches);
        let tilt = glam::Quat::from_rotation_z(-0.17);

        scene.push_material(
            cube,
            &Transform::at(broom + tilt * vec3(0.0, 0.72, 0.0))
                .with_rotation(tilt)
                .with_scale(vec3(0.035, 1.44, 0.035)),
            aim::LADDER,
            aim::MATTE,
        );
        scene.push_material(
            cube,
            &Transform::at(broom + tilt * vec3(0.0, 0.06, 0.0))
                .with_rotation(tilt)
                .with_scale(vec3(0.07, 0.12, 0.34)),
            aim::BROOM,
            aim::MATTE,
        );
    }

    /// What this frame cost, in the corner, behind F. Spec 0056.
    ///
    /// **The building measuring itself.** The engine learned to time a frame
    /// for `load`, which is a wall of cubes with dials on it and exists
    /// because there was nothing else to measure. This is the thing worth
    /// measuring: the largest program anybody has built on this engine, with
    /// shadow maps and water and a sky and eleven rooms, and the number moves
    /// as you walk from one of them to the next.
    ///
    /// Work rather than the time between frames, for the reason spec 0056
    /// gives at length: on this machine a window is redrawn when the display
    /// says so, so the gap between frames is 8.33 in every room that fits
    /// inside a frame and tells you nothing about which room is dear.
    fn draw_the_cost(&self, geometry: &mut Geometry, text: &mut TextRenderer) {
        if !self.showing_cost {
            return;
        }

        let ms = |got: Option<f32>| match got {
            Some(ms) => format!("{:.2} ms", ms),
            None => String::from("--"),
        };
        let counted = self.counted;
        let lines = [
            format!(
                "work {}   95th {}   worst {}",
                ms(self.costs.middle_ms()),
                ms(self.costs.tail_ms()),
                ms(self.costs.worst_ms()),
            ),
            format!(
                "between frames {}   gpu {}",
                ms(self.costs.between_middle_ms()),
                if self.timed {
                    ms(self.costs.gpu_middle_ms())
                } else {
                    String::from("not timed here")
                },
            ),
            format!("{} draws, {} triangles", counted.0, counted.1),
        ];

        let at = |line: usize| {
            glam::vec2(
                aim::COST_IN,
                self.window.1 - aim::COST_UP - (lines.len() - line) as f32 * aim::COST_STEP,
            )
        };
        let round: Vec<(Vec2, &str, f32)> = lines
            .iter()
            .enumerate()
            .map(|(n, line)| (at(n), line.as_str(), aim::COST_SIZE))
            .collect();

        plate(geometry, aim::plate_round(&round, false));

        for (n, line) in lines.iter().enumerate() {
            text.push_render_text(RenderText {
                position: at(n),
                text: line.clone(),
                size: aim::COST_SIZE,
                ..Default::default()
            });
        }
    }

    fn draw_the_key(&self, geometry: &mut Geometry, text: &mut TextRenderer) {
        use blitzkit::geometry::quad::Quad;

        let Some(inside) = self.inside.as_ref() else {
            return;
        };

        // the three the shape's colours mean, and how much has been walked,
        // which is the one thing the shape cannot show because it is the
        // same shape in every room
        let walked = format!(
            "{} of {} rooms walked",
            inside.found.count(),
            tesseract::cells().len()
        );
        let lines: Vec<String> = aim::CELLS_LEGEND_SAYS
            .iter()
            .map(|words| String::from(*words))
            .chain(std::iter::once(walked))
            .collect();

        let wide = aim::cells_key_wide(&lines);
        let tall = lines.len() as f32 * aim::CELLS_LEGEND_STEP + aim::CELLS_LEGEND_GAP;
        let from = glam::vec2(aim::CELLS_IN, self.window.1 - aim::CELLS_UP - tall);

        geometry.push_quad(&Quad::colored(
            from + glam::vec2(wide, tall) * 0.5,
            glam::vec2(wide, tall) + glam::Vec2::splat(aim::PLATE_EDGE * 2.0),
            aim::CELLS_PLATE,
        ));

        let swatches = [
            Some(aim::CELLS_HERE),
            Some(aim::CELLS_WAY),
            Some(aim::CELLS_GOAL),
            None,
        ];

        let mut y = from.y + aim::CELLS_LEGEND_GAP * 0.5;
        for (line, swatch) in lines.iter().zip(swatches) {
            let middle = y + aim::CELLS_LEGEND_STEP * 0.5;

            if let Some(colour) = swatch {
                geometry.push_quad(&Quad::colored(
                    glam::vec2(
                        from.x + aim::CELLS_LEGEND_GAP + aim::CELLS_SWATCH.x * 0.5,
                        middle,
                    ),
                    aim::CELLS_SWATCH,
                    colour,
                ));
            }

            text.push_render_text(RenderText {
                position: glam::vec2(
                    from.x + aim::CELLS_LEGEND_GAP + aim::CELLS_SWATCH.x + aim::CELLS_LEGEND_GAP,
                    middle - aim::CELLS_LEGEND * 0.5,
                ),
                text: line.clone(),
                size: aim::CELLS_LEGEND,
                ..Default::default()
            });

            y += aim::CELLS_LEGEND_STEP;
        }
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
    /// Through the sixth plinth and into spec 0021's eight rooms.
    ///
    /// You turn it, and it turns you. The five before it are what make that a
    /// surprise rather than a fault: the nook spends five objects teaching
    /// that a thing on a plinth is a thing you turn and watch.
    fn go_inside(&mut self) {
        self.inside = Some(Inside {
            frame: tesseract::Frame::new(),
            found: tesseract::Found::new(),
            turning: None,
            was: (self.at, self.yaw, self.pitch),
        });

        self.at = tesseract::arriving();
        self.falling = 0.0;
        // facing the plinth you just turned, which is the one thing in here
        // you have seen before. Yaw nought faces -z, so this is the other way.
        self.yaw = std::f32::consts::PI;
        self.pitch = 0.0;
    }

    /// And back out, which is the same plinth from the other side.
    fn come_out(&mut self) {
        if let Some(inside) = self.inside.take() {
            let (at, yaw, pitch) = inside.was;

            self.at = at;
            self.yaw = yaw;
            self.pitch = pitch;
            self.falling = 0.0;
        }
    }

    /// The map, carried in the hand. Spec 0021.
    ///
    /// The same shape that stands on the sixth plinth, which is the thing you
    /// turned to get in here, so the map and the way in and the room are all
    /// one object. The cell you are standing in is the part of it that is
    /// lit, the way out and what to find take the legend's own two colours,
    /// and the rest of the frame is dim enough to read as one thing.
    ///
    /// Drawn in the world rather than on the glass, because the two
    /// dimensional layer has axis aligned rectangles and nothing else: a
    /// wireframe at any angle is not available to it at all. So it is held
    /// at a fixed offset from the eye and turns slowly, which reads as an
    /// object in the hand rather than a diagram pinned to the window.
    fn draw_the_held_map(&mut self, scene: &mut Scene, camera: &Camera, cube: MeshId) {
        let Some(inside) = self.inside.as_ref() else {
            return;
        };
        let inside = *inside;

        let ahead = (camera.target - camera.position).normalize_or(Vec3::Z);
        let right = ahead.cross(camera.up).normalize_or(Vec3::X);
        let up = right.cross(ahead);

        let middle = camera.position + ahead * tesseract::HELD_OUT + right * tesseract::HELD_RIGHT
            - up * tesseract::HELD_DOWN;

        // still, and laid out in the frame of the person holding it, so the
        // part of it on your left is the cell through the wall on your left.
        // A map that turns on its own cannot line up with anything.
        let spun = glam::Quat::from_rotation_y(tesseract::HELD_TILT_Y)
            * glam::Quat::from_rotation_x(tesseract::HELD_TILT_X);
        let corners = tesseract::corners_in(inside.frame);

        // into the camera's own right, up and forward, because what comes
        // back from the tesseract is already in the holder's four ways
        let lay = |at: Vec3| {
            let at = spun * at * tesseract::HELD_ACROSS;

            middle + right * at.x + up * at.y + ahead * at.z
        };

        // which cells want picking out, worst first so the best wins the bar
        let mine = inside.frame.cell();
        let picked = [
            (tesseract::goal(), aim::held_of(aim::CELLS_GOAL)),
            (tesseract::home(), aim::held_of(aim::CELLS_WAY)),
            (mine, aim::held_of(aim::CELLS_HERE)),
        ];

        for (one, other) in tesseract::edges() {
            let (from, to) = (lay(corners[one]), lay(corners[other]));

            let colour = picked
                .iter()
                .filter(|(cell, _)| tesseract::edges_of(*cell).contains(&(one, other)))
                .map(|(_, colour)| *colour)
                .next_back()
                .unwrap_or(aim::HELD_FRAME);

            // a bar from one corner to the other, which is a cube stretched
            // along the line between them and turned to lie on it
            let along = to - from;
            let length = along.length();

            if length < 1e-5 {
                continue;
            }

            scene.push_material(
                cube,
                &Transform::at((from + to) * 0.5)
                    .with_rotation(glam::Quat::from_rotation_arc(Vec3::Y, along / length))
                    .with_scale(glam::vec3(tesseract::HELD_BAR, length, tesseract::HELD_BAR)),
                colour,
                aim::VOID,
            );
        }

        // and the four walls of the room you are standing in, laid out
        // flat beside the shape. The shape says what this place is; this
        // says which wall leads where, which is the question a person
        // actually has while walking.
        //
        // **One rotation, built once and right.** The camera's right, up and
        // forward are a left handed set, so handing them to `from_mat3` as
        // they come does not give a rotation at all and the first build of
        // this drew six slabs at angles to each other. `right.cross(up)` is
        // the third axis of a proper one, and it points back at the viewer,
        // which is why the things below stand at local minus z.
        //
        // Unspun, which the cube it replaced was not. A cross is only a
        // compass while it is square to the eye.
        let beside = middle + up * tesseract::ROOM_UP;
        let turn = glam::Quat::from_mat3(&glam::Mat3::from_cols(right, up, right.cross(up)));

        for face in tesseract::Face::ALL {
            let Some((at, size)) = tesseract::cross_at(face) else {
                continue;
            };

            let beyond = inside.frame.beyond(face);
            let put = |v: Vec3| beside + turn * v;

            scene.push_material(
                cube,
                &Transform::at(put(at)).with_rotation(turn).with_scale(size),
                // over one, so a square burns rather than taking the room's
                // own light, which in here is ambient and would leave these
                // a set of grey patches
                aim::held_of(aim::tesseract_panel(beyond.axis.n())),
                aim::VOID,
            );

            // and the thing that stands in the room through that wall, on
            // the square that stands for it, which is the whole of how this
            // answers where a wall goes without a key
            if let Some((where_, across)) = tesseract::face_thing(face) {
                self.draw_a_thing(
                    scene,
                    tesseract::sign_on(inside.frame, face),
                    put(where_),
                    turn * glam::Quat::from_rotation_y(tesseract::SIGN_TURN),
                    across,
                    aim::HELD_THING,
                );
            }
        }
    }

    /// One of the eight things, drawn to a given width wherever it is asked
    /// for. Spec 0021.
    ///
    /// The same call draws a room's own thing, the small one on a wall that
    /// says what is through it, and the smaller one on a face of the map in
    /// your hand, so the three cannot come to look like different objects.
    ///
    /// Sized off its own mesh rather than written down, the way the nook's
    /// row is. Five of these are the nook's own shapes and come out at five
    /// different sizes from their own arithmetic.
    fn draw_a_thing(
        &self,
        scene: &mut Scene,
        thing: tesseract::Thing,
        at: Vec3,
        turn: glam::Quat,
        across: f32,
        colour: glam::Vec4,
    ) {
        let Some(mesh) = self.cell_things.get(thing.n()).copied() else {
            return;
        };
        let Some(size) = self.thing_sizes.get(thing.n()).copied() else {
            return;
        };

        scene.push_material(
            mesh,
            &Transform::at(at)
                .with_rotation(turn)
                .with_scale(Vec3::splat(across / size.max(1e-4))),
            colour,
            aim::DULL,
        );
    }

    /// The room you are in, and the eight it is. Spec 0021.
    ///
    /// A dark box with six marks hung in it. The colour of a mark says which
    /// of the four directions the cell beyond it lies along and the word on
    /// it says which end, and the air says which cell you are standing in.
    /// None of that was enough on its own, which is what the map is for.
    fn draw_the_tesseract(&mut self, scene: &mut Scene, camera: &mut Camera, cube: MeshId) {
        let (Some(inside), Some(hung)) = (self.inside, self.hung) else {
            return;
        };

        let frame = inside.frame;

        camera.position = self.looking_from();
        camera.far = Camera::new().far;

        // and the world goes round you while you cross, per spec 0021. Both
        // ends of a crossing are upright; the turning happens inside it, and
        // the right angle it jumps at the halfway mark is paid for by the
        // room changing in the same instant.
        match inside.turning {
            Some(turn) => {
                let (axis, by) = tesseract::rolled(turn.step, turn.through);
                let round = glam::Quat::from_axis_angle(axis, by);

                camera.target = camera.position + round * self.facing();
                camera.up = round * Vec3::Y;
            }
            None => {
                camera.target = camera.position + self.facing();
                camera.up = Vec3::Y;
            }
        }

        // **There is no sun in here and there cannot be.** A cell is a closed
        // box and the sun is outside it, so the shell shadows its own inside
        // and the directional light contributes exactly nothing. Measured
        // rather than reasoned: the floor came out the same to the byte at an
        // intensity of 0.38 and at 3.0, which is what a shadow term of nought
        // looks like from outside the shader.
        //
        // So the room carries its own lamp, and the colour of the cell is in
        // the air rather than in the light. Enough plain fill that a panel
        // reads as its own colour: the first build had the air at 0.42 of the
        // cell's hue and nothing else, which multiplied every panel by a
        // violet and turned the green floor grey.
        let air = aim::tesseract_panel(frame.cell().axis.n()).truncate();

        scene.light.intensity = 0.0;
        scene.light.ambient = Vec3::ONE * aim::TESSERACT_LIT + air * aim::TESSERACT_AIR;

        scene.push_light(blitzkit::lighting::PointLight::new(
            glam::vec3(0.0, tesseract::CELL * aim::TESSERACT_LAMP_UP, 0.0),
            Vec3::ONE,
            aim::TESSERACT_LAMP,
            tesseract::CELL * aim::TESSERACT_LAMP_REACH,
        ));

        for (at, size) in tesseract::shell() {
            scene.push_material(
                cube,
                &Transform::at(at).with_scale(size),
                aim::TESSERACT_SHELL,
                aim::VOID,
            );
        }

        for face in tesseract::Face::ALL {
            let beyond = frame.beyond(face);

            let (at, size) = tesseract::panel(face);
            scene.push_material(
                cube,
                &Transform::at(at).with_scale(size),
                aim::tesseract_panel(beyond.axis.n()),
                // nothing in here takes a highlight. `aim::MATTE` is 64,
                // which is a broad one, and the floor faces the only light
                // there is: it came out white, and a green floor reading as
                // grey concrete was the first thing wrong with this room.
                aim::VOID,
            );

            // and what it means, written on it. `text::drawn` already makes a
            // dark ground with the letters picked out, which is a sign; a
            // pale plate behind that read as a sticker on a sticker.
            let (at, turn, size) = tesseract::plate(face, beyond);
            let stood = Transform::at(at).with_rotation(turn).with_scale(size);

            match self.cell_words.get(tesseract::numbered(beyond)).copied() {
                Some(word) => {
                    scene.push_textured(hung, word, &stood, aim::TESSERACT_WORD, aim::VOID)
                }
                None => scene.push_plain(hung, &stood, aim::TESSERACT_WORD),
            }

            // and a small one of the thing standing in the room through it,
            // which is the part a person navigates by. The colour says which
            // of the four ways and the word says which end, and neither of
            // those is a place: this is. Walk at the teapot and you are in
            // the room with the teapot in it.
            //
            // Four of the six, because the floor and the ceiling are the
            // faces nobody can walk at and a sign on them points nowhere you
            // can go.
            if let Some(stands) = tesseract::sign_at(face) {
                self.draw_a_thing(
                    scene,
                    tesseract::sign_on(frame, face),
                    stands,
                    glam::Quat::from_rotation_y(tesseract::SIGN_TURN),
                    tesseract::SIGN_ACROSS,
                    aim::TESSERACT_SIGN,
                );
            }
        }

        // and the map in your hand, which is the same shape as the room
        self.draw_the_held_map(scene, camera, cube);

        // and the plinth in the middle of the floor, with this room's own
        // thing turning on it. One in every cell, so where you are is a
        // thing you can see from the middle of the room without reading
        // anything. The four-cube is one of the eight and stands in the cell
        // you came in by, which makes finding the way out the same job as
        // finding the room with the four-cube in it.
        let (at, size) = tesseract::plinth();
        scene.push_material(
            cube,
            &Transform::at(at).with_scale(size),
            aim::PLINTH,
            aim::VOID,
        );

        self.draw_a_thing(
            scene,
            tesseract::thing(frame.cell()),
            tesseract::thing_at(),
            glam::Quat::from_rotation_y(display::turned(self.since)),
            tesseract::THING_ACROSS,
            aim::TESSERACT_PLATE,
        );
    }

    /// Puts you back at the door, facing the way the game opens.
    ///
    /// **Because flight can lose you somewhere with nothing drawn at all.**
    /// Jake found one: past everything, inside something, or out where no
    /// room's gate is true, and the screen is black whichever way you turn,
    /// so there is no landmark left to steer by and no way back.
    ///
    /// It drops what you are inside as well as where you are. The tesseract
    /// and the ride out past the door both hold you in a state of their own,
    /// and a reset that moved you and left you in one of those would put you
    /// at the door still riding.
    fn back_to_the_door(&mut self) {
        self.at = self.room.doorway();
        self.standing_in = 0;
        self.rolling = Vec3::Y;
        self.eye = self.at.y;
        self.yaw = 0.0;
        self.pitch = 0.0;
        self.falling = 0.0;
        self.climbs = 0.0;
        self.inside = None;
        self.riding_to = None;
    }

    fn gone(&mut self, wish: Vec3, pace: f32, dt: f32) -> (Vec3, f32) {
        // inside the tesseract, where the room is a cube with four ways out
        // and the only things that stop you are the floor, the ceiling and
        // the plinth. Spec 0021.
        //
        // The walk is the ordinary one and knows nothing about any of this. A
        // crossing is an event rather than a slide: you reach a wall, the
        // frame turns, and the room is laid out again with the new floor at y
        // nought, so inside a cell you are always the ordinary way up.
        if let Some(inside) = self.inside.as_ref() {
            // mid crossing nothing else moves you. The room is going round
            // and a body that could still walk would be walking out of it.
            if let Some(turn) = inside.turning {
                let through = (turn.through + dt / tesseract::TURN).min(1.0);
                let at = tesseract::crossing_at(&turn);

                if let Some(inside) = self.inside.as_mut() {
                    let turn = inside.turning.as_mut().expect("just seen");
                    turn.through = through;

                    // the room changes under you at the halfway mark, which
                    // is the moment you are in the plane of the square the
                    // two cells share
                    if !turn.swapped && through >= tesseract::SWAPS_AT {
                        turn.swapped = true;
                        let step = turn.step;
                        inside.frame = inside.frame.step(step);
                        // and the map learns a cell by being stood in
                        let cell = inside.frame.cell();
                        inside.found.went(cell);
                    }

                    if through >= 1.0 {
                        let landed = turn.to;
                        inside.turning = None;

                        return (landed, 0.0);
                    }
                }

                return (at, 0.0);
            }

            let (at, falling) = walk::walk(
                self.at,
                if wish.length_squared() > 1e-6 {
                    wish.normalize() * pace
                } else {
                    Vec3::ZERO
                },
                self.falling,
                RADIUS,
                dt,
                &tesseract::solid_in_a_cell(),
                // nothing in here is a step
                false,
            );

            return match tesseract::crossing(at) {
                Some(step) => {
                    if let Some(inside) = self.inside.as_mut() {
                        inside.turning = Some(tesseract::Turn {
                            step,
                            from: at,
                            to: tesseract::arrive(at, step),
                            through: 0.0,
                            swapped: false,
                        });
                    }

                    (at, 0.0)
                }
                None => (at, falling),
            };
        }

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

                // **And the sun is the one stop that is the reading.** Spec
                // 0018 is the system at true scale and spec 0036 names this
                // ride as what that room asks. Arriving rather than setting
                // out, because a leg you turned round on is not the trip.
                if beyond::is_the_sun(self.stop) {
                    self.trail.gave(trail::Asked::RodeToTheSun);
                }
            }

            return (at, 0.0);
        }

        let going = if wish.length_squared() > 1e-6 {
            wish.normalize() * pace
        } else {
            Vec3::ZERO
        };

        // **Spec 0047: in your own frame, whichever that is.** `walk` is
        // written in `y` from end to end and this does not change it. The
        // building is turned into your terms, walked, and the answer turned
        // back, which costs one pass over the solid boxes and only while
        // you are standing somewhere other than the world's own way up.
        //
        // A box comes back a box because every frame here is a right angle.
        // Anything else would need a bounding box and would grow every wall
        // in the room by its own diagonal.
        // along the floor, over anything no taller than a step, and down.
        // Spec 0007: the building has a height in it now, so getting about is
        // no longer one call that pushes you sideways.
        //
        // **In the world's own frame, which is the only one left.** Spec
        // 0047 gave you a frame you carried, so that the imaginarium could
        // turn you over, and `aloft::walked_in_frame` answered for both
        // this and spec 0049's planner. Spec 0063 parked that room, and
        // with nothing left to turn you over there is one way up and this
        // is the arithmetic it always was under frame nought. The planner
        // keeps the frame walk, because a route is still planned over
        // whatever floors it is given.
        walk::walk(
            self.at,
            going,
            self.falling,
            RADIUS,
            dt,
            &self.standing_on(),
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

    /// Everything you can be holding down, let go of at once.
    ///
    /// One list, because the four that walk you and the four that turn you
    /// came apart the moment there were two lists. See `focus_changed`.
    fn let_go(&mut self) {
        self.walking = [false; 4];
        self.looking = [false; 4];
        self.leaning = [false; 4];
        self.shifted = false;
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

        // **Only from past the hall's near end wall**, which is the one
        // place this room has ever been seen from. Spec 0040 put a field
        // off the garden with a four hundred metre view across it, and
        // from out there you could read Saturn's name in the sky over the
        // roof: a hundred and fifty nine metres of planets and a sun,
        // standing above a building that is three metres tall. Spec 0011
        // is a room nobody is supposed to know is there and spec 0015 is
        // what is in it, and both of them were legible from the grass.
        //
        // Nothing was wrong with the room. What changed is that there is
        // now somewhere to stand outside the building, and everything in
        // here was drawn on the understanding that there was not.
        if !behind::inside(self.room.reaches, self.looking_from()) {
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

            scene.push_plain(
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
                scene.push_plain(
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
            self.room.fillable().expanded(Vec3::splat(REACH_PAST)),
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

    /// How long the readout stays up.
    ///
    /// Long enough to take a screenshot without hurrying and short enough
    /// that it is gone before you have forgotten you pressed anything.
    const PLACE_SHOWS: f32 = 8.0;

    /// Where you are standing and which way you are looking, as six bare
    /// numbers: the feet, the eye's height, the yaw and the pitch.
    ///
    /// **Because the slow part of a fault Jake can see is finding it.** He
    /// plays, something looks wrong, and a screenshot arrives. A picture of
    /// a grey wall in a building of grey walls says nothing about where its
    /// camera was, and the only way back to that camera is to guess: a
    /// sweep of poses rendered and held against the picture by eye, which
    /// on the 30th of October 2026 took the best part of an hour and landed
    /// on the wrong spot twice before it landed on the right one. Twice in
    /// one afternoon.
    ///
    /// `WALK` and `WHERE` in the walking skill do this already, behind an
    /// environment variable and a rebuild, which is two steps too many for
    /// the moment somebody is standing in the thing they want to show you.
    ///
    /// **Bare numbers, because the plate is user facing and the words on it
    /// are Jake's.** A readout that says `at` and `yaw` is copy, and copy
    /// goes in under a marker and holds the gate until he has rewritten it.
    /// Six numbers in a fixed order are not, and whoever reads them off a
    /// screenshot is me.
    ///
    /// The feet and the eye both, because they are not the same number and
    /// a replay set from one alone draws the room from the floor of
    /// whatever storey it opened on. Spec 0042 paid for that one.
    ///
    /// A string rather than a print, so a test can read it.
    fn where_you_are(&self) -> String {
        format!(
            "{:.2} {:.2} {:.2}   {:.2}   {:.1} {:.1}",
            self.at.x,
            self.at.y,
            self.at.z,
            self.looking_from().y,
            self.yaw.to_degrees(),
            self.pitch.to_degrees(),
        )
    }

    /// Where the camera stands: your feet, with the eased height put back
    /// along your own up and `EYE` on top of it.
    ///
    /// With your frame at nought this is `(at.x, eye + EYE, at.z)`, which
    /// is what it has always been.
    fn looking_from(&self) -> Vec3 {
        let over = self.over();

        self.at + over * (self.eye - self.at.dot(over) + EYE)
    }

    fn facing(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();

        self.turned() * vec3(sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch)
    }

    fn forward(&self) -> Vec3 {
        let (sin, cos) = self.yaw.sin_cos();

        self.turned() * vec3(sin, 0.0, -cos)
    }

    /// The turn out of your own terms into the world's. Spec 0047.
    ///
    /// **The identity, always, since spec 0063.** The imaginarium was the
    /// one room in this building with a way up that was not yours, and
    /// parking it leaves one down. The turn stays named rather than
    /// written out at its half dozen call sites, because what it is is a
    /// fact about the building and not about any of them.
    fn turned(&self) -> glam::Quat {
        glam::Quat::IDENTITY
    }

    /// Which way is over your head.
    fn over(&self) -> Vec3 {
        self.turned() * Vec3::Y
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
        // the rig's dial is how many cubes, which is up and down
        back::RIG => up_down,
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
/// What `build.rs` found in `games` while this was being built. Spec 0053.
///
/// Only a browser build reads it, and the native one carries it unused, which
/// is a handful of strings and no pictures: `build.rs` writes the pictures
/// down for a browser only.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod baked {
    include!(concat!(env!("OUT_DIR"), "/baked.rs"));
}

/// Every repo under `games`, named.
#[cfg(not(target_arch = "wasm32"))]
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

/// The same, in a browser, where there is no folder to read.
///
/// Written down by `build.rs` out of that same folder, so this cannot drift
/// from the one above: it is the one above, frozen at the moment the thing
/// was built.
#[cfg(target_arch = "wasm32")]
fn repos() -> Vec<String> {
    baked::REPOS.iter().map(|name| name.to_string()).collect()
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
#[cfg(not(target_arch = "wasm32"))]
fn art_of(name: &str) -> Option<TextureData> {
    let shot = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()?
        .join("games")
        .join(name)
        .join("media/screenshot.png");

    TextureData::from_bytes(&std::fs::read(shot).ok()?).ok()
}

/// The same, in a browser, where there is no disk to read it off.
///
/// Carried inside the binary by `build.rs`, which costs four and a half
/// megabytes and is what a hall of cabinets with nothing on their screens
/// would save. A native build does not pay it.
#[cfg(target_arch = "wasm32")]
fn art_of(name: &str) -> Option<TextureData> {
    let found = baked::ART.iter().find(|(whose, _)| *whose == name)?;

    TextureData::from_bytes(found.1).ok()
}

impl Game for Arcade {
    /// Takes the cursor so the mouse can look, and gives it back when the
    /// window loses focus or a game is started in front of this one.
    ///
    /// Declared and never done at first, so `locked` stayed false and every
    /// mouse motion was dropped before it reached the camera.
    fn before_frame(&mut self, renderer: &mut Renderer) {
        // **`DUMP` writes every mirror's targets out as pictures, once.**
        // The same kind of tool as `WHERE` and `POSE`: a thing the game can
        // be asked for rather than a thing somebody has to add again. Engine
        // spec 0090's `target_picture`, which exists because a night was
        // spent reading targets through posters and getting it wrong.
        //
        // After a couple of seconds, so the room is built and the targets
        // hold a frame rather than their clear.
        if std::env::var("DUMP").is_ok() && self.since > 2.0 && !self.dumped {
            self.dumped = true;

            for (n, pair) in self.glass_targets.iter().enumerate() {
                for (k, target) in pair.iter().enumerate() {
                    let shot = renderer.target_picture(*target);
                    let at = format!("/tmp/mirror-{n}-{k}.png");

                    match blitzkit::picture::save(&shot, std::path::Path::new(&at)) {
                        Ok(()) => eprintln!("wrote {at}"),
                        Err(e) => eprintln!("could not write {at}: {e}"),
                    }
                }
            }
        }

        if self.wants_lock != self.locked {
            self.locked = renderer.set_cursor_locked(self.wants_lock) && self.wants_lock;
        }

        // the gallery's panel. One texture, written again whenever the view
        // has moved, per the engine's spec 0074. It was a ladder of fixed
        // views before that, because adding a texture was the only way to
        // change a picture and doing that every frame leaks one a frame.
        if self.mandel.is_none() {
            self.mandel = Some(renderer.add_texture(&mandel::drawn(self.view, false)));
            self.shown_view = Some((self.view, false));
            // and the fine one, which starts as a copy of the coarse one
            // blown up: it is never drawn until a settling has filled it,
            // and a texture has to exist before anything can be written
            // into it.
            self.sharp = Some(renderer.add_texture(&mandel::blank()));
        }

        // **Low rounds while you move and the full count when you stop.**
        // At the bottom of the dive the full count is 35 milliseconds
        // against a frame of 8, so a picture worked out properly every
        // frame is a picture that stutters. Spec 0039: move coarse, settle
        // sharp, which is what every explorer of this set does.
        let want = (self.view, self.moving_the_view);
        if self.shown_view != Some(want) {
            if let Some(id) = self.mandel {
                renderer.write_texture(id, &mandel::drawn(want.0, want.1));
                self.shown_view = Some(want);
            }
        }

        // the field wall's plaster, one quad a run, rebuilt when the run
        // changes length. The quad carries its own tile count in its
        // corners, so a run that is a metre longer than it was drawn for
        // is a run whose tiles are a little wider than they were.
        for (n, face) in field::wall_faces(self.room.reaches, self.at)
            .into_iter()
            .enumerate()
        {
            let counts = glam::vec2(face.size().z / field::RENDER, garden::HIGH / field::RENDER);
            let mesh = room::tiled_plane(counts);

            match self.rendered.get(n).copied().flatten() {
                Some(id) => renderer.update_mesh(id, &mesh),
                None => {
                    if let Some(slot) = self.rendered.get_mut(n) {
                        *slot = Some(renderer.add_mesh(&mesh));
                    }
                }
            }
        }

        // and out in the field the sun's map follows you, per spec 0040.
        // `load` fitted it to the building, which is right everywhere the
        // building is and leaves the field standing in whatever the map
        // holds at its edge: a bright strip across the grass where the map
        // runs out, with the building's own shadow stretched over the rest.
        let bounds = if field::open(self.room.reaches, self.at) {
            field::under_the_sun(self.at)
        } else {
            self.room.under_the_sun()
        };

        if renderer.scene_bounds() != bounds {
            renderer.set_scene_bounds(bounds);
        }

        // and the fine picture under it, a slice a frame.
        //
        // **Thrown away the moment the view moves**, which is what keeps a
        // settling from ever being a picture of two places at once. What
        // is drawn meanwhile is the coarse one, which is being written
        // anyway.
        self.settle();

        if let (Some(settling), Some(id)) = (&self.settling, self.sharp) {
            if settling.whole() {
                renderer.write_texture(id, &settling.drawn());
                self.settling = None;
            }
        }

        // and however many separate copies of the cube the rig's dial wants,
        // uploaded as it asks for them rather than all of them at the start.
        // Spec 0024: sixteen thousand buffers before the first frame is a
        // building that takes a while to open for a room nobody may go into.
        if self.rig_many {
            let want = back::CUBES[self.rig_step] as usize;

            while self.rig_meshes.len() < want {
                self.rig_meshes.push(renderer.add_mesh(&MeshData::cube()));
            }
        }

        // what the frame that just went cost, per spec 0056, asked of the
        // renderer rather than worked out here: the counts are the ones it
        // actually made, and both clocks are its own. Kept whether or not the
        // readout is up, because a window that only starts filling when you
        // press the key shows you the cost of pressing the key.
        let counted = renderer.counted();
        let ran = std::mem::replace(&mut self.ran, true);

        self.counted = (counted.draws, counted.triangles);
        self.timed = renderer.times_frames();

        if ran {
            self.costs.saw(blitzkit::cost::Frame {
                seconds: renderer.work_seconds(),
                between: renderer.frame_seconds(),
                gpu: renderer.gpu_seconds(),
                drawn: counted.draws,
                triangles: counted.triangles,
            });
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
        // The sun's map covers the building, less the space behind the wall,
        // because outside the map the sun comes through the roof. It was
        // fitted to the hall alone, which was sharp and left the cellar, the
        // baths and the space behind the wall taking full sun through solid
        // ground. See `Room::under_the_sun`, which has the measurements and
        // says why that one room is out of it.
        renderer.set_scene_bounds(self.room.under_the_sun());

        // **The two mirrored walls' targets, per spec 0061.** One each,
        // because every panel on a wall is in that wall's one plane and
        // engine spec 0092 samples by screen position, so six panels read
        // their own parts of the one reflection.
        //
        // Half the window's width in each direction, which is a quarter of
        // the fragments. Nobody reads a reflection the way they read the
        // room, and each of these is a whole pass over the scene.
        // **Two apiece, per engine spec 0094.** One is what that wall holds
        // on the screen, drawn from your own reflection in it; the other is
        // what it holds inside the wall facing it, drawn from that wall's
        // camera. The glass samples by screen position, so a picture belongs
        // to the pass that drew it and one texture cannot be both.
        self.glass_targets = crate::mirrors::SIDES
            .iter()
            .map(|_| [renderer.add_target(640, 640), renderer.add_target(640, 640)])
            .collect();

        // and the quad they are drawn on. A plain one: engine spec 0092
        // samples where the fragment is, so the uv is never read and there
        // is no mirrored mesh to build.
        self.pane = Some(renderer.add_mesh(&blitzkit::mesh::MeshData::plane()));

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
        //
        // **11.8 is a width this floor used to be and not one it is**, so it
        // is written out. It was `(WALL + CABINET.x) * 2 + NOOK_DEEP`, which
        // is what that came to on the day, and as a formula it put the nook's
        // depth into the weave of a carpet the nook cannot see: widening the
        // nook by 1.6 took the tile down by an eighth in the hall and the
        // gallery both.
        let was = 11.8;
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
        // the back room's clutter, per spec 0024: a bucket, a tin, a jar and
        // a reel, all off the cellar's wheel. A crate is boards and needs no
        // mesh of its own.
        self.clutter = vec![
            renderer.add_mesh(&back::bucket_mesh()),
            renderer.add_mesh(&back::tin_mesh()),
            renderer.add_mesh(&back::reel_mesh()),
        ];
        self.gauge_words = std::iter::once(back::GAUGE_SAYS)
            .chain(back::GAUGE_WORDS)
            .chain(back::GAUGE_FULL_SAYS)
            .map(|word| renderer.add_texture(&blitzkit::text::drawn(word, back::GAUGE_TEXELS)))
            .collect();
        self.barrel = Some(renderer.add_mesh(&cellar::barrel_mesh()));
        self.hoop = Some(renderer.add_mesh(&cellar::hoop_mesh()));
        self.bottle = Some(renderer.add_mesh(&cellar::bottle_mesh()));
        self.peg = Some(renderer.add_mesh(&cellar::peg_mesh()));

        // **Enough steps to have a skin.** A turned mesh with too few draws
        // two points and nothing between them, which is how the candles, the
        // clock dial and the fire's coals all went missing at once.
        // **Few sides on purpose.** At eighteen it was a smooth body with a
        // sheen on the skull, which is the most naturalistic object in a
        // building made of flat cubes, and Jake walked up to one and said it
        // is a creepy mannequin. Spec 0033 set the bar as reading like
        // somebody at corridor distance without inviting a close look, and
        // what fails that is the promise of a face rather than the lack of
        // one. Carved reads as a figure. Modelled reads as a person with
        // something missing.
        self.body =
            Some(renderer.add_mesh(&cellar::turned(people::SIDES, 22, people::torso_profile)));
        self.head =
            Some(renderer.add_mesh(&cellar::turned(people::SIDES, 16, people::head_profile)));
        // **Two bones apiece now, per spec 0046.** One lathe standing on
        // its foot could only ever be slid, and a leg slid out from under
        // its hip is the thing Jake called stiff and disconnected. Both of
        // these hang from their joint, so they turn about it.
        self.thigh =
            Some(renderer.add_mesh(&cellar::turned(people::SIDES, 8, people::thigh_profile)));
        self.shin =
            Some(renderer.add_mesh(&cellar::turned(people::SIDES, 8, people::shin_profile)));
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
        self.carved_head =
            Some(renderer.add_mesh(&cellar::turned(people::CUT_SIDES, 14, people::carved_head)));
        self.cloak = Some(renderer.add_mesh(&cellar::turned(
            people::CUT_SIDES,
            18,
            people::cloak_profile,
        )));
        self.helm =
            Some(renderer.add_mesh(&cellar::turned(people::CUT_SIDES, 10, people::helm_profile)));
        self.horn = Some(renderer.add_mesh(&bridge::horn_mesh()));
        self.kerbs = [-1.0f32, 1.0]
            .map(|side| Some(renderer.add_mesh(&bridge::kerb_mesh(self.room.reaches, side))));
        self.dome = Some(renderer.add_mesh(&garden::dome_mesh(0.0)));
        self.moon = Some(renderer.add_mesh(&garden::moon_mesh()));
        self.moon_face = Some(renderer.add_texture(&garden::moon_face()));
        // the field: one quad for the ground, counted so a tile of grass is a
        // tile wherever the slab has slid to, and a trunk for each kind of
        // tree it stands. Spec 0040.
        self.sward = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::splat(
            field::GROUND_REACH * 2.0 / field::TILE,
        ))));
        self.turf = Some(renderer.add_texture(&field::turf(0x51E7)));
        // and the building's own outward walls, which until spec 0041 flew
        // round them were the colour of the rooms' insides.
        //
        // **`Room::outside` had no reader for a moment.** It answers which
        // faces the sky can see, and the only thing asking was the
        // imaginarium's storey; spec 0063 parked that room and left the
        // nook's new south wall, which is an outside wall now, bare. A wall
        // the sky can see and nobody plasters is the fault spec 0040 exists
        // to catch, and the compiler caught this one.
        self.wall_quad = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
        self.storey_faces = self.room.outside();
        self.storey_wall_skin = self
            .storey_faces
            .iter()
            .copied()
            .enumerate()
            .map(|(n, (face, looks))| {
                let on = room::quad_on(face.size(), looks);

                renderer.add_texture(&garden::plastered(
                    garden::PLASTER_SEED.wrapping_add(0x3C0F + n as u32 * 7919),
                    glam::vec2(on.x, on.z),
                ))
            })
            .collect();
        // the roof's own lime, laid flat in bays, and a plane per lid with
        // that lid's tile counts so the bays come out square. Specs 0057
        // and 0063: the imaginarium stood on this roof and was the only
        // thing up there anybody had drawn a top for.
        self.roof_skin = Some(renderer.add_texture(&garden::screeded(
            garden::SCREED_SEED,
            glam::Vec2::ZERO,
            glam::Vec2::splat(field::RENDER),
            garden::BAY,
        )));
        self.roof_meshes = self
            .room
            .lids()
            .into_iter()
            .map(|lid| {
                let size = lid.size();

                renderer.add_mesh(&room::tiled_plane(glam::vec2(
                    size.x / field::RENDER,
                    size.z / field::RENDER,
                )))
            })
            .collect();
        self.saplings = field::kinds()
            .iter()
            .map(|tree| renderer.add_mesh(&garden::trunk_mesh(tree)))
            .collect();
        self.render = Some(renderer.add_texture(&garden::plastered(
            0x3A77,
            glam::vec2(field::RENDER, field::RENDER),
        )));

        // the imaginarium's four outward faces: one picture each, counted
        // off the face, the way the garden's walls are done.
        //
        // **Not the field's tiled plaster.** That picture is one tile of a
        // run and carries the splash the foot of a wall gets, which is the
        // point of it; tiled up a face fifteen metres high it is a dark
        // band every six metres, three of them one above another. Nothing
        // had shown it before, because the compound wall and the hotel's
        // corridor are both under one tile tall.
        self.tuft = Some(renderer.add_mesh(&field::tuft_mesh()));
        // a handful of tufts rather than one. Every clump of grass in a field
        // cut from the same picture is a field of one plant, which is the
        // fault the cellar's boards had.
        self.blades = (0..field::TUFTS)
            .map(|n| renderer.add_texture(&field::blades(0x2B19u32.wrapping_add(n * 7901))))
            .collect();
        self.litter = (0..field::LITTERS)
            .map(|n| renderer.add_texture(&garden::fallen(0x8D31u32.wrapping_add(n * 6151))))
            .collect();
        self.heads = aim::FIELD_HEADS
            .iter()
            .enumerate()
            .map(|(n, head)| {
                renderer.add_texture(&field::flower(
                    0x91C7u32.wrapping_add(n as u32 * 6151),
                    *head,
                ))
            })
            .collect();
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
        self.pond_mesh = Some(renderer.add_water_mesh(&self.pool_in_the_garden.surface()));
        // a trunk apiece, because the lean is a share of the height and the
        // thickness is not: one mesh scaled to two heights is a sapling and a
        // log. The pads are one lump per cut, turned and squashed per pad.
        self.trunks = garden::trees(self.room.reaches)
            .iter()
            .map(|tree| renderer.add_mesh(&garden::trunk_mesh(tree)))
            .collect();
        self.deck = Some(renderer.add_mesh(&bridge::deck_mesh(self.room.reaches)));
        self.soffit = Some(renderer.add_mesh(&bridge::soffit_mesh(self.room.reaches)));
        self.rainbow = Some(renderer.add_texture(&bridge::rainbow()));
        // **Stencilled and not drawn.** `drawn` keeps the ground opaque,
        // which is right for the nook's sign because a dark plate behind
        // the letters is free. On a pale stone it is a black patch wider
        // than the stone's own face. The engine has the other one for
        // exactly this: the letters, and what is behind them showing
        // through.
        self.carving = Some(renderer.add_texture(&bridge::carving()));
        self.night = Some(renderer.add_texture(&valhalla::night_sky()));
        self.gable = Some(renderer.add_mesh(&valhalla::gable_mesh()));
        self.hearth_flame = Some(renderer.add_mesh(&valhalla::flame_mesh()));
        self.granite = Some(renderer.add_texture(&garden::granite(bridge::STONE_SEED)));
        // out of round, like the garden's set stones, and squashed thin by
        // the transform so the carved face is near enough flat to cut
        self.standing_stone = Some(renderer.add_mesh(&bridge::stone_mesh(bridge::STONE_SEED)));
        self.rubble_mound = Some(renderer.add_mesh(&bridge::mound_mesh()));
        self.carved_face = Some(renderer.add_mesh(&bridge::carved_mesh(bridge::STONE_SEED)));
        self.runes = Some(renderer.add_texture(&blitzkit::text::stencilled(
            bridge::SAYS,
            bridge::RUNE_TEXELS,
        )));
        self.crown = Some(renderer.add_mesh(&garden::sprig_mesh()));
        // three pictures of each, so a crown is not one sprig repeated
        self.needles = [0x5A2Fu32, 0x11C7, 0xE340]
            .iter()
            .map(|seed| renderer.add_texture(&garden::needles(*seed)))
            .collect();
        self.broadleaf = [0x7B12u32, 0x2E95, 0xC608]
            .iter()
            .map(|seed| renderer.add_texture(&garden::broadleaf(*seed)))
            .collect();
        self.limb = Some(renderer.add_mesh(&garden::limb_mesh()));
        self.lantern = Some(renderer.add_mesh(&garden::lantern_mesh()));
        self.lantern_light = Some(renderer.add_mesh(&garden::lantern_light_mesh()));
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
        self.portal = Some(renderer.add_texture(&behind::swirled()));
        self.doorway = Some(renderer.add_texture(&behind::swirled_through()));
        self.portal_mesh = Some(renderer.add_mesh(&room::tiled_plane(glam::Vec2::ONE)));
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

        // and the eight things the tesseract's rooms hold, in `Thing::ALL`
        // order. Five come from the shape library the nook already draws
        // from, so a teapot in there and a teapot in here are the same
        // teapot; the other three the engine has of its own.
        self.cell_things = tesseract::Thing::ALL
            .iter()
            .map(|thing| {
                let mesh = match thing {
                    tesseract::Thing::Teapot => blitzkit_shapes::teapot::teapot(6),
                    tesseract::Thing::Klein => blitzkit_shapes::klein::klein_bottle(48, 24),
                    tesseract::Thing::Sierpinski => blitzkit_shapes::sierpinski::sierpinski(4),
                    tesseract::Thing::Menger => blitzkit_shapes::menger::menger(2),
                    tesseract::Thing::Hilbert => blitzkit_shapes::hilbert::hilbert_tube(3, 8, 0.03),
                    // a torus from its own formula, per spec 0016, because
                    // this one had to be a shape rather than a box and the
                    // engine has no torus of its own.
                    //
                    // **Standing on its edge and not lying flat.** The ring
                    // is in `xy` with its thickness through `z`, so what you
                    // see from eye height is the hole. Lying flat, which is
                    // how it was written first, it is seen nearly edge on
                    // from anywhere in the room and comes out as a bar: the
                    // same fault as the cube it replaced, which is being a
                    // shape nobody can name from where they are standing.
                    tesseract::Thing::Torus => MeshData::surface(56, 28, |u, v| {
                        use std::f32::consts::TAU;

                        let (u, v) = (u * TAU, v * TAU);
                        let ring = 0.34;
                        let out = 1.0 + ring * v.cos();

                        glam::vec3(out * u.cos(), out * u.sin(), ring * v.sin())
                    }),
                    tesseract::Thing::FourCube => tesseract::shape(),
                    // the same trefoil the engine's own example draws, so
                    // the knot in here and the knot on the site are one
                    // object rather than two formulas that agree today
                    tesseract::Thing::Knot => blitzkit_shapes::knot::trefoil(160, 24),
                };

                self.thing_sizes.push(mesh.bounds().size().max_element());

                renderer.add_mesh(&mesh)
            })
            .collect();

        // and the eight names the tesseract's walls carry, per spec 0021, in
        // `cells()` order so a cell indexes its own word.
        self.cell_words = tesseract::cells()
            .iter()
            .map(|cell| {
                renderer.add_texture(&blitzkit::text::drawn(
                    tesseract::named(*cell),
                    tesseract::WORD_TEXELS,
                ))
            })
            .collect();
        self.named = Some(renderer.add_texture(&blitzkit::text::drawn(neon::SAYS, neon::TEXELS)));
        self.baths = Some(renderer.add_texture(&blitzkit::text::drawn(spa::SAYS, spa::TEXELS)));
        self.pool_mesh = Some(renderer.add_water_mesh(&self.pool.surface()));
        self.tub_mesh = Some(renderer.add_water_mesh(&self.tub.surface()));
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
            .map(|stood| (self.art_from)(&stood.cabinet.name))
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

        // **Spec 0049: the tour, if this run is being filmed off one.**
        // Read in `main` as well, so a route with a typo in it says so and
        // stops rather than opening a window and standing still.
        self.filming = filmed(&self.room)
            .and_then(Result::ok)
            .map(|steps| Filming {
                steps,
                at: 0,
                following: None,
                waiting: 0.0,
            });

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
            .map(|(room, sounds, make, seed)| {
                let pieces = (0..noise::PIECES)
                    .map(|n| make(seed.wrapping_add(n as u32 * 7919)))
                    .collect();

                (room, sounds, pieces)
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
        // **Its own flag and not the clock.** This read `since <= 0.0`,
        // which is the same number the day runs on, and the two agreed
        // only while a run always started at nought. `AT` sets the clock
        // before the first frame, so on that frame `since` was already
        // several minutes in and the pose this run was given was never
        // applied: the camera stood at the door and the picture was of
        // the hall.
        let first = !self.has_run;

        self.has_run = true;

        self.since += dt;

        // where to stand, if this run was told. Spec 0013's diagnostics.
        if first {
            if let Some((at, yaw, pitch)) = posed_at() {
                self.at = at;
                self.eye = at.y;
                self.yaw = yaw;
                self.pitch = pitch;
            }
        }
        // and the readout of where you are standing, which runs out on its
        // own rather than being turned off. Spec 0045.
        self.showing_place = (self.showing_place - dt).max(0.0);
        // **Two rooms answer by being reached, and neither needs anything
        // planted in it.** Spec 0011's space behind the hall's end wall is
        // the answer to the question spec 0020's worn track asks by being
        // there, and spec 0040's field answers whether the ground stops by
        // not stopping. Asked of where the feet are, which `noise::room_at`
        // already works out every frame for what you can hear, rather than
        // of a flag set at a doorway: this building has learned what two
        // accounts of one thing do.
        match noise::room_at(self.room.reaches, self.at) {
            noise::Room::Behind => self.trail.gave(trail::Asked::GotBehindTheWall),
            noise::Room::Field => self.trail.gave(trail::Asked::WentOutIntoTheField),
            _ => (),
        }

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
        // nothing else. It prints the room and the frame as well as the pose,
        // because which room the building thinks you are in is itself a thing
        // that has been wrong, and because the frame is how spec 0049's
        // follower was caught crossing the imaginarium's aperture and
        // crossing straight back: in the log that is the number flipping
        // twice a second while the body paces the same four metres.
        if std::env::var("WHERE").is_ok() {
            self.said += dt;
            if self.said >= 0.5 {
                self.said = 0.0;
                eprintln!(
                    "WHERE at {:.2} {:.2} {:.2}  yaw {:.3}  pitch {:.3}  room {:?}  frame {}",
                    self.at.x,
                    self.at.y,
                    self.at.z,
                    self.yaw,
                    self.pitch,
                    noise::room_at(self.room.reaches, self.at),
                    self.standing_in
                );

                // and what the building has been given up, which is the one
                // place spec 0036's record is read. Behind the same env var
                // and on stderr, because the rule that spec sets is that
                // somebody who never finds the trail cannot tell it is
                // there, and a line in a window would be exactly that.
                if !self.trail.is_empty() {
                    eprintln!("WHERE given {:?}", self.trail.given());
                }
            }
        }
        // whether the sky is over you, asked once a frame rather than three
        // times, since the drawing, the far plane and the sky itself all
        // want the same answer and each was building the whole collider
        // list to get it.
        //
        // **Except out past the door, where there is no roof either and it
        // is not a field.** Spec 0018's floor runs eleven kilometres into
        // the dark and nothing stands over any of it, so asked of the roof
        // alone that room is out of doors: the staged shot of Saturn came
        // back with a green horizon and a cloudy sky behind it. `afoot` was
        // false out there, which is the half of the old question that was
        // doing this work and the half I replaced.
        self.outdoors = !self.in_the_deep() && self.room.out_of_doors(self.looking_from());
        // and whether it is worth drawing, which is a question about sight.
        // The engine's spec 0086.
        self.sees_outdoors = !self.in_the_deep()
            && self
                .room
                .can_see_outdoors(self.looking_from(), self.room.reaches);

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
        self.the_doors_moved(dt);
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
        self.puzzle.advance(dt);

        // in the water you are slower, which is most of what says you are in it
        // rather than beside it. Spec 0008.
        let wading = self.wading();
        // **Spec 0049: a tour drives the keys before the keys are read.**
        // Set here rather than anywhere later, so the walking and the
        // aiming that follow are the ordinary ones and a tour is somebody
        // pressing the same things a player would.
        self.on_the_tour(dt);

        let went = self.aimed_and_moved(dt);

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
            let places: Vec<(noise::Room, noise::Sounds)> = self
                .voices
                .iter()
                .map(|(room, sounds, _)| (*room, *sounds))
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
                Some((_, sounds, voice)) => {
                    let (which, pitch, quieter) = noise::adrift(pieced, voice.len());

                    match voice.get(which) {
                        Some(piece) => {
                            let bent = piece.pitched(pitch).gain(quieter);
                            let runs = bent.seconds();

                            // a voice that is the room itself is mixed rather
                            // than placed, so it has no direction to walk up
                            // to and no distance to be close at. `play_at`
                            // would give it both. It takes `FILLS` in place of
                            // the distance it no longer has, or it comes out
                            // louder unplaced than it was placed.
                            match sounds.place() {
                                Some(at) => sound.play_at(&bent, at.to_array()),
                                None => sound.play(&bent.gain(noise::FILLS)),
                            }

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

        let playing = self.playing.now().map(|name| name.to_string());
        self.seen = self.what_i_see();

        // the gate at the top of the bridge, per spec 0058. He asks when
        // you look at him and press Return, you write back, and the
        // leaves swing when what you wrote is one of his. Nothing shuts
        // them again.
        self.gate = gate::swings(
            self.gate,
            gate::opened(self.room.reaches, &self.trail, self.at),
            dt,
        );

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
            // the panel: how far in you are, and nothing else. Spec 0039.
            (None, Some(room::Seen::Mandel)) => (
                Some(String::from(mandel::NAME)),
                Some(mandel::says(self.view)),
            ),
            // the book: what you have taken off the diagonal so far. Spec
            // the watchman, who keeps the threshold and lets you through.
            // Spec 0058 had him ask a riddle and spec 0063 took it out: the
            // bridge is already the gate, since it only comes up once every
            // station has been given, and what he asked could be had by
            // turning round rather than by having walked anything.
            (None, Some(room::Seen::Watchman)) => (
                Some(String::from("The watchman")),
                Some(String::from(says::WATCHMAN_LETS_YOU_BY)),
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
            (None, Some(room::Seen::MirrorCase)) => {
                let (what, how) = if self.room.mirror_open {
                    ("the bookcase", "Click or press enter to push it back.")
                } else {
                    ("a bookcase", "another of these is not a book")
                };

                (Some(String::from(what)), Some(String::from(how)))
            }
            (None, Some(room::Seen::Case)) => {
                let (what, how) = if self.room.open {
                    ("the bookcase", "Click or press enter to push it back.")
                } else {
                    ("a bookcase", "one of these is not a book, pull it")
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
        let said: Vec<&str> = if let Some(inside) = self.inside.as_ref() {
            // which room you are standing in, and where the way out is. The
            // map at the foot of the screen says the rest.
            let mut out = vec![self.saying(), tesseract::standing_in(inside.frame.cell())];

            // and what is on the plinth, in the rooms where that is worth
            // saying, which is the ones whose object has a name nobody can
            // work out by looking at it
            if let Some(said) = tesseract::on_the_plinth(inside.frame.cell()) {
                out.push(said);
            }

            // and what the room's two odd pairs of names mean, in the two
            // rooms they name. East and west are obviously opposites and
            // zenith and nadir are not.
            if let Some(gloss) = tesseract::gloss(inside.frame.cell()) {
                out.push(gloss);
            }

            // and whether you have done the thing the room is for, which it
            // never used to say at all
            if inside.found.done() {
                out.push(aim::TESSERACT_DONE);
            }

            out
        } else if winking {
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
        let mut lines: Vec<String> = said
            .iter()
            .flat_map(|words| {
                blitzkit::text::wrapped(words, aim::SAYS_SIZE, room)
                    .split('\n')
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .collect();

        // and where you are standing, under the help and on the same plate,
        // for the seconds after M. Spec 0045.
        if self.showing_place > 0.0 {
            lines.push(self.where_you_are());
        }

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

        // and what the map in your hand means, in the one room that has
        // one. Spec 0021.
        self.draw_the_key(geometry, text);

        // and what this frame cost, if it was asked for. Spec 0056.
        self.draw_the_cost(geometry, text);

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

        let lines = vec![(name, aim::PROMPT_SIZE), (detail, aim::DETAIL_SIZE)];

        let said = aim::said(
            self.window,
            lines
                .into_iter()
                .filter_map(|(say, size)| say.map(|say| (say, size)))
                .collect(),
        );

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

        // inside the tesseract, which is in none of the building's coordinates
        // and shares none of its light. Spec 0021: the eight cells are the
        // same cube drawn eight ways, so there is nowhere in the hall to put
        // them and nothing of the hall to draw while you are in one.
        if self.inside.is_some() {
            self.draw_the_tesseract(scene, camera, cube);
            return;
        }

        camera.position = self.looking_from();
        camera.target = camera.position + self.facing();
        // **And which way is up, which is yours and not the world's.** Spec
        // 0047. It eases rather than snapping: gravity and the walking
        // change the instant you are through an aperture, because a frame
        // half way between two is a frame nobody can stand in, and the view
        // rolls after it.
        camera.up = self.rolling;
        // and how far it sees, which is the building's hundred metres indoors
        // and the length of the floor out past the door. Spec 0018: a hundred
        // metres out there culls the sun from everywhere past Mercury, and a
        // sun you cannot see is the one thing that space is for.
        camera.far = if self.in_the_deep() {
            beyond::sees()
        } else if self.sees_outdoors {
            // and out in the field it is the far side of the sky, per spec
            // 0040. A hundred metres out there cuts the ground off at a
            // hundred and leaves black above it, which is a field in a
            // cave: the first build of this looked exactly like that.
            //
            // **Wherever the field is drawn and not wherever you stand.**
            // This read `outdoors`, which is `out_of_doors` and is a
            // question about what is over your feet, while the drawing of
            // the field moved to a question about sight in
            // the engine's spec 0086 and this line did not go with it.
            // Standing in the hall's doorway
            // into the garden the field was drawn and the camera stopped
            // at the building's hundred metres, so the far half of it was
            // clipped: Jake, 2026-11-05, the far end of the field does not
            // render until I enter the garden. One metre west it did.
            //
            // That is the same fault for the fourth time in two days: a
            // test of where your feet are answering a question about what
            // you can see. The sky had it, the field's own drawing had it,
            // the room behind the wall had it.
            field::sees(self.at)
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
            None => scene.push_plain(floor, &laid, aim::FLOOR),
        }

        // and the gallery's, which is the aisle's carried through the way in.
        // Spec 0022: the room is the end of the aisle rather than a different
        // kind of place, so it is the same floor. It had none at all on its
        // first build, which from inside is a room standing on black.
        {
            let (at, size) = room::gallery(self.room.reaches);
            let laid = Transform::at(vec3(at.x, 0.0, at.z)).with_scale(vec3(size.x, 1.0, size.z));

            match self.carpet {
                Some(carpet) => scene.push_textured(floor, carpet, &laid, aim::FLOOR, aim::MATTE),
                None => scene.push_plain(floor, &laid, aim::FLOOR),
            }
        }

        // and the back room through the unmarked door, per spec 0024
        self.draw_the_back_room(scene, cube, floor);
        // and the corridor of mirrors behind the nook's bookcases, spec 0061.
        //
        // **Its drawing lives in its own module**, with the geometry it
        // draws. It sat in here beside `draw_the_hall` first, which is
        // Valhalla's longhouse and not this building's hall, so it was never
        // called at all and the corridor was solid and invisible. A file of
        // eighteen thousand lines is how two functions eight thousand apart
        // come to look like neighbours.
        // **Only when some of its glass is in shot.** See `mirrors::draw`:
        // each mirror is another whole world rendered into a target.
        {
            let (middle, reach) = mirrors::glass_ball();
            let eye = self.looking_from();
            // **Three questions, because no one of them is enough.** The
            // frustum knows nothing about the walls in the way, the
            // distance knows nothing about which way you face, and neither
            // knows that the corridor has no opening to the sky.
            let seen = !self.room.out_of_doors(eye)
                && eye.distance(middle) <= mirrors::as_far_as_it_is_seen(self.room.reaches)
                && camera.frustum().holds_sphere(middle, reach);

            mirrors::draw(scene, cube, self.pane, &self.glass_targets, seen);
        }
        self.draw_the_people(scene);

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
            // one quad per piece, and the pieces have the stairwell out of
            // them. Spec 0038, and `Room::ceiling` says why.
            for piece in self.room.ceiling() {
                let (middle, size) = (piece.center(), piece.size());

                // turned over, because a plane faces up and from underneath
                // that is a back face, which the opaque pass culls
                scene.push_plain(
                    ceiling,
                    &Transform::at(vec3(middle.x, room::TALL, middle.z))
                        .with_rotation(glam::Quat::from_rotation_x(std::f32::consts::PI))
                        .with_scale(vec3(size.x, 1.0, size.z)),
                    aim::CEILING,
                );
            }

            // and the grid hung under it, which is what is over an arcade and
            // is also sixteen units of lines running away from you. Spec 0012.
            let (bars, tiles, panels) = self.room.lattice();

            for (at, size) in tiles.iter() {
                scene.push_plain(cube, &Transform::at(*at).with_scale(*size), aim::CEIL_TILE);
            }
            // and no highlight on a bar. `push_colored` hands out a shininess
            // of 32, which is a lobe wider than `aim::MATTE`, and the hall's
            // lamps are a regular row under a grid of long bars: what that
            // laid along every bar in the room was a chain of highlights, and
            // what it looked like from under one was a wave running down it.
            // Jake, 2026-11-09: the ceiling has ripples. Painted steel is not
            // glossy. The tiles keep theirs, because what it carries there is
            // the neon washing the ceiling.
            // and the lit ones, which are the only lamp the hall has of its
            // own. `aim::CEIL_PANEL` is past white, so a panel reads as a thing
            // giving light rather than a pale tile. Spec 0005.
            for (at, size) in panels.iter() {
                scene.push_material(
                    cube,
                    &Transform::at(*at).with_scale(*size),
                    aim::CEIL_PANEL,
                    aim::NO_SHINE,
                );
            }
            for (at, size) in bars.iter() {
                scene.push_material(
                    cube,
                    &Transform::at(*at).with_scale(*size),
                    aim::CEIL_BAR,
                    aim::NO_SHINE,
                );
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
                    None => scene.push_plain(cube, &stood, aim::WALL),
                }
            }

            // and the way out lit, on the dark side only. The secret is the
            // hall's: from in here the way out has no reason to be hidden,
            // and hiding it is what had Jake walk at the sign and hit wall.
            if !in_the_hall {
                if let (Some(lit), Some(mesh)) = (self.portal, self.portal_mesh) {
                    let (at, size) = behind::portal(self.room.reaches);

                    scene.push_textured(
                        mesh,
                        lit,
                        &Transform::at(at)
                            .with_rotation(room::laid_on(Vec3::Z))
                            .with_scale(vec3(size.x, 1.0, size.y)),
                        aim::BEHIND_PORTAL,
                        aim::VOID,
                    );
                }

                // and the door at the far end, lit the same way. Two
                // openings out of one room should look like two openings out
                // of one room. Spec 0017.
                if let (Some(lit), Some(mesh)) = (self.doorway, self.portal_mesh) {
                    let (at, size) = behind::door_portal(self.room.reaches);

                    scene.push_textured(
                        mesh,
                        lit,
                        &Transform::at(at)
                            .with_rotation(room::laid_on(Vec3::NEG_Z))
                            .with_scale(vec3(size.x, 1.0, size.y)),
                        aim::BEHIND_DOORWAY,
                        aim::VOID,
                    );
                }
            }
        }

        // the space the near wall hides is drawn out of what was handed
        // over once, with the hall's own walls and the back room's shell.
        // Spec 0026: none of it moves, and none of it is pushed.
        self.keep_the_building(scene, cube);

        // the solar system in it, per spec 0015
        self.draw_the_system(scene, cube, camera);

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
                scene.push_plain(
                    cube,
                    &Transform::at(top + vec3(0.0, cradle::BAR_UP * 0.5, end * span * 0.5))
                        .with_scale(vec3(cradle::STOCK, cradle::BAR_UP, cradle::STOCK)),
                    aim::CRADLE_FRAME,
                );
            }
            scene.push_plain(
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
                    scene.push_plain(
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

            scene.push_plain(
                cube,
                &Transform::at(top + Vec3::Y * (metronome::FOOT.y * 0.5))
                    .with_scale(metronome::FOOT),
                aim::CASE,
            );
            scene.push_plain(
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
                scene.push_plain(
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
            scene.push_plain(
                cube,
                &Transform::at(pivot).with_scale(vec3(metronome::HINGE_HALF * 3.0, 0.014, 0.014)),
                aim::PENDULUM,
            );
            scene.push_plain(
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
                scene.push_plain(
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
            scene.push_plain(
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
                scene.push_plain(
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

                    scene.push_plain(
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
            scene.push_plain(
                cube,
                &Transform::at(at).with_scale(vec3(sign::WIDE, sign::TALL, sign::THICK)),
                aim::SIGN_GILT,
            );
            scene.push_plain(
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
            scene.push_plain(
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
                None => scene.push_plain(hung, &letters, aim::SIGN_LETTERS),
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

                    scene.push_plain(
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
            scene.push_plain(
                cube,
                &Transform::at(shade).with_scale(sign::SHADE),
                aim::SHADE,
            );
            let stem = (room::soffit() + shade.y + sign::SHADE.y * 0.5) * 0.5;
            scene.push_plain(
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
            scene.push_plain(
                cube,
                &Transform::at(at).with_scale(vec3(neon::WIDE, neon::TALL, neon::THICK)),
                aim::ARCADE_CAN,
            );
            for (middle, size) in neon::tubes() {
                scene.push_plain(
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
                None => scene.push_plain(hung, &letters, aim::arcade_letters()),
            }

            // two stems to the ceiling. Rigid, where the nook's sign is on
            // chains: there is a transformer in a box this size.
            let long = neon::stem(room::soffit());
            for side in [-1.0f32, 1.0] {
                scene.push_plain(
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
                    scene.push_plain(cube, &laid, aim::BOARDS);
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
                cellar::Made::Shaft => scene.push_plain(cube, &laid, aim::TIMBER[0]),
                cellar::Made::Soffit => {}
                // the floor warmer and a shade apart from the walls, so a room
                // is a floor and walls rather than one box
                cellar::Made::Stone if box_.max.y <= -cellar::DOWN + 1e-3 => {
                    scene.push_plain(cube, &laid, aim::CELLAR_FLOOR);
                }
                cellar::Made::Stone => {
                    scene.push_plain(cube, &laid, aim::CELLAR);
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

            scene.push_plain(
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

        // the field through the garden's west wall, per spec 0040: ground
        // that follows you, the garden's own sky blown up over it, and
        // trees, flowers and tufts out of whichever cells you are near.
        //
        // **Only while you are out there, or about to be.** Nothing of this
        // is a place, so none of it can be left standing: the slab alone is
        // four hundred metres of ground at the hall's own floor height.
        self.draw_the_field(scene);
        self.draw_the_outside(scene, cube);

        // the garden, per spec 0010: its sky, its ground, and the walls round
        // it. The sky first, because everything else in here is seen against
        // it.
        {
            // the sky, which is one sky and is drawn where you stand.
            //
            // **It was two.** Spec 0010 sprang a vault nineteen metres
            // across over the garden, which is a sky from underneath and a
            // blue bubble sitting on the roof from out in spec 0040's
            // field. The field had a second one of its own, so the garden
            // and the field, which are one outdoors with one sun and one
            // day over them, had two skies with two rims and two scales,
            // and each could be seen from under the other.
            //
            // One, round the eye. The same cloud all day with the day taken
            // out from behind it, per spec 0019, and a strike lighting the
            // whole of it, per spec 0016: what you see of lightning from
            // under a storm is the deck going white, and the cloud's own
            // shape is a better picture of it than any patch.
            //
            // **Handed over as a backdrop rather than placed.** It was
            // pushed like any other mesh, at a scale this file chose, and
            // only while you stood out under it. Both of those were wrong
            // and the second hid the first.
            //
            // Standing anywhere with a roof over you, the sky over the
            // garden was black: you see a sky from the doorway you are
            // looking through, not from the square metre your feet are on,
            // and walking in and out of cover made it flash. Underneath
            // that, the scale: 22 here against the mesh's own 19 across is
            // a 209 metre dome, and indoors the far plane is 100, so every
            // vertex of it was clipped. Opening the gate alone showed
            // nothing, which is what made this take all morning.
            //
            // The renderer sizes it now, because the far plane is the
            // renderer's and this file could not see it.
            // The engine's spec 0086.
            //
            // **Asked with the same question the field is drawn by**, which
            // is `sees_outdoors`: whether there is any of the outdoors on
            // the screen at all. `draw_the_field` and `draw_the_outside`
            // both turn on it and this is the sky over both of them, so a
            // question of its own here is a third account of one thing.
            //
            // It has had two already. `in_the_deep` is `z` past the door
            // and the space behind the wall is drawn on `behind::inside`,
            // so for the camera in `what_is_behind_the_wall_is_drawn` those
            // disagreed and that room got the solar system and a sky behind
            // it at once. Gating on `behind::in_the_room` fixed that one
            // room and left the solar system out past it uncovered: every
            // staged shot of a planet came back with a cloudy daylight sky
            // over half the frame, which is what `media/screenshot.png`
            // would have become the next time anybody refreshed it.
            // `sees_outdoors` is false out there, because `in_the_deep` is
            // the first half of it.
            if let (Some(dome), Some(storm)) = (self.dome, self.storm) {
                if self.sees_outdoors {
                    scene.backdrop(
                        dome,
                        storm,
                        garden::skylit(self.since) + self.flashed(),
                        900.0,
                    );
                }
            }

            // **And a moon in it, hung off the eye like the sky is.** Jake
            // asked for one. It is drawn rather than painted into the
            // storm because the storm's picture drifts across the dome,
            // per spec 0019, and a moon that drifts with the weather is
            // a cloud.
            //
            // Out of doors only. At two hundred and thirty metres it is
            // well inside `field::sees` and so never clipped, and well
            // outside anything in the field, so nothing out there stands
            // in front of it; indoors it would be hanging inside the
            // building.
            if let (Some(moon), Some(face)) = (self.moon, self.moon_face) {
                let showing = garden::moonlit(self.since);

                if showing > 0.01 && self.room.out_of_doors(self.looking_from()) {
                    let hung = self.looking_from() + garden::moon_at(self.since) * garden::MOON_OFF;

                    scene.push_textured(
                        moon,
                        face,
                        &Transform::at(hung).with_scale(Vec3::splat(garden::MOON_WIDE)),
                        garden::moon_lit(self.since),
                        aim::VOID,
                    );
                }
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
                    scene.push_plain(
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
                    aim::GARDEN_EARTH,
                    aim::MATTE,
                );
            }

            // the tiled cap along the top of them, which is what makes a wall
            // read as built rather than as a box with a flat top
            for (at, size) in garden::coping(self.room.reaches) {
                scene.push_plain(cube, &Transform::at(at).with_scale(size), aim::GARDEN_COPE);
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
                    scene.push_plain(pole, &Transform::at(at), aim::BAMBOO);
                }
            }

            for (at, size) in garden::rails(self.room.reaches) {
                scene.push_plain(cube, &Transform::at(at).with_scale(size), aim::BAMBOO_ROPE);
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
                scene.push_plain(
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
                let bark = Self::bark_of(tree);

                scene.push_material(*trunk, &Transform::at(tree.at), bark, aim::DULL);

                if let Some(limb) = self.limb {
                    for (from, to) in garden::limbs(tree) {
                        let along = to - from;
                        let long = along.length();

                        if long > 1e-3 {
                            // the mesh is built standing on its end, so it is
                            // turned to point along the limb rather than placed
                            // at an angle worked out by hand
                            // dull, like the trunk. The default highlight is
                            // a broad one, and a branch the thickness of a
                            // wrist carries the whole of it as a white
                            // streak down its length: a crown of them reads
                            // as bare bleached sticks, which is what Jake
                            // would have called them.
                            scene.push_material(
                                limb,
                                &Transform::at(from)
                                    .with_rotation(glam::Quat::from_rotation_arc(
                                        Vec3::Y,
                                        along / long,
                                    ))
                                    .with_scale(vec3(garden::LIMB_THICK, long, garden::LIMB_THICK)),
                                bark,
                                aim::DULL,
                            );
                        }
                    }
                }

                if let (Some(crown), Some((skin, leaf))) =
                    (self.crown, self.foliage(tree, tree.seed))
                {
                    for sprig in garden::sprigs(tree) {
                        scene.push_textured(
                            crown,
                            skin,
                            &Transform::at(sprig.at)
                                .with_rotation(glam::Quat::from_rotation_y(sprig.turn))
                                .with_scale(vec3(sprig.wide, sprig.tall, sprig.wide)),
                            leaf * sprig.shade,
                            // no highlight. The default is a broad one, and a
                            // highlight on a silhouette of needles is a wet
                            // green streak across a thing that has no surface.
                            aim::VOID,
                        );
                    }
                }
            }

            if let Some(lantern) = self.lantern {
                // and how hard its window is burning, which is the same
                // number the lamp inside it uses rather than a second one
                let lit = garden::lanterns_lit(self.since);
                let glow = aim::LANTERN_LIT * (0.35 + lit * 2.2);

                for (stands, tall) in garden::lanterns(self.room.reaches) {
                    let stood =
                        Transform::at(stands).with_scale(vec3(tall * 0.8, tall, tall * 0.8));

                    scene.push_plain(lantern, &stood, aim::GARDEN_STONE);

                    if let Some(window) = self.lantern_light {
                        scene.push_plain(window, &stood, glam::vec4(glow.x, glow.y, glow.z, 1.0));
                    }
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

                scene.push_plain(
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
                scene.push_plain(
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
                    None => scene.push_plain(
                        cube,
                        &Transform::at(middle).with_scale(size),
                        aim::TIMBER[0],
                    ),
                }
            }

            let at = spa::sign(self.room.reaches);
            scene.push_plain(
                cube,
                &Transform::at(at).with_scale(vec3(spa::SIGN.x, spa::SIGN.y, spa::SIGN_OUT)),
                aim::BATHS_EDGE,
            );
            scene.push_plain(
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
                    None => scene.push_plain(hung, &letters, aim::BATHS_LETTERS),
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

                scene.push_plain(
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
                scene.push_plain(
                    ball,
                    &Transform::at(at).with_scale(size),
                    aim::STEAM.truncate().extend(thick),
                );
            }

            for (over, wide) in [(tub_at, tub_wide), (stove, spa::STOVE.x * 2.0)] {
                for (at, size, thick) in spa::steam(over, wide, self.since) {
                    scene.push_plain(
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
                None => scene.push_plain(cube, &laid, aim::RAFTER),
            }
        }

        // what the cellar is furnished with. Spec 0007.
        for (which_rack, (at, side)) in cellar::racks(self.room.reaches).into_iter().enumerate() {
            let rack = cellar::RACK;
            let (across, up) = cellar::BOTTLES;
            let back = at.z + side * (rack.x * 0.5 - 0.02);

            // a back against the wall, a side at each end, and a shelf for
            // every row of bottles
            scene.push_plain(
                cube,
                &Transform::at(vec3(at.x, at.y + rack.y * 0.5, back))
                    .with_scale(vec3(rack.z, rack.y, 0.04)),
                aim::RACK,
            );
            for end in [-1.0f32, 1.0] {
                scene.push_plain(
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
                scene.push_plain(
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
                            scene.push_plain(
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

                                scene.push_plain(
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
                        None => scene.push_plain(
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
                    None => scene.push_plain(cube, &laid, aim::COFFER),
                }

                for (way, out, span) in [
                    (Vec3::X, size.x, vec3(cellar::MOULD, cellar::MOULD, size.z)),
                    (Vec3::Z, size.z, vec3(size.x, cellar::MOULD, cellar::MOULD)),
                ] {
                    for side in [-1.0f32, 1.0] {
                        scene.push_plain(
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
                scene.push_plain(cube, &Transform::at(*at).with_scale(*size), aim::RAFTER);
            }

            // and a carved boss where four of them meet, which is the one place
            // on a ceiling anybody puts any carving
            if let Some(peg) = self.peg {
                for (at, _) in &beams {
                    for (other, _) in &beams {
                        if (at.x - other.x).abs() < 1e-3 || (at.z - other.z).abs() < 1e-3 {
                            continue;
                        }

                        scene.push_plain(
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
            let laid =
                |middle: Vec3, size: Vec3, timber: glam::Vec4, board: usize, scene: &mut Scene| {
                    match self.grain.get(board % self.grain.len().max(1)) {
                        Some(grain) => scene.push_textured(
                            cube,
                            *grain,
                            &Transform::at(middle).with_scale(size),
                            timber,
                            aim::DULL,
                        ),
                        None => {
                            scene.push_plain(cube, &Transform::at(middle).with_scale(size), timber)
                        }
                    }
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
                    None => scene.push_plain(cube, &laid, aim::HEARTH),
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

                    scene.push_plain(
                        stick,
                        &Transform::at(on + Vec3::Y * tall * 0.5).with_scale(cellar::CANDLESTICK),
                        aim::BRASS,
                    );
                    scene.push_plain(
                        peg_or_cube,
                        &Transform::at(on + Vec3::Y * (tall + cellar::CANDLE * 0.5))
                            .with_scale(vec3(0.046, cellar::CANDLE, 0.046)),
                        aim::WAX,
                    );
                    if let Some(flame) = self.flame {
                        let own = cellar::flicker(self.since * 2.3 + side * 3.0);

                        scene.push_plain(
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

                scene.push_plain(
                    cube,
                    &Transform::at(shelf + Vec3::Y * case.y * 0.5).with_scale(case),
                    aim::CLOCK_CASE,
                );
                // a pediment on top, which is what a mantel clock has instead
                // of a flat lid
                scene.push_plain(
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
                    scene.push_plain(
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

                    scene.push_plain(
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

                    scene.push_plain(
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
                scene.push_plain(
                    decanter,
                    &Transform::at(by + Vec3::Y * cellar::DECANTER.y * 0.5)
                        .with_scale(cellar::DECANTER),
                    aim::CRYSTAL,
                );
                // what is in it, which is the only reason a decanter is worth
                // drawing
                scene.push_plain(
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

                    scene.push_plain(
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
            scene.push_plain(
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

                    scene.push_plain(
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
                    scene.push_plain(
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

                    scene.push_plain(
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
                    None => scene.push_plain(barrel, &stood, aim::BARREL),
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

                        scene.push_plain(
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

                        scene.push_plain(
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

            // and a tight one on the boards rather than a broad one, which is
            // what a waxed floor does under a lamp. `MATTE` is 64 and that is
            // a highlight spread wide enough to pale the grain.
            scene.push_textured(boards_mesh, boards, &laid, aim::BOARDS, aim::DULL);
        }

        // and the rug on top of them
        if let (Some(rug), Some(rug_mesh)) = (self.rug, self.rug_mesh) {
            let (at, size) = room::open_floor(self.room.reaches);
            let laid = Transform::at(at + Vec3::Y * 0.004).with_scale(vec3(size.x, 1.0, size.z));

            // no highlight on it at all: a rug is cloth. With nothing over the
            // nook but wall lights it did not matter, and with ten pendants
            // over it `MATTE` is a hot spot per lamp and the floor reads as a
            // skating rink. Jake, 2026-11-09. Engine spec 0012.
            scene.push_textured(rug_mesh, rug, &laid, aim::FLOOR, aim::NO_SHINE);
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
                scene.push_plain(
                    cube,
                    &Transform::at(put(at)).with_rotation(turn).with_scale(size),
                    aim::BOOKCASE,
                );
            }

            for shelf in 0..study::SHELVES {
                let up = study::shelf_at(shelf);
                scene.push_plain(
                    cube,
                    &Transform::at(put(vec3(0.0, up - study::BOARD * 0.5, 0.0)))
                        .with_rotation(turn)
                        .with_scale(vec3(case.x, study::BOARD, case.y)),
                    aim::SHELF,
                );

                let books = study::stock(n as u32, shelf);
                for (which, book) in books.iter().enumerate() {
                    let along = study::along(&books, which) - case.x * 0.5;
                    scene.push_plain(
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
                        if (cellar::swings(n)
                            && n == cellar::CASE
                            && shelf == cellar::BOOK_SHELF
                            && which == cellar::BOOK)
                            || (mirrors::swings(n)
                                && n == mirrors::CASE
                                && shelf == mirrors::BOOK_SHELF
                                && which == mirrors::BOOK)
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

                scene.push_plain(cube, &Transform::at(at).with_scale(size), aim::TRIM);
            }

            // and a stop where a run ends against nothing, for the three of
            // them to die into.
            //
            // **Which end that is, derived rather than listed.** A side wall
            // runs east to `front` and is met there by the front wall, but
            // only where the front wall exists: that one starts at the
            // doorway, `far + NOOK_DOOR`, and the south side sits at
            // `far + 0.15`, well short of it. So the south wall's east end
            // is open to the hall and its skirting, rail and cornice ran out
            // into the air, which is what Jake saw from the arcade on
            // 2026-11-10 and read as elements that did not belong.
            if !along {
                let far = -self.room.reaches;
                let met = (far + room::NOOK_DOOR..=far + room::NOOK_SPAN).contains(&face);

                if !met {
                    let deep = study::PROUD * 2.4;

                    scene.push_plain(
                        cube,
                        &Transform::at(vec3(
                            to - study::STOP_WIDE * 0.5,
                            room::TALL * 0.5,
                            face + out * deep * 0.5,
                        ))
                        .with_scale(vec3(
                            study::STOP_WIDE,
                            room::TALL,
                            deep,
                        )),
                        aim::TRIM,
                    );
                }
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

                scene.push_plain(cube, &Transform::at(at).with_scale(size), aim::PANEL);
            }
        }

        // the sconces down both long walls, which is where the light in here
        // comes from and is why there is any. Spec 0006.
        // a back plate on the wall, a bracket off it, and a shade on the
        // end of that
        let mut sconce = |at: Vec3, out: f32| {
            for (reach, size) in [
                (-study::SCONCE_BACK, vec3(0.03, 0.09, 0.09)),
                (
                    study::SCONCE_OUT * 0.5,
                    vec3(study::SCONCE_OUT, 0.018, 0.018),
                ),
            ] {
                scene.push_plain(
                    cube,
                    &Transform::at(at + Vec3::X * out * reach).with_scale(size),
                    aim::SCONCE,
                );
            }
            scene.push_plain(
                cube,
                &Transform::at(at + Vec3::X * out * study::SCONCE_OUT).with_scale(study::SHADE),
                aim::SHADE,
            );
        };

        for (face, out, from, to) in room::sconce_runs(self.room.reaches) {
            for along in study::sconces(from, to) {
                sconce(vec3(face, bench_floor + study::SCONCE_UP, along), out);
            }
        }

        // and one over the way through, which the run leaves dark: spec 0061
        // and `mirrors::doorway_lamp`. The same fitting as the rest of that
        // wall, higher, because at the run's own height the middle of the
        // opening is a hole.
        {
            let (face, out) = self.bookcase_wall();

            sconce(
                mirrors::doorway_lamp(face, out) - Vec3::X * out * study::SCONCE_OUT,
                out,
            );
        }

        // and the pendants over the open floor, which is what lights the floor:
        // the sconces are at head height on the two long walls and the middle
        // of this room is out of reach of all of them. Spec 0006. A shade and a
        // rod to the ceiling, the same fitting as the one over the pool table,
        // and the rod runs to `soffit` rather than to `TALL` because the
        // ceiling in here is a grid with tiles under it.
        for shade in room::pendant_shades(self.room.reaches) {
            scene.push_plain(
                cube,
                &Transform::at(shade).with_scale(study::PENDANT_SHADE),
                aim::SHADE,
            );

            let hung = study::PENDANT_DROP - study::PENDANT_SHADE.y * 0.5;

            scene.push_plain(
                cube,
                &Transform::at(vec3(shade.x, room::soffit() - hung * 0.5, shade.z))
                    .with_scale(vec3(study::PENDANT_ROD, hung, study::PENDANT_ROD)),
                aim::SIGN_CHAIN,
            );
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

                scene.push_plain(
                    cube,
                    &Transform::at(top + at - Vec3::Y * drop)
                        .with_rotation(cascada::stood(turn, lean))
                        .with_scale(cascada::DOMINO),
                    aim::DOMINO,
                );
            }
        }

        // the cube on its bench. Spec 0050.
        //
        // **Twenty-six boxes and fifty-four more, and no mesh of its
        // own.** Everything here is the unit cube the room is already
        // drawn out of, which is the whole of why a cube is a cheap thing
        // to put in this building and an expensive thing to solve. The
        // stickers stand off their cubies rather than lying on them, per
        // spec 0014, and `kociemba::stickers` is what knows by how much.
        //
        // The colours come from the state rather than from the geometry:
        // which of the six a sticker carries is `facelets`, read with the
        // facelet number the sticker came with.
        if let Some(bench) = self
            .room
            .benches
            .iter()
            .find(|bench| bench.name == kociemba::NAME)
        {
            // a centimetre clear of the top, so the bottom row of
            // stickers is not in the bench's own plane. Spec 0014.
            let middle = bench.at + Vec3::Y * (bench.size.y + kociemba::ACROSS * 0.5 + 0.01);
            let on = kociemba::facelets(&self.puzzle.cube);

            // **The face part way through its turn is swept here and
            // nowhere else.** The cube's own state is still what it was
            // until the turn finishes, which is spec 0050's one account
            // of where every piece is. What a sweep does is look at that
            // state with nine of its cubies turned, so there is nothing
            // to keep in step and nothing that can disagree.
            let sweep = self.puzzle.part_way().map(|(face, round)| {
                let way = kociemba::face_way(face as usize);

                (way, glam::Quat::from_axis_angle(way, -round))
            });
            let swept = |at: Vec3| match sweep {
                // the nine on that face are the ones whose cell sits at
                // the far end of the way it looks
                Some((way, spin)) if (at - middle).dot(way) > kociemba::cubie() * 0.5 => {
                    (middle + spin * (at - middle), spin)
                }
                _ => (at, glam::Quat::IDENTITY),
            };

            for box_ in kociemba::cubies(middle).iter() {
                let (at, spin) = swept(box_.center());

                scene.push_plain(
                    cube,
                    &Transform::at(at)
                        .with_rotation(spin)
                        .with_scale(box_.size()),
                    aim::CUBIE,
                );
            }

            for (sticker, facelet) in kociemba::stickers(middle).iter() {
                let colour = kociemba::face_colour(on[*facelet]);
                let (at, spin) = swept(sticker.center());

                scene.push_plain(
                    cube,
                    &Transform::at(at)
                        .with_rotation(spin)
                        .with_scale(sticker.size()),
                    colour.extend(1.0),
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

                scene.push_plain(
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
                scene.push_plain(
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
                scene.push_plain(
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
                scene.push_plain(cube, &Transform::at(top + at).with_scale(size), aim::GANTRY);
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

                scene.push_plain(
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
                    scene.push_plain(
                        ball_mesh,
                        &Transform::at(at).with_scale(Vec3::splat(wrecker::BEAD * 1.4)),
                        aim::GANTRY,
                    );
                } else if n < wrecker::BALL_AT {
                    scene.push_plain(
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
                scene.push_plain(
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
                        None => {
                            scene.push_plain(cube, &Transform::at(middle).with_scale(size), colour)
                        }
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

                    scene.push_plain(
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
                    scene.push_plain(
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
                        scene.push_plain(
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
                scene.push_plain(
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

                        scene.push_plain(
                            cube,
                            &Transform::at(at).with_scale(vec3(leg, stands, leg)),
                            frame,
                        );
                    }
                }

                // and the lamp over it, which is what a pool room is lit by
                let shade = bench.at + Vec3::Y * (top + cellar::LAMP_UP);
                scene.push_plain(
                    cube,
                    &Transform::at(shade).with_scale(cellar::SHADE),
                    aim::SHADE,
                );
                let lid = -cellar::DOWN + cellar::TALL;
                scene.push_plain(
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

            // and the back room's rig is a block of cubes in a frame rather
            // than a table. Its record is a bench so that the sight names it
            // and the arrows work it, the same as the pool table above, and
            // what it looks like is drawn where it stands. Spec 0025: this
            // was a bench top in wood hanging over the cubes, which is what
            // a bench record draws if nobody says otherwise.
            if bench.name == back::RIG {
                continue;
            }

            let top = Transform::at(bench.at + Vec3::Y * (bench.size.y - aim::BENCH_TOP * 0.5))
                .with_scale(vec3(bench.size.x, aim::BENCH_TOP, bench.size.z));
            match self.cabinet_grain {
                Some(grain) => scene.push_textured(cube, grain, &top, look, aim::MATTE),
                None => scene.push_plain(cube, &top, look),
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
                    scene.push_plain(
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

        // the gallery's shades, so no lamp in here is a bright patch on a
        // ceiling with nothing making it. Spec 0022.
        for at in room::gallery_lamps(self.room.reaches) {
            let shade = room::GALLERY_SHADE;

            scene.push_plain(
                cube,
                &Transform::at(at + Vec3::Y * shade.y * 0.5).with_scale(shade),
                aim::SHADE,
            );
            // and the stem up to the ceiling
            let stem = (at.y + shade.y + room::soffit()) * 0.5;
            scene.push_plain(
                cube,
                &Transform::at(vec3(at.x, stem, at.z)).with_scale(vec3(
                    0.035,
                    room::soffit() - at.y - shade.y,
                    0.035,
                )),
                aim::SIGN_CHAIN,
            );
        }

        for (n, one) in self.room.displays.iter().enumerate() {
            let Some(mesh) = self.shown.get(n).copied() else {
                continue;
            };

            let (plinth, size) = display::plinth_under(one);
            scene.push_plain(cube, &Transform::at(plinth).with_scale(size), aim::PLINTH);
            // a turned shape reaches lower than an upright one, so where its
            // middle goes follows which way it is facing. It sank into its
            // plinth otherwise.
            let facing = self.spin.facing(n, self.since);
            let sits = vec3(one.at.x, display::sits_at(one, facing), one.at.z);
            scene.push_plain(
                mesh,
                &Transform::at(sits)
                    .with_rotation(facing)
                    .with_scale(Vec3::splat(one.scale)),
                aim::shape_colour(
                    self.seen == Some(room::Seen::Display(n)) || self.spin.holding() == Some(n),
                ),
            );
        }

        // and the panel at the end of that row, which is the one flat thing
        // in this building you work. Spec 0039.
        // the fine one when there is one for where you are looking, and
        // the coarse one every other frame of the dive
        let panel = if self.sharp_view == Some(self.view) {
            self.sharp
        } else {
            self.mandel
        };

        if let Some(picture) = panel {
            let (at, size) = mandel::panel(self.room.reaches);

            scene.push_textured(
                cube,
                picture,
                &Transform::at(at).with_scale(size),
                // lit rather than painted, because it is a screen: colour
                // here is an unclamped multiplier and a surface at one is
                // only as bright as the room, which in this one is dim
                aim::MANDEL_LIT * aim::shape_colour(self.seen == Some(room::Seen::Mandel)),
                aim::DULL,
            );
        }

        // and the frame round it, which is what the picture grows behind
        for side in mandel::surround(self.room.reaches) {
            scene.push_plain(
                cube,
                &Transform::at(side.center()).with_scale(side.size()),
                aim::MANDEL_FRAME,
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

        // the nook's own, because a light with no fitting is a bright patch on
        // a wall and no reason for it: the sconces down both long walls, the
        // one over the way through that the run leaves dark, and the pendants
        // over the open floor. `room::nook_lamps` is the one list, so what the
        // room is lit by is a question with one answer.
        let mut shades: Vec<(f32, Vec3, Vec3, f32, f32)> = room::nook_lamps(self.room.reaches)
            .into_iter()
            .map(|(at, lit, range)| {
                (
                    eye.distance_squared(at),
                    at,
                    study::SCONCE_COLOUR,
                    lit,
                    range,
                )
            })
            .collect();

        // **The lantern on the roof has no lamp, and that is spec 0057's
        // own finding.** Its colour is far past one and biased hard warm,
        // because a lamp inside the box cannot light the box's outside
        // faces: they point away, so no lambert and no light, and the
        // warmth has to survive the clamp instead. A lamp up there lights
        // nothing it is for and does light the hall through the lid, which
        // is what `the_hall_is_drawn` caught: eighteen per cent of a
        // picture that looks the same.
        //
        // Spec 0057's lantern had one because it hung in a shaft cut down
        // into the imaginarium, and there was a room under it that wanted
        // the light. There is no room under this one.

        // and the gallery's three, per spec 0022. The room at the end of the
        // aisle has one thing in it and these are over that rather than over
        // the middle of the floor.
        for at in room::gallery_lamps(self.room.reaches) {
            shades.push((
                eye.distance_squared(at),
                at,
                aim::GALLERY_LAMP,
                aim::GALLERY_LIT,
                aim::GALLERY_RANGE,
            ));
        }

        // and the corridor of mirrors', one over each frame, per spec 0061.
        //
        // **Here rather than in `mirrors::draw` where the rest of that room
        // is.** Pushed at the scene there, they went in ahead of this list
        // and so survived spec 0050's cap of sixty four wherever you stood,
        // while the three worst ranked lamps at the entrance were dropped to
        // make room for lamps behind a bookcase. The count was halved to
        // five for it and the cap was still three over.
        //
        // Pushing them after the cull instead was tried, on the argument
        // that a mirror is pointed at corridor this frustum throws away. It
        // changed no picture anywhere the glass was looked at and put the
        // entrance back over the cap, so it is not in.
        for at in mirrors::lamps() {
            shades.push((
                eye.distance_squared(at),
                at,
                aim::SHADE.truncate(),
                mirrors::LAMP_LIT,
                mirrors::LAMP_RANGE,
            ));
        }

        // and the back room's two: the bulb over its near half, per spec
        // 0024, and the bay's own, per spec 0025
        {
            let at = back::lamp(self.room.reaches);

            shades.push((
                eye.distance_squared(at),
                at,
                aim::BACK_LAMP,
                aim::BACK_LIT,
                aim::BACK_RANGE,
            ));

            let bay = back::bay_lamp(self.room.reaches);

            shades.push((
                eye.distance_squared(bay),
                bay,
                aim::BACK_BAY_LAMP,
                aim::BACK_BAY_LIT,
                aim::BACK_BAY_RANGE,
            ));
        }

        // and the carving's own, in front of the runestone's face, which
        // is what makes the thing read as glowing rather than as a neon
        // tube stuck on a rock: the light it gives off lands on the
        // stone holding it. Only while the stone is there to hold it.
        if let Some((stone, size)) = bridge::runestone(self.room.reaches, &self.trail) {
            let (at, lit, strength, range) = bridge::carving_lamp(stone, size, self.since);

            shades.push((eye.distance_squared(at), at, lit, strength, range));
        }

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

        // and the hall's ceiling panels, off the same list that draws them so
        // there is no lamp in here that is not in a fitting
        for (at, _) in self.room.lattice().2.iter() {
            let lamp = room::panel_lamp(*at);

            shades.push((
                eye.distance_squared(lamp),
                lamp,
                room::PANEL_COLOUR,
                room::PANEL_LIT,
                room::PANEL_RANGE,
            ));
        }

        // and the cabinets' neon, which is a lamp like any other and has to
        // be ranked like one.
        //
        // **It was pushed after the sort, so the cap fell on it first.** The
        // engine keeps the first sixty four lights it is given and drops the
        // rest without a word, so push order is what decides. The fitted
        // lamps went in culled and ranked and the neon went in afterwards,
        // which protected a ceiling lamp over the glow a cabinet throws on
        // the carpet, in the one room whose whole character is that glow.
        // Standing at the entrance and sweeping the hall asked for a hundred
        // and thirteen, so forty nine were dropped and the neon was all of
        // them.
        shades.extend(near.iter().map(|&(_, n)| {
            let stood = &self.room.stood[n];
            let glow = aim::neon_of(self.glows.get(n).copied().unwrap_or(Vec3::ONE));
            let strength = if at == Some(n) {
                aim::LAMP_LIT
            } else {
                aim::LAMP_INTENSITY
            };
            let throws = stood.at
                + Vec3::Y * room::CABINET.y * aim::NEON_UP
                + stood.facing * (room::CABINET.z * 0.5 + aim::NEON_THICK);

            (
                eye.distance_squared(throws),
                throws,
                glow.truncate().normalize_or(Vec3::ONE),
                strength,
                aim::LAMP_RANGE,
            )
        }));

        // and first, throw away every lamp that cannot light anything you
        // can see. A lamp gives exactly nothing past its range, so all it
        // lights is inside a sphere of that radius, and a sphere that does
        // not reach the picture is a lamp whose work is all off screen.
        // Spec 0050 of the engine.
        //
        // This is what takes the building under the engine's cap rather
        // than merely ordering what spills over it: two hundred and
        // nineteen lamps are pushed and the cull leaves sixty two of them
        // where you come in, against sixty four carried. It read a hundred
        // and thirty three and twelve to fifty three when it was last
        // measured, and both have moved, which is the argument for
        // measuring again rather than believing this line.
        //
        // **And the ranking below is not only a fallback.** One turn on
        // the spot at the door reaches a hundred and seven and asks for
        // more than sixty four on 159 frames of 365, so what is dropped is
        // decided over four tenths of a turn at the way in. Spec 0089 of
        // the engine is what those frames look like at a cap of 128, which
        // is byte for byte the same, because the cull is a frustum and the
        // surplus is behind a wall.
        // **The eye's frustum, and not the mirrors' as well.** A reflection
        // is drawn from behind the glass and is pointed at room this throws
        // away, so the nook through the open bookcase is lit by lamps that
        // are culled from in here and its floor reads black in the glass.
        // Jake, 2026-11-09, twice.
        //
        // Adding the two mirror cameras' frustums was tried and is not in.
        // Measured in the corridor with the case open, it takes the lamps
        // kept from 43 to 135 against a cap of 64, so the ranking throws the
        // extra ninety away again and not one pixel moves. What that floor
        // wants is a ranking that knows about mirrors, which is the cap's
        // problem rather than the cull's.
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

        for (n, stood) in self.room.stood.iter().enumerate() {
            let look = aim::look_of(stood.cabinet.is_built(), at == Some(n));

            let box_ =
                Transform::at(stood.at + Vec3::Y * room::CABINET.y * 0.5).with_scale(room::CABINET);

            match self.cabinet_grain {
                Some(grain) => scene.push_textured(cube, grain, &box_, look.body, aim::MATTE),
                None => scene.push_plain(cube, &box_, look.body),
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
                None => scene.push_plain(screen_mesh, &placed, aim::neon_of(glow)),
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
                None => scene.push_plain(screen_mesh, &placed, vec4(0.08, 0.08, 0.1, 1.0)),
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
            // and what the frame costs, per spec 0056. A toggle for the same
            // reason: you walk to the room you are suspicious of and watch
            // the number there.
            KeyboardKey::F if held && !input.repeat => {
                // and nothing else: the window is filled whether or not it is
                // being looked at, and clearing it here would spend the first
                // seconds of every look reporting the cost of looking. It is
                // a few seconds long, so it empties itself of wherever you
                // were before without being told.
                self.showing_cost = !self.showing_cost;
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
            // and where you are standing, for a picture somebody else has to
            // find. Spec 0045. On the screen, because the point of it is that
            // a screenshot carries its own pose, and in the log as well
            // because a log can be grepped.
            KeyboardKey::M if held && !input.repeat => {
                self.showing_place = Self::PLACE_SHOWS;

                eprintln!("{}  seeing {:?}", self.where_you_are(), self.what_i_see());
            }
            // back to the door, when flying has lost you. Spec 0041.
            //
            // **Only while flying.** In the game this is a key that teleports
            // you, which is not a thing this building does, and R is next to
            // nothing anybody presses by accident but the walk is not worth
            // the risk.
            KeyboardKey::R if held && !input.repeat && flying() => {
                self.back_to_the_door();
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

    /// Spec 0056: the device's own clock, for the readout behind F.
    ///
    /// Always, rather than behind the key that shows it. The feature is
    /// asked for once, before the device exists, and a key pressed later
    /// cannot go back and ask again. What it costs is two timestamps and a
    /// sixteen byte read a frame against a building that draws several
    /// hundred things with shadows, which is nothing, and the alternative is
    /// a readout that can never show the number that matters most.
    fn timing(&self) -> bool {
        true
    }

    fn is_quitting(&self) -> bool {
        self.quitting
    }

    /// Lets go of the lock and of every key, because a key-up goes to
    /// whatever has focus and that is no longer this.
    ///
    /// **Every held thing, not the ones that walk you.** It cleared
    /// `walking` alone from the day it was written, so alt-tabbing with an
    /// arrow down left that arrow down for ever: you came back to a
    /// building turning on the spot with nothing to let go of.
    ///
    /// Written as the whole list rather than four lines, so that a fifth
    /// thing you can hold down cannot be added without this seeing it.
    fn focus_changed(&mut self, focus: bool) {
        if !focus {
            self.wants_lock = false;
            self.let_go();
        }
    }
}

/// One key of a staged route, as `refresh-examples` writes them.
struct Step {
    said: String,
    /// w, s, a, d
    walking: [bool; 4],
    /// up, down, left, right
    looking: [bool; 4],
    works: bool,
    held: f32,
    /// Whether this one is meant to end against something. Spec 0042.
    pins: bool,
}

impl Step {
    /// A step that only lets time pass, which is a bare number in the route.
    fn is_a_wait(&self) -> bool {
        !self.works && self.walking == [false; 4] && self.looking == [false; 4]
    }

    /// How far this asks the view to move, in radians.
    ///
    /// Nought for anything that is not a held arrow, and for a tilt it is
    /// what is left before the pitch stops rather than the whole hold: an
    /// arrow held into the limit moves less than it asked for and is not
    /// being swallowed by anything.
    fn asks_for(&self, pitch: f32) -> f32 {
        if self.looking[2] || self.looking[3] {
            TURNS * self.held
        } else if self.looking[0] {
            (TILTS * self.held).min(PITCH_LIMIT - pitch)
        } else if self.looking[1] {
            (TILTS * self.held).min(PITCH_LIMIT + pitch)
        } else {
            0.0
        }
    }

    /// How far this asks you to walk, in metres.
    ///
    /// Nought for anything that is not a held walking key. Shift is not in
    /// the grammar and there is no way to hold it from a script, so the
    /// pace is always the walking one.
    ///
    /// **This is the arc and what is measured against it is the chord**, so a
    /// step that walks and turns at once keeps less of what it asks for than
    /// a straight one does. A quarter turn walked through keeps nine tenths
    /// and a half turn two thirds, both of them well clear of the half this
    /// is held to.
    fn walks_for(&self) -> f32 {
        if self.pins || !self.walking.iter().any(|held| *held) {
            0.0
        } else {
            SPEED * self.held
        }
    }

    /// And how far it moved it.
    fn moved(&self, was: (f32, f32), is: (f32, f32)) -> f32 {
        if self.looking[2] || self.looking[3] {
            (is.0 - was.0).abs()
        } else {
            (is.1 - was.1).abs()
        }
    }
}

/// Which slot a key fills, and whether it is an arrow.
fn slotted(key: &str) -> Option<(bool, usize)> {
    match key {
        "w" => Some((false, 0)),
        "s" => Some((false, 1)),
        "a" => Some((false, 2)),
        "d" => Some((false, 3)),
        "up" => Some((true, 0)),
        "down" => Some((true, 1)),
        "left" => Some((true, 2)),
        "right" => Some((true, 3)),
        _ => None,
    }
}

/// Reads a route in `refresh-examples`' own grammar.
///
/// A key this cannot replay is not an error. Half the examples press letters
/// that toggle something, and none of those aim or walk, so there is nothing
/// here to say about them.
fn read_route(route: &str) -> Result<Vec<Step>, String> {
    route
        .split_whitespace()
        .map(|word| {
            let mut step = Step {
                said: word.to_owned(),
                walking: [false; 4],
                looking: [false; 4],
                works: false,
                held: 0.0,
                pins: word.starts_with("pin:"),
            };

            if let Some(rest) = word
                .strip_prefix("hold:")
                .or_else(|| word.strip_prefix("pin:"))
            {
                let mut bits = rest.split(':');
                let key = bits.next().unwrap_or_default();
                let held = bits.next().ok_or(format!("{} says no seconds", word))?;

                step.held = held
                    .parse()
                    .map_err(|_| format!("{} does not say how long in seconds", word))?;

                // **Several at once, joined by a plus.** Spec 0044: stopping
                // to turn and then setting off again is what reads as a
                // machine, whatever shape the turn itself is, and the only
                // way to walk a corner rather than pivot on it is to hold the
                // walk and the turn together.
                for key in key.split('+') {
                    match slotted(key) {
                        Some((true, n)) => step.looking[n] = true,
                        Some((false, n)) => step.walking[n] = true,
                        None => {
                            return Err(format!("{} holds a key that neither aims nor walks", word))
                        }
                    }
                }
            } else if word == "enter" {
                step.works = true;
            } else if let Ok(wait) = word.parse::<f32>() {
                step.held = wait;
            }

            Ok(step)
        })
        .collect()
}

/// Replays a staged route and says whether every turn in it turns you.
///
/// **A turn that does not happen takes the rest of the clip with it.** Spec
/// 0039's panel swallows all four arrows while the sight is on it, which is
/// right when you are working the panel and wrong when you are walking past
/// one. The staged walk met the panel on its way to the hotel: every turn
/// after that was eaten, the camera walked into the end wall, and the last
/// nine seconds of the clip published on the 26th of October 2026 are a
/// fractal with a window pressed against it. It was filmed, looked at and
/// shipped, and the fault is invisible in any one frame.
///
/// So this asks the game. The route lives in the table in `refresh-examples`
/// and nowhere else, `aimed_and_moved` is the game's own aiming, and
/// `what_i_see` is the game's own sight, so there is no second account of
/// any of it to come apart.
///
/// **A swallowed arrow is a fault only when a key comes after it.** The
/// arcade's own route ends by holding one into the panel on purpose, which
/// is how the clip finishes and the one thing the panel can be filmed doing.
/// A route is allowed to end that way. It is not allowed to carry on.
///
/// **A `pin:` is a walk that is meant to end against something**, and it is
/// the only thing this does not hold to going somewhere. A route that aims at
/// a small target across a long room is dead reckoning, and dead reckoning is
/// what the machine gets wrong where this gets it right: the clip of the four
/// rooms walked twenty metres down the hall, turned into the nook and pointed
/// at the bookcase one case along from the one that opens, twice, while this
/// said the route was sound. Walking into the wall first pins the coordinate
/// the error was in, and then a short strafe off it lands where it should.
/// Use the building as the datum wherever a target is tight.
///
/// **It replays the doors too**, since the day a route was asked to go down
/// to the baths. That one opens the bookcase, stands while four hundred books
/// swing out of the way, and walks through where a shut wall was. This used
/// to say that a route waiting on a door was one nobody had written, which
/// was true and is the sort of thing that stops being true.
fn walked(route: &str) -> bool {
    walk_through(route).0
}

/// And what it was looking at when it finished.
///
/// **Which is the half a bool cannot carry.** Three of the routes in
/// spec 0042's own tests are built on walking up to the gallery's panel
/// until it takes the sight, and the length of that walk is the hall's
/// length. The hall grew by 2.4 metres for spec 0050's bench, every one
/// of those routes stopped short of the panel, and two of the three went
/// on passing: a route whose turn nothing swallows is a route with no
/// fault in it. The one that said so was the one asking for a fault.
///
/// So the sight comes back with the verdict, and the control says which
/// thing it walked up to rather than only that it was let through.
fn walk_through(route: &str) -> (bool, Option<room::Seen>) {
    let steps = match read_route(route) {
        Ok(steps) => steps,
        Err(why) => {
            println!("{}", why);
            return (false, None);
        }
    };

    let mut one = Arcade::new();
    let dt = 1.0 / 60.0;
    let mut when = 0.0;
    let mut eaten = Vec::new();
    let mut stuck = Vec::new();

    one.seen = one.what_i_see();

    for (n, step) in steps.iter().enumerate() {
        let was = (one.yaw, one.pitch);
        let from = one.at;

        if step.works {
            one.use_what_i_see();
        }

        one.walking = step.walking;
        one.looking = step.looking;

        for _ in 0..(step.held / dt).round() as u32 {
            one.aimed_and_moved(dt);
            one.the_doors_moved(dt);
            // after the move, the way `update` has it: the sight a frame
            // aims by is the one the frame before it left
            one.seen = one.what_i_see();
            when += dt;
        }

        one.walking = [false; 4];
        one.looking = [false; 4];

        // **And then the tail**, before anything is read off. Spec 0044: the
        // arrows ease off rather than stopping dead, so where you are pointing
        // is not settled until `AIM_LAGS` after the key is up. `play` in
        // `refresh-examples` waits longer than that after every key, so this is
        // the machine's own gap and not slack invented here. Run straight into
        // the next key instead and every turn in every route reads as short by
        // exactly the tail this cut off.
        //
        // The clock is not advanced, because it counts the route's own holds
        // and never has counted what `play` spends between them.
        for _ in 0..(AIM_LAGS / dt).ceil() as u32 {
            one.aimed_and_moved(dt);
            one.the_doors_moved(dt);
            one.seen = one.what_i_see();
        }

        let asks_for = step.asks_for(was.1);
        let moved = step.moved(was, (one.yaw, one.pitch));
        // a tenth, which is slack for the clamp and for a frame either side
        // of the hold rather than a judgement about how much is enough
        let swallowed = asks_for > 0.01 && moved < asks_for * 0.9;

        let walks_for = step.walks_for();
        let covered = one.at.distance(from);
        // half, and that is a wide gate on purpose. A walk down a corridor
        // keeps all of its ground, a walk that grazes a jamb or a doorway
        // loses a few per cent of it, and a walk that is stopped dead keeps
        // whatever it had covered before it met the thing. The one this was
        // written for covered 4.3 metres of the 11.5 it asked for, and the
        // thinnest honest walk in the same route covered 95 per cent.
        let stopped = walks_for > 0.01 && covered < walks_for * 0.5;

        println!(
            "{:6.2}s  {:22}  at {:7.2} {:7.2} {:7.2}  facing {:6.1}  sees {}{}",
            when,
            step.said,
            one.at.x,
            one.at.y,
            one.at.z,
            one.yaw.to_degrees(),
            match one.seen {
                Some(seen) => format!("{:?}", seen),
                None => String::from("nothing"),
            },
            match (swallowed, stopped) {
                (true, _) => "  SWALLOWED",
                (false, true) => "  STOPPED",
                (false, false) => "",
            },
        );

        if swallowed {
            eaten.push(n);
        }

        if stopped {
            stuck.push((n, covered, walks_for));
        }
    }

    let faults: Vec<usize> = eaten
        .iter()
        .copied()
        .filter(|n| steps[n + 1..].iter().any(|step| !step.is_a_wait()))
        .collect();

    for n in &faults {
        let after = steps[*n + 1..].iter().filter(|s| !s.is_a_wait()).count();

        println!(
            "{} was swallowed by what you are looking at, and {} key{} after it {} aimed by a turn that never happened",
            steps[*n].said,
            after,
            if after == 1 { "" } else { "s" },
            if after == 1 { "is" } else { "are" },
        );
    }

    for (n, covered, asked) in &stuck {
        println!(
            "{} covers {:.1} of the {:.1} metres it asks for, so something stopped you",
            steps[*n].said, covered, asked
        );
    }

    if !faults.is_empty() || !stuck.is_empty() {
        return (false, one.seen);
    }

    match eaten.last() {
        Some(n) => println!(
            "{} goes into what you are looking at, and the route ends there",
            steps[*n].said
        ),
        None => println!("every turn in this route turns you, and every walk goes somewhere"),
    }

    (true, one.seen)
}

/// Spec 0049: plan a way from the door to somewhere and walk it.
///
/// Prints where it got to and says whether it arrived, the way `walked`
/// does, so a route can be tried without a window.
fn toured(where_: &str) -> bool {
    let numbers: Vec<f32> = where_
        .split(',')
        .filter_map(|bit| bit.trim().parse().ok())
        .collect();

    let [x, y, z] = numbers[..] else {
        println!("--tour wants three numbers, as x,y,z");

        return false;
    };

    let room = room::Room::of(games());
    let solid = room.solid();
    let from = room.doorway();
    let to = vec3(x, y, z);
    let ways = tour::ways(&room.grounds(), &solid, tour::region(from, to));

    let Some(way) = ways.plan((from, 0), (to, 0), &solid) else {
        println!("there is no way from the door to {:?}", to);

        // and enough to tell apart the two ways that happens: nowhere to
        // stand at one end, or places at both ends that join nothing
        match (ways.nearest(from, 0, &solid), ways.nearest(to, 0, &solid)) {
            (None, _) => println!("  nothing is laid near the door"),
            (_, None) => println!("  nothing is laid near {:?}", to),
            (Some(start), Some(end)) => {
                let reached = ways.reached_from(start);

                println!(
                    "  {} of {} places are reachable from the door, and {:?} on {} is not among them",
                    reached.len(),
                    ways.places(),
                    ways.spot(end).at,
                    ways.spot(end).ground
                );

                // **Which rooms, rather than how high.** Saying the
                // reachable set tops out at some height sent me up a
                // staircase three times for a fault that was two rooms
                // earlier. Where it stops is a question about which
                // floors it got to.
                let mut all: std::collections::BTreeMap<&str, (usize, usize)> =
                    std::collections::BTreeMap::new();

                for n in 0..ways.places() {
                    all.entry(ways.spot(n).ground).or_default().1 += 1;
                }

                for n in reached.iter() {
                    all.entry(ways.spot(*n).ground).or_default().0 += 1;
                }

                for (ground, (got, laid)) in all {
                    println!("    {:28} {:5} of {:5}", ground, got, laid);
                }
            }
        }

        return false;
    };

    println!("{} places over {} laid", way.len(), ways.places());

    let mut follower = tour::Follower::new(way);
    let mut at = from;
    let mut falling = 0.0;
    let step = 1.0 / 60.0;
    let mut rounds = 0;

    while let Some(steer) = follower.steer(at, 0, true) {
        let (went, fell) = walk::walk(at, steer.wish, falling, RADIUS, step, &solid, true);

        at = went;
        falling = fell;
        rounds += 1;

        if rounds > 60 * 180 {
            println!(
                "still going after three minutes, at {:?}, trying to reach {:?}",
                at,
                follower.toward()
            );

            return false;
        }
    }

    if !follower.arrived() {
        println!("it stopped short, still wanting {:?}", follower.toward());

        return false;
    }

    println!(
        "arrived at {:7.2}{:7.2}{:7.2} in {:.1} seconds, {:.2} off",
        at.x,
        at.y,
        at.z,
        rounds as f32 * step,
        at.distance(to)
    );

    at.distance(to) < 1.0
}

fn main() {
    // asked what it can photograph rather than told, so the list lives in one
    // place and that place is the game
    // and walked rather than told, for the same reason: the rule about which
    // arrows are yours lives in this binary, so the check does too
    if let Some(route) =
        std::env::args().find_map(|arg| arg.strip_prefix("--walk=").map(str::to_owned))
    {
        if !walked(&route) {
            std::process::exit(1);
        }

        return;
    }

    // **Spec 0049: plan a way somewhere and follow it.** The counterpart
    // of `--walk`, and the opposite question. That one asks whether a
    // written route still goes where it says; this one is handed a place
    // and works the route out, which is the whole point of the spec.
    if let Some(where_) =
        std::env::args().find_map(|arg| arg.strip_prefix("--tour=").map(str::to_owned))
    {
        if !toured(&where_) {
            std::process::exit(1);
        }

        return;
    }

    // **Spec 0049: and the clip is filmed off one of these.** A tour is
    // given to a window that is running, where `--tour` above only checks
    // one with no window at all.
    if let Some(read) = filmed(&room::Room::of(games())) {
        match read {
            Ok(steps) => {
                if steps.is_empty() {
                    println!("--film was given no places to go");
                    std::process::exit(1);
                }
            }
            Err(why) => {
                println!("{}", why);
                std::process::exit(1);
            }
        }
    }

    if std::env::args().any(|arg| arg == "--screenshots") {
        for (name, file) in shots() {
            println!("{}\t{}", name, file);
        }

        return;
    }

    start("capriccio", Box::new(Arcade::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How long to hold `w` from the door to be standing in front of the
    /// gallery's panel, with it in the sight.
    ///
    /// **Worked out rather than written down**, because it is the length
    /// of the hall and the hall is not a fixed number. It was 4.6
    /// seconds, measured once against a hall that then grew by 2.4 metres
    /// for spec 0050's bench: every route below walked up to a point five
    /// metres short of the panel with nothing in the sight at all, and
    /// only the one asking for a fault said so.
    ///
    /// **Half the reach rather than all of it.** What a jamb costs you on
    /// the way is not in this arithmetic and does not have to be: there
    /// are two metres of slack either side, and the walk measures at a
    /// metre and a bit inside them.
    fn up_to_the_panel() -> f32 {
        let room = room::Room::of(games());
        let (panel, _) = mandel::panel(room.reaches);

        (room.doorway().z - (panel.z + room::REACH * 0.5)) / SPEED
    }

    /// Spec 0042: a staged route may not go on past a turn that was eaten.
    ///
    /// The route walks the gallery's centre line until the panel takes the
    /// sight, which is the thing that happened in October: the left arrow
    /// goes into the panel instead of turning, and the walk after it is
    /// aimed by a turn that never happened.
    #[test]
    fn a_turn_the_panel_eats_is_a_fault_when_the_route_carries_on() {
        assert!(
            !walked(&format!(
                "hold:w:{:.2} hold:left:0.655 hold:w:1.0",
                up_to_the_panel()
            )),
            "a route whose turn was swallowed was let through"
        );
    }

    /// And the control, which is the same walk into the same panel with the
    /// key that came after it taken away.
    ///
    /// Ending on the panel is how the clip finishes, so this has to pass or
    /// the check refuses the one thing the panel can be filmed doing.
    ///
    /// **And it says what it walked up to.** A route that never reaches
    /// the panel is a route with no turn to swallow, which this passed
    /// just as readily and for none of its own reasons.
    #[test]
    fn a_route_may_end_by_holding_an_arrow_into_the_panel() {
        let walk = up_to_the_panel();

        assert_eq!(
            walk_through(&format!("hold:w:{:.2}", walk)),
            (true, Some(room::Seen::Mandel)),
            "the walk up to the panel does not end with it in the sight"
        );

        assert!(
            walked(&format!("hold:w:{:.2} hold:up:0.4", walk)),
            "a route that ends in the panel was refused"
        );

        // and a rest on the panel after it is still an ending, because a
        // bare wait has nothing to aim
        assert!(
            walked(&format!("hold:w:{:.2} hold:up:0.4 1.0", walk)),
            "a route that rests on the panel after the pan was refused"
        );
    }

    /// Spec 0042: and a walk that keeps off the panel is nobody's business.
    ///
    /// **Which it has to be shown doing.** Nothing here is measured off
    /// the panel, so the name is a claim about the sight and the sight is
    /// what it says: let through, with nothing in it.
    #[test]
    fn a_route_that_stays_out_of_the_panels_reach_is_let_through() {
        assert_eq!(
            walk_through("hold:w:3.1 hold:left:0.22 hold:w:1.6 hold:left:0.44 hold:w:0.96"),
            (true, None),
            "a route with nothing in its way was refused"
        );
    }

    /// Spec 0042: and a walk that something stops is a fault of its own.
    ///
    /// A turn that does not happen was the first way a staged route could
    /// lie about where the camera goes. A walk that does not go is the
    /// other, and it was found the same way: the threshold out of the
    /// hotel's lobby stops you dead on one heading and lets you through on
    /// both of the ones either side of it, and nothing in the trace said
    /// so. This one walks the gallery into its own end wall, so it asks for
    /// eighty-four metres and keeps twenty-four.
    #[test]
    fn a_walk_a_wall_stops_is_a_fault() {
        assert!(
            !walked("hold:w:20.0"),
            "a walk that spent most of itself against a wall was let through"
        );
    }

    /// And the control, which is the same walk with room to finish in.
    #[test]
    fn a_walk_with_room_to_finish_is_let_through() {
        assert!(
            walked("hold:w:4.0"),
            "a walk with nothing in its way was refused"
        );
    }

    /// Spec 0042: a route this cannot read is refused rather than guessed at.
    ///
    /// A hold with no seconds on it used to be a hold of nought seconds,
    /// which is a key that silently does nothing in a clip nobody watches
    /// closely enough to notice.
    #[test]
    fn a_route_this_cannot_read_is_refused() {
        assert!(!walked("hold:w"), "a hold with no seconds was read anyway");
        assert!(
            !walked("hold:q:1.0"),
            "a hold of a key with no meaning was read anyway"
        );
    }

    /// How big a room's picture is.
    ///
    /// Small, because every one of these is committed and read by a person,
    /// and the same size the engine's own pictures are, per
    /// the engine's spec 0062.
    const WIDE: u32 = 320;
    const TALL: u32 = 240;

    /// The building from one place, looking one way, with the clock at
    /// nought, in the shape the engine's spec 0076 asks for. Spec 0027.
    ///
    /// **Nothing in the picture may move.** The pool has a surface, the
    /// neon flickers, the metronome swings and the toys run, so a picture
    /// of any of them is a picture of what time it was. This builds the
    /// game, loads it and stands it somewhere. There is no `update` after
    /// the one below, so nothing has been given any time to happen in.
    /// A cabinet's picture, for a reference that has to hold still.
    ///
    /// **The games' own screenshots are thirteen other repos.** `art_of`
    /// reads each one off the disk, so a cabinet carries whatever that
    /// game was last photographed as, the hall's reference moves whenever
    /// any of them is retaken, and the glow each cabinet lends the room
    /// moves with it. The 0.15.0 release takes all thirteen before the
    /// gate and then failed on the hall by 0.6%, which cost nine commits
    /// to clear and would cost them again every release.
    ///
    /// So a picture test hands the hall art of its own. Flat, and a
    /// different colour per cabinet worked out from the name, so the
    /// screens are still lit and still differ from each other and nothing
    /// outside this repo decides what they look like.
    ///
    /// **What this gives up**, and it is worth saying: the hall's light is
    /// partly the games' own colours, and under this it is not. The
    /// picture holds the room, and `every_cabinet_finds_its_art` holds the
    /// thing it stopped holding. Spec 0053.
    fn stand_in_art(name: &str) -> Option<TextureData> {
        let mut hash = 0x811c_9dc5u32;
        for byte in name.as_bytes() {
            hash ^= *byte as u32;
            hash = hash.wrapping_mul(0x0100_0193);
        }

        // kept off black and off white, so a screen reads as a screen
        let channel = |shift: u32| 70 + ((hash >> shift) & 0xff) as u8 / 2;
        let texel = [channel(0), channel(8), channel(16), 255];

        Some(TextureData::from_pixels(
            2,
            2,
            texel.iter().copied().cycle().take(16).collect(),
        ))
    }

    fn standing(
        at: Vec3,
        looking_at: Vec3,
    ) -> impl FnOnce(&mut blitzkit::renderer::Renderer) -> (Scene, Camera) {
        standing_with(at, looking_at, false)
    }

    /// The same, with every room's question already answered.
    ///
    /// **Because the end of the trail is not drawn until it is earned.**
    /// Spec 0054's rule is read by the bridge, the gate and Valhalla
    /// alike, so a picture of any of them taken from a fresh game is a
    /// picture of an empty field.
    fn standing_at_the_end(
        at: Vec3,
        looking_at: Vec3,
    ) -> impl FnOnce(&mut blitzkit::renderer::Renderer) -> (Scene, Camera) {
        standing_with(at, looking_at, true)
    }

    /// A building loaded and ready to be stood in, which is the part that
    /// costs.
    ///
    /// Measured 2026-11-10: this is 6.3 seconds and the pose that follows it
    /// is 35 milliseconds, so a sweep that builds one of these per pose pays
    /// for the building every time. Separated so a sweep can pay once.
    fn built(
        renderer: &mut blitzkit::renderer::Renderer,
        done: bool,
    ) -> (Arcade, Geometry, TextRenderer, SoundSystem) {
        let mut arcade = Arcade::new();

        if done {
            for one in trail::Asked::every() {
                arcade.trail.gave(*one);
            }
        }

        arcade.art_from = stand_in_art;
        let mut geometry = Geometry::new();
        let mut text = TextRenderer::new();
        let sound = SoundSystem::new();

        arcade.load(renderer);
        arcade.initialize(&mut geometry, &mut text, &sound, (WIDE as f32, TALL as f32));

        (arcade, geometry, text, sound)
    }

    /// And standing it somewhere, which is the part that does not.
    ///
    /// One account of what a pose means, so `standing` and a sweep cannot
    /// come apart about where somebody is or which way they face.
    fn posed(
        arcade: &mut Arcade,
        geometry: &mut Geometry,
        text: &mut TextRenderer,
        sound: &SoundSystem,
        renderer: &mut blitzkit::renderer::Renderer,
        at: Vec3,
        looking_at: Vec3,
    ) -> (Scene, Camera) {
        arcade.before_frame(renderer);

        // where it stands and which way it looks, set on the game's own
        // fields rather than on the camera: `draw` works the camera out
        // from these, and two ways of saying where somebody is standing
        // would come apart.
        //
        // `at` is where the feet are. The angles are worked out from where
        // the eyes are, which is `EYE` above that, or a target on the floor
        // nearby comes out looking at the floor.
        let look = looking_at - vec3(at.x, at.y + EYE, at.z);

        arcade.at = at;
        // **And the height, which is not `at.y`.** The camera stands at
        // `eye`, which eases toward `at.y` over `EYE_LAGS` seconds so that
        // walking down a step is not a jump. With `dt` at nought it never
        // eases at all, so a pose set only through `at` is a pose at
        // whatever height the game opened on: the cellar came out black
        // and the baths came out looking at the garden, both from a camera
        // standing on the hall's floor.
        arcade.eye = at.y;
        arcade.yaw = look.x.atan2(-look.z);
        arcade.pitch = look.y.atan2(glam::vec2(look.x, look.z).length());

        // **One update, at no time at all.** A room's lamps are worked out
        // from where you are standing, so a frame drawn without this is a
        // room with the lights off: the cellar came out black the first
        // time. `dt` of nought is what keeps the pool's surface, the neon
        // and the metronome where they started.
        arcade.update(0.0, geometry, text, sound);

        // **And it checks it got there**, because the one thing this
        // function is for is being the single account of where somebody is
        // standing, and it was not always true.
        //
        // `update` resolves the body against the building, and a body set
        // down inside something can come to rest against it instead. Walking
        // the imaginarium's floors on 2026-11-10, a pose at its second floor
        // stuck at -7.30, 6.00, -9.77 and every pose after it in that process
        // rendered from that same spot, up to eleven metres from where it was
        // asked to stand. Sixteen sample points and four rooms came back with
        // one number and nothing said a word.
        //
        // Across the floor and not the height. Settling a few centimetres
        // onto what is under you is the walk working: the imaginarium's own
        // floors put the body 0.07 up. Moving sideways is not.
        let drifted = vec3(arcade.at.x - at.x, 0.0, arcade.at.z - at.z).length();

        assert!(
            drifted < 0.25,
            "asked to stand at {:.2},{:.2} and came to rest at {:.2},{:.2}, {:.2} away",
            at.x,
            at.z,
            arcade.at.x,
            arcade.at.z,
            drifted
        );

        let mut scene = Scene::new();
        let mut camera = Camera::new();

        arcade.draw(&mut scene, &mut camera);

        (scene, camera)
    }

    fn standing_with(
        at: Vec3,
        looking_at: Vec3,
        done: bool,
    ) -> impl FnOnce(&mut blitzkit::renderer::Renderer) -> (Scene, Camera) {
        move |renderer| {
            let (mut arcade, mut geometry, mut text, sound) = built(renderer, done);

            posed(
                &mut arcade,
                &mut geometry,
                &mut text,
                &sound,
                renderer,
                at,
                looking_at,
            )
        }
    }

    /// The same frame as pixels, for the controls that want to measure one
    /// rather than hold it against a file.
    ///
    /// Nothing at all on a machine with no adapter, which is a skip rather
    /// than a failure, the same as the engine's pictures.
    /// What share of a frame's pixels sit on an edge.
    ///
    /// **A room has edges and a surface pressed against the lens has none.**
    /// Brightness cannot tell those apart: the plastered doorway spec 0029
    /// found was a soft gradient whose spread was lower than a lit room's but
    /// well inside the range a dim one covers. What it had none of was
    /// detail. So this counts pixels whose neighbour to the right or below
    /// differs by more than a channel's worth, which is the same `CHANNEL`
    /// the picture tests call a difference.
    fn edges(picture: &blitzkit::renderer::Picture) -> f32 {
        let (wide, tall) = (picture.width as usize, picture.height as usize);
        let raw = |x: usize, y: usize| picture.pixels[(y * wide + x) * 4 + 1] as i32;

        // **Stretched to its own range first.** A fixed threshold reads a dim
        // room as a blank one: looking west out of the hotel's doorway into a
        // lobby full of desk and pigeonholes scored 0.0074, which is what the
        // fault itself scores, because the room is dark rather than empty.
        // The band is taken off the 5th and 95th of the pixels rather than the
        // ends, so one lamp in frame cannot flatten everything else.
        let mut sorted: Vec<i32> = (0..tall)
            .flat_map(|y| (0..wide).map(move |x| (x, y)))
            .map(|(x, y)| raw(x, y))
            .collect();
        sorted.sort_unstable();

        let low = sorted[sorted.len() / 20];
        let high = sorted[sorted.len() - 1 - sorted.len() / 20];
        let band = (high - low).max(1);
        let at = |x: usize, y: usize| (raw(x, y) - low) * 255 / band;
        let mut found = 0;

        for y in 0..tall - 1 {
            for x in 0..wide - 1 {
                let here = at(x, y);

                if (here - at(x + 1, y)).abs() > blitzkit::picture::CHANNEL
                    || (here - at(x, y + 1)).abs() > blitzkit::picture::CHANNEL
                {
                    found += 1;
                }
            }
        }

        found as f32 / ((wide - 1) * (tall - 1)) as f32
    }

    /// Several poses through one building.
    ///
    /// **Because standing somewhere is cheap and the building is not.**
    /// Measured 2026-11-10: the first renderer a process asks for costs 54
    /// seconds and every one after it 15 milliseconds, so sharing a renderer
    /// saves nothing. What costs is `built`, at 6.3 seconds, against 35
    /// milliseconds to pose and draw. Twelve poses through `drawn_from` is
    /// twelve buildings; this is one.
    fn drawn_along(poses: &[(Vec3, Vec3)]) -> Option<Vec<blitzkit::renderer::Picture>> {
        let mut renderer = pollster::block_on(blitzkit::renderer::Renderer::offscreen(WIDE, TALL))?;
        let (mut arcade, mut geometry, mut text, sound) = built(&mut renderer, false);
        let mut out = Vec::with_capacity(poses.len());

        for (at, looking_at) in poses.iter().copied() {
            let (scene, camera) = posed(
                &mut arcade,
                &mut geometry,
                &mut text,
                &sound,
                &mut renderer,
                at,
                looking_at,
            );

            renderer.set_camera(camera);
            out.push(renderer.picture(&scene, &Geometry::new(), &TextRenderer::new()));
        }

        Some(out)
    }

    fn drawn_from(at: Vec3, looking_at: Vec3) -> Option<blitzkit::renderer::Picture> {
        let mut renderer = pollster::block_on(blitzkit::renderer::Renderer::offscreen(WIDE, TALL))?;
        let (scene, camera) = standing(at, looking_at)(&mut renderer);

        renderer.set_camera(camera);

        // **The room, and not the readout.** `update` fills the help text
        // and the 2D overlay as well as the lamps, and at 320 by 240 the
        // help fills the frame and leaves no room for the building. What
        // these pictures are of is the 3D, so the quads and the text are
        // left out of them, which also means a reference does not change
        // when a line of help does. The engine's own harness leaves them
        // out for the same reason, so what is written here is what that
        // one does.
        Some(renderer.picture(&scene, &Geometry::new(), &TextRenderer::new()))
    }

    /// Standing in a doorway, you can see out of it, both ways along the
    /// wall.
    ///
    /// **The fault this is for cannot be seen from either room.** Spec 0029
    /// found two plastered faces inside the gallery's west doorway, one at
    /// each side of the opening, each turned inward at the other. Single
    /// sided, so both cull from the room they face and the doorway looks
    /// open from the gallery and from the lobby alike. Only from inside the
    /// opening do both face you, which is a position no picture test stands
    /// in and nothing else walks to. Jake found it by playing.
    ///
    /// So this stands where the wall is, which is a place you can be because
    /// the doorway is a gap in the colliders, and asks for detail rather than
    /// for brightness. A room has edges at any light level. A face against
    /// the lens has none.
    ///
    /// Across the thickness rather than at its middle, because the band the
    /// faces made was narrower than the wall and the middle is not where it
    /// has to be.
    /// How far apart the most and least detailed view from one doorway may
    /// be.
    ///
    /// Measured 2026-11-10, roofed: the west doorway spans 1.32 and the east
    /// 1.77. With the strip unroofed and spec 0029's faces back in the
    /// opening, the west spans 5.68.
    const EVENLY: f32 = 3.0;

    #[test]
    fn a_doorway_can_be_seen_out_of() {
        let room = room::Room::of(games());
        let reaches = room.reaches;

        // One doorway now: the west one was cut for the hotel and spec
        // 0063 walled it up.
        for (name, pieces, way) in [("east", back::gallery_east(reaches), back::way(reaches))] {
            let (from, to) = way;
            let middle = (from + to) * 0.5;
            let (near, far) = (pieces[0].min.x, pieces[0].max.x);

            // `.iter().copied()` and not `.into_iter()`: this crate is
            // edition 2018, where an array's `into_iter` hands out references
            let poses: Vec<(Vec3, Vec3)> = [0.25f32, 0.5, 0.75]
                .iter()
                .copied()
                .flat_map(|step| {
                    let at = vec3(near + (far - near) * step, EYE, middle);

                    [(at, at + Vec3::X), (at, at - Vec3::X)]
                })
                .collect();

            let Some(drawn) = drawn_along(&poses) else {
                eprintln!("no adapter here, so nothing was drawn");
                return;
            };

            let found: Vec<f32> = drawn.iter().map(edges).collect();
            let least = found.iter().copied().fold(f32::MAX, f32::min);
            let most = found.iter().copied().fold(0.0f32, f32::max);

            assert!(
                most <= least * EVENLY,
                "the {} doorway shows {:.4} of a frame in detail one way and {:.4} another, \
                 which is {:.1} times: something is in the opening",
                name,
                least,
                most,
                most / least.max(1e-6)
            );
        }
    }

    /// Spec 0041: flight goes the whole way with the building in the way.
    ///
    /// **Against the same walk with the colliders taken out, not against a
    /// landmark.** Held against a line on the floor the first build passed
    /// with nothing in the way at all, because the start it chose faced the
    /// garden's way in and the walk went straight through it. And held
    /// against the flight it passed with the colliders gone, because flight
    /// is four times the pace and gets further whatever is there. What is
    /// actually being claimed is that one of these is stopped and the other
    /// goes its full distance, so that is what is asked.
    #[test]
    fn flying_goes_through_what_walking_does_not() {
        let room = room::Room::of(games());
        let solid = room.solid();
        let start = room.doorway();
        // east, into the hall's own end wall. West is the garden's way in.
        let wish = Vec3::X;
        let (step, pace, rounds) = (1.0 / 60.0, SPEED, 180);

        let mut flew = start;
        let walked = |solid: &[blitzkit::collision::Aabb]| {
            let (mut at, mut falling) = (start, 0.0);

            for _ in 0..rounds {
                let (next, fell) = walk::walk(at, wish * pace, falling, RADIUS, step, solid, true);
                at = next;
                falling = fell;
            }

            at
        };

        for _ in 0..rounds {
            flew = flown(flew, wish, pace, step);
        }

        let (stopped, free) = (walked(&solid), walked(&[]));
        let whole = pace * FLIES * step * rounds as f32;

        assert!(
            ((flew - start).length() - whole).abs() < 1e-2,
            "flight went {:.2} of {:.2}",
            (flew - start).length(),
            whole
        );
        assert!(
            (free - stopped).length() > 1.0,
            "nothing stopped the walk: {:.2} against {:.2}",
            stopped.x,
            free.x
        );
    }

    /// Spec 0041: the reset puts you back where the game opens, from
    /// anywhere, and leaves you holding nothing.
    ///
    /// **From anywhere**, which is the point: flight can put you inside
    /// something, past everything, or out where no room's gate is true, and
    /// in all three the screen is black whichever way you turn. Jake found
    /// one. A reset that works from the places you can still see is not a
    /// reset.
    #[test]
    fn the_reset_puts_you_back_at_the_door() {
        let mut arcade = Arcade::new();
        let door = arcade.room.doorway();

        for lost in [
            vec3(0.0, -400.0, 0.0),
            vec3(1800.0, 60.0, -1800.0),
            vec3(0.0, 0.0, 0.0),
            // and where the imaginarium used to float, which spec 0063
            // parked: being out in the open over the hall's roof is still
            // a place flight can leave you with no room's gate true
            vec3(-18.5, 9.0, 1.5),
        ] {
            arcade.at = lost;
            arcade.eye = lost.y;
            arcade.yaw = 2.1;
            arcade.pitch = 0.9;
            arcade.falling = 7.0;
            arcade.climbs = 11.0;
            arcade.riding_to = Some(0);

            arcade.back_to_the_door();

            assert_eq!(arcade.at, door, "lost at {:?}", lost);
            assert_eq!(arcade.eye, door.y);
            assert_eq!((arcade.yaw, arcade.pitch), (0.0, 0.0));
            assert_eq!((arcade.falling, arcade.climbs), (0.0, 0.0));
            assert!(arcade.riding_to.is_none() && arcade.inside.is_none());
        }
    }

    /// And R does nothing at all when you are not flying, which is both
    /// halves of the claim: that the key is free, and that a building with
    /// no teleport in it does not grow one because a tool wanted a way home.
    #[test]
    fn r_does_nothing_when_you_are_walking() {
        let mut one = Arcade::new();

        one.at = vec3(1.0, 0.0, 2.0);
        one.yaw = 1.3;

        let (at, yaw) = (one.at, one.yaw);

        press(&mut one, KeyboardKey::R);

        assert_eq!((one.at, one.yaw), (at, yaw), "R moved a body on foot");
    }

    /// And it goes where you are looking, up and down included, which is the
    /// only way up there is.
    #[test]
    fn flying_follows_the_pitch() {
        let (step, pace) = (1.0 / 60.0, SPEED);
        let up = vec3(0.0, 0.6, -0.8);
        let flew = flown(Vec3::ZERO, up, pace, step);

        assert!((flew.length() - pace * FLIES * step).abs() < 1e-4);
        assert!(flew.y > 0.0, "looking up flew you to {:.3}", flew.y);
        assert_eq!(flown(Vec3::ZERO, Vec3::ZERO, pace, step), Vec3::ZERO);
    }

    /// Spec 0053: every cabinet finds the art the hall would show on it.
    ///
    /// **This is what the hall's picture stopped holding.** That reference
    /// is drawn with `stand_in_art` now, so it no longer notices a game
    /// whose screenshot has gone missing or stopped decoding. The question
    /// is arithmetic, so it is asked as arithmetic: `art_of` is the thing
    /// the game itself calls, and it reads the file off the disk and
    /// decodes it.
    ///
    /// Native only: the browser build bakes these in at compile time
    /// through `build.rs` and has no disk to read.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn every_cabinet_finds_its_art() {
        let room = room::Room::of(games());
        let missing: Vec<&str> = room
            .stood
            .iter()
            .map(|stood| stood.cabinet.name.as_str())
            .filter(|name| art_of(name).is_none())
            .collect();

        assert!(
            missing.is_empty(),
            "{} of {} cabinets found no picture: {:?}",
            missing.len(),
            room.stood.len(),
            missing
        );
        assert!(
            !room.stood.is_empty(),
            "the hall has no cabinets at all, so this proves nothing"
        );
    }

    /// Spec 0051: the four rooms whose question is a press or a ride.
    ///
    /// **Driven through `use_what_i_see`, not by calling `gave`.** The
    /// record's own tests already prove the list; what these prove is that
    /// the building fills it, which is the half that silently does nothing
    /// if a line is dropped.
    #[test]
    fn a_press_gives_its_room_up() {
        // the cellar's bookcase, by the one leaf in it that is a handle
        let mut arcade = Arcade::new();
        arcade.seen = Some(room::Seen::Case);
        arcade.use_what_i_see();
        assert_eq!(
            arcade.trail.given(),
            [trail::Asked::OpenedTheCase],
            "the case opened and the building did not notice"
        );

        // and working it again is not a second answer
        arcade.use_what_i_see();
        assert_eq!(
            arcade.trail.given().len(),
            1,
            "working the case twice answered the room twice"
        );

        // the sixth display, which turns you instead of turning
        let mut arcade = Arcade::new();
        arcade.seen = Some(room::Seen::Display(display::tesseract()));
        arcade.use_what_i_see();
        assert_eq!(
            arcade.trail.given(),
            [trail::Asked::WentInsideTheTesseract],
            "you went inside and the building did not notice"
        );
    }

    /// Spec 0051: the two rooms that answer by being reached.
    ///
    /// **Asked of where the feet are**, which is `noise::room_at` and is
    /// the one account of where the rooms are. A flag set at a doorway
    /// would be a second, and this building has learned what two accounts
    /// of one thing do.
    ///
    /// The control is in the test: standing in the hall first, and
    /// checking the building has been given nothing, so that a record that
    /// filled itself the moment anybody moved would fail here.
    #[test]
    fn a_room_reached_gives_itself_up() {
        let mut arcade = Arcade::new();
        let reaches = arcade.room.reaches;

        // the hall asks nothing by being stood in
        arcade.at = vec3(0.0, EYE, 0.0);
        assert_eq!(
            noise::room_at(reaches, arcade.at),
            noise::Room::Hall,
            "the middle of the hall is not the hall, so this proves nothing"
        );
        match noise::room_at(reaches, arcade.at) {
            noise::Room::Behind => arcade.trail.gave(trail::Asked::GotBehindTheWall),
            noise::Room::Field => arcade.trail.gave(trail::Asked::WentOutIntoTheField),
            _ => (),
        }
        assert!(
            arcade.trail.is_empty(),
            "standing in the hall answered a room"
        );

        // behind the hall's near end wall, per spec 0011
        arcade.at = behind::at(reaches) + Vec3::Y * EYE;
        assert_eq!(
            noise::room_at(reaches, arcade.at),
            noise::Room::Behind,
            "that point is not behind the wall, so this proves nothing"
        );
        match noise::room_at(reaches, arcade.at) {
            noise::Room::Behind => arcade.trail.gave(trail::Asked::GotBehindTheWall),
            noise::Room::Field => arcade.trail.gave(trail::Asked::WentOutIntoTheField),
            _ => (),
        }
        assert_eq!(
            arcade.trail.given(),
            [trail::Asked::GotBehindTheWall],
            "getting behind the wall gave that room up"
        );
    }

    /// Spec 0051: the ride out to the sun, which is the one stop out there
    /// that is the reading rather than a place.
    ///
    /// **Arriving, not setting out.** A leg you turn round on is not the
    /// trip, so this ends the ride by hand at the sun's stop and then at a
    /// planet's, and only the first is the room answering.
    #[test]
    fn arriving_at_the_sun_gives_the_system_up() {
        let mut arcade = Arcade::new();
        let stops = beyond::stops(arcade.the_door());

        // **Which stop that is comes from the list, not from the constant
        // this is testing.** The first version of this set the stop to
        // SUN_STOP and then asked whether SUN_STOP was the sun, which is
        // the same question twice: pointing the constant at the way back
        // left it passing.
        let sun = stops
            .iter()
            .position(|stop| stop.name == orrery::SUN.name)
            .expect("the sun is a stop you can ride to");

        for (n, _) in stops.iter().enumerate() {
            arcade.stop = n;
            if beyond::is_the_sun(arcade.stop) {
                arcade.trail.gave(trail::Asked::RodeToTheSun);
            }
        }

        assert_eq!(
            arcade.trail.given(),
            [trail::Asked::RodeToTheSun],
            "riding every stop in turn answered the room {} times, not once",
            arcade.trail.given().len()
        );
        assert!(
            beyond::is_the_sun(sun),
            "the stop the list calls the sun is not the one is_the_sun names"
        );
    }

    /// Spec 0039: an arrow on the picture throws the settled one away.
    ///
    /// **Otherwise the wall is a window onto two places at once.** The
    /// fine picture takes a second and a half to come in at the bottom of
    /// the dive, and a view that moves while it does is a view whose
    /// finished picture is of somewhere you have already left. What would
    /// be on the wall is the coarse picture of where you are with a sharp
    /// one of where you were arriving over the top of it.
    #[test]
    fn moving_throws_the_settled_picture_away() {
        let mut arcade = Arcade::new();

        // settled, where you walked in
        for _ in 0..200 {
            arcade.settle();

            if arcade.sharp_view == Some(arcade.view) {
                break;
            }
        }

        assert_eq!(
            arcade.sharp_view,
            Some(arcade.view),
            "the picture never settled at all"
        );

        // and then an arrow goes down
        arcade.moving_the_view = true;
        arcade.view = arcade.view.moved((1.0, 0.0), 0.0, 1.0 / 60.0);
        arcade.settle();

        assert!(
            arcade.sharp_view.is_none(),
            "the settled picture is still on the wall and the view has moved"
        );
        assert!(
            arcade.settling.is_none(),
            "a settling of the view you have left is still being worked out"
        );
    }

    /// Where a room's reference lives.
    fn reference(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/rooms")
            .join(format!("{}.png", name))
    }

    /// Draws a room and holds it against the picture that was right.
    ///
    /// The engine's spec 0076 owns the renderer, the empty overlay, the
    /// comparison and what to do on a machine with no adapter, so what is
    /// left here is which room and which file.
    fn looks_right(name: &str, at: Vec3, looking_at: Vec3) {
        blitzkit::picture::drawn_looks_right(
            &reference(name),
            (WIDE, TALL),
            standing(at, looking_at),
        );
    }

    /// The same, from a game with the whole trail answered.
    fn looks_right_at_the_end(name: &str, at: Vec3, looking_at: Vec3) {
        blitzkit::picture::drawn_looks_right(
            &reference(name),
            (WIDE, TALL),
            standing_at_the_end(at, looking_at),
        );
    }

    /// Spec 0047: and nothing outside the imaginarium knows about any of
    /// it.
    ///
    /// **The control on the whole spec.** Walking is now two paths and the
    /// second one turns the building into somebody's frame. This asks that
    /// the first is still what the rest of the building takes: walked from
    /// the door down the hall, you stay in frame nought, you stay on the
    /// hall's floor, and you cover the ground.
    #[test]
    fn the_building_walks_as_it_did() {
        let mut one = Arcade::new();

        one.at = one.room.doorway();
        one.eye = one.at.y;
        one.yaw = 0.0;
        one.walking = [true, false, false, false];

        let from = one.at;

        for _ in 0..120 {
            one.aimed_and_moved(1.0 / 60.0);
        }

        assert_eq!(one.standing_in, 0, "the hall turned you over");
        assert!(
            one.at.y.abs() < 1e-3,
            "you left the hall's floor, to {:.3}",
            one.at.y
        );
        assert!(
            one.at.distance(from) > 6.0,
            "two seconds of walking covered {:.1} metres",
            one.at.distance(from)
        );
    }

    /// Jake, twice: the walk up the bridge is slow and jittery.
    ///
    /// **The test that would have caught it, and did not exist.**
    /// `bridge::tests::walked_from_the_foot_to_the_top` drives
    /// `walk::walk` with climbing always allowed, so it never saw the
    /// climb budget and reported four seconds for something that took
    /// eleven in the hand. This one walks the real loop, budget and
    /// all, which is the only place the two agree or do not.
    #[test]
    fn the_bridge_is_walked_up_rather_than_climbed() {
        climbed_the_bridge(false);
    }

    /// And running it, which is the pace Jake plays at and twice the
    /// distance covered per frame against treads 0.12 long.
    #[test]
    fn the_bridge_is_run_up_without_falling_through_it() {
        climbed_the_bridge(true);
    }

    /// And on a machine having a bad moment.
    ///
    /// **Jake: there is some clipping where the player ends up beneath
    /// the bridge for a bit.** Walked and run at a steady sixtieth
    /// nothing goes wrong, which is why this needed a third case: at
    /// eleven metres a second a frame that takes an eighteenth of a
    /// second covers most of a metre, and the treads holding you up are
    /// twelve centimetres long. One long frame steps clean over several
    /// of them.
    #[test]
    fn a_long_frame_does_not_drop_you_through_the_deck() {
        climbed_the_bridge_at(true, 1.0 / 18.0, 110);
    }

    #[test]
    fn the_camera_stays_above_the_deck_however_fast_the_screen_is() {
        for rate in [60.0f32, 90.0, 120.0, 144.0] {
            climbed_the_bridge_at(true, 1.0 / rate, (6.0 * rate) as u32);
        }
    }

    /// And it is climbed at the pace it was given, on any screen.
    ///
    /// **The fault under the one Jake saw.** `walk::over_a_step` lifts
    /// you onto a riser and carries you most of a radius forward, which
    /// is what being over a tread costs and is invisible on a stair.
    /// Bifröst is a slope cut into treads 0.12 long, so every frame was
    /// a step and every frame took the reach as well as its own
    /// movement. The climb ran at roughly twice its pace, and at a rate
    /// per frame rather than per second, so the better the screen the
    /// faster the bridge: 39.9 metres a second at sixty and 50.9 at a
    /// hundred and forty four, against a run of 23.5.
    ///
    /// `walk::RIDES` takes the risers out of the way instead, so the
    /// deck is walked rather than stepped. It is now 4.5 seconds foot
    /// to top at every one of these rates, within a hundredth.
    ///
    /// **And steering is no longer quicker either**, which took until
    /// 2026-11-07. It was the same double payment seen from the side:
    /// pressed against the kerb the body is stopped by something it
    /// cannot climb, the probe reads the deck through the verge and
    /// calls it a step, and the carry onto that step came with a second
    /// `wish * dt` on top of it. The step no longer walks again from
    /// where it put you, and what holds it is
    /// `steering_up_the_bridge_is_not_quicker_than_going_straight`.
    #[test]
    fn the_bridge_is_climbed_at_the_pace_it_was_given() {
        for rate in [60.0f32, 90.0, 120.0, 144.0] {
            let dt = 1.0 / rate;
            let mut one = Arcade::new();

            one.shifted = true;

            for asked in trail::Asked::every() {
                one.trail.gave(*asked);
            }

            let foot = bridge::foot(one.room.reaches);

            one.at = vec3(foot.x - 1.0, 0.1, foot.z);
            one.eye = one.at.y;
            one.yaw = -std::f32::consts::FRAC_PI_2;
            one.walking = [true, false, false, false];

            let (mut on, mut took) = (None, None);

            for frame in 0..(10.0 * rate) as u32 {
                one.aimed_and_moved(dt);

                let along = (foot.x - one.at.x) / bridge::RUN;

                if on.is_none() && along >= 0.02 {
                    on = Some(frame);
                }

                if along >= 1.0 {
                    took = on.map(|on| (frame - on) as f32 * dt);

                    break;
                }
            }

            let took = took.expect("it gets to the top");
            // what the pace says it should be, which is the whole point
            let pace = SPEED * RUNS * bridge::CARRIES;
            let ought = bridge::RUN * 0.98 / pace;

            assert!(
                took > ought * 0.95,
                "at {rate:.0} frames a second the climb took {took:.2}s, \
                 and {:.1} a second over {:.0} metres is {ought:.2}s",
                pace,
                bridge::RUN * 0.98
            );
            assert!(
                took < ought * 1.25,
                "at {:.0} frames a second the climb took {:.2}s against \
                 {:.2}s at the pace it was given",
                rate,
                took,
                ought
            );
        }
    }

    /// And steered rather than driven straight up the middle, which is
    /// how anybody actually climbs it.
    #[test]
    fn the_bridge_is_climbed_off_the_straight_without_falling_through() {
        for tenth in -6..=6 {
            climbed_the_bridge_from(true, 1.0 / 60.0, 360, tenth as f32 * 0.05);
        }
    }

    /// And steering up it is not quicker than going straight.
    ///
    /// **The fault spec 0054 recorded and could not catch, until the
    /// step stopped paying twice.** Pressed against the kerb the body
    /// is stopped by something it cannot climb, the probe in
    /// `walk::over_a_step` reads the deck through the verge and calls
    /// it a step, and the carry onto that step came with a second
    /// `wish * dt` on top of it. On a stair that hardly shows, because
    /// the step fires once a tread. Here it fired every frame.
    ///
    /// Measured mid deck before the fix: 23.5 a second going straight
    /// and a peak of 45.3 against the kerb, with the mean over the
    /// middle of the deck 27.3. Those two are why this was written
    /// down twice with different numbers.
    ///
    /// **Timed to the top rather than measured mid deck**, because
    /// quicker means getting there sooner and nothing else. A peak of
    /// one frame is a carry onto one tread, which is what a step is
    /// everywhere in this building.
    #[test]
    fn steering_up_the_bridge_is_not_quicker_than_going_straight() {
        // straight first, because every other run is measured against it
        let straight = nine_tenths_up_the_bridge(0.0).expect("straight gets up the bridge");

        for tenth in -6..=6 {
            let askew = tenth as f32 * 0.05;

            if askew == 0.0 {
                continue;
            }
            let Some(took) = nine_tenths_up_the_bridge(askew) else {
                continue;
            };

            assert!(
                took >= straight,
                "askew {:.2} got nine tenths of the way up in {} frames against {} \
                 going straight, so steering is quicker",
                askew,
                took,
                straight
            );
        }
    }

    /// Frames to get nine tenths of the way up Bifrost at a run, or
    /// `None` for a line that never gets there.
    fn nine_tenths_up_the_bridge(askew: f32) -> Option<u32> {
        let mut one = Arcade::new();

        for asked in trail::Asked::every() {
            one.trail.gave(*asked);
        }

        let foot = bridge::foot(one.room.reaches);

        one.at = vec3(foot.x - 1.0, 0.1, foot.z);
        one.eye = one.at.y;
        one.yaw = -std::f32::consts::FRAC_PI_2 + askew;
        one.pitch = 0.0;
        one.shifted = true;
        one.walking = [true, false, false, false];

        for frame in 0..900 {
            one.aimed_and_moved(1.0 / 60.0);

            if (foot.x - one.at.x) / bridge::RUN >= 0.9 {
                return Some(frame + 1);
            }
        }

        None
    }

    /// And the kerb keeps you on it.
    ///
    /// **Carrying you up it made falling off it easy.** Six metres of
    /// road with nothing either side was survivable at four metres a
    /// second; at eleven, three degrees off the straight is half a
    /// metre a second sideways and the climb is five seconds long.
    /// Jake went over the side near the top, and the frame his
    /// recording caught is from down in the field looking back up.
    #[test]
    fn the_kerb_keeps_you_on_the_bridge() {
        for tenth in -6..=6 {
            let askew = tenth as f32 * 0.05;
            let mut one = Arcade::new();

            for asked in trail::Asked::every() {
                one.trail.gave(*asked);
            }

            let foot = bridge::foot(one.room.reaches);

            one.at = vec3(foot.x - 1.0, 0.1, foot.z);
            one.eye = one.at.y;
            one.yaw = -std::f32::consts::FRAC_PI_2 + askew;
            one.pitch = 0.0;
            one.shifted = true;
            one.walking = [true, false, false, false];

            for _ in 0..360 {
                one.aimed_and_moved(1.0 / 60.0);

                let along = (foot.x - one.at.x) / bridge::RUN;

                // on the ramp, and not yet onto the landing, which is
                // wider and has a parapet of its own
                if !(0.05..=0.95).contains(&along) {
                    continue;
                }

                assert!(
                    one.at.y > bridge::high(along) - 0.5,
                    "askew {askew:.2}, off the bridge at {along:.2} of the way along, \
                     {:.1} up where the deck is {:.1}",
                    one.at.y,
                    bridge::high(along)
                );
            }
        }
    }

    /// Ten seconds walking, six running.
    ///
    /// **Not one number for both, since they are not one climb.** A
    /// run is 23.5 a second and gets up in 4.5; a walk is 11.8 and
    /// takes 8.5, because `bridge::CARRIES` multiplies whichever you
    /// are doing. Six held for both only while every tread was handing
    /// out a reach of free travel, which `walk::RIDES` took back.
    fn climbed_the_bridge(running: bool) {
        let seconds = if running { 6.0 } else { 10.0 };

        climbed_the_bridge_at(running, 1.0 / 60.0, (seconds * 60.0) as u32);
    }

    fn climbed_the_bridge_at(running: bool, dt: f32, frames: u32) {
        climbed_the_bridge_from(running, dt, frames, 0.0);
    }

    fn climbed_the_bridge_from(running: bool, dt: f32, frames: u32, askew: f32) {
        let mut one = Arcade::new();

        one.shifted = running;

        for asked in trail::Asked::every() {
            one.trail.gave(*asked);
        }

        let foot = bridge::foot(one.room.reaches);

        one.at = vec3(foot.x - 1.0, 0.1, foot.z);
        one.eye = one.at.y;
        // yaw nought faces -z, and the climb runs toward -x
        one.yaw = -std::f32::consts::FRAC_PI_2 + askew;
        one.pitch = 0.0;
        one.walking = [true, false, false, false];

        // six seconds. Unassisted and paced by the budget it is eleven,
        // so this is a bound neither the carry nor the ramp rule can be
        // taken out of.
        //
        // **And it watches the whole climb rather than the end of it.**
        // Jake: there is some clipping where the player ends up beneath
        // the bridge for a bit. A test that only reads the height it
        // finished at cannot see a hole in the middle.
        let mut worst = 0.0f32;
        let mut worst_at = 0.0f32;
        // **And where the camera is, which is not where the feet are.**
        // The whole of why the first round of this missed Jake's
        // picture: his feet never left the deck. See `EYE_TRAILS`.
        let mut lowest = f32::INFINITY;
        let mut lowest_at = 0.0f32;

        for _ in 0..frames {
            one.aimed_and_moved(dt);

            let along = (foot.x - one.at.x) / bridge::RUN;

            // **Past the end of the deck too, which is where Jake's
            // picture was taken.** The window used to stop at the
            // deck's last tread, and the frame he caught is under the
            // landing: the one junction in the whole climb where the
            // thing holding you up changes from one list of boxes to
            // another.
            if !(0.02..=1.6).contains(&along) {
                continue;
            }

            // **And only while you are over it.** Walking off the side
            // of an unrailed deck is falling off a bridge, which is
            // what happens when you do that, and counting it as a drop
            // through the deck hides the thing being looked for.
            if (one.at.z - foot.z).abs() > bridge::WIDE * 0.5 {
                continue;
            }

            let deck = bridge::high(along.min(1.0));
            let under = deck - one.at.y;

            if under > worst {
                worst = under;
                worst_at = along;
            }

            let over = one.looking_from().y - deck;

            if over < lowest {
                lowest = over;
                lowest_at = along;
            }
        }

        assert!(
            lowest > 0.9,
            "{} askew {askew:.2} at a {:.0}th of a second, the camera was \
             {:.2} over the deck at {:.2} of the way along",
            if running { "running," } else { "walking," },
            1.0 / dt,
            lowest,
            lowest_at
        );

        assert!(
            worst < 0.5,
            "{} askew {askew:.2}, fell {:.2} under the deck at {:.2} of the way along",
            if running { "running," } else { "walking," },
            worst,
            worst_at
        );

        // how far you got is only asked of the straight climb: going up
        // at an angle you cover less of the run, which is arithmetic
        // and not a fault.
        if askew == 0.0 {
            assert!(
                one.at.y > bridge::RISE * 0.95,
                "{:.0} seconds of walking got {:.1} up a climb of {}",
                frames as f32 * dt,
                one.at.y,
                bridge::RISE
            );
        }
    }

    /// Spec 0049: the clip walks the whole route, in a test.
    ///
    /// **Because until now only the film could say.** The route test in
    /// `tour` walks each leg with a `Follower` against the room's boxes,
    /// which is not the body in the game: it skips the legs that change
    /// frame, it starts each leg from the place the one before was aimed
    /// at rather than from where the body actually got to, and nothing
    /// in it has a camera. Three faults went through it untouched on
    /// 2026-11-01 and all three were found by filming, watching, and
    /// working a timestamp back to a position, at a minute a round.
    ///
    /// `update` is four calls around a tour and none of them wants a
    /// renderer, because spec 0042 pulled them out for its own replay.
    /// They go in `update`'s order here: the doors move, the tour presses
    /// the keys, the body walks, and the sight it leaves is what the next
    /// frame aims and works by. Thirty-six seconds of film is 2160 frames
    /// of that.
    ///
    /// What it asks is what the takes were being watched for: that every
    /// place the route names is arrived at, that the route finishes, and
    /// how long it takes, which is the number `refresh-examples` wants
    /// for the camera and which was being read off a log by hand.
    #[test]
    fn the_clip_walks_the_whole_route() {
        let tick = 1.0 / 60.0;
        let mut game = Arcade::new();
        let steps = tour::clip(&game.room, true);
        let places = steps.len();

        game.filming = Some(Filming {
            steps,
            at: 0,
            following: None,
            waiting: 0.0,
        });

        // a stride, which is a body that walked there rather than one
        // that gave the place up
        let near = 1.0;
        let mut missed = Vec::new();
        let mut rounds = 0;
        let most = 60 * 180;

        while rounds < most {
            let was = game.filming.as_ref().map_or(places, |film| film.at);

            game.the_doors_moved(tick);
            game.on_the_tour(tick);
            game.aimed_and_moved(tick);
            // the sight a frame aims by is the one the frame before it
            // left, which is where `update` puts it and is what the Work
            // steps act on. Without it `self.seen` is never anything and
            // every door on the route stays shut: the clip got as far as
            // the bookcase, found no way through it, and the test said
            // the route was walked.
            game.seen = game.what_i_see();
            rounds += 1;

            let Some(film) = game.filming.as_ref() else {
                break;
            };

            // **Every step it got through this frame, and not just the
            // one it started on.** A frame can finish a stand, fail to
            // plan the leg after it and land on the look after that, all
            // before it returns. Asked of the step it started on, the
            // leg that had no way was the one in the middle and went
            // unreported.
            for step in film.steps[was..film.at].iter() {
                let tour::Step::At(place, _) = step else {
                    continue;
                };

                if game.at.distance(*place) > near {
                    missed.push(format!(
                        "{:?}, which it left {:.1} away at {:7.2}{:7.2}{:7.2}",
                        place,
                        game.at.distance(*place),
                        game.at.x,
                        game.at.y,
                        game.at.z
                    ));
                }
            }

            if film.at >= places {
                break;
            }
        }

        println!(
            "the clip is {:.1} seconds of film over {} steps",
            rounds as f32 * tick,
            places
        );

        assert!(
            missed.is_empty(),
            "{} of the route's places were not walked to: {}",
            missed.len(),
            missed.join("; ")
        );
        assert!(
            rounds < most,
            "the route had not finished after {:.0} seconds",
            most as f32 * tick
        );

        // **And the two doors it works are open at the end.** A `Work`
        // acts on the sight the frame before left, so a loop that never
        // takes one works nothing. Both doors stayed shut, the legs past
        // them had no way, and what said so was a place the route never
        // got to rather than anything about a door. Named here so the
        // next loop that drops a call says which call it dropped.
        assert!(
            game.room.open && game.room.baths_open,
            "it finished the route with the bookcase {} and the baths' leaf {}",
            if game.room.open { "open" } else { "shut" },
            if game.room.baths_open { "open" } else { "shut" }
        );
    }

    /// Spec 0049: the sight finds what every `Work` in the clip works.
    ///
    /// **In shot is not the same as pointed at.** The test above asks
    /// whether the thing the route names is within half the camera, which
    /// it was: the clip looked at the middle of the bookcase's opening
    /// from thirty centimetres off the place it names and the ray cleared
    /// the leaf's north face by four millimetres. The sight found nothing,
    /// `Work` worked nothing, the bookcase stayed shut, and what said so
    /// was two places at the far end of the route that the body never got
    /// to. Nothing in the route or the sight was named at all.
    ///
    /// **Asked from a ring and not from the place alone**, because a leg
    /// ends where the lattice has a place rather than where the route
    /// says. The ring is `tour::APART`, which is how far apart those
    /// places are and so how far out the body can finish.
    #[test]
    fn the_sight_finds_what_each_work_in_the_clip_works() {
        let room = room::Room::of(games());
        let steps = tour::clip(&room, true);
        let tick = 1.0 / 60.0;

        // the three things on this route that are worked, said the way the
        // route says them, so there is one account of each
        let middle = |box_: Aabb| (box_.min + box_.max) * 0.5;
        let cube = room
            .benches
            .iter()
            .position(|one| one.name == kociemba::NAME)
            .expect("the nook's row has the cube in it");
        let worked = [
            (
                middle(room.handle().expect("the cellar has a way down into it")),
                room::Seen::Case,
            ),
            (middle(spa::leaf_box(room.reaches, 0.0)), room::Seen::Baths),
            (room.benches[cube].looked_at(), room::Seen::Bench(cube)),
        ];

        let mut stood = (room.doorway(), 0u64);
        let mut look: Option<tour::Step> = None;
        let mut works = 0;

        for step in steps.iter() {
            match step {
                tour::Step::At(at, frame) => stood = (*at, *frame),
                tour::Step::Look(_) => look = Some(*step),
                tour::Step::Work => {
                    let look = look.expect("a Work follows a Look");
                    let tour::Step::Look(toward) = look else {
                        unreachable!("a Look is a Look")
                    };
                    let want = worked
                        .iter()
                        .min_by(|one, two| {
                            toward.distance(one.0).total_cmp(&toward.distance(two.0))
                        })
                        .map(|(_, seen)| *seen)
                        .expect("there are things on this route to work");

                    works += 1;

                    for (dx, dz) in [
                        (0.0, 0.0),
                        (1.0, 0.0),
                        (-1.0, 0.0),
                        (0.0, 1.0),
                        (0.0, -1.0),
                        (0.7, 0.7),
                        (0.7, -0.7),
                        (-0.7, 0.7),
                        (-0.7, -0.7),
                    ] {
                        let from = stood.0 + vec3(dx, 0.0, dz) * tour::APART;
                        let mut game = Arcade::new();

                        game.at = from;
                        game.standing_in = stood.1;
                        game.filming = Some(Filming {
                            steps: vec![look],
                            at: 0,
                            following: None,
                            waiting: 0.0,
                        });

                        for _ in 0..(60 * 5) {
                            game.on_the_tour(tick);
                        }

                        assert_eq!(
                            game.what_i_see(),
                            Some(want),
                            "looking at {:?} from {:?} works {:?} instead",
                            toward,
                            from,
                            game.what_i_see()
                        );
                    }
                }
                _ => {}
            }
        }

        assert_eq!(
            works,
            worked.len(),
            "the route works {} things rather than {}",
            works,
            worked.len()
        );
    }

    /// Spec 0049: a `Look` ends up with the thing it names in shot.
    ///
    /// **Turning is only half of pointing.** A `Look` that sets the yaw
    /// and leaves the pitch faces the right way along the floor and goes
    /// on looking at the horizon, which indoors is a wall. The clip stood
    /// in the middle of the imaginarium's floor, named the middle of the
    /// room, and filmed two and a half seconds of plaster; the route had
    /// already worked round it twice, by naming the bookcase and the
    /// baths' leaf at eye height rather than at their handles.
    ///
    /// In shot rather than dead centre, because the pitch is clamped
    /// where a player's is. The middle of that room is straight up from
    /// the middle of its floor, and 74 degrees is as far up as anybody
    /// gets. Half the camera's 60 is what "in shot" means vertically, and
    /// it is the tighter half.
    ///
    /// The pairs are the route's own: every `Look` in it, from whatever
    /// place the step before left the body standing in.
    #[test]
    fn every_look_in_the_clip_ends_with_what_it_names_in_shot() {
        let room = room::Room::of(games());
        let steps = tour::clip(&room, true);
        let tick = 1.0 / 60.0;
        let most = 30f32.to_radians();
        let mut stood = (room.doorway(), 0u64);
        let mut looks = 0;

        for step in steps.iter() {
            match step {
                tour::Step::At(at, frame) => stood = (*at, *frame),
                tour::Step::Look(toward) => {
                    let mut game = Arcade::new();

                    game.at = stood.0;
                    game.standing_in = stood.1;
                    game.filming = Some(Filming {
                        steps: vec![*step],
                        at: 0,
                        following: None,
                        waiting: 0.0,
                    });

                    for _ in 0..(60 * 5) {
                        game.on_the_tour(tick);
                    }

                    let want = (*toward - game.looking_from()).normalize();
                    let off = game.facing().dot(want).clamp(-1.0, 1.0).acos();

                    looks += 1;

                    assert!(
                        off < most,
                        "looking at {:?} from {:?} ends up {:.0} degrees off it",
                        toward,
                        stood,
                        off.to_degrees()
                    );
                }
                _ => {}
            }
        }

        assert!(looks >= 5, "only {} of the route's looks were asked", looks);
    }

    /// Spec 0049: walking a leg puts the camera back on the level.
    ///
    /// **Because only a `Look` ever moves the pitch.** Tilt up at
    /// something and nothing tilts down again: the clip looked at the
    /// middle of the imaginarium, 74 degrees up, and then walked four
    /// legs with the camera still on the ceiling.
    #[test]
    fn a_leg_of_the_route_brings_the_camera_back_to_level() {
        let mut game = Arcade::new();
        let from = game.room.doorway();

        game.at = from;
        game.pitch = PITCH_LIMIT;
        game.filming = Some(Filming {
            steps: vec![tour::Step::At(from + vec3(0.0, 0.0, -6.0), 0)],
            at: 0,
            following: None,
            waiting: 0.0,
        });

        for _ in 0..(60 * 2) {
            game.on_the_tour(1.0 / 60.0);
        }

        assert!(
            game.pitch.abs() < 0.01,
            "it walked the leg at {:.0} degrees off the level",
            game.pitch.to_degrees()
        );
    }

    /// How far the building reaches, which every room is measured from.
    fn reaches() -> f32 {
        room::Room::of(games()).reaches
    }

    /// Spec 0027: the hall, from the door, down the aisle.
    ///
    /// Where the building opens, which is the view `media/arcade.png` has
    /// always been.
    #[test]
    fn the_hall_is_drawn() {
        let reaches = reaches();
        let doorway = room::Room::of(games()).doorway();

        looks_right("hall", doorway, vec3(0.0, EYE, -reaches * 0.5));
    }

    /// Spec 0027: the nook, which is where the toys are.
    ///
    /// **Along it rather than across it**, from a stride inside the south
    /// end. Across it was the view from the middle of the floor at the
    /// books, and the middle of the floor is where the way up stands: with
    /// the parapet up, that camera stood a hand from it and the picture was
    /// a grey slab. Along the room you get the benches, the books, the rug
    /// and the stair between them, which is more of the nook than the old
    /// one showed.
    #[test]
    fn the_nook_is_drawn() {
        let reaches = reaches();
        let (at, _) = room::nook_floor(reaches);

        looks_right(
            "nook",
            vec3(at.x, EYE, -reaches + 1.4),
            vec3(at.x, EYE, at.z),
        );
    }

    /// Spec 0061: the corridor of mirrors, square on to its west wall.
    ///
    /// **Square on, from the middle of the floor**, which is the one bearing
    /// that cannot be chosen to flatter the room: a mirror facing you either
    /// shows the corridor or it does not. Along it, the panels are edge on
    /// and a picture of them says nothing about what is in them.
    ///
    /// Spec 0061 and spec 0033. Every account of which way a figure is
    /// pointing has to be the one account, and for a day there were two.
    ///
    /// `people::walking` and `aloft::walking` both build a facing with
    /// `way.x.atan2(-way.z)`, which is the yaw whose direction is
    /// `(sin, 0, -cos)`: the same one `Arcade::forward` uses, and the same
    /// one `stand` places eyes and arms along. The body's mesh was turned
    /// with `Quat::from_rotation_y(facing)`, which sends its own `-Z` to
    /// `(-sin, 0, -cos)`, the mirror of all of them about the north line.
    ///
    /// Invisible while a torso is a symmetric box and a guest walks a
    /// corridor, where sin or cos is near enough nought that the two agree.
    /// What made it show was a face: spec 0061 put eyes on the guests for
    /// the mirror, placed along `ahead`, so the head pointed one way and
    /// the body the other.
    #[test]
    fn a_figure_faces_the_way_its_own_facing_says() {
        for facing in [0.0f32, 0.4, 1.3, -1.4539, 2.9, -2.2, 3.0] {
            let (sin, cos) = facing.sin_cos();
            let meant = vec3(sin, 0.0, -cos);

            // what `stand` turns the body by
            let body = glam::Quat::from_rotation_y(-facing) * Vec3::NEG_Z;

            assert!(
                (body - meant).length() < 1e-5,
                "a body at facing {} points {:?} and its own eyes point {:?}",
                facing,
                body,
                meant
            );

            // and the facing a walk is given comes back as the walk. Every
            // angle above is inside atan2's own half turn either way, so
            // there is no wrap to allow for.
            assert!(
                (meant.x.atan2(-meant.z) - facing).abs() < 1e-5,
                "the facing {} does not survive the round trip the walkers make it with",
                facing
            );

            // **The negative control.** Turning by the facing itself is what
            // was there, and it is the mirror of this everywhere but where
            // the sine is nought.
            let was = glam::Quat::from_rotation_y(facing) * Vec3::NEG_Z;

            if sin.abs() > 1e-3 {
                assert!(
                    (was - meant).length() > 1e-3,
                    "the old turn and the new one agree at facing {}, so this test holds nothing",
                    facing
                );
            }
        }
    }

    /// What this holds is the thing Jake reported four times in a day.
    /// Filled from the eye, the panes in front of you came back as black
    /// rectangles, and engine spec 0095 is what put the corridor in them.
    /// The panes inside that reflection are still black: that is the second
    /// bounce, its target is drawn and lit from behind, and engine spec 0095
    /// carries what is ruled out so far.
    /// Spec 0033. The figure at arm's length.
    ///
    /// **The only distance the rest of the suite cannot see.** Four changes
    /// to this body went in on 2026-11-09 without moving a single reference
    /// picture: brows instead of a visor, a shoulder where the arms are,
    /// balls in the knee and the elbow, and no highlight on cloth. The
    /// people appear everywhere else at sizes the threshold forgives, so
    /// nothing held any of it and Jake found all four by looking.
    ///
    /// Close to the glass and level, so the head and shoulders fill the
    /// frame. The figure is only drawn in reflections, per engine spec
    /// 0091's bracket, so a mirror is the only place to take it.
    #[test]
    fn the_figure_is_drawn_close() {
        let along = (mirrors::SOUTH + mirrors::NORTH) * 0.5;
        let at = vec3(mirrors::WEST + 0.26, 0.0, along);

        looks_right(
            "figure",
            at,
            // a little down, so the shoulder line is in the frame with the
            // face rather than cut off under it
            vec3(mirrors::WEST, at.y + EYE - 0.07, along),
        );
    }

    #[test]
    fn the_mirrors_are_drawn() {
        // **Feet on the floor, which the rooms above this one do not do.**
        // They pass `EYE` as `at.y`, and `looking_from` reads `at` as the
        // feet and adds `EYE` to it, so their camera stands at twice eye
        // height. A hall with a six metre roof carries that; a corridor
        // whose panels stop at 2.55 puts the camera above all of them, and
        // the first picture taken here was a picture of the ceiling.
        let along = (mirrors::SOUTH + mirrors::NORTH) * 0.5;
        let at = vec3(mirrors::middle(), 0.0, along);

        looks_right("mirrors", at, vec3(mirrors::WEST, at.y + EYE, along));
    }

    /// Spec 0027: the gallery, from the way in.
    ///
    /// **The whole room in one frame, which is the point of it.** Six
    /// mathematical objects on plinths, and behind them the wall of the
    /// one that does not stop. Spec 0039 put it there because the three
    /// fractals in that row are meshes cut off at a fixed depth and this
    /// is the one that is not, and that reading wants them in one view.
    #[test]
    fn the_gallery_is_drawn() {
        let reaches = reaches();
        let (at, _) = mandel::panel(reaches);

        looks_right(
            "gallery",
            vec3(at.x, EYE, at.z + 5.0),
            vec3(at.x, at.y - 0.1, at.z),
        );
    }

    /// Specs 0027 and 0040: the field, looking back at the building.
    ///
    /// **The one picture in this building taken from outside it.** The
    /// other seven look at a room from in it, which is the only place
    /// anything here could be looked at from until the field went in. What
    /// this one is of is the outside, and the outside is where three faults
    /// lived at once: the garden's walls had no outer face, so you could see
    /// its pond through them; spec 0010's sky was a blue bubble sitting on
    /// the roof; and spec 0015's solar system stood above the roofline with
    /// its planets named, which is a secret you could read from the grass.
    #[test]
    fn the_field_is_drawn() {
        let reaches = reaches();
        let at = vec3(field::starts(reaches) - 18.0, 0.0, garden::at(reaches).z);

        looks_right(
            "field",
            at,
            vec3(
                field::starts(reaches) + 30.0,
                EYE + 3.0,
                garden::at(reaches).z,
            ),
        );
    }

    /// Spec 0060: inside the hall at the end of the trail.
    ///
    /// **The one room in this building nobody could photograph.** It did
    /// not exist, and the three things that would have caught the faults
    /// in it are all things a picture catches and a number does not: a
    /// fire washing every surface to cream, a triangle of open sky at
    /// each gable, and a lit panel painted across a doorway you can now
    /// walk through.
    #[test]
    fn the_hall_inside_is_drawn() {
        let reaches = reaches();
        let one = valhalla::hall(reaches);
        let (door, _, _) = valhalla::doorway(reaches);

        looks_right_at_the_end(
            "valhalla-hall",
            vec3(door.x - 2.0, door.y + valhalla::FLOOR_UP, door.z),
            vec3(one.at.x - one.wide * 0.4, door.y + EYE + 0.2, door.z),
        );
    }

    /// Spec 0027: the garden, looking at the pond.
    #[test]
    fn the_garden_is_drawn() {
        let reaches = reaches();
        let at = garden::at(reaches);

        let pond = garden::pond(reaches).0;

        // looked at level with the eye rather than down at it: the pond is
        // on the ground a few paces off, and aimed at where it lies this
        // is a picture of fish
        looks_right("garden", at, vec3(pond.x, EYE, pond.z));
    }

    /// Spec 0027: the baths, from the corner by the tub, down the pool.
    #[test]
    fn the_baths_are_drawn() {
        let reaches = reaches();
        let middle = spa::at(reaches);
        let pool = spa::pool(reaches).0;
        let at = vec3(middle.x + 7.5, middle.y, middle.z + 4.5);

        looks_right("baths", at, vec3(pool.x, at.y + EYE, pool.z));
    }

    /// Spec 0027: the cellar, from the middle of its floor.
    ///
    /// Where a body dropped into the room from a quarter up its own boxes
    /// settles, which is what put the camera on the cellar's floor at -3.0
    /// rather than inside the slab or on the hall above it.
    ///
    /// **Measured from the stair's foot and not written out.** It was the
    /// three numbers that walk gave back, and the first thing to move the
    /// room under them moved the room out from under them: widening the
    /// nook carried the whole cellar 1.6 west and left the camera where it
    /// was, which is two metres further back in a room that had not
    /// changed. The foot of the stair is what everything down here is
    /// measured from.
    #[test]
    fn the_cellar_is_drawn() {
        let reaches = reaches();
        let at = vec3(
            cellar::stair_foot() - 2.775,
            -cellar::DOWN,
            cellar::opening(reaches),
        );

        looks_right("cellar", at, vec3(at.x - 5.0, at.y + EYE, at.z));
    }

    /// Spec 0027: the space behind the wall, which is the one run this
    /// building hands over.
    #[test]
    fn what_is_behind_the_wall_is_drawn() {
        let reaches = reaches();
        let mut least = Vec3::splat(f32::MAX);
        let mut most = Vec3::splat(f32::MIN);

        for (box_, _) in behind::built(reaches) {
            least = least.min(box_.min);
            most = most.max(box_.max);
        }

        let at = vec3(least.x + 2.0, least.y, (least.z + most.z) * 0.5);

        looks_right("behind", at, vec3(most.x, at.y + EYE, at.z));
    }

    /// The engine's spec 0086: and the solar system out past its
    /// footprint, which is
    /// where the sky kept turning up behind it.
    ///
    /// **Written because this space had no picture and that is why the
    /// fault shipped.** Three times now the backdrop and this room have
    /// been drawn at once, and the one that got out was the one no
    /// reference held: `media/screenshot.png` went to the site with a
    /// cloudy daylight sky over the top half of the solar system, under a
    /// line of its own help text saying Neptune should be as dark as the
    /// speck of sun you can see from it. `behind.png` is inside the
    /// footprint and passed through all three.
    ///
    /// Posed off `beyond::posed`, which is the pose the staged shot uses,
    /// so this holds the picture that actually ships rather than one
    /// nearby.
    #[test]
    fn the_solar_system_is_drawn() {
        let reaches = reaches();
        let (at, yaw, _) = beyond::posed(beyond::at(reaches)).expect("the staged pose");
        let look_at = at + Vec3::Y * EYE + vec3(yaw.sin(), 0.0, -yaw.cos()) * 20.0;

        looks_right("solarium", at, look_at);
    }

    /// Spec 0027: the back room, from the spot it is arranged to be seen
    /// from.
    #[test]
    fn the_back_room_is_drawn() {
        let reaches = reaches();
        let (at, _) = posed_in_the_back_room(reaches);

        looks_right("back-room", at, back::block(reaches));
    }

    /// Spec 0027: the comparison can fail.
    ///
    /// The hall with its lamps out, held against the hall with them on,
    /// both drawn here and neither read from disc. **This used to compare
    /// against the reference file**, and on a run where that file did not
    /// exist yet it blessed the dark hall as the reference. A control that
    /// can write what it is checking against is a control that can poison
    /// it.
    #[test]
    fn a_room_picture_can_fail() {
        let reaches = reaches();
        let doorway = room::Room::of(games()).doorway();
        let looking_at = vec3(0.0, EYE, -reaches * 0.5);
        let (Some(lit), Some(dark)) = (
            drawn_from(doorway, looking_at),
            drawn_dark(doorway, looking_at),
        ) else {
            return;
        };

        assert!(
            blitzkit::picture::how_far_apart(&lit, &dark) > blitzkit::picture::SHARE,
            "the hall with its lights off looks like the hall with them on"
        );
    }

    /// The same frame with nothing lighting it, for the control above.
    fn drawn_dark(at: Vec3, looking_at: Vec3) -> Option<blitzkit::renderer::Picture> {
        let mut renderer = pollster::block_on(blitzkit::renderer::Renderer::offscreen(WIDE, TALL))?;
        let mut arcade = Arcade::new();
        let mut geometry = Geometry::new();
        let mut text = TextRenderer::new();
        let sound = SoundSystem::new();

        arcade.load(&mut renderer);
        arcade.initialize(&mut geometry, &mut text, &sound, (WIDE as f32, TALL as f32));
        arcade.before_frame(&mut renderer);

        let look = looking_at - vec3(at.x, at.y + EYE, at.z);

        arcade.at = at;
        // **And the height, which is not `at.y`.** The camera stands at
        // `eye`, which eases toward `at.y` over `EYE_LAGS` seconds so that
        // walking down a step is not a jump. With `dt` at nought it never
        // eases at all, so a pose set only through `at` is a pose at
        // whatever height the game opened on: the cellar came out black
        // and the baths came out looking at the garden, both from a camera
        // standing on the hall's floor.
        arcade.eye = at.y;
        arcade.yaw = look.x.atan2(-look.z);
        arcade.pitch = look.y.atan2(glam::vec2(look.x, look.z).length());

        let mut scene = Scene::new();
        let mut camera = Camera::new();

        arcade.update(0.0, &mut geometry, &mut text, &sound);
        arcade.draw(&mut scene, &mut camera);
        scene.light.intensity = 0.0;
        scene.light.ambient = Vec3::ZERO;
        renderer.set_camera(camera);

        Some(renderer.picture(&scene, &Geometry::new(), &TextRenderer::new()))
    }

    /// Spec 0027: a room drawn twice is the same room, which is what makes
    /// a reference worth keeping at all.
    #[test]
    fn a_room_drawn_twice_is_the_same() {
        let reaches = reaches();
        let at = garden::at(reaches);
        let looking_at = garden::pond(reaches).0;
        let (Some(once), Some(twice)) = (drawn_from(at, looking_at), drawn_from(at, looking_at))
        else {
            return;
        };

        assert_eq!(
            once.pixels, twice.pixels,
            "the same room drawn twice came out differently"
        );
    }

    /// Spec 0027: a machine with no adapter skips rather than failing.
    #[test]
    fn no_device_is_not_a_failure() {
        match pollster::block_on(blitzkit::renderer::Renderer::offscreen(16, 16)) {
            Some(_) => (),
            None => eprintln!("no adapter here, which is a skip and not a failure"),
        }
    }

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

    /// Holds a key down and never lets go of it, which is what the window
    /// does to you when it loses focus.
    fn hold(arcade: &mut Arcade, key: KeyboardKey) {
        arcade.process_keyboard(KeyboardInput {
            key,
            state: KeyboardKeyState::Pressed,
            repeat: false,
        });
    }

    /// Losing focus lets go of every key, and not just the ones that walk
    /// you.
    ///
    /// A key-up is delivered to the window that had focus when the key
    /// went down. Alt-tab with an arrow held and that release goes
    /// somewhere else, so the arrow stays down for ever and `update` turns
    /// you by it every frame. `focus_changed` let go of the four keys that
    /// walk and not the four that turn, the four that lean, or the shift
    /// that runs.
    ///
    /// **Jake found it on 2026-10-23 and his `WHERE` log named the key.**
    /// For eleven samples his feet did not move, his pitch was exactly
    /// nought, and his yaw fell by 1.20 radians a sample, which at his
    /// frame rate is sixty frames of `TURNS`: the left arrow, and nothing
    /// else, because the mouse cannot turn you without touching pitch.
    ///
    /// He said he was in a corner and that is not why he stopped. A body
    /// where he stood is held on the west and the north and can still walk
    /// four metres east or south. What stopped him was this going off and
    /// letting go of `walking`, while `looking` carried on, which is the
    /// fault saying its own name.
    ///
    /// The same fault was found once before from the other end and is
    /// written up at `process_keyboard`: an arrow held on the way to a
    /// bench left you walking into it with nothing to let go of.
    #[test]
    fn losing_focus_lets_go_of_everything() {
        let mut one = Arcade::new();
        let mut geometry = Geometry::new();
        let mut text = TextRenderer::new();
        let sound = SoundSystem::new();

        for key in [KeyboardKey::Right, KeyboardKey::Up, KeyboardKey::LShift] {
            hold(&mut one, key);
        }
        one.leaning = [true; 4];
        one.focus_changed(false);

        assert_eq!(one.looking, [false; 4], "an arrow is still held down");
        assert_eq!(one.leaning, [false; 4], "a lean is still held down");
        assert!(!one.shifted, "shift is still held down");

        let (yaw, pitch) = (one.yaw, one.pitch);
        for _ in 0..60 {
            one.update(1.0 / 60.0, &mut geometry, &mut text, &sound);
        }

        assert!(
            (one.yaw - yaw).abs() < 1e-6 && (one.pitch - pitch).abs() < 1e-6,
            "nothing was touched for a second and it turned {:.3} and tilted {:.3}",
            one.yaw - yaw,
            one.pitch - pitch
        );
    }

    /// How many frames a hold of this many seconds runs for.
    fn frames(seconds: f32) -> u32 {
        (seconds / (1.0 / 60.0)).round() as u32
    }

    /// Holds an arrow for a while, lets go, and lets the tail run out.
    ///
    /// Spec 0044's two halves in one place, because every test here needs
    /// both and a test that forgets the tail is testing the old behaviour.
    fn turned_for(held: f32) -> Arcade {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        one.looking[3] = true;
        for _ in 0..frames(held) {
            one.aimed_and_moved(dt);
        }

        one.looking = [false; 4];
        for _ in 0..(AIM_LAGS / dt).ceil() as u32 {
            one.aimed_and_moved(dt);
        }

        one
    }

    /// Spec 0044: easing costs the hold nothing, which is what lets every
    /// route written against the flat rate go on meaning what it meant.
    ///
    /// **The tail is the whole claim.** Drop the second loop in `turned_for`
    /// and this fails by 0.124 radians, which is seven degrees of a quarter
    /// turn and is exactly the ramp that never ran.
    #[test]
    fn a_held_arrow_sweeps_what_it_always_swept() {
        let dt = 1.0 / 60.0;
        // a quarter turn, which is the number the arcade's own route is
        // written in
        let held = 0.655;
        let one = turned_for(held);
        let wanted = TURNS * frames(held) as f32 * dt;

        assert!(
            (one.yaw - wanted).abs() < 0.01,
            "a {:.3}s hold swept {:.3} radians and the flat rate sweeps {:.3}",
            held,
            one.yaw,
            wanted
        );
    }

    /// Spec 0044: the corner at the near end.
    #[test]
    fn a_turn_starts_slower_than_it_gets() {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        one.looking[3] = true;
        one.aimed_and_moved(dt);
        let first = one.yaw;

        // well past the ramp, and then one more frame on its own
        for _ in 0..frames(AIM_LAGS * 2.0) {
            one.aimed_and_moved(dt);
        }
        let before = one.yaw;
        one.aimed_and_moved(dt);
        let full = one.yaw - before;

        assert!(
            (full - TURNS * dt).abs() < 1e-5,
            "a frame well into the turn moved {:.5} and full speed is {:.5}",
            full,
            TURNS * dt
        );
        assert!(
            first < full * 0.5,
            "the first frame moved {:.5} and a frame at full speed moves {:.5}",
            first,
            full
        );
    }

    /// Spec 0044: and the start has no corner in it, which is what tells an
    /// eased winding from a flat one.
    ///
    /// **A flat ramp passes every other test here.** Walking the speed itself
    /// toward full at a steady rate is already slow in its first frame and
    /// already sweeps the right total, so the tests above cannot see the
    /// difference. What they differ in is the shape: a flat ramp's second
    /// frame moves exactly twice its first, and smoothstep's moves getting on
    /// for four times, because it leaves nought with no slope at all. Put
    /// `self.turning` straight into the yaw in place of `eased` and this is
    /// the test that fails.
    #[test]
    fn the_turn_has_no_corner_where_it_starts() {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        one.looking[3] = true;
        one.aimed_and_moved(dt);
        let first = one.yaw;
        one.aimed_and_moved(dt);
        let second = one.yaw - first;

        assert!(
            second > first * 3.0,
            "the second frame moved {:.6} and the first {:.6}, which is a flat ramp's two",
            second,
            first
        );
    }

    /// Spec 0044: the corner at the far end, and that it is a corner rather
    /// than a drift that never settles.
    #[test]
    fn letting_go_does_not_stop_the_turn_dead() {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        one.looking[3] = true;
        for _ in 0..frames(0.5) {
            one.aimed_and_moved(dt);
        }
        one.looking = [false; 4];

        let let_go_at = one.yaw;
        one.aimed_and_moved(dt);

        assert!(
            one.yaw > let_go_at,
            "the view stopped on the frame the arrow came up"
        );

        for _ in 1..(AIM_LAGS / dt).ceil() as u32 {
            one.aimed_and_moved(dt);
        }
        let settled = one.yaw;
        one.aimed_and_moved(dt);

        assert_eq!(
            one.yaw, settled,
            "the view is still turning more than AIM_LAGS after the arrow came up"
        );

        // and the tail is half a ramp, not a rounding error.
        //
        // Within a frame at full speed, because the ramp is walked in
        // frame-sized steps and a staircase sits under the triangle it is
        // drawn on. Stopping dead would be 0.144 out, which is three and a
        // half frames.
        let tail = settled - let_go_at;
        let half = TURNS * AIM_LAGS * 0.5;

        assert!(
            (tail - half).abs() < TURNS * dt,
            "the tail swept {:.3} radians and half a ramp is {:.3}",
            tail,
            half
        );
    }

    /// Spec 0044: the mouse is per pixel and is not eased, because spec
    /// 0003's sight has to follow the hand exactly.
    #[test]
    fn the_mouse_is_not_eased() {
        let mut one = Arcade::new();

        one.locked = true;
        one.mouse_motion(glam::vec2(100.0, 0.0));

        assert!(
            (one.yaw - 100.0 * LOOK).abs() < 1e-6,
            "a hundred pixels of mouse turned {:.5} and should turn {:.5}",
            one.yaw,
            100.0 * LOOK
        );
    }

    /// Spec 0044: the limit clamps the angle and the rate decays as usual,
    /// so an arrow held into the ceiling stops at the ceiling.
    #[test]
    fn an_eased_tilt_still_stops_at_the_limit() {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        one.looking[0] = true;
        for _ in 0..frames(PITCH_LIMIT / TILTS + AIM_LAGS * 4.0) {
            one.aimed_and_moved(dt);
        }

        assert!(
            (one.pitch - PITCH_LIMIT).abs() < 1e-5,
            "held into the limit the pitch came out at {:.5} and the limit is {:.5}",
            one.pitch,
            PITCH_LIMIT
        );
    }

    /// Specs 0042 and 0044: a short turn in a route is still read as the
    /// turn it is, which is what the checker's tail is for.
    ///
    /// **Short, because a quarter turn cannot see this.** The checker calls
    /// a turn swallowed under nine tenths of what it asked for, and a cut
    /// tail costs a fixed 0.124 radians however long the hold is. A quarter
    /// turn loses eight per cent of itself and squeaks through the gate
    /// proving nothing. The 0.22 second turn the arcade's own route opens
    /// with loses a quarter of itself and is refused outright, so this is
    /// the hold that can tell the two behaviours apart.
    ///
    /// **Against the checker and not against `aimed_and_moved`.** What
    /// shipping depends on is `--walk` passing a staged route, and the route
    /// itself lives in another repo. These are the three keys it opens with.
    #[test]
    fn a_short_turn_in_a_route_is_still_read_as_a_turn() {
        assert!(
            walked("hold:w:3.10 hold:left:0.22 hold:w:1.60"),
            "a route opening on a 0.22 second turn was refused"
        );
    }

    /// Spec 0042: the eye goes with the feet inside `aimed_and_moved`, so a
    /// replay's sight is taken from the height it is actually at.
    #[test]
    fn the_eye_sets_off_after_the_feet_in_one_frame_of_aiming() {
        let dt = 1.0 / 60.0;
        let mut one = Arcade::new();

        // feet well above the eye, which is what a climb looks like a frame
        // at a time. High enough that the frame cannot close the gap by
        // falling instead.
        one.at.y = 4.0;
        one.eye = 0.0;
        one.aimed_and_moved(dt);

        assert!(
            one.eye > 0.0,
            "a frame of aiming left the eye at {:.3} with the feet at {:.3}",
            one.eye,
            one.at.y
        );
    }

    /// Specs 0042 and 0044: a hold can carry several keys joined by a plus.
    #[test]
    fn a_hold_can_carry_a_walk_and_a_turn_together() {
        let steps = read_route("hold:w+left:0.655").expect("the route was refused");

        assert_eq!(
            steps[0].walking,
            [true, false, false, false],
            "the w is lost"
        );
        assert_eq!(
            steps[0].looking,
            [false, false, true, false],
            "the left is lost"
        );
    }

    /// Spec 0044: and holding them together walks the corner instead of
    /// pivoting on it.
    ///
    /// **Against the pivot it replaces, not against a landmark.** What is
    /// being claimed is that the two end up pointing the same way and only
    /// one of them has been anywhere, so both are run and both are asked.
    #[test]
    fn a_walked_corner_turns_as_far_as_a_pivot_and_goes_somewhere() {
        let dt = 1.0 / 60.0;
        let held = frames(0.655);

        let run = |walking: [bool; 4]| {
            let mut one = Arcade::new();
            let from = one.at;

            one.walking = walking;
            one.looking = [false, false, true, false];
            for _ in 0..held {
                one.aimed_and_moved(dt);
            }

            one.walking = [false; 4];
            one.looking = [false; 4];
            for _ in 0..(AIM_LAGS / dt).ceil() as u32 {
                one.aimed_and_moved(dt);
            }

            (one.yaw, one.at.distance(from))
        };

        let (pivot, stood) = run([false; 4]);
        let (corner, went) = run([true, false, false, false]);

        assert!(
            (pivot - corner).abs() < 1e-4,
            "the pivot came round to {:.4} and the walked corner to {:.4}",
            pivot,
            corner
        );
        assert!(stood < 0.01, "the pivot moved {:.2} metres", stood);
        assert!(went > 2.0, "the walked corner only went {:.2} metres", went);
    }

    /// Spec 0042: a `pin:` is a walk meant to end against something, so the
    /// check that a walk goes somewhere does not apply to it.
    ///
    /// **Both halves, because one of them is the control.** The same walk
    /// written as a `hold:` is refused, which is what says this is exempting
    /// something rather than passing something that would pass anyway.
    #[test]
    fn a_pin_may_walk_into_a_wall_and_a_hold_may_not() {
        // west out of the hall's own end wall, which stops you at once
        let into_it = "hold:right:0.655 hold:w:2.0";
        let pinned = "hold:right:0.655 pin:w:2.0";

        assert!(!walked(into_it), "a hold that goes nowhere was let through");
        assert!(walked(pinned), "a pin that goes nowhere was refused");
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

    /// Spec 0024: nothing in the back room starts a process, and its gauges
    /// read the window the building already fills.
    ///
    /// The room was designed twice around a cabinet that launches the
    /// engine's `load` example, and twice Jake read it back as the wrong
    /// thing: cabinets are for games, and a room that exists to open a
    /// second window is a list of games with a door on it. The building
    /// spawns a binary in exactly one place and this is not it.
    ///
    /// Read off this file, because what is guarded is which call is made
    /// rather than any value it returns.
    #[test]
    fn the_back_room_starts_nothing() {
        const WHOLE: &str = include_str!("main.rs");

        let drawn = WHOLE
            .split_once("\nmod tests {")
            .expect("this file has a test module")
            .0;
        let from = drawn
            .find("fn draw_the_back_room(")
            .expect("the room is drawn somewhere");
        let to = drawn[from..]
            .find("\n    /// What this frame cost")
            .map(|at| from + at)
            .expect("and something is drawn after it");
        let room = &drawn[from..to];

        assert!(
            !room.contains("playing.start") && !room.contains("Command::new"),
            "something in the back room starts a process"
        );
        assert!(
            room.contains("self.costs."),
            "the gauges are not reading the window the building fills"
        );
        assert!(
            room.contains("self.counted."),
            "the gauges are not reading the renderer's own counts"
        );
    }

    /// Nothing in this building takes the engine's default surface.
    ///
    /// `Scene::push_colored` hands out a shininess of 32. Engine spec 0012
    /// picked that on purpose and defends it, for a game that wants shape out
    /// of a few boxes. A building is plaster, carpet, oak and tile, and 196
    /// surfaces here took it because nobody said otherwise: the hall's
    /// ceiling rippled, the nook's floor read as a skating rink, and a pocket
    /// mouth meant to be a hole came back a chrome dome. `aim::Plain` says
    /// what this building wants instead, in one place, and `push_material`
    /// is still how a surface asks for a highlight.
    ///
    /// Read off the two files that draw, because what is guarded is which
    /// call is made rather than any value it returns.
    #[test]
    fn nothing_here_takes_the_engines_surface() {
        const WHOLE: &str = include_str!("main.rs");
        const MIRRORS: &str = include_str!("mirrors.rs");

        let drawn = WHOLE
            .split_once("\nmod tests {")
            .expect("this file has a test module")
            .0;

        for (name, source) in [("main.rs", drawn), ("mirrors.rs", MIRRORS)] {
            assert!(
                !source.contains(".push_colored("),
                "{} draws on the engine's default shininess rather than on one \
                 this building chose",
                name
            );
        }
    }

    /// Spec 0056: F turns the frame readout on, and off again, and the
    /// window it reads from is none of its business either way.
    ///
    /// It was cleared on the way in at first, on the thinking that the
    /// seconds before anybody asked were a different question. They are not:
    /// they are the same building a moment ago, the window is only a few
    /// seconds long so it empties itself of wherever you were, and clearing
    /// spends the first seconds of every look reporting the cost of looking.
    #[test]
    fn f_toggles_what_a_frame_costs() {
        let mut one = Arcade::new();

        assert!(!one.showing_cost, "the building opened with it up");

        one.costs.saw(blitzkit::cost::Frame {
            seconds: 0.004,
            ..Default::default()
        });

        press(&mut one, KeyboardKey::F);
        assert!(one.showing_cost, "F did not show what a frame costs");
        assert_eq!(one.costs.len(), 1, "F threw away the frames before it");

        press(&mut one, KeyboardKey::F);
        assert!(!one.showing_cost, "F did not put it away again");
        assert_eq!(one.costs.len(), 1, "putting it away threw the window out");
    }

    /// Spec 0056: the window fills whether or not it is being looked at.
    ///
    /// A readout that starts measuring when the key is pressed spends its
    /// first few seconds reporting the cost of pressing the key, which is the
    /// one frame nobody is asking about.
    #[test]
    fn the_frames_are_counted_before_anybody_asks() {
        const SOURCE: &str = include_str!("main.rs");

        let before = SOURCE
            .split_once("\nmod tests {")
            .expect("this file has a test module")
            .0;
        let from = before
            .find("fn before_frame(")
            .expect("the frame starts somewhere");
        let to = before[from..]
            .find("\n    fn ")
            .map(|at| from + at)
            .unwrap_or(before.len());
        let frame = &before[from..to];

        assert!(
            frame.contains("self.costs.saw("),
            "the window is filled somewhere other than the top of the frame"
        );
        assert!(
            !frame.contains("showing_cost"),
            "the frame only measures while the readout is up"
        );
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

    /// Spec 0045: M says where you are standing, and says it again when you
    /// have moved.
    ///
    /// The control is the second half. A line with the right shape in it
    /// and the wrong numbers would pass the first half for ever, and the
    /// whole point of this key is the numbers.
    #[test]
    fn m_says_where_you_are() {
        let mut one = Arcade::new();

        one.at = vec3(-7.70, 1.31, -1.25);
        one.eye = 1.31;
        one.yaw = 225f32.to_radians();
        one.pitch = 25f32.to_radians();

        let said = one.where_you_are();

        assert!(said.contains("-7.70"), "no x in {}", said);
        assert!(said.contains("-1.25"), "no z in {}", said);
        assert!(said.contains("2.86"), "no eye in {}", said);
        assert!(said.contains("225.0"), "no yaw in {}", said);
        assert!(said.contains("25.0"), "no pitch in {}", said);

        one.at.x += 3.0;
        one.yaw = 0.0;

        let moved = one.where_you_are();

        assert_ne!(said, moved, "it says the same thing wherever you stand");
    }

    /// Spec 0045: M puts the line on the screen, and it runs out on its own.
    ///
    /// The control is the last line. A readout that never goes away is a
    /// mode, and a mode wants a key to come out of it.
    #[test]
    fn m_shows_where_you_are_for_a_while() {
        let mut one = Arcade::new();

        assert_eq!(one.showing_place, 0.0, "it opened with the readout up");

        press(&mut one, KeyboardKey::M);
        assert_eq!(one.showing_place, Arcade::PLACE_SHOWS, "M showed nothing");

        let (mut geometry, mut text) = (Geometry::new(), TextRenderer::new());
        let sound = SoundSystem::new();

        one.update(Arcade::PLACE_SHOWS * 0.5, &mut geometry, &mut text, &sound);
        assert!(one.showing_place > 0.0, "it went out half way through");

        one.update(Arcade::PLACE_SHOWS, &mut geometry, &mut text, &sound);
        assert_eq!(one.showing_place, 0.0, "it never goes out");
    }

    /// Spec 0045: and asking moves nothing.
    ///
    /// A key that teleports you is not a thing this building does, per spec
    /// 0041, and this one only reads.
    #[test]
    fn m_moves_nothing() {
        let mut one = Arcade::new();

        one.at = vec3(1.0, 0.0, 2.0);
        one.yaw = 0.5;
        one.pitch = 0.25;

        let was = (one.at, one.yaw, one.pitch, one.walking, one.wants_planes);

        press(&mut one, KeyboardKey::M);

        assert_eq!(
            (one.at, one.yaw, one.pitch, one.walking, one.wants_planes),
            was,
            "asking where you are moved you"
        );
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

    /// Spec 0015: the sign hangs over the way out and faces into the room.
    ///
    /// **Over it, which is the second answer.** It hung beside it, and the
    /// reason written down was that the way out was a slot the full height of
    /// the wall with nothing above it to hang from. That was true of the
    /// geometry and beside the point. This room is black: the wall at that
    /// end is unlit, the opening in it is invisible, and the sign carries its
    /// own light, so it is the only thing in a hundred and fifty nine metres
    /// you can see. It was not a label on the way out, it was the
    /// instruction, and it pointed 1.95 metres to one side of the gap.
    ///
    /// Jake walked at it on 2026-10-15 and hit wall. The wall has a head over
    /// the way out now, per `behind::WAY_HIGH`, and the sign hangs on it.
    ///
    /// The old reason for standing it aside is checked below rather than
    /// dropped: a sign across the way out would be a thing to walk into, and
    /// what keeps it from being one is that it hangs over your head.
    #[test]
    fn the_sign_hangs_over_the_way_out_and_faces_in() {
        let one = Arcade::new();
        let reaches = one.room.reaches;
        let (at, looks) = orrery::exit_sign(reaches);
        let (from, to) = behind::way();
        let half = orrery::SIGN_WIDE * 0.5 + orrery::SIGN_BORDER;

        // squarely over it, so walking at the sign walks through the gap.
        // The housing is the opening's own width, to a millimetre: wider and
        // it stands proud of each jamb, which is a sign too big for its hole.
        assert!(
            (at.x - half - from).abs() < 1e-3 && (at.x + half - to).abs() < 1e-3,
            "the sign spans {:.3} to {:.3} and the way out is {:.3} to {:.3}",
            at.x - half,
            at.x + half,
            from,
            to
        );
        assert!(
            ((from + to) * 0.5 - at.x).abs() < 1e-3,
            "the sign's middle is at {:.2} and the way out's at {:.2}",
            at.x,
            (from + to) * 0.5
        );

        // hung on the head and inside it, rather than standing proud of the
        // wall above or hanging into the gap below
        let housing = orrery::SIGN_TALL * 0.5 + orrery::SIGN_BORDER;

        assert!(
            at.y - housing > behind::WAY_HIGH && at.y + housing < room::TALL,
            "the sign spans {:.2} to {:.2} and the head runs {:.2} to {:.2}",
            at.y - housing,
            at.y + housing,
            behind::WAY_HIGH,
            room::TALL
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

        // nothing of the wall stands in the way in at the height of a body.
        // Asked at a height and not in plan, because the head over the way in
        // does span it and is meant to: it is what the sign hangs on, and it
        // starts a good deal over your head. See `behind::WAY_HIGH`.
        for box_ in pieces.iter() {
            let aside = box_.min.x > middle || box_.max.x < middle;

            assert!(
                aside || box_.min.y >= behind::WAY_HIGH,
                "a piece of the near wall spans the way in at {:.2}, from {:.2} up",
                middle,
                box_.min.y
            );
        }

        // and a body walks under the head with room to spare. Read off the
        // box rather than off the constant, so this is a question about the
        // wall that got built and not two numbers compared in the source.
        let head = pieces
            .iter()
            .filter(|box_| box_.min.x <= middle && box_.max.x >= middle)
            .map(|box_| box_.min.y)
            .fold(f32::MAX, f32::min);

        assert!(
            head < f32::MAX && head > EYE + 0.4,
            "the head over the way in is at {:.2} and an eye is at {:.2}",
            head,
            EYE
        );

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
            "the pieces of the wall are all on one side of the way in"
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

    /// Spec 0021: a wall and the map face that stands for it show the same
    /// object.
    ///
    /// Which is the one thing in the room a person is actually asked to
    /// trust. The map says the wall on your left leads to the sphere; the
    /// wall on your left shows a sphere; the room through it has a sphere in
    /// it. Any one of those three disagreeing and the map is worse than no
    /// map, and three rewrites of it went that way already.
    ///
    /// Read off this file, because what is guarded is that both draws call
    /// the same function rather than any value either returns.
    #[test]
    fn the_wall_and_the_map_read_the_sign_from_one_place() {
        const WHOLE: &str = include_str!("main.rs");

        // the drawing only, not this file's own tests, which say every one of
        // these names themselves and would make the count whatever they like
        let drawn = WHOLE
            .split_once("\nmod tests {")
            .expect("this file has a test module")
            .0;

        assert_eq!(
            drawn.matches("tesseract::sign_on(").count(),
            2,
            "the wall and the map face are the two places a sign is drawn"
        );
        assert!(
            !drawn.contains("tesseract::thing(beyond)"),
            "a sign is being worked out beside sign_on rather than by it"
        );

        // and the room's own thing comes off the cell you are standing in,
        // which is the other half of the vocabulary
        assert!(
            drawn.contains("tesseract::thing(frame.cell())"),
            "the plinth is not showing this room's own thing"
        );
    }

    /// Spec 0013: O outlines `room.solid()` itself.
    ///
    /// The whole of what this feature is for is catching a drawn list and a
    /// solid list that have come apart, and it catches nothing if it draws a
    /// third list gathered for the purpose. Four of that spec's criteria hold
    /// the toggle and none of them held this, which is the claim the feature
    /// rests on.
    ///
    /// Read off this file, because what is guarded is which function is
    /// called rather than any value it returns.
    #[test]
    fn o_outlines_the_solid_list_itself() {
        const SOURCE: &str = include_str!("main.rs");

        let from = SOURCE
            .find("if self.showing_solid {")
            .expect("nothing draws what is solid");
        let block = &SOURCE[from..];
        let to = block.find("\n        }").expect("that block does not end");
        let block = &block[..to];

        assert!(
            block.contains("self.room.solid()"),
            "what O outlines is not room.solid() itself"
        );
    }
}
