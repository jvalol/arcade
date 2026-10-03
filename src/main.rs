//! arcade: a room of cabinets, one per game. See `specs/`.

mod cabinet;
mod room;

use blitzkit::camera::Camera;
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

struct Arcade {
    room: Room,
    playing: Playing,
    /// One per cabinet, in the room's own order. A game with no screenshot gets
    /// none and its screen stays blank.
    art: Vec<Option<TextureId>>,
    cube: Option<MeshId>,
    floor: Option<MeshId>,
    screen: Option<MeshId>,

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

        Self {
            at: room.doorway(),
            room,
            playing: Playing::new(),
            art: Vec::new(),
            cube: None,
            floor: None,
            screen: None,
            // looking down the room from the end you start at: forward is
            // (sin yaw, 0, -cos yaw), so nought faces -z
            yaw: 0.0,
            pitch: 0.0,
            walking: [false; 4],
            wants_lock: true,
            locked: false,
            quitting: false,
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
        self.cube = Some(renderer.add_mesh(&MeshData::cube()));
        self.floor = Some(renderer.add_mesh(&MeshData::plane()));
        self.screen = Some(renderer.add_mesh(&room::screen_mesh()));
        self.art = self
            .room
            .stood
            .iter()
            .map(|stood| art_of(&stood.cabinet.name).map(|art| renderer.add_texture(&art)))
            .collect();
    }

    fn initialize(
        &mut self,
        _geometry: &mut Geometry,
        _text: &mut TextRenderer,
        _sound: &SoundSystem,
        _size: (f32, f32),
    ) {
    }

    fn update(
        &mut self,
        dt: f32,
        _geometry: &mut Geometry,
        text: &mut TextRenderer,
        _sound: &SoundSystem,
    ) {
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
        let at = self.room.at(self.at, self.facing());

        let saying = match (&playing, at) {
            (Some(name), _) => format!("{} is playing. Quit to come back.", name),
            (None, Some(n)) => {
                let stood = &self.room.stood[n];
                if stood.cabinet.is_built() {
                    format!("{}. Press enter to play.", stood.cabinet.name)
                } else {
                    format!(
                        "{} has not been built. Run ./check-all.",
                        stood.cabinet.name
                    )
                }
            }
            (None, None) => String::from("Walk up to a cabinet to play."),
        };

        text.reset();
        for (line, say) in vec![
            saying,
            String::from(
                "WASD or arrow keys to walk around. Use the mouse to look around. Press escape to quit.",
            ),
        ]
        .into_iter()
        .enumerate()
        {
            text.push_render_text(RenderText {
                position: vec2(20.0, 20.0 + line as f32 * 24.0),
                text: say,
                size: 14.0,
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
        // enough that a lit cabinet is the thing you look at
        scene.light.intensity = 0.55;
        scene.light.ambient = Vec3::splat(0.28);

        scene.push_colored(
            floor,
            &Transform::at(Vec3::ZERO).with_scale(vec3(
                (room::WALL + room::CABINET.x) * 2.0,
                1.0,
                self.room.reaches * 2.0,
            )),
            vec4(0.12, 0.11, 0.14, 1.0),
        );

        for wall in self.room.walls.iter() {
            scene.push_colored(
                cube,
                &Transform::at(wall.center()).with_scale(wall.size()),
                vec4(0.17, 0.16, 0.20, 1.0),
            );
        }

        let at = self.room.at(self.at, self.facing());
        for (n, stood) in self.room.stood.iter().enumerate() {
            let lit = at == Some(n) && stood.cabinet.is_built();
            let body = if stood.cabinet.is_built() {
                vec4(0.20, 0.19, 0.24, 1.0)
            } else {
                vec4(0.13, 0.13, 0.14, 1.0)
            };

            scene.push_colored(
                cube,
                &Transform::at(stood.at + Vec3::Y * room::CABINET.y * 0.5)
                    .with_scale(room::CABINET),
                body,
            );

            // the screen, a thin slab on the face that looks into the room
            let screen = stood.at
                + Vec3::Y * room::CABINET.y * 0.68
                + stood.facing * (room::CABINET.z * 0.5 + 0.02);
            // thin along the way the cabinet looks, and landscape across it, so
            // the screenshot is the shape it was taken at
            let wide = room::CABINET.z * room::SCREEN;
            let placed =
                Transform::at(screen).with_scale(vec3(0.05, wide * room::SCREEN_SHAPE, wide));
            let tint = if lit {
                vec4(1.0, 1.0, 1.0, 1.0)
            } else {
                vec4(0.78, 0.78, 0.84, 1.0)
            };

            match self.art.get(n).copied().flatten() {
                Some(art) => scene.push_textured(screen_mesh, art, &placed, tint, 32.0),
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
                if let Some(n) = self.room.at(self.at, self.facing()) {
                    let stood = self.room.stood[n].clone();
                    if self.playing.start(&stood.cabinet) {
                        // the game wants the mouse now
                        self.wants_lock = false;
                    }
                }
            }
            KeyboardKey::Escape => self.quitting = held,
            _ => (),
        }
    }

    fn process_mouse(&mut self, input: MouseInput) {
        if input.button == MouseButton::Left && input.is_pressed() && !self.locked {
            self.wants_lock = true;
        }
    }

    fn mouse_motion(&mut self, delta: Vec2) {
        if !self.locked {
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
