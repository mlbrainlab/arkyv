#!/usr/bin/env bash
# Builds the Linux packages into target/dist/: .deb (cargo-deb), .rpm
# (cargo-generate-rpm), AppImage (appimagetool), and SHA256SUMS. The release
# workflow runs this; locally:
#
#   cargo install --locked cargo-deb cargo-generate-rpm   # once
#   scripts/package-linux.sh
#   SKIP_BUILD=1 scripts/package-linux.sh                 # reuse target/release
#
# appimagetool: https://github.com/AppImage/appimagetool (APPIMAGETOOL=path
# to use another copy).
set -euo pipefail
cd "$(dirname "$0")/.."

APP_ID=io.github.mlbrainlab.arkyv
NAME=arkyv
VERSION=$(cargo metadata --no-deps --format-version 1 |
  python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"]=="arkyv-app"))')
ARCH=$(uname -m)
DIST=target/dist
STAGE=target/packaging
APPIMAGETOOL=${APPIMAGETOOL:-appimagetool}

if [[ -z "${SKIP_BUILD:-}" ]]; then
  cargo build --release -p arkyv-app -p arkyv-cli
fi
for bin in arkyv-app arkyv; do
  [[ -x target/release/$bin ]] || { echo "target/release/$bin is missing" >&2; exit 1; }
done

rm -rf "$DIST" "$STAGE"
mkdir -p "$DIST" "$STAGE"

# The .desktop file as installed: Exec is the binary on PATH.
sed 's|@EXEC@|arkyv-app|' packaging/linux/$APP_ID.desktop > "$STAGE/$APP_ID.desktop"
if command -v desktop-file-validate >/dev/null; then
  desktop-file-validate "$STAGE/$APP_ID.desktop"
fi
if command -v appstreamcli >/dev/null; then
  appstreamcli validate --no-net --explain packaging/linux/$APP_ID.metainfo.xml || true
fi

echo "== .deb"
cargo deb -p arkyv-app --no-build --no-strip --output "$DIST/"

echo "== .rpm"
cargo generate-rpm -p app --output "$DIST/"

echo "== AppImage"
APPDIR=$STAGE/AppDir
install -Dm755 target/release/arkyv-app "$APPDIR/usr/bin/arkyv-app"
install -Dm755 target/release/arkyv "$APPDIR/usr/bin/arkyv"
install -Dm644 "$STAGE/$APP_ID.desktop" "$APPDIR/usr/share/applications/$APP_ID.desktop"
install -Dm644 packaging/linux/$APP_ID.metainfo.xml "$APPDIR/usr/share/metainfo/$APP_ID.appdata.xml"
for s in 16 24 32 48 64 128 256 512; do
  install -Dm644 "assets/icons/png/arkyv-$s.png" "$APPDIR/usr/share/icons/hicolor/${s}x${s}/apps/$APP_ID.png"
done
install -Dm644 assets/icons/arkyv-icon.svg "$APPDIR/usr/share/icons/hicolor/scalable/apps/$APP_ID.svg"
# At the AppDir's top: the entry, its icon, and AppRun.
cp "$STAGE/$APP_ID.desktop" "$APPDIR/$APP_ID.desktop"
cp assets/icons/png/arkyv-256.png "$APPDIR/$APP_ID.png"
cp assets/icons/png/arkyv-256.png "$APPDIR/.DirIcon"
cat > "$APPDIR/AppRun" <<'RUN'
#!/bin/sh
# `arkyv-appimage --cli …` runs the command-line tool; anything else the app.
HERE=$(dirname "$(readlink -f "$0")")
if [ "$1" = "--cli" ]; then
  shift
  exec "$HERE/usr/bin/arkyv" "$@"
fi
exec "$HERE/usr/bin/arkyv-app" "$@"
RUN
chmod 755 "$APPDIR/AppRun"
ARCH=$ARCH "$APPIMAGETOOL" --no-appstream "$APPDIR" "$DIST/Arkyv-$VERSION-$ARCH.AppImage" >/dev/null

echo "== SHA256SUMS"
(cd "$DIST" && sha256sum -- *.deb *.rpm *.AppImage > SHA256SUMS && cat SHA256SUMS)
