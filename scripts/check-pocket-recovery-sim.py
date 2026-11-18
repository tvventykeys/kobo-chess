#!/usr/bin/env python3
"""Test startup recovery, failed writes, quiet idle and clean exit on the host only."""
from http.client import RemoteDisconnected
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import subprocess
import time
import tomllib
import urllib.request

spec = importlib.util.spec_from_file_location('pocket_sim', Path(__file__).with_name('check-pocket-sim.py'))
sim = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sim)

def stored(root):
    matches = list(Path(root).rglob('pocket.progress.v1'))
    assert len(matches) == 1, matches
    return matches[0]

def idle_metrics(proc, address, folder):
    # Locate this simulator's direct app child, never a reader or another run.
    children = subprocess.check_output(['ps','--ppid',str(proc.pid),'-o','pid=,comm='],text=True).splitlines()
    pids = [row.split()[0] for row in children if row.split()[1] == tomllib.loads((sim.repo/'Cargo.toml').read_text())['package']['name'][:15]]
    assert len(pids) == 1, children
    def ticks():
        fields = Path(f'/proc/{pids[0]}/stat').read_text().split(') ', 1)[1].split()
        return int(fields[11]) + int(fields[12])
    first = ticks()
    paints = sim.get(address, '/layout')['paints']
    time.sleep(2)
    result = dict(interval_seconds=2, cpu_ticks=ticks()-first,
                  paints_before=paints, paints_after=sim.get(address, '/layout')['paints'],
                  status=Path(f'/proc/{pids[0]}/status').read_text())
    assert result['cpu_ticks'] == 0, result
    assert result['paints_before'] == result['paints_after'], result
    (folder/'idle.json').write_text(json.dumps(result, indent=2))

for scale in ['default', '170']:
    folder = sim.out/'recovery'/scale
    folder.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='pocket-recovery-') as temp:
        env = dict(os.environ, TMPDIR=temp, RUSTUP_TOOLCHAIN='1.85.1',
                   KOBO_SIM_PROFILE=sim.profile_id, KOBO_SIM_OFFLINE='1',
                   KOBO_SIM_DATA=temp+'/data', KOBO_TEXT_SCALE=scale)
        proc, log, address = sim.start(env, folder)
        try:
            sim.drive(address, folder, ['tap-id continue','tap-id hint','clean'])
            before = sim.get(address, '/layout')['paints']
            sim.drive(address, folder, ['tap-id hint'])
            assert sim.get(address, '/layout')['paints'] == before
            idle_metrics(proc, address, folder)
            state = stored(temp+'/data')
            deadline = time.monotonic()+5
            while not state.exists() and time.monotonic()<deadline:
                time.sleep(.1)
        finally:
            sim.stop(proc, log)
        # Only this temporary simulator fixture is damaged, not user progress.
        state.write_bytes(b'{damaged fixture')
        bad = folder/'corrupt';bad.mkdir(exist_ok=True)
        proc, log, address = sim.start(env, bad)
        try:
            sim.drive(address, bad, ['expect Progress damaged.', 'clean', 'shot preserved', 'tap-id storage-dismiss',
                'tap-id continue','clean','shot readonly-board',
                'tap-id home','tap-id exit','expect Exit without saving','expect-missing Retry save',
                'clean','shot readonly-exit','tap-id cancel-exit','tap-id continue'])
            assert state.read_bytes() == b'{damaged fixture'
            state.unlink()
            sim.drive(address, bad, ['tap-id storage-open','tap-id save-progress','wait-for Continue studying','clean',
                'expect-missing Progress damaged.','shot recovered'])
            state.parent.chmod(0o500)
            sim.drive(address, bad, ['tap-id continue','wait-for Progress not saved.',
                'clean','shot failed-write','tap-id allow-sleep','clean','shot sleep-allowed',
                'tap-id home','tap-id exit','expect Exit without saving','clean','shot exit-warning',
                'tap-id cancel-exit'])
            state.parent.chmod(0o700)
            sim.drive(address, bad, ['tap-id storage-open','tap-id save-progress','wait-idle','expect-missing Progress not saved.',
                'clean','shot write-recovered'])
            # Post the actual exit tap without the drive client's follow-up
            # activity read. On successful exit that listener may already be gone.
            layout = sim.get(address, '/layout')
            exits = [node for node in layout['nodes'] if node['lines'] == ['Exit to launcher']]
            assert len(exits) == 1, exits
            point = exits[0]['centre']
            request = urllib.request.Request(
                f'http://{address}/touch', data=f"x={point['x']}&y={point['y']}".encode()
            )
            try:
                with urllib.request.urlopen(request, timeout=5) as response:
                    response.read()
                outcome = 'Exit touch acknowledged'
            except (RemoteDisconnected, ConnectionResetError):
                outcome = 'Simulator closed its established exit touch connection'
            (bad/'exit-driver.log').write_text(outcome + '\n')
            # A normal successful app exit must be observed, not assumed from a tap.
            deadline = time.monotonic()+10
            while proc.poll() is None and time.monotonic()<deadline:
                time.sleep(.1)
            assert proc.poll() == 0, ('App/CLI did not exit cleanly',proc.poll())
            assert json.loads(state.read_text())['session']['puzzle_id']
        finally:
            state.parent.chmod(0o700)
            if proc.poll() is None:
                sim.stop(proc, log)
            else:
                log.close()
    print(f'PASS recovery {scale}: corrupt data preserved, reload safe, write failure/retry, explicit loss warning, idle and exit', flush=True)
