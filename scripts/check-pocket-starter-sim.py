#!/usr/bin/env python3
"""Drive an actual generated starter and verify preview/demo storage isolation."""
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import time

import runpy

sim = runpy.run_path(str(Path(__file__).with_name("check-pocket-sim.py")))
start, stop, drive, page = (sim[name] for name in ("start", "stop", "drive", "page"))
out = Path(os.environ["POCKET_EVIDENCE"])
index_file = Path(os.environ["POCKET_STARTER_PACK"]).resolve()
data = index_file.read_bytes()
assert data[:8] == b"PCSIDX01"
metadata_size, count = struct.unpack_from("<II", data, 8)
assert count == 20_000, count
matches = []
low_count = 0
for offset in range(16 + metadata_size, 16 + metadata_size + 12 * count, 12):
    puzzle_id, _, rating, themes = struct.unpack_from("<5sIHB", data, offset)
    assert 80 <= rating <= 1500, rating
    if rating < 1200:
        low_count += 1
        if themes & 1:
            matches.append(puzzle_id.decode("ascii"))
assert len(matches) > 3


def state_file(root, key):
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        files = list(root.rglob(key))
        if files:
            return files[0]
        time.sleep(0.05)
    raise RuntimeError(f"Missing acknowledged store file for {key}")


def environment(root, profile, scale, preview):
    env = dict(os.environ, TMPDIR=str(root), KOBO_SIM_DATA=str(root / "data"),
               KOBO_SIM_PROFILE=profile, KOBO_SIM_OFFLINE="1", KOBO_TEXT_SCALE=scale,
               RUSTUP_TOOLCHAIN="1.85.1")
    if not preview:
        env.pop("POCKET_STARTER_PACK")
        env["CARGO_TARGET_DIR"] = os.environ["POCKET_DEMO_TARGET"]
    return env


def scenario(template, profile, scale, folder):
    folder.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="pocket-starter-") as temp:
        root = Path(temp)
        shutil.copytree(template, root / "data")
        demo = state_file(root, "pocket.progress.v1")
        original = demo.read_bytes()
        env = environment(root, profile, scale, True)
        before = time.monotonic()
        proc, log, address = start(env, folder)
        ready_seconds = time.monotonic() - before
        try:
            drive(address, folder, [
                "tap-id continue", "expect 1 / 20", "tap-id home", "tap-id collections",
                "expect 20 verified puzzles", "tap-id browse-starter", "expect 20000 matches",
                "tap-id rating-filter", f"expect {low_count} matches", "tap-id theme-filter",
                f"expect {len(matches)} matches", f"expect {matches[0]}", "clean", "shot filtered",
                "tap-id browse-next", f"expect {matches[3]}", "tap-id browse-prev",
                "tap-id pick-0", "expect Leave this attempt?", "tap-id confirm-leave", f"expect Lichess {matches[0]}", "tap-id hint",
            ])
            page(address, "next")
            drive(address, folder, ["wait-for Leave this attempt?", "tap-id cancel",
                                    f"expect Lichess {matches[0]}"])
            page(address, "next")
            drive(address, folder, ["wait-for Leave this attempt?", "tap-id confirm-leave",
                                    f"expect Lichess {matches[1]}", "tap-id reveal",
                                    "tap-id open-line", "expect Solution line", "clean",
                                    "shot solution-line", "tap Back", "wait-for Revealed.",
                                    "tap-id home", "expect Continue studying", "wait-idle"])
            preview = state_file(root, "pocket.preview.80-1500.v1")
            saved = json.loads(preview.read_bytes())
            assert saved["session"]["puzzle_id"] == matches[1], saved["session"]
            assert saved["session"]["revealed"]
            assert saved["records"][matches[0]]["hints_used"] == 1
            assert saved["records"][json.loads(original)["session"]["puzzle_id"]]["hints_used"] == 1
            assert demo.read_bytes() == original
            children = subprocess.check_output(["ps", "--ppid", str(proc.pid), "-o", "pid=,comm="], text=True)
            pid = next(row.split()[0] for row in children.splitlines() if row.split()[1] == "polgar-pocket")
            status = Path(f"/proc/{pid}/status").read_text().splitlines()
            rss = int(next(line for line in status if line.startswith("VmRSS:")).split()[1])
        finally:
            stop(proc, log)
        restart = folder / "restart"
        restart.mkdir(exist_ok=True)
        proc, log, address = start(env, restart)
        try:
            drive(address, restart, ["tap-id continue", f"expect Lichess {matches[1]}",
                                     "expect Solution revealed", "tap-id home"])
            assert demo.read_bytes() == original
        finally:
            stop(proc, log)
        if profile == "clara-bw-391" and scale == "default":
            # Work only on this temporary copy. A corrupt preview should not
            # trigger a fallback that replaces it with the demo history.
            preview.write_bytes(b"{damaged preview")
            recovery = folder / "recovery"
            recovery.mkdir(exist_ok=True)
            proc, log, address = start(env, recovery)
            try:
                drive(address, recovery, ["expect Progress damaged.", "tap-id storage-dismiss",
                                          "tap-id continue", "expect 1 / 20"])
                assert preview.read_bytes() == b"{damaged preview"
                assert demo.read_bytes() == original
            finally:
                stop(proc, log)
            print("PASS corrupt preview preserved; demo unchanged", flush=True)
        print(json.dumps(dict(profile=profile, text_scale=scale, ready_seconds=round(ready_seconds, 3),
                              app_rss_kib=rss, starter_count=count, low_rating_mate_count=len(matches),
                              restart="passed", demo_unchanged=True)), flush=True)


with tempfile.TemporaryDirectory(prefix="pocket-demo-seed-") as temp:
    root = Path(temp)
    evidence = out / "demo-seed"
    evidence.mkdir(parents=True, exist_ok=True)
    proc, log, address = start(environment(root, "clara-bw-391", "default", False), evidence)
    try:
        drive(address, evidence, ["tap-id continue", "tap-id hint", "wait-idle"])
        seed = json.loads(state_file(root, "pocket.progress.v1").read_bytes())
        assert seed["session"]["hints"] == 1
    finally:
        stop(proc, log)
    profiles = [os.environ["POCKET_PROFILE"]] if "POCKET_PROFILE" in os.environ else ["clara-bw-391", "libra-colour-390"]
    scales = [os.environ["POCKET_TEXT_SCALE"]] if "POCKET_TEXT_SCALE" in os.environ else ["default", "170"]
    for profile in profiles:
        for scale in scales:
            scenario(root / "data", profile, scale, out / profile / scale)
