//! The app icon picked in Preferences → Appearance (Pro, M9) reaching the
//! system, beyond Arkyv's own windows (which follow `Tokens.app-icon`):
//!
//! - Linux: the icon theme in the user's data folder, which the `.desktop`
//!   entry's icon name finds first (GNOME on Wayland shows that icon, not
//!   the window's). Not in a Flatpak or snap, which can't write there.
//! - Windows: the Start menu and pinned taskbar shortcuts.
//! - macOS: the Dock icon while Arkyv runs (each window is a process, so
//!   each sets it).
//!
//! Desktops may hold on to the old icon until they restart.

use std::path::PathBuf;

/// Same order as `Tokens.app-icons` (tokens.slint).
pub const COUNT: usize = 8;
const NAMES: [&str; COUNT] = [
    "layers", "open-lid", "ark", "zip", "squeeze", "monogram", "open-box", "vault",
];
const PNG: [&[u8]; COUNT] = [
    include_bytes!("../../assets/icons/png/arkyv-256.png"),
    include_bytes!("../../assets/icons/alt/png/open-lid-256.png"),
    include_bytes!("../../assets/icons/alt/png/ark-256.png"),
    include_bytes!("../../assets/icons/alt/png/zip-256.png"),
    include_bytes!("../../assets/icons/alt/png/squeeze-256.png"),
    include_bytes!("../../assets/icons/alt/png/monogram-256.png"),
    include_bytes!("../../assets/icons/alt/png/open-box-256.png"),
    include_bytes!("../../assets/icons/alt/png/vault-256.png"),
];
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
const SVG: [&str; COUNT] = [
    include_str!("../../assets/icons/arkyv-icon.svg"),
    include_str!("../../assets/icons/alt/open-lid.svg"),
    include_str!("../../assets/icons/alt/ark.svg"),
    include_str!("../../assets/icons/alt/zip.svg"),
    include_str!("../../assets/icons/alt/squeeze.svg"),
    include_str!("../../assets/icons/alt/monogram.svg"),
    include_str!("../../assets/icons/alt/open-box.svg"),
    include_str!("../../assets/icons/alt/vault.svg"),
];

/// One icon, as the platform code needs it.
pub struct AppIcon {
    /// 0 is Layers, the icon the packages install: the platform code
    /// removes what it put in place instead of writing it.
    pub index: usize,
    pub name: &'static str,
    /// 256 px, a full-bleed rounded tile.
    pub png: &'static [u8],
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub svg: &'static str,
}

impl AppIcon {
    /// The tile at `size` px.
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn rgba(&self, size: u32) -> Option<image::RgbaImage> {
        let img = image::load_from_memory_with_format(self.png, image::ImageFormat::Png).ok()?;
        Some(image::imageops::resize(
            &img.to_rgba8(),
            size,
            size,
            image::imageops::FilterType::Lanczos3,
        ))
    }
}

pub fn icon(index: usize) -> AppIcon {
    let i = if index < COUNT { index } else { 0 };
    AppIcon {
        index: i,
        name: NAMES[i],
        png: PNG[i],
        svg: SVG[i],
    }
}

/// Puts icon `index` in place if it isn't already. Called whenever a
/// window applies the preferences ([`crate::prefs::Prefs::apply_theme`]),
/// so a change in Preferences, a removed license (back to Layers) or a new
/// install catches up. The file work runs on its own thread, once per
/// process and icon. On Linux the icon last put in place is kept in the
/// state folder, so it runs once per change; Windows checks its shortcuts
/// every time instead, as an update or a new pin brings shortcuts without
/// the icon ([`crate::platform::APP_ICON_STAMP`]).
pub fn sync(index: u8) {
    let index = usize::from(index).min(COUNT - 1);
    if !crate::platform::APP_ICON_FILES {
        // macOS: only the running process's Dock icon.
        crate::platform::set_app_icon(&icon(index));
        return;
    }
    thread_local! {
        static SENT: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
    }
    if SENT.with(|s| s.replace(Some(index))) == Some(index) {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("app-icon".into())
        .spawn(move || {
            if !crate::platform::APP_ICON_STAMP {
                crate::platform::set_app_icon(&icon(index));
                return;
            }
            let Some(stamp) = stamp() else { return };
            let applied = std::fs::read_to_string(&stamp)
                .ok()
                .and_then(|s| s.trim().parse::<usize>().ok())
                // Nothing applied yet: the packages' own icon is Layers.
                .unwrap_or(0);
            if applied == index {
                return;
            }
            if crate::platform::set_app_icon(&icon(index)) {
                if let Some(dir) = stamp.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                let _ = std::fs::write(&stamp, index.to_string());
            }
        });
}

/// Which icon the system files show.
fn stamp() -> Option<PathBuf> {
    crate::dirs::state().map(|s| crate::dirs::app(s).join("app-icon"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icons_decode_at_every_size() {
        for i in 0..COUNT {
            let icon = icon(i);
            assert!(icon.svg.contains("<svg"), "{}", icon.name);
            for size in [16, 48, 256] {
                let img = icon.rgba(size).expect(icon.name);
                assert_eq!(img.dimensions(), (size, size));
            }
        }
        assert_eq!(icon(99).index, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_icon_theme_written_and_removed() {
        let dir = tempfile::tempdir().unwrap();
        let theme = dir.path().join("icons/hicolor");
        let id = crate::platform::APP_ID;
        assert!(crate::platform::icon_theme(&theme, &icon(3)));
        for s in [16, 48, 512] {
            let png = theme.join(format!("{s}x{s}/apps/{id}.png"));
            let img = image::open(&png).unwrap();
            assert_eq!(img.width(), s);
        }
        let svg = std::fs::read_to_string(theme.join(format!("scalable/apps/{id}.svg"))).unwrap();
        assert_eq!(svg, SVG[3]);
        // Layers: the installed icon comes back.
        assert!(crate::platform::icon_theme(&theme, &icon(0)));
        assert!(!theme.join(format!("48x48/apps/{id}.png")).exists());
        assert!(!theme.join(format!("scalable/apps/{id}.svg")).exists());
    }
}
