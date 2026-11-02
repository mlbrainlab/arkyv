//! Crate desktop app (Slint UI): the windows and their behavior. Archive work
//! lives in `crate-core`; `main.rs` only parses the command line.

pub mod about;
pub mod batch;
pub mod dirs;
pub mod first_run;
pub mod home;
pub mod i18n;
pub mod instance;
pub mod license;
pub mod model;
pub mod ops;
pub mod ops_panel;
pub mod platform;
pub mod prefs;
pub mod preview;
pub mod pro;
pub mod settings;
pub mod shortcuts;
pub mod tree;
pub mod view;

slint::include_modules!();

use std::time::Duration;

use slint::ComponentHandle;
use slint::winit_030::WinitWindowAccessor;

/// Hands an interactive move to the window manager. The compositor keeps the
/// button release that ends the move, so tell Slint the button is up; otherwise
/// the header keeps the pointer grab and every later click lands on it.
/// The release is sent from a zero-delay timer because Slint ignores events
/// dispatched while it is still handling the pointer move that started the drag.
pub fn begin_window_drag<C: ComponentHandle + 'static>(ui: &C) {
    hand_to_window_manager(ui, |w| {
        let _ = w.drag_window();
    });
}

/// Like [`begin_window_drag`], for a resize from one of Crate's own frame
/// edges (`edge`: 0 north, then clockwise to 7 north-west).
pub fn begin_window_resize<C: ComponentHandle + 'static>(ui: &C, edge: i32) {
    use slint::winit_030::winit::window::ResizeDirection as D;
    let direction = match edge {
        0 => D::North,
        1 => D::NorthEast,
        2 => D::East,
        3 => D::SouthEast,
        4 => D::South,
        5 => D::SouthWest,
        6 => D::West,
        _ => D::NorthWest,
    };
    hand_to_window_manager(ui, |w| {
        let _ = w.drag_resize_window(direction);
    });
}

fn hand_to_window_manager<C: ComponentHandle + 'static>(
    ui: &C,
    start: impl FnOnce(&slint::winit_030::winit::window::Window),
) {
    ui.window().with_winit_window(start);
    let weak = ui.as_weak();
    slint::Timer::single_shot(Duration::ZERO, move || {
        if let Some(ui) = weak.upgrade() {
            ui.window()
                .dispatch_event(slint::platform::WindowEvent::PointerReleased {
                    position: slint::LogicalPosition::new(-1.0, -1.0),
                    button: slint::platform::PointerEventButton::Left,
                });
        }
    });
}

/// Crate's own frame around a window (app/ui/frame.slint), where Crate
/// draws its header (`custom`): resizing from its edges, and no margin
/// while the window is maximized or full screen.
pub fn connect_frame<C>(ui: &C, custom: bool)
where
    C: ComponentHandle + 'static,
    for<'a> Frame<'a>: slint::Global<'a, C>,
{
    let frame = ui.global::<Frame>();
    frame.set_custom(custom);
    let weak = ui.as_weak();
    frame.on_resized(move || {
        if let Some(ui) = weak.upgrade() {
            let filled = ui
                .window()
                .with_winit_window(|w| w.is_maximized() || w.fullscreen().is_some())
                .unwrap_or(false);
            ui.global::<Frame>().set_maximized(filled);
        }
    });
    let weak = ui.as_weak();
    frame.on_start_resize(move |edge| {
        if let Some(ui) = weak.upgrade() {
            begin_window_resize(&ui, edge);
        }
    });
}
