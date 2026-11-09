# M9: Pro: Glass theme + alternate icons

**Goal:** the first Pro features: the Liquid Glass theme and alternate app icons, unlocked with a license key.
**Done when:** buying a key, entering it, and switching to Glass works on all three platforms.

## Decisions (2026-10-31)
- [x] Pro model: honor-system license key, with all code public
- [x] Payment provider: Lemon Squeezy (merchant of record; its docs list bank payouts to Saudi Arabia in USD, and PayPal). Confirm when signing up that it still takes new sellers.
- [x] Pro price: $9, one-time
- [x] Key delivery: the buyer pastes Lemon Squeezy's license key into Arkyv, which exchanges it once with a small signer (a serverless function) for a key signed with Arkyv's private key. From then on the check is offline. Signed keys can also be pasted directly (sponsors, keys made by hand). This replaces "no phone-home" with "one call at activation, none after".

## Tasks
- [x] License check: signed key verified offline with a public key in the app; no account, one call at activation (0.2.18)
- [x] Upgrade sheet and license entry (designed in `Pro.dc.html`, built in 0.2.19); the "PRO" labels come with the Glass theme and the icons, the items they mark
- [x] Glass theme, the same window on every platform (`GlassMini*.dc.html`, `GlassBrowse*.dc.html`):
  - Arkyv draws its own frame (`no-frame`, transparent window background), with rounded corners and a shadow drawn in Slint
  - Its own resize handles + header drag region (Linux 0.2.28, Windows 0.2.29)
  - Windows: keep Snap Layouts. Windows shows them only when `WM_NCHITTEST` answers `HTMAXBUTTON` over the maximize button; winit has no hook for that, so hook the window procedure and hit-test Arkyv's own maximize button. Caption buttons 46 × 32, Close red on hover (DESIGN.md). Kept the native frame in M7 for this reason (2026-10-29)
  - Material: macOS vibrancy, Windows Acrylic (`window-vibrancy` crate on the winit window handle), Linux Frosted fallback (true blur only through the compositor / a blur extension). Slint can't blur inside the window, so panes are translucent fills over the OS material
  - Respect "Reduce transparency" → solid panes (0.3.2)
- [x] Alternate app icons (seven, from the Arkyv icon sheet, 2026-11-07): macOS can switch the Dock icon live; Windows/Linux update the shortcut/desktop icon (may need a restart)
  - [x] The App icon row in Preferences → Appearance, with PRO, and every window's icon following it (0.2.34)
  - [x] The taskbar pin / Start menu shortcut on Windows, the `.desktop` icon on Linux (GNOME on Wayland shows that one, not the window's), the Dock on macOS (M8)
- [ ] Give GitHub sponsors a free key (manual at first)
- [ ] README note for blur on Linux: "For the Glass theme's see-through blur on GNOME, install a blur extension such as Blur my Shell; KDE Plasma blurs natively." Test the exact extension name and setting yourself before publishing (moved from M6, 2026-10-28: the note ships with the theme)

## Notes

### License check (2026-10-31, 0.2.18)
- `app/src/license.rs`: `verify` (offline, Ed25519 with `PUBLIC_KEY`), `current` / `is_pro` (the key saved as `pro.key` in settings), `activate` (a `ARKYV-PRO-…` key is checked and saved; anything else goes to the signer as a Lemon Squeezy key; blocking, for a worker thread) and `remove`. Errors are an enum (`Rejected`, `Network`, `Unavailable`) for the upgrade sheet to word.
- Key: `ARKYV-PRO-` + base64url(payload + signature), payload `1|<id>|<YYYY-MM-DD>`, id `ls:<order>` or `gh:<sponsor>`. No name or email in the key.
- The private key is `~/crate-license/license-ed25519.pem` on the dev machine, made on 2026-10-31. It needs an offline backup, like ~/apple-signing.
- `signer/`: the Cloudflare Worker (asks Lemon Squeezy's License API, checks the store and product id, signs), `sign.mjs` for keys made by hand, and the deploy steps. The release workflow passes the repository variable `ARKYV_LICENSE_URL` to the build. Until it's set, builds take only signed keys.
- Tests: good and tampered keys, and `sign.mjs` keys checked by the Rust code. The Worker was tried by hand against a mocked Lemon Squeezy. Not tried yet: the real Lemon Squeezy API, which needs the store set up.
- **To do (you):** create the Lemon Squeezy store and the Arkyv Pro product, deploy the Worker (signer/README.md), set `ARKYV_LICENSE_URL`, and back up the private key.

### Upgrade sheet and key entry (2026-10-31, 0.2.19)
- Design: `design/canvas/Pro.dc.html` (upgrade sheet, key entry and its errors, thank you, the Arkyv Pro group, the PRO labels).
- `app/ui/pro.slint` (`ProUi`, `ProSheet`) over Preferences, and `app/src/pro.rs`: the group's two states, Buy, activation on a worker thread (a late answer after Cancel doesn't reopen the sheet, but a key that went through is kept), the error wording per `ActivateError`, Remove license.
- Buy opens `ARKYV_BUY_URL`, a repository variable passed to release builds like `ARKYV_LICENSE_URL`. Without it the button is hidden and "I have a license key" becomes the primary button. The price is `PRICE` in `pro.rs`.
- Not built yet: the PRO labels on Theme → Glass and the App icon row. They come with those features, so nothing points at something that doesn't exist. When Glass ships, change the thank-you text ("arrive … with the next updates") to the mockup's.
- Tested: `app/tests/pro_flow.rs` (shots 140–142, light and dark): the group, the sheet, a bad signed key, a store key in a build without the signer, Cancel. Not tested: the active state and the thank-you page with a real key (test builds can't sign one).
- **To do (you):** set `ARKYV_BUY_URL` to the Lemon Squeezy checkout once the product exists.

### Glass, part 1 (2026-10-31, 0.2.20)
- Plan: (1) the option, the tokens and the mini view; (2) the full view's floating panes (GlassBrowse); (3) Arkyv's own frame: corners, shadow, resize handles, window buttons; (4) the OS material: vibrancy, Acrylic/Mica, blur on KDE; (5) Reduce transparency and Snap Layouts.
- `Tokens.glass` (set per archive window by `Prefs::apply_archive_theme`) switches the tokens to the glass values in DESIGN.md. It also forces light. New tokens: `window` (the window background), `sheet` (cards and drop-downs, near-opaque in glass), `pro`.
- Theme 3 = Glass, read only with a valid key (`license::from_settings`), else System. Home and Preferences stay light under Glass.
- Slint 1.18's winit backend makes windows transparent on Linux and Windows, and on macOS when the background isn't opaque, so a translucent `Window.background` shows what's behind it. Without the OS material that's the desktop, unblurred: the panes are near-opaque so text stays readable.
- `PRO` badges: `ProBadge` in parts.slint; `Segmented` takes `badge-index` / `badge`.
- Tests: `glass_flow.rs` (shots 150–153). Screenshots now composite a translucent window over a stand-in wallpaper (`common::shot_now`).
- Not done: the PRO label on an App icon row (that row arrives with the icons). Done in 0.2.34.

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
- The app keeps `pro.store_key` and `pro.instance` beside the signed key. Remove license clears them and frees the activation on a worker thread; if that fails (offline) it goes into `pro.deactivate` and is retried when Preferences opens. The activation's name is "Arkyv on <os>": no host or user name.
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

### Arkyv's own frame on Linux (2026-11-03, 0.2.28)
- Slice 3, first half. Where Arkyv draws its header (`CUSTOM_FRAME`, Linux), in every theme: `app/ui/frame.slint`. The window keeps a 12 px clear margin around its body; the body has rounded corners (12 px, Ubuntu's; 16 px on glass, DESIGN.md's table) and a 1 px rim (`line`, `glass-rim` on glass).
- The shadow is 10 fading 1 px rings outside the body, not a `drop-shadow`: Slint draws that as one blurred shape under the body, and a see-through glass body would show it.
- Resizing: the margin is the grip, sides and 20 px corners, with the matching pointer; a press hands the resize to the window manager (`begin_window_resize`, like the header's move). Before this a Linux window couldn't be resized from its edges at all.
- Maximized or full screen (winit's state, checked whenever the size changes: `Frame.resized`), the margin, corners and shadow go away and the body fills the window.
- Rounded with the body: the headers, the footers, the full view's sidebar (its outer bottom corner, mirrored in Arabic), Home's body, the operations page and every dialog's dimmed backdrop (`Scrim`). Slint can't clip children to rounded corners (femtovg only through an offscreen layer, the test renderer not at all), so each edge piece rounds its own corners from `Frame.radius`.
- Limits: GNOME's half-screen tiling counts the clear margin as part of the window, so a tiled window keeps a 12 px gap (winit can't tell the compositor the body's geometry). X11 without a compositor shows the margin black.
- Tests: `glass_flow.rs` draws its windows in the frame; new shots 159 (mini), 160 (full), 161 (Home), plain theme.
- Next: Windows. Glass gets the same frame there with 46 × 32 caption buttons and Snap Layouts on maximize (`HTMAXBUTTON`); Windows 11 draws the 8 px corners and the shadow itself. macOS keeps its own frame: the traffic lights, corners and shadow are the system's, and Glass already runs under its transparent title bar.

### Arkyv's own frame on Windows, Snap Layouts (2026-11-03, 0.2.29)
- Slice 3, second half, and Snap Layouts from slice 5. On Windows, Glass drops the system title bar (`no-frame`) for `CaptionBar` (frame.slint): a 32 px strip with the app icon, the window title and 46 × 32 caption buttons (`caption-hover`, Close `caption-close` #C42B1C with a white icon), as in `GlassMiniWin.dc.html`. It's the drag region; a double-click maximizes. In Arabic the buttons sit at the left, Close first. The mini view's own header (name and round Close) steps aside for it. Other themes keep the system title bar.
- `Prefs::apply_window` (was `apply_blur`) sets `Frame.caption` with the theme and calls `platform::own_caption`, which on Windows:
  - keeps the system shadow (`set_undecorated_shadow`) and asks for Windows 11's rounded corners (`DWMWA_WINDOW_CORNER_PREFERENCE`, 8 px);
  - hooks the window procedure (`GWLP_WNDPROC`; not `SetWindowSubclass`: comctl32 exports it only by ordinal without a common-controls v6 manifest, so the test binaries didn't start, STATUS_ENTRYPOINT_NOT_FOUND on CI): `WM_NCHITTEST` answers the resize edges (6 px inside, scaled with the DPI; not when maximized) and `HTMAXBUTTON` over Arkyv's maximize button (46–92 px from the end, top 32 px), which is what makes Windows 11 show Snap Layouts there.
  - Windows then sends the pointer over that button as non-client messages: the hook turns them into the button's hover (`Frame.max-hover`, `TrackMouseEvent` for the leave) and a click into maximize/restore, and swallows the press so Windows doesn't draw its old button.
  - Switching away from Glass turns the hook off and gives the title bar back.
- Tests: shots 162 (mini) and 163 (full view, maximize hovered). **Not seen yet on Windows:** this machine can't build for Windows (no Windows C headers for the C dependencies); CI compiles it. To check there: drag, double-click, the three buttons, Snap Layouts on resting over maximize, resizing from every edge, the rounded corners and shadow, maximized and restored, and Arabic.

### Acrylic and Mica that reach the window (2026-11-03, 0.2.30)
- Your report on 0.2.29: macOS blur is right; Windows showed only a color. `apply_acrylic` set the system backdrop, but Windows paints a backdrop only under the window's frame, and Arkyv's frame stopped at the edges. `blur` now extends the frame over the whole window (`DwmExtendFrameIntoClientArea`, margins -1).
- With the frame over the window, Windows would draw its own caption buttons under the see-through strip, and it draws them only for a window with a system menu. The caption hook takes `WS_SYSMENU` off while Glass is on, and again when winit resets the style (`WM_STYLECHANGING`). So Alt+Space has no menu on Glass.
- Material (design: GlassMiniWin, GlassBrowseWin): `pref.glass_material`, 0 Acrylic, 1 Mica, a row under Glass on Windows only. Mica on Windows 10 falls back to Acrylic. `Tokens.mica` makes panes firmer (75%, as in the mockup). The mockup's 95% window fill stood in for the material itself, so the real window keeps its tint from the slider.
- Windows only paints either material while the window is active: an inactive window shows a plain tint. Transparency effects off, battery saver, or a VM without a GPU (Slint falls back to its software renderer, which has no alpha) also give a plain color.
- Also: a Preferences button in the full view's header; Preferences hides its own title where the system's title bar or the caption strip has it.
- Tests: `glass_flow.rs`, the setting and shot 164. **Not seen yet on Windows:** the backdrop, the hidden system buttons, Snap Layouts and Alt+F4 without the system menu.

### Windows: the material's own tint (2026-11-05, 0.2.32)
Tested in a Windows 11 VM: the software renderer does keep alpha (Home's pixels follow each backdrop type; Mica's come out as 40% white over `#F3F3F3`). So 0.2.31's see-through works, and the flat look has other causes:
- The VM has no GPU and its materials are flat colors, over Remote Desktop and on the console, so **the VM can't show blur**. Check blur on a machine with a GPU.
- The 40% window tint and the panes came from the mockups, where CSS blur has no tint. Real Acrylic and Mica have their own, so on Windows the window tint now starts at 0 and panes are lighter (`Tokens.material`).
- What looked fixed in Operations was Home showing through the page. Home isn't drawn under it now.
- It was still flat on a laptop with a GPU: the body one grey (106) from edge to edge. Windows 11's system Acrylic (`DWMSBT_TRANSIENTWINDOW`) shows its flat fallback whenever the window isn't active, and screenshot tools take the focus. So Acrylic is now the `SetWindowCompositionAttribute` blur, untinted, with Arkyv's own tint over it as in the mockup; Mica keeps the system backdrop.

### Windows draws in software (2026-11-04, 0.2.31)
- Your screenshots of 0.2.30, from two PCs with full GPU access: Preferences' body was #666, which is the 40% white tint shown opaque over black. So Windows was ignoring the window's alpha, whatever the backdrop behind it.
- Cause, as far as the sources go: FemtoVG on WGL. Slint's config picker tests `supports_transparency`, which on WGL means a color-keyed format, not alpha, and it has no way to ask for `PFD_SUPPORT_COMPOSITION`. Slint offers no hook for the pixel format. Not confirmed on a Windows machine.
- Fix: `select_backend` asks for the software renderer on Windows, in every theme, as one process can't mix renderers and the theme can change while it runs. softbuffer copies a 32-bit image with `BitBlt`; with the frame extended, alpha in that image is how classic Windows glass worked. Slint writes premultiplied alpha there.
- Rejected: the FemtoVG wgpu renderer through DX12 and DirectComposition, which is the modern route and may be needed if this falls short. It adds wgpu to the binary and to startup, and needs more code that can't run here.
- Cost: drawing is on the CPU. Slint repaints only what changed, and Arkyv's UI is simple, but large previews, big grids and maximized 4K windows are the places to watch. All test screenshots already use this renderer, so the look is known.
- **Not seen yet on Windows.** To check there: see-through with both materials; text, Arabic, and non-Latin file names; scrolling a long list and resizing at full size.

### App icons (2026-11-07, 0.2.34)
- Eight icons, drawn for the rename to Arkyv (Arkyv icon concepts canvas): Open Lid (today's, free) and seven Pro alternates: Ark, Zip, Layers (rounded corners), Squeeze, A monogram, Open box, Vault. Masters in `assets/icons/alt/*.svg`, 256 px PNGs in `assets/icons/alt/png/`, rendered with `resvg` (ImageMagick's own SVG renderer drops strokes and transforms).
- `Tokens.app-icon` picks from `Tokens.app-icons`; every window's `icon` and the Glass caption bar use `Tokens.app-image`. `pref.app_icon` counts only with a key, like Glass, else Open Lid. `apply_theme` sets it, so it follows the theme's path to the window that opened Preferences, and the others as they next open.
- The row: title and PRO, then the eight icons under it (beside the title they don't fit at the window's minimum width). Without a key the alternates are faded and open the upgrade sheet; arrows step through them only with a key. Each is a checkable button with its name, for screen readers.
- What shows the window icon: the Windows taskbar and Alt+Tab, X11 docks. Not yet: a pinned taskbar shortcut, the Start menu, GNOME on Wayland (it shows the `.desktop` file's icon), the macOS Dock.
- Tested: `glass_flow.rs`, shots 165 (no key) and 166 (an alternate chosen).
- Later the same day: Layers became the app icon everywhere (`assets/icons/arkyv-icon.svg`, the PNGs, `.ico`, `.icns`) and the free choice; Open Lid moved to `alt/` as a Pro alternate (0.2.35).

### Windows: Clear (2026-11-08, 0.2.37)
- On a real PC with a GPU (0.2.36), neither material looked like the mockup. Mica only takes a color from the wallpaper and never shows other windows; Acrylic blurs so heavily that Explorer behind it is a soft wash, and Windows has no public way to make the blur lighter. Arkyv's own white (40% window, 55% panes at the clear end) then covered most of it. Acrylic also looked the same tan over Explorer as over the sky, so it may have been showing a fallback color: to check (Transparency effects on? half the window over Explorer, half over the wallpaper).
- A third Material, **Clear**: no system material, only the extended frame, so the software renderer's see-through pixels show the windows and wallpaper behind as they are. Tokens treat it like Mica (`Tokens.material`): no window tint at frost 0, panes from 30% white. No blur: Windows offers none that can be turned down.
- `platform::Material` (Acrylic 0, Mica 1, Clear 2) replaces the `glass_mica` flag; `pref.glass_material` keeps its numbers.
- 0.2.37 on the PC: Clear worked in Preferences only; Home and archive windows were solid white. Reproduced in the VM (every pixel 255). Clearing Acrylic and Mica (`window_vibrancy::clear_*`, the accent off) also drops the "blur behind" that winit sets on transparent windows, and without it DWM ignores the window's alpha. Setting it again over an empty region (`see_through`) made the archive window see-through at once in the VM, Notepad and the wallpaper sharp behind it (0.2.38). Preferences had kept it by chance, switching materials while open.

### The app icon beyond Arkyv's windows, and Reduce transparency (2026-11-08, 0.3.2)
- `app/src/app_icon.rs` holds the eight icons (256 px PNGs and the SVGs, in `Tokens.app-icons`' order). `Prefs::apply_theme` calls `app_icon::sync`, so a change in Preferences, a removed license (back to Layers) or a fresh install catches up wherever a window applies the preferences. On Linux and Windows the file work runs once per change on its own thread; `~/.local/state/arkyv/app-icon` keeps which icon the system files show.
- **Linux:** every size (16–512) and the SVG go into the user's icon theme (`~/.local/share/icons/hicolor/<size>/apps/io.github.mlbrainlab.arkyv.png`). The theme lookup searches the user's folder before `/usr/share` size by size, so the `.desktop` entry's icon name finds it: GNOME's dock and app grid, the app menu. Layers deletes them, which brings back the installed icon (it also deletes the dev install's copies of Layers; `scripts/dev-install-desktop.sh` puts them back). A user icon cache, if there is one, is rebuilt. Not in the Flatpak or the snap: their sandboxes can't write there. Tested: `app_icon::tests` writes and removes the files in a temp theme.
- **Windows:** an .ico (16–256 px) in `%LOCALAPPDATA%\arkyv\icons`; the Start menu shortcut (the installer's, per user or for everyone) and every taskbar pin whose target is `arkyv-app.exe` get it as their icon (`IShellLinkW::SetIconLocation`), and the shell is told (`SHChangeNotify`). Layers clears the shortcuts' icon so they show the executable's again. Shortcuts for everyone need administrator rights; without them only the user's change. Explorer may show the old icon until it restarts.
- **macOS:** the running process's Dock icon (`setApplicationIconImage`), the tile shrunk onto macOS's icon grid (824 on 1024); Layers gives the bundle's back. Each window is a process, so each sets it as it opens. Finder and the Dock show the bundle's icon while Arkyv isn't running: changing the bundle would break its signature.
- **Reduce transparency:** `platform::reduce_transparency` (macOS: Accessibility → Display; Windows: Transparency effects off; nothing on Linux). With it, Glass keeps its layout and controls but its panes are solid (frost 1, no material) and no blur is asked for. Read when a window applies the preferences, not watched live.
- README: Snap and the macOS .dmg in the install table, and the note on blur for Glass on GNOME (Blur my Shell's application blur) and KDE.
- Windows VM, 0.3.2 (2026-11-10): picking Vault set the Start menu shortcut's icon to `%LOCALAPPDATA%\arkyv\icons\vault.ico`, and Layers cleared it again. But the 0.3.2 installer had recreated the shortcut without an icon after an earlier run had set it, and the stamp said it was done, so it stayed plain until the stamp was removed. 0.3.3: Windows keeps no stamp (`platform::APP_ICON_STAMP`); each process checks the shortcuts and saves only those that differ, so updates and later pins get the icon on the next start. The .ico is written once.
- **To check:** each system shows the picked icon in the places above (log out and in on GNOME if the dock keeps the old one); Layers brings the original back; with Reduce transparency on, Glass is solid; the README's Blur my Shell steps work as written.

## Acceptance
- [ ] Glass looks the same on all 3 OSes (shape, layout, controls); only the blur strength differs
- [ ] Without a key, Glass and the icons show a "PRO" label and the upgrade sheet; nothing else is locked

## Prompt for Claude Code
> Read CLAUDE.md, docs/milestones/M9-pro-glass.md and the Glass section of design/DESIGN.md. Build the license check in crate-core-independent code (`app/src/license.rs`).
