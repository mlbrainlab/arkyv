# Crate app icon: concept #1 "Open Lid"

| File | Use |
|---|---|
| `crate-icon.svg` | Master artwork, full-bleed rounded tile (Linux, Windows, README, website) |
| `crate-icon-macos.svg` | macOS master: an 824 px tile with a shadow on a 1024 canvas, the size macOS expects |
| `png/crate-{16…1024}.png` | Rendered sizes for Linux (hicolor theme, Flatpak/AppStream) and docs |
| `png/crate-macos-1024.png` | Source for the macOS icon |
| `crate.ico` | Windows (16–256) |
| `crate.icns` | macOS |

## In the app
- Window icon: `png/crate-256.png`, set with `icon:` on the windows in `app/ui/`.
- Linux packages (M6): install `png/crate-{16…512}.png` into the hicolor theme as `io.github.mlbrainlab.crate.png` (the Wayland app id), referenced by the `.desktop` file.
- Windows (M7): `crate.ico` as the executable resource. macOS (M8): `crate.icns` (the padded version) in the app bundle.

Colors: tile `#3352E0`, lid `#DDE4FF`, box `#FFFFFF`, document `#FFD27A`.

## Alternate icons (Pro)
Seven more, picked in Preferences → Appearance → App icon: `alt/{ark,zip,layers,squeeze,monogram,open-crate,vault}.svg`, on the same 100-unit grid and tile as Open Lid, with 256 px PNGs in `alt/png/` for the window icon. Render them with `resvg -w 256 -h 256 alt/<name>.svg alt/png/<name>-256.png` (`cargo install resvg`); ImageMagick's own SVG renderer gets strokes and transforms wrong. Extra artwork colors: `#8EA0F0`, `#C7D0F8`, `#1E2F8F`.
