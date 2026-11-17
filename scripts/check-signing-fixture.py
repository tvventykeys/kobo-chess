#!/usr/bin/env python3
"""Require a local verification bundle to fail stock Cobalt release trust.
This never installs, publishes, supplies release credentials or changes trust.
"""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('bundle_dir', type=Path)
args = parser.parse_args()
repo = Path(__file__).resolve().parents[1]
source = Path(os.environ.get('COBALT_SOURCE', '/workspace/deps/Cobalt'))
target = Path(os.environ.get('CARGO_TARGET_DIR', '/workspace/build/cobalt'))
version = json.loads((repo/'cobalt-app.json').read_text())['version']
text = (source/'crates/kobo-app-store/src/lib.rs').read_text()
match = re.search(r'PUBLIC_RELEASE_KEY_HEX:\s*&str\s*=\s*"([a-f0-9]{64})"', text)
if not match:
    raise SystemExit('Cannot identify pinned Cobalt release public key; review the source')
key = args.bundle_dir/'cobalt-release-public-key.txt'
key.write_text(match[1]+'\n')
result = subprocess.run([str(target/'debug/kobo'), 'app-verify',
    '--package', str(args.bundle_dir/f'polgar-pocket-{version}-verification-only.cobaltapp'),
    '--public-key', str(key), '--manifest', str(args.bundle_dir/'manifest.json'),
    '--binary', str(target/'armv7-unknown-linux-musleabihf/release/polgar-pocket')],
    text=True, capture_output=True, check=False)
(args.bundle_dir/'release-key-rejection.log').write_text(result.stdout+result.stderr)
if result.returncode == 0 or 'signature' not in result.stderr.lower():
    raise SystemExit('Expected release-key signature refusal was not observed: '+result.stderr)
print('PASS: verification fixture refused under the actual Cobalt release public key')
