//! The Glass theme (Pro): the Theme row in Preferences, and archive windows
//! in glass. Test builds can't make a real key, so the windows get glass the
//! way `Prefs::apply_theme` gives it to them, frosted as on Linux. Shots are composited
//! over a stand-in wallpaper.

#[path = "../../crates/crate-core/tests/support/mod.rs"]
mod support;

mod common;

use common::{Harness, g};
use crate_app::prefs::{Prefs, Theme};
use crate_app::{PrefsUi, ProUi, Tokens};
use slint::ComponentHandle;

#[test]
fn glass_flow() {
    let h = Harness::new();
    let dir = tempfile::tempdir().unwrap();

    let prefs = h.prefs();
    // The Glass slider, saved and applied (only shown with Glass on, so
    // drive the setting directly).
    prefs
        .global::<PrefsUi>()
        .invoke_choose("glass_frost".into(), 85);
    assert_eq!(Prefs::load().glass_frost, 85);
    assert_eq!(prefs.global::<PrefsUi>().get_glass_frost(), 85);
    assert!((prefs.global::<Tokens>().get_frost() - 0.85).abs() < 1e-6);
    prefs
        .global::<PrefsUi>()
        .invoke_choose("glass_frost".into(), 60);

    // Without a key, Glass carries PRO and opens the upgrade sheet.
    prefs.global::<PrefsUi>().invoke_choose("theme".into(), 3);
    assert_eq!(Prefs::load().theme, Theme::System, "no key, no Glass");
    h.shot("150-prefs-glass-pro");
    prefs.global::<ProUi>().invoke_open(0);
    assert!(prefs.global::<ProUi>().get_show());
    prefs.global::<ProUi>().invoke_cancel();
    prefs.global::<Tokens>().set_glass(true);
    prefs.global::<Tokens>().set_frost(0.6);
    prefs.global::<PrefsUi>().set_theme(3); // shows the Glass slider
    h.shot("154-glass-prefs");
    prefs.global::<PrefsUi>().set_theme(0);
    prefs.global::<Tokens>().set_glass(false);

    // The mini view in glass.
    support::sample_zip(&dir.path().join("project-assets.zip"));
    let ui = h.open(&dir.path().join("project-assets.zip"));
    h.until("listing", || {
        g(&ui).get_status() == "ready" && !g(&ui).get_streaming()
    });
    ui.global::<Tokens>().set_glass(true);
    ui.global::<Tokens>().set_frost(0.6);
    h.shot("151-glass-mini");
    g(&ui).invoke_row_pressed(0, false, false);
    h.shot("152-glass-mini-selected");

    // The full view in glass.
    g(&ui).invoke_toggle_full();
    h.until("full", || g(&ui).get_full());
    h.shot("153-glass-full");

    // Dark glass: Glass follows the system's dark mode.
    ui.global::<crate_app::Palette>()
        .set_color_scheme(slint::language::ColorScheme::Dark);
    h.shot("155-glass-full-dark");
    g(&ui).invoke_toggle_full();
    h.until("mini", || !g(&ui).get_full());
    h.shot("156-glass-mini-dark");

    // Home in glass.
    let home = h.home(vec![]);
    home.global::<Tokens>().set_glass(true);
    home.global::<Tokens>().set_frost(0.6);
    h.shot("157-glass-home");
    home.global::<crate_app::Palette>()
        .set_color_scheme(slint::language::ColorScheme::Dark);
    h.shot("158-glass-home-dark");
}
