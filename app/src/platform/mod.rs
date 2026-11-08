//! Platform-only code: Linux, Windows (M7), macOS (M8), and stubs elsewhere.

/// Reverse-DNS app id; also the Wayland app id and, later, the .desktop file name.
pub const APP_ID: &str = "io.github.mlbrainlab.arkyv";

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

// Not `mod windows`: that name is the `windows` crate's.
#[cfg(windows)]
#[path = "windows.rs"]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
mod other;
#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
pub use other::*;

/// Files the system asked Arkyv to open since the event loop started and
/// nobody has taken yet. Only macOS sends them this way (Finder's "open
/// documents" event); elsewhere they arrive as arguments.
#[cfg(not(target_os = "macos"))]
pub fn take_opened_files() -> Vec<std::path::PathBuf> {
    Vec::new()
}

/// Calls `open` for files the system asks Arkyv to open from now on (see
/// [`take_opened_files`]).
#[cfg(not(target_os = "macos"))]
pub fn on_open_files(_open: impl Fn(Vec<std::path::PathBuf>) + 'static) {}

/// The windows' menus go in the screen's menu bar (macOS); elsewhere the
/// header has them.
pub const NATIVE_MENU: bool = cfg!(target_os = "macos");

/// The UI uses the system's fonts (macOS: San Francisco and SF Arabic);
/// elsewhere the bundled Inter and Crate Arabic.
pub const SYSTEM_FONTS: bool = cfg!(target_os = "macos");

/// Blurs what's behind a Glass window (`on`), or stops: the system's
/// material where there is one ([`blur`]: Acrylic or Mica on Windows, the
/// window blur on macOS, KWin's blur on Wayland). GNOME and X11 have none, so
/// there Glass leans on the frost slider instead ([`glass_frost`]).
/// What's behind a Glass window on Windows (Preferences → Appearance →
/// Material). Other systems have one material and ignore it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Material {
    /// A heavy blur of everything behind the window.
    #[default]
    Acrylic = 0,
    /// Windows 11's backdrop: a tint from the wallpaper, never other windows.
    Mica = 1,
    /// No system material: what's behind shows as it is, sharp.
    Clear = 2,
}

pub fn set_blur<C: slint::ComponentHandle + 'static>(ui: &C, on: bool, material: Material) {
    use slint::winit_030::WinitWindowAccessor;

    // The winit window may not exist yet (a window shown before the event
    // loop runs).
    let ui = ui.clone_strong();
    let _ = slint::spawn_local(async move {
        if let Ok(w) = ui.window().winit_window().await {
            blur(&w, on, material);
        }
    });
}

/// Glass offers a choice of material, Acrylic, Mica or Clear (Windows only).
pub const GLASS_MATERIALS: bool = cfg!(windows);

/// Glass draws its own caption strip in place of the system title bar
/// (Windows only; see `own_caption` there).
#[cfg(not(windows))]
pub const GLASS_CAPTION: bool = false;

/// Windows: hands the window's title bar to Arkyv's caption strip. Nothing
/// elsewhere.
#[cfg(not(windows))]
pub fn own_caption<C: slint::ComponentHandle + 'static>(
    _ui: &C,
    _on: bool,
    _hover: impl Fn(bool) + 'static,
) {
}

/// macOS: puts the traffic lights inside Arkyv's own header (`on`), or gives
/// the window its plain title bar back, and calls `inset` with the width the
/// header leaves free for them at the left and the height it takes on to
/// line up with them (zeros for the plain bar). Elsewhere it does nothing:
/// Linux draws its own frame, Windows keeps the system's.
#[cfg(not(target_os = "macos"))]
pub fn overlay_titlebar<C: slint::ComponentHandle + 'static>(
    _ui: &C,
    _on: bool,
    _inset: impl Fn(f32, f32) + 'static,
) {
}

/// The settings page where the user gives Arkyv access to folders, when the
/// system guards them (macOS: Privacy & Security → Files and Folders).
pub const PRIVACY_SETTINGS: Option<&str> = if cfg!(target_os = "macos") {
    Some("x-apple.systempreferences:com.apple.preference.security?Privacy_FilesAndFolders")
} else {
    None
};

/// The naming rules of the disk Arkyv extracts to: this system's, or
/// Windows' with `ARKYV_NAME_RULES=windows` (UI tests, and trying the
/// rename prompt on Linux).
pub fn name_rules() -> crate_core::names::NameRules {
    match std::env::var("ARKYV_NAME_RULES").as_deref() {
        Ok("windows") => crate_core::names::NameRules::windows(),
        _ => crate_core::names::NameRules::native(),
    }
}

/// A file type Arkyv can be the default app for (First run, Preferences).
#[derive(Debug)]
pub struct FileType {
    /// ".zip", ".tar.*"
    pub label: &'static str,
    /// The MIME types it covers, the most common first; the same ones the
    /// .desktop file lists.
    pub mimes: &'static [&'static str],
}

pub const FILE_TYPES: [FileType; 4] = [
    FileType {
        label: ".zip",
        mimes: &["application/zip", "application/x-zip-compressed"],
    },
    FileType {
        label: ".7z",
        mimes: &["application/x-7z-compressed"],
    },
    FileType {
        label: ".rar",
        mimes: &[
            "application/vnd.rar",
            "application/x-rar",
            "application/x-rar-compressed",
        ],
    },
    FileType {
        label: ".tar.*",
        mimes: &[
            "application/x-compressed-tar",
            "application/x-tar",
            "application/x-gtar",
            "application/x-xz-compressed-tar",
            "application/x-zstd-compressed-tar",
            "application/x-bzip2-compressed-tar",
            "application/x-bzip-compressed-tar",
        ],
    },
];

/// Whether Arkyv can make itself the default app here. The numbers are
/// what the UI's `defaults` properties take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defaults {
    /// Set directly (xdg-mime on Linux).
    Direct = 0,
    /// Inside Flatpak: the sandbox can't, so the user does it in Files.
    Flatpak = 1,
    /// No installed .desktop file to point at (e.g. `cargo run`).
    NotInstalled = 2,
    /// Systems Arkyv doesn't know how to ask.
    Unsupported = 3,
    /// Windows: only the user can choose, in Settings → Default apps; Arkyv
    /// opens that page.
    SettingsPage = 4,
    /// macOS: Arkyv asks, and the system asks the user to confirm each type.
    Confirm = 5,
    /// Inside a snap: like Flatpak, confinement keeps Arkyv away from the
    /// user's `mimeapps.list`, so the user does it in Files.
    Snap = 6,
}

/// How default apps are read and written: the system's tools, or a stand-in
/// for tests (so they never touch the real `mimeapps.list`).
#[derive(Clone, Copy)]
pub struct MimeBackend {
    pub support: fn() -> Defaults,
    /// The .desktop file that opens `mime` by default.
    pub query: fn(&str) -> Option<String>,
    /// Makes `desktop` the default for every one of `mimes`.
    pub set: fn(&str, &[&str]) -> std::io::Result<()>,
}

thread_local! {
    static MIME: std::cell::Cell<MimeBackend> = const { std::cell::Cell::new(SYSTEM_MIME) };
}

pub fn set_mime_backend(backend: MimeBackend) {
    MIME.with(|m| m.set(backend));
}

pub fn defaults_support() -> Defaults {
    (MIME.with(|m| m.get()).support)()
}

fn desktop_file() -> String {
    format!("{APP_ID}.desktop")
}

/// True when Arkyv opens this type by default (judged by its main MIME type).
pub fn is_default(t: &FileType) -> bool {
    (MIME.with(|m| m.get()).query)(t.mimes[0]).as_deref() == Some(desktop_file().as_str())
}

/// Makes Arkyv the default app for these types.
pub fn make_default(types: &[&FileType]) -> std::io::Result<()> {
    let mimes: Vec<&str> = types.iter().flat_map(|t| t.mimes.iter().copied()).collect();
    if mimes.is_empty() {
        return Ok(());
    }
    (MIME.with(|m| m.get()).set)(&desktop_file(), &mimes)
}
