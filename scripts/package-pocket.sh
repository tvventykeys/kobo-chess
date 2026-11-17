#!/usr/bin/env bash
# Development bundle only. Does not install, publish or write to a device.
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
pocket_out="${1:-/workspace/verification/pocket-bundle}"
mkdir -p "$pocket_out"
python3 - "$pocket_out" "$CARGO_TARGET_DIR" <<'PY'
import hashlib, json, pathlib, secrets, sys
root=pathlib.Path(sys.argv[1])
binary=pathlib.Path(sys.argv[2])/'armv7-unknown-linux-musleabihf/release/polgar-pocket'
source=json.loads(pathlib.Path('cobalt-app.json').read_text())
manifest={key:source[key] for key in ['id','display_name','short_label','summary','version','glyph','capabilities']}
manifest.update(format_version=1,minimum_cobalt_version='0.3.37',
                binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),binary_bytes=binary.stat().st_size)
(root/'manifest.json').write_text(json.dumps(manifest,separators=(',',':')))
seed=root/'development.seed'
if not seed.exists():
    with seed.open('xb') as stream:
        stream.write(secrets.token_bytes(32))
    seed.chmod(0o600)
PY
"$CARGO_TARGET_DIR/debug/kobo" app-key --seed "$pocket_out/development.seed" > "$pocket_out/development-public-key.txt"
"$CARGO_TARGET_DIR/debug/kobo" app-bundle --manifest "$pocket_out/manifest.json" \
    --binary "$CARGO_TARGET_DIR/armv7-unknown-linux-musleabihf/release/polgar-pocket" \
    --seed "$pocket_out/development.seed" --out "$pocket_out/polgar-pocket-0.1.0-development.cobaltapp"
"$CARGO_TARGET_DIR/debug/kobo" app-verify --package "$pocket_out/polgar-pocket-0.1.0-development.cobaltapp" \
    --public-key "$pocket_out/development-public-key.txt" --manifest "$pocket_out/manifest.json" \
    --binary "$CARGO_TARGET_DIR/armv7-unknown-linux-musleabihf/release/polgar-pocket"
echo 'Development signature verified. Stock Cobalt installation requires trusted release signing.'
