use std::time::Duration;

use super::{Defaults, MimeBackend};

/// The native title bar for now (the custom frame is its own M7 task).
pub const CUSTOM_FRAME: bool = false;
/// Glass gets real blur from the OS here ([`blur`]): it starts as clear
/// glass on the slider in Preferences.
pub fn glass_frost() -> u8 {
    0
}

/// Acrylic behind the window (Windows 10 1809 and later; winit has no blur
/// here). Fails quietly on older systems: Glass is then see-through only.
pub(super) fn blur(w: &slint::winit_030::winit::window::Window, on: bool) {
    let _ = if on {
        window_vibrancy::apply_acrylic(w, None)
    } else {
        window_vibrancy::clear_acrylic(w)
    };
}

pub fn select_backend() -> Result<(), slint::PlatformError> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .select()
}

/// Glass draws its own caption strip here (frame.slint's `CaptionBar`) in
/// place of the system title bar; see [`own_caption`].
pub const GLASS_CAPTION: bool = true;

/// Glass without the system title bar: the window keeps the system's
/// shadow and rounded corners (Windows 11), and Crate answers the system's
/// hit-testing for the window's edges (resizing) and for its own maximize
/// button, so Windows 11 shows Snap Layouts when the pointer rests on it.
/// Windows then sends the pointer's moves over that button to the frame,
/// not to Slint, so `hover` hears when it's on it. `on: false` gives the
/// window back to the system.
pub fn own_caption<C: slint::ComponentHandle + 'static>(
    ui: &C,
    on: bool,
    hover: impl Fn(bool) + 'static,
) {
    use slint::winit_030::WinitWindowAccessor;
    use slint::winit_030::winit::platform::windows::WindowExtWindows;

    // The winit window may not exist yet (a window shown before the event
    // loop runs).
    let ui = ui.clone_strong();
    let _ = slint::spawn_local(async move {
        let Ok(w) = ui.window().winit_window().await else {
            return;
        };
        let Some(hwnd) = hwnd(&w) else { return };
        w.set_undecorated_shadow(on);
        round_corners(hwnd, on);
        let weak = ui.as_weak();
        caption::set(
            hwnd,
            on,
            move || {
                caption::toggle_maximized(weak.clone());
            },
            hover,
        );
    });
}

fn hwnd(w: &slint::winit_030::winit::window::Window) -> Option<::windows::Win32::Foundation::HWND> {
    use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let RawWindowHandle::Win32(h) = w.window_handle().ok()?.as_raw() else {
        return None;
    };
    Some(::windows::Win32::Foundation::HWND(h.hwnd.get() as *mut _))
}

/// Windows 11's rounded corners on a window without a title bar (a no-op on
/// Windows 10, which has none).
fn round_corners(hwnd: ::windows::Win32::Foundation::HWND, on: bool) {
    use ::windows::Win32::Graphics::Dwm::{
        DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DEFAULT, DWMWCP_ROUND, DwmSetWindowAttribute,
    };
    let corners = if on { DWMWCP_ROUND } else { DWMWCP_DEFAULT };
    // SAFETY: a live window handle and a 4-byte value, as the attribute takes.
    unsafe {
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            (&raw const corners).cast(),
            size_of_val(&corners) as u32,
        );
    }
}

/// The window procedure hook behind [`own_caption`].
mod caption {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    use ::windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use ::windows::Win32::UI::HiDpi::GetDpiForWindow;
    use ::windows::Win32::UI::Input::KeyboardAndMouse::{
        TME_LEAVE, TME_NONCLIENT, TRACKMOUSEEVENT, TrackMouseEvent,
    };
    use ::windows::Win32::UI::Shell::{
        DefSubclassProc, GetWindowSubclass, RemoveWindowSubclass, SetWindowSubclass,
    };
    use ::windows::Win32::UI::WindowsAndMessaging::{
        GetWindowRect, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTLEFT, HTMAXBUTTON, HTRIGHT, HTTOP,
        HTTOPLEFT, HTTOPRIGHT, IsZoomed, WM_MOUSEMOVE, WM_NCDESTROY, WM_NCHITTEST,
        WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE,
    };

    const ID: usize = 0x4352_4154; // "CRAT"

    /// The resize grip inside the window's edges, and the caption buttons
    /// (frame.slint): 46 × 32 each, the maximize button second from the end.
    const GRIP: i32 = 6;
    const BUTTON_W: i32 = 46;
    const BUTTON_H: i32 = 32;

    struct State {
        on: Cell<bool>,
        hovering: Cell<bool>,
        toggle: Box<dyn Fn()>,
        hover: Rc<dyn Fn(bool)>,
    }

    pub(super) fn set(
        hwnd: HWND,
        on: bool,
        toggle: impl Fn() + 'static,
        hover: impl Fn(bool) + 'static,
    ) {
        let mut data = 0usize;
        // SAFETY: the hook is ours: `data` is the `State` it was installed with.
        unsafe {
            if GetWindowSubclass(hwnd, Some(proc), ID, Some(&mut data)).as_bool() {
                (*(data as *const State)).on.set(on);
                return;
            }
        }
        if !on {
            return;
        }
        let state = Box::new(State {
            on: Cell::new(true),
            hovering: Cell::new(false),
            toggle: Box::new(toggle),
            hover: Rc::new(hover),
        });
        // SAFETY: the State lives until WM_NCDESTROY frees it.
        unsafe {
            let _ = SetWindowSubclass(hwnd, Some(proc), ID, Box::into_raw(state) as usize);
        }
    }

    /// Maximize or restore, after the window procedure returns.
    pub(super) fn toggle_maximized<C: slint::ComponentHandle + 'static>(weak: slint::Weak<C>) {
        use slint::winit_030::WinitWindowAccessor;
        slint::Timer::single_shot(Duration::ZERO, move || {
            if let Some(ui) = weak.upgrade() {
                ui.window()
                    .with_winit_window(|w| w.set_maximized(!w.is_maximized()));
            }
        });
    }

    fn set_hovering(state: &State, hwnd: HWND, on: bool) {
        if state.hovering.replace(on) == on {
            return;
        }
        if on {
            let mut track = TRACKMOUSEEVENT {
                cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_LEAVE | TME_NONCLIENT,
                hwndTrack: hwnd,
                dwHoverTime: 0,
            };
            // SAFETY: a filled-in struct for a live window.
            unsafe {
                let _ = TrackMouseEvent(&mut track);
            }
        }
        // Into Slint after the window procedure returns.
        let hover = state.hover.clone();
        slint::Timer::single_shot(Duration::ZERO, move || hover(on));
    }

    /// What's under the screen point in `lparam`: a resize edge, the
    /// maximize button, or `None` for the system's answer.
    fn hit(hwnd: HWND, lparam: LPARAM) -> Option<u32> {
        let x = (lparam.0 & 0xffff) as i16 as i32;
        let y = ((lparam.0 >> 16) & 0xffff) as i16 as i32;
        let mut r = RECT::default();
        // SAFETY: a live window handle.
        let (dpi, zoomed) = unsafe {
            GetWindowRect(hwnd, &mut r).ok()?;
            (GetDpiForWindow(hwnd) as i32, IsZoomed(hwnd).as_bool())
        };
        let px = |v: i32| v * dpi.max(96) / 96;
        let (x, y, w, h) = (x - r.left, y - r.top, r.right - r.left, r.bottom - r.top);
        if !zoomed {
            let g = px(GRIP);
            let (left, right) = (x < g, x >= w - g);
            let (top, bottom) = (y < g, y >= h - g);
            let edge = match (top, bottom, left, right) {
                (true, _, true, _) => Some(HTTOPLEFT),
                (true, _, _, true) => Some(HTTOPRIGHT),
                (_, true, true, _) => Some(HTBOTTOMLEFT),
                (_, true, _, true) => Some(HTBOTTOMRIGHT),
                (true, ..) => Some(HTTOP),
                (_, true, ..) => Some(HTBOTTOM),
                (_, _, true, _) => Some(HTLEFT),
                (.., true) => Some(HTRIGHT),
                _ => None,
            };
            if edge.is_some() {
                return edge;
            }
        }
        // Arabic: the caption buttons sit at the left, Close first.
        let (from, to) = if crate::i18n::current().rtl() {
            (px(BUTTON_W), px(2 * BUTTON_W))
        } else {
            (w - px(2 * BUTTON_W), w - px(BUTTON_W))
        };
        (y < px(BUTTON_H) && x >= from && x < to).then_some(HTMAXBUTTON)
    }

    unsafe extern "system" fn proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _id: usize,
        data: usize,
    ) -> LRESULT {
        // SAFETY: `data` is the State `set` installed; freed only below.
        let state = unsafe { &*(data as *const State) };
        if msg == WM_NCDESTROY {
            // SAFETY: the hook goes first, so nothing reads the State after.
            unsafe {
                let _ = RemoveWindowSubclass(hwnd, Some(proc), ID);
                drop(Box::from_raw(data as *mut State));
                return DefSubclassProc(hwnd, msg, wparam, lparam);
            }
        }
        if state.on.get() {
            let on_max = wparam.0 == HTMAXBUTTON as usize;
            match msg {
                WM_NCHITTEST => {
                    if let Some(code) = hit(hwnd, lparam) {
                        return LRESULT(code as isize);
                    }
                }
                WM_NCMOUSEMOVE => set_hovering(state, hwnd, on_max),
                WM_NCMOUSELEAVE | WM_MOUSEMOVE => set_hovering(state, hwnd, false),
                // The system would draw its own old button while pressed.
                WM_NCLBUTTONDOWN | WM_NCLBUTTONDBLCLK if on_max => return LRESULT(0),
                WM_NCLBUTTONUP if on_max => {
                    (state.toggle)();
                    return LRESULT(0);
                }
                _ => {}
            }
        }
        // SAFETY: passing the message on, unchanged.
        unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) }
    }
}

/// Time spent before `main`; only measured on Linux.
pub fn process_age() -> Option<Duration> {
    None
}

/// Consolas ships with every Windows since Vista.
pub fn monospace_family() -> Option<String> {
    Some("Consolas".into())
}

/// Default apps: Windows doesn't let apps choose themselves, so "making
/// Crate the default" opens Settings → Default apps on Crate's page (the
/// installer registers it as "Crate", see packaging/windows/crate.iss).
pub const SYSTEM_MIME: MimeBackend = MimeBackend {
    support: || {
        if installed() {
            Defaults::SettingsPage
        } else {
            Defaults::NotInstalled
        }
    },
    query: |mime| {
        let ext = extension(mime)?;
        let key = format!(
            r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{ext}\UserChoice"
        );
        let prog_id = reg_string(HKEY_CURRENT_USER, &key, "ProgId")?;
        (prog_id == PROG_ID).then(|| format!("{}.desktop", super::APP_ID))
    },
    set: |_, _| open::that_detached("ms-settings:defaultapps?registeredAppUser=Crate"),
};

use ::windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

/// The installer's ProgID for archives.
const PROG_ID: &str = "Crate.Archive";

/// Installed for this user or for everyone.
fn installed() -> bool {
    [HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE]
        .into_iter()
        .any(|root| reg_string(root, r"Software\RegisteredApplications", "Crate").is_some())
}

/// The extension Explorer keys the choice by, for a type's main MIME type.
fn extension(mime: &str) -> Option<&'static str> {
    Some(match mime {
        "application/zip" => ".zip",
        "application/x-7z-compressed" => ".7z",
        "application/vnd.rar" => ".rar",
        "application/x-compressed-tar" => ".tgz",
        _ => return None,
    })
}

/// A string value from the registry.
fn reg_string(root: HKEY, key: &str, value: &str) -> Option<String> {
    use ::windows::Win32::Foundation::ERROR_SUCCESS;
    use ::windows::Win32::System::Registry::{RRF_RT_REG_SZ, RegGetValueW};
    use ::windows::core::HSTRING;

    let (key, value) = (HSTRING::from(key), HSTRING::from(value));
    let mut buf = [0u16; 512];
    let mut size = std::mem::size_of_val(&buf) as u32;
    // SAFETY: the buffer and its size in bytes match.
    let result = unsafe {
        RegGetValueW(
            root,
            &key,
            &value,
            RRF_RT_REG_SZ,
            None,
            Some(buf.as_mut_ptr().cast()),
            Some(&raw mut size),
        )
    };
    if result != ERROR_SUCCESS {
        return None;
    }
    // The size includes the terminating NUL.
    let len = (size as usize / 2).saturating_sub(1);
    Some(String::from_utf16_lossy(&buf[..len]))
}

/// Progress on the window's taskbar button: a fraction from 0 to 1, or
/// `None` to clear it. Failures are ignored (no taskbar, Explorer
/// restarting).
pub fn taskbar_progress(window: &slint::Window, fraction: Option<f32>) {
    use ::windows::Win32::Foundation::HWND;
    use ::windows::Win32::UI::Shell::{TBPF_NOPROGRESS, TBPF_NORMAL};
    use slint::winit_030::WinitWindowAccessor;
    use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};

    window.with_winit_window(|w| {
        let Ok(handle) = w.window_handle() else {
            return;
        };
        let RawWindowHandle::Win32(h) = handle.as_raw() else {
            return;
        };
        let hwnd = HWND(h.hwnd.get() as *mut _);
        let Some(taskbar) = taskbar() else {
            return;
        };
        // SAFETY: COM calls on the UI thread with a live window handle.
        unsafe {
            match fraction {
                Some(f) => {
                    let done = (f.clamp(0.0, 1.0) * 1000.0) as u64;
                    let _ = taskbar.SetProgressState(hwnd, TBPF_NORMAL);
                    let _ = taskbar.SetProgressValue(hwnd, done, 1000);
                }
                None => {
                    let _ = taskbar.SetProgressState(hwnd, TBPF_NOPROGRESS);
                }
            }
        }
    });
}

thread_local! {
    static TASKBAR: std::cell::OnceCell<Option<::windows::Win32::UI::Shell::ITaskbarList3>> =
        const { std::cell::OnceCell::new() };
}

/// The taskbar's COM object, made once per (UI) thread.
fn taskbar() -> Option<::windows::Win32::UI::Shell::ITaskbarList3> {
    use ::windows::Win32::System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    };
    use ::windows::Win32::UI::Shell::{ITaskbarList3, TaskbarList};

    TASKBAR.with(|t| {
        t.get_or_init(|| {
            // SAFETY: plain COM setup; winit has usually initialized COM on
            // this thread already, which makes this a no-op.
            unsafe {
                let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                let list: ITaskbarList3 =
                    CoCreateInstance(&TaskbarList, None, CLSCTX_INPROC_SERVER).ok()?;
                list.HrInit().ok()?;
                Some(list)
            }
        })
        .clone()
    })
}

/// The user's Downloads (`DOWNLOAD`) or Desktop (`DESKTOP`) folder from the
/// shell, which knows when it was moved (to OneDrive, another drive).
pub fn known_folder(key: &str) -> Option<std::path::PathBuf> {
    use ::windows::Win32::System::Com::CoTaskMemFree;
    use ::windows::Win32::UI::Shell::{
        FOLDERID_Desktop, FOLDERID_Downloads, KF_FLAG_DEFAULT, SHGetKnownFolderPath,
    };

    let id = match key {
        "DOWNLOAD" => &FOLDERID_Downloads,
        "DESKTOP" => &FOLDERID_Desktop,
        _ => return None,
    };
    // SAFETY: the shell allocates the string; it's copied, then freed.
    unsafe {
        let path = SHGetKnownFolderPath(id, KF_FLAG_DEFAULT, None).ok()?;
        let text = path.to_string().ok();
        CoTaskMemFree(Some(path.0 as *const _));
        text.map(std::path::PathBuf::from)
    }
}
