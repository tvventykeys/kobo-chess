#!/usr/bin/env python3
"""Prepare an owner-reviewable contribution tree outside the upstream checkout.
No upstream edits, device access, signing, contact or publication are performed.
"""
import argparse
import json
from pathlib import Path
import shutil

parser = argparse.ArgumentParser()
parser.add_argument('output', type=Path)
args = parser.parse_args()
repo = Path(__file__).resolve().parents[1]
output = args.output.resolve()
if output.exists():
    parser.error('output must be a new directory')
if output == repo or repo in output.parents or Path('/workspace/deps/Cobalt') in output.parents:
    parser.error('write the review tree outside the app and upstream checkouts')
app = output/'apps/polgar-pocket'
app.mkdir(parents=True)
for name in ['src','packs']:
    shutil.copytree(repo/name, app/name)
for name in ['cobalt-app.json','LICENSE','THIRD-PARTY.md']:
    shutil.copy2(repo/name, app/name)
manifest = json.loads((app/'cobalt-app.json').read_text())
manifest['minimum_cobalt_version'] = '0.3.37'
(app/'cobalt-app.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n')
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
serde_json = "=1.0.145"

[dev-dependencies]
kobo-profile = { path = "../../crates/kobo-profile" }
''')
(output/'REVIEW-BEFORE-SUBMISSION.md').write_text('''# Prepared source contribution — not submitted

This tree is only a proposed `apps/polgar-pocket` directory. It has not been
added to Cobalt's workspace, generated registry, website, CI or release catalogs.
No Cobalt maintainer has approved it. There is no release credential here.

If the owner later authorizes an upstream contribution, a separate reviewed
checkout would need `apps/polgar-pocket` added to workspace members and the
maintainer's official contributor check run against that checkout. That check
updates generated website/provenance outputs and derives platform minimums.
The adapted manifest inherits the upstream version, while the app release
version remains in cobalt-app.json. Re-run its tests/Clippy/ARM build there;
the source is tested in the independent application, but this adapted tree has
not been compiled as a member of upstream. Preserve all license notices.

The documented public route targets Cobalt's beta branch. Merge triggers
maintainer-controlled signing and publication to the Beta app catalog. The
owner has NOT authorized that publication. Do not open that upstream PR or
publish anything as part of this preparation. Ask maintainers about an approved
non-Store personal-testing path before assuming one exists.
''')
print(f'Prepared review tree at {output}; upstream remains untouched')
