//! What the hall needs to know before it is built, for spec 0053.
//!
//! The arcade finds its cabinets by reading the `games` folder and loads each
//! one's screenshot off disk when it draws it. Both are right on a machine and
//! impossible in a browser, which has no folder to read and no disk to read
//! it from, so this reads them here instead and writes down what it found.
//!
//! The same folder either way, so the two accounts cannot drift: one of them
//! is the other, frozen at the moment this ran.
//!
//! The pictures are only written down for a browser. A native build goes on
//! reading them off disk, where they can change without a rebuild, and where
//! four and a half megabytes of them do not want to be inside the binary.
//!
//! And they are shrunk on the way in. A game's screenshot is 1200 across
//! because that is what the site wants of it; a cabinet screen in the hall is
//! a couple of hundred pixels of a window. Carried whole they put 4.6
//! megabytes inside the binary and none of it compressed, because a PNG does
//! not gzip twice: 10.4 megabytes out of wasm-bindgen and 7.6 over the wire,
//! against half a megabyte for an example.

use std::path::{Path, PathBuf};

/// How wide a screenshot is kept for a cabinet screen.
///
/// `room::SCREEN_SHAPE` is 948 by 1200, so this is the long side and the
/// short one follows. Twice what the screen is ever drawn at, which leaves
/// something for standing close.
const ACROSS: u32 = 512;

fn main() {
    let root = match Path::new(env!("CARGO_MANIFEST_DIR")).parent() {
        Some(root) => root.join("games"),
        None => return,
    };

    println!("cargo:rerun-if-changed={}", root.display());

    let mut names: Vec<String> = std::fs::read_dir(&root)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().join("Cargo.toml").is_file())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();

    let for_a_browser = std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("wasm32");
    let mut out = String::new();

    out.push_str("/// Every repo under `games`, as this was built.\n");
    out.push_str("pub const REPOS: &[&str] = &[\n");
    for name in &names {
        out.push_str(&format!("    {:?},\n", name));
    }
    out.push_str("];\n\n");

    out.push_str("/// Each one's screenshot, for a build that cannot read a disk.\n");
    out.push_str("pub const ART: &[(&str, &[u8])] = &[\n");
    if for_a_browser {
        let into = PathBuf::from(std::env::var("OUT_DIR").expect("cargo says where"));

        for name in &names {
            let shot: PathBuf = root.join(name).join("media/screenshot.png");
            if !shot.is_file() {
                continue;
            }

            println!("cargo:rerun-if-changed={}", shot.display());

            let small = into.join(format!("{}-screen.png", name));
            match image::open(&shot) {
                Ok(picture) => {
                    picture
                        .resize(ACROSS, ACROSS, image::imageops::FilterType::Lanczos3)
                        .save(&small)
                        .expect("written");
                }
                // a picture that will not open is a cabinet with a blank
                // screen, which is what a machine with no screenshot already
                // gets. Better than a build that will not finish.
                Err(why) => {
                    println!("cargo:warning={} would not open: {}", shot.display(), why);
                    continue;
                }
            }

            out.push_str(&format!(
                "    ({:?}, include_bytes!({:?})),\n",
                name,
                small.display().to_string()
            ));
        }
    }
    out.push_str("];\n");

    let into = Path::new(&std::env::var("OUT_DIR").expect("cargo says where")).join("baked.rs");
    std::fs::write(into, out).expect("written");
}
