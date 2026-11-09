use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use super::{Defaults, MimeBackend};

use slint::winit_030::winit::platform::wayland::WindowAttributesExtWayland;

/// Linux draws its own header bar (design/DESIGN.md → Window frame).
pub const CUSTOM_FRAME: bool = true;
/// Where Glass starts on the slider in Preferences, in percent. KWin
/// blurs behind windows on Wayland ([`blur`]), so Glass starts clear
/// there; GNOME and X11 don't, so it starts well frosted and text stays
/// readable.
pub fn glass_frost() -> u8 {
    let kde = std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|d| d.contains("KDE"));
    if kde && std::env::var_os("WAYLAND_DISPLAY").is_some() {
        0
    } else {
        60
    }
}

/// KWin's blur protocol on Wayland, through winit; nothing on GNOME (no
/// such protocol) or X11 (winit doesn't set KDE's blur property there).
pub(super) fn blur(
    w: &slint::winit_030::winit::window::Window,
    on: bool,
    _material: super::Material,
    _frost: u8,
) {
    w.set_blur(on);
}

/// Winit backend with Arkyv's app id, so GNOME matches windows to the
/// .desktop file and its icon.
///
/// The desktop's launch token (`XDG_ACTIVATION_TOKEN` on Wayland,
/// `DESKTOP_STARTUP_ID` on X11) goes to the first window: showing it then
/// tells the shell the launch is over. Without it GNOME keeps the busy
/// cursor until its timeout although the window is up. The variables are
/// cleared, so windows Arkyv starts itself don't reuse the token.
pub fn select_backend() -> Result<(), slint::PlatformError> {
    use slint::winit_030::winit::platform::startup_notify::{
        WindowAttributesExtStartupNotify, reset_activation_token_env,
    };
    use slint::winit_030::winit::window::ActivationToken;

    let token = std::env::var("XDG_ACTIVATION_TOKEN")
        .or_else(|_| std::env::var("DESKTOP_STARTUP_ID"))
        .ok()
        .filter(|t| !t.is_empty());
    reset_activation_token_env();
    let token = std::sync::Mutex::new(token);
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .with_winit_window_attributes_hook(move |attrs| {
            let attrs = attrs.with_name(super::APP_ID, "arkyv");
            match token.lock().unwrap().take() {
                Some(t) => attrs.with_activation_token(ActivationToken::from_raw(t)),
                None => attrs,
            }
        })
        .select()
}

/// How long the process existed before this call: exec, dynamic loading and
/// static initializers all happen before `main` runs. Read from /proc, so the
/// resolution is one clock tick (10 ms).
pub fn process_age() -> Option<Duration> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    // Fields after the ")" that closes the command name; starttime is field 22 overall.
    let start_ticks: f64 = stat
        .rsplit_once(')')?
        .1
        .split_whitespace()
        .nth(19)?
        .parse()
        .ok()?;
    let uptime: f64 = std::fs::read_to_string("/proc/uptime")
        .ok()?
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;
    let ticks_per_sec = 100.0; // USER_HZ, fixed at 100 on Linux.
    let age = uptime - start_ticks / ticks_per_sec;
    (age >= 0.0).then(|| Duration::from_secs_f64(age))
}

/// Windows shows progress on the taskbar button; Linux docks have no
/// common way (the window shows its own).
pub fn taskbar_progress(_window: &slint::Window, _fraction: Option<f32>) {}

/// The family fontconfig picks for "monospace" (e.g. "DejaVu Sans Mono").
pub fn monospace_family() -> Option<String> {
    let out = std::process::Command::new("fc-match")
        .args(["-f", "%{family[0]}", "monospace"])
        .output()
        .ok()?;
    let family = String::from_utf8(out.stdout).ok()?;
    let family = family.trim();
    (out.status.success() && !family.is_empty()).then(|| family.to_owned())
}

/// Default apps through xdg-utils (`xdg-mime`), which writes the user's
/// `mimeapps.list` for every desktop.
pub const SYSTEM_MIME: MimeBackend = MimeBackend {
    support: || {
        if std::env::var_os("FLATPAK_ID").is_some() {
            Defaults::Flatpak
        } else if std::env::var_os("SNAP_NAME").is_some() {
            Defaults::Snap
        } else if installed_desktop_file().is_none() {
            Defaults::NotInstalled
        } else {
            Defaults::Direct
        }
    },
    query: |mime| gio_default(mime).unwrap_or_else(|| xdg_mime_default(mime)),
    set: |desktop, mimes| {
        let status = Command::new("xdg-mime")
            .args(["default", desktop])
            .args(mimes)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::other(format!("xdg-mime failed ({status})")))
        }
    },
};

/// The default app as GLib (and so GNOME Files) sees it; `None` when `gio`
/// isn't installed. `xdg-mime query` misreads `mimeapps.list` on Ubuntu's
/// GNOME (it reported Files while GLib, and Files, used Arkyv).
fn gio_default(mime: &str) -> Option<Option<String>> {
    let out = Command::new("gio")
        .args(["mime", mime])
        .env("LC_ALL", "C")
        .output()
        .ok()?;
    Some(parse_gio_mime(&String::from_utf8_lossy(&out.stdout)))
}

/// `Default application for “application/zip”: arkyv.desktop` → the app.
fn parse_gio_mime(text: &str) -> Option<String> {
    text.lines()
        .next()?
        .strip_prefix("Default application for")?
        .rsplit_once(": ")
        .map(|(_, app)| app.trim().to_owned())
        .filter(|app| !app.is_empty())
}

fn xdg_mime_default(mime: &str) -> Option<String> {
    let out = Command::new("xdg-mime")
        .args(["query", "default", mime])
        .output()
        .ok()?;
    let name = String::from_utf8(out.stdout).ok()?;
    let name = name.trim();
    (out.status.success() && !name.is_empty()).then(|| name.to_owned())
}

/// Arkyv's .desktop file in the XDG data folders, if it is installed.
pub fn installed_desktop_file() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| home.map(|h| h.join(".local/share")));
    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .ok()
        .filter(|d| !d.is_empty())
        .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    data_home
        .into_iter()
        .chain(data_dirs.split(':').map(PathBuf::from))
        .map(|d| {
            d.join("applications")
                .join(format!("{}.desktop", super::APP_ID))
        })
        .find(|f| f.is_file())
}

/// The user's Downloads or Desktop folder from the system, when it says
/// (`settings::user_dir` reads the XDG user dirs instead).
pub fn known_folder(_key: &str) -> Option<std::path::PathBuf> {
    None
}

/// The app icon is files on the system ([`crate::app_icon`]), written once
/// per change: nothing else touches the user's icon theme.
pub const APP_ICON_FILES: bool = true;
pub const APP_ICON_STAMP: bool = true;

/// Every size of the icon, and the SVG, in the user's icon theme
/// (`~/.local/share/icons/hicolor`) under Arkyv's app id: the theme lookup
/// searches the user's folder before `/usr/share`, size by size, so the
/// `.desktop` entry's icon (and GNOME's dock and app grid) shows it. Layers
/// removes them, which brings back the installed icon. Flatpak and snap
/// can't write there. Returns whether the icon is now in place.
pub fn set_app_icon(icon: &crate::app_icon::AppIcon) -> bool {
    if std::env::var_os("FLATPAK_ID").is_some() || std::env::var_os("SNAP_NAME").is_some() {
        return false;
    }
    let Some(data) = crate::dirs::xdg("XDG_DATA_HOME")
        .or_else(|| crate::dirs::home().map(|h| h.join(".local/share")))
    else {
        return false;
    };
    icon_theme(&data.join("icons/hicolor"), icon)
}

/// [`set_app_icon`] in the hicolor theme at `theme`.
pub(crate) fn icon_theme(theme: &std::path::Path, icon: &crate::app_icon::AppIcon) -> bool {
    const SIZES: [u32; 8] = [16, 24, 32, 48, 64, 128, 256, 512];
    let theme = theme.to_path_buf();
    let png = |s: u32| theme.join(format!("{s}x{s}/apps/{}.png", super::APP_ID));
    let svg = theme.join(format!("scalable/apps/{}.svg", super::APP_ID));
    let ok = if icon.index == 0 {
        for s in SIZES {
            let _ = std::fs::remove_file(png(s));
        }
        let _ = std::fs::remove_file(&svg);
        true
    } else {
        let write = |path: &std::path::Path, bytes: &[u8]| -> std::io::Result<()> {
            std::fs::create_dir_all(path.parent().unwrap_or(&theme))?;
            std::fs::write(path, bytes)
        };
        SIZES.iter().all(|&s| {
            let Some(img) = icon.rgba(s) else {
                return false;
            };
            let mut bytes = Vec::new();
            img.write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            )
            .is_ok()
                && write(&png(s), &bytes).is_ok()
        }) && write(&svg, icon.svg.as_bytes()).is_ok()
    };
    // A user icon cache hides new files until it's rebuilt (never make one:
    // a stale cache would hide icons other apps add later).
    if theme.join("icon-theme.cache").exists() {
        let _ = Command::new("gtk-update-icon-cache")
            .args(["-q", "-t"])
            .arg(&theme)
            .status();
    }
    ok
}

/// No desktop setting for less transparency here (GNOME's high contrast
/// is a different theme): Glass stays as chosen.
pub fn reduce_transparency() -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_gio_mime() {
        let out = "Default application for “application/zip”: io.github.mlbrainlab.arkyv.desktop\nRegistered applications:\n\tx.desktop\n";
        assert_eq!(
            super::parse_gio_mime(out).as_deref(),
            Some("io.github.mlbrainlab.arkyv.desktop")
        );
        assert_eq!(
            super::parse_gio_mime("No default applications for “application/x-foo”\n"),
            None
        );
    }
}
