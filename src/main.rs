//! arcade: a room of cabinets, one per game. See `specs/`.

mod aim;
mod cabinet;
mod carpet;
mod cascada;
mod cellar;
mod cradle;
mod display;
mod globe;
mod gyro;
mod metronome;
mod room;
mod sign;
mod study;
mod walk;
mod wrecker;

use blitzkit::camera::Camera;
use blitzkit::collision::Aabb;
use blitzkit::geometry::Geometry;
use blitzkit::keyboard::{KeyboardInput, KeyboardKey, KeyboardKeyState};
use blitzkit::mesh::{MeshData, Transform};
use blitzkit::mouse::{MouseButton, MouseInput};
use blitzkit::renderer::render_text::{RenderText, TextRenderer};
use blitzkit::renderer::scene::{MeshId, Scene, TextureId};
use blitzkit::renderer::Renderer;
use blitzkit::sound::SoundSystem;
use blitzkit::texture::TextureData;
use blitzkit::{start, Game};
use cabinet::{Cabinet, Playing};
use glam::{vec2, vec3, vec4, Vec2, Vec3};
use room::Room;

/// Where you stand, how fast, and how far you see.
const RADIUS: f32 = 0.45;
const EYE: f32 = 1.55;

/// How long the eye takes to catch the feet up, in seconds.
///
/// Short enough that it is not a thing you notice on the level, where there is
/// nothing to catch up with anyway, and long enough to turn fourteen quarter
/// unit drops a second into going down a slope.
const EYE_LAGS: f32 = 0.09;
const SPEED: f32 = 4.2;
const LOOK: f32 = 0.0022;
const PITCH_LIMIT: f32 = 1.3;

/// What the toy on the bench falls under.
const GRAVITY: Vec3 = vec3(0.0, -9.81, 0.0);

/// How wide the gyroscope's ring is across the axle, which is wider than it is
/// deep so it reads as a hoop rather than a wire.
const GIMBAL_WIDE: f32 = 0.022;

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
fn staged() -> bool {
    std::env::args().any(|arg| arg == "--screenshot")
}

/// Where the camera stands for a staged shot, and which way it looks.
///
/// Spec 0003 made the screens go dark until you are within reach of one, so
/// the view from the doorway is now a dark room with nothing answering. The
/// picture has to be taken from where the room is doing something: a step back
/// from a cabinet, with it lit and named and offering its key.
/// Aimed at a cabinet rather than at a bearing. It was a bearing, and adding a
/// thirteenth game moved the rows under it: the sight landed in the gap between
/// two cabinets and the picture was a wall.
const POSED_AT: Vec3 = vec3(0.0, 0.0, 3.6);
const POSED_YAW: f32 = 0.96;
const POSED_PITCH: f32 = -0.1;

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
    /// The rug in the nook, which is not the arcade's carpet.
    rug: Option<TextureId>,
    rug_mesh: Option<MeshId>,
    /// The grain on the walls and on the cabinets, so neither is a flat face.
    says: Option<TextureId>,
    point_mesh: Option<MeshId>,
    hung: Option<MeshId>,
    boards: Option<TextureId>,
    boards_mesh: Option<MeshId>,
    barrel: Option<MeshId>,
    hoop: Option<MeshId>,
    bottle: Option<MeshId>,
    peg: Option<MeshId>,
    stave: Option<TextureId>,
    flame: Option<MeshId>,
    candlestick: Option<MeshId>,
    decanter: Option<MeshId>,
    wineglass: Option<MeshId>,
    afghan: Option<TextureId>,
    grain: Vec<TextureId>,
    stonework: Option<TextureId>,
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
    walking: [bool; 4],
    /// Whether a shift key is down, which is what turns the arrows from hauling
    /// a toy about into working its one other control.
    shifted: bool,
    /// Which arrows are being held on a toy that is leaned on rather than
    /// pressed, in the same order as `walking`.
    leaning: [bool; 4],
    wants_lock: bool,
    locked: bool,
    quitting: bool,
}

impl Arcade {
    fn new() -> Self {
        let room = Room::of(games());
        let spin = display::Spin::of(room.displays.len());
        let (cradle, ropes) = cradle::strung();

        Self {
            at: if staged() { POSED_AT } else { room.doorway() },
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
            rug: None,
            rug_mesh: None,
            says: None,
            point_mesh: None,
            hung: None,
            boards: None,
            boards_mesh: None,
            barrel: None,
            hoop: None,
            bottle: None,
            peg: None,
            stave: None,
            flame: None,
            candlestick: None,
            decanter: None,
            wineglass: None,
            afghan: None,
            grain: Vec::new(),
            stonework: None,
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
            yaw: if staged() { POSED_YAW } else { 0.0 },
            pitch: if staged() { POSED_PITCH } else { 0.0 },
            walking: [false; 4],
            shifted: false,
            leaning: [false; 4],
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

        let arrow = arrow_at(input.key);
        if let Some(which) = arrow {
            // whatever the toy does with it, the arrow is not walking you, and
            // letting go always says so: an arrow held on the way to a bench
            // would otherwise leave you walking into it
            self.walking[which] = false;
            self.leaning[which] = false;
        }
        let arrow = arrow.is_some();
        if arrow && !held {
            return true;
        }
        match self.room.benches[n].name {
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
                    "Press enter to start. The weight is on notch {} of {}, about {:.0} beats a minute.",
                    self.metronome.notch + 1,
                    metronome::NOTCHES,
                    metronome::beats(self.metronome.notch)
                ),
            },
            globe::NAME => {
                if self.globe.going() {
                    String::from("Spinning. Arrows spin it faster, enter stops it.")
                } else {
                    String::from("Left and right arrows spin it.")
                }
            }
            gyro::NAME => {
                let dial = format!("Spin {} of {}", self.gyro.notch + 1, gyro::SPINS);
                if !self.gyro.going {
                    format!(
                        "Press enter to spin it up. {}, shift with up or down changes it.",
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
            wrecker::NAME => format!(
                "{} of {} standing. Swing the ball with arrow keys. Use shift with up and down keys to wind the chain up or down. Press enter to rebuild the wall.",
                self.wrecker.standing(),
                wrecker::bricks()
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
                String::from("Press enter to set it going again")
            }
            _ => String::from("Press enter to set it going"),
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

    /// Where you are looking from, which is your eye and not your feet.
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
    }

    fn load(&mut self, renderer: &mut Renderer) {
        // The sun's map covers the room and no more. The default is forty
        // across, and this room is five by fifteen, so a texel was 0.02 wide
        // and the shapes on the far wall cast smears rather than shadows: the
        // Sierpinski tetrahedron's finest face is 0.03 across, under two
        // texels.
        // the nook included. It was the aisle only, so the nook stood outside
        // the sun's map entirely and nothing in it cast anything.
        renderer.set_scene_bounds(Aabb::from_center_size(
            vec3(-room::NOOK_DEEP * 0.5, room::TALL * 0.5, 0.0),
            vec3(
                (room::WALL + room::CABINET.x) * 2.0 + room::NOOK_DEEP,
                room::TALL,
                self.room.reaches * 2.0,
            ),
        ));

        self.cube = Some(renderer.add_mesh(&MeshData::cube()));
        self.floor = Some(renderer.add_mesh(&room::tiled_floor(carpet::TILES)));
        self.ceiling_mesh = Some(renderer.add_mesh(&MeshData::plane()));
        self.carpet = Some(renderer.add_texture(&carpet::woven()));
        let (_, rug) = room::open_floor(self.room.reaches);
        self.rug = Some(renderer.add_texture(&carpet::rug(rug.z / rug.x)));
        self.boards = Some(renderer.add_texture(&carpet::boards(carpet::BOARD_SEED)));
        self.barrel = Some(renderer.add_mesh(&cellar::barrel_mesh()));
        self.hoop = Some(renderer.add_mesh(&cellar::hoop_mesh()));
        self.bottle = Some(renderer.add_mesh(&cellar::bottle_mesh()));
        self.peg = Some(renderer.add_mesh(&cellar::peg_mesh()));
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
        self.stonework = Some(renderer.add_texture(&carpet::stonework(carpet::STONE_SEED)));
        self.stave =
            Some(renderer.add_texture(&carpet::staves(carpet::STAVE_SEED, cellar::STAVES)));
        self.says = Some(renderer.add_texture(&blitzkit::text::drawn(sign::SAYS, sign::TEXELS)));
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
    }

    fn resized(&mut self, window_size: (f32, f32)) {
        self.window = window_size;
    }

    fn update(
        &mut self,
        dt: f32,
        _geometry: &mut Geometry,
        text: &mut TextRenderer,
        _sound: &SoundSystem,
    ) {
        self.since += dt;

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

        // along the floor, over anything no taller than a step, and down.
        // Spec 0007: the building has a height in it now, so getting about is
        // no longer one call that pushes you sideways.
        let (at, falling) = walk::walk(
            self.at,
            if wish.length_squared() > 1e-6 {
                wish.normalize() * SPEED
            } else {
                Vec3::ZERO
            },
            self.falling,
            RADIUS,
            dt,
            &self.room.solid(),
        );
        self.at = at;
        self.falling = falling;

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
                        "Press enter to play"
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
                    "Click to let go"
                } else {
                    "Click to turn it"
                })),
            ),
            (None, Some(room::Seen::Bench(n))) => {
                let name = self.room.benches[n].name;
                (Some(name.to_string()), Some(self.about(name)))
            }
            (None, Some(room::Seen::Case)) => (
                Some(String::from("a book")),
                Some(String::from(if self.room.open {
                    "Push it back."
                } else {
                    "This one is not a book. Pull it."
                })),
            ),
            (None, None) => (None, None),
        };

        text.reset();

        // the corner says how to move, and nothing else. What to press is under
        // the sight, and saying it here as well is saying it twice.
        text.push_render_text(RenderText {
            position: vec2(20.0, 20.0),
            text: String::from(
                "WASD and the mouse to get about. Arrow keys work the toy you are looking at. Escape quits.",
            ),
            size: 14.0,
            ..Default::default()
        });

        // and the wink, once, while the sight is on any of the five. Spec 0004:
        // it was under every shape, five times, which is five times too many
        // for a joke.
        if matches!(self.seen, Some(room::Seen::Display(_))) || self.spin.holding().is_some() {
            text.push_render_text(RenderText {
                position: vec2(20.0, 44.0),
                text: String::from("If you know, you know. If you don't, play with one."),
                size: 14.0,
                ..Default::default()
            });
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

        for (line, say, size) in vec![(0, name, aim::PROMPT_SIZE), (1, detail, aim::DETAIL_SIZE)]
            .into_iter()
            .filter_map(|(line, say, size)| say.map(|say| (line, say, size)))
        {
            let at = aim::prompt_at(self.window, line);

            // its own shadow first, since it lands on whatever the sight just
            // lit and that is the brightest thing in the room
            text.push_render_text(RenderText {
                position: aim::shadow_at(at),
                bounds: aim::prompt_bounds(self.window),
                text: say.clone(),
                size,
                color: aim::SHADOW_COLOUR,
                centered: true,
                ..Default::default()
            });
            text.push_render_text(RenderText {
                position: at,
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

        // dim and overhead, like the room it is: bright enough to walk, dark
        // enough that a lit cabinet is the thing you look at. Warm from above
        // and cool in what it misses, because a white sun over a neutral fill
        // is what made every surface in here grey.
        scene.light.color = aim::SUN;
        scene.light.intensity = aim::SUN_STRENGTH;
        scene.light.ambient = aim::FILL;

        // wide enough to reach under the nook as well as the aisle, and shifted
        // to cover it. One quad rather than two: the weave's tile count is
        // baked into the mesh's uvs, so a second quad of a different size would
        // lay a carpet of a different scale beside the first.
        let across = (room::WALL + room::CABINET.x) * 2.0 + room::NOOK_DEEP;
        let along = self.room.reaches * 2.0;
        let laid = Transform::at(vec3(-room::NOOK_DEEP * 0.5, 0.0, 0.0))
            .with_scale(vec3(across, 1.0, along));

        match self.carpet {
            // matte. A low shininess is a specular highlight spread over the
            // whole floor, which washed the weave's dark ground out to pale
            // grey the first time this was laid.
            Some(carpet) => scene.push_textured(floor, carpet, &laid, aim::FLOOR, aim::MATTE),
            None => scene.push_colored(floor, &laid, aim::FLOOR),
        }

        // a ceiling, because the room opened onto nothing and a corridor with
        // no lid is a corridor you are standing outside of
        if let Some(ceiling) = self.ceiling_mesh {
            // turned over, because a plane faces up and from underneath that is
            // a back face, which the opaque pass culls
            // shifted with the carpet, not centred on the aisle. It was wide
            // enough to cover the nook and sitting a nook's depth away from it,
            // so from inside the nook half the ceiling was open sky.
            scene.push_colored(
                ceiling,
                &Transform::at(vec3(-room::NOOK_DEEP * 0.5, room::TALL, 0.0))
                    .with_rotation(glam::Quat::from_rotation_x(std::f32::consts::PI))
                    .with_scale(vec3(across, 1.0, along)),
                aim::CEILING,
            );
        }

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

            scene.push_colored(
                cube,
                &Transform::at(top + Vec3::Y * (metronome::FOOT.y * 0.5))
                    .with_scale(metronome::FOOT),
                aim::CASE,
            );
            scene.push_colored(
                cube,
                &Transform::at(
                    top + vec3(
                        -metronome::PLATE_BACK,
                        metronome::FOOT.y + metronome::PLATE_TALL * 0.5,
                        0.0,
                    ),
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
                        top + vec3(
                            -metronome::PLATE_BACK + metronome::PLATE_THICK,
                            up,
                            metronome::NOTCH_AT,
                        ),
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
            let links = sign::links(room::TALL);
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
            let stem = (room::TALL + shade.y + sign::SHADE.y * 0.5) * 0.5;
            scene.push_colored(
                cube,
                &Transform::at(vec3(shade.x, stem, shade.z)).with_scale(vec3(
                    0.016,
                    room::TALL - shade.y - sign::SHADE.y * 0.5,
                    0.016,
                )),
                aim::SIGN_CHAIN,
            );
        }

        // the way down, and the room at the bottom of it. Spec 0007. Boxes, so
        // they are drawn as the boxes they are: a stair of slabs is what a
        // stair looks like from the side anyway.
        for (box_, made) in cellar::built(self.room.reaches) {
            scene.push_colored(
                cube,
                &Transform::at(box_.center()).with_scale(box_.size()),
                match made {
                    cellar::Made::Tread => aim::BOARDS,
                    // the floor warmer and a shade apart from the walls, so a
                    // room is a floor and walls rather than one grey box
                    cellar::Made::Stone if box_.max.y <= -cellar::DOWN + 1e-3 => aim::CELLAR_FLOOR,
                    cellar::Made::Stone => aim::CELLAR,
                },
            );
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
            let round = cellar::FIRE_ROUND;
            let out = room::THICK;

            // the surround: two jambs and a lintel, standing out of the wall
            for (middle, size) in [
                (
                    vec3(
                        at.x + out * 0.5,
                        at.y + high * 0.5,
                        at.z - (wide + round) * 0.5,
                    ),
                    vec3(out, high + round, round),
                ),
                (
                    vec3(
                        at.x + out * 0.5,
                        at.y + high * 0.5,
                        at.z + (wide + round) * 0.5,
                    ),
                    vec3(out, high + round, round),
                ),
                (
                    vec3(at.x + out * 0.5, at.y + high + round * 0.5, at.z),
                    vec3(out, round, wide + round * 2.0),
                ),
                // and a mantel over the lot, placed from the same function
                // that the things standing on it are, so the shelf and what is
                // on it cannot disagree about where it is
                (
                    cellar::mantel_top(self.room.reaches) - Vec3::Y * cellar::MANTEL * 0.3,
                    vec3(
                        out + cellar::MANTEL,
                        cellar::MANTEL * 0.6,
                        wide + round * 3.0,
                    ),
                ),
            ] {
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
                let rail = aim::RAIL;
                let frame = if lit { aim::BENCH_ON } else { aim::TRIM };

                // the cloth, inside the rails, with its face at the top
                scene.push_colored(
                    cube,
                    &Transform::at(bench.at + Vec3::Y * (top - rail * 0.5)).with_scale(vec3(
                        bench.size.x - rail * 2.0,
                        rail,
                        bench.size.z - rail * 2.0,
                    )),
                    aim::CLOTH,
                );

                // the four rails round it. Each sits out at its own axis's half
                // width, which is the thing this got wrong: taken from the
                // other axis they stand at twice the table's width, out in the
                // room, and lie across the cloth on the way.
                let high = rail * 1.8;
                for (way, out, span) in [
                    (Vec3::X, bench.size.x, vec3(rail, high, bench.size.z)),
                    (Vec3::Z, bench.size.z, vec3(bench.size.x, high, rail)),
                ] {
                    for side in [-1.0f32, 1.0] {
                        let at = bench.at
                            + way * side * (out - rail) * 0.5
                            + Vec3::Y * (top - high * 0.5);

                        scene.push_colored(cube, &Transform::at(at).with_scale(span), frame);
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
        // nearest first, because what the engine drops when a building outgrows
        // it should be the lamp in the furthest room and not whichever was
        // pushed last. Not a ration any more: every fitting in the building is
        // pushed and the engine carries sixty four of them, where it carried
        // eight and the nook's walls took six of those whether or not you were
        // standing in the nook. Spec 0020 of the engine.
        shades.sort_by(|one, other| one.0.total_cmp(&other.0));

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

        match input.key {
            KeyboardKey::W | KeyboardKey::Up => self.walking[0] = held,
            KeyboardKey::S | KeyboardKey::Down => self.walking[1] = held,
            KeyboardKey::A | KeyboardKey::Left => self.walking[2] = held,
            KeyboardKey::D | KeyboardKey::Right => self.walking[3] = held,
            KeyboardKey::Return if held => {
                self.use_what_i_see();
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
    start("arcade", Box::new(Arcade::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

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
