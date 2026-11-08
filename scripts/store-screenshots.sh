#!/usr/bin/env bash
# Store screenshots (Snap Store, Flathub) from the real app on this desktop:
# each window is captured on X11 (XWayland under a Wayland session), then
# framed with its rounded corners and a shadow on a 1920×1080 background.
# Writes docs/store-screenshots/. Needs xwininfo and ImageMagick (import,
# convert). Windows open on screen for a few seconds each.
#
# The windows read a stand-in home folder with sample files, so nothing of
# yours appears in them.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release -p arkyv-app
APP=$PWD/target/release/arkyv-app
OUT=$PWD/docs/store-screenshots
W=$(mktemp -d)
trap 'rm -rf "$W"' EXIT
mkdir -p "$OUT" "$W/raw" "$W/home/Downloads" "$W/home/Documents/Trip-photos"

python3 -I - "$W/home" <<'EOF'
import random, sys, zipfile
home = sys.argv[1]
r = random.Random(1)
noise = lambda n: bytes(r.getrandbits(8) for _ in range(n))
text = lambda n: ("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n" * n).encode()
files = [("README.md", text(40)), ("tokens.json", text(120)), ("CHANGELOG.md", text(30)),
         ("LICENSE.txt", text(12)), ("docs/colors.pdf", noise(180000)),
         ("docs/typography.pdf", noise(240000)), ("docs/spacing.md", text(60)),
         ("icons/archive.svg", text(8)), ("icons/folder.svg", text(8)),
         ("images/hero@2x.png", noise(420000)), ("images/cover.jpg", noise(310000)),
         ("src/button.tsx", text(90)), ("src/theme.ts", text(70)),
         ("Brand guidelines.pdf", noise(820000)), ("Presentation.key", noise(1200000))]
with zipfile.ZipFile(f"{home}/Downloads/design-system.zip", "w", zipfile.ZIP_DEFLATED) as z:
    for name, data in files:
        z.writestr(name, data)
for i, name in enumerate(["beach.jpg", "harbour.jpg", "old-town.jpg", "sunset.jpg", "market.jpg", "ferry.jpg"]):
    open(f"{home}/Documents/Trip-photos/{name}", "wb").write(noise(150000 + i * 20000))
open(f"{home}/Documents/Trip-photos/notes.txt", "wb").write(text(10))
EOF

# capture <name> <scale> <state.ini lines> <args…>
capture() {
  local name=$1 scale=$2 prefs=$3
  shift 3
  local cfg=$W/cfg-$name
  mkdir -p "$cfg/arkyv"
  printf 'first_run_done=1\n%b' "$prefs" > "$cfg/arkyv/state.ini"
  env -u WAYLAND_DISPLAY -u LC_ALL -u LC_MESSAGES SLINT_SCALE_FACTOR="$scale" LANG=en_US.UTF-8 \
    HOME="$W/home" XDG_CONFIG_HOME="$cfg" XDG_STATE_HOME="$cfg/state" XDG_CACHE_HOME="$cfg/cache" \
    ARKYV_OPS_DIR="$cfg/ops" "$APP" "$@" &
  local pid=$! id=
  for _ in $(seq 50); do
    sleep 0.2
    id=$(xwininfo -root -tree | grep '"io.github.mlbrainlab.arkyv"' | awk '{print $1}' | head -1)
    [ -n "$id" ] && break
  done
  sleep 2 # the listing and first frame
  import -window "$id" -frame "$W/raw/$name.png"
  kill "$pid"
  wait "$pid" 2>/dev/null || true
}

# frame <name> <scale> <background gradient>: the app draws a 12 px shadow
# margin, which X11 captures as black; cut it, round the corners, add a shadow.
frame() {
  local name=$1 scale=$2 bg=$3 m w h
  m=$(python3 -c "print(round(12 * $scale))")
  convert "$W/raw/$name.png" -shave "${m}x${m}" +repage "$W/t.png"
  w=$(identify -format %w "$W/t.png")
  h=$(identify -format %h "$W/t.png")
  convert "$W/t.png" \( -size "${w}x${h}" xc:black -fill white \
    -draw "roundrectangle 0,0,$((w - 1)),$((h - 1)),$m,$m" \) \
    -alpha off -compose CopyOpacity -composite "$W/r.png"
  convert "$W/r.png" \( +clone -background black -shadow 30x28+0+18 \) +swap \
    -background none -layers merge +repage "$W/s.png"
  convert -size 1920x1080 "gradient:$bg" "$W/s.png" -gravity center -geometry +0+6 \
    -composite -strip "$OUT/$name.png"
}

Z=$W/home/Downloads/design-system.zip
LIGHT='#eef1fd-#bccaff'
DARK='#2a3566-#0f1115'
FULL='pref.open_with=1\nfull_width=1180\nfull_height=700\n'
capture 1-mini 1.5 '' "$Z" && frame 1-mini 1.5 "$LIGHT"
capture 2-full 1.25 "$FULL" "$Z" && frame 2-full 1.25 "$LIGHT"
capture 3-dark 1.25 "${FULL}pref.theme=2\n" "$Z" && frame 3-dark 1.25 "$DARK"
capture 4-arabic 1.5 'pref.language=ar\n' "$Z" && frame 4-arabic 1.5 "$LIGHT"
capture 5-create 1.25 '' --compress "$W/home/Documents/Trip-photos" && frame 5-create 1.25 "$LIGHT"
ls -l "$OUT"
