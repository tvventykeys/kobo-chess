#!/usr/bin/env python3
"""Materialize the reviewed Beta submission in a separate, clean Cobalt checkout.
Only local source/assets/generated pages are written. No fetch, signing, remote
push, PR, release, catalog publication or device operation occurs here.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('checkout', type=Path)
args = parser.parse_args()
repo = Path(__file__).resolve().parents[1]
root = args.checkout.resolve()
policy = json.loads((repo/'submission/upstream.json').read_text())

def git(*arguments):
    return subprocess.check_output(['git', '-C', str(root), *arguments], text=True).strip()

if root == repo or repo in root.parents or root == Path('/workspace/deps/Cobalt'):
    parser.error('use a separate Cobalt checkout; do not modify the app or pinned SDK')
if git('rev-parse', 'HEAD') != policy['base_commit']:
    parser.error('checkout HEAD differs from the reviewed Beta base; refresh and review the submission first')
if git('status', '--porcelain'):
    parser.error('checkout must be clean; preserve existing work instead of overwriting it')
app = root/'apps'/policy['app_id']
if app.exists():
    parser.error('an app already occupies this identity')
manifest = json.loads((repo/'cobalt-app.json').read_text())
if (manifest['version'], manifest['id'], manifest['glyph'], manifest['capabilities']) != (
        policy['version'], policy['app_id'], policy['glyph'], policy['capabilities']):
    parser.error('manifest and reviewed submission policy disagree')
lines = [json.loads(line) for line in (repo/'packs/lichess-20.jsonl').read_text().splitlines()]
if len(lines) != 21 or lines[0]['license'] != 'CC0-1.0' or lines[0]['collection_id'] != 'lichess-tactics':
    parser.error('only the reviewed twenty-puzzle CC0 pack may be submitted')
captures = json.loads((repo/'submission/CAPTURES.json').read_text())
for capture in captures['captures']:
    path = repo/'submission/screenshots'/capture['file']
    if path.name != capture['file'] or hashlib.sha256(path.read_bytes()).hexdigest() != capture['sha256']:
        parser.error('screenshot path or digest is not the reviewed capture')
    if path.read_bytes()[:8] != b'\x89PNG\r\n\x1a\n':
        parser.error('screenshot is not a PNG')

# Prepare the app from explicit public allowlists, not arbitrary local files.
(app/'src').mkdir(parents=True)
(app/'packs').mkdir()
for path in (repo/'src').glob('*.rs'):
    shutil.copy2(path, app/'src'/path.name)
for name in ['lichess-20.jsonl', 'lichess-20-source.csv', 'SOURCE.md']:
    shutil.copy2(repo/'packs'/name, app/'packs'/name)
for name in ['LICENSE', 'THIRD-PARTY.md']:
    shutil.copy2(repo/name, app/name)
manifest['minimum_cobalt_version'] = policy['minimum_cobalt_version']
(app/'cobalt-app.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n')
(app/'Cargo.toml').write_text('''[package]
name = "kobo-polgar-pocket"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish = false

[dependencies]
kobo-sdk = { path = "../../crates/kobo-sdk" }
shakmaty = "=0.27.3"
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"

[dev-dependencies]
kobo-ui = { path = "../../crates/kobo-ui" }
kobo-text = { path = "../../crates/kobo-text" }
kobo-profile = { path = "../../crates/kobo-profile" }

[lints]
workspace = true
''')
shutil.copy2(repo/'submission/README-upstream.md', app/'README.md')
shutil.copy2(repo/'submission/drive.kobo', app/'drive.kobo')
shutil.copy2(repo/'submission/CAPTURES.json', app/'CAPTURES.json')
(app/'screenshots').mkdir()
(app/'capture-metadata').mkdir()
gallery = root/'docs/media/site/apps'/policy['app_id']
gallery.mkdir()
captions = {}
for capture in captures['captures']:
    path = repo/'submission/screenshots'/capture['file']
    shutil.copy2(path, app/'screenshots'/path.name)
    metadata = Path(capture['metadata_file'])
    if metadata.parent != Path('capture-metadata') or metadata.name != path.name+'.json':
        raise SystemExit('unexpected screenshot metadata path')
    shutil.copy2(repo/'submission'/metadata, app/metadata)
    shutil.copy2(path, gallery/path.name)
    captions[path.name] = capture['caption']
    if capture.get('primary'):
        shutil.copy2(path, root/'docs/media/site/apps/polgar-pocket.png')
(gallery/'captions.json').write_text(json.dumps(captions, ensure_ascii=False, indent=2)+'\n')
path = root/'Cargo.toml'
text = path.read_text()
if 'members = [' not in text:
    raise SystemExit('workspace member structure changed; review required')
path.write_text(text.replace('members = [', 'members = [\n    "apps/polgar-pocket",', 1))
path = root/'tools/generate-app-pages.mjs'
text = path.read_text()
for expected in ['const screenshots = {', 'const categories = {']:
    if expected not in text:
        raise SystemExit('website generator changed; review required')
text = text.replace('const screenshots = {', 'const screenshots = {\n  "polgar-pocket": ["polgar-pocket.png", "Actual Clara BW simulator capture of Polgár Trainer Pocket with a large chessboard and its offline study controls."],', 1)
text = text.replace('const categories = {', 'const categories = {\n  "polgar-pocket": "Games",', 1)
path.write_text(text)
path = root/'THIRD-PARTY.md'
path.write_text(path.read_text()+'\n'+(repo/'submission/upstream-third-party.md').read_text())
# Resolve only the new package into the existing lockfile, without downloads or builds.
lock = root/'Cargo.lock'
original_lock = lock.read_text()
subprocess.run(['cargo', '+1.85.1', 'metadata', '--offline', '--format-version', '1'],
               cwd=root, stdout=subprocess.DEVNULL, check=True)
import re
resolved = lock.read_text()
without_app = re.sub(r'\[\[package\]\]\nname = "kobo-polgar-pocket"\n.*?(?=\[\[package\]\]|\Z)',
                     '', resolved, flags=re.S)
if without_app != original_lock:
    raise SystemExit('dependency resolution changed more than the new app entry; review required')
# The active protocol's exact-input CI assertion also inventories workspace packages.
# Refresh only its Cargo.lock output hash; preserve its base and every SDK blob.
review = root/'tools/app-release-compatible-changes.json'
original = review.read_text()
data = json.loads(original)
active = [c for c in data['changes'] if c['protocol_version'] == 18]
if len(active) != 1:
    raise SystemExit('active compatible-change review structure changed')
entry = next(f for f in active[0]['files'] if f['path'] == 'Cargo.lock')
old_blob = entry['compatible_blob']
new_blob = git('hash-object', 'Cargo.lock')
reason = active[0]['reason']
addition = ' Proposed Polgár Trainer Pocket registration subsequently adds only the kobo-polgar-pocket workspace package and its edges to already-resolved dependencies; no existing package identity, dependency edge, third-party version or SDK blob changes. The Cargo.lock output hash is refreshed for this app-only inventory addition and requires maintainer review with this contribution.'
original = original.replace(json.dumps(reason), json.dumps(reason+addition, ensure_ascii=False), 1)
# Restrict replacement to the one active entry rather than historical exemptions.
marker = '\"protocol_version\": 18'
pos = original.index(marker)
original = original[:pos] + original[pos:].replace(old_blob, new_blob, 1)
review.write_text(original)
subprocess.run(['node', 'tools/generate-app-pages.mjs'], cwd=root, check=True)
print(f'Prepared local Beta contribution in {root}; run locked checks and official --dry-run next')
