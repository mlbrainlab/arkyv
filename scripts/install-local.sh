#!/usr/bin/env bash
# Builds this checkout and installs it as the system's Arkyv, replacing an
# older arkyv .deb (the one a release installs). Asks for sudo for
# the install step only.
#
#   scripts/install-local.sh               # build release, package, install
#   SKIP_BUILD=1 scripts/install-local.sh  # reuse target/release
#
# Needs cargo-deb: cargo install --locked cargo-deb
set -euo pipefail
cd "$(dirname "$0")/.."

OUT=target/dist-local

if [[ -z "${SKIP_BUILD:-}" ]]; then
  cargo build --release -p arkyv-app -p arkyv-cli
fi
for bin in arkyv-app arkyv; do
  [[ -x target/release/$bin ]] || { echo "target/release/$bin is missing" >&2; exit 1; }
done

rm -rf "$OUT"
mkdir -p "$OUT"
cargo deb -p arkyv-app --no-build --no-strip --output "$OUT/"
DEB=$(ls "$OUT"/*.deb)

echo "Installing $DEB"
sudo apt-get install -y --allow-downgrades "./$DEB"

# Running windows keep the old binary until they're closed.
if pgrep -x arkyv-app >/dev/null; then
  echo "Arkyv is running: close its windows to use the new version."
fi
echo "Installed: $(dpkg-query -W -f='${Package} ${Version}' arkyv)"
