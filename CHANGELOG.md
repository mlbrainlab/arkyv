# Changelog

Every finished task bumps the version (see "Versions" in `docs/ROADMAP.md`): the patch number per task, 0.1.0 for the first Linux release (M6), 0.2.0 Windows (M7), 0.3.0 macOS (M8), and 1.0.0 with the final milestone (M9).

## 0.3.4 — 2026-11-13
- Full view: a Type column ("PNG image", "Folder", "SH source"), which sorts by kind. When the list is narrow the optional Ratio column gives way to it first.
- Full view: "Extract N selected" appears beside Extract all whenever something is selected. Extract all now extracts everything without clearing the selection.

## 0.3.3 — 2026-11-10
- Windows, Arkyv Pro: the app icon you picked comes back on the Start menu entry after an update (the installer recreates it), and reaches taskbar pins made later, the next time Arkyv starts.

## 0.3.2 — 2026-11-08
- Arkyv Pro: the app icon you pick now shows outside Arkyv's windows too: in GNOME's dock and app grid on Linux (not in the Flatpak or snap), on the Start menu entry and taskbar pins on Windows, and in the Dock on macOS while Arkyv runs. Picking Layers again brings the original back. Some desktops show the change only after logging out and in.
- Glass follows the system's "Reduce transparency" (macOS) or "Transparency effects" off (Windows): the panes go solid and nothing is blurred behind the window.
- README: the Snap Store and the macOS download in the install table, and how to get Glass's blur on GNOME.

## 0.3.1 — 2026-11-08
- M5 (Linux polish) is done: the last checks passed on Ubuntu (making Arkyv the default from first run, the Arabic UI, keyboard-only use, a RAR set with a missing part, and Orca).

## 0.3.0 — 2026-11-08
Arkyv for macOS.

- **Install:** `Arkyv-0.3.0-universal.dmg` below, for macOS 12 and later on Apple Silicon and Intel. It's signed with Arkyv's Developer ID and notarized by Apple, so it opens without the "unidentified developer" warning. A Homebrew cask is ready (`scripts/homebrew-cask.py`).
- **At home on the Mac:** the traffic lights sit in Arkyv's own header, the menus are in the menu bar with their ⌘ shortcuts (and a Window menu), Finder's keys work in archives (⌘↑, ⌘↓, ⌘⌫, Return to rename), and the UI uses San Francisco (SF Arabic in Arabic).
- **Open With and default app:** Arkyv shows in Finder's Open With for ZIP, 7z, RAR, tar and the compressed tars, zst, xz and split sets; first run and Preferences make it the default for the types you pick. Double-clicking an archive opens the mini view.
- **Progress and folder access:** extraction progress on the Dock icon; when macOS keeps Arkyv out of a folder, the card says where to allow it and opens the right settings page.
- **Glass (Pro):** the window server's blur behind the window, from a light blur at Clear to a heavy one at Frosted.

## 0.2.42 — 2026-11-08
- Home, About and the first-run screen show Arkyv's app icon (or the Pro icon you picked) instead of a generic box.
- macOS: the mini view puts the traffic lights in its own header, like Home and the full view, so the top of the window is no longer see-through with Glass on. The red light closes it, so its own Close button is gone there.
- macOS Glass: the slider now goes from a light blur (Clear) to a heavy one (Frosted), and the glass stays light at every setting; before, the blur was always heavy and Frosted ended in solid white.

## 0.2.41 — 2026-11-08
- Snap Store listing artwork in `assets/store/`: a dark version of the icon and a 1920×640 banner.
- Releases: a "Snap release" workflow moves an uploaded version to a Snap Store channel (back to edge, or on to stable) for every architecture.

## 0.2.40 — 2026-11-08
- Linux: Arkyv is on the Snap Store (`sudo snap install arkyv --edge` for now). Each release uploads it for amd64 and arm64; `arkyv` is the command-line tool and `arkyv.app` the app.
- In the snap, "Open these with Arkyv" explains how to make Arkyv the default in Files, as in the Flatpak: confinement keeps snaps from changing default apps themselves.
- Store screenshots taken from the app on Ubuntu, in `docs/store-screenshots/` (`scripts/store-screenshots.sh` makes them again).
- Releases: the release workflow retries downloads when GitHub answers 503/504, and the Snap Store upload no longer waits for the GitHub release.
- `arkyv --version` says arkyv, not crate.

## 0.2.38 — 2026-11-08
- Windows, Glass with Clear: Home and archive windows are see-through too. Before, only Preferences was; the others showed solid white.

## 0.2.37 — 2026-11-08
- Windows, Glass theme: a new Material choice, Clear. Other windows and the wallpaper show through as they are, sharp, with only a light tint behind the text. Acrylic and Mica stay for a frosted look.

## 0.2.36 — 2026-11-07
- New name: Arkyv. The app, its windows, the installers and the command-line tool (`arkyv`) all carry it, and the project lives at github.com/mlbrainlab/arkyv.
- Updating keeps everything: preferences, history and a Pro license move over the first time Arkyv starts. On Windows the update replaces the old install, its Start menu entry and its right-click menu items; on Linux the new package replaces the old one.
- Pro keys now start with ARKYV-PRO. Keys given out before still work.

## 0.2.35 — 2026-11-07
- New app icon: Layers, three stacked slabs, everywhere Arkyv's icon appears. Open Lid, the old icon, is now one of the Pro icons.
- README: the app icon at the top, and a line on Arkyv Pro.

## 0.2.34 — 2026-11-07
- Arkyv Pro: pick the app icon in Preferences → Appearance, from Open Lid and seven new ones. Arkyv's windows and their taskbar buttons show the one you pick. Pinned shortcuts, the Linux app menu and the macOS Dock follow later.

## 0.2.33 — 2026-11-07
- The project moved to github.com/mlbrainlab/arkyv, ahead of the app's new name, Arkyv. Old links redirect. About, the installers and the store listings point at the new address.

## 0.2.32 — 2026-11-05
- Windows Glass: Acrylic is now a real see-through blur that keeps working when the window isn't active, with Arkyv's tint as in the design. Before, it showed as a flat grey. Arkyv no longer lays its own tint over Mica, which already has one.
- Glass: the Home page no longer shows through Operations.

## 0.2.31 — 2026-11-04
- Windows: Arkyv now draws its windows without OpenGL, so Glass can really be see-through, with Acrylic or Mica behind it. Before, Windows showed Glass windows as plain grey.

## 0.2.30 — 2026-11-03
- Windows, Glass theme: the blur now reaches the whole window, where before it stayed hidden behind the window's content. A new Material choice under Preferences → Appearance picks Acrylic (see-through blur) or Mica (tinted by the wallpaper, Windows 11).
- Full view: a Preferences button in the header.
- Windows: Preferences shows its title once, in the title bar.

## 0.2.29 — 2026-11-03
- Windows, Glass theme: Arkyv draws its own slim title bar with Windows-style minimize, maximize and close buttons, and keeps the system's shadow and rounded corners. Snap Layouts still appear when you rest the pointer on maximize, and the window resizes from every edge.

## 0.2.28 — 2026-11-03
- Linux: Arkyv draws its whole window frame: rounded corners, a soft shadow and a thin rim, in every theme. Windows can now be resized by dragging their edges and corners. Maximized, the window fills the screen edge to edge.

## 0.2.27 — 2026-11-02
- Glass theme: real blur behind the window where the system has it: Acrylic on Windows, the window blur on macOS, and KDE Plasma's blur on Wayland. There Glass starts as clear glass; on GNOME it stays at 60% frosted, since GNOME doesn't blur.

## 0.2.26 — 2026-10-31
- Pro: a Lemon Squeezy key works on up to 5 computers. Entering it again on the same computer doesn't use another, and Remove license frees that computer's place (or the next time Preferences opens, if the computer was offline). Keys signed by hand, for sponsors, have no limit.

## 0.2.25 — 2026-10-31
- Glass theme: a Glass slider in Preferences → Appearance, from clear glass (a 40% tint) to fully frosted. Every Arkyv window follows it; Preferences shows each step while you drag. It starts at 60% on Linux, where GNOME doesn't blur, and at clear glass elsewhere.

## 0.2.24 — 2026-10-31
- Glass theme on Linux is frosted: GNOME doesn't blur what's behind a window, so the tint and panes are stronger and text stays readable over other windows and the wallpaper. macOS and Windows keep the lighter glass, for the blur they'll get.
- Glass theme: every button is glass with a white rim: icon buttons are circles, the rest pills.

## 0.2.23 — 2026-10-31
- Glass theme on Linux: windows are see-through, not white. The glass tint was the window's clear color, which isn't premultiplied, and GNOME showed it as solid white. Glass windows are now cleared to transparent and the tint is drawn inside them.
- Glass theme: light-glass dialogs are a soft grey-blue, so the glass controls in them show; the first-run card's footer keeps the card's rounded corners.

## 0.2.22 — 2026-10-31
- Glass theme: Home and Preferences are glass too, and Glass has a dark version that follows the system's dark mode.
- Glass theme: choosing Glass right after entering a key in the same Preferences window now works. Before, Preferences didn't see the new key, so it switched to System instead.
- Glass theme: dialogs over glass are solid, so the window behind doesn't show through their text.

## 0.2.21 — 2026-10-31
- Glass theme: in the full view the sidebar and the content float as glass panes in the window, and buttons turn into pills.

## 0.2.20 — 2026-10-31
- Pro (M9): the Glass theme, first part. Preferences → Theme has Glass, with a PRO label; without a key it opens the upgrade sheet. With Glass, archive windows are translucent white glass: the mini view's panes float in the window, with a pill search field and a pill Extract button. The full view picks up the glass colors; its floating panes, Arkyv's own window frame and the real blur behind the window come next.

## 0.2.19 — 2026-10-31
- Pro (M9): Preferences has an Arkyv Pro group. Upgrade… opens a sheet that says what Pro adds ($9 once), with Buy (once the store exists) and "I have a license key". Entering a key activates Pro, and Remove license takes it off. Nothing in Arkyv is locked yet: the Glass theme and the extra icons, the only Pro features, are still to come.

## 0.2.18 — 2026-10-31
- Pro (M9), groundwork: the license check. A Arkyv Pro key is signed with Arkyv's private key and checked offline with the public key in the app. A Lemon Squeezy key is exchanged once for a signed key through a small signer (`signer/`); nothing is sent after that. Nothing in the app uses it yet.

## 0.2.17 — 2026-10-31
- macOS: making Arkyv the default app waits for macOS's answer, so Preferences shows the new state right away and a failure shows as an error instead of nothing happening. The first-run screen and Preferences no longer say macOS asks you to confirm: it doesn't, for file types.

## 0.2.16 — 2026-10-31
- macOS: a Window menu in the menu bar, with Minimize (⌘M) and Zoom, in the home and archive windows.

## 0.2.15 — 2026-10-31
- Full view: long folder and archive names in the breadcrumbs shorten with "…" instead of pushing the file list past the window's right edge (the Packed, Ratio and Modified columns were cut off unless the window was maximized).
- Full view: in a narrow window the toolbar shows its buttons' icons only, so it fits down to the window's minimum width.

## 0.2.14 — 2026-10-31
- Windows: Arkyv starts on a fresh Windows. The Visual C++ runtime is now built into the binaries instead of needing VCRUNTIME140.dll and MSVCP140.dll, and packaging fails if a binary needs them again.

## 0.2.13 — 2026-10-31
- Tests: the CLI's Ctrl+C test no longer loses the race against a fast disk (macOS runners finished the extract before the signal arrived). With `ARKYV_TEST_SLOW` set, the CLI pauses at every progress step, and the test signals the process directly.
- Tests: the UI tests wait until an archive has been read completely, not just until its first entries show. Extract does nothing before then, which made `names_flow` time out about one run in five.

## 0.2.12 — 2026-10-31
- macOS: Finder's keys in an archive: ⌘↓ opens the item under the cursor, ⌘↑ goes up a folder, ⌘⌫ deletes the selection from the archive, and Return renames (it still opens in archives Arkyv can't edit). The shortcuts sheet already listed them.

## 0.2.11 — 2026-10-31
- macOS: the traffic lights share the header buttons' centre line (the header's height now comes from where the lights actually are).
- macOS: Preferences puts the traffic lights in its header too, so its title shows once.
- macOS: the Operations page's Back button moves clear of the traffic lights.
- The header title is centred again, as in the mockups (0.2.9 had moved it to the left edge).

## 0.2.10 — 2026-10-31
- macOS: the traffic lights line up with the header's buttons (the title bar is now a unified toolbar's height, and the header takes on that height), and stay in place through resizing and full screen.
- macOS: Home's cards show ⌘N and ⌘O instead of Ctrl N and Ctrl O.

## 0.2.9 — 2026-10-30
- macOS: Home and the full view put the traffic lights in Arkyv's own header, next to its buttons, as current Mac apps do; the mini view keeps the plain title bar.
- macOS: the UI uses the system's fonts (San Francisco, and SF Arabic for the Arabic UI) instead of the bundled Inter and IBM Plex Sans Arabic.

## 0.2.8 — 2026-10-30
- macOS: the app and the `arkyv` CLI no longer fail to start on Macs without Homebrew's xz: xz is built into them, and packaging now fails if a binary needs any library macOS doesn't ship.

## 0.2.7 — 2026-10-30
- macOS: releases sign the app and the `.dmg` with Arkyv's Developer ID certificate, notarize the `.dmg` with Apple and staple the ticket, so it opens without the "unidentified developer" warning; the `.dmg` is now part of each GitHub Release (`docs/macos-signing.md`).

## 0.2.6 — 2026-10-29
- Homebrew: `scripts/homebrew-cask.py` writes the cask (`arkyv`) for a published version, and `docs/homebrew.md` has the ways to publish it.

## 0.2.5 — 2026-10-29
- macOS: when macOS hasn't let Arkyv into a folder, the "can't write" card says where to allow it and opens Privacy & Security → Files and Folders; macOS's own prompts explain why Arkyv asks, in English and Arabic.

## 0.2.4 — 2026-10-29
- macOS: Arkyv's menus are in the menu bar: Arkyv (About, Preferences, Quit), File, Edit, Archive, View and Help, with their ⌘ shortcuts.

## 0.2.3 — 2026-10-29
- macOS: first run and Preferences make Arkyv the default app for the archive types you pick; macOS asks you to confirm each one.

## 0.2.2 — 2026-10-29
- macOS: Arkyv opens the archives Finder sends it (double-click, Open With), and extraction progress shows on the Dock icon.

## 0.2.1 — 2026-10-29
- macOS builds: every change is now built and tested on macOS in CI, and `scripts/package-macos.sh` makes a universal `Arkyv.app` (Apple Silicon and Intel, macOS 12 and later) in a `.dmg`, with Arkyv listed in Finder's "Open With" for the same archive types as on Windows. Not in the release yet: it comes once it's signed and notarized.

## 0.2.0 — 2026-10-29
Arkyv for Windows.

- **Install:** `Arkyv-0.2.0-x64-setup.exe` below, for Windows 10 and 11. It needs no administrator rights (or installs for everyone if you choose), and adds Arkyv to "Open with", Settings → Default apps and Explorer's right-click menu (Open with Arkyv, Extract with Arkyv, Compress with Arkyv…). It isn't code-signed yet: SmartScreen asks once, "More info → Run anyway"; `SHA256SUMS` has its checksum.
- **At home on Windows:** the native title bar with Snap Layouts, progress on the taskbar button, Downloads and Desktop wherever Windows keeps them (OneDrive too), and paths shown as Explorer shows them.
- **Safe on Windows:** names Windows won't accept (`CON.txt`, `report: v2?.pdf`, `notes.`) are found before extracting, to rename, edit or skip; files that differ only by case don't overwrite each other; very long paths work; extracted files keep the "downloaded from the internet" mark, so SmartScreen and Office still check them.
- **Everywhere:** the `arkyv` tool says which names it changed, and the same checks run on Windows in CI for every change.

## 0.1.7 — 2026-10-28
- winget: `scripts/winget-manifest.py` writes the manifests for a published version, and `docs/winget.md` has the steps to submit them.

## 0.1.6 — 2026-10-28
- Windows installer (`Arkyv-…-x64-setup.exe`): installs for you alone without administrator rights, or for everyone. It adds Arkyv to "Open with" and to Settings → Default apps for zip, 7z, rar and tar files, and puts "Open with Arkyv", "Extract with Arkyv" and "Compress with Arkyv…" in Explorer's right-click menu.
- Releases build the Windows installer next to the Linux packages, after installing and uninstalling it once on a clean runner.

## 0.1.5 — 2026-10-28
- Windows: extraction, tests and edits show their progress on the taskbar button.
- Windows: First run and Preferences → "Open these with Arkyv" open Settings → Default apps on Arkyv's page (Windows lets only you choose default apps), and Preferences shows which types already open with Arkyv.
- Windows: `arkyv-app.exe` carries Arkyv's icon.

## 0.1.4 — 2026-10-28
- Windows: files extracted from a downloaded archive keep the "downloaded from the internet" mark, as with Explorer's own Extract, so SmartScreen and Office's Protected View still check them.

## 0.1.3 — 2026-10-28
- Names a Windows disk won't take (`CON.txt`, `report: v2?.pdf`, `notes.`) are found before extracting: Arkyv shows the new names and lets you rename, edit the names, or skip those files. Very long paths extract in long-path mode.
- Two files whose names differ only by case (`README.md`, `readme.md`) no longer overwrite each other on Windows and macOS: keep both, keep the newer one, or skip.
- The `arkyv` tool says which names it changed.

## 0.1.2 — 2026-10-28
- Windows: Arkyv builds and its whole test suite runs on Windows in CI. RAR archives open by their full Unicode path there.
- Windows: settings live in AppData, Downloads and Desktop follow the system (OneDrive too), paths show as Explorer shows them, archives can be edited while their window is open, and links to files extract as copies.

## 0.1.1 — 2026-10-28
- The busy cursor no longer lingers after Arkyv's window opens: the window now tells the desktop the launch is over (startup notification on Wayland and X11).

## 0.1.0 — 2026-10-28
The first release, for Linux.

- **Open anything common:** ZIP, 7z, RAR and tar (gz, xz, zst, bz2), including split sets (`.7z.001`, `.part1.rar`) and archives inside archives. Make ZIP, 7z and tar archives, with passwords (AES-256) and split volumes.
- **One click to extract:** double-click an archive for the mini view, or browse, preview and edit it in the full view. Batch extract several at once; long jobs show in the Operations panel and notify you when they finish in the background.
- **Safe by default:** files that try to leave the folder, links pointing outside it and decompression bombs are stopped and reported; archives are never rewritten in place.
- **Readable everywhere:** garbled names in old ZIPs are fixed (and remembered), the interface is in English and Arabic (right to left), light and dark, usable with the keyboard and a screen reader.
- **Install:** `.deb`, `.rpm` or AppImage below; the `arkyv` command-line tool comes with them.

## 0.0.9 — 2026-10-28
M6: fuzzing.
- The ZIP, tar, tar.gz and 7z readers are fuzzed (cargo-fuzz): every input is opened, listed, tested and extracted, and nothing may land outside the destination. 1.8 million inputs so far, no problems; CI fuzzes on every push.

## 0.0.8 — 2026-10-28
M6: for contributors.
- Issue templates for bugs and ideas; security problems are reported privately.
- CONTRIBUTING.md: how the repository is organised, the checks, the rules for pull requests, adding a translation.
- `arkyv-app --version`.

## 0.0.7 — 2026-10-28
M6: README.
- A new README: screenshots, what Arkyv does, install table, how it's built, real benchmarks (`scripts/bench-cli.sh`).
- Creating a tar.zst uses every core (about twice as fast on 1 GB).
- New archive shows the output path as `~/…`.

## 0.0.6 — 2026-10-28
M6: Linux packages.
- .deb, .rpm and AppImage builds (`scripts/package-linux.sh`), named `arkyv`, with the `arkyv` command-line tool, icons, the desktop entry and AppStream metadata.
- Built on Ubuntu 22.04 so they run on most current distributions; checked on Ubuntu 24.04 and Fedora 44.
- A release workflow publishes them from a version tag, with checksums.

## 0.0.5 — 2026-10-28
M5: operations panel. Every M5 task is done.
- Home has an Operations page listing every job of every Arkyv window: progress, time left, current file, speed, with Pause / Resume and Cancel; queued archives of a batch can be removed.
- Finished jobs are kept in a history with Open / Show in folder; Clear finished empties it.
- A job that ends while its window is in the background sends a desktop notification.

## 0.0.4 — 2026-10-27
M5: nested archives and missing volumes.
- Split archives (`name.7z.001`, `.002`…, the kind Arkyv's own "Split into volumes" makes) open from any part; RAR sets (`.part1.rar`, `.r00`) are checked for gaps.
- A missing part says which one, lists the parts found, lets you locate it in another folder, and the window opens the archive by itself once the part appears (a download finishing). Batch extract can locate it too.
- An archive inside an archive opens in Arkyv; edits come back through "Update archive".
- New in the Extract dialog: "Also unpack archives found inside" (up to 3 levels, every level with the usual safety checks).

## 0.0.3 — 2026-10-27
M5: batch extract.
- Pick or drop several archives on Home, or run `arkyv-app --extract-each <archives…>`: Arkyv extracts them one after another, each into its own folder (or all into one), with overall progress and time left.
- Archives that need a password wait for it without holding up the rest; damaged ones show the error with Retry; any row can be paused, opened when done, or skipped.
- Optionally moves each archive to the Trash once it's extracted.

## 0.0.2 — 2026-10-27
M5: accessibility pass.
- Every control can be reached with Tab and pressed with Space or Enter, with a focus ring that shows only for keyboard users; Up / Down change drop-downs and segmented controls.
- Screen readers get a name and role for every control, including icon buttons, checkboxes, switches, tabs and list rows (with size and selected state); toasts are announced.
- A test fails if any control on the main screens has no name.

## 0.0.1 — up to 2026-10-27
Everything before per-task versions:
- M0–M4: the Slint app, safe listing and extraction (`crate-core`), the CLI, the mini view, the full view with preview and grid, creating archives, editing inside archives, Properties, testing and converting.
- M5 so far: mini view folder navigation, Preferences, First run with "Make Arkyv the default", the Arabic UI with a mirrored layout, the keyboard shortcuts sheet, dark mode on every screen, the filename encoding picker with "Remember for this archive", and the About & support screen.
