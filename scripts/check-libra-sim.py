#!/usr/bin/env python3
"""Exercise upstream Lichess on the Libra Colour browser simulator, locally."""
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import tempfile
import time
import urllib.request

root = Path(os.environ.get('COBALT_SOURCE', '/workspace/deps/Cobalt'))
cli = Path(os.environ.get('CARGO_TARGET_DIR', '/workspace/build/cobalt')) / 'debug/kobo'
out = Path('/workspace/verification/libra-colour')
out.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix='kobo-chess-sim-') as state, (out / 'simulator.log').open('w') as log:
    env = dict(os.environ, TMPDIR=state, KOBO_SIM_DATA=str(Path(state) / 'data'),
               KOBO_SIM_PROFILE='libra-colour-390', KOBO_SIM_OFFLINE='1')
    env.pop('KOBO_LICHESS_DEMO', None)
    proc = subprocess.Popen([str(cli), 'dev', '127.0.0.1:0'], cwd=root / 'apps/lichess',
                            env=env, stdout=log, stderr=log, start_new_session=True)
    try:
        deadline = time.monotonic() + 180
        while time.monotonic() < deadline:
            if proc.poll() is not None:
                raise RuntimeError('Simulator exited; see simulator.log')
            match = re.search(r'Kobo app simulator: http://(127\.0\.0\.1:\d+)', (out / 'simulator.log').read_text())
            if match:
                address = match[1]
                try:
                    with urllib.request.urlopen('http://' + address + '/layout', timeout=2) as response:
                        layout = json.load(response)
                    if 'Play the computer' in json.dumps(layout):
                        break
                except OSError:
                    pass
            time.sleep(0.3)
        else:
            raise RuntimeError('No Lichess home screen')
        steps = ['expect Lichess', 'expect Play the computer', 'shot libra-home',
                 'tap-id play-computer', 'wait-for Start game', 'tap-id start-computer',
                 'wait-for Kobo', 'shot libra-board']
        command = [str(cli), 'drive', '--address', address, '--ideal', '--shots', str(out)]
        for step in steps:
            command += ['--step', step]
        subprocess.run(command, cwd=root, env=env, check=True, timeout=120)
        with urllib.request.urlopen('http://' + address + '/simulation', timeout=5) as response:
            simulation = json.load(response)
        assert simulation['profile']['id'] == 'libra-colour-390', simulation['profile']
        assert (simulation['profile']['width'], simulation['profile']['height']) == (1264, 1680)
        assert simulation['profile']['colourPanel'] is True
        (out / 'simulation.json').write_text(json.dumps(simulation, indent=2) + '\n')
        (out / 'home-layout.json').write_text(json.dumps(layout, indent=2) + '\n')
        print('PASS: Libra Colour simulator rendered Lichess home and started an offline game')
    finally:
        os.killpg(proc.pid, signal.SIGTERM)
        try:
            proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
