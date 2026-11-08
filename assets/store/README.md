# Store listing artwork

For the Snap Store listing page (snapcraft.io → arkyv → Listing), uploaded by hand; the store doesn't take these from the snap.

| File | Use |
|---|---|
| `arkyv-icon-dark.svg` | Dark version of the Layers icon: tile `#1C2233`→`#0F1115`, a blue glow, slabs `#EEF1FD`, `#5D76F6`, `#FFD27A` |
| `arkyv-icon-dark-256.png` / `-512.png` | Store icon (square, 40–512 px, up to 256 KB; 256 recommended) |
| `banner.svg` | Banner background and text (font "Crate Inter" from `assets/fonts/inter/`) |
| `banner.png` | Store banner, 1920×640 (3:1, 720×240 to 4320×1440, up to 2 MB): the SVG plus the dark full-view window from `docs/store-screenshots/3-dark.png` |

Rebuild: `resvg -w 512 -h 512 arkyv-icon-dark.svg arkyv-icon-dark-512.png`; for the banner, `resvg --skip-system-fonts --use-fonts-dir ../fonts/inter --resources-dir . banner.svg`, then lay the window (cropped `1448x844+236+106` from `3-dark.png`, corners rounded at 15 px, resized to 1100 px wide, shadow `50x30+0+20`) at `+930+70` with ImageMagick.
