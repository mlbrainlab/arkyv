use std::time::Duration;

use super::{Defaults, MimeBackend};

/// macOS keeps the native title bar (M8).
pub const CUSTOM_FRAME: bool = false;
/// Blur behind windows isn't known here: Glass starts well frosted, as on
/// GNOME.
pub fn glass_frost() -> u8 {
    60
}

/// Blur where winit has it.
pub(super) fn blur(
    w: &slint::winit_030::winit::window::Window,
    on: bool,
    _material: super::Material,
    _frost: u8,
) {
    w.set_blur(on);
}

pub fn select_backend() -> Result<(), slint::PlatformError> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .select()
}

/// Time spent before `main`; only measured on Linux for now.
pub fn process_age() -> Option<Duration> {
    None
}

/// The system's monospace family (M7/M8: use the platform's own).
pub fn monospace_family() -> Option<String> {
    None
}

/// Default apps come with M8.
pub const SYSTEM_MIME: MimeBackend = MimeBackend {
    support: || Defaults::Unsupported,
    query: |_| None,
    set: |_, _| Err(std::io::Error::other("not supported yet")),
};

/// Dock progress comes with M8.
pub fn taskbar_progress(_window: &slint::Window, _fraction: Option<f32>) {}

/// The user's Downloads or Desktop folder from the system, when it says
/// (`settings::user_dir` reads the XDG user dirs instead).
pub fn known_folder(_key: &str) -> Option<std::path::PathBuf> {
    None
}

/// No system app icon to change here.
pub const APP_ICON_FILES: bool = false;
pub fn set_app_icon(_icon: &crate::app_icon::AppIcon) -> bool {
    false
}

pub fn reduce_transparency() -> bool {
    false
}
