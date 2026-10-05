//! arcade: a room of cabinets, one per game. See `specs/`.

mod aim;
mod cabinet;
mod carpet;
mod display;
mod room;

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
const SPEED: f32 = 4.2;
const LOOK: f32 = 0.0022;
const PITCH_LIMIT: f32 = 1.3;

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
const POSED_AT: Vec3 = vec3(0.0, 0.0, 2.45);
const POSED_YAW: f32 = 1.0;
const POSED_PITCH: f32 = -0.1;

struct Arcade {
    room: Room,
    playing: Playing,
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
    /// The grain on the walls and on the cabinets, so neither is a flat face.
    wall_grain: Option<TextureId>,
    cabinet_grain: Option<TextureId>,
    ceiling_mesh: Option<MeshId>,
    screen: Option<MeshId>,
    /// The engine's own shapes, turning at the end of the room. Spec 0002.
    /// Where they stand is the room's; these are the meshes.
    shown: Vec<MeshId>,
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
    yaw: f32,
    pitch: f32,
    walking: [bool; 4],
    wants_lock: bool,
    locked: bool,
    quitting: bool,
}

impl Arcade {
    fn new() -> Self {
        let room = Room::of(games());
        let spin = display::Spin::of(room.displays.len());

        Self {
            at: if staged() { POSED_AT } else { room.doorway() },
            room,
            spin,
            playing: Playing::new(),
            art: Vec::new(),
            glows: Vec::new(),
            signs: Vec::new(),
            cube: None,
            floor: None,
            carpet: None,
            wall_grain: None,
            cabinet_grain: None,
            ceiling_mesh: None,
            screen: None,
            shown: Vec::new(),
            since: 0.0,
            seen: None,
            window: (800.0, 600.0),
            // looking down the room from the end you start at: forward is
            // (sin yaw, 0, -cos yaw), so nought faces -z
            yaw: if staged() { POSED_YAW } else { 0.0 },
            pitch: if staged() { POSED_PITCH } else { 0.0 },
            walking: [false; 4],
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
            None => (),
        }
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

/// Every game this project has, and where its binary is.
///
/// The names are the folders under `games`, read at build time rather than
/// kept in a list here: a list is a thing that goes out of date, and the
/// project already refuses to let a game exist without a folder.
fn games() -> Vec<Cabinet> {
    let beside = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|at| at.to_path_buf()))
        .unwrap_or_default();

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
        .iter()
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
        renderer.set_scene_bounds(Aabb::from_center_size(
            vec3(0.0, room::TALL * 0.5, 0.0),
            vec3(
                (room::WALL + room::CABINET.x) * 2.0,
                room::TALL,
                self.room.reaches * 2.0,
            ),
        ));

        self.cube = Some(renderer.add_mesh(&MeshData::cube()));
        self.floor = Some(renderer.add_mesh(&room::tiled_floor(carpet::TILES)));
        self.ceiling_mesh = Some(renderer.add_mesh(&MeshData::plane()));
        self.carpet = Some(renderer.add_texture(&carpet::woven()));
        self.wall_grain =
            Some(renderer.add_texture(&carpet::mottled(carpet::WALL_SEED, [220, 220, 220], 34)));
        self.cabinet_grain =
            Some(renderer.add_texture(&carpet::mottled(carpet::CABINET_SEED, [228, 228, 228], 22)));
        self.screen = Some(renderer.add_mesh(&room::screen_mesh()));
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

        if wish.length_squared() > 1e-6 {
            let body = blitzkit::collision::Sphere::new(self.at + Vec3::Y * RADIUS, RADIUS);
            self.at = blitzkit::collision::move_and_slide(
                body,
                wish.normalize() * SPEED,
                dt,
                &self.room.solid(),
            ) - Vec3::Y * RADIUS;
        }

        let playing = self.playing.now().map(|name| name.to_string());
        self.seen = self.room.looking_at(self.at + Vec3::Y * EYE, self.facing());

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
                        "Not built. Run ./check-all"
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
            (None, None) => (None, None),
        };

        text.reset();

        // the corner says how to move, and nothing else. What to press is under
        // the sight, and saying it here as well is saying it twice.
        text.push_render_text(RenderText {
            position: vec2(20.0, 20.0),
            text: String::from(
                "WASD or arrow keys to walk around. Use the mouse to look around. Press escape to quit.",
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

        camera.position = self.at + Vec3::Y * EYE;
        camera.target = camera.position + self.facing();

        // dim and overhead, like the room it is: bright enough to walk, dark
        // enough that a lit cabinet is the thing you look at. Warm from above
        // and cool in what it misses, because a white sun over a neutral fill
        // is what made every surface in here grey.
        scene.light.color = aim::SUN;
        scene.light.intensity = aim::SUN_STRENGTH;
        scene.light.ambient = aim::FILL;

        let across = (room::WALL + room::CABINET.x) * 2.0;
        let along = self.room.reaches * 2.0;
        let laid = Transform::at(Vec3::ZERO).with_scale(vec3(across, 1.0, along));

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
            scene.push_colored(
                ceiling,
                &Transform::at(Vec3::Y * room::TALL)
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
        // that throws nothing is a coloured rectangle. The engine carries
        // eight, so the nearest eight get one.
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

        for (_, n) in near.into_iter().take(aim::LAMPS) {
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

        let mut ours: Vec<String> = games().into_iter().map(|cabinet| cabinet.name).collect();
        ours.sort();

        assert!(!theirs.is_empty(), "list-repos named no games at all");
        assert_eq!(ours, theirs, "the arcade and list-repos disagree");
    }
}
