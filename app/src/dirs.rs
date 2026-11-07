//! Where Arkyv keeps its files on each system. The XDG variables win
//! wherever they're set (the tests point them at temporary folders); then
//! Linux uses the XDG defaults under the home folder, Windows `%APPDATA%`
//! and `%LOCALAPPDATA%`.

use std::path::PathBuf;

/// The user's home folder.
pub fn home() -> Option<PathBuf> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os(var)
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

fn xdg(var: &str) -> Option<PathBuf> {
    std::env::var_os(var)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// The folder Arkyv keeps its own files in, inside `config()`, `state()`,
/// `cache()` or the runtime folder.
pub const APP: &str = "arkyv";

/// `base/arkyv`. The app was called Crate until 0.2.36 and used
/// `base/crate`: that folder is moved over the first time, so preferences,
/// the Pro license and history come along.
pub fn app(base: PathBuf) -> PathBuf {
    let new = base.join(APP);
    if !new.exists() {
        let old = base.join("crate");
        if old.is_dir() {
            let _ = std::fs::rename(&old, &new);
        }
    }
    new
}

/// Settings (`state.ini` goes in `arkyv/` inside): `~/.config`, or
/// `%APPDATA%` (roaming) on Windows.
pub fn config() -> Option<PathBuf> {
    xdg("XDG_CONFIG_HOME").or_else(|| {
        if cfg!(windows) {
            xdg("APPDATA")
        } else {
            home().map(|h| h.join(".config"))
        }
    })
}

/// History and other state worth keeping: `~/.local/state`, or
/// `%LOCALAPPDATA%` on Windows.
pub fn state() -> Option<PathBuf> {
    xdg("XDG_STATE_HOME").or_else(|| {
        if cfg!(windows) {
            xdg("LOCALAPPDATA")
        } else {
            home().map(|h| h.join(".local/state"))
        }
    })
}

/// Things that can be thrown away: `~/.cache`, or `%LOCALAPPDATA%` on
/// Windows.
pub fn cache() -> Option<PathBuf> {
    xdg("XDG_CACHE_HOME").or_else(|| {
        if cfg!(windows) {
            xdg("LOCALAPPDATA")
        } else {
            home().map(|h| h.join(".cache"))
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn moves_the_old_folder_once() {
        let base = tempfile::tempdir().unwrap();
        let old = base.path().join("crate");
        std::fs::create_dir(&old).unwrap();
        std::fs::write(old.join("state.ini"), "pro.key=x\n").unwrap();
        let new = super::app(base.path().to_path_buf());
        assert_eq!(new, base.path().join("arkyv"));
        assert_eq!(
            std::fs::read_to_string(new.join("state.ini")).unwrap(),
            "pro.key=x\n"
        );
        assert!(!old.exists());
        // Later runs leave both alone.
        std::fs::create_dir(&old).unwrap();
        assert_eq!(super::app(base.path().to_path_buf()), new);
        assert!(old.exists());
    }
}
