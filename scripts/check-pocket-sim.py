#!/usr/bin/env python3
"""Real touch/hit-test integration on the pinned Cobalt simulator, no network."""
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import tempfile
import time
import urllib.request

repo = Path(os.environ.get('POCKET_APP_DIR', Path(__file__).resolve().parents[1]))
cli = Path(os.environ.get('POCKET_CLI', str(Path(os.environ.get('CARGO_TARGET_DIR', '/workspace/build/cobalt')) / 'debug/kobo')))
out = Path(os.environ.get('POCKET_EVIDENCE', '/workspace/verification/pocket-v02'))
profile_id = os.environ.get('POCKET_PROFILE', 'libra-colour-390')
pack = [json.loads(line) for line in (repo / 'packs/lichess-20.jsonl').read_text().splitlines()]
puzzles = pack[1:]

def get(address, endpoint):
    with urllib.request.urlopen('http://' + address + endpoint, timeout=5) as response:
        return json.load(response)

def drive(address, folder, steps):
    command = [str(cli), 'drive', '--address', address, '--ideal', '--shots', str(folder)]
    for step in steps:
        command += ['--step', step]
    subprocess.run(command, cwd=repo, check=True, timeout=180)

def page(address, direction):
    request = urllib.request.Request('http://' + address + '/input', data=('page ' + direction).encode(), method='POST')
    with urllib.request.urlopen(request, timeout=5) as response:
        response.read()

def start(env, folder):
    log_path = folder / 'simulator.log'
    log = log_path.open('w')
    proc = subprocess.Popen([str(cli), 'dev', '127.0.0.1:0'], cwd=repo, env=env,
                            stdout=log, stderr=log, start_new_session=True)
    try:
        deadline = time.monotonic() + 180
        while time.monotonic() < deadline:
            if proc.poll() is not None:
                raise RuntimeError('Simulator exited: ' + log_path.read_text()[-4000:])
            match = re.search(r'Kobo app simulator: http://(127\.0\.0\.1:\d+)', log_path.read_text())
            if match:
                address = match[1]
                try:
                    layout = get(address, '/layout')
                    if 'Continue studying' in json.dumps(layout):
                        return proc, log, address
                except OSError:
                    pass
            time.sleep(0.2)
        raise RuntimeError('Startup timeout')
    except BaseException:
        stop(proc, log)
        raise

def stop(proc, log):
    os.killpg(proc.pid, signal.SIGTERM)
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid, signal.SIGKILL)
        proc.wait()
    log.close()

def main():
    for scale in ['default', '170']:
        folder = out / scale
        folder.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='pocket-sim-') as temp:
            env = dict(os.environ, TMPDIR=temp, RUSTUP_TOOLCHAIN='1.85.1',
                       KOBO_SIM_PROFILE=profile_id, KOBO_SIM_OFFLINE='1',
                       KOBO_SIM_DATA=str(Path(temp) / 'data'), KOBO_TEXT_SCALE=scale)
            proc, log, address = start(env, folder)
            try:
                simulation = get(address, '/simulation')
                profile = simulation['profile']
                assert profile['id'] == profile_id
                assert (profile['width'], profile['height'], profile['colourPanel']) == {'libra-colour-390': (1264, 1680, True), 'clara-bw-391': (1072, 1448, False)}[profile_id]
                (folder / 'simulation.json').write_text(json.dumps(simulation, indent=2))
                drive(address, folder, ['clean', 'shot home', 'tap-id collections', 'clean',
                    'expect 20 verified puzzles', 'shot collections', 'tap-id select-pack',
                    'clean', 'shot board', 'tap-id coordinates', 'clean', 'shot coordinates',
                    'tap-id coordinates', 'tap-id square-g8', 'tap-id square-h8',
                    'expect Legal move, but not this solution.', 'clean', 'shot wrong-move',
                    'tap-id retry', 'tap-id hint', 'expect Hint: move the piece on',
                    'clean', 'shot hint', 'tap-id next', 'expect Leave this attempt?',
                    'clean', 'shot leave-confirmation', 'tap-id cancel', 'tap-id retry',
                    'tap-id reveal', 'expect Revealed.', 'clean', 'shot revealed',
                    'tap-id home', 'expect 0 / 20 solved', 'tap-id settings',
                    'clean', 'shot settings', 'tap-id flip', 'tap-id home', 'tap-id continue',
                    'tap-id retry'])
                page(address, 'next')
                drive(address, folder, ['expect 2 / 20', 'clean'])
                page(address, 'back')
                drive(address, folder, ['expect 1 / 20', 'clean'])
                # Solve every authentic line by tapping source and destination through
                # renderer hit testing. Opponent moves must happen automatically.
                for i, puzzle in enumerate(puzzles):
                    steps = [f'expect {i+1} / 20']
                    for move in puzzle['solution'][::2]:
                        steps += [f'tap-id square-{move[:2]}', f'tap-id square-{move[2:4]}']
                        if len(move) == 5:
                            steps += [f'tap-id promote-{move[4]}']
                    steps += ['expect Solved independently.', 'clean']
                    if i in [0, 19]:
                        steps += [f'shot solved-{i+1}']
                    if i < 19:
                        steps += ['tap-id next']
                    drive(address, folder, steps)
                drive(address, folder, ['tap-id home', 'expect 20 / 20 solved', 'clean',
                                        'shot progress', 'tap-id continue'])
                (folder / 'final-layout.json').write_text(json.dumps(get(address, '/layout'), indent=2))
                # Wait for actual store acknowledgements and check disk contains progress.
                deadline = time.monotonic() + 10
                while time.monotonic() < deadline:
                    files = list((Path(temp) / 'data').rglob('*'))
                    values = []
                    for file in files:
                        if file.is_file():
                            try:
                                item = json.loads(file.read_text())
                                if isinstance(item, dict) and item.get('collection_id') == 'lichess-tactics':
                                    values.append(item)
                            except (ValueError, UnicodeError):
                                pass
                    if values and sum(r['completed'] for r in values[-1]['records'].values()) == 20:
                        break
                    time.sleep(0.1)
                else:
                    raise AssertionError('No durable twenty-solve snapshot')
                saved = values[-1]
                assert saved['records'][puzzles[0]['id']]['revealed'] is True
                assert saved['records'][puzzles[0]['id']]['attempts'] >= 3
                (folder / 'persisted-progress.json').write_text(json.dumps(saved, indent=2))
            finally:
                stop(proc, log)
            restart = folder / 'restart'
            restart.mkdir(exist_ok=True)
            proc, log, address = start(env, restart)
            try:
                drive(address, restart, ['expect 20 / 20 solved', 'clean', 'shot restored-home',
                    'tap-id continue', 'expect 20 / 20', 'expect Solved', 'clean', 'shot restored-board'])
                # Persist and restore a partially played three-ply puzzle too.
                move = puzzles[0]['solution'][0]
                drive(address, restart, ['tap-id next', 'tap-id retry',
                    f'tap-id square-{move[:2]}', f'tap-id square-{move[2:4]}',
                    'expect Opponent replied', 'tap-id hint', 'clean', 'shot partial-attempt'])
            finally:
                stop(proc, log)
            partial = folder / 'partial-restart'
            partial.mkdir(exist_ok=True)
            proc, log, address = start(env, partial)
            try:
                drive(address, partial, ['expect 20 / 20 solved', 'tap-id continue',
                    'expect 1 / 20', 'clean', 'shot restored-partial'])
                move = puzzles[0]['solution'][2]
                drive(address, partial, [f'tap-id square-{move[:2]}', f'tap-id square-{move[2:4]}',
                    'expect Solved with assistance.', 'clean', 'shot finished-restored-partial'])
            finally:
                stop(proc, log)
            print(f'PASS {profile_id} {scale}: all 20 puzzles via touch, controls, page events, completed and partial restarts', flush=True)
    print('PASS: normal and 170% selected profile, 42 puzzle solves, 4 process restarts; no reader touched')


if __name__ == "__main__":
    main()
