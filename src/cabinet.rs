//! A cabinet: which game it is, whether that game is built, and running it.
//! Spec 0001.
//!
//! The games are not linked into this and not changed by it. A cabinet knows a
//! name and runs a binary, which is the same binary a person runs from the
//! game's own folder.

use std::path::{Path, PathBuf};
use std::process::Child;

/// A game the arcade knows about.
#[derive(Debug, Clone)]
pub struct Cabinet {
    pub name: String,
    /// The binary, if it has been built. A cabinet with none is dark.
    pub binary: Option<PathBuf>,
}

impl Cabinet {
    /// Looks for the game's binary beside the arcade's own.
    ///
    /// Every crate in this project builds into one shared `target`, per the
    /// `[build]` in `.cargo/config.toml`, so the arcade's neighbours in its own
    /// directory are the games. Asking where the project is, or walking up from
    /// the working directory, would both be guesses that break the moment the
    /// thing is run from somewhere else.
    pub fn found(name: &str, beside: &Path) -> Self {
        let binary = beside.join(name);

        Self {
            name: name.to_string(),
            binary: binary.is_file().then_some(binary),
        }
    }

    /// Whether this one can be played.
    pub fn is_built(&self) -> bool {
        self.binary.is_some()
    }
}

/// The one game that may be running, per spec 0001.
///
/// One at a time. A room where every cabinet starts a window is a dozen windows
/// and a dozen sound devices, and the thing that was meant to show the games off
/// becomes a way to lose them behind each other.
#[derive(Debug, Default)]
pub struct Playing {
    running: Option<(String, Child)>,
}

impl Playing {
    pub fn new() -> Self {
        Self::default()
    }

    /// Which game is up, if any. Asked every frame, so it also reaps one that
    /// has quit: a child that has exited is no longer playing.
    pub fn now(&mut self) -> Option<&str> {
        let done = match self.running.as_mut() {
            Some((_, child)) => child.try_wait().map(|at| at.is_some()).unwrap_or(true),
            None => return None,
        };

        if done {
            self.running = None;
            return None;
        }

        self.running.as_ref().map(|(name, _)| name.as_str())
    }

    /// Starts one, unless something is already up or it was never built. Says
    /// whether it went.
    pub fn start(&mut self, cabinet: &Cabinet) -> bool {
        if self.now().is_some() {
            return false;
        }

        let Some(binary) = cabinet.binary.as_ref() else {
            return false;
        };

        self.run(&cabinet.name, std::process::Command::new(binary))
    }

    /// The same, given the command rather than building it.
    ///
    /// Split out so the rule above can be tested against something that stays
    /// alive. A test that spawns a game's binary is a test that opens a window,
    /// and one that spawns anything short lived is a test that races the thing
    /// it is checking.
    fn run(&mut self, name: &str, mut command: std::process::Command) -> bool {
        if self.running.is_some() {
            return false;
        }

        match command.spawn() {
            Ok(child) => {
                self.running = Some((name.to_string(), child));
                true
            }
            Err(_) => false,
        }
    }

    /// Stops the one that is up, if there is one.
    fn stop(&mut self) {
        if let Some((_, mut child)) = self.running.take() {
            let _ = child.kill();
            // and reaped, so what is left is not a zombie
            let _ = child.wait();
        }
    }
}

/// The hall shuts the game it opened.
///
/// A `Child` is not killed when it is dropped; the standard library says so and
/// means it. So quitting the arcade left whatever you had started running with
/// nothing on screen to say so, and a game drawing as fast as it can is a whole
/// core gone. The next arcade stuttered in the picture and in the sound, and
/// the cause was the last one.
///
/// The hall owns what it opened. Walk away from a cabinet and the game it ran
/// goes with it, the same as switching a machine off.
impl Drop for Playing {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec 0001: a game that is not built is a cabinet that will not start.
    #[test]
    fn one_that_is_not_built_will_not_start() {
        let dark = Cabinet::found("no-such-game", Path::new("/nowhere"));
        assert!(!dark.is_built());
        assert_eq!(dark.name, "no-such-game");

        let mut playing = Playing::new();
        assert!(!playing.start(&dark), "a dark cabinet started something");
        assert_eq!(playing.now(), None);
    }

    /// Something that stays up long enough to be asked about, standing in for
    /// a game.
    fn a_long_one() -> std::process::Command {
        let mut command = std::process::Command::new("/bin/sleep");
        command.arg("30");
        command
    }

    /// Spec 0001: and only one runs at a time.
    #[test]
    fn only_one_runs_at_a_time() {
        let mut playing = Playing::new();

        assert!(playing.run("first", a_long_one()), "it would not start one");
        assert_eq!(playing.now(), Some("first"));
        assert!(
            !playing.run("second", a_long_one()),
            "it started a second while the first was up"
        );
        assert_eq!(playing.now(), Some("first"), "the first was lost");

        // and a cabinet cannot start one either while that is up
        let anything = Cabinet::found("sh", Path::new("/bin"));
        assert!(!playing.start(&anything));
    }

    /// Spec 0001: and the hall shuts it on the way out.
    ///
    /// A child outliving its parent is the default and not an accident, so this
    /// asks the system whether the process is still there rather than asking
    /// the struct that just dropped it.
    #[test]
    fn quitting_the_hall_stops_the_game() {
        let mut playing = Playing::new();
        assert!(playing.run("long", a_long_one()), "it would not start one");

        let pid = playing
            .running
            .as_ref()
            .map(|(_, child)| child.id())
            .expect("something to have been started");

        drop(playing);

        let still = std::process::Command::new("/bin/ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
            .expect("ps to run");

        assert!(
            !String::from_utf8_lossy(&still.stdout).contains(&pid.to_string()),
            "the game outlived the hall: {} is still up",
            pid
        );
    }

    /// And when the one that was up quits, the room is free again.
    #[test]
    fn it_is_free_again_once_that_one_quits() {
        let mut playing = Playing::new();
        let mut quick = std::process::Command::new("/bin/sh");
        quick.arg("-c").arg("exit 0");

        assert!(playing.run("quick", quick));
        for _ in 0..200 {
            if playing.now().is_none() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        panic!("it never noticed the one that was up had quit");
    }

    /// And it finds one that is there.
    #[test]
    fn it_finds_a_binary_beside_it() {
        let found = Cabinet::found("sh", Path::new("/bin"));

        assert!(found.is_built(), "it did not find /bin/sh");
        assert_eq!(found.binary.as_deref(), Some(Path::new("/bin/sh")));
    }
}
