//! The Glass theme (Pro): the Theme row in Preferences, and archive windows
//! in glass. Test builds can't make a real key, so the windows get glass the
//! way `Prefs::apply_theme` gives it to them, frosted as on Linux, in
//! Arkyv's own frame (Linux). Shots are composited over a stand-in
//! wallpaper.

#[path = "../../crates/crate-core/tests/support/mod.rs"]
mod support;

mod common;

use arkyv_app::platform::Material;
use arkyv_app::prefs::{Prefs, Theme};
use arkyv_app::{PrefsUi, ProUi, Tokens};
use common::{Harness, g};
use slint::{ComponentHandle, Model};

/// Arkyv's own frame and header, as on Linux.
macro_rules! linux_frame {
    ($ui:expr) => {{
        $ui.set_custom_frame(true);
        arkyv_app::connect_frame($ui, true);
    }};
}

#[test]
fn glass_flow() {
    let h = Harness::new();
    let dir = tempfile::tempdir().unwrap();

    let prefs = h.prefs();
    linux_frame!(&prefs);
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
    // Windows: the Material row, and no second "Preferences" under the
    // system's title.
    prefs
        .global::<PrefsUi>()
        .invoke_choose("glass_material".into(), 1);
    assert_eq!(Prefs::load().glass_material, Material::Mica);
    assert_eq!(prefs.global::<PrefsUi>().get_glass_material(), 1);
    prefs
        .global::<PrefsUi>()
        .invoke_choose("glass_material".into(), 2);
    assert_eq!(Prefs::load().glass_material, Material::Clear);
    prefs
        .global::<PrefsUi>()
        .invoke_choose("glass_material".into(), 0);
    assert_eq!(Prefs::load().glass_material, Material::Acrylic);
    // (Choosing refills the window from the saved settings: no key, no Glass.)
    prefs.global::<Tokens>().set_glass(true);
    prefs.global::<Tokens>().set_frost(0.6);
    prefs.global::<PrefsUi>().set_theme(3);
    prefs.global::<PrefsUi>().set_glass_materials(true);
    prefs.set_custom_frame(false);
    prefs.global::<arkyv_app::Frame>().set_custom(false);
    prefs.global::<arkyv_app::Frame>().set_caption(true);
    h.shot("164-glass-prefs-windows");
    prefs.global::<arkyv_app::Frame>().set_caption(false);
    prefs.global::<PrefsUi>().set_glass_materials(false);

    // App icon: without a key the alternates carry PRO, and choosing one
    // keeps Layers.
    prefs
        .global::<PrefsUi>()
        .invoke_choose("app_icon".into(), 3);
    assert_eq!(Prefs::load().app_icon, 0, "no key, no alternate icon");
    assert_eq!(prefs.global::<Tokens>().get_app_icon(), 0);
    assert_eq!(prefs.global::<Tokens>().get_app_icons().row_count(), 8);
    h.shot("165-app-icon-pro");
    // With a key, as `apply_theme` gives it: Zip chosen, the window's
    // icon follows.
    prefs.global::<ProUi>().set_active(true);
    prefs.global::<PrefsUi>().set_app_icon(3);
    prefs.global::<Tokens>().set_app_icon(3);
    h.shot("166-app-icon");
    prefs.global::<ProUi>().set_active(false);
    prefs.global::<PrefsUi>().set_app_icon(0);
    prefs.global::<Tokens>().set_app_icon(0);
    linux_frame!(&prefs);
    prefs.global::<PrefsUi>().set_theme(0);
    prefs.global::<Tokens>().set_glass(false);

    // The mini view in glass.
    support::sample_zip(&dir.path().join("project-assets.zip"));
    let ui = h.open(&dir.path().join("project-assets.zip"));
    linux_frame!(&ui);
    h.until("listing", || {
        g(&ui).get_status() == "ready" && !g(&ui).get_streaming()
    });
    h.shot("159-frame-mini");
    g(&ui).invoke_toggle_full();
    h.until("full", || g(&ui).get_full());
    h.shot("160-frame-full");
    g(&ui).invoke_toggle_full();
    h.until("mini", || !g(&ui).get_full());
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
    ui.global::<arkyv_app::Palette>()
        .set_color_scheme(slint::language::ColorScheme::Dark);
    h.shot("155-glass-full-dark");
    g(&ui).invoke_toggle_full();
    h.until("mini", || !g(&ui).get_full());
    h.shot("156-glass-mini-dark");

    // Windows: the system's frame, but Arkyv's caption strip on glass.
    ui.global::<arkyv_app::Palette>()
        .set_color_scheme(slint::language::ColorScheme::Light);
    ui.set_custom_frame(false);
    ui.global::<arkyv_app::Frame>().set_custom(false);
    ui.global::<arkyv_app::Frame>().set_caption(true);
    h.shot("162-glass-mini-windows");
    ui.global::<arkyv_app::Frame>().set_max_hover(true);
    g(&ui).invoke_toggle_full();
    h.until("full", || g(&ui).get_full());
    h.shot("163-glass-full-windows");

    // Home in glass.
    let home = h.home(vec![]);
    linux_frame!(&home);
    h.shot("161-frame-home");
    home.global::<Tokens>().set_glass(true);
    home.global::<Tokens>().set_frost(0.6);
    h.shot("157-glass-home");
    home.global::<arkyv_app::Palette>()
        .set_color_scheme(slint::language::ColorScheme::Dark);
    h.shot("158-glass-home-dark");
}
