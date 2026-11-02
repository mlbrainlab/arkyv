# M9: Pro: Glass theme + alternate icons

**Goal:** the first Pro features: the Liquid Glass theme and alternate app icons, unlocked with a license key.
**Done when:** buying a key, entering it, and switching to Glass works on all three platforms.

## Decisions (2026-10-31)
- [x] Pro model: honor-system license key, with all code public
- [x] Payment provider: Lemon Squeezy (merchant of record; its docs list bank payouts to Saudi Arabia in USD, and PayPal). Confirm when signing up that it still takes new sellers.
- [x] Pro price: $9, one-time
- [x] Key delivery: the buyer pastes Lemon Squeezy's license key into Crate, which exchanges it once with a small signer (a serverless function) for a key signed with Crate's private key. From then on the check is offline. Signed keys can also be pasted directly (sponsors, keys made by hand). This replaces "no phone-home" with "one call at activation, none after".

## Tasks
- [x] License check: signed key verified offline with a public key in the app; no account, one call at activation (0.2.18)
- [x] Upgrade sheet and license entry (designed in `Pro.dc.html`, built in 0.2.19); the "PRO" labels come with the Glass theme and the icons, the items they mark
- [ ] Glass theme, the same window on every platform (`GlassMini*.dc.html`, `GlassBrowse*.dc.html`):
  - Crate draws its own frame (`no-frame`, transparent window background), with rounded corners and a shadow drawn in Slint
  - Its own resize handles + header drag region
  - Windows: keep Snap Layouts. Windows shows them only when `WM_NCHITTEST` answers `HTMAXBUTTON` over the maximize button; winit has no hook for that, so subclass the window procedure (`SetWindowSubclass`) and hit-test Crate's own maximize button. Caption buttons 46 × 32, Close red on hover (DESIGN.md). Kept the native frame in M7 for this reason (2026-10-29)
  - Material: macOS vibrancy, Windows Acrylic (`window-vibrancy` crate on the winit window handle), Linux Frosted fallback (true blur only through the compositor / a blur extension). Slint can't blur inside the window, so panes are translucent fills over the OS material
  - Respect "Reduce transparency" → solid panes
- [ ] Alternate app icons (4 concepts): macOS can switch the Dock icon live; Windows/Linux update the shortcut/desktop icon (may need a restart)
- [ ] Give GitHub sponsors a free key (manual at first)
- [ ] README note for blur on Linux: "For the Glass theme's see-through blur on GNOME, install a blur extension such as Blur my Shell; KDE Plasma blurs natively." Test the exact extension name and setting yourself before publishing (moved from M6, 2026-10-28: the note ships with the theme)

## Notes

### License check (2026-10-31, 0.2.18)
- `app/src/license.rs`: `verify` (offline, Ed25519 with `PUBLIC_KEY`), `current` / `is_pro` (the key saved as `pro.key` in settings), `activate` (a `CRATE-PRO-…` key is checked and saved; anything else goes to the signer as a Lemon Squeezy key; blocking, for a worker thread) and `remove`. Errors are an enum (`Rejected`, `Network`, `Unavailable`) for the upgrade sheet to word.
- Key: `CRATE-PRO-` + base64url(payload + signature), payload `1|<id>|<YYYY-MM-DD>`, id `ls:<order>` or `gh:<sponsor>`. No name or email in the key.
- The private key is `~/crate-license/license-ed25519.pem` on the dev machine, made on 2026-10-31. It needs an offline backup, like ~/apple-signing.
- `signer/`: the Cloudflare Worker (asks Lemon Squeezy's License API, checks the store and product id, signs), `sign.mjs` for keys made by hand, and the deploy steps. The release workflow passes the repository variable `CRATE_LICENSE_URL` to the build. Until it's set, builds take only signed keys.
- Tests: good and tampered keys, and `sign.mjs` keys checked by the Rust code. The Worker was tried by hand against a mocked Lemon Squeezy. Not tried yet: the real Lemon Squeezy API, which needs the store set up.
- **To do (you):** create the Lemon Squeezy store and the Crate Pro product, deploy the Worker (signer/README.md), set `CRATE_LICENSE_URL`, and back up the private key.

### Upgrade sheet and key entry (2026-10-31, 0.2.19)
- Design: `design/canvas/Pro.dc.html` (upgrade sheet, key entry and its errors, thank you, the Crate Pro group, the PRO labels).
- `app/ui/pro.slint` (`ProUi`, `ProSheet`) over Preferences, and `app/src/pro.rs`: the group's two states, Buy, activation on a worker thread (a late answer after Cancel doesn't reopen the sheet, but a key that went through is kept), the error wording per `ActivateError`, Remove license.
- Buy opens `CRATE_BUY_URL`, a repository variable passed to release builds like `CRATE_LICENSE_URL`. Without it the button is hidden and "I have a license key" becomes the primary button. The price is `PRICE` in `pro.rs`.
- Not built yet: the PRO labels on Theme → Glass and the App icon row. They come with those features, so nothing points at something that doesn't exist. When Glass ships, change the thank-you text ("arrive … with the next updates") to the mockup's.
- Tested: `app/tests/pro_flow.rs` (shots 140–142, light and dark): the group, the sheet, a bad signed key, a store key in a build without the signer, Cancel. Not tested: the active state and the thank-you page with a real key (test builds can't sign one).
- **To do (you):** set `CRATE_BUY_URL` to the Lemon Squeezy checkout once the product exists.

### Glass, part 1 (2026-10-31, 0.2.20)
- Plan: (1) the option, the tokens and the mini view; (2) the full view's floating panes (GlassBrowse); (3) Crate's own frame: corners, shadow, resize handles, window buttons; (4) the OS material: vibrancy, Acrylic/Mica, blur on KDE; (5) Reduce transparency and Snap Layouts.
- `Tokens.glass` (set per archive window by `Prefs::apply_archive_theme`) switches the tokens to the glass values in DESIGN.md. It also forces light. New tokens: `window` (the window background), `sheet` (cards and drop-downs, near-opaque in glass), `pro`.
- Theme 3 = Glass, read only with a valid key (`license::from_settings`), else System. Home and Preferences stay light under Glass.
- Slint 1.18's winit backend makes windows transparent on Linux and Windows, and on macOS when the background isn't opaque, so a translucent `Window.background` shows what's behind it. Without the OS material that's the desktop, unblurred: the panes are near-opaque so text stays readable.
- `PRO` badges: `ProBadge` in parts.slint; `Segmented` takes `badge-index` / `badge`.
- Tests: `glass_flow.rs` (shots 150–153). Screenshots now composite a translucent window over a stand-in wallpaper (`common::shot_now`).
- Not done: the PRO label on an App icon row (that row arrives with the icons).

### Glass, part 2 (2026-10-31, 0.2.21)
- Full view: the window gets 12 px padding; the sidebar and the content column are panes (22 px radius, glass fill, white border); the header sits on the glass with no line under it. `Button` and `ToolButton` are pills under Glass.
- Kept from the normal layout rather than GlassBrowse's: the toolbar row and the header with the archive name. GlassBrowse groups the tools into capsules beside the folder name; that's a bigger rework, left for when the rest of Glass works on the real OS windows.

### Glass on every window, and dark glass (2026-10-31, 0.2.22)
- From your first try on Ubuntu with 0.2.21: Home and Preferences didn't change (by design then: archive windows only), and Glass needs a dark version. Now `Prefs::apply_theme(palette, tokens)` sets glass on every window, and Glass follows the system's light or dark (color scheme Unknown). Dark glass tokens are in DESIGN.md. New token `card` for Preferences groups.
- Bug: Preferences keeps the `Settings` it opened with, so after entering a key there, choosing Glass read no key and fell back to System. `Prefs::read` now also checks the key on disk.
- Still open from the same report: "no glass effect" on the real Ubuntu window. Slint creates winit windows with `transparent: true`, and femtovg asks for an alpha GL config, so the desktop should show through the window. Check this with 0.2.22 first; it may have been the bug above.

### See-through on Linux (2026-10-31, 0.2.23)
- Your 0.2.22 screenshots: Home, Preferences and the mini view came out white, with the glass controls invisible, while the panes and pills were there. Cause: a translucent `Window.background` is femtovg's GL clear color, passed straight to `glClearColor` without premultiplying. Wayland compositors read the buffer as premultiplied, so white at 40% became solid white. Glass windows now set `background: transparent` (cleared to 0,0,0,0) and draw `Tokens.window` in a Rectangle under everything.
- Light glass `sheet` is `#EEF1F6` instead of white: white glass controls (segmented track, chips) vanished on a white card.
- Found on the way: `Card { clip: true }` doesn't round its children's corners, so the first-run footer stuck out at the bottom corners in every theme. The footer rounds its own bottom corners now.

### Frosted on Linux, glass buttons (2026-10-31, 0.2.24)
- From your 0.2.23 screenshot: see-through works, but with no blur on GNOME the text over other windows was barely readable. `Tokens.frosted` (`platform::GLASS_FROSTED`, Linux only) raises the window tint to 75% and the panes to 85% (DESIGN.md's "Frosted" variant). Real blur on KDE stays in slice 4.
- Buttons on glass, as you asked: `IconButton`, `WindowButton` and the mini view's close button are circles; `Button` and `ToolButton` are pills; all with `glass-fill` and a white `glass-rim`, and a soft shadow so the rim reads on light panes.

### The Glass slider (2026-10-31, 0.2.25)
- Asked for after 0.2.24 ("not frosted at all": the window was still the 0.2.23 build, but a control is better than a fixed value anyway). `Tokens.frost` (0–1) replaces the `frosted` flag: window tint 40% → 100%, panes 55% → 100% (dark: 45% / 65% → 100%). Saved as `pref.glass_frost` (0–100); `platform::GLASS_FROST` is the default, 60 on Linux, 0 elsewhere.
- `FrostSlider` in prefs-window.slint: drag sets `Tokens.frost` live in Preferences and saves on release; arrows step 5%; mirrored for Arabic; accessible as a slider. The row shows only while the theme is Glass. The window that opened Preferences follows on release; other archive windows (their own processes) on their next open.

### Five computers per key (2026-10-31, 0.2.26)
- Your decision: a key activates at most 5 computers. The signer now uses Lemon Squeezy's `activate` (one activation per computer) instead of `validate`, refuses with 409 past `MAX_ACTIVATIONS` (5, checked in the Worker too, so a product without a limit in Lemon Squeezy still stops at 5), and has `POST /deactivate`. A computer that sends its saved `instance_id` is validated and signed again without a new activation.
- The app keeps `pro.store_key` and `pro.instance` beside the signed key. Remove license clears them and frees the activation on a worker thread; if that fails (offline) it goes into `pro.deactivate` and is retried when Preferences opens. The activation's name is "Crate on <os>": no host or user name.
- Signed keys made with `sign.mjs` (sponsors) skip Lemon Squeezy and have no limit. Reinstalling or wiping the settings without Remove license leaves an activation behind; you can free it in the Lemon Squeezy dashboard.
- Tests: `app/tests/license_flow.rs` against a stand-in signer on localhost; the Worker by hand against a stand-in Lemon Squeezy (5 activations, the 6th refused, the same computer again, deactivate, a key that isn't ours).
- **To do (you):** set the product's activation limit to 5 in Lemon Squeezy, and redeploy the Worker.

### Blur behind Glass windows (2026-11-02, 0.2.27)
- Slice 4. `platform::set_blur` (called by `Prefs::apply_blur` wherever a window's theme is applied) waits for the winit window, then each platform's `blur`:
  - Windows: Acrylic through `window-vibrancy` (`apply_acrylic` / `clear_acrylic`). On Windows 11 22H2+ that is the system backdrop; on Windows 10 1809+ the older accent blur, which can lag while dragging the window. Older systems: see-through only.
  - macOS: winit's `set_blur`, the window-server blur behind the window (radius 80). Not `NSVisualEffectView` vibrancy: that view would have to sit under Slint's drawing, which is winit's own content view, so it would cover it.
  - Linux: winit's `set_blur`, KWin's `org_kde_kwin_blur` on Wayland. GNOME has no blur protocol, and winit doesn't set KDE's blur property on X11, so those stay unblurred.
- `platform::glass_frost()` replaces `GLASS_FROST`: the slider starts at 0 (clear glass) on macOS, Windows and KDE Wayland, and at 60% on GNOME, X11 and anything else.
- Checked here: builds and the tests pass on Ubuntu (GNOME: no change). **Not seen yet:** Windows, macOS and KDE; CI only builds them.

## Acceptance
- [ ] Glass looks the same on all 3 OSes (shape, layout, controls); only the blur strength differs
- [ ] Without a key, Glass and the icons show a "PRO" label and the upgrade sheet; nothing else is locked

## Prompt for Claude Code
> Read CLAUDE.md, docs/milestones/M9-pro-glass.md and the Glass section of design/DESIGN.md. Build the license check in crate-core-independent code (`app/src/license.rs`).
